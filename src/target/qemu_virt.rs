#![cfg_attr(any(test, talos_target_rpi5_bcm2712), allow(dead_code))]

#[cfg(any(
    talos_qemu_context_switch_smoke,
    talos_qemu_scheduler_yield_smoke,
    talos_qemu_timer_preemption_smoke
))]
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(any(
    talos_qemu_context_switch_smoke,
    talos_qemu_scheduler_yield_smoke,
    talos_qemu_timer_preemption_smoke
))]
use crate::scheduler::ContextFrame;
#[cfg(any(talos_qemu_scheduler_yield_smoke, talos_qemu_timer_preemption_smoke))]
use crate::scheduler::{KernelStack, SingleCoreScheduler, Task, TaskId, TaskState};
#[cfg(not(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
)))]
use crate::smp::MAX_CORES;
#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
))]
use crate::smp::{
    self, CoreLifecycle, CoreStackLayout, MAX_CORES, SECONDARY_CORE_STATES,
    SECONDARY_CORE_WORKLOAD_TARGET, SECONDARY_KERNEL_STACK_SIZE,
};
#[cfg(talos_qemu_smp_lock_contention_smoke)]
use crate::smp_sync::{SpinLock, smp_full_barrier};
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
#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
))]
const QEMU_SECONDARY_WAIT_LIMIT: usize = 10_000_000;
#[cfg(talos_qemu_smp_lock_contention_smoke)]
const SMP_LOCK_CONTENTION_TARGET_PER_CORE: u64 = 64;
#[cfg(any(
    talos_qemu_context_switch_smoke,
    talos_qemu_scheduler_yield_smoke,
    talos_qemu_timer_preemption_smoke
))]
const CONTEXT_SWITCH_STACK_SIZE: usize = 4096;
#[cfg(talos_qemu_context_switch_smoke)]
const CONTEXT_SWITCH_TARGET_PROGRESS: u64 = 2;
#[cfg(talos_qemu_scheduler_yield_smoke)]
const SCHEDULER_YIELD_TARGET_PROGRESS: u64 = 3;
#[cfg(talos_qemu_timer_preemption_smoke)]
const TIMER_PREEMPTION_TARGET_PROGRESS: u64 = 3;
#[cfg(talos_qemu_timer_preemption_smoke)]
const TIMER_PREEMPTION_TARGET_SWITCHES: u64 = 6;

const MMIO_REGIONS: &[MmioRegion] = &[
    MmioRegion::new("qemu-virt-gicv2-distributor", GICD_BASE, 0x0001_0000),
    MmioRegion::new("qemu-virt-gicv2-cpu-interface", GICC_BASE, 0x0001_0000),
    MmioRegion::new("qemu-virt-pl011-uart0", PL011_BASE, 0x1000),
];

static LAST_IRQ_VECTOR: AtomicU64 = AtomicU64::new(0);
static LAST_IAR: AtomicU64 = AtomicU64::new(0);
static LAST_INTID: AtomicU64 = AtomicU64::new(0);
static UNEXPECTED_GIC_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_qemu_timer_preemption_smoke)]
static TIMER_PREEMPTION_REQUESTS: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
))]
unsafe extern "C" {
    fn talos_aarch64_qemu_secondary_entry();
    static talos_secondary_core_stacks: u8;
    static talos_secondary_core_stacks_end: u8;
}

#[cfg(any(
    talos_qemu_context_switch_smoke,
    talos_qemu_scheduler_yield_smoke,
    talos_qemu_timer_preemption_smoke
))]
#[repr(align(16))]
struct KernelThreadStack([u8; CONTEXT_SWITCH_STACK_SIZE]);

#[cfg(any(
    talos_qemu_context_switch_smoke,
    talos_qemu_scheduler_yield_smoke,
    talos_qemu_timer_preemption_smoke
))]
impl KernelThreadStack {
    const fn new() -> Self {
        Self([0; CONTEXT_SWITCH_STACK_SIZE])
    }

    fn top(&self) -> usize {
        self.0.as_ptr() as usize + self.0.len()
    }
}

#[cfg(talos_qemu_context_switch_smoke)]
struct ContextSwitchSmokeState {
    main_context: ContextFrame,
    worker_contexts: [ContextFrame; 2],
    worker_stacks: [KernelThreadStack; 2],
    progress: [u64; 2],
    switch_count: u64,
    current_task: u64,
    runnable_task: u64,
}

#[cfg(talos_qemu_context_switch_smoke)]
impl ContextSwitchSmokeState {
    const fn new() -> Self {
        Self {
            main_context: ContextFrame::new(0, 0),
            worker_contexts: [ContextFrame::new(0, 0); 2],
            worker_stacks: [KernelThreadStack::new(), KernelThreadStack::new()],
            progress: [0; 2],
            switch_count: 0,
            current_task: 0,
            runnable_task: 0,
        }
    }

    fn reset(&mut self) {
        self.progress = [0; 2];
        self.switch_count = 0;
        self.current_task = 0;
        self.runnable_task = 1;
        self.worker_contexts[0] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[0].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_context_switch_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_context_switch_thread as *const () as usize,
            1,
        );
    }

    fn all_workers_made_progress(&self) -> bool {
        self.progress[0] >= CONTEXT_SWITCH_TARGET_PROGRESS
            && self.progress[1] >= CONTEXT_SWITCH_TARGET_PROGRESS
    }
}

#[cfg(talos_qemu_context_switch_smoke)]
struct ContextSwitchSmokeCell(UnsafeCell<ContextSwitchSmokeState>);

#[cfg(talos_qemu_context_switch_smoke)]
unsafe impl Sync for ContextSwitchSmokeCell {}

#[cfg(talos_qemu_context_switch_smoke)]
impl ContextSwitchSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(ContextSwitchSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut ContextSwitchSmokeState {
        self.0.get()
    }
}

#[cfg(talos_qemu_context_switch_smoke)]
static CONTEXT_SWITCH_SMOKE: ContextSwitchSmokeCell = ContextSwitchSmokeCell::new();

#[cfg(talos_qemu_scheduler_yield_smoke)]
struct SchedulerYieldSmokeState {
    main_context: ContextFrame,
    worker_contexts: [ContextFrame; 2],
    worker_stacks: [KernelThreadStack; 2],
    tasks: [Option<Task>; 2],
    scheduler: SingleCoreScheduler<2>,
    progress: [u64; 2],
    current_task: u64,
    runnable_task: u64,
    yielded_task: u64,
}

#[cfg(talos_qemu_scheduler_yield_smoke)]
impl SchedulerYieldSmokeState {
    const fn new() -> Self {
        Self {
            main_context: ContextFrame::new(0, 0),
            worker_contexts: [ContextFrame::new(0, 0); 2],
            worker_stacks: [KernelThreadStack::new(), KernelThreadStack::new()],
            tasks: [None, None],
            scheduler: SingleCoreScheduler::new(),
            progress: [0; 2],
            current_task: 0,
            runnable_task: 0,
            yielded_task: 0,
        }
    }

    fn reset(&mut self) {
        self.progress = [0; 2];
        self.current_task = 1;
        self.runnable_task = 2;
        self.yielded_task = 0;
        self.scheduler = SingleCoreScheduler::new();

        self.worker_contexts[0] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[0].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_scheduler_yield_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_scheduler_yield_thread as *const () as usize,
            1,
        );

        let task1_id = TaskId::new(1).expect("nonzero task id");
        let task2_id = TaskId::new(2).expect("nonzero task id");
        let stack1 = KernelStack::new(
            self.worker_stacks[0].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid task 1 stack");
        let stack2 = KernelStack::new(
            self.worker_stacks[1].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid task 2 stack");
        let mut task1 = Task::kernel_thread(task1_id, stack1, self.worker_contexts[0]);
        let mut task2 = Task::kernel_thread(task2_id, stack2, self.worker_contexts[1]);
        task1.set_state(TaskState::Running);
        self.scheduler
            .make_runnable(&mut task2)
            .expect("scheduler-yield smoke has runnable capacity");
        self.tasks = [Some(task1), Some(task2)];
    }

    fn all_workers_made_progress(&self) -> bool {
        self.progress[0] >= SCHEDULER_YIELD_TARGET_PROGRESS
            && self.progress[1] >= SCHEDULER_YIELD_TARGET_PROGRESS
    }

    fn dispatch_voluntary_yield_from(&mut self, task_index: usize) -> usize {
        let current = self.tasks[task_index]
            .as_mut()
            .expect("current scheduler-yield task exists");
        let yielded_task = current.id();
        let next_task = self
            .scheduler
            .voluntary_yield(current)
            .expect("scheduler-yield smoke has a runnable peer");
        let next_task_index = (next_task.raw() - 1) as usize;
        self.tasks[next_task_index]
            .as_mut()
            .expect("next scheduler-yield task exists")
            .set_state(TaskState::Running);
        self.current_task = next_task.raw();
        self.runnable_task = self
            .scheduler
            .runnable()
            .front()
            .map_or(0, |task_id| task_id.raw());
        self.yielded_task = yielded_task.raw();
        next_task_index
    }
}

#[cfg(talos_qemu_scheduler_yield_smoke)]
struct SchedulerYieldSmokeCell(UnsafeCell<SchedulerYieldSmokeState>);

#[cfg(talos_qemu_scheduler_yield_smoke)]
unsafe impl Sync for SchedulerYieldSmokeCell {}

#[cfg(talos_qemu_scheduler_yield_smoke)]
impl SchedulerYieldSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(SchedulerYieldSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut SchedulerYieldSmokeState {
        self.0.get()
    }
}

#[cfg(talos_qemu_scheduler_yield_smoke)]
static SCHEDULER_YIELD_SMOKE: SchedulerYieldSmokeCell = SchedulerYieldSmokeCell::new();

#[cfg(talos_qemu_timer_preemption_smoke)]
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

#[cfg(talos_qemu_timer_preemption_smoke)]
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
            qemu_timer_preemption_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_timer_preemption_thread as *const () as usize,
            1,
        );

        let task1_id = TaskId::new(1).expect("nonzero task id");
        let task2_id = TaskId::new(2).expect("nonzero task id");
        let stack1 = KernelStack::new(
            self.worker_stacks[0].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid timer-preemption task 1 stack");
        let stack2 = KernelStack::new(
            self.worker_stacks[1].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid timer-preemption task 2 stack");
        let mut task1 = Task::kernel_thread(task1_id, stack1, self.worker_contexts[0]);
        let mut task2 = Task::kernel_thread(task2_id, stack2, self.worker_contexts[1]);
        task1.set_state(TaskState::Running);
        self.scheduler
            .make_runnable(&mut task2)
            .expect("timer-preemption smoke has runnable capacity");
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
            .expect("current timer-preemption task exists");
        let preempted_task = current.id();
        let next_task = self
            .scheduler
            .timer_preempt(current)
            .expect("timer-preemption smoke has a runnable peer");
        let next_task_index = (next_task.raw() - 1) as usize;
        self.tasks[next_task_index]
            .as_mut()
            .expect("next timer-preemption task exists")
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

#[cfg(talos_qemu_timer_preemption_smoke)]
struct TimerPreemptionSmokeCell(UnsafeCell<TimerPreemptionSmokeState>);

#[cfg(talos_qemu_timer_preemption_smoke)]
unsafe impl Sync for TimerPreemptionSmokeCell {}

#[cfg(talos_qemu_timer_preemption_smoke)]
impl TimerPreemptionSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(TimerPreemptionSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut TimerPreemptionSmokeState {
        self.0.get()
    }
}

#[cfg(talos_qemu_timer_preemption_smoke)]
static TIMER_PREEMPTION_SMOKE: TimerPreemptionSmokeCell = TimerPreemptionSmokeCell::new();

#[derive(Clone, Copy)]
struct SingleCoreIrqMaskProbe {
    nested_start_masked: bool,
    inner_restored_masked: bool,
    outer_restored_masked: bool,
    unmasked_start: bool,
    saved_unmasked_masked: bool,
    restored_unmasked: bool,
}

#[allow(dead_code)]
pub const fn qemu_logical_cpu_from_mpidr_affinity(affinity: u64) -> Option<usize> {
    if affinity < MAX_CORES as u64 {
        Some(affinity as usize)
    } else {
        None
    }
}

#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
))]
fn secondary_stack_layout() -> CoreStackLayout {
    let base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    CoreStackLayout::new(base, end, MAX_CORES, SECONDARY_KERNEL_STACK_SIZE)
        .expect("valid linked secondary-core stack layout")
}

#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
))]
fn secondary_state_name(state: u64) -> &'static str {
    CoreLifecycle::from_raw(state).map_or("unknown", CoreLifecycle::name)
}

#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
))]
unsafe fn psci_cpu_on_smc(target_affinity: u64, entry: usize, context: usize) -> i64 {
    let mut function_id = 0xc400_0003u64;
    unsafe {
        core::arch::asm!(
            "smc #0",
            inout("x0") function_id,
            in("x1") target_affinity,
            in("x2") entry as u64,
            in("x3") context as u64,
            options(nostack)
        );
    }
    function_id as i64
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

#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke
))]
#[unsafe(no_mangle)]
pub extern "C" fn talos_qemu_secondary_entry(context: usize) -> ! {
    let mpidr = aarch64::mpidr_el1();
    let affinity = aarch64::mpidr_affinity(mpidr);
    let logical_cpu = qemu_logical_cpu_from_mpidr_affinity(affinity).unwrap_or(context);
    if logical_cpu < MAX_CORES {
        let core_state = &SECONDARY_CORE_STATES[logical_cpu];
        core_state.enter(context, mpidr, affinity);

        let stack_pointer: u64;
        unsafe {
            core::arch::asm!("mov {stack_pointer}, sp", stack_pointer = out(reg) stack_pointer, options(nomem, nostack, preserves_flags));
        }
        core_state.mark_stack_ready(stack_pointer as usize);
        core_state.mark_registered();
        core_state.mark_handoff_ready();
        #[cfg(talos_qemu_secondary_core_workload_smoke)]
        smp::run_controlled_secondary_workload(core_state, SECONDARY_CORE_WORKLOAD_TARGET);
        #[cfg(talos_qemu_smp_lock_contention_smoke)]
        run_smp_lock_contention_secondary(core_state, logical_cpu);
    }

    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[cfg(talos_qemu_smp_lock_contention_smoke)]
#[derive(Clone, Copy)]
struct SmpLockContentionState {
    shared_counter: u64,
    per_core_counts: [u64; MAX_CORES],
    error_count: u64,
}

#[cfg(talos_qemu_smp_lock_contention_smoke)]
impl SmpLockContentionState {
    const fn new() -> Self {
        Self {
            shared_counter: 0,
            per_core_counts: [0; MAX_CORES],
            error_count: 0,
        }
    }
}

#[cfg(talos_qemu_smp_lock_contention_smoke)]
static SMP_LOCK_CONTENTION_STATE: SpinLock<SmpLockContentionState> =
    SpinLock::new(SmpLockContentionState::new());

#[cfg(talos_qemu_smp_lock_contention_smoke)]
fn reset_smp_lock_contention_state() {
    let mut state = SMP_LOCK_CONTENTION_STATE.lock();
    *state = SmpLockContentionState::new();
}

#[cfg(talos_qemu_smp_lock_contention_smoke)]
fn run_smp_lock_contention_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut progress = 0;
    while progress < SMP_LOCK_CONTENTION_TARGET_PER_CORE {
        let expected_after = {
            let mut state = SMP_LOCK_CONTENTION_STATE.lock();
            let before = state.shared_counter;
            state.shared_counter = before + 1;
            state.per_core_counts[logical_cpu] += 1;
            if state.shared_counter != before + 1 {
                state.error_count += 1;
            }
            state.per_core_counts[logical_cpu]
        };
        progress += 1;
        if expected_after != progress {
            let mut state = SMP_LOCK_CONTENTION_STATE.lock();
            state.error_count += 1;
        }
        core_state.record_workload_progress(progress);
        if progress == SMP_LOCK_CONTENTION_TARGET_PER_CORE || progress & 0xf == 0 {
            core_state.clean_to_poc();
        }
        smp_full_barrier();
        core::hint::spin_loop();
    }

    core_state.mark_workload_complete(progress);
    core_state.clean_to_poc();
}

#[cfg(talos_qemu_secondary_core_discriminator)]
pub fn run_secondary_core_discriminator() -> bool {
    smp::reset_secondary_core_states();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-secondary-core-discriminator: start conduit=smc cores={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-secondary-core-discriminator: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_ready = (1..MAX_CORES).all(|logical_cpu| {
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
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let report_ok = report.lifecycle >= CoreLifecycle::HandoffReady
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-secondary-core-discriminator: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) ok={}",
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
    }

    crate::println!(
        "qemu-secondary-core-discriminator: wait-remaining={} classification={}",
        remaining,
        if reports_ok {
            "qemu-psci-smc-secondary-cores-alive"
        } else if cpu_on_ok {
            "qemu-psci-smc-started-but-report-incomplete"
        } else {
            "qemu-psci-smc-cpu-on-failed"
        }
    );

    if reports_ok {
        crate::println!("qemu-secondary-core-discriminator: PASS");
    } else {
        crate::println!("qemu-secondary-core-discriminator: FAIL");
    }

    reports_ok
}

#[cfg(talos_qemu_secondary_core_workload_smoke)]
pub fn run_secondary_core_workload_smoke() -> bool {
    smp::reset_secondary_core_states();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-secondary-core-workload: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SECONDARY_CORE_WORKLOAD_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-secondary-core-workload: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
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
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == SECONDARY_CORE_WORKLOAD_TARGET;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-secondary-core-workload: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) progress={} target={} ok={}",
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
    }

    let classification = if reports_ok {
        "qemu-secondary-core-controlled-workload-complete"
    } else if cpu_on_ok {
        "qemu-secondary-core-workload-incomplete"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-secondary-core-workload: wait-remaining={} classification={}",
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-secondary-core-workload: PASS");
    } else {
        crate::println!("qemu-secondary-core-workload: FAIL");
    }

    reports_ok
}

#[cfg(talos_qemu_smp_lock_contention_smoke)]
pub fn run_smp_lock_contention_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_smp_lock_contention_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_total = SMP_LOCK_CONTENTION_TARGET_PER_CORE * (MAX_CORES as u64 - 1);

    crate::println!(
        "qemu-smp-lock-contention: start conduit=smc cores={} target-per-core={} expected-total={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SMP_LOCK_CONTENTION_TARGET_PER_CORE,
        expected_total,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-smp-lock-contention: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
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

    let final_state = SMP_LOCK_CONTENTION_STATE.try_lock().map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SmpLockContentionState::new);
    let mut participants = 0;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && lock_available
        && final_state.shared_counter == expected_total
        && final_state.error_count == 0;

    for logical_cpu in 1..MAX_CORES {
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let locked_count = final_state.per_core_counts[logical_cpu];
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == SMP_LOCK_CONTENTION_TARGET_PER_CORE
            && locked_count == SMP_LOCK_CONTENTION_TARGET_PER_CORE;
        if locked_count == SMP_LOCK_CONTENTION_TARGET_PER_CORE {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "qemu-smp-lock-contention: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) lock-count={} progress={} target={} ok={}",
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
            SMP_LOCK_CONTENTION_TARGET_PER_CORE,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-smp-lock-contention-complete"
    } else if !lock_available {
        "qemu-smp-lock-contention-lock-still-held"
    } else if cpu_on_ok {
        "qemu-smp-lock-contention-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-smp-lock-contention: final counter={} expected={} participants={} errors={} lock-available={} wait-remaining={} classification={}",
        final_state.shared_counter,
        expected_total,
        participants,
        final_state.error_count,
        lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-smp-lock-contention: PASS");
    } else {
        crate::println!("qemu-smp-lock-contention: FAIL");
    }

    reports_ok
}

#[cfg(talos_qemu_polling_tty_rx_diagnostic)]
pub fn run_polling_tty_rx_diagnostic() -> bool {
    crate::println!(
        "qemu-tty-rx-diagnostic: ready capacity={} wait-limit={} backend=runtime-console0/qemu-virt-pl011",
        crate::tty::CANONICAL_LINE_CAPACITY,
        crate::tty::POLLING_RX_WAIT_LIMIT
    );

    let result = crate::tty::run_polling_rx_diagnostic(console());
    crate::println!();
    crate::println!(
        "qemu-tty-rx-diagnostic: raw-len={} line-len={} terminated={} timeout={} outcome={} truncated={} backspaces={} deletes={} controls={}",
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
    crate::print!("qemu-tty-rx-diagnostic: line-hex=");
    print_hex_bytes(result.line());
    crate::println!();
    crate::print!("qemu-tty-rx-diagnostic: echo-hex=");
    print_hex_bytes(result.echo());
    crate::println!();
    crate::print!("qemu-tty-rx-diagnostic: control-events=");
    print_control_events(result.controls());
    crate::println!();

    if result.passed() && result.truncated() && !result.controls().is_empty() {
        crate::println!("qemu-tty-rx-diagnostic: PASS");
        true
    } else {
        crate::println!("qemu-tty-rx-diagnostic: FAIL");
        false
    }
}

#[cfg(talos_qemu_diagnostic_command_channel_smoke)]
pub fn run_diagnostic_command_channel_smoke() -> bool {
    crate::println!(
        "qemu-diagnostic-command-channel-smoke: start command-count=4 backend=runtime-console0/qemu-virt-pl011 input=tty-canonical-lite"
    );

    let mut passed = true;

    for command_index in 0..4 {
        crate::println!(
            "qemu-diagnostic-command-channel-smoke: ready command={}",
            command_index
        );

        let result = crate::tty::run_polling_rx_diagnostic(console());
        crate::println!();
        crate::print!(
            "qemu-diagnostic-command-channel-smoke: line command={} hex=",
            command_index
        );
        print_hex_bytes(result.line());
        crate::println!();

        if !result.passed() || result.truncated() || !result.controls().is_empty() {
            crate::println!(
                "qemu-diagnostic-command-channel-smoke: input-fail command={} outcome={} truncated={} controls={}",
                command_index,
                result.outcome_name(),
                result.truncated(),
                result.controls().len()
            );
            passed = false;
            continue;
        }

        let mut sink = crate::runtime_console::RuntimeConsole::new(console());
        let dispatch = crate::diagnostic_command::dispatch_default_diagnostic_command(
            result.line(),
            &mut sink,
        );
        let dispatch = match dispatch {
            Ok(dispatch) => dispatch,
            Err(_) => {
                crate::println!(
                    "qemu-diagnostic-command-channel-smoke: dispatch-fail command={} response-write-failed",
                    command_index
                );
                passed = false;
                continue;
            }
        };

        let status_name = diagnostic_dispatch_status_name(dispatch.status);
        crate::println!(
            "qemu-diagnostic-command-channel-smoke: dispatch command={} status={} responses={}",
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
    }

    if passed {
        crate::println!("qemu-diagnostic-command-channel-smoke: PASS");
    } else {
        crate::println!("qemu-diagnostic-command-channel-smoke: FAIL");
    }

    passed
}

#[cfg(talos_qemu_diagnostic_command_channel_smoke)]
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

#[cfg(talos_qemu_diagnostic_command_channel_smoke)]
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
    talos_qemu_polling_tty_rx_diagnostic,
    talos_qemu_diagnostic_command_channel_smoke
))]
fn print_hex_bytes(bytes: &[u8]) {
    for (index, byte) in bytes.iter().enumerate() {
        if index != 0 {
            crate::print!(" ");
        }
        crate::print!("{:02x}", byte);
    }
}

#[cfg(talos_qemu_polling_tty_rx_diagnostic)]
fn print_control_events(events: &[Option<crate::tty::TtyControlEvent>]) {
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
        #[cfg(talos_qemu_timer_preemption_smoke)]
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

#[cfg(talos_qemu_timer_preemption_smoke)]
extern "C" fn qemu_timer_preemption_thread(raw_task_index: usize) -> ! {
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

#[cfg(talos_qemu_timer_preemption_smoke)]
pub fn run_el2_timer_preemption_smoke() -> bool {
    let _keep_timer_smoke_reachable: fn() -> bool = run_el2_timer_irq_smoke;

    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
    }
    unsafe {
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
            "qemu-timer-preemption-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "qemu-timer-preemption-smoke: gicd={:#010x} gicc={:#010x} intid={} cntfrq={} start={} cval={} delta={}",
            GICD_BASE,
            GICC_BASE,
            EL2_PHYSICAL_TIMER_INTID,
            freq,
            start,
            compare,
            delta
        );
        crate::println!(
            "qemu-timer-preemption-smoke: start current={} runnable={} preempted={} requests={}",
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
    let daif = aarch64::daif();
    let control = generic_timer::el2_physical_control();

    crate::println!(
        "qemu-timer-preemption-smoke: progress task1={} task2={} ticks={} requests={} handled={} timer-preemptions={} dispatch-switches={} voluntary-yields={} transitions={} current={} runnable={} preempted={}",
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
        "qemu-timer-preemption-smoke: irq vector={} iar={:#010x} intid={} unexpected={} ctl={:#x} daif={:#x}",
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control,
        daif
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
        crate::println!("qemu-timer-preemption-smoke: PASS");
    } else {
        crate::println!("qemu-timer-preemption-smoke: FAIL");
    }

    passed
}

#[cfg(talos_qemu_scheduler_yield_smoke)]
extern "C" fn qemu_scheduler_yield_thread(raw_task_index: usize) -> ! {
    let task_index = raw_task_index & 1;
    loop {
        unsafe {
            let state = SCHEDULER_YIELD_SMOKE.get();
            (*state).current_task = task_index as u64 + 1;
            (*state).progress[task_index] += 1;

            if (*state).all_workers_made_progress() {
                (*state).runnable_task = (*state)
                    .scheduler
                    .runnable()
                    .front()
                    .map_or(0, |task_id| task_id.raw());
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).main_context),
                );
            } else {
                // This is the accepted single-core critical section: scheduler-owned
                // queue/current/yielded state is mutated with IRQs masked, and the
                // section performs no allocation, formatting, printing, or callbacks.
                let irq_state = aarch64::single_core_irq_mask_save();
                let next_task_index = (*state).dispatch_voluntary_yield_from(task_index);
                aarch64::single_core_irq_restore(irq_state);
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).worker_contexts[next_task_index]),
                );
            }
        }
    }
}

#[cfg(talos_qemu_scheduler_yield_smoke)]
pub fn run_el2_scheduler_yield_smoke() -> bool {
    let _keep_timer_smoke_reachable: fn() -> bool = run_el2_timer_irq_smoke;

    unsafe {
        aarch64::disable_irq();
    }

    unsafe {
        let state = SCHEDULER_YIELD_SMOKE.get();
        (*state).reset();
        crate::println!(
            "qemu-scheduler-yield-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "qemu-scheduler-yield-smoke: start current={} runnable={} yielded={}",
            (*state).current_task,
            (*state).runnable_task,
            (*state).yielded_task
        );

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
        dispatch_switches,
        current_task,
        runnable_task,
        yielded_task,
    ) = unsafe {
        let state = SCHEDULER_YIELD_SMOKE.get();
        let counters = (*state).scheduler.counters();
        (
            (*state).progress[0],
            (*state).progress[1],
            counters.state_transitions(),
            counters.voluntary_yields(),
            counters.context_switches(),
            (*state).current_task,
            (*state).runnable_task,
            (*state).yielded_task,
        )
    };

    crate::println!(
        "qemu-scheduler-yield-smoke: progress task1={} task2={} yields={} dispatch-switches={} transitions={} current={} runnable={} yielded={}",
        progress0,
        progress1,
        voluntary_yields,
        dispatch_switches,
        state_transitions,
        current_task,
        runnable_task,
        yielded_task
    );

    let passed = progress0 >= SCHEDULER_YIELD_TARGET_PROGRESS
        && progress1 >= SCHEDULER_YIELD_TARGET_PROGRESS
        && voluntary_yields >= 5
        && dispatch_switches == voluntary_yields
        && state_transitions >= voluntary_yields
        && current_task != 0
        && runnable_task != 0
        && yielded_task != 0;

    if passed {
        crate::println!("qemu-scheduler-yield-smoke: PASS");
    } else {
        crate::println!("qemu-scheduler-yield-smoke: FAIL");
    }

    passed
}

#[cfg(talos_qemu_context_switch_smoke)]
extern "C" fn qemu_context_switch_thread(raw_task_index: usize) -> ! {
    let task_index = raw_task_index & 1;
    loop {
        unsafe {
            let state = CONTEXT_SWITCH_SMOKE.get();
            (*state).current_task = task_index as u64 + 1;
            (*state).progress[task_index] += 1;

            if (*state).all_workers_made_progress() {
                (*state).runnable_task = 0;
                (*state).switch_count += 1;
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).main_context),
                );
            } else {
                let next_task_index = 1 - task_index;
                (*state).runnable_task = next_task_index as u64 + 1;
                (*state).switch_count += 1;
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).worker_contexts[next_task_index]),
                );
            }
        }
    }
}

#[cfg(talos_qemu_context_switch_smoke)]
pub fn run_el2_context_switch_smoke() -> bool {
    let _keep_timer_smoke_reachable: fn() -> bool = run_el2_timer_irq_smoke;

    unsafe {
        aarch64::disable_irq();
    }

    unsafe {
        let state = CONTEXT_SWITCH_SMOKE.get();
        (*state).reset();
        crate::println!(
            "qemu-context-switch-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "qemu-context-switch-smoke: start current={} runnable={}",
            (*state).current_task,
            (*state).runnable_task
        );

        (*state).current_task = 0;
        (*state).runnable_task = 1;
        (*state).switch_count += 1;
        aarch64::cooperative_context_switch(
            core::ptr::addr_of_mut!((*state).main_context),
            core::ptr::addr_of!((*state).worker_contexts[0]),
        );
    }

    unsafe {
        aarch64::disable_irq();
    }

    let (progress0, progress1, switch_count, current_task, runnable_task) = unsafe {
        let state = CONTEXT_SWITCH_SMOKE.get();
        (
            (*state).progress[0],
            (*state).progress[1],
            (*state).switch_count,
            (*state).current_task,
            (*state).runnable_task,
        )
    };

    crate::println!(
        "qemu-context-switch-smoke: progress task1={} task2={} switches={} current={} runnable={}",
        progress0,
        progress1,
        switch_count,
        current_task,
        runnable_task
    );

    let passed = progress0 >= CONTEXT_SWITCH_TARGET_PROGRESS
        && progress1 >= CONTEXT_SWITCH_TARGET_PROGRESS
        && switch_count >= 5
        && current_task != 0
        && runnable_task == 0;

    if passed {
        crate::println!("qemu-context-switch-smoke: PASS");
    } else {
        crate::println!("qemu-context-switch-smoke: FAIL");
    }

    passed
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

#[cfg(test)]
mod tests {
    use super::qemu_logical_cpu_from_mpidr_affinity;

    #[test_case]
    fn qemu_mpidr_affinity_maps_four_virt_cpus() {
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(0), Some(0));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(1), Some(1));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(2), Some(2));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(3), Some(3));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(0x100), None);
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
