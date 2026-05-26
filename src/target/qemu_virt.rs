#![cfg_attr(any(test, talos_target_rpi5_bcm2712), allow(dead_code))]

#[cfg(any(
    talos_qemu_context_switch_smoke,
    talos_qemu_scheduler_yield_smoke,
    talos_qemu_timer_preemption_smoke
))]
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(talos_qemu_remote_wake_to_local_runnable_smoke)]
use crate::scheduler::TargetWakeConsumptionError;
#[cfg(any(
    talos_qemu_context_switch_smoke,
    talos_qemu_scheduler_yield_smoke,
    talos_qemu_timer_preemption_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
use crate::scheduler::{ContextFrame, KernelStack, SingleCoreScheduler, Task, TaskId, TaskState};
#[cfg(any(
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
use crate::scheduler::{
    LogicalCpuId, PerCoreScheduler, PerCoreSchedulerAccessError, ProductionDispatchError,
    SchedulerCoreRole, SharedSchedulerMetadata, SharedSchedulerMetadataError,
    SharedSchedulerMetadataLock,
};
#[cfg(talos_qemu_remote_wakeup_request_smoke)]
use crate::scheduler::{RemoteWakePublishOutcome, RemoteWakeQueue};
#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
use crate::scheduler::{
    RemoteWakeQueue, SecondarySchedulerServiceLoop, SecondarySchedulerServiceLoopError,
};
#[cfg(not(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
)))]
use crate::smp::MAX_CORES;
#[cfg(talos_qemu_secondary_core_workload_smoke)]
use crate::smp::SECONDARY_CORE_WORKLOAD_TARGET;
#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
use crate::smp::{
    self, CoreLifecycle, CoreStackLayout, MAX_CORES, SECONDARY_CORE_STATES,
    SECONDARY_KERNEL_STACK_SIZE,
};
#[cfg(talos_qemu_smp_lock_contention_smoke)]
use crate::smp_sync::{SpinLock, smp_full_barrier};
#[cfg(any(
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
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
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
const QEMU_SECONDARY_WAIT_LIMIT: usize = 10_000_000;
#[cfg(any(
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke
))]
const QEMU_CROSS_CORE_IPI_SGI_INTID: u32 = 1;
#[cfg(talos_qemu_remote_wakeup_request_smoke)]
const REMOTE_WAKE_QUEUE_CAPACITY: usize = 4;
#[cfg(talos_qemu_smp_lock_contention_smoke)]
const SMP_LOCK_CONTENTION_TARGET_PER_CORE: u64 = 64;
#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
const PER_CORE_SCHEDULER_PROGRESS_TARGET: u64 = 4;
#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
const PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET: u64 = 3;
#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
const SHARED_SCHEDULER_METADATA_TASK_CAPACITY: usize = MAX_CORES;
#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
const SHARED_SCHEDULER_METADATA_WAIT_LIMIT: usize = 100_000_000;
#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
const SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY: usize = 1;
#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
const SECONDARY_SCHEDULER_SERVICE_LOOP_WAIT_LIMIT: usize = 100_000_000;
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
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
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
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
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
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
fn secondary_state_name(state: u64) -> &'static str {
    CoreLifecycle::from_raw(state).map_or("unknown", CoreLifecycle::name)
}

#[cfg(any(
    talos_qemu_secondary_core_discriminator,
    talos_qemu_secondary_core_workload_smoke,
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
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
    talos_qemu_smp_lock_contention_smoke,
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
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
        #[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
        run_per_core_scheduler_ownership_secondary(core_state, logical_cpu);
        #[cfg(talos_qemu_cross_core_ipi_delivery_smoke)]
        run_cross_core_ipi_delivery_secondary(core_state, logical_cpu);
        #[cfg(talos_qemu_remote_wakeup_request_smoke)]
        run_remote_wakeup_request_secondary(core_state, logical_cpu);
        #[cfg(talos_qemu_production_secondary_dispatch_smoke)]
        run_production_secondary_dispatch_secondary(core_state, logical_cpu);
        #[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
        run_shared_scheduler_metadata_secondary(core_state, logical_cpu);
        #[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
        run_secondary_scheduler_service_loop_secondary(core_state, logical_cpu);
    }

    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
#[derive(Clone, Copy)]
struct PerCoreSchedulerReport {
    owner: u64,
    role: SchedulerCoreRole,
    production_dispatch_enabled: bool,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    progress: u64,
    state_transitions: u64,
    dispatch_deferred: bool,
    errors: u64,
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
impl PerCoreSchedulerReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            production_dispatch_enabled: false,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            progress: 0,
            state_transitions: 0,
            dispatch_deferred: false,
            errors: 0,
        }
    }
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
#[derive(Clone, Copy)]
struct PerCoreSchedulerOwnershipState {
    reports: [PerCoreSchedulerReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
impl PerCoreSchedulerOwnershipState {
    const fn new() -> Self {
        Self {
            reports: [PerCoreSchedulerReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
static PER_CORE_SCHEDULER_OWNERSHIP_STATE: SpinLock<PerCoreSchedulerOwnershipState> =
    SpinLock::new(PerCoreSchedulerOwnershipState::new());

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
fn reset_per_core_scheduler_ownership_state() {
    let mut state = unsafe { PER_CORE_SCHEDULER_OWNERSHIP_STATE.lock_irqsave() };
    *state = PerCoreSchedulerOwnershipState::new();
}

#[cfg(any(
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
fn scheduler_role_name(role: SchedulerCoreRole) -> &'static str {
    match role {
        SchedulerCoreRole::BootCpuProduction => "boot-production",
        SchedulerCoreRole::SecondaryDeferred => "secondary-deferred",
        SchedulerCoreRole::SecondaryProductionDiagnostic => "secondary-production-diagnostic",
    }
}

#[cfg(any(
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
fn task_id(raw: u64) -> TaskId {
    TaskId::new(raw).expect("diagnostic task IDs are nonzero")
}

#[cfg(any(
    talos_qemu_per_core_scheduler_ownership_smoke,
    talos_qemu_production_secondary_dispatch_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
fn scheduler_task(logical_cpu: usize, progress: u64) -> Task {
    let raw_task_id = (logical_cpu as u64 + 1) * 100 + progress;
    let stack_base = 0x8000_0000 + logical_cpu * 0x10000 + progress as usize * 0x1000;
    let stack = KernelStack::new(stack_base, 0x1000).expect("diagnostic stack bounds are valid");
    let context = ContextFrame::new(stack.limit() & !0xf, 0x4000_0000 + raw_task_id as usize);
    Task::kernel_thread(task_id(raw_task_id), stack, context)
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
fn build_per_core_scheduler_report(
    logical_cpu: usize,
    scheduler: &mut PerCoreScheduler<2>,
) -> PerCoreSchedulerReport {
    let requester = LogicalCpuId::new(logical_cpu);
    let mut errors = 0;
    let dispatch_deferred = match scheduler.production_scheduler_mut(requester) {
        Ok(_) => {
            errors += 1;
            false
        }
        Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred { owner }) => {
            if owner != requester {
                errors += 1;
            }
            true
        }
        Err(_) => {
            errors += 1;
            false
        }
    };

    let mut progress = 0;
    if let Ok(local_scheduler) = scheduler.local_scheduler_mut(requester) {
        while progress < PER_CORE_SCHEDULER_PROGRESS_TARGET {
            progress += 1;
            let mut task = scheduler_task(logical_cpu, progress);
            if local_scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
                break;
            }
            if local_scheduler.pick_next() != Some(task.id()) {
                errors += 1;
                break;
            }
        }
    } else {
        errors += 1;
    }

    let local_scheduler = scheduler.scheduler();
    PerCoreSchedulerReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        progress,
        state_transitions: local_scheduler.counters().state_transitions(),
        dispatch_deferred,
        errors,
    }
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
fn build_boot_scheduler_report() -> PerCoreSchedulerReport {
    let mut scheduler = PerCoreScheduler::<2>::boot_cpu();
    let requester = LogicalCpuId::BOOT;
    let mut errors = 0;
    if scheduler.set_current_task(requester, task_id(1)).is_err() {
        errors += 1;
    }

    let mut progress = 0;
    if let Ok(local_scheduler) = scheduler.production_scheduler_mut(requester) {
        while progress < PER_CORE_SCHEDULER_PROGRESS_TARGET {
            progress += 1;
            let mut task = scheduler_task(0, progress);
            if local_scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
                break;
            }
            if local_scheduler.pick_next() != Some(task.id()) {
                errors += 1;
                break;
            }
        }
    } else {
        errors += 1;
    }

    let local_scheduler = scheduler.scheduler();
    PerCoreSchedulerReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        progress,
        state_transitions: local_scheduler.counters().state_transitions(),
        dispatch_deferred: false,
        errors,
    }
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
fn publish_per_core_scheduler_report(logical_cpu: usize, report: PerCoreSchedulerReport) {
    let mut state = PER_CORE_SCHEDULER_OWNERSHIP_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress;
}

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
fn run_per_core_scheduler_ownership_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(LogicalCpuId::new(logical_cpu));
    let report = build_per_core_scheduler_report(logical_cpu, &mut scheduler);
    publish_per_core_scheduler_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress);
    core_state.clean_to_poc();
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
#[derive(Clone, Copy)]
struct ProductionSecondaryDispatchReport {
    owner: u64,
    role: SchedulerCoreRole,
    production_dispatch_enabled: bool,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    progress: u64,
    state_transitions: u64,
    production_dispatches: u64,
    context_switches: u64,
    cross_owner_rejected: bool,
    cross_owner_dispatch_rejected: bool,
    errors: u64,
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
impl ProductionSecondaryDispatchReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            production_dispatch_enabled: false,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            progress: 0,
            state_transitions: 0,
            production_dispatches: 0,
            context_switches: 0,
            cross_owner_rejected: false,
            cross_owner_dispatch_rejected: false,
            errors: 0,
        }
    }
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
#[derive(Clone, Copy)]
struct ProductionSecondaryDispatchState {
    reports: [ProductionSecondaryDispatchReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
impl ProductionSecondaryDispatchState {
    const fn new() -> Self {
        Self {
            reports: [ProductionSecondaryDispatchReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
static PRODUCTION_SECONDARY_DISPATCH_STATE: SpinLock<ProductionSecondaryDispatchState> =
    SpinLock::new(ProductionSecondaryDispatchState::new());

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
fn reset_production_secondary_dispatch_state() {
    let mut state = unsafe { PRODUCTION_SECONDARY_DISPATCH_STATE.lock_irqsave() };
    *state = ProductionSecondaryDispatchState::new();
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
fn build_production_secondary_dispatch_report(
    logical_cpu: usize,
    scheduler: &mut PerCoreScheduler<2>,
) -> ProductionSecondaryDispatchReport {
    let requester = LogicalCpuId::new(logical_cpu);
    let wrong_requester = LogicalCpuId::BOOT;
    let mut errors = 0;

    let cross_owner_rejected = match scheduler.local_scheduler_mut(wrong_requester) {
        Err(PerCoreSchedulerAccessError::WrongOwner {
            owner,
            requester: wrong,
        }) => owner == requester && wrong == wrong_requester,
        _ => false,
    };
    if !cross_owner_rejected {
        errors += 1;
    }

    let mut wrong_owner_task = scheduler_task(logical_cpu, 99);
    let cross_owner_dispatch_rejected = match scheduler
        .dispatch_cpu_local_diagnostic_task(wrong_requester, &mut wrong_owner_task)
    {
        Err(ProductionDispatchError::WrongOwner {
            owner,
            requester: wrong,
        }) => owner == requester && wrong == wrong_requester,
        _ => false,
    };
    if !cross_owner_dispatch_rejected {
        errors += 1;
    }

    let mut progress = 0;
    while progress < PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET {
        let next_progress = progress + 1;
        let mut task = scheduler_task(logical_cpu, next_progress);
        match scheduler.local_scheduler_mut(requester) {
            Ok(local_scheduler) => {
                if local_scheduler.make_runnable(&mut task).is_err() {
                    errors += 1;
                    break;
                }
            }
            Err(_) => {
                errors += 1;
                break;
            }
        }

        match scheduler.dispatch_cpu_local_diagnostic_task(requester, &mut task) {
            Ok(task_id) if task_id == task.id() && task.state() == TaskState::Running => {
                progress = next_progress;
            }
            _ => {
                errors += 1;
                break;
            }
        }
    }

    let local_scheduler = scheduler.scheduler();
    let counters = local_scheduler.counters();
    ProductionSecondaryDispatchReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        progress,
        state_transitions: counters.state_transitions(),
        production_dispatches: counters.production_dispatches(),
        context_switches: counters.context_switches(),
        cross_owner_rejected,
        cross_owner_dispatch_rejected,
        errors,
    }
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
fn publish_production_secondary_dispatch_report(
    logical_cpu: usize,
    report: ProductionSecondaryDispatchReport,
) {
    let mut state = PRODUCTION_SECONDARY_DISPATCH_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress;
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
fn run_production_secondary_dispatch_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut scheduler =
        PerCoreScheduler::<2>::production_secondary_diagnostic(LogicalCpuId::new(logical_cpu));
    let report = build_production_secondary_dispatch_report(logical_cpu, &mut scheduler);
    publish_production_secondary_dispatch_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress);
    core_state.clean_to_poc();
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
#[derive(Clone, Copy)]
struct SharedSchedulerMetadataReport {
    owner: u64,
    role: SchedulerCoreRole,
    production_dispatch_enabled: bool,
    task_id: u64,
    task_state: u64,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    metadata_len: u64,
    metadata_generation: u64,
    lookup_owner: u64,
    lookup_task: u64,
    lookup_generation: u64,
    boot_lookup_owner: u64,
    boot_lookup_task: u64,
    boot_lookup_generation: u64,
    cross_owner_rejected: bool,
    metadata_cross_owner_rejected: bool,
    local_queue_preserved: bool,
    errors: u64,
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
impl SharedSchedulerMetadataReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            production_dispatch_enabled: false,
            task_id: 0,
            task_state: 0,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            metadata_len: 0,
            metadata_generation: 0,
            lookup_owner: u64::MAX,
            lookup_task: 0,
            lookup_generation: 0,
            boot_lookup_owner: u64::MAX,
            boot_lookup_task: 0,
            boot_lookup_generation: 0,
            cross_owner_rejected: false,
            metadata_cross_owner_rejected: false,
            local_queue_preserved: false,
            errors: 0,
        }
    }
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
#[derive(Clone, Copy)]
struct SharedSchedulerMetadataSmokeState {
    reports: [SharedSchedulerMetadataReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
impl SharedSchedulerMetadataSmokeState {
    const fn new() -> Self {
        Self {
            reports: [SharedSchedulerMetadataReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
static SHARED_SCHEDULER_METADATA_SMOKE_STATE: SpinLock<SharedSchedulerMetadataSmokeState> =
    SpinLock::new(SharedSchedulerMetadataSmokeState::new());

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
static SHARED_SCHEDULER_METADATA_SMOKE_TABLE: SharedSchedulerMetadataLock<
    SHARED_SCHEDULER_METADATA_TASK_CAPACITY,
    MAX_CORES,
> = SpinLock::new(SharedSchedulerMetadata::new());

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
fn reset_shared_scheduler_metadata_smoke_state() {
    let mut state = unsafe { SHARED_SCHEDULER_METADATA_SMOKE_STATE.lock_irqsave() };
    *state = SharedSchedulerMetadataSmokeState::new();
    let mut metadata = unsafe { SHARED_SCHEDULER_METADATA_SMOKE_TABLE.lock_irqsave() };
    *metadata = SharedSchedulerMetadata::new();
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
fn build_shared_scheduler_metadata_report(
    logical_cpu: usize,
    scheduler: &mut PerCoreScheduler<2>,
) -> SharedSchedulerMetadataReport {
    let requester = LogicalCpuId::new(logical_cpu);
    let wrong_requester = if logical_cpu == 0 {
        LogicalCpuId::new(1)
    } else {
        LogicalCpuId::BOOT
    };
    let mut task = scheduler_task(logical_cpu, 1);
    let mut errors = 0;

    match scheduler.local_scheduler_mut(requester) {
        Ok(local_scheduler) => {
            if local_scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
            }
        }
        Err(_) => errors += 1,
    }

    if scheduler.dispatch_cpu_local_diagnostic_task(requester, &mut task) != Ok(task.id()) {
        errors += 1;
    }

    let queue_len_before_cross_owner = scheduler.scheduler().runnable().len();
    let cross_owner_rejected = match scheduler.local_scheduler_mut(wrong_requester) {
        Err(PerCoreSchedulerAccessError::WrongOwner { owner, requester }) => {
            owner == scheduler.owner() && requester == wrong_requester
        }
        _ => false,
    };
    if !cross_owner_rejected {
        errors += 1;
    }
    let local_queue_preserved =
        scheduler.scheduler().runnable().len() == queue_len_before_cross_owner;
    if !local_queue_preserved {
        errors += 1;
    }

    let (
        metadata_cross_owner_rejected,
        metadata_len,
        metadata_generation,
        lookup_owner,
        lookup_task,
        lookup_generation,
        boot_lookup_owner,
        boot_lookup_task,
        boot_lookup_generation,
    ) = {
        let mut metadata = unsafe { SHARED_SCHEDULER_METADATA_SMOKE_TABLE.lock_irqsave() };
        if metadata
            .register_local_task(requester, scheduler, &task)
            .is_err()
        {
            errors += 1;
        }

        let own_lookup = metadata.lookup_task(task.id());
        let boot_lookup = metadata.lookup_task(task_id(101));
        let metadata_cross_owner_rejected =
            match metadata.register_local_task(wrong_requester, scheduler, &task) {
                Err(SharedSchedulerMetadataError::WrongOwner { owner, requester }) => {
                    owner == scheduler.owner() && requester == wrong_requester
                }
                _ => false,
            };
        if !metadata_cross_owner_rejected {
            errors += 1;
        }

        let (lookup_owner, lookup_task, lookup_generation) = match own_lookup {
            Ok(snapshot) => (
                snapshot.owner().raw() as u64,
                snapshot.task_id().raw(),
                snapshot.generation(),
            ),
            Err(_) => {
                errors += 1;
                (u64::MAX, 0, 0)
            }
        };
        let (boot_lookup_owner, boot_lookup_task, boot_lookup_generation) = match boot_lookup {
            Ok(snapshot) => (
                snapshot.owner().raw() as u64,
                snapshot.task_id().raw(),
                snapshot.generation(),
            ),
            Err(_) => {
                errors += 1;
                (u64::MAX, 0, 0)
            }
        };

        (
            metadata_cross_owner_rejected,
            metadata.len() as u64,
            metadata.generation(),
            lookup_owner,
            lookup_task,
            lookup_generation,
            boot_lookup_owner,
            boot_lookup_task,
            boot_lookup_generation,
        )
    };

    let local_scheduler = scheduler.scheduler();
    SharedSchedulerMetadataReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        task_id: task.id().raw(),
        task_state: task_state_code(task.state()),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        metadata_len,
        metadata_generation,
        lookup_owner,
        lookup_task,
        lookup_generation,
        boot_lookup_owner,
        boot_lookup_task,
        boot_lookup_generation,
        cross_owner_rejected,
        metadata_cross_owner_rejected,
        local_queue_preserved,
        errors,
    }
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
fn publish_shared_scheduler_metadata_report(
    logical_cpu: usize,
    report: SharedSchedulerMetadataReport,
) {
    let mut state = SHARED_SCHEDULER_METADATA_SMOKE_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = u64::from(report.errors == 0);
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
fn run_shared_scheduler_metadata_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut scheduler =
        PerCoreScheduler::<2>::production_secondary_diagnostic(LogicalCpuId::new(logical_cpu));
    let report = build_shared_scheduler_metadata_report(logical_cpu, &mut scheduler);
    publish_shared_scheduler_metadata_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.lock_progress());
    core_state.clean_to_poc();
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
#[derive(Clone, Copy)]
struct SecondarySchedulerServiceLoopReportLine {
    owner: u64,
    role: SchedulerCoreRole,
    task_id: u64,
    task_state: u64,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    remote_wake_task: u64,
    dispatch_task: u64,
    no_work_did_work: bool,
    metadata_generation: u64,
    metadata_len: u64,
    observed_remote_wake: bool,
    pending_timer_preemption: bool,
    dispatch_requested: bool,
    cross_owner_rejected: bool,
    deferred_role_rejected: bool,
    local_queue_preserved: bool,
    errors: u64,
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
impl SecondarySchedulerServiceLoopReportLine {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            task_id: 0,
            task_state: 0,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            remote_wake_task: 0,
            dispatch_task: 0,
            no_work_did_work: true,
            metadata_generation: 0,
            metadata_len: 0,
            observed_remote_wake: false,
            pending_timer_preemption: true,
            dispatch_requested: false,
            cross_owner_rejected: false,
            deferred_role_rejected: false,
            local_queue_preserved: false,
            errors: 0,
        }
    }

    const fn progress(self) -> u64 {
        if self.errors == 0 && self.remote_wake_task != 0 && self.dispatch_task != 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
struct SecondarySchedulerServiceLoopState {
    owner: [AtomicU64; MAX_CORES],
    role: [AtomicU64; MAX_CORES],
    task_id: [AtomicU64; MAX_CORES],
    task_state: [AtomicU64; MAX_CORES],
    current_task: [AtomicU64; MAX_CORES],
    queue_len: [AtomicU64; MAX_CORES],
    front_task: [AtomicU64; MAX_CORES],
    remote_wake_task: [AtomicU64; MAX_CORES],
    dispatch_task: [AtomicU64; MAX_CORES],
    no_work_did_work: [AtomicU64; MAX_CORES],
    metadata_generation: [AtomicU64; MAX_CORES],
    metadata_len: [AtomicU64; MAX_CORES],
    observed_remote_wake: [AtomicU64; MAX_CORES],
    pending_timer_preemption: [AtomicU64; MAX_CORES],
    dispatch_requested: [AtomicU64; MAX_CORES],
    cross_owner_rejected: [AtomicU64; MAX_CORES],
    deferred_role_rejected: [AtomicU64; MAX_CORES],
    local_queue_preserved: [AtomicU64; MAX_CORES],
    errors: [AtomicU64; MAX_CORES],
    progress: [AtomicU64; MAX_CORES],
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
impl SecondarySchedulerServiceLoopState {
    const fn new() -> Self {
        Self {
            owner: [const { AtomicU64::new(u64::MAX) }; MAX_CORES],
            role: [const { AtomicU64::new(2) }; MAX_CORES],
            task_id: [const { AtomicU64::new(0) }; MAX_CORES],
            task_state: [const { AtomicU64::new(0) }; MAX_CORES],
            current_task: [const { AtomicU64::new(0) }; MAX_CORES],
            queue_len: [const { AtomicU64::new(0) }; MAX_CORES],
            front_task: [const { AtomicU64::new(0) }; MAX_CORES],
            remote_wake_task: [const { AtomicU64::new(0) }; MAX_CORES],
            dispatch_task: [const { AtomicU64::new(0) }; MAX_CORES],
            no_work_did_work: [const { AtomicU64::new(1) }; MAX_CORES],
            metadata_generation: [const { AtomicU64::new(0) }; MAX_CORES],
            metadata_len: [const { AtomicU64::new(0) }; MAX_CORES],
            observed_remote_wake: [const { AtomicU64::new(0) }; MAX_CORES],
            pending_timer_preemption: [const { AtomicU64::new(1) }; MAX_CORES],
            dispatch_requested: [const { AtomicU64::new(0) }; MAX_CORES],
            cross_owner_rejected: [const { AtomicU64::new(0) }; MAX_CORES],
            deferred_role_rejected: [const { AtomicU64::new(0) }; MAX_CORES],
            local_queue_preserved: [const { AtomicU64::new(0) }; MAX_CORES],
            errors: [const { AtomicU64::new(0) }; MAX_CORES],
            progress: [const { AtomicU64::new(0) }; MAX_CORES],
        }
    }

    fn reset(&self) {
        for logical_cpu in 0..MAX_CORES {
            self.owner[logical_cpu].store(u64::MAX, Ordering::Release);
            self.role[logical_cpu].store(2, Ordering::Release);
            self.task_id[logical_cpu].store(0, Ordering::Release);
            self.task_state[logical_cpu].store(0, Ordering::Release);
            self.current_task[logical_cpu].store(0, Ordering::Release);
            self.queue_len[logical_cpu].store(0, Ordering::Release);
            self.front_task[logical_cpu].store(0, Ordering::Release);
            self.remote_wake_task[logical_cpu].store(0, Ordering::Release);
            self.dispatch_task[logical_cpu].store(0, Ordering::Release);
            self.no_work_did_work[logical_cpu].store(1, Ordering::Release);
            self.metadata_generation[logical_cpu].store(0, Ordering::Release);
            self.metadata_len[logical_cpu].store(0, Ordering::Release);
            self.observed_remote_wake[logical_cpu].store(0, Ordering::Release);
            self.pending_timer_preemption[logical_cpu].store(1, Ordering::Release);
            self.dispatch_requested[logical_cpu].store(0, Ordering::Release);
            self.cross_owner_rejected[logical_cpu].store(0, Ordering::Release);
            self.deferred_role_rejected[logical_cpu].store(0, Ordering::Release);
            self.local_queue_preserved[logical_cpu].store(0, Ordering::Release);
            self.errors[logical_cpu].store(0, Ordering::Release);
            self.progress[logical_cpu].store(0, Ordering::Release);
        }
    }
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
static SECONDARY_SCHEDULER_SERVICE_LOOP_STATE: SecondarySchedulerServiceLoopState =
    SecondarySchedulerServiceLoopState::new();

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
fn reset_secondary_scheduler_service_loop_state() {
    SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.reset();
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
fn build_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
) -> SecondarySchedulerServiceLoopReportLine {
    let owner = LogicalCpuId::new(logical_cpu);
    let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
    let mut remote_wakes = RemoteWakeQueue::<2>::new(owner);
    let mut metadata =
        SharedSchedulerMetadata::<SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY, MAX_CORES>::new();
    let mut task = scheduler_task(logical_cpu, 1);
    task.set_state(TaskState::Blocked);

    let mut errors = 0;
    let mut metadata_generation = 0;

    if metadata
        .register_local_task(owner, &scheduler, &task)
        .is_err()
    {
        errors += 1;
    }

    if remote_wakes
        .publish(LogicalCpuId::BOOT, owner, task.id())
        .is_err()
    {
        errors += 1;
    }

    let cross_owner_rejected = matches!(
        SecondarySchedulerServiceLoop::run_once(
            LogicalCpuId::BOOT,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            false,
        ),
        Err(SecondarySchedulerServiceLoopError::WrongOwner { .. })
    );
    if !cross_owner_rejected {
        errors += 1;
    }

    let mut deferred = PerCoreScheduler::<2>::deferred_secondary(owner);
    let deferred_role_rejected = matches!(
        SecondarySchedulerServiceLoop::run_once(
            owner,
            &mut deferred,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            false,
        ),
        Err(SecondarySchedulerServiceLoopError::ProductionDispatchDeferred { .. })
    );
    if !deferred_role_rejected {
        errors += 1;
    }

    let first_cycle = SecondarySchedulerServiceLoop::run_once(
        owner,
        &mut scheduler,
        &mut remote_wakes,
        &mut metadata,
        &mut task,
        None,
        false,
        true,
    );

    let (
        remote_wake_task,
        dispatch_task,
        observed_remote_wake,
        pending_timer_preemption,
        dispatch_requested,
    ) = match first_cycle {
        Ok(report) => (
            report.cycle().remote_wake().map_or(0, TaskId::raw),
            report.cycle().dispatch().map_or(0, TaskId::raw),
            report.observed_remote_wake(),
            report.pending_timer_preemption(),
            report.dispatch_requested(),
        ),
        Err(_) => {
            errors += 1;
            (0, 0, false, true, false)
        }
    };

    let no_work_did_work = match SecondarySchedulerServiceLoop::run_once(
        owner,
        &mut scheduler,
        &mut remote_wakes,
        &mut metadata,
        &mut task,
        None,
        false,
        false,
    ) {
        Ok(report) => {
            metadata_generation = report.cycle().metadata().generation();
            report.did_work()
        }
        Err(_) => {
            errors += 1;
            true
        }
    };

    let metadata_len = metadata.len() as u64;

    let queue_len = scheduler.scheduler().runnable().len() as u64;
    let front_task = scheduler
        .scheduler()
        .runnable()
        .front()
        .map_or(0, TaskId::raw);
    let current_task = scheduler.current_task().map_or(0, TaskId::raw);
    let local_queue_preserved = queue_len == 0
        && front_task == 0
        && current_task == task.id().raw()
        && remote_wakes.is_empty()
        && task.state() == TaskState::Running;

    if !local_queue_preserved {
        errors += 1;
    }

    SecondarySchedulerServiceLoopReportLine {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        task_id: task.id().raw(),
        task_state: task_state_code(task.state()),
        current_task,
        queue_len,
        front_task,
        remote_wake_task,
        dispatch_task,
        no_work_did_work,
        metadata_generation,
        metadata_len,
        observed_remote_wake,
        pending_timer_preemption,
        dispatch_requested,
        cross_owner_rejected,
        deferred_role_rejected,
        local_queue_preserved,
        errors,
    }
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
fn publish_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
    report: SecondarySchedulerServiceLoopReportLine,
) {
    let state = &SECONDARY_SCHEDULER_SERVICE_LOOP_STATE;
    state.owner[logical_cpu].store(report.owner, Ordering::Release);
    state.role[logical_cpu].store(scheduler_role_code(report.role), Ordering::Release);
    state.task_id[logical_cpu].store(report.task_id, Ordering::Release);
    state.task_state[logical_cpu].store(report.task_state, Ordering::Release);
    state.current_task[logical_cpu].store(report.current_task, Ordering::Release);
    state.queue_len[logical_cpu].store(report.queue_len, Ordering::Release);
    state.front_task[logical_cpu].store(report.front_task, Ordering::Release);
    state.remote_wake_task[logical_cpu].store(report.remote_wake_task, Ordering::Release);
    state.dispatch_task[logical_cpu].store(report.dispatch_task, Ordering::Release);
    state.no_work_did_work[logical_cpu]
        .store(u64::from(report.no_work_did_work), Ordering::Release);
    state.metadata_generation[logical_cpu].store(report.metadata_generation, Ordering::Release);
    state.metadata_len[logical_cpu].store(report.metadata_len, Ordering::Release);
    state.observed_remote_wake[logical_cpu]
        .store(u64::from(report.observed_remote_wake), Ordering::Release);
    state.pending_timer_preemption[logical_cpu].store(
        u64::from(report.pending_timer_preemption),
        Ordering::Release,
    );
    state.dispatch_requested[logical_cpu]
        .store(u64::from(report.dispatch_requested), Ordering::Release);
    state.cross_owner_rejected[logical_cpu]
        .store(u64::from(report.cross_owner_rejected), Ordering::Release);
    state.deferred_role_rejected[logical_cpu]
        .store(u64::from(report.deferred_role_rejected), Ordering::Release);
    state.local_queue_preserved[logical_cpu]
        .store(u64::from(report.local_queue_preserved), Ordering::Release);
    state.errors[logical_cpu].store(report.errors, Ordering::Release);
    state.progress[logical_cpu].store(report.progress(), Ordering::Release);
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
fn scheduler_role_code(role: SchedulerCoreRole) -> u64 {
    match role {
        SchedulerCoreRole::BootCpuProduction => 1,
        SchedulerCoreRole::SecondaryDeferred => 2,
        SchedulerCoreRole::SecondaryProductionDiagnostic => 3,
    }
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
fn scheduler_role_from_code(code: u64) -> SchedulerCoreRole {
    match code {
        1 => SchedulerCoreRole::BootCpuProduction,
        3 => SchedulerCoreRole::SecondaryProductionDiagnostic,
        _ => SchedulerCoreRole::SecondaryDeferred,
    }
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
fn load_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
) -> SecondarySchedulerServiceLoopReportLine {
    let state = &SECONDARY_SCHEDULER_SERVICE_LOOP_STATE;
    SecondarySchedulerServiceLoopReportLine {
        owner: state.owner[logical_cpu].load(Ordering::Acquire),
        role: scheduler_role_from_code(state.role[logical_cpu].load(Ordering::Acquire)),
        task_id: state.task_id[logical_cpu].load(Ordering::Acquire),
        task_state: state.task_state[logical_cpu].load(Ordering::Acquire),
        current_task: state.current_task[logical_cpu].load(Ordering::Acquire),
        queue_len: state.queue_len[logical_cpu].load(Ordering::Acquire),
        front_task: state.front_task[logical_cpu].load(Ordering::Acquire),
        remote_wake_task: state.remote_wake_task[logical_cpu].load(Ordering::Acquire),
        dispatch_task: state.dispatch_task[logical_cpu].load(Ordering::Acquire),
        no_work_did_work: state.no_work_did_work[logical_cpu].load(Ordering::Acquire) != 0,
        metadata_generation: state.metadata_generation[logical_cpu].load(Ordering::Acquire),
        metadata_len: state.metadata_len[logical_cpu].load(Ordering::Acquire),
        observed_remote_wake: state.observed_remote_wake[logical_cpu].load(Ordering::Acquire) != 0,
        pending_timer_preemption: state.pending_timer_preemption[logical_cpu]
            .load(Ordering::Acquire)
            != 0,
        dispatch_requested: state.dispatch_requested[logical_cpu].load(Ordering::Acquire) != 0,
        cross_owner_rejected: state.cross_owner_rejected[logical_cpu].load(Ordering::Acquire) != 0,
        deferred_role_rejected: state.deferred_role_rejected[logical_cpu].load(Ordering::Acquire)
            != 0,
        local_queue_preserved: state.local_queue_preserved[logical_cpu].load(Ordering::Acquire)
            != 0,
        errors: state.errors[logical_cpu].load(Ordering::Acquire),
    }
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
fn run_secondary_scheduler_service_loop_secondary(
    core_state: &smp::PerCoreState,
    logical_cpu: usize,
) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let report = build_secondary_scheduler_service_loop_report(logical_cpu);
    publish_secondary_scheduler_service_loop_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress());
    core_state.clean_to_poc();
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
impl SharedSchedulerMetadataReport {
    const fn lock_progress(self) -> u64 {
        if self.errors == 0 { 1 } else { 0 }
    }
}

#[cfg(talos_qemu_cross_core_ipi_delivery_smoke)]
struct CrossCoreIpiDeliveryState {
    ready_mask: AtomicU64,
    complete_mask: AtomicU64,
    sent_values: [AtomicU64; MAX_CORES],
    target_bits: [AtomicU64; MAX_CORES],
    receive_counts: [AtomicU64; MAX_CORES],
    eoi_counts: [AtomicU64; MAX_CORES],
    last_vectors: [AtomicU64; MAX_CORES],
    last_iars: [AtomicU64; MAX_CORES],
    last_intids: [AtomicU64; MAX_CORES],
    errors: AtomicU64,
}

#[cfg(talos_qemu_cross_core_ipi_delivery_smoke)]
impl CrossCoreIpiDeliveryState {
    const fn new() -> Self {
        Self {
            ready_mask: AtomicU64::new(0),
            complete_mask: AtomicU64::new(0),
            sent_values: [const { AtomicU64::new(0) }; MAX_CORES],
            target_bits: [const { AtomicU64::new(0) }; MAX_CORES],
            receive_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            eoi_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            last_vectors: [const { AtomicU64::new(0) }; MAX_CORES],
            last_iars: [const { AtomicU64::new(0) }; MAX_CORES],
            last_intids: [const { AtomicU64::new(0) }; MAX_CORES],
            errors: AtomicU64::new(0),
        }
    }

    fn reset(&self) {
        self.ready_mask.store(0, Ordering::Release);
        self.complete_mask.store(0, Ordering::Release);
        self.errors.store(0, Ordering::Release);
        for logical_cpu in 0..MAX_CORES {
            self.sent_values[logical_cpu].store(0, Ordering::Release);
            self.target_bits[logical_cpu].store(0, Ordering::Release);
            self.receive_counts[logical_cpu].store(0, Ordering::Release);
            self.eoi_counts[logical_cpu].store(0, Ordering::Release);
            self.last_vectors[logical_cpu].store(0, Ordering::Release);
            self.last_iars[logical_cpu].store(0, Ordering::Release);
            self.last_intids[logical_cpu].store(0, Ordering::Release);
        }
    }

    fn mark_ready(&self, logical_cpu: usize) {
        self.ready_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn mark_complete(&self, logical_cpu: usize) {
        self.complete_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn record_send(&self, logical_cpu: usize, target_bit: u8, sgir_value: u32) {
        self.target_bits[logical_cpu].store(target_bit as u64, Ordering::Release);
        self.sent_values[logical_cpu].store(sgir_value as u64, Ordering::Release);
    }

    fn record_receive(&self, logical_cpu: Option<usize>, vector: u64, iar: u32, intid: u32) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.last_vectors[logical_cpu].store(vector, Ordering::Release);
            self.last_iars[logical_cpu].store(iar as u64, Ordering::Release);
            self.last_intids[logical_cpu].store(intid as u64, Ordering::Release);
            self.receive_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn record_eoi(&self, logical_cpu: Option<usize>) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.eoi_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn receive_count(&self, logical_cpu: usize) -> u64 {
        self.receive_counts[logical_cpu].load(Ordering::Acquire)
    }
}

#[cfg(talos_qemu_cross_core_ipi_delivery_smoke)]
static CROSS_CORE_IPI_DELIVERY_STATE: CrossCoreIpiDeliveryState = CrossCoreIpiDeliveryState::new();

#[cfg(any(
    talos_qemu_cross_core_ipi_delivery_smoke,
    talos_qemu_remote_wakeup_request_smoke
))]
fn current_qemu_logical_cpu() -> Option<usize> {
    qemu_logical_cpu_from_mpidr_affinity(aarch64::mpidr_affinity(aarch64::mpidr_el1()))
}

#[cfg(talos_qemu_cross_core_ipi_delivery_smoke)]
fn run_cross_core_ipi_delivery_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        aarch64::enable_irq();
    }

    CROSS_CORE_IPI_DELIVERY_STATE.mark_ready(logical_cpu);

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while CROSS_CORE_IPI_DELIVERY_STATE.receive_count(logical_cpu) == 0 && remaining > 0 {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }
    CROSS_CORE_IPI_DELIVERY_STATE.mark_complete(logical_cpu);
    core_state.mark_workload_complete(CROSS_CORE_IPI_DELIVERY_STATE.receive_count(logical_cpu));
    core_state.clean_to_poc();
}

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
struct RemoteWakeRequestSmokeState {
    ready_mask: AtomicU64,
    complete_mask: AtomicU64,
    sent_values: [AtomicU64; MAX_CORES],
    target_bits: [AtomicU64; MAX_CORES],
    receive_counts: [AtomicU64; MAX_CORES],
    eoi_counts: [AtomicU64; MAX_CORES],
    pending_counts: [AtomicU64; MAX_CORES],
    consumed_task_ids: [AtomicU64; MAX_CORES],
    duplicate_counts: [AtomicU64; MAX_CORES],
    queue_lens_after: [AtomicU64; MAX_CORES],
    cross_owner_rejections: [AtomicU64; MAX_CORES],
    production_deferrals: [AtomicU64; MAX_CORES],
    local_wake_task_ids: [AtomicU64; MAX_CORES],
    local_runnable_lens: [AtomicU64; MAX_CORES],
    local_state_before: [AtomicU64; MAX_CORES],
    local_state_after: [AtomicU64; MAX_CORES],
    duplicate_local_rejections: [AtomicU64; MAX_CORES],
    last_vectors: [AtomicU64; MAX_CORES],
    last_iars: [AtomicU64; MAX_CORES],
    last_intids: [AtomicU64; MAX_CORES],
    errors: AtomicU64,
}

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
impl RemoteWakeRequestSmokeState {
    const fn new() -> Self {
        Self {
            ready_mask: AtomicU64::new(0),
            complete_mask: AtomicU64::new(0),
            sent_values: [const { AtomicU64::new(0) }; MAX_CORES],
            target_bits: [const { AtomicU64::new(0) }; MAX_CORES],
            receive_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            eoi_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            pending_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            consumed_task_ids: [const { AtomicU64::new(0) }; MAX_CORES],
            duplicate_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            queue_lens_after: [const { AtomicU64::new(0) }; MAX_CORES],
            cross_owner_rejections: [const { AtomicU64::new(0) }; MAX_CORES],
            production_deferrals: [const { AtomicU64::new(0) }; MAX_CORES],
            local_wake_task_ids: [const { AtomicU64::new(0) }; MAX_CORES],
            local_runnable_lens: [const { AtomicU64::new(0) }; MAX_CORES],
            local_state_before: [const { AtomicU64::new(0) }; MAX_CORES],
            local_state_after: [const { AtomicU64::new(0) }; MAX_CORES],
            duplicate_local_rejections: [const { AtomicU64::new(0) }; MAX_CORES],
            last_vectors: [const { AtomicU64::new(0) }; MAX_CORES],
            last_iars: [const { AtomicU64::new(0) }; MAX_CORES],
            last_intids: [const { AtomicU64::new(0) }; MAX_CORES],
            errors: AtomicU64::new(0),
        }
    }

    fn reset(&self) {
        self.ready_mask.store(0, Ordering::Release);
        self.complete_mask.store(0, Ordering::Release);
        self.errors.store(0, Ordering::Release);
        for logical_cpu in 0..MAX_CORES {
            self.sent_values[logical_cpu].store(0, Ordering::Release);
            self.target_bits[logical_cpu].store(0, Ordering::Release);
            self.receive_counts[logical_cpu].store(0, Ordering::Release);
            self.eoi_counts[logical_cpu].store(0, Ordering::Release);
            self.pending_counts[logical_cpu].store(0, Ordering::Release);
            self.consumed_task_ids[logical_cpu].store(0, Ordering::Release);
            self.duplicate_counts[logical_cpu].store(0, Ordering::Release);
            self.queue_lens_after[logical_cpu].store(0, Ordering::Release);
            self.cross_owner_rejections[logical_cpu].store(0, Ordering::Release);
            self.production_deferrals[logical_cpu].store(0, Ordering::Release);
            self.local_wake_task_ids[logical_cpu].store(0, Ordering::Release);
            self.local_runnable_lens[logical_cpu].store(0, Ordering::Release);
            self.local_state_before[logical_cpu].store(0, Ordering::Release);
            self.local_state_after[logical_cpu].store(0, Ordering::Release);
            self.duplicate_local_rejections[logical_cpu].store(0, Ordering::Release);
            self.last_vectors[logical_cpu].store(0, Ordering::Release);
            self.last_iars[logical_cpu].store(0, Ordering::Release);
            self.last_intids[logical_cpu].store(0, Ordering::Release);
        }
    }

    fn mark_ready(&self, logical_cpu: usize) {
        self.ready_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn mark_complete(&self, logical_cpu: usize) {
        self.complete_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn record_send(&self, logical_cpu: usize, target_bit: u8, sgir_value: u32) {
        self.target_bits[logical_cpu].store(target_bit as u64, Ordering::Release);
        self.sent_values[logical_cpu].store(sgir_value as u64, Ordering::Release);
    }

    fn record_receive(&self, logical_cpu: Option<usize>, vector: u64, iar: u32, intid: u32) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.last_vectors[logical_cpu].store(vector, Ordering::Release);
            self.last_iars[logical_cpu].store(iar as u64, Ordering::Release);
            self.last_intids[logical_cpu].store(intid as u64, Ordering::Release);
            self.receive_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
            self.pending_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn record_eoi(&self, logical_cpu: Option<usize>) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.eoi_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn receive_count(&self, logical_cpu: usize) -> u64 {
        self.receive_counts[logical_cpu].load(Ordering::Acquire)
    }
}

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
static REMOTE_WAKE_REQUEST_SMOKE_STATE: RemoteWakeRequestSmokeState =
    RemoteWakeRequestSmokeState::new();

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
static REMOTE_WAKE_QUEUES: [SpinLock<RemoteWakeQueue<REMOTE_WAKE_QUEUE_CAPACITY>>; MAX_CORES] = [
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(0))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(1))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(2))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(3))),
];

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
fn reset_remote_wakeup_request_state() {
    REMOTE_WAKE_REQUEST_SMOKE_STATE.reset();
    for logical_cpu in 0..MAX_CORES {
        let mut queue = unsafe { REMOTE_WAKE_QUEUES[logical_cpu].lock_irqsave() };
        *queue = RemoteWakeQueue::new(LogicalCpuId::new(logical_cpu));
    }
}

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
fn publish_remote_wake_request(target: usize, task_id: TaskId) -> bool {
    let target_cpu = LogicalCpuId::new(target);
    let result = {
        let mut queue = unsafe { REMOTE_WAKE_QUEUES[target].lock_irqsave() };
        queue.publish(LogicalCpuId::BOOT, target_cpu, task_id)
    };
    smp_full_barrier();

    match result {
        Ok(RemoteWakePublishOutcome::Inserted) => {
            crate::println!(
                "qemu-remote-wakeup-request: publish requester=0 target={} task={} outcome=inserted",
                target,
                task_id.raw()
            );
            true
        }
        Ok(RemoteWakePublishOutcome::Duplicate) => {
            crate::println!(
                "qemu-remote-wakeup-request: publish requester=0 target={} task={} outcome=duplicate",
                target,
                task_id.raw()
            );
            true
        }
        Err(error) => {
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
            crate::println!(
                "qemu-remote-wakeup-request: publish requester=0 target={} task={} outcome=error {:?}",
                target,
                task_id.raw(),
                error
            );
            false
        }
    }
}

#[cfg(any(
    talos_qemu_remote_wake_to_local_runnable_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
fn task_state_code(state: TaskState) -> u64 {
    match state {
        TaskState::Running => 1,
        TaskState::Runnable => 2,
        TaskState::Blocked => 3,
    }
}

#[cfg(any(
    talos_qemu_remote_wake_to_local_runnable_smoke,
    talos_qemu_shared_scheduler_metadata_smoke,
    talos_qemu_secondary_scheduler_service_loop_smoke
))]
fn task_state_name(code: u64) -> &'static str {
    match code {
        1 => "running",
        2 => "runnable",
        3 => "blocked",
        _ => "unknown",
    }
}

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
fn run_remote_wakeup_request_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        aarch64::enable_irq();
    }

    REMOTE_WAKE_REQUEST_SMOKE_STATE.mark_ready(logical_cpu);

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while REMOTE_WAKE_REQUEST_SMOKE_STATE.receive_count(logical_cpu) == 0 && remaining > 0 {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let requester = LogicalCpuId::new(logical_cpu);
    let (consumed_request, duplicates, queue_len_after) = {
        let mut queue = unsafe { REMOTE_WAKE_QUEUES[logical_cpu].lock_irqsave() };
        let consumed = queue.consume_next(requester).ok().flatten();
        (consumed, queue.duplicate_count(), queue.len())
    };
    let consumed_task = consumed_request
        .map(|request| request.task_id().raw())
        .unwrap_or(0);

    REMOTE_WAKE_REQUEST_SMOKE_STATE.consumed_task_ids[logical_cpu]
        .store(consumed_task, Ordering::Release);
    REMOTE_WAKE_REQUEST_SMOKE_STATE.duplicate_counts[logical_cpu]
        .store(duplicates, Ordering::Release);
    REMOTE_WAKE_REQUEST_SMOKE_STATE.queue_lens_after[logical_cpu]
        .store(queue_len_after as u64, Ordering::Release);

    let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(requester);
    if matches!(
        scheduler.local_scheduler_mut(LogicalCpuId::BOOT),
        Err(PerCoreSchedulerAccessError::WrongOwner { .. })
    ) {
        REMOTE_WAKE_REQUEST_SMOKE_STATE.cross_owner_rejections[logical_cpu]
            .store(1, Ordering::Release);
    }
    if matches!(
        scheduler.production_scheduler_mut(requester),
        Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred { .. })
    ) {
        REMOTE_WAKE_REQUEST_SMOKE_STATE.production_deferrals[logical_cpu]
            .store(1, Ordering::Release);
    }

    #[cfg(talos_qemu_remote_wake_to_local_runnable_smoke)]
    {
        let local_task_id =
            TaskId::new(200 + logical_cpu as u64).expect("diagnostic task ID is nonzero");
        let stack_base = 0x80_0000 + logical_cpu * 0x1000;
        let mut task = Task::kernel_thread(
            local_task_id,
            KernelStack::new(stack_base, 0x1000).expect("diagnostic stack is valid"),
            ContextFrame::new(stack_base + 0xff0, 0x40_0000),
        );
        task.set_state(TaskState::Blocked);
        REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_before[logical_cpu]
            .store(task_state_code(task.state()), Ordering::Release);

        let wake_result = consumed_request
            .map(|request| {
                scheduler.wake_blocked_local_task_from_remote_request(requester, request, &mut task)
            })
            .unwrap_or(Err(TargetWakeConsumptionError::TaskMismatch {
                requested: local_task_id,
                local: local_task_id,
            }));
        if let Ok(woken_task) = wake_result {
            REMOTE_WAKE_REQUEST_SMOKE_STATE.local_wake_task_ids[logical_cpu]
                .store(woken_task.raw(), Ordering::Release);
        } else {
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
        }

        let duplicate_rejected = consumed_request
            .map(|request| {
                scheduler
                    .wake_blocked_local_task_from_remote_request(requester, request, &mut task)
                    .is_err()
            })
            .unwrap_or(false);
        if duplicate_rejected {
            REMOTE_WAKE_REQUEST_SMOKE_STATE.duplicate_local_rejections[logical_cpu]
                .store(1, Ordering::Release);
        } else {
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
        }

        REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_after[logical_cpu]
            .store(task_state_code(task.state()), Ordering::Release);
        REMOTE_WAKE_REQUEST_SMOKE_STATE.local_runnable_lens[logical_cpu].store(
            scheduler.scheduler().runnable().len() as u64,
            Ordering::Release,
        );
    }

    REMOTE_WAKE_REQUEST_SMOKE_STATE.mark_complete(logical_cpu);
    core_state.mark_workload_complete(consumed_task);
    core_state.clean_to_poc();
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

#[cfg(talos_qemu_per_core_scheduler_ownership_smoke)]
pub fn run_per_core_scheduler_ownership_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_per_core_scheduler_ownership_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-per-core-scheduler-ownership: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        PER_CORE_SCHEDULER_PROGRESS_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let irq_mask_probe = run_single_core_irq_mask_probe();
    publish_per_core_scheduler_report(0, build_boot_scheduler_report());

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-per-core-scheduler-ownership: cpu-on logical={} target-affinity={:#x} result={}",
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

    let final_state = PER_CORE_SCHEDULER_OWNERSHIP_STATE
        .try_lock()
        .map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(PerCoreSchedulerOwnershipState::new);
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && lock_available;

    for logical_cpu in 0..MAX_CORES {
        let report = final_state.reports[logical_cpu];
        let (lifecycle, context, mapped, stack_owned) = if logical_cpu == 0 {
            (CoreLifecycle::WorkloadComplete, 0, boot_logical, true)
        } else {
            let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
            let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
            let stack_slot = stack_layout
                .slot(logical_cpu)
                .expect("stack slot for possible QEMU core");
            (
                core_report.lifecycle,
                core_report.context,
                logical_from_mpidr,
                stack_slot.contains_stack_pointer(core_report.stack_pointer),
            )
        };

        let role_ok = if logical_cpu == 0 {
            report.role == SchedulerCoreRole::BootCpuProduction
                && report.production_dispatch_enabled
                && !report.dispatch_deferred
                && report.current_task == 1
        } else {
            report.role == SchedulerCoreRole::SecondaryDeferred
                && !report.production_dispatch_enabled
                && report.dispatch_deferred
                && report.current_task == 0
        };
        let report_ok = lifecycle >= CoreLifecycle::WorkloadComplete
            && context == logical_cpu
            && mapped == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && role_ok
            && report.queue_len == 0
            && report.front_task == 0
            && report.progress == PER_CORE_SCHEDULER_PROGRESS_TARGET
            && report.state_transitions == PER_CORE_SCHEDULER_PROGRESS_TARGET
            && final_state.lock_progress[logical_cpu] == PER_CORE_SCHEDULER_PROGRESS_TARGET
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-per-core-scheduler-ownership: report logical={} state={} context={} mapped={:?} owner={} role={} production={} current={} queue-len={} front={} progress={} transitions={} dispatch-deferred={} lock-progress={} irq-ok={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(lifecycle.raw()),
            context,
            mapped,
            report.owner,
            scheduler_role_name(report.role),
            report.production_dispatch_enabled,
            report.current_task,
            report.queue_len,
            report.front_task,
            report.progress,
            report.state_transitions,
            report.dispatch_deferred,
            final_state.lock_progress[logical_cpu],
            irq_mask_probe.passed(),
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok && irq_mask_probe.passed() {
        "qemu-per-core-scheduler-ownership-complete"
    } else if !lock_available {
        "qemu-per-core-scheduler-ownership-lock-still-held"
    } else if cpu_on_ok {
        "qemu-per-core-scheduler-ownership-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-per-core-scheduler-ownership: final participants={} expected={} errors={} lock-available={} irq-ok={} wait-remaining={} classification={}",
        participants,
        MAX_CORES,
        errors,
        lock_available,
        irq_mask_probe.passed(),
        remaining,
        classification
    );

    if reports_ok && irq_mask_probe.passed() {
        crate::println!("qemu-per-core-scheduler-ownership: PASS");
    } else {
        crate::println!("qemu-per-core-scheduler-ownership: FAIL");
    }

    reports_ok && irq_mask_probe.passed()
}

#[cfg(talos_qemu_production_secondary_dispatch_smoke)]
pub fn run_production_secondary_dispatch_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_production_secondary_dispatch_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-production-secondary-dispatch: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET,
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
            "qemu-production-secondary-dispatch: cpu-on logical={} target-affinity={:#x} result={}",
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

    let final_state = PRODUCTION_SECONDARY_DISPATCH_STATE
        .try_lock()
        .map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(ProductionSecondaryDispatchState::new);
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && lock_available;

    for logical_cpu in 1..MAX_CORES {
        let report = final_state.reports[logical_cpu];
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_current =
            (logical_cpu as u64 + 1) * 100 + PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET;
        let role_ok = report.role == SchedulerCoreRole::SecondaryProductionDiagnostic
            && report.production_dispatch_enabled
            && report.current_task == expected_current;
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && role_ok
            && report.queue_len == 0
            && report.front_task == 0
            && report.progress == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && report.state_transitions == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET * 2
            && report.production_dispatches == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && report.context_switches == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && final_state.lock_progress[logical_cpu]
                == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && report.cross_owner_rejected
            && report.cross_owner_dispatch_rejected
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-production-secondary-dispatch: report logical={} state={} context={} mapped={:?} owner={} role={} production={} current={} queue-len={} front={} progress={} transitions={} production-dispatches={} context-switches={} cross-owner-rejected={} cross-owner-dispatch-rejected={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            report.owner,
            scheduler_role_name(report.role),
            report.production_dispatch_enabled,
            report.current_task,
            report.queue_len,
            report.front_task,
            report.progress,
            report.state_transitions,
            report.production_dispatches,
            report.context_switches,
            report.cross_owner_rejected,
            report.cross_owner_dispatch_rejected,
            final_state.lock_progress[logical_cpu],
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-production-secondary-dispatch-complete"
    } else if !lock_available {
        "qemu-production-secondary-dispatch-lock-still-held"
    } else if cpu_on_ok {
        "qemu-production-secondary-dispatch-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-production-secondary-dispatch: final participants={} expected={} errors={} lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-production-secondary-dispatch: PASS");
    } else {
        crate::println!("qemu-production-secondary-dispatch: FAIL");
    }

    reports_ok
}

#[cfg(talos_qemu_shared_scheduler_metadata_smoke)]
pub fn run_shared_scheduler_metadata_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_shared_scheduler_metadata_smoke_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-shared-scheduler-metadata: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SHARED_SCHEDULER_METADATA_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut boot_scheduler = PerCoreScheduler::<2>::boot_cpu();
    let boot_report = build_shared_scheduler_metadata_report(0, &mut boot_scheduler);
    publish_shared_scheduler_metadata_report(0, boot_report);

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-shared-scheduler-metadata: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = SHARED_SCHEDULER_METADATA_WAIT_LIMIT;
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

    let final_state = SHARED_SCHEDULER_METADATA_SMOKE_STATE
        .try_lock()
        .map(|state| *state);
    let state_lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SharedSchedulerMetadataSmokeState::new);
    let final_metadata = SHARED_SCHEDULER_METADATA_SMOKE_TABLE
        .try_lock()
        .map(|metadata| (metadata.len(), metadata.generation()));
    let metadata_lock_available = final_metadata.is_some();
    let (final_metadata_len, final_metadata_generation) = final_metadata.unwrap_or((0, 0));

    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && state_lock_available
        && metadata_lock_available
        && final_metadata_len == MAX_CORES;

    for logical_cpu in 0..MAX_CORES {
        let report = final_state.reports[logical_cpu];
        let (lifecycle, context, mapped, stack_owned) = if logical_cpu == 0 {
            (CoreLifecycle::WorkloadComplete, 0, boot_logical, true)
        } else {
            let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
            let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
            let stack_slot = stack_layout
                .slot(logical_cpu)
                .expect("stack slot for possible QEMU core");
            (
                core_report.lifecycle,
                core_report.context,
                logical_from_mpidr,
                stack_slot.contains_stack_pointer(core_report.stack_pointer),
            )
        };
        let expected_task = (logical_cpu as u64 + 1) * 100 + 1;
        let expected_role = if logical_cpu == 0 {
            SchedulerCoreRole::BootCpuProduction
        } else {
            SchedulerCoreRole::SecondaryProductionDiagnostic
        };
        let report_ok = lifecycle >= CoreLifecycle::WorkloadComplete
            && context == logical_cpu
            && mapped == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && report.role == expected_role
            && report.production_dispatch_enabled
            && report.task_id == expected_task
            && report.task_state == task_state_code(TaskState::Running)
            && report.current_task == expected_task
            && report.queue_len == 0
            && report.front_task == 0
            && report.lookup_owner == logical_cpu as u64
            && report.lookup_task == expected_task
            && report.lookup_generation > 0
            && report.boot_lookup_owner == 0
            && report.boot_lookup_task == 101
            && report.boot_lookup_generation > 0
            && report.cross_owner_rejected
            && report.metadata_cross_owner_rejected
            && report.local_queue_preserved
            && final_state.lock_progress[logical_cpu] == 1
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-shared-scheduler-metadata: report logical={} state={} context={} mapped={:?} owner={} role={} production={} task={} task-state={} current={} queue-len={} front={} metadata-len={} metadata-generation={} lookup-owner={} lookup-task={} lookup-generation={} boot-lookup-owner={} boot-lookup-task={} boot-lookup-generation={} cross-owner-rejected={} metadata-cross-owner-rejected={} local-queue-preserved={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(lifecycle.raw()),
            context,
            mapped,
            report.owner,
            scheduler_role_name(report.role),
            report.production_dispatch_enabled,
            report.task_id,
            task_state_name(report.task_state),
            report.current_task,
            report.queue_len,
            report.front_task,
            report.metadata_len,
            report.metadata_generation,
            report.lookup_owner,
            report.lookup_task,
            report.lookup_generation,
            report.boot_lookup_owner,
            report.boot_lookup_task,
            report.boot_lookup_generation,
            report.cross_owner_rejected,
            report.metadata_cross_owner_rejected,
            report.local_queue_preserved,
            final_state.lock_progress[logical_cpu],
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-shared-scheduler-metadata-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "qemu-shared-scheduler-metadata-lock-still-held"
    } else if cpu_on_ok {
        "qemu-shared-scheduler-metadata-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-shared-scheduler-metadata: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} final-metadata-len={} final-metadata-generation={} wait-remaining={} classification={}",
        participants,
        MAX_CORES,
        errors,
        state_lock_available,
        metadata_lock_available,
        final_metadata_len,
        final_metadata_generation,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-shared-scheduler-metadata: PASS");
    } else {
        crate::println!("qemu-shared-scheduler-metadata: FAIL");
    }

    reports_ok
}

#[cfg(talos_qemu_secondary_scheduler_service_loop_smoke)]
pub fn run_secondary_scheduler_service_loop_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_secondary_scheduler_service_loop_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-secondary-scheduler-service-loop: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY,
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
            "qemu-secondary-scheduler-service-loop: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = SECONDARY_SCHEDULER_SERVICE_LOOP_WAIT_LIMIT;
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

    let state_lock_available = true;
    let metadata_lock_available = true;
    let mut final_metadata_len = 0;
    let mut final_metadata_generation = 0;
    for logical_cpu in 1..MAX_CORES {
        let report = load_secondary_scheduler_service_loop_report(logical_cpu);
        final_metadata_len += report.metadata_len;
        final_metadata_generation += report.metadata_generation;
    }

    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && state_lock_available
        && final_metadata_len == (MAX_CORES - 1) as u64;

    for logical_cpu in 1..MAX_CORES {
        let report = load_secondary_scheduler_service_loop_report(logical_cpu);
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_task = (logical_cpu as u64 + 1) * 100 + 1;
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && report.role == SchedulerCoreRole::SecondaryProductionDiagnostic
            && report.task_id == expected_task
            && report.task_state == task_state_code(TaskState::Running)
            && report.current_task == expected_task
            && report.queue_len == 0
            && report.front_task == 0
            && report.remote_wake_task == expected_task
            && report.dispatch_task == expected_task
            && !report.no_work_did_work
            && report.metadata_generation > 0
            && report.observed_remote_wake
            && !report.pending_timer_preemption
            && report.dispatch_requested
            && report.cross_owner_rejected
            && report.deferred_role_rejected
            && report.local_queue_preserved
            && SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.progress[logical_cpu].load(Ordering::Acquire)
                == 1
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-secondary-scheduler-service-loop: report logical={} state={} context={} mapped={:?} owner={} role={} task={} task-state={} current={} queue-len={} front={} remote-wake={} dispatch={} no-work-did-work={} metadata-len={} metadata-generation={} observed-remote-wake={} pending-timer-preemption={} dispatch-requested={} cross-owner-rejected={} deferred-role-rejected={} local-queue-preserved={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            report.owner,
            scheduler_role_name(report.role),
            report.task_id,
            task_state_name(report.task_state),
            report.current_task,
            report.queue_len,
            report.front_task,
            report.remote_wake_task,
            report.dispatch_task,
            report.no_work_did_work,
            report.metadata_len,
            report.metadata_generation,
            report.observed_remote_wake,
            report.pending_timer_preemption,
            report.dispatch_requested,
            report.cross_owner_rejected,
            report.deferred_role_rejected,
            report.local_queue_preserved,
            SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.progress[logical_cpu].load(Ordering::Acquire),
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-secondary-scheduler-service-loop-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "qemu-secondary-scheduler-service-loop-lock-still-held"
    } else if cpu_on_ok {
        "qemu-secondary-scheduler-service-loop-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-secondary-scheduler-service-loop: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} final-metadata-len={} final-metadata-generation={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        state_lock_available,
        metadata_lock_available,
        final_metadata_len,
        final_metadata_generation,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-secondary-scheduler-service-loop: PASS");
    } else {
        crate::println!("qemu-secondary-scheduler-service-loop: FAIL");
    }

    reports_ok
}

#[cfg(talos_qemu_cross_core_ipi_delivery_smoke)]
pub fn run_cross_core_ipi_delivery_smoke() -> bool {
    smp::reset_secondary_core_states();
    CROSS_CORE_IPI_DELIVERY_STATE.reset();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        gic.enable_distributor();
    }

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_mask = ((1u64 << MAX_CORES) - 1) & !1;

    crate::println!(
        "qemu-cross-core-ipi-delivery: start conduit=smc cores={} sgi-intid={} expected-mask={:#x} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        QEMU_CROSS_CORE_IPI_SGI_INTID,
        expected_mask,
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
            "qemu-cross-core-ipi-delivery: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut ready_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while ready_remaining > 0
        && (CROSS_CORE_IPI_DELIVERY_STATE
            .ready_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        ready_remaining -= 1;
    }

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    for logical_cpu in 1..MAX_CORES {
        let target_bit = 1u8 << logical_cpu;
        let sgir_value =
            unsafe { gic.send_sgi_to_target_list(QEMU_CROSS_CORE_IPI_SGI_INTID, target_bit) };
        CROSS_CORE_IPI_DELIVERY_STATE.record_send(logical_cpu, target_bit, sgir_value);
        crate::println!(
            "qemu-cross-core-ipi-delivery: send sender=0 target-logical={} target-list-bit={:#04x} sgi-intid={} sgir={:#010x}",
            logical_cpu,
            target_bit,
            QEMU_CROSS_CORE_IPI_SGI_INTID,
            sgir_value
        );
    }

    let mut complete_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while complete_remaining > 0
        && (CROSS_CORE_IPI_DELIVERY_STATE
            .complete_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        complete_remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let ready_mask = CROSS_CORE_IPI_DELIVERY_STATE
        .ready_mask
        .load(Ordering::Acquire);
    let complete_mask = CROSS_CORE_IPI_DELIVERY_STATE
        .complete_mask
        .load(Ordering::Acquire);
    let mut participants = 0;
    let mut reports_ok =
        cpu_on_ok && boot_logical == Some(0) && (ready_mask & expected_mask) == expected_mask;

    for logical_cpu in 1..MAX_CORES {
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let target_bit =
            CROSS_CORE_IPI_DELIVERY_STATE.target_bits[logical_cpu].load(Ordering::Acquire);
        let sgir_value =
            CROSS_CORE_IPI_DELIVERY_STATE.sent_values[logical_cpu].load(Ordering::Acquire);
        let receive_count =
            CROSS_CORE_IPI_DELIVERY_STATE.receive_counts[logical_cpu].load(Ordering::Acquire);
        let eoi_count =
            CROSS_CORE_IPI_DELIVERY_STATE.eoi_counts[logical_cpu].load(Ordering::Acquire);
        let last_vector =
            CROSS_CORE_IPI_DELIVERY_STATE.last_vectors[logical_cpu].load(Ordering::Acquire);
        let last_iar = CROSS_CORE_IPI_DELIVERY_STATE.last_iars[logical_cpu].load(Ordering::Acquire);
        let last_intid =
            CROSS_CORE_IPI_DELIVERY_STATE.last_intids[logical_cpu].load(Ordering::Acquire);
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && target_bit == (1u64 << logical_cpu)
            && receive_count == 1
            && eoi_count == 1
            && last_intid == QEMU_CROSS_CORE_IPI_SGI_INTID as u64;
        if report_ok {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "qemu-cross-core-ipi-delivery: report sender=0 receiver={} state={} context={} mapped={:?} target-list-bit={:#04x} sgir={:#010x} vector={} iar={:#010x} intid={} receive-count={} eoi-count={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            target_bit,
            sgir_value,
            last_vector,
            last_iar,
            last_intid,
            receive_count,
            eoi_count,
            CROSS_CORE_IPI_DELIVERY_STATE.errors.load(Ordering::Acquire),
            report_ok
        );
    }

    let errors = CROSS_CORE_IPI_DELIVERY_STATE.errors.load(Ordering::Acquire);
    let classification = if reports_ok && errors == 0 {
        "qemu-cross-core-ipi-delivery-complete"
    } else if (ready_mask & expected_mask) != expected_mask {
        "qemu-cross-core-ipi-delivery-secondaries-not-ready"
    } else if cpu_on_ok {
        "qemu-cross-core-ipi-delivery-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-cross-core-ipi-delivery: final participants={} expected={} errors={} ready-mask={:#x} complete-mask={:#x} ready-wait-remaining={} complete-wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        ready_mask,
        complete_mask,
        ready_remaining,
        complete_remaining,
        classification
    );

    if reports_ok && errors == 0 {
        crate::println!("qemu-cross-core-ipi-delivery: PASS");
    } else {
        crate::println!("qemu-cross-core-ipi-delivery: FAIL");
    }

    reports_ok && errors == 0
}

#[cfg(talos_qemu_remote_wakeup_request_smoke)]
pub fn run_remote_wakeup_request_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_remote_wakeup_request_state();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        gic.enable_distributor();
    }

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_mask = ((1u64 << MAX_CORES) - 1) & !1;

    crate::println!(
        "qemu-remote-wakeup-request: start conduit=smc cores={} sgi-intid={} queue-capacity={} expected-mask={:#x} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        QEMU_CROSS_CORE_IPI_SGI_INTID,
        REMOTE_WAKE_QUEUE_CAPACITY,
        expected_mask,
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
            "qemu-remote-wakeup-request: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut ready_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while ready_remaining > 0
        && (REMOTE_WAKE_REQUEST_SMOKE_STATE
            .ready_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        ready_remaining -= 1;
    }

    let mut publish_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let task_id = TaskId::new(200 + logical_cpu as u64).expect("diagnostic task ID is nonzero");
        publish_ok &= publish_remote_wake_request(logical_cpu, task_id);
        if logical_cpu == 1 {
            publish_ok &= publish_remote_wake_request(logical_cpu, task_id);
        }
    }

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    for logical_cpu in 1..MAX_CORES {
        let target_bit = 1u8 << logical_cpu;
        let sgir_value =
            unsafe { gic.send_sgi_to_target_list(QEMU_CROSS_CORE_IPI_SGI_INTID, target_bit) };
        REMOTE_WAKE_REQUEST_SMOKE_STATE.record_send(logical_cpu, target_bit, sgir_value);
        crate::println!(
            "qemu-remote-wakeup-request: send sender=0 target-logical={} target-list-bit={:#04x} sgi-intid={} sgir={:#010x}",
            logical_cpu,
            target_bit,
            QEMU_CROSS_CORE_IPI_SGI_INTID,
            sgir_value
        );
    }

    let mut complete_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while complete_remaining > 0
        && (REMOTE_WAKE_REQUEST_SMOKE_STATE
            .complete_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        complete_remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let ready_mask = REMOTE_WAKE_REQUEST_SMOKE_STATE
        .ready_mask
        .load(Ordering::Acquire);
    let complete_mask = REMOTE_WAKE_REQUEST_SMOKE_STATE
        .complete_mask
        .load(Ordering::Acquire);
    let mut participants = 0;
    let mut reports_ok = cpu_on_ok
        && publish_ok
        && boot_logical == Some(0)
        && (ready_mask & expected_mask) == expected_mask;

    for logical_cpu in 1..MAX_CORES {
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_task = 200 + logical_cpu as u64;
        let target_bit =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.target_bits[logical_cpu].load(Ordering::Acquire);
        let sgir_value =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.sent_values[logical_cpu].load(Ordering::Acquire);
        let receive_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.receive_counts[logical_cpu].load(Ordering::Acquire);
        let eoi_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.eoi_counts[logical_cpu].load(Ordering::Acquire);
        let pending_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.pending_counts[logical_cpu].load(Ordering::Acquire);
        let consumed_task =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.consumed_task_ids[logical_cpu].load(Ordering::Acquire);
        let duplicate_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.duplicate_counts[logical_cpu].load(Ordering::Acquire);
        let queue_len_after =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.queue_lens_after[logical_cpu].load(Ordering::Acquire);
        let cross_owner_rejected = REMOTE_WAKE_REQUEST_SMOKE_STATE.cross_owner_rejections
            [logical_cpu]
            .load(Ordering::Acquire)
            == 1;
        let production_deferred = REMOTE_WAKE_REQUEST_SMOKE_STATE.production_deferrals[logical_cpu]
            .load(Ordering::Acquire)
            == 1;
        let last_vector =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.last_vectors[logical_cpu].load(Ordering::Acquire);
        let last_iar =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.last_iars[logical_cpu].load(Ordering::Acquire);
        let last_intid =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.last_intids[logical_cpu].load(Ordering::Acquire);
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && target_bit == (1u64 << logical_cpu)
            && receive_count == 1
            && eoi_count == 1
            && pending_count == 1
            && last_intid == QEMU_CROSS_CORE_IPI_SGI_INTID as u64
            && consumed_task == expected_task
            && queue_len_after == 0
            && cross_owner_rejected
            && production_deferred
            && (logical_cpu != 1 || duplicate_count == 1)
            && (logical_cpu == 1 || duplicate_count == 0);
        #[cfg(talos_qemu_remote_wake_to_local_runnable_smoke)]
        let report_ok = {
            let mut report_ok = report_ok;
            let local_wake_task = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_wake_task_ids[logical_cpu]
                .load(Ordering::Acquire);
            let local_runnable_len = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_runnable_lens
                [logical_cpu]
                .load(Ordering::Acquire);
            let local_state_before = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_before
                [logical_cpu]
                .load(Ordering::Acquire);
            let local_state_after = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_after[logical_cpu]
                .load(Ordering::Acquire);
            let duplicate_local_rejected = REMOTE_WAKE_REQUEST_SMOKE_STATE
                .duplicate_local_rejections[logical_cpu]
                .load(Ordering::Acquire)
                == 1;
            report_ok &= local_wake_task == expected_task
                && local_runnable_len == 1
                && task_state_name(local_state_before) == "blocked"
                && task_state_name(local_state_after) == "runnable"
                && duplicate_local_rejected;
            crate::println!(
                "qemu-remote-wake-to-local-runnable: local receiver={} state-before={} state-after={} woke-task={} local-runnable-len={} duplicate-local-rejected={} ok={}",
                logical_cpu,
                task_state_name(local_state_before),
                task_state_name(local_state_after),
                local_wake_task,
                local_runnable_len,
                duplicate_local_rejected,
                report_ok
            );
            report_ok
        };
        if report_ok {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "qemu-remote-wakeup-request: report sender=0 receiver={} state={} context={} mapped={:?} target-list-bit={:#04x} sgir={:#010x} vector={} iar={:#010x} intid={} receive-count={} eoi-count={} pending-count={} consumed-task={} duplicate-count={} queue-len-after={} cross-owner-rejected={} production-deferred={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            target_bit,
            sgir_value,
            last_vector,
            last_iar,
            last_intid,
            receive_count,
            eoi_count,
            pending_count,
            consumed_task,
            duplicate_count,
            queue_len_after,
            cross_owner_rejected,
            production_deferred,
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .load(Ordering::Acquire),
            report_ok
        );
    }

    let errors = REMOTE_WAKE_REQUEST_SMOKE_STATE
        .errors
        .load(Ordering::Acquire);
    let classification = if reports_ok && errors == 0 {
        if cfg!(talos_qemu_remote_wake_to_local_runnable_smoke) {
            "qemu-remote-wake-to-local-runnable-complete"
        } else {
            "qemu-remote-wakeup-request-complete"
        }
    } else if (ready_mask & expected_mask) != expected_mask {
        "qemu-remote-wakeup-request-secondaries-not-ready"
    } else if cpu_on_ok {
        "qemu-remote-wakeup-request-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-remote-wakeup-request: final participants={} expected={} errors={} ready-mask={:#x} complete-mask={:#x} ready-wait-remaining={} complete-wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        ready_mask,
        complete_mask,
        ready_remaining,
        complete_remaining,
        classification
    );

    if reports_ok && errors == 0 {
        crate::println!("qemu-remote-wakeup-request: PASS");
    } else {
        crate::println!("qemu-remote-wakeup-request: FAIL");
    }

    reports_ok && errors == 0
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

    #[cfg(talos_qemu_cross_core_ipi_delivery_smoke)]
    if intid == QEMU_CROSS_CORE_IPI_SGI_INTID {
        let logical_cpu = current_qemu_logical_cpu();
        CROSS_CORE_IPI_DELIVERY_STATE.record_receive(logical_cpu, vector, iar, intid);
        unsafe {
            gic.end_interrupt(iar);
        }
        CROSS_CORE_IPI_DELIVERY_STATE.record_eoi(logical_cpu);
        return true;
    }

    #[cfg(talos_qemu_remote_wakeup_request_smoke)]
    if intid == QEMU_CROSS_CORE_IPI_SGI_INTID {
        let logical_cpu = current_qemu_logical_cpu();
        REMOTE_WAKE_REQUEST_SMOKE_STATE.record_receive(logical_cpu, vector, iar, intid);
        unsafe {
            gic.end_interrupt(iar);
        }
        REMOTE_WAKE_REQUEST_SMOKE_STATE.record_eoi(logical_cpu);
        return true;
    }

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
