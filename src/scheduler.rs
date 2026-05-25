//! Scheduler data structures.
//!
//! This module owns the first boot-CPU scheduler shape plus the Phase 6.3
//! CPU-local ownership wrapper. Task identifiers remain scheduler-local,
//! runnable queues are still owned by exactly one logical CPU, and the wrapper
//! does not add migration, shared queues, or secondary-core production
//! dispatch. The remote wake-request queue is a bounded signal mailbox only:
//! it records target-owned wake intent without mutating another CPU's local
//! runnable queue.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogicalCpuId(usize);

impl LogicalCpuId {
    pub const BOOT: Self = Self(0);

    pub const fn new(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskId(u64);

impl TaskId {
    pub const fn new(raw: u64) -> Option<Self> {
        if raw == 0 { None } else { Some(Self(raw)) }
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskState {
    Running,
    Runnable,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessOwnerId(u64);

impl ProcessOwnerId {
    pub const fn new(raw: u64) -> Option<Self> {
        if raw == 0 { None } else { Some(Self(raw)) }
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelStack {
    base: usize,
    size: usize,
}

impl KernelStack {
    pub const fn new(base: usize, size: usize) -> Option<Self> {
        if size == 0 || base.checked_add(size).is_none() {
            None
        } else {
            Some(Self { base, size })
        }
    }

    pub const fn base(self) -> usize {
        self.base
    }

    pub const fn size(self) -> usize {
        self.size
    }

    pub const fn limit(self) -> usize {
        self.base + self.size
    }

    pub const fn contains(self, address: usize) -> bool {
        self.base <= address && address < self.limit()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ContextFrame {
    x19: usize,
    x20: usize,
    x21: usize,
    x22: usize,
    x23: usize,
    x24: usize,
    x25: usize,
    x26: usize,
    x27: usize,
    x28: usize,
    x29: usize,
    link_register: usize,
    stack_pointer: usize,
}

impl ContextFrame {
    pub const fn new(stack_pointer: usize, program_counter: usize) -> Self {
        Self {
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            link_register: program_counter,
            stack_pointer,
        }
    }

    pub const fn kernel_thread_bootstrap(
        stack_pointer: usize,
        trampoline: usize,
        entry: usize,
        argument: usize,
    ) -> Self {
        Self {
            x19: argument,
            x20: entry,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            link_register: trampoline,
            stack_pointer,
        }
    }

    pub const fn stack_pointer(self) -> usize {
        self.stack_pointer
    }

    pub const fn program_counter(self) -> usize {
        self.link_register
    }

    pub const fn bootstrap_argument(self) -> usize {
        self.x19
    }

    pub const fn bootstrap_entry(self) -> usize {
        self.x20
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Task {
    id: TaskId,
    state: TaskState,
    kernel_stack: KernelStack,
    context: ContextFrame,
    process_owner: Option<ProcessOwnerId>,
}

impl Task {
    pub const fn kernel_thread(
        id: TaskId,
        kernel_stack: KernelStack,
        context: ContextFrame,
    ) -> Self {
        Self {
            id,
            state: TaskState::Runnable,
            kernel_stack,
            context,
            process_owner: None,
        }
    }

    pub const fn id(self) -> TaskId {
        self.id
    }

    pub const fn state(self) -> TaskState {
        self.state
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub const fn kernel_stack(self) -> KernelStack {
        self.kernel_stack
    }

    pub const fn context(self) -> ContextFrame {
        self.context
    }

    pub const fn process_owner(self) -> Option<ProcessOwnerId> {
        self.process_owner
    }

    pub fn attach_process_owner(&mut self, process_owner: ProcessOwnerId) {
        self.process_owner = Some(process_owner);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunnableQueueError {
    Full,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoluntaryYieldError {
    CurrentTaskNotRunning,
    NoRunnableTask,
    RunnableQueueFull,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerPreemptError {
    CurrentTaskNotRunning,
    NoRunnableTask,
    RunnableQueueFull,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulerCoreRole {
    BootCpuProduction,
    SecondaryDeferred,
}

impl SchedulerCoreRole {
    pub const fn production_dispatch_enabled(self) -> bool {
        matches!(self, Self::BootCpuProduction)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerCoreSchedulerAccessError {
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    ProductionDispatchDeferred {
        owner: LogicalCpuId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoteWakeRequestError {
    Full,
    SelfTarget {
        target: LogicalCpuId,
    },
    WrongTarget {
        owner: LogicalCpuId,
        target: LogicalCpuId,
    },
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoteWakePublishOutcome {
    Inserted,
    Duplicate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetWakeConsumptionError {
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    WrongTarget {
        owner: LogicalCpuId,
        target: LogicalCpuId,
    },
    TaskMismatch {
        requested: TaskId,
        local: TaskId,
    },
    TaskNotBlocked {
        task_id: TaskId,
        state: TaskState,
    },
    DuplicateLocalRunnable {
        task_id: TaskId,
    },
    RunnableQueueFull {
        task_id: TaskId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemoteWakeRequest {
    requester: LogicalCpuId,
    target: LogicalCpuId,
    task_id: TaskId,
}

impl RemoteWakeRequest {
    pub const fn requester(self) -> LogicalCpuId {
        self.requester
    }

    pub const fn target(self) -> LogicalCpuId {
        self.target
    }

    pub const fn task_id(self) -> TaskId {
        self.task_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteWakeQueue<const CAPACITY: usize> {
    owner: LogicalCpuId,
    entries: [Option<RemoteWakeRequest>; CAPACITY],
    len: usize,
    duplicate_count: u64,
}

impl<const CAPACITY: usize> RemoteWakeQueue<CAPACITY> {
    pub const fn new(owner: LogicalCpuId) -> Self {
        Self {
            owner,
            entries: [None; CAPACITY],
            len: 0,
            duplicate_count: 0,
        }
    }

    pub const fn owner(&self) -> LogicalCpuId {
        self.owner
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn duplicate_count(&self) -> u64 {
        self.duplicate_count
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == CAPACITY
    }

    pub fn publish(
        &mut self,
        requester: LogicalCpuId,
        target: LogicalCpuId,
        task_id: TaskId,
    ) -> Result<RemoteWakePublishOutcome, RemoteWakeRequestError> {
        if target != self.owner {
            return Err(RemoteWakeRequestError::WrongTarget {
                owner: self.owner,
                target,
            });
        }
        if requester == self.owner {
            return Err(RemoteWakeRequestError::SelfTarget { target });
        }
        if self.contains(task_id) {
            self.duplicate_count += 1;
            return Ok(RemoteWakePublishOutcome::Duplicate);
        }
        if self.is_full() {
            return Err(RemoteWakeRequestError::Full);
        }

        self.entries[self.len] = Some(RemoteWakeRequest {
            requester,
            target,
            task_id,
        });
        self.len += 1;
        Ok(RemoteWakePublishOutcome::Inserted)
    }

    pub fn consume_next(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<Option<RemoteWakeRequest>, RemoteWakeRequestError> {
        if requester != self.owner {
            return Err(RemoteWakeRequestError::WrongOwner {
                owner: self.owner,
                requester,
            });
        }
        if self.is_empty() {
            return Ok(None);
        }

        let request = self.entries[0].take();
        let mut index = 1;
        while index < self.len {
            self.entries[index - 1] = self.entries[index].take();
            index += 1;
        }
        self.len -= 1;
        if self.len < CAPACITY {
            self.entries[self.len] = None;
        }
        Ok(request)
    }

    pub fn contains(&self, task_id: TaskId) -> bool {
        let mut index = 0;
        while index < self.len {
            if let Some(request) = self.entries[index]
                && request.task_id == task_id
            {
                return true;
            }
            index += 1;
        }
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunnableQueue<const CAPACITY: usize> {
    entries: [Option<TaskId>; CAPACITY],
    head: usize,
    len: usize,
}

impl<const CAPACITY: usize> RunnableQueue<CAPACITY> {
    pub const fn new() -> Self {
        Self {
            entries: [None; CAPACITY],
            head: 0,
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        CAPACITY
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == CAPACITY
    }

    pub fn enqueue(&mut self, task_id: TaskId) -> Result<(), RunnableQueueError> {
        if self.is_full() {
            return Err(RunnableQueueError::Full);
        }

        let tail = self.index_from_head(self.len);
        self.entries[tail] = Some(task_id);
        self.len += 1;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<TaskId> {
        if self.is_empty() {
            return None;
        }

        let task_id = self.entries[self.head].take();
        self.head = self.index_from_head(1);
        self.len -= 1;
        if self.len == 0 {
            self.head = 0;
        }
        task_id
    }

    pub fn front(&self) -> Option<TaskId> {
        if self.is_empty() {
            None
        } else {
            self.entries[self.head]
        }
    }

    pub fn contains(&self, task_id: TaskId) -> bool {
        let mut offset = 0;
        while offset < self.len {
            if self.entries[self.index_from_head(offset)] == Some(task_id) {
                return true;
            }
            offset += 1;
        }
        false
    }

    const fn index_from_head(&self, offset: usize) -> usize {
        if CAPACITY == 0 {
            0
        } else {
            (self.head + offset) % CAPACITY
        }
    }
}

impl<const CAPACITY: usize> Default for RunnableQueue<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchedulerCounters {
    state_transitions: u64,
    voluntary_yields: u64,
    timer_preemptions: u64,
    context_switches: u64,
}

impl SchedulerCounters {
    pub const fn state_transitions(self) -> u64 {
        self.state_transitions
    }

    pub const fn voluntary_yields(self) -> u64 {
        self.voluntary_yields
    }

    pub const fn timer_preemptions(self) -> u64 {
        self.timer_preemptions
    }

    pub const fn context_switches(self) -> u64 {
        self.context_switches
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SingleCoreScheduler<const RUNNABLE_CAPACITY: usize> {
    runnable: RunnableQueue<RUNNABLE_CAPACITY>,
    counters: SchedulerCounters,
}

impl<const RUNNABLE_CAPACITY: usize> SingleCoreScheduler<RUNNABLE_CAPACITY> {
    pub const fn new() -> Self {
        Self {
            runnable: RunnableQueue::new(),
            counters: SchedulerCounters {
                state_transitions: 0,
                voluntary_yields: 0,
                timer_preemptions: 0,
                context_switches: 0,
            },
        }
    }

    pub const fn runnable(&self) -> &RunnableQueue<RUNNABLE_CAPACITY> {
        &self.runnable
    }

    pub const fn counters(&self) -> SchedulerCounters {
        self.counters
    }

    pub fn make_runnable(&mut self, task: &mut Task) -> Result<(), RunnableQueueError> {
        task.set_state(TaskState::Runnable);
        self.counters.state_transitions += 1;
        self.runnable.enqueue(task.id())
    }

    pub fn pick_next(&mut self) -> Option<TaskId> {
        self.runnable.dequeue()
    }

    pub fn voluntary_yield(&mut self, current: &mut Task) -> Result<TaskId, VoluntaryYieldError> {
        if current.state() != TaskState::Running {
            return Err(VoluntaryYieldError::CurrentTaskNotRunning);
        }
        if self.runnable.is_empty() {
            return Err(VoluntaryYieldError::NoRunnableTask);
        }
        if self.runnable.is_full() {
            return Err(VoluntaryYieldError::RunnableQueueFull);
        }

        current.set_state(TaskState::Runnable);
        self.counters.state_transitions += 1;
        self.runnable
            .enqueue(current.id())
            .map_err(|_| VoluntaryYieldError::RunnableQueueFull)?;

        let next = self
            .runnable
            .dequeue()
            .expect("non-empty runnable queue after preflight");
        self.counters.voluntary_yields += 1;
        self.counters.context_switches += 1;
        Ok(next)
    }

    pub fn timer_preempt(&mut self, current: &mut Task) -> Result<TaskId, TimerPreemptError> {
        if current.state() != TaskState::Running {
            return Err(TimerPreemptError::CurrentTaskNotRunning);
        }
        if self.runnable.is_empty() {
            return Err(TimerPreemptError::NoRunnableTask);
        }
        if self.runnable.is_full() {
            return Err(TimerPreemptError::RunnableQueueFull);
        }

        current.set_state(TaskState::Runnable);
        self.counters.state_transitions += 1;
        self.runnable
            .enqueue(current.id())
            .map_err(|_| TimerPreemptError::RunnableQueueFull)?;

        let next = self
            .runnable
            .dequeue()
            .expect("non-empty runnable queue after preflight");
        self.counters.timer_preemptions += 1;
        self.counters.context_switches += 1;
        Ok(next)
    }
}

impl<const RUNNABLE_CAPACITY: usize> Default for SingleCoreScheduler<RUNNABLE_CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerCoreScheduler<const RUNNABLE_CAPACITY: usize> {
    owner: LogicalCpuId,
    role: SchedulerCoreRole,
    current_task: Option<TaskId>,
    scheduler: SingleCoreScheduler<RUNNABLE_CAPACITY>,
}

impl<const RUNNABLE_CAPACITY: usize> PerCoreScheduler<RUNNABLE_CAPACITY> {
    pub const fn new(owner: LogicalCpuId, role: SchedulerCoreRole) -> Self {
        Self {
            owner,
            role,
            current_task: None,
            scheduler: SingleCoreScheduler::new(),
        }
    }

    pub const fn boot_cpu() -> Self {
        Self::new(LogicalCpuId::BOOT, SchedulerCoreRole::BootCpuProduction)
    }

    pub const fn deferred_secondary(owner: LogicalCpuId) -> Self {
        Self::new(owner, SchedulerCoreRole::SecondaryDeferred)
    }

    pub const fn owner(&self) -> LogicalCpuId {
        self.owner
    }

    pub const fn role(&self) -> SchedulerCoreRole {
        self.role
    }

    pub const fn production_dispatch_enabled(&self) -> bool {
        self.role.production_dispatch_enabled()
    }

    pub const fn current_task(&self) -> Option<TaskId> {
        self.current_task
    }

    pub const fn scheduler(&self) -> &SingleCoreScheduler<RUNNABLE_CAPACITY> {
        &self.scheduler
    }

    pub fn local_scheduler_mut(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<&mut SingleCoreScheduler<RUNNABLE_CAPACITY>, PerCoreSchedulerAccessError> {
        self.ensure_local_owner(requester)?;
        Ok(&mut self.scheduler)
    }

    pub fn production_scheduler_mut(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<&mut SingleCoreScheduler<RUNNABLE_CAPACITY>, PerCoreSchedulerAccessError> {
        self.ensure_production_owner(requester)?;
        Ok(&mut self.scheduler)
    }

    pub fn set_current_task(
        &mut self,
        requester: LogicalCpuId,
        task_id: TaskId,
    ) -> Result<(), PerCoreSchedulerAccessError> {
        self.ensure_production_owner(requester)?;
        self.current_task = Some(task_id);
        Ok(())
    }

    pub fn clear_current_task(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<Option<TaskId>, PerCoreSchedulerAccessError> {
        self.ensure_production_owner(requester)?;
        Ok(self.current_task.take())
    }

    pub fn wake_blocked_local_task_from_remote_request(
        &mut self,
        requester: LogicalCpuId,
        request: RemoteWakeRequest,
        task: &mut Task,
    ) -> Result<TaskId, TargetWakeConsumptionError> {
        self.ensure_local_owner(requester)
            .map_err(|error| match error {
                PerCoreSchedulerAccessError::WrongOwner { owner, requester } => {
                    TargetWakeConsumptionError::WrongOwner { owner, requester }
                }
                PerCoreSchedulerAccessError::ProductionDispatchDeferred { owner } => {
                    TargetWakeConsumptionError::WrongOwner { owner, requester }
                }
            })?;
        if request.target() != self.owner {
            return Err(TargetWakeConsumptionError::WrongTarget {
                owner: self.owner,
                target: request.target(),
            });
        }
        if request.task_id() != task.id() {
            return Err(TargetWakeConsumptionError::TaskMismatch {
                requested: request.task_id(),
                local: task.id(),
            });
        }
        if self.scheduler.runnable().contains(task.id()) {
            return Err(TargetWakeConsumptionError::DuplicateLocalRunnable { task_id: task.id() });
        }
        if task.state() != TaskState::Blocked {
            return Err(TargetWakeConsumptionError::TaskNotBlocked {
                task_id: task.id(),
                state: task.state(),
            });
        }

        let task_id = task.id();
        self.scheduler
            .make_runnable(task)
            .map_err(|_| TargetWakeConsumptionError::RunnableQueueFull { task_id })?;
        Ok(task_id)
    }

    fn ensure_local_owner(
        &self,
        requester: LogicalCpuId,
    ) -> Result<(), PerCoreSchedulerAccessError> {
        if requester == self.owner {
            Ok(())
        } else {
            Err(PerCoreSchedulerAccessError::WrongOwner {
                owner: self.owner,
                requester,
            })
        }
    }

    fn ensure_production_owner(
        &self,
        requester: LogicalCpuId,
    ) -> Result<(), PerCoreSchedulerAccessError> {
        self.ensure_local_owner(requester)?;
        if self.production_dispatch_enabled() {
            Ok(())
        } else {
            Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred { owner: self.owner })
        }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{offset_of, size_of};

    use super::{
        ContextFrame, KernelStack, LogicalCpuId, PerCoreScheduler, PerCoreSchedulerAccessError,
        ProcessOwnerId, RemoteWakePublishOutcome, RemoteWakeQueue, RemoteWakeRequest,
        RemoteWakeRequestError, RunnableQueue, RunnableQueueError, SchedulerCoreRole,
        SingleCoreScheduler, TargetWakeConsumptionError, Task, TaskId, TaskState,
        TimerPreemptError, VoluntaryYieldError,
    };

    fn task_id(raw: u64) -> TaskId {
        TaskId::new(raw).expect("nonzero task id")
    }

    fn kernel_stack() -> KernelStack {
        KernelStack::new(0x8000, 0x1000).expect("valid kernel stack")
    }

    fn context() -> ContextFrame {
        ContextFrame::new(0x8ff0, 0x4000)
    }

    #[test_case]
    fn task_id_rejects_zero_and_preserves_scheduler_local_value() {
        assert_eq!(TaskId::new(0), None);
        assert_eq!(task_id(7).raw(), 7);
    }

    #[test_case]
    fn logical_cpu_id_preserves_cpu_local_owner_identity() {
        assert_eq!(LogicalCpuId::BOOT.raw(), 0);
        assert_eq!(LogicalCpuId::new(3).raw(), 3);
    }

    #[test_case]
    fn kernel_stack_records_bounds_without_owning_process_resources() {
        let stack = kernel_stack();

        assert_eq!(stack.base(), 0x8000);
        assert_eq!(stack.size(), 0x1000);
        assert_eq!(stack.limit(), 0x9000);
        assert!(stack.contains(0x8000));
        assert!(stack.contains(0x8fff));
        assert!(!stack.contains(0x9000));
        assert_eq!(KernelStack::new(usize::MAX, 2), None);
        assert_eq!(KernelStack::new(0x1000, 0), None);
    }

    #[test_case]
    fn kernel_thread_starts_runnable_without_process_owner() {
        let task = Task::kernel_thread(task_id(1), kernel_stack(), context());

        assert_eq!(task.id(), task_id(1));
        assert_eq!(task.state(), TaskState::Runnable);
        assert_eq!(task.kernel_stack(), kernel_stack());
        assert_eq!(task.context(), context());
        assert_eq!(task.context().stack_pointer(), 0x8ff0);
        assert_eq!(task.context().program_counter(), 0x4000);
        assert_eq!(task.process_owner(), None);
    }

    #[test_case]
    fn process_owner_is_explicit_future_extension_point() {
        let mut task = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let owner = ProcessOwnerId::new(42).expect("nonzero process owner id");

        task.attach_process_owner(owner);

        assert_eq!(task.process_owner(), Some(owner));
        assert_eq!(owner.raw(), 42);
        assert_eq!(ProcessOwnerId::new(0), None);
    }

    #[test_case]
    fn runnable_queue_dequeues_in_fifo_order_and_resets_when_empty() {
        let mut queue = RunnableQueue::<3>::new();

        assert_eq!(queue.capacity(), 3);
        assert!(queue.is_empty());
        queue.enqueue(task_id(1)).expect("enqueue task 1");
        queue.enqueue(task_id(2)).expect("enqueue task 2");

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.front(), Some(task_id(1)));
        assert!(queue.contains(task_id(1)));
        assert_eq!(queue.dequeue(), Some(task_id(1)));
        assert_eq!(queue.front(), Some(task_id(2)));
        assert_eq!(queue.dequeue(), Some(task_id(2)));
        assert_eq!(queue.front(), None);
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());

        queue
            .enqueue(task_id(3))
            .expect("enqueue after empty reset");
        assert_eq!(queue.dequeue(), Some(task_id(3)));
    }

    #[test_case]
    fn runnable_queue_wraps_tail_and_rejects_overflow() {
        let mut queue = RunnableQueue::<2>::new();

        queue.enqueue(task_id(1)).expect("enqueue task 1");
        queue.enqueue(task_id(2)).expect("enqueue task 2");
        assert_eq!(queue.enqueue(task_id(3)), Err(RunnableQueueError::Full));
        assert_eq!(queue.dequeue(), Some(task_id(1)));
        queue.enqueue(task_id(3)).expect("enqueue wrapped tail");

        assert_eq!(queue.dequeue(), Some(task_id(2)));
        assert_eq!(queue.dequeue(), Some(task_id(3)));
        assert_eq!(queue.dequeue(), None);
    }

    #[test_case]
    fn scheduler_tracks_state_transition_when_making_task_runnable() {
        let mut scheduler = SingleCoreScheduler::<2>::new();
        let mut task = Task::kernel_thread(task_id(1), kernel_stack(), context());
        task.set_state(TaskState::Blocked);

        scheduler
            .make_runnable(&mut task)
            .expect("make task runnable");

        assert_eq!(task.state(), TaskState::Runnable);
        assert_eq!(scheduler.counters().state_transitions(), 1);
        assert!(scheduler.runnable().contains(task_id(1)));
        assert_eq!(scheduler.pick_next(), Some(task_id(1)));
    }

    #[test_case]
    fn voluntary_yield_requeues_current_and_counts_dispatch_switch() {
        let mut scheduler = SingleCoreScheduler::<2>::new();
        let mut current = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(2), kernel_stack(), context());
        current.set_state(TaskState::Running);

        scheduler
            .make_runnable(&mut next)
            .expect("make next runnable");

        let next_id = scheduler
            .voluntary_yield(&mut current)
            .expect("yield to runnable task");

        assert_eq!(next_id, task_id(2));
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(scheduler.runnable().front(), Some(task_id(1)));
        assert_eq!(scheduler.counters().voluntary_yields(), 1);
        assert_eq!(scheduler.counters().context_switches(), 1);
        assert_eq!(scheduler.counters().state_transitions(), 2);
    }

    #[test_case]
    fn timer_preempt_requeues_current_without_counting_voluntary_yield() {
        let mut scheduler = SingleCoreScheduler::<2>::new();
        let mut current = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(2), kernel_stack(), context());
        current.set_state(TaskState::Running);

        scheduler
            .make_runnable(&mut next)
            .expect("make next runnable");

        let next_id = scheduler
            .timer_preempt(&mut current)
            .expect("timer preempts to runnable task");

        assert_eq!(next_id, task_id(2));
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(scheduler.runnable().front(), Some(task_id(1)));
        assert_eq!(scheduler.counters().voluntary_yields(), 0);
        assert_eq!(scheduler.counters().timer_preemptions(), 1);
        assert_eq!(scheduler.counters().context_switches(), 1);
        assert_eq!(scheduler.counters().state_transitions(), 2);
    }

    #[test_case]
    fn voluntary_yield_rejects_non_running_or_empty_dispatch() {
        let mut scheduler = SingleCoreScheduler::<1>::new();
        let mut current = Task::kernel_thread(task_id(1), kernel_stack(), context());

        assert_eq!(
            scheduler.voluntary_yield(&mut current),
            Err(VoluntaryYieldError::CurrentTaskNotRunning)
        );

        current.set_state(TaskState::Running);
        assert_eq!(
            scheduler.voluntary_yield(&mut current),
            Err(VoluntaryYieldError::NoRunnableTask)
        );
        assert_eq!(
            scheduler.timer_preempt(&mut current),
            Err(TimerPreemptError::NoRunnableTask)
        );
    }

    #[test_case]
    fn task_can_be_marked_running_without_context_switch_implementation() {
        let mut task = Task::kernel_thread(task_id(2), kernel_stack(), context());

        task.set_state(TaskState::Running);

        assert_eq!(task.state(), TaskState::Running);
    }

    #[test_case]
    fn per_core_scheduler_records_boot_cpu_owner_and_current_task() {
        let mut per_core = PerCoreScheduler::<2>::boot_cpu();

        assert_eq!(per_core.owner(), LogicalCpuId::BOOT);
        assert_eq!(per_core.role(), SchedulerCoreRole::BootCpuProduction);
        assert!(per_core.production_dispatch_enabled());
        assert_eq!(per_core.current_task(), None);

        per_core
            .set_current_task(LogicalCpuId::BOOT, task_id(1))
            .expect("boot CPU owns production scheduler state");

        assert_eq!(per_core.current_task(), Some(task_id(1)));
        assert_eq!(
            per_core
                .clear_current_task(LogicalCpuId::BOOT)
                .expect("boot CPU clears current task"),
            Some(task_id(1))
        );
        assert_eq!(per_core.current_task(), None);
    }

    #[test_case]
    fn per_core_scheduler_rejects_cross_owner_queue_mutation() {
        let mut per_core = PerCoreScheduler::<2>::boot_cpu();
        let requester = LogicalCpuId::new(1);

        assert_eq!(
            per_core.local_scheduler_mut(requester).unwrap_err(),
            PerCoreSchedulerAccessError::WrongOwner {
                owner: LogicalCpuId::BOOT,
                requester
            }
        );
        assert_eq!(per_core.scheduler().runnable().len(), 0);
    }

    #[test_case]
    fn per_core_scheduler_keeps_secondary_dispatch_deferred() {
        let mut per_core = PerCoreScheduler::<2>::deferred_secondary(LogicalCpuId::new(1));

        assert_eq!(per_core.owner(), LogicalCpuId::new(1));
        assert_eq!(per_core.role(), SchedulerCoreRole::SecondaryDeferred);
        assert!(!per_core.production_dispatch_enabled());
        assert_eq!(
            per_core.production_scheduler_mut(LogicalCpuId::new(1)),
            Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred {
                owner: LogicalCpuId::new(1)
            })
        );
        assert_eq!(
            per_core.set_current_task(LogicalCpuId::new(1), task_id(2)),
            Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred {
                owner: LogicalCpuId::new(1)
            })
        );
    }

    #[test_case]
    fn per_core_scheduler_local_queue_keeps_single_core_invariants() {
        let mut per_core = PerCoreScheduler::<2>::boot_cpu();
        let mut current = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(2), kernel_stack(), context());
        current.set_state(TaskState::Running);

        per_core
            .set_current_task(LogicalCpuId::BOOT, current.id())
            .expect("boot CPU sets current task");
        let scheduler = per_core
            .production_scheduler_mut(LogicalCpuId::BOOT)
            .expect("boot CPU mutates its local production scheduler");
        scheduler
            .make_runnable(&mut next)
            .expect("local queue has capacity");
        let next_id = scheduler
            .timer_preempt(&mut current)
            .expect("local timer preempts to runnable task");

        assert_eq!(next_id, task_id(2));
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(scheduler.runnable().front(), Some(task_id(1)));
        assert_eq!(scheduler.counters().timer_preemptions(), 1);
        assert_eq!(scheduler.counters().context_switches(), 1);
    }

    #[test_case]
    fn remote_wake_queue_coalesces_duplicate_task_requests() {
        let mut queue = RemoteWakeQueue::<2>::new(LogicalCpuId::new(1));
        let requester = LogicalCpuId::BOOT;
        let target = LogicalCpuId::new(1);
        let task = task_id(7);

        assert_eq!(
            queue.publish(requester, target, task),
            Ok(RemoteWakePublishOutcome::Inserted)
        );
        assert_eq!(
            queue.publish(requester, target, task),
            Ok(RemoteWakePublishOutcome::Duplicate)
        );

        assert_eq!(queue.owner(), target);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.duplicate_count(), 1);
        let request = queue
            .consume_next(target)
            .expect("target owner can consume")
            .expect("request is pending");
        assert_eq!(request.requester(), requester);
        assert_eq!(request.target(), target);
        assert_eq!(request.task_id(), task);
        assert!(queue.is_empty());
    }

    #[test_case]
    fn remote_wake_queue_rejects_self_and_wrong_target_publication() {
        let mut queue = RemoteWakeQueue::<2>::new(LogicalCpuId::new(1));

        assert_eq!(
            queue.publish(LogicalCpuId::new(1), LogicalCpuId::new(1), task_id(1)),
            Err(RemoteWakeRequestError::SelfTarget {
                target: LogicalCpuId::new(1)
            })
        );
        assert_eq!(
            queue.publish(LogicalCpuId::BOOT, LogicalCpuId::new(2), task_id(1)),
            Err(RemoteWakeRequestError::WrongTarget {
                owner: LogicalCpuId::new(1),
                target: LogicalCpuId::new(2)
            })
        );
    }

    #[test_case]
    fn remote_wake_queue_rejects_overflow_and_cross_owner_consumption() {
        let mut queue = RemoteWakeQueue::<1>::new(LogicalCpuId::new(2));

        queue
            .publish(LogicalCpuId::BOOT, LogicalCpuId::new(2), task_id(1))
            .expect("first request fits");

        assert_eq!(
            queue.publish(LogicalCpuId::BOOT, LogicalCpuId::new(2), task_id(2)),
            Err(RemoteWakeRequestError::Full)
        );
        assert_eq!(
            queue.consume_next(LogicalCpuId::new(1)),
            Err(RemoteWakeRequestError::WrongOwner {
                owner: LogicalCpuId::new(2),
                requester: LogicalCpuId::new(1)
            })
        );
        assert_eq!(
            queue
                .consume_next(LogicalCpuId::new(2))
                .expect("target owner can consume")
                .map(|request| request.task_id()),
            Some(task_id(1))
        );
    }

    #[test_case]
    fn target_owned_wake_consumption_makes_only_local_blocked_task_runnable() {
        let target = LogicalCpuId::new(2);
        let mut queue = RemoteWakeQueue::<2>::new(target);
        let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(target);
        let mut task = Task::kernel_thread(task_id(9), kernel_stack(), context());
        task.set_state(TaskState::Blocked);

        queue
            .publish(LogicalCpuId::BOOT, target, task_id(9))
            .expect("request publication succeeds");
        let request = queue
            .consume_next(target)
            .expect("target owns queue")
            .expect("request is pending");
        assert_eq!(queue.len(), 0);

        assert_eq!(
            scheduler.wake_blocked_local_task_from_remote_request(target, request, &mut task),
            Ok(task_id(9))
        );
        assert_eq!(task.state(), TaskState::Runnable);
        assert_eq!(scheduler.scheduler().runnable().front(), Some(task_id(9)));
        assert_eq!(scheduler.scheduler().counters().state_transitions(), 1);
    }

    #[test_case]
    fn target_owned_wake_consumption_rejects_cross_owner_and_nonlocal_tasks() {
        let target = LogicalCpuId::new(1);
        let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(target);
        let mut task = Task::kernel_thread(task_id(10), kernel_stack(), context());
        task.set_state(TaskState::Blocked);
        let request = RemoteWakeRequest {
            requester: LogicalCpuId::BOOT,
            target,
            task_id: task_id(10),
        };

        assert_eq!(
            scheduler.wake_blocked_local_task_from_remote_request(
                LogicalCpuId::new(2),
                request,
                &mut task
            ),
            Err(TargetWakeConsumptionError::WrongOwner {
                owner: target,
                requester: LogicalCpuId::new(2)
            })
        );
        assert_eq!(task.state(), TaskState::Blocked);
        assert_eq!(scheduler.scheduler().runnable().len(), 0);

        let wrong_task_request = RemoteWakeRequest {
            requester: LogicalCpuId::BOOT,
            target,
            task_id: task_id(11),
        };
        assert_eq!(
            scheduler.wake_blocked_local_task_from_remote_request(
                target,
                wrong_task_request,
                &mut task
            ),
            Err(TargetWakeConsumptionError::TaskMismatch {
                requested: task_id(11),
                local: task_id(10)
            })
        );
        assert_eq!(task.state(), TaskState::Blocked);
        assert_eq!(scheduler.scheduler().runnable().len(), 0);
    }

    #[test_case]
    fn target_owned_wake_consumption_rejects_duplicate_local_enqueue() {
        let target = LogicalCpuId::new(3);
        let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(target);
        let mut task = Task::kernel_thread(task_id(12), kernel_stack(), context());
        task.set_state(TaskState::Blocked);
        let request = RemoteWakeRequest {
            requester: LogicalCpuId::BOOT,
            target,
            task_id: task_id(12),
        };

        scheduler
            .wake_blocked_local_task_from_remote_request(target, request, &mut task)
            .expect("first target-owned wake succeeds");
        assert_eq!(
            scheduler.wake_blocked_local_task_from_remote_request(target, request, &mut task),
            Err(TargetWakeConsumptionError::DuplicateLocalRunnable {
                task_id: task_id(12)
            })
        );
        assert_eq!(task.state(), TaskState::Runnable);
        assert_eq!(scheduler.scheduler().runnable().len(), 1);
    }

    #[test_case]
    fn context_frame_bootstrap_records_thread_entry_and_argument() {
        let context = ContextFrame::kernel_thread_bootstrap(0x8ff0, 0x4000, 0x5000, 7);

        assert_eq!(context.stack_pointer(), 0x8ff0);
        assert_eq!(context.program_counter(), 0x4000);
        assert_eq!(context.bootstrap_entry(), 0x5000);
        assert_eq!(context.bootstrap_argument(), 7);
    }

    #[test_case]
    fn context_frame_layout_matches_aarch64_switch_offsets() {
        assert_eq!(offset_of!(ContextFrame, x19), 0);
        assert_eq!(offset_of!(ContextFrame, x20), 8);
        assert_eq!(offset_of!(ContextFrame, x21), 16);
        assert_eq!(offset_of!(ContextFrame, x22), 24);
        assert_eq!(offset_of!(ContextFrame, x23), 32);
        assert_eq!(offset_of!(ContextFrame, x24), 40);
        assert_eq!(offset_of!(ContextFrame, x25), 48);
        assert_eq!(offset_of!(ContextFrame, x26), 56);
        assert_eq!(offset_of!(ContextFrame, x27), 64);
        assert_eq!(offset_of!(ContextFrame, x28), 72);
        assert_eq!(offset_of!(ContextFrame, x29), 80);
        assert_eq!(offset_of!(ContextFrame, link_register), 88);
        assert_eq!(offset_of!(ContextFrame, stack_pointer), 96);
        assert_eq!(size_of::<ContextFrame>(), 104);
    }
}
