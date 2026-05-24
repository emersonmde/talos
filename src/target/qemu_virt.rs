#![cfg_attr(any(test, talos_target_rpi5_bcm2712), allow(dead_code))]

use core::sync::atomic::{AtomicU64, Ordering};

use crate::{
    arch::aarch64::{
        self, generic_timer,
        gicv2::{GicV2, SPURIOUS_INTID},
    },
    boot::BootInfo,
    device_tree::DeviceTree,
    mmio::{MmioMap, MmioRegion},
    pl011::Pl011,
    target::{InterruptControllerKind, TargetServices, TimerKind, UartKind},
};

const PL011_BASE: usize = 0x0900_0000;
const GICD_BASE: usize = 0x0800_0000;
const GICC_BASE: usize = 0x0801_0000;
const EL2_PHYSICAL_TIMER_INTID: u32 = 26;
const TIMER_IRQ_WAIT_LIMIT: usize = 1_000_000;

const MMIO_REGIONS: &[MmioRegion] = &[
    MmioRegion::new("qemu-virt-gicv2-distributor", GICD_BASE, 0x0001_0000),
    MmioRegion::new("qemu-virt-gicv2-cpu-interface", GICC_BASE, 0x0001_0000),
    MmioRegion::new("qemu-virt-pl011-uart0", PL011_BASE, 0x1000),
];

static LAST_IRQ_VECTOR: AtomicU64 = AtomicU64::new(0);
static LAST_IAR: AtomicU64 = AtomicU64::new(0);
static LAST_INTID: AtomicU64 = AtomicU64::new(0);
static UNEXPECTED_GIC_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
struct SingleCoreIrqMaskProbe {
    nested_start_masked: bool,
    inner_restored_masked: bool,
    outer_restored_masked: bool,
    unmasked_start: bool,
    saved_unmasked_masked: bool,
    restored_unmasked: bool,
}

impl SingleCoreIrqMaskProbe {
    const fn passed(self) -> bool {
        self.nested_start_masked
            && self.inner_restored_masked
            && self.outer_restored_masked
            && self.unmasked_start
            && self.saved_unmasked_masked
            && self.restored_unmasked
    }
}

pub fn init() {
    console().init_early();
}

pub fn console() -> Pl011 {
    Pl011::new(PL011_BASE)
}

pub fn services(boot_info: &BootInfo) -> TargetServices {
    TargetServices {
        uart: UartKind::Pl011,
        timer: TimerKind::ArmGeneric,
        interrupt_controller: InterruptControllerKind::GicV2,
        mmio_map: MmioMap::new(MMIO_REGIONS),
        device_tree: DeviceTree::from_physical_address(boot_info.dtb_pa),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerIrqSnapshot {
    pub timer_count: u64,
    pub last_vector: u64,
    pub last_iar: u64,
    pub last_intid: u64,
    pub unexpected_gic_count: u64,
}

pub fn timer_irq_snapshot() -> TimerIrqSnapshot {
    TimerIrqSnapshot {
        timer_count: generic_timer::monotonic_ticks(),
        last_vector: LAST_IRQ_VECTOR.load(Ordering::Relaxed),
        last_iar: LAST_IAR.load(Ordering::Relaxed),
        last_intid: LAST_INTID.load(Ordering::Relaxed),
        unexpected_gic_count: UNEXPECTED_GIC_IRQ_COUNT.load(Ordering::Relaxed),
    }
}

pub fn handle_irq(vector: u64) -> bool {
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let iar = unsafe { gic.acknowledge() };
    let intid = iar & 0x03ff;

    LAST_IRQ_VECTOR.store(vector, Ordering::Relaxed);
    LAST_IAR.store(iar as u64, Ordering::Relaxed);
    LAST_INTID.store(intid as u64, Ordering::Relaxed);

    if intid == EL2_PHYSICAL_TIMER_INTID {
        unsafe { generic_timer::record_el2_physical_tick_and_rearm() };
        unsafe {
            gic.end_interrupt(iar);
        }
        return true;
    }

    UNEXPECTED_GIC_IRQ_COUNT.fetch_add(1, Ordering::Relaxed);
    if intid != SPURIOUS_INTID {
        unsafe {
            gic.end_interrupt(iar);
        }
    }
    true
}

fn run_single_core_irq_mask_probe() -> SingleCoreIrqMaskProbe {
    unsafe {
        aarch64::disable_irq();
    }
    let nested_start_masked = aarch64::irq_masked();
    let outer = unsafe { aarch64::single_core_irq_mask_save() };
    let inner = unsafe { aarch64::single_core_irq_mask_save() };
    unsafe {
        aarch64::single_core_irq_restore(inner);
    }
    let inner_restored_masked = aarch64::irq_masked();
    unsafe {
        aarch64::single_core_irq_restore(outer);
    }
    let outer_restored_masked = aarch64::irq_masked();

    unsafe {
        aarch64::enable_irq();
    }
    let unmasked_start = !aarch64::irq_masked();
    let unmasked = unsafe { aarch64::single_core_irq_mask_save() };
    let saved_unmasked_masked = !unmasked.was_irq_masked() && aarch64::irq_masked();
    unsafe {
        aarch64::single_core_irq_restore(unmasked);
    }
    let restored_unmasked = !aarch64::irq_masked();
    unsafe {
        aarch64::disable_irq();
    }

    SingleCoreIrqMaskProbe {
        nested_start_masked,
        inner_restored_masked,
        outer_restored_masked,
        unmasked_start,
        saved_unmasked_masked,
        restored_unmasked,
    }
}

pub fn run_el2_timer_irq_smoke() -> bool {
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
    }
    let irq_mask_probe = run_single_core_irq_mask_probe();
    unsafe {
        GicV2::new(GICD_BASE, GICC_BASE).enable_ppi_or_spi(EL2_PHYSICAL_TIMER_INTID);
    }
    LAST_IRQ_VECTOR.store(0, Ordering::Relaxed);
    LAST_IAR.store(0, Ordering::Relaxed);
    LAST_INTID.store(0, Ordering::Relaxed);
    UNEXPECTED_GIC_IRQ_COUNT.store(0, Ordering::Relaxed);
    generic_timer::reset_monotonic_ticks();

    let freq = generic_timer::counter_frequency_hz();
    let start = generic_timer::physical_count();
    let delta = generic_timer::periodic_tick_delta_ticks(freq);
    let compare = start.wrapping_add(delta);
    let target_ticks = generic_timer::periodic_tick_proof_count();
    generic_timer::configure_periodic_tick_delta(delta);

    crate::println!(
        "qemu-timer-irq-smoke: gicd={:#010x} gicc={:#010x} intid={}",
        GICD_BASE,
        GICC_BASE,
        EL2_PHYSICAL_TIMER_INTID
    );
    crate::println!(
        "qemu-timer-irq-smoke: cntfrq={} start={} cval={} delta={} target-ticks={}",
        freq,
        start,
        compare,
        delta,
        target_ticks
    );
    crate::println!(
        "qemu-timer-irq-smoke: irq-mask nested-start={} inner-restored={} outer-restored={} unmasked-start={} saved-mask={} restored-unmasked={}",
        irq_mask_probe.nested_start_masked,
        irq_mask_probe.inner_restored_masked,
        irq_mask_probe.outer_restored_masked,
        irq_mask_probe.unmasked_start,
        irq_mask_probe.saved_unmasked_masked,
        irq_mask_probe.restored_unmasked
    );

    let mut workload = 0x1234_5678_9abc_def0u64;
    unsafe {
        generic_timer::program_el2_physical_compare(compare);
        aarch64::enable_irq();
    }

    let mut remaining = TIMER_IRQ_WAIT_LIMIT;
    let mut critical_sections = 0usize;
    while timer_irq_snapshot().timer_count < target_ticks && remaining > 0 {
        let saved_irq_state = unsafe { aarch64::single_core_irq_mask_save() };
        workload = workload.rotate_left(7) ^ 0x0f0e_0d0c_0b0a_0908;
        unsafe {
            aarch64::single_core_irq_restore(saved_irq_state);
        }
        critical_sections += 1;
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let snapshot = timer_irq_snapshot();
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let (enable_bits, pending_bits, active_bits, highest_pending) = unsafe {
        (
            gic.enable_bits(EL2_PHYSICAL_TIMER_INTID),
            gic.pending_bits(EL2_PHYSICAL_TIMER_INTID),
            gic.active_bits(EL2_PHYSICAL_TIMER_INTID),
            gic.highest_pending(),
        )
    };
    let daif = aarch64::daif();
    let control = generic_timer::el2_physical_control();
    crate::println!(
        "qemu-timer-irq-smoke: tick-count={} target={} vector={} iar={:#010x} intid={} unexpected={} ctl={:#x}",
        snapshot.timer_count,
        target_ticks,
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control
    );
    crate::println!(
        "qemu-timer-irq-smoke: gic enable={:#010x} pending={:#010x} active={:#010x} hppir={:#010x} daif={:#x}",
        enable_bits,
        pending_bits,
        active_bits,
        highest_pending,
        daif
    );
    crate::println!(
        "qemu-timer-irq-smoke: post-irq workload={:#018x} remaining={} critical-sections={}",
        workload,
        remaining,
        critical_sections
    );

    let passed = snapshot.timer_count > 0
        && snapshot.timer_count >= target_ticks
        && snapshot.last_intid == EL2_PHYSICAL_TIMER_INTID as u64
        && snapshot.unexpected_gic_count == 0
        && irq_mask_probe.passed()
        && critical_sections > 0;

    if passed {
        crate::println!("qemu-timer-irq-smoke: PASS");
    } else {
        crate::println!("qemu-timer-irq-smoke: FAIL");
    }

    passed
}
