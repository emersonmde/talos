#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
use crate::arch::aarch64::{
    self, generic_timer,
    gicv2::{GicV2, SPURIOUS_INTID},
};
#[cfg(talos_rpi5_timer_preemption_diagnostic)]
use crate::scheduler::{ContextFrame, KernelStack, SingleCoreScheduler, Task, TaskId, TaskState};
#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
use crate::smp::{
    self, CoreLifecycle, CoreStackLayout, MAX_CORES, SECONDARY_CORE_STATES,
    SECONDARY_CORE_WORKLOAD_TARGET, SECONDARY_KERNEL_STACK_SIZE,
    pi5_logical_cpu_from_mpidr_affinity,
};
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
use crate::smp_sync::{SpinLock, smp_full_barrier};
use crate::{
    boot::BootInfo,
    device_tree::DeviceTree,
    mmio::{MmioMap, MmioRegion},
    target::{InterruptControllerKind, TargetServices, TimerKind, UartKind},
};
#[cfg(talos_rpi5_timer_preemption_diagnostic)]
use core::cell::UnsafeCell;
#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic,
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
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
#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
const GICD_BASE: usize = 0x10_7fff_9000;
#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
const GICC_BASE: usize = 0x10_7fff_a000;
#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
const EL2_PHYSICAL_TIMER_INTID: u32 = 26;
#[cfg(talos_rpi5_timer_irq_diagnostic)]
const TIMER_IRQ_WAIT_LIMIT: usize = 8_000_000;
#[cfg(any(
    talos_rpi5_uart10_polling_rx_diagnostic,
    talos_rpi5_diagnostic_command_channel_proof
))]
const UART10_RX_WAIT_LIMIT: usize = 200_000_000;
#[cfg(talos_rpi5_diagnostic_command_channel_proof)]
const DIAGNOSTIC_COMMAND_CAPTURE_SETTLE_SPINS: usize = 10_000_000;
#[cfg(talos_rpi5_timer_preemption_diagnostic)]
const CONTEXT_SWITCH_STACK_SIZE: usize = 4096;
#[cfg(talos_rpi5_timer_preemption_diagnostic)]
const TIMER_PREEMPTION_TARGET_PROGRESS: u64 = 3;
#[cfg(talos_rpi5_timer_preemption_diagnostic)]
const TIMER_PREEMPTION_TARGET_SWITCHES: u64 = 6;
#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
const RPI5_SECONDARY_WAIT_LIMIT: usize = 200_000_000;
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
const RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE: u64 = 64;
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
const RPI5_SMP_LOCK_ACQUIRE_SPIN_LIMIT: u64 = 1_000_000;
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
const RPI5_SMP_LOCK_WAIT_POLL_INTERVAL: usize = 20_000_000;
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
const RPI5_SCTLR_M_ENABLE: u64 = 1 << 0;
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
const RPI5_SCTLR_C_ENABLE: u64 = 1 << 2;
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
const RPI5_SCTLR_I_ENABLE: u64 = 1 << 12;
#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
const PSCI_AFFINITY_INFO: u64 = 0x8400_0004;
#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
const PSCI_CPU_ON: u64 = 0xc400_0003;

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

#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
static LAST_IRQ_VECTOR: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
static LAST_IAR: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
static LAST_INTID: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
static UNEXPECTED_GIC_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_timer_preemption_diagnostic)]
static TIMER_PREEMPTION_REQUESTS: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
unsafe extern "C" {
    fn talos_aarch64_rpi5_secondary_entry();
    static talos_secondary_core_stacks: u8;
    static talos_secondary_core_stacks_end: u8;
}

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

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
fn secondary_stack_layout() -> CoreStackLayout {
    let base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    CoreStackLayout::new(base, end, MAX_CORES, SECONDARY_KERNEL_STACK_SIZE)
        .expect("valid linked secondary-core stack layout")
}

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
fn secondary_state_name(state: u64) -> &'static str {
    CoreLifecycle::from_raw(state).map_or("unknown", CoreLifecycle::name)
}

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
unsafe fn psci_smc(function_id: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let mut result = function_id;
    let scratch1 = arg1;
    let scratch2 = arg2;
    let scratch3 = arg3;
    unsafe {
        core::arch::asm!(
            "smc #0",
            inout("x0") result,
            inout("x1") scratch1 => _,
            inout("x2") scratch2 => _,
            inout("x3") scratch3 => _,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            options(nostack)
        );
    }
    result as i64
}

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
unsafe fn psci_cpu_on_smc(target_affinity: u64, entry: usize, context: usize) -> i64 {
    unsafe { psci_smc(PSCI_CPU_ON, target_affinity, entry as u64, context as u64) }
}

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
unsafe fn psci_affinity_info_smc(target_affinity: u64, lowest_affinity_level: u64) -> i64 {
    unsafe {
        psci_smc(
            PSCI_AFFINITY_INFO,
            target_affinity,
            lowest_affinity_level,
            0,
        )
    }
}

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
fn psci_affinity_state_name(state: i64) -> &'static str {
    match state {
        0 => "on",
        1 => "off",
        2 => "on-pending",
        _ => "error-or-unknown",
    }
}

#[cfg(any(
    talos_rpi5_psci_secondary_core_alive_proof,
    talos_rpi5_secondary_core_workload_proof,
    talos_rpi5_smp_lock_cache_coherence_proof
))]
#[unsafe(no_mangle)]
pub extern "C" fn talos_rpi5_secondary_entry(context: usize) -> ! {
    write_uart10_bytes_early_phase(b"TALOS: secondary_rust_entry\r\n");

    let mpidr = crate::arch::aarch64::mpidr_el1();
    let affinity = crate::arch::aarch64::mpidr_affinity(mpidr);
    let logical_cpu = pi5_logical_cpu_from_mpidr_affinity(affinity).unwrap_or(context);
    if logical_cpu < MAX_CORES {
        let core_state = &SECONDARY_CORE_STATES[logical_cpu];
        core_state.enter(context, mpidr, affinity);

        let stack_pointer: u64;
        unsafe {
            core::arch::asm!("mov {stack_pointer}, sp", stack_pointer = out(reg) stack_pointer, options(nomem, nostack, preserves_flags));
        }
        core_state.mark_stack_ready(stack_pointer as usize);
        core_state.mark_registered();
        #[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
        if !enter_secondary_cacheable_mmu_handoff(logical_cpu) {
            core_state.clean_to_poc();
            write_uart10_bytes_early_phase(b"TALOS: secondary_cacheable_mmu_handoff_failed\r\n");
            loop {
                unsafe {
                    core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
                }
            }
        }
        #[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
        core_state.republish_identity(context, mpidr, affinity, stack_pointer as usize);
        core_state.mark_handoff_ready();
        core_state.clean_to_poc();
        write_uart10_bytes_early_phase(b"TALOS: secondary_state_published\r\n");
        #[cfg(talos_rpi5_secondary_core_workload_proof)]
        {
            smp::run_controlled_secondary_workload(core_state, SECONDARY_CORE_WORKLOAD_TARGET);
            write_uart10_bytes_early_phase(b"TALOS: secondary_workload_complete\r\n");
        }
        #[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
        {
            run_smp_lock_contention_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_lock_contention_complete\r\n");
        }
    }

    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
#[derive(Clone, Copy)]
struct SmpLockContentionState {
    shared_counter: u64,
    per_core_counts: [u64; MAX_CORES],
    error_count: u64,
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
impl SmpLockContentionState {
    const fn new() -> Self {
        Self {
            shared_counter: 0,
            per_core_counts: [0; MAX_CORES],
            error_count: 0,
        }
    }
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SMP_LOCK_CONTENTION_STATE: SpinLock<SmpLockContentionState> =
    SpinLock::new(SmpLockContentionState::new());

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u64)]
enum SmpLockDiagnosticPhase {
    Idle = 0,
    SecondaryEntered = 1,
    BeforeLockAttempt = 2,
    WaitingForLock = 3,
    LockAcquired = 4,
    LockReleased = 5,
    IterationComplete = 6,
    WorkloadComplete = 7,
    LockAcquireTimeout = 8,
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
impl SmpLockDiagnosticPhase {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            1 => Self::SecondaryEntered,
            2 => Self::BeforeLockAttempt,
            3 => Self::WaitingForLock,
            4 => Self::LockAcquired,
            5 => Self::LockReleased,
            6 => Self::IterationComplete,
            7 => Self::WorkloadComplete,
            8 => Self::LockAcquireTimeout,
            _ => Self::Idle,
        }
    }

    const fn raw(self) -> u64 {
        self as u64
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::SecondaryEntered => "secondary-entered",
            Self::BeforeLockAttempt => "before-lock-attempt",
            Self::WaitingForLock => "waiting-for-lock",
            Self::LockAcquired => "lock-acquired",
            Self::LockReleased => "lock-released",
            Self::IterationComplete => "iteration-complete",
            Self::WorkloadComplete => "workload-complete",
            Self::LockAcquireTimeout => "lock-acquire-timeout",
        }
    }
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
#[derive(Clone, Copy)]
struct SmpLockDiagnosticSnapshot {
    phase: SmpLockDiagnosticPhase,
    progress: u64,
    attempts: u64,
    timeouts: u64,
    releases: u64,
    sctlr_el2: u64,
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SMP_LOCK_DIAGNOSTIC_PHASES: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(SmpLockDiagnosticPhase::Idle.raw()) }; MAX_CORES];
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SMP_LOCK_DIAGNOSTIC_PROGRESS: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SMP_LOCK_DIAGNOSTIC_ATTEMPTS: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SMP_LOCK_DIAGNOSTIC_TIMEOUTS: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SMP_LOCK_DIAGNOSTIC_RELEASES: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SMP_LOCK_DIAGNOSTIC_SCTLR_EL2: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SECONDARY_CACHEABLE_MMU_HANDOFF_READY: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
static SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2: AtomicU64 = AtomicU64::new(0);

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn clean_secondary_cacheable_mmu_handoff_plan() {
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_READY);
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn publish_secondary_cacheable_mmu_handoff_plan(
    regime: crate::arch::aarch64::El2Stage1CacheRegime,
) {
    SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
    clean_secondary_cacheable_mmu_handoff_plan();
    SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2.store(regime.mair, Ordering::Release);
    SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2.store(regime.tcr, Ordering::Release);
    SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2.store(regime.ttbr0, Ordering::Release);
    SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2.store(
        regime.sctlr | RPI5_SCTLR_M_ENABLE | RPI5_SCTLR_I_ENABLE | RPI5_SCTLR_C_ENABLE,
        Ordering::Release,
    );
    SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(1, Ordering::Release);
    clean_secondary_cacheable_mmu_handoff_plan();
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn secondary_cacheable_mmu_handoff_plan() -> Option<crate::arch::aarch64::El2Stage1CacheRegime> {
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_READY);
    if SECONDARY_CACHEABLE_MMU_HANDOFF_READY.load(Ordering::Acquire) != 1 {
        return None;
    }

    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2);
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2);
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2);
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2);
    Some(crate::arch::aarch64::El2Stage1CacheRegime {
        mair: SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2.load(Ordering::Acquire),
        tcr: SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2.load(Ordering::Acquire),
        ttbr0: SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2.load(Ordering::Acquire),
        sctlr: SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2.load(Ordering::Acquire),
    })
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn enter_secondary_cacheable_mmu_handoff(logical_cpu: usize) -> bool {
    let Some(plan) = secondary_cacheable_mmu_handoff_plan() else {
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::SecondaryEntered,
            0,
            0,
            0,
            0,
            current_sctlr_el2(),
        );
        return false;
    };

    let Some(after) = (unsafe { crate::arch::aarch64::install_el2_stage1_cache_regime(plan) })
    else {
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::SecondaryEntered,
            0,
            0,
            0,
            0,
            current_sctlr_el2(),
        );
        return false;
    };
    record_smp_lock_diagnostic(
        logical_cpu,
        SmpLockDiagnosticPhase::SecondaryEntered,
        0,
        0,
        0,
        0,
        after.sctlr,
    );
    cacheable_mmu_enabled(after.sctlr)
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn reset_smp_lock_contention_state() {
    let mut state = SMP_LOCK_CONTENTION_STATE.lock();
    *state = SmpLockContentionState::new();
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn reset_smp_lock_diagnostic_state() {
    for logical_cpu in 0..MAX_CORES {
        record_smp_lock_diagnostic(logical_cpu, SmpLockDiagnosticPhase::Idle, 0, 0, 0, 0, 0);
    }
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn record_smp_lock_diagnostic(
    logical_cpu: usize,
    phase: SmpLockDiagnosticPhase,
    progress: u64,
    attempts: u64,
    timeouts: u64,
    releases: u64,
    sctlr_el2: u64,
) {
    if logical_cpu >= MAX_CORES {
        return;
    }
    SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu].store(progress, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu].store(attempts, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu].store(timeouts, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu].store(releases, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu].store(sctlr_el2, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu].store(phase.raw(), Ordering::Release);
    clean_smp_lock_diagnostic(logical_cpu);
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn clean_smp_lock_diagnostic(logical_cpu: usize) {
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu]);
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn snapshot_smp_lock_diagnostic(logical_cpu: usize) -> SmpLockDiagnosticSnapshot {
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu]);
    SmpLockDiagnosticSnapshot {
        phase: SmpLockDiagnosticPhase::from_raw(
            SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu].load(Ordering::Acquire),
        ),
        progress: SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu].load(Ordering::Acquire),
        attempts: SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu].load(Ordering::Acquire),
        timeouts: SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu].load(Ordering::Acquire),
        releases: SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu].load(Ordering::Acquire),
        sctlr_el2: SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu].load(Ordering::Acquire),
    }
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn write_smp_lock_wait_observation(logical_cpu: usize, remaining: usize) {
    SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
    let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
    let diagnostic = snapshot_smp_lock_diagnostic(logical_cpu);
    crate::println!(
        "rpi5-smp-lock-cache-coherence: wait logical={} remaining={} state={} progress={} diag-phase={} diag-progress={} diag-attempts={} diag-timeouts={} diag-releases={} diag-sctlr-el2={:#018x} diag-cacheable-mmu={}",
        logical_cpu,
        remaining,
        secondary_state_name(report.lifecycle.raw()),
        report.workload_progress,
        diagnostic.phase.name(),
        diagnostic.progress,
        diagnostic.attempts,
        diagnostic.timeouts,
        diagnostic.releases,
        diagnostic.sctlr_el2,
        cacheable_mmu_enabled(diagnostic.sctlr_el2)
    );
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn current_sctlr_el2() -> u64 {
    let sctlr: u64;
    unsafe {
        core::arch::asm!("mrs {sctlr}, SCTLR_EL2", sctlr = out(reg) sctlr, options(nostack, preserves_flags));
    }
    sctlr
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn cacheable_mmu_enabled(sctlr: u64) -> bool {
    (sctlr & (RPI5_SCTLR_M_ENABLE | RPI5_SCTLR_C_ENABLE))
        == (RPI5_SCTLR_M_ENABLE | RPI5_SCTLR_C_ENABLE)
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn clean_cache_line_to_poc<T>(value: &T) {
    unsafe {
        core::arch::asm!(
            "dc cvac, {addr}",
            "dsb sy",
            addr = in(reg) value as *const T as usize,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn invalidate_cache_line_from_poc<T>(value: &T) {
    unsafe {
        core::arch::asm!(
            "dc ivac, {addr}",
            "dsb sy",
            addr = in(reg) value as *const T as usize,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
fn run_smp_lock_contention_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();
    let sctlr_el2 = current_sctlr_el2();
    let mut attempts = 0;
    let mut timeouts = 0;
    let mut releases = 0;
    record_smp_lock_diagnostic(
        logical_cpu,
        SmpLockDiagnosticPhase::SecondaryEntered,
        0,
        attempts,
        timeouts,
        releases,
        sctlr_el2,
    );

    let mut progress = 0;
    while progress < RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE {
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::BeforeLockAttempt,
            progress,
            attempts,
            timeouts,
            releases,
            sctlr_el2,
        );
        let mut waited = 0;
        let expected_after = {
            let mut state = loop {
                attempts += 1;
                if let Some(state) = SMP_LOCK_CONTENTION_STATE.try_lock() {
                    break state;
                }
                waited += 1;
                if waited >= RPI5_SMP_LOCK_ACQUIRE_SPIN_LIMIT {
                    timeouts += 1;
                    record_smp_lock_diagnostic(
                        logical_cpu,
                        SmpLockDiagnosticPhase::LockAcquireTimeout,
                        progress,
                        attempts,
                        timeouts,
                        releases,
                        sctlr_el2,
                    );
                    core_state.record_workload_progress(progress);
                    core_state.mark_workload_complete(progress);
                    core_state.clean_to_poc();
                    return;
                }
                if waited & 0xffff == 0 {
                    record_smp_lock_diagnostic(
                        logical_cpu,
                        SmpLockDiagnosticPhase::WaitingForLock,
                        progress,
                        attempts,
                        timeouts,
                        releases,
                        sctlr_el2,
                    );
                }
                core::hint::spin_loop();
            };
            record_smp_lock_diagnostic(
                logical_cpu,
                SmpLockDiagnosticPhase::LockAcquired,
                progress,
                attempts,
                timeouts,
                releases,
                sctlr_el2,
            );
            let before = state.shared_counter;
            state.shared_counter = before + 1;
            state.per_core_counts[logical_cpu] += 1;
            if state.shared_counter != before + 1 {
                state.error_count += 1;
            }
            state.per_core_counts[logical_cpu]
        };
        releases += 1;
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::LockReleased,
            progress,
            attempts,
            timeouts,
            releases,
            sctlr_el2,
        );
        progress += 1;
        if expected_after != progress {
            let mut state = SMP_LOCK_CONTENTION_STATE.lock();
            state.error_count += 1;
        }
        core_state.record_workload_progress(progress);
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::IterationComplete,
            progress,
            attempts,
            timeouts,
            releases,
            sctlr_el2,
        );
        if progress == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE || progress & 0xf == 0 {
            core_state.clean_to_poc();
        }
        smp_full_barrier();
        core::hint::spin_loop();
    }

    core_state.mark_workload_complete(progress);
    core_state.clean_to_poc();
    record_smp_lock_diagnostic(
        logical_cpu,
        SmpLockDiagnosticPhase::WorkloadComplete,
        progress,
        attempts,
        timeouts,
        releases,
        sctlr_el2,
    );
}

#[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
pub fn run_smp_lock_cache_coherence_proof() -> bool {
    let boot_mpidr = crate::arch::aarch64::mpidr_el1();
    let boot_affinity = crate::arch::aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_total = RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE * (MAX_CORES as u64 - 1);

    crate::println!(
        "rpi5-smp-lock-cache-coherence: start conduit=smc cores={} target-per-core={} expected-total={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x}) cache-policy=generic-lock-no-cache-maintenance acquire-spin-limit={}",
        MAX_CORES,
        RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE,
        expected_total,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end,
        RPI5_SMP_LOCK_ACQUIRE_SPIN_LIMIT
    );
    wait_uart10_empty_early_phase();

    smp::reset_secondary_core_states();
    reset_smp_lock_contention_state();
    reset_smp_lock_diagnostic_state();
    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-smp-lock-cache-coherence: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-smp-lock-cache-coherence: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-smp-lock-cache-coherence: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-smp-lock-cache-coherence: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        if remaining == RPI5_SECONDARY_WAIT_LIMIT
            || remaining % RPI5_SMP_LOCK_WAIT_POLL_INTERVAL == 0
        {
            for logical_cpu in 1..MAX_CORES {
                write_smp_lock_wait_observation(logical_cpu, remaining);
            }
            wait_uart10_empty_early_phase();
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let final_state = SMP_LOCK_CONTENTION_STATE.try_lock().map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SmpLockContentionState::new);
    let mut participants = 0;
    let mut diagnostic_participants = 0;
    let mut any_pre_lock_stall = false;
    let mut any_lock_acquire_timeout = false;
    let mut any_lock_held_stall = !lock_available;
    let mut all_diagnostic_progress_complete = true;
    let mut any_mixed_cache_mmu = false;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && lock_available
        && final_state.shared_counter == expected_total
        && final_state.error_count == 0;

    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let diagnostic = snapshot_smp_lock_diagnostic(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let locked_count = final_state.per_core_counts[logical_cpu];
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE
            && locked_count == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE;
        if locked_count == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE {
            participants += 1;
        }
        if diagnostic.progress == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE {
            diagnostic_participants += 1;
        }
        any_pre_lock_stall |= report.lifecycle >= CoreLifecycle::WorkloadRunning
            && diagnostic.phase < SmpLockDiagnosticPhase::BeforeLockAttempt;
        any_lock_acquire_timeout |= diagnostic.timeouts > 0
            || diagnostic.phase == SmpLockDiagnosticPhase::LockAcquireTimeout;
        any_lock_held_stall &= diagnostic.phase == SmpLockDiagnosticPhase::LockAcquired;
        all_diagnostic_progress_complete &= diagnostic.progress
            == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE
            && diagnostic.releases == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE;
        any_mixed_cache_mmu |=
            cacheable_mmu_enabled(boot_sctlr_el2) && !cacheable_mmu_enabled(diagnostic.sctlr_el2);
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-smp-lock-cache-coherence: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) lock-count={} progress={} target={} diag-phase={} diag-progress={} diag-attempts={} diag-timeouts={} diag-releases={} diag-sctlr-el2={:#018x} diag-cacheable-mmu={} ok={}",
            logical_cpu,
            secondary_state_name(report.lifecycle.raw()),
            report.context,
            report.mpidr,
            report.affinity,
            logical_from_mpidr,
            report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
            locked_count,
            report.workload_progress,
            RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE,
            diagnostic.phase.name(),
            diagnostic.progress,
            diagnostic.attempts,
            diagnostic.timeouts,
            diagnostic.releases,
            diagnostic.sctlr_el2,
            cacheable_mmu_enabled(diagnostic.sctlr_el2),
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let generic_state_visible = lock_available
        && final_state.shared_counter == expected_total
        && final_state.per_core_counts[1..]
            .iter()
            .all(|count| *count == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE);
    let non_visible_progress = all_diagnostic_progress_complete && !generic_state_visible;
    let classification = if reports_ok {
        "pi5-smp-lock-cache-coherence-complete"
    } else if any_mixed_cache_mmu {
        "pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime"
    } else if any_pre_lock_stall {
        "pi5-smp-lock-cache-coherence-pre-lock-stall"
    } else if any_lock_held_stall {
        "pi5-smp-lock-cache-coherence-lock-held-stall"
    } else if non_visible_progress {
        "pi5-smp-lock-cache-coherence-non-visible-progress"
    } else if any_lock_acquire_timeout {
        "pi5-smp-lock-cache-coherence-lock-acquire-timeout"
    } else if !lock_available {
        "pi5-smp-lock-cache-coherence-lock-still-held"
    } else if !cpu_on_ok {
        "pi5-psci-smc-cpu-on-failed"
    } else if boot_logical != Some(0) {
        "pi5-psci-boot-core-identity-mismatch"
    } else {
        "pi5-smp-lock-cache-coherence-invariant-failed"
    };
    crate::println!(
        "rpi5-smp-lock-cache-coherence: final counter={} expected={} participants={} diag-participants={} errors={} lock-available={} generic-state-visible={} mixed-cache-mmu={} non-visible-progress={} wait-remaining={} classification={}",
        final_state.shared_counter,
        expected_total,
        participants,
        diagnostic_participants,
        final_state.error_count,
        lock_available,
        generic_state_visible,
        any_mixed_cache_mmu,
        non_visible_progress,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-smp-lock-cache-coherence: PASS");
    } else {
        crate::println!("rpi5-smp-lock-cache-coherence: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_rpi5_psci_secondary_core_alive_proof)]
pub fn run_psci_secondary_core_alive_proof() -> bool {
    smp::reset_secondary_core_states();

    let boot_mpidr = crate::arch::aarch64::mpidr_el1();
    let boot_affinity = crate::arch::aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-psci-secondary-core-alive: start conduit=smc cores={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-psci-secondary-core-alive: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-psci-secondary-core-alive: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_ready = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::HandoffReady
        });
        if all_ready {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let mut reports_ok = cpu_on_ok && boot_logical == Some(0);
    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let report_ok = report.lifecycle >= CoreLifecycle::HandoffReady
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned;
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-psci-secondary-core-alive: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) ok={}",
            logical_cpu,
            secondary_state_name(report.lifecycle.raw()),
            report.context,
            report.mpidr,
            report.affinity,
            logical_from_mpidr,
            report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-psci-smc-secondary-cores-alive"
    } else if !cpu_on_ok {
        "pi5-psci-smc-cpu-on-failed"
    } else if boot_logical != Some(0) {
        "pi5-psci-boot-core-identity-mismatch"
    } else {
        "pi5-psci-started-but-state-or-stack-incomplete"
    };
    crate::println!(
        "rpi5-psci-secondary-core-alive: wait-remaining={} classification={}",
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-psci-secondary-core-alive: PASS");
    } else {
        crate::println!("rpi5-psci-secondary-core-alive: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_rpi5_secondary_core_workload_proof)]
pub fn run_secondary_core_workload_proof() -> bool {
    smp::reset_secondary_core_states();

    let boot_mpidr = crate::arch::aarch64::mpidr_el1();
    let boot_affinity = crate::arch::aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-secondary-core-workload: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x}) scheduler=single-core-deferred",
        MAX_CORES,
        SECONDARY_CORE_WORKLOAD_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-secondary-core-workload: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-secondary-core-workload: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let mut reports_ok = cpu_on_ok && boot_logical == Some(0);
    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == SECONDARY_CORE_WORKLOAD_TARGET;
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-secondary-core-workload: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) progress={} target={} ok={}",
            logical_cpu,
            secondary_state_name(report.lifecycle.raw()),
            report.context,
            report.mpidr,
            report.affinity,
            logical_from_mpidr,
            report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
            report.workload_progress,
            SECONDARY_CORE_WORKLOAD_TARGET,
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-secondary-core-controlled-workload-complete"
    } else if !cpu_on_ok {
        "pi5-psci-smc-cpu-on-failed"
    } else if boot_logical != Some(0) {
        "pi5-psci-boot-core-identity-mismatch"
    } else {
        "pi5-secondary-core-workload-incomplete"
    };
    crate::println!(
        "rpi5-secondary-core-workload: wait-remaining={} classification={}",
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-secondary-core-workload: PASS");
    } else {
        crate::println!("rpi5-secondary-core-workload: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_rpi5_uart10_polling_rx_diagnostic)]
pub fn run_uart10_polling_tty_rx_diagnostic() -> bool {
    crate::println!(
        "rpi5-uart10-rx-diagnostic: ready capacity={} wait-limit={} backend=runtime-console0/bcm2712-uart10-pl011 inject-hex=61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d",
        crate::tty::CANONICAL_LINE_CAPACITY,
        UART10_RX_WAIT_LIMIT
    );
    wait_uart10_empty_early_phase();

    let result =
        crate::tty::run_polling_rx_diagnostic_with_limit(firmware_console(), UART10_RX_WAIT_LIMIT);
    crate::println!();
    crate::println!(
        "rpi5-uart10-rx-diagnostic: raw-len={} line-len={} terminated={} timeout={} outcome={} truncated={} backspaces={} deletes={} controls={}",
        result.raw_bytes(),
        result.line().len(),
        result.terminated(),
        result.timed_out(),
        result.outcome_name(),
        result.truncated(),
        result.backspaces(),
        result.deletes(),
        result.controls().len()
    );
    crate::print!("rpi5-uart10-rx-diagnostic: line-hex=");
    print_tty_hex_bytes(result.line());
    crate::println!();
    crate::print!("rpi5-uart10-rx-diagnostic: echo-hex=");
    print_tty_hex_bytes(result.echo());
    crate::println!();
    crate::print!("rpi5-uart10-rx-diagnostic: control-events=");
    print_tty_control_events(result.controls());
    crate::println!();

    let passed = result.passed() && result.truncated() && !result.controls().is_empty();
    if passed {
        crate::println!("rpi5-uart10-rx-diagnostic: PASS");
    } else {
        crate::println!("rpi5-uart10-rx-diagnostic: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_rpi5_diagnostic_command_channel_proof)]
pub fn run_diagnostic_command_channel_proof() -> bool {
    crate::println!(
        "rpi5-diagnostic-command-channel-proof: start command-count=4 backend=runtime-console0/bcm2712-uart10-pl011 input=tty-canonical-lite"
    );
    wait_uart10_empty_early_phase();

    let mut passed = true;

    for command_index in 0..4 {
        crate::println!(
            "rpi5-diagnostic-command-channel-proof: ready command={}",
            command_index
        );
        wait_uart10_empty_early_phase();

        let result = crate::tty::run_polling_rx_diagnostic_with_limit(
            firmware_console(),
            UART10_RX_WAIT_LIMIT,
        );
        settle_for_serial_capture();
        crate::println!();
        crate::print!(
            "rpi5-diagnostic-command-channel-proof: line command={} hex=",
            command_index
        );
        print_tty_hex_bytes(result.line());
        crate::println!();

        if !result.passed() || result.truncated() || !result.controls().is_empty() {
            crate::println!(
                "rpi5-diagnostic-command-channel-proof: input-fail command={} outcome={} truncated={} controls={}",
                command_index,
                result.outcome_name(),
                result.truncated(),
                result.controls().len()
            );
            passed = false;
            continue;
        }

        let mut sink = crate::runtime_console::RuntimeConsole::new(firmware_console());
        let dispatch = crate::diagnostic_command::dispatch_default_diagnostic_command(
            result.line(),
            &mut sink,
        );
        let dispatch = match dispatch {
            Ok(dispatch) => dispatch,
            Err(_) => {
                crate::println!(
                    "rpi5-diagnostic-command-channel-proof: dispatch-fail command={} response-write-failed",
                    command_index
                );
                passed = false;
                continue;
            }
        };

        let status_name = diagnostic_dispatch_status_name(dispatch.status);
        crate::println!(
            "rpi5-diagnostic-command-channel-proof: dispatch command={} status={} responses={}",
            command_index,
            status_name,
            dispatch.response_lines
        );

        if !expected_diagnostic_dispatch(
            command_index,
            result.line(),
            dispatch.status,
            dispatch.response_lines,
        ) {
            passed = false;
        }
        wait_uart10_empty_early_phase();
    }

    if passed {
        crate::println!("rpi5-diagnostic-command-channel-proof: PASS");
    } else {
        crate::println!("rpi5-diagnostic-command-channel-proof: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_rpi5_diagnostic_command_channel_proof)]
fn settle_for_serial_capture() {
    for _ in 0..DIAGNOSTIC_COMMAND_CAPTURE_SETTLE_SPINS {
        core::hint::spin_loop();
    }
}

#[cfg(talos_rpi5_diagnostic_command_channel_proof)]
fn diagnostic_dispatch_status_name(
    status: crate::diagnostic_command::DiagnosticDispatchStatus,
) -> &'static str {
    match status {
        crate::diagnostic_command::DiagnosticDispatchStatus::Handled => "handled",
        crate::diagnostic_command::DiagnosticDispatchStatus::UnknownCommand => "unknown-command",
        crate::diagnostic_command::DiagnosticDispatchStatus::UnexpectedArgument => {
            "unexpected-argument"
        }
        crate::diagnostic_command::DiagnosticDispatchStatus::ParseError(_) => "parse-error",
    }
}

#[cfg(talos_rpi5_diagnostic_command_channel_proof)]
fn expected_diagnostic_dispatch(
    command_index: usize,
    line: &[u8],
    status: crate::diagnostic_command::DiagnosticDispatchStatus,
    response_lines: usize,
) -> bool {
    use crate::diagnostic_command::DiagnosticDispatchStatus::{Handled, UnknownCommand};

    match command_index {
        0 => line == b"help" && status == Handled && response_lines == 2,
        1 => line == b"list" && status == Handled && response_lines == 2,
        2 => line == b"bogus" && status == UnknownCommand && response_lines == 1,
        3 => line == b"status" && status == Handled && response_lines == 6,
        _ => false,
    }
}

#[cfg(any(
    talos_rpi5_uart10_polling_rx_diagnostic,
    talos_rpi5_diagnostic_command_channel_proof
))]
fn print_tty_hex_bytes(bytes: &[u8]) {
    for (index, byte) in bytes.iter().enumerate() {
        if index != 0 {
            crate::print!(" ");
        }
        crate::print!("{:02x}", byte);
    }
}

#[cfg(talos_rpi5_uart10_polling_rx_diagnostic)]
fn print_tty_control_events(events: &[Option<crate::tty::TtyControlEvent>]) {
    if events.is_empty() {
        crate::print!("none");
        return;
    }

    for (index, event) in events.iter().enumerate() {
        if index != 0 {
            crate::print!(",");
        }
        match event {
            Some(event) => {
                crate::print!("{}", event.name());
            }
            None => {
                crate::print!("empty");
            }
        }
    }
}

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
#[repr(align(16))]
struct KernelThreadStack([u8; CONTEXT_SWITCH_STACK_SIZE]);

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
impl KernelThreadStack {
    const fn new() -> Self {
        Self([0; CONTEXT_SWITCH_STACK_SIZE])
    }

    fn top(&self) -> usize {
        self.0.as_ptr() as usize + self.0.len()
    }
}

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
struct TimerPreemptionSmokeState {
    main_context: ContextFrame,
    worker_contexts: [ContextFrame; 2],
    worker_stacks: [KernelThreadStack; 2],
    tasks: [Option<Task>; 2],
    scheduler: SingleCoreScheduler<2>,
    progress: [u64; 2],
    handled_requests: u64,
    current_task: u64,
    runnable_task: u64,
    preempted_task: u64,
}

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
impl TimerPreemptionSmokeState {
    const fn new() -> Self {
        Self {
            main_context: ContextFrame::new(0, 0),
            worker_contexts: [ContextFrame::new(0, 0); 2],
            worker_stacks: [KernelThreadStack::new(), KernelThreadStack::new()],
            tasks: [None, None],
            scheduler: SingleCoreScheduler::new(),
            progress: [0; 2],
            handled_requests: 0,
            current_task: 0,
            runnable_task: 0,
            preempted_task: 0,
        }
    }

    fn reset(&mut self) {
        self.progress = [0; 2];
        self.handled_requests = 0;
        self.current_task = 1;
        self.runnable_task = 2;
        self.preempted_task = 0;
        self.scheduler = SingleCoreScheduler::new();

        self.worker_contexts[0] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[0].top(),
            aarch64::kernel_thread_trampoline_address(),
            rpi5_timer_preemption_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            rpi5_timer_preemption_thread as *const () as usize,
            1,
        );

        let task1_id = TaskId::new(1).expect("nonzero task id");
        let task2_id = TaskId::new(2).expect("nonzero task id");
        let stack1 = KernelStack::new(
            self.worker_stacks[0].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid Pi 5 timer-preemption task 1 stack");
        let stack2 = KernelStack::new(
            self.worker_stacks[1].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid Pi 5 timer-preemption task 2 stack");
        let mut task1 = Task::kernel_thread(task1_id, stack1, self.worker_contexts[0]);
        let mut task2 = Task::kernel_thread(task2_id, stack2, self.worker_contexts[1]);
        task1.set_state(TaskState::Running);
        self.scheduler
            .make_runnable(&mut task2)
            .expect("Pi 5 timer-preemption smoke has runnable capacity");
        self.tasks = [Some(task1), Some(task2)];
    }

    fn proof_complete(&self) -> bool {
        let counters = self.scheduler.counters();
        self.progress[0] >= TIMER_PREEMPTION_TARGET_PROGRESS
            && self.progress[1] >= TIMER_PREEMPTION_TARGET_PROGRESS
            && counters.timer_preemptions() >= TIMER_PREEMPTION_TARGET_SWITCHES
    }

    fn dispatch_timer_preemption_from(&mut self, task_index: usize, request_count: u64) -> usize {
        let current = self.tasks[task_index]
            .as_mut()
            .expect("current Pi 5 timer-preemption task exists");
        let preempted_task = current.id();
        let next_task = self
            .scheduler
            .timer_preempt(current)
            .expect("Pi 5 timer-preemption smoke has a runnable peer");
        let next_task_index = (next_task.raw() - 1) as usize;
        self.tasks[next_task_index]
            .as_mut()
            .expect("next Pi 5 timer-preemption task exists")
            .set_state(TaskState::Running);
        self.handled_requests = request_count;
        self.current_task = next_task.raw();
        self.runnable_task = self
            .scheduler
            .runnable()
            .front()
            .map_or(0, |task_id| task_id.raw());
        self.preempted_task = preempted_task.raw();
        next_task_index
    }
}

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
struct TimerPreemptionSmokeCell(UnsafeCell<TimerPreemptionSmokeState>);

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
unsafe impl Sync for TimerPreemptionSmokeCell {}

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
impl TimerPreemptionSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(TimerPreemptionSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut TimerPreemptionSmokeState {
        self.0.get()
    }
}

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
static TIMER_PREEMPTION_SMOKE: TimerPreemptionSmokeCell = TimerPreemptionSmokeCell::new();

#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerIrqSnapshot {
    pub timer_count: u64,
    pub last_vector: u64,
    pub last_iar: u64,
    pub last_intid: u64,
    pub unexpected_gic_count: u64,
}

#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
pub fn timer_irq_snapshot() -> TimerIrqSnapshot {
    TimerIrqSnapshot {
        timer_count: generic_timer::monotonic_ticks(),
        last_vector: LAST_IRQ_VECTOR.load(Ordering::Relaxed),
        last_iar: LAST_IAR.load(Ordering::Relaxed),
        last_intid: LAST_INTID.load(Ordering::Relaxed),
        unexpected_gic_count: UNEXPECTED_GIC_IRQ_COUNT.load(Ordering::Relaxed),
    }
}

#[cfg(any(
    talos_rpi5_timer_irq_diagnostic,
    talos_rpi5_timer_preemption_diagnostic
))]
pub fn handle_irq(vector: u64) -> bool {
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let iar = unsafe { gic.acknowledge() };
    let intid = iar & 0x03ff;

    LAST_IRQ_VECTOR.store(vector, Ordering::Relaxed);
    LAST_IAR.store(iar as u64, Ordering::Relaxed);
    LAST_INTID.store(intid as u64, Ordering::Relaxed);

    if intid == EL2_PHYSICAL_TIMER_INTID {
        unsafe { generic_timer::record_el2_physical_tick_and_rearm() };
        #[cfg(talos_rpi5_timer_preemption_diagnostic)]
        TIMER_PREEMPTION_REQUESTS.fetch_add(1, Ordering::Relaxed);
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

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
extern "C" fn rpi5_timer_preemption_thread(raw_task_index: usize) -> ! {
    let task_index = raw_task_index & 1;
    loop {
        unsafe {
            let state = TIMER_PREEMPTION_SMOKE.get();
            (*state).current_task = task_index as u64 + 1;

            if (*state).proof_complete() {
                (*state).runnable_task = (*state)
                    .scheduler
                    .runnable()
                    .front()
                    .map_or(0, |task_id| task_id.raw());
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).main_context),
                );
            }

            let request_count = TIMER_PREEMPTION_REQUESTS.load(Ordering::Relaxed);
            if request_count != (*state).handled_requests {
                (*state).progress[task_index] += 1;
                let irq_state = aarch64::single_core_irq_mask_save();
                let next_task_index =
                    (*state).dispatch_timer_preemption_from(task_index, request_count);
                aarch64::single_core_irq_restore(irq_state);
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).worker_contexts[next_task_index]),
                );
            }
        }

        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[cfg(talos_rpi5_timer_preemption_diagnostic)]
pub fn run_el2_timer_preemption_smoke() -> bool {
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
        GicV2::new(GICD_BASE, GICC_BASE).enable_ppi_or_spi(EL2_PHYSICAL_TIMER_INTID);
    }
    LAST_IRQ_VECTOR.store(0, Ordering::Relaxed);
    LAST_IAR.store(0, Ordering::Relaxed);
    LAST_INTID.store(0, Ordering::Relaxed);
    UNEXPECTED_GIC_IRQ_COUNT.store(0, Ordering::Relaxed);
    TIMER_PREEMPTION_REQUESTS.store(0, Ordering::Relaxed);
    generic_timer::reset_monotonic_ticks();

    let freq = generic_timer::counter_frequency_hz();
    let start = generic_timer::physical_count();
    let delta = generic_timer::periodic_tick_delta_ticks(freq);
    let compare = start.wrapping_add(delta);
    generic_timer::configure_periodic_tick_delta(delta);

    unsafe {
        let state = TIMER_PREEMPTION_SMOKE.get();
        (*state).reset();
        crate::println!(
            "rpi5-timer-preemption-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "rpi5-timer-preemption-smoke: gicd={:#014x} gicc={:#014x} intid={} cntfrq={} start={} cval={} delta={}",
            GICD_BASE,
            GICC_BASE,
            EL2_PHYSICAL_TIMER_INTID,
            freq,
            start,
            compare,
            delta
        );
        crate::println!(
            "rpi5-timer-preemption-smoke: start current={} runnable={} preempted={} requests={}",
            (*state).current_task,
            (*state).runnable_task,
            (*state).preempted_task,
            TIMER_PREEMPTION_REQUESTS.load(Ordering::Relaxed)
        );

        generic_timer::program_el2_physical_compare(compare);
        aarch64::enable_irq();
        aarch64::cooperative_context_switch(
            core::ptr::addr_of_mut!((*state).main_context),
            core::ptr::addr_of!((*state).worker_contexts[0]),
        );
    }

    unsafe {
        aarch64::disable_irq();
    }

    let (
        progress0,
        progress1,
        state_transitions,
        voluntary_yields,
        timer_preemptions,
        dispatch_switches,
        handled_requests,
        current_task,
        runnable_task,
        preempted_task,
    ) = unsafe {
        let state = TIMER_PREEMPTION_SMOKE.get();
        let counters = (*state).scheduler.counters();
        (
            (*state).progress[0],
            (*state).progress[1],
            counters.state_transitions(),
            counters.voluntary_yields(),
            counters.timer_preemptions(),
            counters.context_switches(),
            (*state).handled_requests,
            (*state).current_task,
            (*state).runnable_task,
            (*state).preempted_task,
        )
    };
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
        "rpi5-timer-preemption-smoke: progress task1={} task2={} ticks={} requests={} handled={} timer-preemptions={} dispatch-switches={} voluntary-yields={} transitions={} current={} runnable={} preempted={}",
        progress0,
        progress1,
        snapshot.timer_count,
        TIMER_PREEMPTION_REQUESTS.load(Ordering::Relaxed),
        handled_requests,
        timer_preemptions,
        dispatch_switches,
        voluntary_yields,
        state_transitions,
        current_task,
        runnable_task,
        preempted_task
    );
    crate::println!(
        "rpi5-timer-preemption-smoke: irq vector={} iar={:#010x} intid={} unexpected={} ctl={:#x} daif={:#x}",
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control,
        daif
    );
    crate::println!(
        "rpi5-timer-preemption-smoke: gic enable={:#010x} pending={:#010x} active={:#010x} hppir={:#010x}",
        enable_bits,
        pending_bits,
        active_bits,
        highest_pending
    );

    let passed = progress0 >= TIMER_PREEMPTION_TARGET_PROGRESS
        && progress1 >= TIMER_PREEMPTION_TARGET_PROGRESS
        && snapshot.timer_count >= TIMER_PREEMPTION_TARGET_SWITCHES
        && handled_requests >= TIMER_PREEMPTION_TARGET_SWITCHES
        && timer_preemptions >= TIMER_PREEMPTION_TARGET_SWITCHES
        && dispatch_switches == timer_preemptions
        && voluntary_yields == 0
        && snapshot.last_intid == EL2_PHYSICAL_TIMER_INTID as u64
        && snapshot.unexpected_gic_count == 0
        && current_task != 0
        && runnable_task != 0
        && preempted_task != 0;

    if passed {
        crate::println!("rpi5-timer-preemption-smoke: PASS");
    } else {
        crate::println!("rpi5-timer-preemption-smoke: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_rpi5_timer_irq_diagnostic)]
pub fn run_el2_timer_irq_smoke() -> bool {
    unsafe {
        crate::arch::aarch64::disable_irq();
        crate::arch::aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
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
        "rpi5-timer-irq-smoke: gicd={:#014x} gicc={:#014x} intid={}",
        GICD_BASE,
        GICC_BASE,
        EL2_PHYSICAL_TIMER_INTID
    );
    crate::println!(
        "rpi5-timer-irq-smoke: cntfrq={} start={} cval={} delta={} target-ticks={}",
        freq,
        start,
        compare,
        delta,
        target_ticks
    );

    let mut workload = 0x1234_5678_9abc_def0u64;
    unsafe {
        generic_timer::program_el2_physical_compare(compare);
        crate::arch::aarch64::enable_irq();
    }

    let mut remaining = TIMER_IRQ_WAIT_LIMIT;
    while timer_irq_snapshot().timer_count < target_ticks && remaining > 0 {
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
        "rpi5-timer-irq-smoke: tick-count={} target={} vector={} iar={:#010x} intid={} unexpected={} ctl={:#x}",
        snapshot.timer_count,
        target_ticks,
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
        && snapshot.timer_count >= target_ticks
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

#[cfg(all(
    talos_target_rpi5_bcm2712,
    any(
        talos_rpi5_psci_secondary_core_alive_proof,
        talos_rpi5_secondary_core_workload_proof,
        talos_rpi5_smp_lock_cache_coherence_proof
    )
))]
pub(crate) fn write_uart10_bytes_early_phase(bytes: &[u8]) {
    for &byte in bytes {
        write_uart10_byte_early_phase(byte);
    }
    wait_uart10_empty_early_phase();
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
