//! Scheduler data structures.
//!
//! This module owns the first boot-CPU scheduler shape plus the Phase 6.3
//! CPU-local ownership wrapper. Task identifiers remain scheduler-local,
//! runnable queues are still owned by exactly one logical CPU, and secondary
//! production dispatch is limited to an explicit diagnostic role. The first
//! shared run queue is a bounded owner-transfer surface only: it removes a
//! runnable task from its source-local queue, publishes a complete handoff
//! entry, and lets the destination owner consume that entry into its own local
//! queue. The remote wake-request queue remains a bounded signal mailbox only:
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
pub enum PreemptionRecordOutcome {
    Inserted,
    Coalesced,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerCorePreemptionStateError {
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    OwnerMismatch {
        state_owner: LogicalCpuId,
        scheduler_owner: LogicalCpuId,
    },
    ServiceDeferred {
        owner: LogicalCpuId,
        disable_depth: u8,
    },
    DisableDepthOverflow {
        owner: LogicalCpuId,
    },
    DisableDepthUnderflow {
        owner: LogicalCpuId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerCorePreemptionCounters {
    recorded_requests: u64,
    coalesced_requests: u64,
    serviced_requests: u64,
    deferred_while_disabled: u64,
}

impl PerCorePreemptionCounters {
    pub const fn recorded_requests(self) -> u64 {
        self.recorded_requests
    }

    pub const fn coalesced_requests(self) -> u64 {
        self.coalesced_requests
    }

    pub const fn serviced_requests(self) -> u64 {
        self.serviced_requests
    }

    pub const fn deferred_while_disabled(self) -> u64 {
        self.deferred_while_disabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerCorePreemptionState {
    owner: LogicalCpuId,
    pending_timer_request: bool,
    disable_depth: u8,
    counters: PerCorePreemptionCounters,
}

impl PerCorePreemptionState {
    pub const fn new(owner: LogicalCpuId) -> Self {
        Self {
            owner,
            pending_timer_request: false,
            disable_depth: 0,
            counters: PerCorePreemptionCounters {
                recorded_requests: 0,
                coalesced_requests: 0,
                serviced_requests: 0,
                deferred_while_disabled: 0,
            },
        }
    }

    pub const fn owner(&self) -> LogicalCpuId {
        self.owner
    }

    pub const fn pending_timer_request(&self) -> bool {
        self.pending_timer_request
    }

    pub const fn disable_depth(&self) -> u8 {
        self.disable_depth
    }

    pub const fn counters(&self) -> PerCorePreemptionCounters {
        self.counters
    }

    /// Bounded timer-IRQ side recording hook.
    ///
    /// This method deliberately records only local pending state. It does not
    /// inspect runnable queues, choose a next task, refresh metadata, or mutate
    /// scheduler-owned current-task state.
    pub fn record_local_timer_irq(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<PreemptionRecordOutcome, PerCorePreemptionStateError> {
        self.ensure_owner(requester)?;
        if self.pending_timer_request {
            self.counters.coalesced_requests += 1;
            Ok(PreemptionRecordOutcome::Coalesced)
        } else {
            self.pending_timer_request = true;
            self.counters.recorded_requests += 1;
            Ok(PreemptionRecordOutcome::Inserted)
        }
    }

    pub fn enter_preemption_disabled(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<u8, PerCorePreemptionStateError> {
        self.ensure_owner(requester)?;
        self.disable_depth = self
            .disable_depth
            .checked_add(1)
            .ok_or(PerCorePreemptionStateError::DisableDepthOverflow { owner: self.owner })?;
        Ok(self.disable_depth)
    }

    pub fn exit_preemption_disabled(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<u8, PerCorePreemptionStateError> {
        self.ensure_owner(requester)?;
        self.disable_depth = self
            .disable_depth
            .checked_sub(1)
            .ok_or(PerCorePreemptionStateError::DisableDepthUnderflow { owner: self.owner })?;
        Ok(self.disable_depth)
    }

    fn prepare_owner_local_service(
        &mut self,
        requester: LogicalCpuId,
        scheduler_owner: LogicalCpuId,
    ) -> Result<bool, PerCorePreemptionStateError> {
        self.ensure_owner(requester)?;
        if scheduler_owner != self.owner {
            return Err(PerCorePreemptionStateError::OwnerMismatch {
                state_owner: self.owner,
                scheduler_owner,
            });
        }
        if self.pending_timer_request && self.disable_depth != 0 {
            self.counters.deferred_while_disabled += 1;
            return Err(PerCorePreemptionStateError::ServiceDeferred {
                owner: self.owner,
                disable_depth: self.disable_depth,
            });
        }
        Ok(self.pending_timer_request)
    }

    fn complete_owner_local_service(&mut self) {
        self.pending_timer_request = false;
        self.counters.serviced_requests += 1;
    }

    fn ensure_owner(&self, requester: LogicalCpuId) -> Result<(), PerCorePreemptionStateError> {
        if requester == self.owner {
            Ok(())
        } else {
            Err(PerCorePreemptionStateError::WrongOwner {
                owner: self.owner,
                requester,
            })
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionDispatchError {
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    ProductionDispatchDeferred {
        owner: LogicalCpuId,
    },
    NoRunnableTask,
    SelectedTaskMismatch {
        queued: TaskId,
        provided: TaskId,
    },
    TaskNotRunnable {
        task_id: TaskId,
        state: TaskState,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulerCoreRole {
    BootCpuProduction,
    SecondaryDeferred,
    SecondaryProductionDiagnostic,
}

impl SchedulerCoreRole {
    pub const fn production_dispatch_enabled(self) -> bool {
        matches!(
            self,
            Self::BootCpuProduction | Self::SecondaryProductionDiagnostic
        )
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
pub enum MigrationState {
    OwnerLocal,
    MigrationReserved,
    SharedQueued,
    DestinationEnqueued,
    MigrationRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedRunQueueError {
    InvalidSourceOwner {
        source_owner: LogicalCpuId,
        cpu_capacity: usize,
    },
    InvalidDestinationOwner {
        destination_owner: LogicalCpuId,
        cpu_capacity: usize,
    },
    SameOwner {
        owner: LogicalCpuId,
    },
    WrongSourceOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    WrongDestinationOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    MetadataOwnerMismatch {
        task_id: TaskId,
        metadata_owner: LogicalCpuId,
        expected_owner: LogicalCpuId,
    },
    UnknownTask {
        task_id: TaskId,
    },
    StaleMetadataGeneration {
        task_id: TaskId,
        expected_generation: u64,
        actual_generation: u64,
    },
    DuplicateLocalRunnable {
        task_id: TaskId,
        owner: LogicalCpuId,
    },
    DuplicateSharedQueued {
        task_id: TaskId,
    },
    Full,
    SourceTaskNotQueued {
        task_id: TaskId,
        owner: LogicalCpuId,
    },
    SourceTaskNotRunnable {
        task_id: TaskId,
        state: TaskState,
    },
    RunningTaskMigrationDeferred {
        task_id: TaskId,
    },
    BlockedTaskMigrationUnsupported {
        task_id: TaskId,
    },
    DestinationLocalQueueFull {
        task_id: TaskId,
    },
    DeferredSecondaryRole {
        owner: LogicalCpuId,
    },
    TaskMismatch {
        queued: TaskId,
        provided: TaskId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SharedRunQueueEntry {
    task_id: TaskId,
    source_owner: LogicalCpuId,
    destination_owner: LogicalCpuId,
    metadata_generation: u64,
    state: MigrationState,
}

impl SharedRunQueueEntry {
    pub const fn task_id(self) -> TaskId {
        self.task_id
    }

    pub const fn source_owner(self) -> LogicalCpuId {
        self.source_owner
    }

    pub const fn destination_owner(self) -> LogicalCpuId {
        self.destination_owner
    }

    pub const fn metadata_generation(self) -> u64 {
        self.metadata_generation
    }

    pub const fn state(self) -> MigrationState {
        self.state
    }

    const fn with_state(self, state: MigrationState) -> Self {
        Self { state, ..self }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SharedRunQueuePublishReport {
    reserved: SharedRunQueueEntry,
    queued: SharedRunQueueEntry,
}

impl SharedRunQueuePublishReport {
    pub const fn reserved(self) -> SharedRunQueueEntry {
        self.reserved
    }

    pub const fn queued(self) -> SharedRunQueueEntry {
        self.queued
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SharedRunQueueConsumeReport {
    queued: SharedRunQueueEntry,
    destination_enqueued: SharedRunQueueEntry,
    metadata: SchedulerTaskSnapshot,
}

impl SharedRunQueueConsumeReport {
    pub const fn queued(self) -> SharedRunQueueEntry {
        self.queued
    }

    pub const fn destination_enqueued(self) -> SharedRunQueueEntry {
        self.destination_enqueued
    }

    pub const fn metadata(self) -> SchedulerTaskSnapshot {
        self.metadata
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadBalancingPlan {
    task_id: TaskId,
    source_owner: LogicalCpuId,
    destination_owner: LogicalCpuId,
    metadata_generation: u64,
}

impl LoadBalancingPlan {
    pub const fn task_id(self) -> TaskId {
        self.task_id
    }

    pub const fn source_owner(self) -> LogicalCpuId {
        self.source_owner
    }

    pub const fn destination_owner(self) -> LogicalCpuId {
        self.destination_owner
    }

    pub const fn metadata_generation(self) -> u64 {
        self.metadata_generation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadBalancingPublishReport {
    plan: LoadBalancingPlan,
    migration: SharedRunQueuePublishReport,
}

impl LoadBalancingPublishReport {
    pub const fn plan(self) -> LoadBalancingPlan {
        self.plan
    }

    pub const fn migration(self) -> SharedRunQueuePublishReport {
        self.migration
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadBalancingPolicyError {
    InvalidSourceOwner {
        source_owner: LogicalCpuId,
        cpu_capacity: usize,
    },
    InvalidDestinationOwner {
        destination_owner: LogicalCpuId,
        cpu_capacity: usize,
    },
    SameOwner {
        owner: LogicalCpuId,
    },
    WrongSourceOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    DestinationRoleIneligible {
        owner: LogicalCpuId,
        role: SchedulerCoreRole,
    },
    NoSourceCandidate {
        source_owner: LogicalCpuId,
    },
    SourceCandidateMismatch {
        selected: TaskId,
        provided: TaskId,
    },
    SourceCandidateUnknown {
        task_id: TaskId,
    },
    SourceCandidateOwnerMismatch {
        task_id: TaskId,
        metadata_owner: LogicalCpuId,
        expected_owner: LogicalCpuId,
    },
    SourceCandidateRunning {
        task_id: TaskId,
    },
    SourceCandidateBlocked {
        task_id: TaskId,
    },
    SourceCandidateNotRunnable {
        task_id: TaskId,
        state: TaskState,
    },
    SourceCandidateNotQueued {
        task_id: TaskId,
        owner: LogicalCpuId,
    },
    DestinationLocalQueueFull {
        destination_owner: LogicalCpuId,
        task_id: TaskId,
    },
    SharedRunQueueFull,
    DuplicateSharedQueued {
        task_id: TaskId,
    },
    MigrationRejected(SharedRunQueueError),
}

pub struct LoadBalancingPolicy;

impl LoadBalancingPolicy {
    pub fn plan_front_runnable<
        const SOURCE_RUNNABLE_CAPACITY: usize,
        const DESTINATION_RUNNABLE_CAPACITY: usize,
        const TASK_CAPACITY: usize,
        const SHARED_CAPACITY: usize,
        const CPU_CAPACITY: usize,
    >(
        requester: LogicalCpuId,
        source_scheduler: &PerCoreScheduler<SOURCE_RUNNABLE_CAPACITY>,
        destination_scheduler: &PerCoreScheduler<DESTINATION_RUNNABLE_CAPACITY>,
        metadata: &SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        shared_queue: &SharedRunQueue<SHARED_CAPACITY, CPU_CAPACITY>,
    ) -> Result<LoadBalancingPlan, LoadBalancingPolicyError> {
        let source_owner = source_scheduler.owner();
        let destination_owner = destination_scheduler.owner();
        Self::ensure_valid_source::<CPU_CAPACITY>(source_owner)?;
        Self::ensure_valid_destination::<CPU_CAPACITY>(destination_owner)?;
        if source_owner == destination_owner {
            return Err(LoadBalancingPolicyError::SameOwner {
                owner: source_owner,
            });
        }
        if requester != source_owner {
            return Err(LoadBalancingPolicyError::WrongSourceOwner {
                owner: source_owner,
                requester,
            });
        }
        if !destination_scheduler.production_dispatch_enabled() {
            return Err(LoadBalancingPolicyError::DestinationRoleIneligible {
                owner: destination_owner,
                role: destination_scheduler.role(),
            });
        }

        let candidate = source_scheduler
            .scheduler()
            .runnable()
            .front()
            .ok_or(LoadBalancingPolicyError::NoSourceCandidate { source_owner })?;
        if source_scheduler.current_task() == Some(candidate) {
            return Err(LoadBalancingPolicyError::SourceCandidateRunning { task_id: candidate });
        }
        let snapshot = metadata
            .lookup_task(candidate)
            .map_err(|_| LoadBalancingPolicyError::SourceCandidateUnknown { task_id: candidate })?;
        Self::ensure_snapshot_matches_source(snapshot, source_owner)?;
        if destination_scheduler.scheduler().runnable().is_full() {
            return Err(LoadBalancingPolicyError::DestinationLocalQueueFull {
                destination_owner,
                task_id: candidate,
            });
        }
        if shared_queue.contains(candidate) {
            return Err(LoadBalancingPolicyError::DuplicateSharedQueued { task_id: candidate });
        }
        if shared_queue.is_full() {
            return Err(LoadBalancingPolicyError::SharedRunQueueFull);
        }

        Ok(LoadBalancingPlan {
            task_id: candidate,
            source_owner,
            destination_owner,
            metadata_generation: snapshot.generation(),
        })
    }

    pub fn publish_plan<
        const SOURCE_RUNNABLE_CAPACITY: usize,
        const TASK_CAPACITY: usize,
        const SHARED_CAPACITY: usize,
        const CPU_CAPACITY: usize,
    >(
        requester: LogicalCpuId,
        source_scheduler: &mut PerCoreScheduler<SOURCE_RUNNABLE_CAPACITY>,
        metadata: &SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        shared_queue: &mut SharedRunQueue<SHARED_CAPACITY, CPU_CAPACITY>,
        task: &Task,
        plan: LoadBalancingPlan,
    ) -> Result<LoadBalancingPublishReport, LoadBalancingPolicyError> {
        Self::ensure_valid_source::<CPU_CAPACITY>(plan.source_owner)?;
        Self::ensure_valid_destination::<CPU_CAPACITY>(plan.destination_owner)?;
        if requester != plan.source_owner {
            return Err(LoadBalancingPolicyError::WrongSourceOwner {
                owner: plan.source_owner,
                requester,
            });
        }
        if source_scheduler.owner() != plan.source_owner {
            return Err(LoadBalancingPolicyError::WrongSourceOwner {
                owner: source_scheduler.owner(),
                requester: plan.source_owner,
            });
        }
        if task.id() != plan.task_id {
            return Err(LoadBalancingPolicyError::SourceCandidateMismatch {
                selected: plan.task_id,
                provided: task.id(),
            });
        }

        let migration = shared_queue
            .publish_migration(
                requester,
                source_scheduler,
                plan.destination_owner,
                metadata,
                task,
                plan.metadata_generation,
            )
            .map_err(LoadBalancingPolicyError::MigrationRejected)?;
        Ok(LoadBalancingPublishReport { plan, migration })
    }

    pub fn publish_front_runnable<
        const SOURCE_RUNNABLE_CAPACITY: usize,
        const DESTINATION_RUNNABLE_CAPACITY: usize,
        const TASK_CAPACITY: usize,
        const SHARED_CAPACITY: usize,
        const CPU_CAPACITY: usize,
    >(
        requester: LogicalCpuId,
        source_scheduler: &mut PerCoreScheduler<SOURCE_RUNNABLE_CAPACITY>,
        destination_scheduler: &PerCoreScheduler<DESTINATION_RUNNABLE_CAPACITY>,
        metadata: &SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        shared_queue: &mut SharedRunQueue<SHARED_CAPACITY, CPU_CAPACITY>,
        task: &Task,
    ) -> Result<LoadBalancingPublishReport, LoadBalancingPolicyError> {
        let plan = Self::plan_front_runnable(
            requester,
            source_scheduler,
            destination_scheduler,
            metadata,
            shared_queue,
        )?;
        Self::publish_plan(
            requester,
            source_scheduler,
            metadata,
            shared_queue,
            task,
            plan,
        )
    }

    fn ensure_valid_source<const CPU_CAPACITY: usize>(
        source_owner: LogicalCpuId,
    ) -> Result<(), LoadBalancingPolicyError> {
        if source_owner.raw() < CPU_CAPACITY {
            Ok(())
        } else {
            Err(LoadBalancingPolicyError::InvalidSourceOwner {
                source_owner,
                cpu_capacity: CPU_CAPACITY,
            })
        }
    }

    fn ensure_valid_destination<const CPU_CAPACITY: usize>(
        destination_owner: LogicalCpuId,
    ) -> Result<(), LoadBalancingPolicyError> {
        if destination_owner.raw() < CPU_CAPACITY {
            Ok(())
        } else {
            Err(LoadBalancingPolicyError::InvalidDestinationOwner {
                destination_owner,
                cpu_capacity: CPU_CAPACITY,
            })
        }
    }

    fn ensure_snapshot_matches_source(
        snapshot: SchedulerTaskSnapshot,
        source_owner: LogicalCpuId,
    ) -> Result<(), LoadBalancingPolicyError> {
        let task_id = snapshot.task_id();
        if snapshot.owner() != source_owner {
            return Err(LoadBalancingPolicyError::SourceCandidateOwnerMismatch {
                task_id,
                metadata_owner: snapshot.owner(),
                expected_owner: source_owner,
            });
        }
        if snapshot.state() == TaskState::Running || snapshot.current_on_owner() {
            return Err(LoadBalancingPolicyError::SourceCandidateRunning { task_id });
        }
        if snapshot.state() == TaskState::Blocked {
            return Err(LoadBalancingPolicyError::SourceCandidateBlocked { task_id });
        }
        if snapshot.state() != TaskState::Runnable {
            return Err(LoadBalancingPolicyError::SourceCandidateNotRunnable {
                task_id,
                state: snapshot.state(),
            });
        }
        if !snapshot.runnable_on_owner() {
            return Err(LoadBalancingPolicyError::SourceCandidateNotQueued {
                task_id,
                owner: source_owner,
            });
        }
        Ok(())
    }
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
pub struct SharedRunQueue<const CAPACITY: usize, const CPU_CAPACITY: usize> {
    entries: [Option<SharedRunQueueEntry>; CAPACITY],
    len: usize,
    rejected_count: u64,
}

/// SMP-protected shared run-queue boundary.
///
/// Callers must use the accepted local-IRQ-save then SMP-lock ordering before
/// mutating this queue on AArch64. The queue itself stays target-independent
/// and does not mask IRQs internally.
pub type SharedRunQueueLock<const CAPACITY: usize, const CPU_CAPACITY: usize> =
    crate::smp_sync::SpinLock<SharedRunQueue<CAPACITY, CPU_CAPACITY>>;

impl<const CAPACITY: usize, const CPU_CAPACITY: usize> SharedRunQueue<CAPACITY, CPU_CAPACITY> {
    pub const fn new() -> Self {
        Self {
            entries: [None; CAPACITY],
            len: 0,
            rejected_count: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn capacity(&self) -> usize {
        CAPACITY
    }

    pub const fn rejected_count(&self) -> u64 {
        self.rejected_count
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == CAPACITY
    }

    pub fn publish_migration<const RUNNABLE_CAPACITY: usize, const TASK_CAPACITY: usize>(
        &mut self,
        requester: LogicalCpuId,
        source_scheduler: &mut PerCoreScheduler<RUNNABLE_CAPACITY>,
        destination_owner: LogicalCpuId,
        metadata: &SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        task: &Task,
        expected_generation: u64,
    ) -> Result<SharedRunQueuePublishReport, SharedRunQueueError> {
        let source_owner = source_scheduler.owner();
        self.ensure_valid_source(source_owner)?;
        self.ensure_valid_destination(destination_owner)?;
        if source_owner == destination_owner {
            return self.reject(SharedRunQueueError::SameOwner {
                owner: source_owner,
            });
        }
        if requester != source_owner {
            return self.reject(SharedRunQueueError::WrongSourceOwner {
                owner: source_owner,
                requester,
            });
        }
        self.ensure_task_is_migratable_from_source(source_scheduler, task)?;
        self.ensure_metadata_matches_source(
            metadata,
            source_owner,
            task.id(),
            expected_generation,
        )?;
        if self.contains(task.id()) {
            return self.reject(SharedRunQueueError::DuplicateSharedQueued { task_id: task.id() });
        }
        if self.is_full() {
            return self.reject(SharedRunQueueError::Full);
        }

        if !source_scheduler.scheduler.runnable.remove(task.id()) {
            return self.reject(SharedRunQueueError::SourceTaskNotQueued {
                task_id: task.id(),
                owner: source_owner,
            });
        }

        let reserved = SharedRunQueueEntry {
            task_id: task.id(),
            source_owner,
            destination_owner,
            metadata_generation: expected_generation,
            state: MigrationState::MigrationReserved,
        };
        let queued = reserved.with_state(MigrationState::SharedQueued);
        self.entries[self.len] = Some(queued);
        self.len += 1;
        Ok(SharedRunQueuePublishReport { reserved, queued })
    }

    pub fn consume_for_destination<const RUNNABLE_CAPACITY: usize, const TASK_CAPACITY: usize>(
        &mut self,
        requester: LogicalCpuId,
        destination_scheduler: &mut PerCoreScheduler<RUNNABLE_CAPACITY>,
        metadata: &mut SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        task: &mut Task,
    ) -> Result<Option<SharedRunQueueConsumeReport>, SharedRunQueueError> {
        let destination_owner = destination_scheduler.owner();
        self.ensure_valid_destination(destination_owner)?;
        if requester != destination_owner {
            return self.reject(SharedRunQueueError::WrongDestinationOwner {
                owner: destination_owner,
                requester,
            });
        }
        if !destination_scheduler.production_dispatch_enabled() {
            return self.reject(SharedRunQueueError::DeferredSecondaryRole {
                owner: destination_owner,
            });
        }
        let Some(index) = self.find_destination_index(destination_owner) else {
            return Ok(None);
        };
        let queued = self.entries[index].expect("destination index points at a queued entry");
        if queued.task_id != task.id() {
            return self.reject(SharedRunQueueError::TaskMismatch {
                queued: queued.task_id,
                provided: task.id(),
            });
        }
        if task.state() == TaskState::Running {
            return self
                .reject(SharedRunQueueError::RunningTaskMigrationDeferred { task_id: task.id() });
        }
        if task.state() == TaskState::Blocked {
            return self.reject(SharedRunQueueError::BlockedTaskMigrationUnsupported {
                task_id: task.id(),
            });
        }
        if task.state() != TaskState::Runnable {
            return self.reject(SharedRunQueueError::SourceTaskNotRunnable {
                task_id: task.id(),
                state: task.state(),
            });
        }
        self.ensure_metadata_matches_source(
            metadata,
            queued.source_owner,
            queued.task_id,
            queued.metadata_generation,
        )?;
        if destination_scheduler.scheduler.runnable.contains(task.id()) {
            return self.reject(SharedRunQueueError::DuplicateLocalRunnable {
                task_id: task.id(),
                owner: destination_owner,
            });
        }
        if destination_scheduler.scheduler.runnable.is_full() {
            return self
                .reject(SharedRunQueueError::DestinationLocalQueueFull { task_id: task.id() });
        }

        destination_scheduler
            .production_scheduler_mut(requester)
            .map_err(|error| match error {
                PerCoreSchedulerAccessError::WrongOwner { owner, requester } => {
                    SharedRunQueueError::WrongDestinationOwner { owner, requester }
                }
                PerCoreSchedulerAccessError::ProductionDispatchDeferred { owner } => {
                    SharedRunQueueError::DeferredSecondaryRole { owner }
                }
            })?
            .make_runnable(task)
            .map_err(|_| SharedRunQueueError::DestinationLocalQueueFull { task_id: task.id() })?;

        let metadata_snapshot = metadata.refresh_migrated_task(
            requester,
            destination_scheduler,
            task,
            queued.source_owner,
            queued.metadata_generation,
        )?;
        self.remove_index(index);
        let destination_enqueued = queued.with_state(MigrationState::DestinationEnqueued);
        Ok(Some(SharedRunQueueConsumeReport {
            queued,
            destination_enqueued,
            metadata: metadata_snapshot,
        }))
    }

    pub fn contains(&self, task_id: TaskId) -> bool {
        let mut index = 0;
        while index < self.len {
            if let Some(entry) = self.entries[index]
                && entry.task_id == task_id
            {
                return true;
            }
            index += 1;
        }
        false
    }

    pub fn front(&self) -> Option<SharedRunQueueEntry> {
        if self.is_empty() {
            None
        } else {
            self.entries[0]
        }
    }

    fn reject<T>(&mut self, error: SharedRunQueueError) -> Result<T, SharedRunQueueError> {
        self.rejected_count += 1;
        Err(error)
    }

    fn ensure_valid_source(
        &mut self,
        source_owner: LogicalCpuId,
    ) -> Result<(), SharedRunQueueError> {
        if source_owner.raw() < CPU_CAPACITY {
            Ok(())
        } else {
            self.reject(SharedRunQueueError::InvalidSourceOwner {
                source_owner,
                cpu_capacity: CPU_CAPACITY,
            })
        }
    }

    fn ensure_valid_destination(
        &mut self,
        destination_owner: LogicalCpuId,
    ) -> Result<(), SharedRunQueueError> {
        if destination_owner.raw() < CPU_CAPACITY {
            Ok(())
        } else {
            self.reject(SharedRunQueueError::InvalidDestinationOwner {
                destination_owner,
                cpu_capacity: CPU_CAPACITY,
            })
        }
    }

    fn ensure_task_is_migratable_from_source<const RUNNABLE_CAPACITY: usize>(
        &mut self,
        source_scheduler: &PerCoreScheduler<RUNNABLE_CAPACITY>,
        task: &Task,
    ) -> Result<(), SharedRunQueueError> {
        if source_scheduler.current_task() == Some(task.id()) || task.state() == TaskState::Running
        {
            return self
                .reject(SharedRunQueueError::RunningTaskMigrationDeferred { task_id: task.id() });
        }
        if task.state() == TaskState::Blocked {
            return self.reject(SharedRunQueueError::BlockedTaskMigrationUnsupported {
                task_id: task.id(),
            });
        }
        if task.state() != TaskState::Runnable {
            return self.reject(SharedRunQueueError::SourceTaskNotRunnable {
                task_id: task.id(),
                state: task.state(),
            });
        }
        if !source_scheduler.scheduler.runnable.contains(task.id()) {
            return self.reject(SharedRunQueueError::SourceTaskNotQueued {
                task_id: task.id(),
                owner: source_scheduler.owner(),
            });
        }
        Ok(())
    }

    fn ensure_metadata_matches_source<const TASK_CAPACITY: usize>(
        &mut self,
        metadata: &SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        source_owner: LogicalCpuId,
        task_id: TaskId,
        expected_generation: u64,
    ) -> Result<(), SharedRunQueueError> {
        let snapshot = metadata.find_task(task_id).ok_or_else(|| {
            self.rejected_count += 1;
            SharedRunQueueError::UnknownTask { task_id }
        })?;
        if snapshot.generation() != expected_generation {
            return self.reject(SharedRunQueueError::StaleMetadataGeneration {
                task_id,
                expected_generation,
                actual_generation: snapshot.generation(),
            });
        }
        if snapshot.owner() != source_owner {
            return self.reject(SharedRunQueueError::MetadataOwnerMismatch {
                task_id,
                metadata_owner: snapshot.owner(),
                expected_owner: source_owner,
            });
        }
        if snapshot.state() == TaskState::Running || snapshot.current_on_owner() {
            return self.reject(SharedRunQueueError::RunningTaskMigrationDeferred { task_id });
        }
        if snapshot.state() == TaskState::Blocked {
            return self.reject(SharedRunQueueError::BlockedTaskMigrationUnsupported { task_id });
        }
        if snapshot.state() != TaskState::Runnable {
            return self.reject(SharedRunQueueError::SourceTaskNotRunnable {
                task_id,
                state: snapshot.state(),
            });
        }
        if !snapshot.runnable_on_owner() {
            return self.reject(SharedRunQueueError::SourceTaskNotQueued {
                task_id,
                owner: source_owner,
            });
        }
        Ok(())
    }

    fn find_destination_index(&self, destination_owner: LogicalCpuId) -> Option<usize> {
        let mut index = 0;
        while index < self.len {
            if let Some(entry) = self.entries[index]
                && entry.destination_owner == destination_owner
            {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn remove_index(&mut self, index: usize) {
        let mut cursor = index + 1;
        while cursor < self.len {
            self.entries[cursor - 1] = self.entries[cursor].take();
            cursor += 1;
        }
        self.len -= 1;
        if self.len < CAPACITY {
            self.entries[self.len] = None;
        }
    }
}

impl<const CAPACITY: usize, const CPU_CAPACITY: usize> Default
    for SharedRunQueue<CAPACITY, CPU_CAPACITY>
{
    fn default() -> Self {
        Self::new()
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

    pub fn remove(&mut self, task_id: TaskId) -> bool {
        let mut found = false;
        let mut offset = 0;
        while offset < self.len {
            let index = self.index_from_head(offset);
            if !found && self.entries[index] == Some(task_id) {
                found = true;
            }
            if found && offset + 1 < self.len {
                let next = self.index_from_head(offset + 1);
                self.entries[index] = self.entries[next].take();
            }
            offset += 1;
        }

        if found {
            self.len -= 1;
            if self.len == 0 {
                self.head = 0;
            } else {
                let tail = self.index_from_head(self.len);
                self.entries[tail] = None;
            }
        }
        found
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
    production_dispatches: u64,
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

    pub const fn production_dispatches(self) -> u64 {
        self.production_dispatches
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
                production_dispatches: 0,
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
        if self.runnable.is_full() {
            return Err(RunnableQueueError::Full);
        }

        task.set_state(TaskState::Runnable);
        self.counters.state_transitions += 1;
        self.runnable.enqueue(task.id())
    }

    pub fn pick_next(&mut self) -> Option<TaskId> {
        self.runnable.dequeue()
    }

    pub fn dispatch_next_production_task(
        &mut self,
        next_task: &mut Task,
    ) -> Result<TaskId, ProductionDispatchError> {
        let queued = self
            .runnable
            .front()
            .ok_or(ProductionDispatchError::NoRunnableTask)?;
        if queued != next_task.id() {
            return Err(ProductionDispatchError::SelectedTaskMismatch {
                queued,
                provided: next_task.id(),
            });
        }
        if next_task.state() != TaskState::Runnable {
            return Err(ProductionDispatchError::TaskNotRunnable {
                task_id: next_task.id(),
                state: next_task.state(),
            });
        }

        let task_id = self
            .runnable
            .dequeue()
            .expect("front task is present after preflight");
        next_task.set_state(TaskState::Running);
        self.counters.state_transitions += 1;
        self.counters.context_switches += 1;
        self.counters.production_dispatches += 1;
        Ok(task_id)
    }

    pub fn voluntary_yield(&mut self, current: &mut Task) -> Result<TaskId, VoluntaryYieldError> {
        if current.state() != TaskState::Running {
            return Err(VoluntaryYieldError::CurrentTaskNotRunning);
        }
        let next = self
            .runnable
            .dequeue()
            .ok_or(VoluntaryYieldError::NoRunnableTask)?;

        current.set_state(TaskState::Runnable);
        self.counters.state_transitions += 1;
        self.runnable
            .enqueue(current.id())
            .map_err(|_| VoluntaryYieldError::RunnableQueueFull)?;

        self.counters.voluntary_yields += 1;
        self.counters.context_switches += 1;
        Ok(next)
    }

    pub fn timer_preempt(&mut self, current: &mut Task) -> Result<TaskId, TimerPreemptError> {
        if current.state() != TaskState::Running {
            return Err(TimerPreemptError::CurrentTaskNotRunning);
        }
        let next = self
            .runnable
            .dequeue()
            .ok_or(TimerPreemptError::NoRunnableTask)?;

        current.set_state(TaskState::Runnable);
        self.counters.state_transitions += 1;
        self.runnable
            .enqueue(current.id())
            .map_err(|_| TimerPreemptError::RunnableQueueFull)?;

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

    pub const fn production_secondary_diagnostic(owner: LogicalCpuId) -> Self {
        Self::new(owner, SchedulerCoreRole::SecondaryProductionDiagnostic)
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

    pub fn dispatch_cpu_local_diagnostic_task(
        &mut self,
        requester: LogicalCpuId,
        task: &mut Task,
    ) -> Result<TaskId, ProductionDispatchError> {
        self.ensure_production_owner(requester)
            .map_err(ProductionDispatchError::from)?;

        let task_id = self.scheduler.dispatch_next_production_task(task)?;
        self.current_task = Some(task_id);
        Ok(task_id)
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchedulerTaskSnapshot {
    task_id: TaskId,
    owner: LogicalCpuId,
    state: TaskState,
    process_owner: Option<ProcessOwnerId>,
    kernel_stack: KernelStack,
    current_on_owner: bool,
    runnable_on_owner: bool,
    generation: u64,
}

impl SchedulerTaskSnapshot {
    pub const fn task_id(self) -> TaskId {
        self.task_id
    }

    pub const fn owner(self) -> LogicalCpuId {
        self.owner
    }

    pub const fn state(self) -> TaskState {
        self.state
    }

    pub const fn process_owner(self) -> Option<ProcessOwnerId> {
        self.process_owner
    }

    pub const fn kernel_stack(self) -> KernelStack {
        self.kernel_stack
    }

    pub const fn current_on_owner(self) -> bool {
        self.current_on_owner
    }

    pub const fn runnable_on_owner(self) -> bool {
        self.runnable_on_owner
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedSchedulerMetadataError {
    InvalidOwner {
        owner: LogicalCpuId,
        cpu_capacity: usize,
    },
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    Full,
    DuplicateTask {
        task_id: TaskId,
        existing_owner: LogicalCpuId,
        attempted_owner: LogicalCpuId,
    },
    UnknownTask {
        task_id: TaskId,
    },
    StaleSnapshot {
        task_id: TaskId,
        expected_generation: u64,
        actual_generation: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CpuLocalSchedulerServiceError {
    RemoteWakeQueue(RemoteWakeRequestError),
    RemoteWake(TargetWakeConsumptionError),
    PreemptionState(PerCorePreemptionStateError),
    MissingCurrentTaskForTimerPreemption,
    CurrentTaskMismatch {
        current_task: Option<TaskId>,
        provided: TaskId,
    },
    TimerPreempt(TimerPreemptError),
    ProductionDispatch(ProductionDispatchError),
    Metadata(SharedSchedulerMetadataError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionTimerPreemptionRecordError {
    MissingLogicalCpu,
    InvalidLogicalCpu {
        logical_cpu: usize,
        cpu_capacity: usize,
    },
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    ProductionDispatchDeferred {
        owner: LogicalCpuId,
    },
    PreemptionState(PerCorePreemptionStateError),
}

impl From<PerCorePreemptionStateError> for ProductionTimerPreemptionRecordError {
    fn from(error: PerCorePreemptionStateError) -> Self {
        match error {
            PerCorePreemptionStateError::WrongOwner { owner, requester } => {
                Self::WrongOwner { owner, requester }
            }
            error => Self::PreemptionState(error),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionSchedulerRuntime<
    const RUNNABLE_CAPACITY: usize,
    const REMOTE_WAKE_CAPACITY: usize,
> {
    scheduler: PerCoreScheduler<RUNNABLE_CAPACITY>,
    preemption_state: PerCorePreemptionState,
    remote_wake_queue: RemoteWakeQueue<REMOTE_WAKE_CAPACITY>,
    timer_record_rejections: u64,
}

impl<const RUNNABLE_CAPACITY: usize, const REMOTE_WAKE_CAPACITY: usize>
    ProductionSchedulerRuntime<RUNNABLE_CAPACITY, REMOTE_WAKE_CAPACITY>
{
    pub const fn new(owner: LogicalCpuId, role: SchedulerCoreRole) -> Self {
        Self {
            scheduler: PerCoreScheduler::new(owner, role),
            preemption_state: PerCorePreemptionState::new(owner),
            remote_wake_queue: RemoteWakeQueue::new(owner),
            timer_record_rejections: 0,
        }
    }

    pub const fn boot_cpu() -> Self {
        Self::new(LogicalCpuId::BOOT, SchedulerCoreRole::BootCpuProduction)
    }

    pub const fn deferred_secondary(owner: LogicalCpuId) -> Self {
        Self::new(owner, SchedulerCoreRole::SecondaryDeferred)
    }

    pub const fn production_secondary_diagnostic(owner: LogicalCpuId) -> Self {
        Self::new(owner, SchedulerCoreRole::SecondaryProductionDiagnostic)
    }

    pub const fn owner(&self) -> LogicalCpuId {
        self.scheduler.owner()
    }

    pub const fn role(&self) -> SchedulerCoreRole {
        self.scheduler.role()
    }

    pub const fn scheduler(&self) -> &PerCoreScheduler<RUNNABLE_CAPACITY> {
        &self.scheduler
    }

    pub const fn preemption_state(&self) -> &PerCorePreemptionState {
        &self.preemption_state
    }

    pub const fn remote_wake_queue(&self) -> &RemoteWakeQueue<REMOTE_WAKE_CAPACITY> {
        &self.remote_wake_queue
    }

    pub const fn timer_record_rejections(&self) -> u64 {
        self.timer_record_rejections
    }

    pub fn scheduler_mut(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<&mut PerCoreScheduler<RUNNABLE_CAPACITY>, PerCoreSchedulerAccessError> {
        self.scheduler.ensure_local_owner(requester)?;
        Ok(&mut self.scheduler)
    }

    pub fn remote_wake_queue_mut(
        &mut self,
        requester: LogicalCpuId,
    ) -> Result<&mut RemoteWakeQueue<REMOTE_WAKE_CAPACITY>, RemoteWakeRequestError> {
        if requester != self.remote_wake_queue.owner() {
            Err(RemoteWakeRequestError::WrongOwner {
                owner: self.remote_wake_queue.owner(),
                requester,
            })
        } else {
            Ok(&mut self.remote_wake_queue)
        }
    }

    pub fn record_timer_irq<const CPU_CAPACITY: usize>(
        &mut self,
        logical_cpu: Option<usize>,
    ) -> Result<PreemptionRecordOutcome, ProductionTimerPreemptionRecordError> {
        let logical_cpu = logical_cpu.ok_or_else(|| {
            self.timer_record_rejections += 1;
            ProductionTimerPreemptionRecordError::MissingLogicalCpu
        })?;
        if logical_cpu >= CPU_CAPACITY {
            self.timer_record_rejections += 1;
            return Err(ProductionTimerPreemptionRecordError::InvalidLogicalCpu {
                logical_cpu,
                cpu_capacity: CPU_CAPACITY,
            });
        }

        let requester = LogicalCpuId::new(logical_cpu);
        if requester != self.owner() {
            self.timer_record_rejections += 1;
            return Err(ProductionTimerPreemptionRecordError::WrongOwner {
                owner: self.owner(),
                requester,
            });
        }
        if !self.scheduler.production_dispatch_enabled() {
            self.timer_record_rejections += 1;
            return Err(
                ProductionTimerPreemptionRecordError::ProductionDispatchDeferred {
                    owner: self.owner(),
                },
            );
        }

        self.preemption_state
            .record_local_timer_irq(requester)
            .map_err(|error| {
                self.timer_record_rejections += 1;
                ProductionTimerPreemptionRecordError::from(error)
            })
    }

    pub fn service_pending_preemption<const TASK_CAPACITY: usize, const CPU_CAPACITY: usize>(
        &mut self,
        requester: LogicalCpuId,
        metadata: &mut SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        local_task: &mut Task,
        current_task_for_timer_preemption: Option<&mut Task>,
        dispatch_local_task: bool,
    ) -> Result<CpuLocalSchedulerServiceReport, CpuLocalSchedulerServiceError> {
        CpuLocalSchedulerService::run_preemption_cycle(
            requester,
            &mut self.scheduler,
            &mut self.preemption_state,
            &mut self.remote_wake_queue,
            metadata,
            local_task,
            current_task_for_timer_preemption,
            dispatch_local_task,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecondarySchedulerServiceLoopError {
    BootCpuNotSecondary {
        owner: LogicalCpuId,
    },
    WrongOwner {
        owner: LogicalCpuId,
        requester: LogicalCpuId,
    },
    ProductionDispatchDeferred {
        owner: LogicalCpuId,
    },
    CpuLocal(CpuLocalSchedulerServiceError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuLocalSchedulerServiceReport {
    remote_wake: Option<TaskId>,
    timer_preemption: Option<TaskId>,
    dispatch: Option<TaskId>,
    metadata: SchedulerTaskSnapshot,
}

impl CpuLocalSchedulerServiceReport {
    pub const fn remote_wake(self) -> Option<TaskId> {
        self.remote_wake
    }

    pub const fn timer_preemption(self) -> Option<TaskId> {
        self.timer_preemption
    }

    pub const fn dispatch(self) -> Option<TaskId> {
        self.dispatch
    }

    pub const fn metadata(self) -> SchedulerTaskSnapshot {
        self.metadata
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecondarySchedulerServiceLoopReport {
    cycle: CpuLocalSchedulerServiceReport,
    observed_remote_wake: bool,
    pending_timer_preemption: bool,
    dispatch_requested: bool,
}

impl SecondarySchedulerServiceLoopReport {
    pub const fn cycle(self) -> CpuLocalSchedulerServiceReport {
        self.cycle
    }

    pub const fn observed_remote_wake(self) -> bool {
        self.observed_remote_wake
    }

    pub const fn pending_timer_preemption(self) -> bool {
        self.pending_timer_preemption
    }

    pub const fn dispatch_requested(self) -> bool {
        self.dispatch_requested
    }

    pub const fn did_work(self) -> bool {
        self.cycle.remote_wake().is_some()
            || self.cycle.timer_preemption().is_some()
            || self.cycle.dispatch().is_some()
    }
}

pub struct CpuLocalSchedulerService;

impl CpuLocalSchedulerService {
    pub fn run_preemption_cycle<
        const RUNNABLE_CAPACITY: usize,
        const REMOTE_WAKE_CAPACITY: usize,
        const TASK_CAPACITY: usize,
        const CPU_CAPACITY: usize,
    >(
        requester: LogicalCpuId,
        scheduler: &mut PerCoreScheduler<RUNNABLE_CAPACITY>,
        preemption_state: &mut PerCorePreemptionState,
        remote_wake_queue: &mut RemoteWakeQueue<REMOTE_WAKE_CAPACITY>,
        metadata: &mut SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        local_task: &mut Task,
        current_task_for_timer_preemption: Option<&mut Task>,
        dispatch_local_task: bool,
    ) -> Result<CpuLocalSchedulerServiceReport, CpuLocalSchedulerServiceError> {
        let pending_timer_preemption = preemption_state
            .prepare_owner_local_service(requester, scheduler.owner())
            .map_err(CpuLocalSchedulerServiceError::PreemptionState)?;

        if pending_timer_preemption {
            scheduler
                .ensure_production_owner(requester)
                .map_err(|error| {
                    CpuLocalSchedulerServiceError::ProductionDispatch(
                        ProductionDispatchError::from(error),
                    )
                })?;
            let current_task = current_task_for_timer_preemption
                .as_deref()
                .ok_or(CpuLocalSchedulerServiceError::MissingCurrentTaskForTimerPreemption)?;
            if scheduler.current_task() != Some(current_task.id()) {
                return Err(CpuLocalSchedulerServiceError::CurrentTaskMismatch {
                    current_task: scheduler.current_task(),
                    provided: current_task.id(),
                });
            }
        }

        let report = Self::run_cycle(
            requester,
            scheduler,
            remote_wake_queue,
            metadata,
            local_task,
            current_task_for_timer_preemption,
            pending_timer_preemption,
            dispatch_local_task,
        )?;

        if report.timer_preemption().is_some() {
            preemption_state.complete_owner_local_service();
        }

        Ok(report)
    }

    pub fn run_cycle<
        const RUNNABLE_CAPACITY: usize,
        const REMOTE_WAKE_CAPACITY: usize,
        const TASK_CAPACITY: usize,
        const CPU_CAPACITY: usize,
    >(
        requester: LogicalCpuId,
        scheduler: &mut PerCoreScheduler<RUNNABLE_CAPACITY>,
        remote_wake_queue: &mut RemoteWakeQueue<REMOTE_WAKE_CAPACITY>,
        metadata: &mut SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        local_task: &mut Task,
        current_task_for_timer_preemption: Option<&mut Task>,
        pending_timer_preemption: bool,
        dispatch_local_task: bool,
    ) -> Result<CpuLocalSchedulerServiceReport, CpuLocalSchedulerServiceError> {
        let remote_wake = match remote_wake_queue
            .consume_next(requester)
            .map_err(CpuLocalSchedulerServiceError::RemoteWakeQueue)?
        {
            Some(request) => Some(
                scheduler
                    .wake_blocked_local_task_from_remote_request(requester, request, local_task)
                    .map_err(CpuLocalSchedulerServiceError::RemoteWake)?,
            ),
            None => None,
        };

        let timer_preemption = if pending_timer_preemption {
            let current_task = current_task_for_timer_preemption
                .ok_or(CpuLocalSchedulerServiceError::MissingCurrentTaskForTimerPreemption)?;
            let next_task_id = scheduler
                .production_scheduler_mut(requester)
                .map_err(|error| {
                    CpuLocalSchedulerServiceError::ProductionDispatch(
                        ProductionDispatchError::from(error),
                    )
                })?
                .timer_preempt(current_task)
                .map_err(CpuLocalSchedulerServiceError::TimerPreempt)?;
            if local_task.id() == next_task_id {
                local_task.set_state(TaskState::Running);
            }
            scheduler
                .set_current_task(requester, next_task_id)
                .map_err(|error| {
                    CpuLocalSchedulerServiceError::ProductionDispatch(
                        ProductionDispatchError::from(error),
                    )
                })?;
            Some(next_task_id)
        } else {
            None
        };

        let dispatch = if dispatch_local_task && timer_preemption.is_none() {
            Some(
                scheduler
                    .dispatch_cpu_local_diagnostic_task(requester, local_task)
                    .map_err(CpuLocalSchedulerServiceError::ProductionDispatch)?,
            )
        } else {
            None
        };

        let metadata = metadata
            .refresh_local_task(requester, scheduler, local_task)
            .map_err(CpuLocalSchedulerServiceError::Metadata)?;

        Ok(CpuLocalSchedulerServiceReport {
            remote_wake,
            timer_preemption,
            dispatch,
            metadata,
        })
    }
}

pub struct SecondarySchedulerServiceLoop;

impl SecondarySchedulerServiceLoop {
    pub fn run_once<
        const RUNNABLE_CAPACITY: usize,
        const REMOTE_WAKE_CAPACITY: usize,
        const TASK_CAPACITY: usize,
        const CPU_CAPACITY: usize,
    >(
        requester: LogicalCpuId,
        scheduler: &mut PerCoreScheduler<RUNNABLE_CAPACITY>,
        remote_wake_queue: &mut RemoteWakeQueue<REMOTE_WAKE_CAPACITY>,
        metadata: &mut SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>,
        local_task: &mut Task,
        current_task_for_timer_preemption: Option<&mut Task>,
        pending_timer_preemption: bool,
        dispatch_local_task: bool,
    ) -> Result<SecondarySchedulerServiceLoopReport, SecondarySchedulerServiceLoopError> {
        Self::ensure_secondary_owner(requester, scheduler)?;

        let observed_remote_wake = !remote_wake_queue.is_empty();
        let cycle = CpuLocalSchedulerService::run_cycle(
            requester,
            scheduler,
            remote_wake_queue,
            metadata,
            local_task,
            current_task_for_timer_preemption,
            pending_timer_preemption,
            dispatch_local_task,
        )
        .map_err(SecondarySchedulerServiceLoopError::CpuLocal)?;

        Ok(SecondarySchedulerServiceLoopReport {
            cycle,
            observed_remote_wake,
            pending_timer_preemption,
            dispatch_requested: dispatch_local_task,
        })
    }

    fn ensure_secondary_owner<const RUNNABLE_CAPACITY: usize>(
        requester: LogicalCpuId,
        scheduler: &PerCoreScheduler<RUNNABLE_CAPACITY>,
    ) -> Result<(), SecondarySchedulerServiceLoopError> {
        let owner = scheduler.owner();
        if requester != owner {
            return Err(SecondarySchedulerServiceLoopError::WrongOwner { owner, requester });
        }
        if owner == LogicalCpuId::BOOT {
            return Err(SecondarySchedulerServiceLoopError::BootCpuNotSecondary { owner });
        }
        if scheduler.role() != SchedulerCoreRole::SecondaryProductionDiagnostic {
            return Err(SecondarySchedulerServiceLoopError::ProductionDispatchDeferred { owner });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedSchedulerMetadata<const TASK_CAPACITY: usize, const CPU_CAPACITY: usize> {
    entries: [Option<SchedulerTaskSnapshot>; TASK_CAPACITY],
    len: usize,
    generation: u64,
}

/// SMP-protected shared scheduler metadata boundary.
///
/// Local runnable queues remain owned by `PerCoreScheduler`; this lock may
/// protect only the read-oriented metadata table, with IRQ masking handled by
/// the caller according to the accepted scheduler lock-ordering rule.
pub type SharedSchedulerMetadataLock<const TASK_CAPACITY: usize, const CPU_CAPACITY: usize> =
    crate::smp_sync::SpinLock<SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>>;

impl<const TASK_CAPACITY: usize, const CPU_CAPACITY: usize>
    SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>
{
    pub const fn new() -> Self {
        Self {
            entries: [None; TASK_CAPACITY],
            len: 0,
            generation: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn capacity(&self) -> usize {
        TASK_CAPACITY
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn register_local_task<const RUNNABLE_CAPACITY: usize>(
        &mut self,
        requester: LogicalCpuId,
        owner_scheduler: &PerCoreScheduler<RUNNABLE_CAPACITY>,
        task: &Task,
    ) -> Result<SchedulerTaskSnapshot, SharedSchedulerMetadataError> {
        let owner = owner_scheduler.owner();
        self.ensure_valid_owner(owner)?;
        self.ensure_requester_owns_scheduler(requester, owner)?;
        if let Some(existing) = self.find_task(task.id()) {
            return Err(SharedSchedulerMetadataError::DuplicateTask {
                task_id: task.id(),
                existing_owner: existing.owner,
                attempted_owner: owner,
            });
        }
        if self.len == TASK_CAPACITY {
            return Err(SharedSchedulerMetadataError::Full);
        }

        let generation = self.advance_generation();
        let snapshot = Self::snapshot_from_local_scheduler(owner_scheduler, task, generation);
        self.entries[self.len] = Some(snapshot);
        self.len += 1;
        Ok(snapshot)
    }

    pub fn refresh_local_task<const RUNNABLE_CAPACITY: usize>(
        &mut self,
        requester: LogicalCpuId,
        owner_scheduler: &PerCoreScheduler<RUNNABLE_CAPACITY>,
        task: &Task,
    ) -> Result<SchedulerTaskSnapshot, SharedSchedulerMetadataError> {
        let owner = owner_scheduler.owner();
        self.ensure_valid_owner(owner)?;
        self.ensure_requester_owns_scheduler(requester, owner)?;
        let index = self
            .find_task_index(task.id())
            .ok_or(SharedSchedulerMetadataError::UnknownTask { task_id: task.id() })?;
        let existing = self.entries[index].expect("find_task_index returns a populated slot");
        if existing.owner != owner {
            return Err(SharedSchedulerMetadataError::WrongOwner {
                owner: existing.owner,
                requester,
            });
        }

        let generation = self.advance_generation();
        let snapshot = Self::snapshot_from_local_scheduler(owner_scheduler, task, generation);
        self.entries[index] = Some(snapshot);
        Ok(snapshot)
    }

    fn refresh_migrated_task<const RUNNABLE_CAPACITY: usize>(
        &mut self,
        requester: LogicalCpuId,
        destination_scheduler: &PerCoreScheduler<RUNNABLE_CAPACITY>,
        task: &Task,
        source_owner: LogicalCpuId,
        expected_generation: u64,
    ) -> Result<SchedulerTaskSnapshot, SharedRunQueueError> {
        let destination_owner = destination_scheduler.owner();
        if destination_owner.raw() >= CPU_CAPACITY {
            return Err(SharedRunQueueError::InvalidDestinationOwner {
                destination_owner,
                cpu_capacity: CPU_CAPACITY,
            });
        }
        if requester != destination_owner {
            return Err(SharedRunQueueError::WrongDestinationOwner {
                owner: destination_owner,
                requester,
            });
        }

        let index = self
            .find_task_index(task.id())
            .ok_or(SharedRunQueueError::UnknownTask { task_id: task.id() })?;
        let existing = self.entries[index].expect("find_task_index returns a populated slot");
        if existing.owner != source_owner {
            return Err(SharedRunQueueError::MetadataOwnerMismatch {
                task_id: task.id(),
                metadata_owner: existing.owner,
                expected_owner: source_owner,
            });
        }
        if existing.generation != expected_generation {
            return Err(SharedRunQueueError::StaleMetadataGeneration {
                task_id: task.id(),
                expected_generation,
                actual_generation: existing.generation,
            });
        }

        let generation = self.advance_generation();
        let snapshot = Self::snapshot_from_local_scheduler(destination_scheduler, task, generation);
        self.entries[index] = Some(snapshot);
        Ok(snapshot)
    }

    pub fn lookup_task(
        &self,
        task_id: TaskId,
    ) -> Result<SchedulerTaskSnapshot, SharedSchedulerMetadataError> {
        self.find_task(task_id)
            .ok_or(SharedSchedulerMetadataError::UnknownTask { task_id })
    }

    pub fn lookup_task_at_generation(
        &self,
        task_id: TaskId,
        expected_generation: u64,
    ) -> Result<SchedulerTaskSnapshot, SharedSchedulerMetadataError> {
        let snapshot = self.lookup_task(task_id)?;
        if snapshot.generation == expected_generation {
            Ok(snapshot)
        } else {
            Err(SharedSchedulerMetadataError::StaleSnapshot {
                task_id,
                expected_generation,
                actual_generation: snapshot.generation,
            })
        }
    }

    fn ensure_valid_owner(&self, owner: LogicalCpuId) -> Result<(), SharedSchedulerMetadataError> {
        if owner.raw() < CPU_CAPACITY {
            Ok(())
        } else {
            Err(SharedSchedulerMetadataError::InvalidOwner {
                owner,
                cpu_capacity: CPU_CAPACITY,
            })
        }
    }

    fn ensure_requester_owns_scheduler(
        &self,
        requester: LogicalCpuId,
        owner: LogicalCpuId,
    ) -> Result<(), SharedSchedulerMetadataError> {
        if requester == owner {
            Ok(())
        } else {
            Err(SharedSchedulerMetadataError::WrongOwner { owner, requester })
        }
    }

    fn snapshot_from_local_scheduler<const RUNNABLE_CAPACITY: usize>(
        owner_scheduler: &PerCoreScheduler<RUNNABLE_CAPACITY>,
        task: &Task,
        generation: u64,
    ) -> SchedulerTaskSnapshot {
        SchedulerTaskSnapshot {
            task_id: task.id(),
            owner: owner_scheduler.owner(),
            state: task.state(),
            process_owner: task.process_owner(),
            kernel_stack: task.kernel_stack(),
            current_on_owner: owner_scheduler.current_task() == Some(task.id()),
            runnable_on_owner: owner_scheduler.scheduler().runnable().contains(task.id()),
            generation,
        }
    }

    fn find_task(&self, task_id: TaskId) -> Option<SchedulerTaskSnapshot> {
        self.find_task_index(task_id)
            .and_then(|index| self.entries[index])
    }

    fn find_task_index(&self, task_id: TaskId) -> Option<usize> {
        let mut index = 0;
        while index < self.len {
            if let Some(snapshot) = self.entries[index]
                && snapshot.task_id == task_id
            {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn advance_generation(&mut self) -> u64 {
        self.generation = self.generation.checked_add(1).unwrap_or(1);
        self.generation
    }
}

impl<const TASK_CAPACITY: usize, const CPU_CAPACITY: usize> Default
    for SharedSchedulerMetadata<TASK_CAPACITY, CPU_CAPACITY>
{
    fn default() -> Self {
        Self::new()
    }
}

impl From<PerCoreSchedulerAccessError> for ProductionDispatchError {
    fn from(error: PerCoreSchedulerAccessError) -> Self {
        match error {
            PerCoreSchedulerAccessError::WrongOwner { owner, requester } => {
                Self::WrongOwner { owner, requester }
            }
            PerCoreSchedulerAccessError::ProductionDispatchDeferred { owner } => {
                Self::ProductionDispatchDeferred { owner }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{offset_of, size_of};

    use super::{
        ContextFrame, CpuLocalSchedulerService, CpuLocalSchedulerServiceError, KernelStack,
        LoadBalancingPolicy, LoadBalancingPolicyError, LogicalCpuId, PerCorePreemptionState,
        PerCorePreemptionStateError, PerCoreScheduler, PerCoreSchedulerAccessError,
        PreemptionRecordOutcome, ProcessOwnerId, ProductionDispatchError,
        ProductionSchedulerRuntime, ProductionTimerPreemptionRecordError, RemoteWakePublishOutcome,
        RemoteWakeQueue, RemoteWakeRequest, RemoteWakeRequestError, RunnableQueue,
        RunnableQueueError, SchedulerCoreRole, SecondarySchedulerServiceLoop,
        SecondarySchedulerServiceLoopError, SharedRunQueue, SharedRunQueueError,
        SharedRunQueueLock, SharedSchedulerMetadata, SharedSchedulerMetadataError,
        SharedSchedulerMetadataLock, SingleCoreScheduler, TargetWakeConsumptionError, Task, TaskId,
        TaskState, TimerPreemptError, VoluntaryYieldError,
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
    fn runnable_queue_removes_middle_and_preserves_fifo_order() {
        let mut queue = RunnableQueue::<4>::new();

        queue.enqueue(task_id(1)).expect("enqueue task 1");
        queue.enqueue(task_id(2)).expect("enqueue task 2");
        queue.enqueue(task_id(3)).expect("enqueue task 3");

        assert!(queue.remove(task_id(2)));
        assert!(!queue.contains(task_id(2)));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.dequeue(), Some(task_id(1)));
        assert_eq!(queue.dequeue(), Some(task_id(3)));
        assert_eq!(queue.dequeue(), None);
        assert!(!queue.remove(task_id(2)));
    }

    #[test_case]
    fn runnable_queue_removes_wrapped_entry_and_keeps_tail_reusable() {
        let mut queue = RunnableQueue::<3>::new();

        queue.enqueue(task_id(1)).expect("enqueue task 1");
        queue.enqueue(task_id(2)).expect("enqueue task 2");
        queue.enqueue(task_id(3)).expect("enqueue task 3");
        assert_eq!(queue.dequeue(), Some(task_id(1)));
        queue.enqueue(task_id(4)).expect("enqueue wrapped task");

        assert!(queue.remove(task_id(3)));
        assert_eq!(queue.dequeue(), Some(task_id(2)));
        assert_eq!(queue.dequeue(), Some(task_id(4)));
        queue.enqueue(task_id(5)).expect("tail slot reusable");
        assert_eq!(queue.dequeue(), Some(task_id(5)));
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
    fn make_runnable_rejects_full_queue_without_mutating_task_or_counters() {
        let mut scheduler = SingleCoreScheduler::<1>::new();
        let mut queued = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let mut blocked = Task::kernel_thread(task_id(2), kernel_stack(), context());
        blocked.set_state(TaskState::Blocked);

        scheduler
            .make_runnable(&mut queued)
            .expect("first runnable fills queue");
        let transitions = scheduler.counters().state_transitions();

        assert_eq!(
            scheduler.make_runnable(&mut blocked),
            Err(RunnableQueueError::Full)
        );
        assert_eq!(blocked.state(), TaskState::Blocked);
        assert_eq!(scheduler.counters().state_transitions(), transitions);
        assert_eq!(scheduler.runnable().front(), Some(queued.id()));
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
    fn voluntary_yield_allows_one_slot_queue_by_dequeuing_before_requeue() {
        let mut scheduler = SingleCoreScheduler::<1>::new();
        let mut current = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(2), kernel_stack(), context());
        current.set_state(TaskState::Running);

        scheduler
            .make_runnable(&mut next)
            .expect("single runnable slot holds next task");

        let next_id = scheduler
            .voluntary_yield(&mut current)
            .expect("yield switches despite full one-slot runnable queue");

        assert_eq!(next_id, next.id());
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(scheduler.runnable().front(), Some(current.id()));
        assert_eq!(scheduler.runnable().len(), 1);
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
    fn timer_preempt_allows_one_slot_queue_by_dequeuing_before_requeue() {
        let mut scheduler = SingleCoreScheduler::<1>::new();
        let mut current = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(2), kernel_stack(), context());
        current.set_state(TaskState::Running);

        scheduler
            .make_runnable(&mut next)
            .expect("single runnable slot holds next task");

        let next_id = scheduler
            .timer_preempt(&mut current)
            .expect("timer preemption switches despite full one-slot runnable queue");

        assert_eq!(next_id, next.id());
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(scheduler.runnable().front(), Some(current.id()));
        assert_eq!(scheduler.runnable().len(), 1);
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
    fn production_secondary_diagnostic_dispatches_only_local_runnable_task() {
        let owner = LogicalCpuId::new(2);
        let mut per_core = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut task = Task::kernel_thread(task_id(202), kernel_stack(), context());

        assert_eq!(per_core.owner(), owner);
        assert_eq!(
            per_core.role(),
            SchedulerCoreRole::SecondaryProductionDiagnostic
        );
        assert!(per_core.production_dispatch_enabled());

        per_core
            .local_scheduler_mut(owner)
            .expect("owner seeds local diagnostic task")
            .make_runnable(&mut task)
            .expect("local queue has capacity");
        let dispatched = per_core
            .dispatch_cpu_local_diagnostic_task(owner, &mut task)
            .expect("owner dispatches local diagnostic task");

        assert_eq!(dispatched, task_id(202));
        assert_eq!(per_core.current_task(), Some(task_id(202)));
        assert_eq!(task.state(), TaskState::Running);
        assert_eq!(per_core.scheduler().runnable().len(), 0);
        assert_eq!(per_core.scheduler().counters().production_dispatches(), 1);
        assert_eq!(per_core.scheduler().counters().context_switches(), 1);
    }

    #[test_case]
    fn production_secondary_diagnostic_rejects_cross_owner_dispatch() {
        let owner = LogicalCpuId::new(1);
        let requester = LogicalCpuId::new(3);
        let mut per_core = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut task = Task::kernel_thread(task_id(101), kernel_stack(), context());

        per_core
            .local_scheduler_mut(owner)
            .expect("owner seeds local diagnostic task")
            .make_runnable(&mut task)
            .expect("local queue has capacity");

        assert_eq!(
            per_core.dispatch_cpu_local_diagnostic_task(requester, &mut task),
            Err(ProductionDispatchError::WrongOwner { owner, requester })
        );
        assert_eq!(task.state(), TaskState::Runnable);
        assert_eq!(per_core.current_task(), None);
        assert_eq!(per_core.scheduler().runnable().front(), Some(task_id(101)));
    }

    #[test_case]
    fn production_secondary_diagnostic_rejects_mismatched_or_nonrunnable_task() {
        let owner = LogicalCpuId::new(3);
        let mut per_core = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut queued = Task::kernel_thread(task_id(301), kernel_stack(), context());
        let mut other = Task::kernel_thread(task_id(302), kernel_stack(), context());

        per_core
            .local_scheduler_mut(owner)
            .expect("owner seeds local diagnostic task")
            .make_runnable(&mut queued)
            .expect("local queue has capacity");

        assert_eq!(
            per_core.dispatch_cpu_local_diagnostic_task(owner, &mut other),
            Err(ProductionDispatchError::SelectedTaskMismatch {
                queued: task_id(301),
                provided: task_id(302)
            })
        );
        assert_eq!(per_core.scheduler().runnable().front(), Some(task_id(301)));

        queued.set_state(TaskState::Blocked);
        assert_eq!(
            per_core.dispatch_cpu_local_diagnostic_task(owner, &mut queued),
            Err(ProductionDispatchError::TaskNotRunnable {
                task_id: task_id(301),
                state: TaskState::Blocked
            })
        );
        assert_eq!(per_core.current_task(), None);
        assert_eq!(per_core.scheduler().runnable().front(), Some(task_id(301)));
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
    fn shared_scheduler_metadata_registers_local_task_snapshot() {
        let owner = LogicalCpuId::new(1);
        let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut task = Task::kernel_thread(task_id(101), kernel_stack(), context());
        task.attach_process_owner(ProcessOwnerId::new(7).expect("process owner"));
        scheduler
            .set_current_task(owner, task.id())
            .expect("diagnostic owner records current task");
        let mut metadata = SharedSchedulerMetadata::<4, 4>::new();

        assert!(metadata.is_empty());
        assert_eq!(metadata.capacity(), 4);
        let snapshot = metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("owner publishes local metadata");

        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata.generation(), 1);
        assert_eq!(snapshot.task_id(), task.id());
        assert_eq!(snapshot.owner(), owner);
        assert_eq!(snapshot.state(), TaskState::Runnable);
        assert_eq!(snapshot.process_owner(), task.process_owner());
        assert_eq!(snapshot.kernel_stack(), task.kernel_stack());
        assert!(snapshot.current_on_owner());
        assert!(!snapshot.runnable_on_owner());
        assert_eq!(
            metadata
                .lookup_task(task.id())
                .expect("snapshot is present"),
            snapshot
        );
    }

    #[test_case]
    fn shared_scheduler_metadata_refresh_tracks_owner_local_membership() {
        let owner = LogicalCpuId::new(2);
        let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(owner);
        let mut task = Task::kernel_thread(task_id(202), kernel_stack(), context());
        task.set_state(TaskState::Blocked);
        let mut metadata = SharedSchedulerMetadata::<4, 4>::new();
        let first = metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("owner registers blocked task");
        assert_eq!(first.state(), TaskState::Blocked);
        assert!(!first.runnable_on_owner());

        task.set_state(TaskState::Runnable);
        scheduler
            .local_scheduler_mut(owner)
            .expect("owner mutates local queue")
            .make_runnable(&mut task)
            .expect("local queue has capacity");
        let refreshed = metadata
            .refresh_local_task(owner, &scheduler, &task)
            .expect("owner refreshes local metadata");

        assert_eq!(refreshed.state(), TaskState::Runnable);
        assert!(refreshed.runnable_on_owner());
        assert_eq!(refreshed.generation(), first.generation() + 1);
        assert_eq!(
            metadata.lookup_task_at_generation(task.id(), first.generation()),
            Err(SharedSchedulerMetadataError::StaleSnapshot {
                task_id: task.id(),
                expected_generation: first.generation(),
                actual_generation: refreshed.generation()
            })
        );
    }

    #[test_case]
    fn shared_scheduler_metadata_rejects_duplicate_and_unknown_tasks() {
        let owner = LogicalCpuId::BOOT;
        let scheduler = PerCoreScheduler::<2>::boot_cpu();
        let task = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let unknown = task_id(2);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();

        metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("first registration succeeds");

        assert_eq!(
            metadata.register_local_task(owner, &scheduler, &task),
            Err(SharedSchedulerMetadataError::DuplicateTask {
                task_id: task.id(),
                existing_owner: owner,
                attempted_owner: owner
            })
        );
        assert_eq!(
            metadata.lookup_task(unknown),
            Err(SharedSchedulerMetadataError::UnknownTask { task_id: unknown })
        );
    }

    #[test_case]
    fn shared_scheduler_metadata_rejects_invalid_owner_and_task_id_zero() {
        let invalid_owner = LogicalCpuId::new(4);
        let scheduler = PerCoreScheduler::<2>::deferred_secondary(invalid_owner);
        let task = Task::kernel_thread(task_id(4), kernel_stack(), context());
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();

        assert_eq!(TaskId::new(0), None);
        assert_eq!(
            metadata.register_local_task(invalid_owner, &scheduler, &task),
            Err(SharedSchedulerMetadataError::InvalidOwner {
                owner: invalid_owner,
                cpu_capacity: 4
            })
        );
    }

    #[test_case]
    fn shared_scheduler_metadata_rejects_cross_owner_publication_without_queue_mutation() {
        let owner = LogicalCpuId::new(1);
        let requester = LogicalCpuId::BOOT;
        let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(owner);
        let mut task = Task::kernel_thread(task_id(55), kernel_stack(), context());
        scheduler
            .local_scheduler_mut(owner)
            .expect("owner seeds local queue")
            .make_runnable(&mut task)
            .expect("local queue has capacity");
        let queue_len = scheduler.scheduler().runnable().len();
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();

        assert_eq!(
            metadata.register_local_task(requester, &scheduler, &task),
            Err(SharedSchedulerMetadataError::WrongOwner { owner, requester })
        );

        assert_eq!(metadata.len(), 0);
        assert_eq!(scheduler.scheduler().runnable().len(), queue_len);
        assert_eq!(scheduler.scheduler().runnable().front(), Some(task.id()));
    }

    #[test_case]
    fn shared_scheduler_metadata_lock_protects_metadata_table_only() {
        let lock = SharedSchedulerMetadataLock::<2, 4>::new(SharedSchedulerMetadata::new());

        {
            let metadata = lock.lock();
            assert!(metadata.is_empty());
            assert_eq!(metadata.capacity(), 2);
            assert_eq!(metadata.generation(), 0);
        }

        assert!(!lock.is_locked());
    }

    #[test_case]
    fn shared_run_queue_publishes_migration_without_cross_owner_local_enqueue() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut shared = SharedRunQueue::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(70), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("source local queue has capacity");
        let snapshot = metadata
            .register_local_task(source, &source_scheduler, &task)
            .expect("source publishes task metadata");

        let report = shared
            .publish_migration(
                source,
                &mut source_scheduler,
                destination,
                &metadata,
                &task,
                snapshot.generation(),
            )
            .expect("source publishes shared handoff");

        assert_eq!(
            super::MigrationState::OwnerLocal,
            super::MigrationState::OwnerLocal
        );
        assert_eq!(
            super::MigrationState::MigrationRejected,
            super::MigrationState::MigrationRejected
        );
        assert_eq!(report.queued().task_id(), task.id());
        assert_eq!(report.queued().source_owner(), source);
        assert_eq!(report.queued().destination_owner(), destination);
        assert_eq!(report.queued().metadata_generation(), snapshot.generation());
        assert_eq!(
            report.reserved().state(),
            super::MigrationState::MigrationReserved
        );
        assert_eq!(report.queued().state(), super::MigrationState::SharedQueued);
        assert_eq!(shared.front(), Some(report.queued()));
        assert_eq!(shared.len(), 1);
        assert!(!source_scheduler.scheduler().runnable().contains(task.id()));
        assert_eq!(source_scheduler.scheduler().runnable().len(), 0);
        assert_eq!(
            metadata
                .lookup_task(task.id())
                .expect("metadata remains source-owned until destination consumes")
                .owner(),
            source
        );
    }

    #[test_case]
    fn shared_run_queue_destination_consumes_and_updates_metadata_owner() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut destination_scheduler =
            PerCoreScheduler::<2>::production_secondary_diagnostic(destination);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut shared = SharedRunQueue::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(71), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("source local queue has capacity");
        let snapshot = metadata
            .register_local_task(source, &source_scheduler, &task)
            .expect("source publishes task metadata");
        let queued = shared
            .publish_migration(
                source,
                &mut source_scheduler,
                destination,
                &metadata,
                &task,
                snapshot.generation(),
            )
            .expect("source publishes shared handoff")
            .queued();

        let report = shared
            .consume_for_destination(
                destination,
                &mut destination_scheduler,
                &mut metadata,
                &mut task,
            )
            .expect("destination consume returns result")
            .expect("handoff is pending for destination");

        assert_eq!(report.queued(), queued);
        assert_eq!(
            report.destination_enqueued().state(),
            super::MigrationState::DestinationEnqueued
        );
        assert!(shared.is_empty());
        assert!(
            destination_scheduler
                .scheduler()
                .runnable()
                .contains(task.id())
        );
        assert_eq!(
            destination_scheduler.scheduler().runnable().front(),
            Some(task.id())
        );
        assert_eq!(report.metadata().owner(), destination);
        assert_eq!(report.metadata().state(), TaskState::Runnable);
        assert!(report.metadata().runnable_on_owner());
        assert_eq!(
            metadata.lookup_task_at_generation(task.id(), snapshot.generation()),
            Err(SharedSchedulerMetadataError::StaleSnapshot {
                task_id: task.id(),
                expected_generation: snapshot.generation(),
                actual_generation: report.metadata().generation()
            })
        );
    }

    #[test_case]
    fn shared_run_queue_rejects_stale_generation_without_source_queue_mutation() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut shared = SharedRunQueue::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(72), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("source local queue has capacity");
        let snapshot = metadata
            .register_local_task(source, &source_scheduler, &task)
            .expect("source publishes task metadata");

        assert_eq!(
            shared.publish_migration(
                source,
                &mut source_scheduler,
                destination,
                &metadata,
                &task,
                snapshot.generation() + 1,
            ),
            Err(SharedRunQueueError::StaleMetadataGeneration {
                task_id: task.id(),
                expected_generation: snapshot.generation() + 1,
                actual_generation: snapshot.generation()
            })
        );
        assert_eq!(shared.rejected_count(), 1);
        assert!(source_scheduler.scheduler().runnable().contains(task.id()));
        assert!(shared.is_empty());
    }

    #[test_case]
    fn shared_run_queue_rejects_running_blocked_duplicate_and_full_cases() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut shared = SharedRunQueue::<1, 4>::new();

        let mut running_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let running = Task::kernel_thread(task_id(73), kernel_stack(), context());
        running_scheduler
            .set_current_task(source, running.id())
            .expect("boot CPU records current task");
        let mut metadata = SharedSchedulerMetadata::<4, 4>::new();
        metadata
            .register_local_task(source, &running_scheduler, &running)
            .expect("running task metadata");
        assert_eq!(
            shared.publish_migration(
                source,
                &mut running_scheduler,
                destination,
                &metadata,
                &running,
                1
            ),
            Err(SharedRunQueueError::RunningTaskMigrationDeferred {
                task_id: running.id()
            })
        );

        let mut blocked_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut blocked = Task::kernel_thread(task_id(74), kernel_stack(), context());
        blocked.set_state(TaskState::Blocked);
        let mut blocked_metadata = SharedSchedulerMetadata::<4, 4>::new();
        blocked_metadata
            .register_local_task(source, &blocked_scheduler, &blocked)
            .expect("blocked task metadata");
        assert_eq!(
            shared.publish_migration(
                source,
                &mut blocked_scheduler,
                destination,
                &blocked_metadata,
                &blocked,
                1,
            ),
            Err(SharedRunQueueError::BlockedTaskMigrationUnsupported {
                task_id: blocked.id()
            })
        );

        let mut first_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut first_metadata = SharedSchedulerMetadata::<4, 4>::new();
        let mut first = Task::kernel_thread(task_id(75), kernel_stack(), context());
        first_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut first)
            .expect("source local queue has capacity");
        first_metadata
            .register_local_task(source, &first_scheduler, &first)
            .expect("first task metadata");
        shared
            .publish_migration(
                source,
                &mut first_scheduler,
                destination,
                &first_metadata,
                &first,
                1,
            )
            .expect("first handoff fills shared queue");

        let mut second_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut second_metadata = SharedSchedulerMetadata::<4, 4>::new();
        let mut second = Task::kernel_thread(task_id(76), kernel_stack(), context());
        second_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut second)
            .expect("source local queue has capacity");
        second_metadata
            .register_local_task(source, &second_scheduler, &second)
            .expect("second task metadata");
        assert_eq!(
            shared.publish_migration(
                source,
                &mut second_scheduler,
                LogicalCpuId::new(2),
                &second_metadata,
                &second,
                1,
            ),
            Err(SharedRunQueueError::Full)
        );
        assert!(
            second_scheduler
                .scheduler()
                .runnable()
                .contains(second.id())
        );
    }

    #[test_case]
    fn shared_run_queue_rejects_deferred_destination_and_duplicate_local_enqueue() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut deferred_destination = PerCoreScheduler::<2>::deferred_secondary(destination);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut shared = SharedRunQueue::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(77), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("source local queue has capacity");
        let snapshot = metadata
            .register_local_task(source, &source_scheduler, &task)
            .expect("source publishes task metadata");
        shared
            .publish_migration(
                source,
                &mut source_scheduler,
                destination,
                &metadata,
                &task,
                snapshot.generation(),
            )
            .expect("source publishes shared handoff");

        assert_eq!(
            shared.consume_for_destination(
                destination,
                &mut deferred_destination,
                &mut metadata,
                &mut task,
            ),
            Err(SharedRunQueueError::DeferredSecondaryRole { owner: destination })
        );
        assert!(shared.contains(task.id()));

        let mut production_destination =
            PerCoreScheduler::<2>::production_secondary_diagnostic(destination);
        production_destination
            .local_scheduler_mut(destination)
            .expect("destination owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("destination local queue has capacity");
        assert_eq!(
            shared.consume_for_destination(
                destination,
                &mut production_destination,
                &mut metadata,
                &mut task,
            ),
            Err(SharedRunQueueError::DuplicateLocalRunnable {
                task_id: task.id(),
                owner: destination
            })
        );
    }

    #[test_case]
    fn shared_run_queue_lock_wraps_only_shared_handoff_queue() {
        let lock = SharedRunQueueLock::<2, 4>::new(SharedRunQueue::new());

        {
            let queue = lock.lock();
            assert!(queue.is_empty());
            assert_eq!(queue.capacity(), 2);
        }

        assert!(!lock.is_locked());
    }

    #[test_case]
    fn load_balancing_policy_selects_front_candidate_and_publishes_shared_handoff() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let destination_scheduler =
            PerCoreScheduler::<2>::production_secondary_diagnostic(destination);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut shared = SharedRunQueue::<2, 4>::new();
        let mut first = Task::kernel_thread(task_id(90), kernel_stack(), context());
        let mut second = Task::kernel_thread(task_id(91), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut first)
            .expect("source local queue has capacity");
        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut second)
            .expect("source local queue has capacity");
        let snapshot = metadata
            .register_local_task(source, &source_scheduler, &first)
            .expect("source publishes first task metadata");
        metadata
            .register_local_task(source, &source_scheduler, &second)
            .expect("source publishes second task metadata");

        let report = LoadBalancingPolicy::publish_front_runnable(
            source,
            &mut source_scheduler,
            &destination_scheduler,
            &metadata,
            &mut shared,
            &first,
        )
        .expect("policy publishes front candidate through shared queue");

        assert_eq!(report.plan().task_id(), first.id());
        assert_eq!(report.plan().source_owner(), source);
        assert_eq!(report.plan().destination_owner(), destination);
        assert_eq!(report.plan().metadata_generation(), snapshot.generation());
        assert_eq!(report.migration().queued().task_id(), first.id());
        assert!(shared.contains(first.id()));
        assert!(!source_scheduler.scheduler().runnable().contains(first.id()));
        assert!(
            source_scheduler
                .scheduler()
                .runnable()
                .contains(second.id())
        );
        assert!(
            !destination_scheduler
                .scheduler()
                .runnable()
                .contains(first.id())
        );
        assert_eq!(
            metadata
                .lookup_task(first.id())
                .expect("metadata remains source-owned until destination consumes")
                .owner(),
            source
        );
    }

    #[test_case]
    fn load_balancing_policy_rejects_stale_plan_without_source_queue_mutation() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let destination_scheduler =
            PerCoreScheduler::<2>::production_secondary_diagnostic(destination);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut shared = SharedRunQueue::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(92), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("source local queue has capacity");
        let first_snapshot = metadata
            .register_local_task(source, &source_scheduler, &task)
            .expect("source publishes task metadata");
        let plan = LoadBalancingPolicy::plan_front_runnable(
            source,
            &source_scheduler,
            &destination_scheduler,
            &metadata,
            &shared,
        )
        .expect("policy records initial generation");
        let refreshed = metadata
            .refresh_local_task(source, &source_scheduler, &task)
            .expect("source refresh makes plan stale");

        assert_eq!(
            LoadBalancingPolicy::publish_plan(
                source,
                &mut source_scheduler,
                &metadata,
                &mut shared,
                &task,
                plan,
            ),
            Err(LoadBalancingPolicyError::MigrationRejected(
                SharedRunQueueError::StaleMetadataGeneration {
                    task_id: task.id(),
                    expected_generation: first_snapshot.generation(),
                    actual_generation: refreshed.generation()
                }
            ))
        );
        assert!(source_scheduler.scheduler().runnable().contains(task.id()));
        assert!(shared.is_empty());
    }

    #[test_case]
    fn load_balancing_policy_rejects_destination_backpressure_before_publication() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut destination_scheduler =
            PerCoreScheduler::<1>::production_secondary_diagnostic(destination);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let shared = SharedRunQueue::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(93), kernel_stack(), context());
        let mut destination_task = Task::kernel_thread(task_id(94), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("source local queue has capacity");
        metadata
            .register_local_task(source, &source_scheduler, &task)
            .expect("source publishes task metadata");
        destination_scheduler
            .local_scheduler_mut(destination)
            .expect("destination owner mutates local scheduler")
            .make_runnable(&mut destination_task)
            .expect("destination local queue is now full");

        assert_eq!(
            LoadBalancingPolicy::plan_front_runnable(
                source,
                &source_scheduler,
                &destination_scheduler,
                &metadata,
                &shared,
            ),
            Err(LoadBalancingPolicyError::DestinationLocalQueueFull {
                destination_owner: destination,
                task_id: task.id()
            })
        );
        assert!(source_scheduler.scheduler().runnable().contains(task.id()));
        assert!(shared.is_empty());
    }

    #[test_case]
    fn load_balancing_policy_rejects_invalid_destination_role_and_cpu_id() {
        let source = LogicalCpuId::BOOT;
        let deferred_destination = LogicalCpuId::new(1);
        let invalid_destination = LogicalCpuId::new(4);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let deferred_scheduler = PerCoreScheduler::<2>::deferred_secondary(deferred_destination);
        let invalid_scheduler =
            PerCoreScheduler::<2>::production_secondary_diagnostic(invalid_destination);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let shared = SharedRunQueue::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(95), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("source local queue has capacity");
        metadata
            .register_local_task(source, &source_scheduler, &task)
            .expect("source publishes task metadata");

        assert_eq!(
            LoadBalancingPolicy::plan_front_runnable(
                source,
                &source_scheduler,
                &deferred_scheduler,
                &metadata,
                &shared,
            ),
            Err(LoadBalancingPolicyError::DestinationRoleIneligible {
                owner: deferred_destination,
                role: SchedulerCoreRole::SecondaryDeferred
            })
        );
        assert_eq!(
            LoadBalancingPolicy::plan_front_runnable(
                source,
                &source_scheduler,
                &invalid_scheduler,
                &metadata,
                &shared,
            ),
            Err(LoadBalancingPolicyError::InvalidDestinationOwner {
                destination_owner: invalid_destination,
                cpu_capacity: 4
            })
        );
    }

    #[test_case]
    fn load_balancing_policy_rejects_shared_backpressure_and_preserves_single_owner_queue() {
        let source = LogicalCpuId::BOOT;
        let destination = LogicalCpuId::new(1);
        let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
        let destination_scheduler =
            PerCoreScheduler::<2>::production_secondary_diagnostic(destination);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut shared = SharedRunQueue::<1, 4>::new();
        let mut first = Task::kernel_thread(task_id(96), kernel_stack(), context());
        let mut second = Task::kernel_thread(task_id(97), kernel_stack(), context());

        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut first)
            .expect("source local queue has capacity");
        source_scheduler
            .local_scheduler_mut(source)
            .expect("source owner mutates local scheduler")
            .make_runnable(&mut second)
            .expect("source local queue has capacity");
        metadata
            .register_local_task(source, &source_scheduler, &first)
            .expect("source publishes first task metadata");
        metadata
            .register_local_task(source, &source_scheduler, &second)
            .expect("source publishes second task metadata");
        LoadBalancingPolicy::publish_front_runnable(
            source,
            &mut source_scheduler,
            &destination_scheduler,
            &metadata,
            &mut shared,
            &first,
        )
        .expect("first load-balancing handoff fills shared queue");

        assert_eq!(
            LoadBalancingPolicy::plan_front_runnable(
                source,
                &source_scheduler,
                &destination_scheduler,
                &metadata,
                &shared,
            ),
            Err(LoadBalancingPolicyError::SharedRunQueueFull)
        );
        assert!(
            source_scheduler
                .scheduler()
                .runnable()
                .contains(second.id())
        );
        assert!(!source_scheduler.scheduler().runnable().contains(first.id()));
        assert!(shared.contains(first.id()));
    }

    #[test_case]
    fn cpu_local_scheduler_service_drains_wakes_dispatches_and_refreshes_metadata() {
        let owner = LogicalCpuId::new(2);
        let requester = LogicalCpuId::BOOT;
        let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut remote_wakes = RemoteWakeQueue::<2>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut task = Task::kernel_thread(task_id(88), kernel_stack(), context());
        task.set_state(TaskState::Blocked);

        metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("task starts in shared metadata");
        remote_wakes
            .publish(requester, owner, task.id())
            .expect("remote wake publication succeeds");

        let report = CpuLocalSchedulerService::run_cycle(
            owner,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            true,
        )
        .expect("service cycle completes");

        assert_eq!(report.remote_wake(), Some(task.id()));
        assert_eq!(report.timer_preemption(), None);
        assert_eq!(report.dispatch(), Some(task.id()));
        assert!(remote_wakes.is_empty());
        assert_eq!(task.state(), TaskState::Running);
        assert_eq!(scheduler.current_task(), Some(task.id()));
        assert_eq!(scheduler.scheduler().runnable().len(), 0);
        assert_eq!(scheduler.scheduler().counters().state_transitions(), 2);
        assert_eq!(scheduler.scheduler().counters().production_dispatches(), 1);
        assert_eq!(report.metadata().state(), TaskState::Running);
        assert!(report.metadata().current_on_owner());
        assert!(!report.metadata().runnable_on_owner());
    }

    #[test_case]
    fn cpu_local_scheduler_service_handles_timer_preemption_before_metadata_refresh() {
        let owner = LogicalCpuId::BOOT;
        let requester = LogicalCpuId::new(1);
        let mut scheduler = PerCoreScheduler::<2>::boot_cpu();
        let mut remote_wakes = RemoteWakeQueue::<2>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut current = Task::kernel_thread(task_id(1), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(2), kernel_stack(), context());
        current.set_state(TaskState::Running);
        next.set_state(TaskState::Blocked);

        scheduler
            .set_current_task(owner, current.id())
            .expect("boot CPU owns current task");
        metadata
            .register_local_task(owner, &scheduler, &next)
            .expect("next task starts in shared metadata");
        remote_wakes
            .publish(requester, owner, next.id())
            .expect("remote wake publication succeeds");

        let report = CpuLocalSchedulerService::run_cycle(
            owner,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut next,
            Some(&mut current),
            true,
            true,
        )
        .expect("timer service cycle completes");

        assert_eq!(report.remote_wake(), Some(next.id()));
        assert_eq!(report.timer_preemption(), Some(next.id()));
        assert_eq!(report.dispatch(), None);
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(next.state(), TaskState::Running);
        assert_eq!(scheduler.current_task(), Some(next.id()));
        assert_eq!(scheduler.scheduler().runnable().front(), Some(current.id()));
        assert_eq!(scheduler.scheduler().counters().timer_preemptions(), 1);
        assert_eq!(report.metadata().state(), TaskState::Running);
        assert!(report.metadata().current_on_owner());
        assert!(!report.metadata().runnable_on_owner());
    }

    #[test_case]
    fn per_core_preemption_state_records_only_local_timer_requests() {
        let owner = LogicalCpuId::new(2);
        let other = LogicalCpuId::new(3);
        let mut state = PerCorePreemptionState::new(owner);

        assert_eq!(state.owner(), owner);
        assert!(!state.pending_timer_request());
        assert_eq!(
            state.record_local_timer_irq(other),
            Err(PerCorePreemptionStateError::WrongOwner {
                owner,
                requester: other
            })
        );
        assert_eq!(
            state.record_local_timer_irq(owner),
            Ok(PreemptionRecordOutcome::Inserted)
        );
        assert_eq!(
            state.record_local_timer_irq(owner),
            Ok(PreemptionRecordOutcome::Coalesced)
        );

        assert!(state.pending_timer_request());
        assert_eq!(state.counters().recorded_requests(), 1);
        assert_eq!(state.counters().coalesced_requests(), 1);
        assert_eq!(state.counters().serviced_requests(), 0);
    }

    #[test_case]
    fn preemption_state_defers_service_while_disabled_and_balances_nested_sections() {
        let owner = LogicalCpuId::BOOT;
        let mut scheduler = PerCoreScheduler::<1>::boot_cpu();
        let mut preemption = PerCorePreemptionState::new(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut current = Task::kernel_thread(task_id(60), kernel_stack(), context());
        let mut local_task = Task::kernel_thread(task_id(61), kernel_stack(), context());
        current.set_state(TaskState::Running);

        scheduler
            .set_current_task(owner, current.id())
            .expect("owner sets current task");
        metadata
            .register_local_task(owner, &scheduler, &local_task)
            .expect("local task starts in metadata");
        preemption
            .record_local_timer_irq(owner)
            .expect("timer IRQ records local request");
        assert_eq!(preemption.enter_preemption_disabled(owner), Ok(1));
        assert_eq!(preemption.enter_preemption_disabled(owner), Ok(2));
        assert_eq!(preemption.disable_depth(), 2);

        assert_eq!(
            CpuLocalSchedulerService::run_preemption_cycle(
                owner,
                &mut scheduler,
                &mut preemption,
                &mut remote_wakes,
                &mut metadata,
                &mut local_task,
                Some(&mut current),
                false,
            ),
            Err(CpuLocalSchedulerServiceError::PreemptionState(
                PerCorePreemptionStateError::ServiceDeferred {
                    owner,
                    disable_depth: 2
                }
            ))
        );
        assert!(preemption.pending_timer_request());
        assert_eq!(preemption.counters().deferred_while_disabled(), 1);
        assert_eq!(current.state(), TaskState::Running);
        assert_eq!(scheduler.current_task(), Some(current.id()));

        assert_eq!(preemption.exit_preemption_disabled(owner), Ok(1));
        assert_eq!(preemption.exit_preemption_disabled(owner), Ok(0));
        assert_eq!(preemption.disable_depth(), 0);
        assert_eq!(
            preemption.exit_preemption_disabled(owner),
            Err(PerCorePreemptionStateError::DisableDepthUnderflow { owner })
        );
    }

    #[test_case]
    fn cpu_local_preemption_cycle_services_owner_local_pending_request() {
        let owner = LogicalCpuId::new(1);
        let requester = LogicalCpuId::BOOT;
        let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut preemption = PerCorePreemptionState::new(owner);
        let mut remote_wakes = RemoteWakeQueue::<2>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut current = Task::kernel_thread(task_id(62), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(63), kernel_stack(), context());
        current.set_state(TaskState::Running);
        next.set_state(TaskState::Blocked);

        scheduler
            .set_current_task(owner, current.id())
            .expect("owner sets current task");
        metadata
            .register_local_task(owner, &scheduler, &next)
            .expect("next task starts in metadata");
        remote_wakes
            .publish(requester, owner, next.id())
            .expect("remote wake publication succeeds");
        preemption
            .record_local_timer_irq(owner)
            .expect("local timer IRQ records bounded request");

        let report = CpuLocalSchedulerService::run_preemption_cycle(
            owner,
            &mut scheduler,
            &mut preemption,
            &mut remote_wakes,
            &mut metadata,
            &mut next,
            Some(&mut current),
            true,
        )
        .expect("owner-local preemption cycle completes");

        assert_eq!(report.remote_wake(), Some(next.id()));
        assert_eq!(report.timer_preemption(), Some(next.id()));
        assert_eq!(report.dispatch(), None);
        assert!(!preemption.pending_timer_request());
        assert_eq!(preemption.counters().serviced_requests(), 1);
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(next.state(), TaskState::Running);
        assert_eq!(scheduler.current_task(), Some(next.id()));
        assert_eq!(scheduler.scheduler().runnable().front(), Some(current.id()));
        assert!(report.metadata().current_on_owner());
        assert!(!report.metadata().runnable_on_owner());
    }

    #[test_case]
    fn cpu_local_preemption_cycle_rejects_wrong_owner_before_queue_mutation() {
        let owner = LogicalCpuId::new(1);
        let requester = LogicalCpuId::new(2);
        let mut scheduler = PerCoreScheduler::<1>::production_secondary_diagnostic(owner);
        let mut preemption = PerCorePreemptionState::new(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut current = Task::kernel_thread(task_id(64), kernel_stack(), context());
        let mut local_task = Task::kernel_thread(task_id(65), kernel_stack(), context());
        current.set_state(TaskState::Running);
        local_task.set_state(TaskState::Blocked);

        scheduler
            .set_current_task(owner, current.id())
            .expect("owner sets current task");
        metadata
            .register_local_task(owner, &scheduler, &local_task)
            .expect("local task starts in metadata");
        remote_wakes
            .publish(LogicalCpuId::BOOT, owner, local_task.id())
            .expect("remote wake publication succeeds");
        preemption
            .record_local_timer_irq(owner)
            .expect("local timer IRQ records request");

        assert_eq!(
            CpuLocalSchedulerService::run_preemption_cycle(
                requester,
                &mut scheduler,
                &mut preemption,
                &mut remote_wakes,
                &mut metadata,
                &mut local_task,
                Some(&mut current),
                true,
            ),
            Err(CpuLocalSchedulerServiceError::PreemptionState(
                PerCorePreemptionStateError::WrongOwner { owner, requester }
            ))
        );
        assert_eq!(remote_wakes.len(), 1);
        assert_eq!(local_task.state(), TaskState::Blocked);
        assert_eq!(current.state(), TaskState::Running);
        assert!(preemption.pending_timer_request());
    }

    #[test_case]
    fn cpu_local_preemption_cycle_rejects_mismatched_current_task_before_service() {
        let owner = LogicalCpuId::BOOT;
        let mut scheduler = PerCoreScheduler::<1>::boot_cpu();
        let mut preemption = PerCorePreemptionState::new(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut current = Task::kernel_thread(task_id(66), kernel_stack(), context());
        let mut provided = Task::kernel_thread(task_id(67), kernel_stack(), context());
        let mut local_task = Task::kernel_thread(task_id(68), kernel_stack(), context());
        current.set_state(TaskState::Running);
        provided.set_state(TaskState::Running);
        local_task.set_state(TaskState::Blocked);

        scheduler
            .set_current_task(owner, current.id())
            .expect("owner sets current task");
        metadata
            .register_local_task(owner, &scheduler, &local_task)
            .expect("local task starts in metadata");
        remote_wakes
            .publish(LogicalCpuId::new(1), owner, local_task.id())
            .expect("remote wake publication succeeds");
        preemption
            .record_local_timer_irq(owner)
            .expect("local timer IRQ records request");

        assert_eq!(
            CpuLocalSchedulerService::run_preemption_cycle(
                owner,
                &mut scheduler,
                &mut preemption,
                &mut remote_wakes,
                &mut metadata,
                &mut local_task,
                Some(&mut provided),
                true,
            ),
            Err(CpuLocalSchedulerServiceError::CurrentTaskMismatch {
                current_task: Some(current.id()),
                provided: provided.id()
            })
        );
        assert_eq!(remote_wakes.len(), 1);
        assert_eq!(local_task.state(), TaskState::Blocked);
        assert!(preemption.pending_timer_request());
        assert_eq!(preemption.counters().serviced_requests(), 0);
    }

    #[test_case]
    fn cpu_local_preemption_cycle_keeps_pending_request_after_no_runnable_error() {
        let owner = LogicalCpuId::BOOT;
        let mut scheduler = PerCoreScheduler::<1>::boot_cpu();
        let mut preemption = PerCorePreemptionState::new(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut current = Task::kernel_thread(task_id(69), kernel_stack(), context());
        let mut local_task = Task::kernel_thread(task_id(70), kernel_stack(), context());
        current.set_state(TaskState::Running);

        scheduler
            .set_current_task(owner, current.id())
            .expect("owner sets current task");
        metadata
            .register_local_task(owner, &scheduler, &local_task)
            .expect("local task starts in metadata");
        preemption
            .record_local_timer_irq(owner)
            .expect("local timer IRQ records request");

        assert_eq!(
            CpuLocalSchedulerService::run_preemption_cycle(
                owner,
                &mut scheduler,
                &mut preemption,
                &mut remote_wakes,
                &mut metadata,
                &mut local_task,
                Some(&mut current),
                false,
            ),
            Err(CpuLocalSchedulerServiceError::TimerPreempt(
                TimerPreemptError::NoRunnableTask
            ))
        );
        assert!(preemption.pending_timer_request());
        assert_eq!(preemption.counters().serviced_requests(), 0);
        assert_eq!(current.state(), TaskState::Running);
        assert_eq!(scheduler.current_task(), Some(current.id()));
    }

    #[test_case]
    fn production_scheduler_runtime_records_timer_irq_without_scheduler_mutation() {
        let owner = LogicalCpuId::BOOT;
        let mut runtime = ProductionSchedulerRuntime::<2, 1>::boot_cpu();

        assert_eq!(
            runtime.record_timer_irq::<4>(Some(owner.raw())),
            Ok(PreemptionRecordOutcome::Inserted)
        );
        assert_eq!(
            runtime.record_timer_irq::<4>(Some(owner.raw())),
            Ok(PreemptionRecordOutcome::Coalesced)
        );

        assert!(runtime.preemption_state().pending_timer_request());
        assert_eq!(runtime.preemption_state().counters().recorded_requests(), 1);
        assert_eq!(
            runtime.preemption_state().counters().coalesced_requests(),
            1
        );
        assert_eq!(runtime.scheduler().current_task(), None);
        assert_eq!(runtime.scheduler().scheduler().runnable().len(), 0);
        assert_eq!(runtime.remote_wake_queue().len(), 0);
        assert_eq!(runtime.timer_record_rejections(), 0);
    }

    #[test_case]
    fn production_scheduler_runtime_rejects_unowned_or_deferred_timer_irq_recording() {
        let owner = LogicalCpuId::new(1);
        let mut runtime = ProductionSchedulerRuntime::<1, 1>::deferred_secondary(owner);
        let diagnostic_runtime =
            ProductionSchedulerRuntime::<1, 1>::production_secondary_diagnostic(owner);

        assert_eq!(
            diagnostic_runtime.role(),
            SchedulerCoreRole::SecondaryProductionDiagnostic
        );

        assert_eq!(
            runtime.record_timer_irq::<4>(None),
            Err(ProductionTimerPreemptionRecordError::MissingLogicalCpu)
        );
        assert_eq!(
            runtime.record_timer_irq::<4>(Some(4)),
            Err(ProductionTimerPreemptionRecordError::InvalidLogicalCpu {
                logical_cpu: 4,
                cpu_capacity: 4
            })
        );
        assert_eq!(
            runtime.record_timer_irq::<4>(Some(LogicalCpuId::BOOT.raw())),
            Err(ProductionTimerPreemptionRecordError::WrongOwner {
                owner,
                requester: LogicalCpuId::BOOT
            })
        );
        assert_eq!(
            runtime.record_timer_irq::<4>(Some(owner.raw())),
            Err(ProductionTimerPreemptionRecordError::ProductionDispatchDeferred { owner })
        );

        assert!(!runtime.preemption_state().pending_timer_request());
        assert_eq!(runtime.timer_record_rejections(), 4);
        assert_eq!(runtime.scheduler().scheduler().runnable().len(), 0);
        assert_eq!(runtime.remote_wake_queue().len(), 0);
    }

    #[test_case]
    fn production_scheduler_runtime_services_pending_preemption_owner_locally() {
        let owner = LogicalCpuId::BOOT;
        let requester = LogicalCpuId::new(1);
        let mut runtime = ProductionSchedulerRuntime::<2, 1>::boot_cpu();
        let mut metadata = SharedSchedulerMetadata::<2, 4>::new();
        let mut current = Task::kernel_thread(task_id(71), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(72), kernel_stack(), context());
        current.set_state(TaskState::Running);
        next.set_state(TaskState::Blocked);

        runtime
            .scheduler_mut(owner)
            .expect("owner accesses scheduler")
            .set_current_task(owner, current.id())
            .expect("owner sets current task");
        metadata
            .register_local_task(owner, runtime.scheduler(), &next)
            .expect("next task starts in metadata");
        runtime
            .remote_wake_queue_mut(owner)
            .expect("owner accesses remote wake queue")
            .publish(requester, owner, next.id())
            .expect("remote wake publication succeeds");
        runtime
            .record_timer_irq::<4>(Some(owner.raw()))
            .expect("timer IRQ records bounded local state");

        let report = runtime
            .service_pending_preemption(owner, &mut metadata, &mut next, Some(&mut current), true)
            .expect("owner-local production service completes");

        assert_eq!(report.remote_wake(), Some(next.id()));
        assert_eq!(report.timer_preemption(), Some(next.id()));
        assert_eq!(report.dispatch(), None);
        assert!(!runtime.preemption_state().pending_timer_request());
        assert_eq!(runtime.preemption_state().counters().serviced_requests(), 1);
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(next.state(), TaskState::Running);
        assert_eq!(runtime.scheduler().current_task(), Some(next.id()));
        assert_eq!(
            runtime.scheduler().scheduler().runnable().front(),
            Some(current.id())
        );
        assert_eq!(runtime.remote_wake_queue().len(), 0);
        assert!(report.metadata().current_on_owner());
    }

    #[test_case]
    fn cpu_local_scheduler_service_preserves_explicit_error_boundaries() {
        let owner = LogicalCpuId::new(1);
        let mut deferred = PerCoreScheduler::<1>::deferred_secondary(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut task = Task::kernel_thread(task_id(40), kernel_stack(), context());

        metadata
            .register_local_task(owner, &deferred, &task)
            .expect("task starts in shared metadata");
        assert_eq!(
            CpuLocalSchedulerService::run_cycle(
                owner,
                &mut deferred,
                &mut remote_wakes,
                &mut metadata,
                &mut task,
                None,
                false,
                true,
            ),
            Err(CpuLocalSchedulerServiceError::ProductionDispatch(
                ProductionDispatchError::ProductionDispatchDeferred { owner }
            ))
        );

        let mut scheduler = PerCoreScheduler::<1>::production_secondary_diagnostic(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        assert_eq!(
            CpuLocalSchedulerService::run_cycle(
                owner,
                &mut scheduler,
                &mut remote_wakes,
                &mut metadata,
                &mut task,
                None,
                false,
                false,
            ),
            Err(CpuLocalSchedulerServiceError::Metadata(
                SharedSchedulerMetadataError::UnknownTask { task_id: task.id() }
            ))
        );

        let mut duplicate_scheduler = PerCoreScheduler::<1>::deferred_secondary(owner);
        let mut duplicate_metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut duplicate_task = Task::kernel_thread(task_id(41), kernel_stack(), context());
        duplicate_task.set_state(TaskState::Blocked);
        duplicate_scheduler
            .local_scheduler_mut(owner)
            .expect("owner mutates local queue")
            .make_runnable(&mut duplicate_task)
            .expect("local queue accepts duplicate setup task");
        duplicate_metadata
            .register_local_task(owner, &duplicate_scheduler, &duplicate_task)
            .expect("duplicate task starts in metadata");
        remote_wakes
            .publish(LogicalCpuId::BOOT, owner, duplicate_task.id())
            .expect("remote wake publication succeeds");
        assert_eq!(
            CpuLocalSchedulerService::run_cycle(
                owner,
                &mut duplicate_scheduler,
                &mut remote_wakes,
                &mut duplicate_metadata,
                &mut duplicate_task,
                None,
                false,
                false,
            ),
            Err(CpuLocalSchedulerServiceError::RemoteWake(
                TargetWakeConsumptionError::DuplicateLocalRunnable {
                    task_id: duplicate_task.id()
                }
            ))
        );

        let mut no_runnable_scheduler =
            PerCoreScheduler::<1>::production_secondary_diagnostic(owner);
        let mut no_runnable_metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut no_runnable_task = Task::kernel_thread(task_id(42), kernel_stack(), context());
        no_runnable_metadata
            .register_local_task(owner, &no_runnable_scheduler, &no_runnable_task)
            .expect("no-runnable task starts in metadata");
        let mut empty_wakes = RemoteWakeQueue::<1>::new(owner);
        assert_eq!(
            CpuLocalSchedulerService::run_cycle(
                owner,
                &mut no_runnable_scheduler,
                &mut empty_wakes,
                &mut no_runnable_metadata,
                &mut no_runnable_task,
                None,
                false,
                true,
            ),
            Err(CpuLocalSchedulerServiceError::ProductionDispatch(
                ProductionDispatchError::NoRunnableTask
            ))
        );
    }

    #[test_case]
    fn secondary_scheduler_service_loop_rejects_wrong_owner_before_consuming_wake() {
        let owner = LogicalCpuId::new(1);
        let requester = LogicalCpuId::new(2);
        let mut scheduler = PerCoreScheduler::<1>::production_secondary_diagnostic(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut task = Task::kernel_thread(task_id(50), kernel_stack(), context());
        task.set_state(TaskState::Blocked);
        metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("task starts in metadata");
        remote_wakes
            .publish(LogicalCpuId::BOOT, owner, task.id())
            .expect("remote wake publication succeeds");

        assert_eq!(
            SecondarySchedulerServiceLoop::run_once(
                requester,
                &mut scheduler,
                &mut remote_wakes,
                &mut metadata,
                &mut task,
                None,
                false,
                false,
            ),
            Err(SecondarySchedulerServiceLoopError::WrongOwner { owner, requester })
        );
        assert_eq!(remote_wakes.len(), 1);
        assert_eq!(task.state(), TaskState::Blocked);
    }

    #[test_case]
    fn secondary_scheduler_service_loop_rejects_deferred_secondary_role() {
        let owner = LogicalCpuId::new(1);
        let mut scheduler = PerCoreScheduler::<1>::deferred_secondary(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut task = Task::kernel_thread(task_id(51), kernel_stack(), context());
        metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("task starts in metadata");

        assert_eq!(
            SecondarySchedulerServiceLoop::run_once(
                owner,
                &mut scheduler,
                &mut remote_wakes,
                &mut metadata,
                &mut task,
                None,
                false,
                false,
            ),
            Err(SecondarySchedulerServiceLoopError::ProductionDispatchDeferred { owner })
        );
    }

    #[test_case]
    fn secondary_scheduler_service_loop_reports_no_work_after_metadata_refresh() {
        let owner = LogicalCpuId::new(1);
        let mut scheduler = PerCoreScheduler::<1>::production_secondary_diagnostic(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut task = Task::kernel_thread(task_id(52), kernel_stack(), context());
        metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("task starts in metadata");

        let report = SecondarySchedulerServiceLoop::run_once(
            owner,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            false,
        )
        .expect("idle service loop cycle refreshes metadata");

        assert!(!report.observed_remote_wake());
        assert!(!report.pending_timer_preemption());
        assert!(!report.dispatch_requested());
        assert!(!report.did_work());
        assert_eq!(report.cycle().metadata().task_id(), task.id());
        assert_eq!(report.cycle().metadata().generation(), 2);
    }

    #[test_case]
    fn secondary_scheduler_service_loop_consumes_remote_wake_without_dispatch() {
        let owner = LogicalCpuId::new(1);
        let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut task = Task::kernel_thread(task_id(53), kernel_stack(), context());
        task.set_state(TaskState::Blocked);
        metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("task starts in metadata");
        remote_wakes
            .publish(LogicalCpuId::BOOT, owner, task.id())
            .expect("remote wake publication succeeds");

        let report = SecondarySchedulerServiceLoop::run_once(
            owner,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            false,
        )
        .expect("remote wake service loop cycle completes");

        assert!(report.observed_remote_wake());
        assert!(report.did_work());
        assert_eq!(report.cycle().remote_wake(), Some(task.id()));
        assert_eq!(report.cycle().dispatch(), None);
        assert_eq!(task.state(), TaskState::Runnable);
        assert!(scheduler.scheduler().runnable().contains(task.id()));
        assert!(report.cycle().metadata().runnable_on_owner());
        assert!(remote_wakes.is_empty());
    }

    #[test_case]
    fn secondary_scheduler_service_loop_dispatches_local_runnable_task() {
        let owner = LogicalCpuId::new(2);
        let mut scheduler = PerCoreScheduler::<1>::production_secondary_diagnostic(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut task = Task::kernel_thread(task_id(54), kernel_stack(), context());
        scheduler
            .local_scheduler_mut(owner)
            .expect("owner mutates local scheduler")
            .make_runnable(&mut task)
            .expect("local queue accepts task");
        metadata
            .register_local_task(owner, &scheduler, &task)
            .expect("task starts in metadata");

        let report = SecondarySchedulerServiceLoop::run_once(
            owner,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            true,
        )
        .expect("dispatch service loop cycle completes");

        assert!(report.dispatch_requested());
        assert!(report.did_work());
        assert_eq!(report.cycle().dispatch(), Some(task.id()));
        assert_eq!(task.state(), TaskState::Running);
        assert_eq!(scheduler.current_task(), Some(task.id()));
        assert!(report.cycle().metadata().current_on_owner());
    }

    #[test_case]
    fn secondary_scheduler_service_loop_handles_timer_preemption_before_dispatch() {
        let owner = LogicalCpuId::new(3);
        let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
        let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
        let mut metadata = SharedSchedulerMetadata::<1, 4>::new();
        let mut current = Task::kernel_thread(task_id(55), kernel_stack(), context());
        let mut next = Task::kernel_thread(task_id(56), kernel_stack(), context());
        current.set_state(TaskState::Running);
        next.set_state(TaskState::Blocked);
        scheduler
            .set_current_task(owner, current.id())
            .expect("owner sets current task");
        metadata
            .register_local_task(owner, &scheduler, &next)
            .expect("next task starts in metadata");
        remote_wakes
            .publish(LogicalCpuId::BOOT, owner, next.id())
            .expect("remote wake publication succeeds");

        let report = SecondarySchedulerServiceLoop::run_once(
            owner,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut next,
            Some(&mut current),
            true,
            true,
        )
        .expect("timer-preemption service loop cycle completes");

        assert!(report.pending_timer_preemption());
        assert!(report.did_work());
        assert_eq!(report.cycle().remote_wake(), Some(next.id()));
        assert_eq!(report.cycle().timer_preemption(), Some(next.id()));
        assert_eq!(report.cycle().dispatch(), None);
        assert_eq!(current.state(), TaskState::Runnable);
        assert_eq!(next.state(), TaskState::Running);
        assert_eq!(scheduler.current_task(), Some(next.id()));
        assert_eq!(scheduler.scheduler().runnable().front(), Some(current.id()));
        assert!(report.cycle().metadata().current_on_owner());
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
