//! Single-core scheduler data structures.
//!
//! This module owns only the first boot-CPU scheduler shape: scheduler-local
//! task identifiers, kernel-thread state, kernel stack descriptors, saved
//! context placeholders, and a fixed runnable queue. It does not switch
//! contexts, sleep tasks, implement preemption, or create process resources.

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
}

impl SchedulerCounters {
    pub const fn state_transitions(self) -> u64 {
        self.state_transitions
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
}

impl<const RUNNABLE_CAPACITY: usize> Default for SingleCoreScheduler<RUNNABLE_CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{offset_of, size_of};

    use super::{
        ContextFrame, KernelStack, ProcessOwnerId, RunnableQueue, RunnableQueueError,
        SingleCoreScheduler, Task, TaskId, TaskState,
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
        assert!(queue.contains(task_id(1)));
        assert_eq!(queue.dequeue(), Some(task_id(1)));
        assert_eq!(queue.dequeue(), Some(task_id(2)));
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
    fn task_can_be_marked_running_without_context_switch_implementation() {
        let mut task = Task::kernel_thread(task_id(2), kernel_stack(), context());

        task.set_state(TaskState::Running);

        assert_eq!(task.state(), TaskState::Running);
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
