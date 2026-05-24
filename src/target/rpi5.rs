#[cfg(talos_rpi5_timer_irq_diagnostic)]
use crate::arch::aarch64::{
    generic_timer,
    gicv2::{GicV2, SPURIOUS_INTID},
};
use crate::{
    boot::BootInfo,
    device_tree::DeviceTree,
    mmio::{MmioMap, MmioRegion},
    target::{InterruptControllerKind, TargetServices, TimerKind, UartKind},
};
#[cfg(talos_rpi5_timer_irq_diagnostic)]
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(talos_target_rpi5_bcm2712)]
use crate::pl011::Pl011;

pub const UART10_BASE: usize = 0x10_7d00_1000;
pub const RP1_UART0_PCIE2_BASE: usize = 0x1f_0003_0000;
pub const RP1_UART0_FIRMWARE_BASE: usize = 0x1c_0003_0000;
pub const RP1_UART0_BASE: usize = RP1_UART0_PCIE2_BASE;
#[allow(dead_code)]
pub const RP1_UART0_GPIO14_PAD: usize = 0x1f_000f_003c;
#[allow(dead_code)]
pub const RP1_UART0_GPIO15_PAD: usize = 0x1f_000f_0040;
#[allow(dead_code)]
pub const RP1_UART0_GPIO14_CTRL: usize = 0x1f_000d_0074;
#[allow(dead_code)]
pub const RP1_UART0_GPIO15_CTRL: usize = 0x1f_000d_007c;
#[cfg(talos_rpi5_timer_irq_diagnostic)]
const GICD_BASE: usize = 0x10_7fff_9000;
#[cfg(talos_rpi5_timer_irq_diagnostic)]
const GICC_BASE: usize = 0x10_7fff_a000;
#[cfg(talos_rpi5_timer_irq_diagnostic)]
const EL2_PHYSICAL_TIMER_INTID: u32 = 26;
#[cfg(talos_rpi5_timer_irq_diagnostic)]
const TIMER_IRQ_WAIT_LIMIT: usize = 8_000_000;

const MMIO_REGIONS: &[MmioRegion] = &[
    MmioRegion::new("bcm2712-local-peripherals", 0x10_7c00_0000, 0x0400_0000),
    MmioRegion::new("bcm2712-gic-400", 0x10_7fff_9000, 0x0001_0000),
    MmioRegion::new("bcm2712-uart10-pl011", UART10_BASE, 0x0000_0200),
    MmioRegion::new("rp1-uart0-pl011-pcie2", RP1_UART0_BASE, 0x0000_0100),
    MmioRegion::new("rp1-gpio-pads", 0x1f_000f_0000, 0x0000_1000),
    MmioRegion::new("rp1-gpio-ctrl", 0x1f_000d_0000, 0x0000_1000),
    MmioRegion::new(
        "rp1-uart0-pl011-firmware-preserved",
        RP1_UART0_FIRMWARE_BASE,
        0x0000_0100,
    ),
];

#[cfg(talos_rpi5_timer_irq_diagnostic)]
static TIMER_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_timer_irq_diagnostic)]
static LAST_IRQ_VECTOR: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_timer_irq_diagnostic)]
static LAST_IAR: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_timer_irq_diagnostic)]
static LAST_INTID: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_timer_irq_diagnostic)]
static UNEXPECTED_GIC_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn init_stub() {
    init_rp1_uart0_pins();
    // serial10 is already active for firmware/BL31 logs; avoid disturbing baud
    // while testing Talos' runtime console path.
}

#[cfg(talos_target_rpi5_bcm2712)]
fn init_rp1_uart0_pins() {
    write_rp1_reg_flush(RP1_UART0_GPIO14_PAD, 0x40);
    write_rp1_reg_flush(RP1_UART0_GPIO15_PAD, 0x48);
    write_rp1_reg_flush(RP1_UART0_GPIO14_CTRL, 4);
    write_rp1_reg_flush(RP1_UART0_GPIO15_CTRL, 4);
}

#[cfg(not(talos_target_rpi5_bcm2712))]
fn init_rp1_uart0_pins() {}

#[cfg(talos_target_rpi5_bcm2712)]
fn write_rp1_reg_flush(addr: usize, value: u32) {
    let reg = addr as *mut u32;
    unsafe {
        core::ptr::write_volatile(reg, value);
        let _ = core::ptr::read_volatile(reg);
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn firmware_console() -> Pl011 {
    Pl011::new_with_posted_write_flush(UART10_BASE)
}

#[cfg(talos_rpi5_timer_irq_diagnostic)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerIrqSnapshot {
    pub timer_count: u64,
    pub last_vector: u64,
    pub last_iar: u64,
    pub last_intid: u64,
    pub unexpected_gic_count: u64,
}

#[cfg(talos_rpi5_timer_irq_diagnostic)]
pub fn timer_irq_snapshot() -> TimerIrqSnapshot {
    TimerIrqSnapshot {
        timer_count: TIMER_IRQ_COUNT.load(Ordering::Relaxed),
        last_vector: LAST_IRQ_VECTOR.load(Ordering::Relaxed),
        last_iar: LAST_IAR.load(Ordering::Relaxed),
        last_intid: LAST_INTID.load(Ordering::Relaxed),
        unexpected_gic_count: UNEXPECTED_GIC_IRQ_COUNT.load(Ordering::Relaxed),
    }
}

#[cfg(talos_rpi5_timer_irq_diagnostic)]
pub fn handle_irq(vector: u64) -> bool {
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let iar = unsafe { gic.acknowledge() };
    let intid = iar & 0x03ff;

    LAST_IRQ_VECTOR.store(vector, Ordering::Relaxed);
    LAST_IAR.store(iar as u64, Ordering::Relaxed);
    LAST_INTID.store(intid as u64, Ordering::Relaxed);

    if intid == EL2_PHYSICAL_TIMER_INTID {
        unsafe {
            generic_timer::mask_el2_physical_timer();
            gic.end_interrupt(iar);
        }
        TIMER_IRQ_COUNT.fetch_add(1, Ordering::Relaxed);
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

#[cfg(talos_rpi5_timer_irq_diagnostic)]
pub fn run_el2_timer_irq_smoke() -> bool {
    unsafe {
        crate::arch::aarch64::disable_irq();
        crate::arch::aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
        GicV2::new(GICD_BASE, GICC_BASE).enable_ppi_or_spi(EL2_PHYSICAL_TIMER_INTID);
    }

    let freq = generic_timer::counter_frequency_hz();
    let start = generic_timer::physical_count();
    let delta = generic_timer::first_smoke_delta_ticks(freq);
    let compare = start.wrapping_add(delta);

    crate::println!(
        "rpi5-timer-irq-smoke: gicd={:#014x} gicc={:#014x} intid={}",
        GICD_BASE,
        GICC_BASE,
        EL2_PHYSICAL_TIMER_INTID
    );
    crate::println!(
        "rpi5-timer-irq-smoke: cntfrq={} start={} cval={} delta={}",
        freq,
        start,
        compare,
        delta
    );

    let mut workload = 0x1234_5678_9abc_def0u64;
    unsafe {
        generic_timer::program_el2_physical_compare(compare);
        crate::arch::aarch64::enable_irq();
    }

    let mut remaining = TIMER_IRQ_WAIT_LIMIT;
    while timer_irq_snapshot().timer_count == 0 && remaining > 0 {
        workload = workload.rotate_left(7) ^ 0x0f0e_0d0c_0b0a_0908;
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        crate::arch::aarch64::disable_irq();
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
    let daif = crate::arch::aarch64::daif();
    let control = generic_timer::el2_physical_control();
    crate::println!(
        "rpi5-timer-irq-smoke: irq-count={} vector={} iar={:#010x} intid={} unexpected={} ctl={:#x}",
        snapshot.timer_count,
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control
    );
    crate::println!(
        "rpi5-timer-irq-smoke: gic enable={:#010x} pending={:#010x} active={:#010x} hppir={:#010x} daif={:#x}",
        enable_bits,
        pending_bits,
        active_bits,
        highest_pending,
        daif
    );
    crate::println!(
        "rpi5-timer-irq-smoke: post-irq workload={:#018x} remaining={}",
        workload,
        remaining
    );

    let passed = snapshot.timer_count > 0
        && snapshot.last_intid == EL2_PHYSICAL_TIMER_INTID as u64
        && snapshot.unexpected_gic_count == 0;

    if passed {
        crate::println!("rpi5-timer-irq-smoke: PASS");
    } else {
        crate::println!("rpi5-timer-irq-smoke: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_target_rpi5_bcm2712)]
pub enum EarlyPhaseLine {
    RustEntry,
    BootInfoParsed,
    TargetInit,
    ExceptionsReady,
    KernelMain,
    DtbReservationsStart,
    DtbReservationsDone,
    DtbMemoryScanStart,
    DtbMemoryScanDone,
    MmuEnableStart,
    MmuEnableDone,
    IcacheEnableStart,
    IcacheEnableDone,
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_phase_line(line: EarlyPhaseLine) {
    write_uart10_byte_early_phase(b'T');
    write_uart10_byte_early_phase(b'A');
    write_uart10_byte_early_phase(b'L');
    write_uart10_byte_early_phase(b'O');
    write_uart10_byte_early_phase(b'S');
    write_uart10_byte_early_phase(b':');
    write_uart10_byte_early_phase(b' ');

    match line {
        EarlyPhaseLine::RustEntry => {
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'_');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
        }
        EarlyPhaseLine::BootInfoParsed => {
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'f');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'p');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
        }
        EarlyPhaseLine::TargetInit => {
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'g');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::ExceptionsReady => {
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'x');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'p');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'y');
        }
        EarlyPhaseLine::KernelMain => {
            write_uart10_byte_early_phase(b'k');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'_');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
        }
        EarlyPhaseLine::DtbReservationsStart => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'v');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::DtbReservationsDone => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'v');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::DtbMemoryScanStart => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::DtbMemoryScanDone => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::MmuEnableStart => {
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::MmuEnableDone => {
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::IcacheEnableStart => {
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'h');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::IcacheEnableDone => {
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'h');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
    }

    write_uart10_byte_early_phase(b'\r');
    write_uart10_byte_early_phase(b'\n');
    wait_uart10_empty_early_phase();
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_hex_u64(value: u64) {
    write_uart10_byte_early_phase(b'0');
    write_uart10_byte_early_phase(b'x');

    let mut started = false;
    let mut shift = u64::BITS;
    while shift != 0 {
        shift -= 4;
        let nibble = ((value >> shift) & 0xf) as u8;
        if nibble != 0 || started || shift == 0 {
            started = true;
            write_early_hex_digit(nibble);
        }
    }

    wait_uart10_empty_early_phase();
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_static(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            write_uart10_byte_early_phase(b'\r');
            wait_uart10_empty_early_phase();
        }
        write_uart10_byte_early_phase(byte);
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
fn write_early_hex_digit(nibble: u8) {
    let digit = if nibble < 10 {
        b'0'.wrapping_add(nibble)
    } else if nibble < 16 {
        b'a'.wrapping_add(nibble.wrapping_sub(10))
    } else {
        b'?'
    };
    write_uart10_byte_early_phase(digit);
}

#[cfg(talos_target_rpi5_bcm2712)]
#[inline(always)]
pub(crate) fn write_uart10_byte_early_phase(byte: u8) {
    let value = byte as u32;
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "dsb sy",
            in("w11") value,
            lateout("x9") _,
            lateout("x10") _,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn wait_uart10_empty_early_phase() {
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "mov x21, #0x200000",
            "2:",
            "ldr w10, [x9, #0x18]",
            "tbnz w10, #7, 3f",
            "subs x21, x21, #1",
            "b.ne 2b",
            "3:",
            "dsb sy",
            lateout("x9") _,
            lateout("x10") _,
            lateout("x21") _,
            options(nostack)
        );
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn relocate_early_linked_addr(addr: usize) -> usize {
    // The accepted normal Pi 5 Image links and runs at 0x200000, so this is
    // normally a no-op. Keep the helper for vector installation and explicit
    // address-contract diagnostics while that part of bring-up is still active.
    addr.wrapping_add(runtime_relocation_delta())
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn runtime_relocation_delta() -> usize {
    let mut runtime_pc: usize;
    let mut linked_pc: usize;
    unsafe {
        core::arch::asm!(
            "adr {runtime}, 1f",
            "ldr {linked}, =1f",
            "1:",
            runtime = out(reg) runtime_pc,
            linked = out(reg) linked_pc,
            options(nostack, preserves_flags)
        );
    }
    runtime_pc.wrapping_sub(linked_pc)
}

pub fn services(boot_info: &BootInfo) -> TargetServices {
    TargetServices {
        uart: UartKind::FirmwarePreserved,
        timer: TimerKind::ArmGeneric,
        interrupt_controller: InterruptControllerKind::GicV2,
        mmio_map: MmioMap::new(MMIO_REGIONS),
        device_tree: DeviceTree::from_physical_address(boot_info.dtb_pa),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn pi5_uart10_address_matches_bcm2712_soc_range() {
        assert_eq!(UART10_BASE, 0x10_7d00_1000);
        assert_eq!(RP1_UART0_PCIE2_BASE, 0x1f_0003_0000);
        assert_eq!(RP1_UART0_FIRMWARE_BASE, 0x1c_0003_0000);
        assert_eq!(RP1_UART0_GPIO14_PAD, 0x1f_000f_003c);
        assert_eq!(RP1_UART0_GPIO15_PAD, 0x1f_000f_0040);
        assert_eq!(RP1_UART0_GPIO14_CTRL, 0x1f_000d_0074);
        assert_eq!(RP1_UART0_GPIO15_CTRL, 0x1f_000d_007c);
        assert_eq!(RP1_UART0_BASE, RP1_UART0_PCIE2_BASE);
    }
}
