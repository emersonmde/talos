//! Target-independent live address-space activation preflight.
//!
//! This module consumes accepted Phase 8 image/install/address-space,
//! materialization, launch, and stack records and emits an inspectable
//! activation plan only. It does not write translation registers, invalidate
//! live TLB state, allocate an ASID, publish scheduler state, or enter lower
//! EL.

use crate::{
    initial_process_launch::{
        INITIAL_ACTIVATION_BLOCKED, INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        InitialProcessLaunchPlan, InitialProcessLaunchSideEffects,
    },
    initial_user_stack::{
        INITIAL_USER_STACK_BOUNDARY_IDENTITY, INITIAL_USER_STACK_READY, InitialUserStackPlan,
    },
    posix::{PosixError, USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, UserMappingPermissions},
    process_address_space::{PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace},
    process_install::{PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan},
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessPageDescriptorRecord,
        ProcessPageTableMaterialization,
    },
    program_loader::{LOADER_PAGE_SIZE, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan},
};

pub(crate) const LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY: &str =
    "phase8-live-address-space-activation-plan-v1";
pub(crate) const LIVE_ADDRESS_SPACE_ACTIVATION_POLICY: &str =
    "preflight-split-user-ttbr0-kernel-reachability-blocked-v1";
pub(crate) const TTBR0_ROOT_PROVENANCE: &str = "materialized-process-root-lease";
pub(crate) const TTBR1_KERNEL_POLICY_BLOCKED: &str = "blocked-no-accepted-kernel-half-map";
pub(crate) const TCR_COMPATIBILITY_RECORD_ONLY: &str = "compatibility-record-only";
pub(crate) const MAIR_COMPATIBILITY_RECORD_ONLY: &str = "compatibility-record-only";
pub(crate) const SCTLR_MUTATION_BLOCKED: &str = "mutation-blocked";
pub(crate) const ASID_ALLOCATION_BLOCKED: &str = "blocked-no-asid-allocation";
pub(crate) const TLB_INVALIDATION_BLOCKED: &str = "blocked-no-live-tlbi";
pub(crate) const BARRIER_SEQUENCE_PLANNED_ONLY: &str = "planned-only-no-live-dsb-isb";
pub(crate) const LIVE_REGISTER_SEQUENCE_BLOCKED: &str = "blocked-no-live-register-sequence";
pub(crate) const LOWER_EL_ERET_BLOCKED: &str = "blocked-no-lower-el-eret";
pub(crate) const RUNNABLE_PUBLICATION_BLOCKED: &str = "blocked-no-runnable-publication";
pub(crate) const PROCESS_LIFECYCLE_BLOCKED: &str = "blocked-no-process-lifecycle";
pub(crate) const STARTUP_ABI_EXPANSION_BLOCKED: &str = "blocked-no-startup-abi-expansion";
pub(crate) const FILESYSTEM_SYSCALLS_BLOCKED: &str = "blocked-no-filesystem-syscalls";
pub(crate) const PI5_HARDWARE_PROOF_BLOCKED: &str = "blocked-no-pi5-hardware-proof";
pub(crate) const ACTIVATION_PREFLIGHT_READY: &str = "model-only-activation-preflight-ready";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveAddressSpaceActivationRequest {
    PreflightOnly,
    LiveRegisterSequence,
    PublishSchedulerRunnable,
    LowerElLaunch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveAddressSpaceActivationCommitTarget {
    LiveRegisters,
    Runnable,
    LowerEl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveAddressSpaceActivationCommitRejection {
    error: PosixError,
    no_partial_activation: bool,
    no_runnable_publication: bool,
}

impl LiveAddressSpaceActivationCommitRejection {
    pub(crate) const fn error(self) -> PosixError {
        self.error
    }

    pub(crate) const fn no_partial_activation(self) -> bool {
        self.no_partial_activation
    }

    pub(crate) const fn no_runnable_publication(self) -> bool {
        self.no_runnable_publication
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ActivationPlanToken(u64);

impl ActivationPlanToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveAddressSpaceActivationLeaseSnapshot {
    pub(crate) plan_records_leased: usize,
    pub(crate) plan_record_releases: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveAddressSpaceActivationLeaseSource {
    plan_record_capacity: usize,
    next_token: u64,
    plan_records_leased: usize,
    plan_record_releases: usize,
}

impl LiveAddressSpaceActivationLeaseSource {
    pub(crate) const fn with_plan_record_capacity(plan_record_capacity: usize) -> Self {
        Self {
            plan_record_capacity,
            next_token: 1,
            plan_records_leased: 0,
            plan_record_releases: 0,
        }
    }

    pub(crate) const fn for_single_plan() -> Self {
        Self::with_plan_record_capacity(1)
    }

    pub(crate) const fn snapshot(self) -> LiveAddressSpaceActivationLeaseSnapshot {
        LiveAddressSpaceActivationLeaseSnapshot {
            plan_records_leased: self.plan_records_leased,
            plan_record_releases: self.plan_record_releases,
        }
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        self.plan_records_leased
    }

    fn lease_plan_record(&mut self) -> Result<ActivationPlanToken, PosixError> {
        if self.plan_records_leased == self.plan_record_capacity {
            return Err(PosixError::NoMemory);
        }
        let token = ActivationPlanToken(self.next_token);
        self.next_token += 1;
        self.plan_records_leased += 1;
        Ok(token)
    }

    fn release_plan_record(&mut self, token: &mut LiveAddressSpaceActivationPlanLease) {
        if !token.released {
            token.released = true;
            if self.plan_records_leased != 0 {
                self.plan_records_leased -= 1;
            }
            self.plan_record_releases += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveAddressSpaceActivationPlanLease {
    token: ActivationPlanToken,
    released: bool,
}

impl LiveAddressSpaceActivationPlanLease {
    pub(crate) const fn token(self) -> ActivationPlanToken {
        self.token
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ActivationRootProvenance {
    state: &'static str,
    materialization_id: u64,
    root_token: u64,
    root_physical_frame: u64,
    ttbr0_written: bool,
}

impl ActivationRootProvenance {
    pub(crate) const fn state(self) -> &'static str {
        self.state
    }

    pub(crate) const fn materialization_id(self) -> u64 {
        self.materialization_id
    }

    pub(crate) const fn root_token(self) -> u64 {
        self.root_token
    }

    pub(crate) const fn root_physical_frame(self) -> u64 {
        self.root_physical_frame
    }

    pub(crate) const fn ttbr0_written(self) -> bool {
        self.ttbr0_written
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelReachabilityChecklist {
    vbar_el1: bool,
    exception_vectors: bool,
    active_kernel_stack: bool,
    kernel_text_data: bool,
    allocator: bool,
    uart_mmio_diagnostics: bool,
    scheduler_code_data: bool,
    panic_fault_reporting: bool,
}

impl KernelReachabilityChecklist {
    pub(crate) const REQUIRED: Self = Self {
        vbar_el1: true,
        exception_vectors: true,
        active_kernel_stack: true,
        kernel_text_data: true,
        allocator: true,
        uart_mmio_diagnostics: true,
        scheduler_code_data: true,
        panic_fault_reporting: true,
    };

    pub(crate) const fn vbar_el1(self) -> bool {
        self.vbar_el1
    }

    pub(crate) const fn exception_vectors(self) -> bool {
        self.exception_vectors
    }

    pub(crate) const fn active_kernel_stack(self) -> bool {
        self.active_kernel_stack
    }

    pub(crate) const fn kernel_text_data(self) -> bool {
        self.kernel_text_data
    }

    pub(crate) const fn allocator(self) -> bool {
        self.allocator
    }

    pub(crate) const fn uart_mmio_diagnostics(self) -> bool {
        self.uart_mmio_diagnostics
    }

    pub(crate) const fn scheduler_code_data(self) -> bool {
        self.scheduler_code_data
    }

    pub(crate) const fn panic_fault_reporting(self) -> bool {
        self.panic_fault_reporting
    }

    const fn all_required(self) -> bool {
        self.vbar_el1
            && self.exception_vectors
            && self.active_kernel_stack
            && self.kernel_text_data
            && self.allocator
            && self.uart_mmio_diagnostics
            && self.scheduler_code_data
            && self.panic_fault_reporting
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveAddressSpaceActivationSideEffects {
    ttbr_mutated: bool,
    tcr_mutated: bool,
    mair_mutated: bool,
    sctlr_mutated: bool,
    asid_allocated: bool,
    tlb_mutated: bool,
    live_dsb_isb: bool,
    lower_el_eret: bool,
    scheduler_published: bool,
    process_table_mutated: bool,
    descriptor_table_mutated: bool,
}

impl LiveAddressSpaceActivationSideEffects {
    pub(crate) const NONE: Self = Self {
        ttbr_mutated: false,
        tcr_mutated: false,
        mair_mutated: false,
        sctlr_mutated: false,
        asid_allocated: false,
        tlb_mutated: false,
        live_dsb_isb: false,
        lower_el_eret: false,
        scheduler_published: false,
        process_table_mutated: false,
        descriptor_table_mutated: false,
    };

    pub(crate) const fn ttbr_mutated(self) -> bool {
        self.ttbr_mutated
    }

    pub(crate) const fn tcr_mutated(self) -> bool {
        self.tcr_mutated
    }

    pub(crate) const fn mair_mutated(self) -> bool {
        self.mair_mutated
    }

    pub(crate) const fn sctlr_mutated(self) -> bool {
        self.sctlr_mutated
    }

    pub(crate) const fn asid_allocated(self) -> bool {
        self.asid_allocated
    }

    pub(crate) const fn tlb_mutated(self) -> bool {
        self.tlb_mutated
    }

    pub(crate) const fn live_dsb_isb(self) -> bool {
        self.live_dsb_isb
    }

    pub(crate) const fn lower_el_eret(self) -> bool {
        self.lower_el_eret
    }

    pub(crate) const fn scheduler_published(self) -> bool {
        self.scheduler_published
    }

    pub(crate) const fn process_table_mutated(self) -> bool {
        self.process_table_mutated
    }

    pub(crate) const fn descriptor_table_mutated(self) -> bool {
        self.descriptor_table_mutated
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveAddressSpaceActivationTeardownReport {
    plan_record_released: bool,
    materialization_owned: bool,
    launch_owned: bool,
    stack_owned: bool,
    image_owned: bool,
    already_destroyed: bool,
}

impl LiveAddressSpaceActivationTeardownReport {
    pub(crate) const fn plan_record_released(self) -> bool {
        self.plan_record_released
    }

    pub(crate) const fn materialization_owned(self) -> bool {
        self.materialization_owned
    }

    pub(crate) const fn launch_owned(self) -> bool {
        self.launch_owned
    }

    pub(crate) const fn stack_owned(self) -> bool {
        self.stack_owned
    }

    pub(crate) const fn image_owned(self) -> bool {
        self.image_owned
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveAddressSpaceActivationPlan {
    boundary_identity: &'static str,
    activation_policy: &'static str,
    image_fixture_identity: &'static str,
    install_boundary_identity: &'static str,
    address_space_boundary_identity: &'static str,
    materialization_boundary_identity: &'static str,
    launch_boundary_identity: &'static str,
    stack_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    address_space_id: u64,
    materialization_id: u64,
    entry_pc: u64,
    initial_sp: u64,
    plan_lease: LiveAddressSpaceActivationPlanLease,
    root_provenance: ActivationRootProvenance,
    ttbr1_kernel_policy: &'static str,
    tcr_state: &'static str,
    mair_state: &'static str,
    sctlr_state: &'static str,
    asid_state: &'static str,
    tlb_state: &'static str,
    barrier_state: &'static str,
    live_register_sequence_state: &'static str,
    lower_el_eret_state: &'static str,
    runnable_publication_state: &'static str,
    process_lifecycle_state: &'static str,
    startup_abi_state: &'static str,
    filesystem_syscall_state: &'static str,
    pi5_hardware_proof_state: &'static str,
    launch_activation_state: &'static str,
    kernel_reachability: KernelReachabilityChecklist,
    side_effects: LiveAddressSpaceActivationSideEffects,
    published: bool,
    destroyed: bool,
}

impl LiveAddressSpaceActivationPlan {
    pub(crate) const fn boundary_identity(self) -> &'static str {
        self.boundary_identity
    }

    pub(crate) const fn activation_policy(self) -> &'static str {
        self.activation_policy
    }

    pub(crate) const fn image_fixture_identity(self) -> &'static str {
        self.image_fixture_identity
    }

    pub(crate) const fn install_boundary_identity(self) -> &'static str {
        self.install_boundary_identity
    }

    pub(crate) const fn address_space_boundary_identity(self) -> &'static str {
        self.address_space_boundary_identity
    }

    pub(crate) const fn materialization_boundary_identity(self) -> &'static str {
        self.materialization_boundary_identity
    }

    pub(crate) const fn launch_boundary_identity(self) -> &'static str {
        self.launch_boundary_identity
    }

    pub(crate) const fn stack_boundary_identity(self) -> &'static str {
        self.stack_boundary_identity
    }

    pub(crate) const fn source_path(self) -> &'static [u8] {
        self.source_path
    }

    pub(crate) const fn source_digest(self) -> u64 {
        self.source_digest
    }

    pub(crate) const fn address_space_id(self) -> u64 {
        self.address_space_id
    }

    pub(crate) const fn materialization_id(self) -> u64 {
        self.materialization_id
    }

    pub(crate) const fn entry_pc(self) -> u64 {
        self.entry_pc
    }

    pub(crate) const fn initial_sp(self) -> u64 {
        self.initial_sp
    }

    pub(crate) const fn plan_lease(self) -> LiveAddressSpaceActivationPlanLease {
        self.plan_lease
    }

    pub(crate) const fn root_provenance(self) -> ActivationRootProvenance {
        self.root_provenance
    }

    pub(crate) const fn ttbr1_kernel_policy(self) -> &'static str {
        self.ttbr1_kernel_policy
    }

    pub(crate) const fn tcr_state(self) -> &'static str {
        self.tcr_state
    }

    pub(crate) const fn mair_state(self) -> &'static str {
        self.mair_state
    }

    pub(crate) const fn sctlr_state(self) -> &'static str {
        self.sctlr_state
    }

    pub(crate) const fn asid_state(self) -> &'static str {
        self.asid_state
    }

    pub(crate) const fn tlb_state(self) -> &'static str {
        self.tlb_state
    }

    pub(crate) const fn barrier_state(self) -> &'static str {
        self.barrier_state
    }

    pub(crate) const fn live_register_sequence_state(self) -> &'static str {
        self.live_register_sequence_state
    }

    pub(crate) const fn lower_el_eret_state(self) -> &'static str {
        self.lower_el_eret_state
    }

    pub(crate) const fn runnable_publication_state(self) -> &'static str {
        self.runnable_publication_state
    }

    pub(crate) const fn process_lifecycle_state(self) -> &'static str {
        self.process_lifecycle_state
    }

    pub(crate) const fn startup_abi_state(self) -> &'static str {
        self.startup_abi_state
    }

    pub(crate) const fn filesystem_syscall_state(self) -> &'static str {
        self.filesystem_syscall_state
    }

    pub(crate) const fn pi5_hardware_proof_state(self) -> &'static str {
        self.pi5_hardware_proof_state
    }

    pub(crate) const fn launch_activation_state(self) -> &'static str {
        self.launch_activation_state
    }

    pub(crate) const fn kernel_reachability(self) -> KernelReachabilityChecklist {
        self.kernel_reachability
    }

    pub(crate) const fn side_effects(self) -> LiveAddressSpaceActivationSideEffects {
        self.side_effects
    }

    pub(crate) const fn published(self) -> bool {
        self.published
    }

    pub(crate) const fn destroyed(self) -> bool {
        self.destroyed
    }

    pub(crate) const fn commit_request(
        self,
        _target: LiveAddressSpaceActivationCommitTarget,
    ) -> Result<(), LiveAddressSpaceActivationCommitRejection> {
        Err(LiveAddressSpaceActivationCommitRejection {
            error: PosixError::NotImplemented,
            no_partial_activation: true,
            no_runnable_publication: true,
        })
    }

    pub(crate) fn destroy(
        &mut self,
        lease_source: &mut LiveAddressSpaceActivationLeaseSource,
    ) -> LiveAddressSpaceActivationTeardownReport {
        if self.destroyed {
            return LiveAddressSpaceActivationTeardownReport {
                plan_record_released: false,
                materialization_owned: true,
                launch_owned: true,
                stack_owned: true,
                image_owned: true,
                already_destroyed: true,
            };
        }

        lease_source.release_plan_record(&mut self.plan_lease);
        self.published = false;
        self.destroyed = true;

        LiveAddressSpaceActivationTeardownReport {
            plan_record_released: true,
            materialization_owned: true,
            launch_owned: true,
            stack_owned: true,
            image_owned: true,
            already_destroyed: false,
        }
    }
}

pub(crate) fn preflight_live_address_space_activation(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    launch_plan: InitialProcessLaunchPlan,
    stack_plan: InitialUserStackPlan,
    request: LiveAddressSpaceActivationRequest,
    lease_source: &mut LiveAddressSpaceActivationLeaseSource,
) -> Result<LiveAddressSpaceActivationPlan, PosixError> {
    if request != LiveAddressSpaceActivationRequest::PreflightOnly {
        return Err(PosixError::NotImplemented);
    }

    validate_lineage(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
    )?;
    validate_descriptor_stack_and_entry(
        image,
        install_plan,
        address_space,
        materialization,
        stack_plan,
    )?;
    let kernel_reachability = KernelReachabilityChecklist::REQUIRED;
    if !kernel_reachability.all_required() {
        return Err(PosixError::InvalidArgument);
    }

    let plan_lease = LiveAddressSpaceActivationPlanLease {
        token: lease_source.lease_plan_record()?,
        released: false,
    };

    Ok(LiveAddressSpaceActivationPlan {
        boundary_identity: LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        activation_policy: LIVE_ADDRESS_SPACE_ACTIVATION_POLICY,
        image_fixture_identity: image.fixture_identity(),
        install_boundary_identity: install_plan.install_boundary_identity(),
        address_space_boundary_identity: address_space.boundary_identity(),
        materialization_boundary_identity: materialization.boundary_identity(),
        launch_boundary_identity: launch_plan.boundary_identity(),
        stack_boundary_identity: stack_plan.boundary_identity(),
        source_path: image.source_path(),
        source_digest: image.source_digest(),
        address_space_id: address_space.id().raw(),
        materialization_id: materialization.id(),
        entry_pc: image.entry(),
        initial_sp: stack_plan.layout().initial_sp(),
        plan_lease,
        root_provenance: ActivationRootProvenance {
            state: TTBR0_ROOT_PROVENANCE,
            materialization_id: materialization.id(),
            root_token: materialization.root().token().raw(),
            root_physical_frame: materialization.root().physical_frame(),
            ttbr0_written: false,
        },
        ttbr1_kernel_policy: TTBR1_KERNEL_POLICY_BLOCKED,
        tcr_state: TCR_COMPATIBILITY_RECORD_ONLY,
        mair_state: MAIR_COMPATIBILITY_RECORD_ONLY,
        sctlr_state: SCTLR_MUTATION_BLOCKED,
        asid_state: ASID_ALLOCATION_BLOCKED,
        tlb_state: TLB_INVALIDATION_BLOCKED,
        barrier_state: BARRIER_SEQUENCE_PLANNED_ONLY,
        live_register_sequence_state: LIVE_REGISTER_SEQUENCE_BLOCKED,
        lower_el_eret_state: LOWER_EL_ERET_BLOCKED,
        runnable_publication_state: RUNNABLE_PUBLICATION_BLOCKED,
        process_lifecycle_state: PROCESS_LIFECYCLE_BLOCKED,
        startup_abi_state: STARTUP_ABI_EXPANSION_BLOCKED,
        filesystem_syscall_state: FILESYSTEM_SYSCALLS_BLOCKED,
        pi5_hardware_proof_state: PI5_HARDWARE_PROOF_BLOCKED,
        launch_activation_state: ACTIVATION_PREFLIGHT_READY,
        kernel_reachability,
        side_effects: LiveAddressSpaceActivationSideEffects::NONE,
        published: true,
        destroyed: false,
    })
}

fn validate_lineage(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    launch_plan: InitialProcessLaunchPlan,
    stack_plan: InitialUserStackPlan,
) -> Result<(), PosixError> {
    if image.fixture_identity() != PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        || install_plan.fixture_identity() != image.fixture_identity()
        || install_plan.install_boundary_identity() != PROCESS_INSTALL_BOUNDARY_IDENTITY
        || install_plan.source_path() != image.source_path()
        || install_plan.source_digest() != image.source_digest()
        || install_plan.entry() != image.entry()
        || !install_plan.lower_el_launch_blocked()
        || address_space.boundary_identity() != PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        || !address_space.published()
        || address_space.destroyed()
        || address_space.root().released()
        || address_space.mapping_count() != install_plan.page_count()
        || materialization.boundary_identity()
            != PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        || materialization.kernel_mapping_policy() != PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY
        || materialization.source_digest() != image.source_digest()
        || !materialization.published()
        || materialization.destroyed()
        || !materialization.activation_blocked()
        || materialization.root().released()
        || materialization.descriptor_count() != address_space.mapping_count()
        || materialization.user_frame_count() != address_space.user_frame_lease_count()
        || launch_plan.boundary_identity() != INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
        || launch_plan.image_fixture_identity() != image.fixture_identity()
        || launch_plan.install_boundary_identity() != install_plan.install_boundary_identity()
        || launch_plan.address_space_boundary_identity() != address_space.boundary_identity()
        || launch_plan.materialization_boundary_identity() != materialization.boundary_identity()
        || launch_plan.source_path() != image.source_path()
        || launch_plan.source_digest() != image.source_digest()
        || launch_plan.address_space_id() != address_space.id().raw()
        || launch_plan.materialization_id() != materialization.id()
        || launch_plan.entry_pc() != image.entry()
        || launch_plan.activation_state() != INITIAL_ACTIVATION_BLOCKED
        || launch_plan.side_effects() != InitialProcessLaunchSideEffects::NONE
        || !launch_plan.published()
        || stack_plan.boundary_identity() != INITIAL_USER_STACK_BOUNDARY_IDENTITY
        || stack_plan.image_fixture_identity() != image.fixture_identity()
        || stack_plan.install_boundary_identity() != install_plan.install_boundary_identity()
        || stack_plan.address_space_boundary_identity() != address_space.boundary_identity()
        || stack_plan.materialization_boundary_identity() != materialization.boundary_identity()
        || stack_plan.launch_boundary_identity() != launch_plan.boundary_identity()
        || stack_plan.source_path() != image.source_path()
        || stack_plan.source_digest() != image.source_digest()
        || stack_plan.address_space_id() != address_space.id().raw()
        || stack_plan.materialization_id() != materialization.id()
        || stack_plan.entry_pc() != image.entry()
        || !stack_plan.published()
        || stack_plan.destroyed()
    {
        return Err(PosixError::InvalidArgument);
    }

    let binding = stack_plan.launch_binding();
    if binding.user_sp_state() != INITIAL_USER_STACK_READY
        || binding.saved_frame_sp_el0() != stack_plan.layout().initial_sp()
        || binding.activation_state() != INITIAL_ACTIVATION_BLOCKED
        || !binding.no_partial_launch()
        || !binding.no_runnable_publication()
        || binding.side_effects() != InitialProcessLaunchSideEffects::NONE
    {
        return Err(PosixError::NotExecutable);
    }
    Ok(())
}

fn validate_descriptor_stack_and_entry(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    stack_plan: InitialUserStackPlan,
) -> Result<(), PosixError> {
    validate_user_range(image.entry(), 4)?;
    let entry_page = image.entry() & !(LOADER_PAGE_SIZE - 1);
    let mapping = entry_mapping(address_space, image.entry())?;
    let descriptor = descriptor_for_virtual_page(materialization, entry_page)?;
    if mapping.virtual_start() != entry_page
        || !mapping
            .permissions()
            .contains(UserMappingPermissions::EXECUTE)
        || descriptor.virtual_page() != entry_page
        || !descriptor.executable()
        || descriptor.user_execute_never()
    {
        return Err(PosixError::NotExecutable);
    }
    let text_page = install_page_for_entry(install_plan, image.entry())?;
    if text_page.virtual_start() != entry_page
        || !text_page
            .permissions()
            .contains(UserMappingPermissions::EXECUTE)
    {
        return Err(PosixError::NotExecutable);
    }

    let layout = stack_plan.layout();
    validate_user_range(
        layout.guard_start(),
        (layout.usable_end() - layout.guard_start()) as usize,
    )?;
    if !layout.sp_aligned_16()
        || layout.initial_sp() != layout.usable_end()
        || layout.permissions() != UserMappingPermissions::USER_DATA
        || stack_plan.page_lease_count() == 0
    {
        return Err(PosixError::NotExecutable);
    }

    let mut index = 0;
    while index < stack_plan.page_lease_count() {
        let lease = stack_plan
            .page_lease(index)
            .ok_or(PosixError::InvalidArgument)?;
        validate_user_range(lease.virtual_page(), LOADER_PAGE_SIZE as usize)?;
        if lease.permissions() != UserMappingPermissions::USER_DATA
            || !lease.zeroed_before_copy()
            || lease.released()
        {
            return Err(PosixError::NotExecutable);
        }
        index += 1;
    }

    Ok(())
}

fn entry_mapping(
    address_space: ProcessAddressSpace,
    entry: u64,
) -> Result<crate::process_address_space::ProcessUserMapping, PosixError> {
    let mut index = 0;
    while index < address_space.mapping_count() {
        let mapping = address_space
            .mapping(index)
            .ok_or(PosixError::InvalidArgument)?;
        if mapping.virtual_start() <= entry && entry < mapping.virtual_end() {
            return Ok(mapping);
        }
        index += 1;
    }
    Err(PosixError::NotExecutable)
}

fn install_page_for_entry(
    install_plan: ProcessImageInstallPlan,
    entry: u64,
) -> Result<crate::process_install::ProcessImagePageInstallRecord, PosixError> {
    let mut index = 0;
    while index < install_plan.page_count() {
        let page = install_plan
            .page(index)
            .ok_or(PosixError::InvalidArgument)?;
        if page.virtual_start() <= entry && entry < page.virtual_end() {
            return Ok(page);
        }
        index += 1;
    }
    Err(PosixError::NotExecutable)
}

fn descriptor_for_virtual_page(
    materialization: ProcessPageTableMaterialization,
    virtual_page: u64,
) -> Result<ProcessPageDescriptorRecord, PosixError> {
    let mut index = 0;
    while index < materialization.descriptor_count() {
        let descriptor = materialization
            .descriptor(index)
            .ok_or(PosixError::InvalidArgument)?;
        if descriptor.virtual_page() == virtual_page {
            return Ok(descriptor);
        }
        index += 1;
    }
    Err(PosixError::NotExecutable)
}

fn validate_user_range(start: u64, len: usize) -> Result<(), PosixError> {
    let end = start
        .checked_add(len as u64)
        .ok_or(PosixError::AccessDenied)?;
    if start < USER_NULL_GUARD_END || end > USER_ADDRESS_SPACE_END || end < start {
        return Err(PosixError::AccessDenied);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initial_process_launch::{InitialProcessLaunchRequest, prepare_initial_process_launch},
        initial_user_stack::{
            InitialUserStackLeaseSource, InitialUserStackRequest, plan_initial_user_stack,
        },
        initramfs::{PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
        process_address_space::{
            ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource, install_process_address_space,
        },
        process_install::{ProcessInstallSideEffects, plan_process_image_install},
        process_page_table_materialization::{
            ProcessMaterializationRequest, ProcessPageTableMaterializationLeaseSource,
            materialize_process_page_tables,
        },
        program_loader::{MAX_LOAD_SEGMENTS, plan_phase8_init_image},
        scheduler::ProcessOwnerId,
    };

    fn fixture() -> (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        InitialProcessLaunchPlan,
        InitialUserStackPlan,
    ) {
        let image =
            plan_phase8_init_image(phase8_readonly_initramfs_fixture()).expect("program image");
        let install_plan = plan_process_image_install(image).expect("install plan");
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(0x8800_3001).expect("address-space id"),
            Some(ProcessOwnerId::new(0x8800_3002).expect("owner id")),
            &mut address_source,
        )
        .expect("address space");
        let mut materialization_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
        let materialization = materialize_process_page_tables(
            image,
            install_plan,
            address_space,
            ProcessMaterializationRequest::DescriptorImageOnly,
            &mut materialization_source,
        )
        .expect("materialization");
        let launch_plan = prepare_initial_process_launch(
            image,
            install_plan,
            address_space,
            materialization,
            InitialProcessLaunchRequest::PreparePlanOnly,
        )
        .expect("launch plan");
        let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
        let stack_plan = plan_initial_user_stack(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            InitialUserStackRequest::PlanOnly,
            &mut stack_source,
        )
        .expect("stack plan");
        (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
        )
    }

    fn image_with_identity(identity: &'static str) -> ProgramImagePlan {
        let image =
            plan_phase8_init_image(phase8_readonly_initramfs_fixture()).expect("program image");
        let mut segments = [None; MAX_LOAD_SEGMENTS];
        let mut index = 0;
        while index < image.segment_count() {
            segments[index] = image.segment(index);
            index += 1;
        }
        ProgramImagePlan::for_test_unchecked(
            image.source_path(),
            identity,
            image.source_len(),
            image.source_digest(),
            image.entry(),
            image.segment_count(),
            segments,
            image.memory_start(),
            image.memory_end(),
            image.memory_footprint(),
        )
    }

    fn install_with_entry(
        install_plan: ProcessImageInstallPlan,
        entry: u64,
    ) -> ProcessImageInstallPlan {
        let mut pages = [None; crate::process_install::MAX_PROCESS_INSTALL_PAGES];
        let mut index = 0;
        while index < install_plan.page_count() {
            pages[index] = install_plan.page(index);
            index += 1;
        }
        ProcessImageInstallPlan::for_test_unchecked(
            install_plan.fixture_identity(),
            install_plan.install_boundary_identity(),
            install_plan.source_path(),
            install_plan.source_digest(),
            entry,
            install_plan.memory_footprint(),
            install_plan.page_count(),
            pages,
            ProcessInstallSideEffects::NONE,
            install_plan.lower_el_launch_blocked(),
        )
    }

    #[test_case]
    fn builds_preflight_plan_with_blocked_live_activation_fields() {
        let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
            fixture();
        let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();

        let plan = preflight_live_address_space_activation(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            LiveAddressSpaceActivationRequest::PreflightOnly,
            &mut activation_source,
        )
        .expect("activation preflight");

        assert_eq!(
            plan.boundary_identity(),
            LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        );
        assert_eq!(
            plan.activation_policy(),
            LIVE_ADDRESS_SPACE_ACTIVATION_POLICY
        );
        assert_eq!(
            plan.image_fixture_identity(),
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        );
        assert_eq!(
            plan.install_boundary_identity(),
            PROCESS_INSTALL_BOUNDARY_IDENTITY
        );
        assert_eq!(
            plan.address_space_boundary_identity(),
            PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        );
        assert_eq!(
            plan.materialization_boundary_identity(),
            PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        );
        assert_eq!(
            plan.launch_boundary_identity(),
            INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
        );
        assert_eq!(
            plan.stack_boundary_identity(),
            INITIAL_USER_STACK_BOUNDARY_IDENTITY
        );
        assert_eq!(plan.source_path(), PHASE8_INIT_PATH);
        assert_eq!(plan.source_digest(), image.source_digest());
        assert_eq!(plan.address_space_id(), address_space.id().raw());
        assert_eq!(plan.materialization_id(), materialization.id());
        assert_eq!(plan.entry_pc(), image.entry());
        assert_eq!(plan.initial_sp(), stack_plan.layout().initial_sp());
        assert!(plan.published());
        assert!(!plan.destroyed());

        let root = plan.root_provenance();
        assert_eq!(root.state(), TTBR0_ROOT_PROVENANCE);
        assert_eq!(root.materialization_id(), materialization.id());
        assert_eq!(root.root_token(), materialization.root().token().raw());
        assert_eq!(
            root.root_physical_frame(),
            materialization.root().physical_frame()
        );
        assert!(!root.ttbr0_written());

        assert_eq!(plan.ttbr1_kernel_policy(), TTBR1_KERNEL_POLICY_BLOCKED);
        assert_eq!(plan.tcr_state(), TCR_COMPATIBILITY_RECORD_ONLY);
        assert_eq!(plan.mair_state(), MAIR_COMPATIBILITY_RECORD_ONLY);
        assert_eq!(plan.sctlr_state(), SCTLR_MUTATION_BLOCKED);
        assert_eq!(plan.asid_state(), ASID_ALLOCATION_BLOCKED);
        assert_eq!(plan.tlb_state(), TLB_INVALIDATION_BLOCKED);
        assert_eq!(plan.barrier_state(), BARRIER_SEQUENCE_PLANNED_ONLY);
        assert_eq!(
            plan.live_register_sequence_state(),
            LIVE_REGISTER_SEQUENCE_BLOCKED
        );
        assert_eq!(plan.lower_el_eret_state(), LOWER_EL_ERET_BLOCKED);
        assert_eq!(
            plan.runnable_publication_state(),
            RUNNABLE_PUBLICATION_BLOCKED
        );
        assert_eq!(plan.process_lifecycle_state(), PROCESS_LIFECYCLE_BLOCKED);
        assert_eq!(plan.startup_abi_state(), STARTUP_ABI_EXPANSION_BLOCKED);
        assert_eq!(plan.filesystem_syscall_state(), FILESYSTEM_SYSCALLS_BLOCKED);
        assert_eq!(plan.pi5_hardware_proof_state(), PI5_HARDWARE_PROOF_BLOCKED);
        assert_eq!(plan.launch_activation_state(), ACTIVATION_PREFLIGHT_READY);
        assert_ne!(plan.plan_lease().token().raw(), 0);
        assert!(!plan.plan_lease().released());
        assert_eq!(activation_source.outstanding_leases(), 1);
    }

    #[test_case]
    fn records_required_kernel_reachability_without_live_side_effects() {
        let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
            fixture();
        let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();

        let plan = preflight_live_address_space_activation(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            LiveAddressSpaceActivationRequest::PreflightOnly,
            &mut activation_source,
        )
        .expect("activation preflight");

        let reachability = plan.kernel_reachability();
        assert!(reachability.vbar_el1());
        assert!(reachability.exception_vectors());
        assert!(reachability.active_kernel_stack());
        assert!(reachability.kernel_text_data());
        assert!(reachability.allocator());
        assert!(reachability.uart_mmio_diagnostics());
        assert!(reachability.scheduler_code_data());
        assert!(reachability.panic_fault_reporting());

        let side_effects = plan.side_effects();
        assert!(!side_effects.ttbr_mutated());
        assert!(!side_effects.tcr_mutated());
        assert!(!side_effects.mair_mutated());
        assert!(!side_effects.sctlr_mutated());
        assert!(!side_effects.asid_allocated());
        assert!(!side_effects.tlb_mutated());
        assert!(!side_effects.live_dsb_isb());
        assert!(!side_effects.lower_el_eret());
        assert!(!side_effects.scheduler_published());
        assert!(!side_effects.process_table_mutated());
        assert!(!side_effects.descriptor_table_mutated());
    }

    #[test_case]
    fn rejects_live_register_and_publication_requests_without_partial_activation() {
        let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
            fixture();

        for request in [
            LiveAddressSpaceActivationRequest::LiveRegisterSequence,
            LiveAddressSpaceActivationRequest::PublishSchedulerRunnable,
            LiveAddressSpaceActivationRequest::LowerElLaunch,
        ] {
            let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
            assert_eq!(
                preflight_live_address_space_activation(
                    image,
                    install_plan,
                    address_space,
                    materialization,
                    launch_plan,
                    stack_plan,
                    request,
                    &mut activation_source,
                ),
                Err(PosixError::NotImplemented)
            );
            assert_eq!(activation_source.outstanding_leases(), 0);
        }

        let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
        let plan = preflight_live_address_space_activation(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            LiveAddressSpaceActivationRequest::PreflightOnly,
            &mut activation_source,
        )
        .expect("activation preflight");
        for target in [
            LiveAddressSpaceActivationCommitTarget::LiveRegisters,
            LiveAddressSpaceActivationCommitTarget::Runnable,
            LiveAddressSpaceActivationCommitTarget::LowerEl,
        ] {
            let rejection = plan
                .commit_request(target)
                .expect_err("live commit remains blocked");
            assert_eq!(rejection.error(), PosixError::NotImplemented);
            assert!(rejection.no_partial_activation());
            assert!(rejection.no_runnable_publication());
        }
        assert!(!plan.side_effects().scheduler_published());
    }

    #[test_case]
    fn teardown_releases_only_plan_local_lease_and_is_idempotent() {
        let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
            fixture();
        let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
        let mut plan = preflight_live_address_space_activation(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            LiveAddressSpaceActivationRequest::PreflightOnly,
            &mut activation_source,
        )
        .expect("activation preflight");

        let first = plan.destroy(&mut activation_source);
        assert!(first.plan_record_released());
        assert!(first.materialization_owned());
        assert!(first.launch_owned());
        assert!(first.stack_owned());
        assert!(first.image_owned());
        assert!(!first.already_destroyed());
        assert!(!plan.published());
        assert!(plan.destroyed());
        assert_eq!(activation_source.outstanding_leases(), 0);
        assert_eq!(activation_source.snapshot().plan_record_releases, 1);

        let second = plan.destroy(&mut activation_source);
        assert!(!second.plan_record_released());
        assert!(second.already_destroyed());
        assert_eq!(activation_source.snapshot().plan_record_releases, 1);
    }

    #[test_case]
    fn rejects_identity_and_entry_disagreements_before_publication() {
        let (_, install_plan, address_space, materialization, launch_plan, stack_plan) = fixture();
        let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();

        assert_eq!(
            preflight_live_address_space_activation(
                image_with_identity("wrong-fixture"),
                install_plan,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                LiveAddressSpaceActivationRequest::PreflightOnly,
                &mut activation_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(activation_source.outstanding_leases(), 0);

        let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
            fixture();
        let bad_install = install_with_entry(install_plan, image.entry() + 4);
        let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
        assert_eq!(
            preflight_live_address_space_activation(
                image,
                bad_install,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                LiveAddressSpaceActivationRequest::PreflightOnly,
                &mut activation_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(activation_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rolls_back_resource_exhaustion_without_partial_activation() {
        let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
            fixture();
        let mut activation_source =
            LiveAddressSpaceActivationLeaseSource::with_plan_record_capacity(0);

        assert_eq!(
            preflight_live_address_space_activation(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                LiveAddressSpaceActivationRequest::PreflightOnly,
                &mut activation_source,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(activation_source.outstanding_leases(), 0);
        assert_eq!(activation_source.snapshot().plan_record_releases, 0);
    }
}
