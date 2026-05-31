//! Model-only live translation-register activation intent.
//!
//! This module consumes the accepted live descriptor-image installation binding
//! and emits an inspectable activation-commit intent below architectural
//! translation-register mutation. It does not write TTBR0_EL1/TTBR1_EL1, mutate
//! TCR_EL1/MAIR_EL1/SCTLR_EL1, copy descriptor images into the active root,
//! allocate an ASID, invalidate live TLB state, execute activation barriers,
//! publish scheduler state, mutate process/descriptor tables, or touch hardware.

use crate::{
    kernel_half_descriptor_image::{
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY, TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY,
    },
    kernel_half_reachability::KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
    live_address_space_activation::{
        ASID_ALLOCATION_BLOCKED, BARRIER_SEQUENCE_PLANNED_ONLY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY, LIVE_REGISTER_SEQUENCE_BLOCKED,
        LOWER_EL_ERET_BLOCKED, RUNNABLE_PUBLICATION_BLOCKED, SCTLR_MUTATION_BLOCKED,
        TCR_COMPATIBILITY_RECORD_ONLY, TLB_INVALIDATION_BLOCKED,
    },
    live_descriptor_image_installation::{
        DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE, KernelHalfDescriptorImageInstallation,
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY,
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY, TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE,
    },
    posix::PosixError,
    process_page_table_materialization::PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
    program_loader::PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
};

pub(crate) const LIVE_TRANSLATION_REGISTER_ACTIVATION_BOUNDARY_IDENTITY: &str =
    "phase8-live-translation-register-activation-v1";
pub(crate) const LIVE_TRANSLATION_REGISTER_ACTIVATION_POLICY: &str =
    "model-ttbr0-ttbr1-activation-commit-below-live-registers-v1";
pub(crate) const LIVE_TRANSLATION_REGISTER_ACTIVATION_STATE: &str =
    "model-only-activation-commit-intent";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveTranslationRegisterActivationRequest {
    CommitModelIntent,
    IdentityMismatch,
    LineageMismatch,
    StaleRootProvenance,
    AlreadyConsumedInstallation,
    ForbiddenEl0KernelAccess,
    DiagnosticReachabilityLoss,
    LiveRegisterRequest,
    ActiveRootCopyRequest,
    LowerElLaunchRequest,
    SchedulerPublicationRequest,
    FilesystemRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationToken(u64);

impl LiveTranslationRegisterActivationToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationLeaseSource {
    activation_record_capacity: usize,
    next_token: u64,
    activation_records: usize,
    activation_record_releases: usize,
}

impl LiveTranslationRegisterActivationLeaseSource {
    pub(crate) const fn with_activation_record_capacity(activation_record_capacity: usize) -> Self {
        Self {
            activation_record_capacity,
            next_token: 1,
            activation_records: 0,
            activation_record_releases: 0,
        }
    }

    pub(crate) const fn for_single_activation() -> Self {
        Self::with_activation_record_capacity(1)
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        self.activation_records
    }

    fn lease_activation_record(
        &mut self,
    ) -> Result<LiveTranslationRegisterActivationLease, PosixError> {
        if self.activation_records == self.activation_record_capacity {
            return Err(PosixError::NoMemory);
        }
        let lease = LiveTranslationRegisterActivationLease {
            token: LiveTranslationRegisterActivationToken(self.next_token),
            released: false,
        };
        self.next_token += 1;
        self.activation_records += 1;
        Ok(lease)
    }

    fn release_activation_record(&mut self, lease: &mut LiveTranslationRegisterActivationLease) {
        if !lease.released {
            lease.released = true;
            if self.activation_records != 0 {
                self.activation_records -= 1;
            }
            self.activation_record_releases += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationLease {
    token: LiveTranslationRegisterActivationToken,
    released: bool,
}

impl LiveTranslationRegisterActivationLease {
    pub(crate) const fn token(self) -> LiveTranslationRegisterActivationToken {
        self.token
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationInputState {
    installation_published: bool,
    installation_destroyed: bool,
    below_live_registers: bool,
    descriptor_image_installed: bool,
    ttbr0_written: bool,
    ttbr1_written: bool,
    sctlr_mutated: bool,
    active_root_copied: bool,
}

impl LiveTranslationRegisterActivationInputState {
    pub(crate) const fn installation_published(self) -> bool {
        self.installation_published
    }

    pub(crate) const fn installation_destroyed(self) -> bool {
        self.installation_destroyed
    }

    pub(crate) const fn below_live_registers(self) -> bool {
        self.below_live_registers
    }

    pub(crate) const fn descriptor_image_installed(self) -> bool {
        self.descriptor_image_installed
    }

    pub(crate) const fn ttbr0_written(self) -> bool {
        self.ttbr0_written
    }

    pub(crate) const fn ttbr1_written(self) -> bool {
        self.ttbr1_written
    }

    pub(crate) const fn sctlr_mutated(self) -> bool {
        self.sctlr_mutated
    }

    pub(crate) const fn active_root_copied(self) -> bool {
        self.active_root_copied
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationTtbrProvenance {
    ttbr0_root: &'static str,
    ttbr0_written: bool,
    ttbr1_root: &'static str,
    ttbr1_written: bool,
    active_root_copied: bool,
}

impl LiveTranslationRegisterActivationTtbrProvenance {
    pub(crate) const fn ttbr0_root(self) -> &'static str {
        self.ttbr0_root
    }

    pub(crate) const fn ttbr0_written(self) -> bool {
        self.ttbr0_written
    }

    pub(crate) const fn ttbr1_root(self) -> &'static str {
        self.ttbr1_root
    }

    pub(crate) const fn ttbr1_written(self) -> bool {
        self.ttbr1_written
    }

    pub(crate) const fn active_root_copied(self) -> bool {
        self.active_root_copied
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationReachability {
    vbar: bool,
    vectors: bool,
    active_stack: bool,
    kernel_text: bool,
    rodata: bool,
    data: bool,
    bss: bool,
    heap: bool,
    allocator: bool,
    uart_mmio_diagnostics: bool,
    scheduler_code_data: bool,
    runtime_console: bool,
    panic_fault_reporting: bool,
}

impl LiveTranslationRegisterActivationReachability {
    pub(crate) const fn vbar(self) -> bool {
        self.vbar
    }

    pub(crate) const fn vectors(self) -> bool {
        self.vectors
    }

    pub(crate) const fn active_stack(self) -> bool {
        self.active_stack
    }

    pub(crate) const fn kernel_text(self) -> bool {
        self.kernel_text
    }

    pub(crate) const fn rodata(self) -> bool {
        self.rodata
    }

    pub(crate) const fn data(self) -> bool {
        self.data
    }

    pub(crate) const fn bss(self) -> bool {
        self.bss
    }

    pub(crate) const fn heap(self) -> bool {
        self.heap
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

    pub(crate) const fn runtime_console(self) -> bool {
        self.runtime_console
    }

    pub(crate) const fn panic_fault_reporting(self) -> bool {
        self.panic_fault_reporting
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationSideEffects {
    ttbr_mutated: bool,
    tcr_mutated: bool,
    mair_mutated: bool,
    sctlr_mutated: bool,
    active_root_copied: bool,
    asid_allocated: bool,
    tlb_mutated: bool,
    live_dsb_isb: bool,
    lower_el_eret: bool,
    scheduler_published: bool,
    process_table_mutated: bool,
    descriptor_table_published: bool,
    filesystem_mutated: bool,
    hardware_action: bool,
}

impl LiveTranslationRegisterActivationSideEffects {
    pub(crate) const NONE: Self = Self {
        ttbr_mutated: false,
        tcr_mutated: false,
        mair_mutated: false,
        sctlr_mutated: false,
        active_root_copied: false,
        asid_allocated: false,
        tlb_mutated: false,
        live_dsb_isb: false,
        lower_el_eret: false,
        scheduler_published: false,
        process_table_mutated: false,
        descriptor_table_published: false,
        filesystem_mutated: false,
        hardware_action: false,
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

    pub(crate) const fn active_root_copied(self) -> bool {
        self.active_root_copied
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

    pub(crate) const fn descriptor_table_published(self) -> bool {
        self.descriptor_table_published
    }

    pub(crate) const fn filesystem_mutated(self) -> bool {
        self.filesystem_mutated
    }

    pub(crate) const fn hardware_action(self) -> bool {
        self.hardware_action
    }

    const fn is_empty(self) -> bool {
        !self.ttbr_mutated
            && !self.tcr_mutated
            && !self.mair_mutated
            && !self.sctlr_mutated
            && !self.active_root_copied
            && !self.asid_allocated
            && !self.tlb_mutated
            && !self.live_dsb_isb
            && !self.lower_el_eret
            && !self.scheduler_published
            && !self.process_table_mutated
            && !self.descriptor_table_published
            && !self.filesystem_mutated
            && !self.hardware_action
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivationTeardownReport {
    activation_cleared: bool,
    installation_input_owned: bool,
    descriptor_input_owned: bool,
    activation_plan_owned: bool,
    materialized_root_owned: bool,
    live_state_mutated: bool,
    already_destroyed: bool,
}

impl LiveTranslationRegisterActivationTeardownReport {
    pub(crate) const fn activation_cleared(self) -> bool {
        self.activation_cleared
    }

    pub(crate) const fn installation_input_owned(self) -> bool {
        self.installation_input_owned
    }

    pub(crate) const fn descriptor_input_owned(self) -> bool {
        self.descriptor_input_owned
    }

    pub(crate) const fn activation_plan_owned(self) -> bool {
        self.activation_plan_owned
    }

    pub(crate) const fn materialized_root_owned(self) -> bool {
        self.materialized_root_owned
    }

    pub(crate) const fn live_state_mutated(self) -> bool {
        self.live_state_mutated
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTranslationRegisterActivation {
    boundary_identity: &'static str,
    policy_identity: &'static str,
    image_fixture_identity: &'static str,
    install_boundary_identity: &'static str,
    address_space_boundary_identity: &'static str,
    materialization_boundary_identity: &'static str,
    launch_boundary_identity: &'static str,
    stack_boundary_identity: &'static str,
    activation_plan_boundary_identity: &'static str,
    reachability_boundary_identity: &'static str,
    descriptor_image_boundary_identity: &'static str,
    installation_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    address_space_id: u64,
    materialization_id: u64,
    entry_pc: u64,
    initial_sp: u64,
    lease: LiveTranslationRegisterActivationLease,
    input_state: LiveTranslationRegisterActivationInputState,
    ttbr_provenance: LiveTranslationRegisterActivationTtbrProvenance,
    reachability: LiveTranslationRegisterActivationReachability,
    previous_state: &'static str,
    next_state: &'static str,
    tcr_state: &'static str,
    mair_state: &'static str,
    sctlr_state: &'static str,
    asid_state: &'static str,
    tlb_state: &'static str,
    barrier_state: &'static str,
    live_register_sequence_state: &'static str,
    lower_el_eret_state: &'static str,
    runnable_publication_state: &'static str,
    side_effects: LiveTranslationRegisterActivationSideEffects,
    published: bool,
    destroyed: bool,
}

impl LiveTranslationRegisterActivation {
    pub(crate) const fn boundary_identity(self) -> &'static str {
        self.boundary_identity
    }

    pub(crate) const fn policy_identity(self) -> &'static str {
        self.policy_identity
    }

    pub(crate) const fn image_fixture_identity(self) -> &'static str {
        self.image_fixture_identity
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

    pub(crate) const fn lease(self) -> LiveTranslationRegisterActivationLease {
        self.lease
    }

    pub(crate) const fn input_state(self) -> LiveTranslationRegisterActivationInputState {
        self.input_state
    }

    pub(crate) const fn ttbr_provenance(self) -> LiveTranslationRegisterActivationTtbrProvenance {
        self.ttbr_provenance
    }

    pub(crate) const fn reachability(self) -> LiveTranslationRegisterActivationReachability {
        self.reachability
    }

    pub(crate) const fn previous_state(self) -> &'static str {
        self.previous_state
    }

    pub(crate) const fn next_state(self) -> &'static str {
        self.next_state
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

    pub(crate) const fn side_effects(self) -> LiveTranslationRegisterActivationSideEffects {
        self.side_effects
    }

    pub(crate) const fn published(self) -> bool {
        self.published
    }

    pub(crate) const fn destroyed(self) -> bool {
        self.destroyed
    }

    pub(crate) fn destroy(
        &mut self,
        lease_source: &mut LiveTranslationRegisterActivationLeaseSource,
    ) -> LiveTranslationRegisterActivationTeardownReport {
        if self.destroyed {
            return LiveTranslationRegisterActivationTeardownReport {
                activation_cleared: false,
                installation_input_owned: true,
                descriptor_input_owned: true,
                activation_plan_owned: true,
                materialized_root_owned: true,
                live_state_mutated: !self.side_effects.is_empty(),
                already_destroyed: true,
            };
        }

        lease_source.release_activation_record(&mut self.lease);
        self.published = false;
        self.destroyed = true;

        LiveTranslationRegisterActivationTeardownReport {
            activation_cleared: true,
            installation_input_owned: true,
            descriptor_input_owned: true,
            activation_plan_owned: true,
            materialized_root_owned: true,
            live_state_mutated: !self.side_effects.is_empty(),
            already_destroyed: false,
        }
    }
}

pub(crate) fn commit_live_translation_register_activation(
    installation: Option<KernelHalfDescriptorImageInstallation>,
    request: LiveTranslationRegisterActivationRequest,
    lease_source: &mut LiveTranslationRegisterActivationLeaseSource,
) -> Result<LiveTranslationRegisterActivation, PosixError> {
    match request {
        LiveTranslationRegisterActivationRequest::CommitModelIntent => {}
        LiveTranslationRegisterActivationRequest::IdentityMismatch => {
            return Err(PosixError::InvalidArgument);
        }
        LiveTranslationRegisterActivationRequest::LineageMismatch => {
            return Err(PosixError::NotExecutable);
        }
        LiveTranslationRegisterActivationRequest::StaleRootProvenance
        | LiveTranslationRegisterActivationRequest::AlreadyConsumedInstallation => {
            return Err(PosixError::Busy);
        }
        LiveTranslationRegisterActivationRequest::ForbiddenEl0KernelAccess
        | LiveTranslationRegisterActivationRequest::DiagnosticReachabilityLoss => {
            return Err(PosixError::AccessDenied);
        }
        LiveTranslationRegisterActivationRequest::LiveRegisterRequest
        | LiveTranslationRegisterActivationRequest::ActiveRootCopyRequest
        | LiveTranslationRegisterActivationRequest::LowerElLaunchRequest
        | LiveTranslationRegisterActivationRequest::SchedulerPublicationRequest
        | LiveTranslationRegisterActivationRequest::FilesystemRequest => {
            return Err(PosixError::NotImplemented);
        }
    }

    let installation = installation.ok_or(PosixError::InvalidArgument)?;
    validate_installation(installation)?;
    let lease = lease_source.lease_activation_record()?;
    let install_input = installation.input_state();
    let install_ttbr = installation.ttbr_provenance();
    let coverage = installation.coverage();

    Ok(LiveTranslationRegisterActivation {
        boundary_identity: LIVE_TRANSLATION_REGISTER_ACTIVATION_BOUNDARY_IDENTITY,
        policy_identity: LIVE_TRANSLATION_REGISTER_ACTIVATION_POLICY,
        image_fixture_identity: installation.image_fixture_identity(),
        install_boundary_identity: installation.install_boundary_identity(),
        address_space_boundary_identity: installation.address_space_boundary_identity(),
        materialization_boundary_identity: installation.materialization_boundary_identity(),
        launch_boundary_identity: installation.launch_boundary_identity(),
        stack_boundary_identity: installation.stack_boundary_identity(),
        activation_plan_boundary_identity: installation.activation_boundary_identity(),
        reachability_boundary_identity: installation.reachability_boundary_identity(),
        descriptor_image_boundary_identity: installation.descriptor_image_boundary_identity(),
        installation_boundary_identity: installation.boundary_identity(),
        source_path: installation.source_path(),
        source_digest: installation.source_digest(),
        address_space_id: installation.address_space_id(),
        materialization_id: installation.materialization_id(),
        entry_pc: installation.entry_pc(),
        initial_sp: installation.initial_sp(),
        lease,
        input_state: LiveTranslationRegisterActivationInputState {
            installation_published: installation.published(),
            installation_destroyed: installation.destroyed(),
            below_live_registers: installation.live_register_state()
                == LIVE_REGISTER_SEQUENCE_BLOCKED,
            descriptor_image_installed: install_input.descriptor_image_installed(),
            ttbr0_written: install_ttbr.ttbr0_written(),
            ttbr1_written: install_ttbr.ttbr1_written(),
            sctlr_mutated: installation.sctlr_state() != SCTLR_MUTATION_BLOCKED,
            active_root_copied: install_ttbr.active_root_copied(),
        },
        ttbr_provenance: LiveTranslationRegisterActivationTtbrProvenance {
            ttbr0_root: install_ttbr.ttbr0_root(),
            ttbr0_written: install_ttbr.ttbr0_written(),
            ttbr1_root: install_ttbr.ttbr1_root(),
            ttbr1_written: install_ttbr.ttbr1_written(),
            active_root_copied: install_ttbr.active_root_copied(),
        },
        reachability: LiveTranslationRegisterActivationReachability {
            vbar: coverage.vectors(),
            vectors: coverage.vectors(),
            active_stack: coverage.active_stack(),
            kernel_text: coverage.kernel_text(),
            rodata: coverage.rodata(),
            data: coverage.data(),
            bss: coverage.bss(),
            heap: coverage.heap(),
            allocator: coverage.page_frames(),
            uart_mmio_diagnostics: coverage.uart_mmio_diagnostics(),
            scheduler_code_data: coverage.scheduler_code_data(),
            runtime_console: coverage.runtime_console(),
            panic_fault_reporting: coverage.panic_fault_reporting(),
        },
        previous_state: installation.next_state(),
        next_state: LIVE_TRANSLATION_REGISTER_ACTIVATION_STATE,
        tcr_state: installation.tcr_state(),
        mair_state: installation.mair_state(),
        sctlr_state: installation.sctlr_state(),
        asid_state: installation.asid_state(),
        tlb_state: installation.tlb_state(),
        barrier_state: installation.barrier_state(),
        live_register_sequence_state: LIVE_REGISTER_SEQUENCE_BLOCKED,
        lower_el_eret_state: LOWER_EL_ERET_BLOCKED,
        runnable_publication_state: RUNNABLE_PUBLICATION_BLOCKED,
        side_effects: LiveTranslationRegisterActivationSideEffects::NONE,
        published: true,
        destroyed: false,
    })
}

fn validate_installation(
    installation: KernelHalfDescriptorImageInstallation,
) -> Result<(), PosixError> {
    if installation.boundary_identity() != LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY
        || installation.policy_identity() != LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY
        || installation.image_fixture_identity() != PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        || installation.materialization_boundary_identity()
            != PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        || installation.activation_boundary_identity()
            != LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        || installation.reachability_boundary_identity()
            != KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        || installation.descriptor_image_boundary_identity()
            != KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY
        || !installation.published()
        || installation.destroyed()
        || installation.next_state() != DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE
    {
        return Err(PosixError::InvalidArgument);
    }

    let ttbr = installation.ttbr_provenance();
    if ttbr.ttbr0_root() != TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY
        || ttbr.ttbr1_root() != TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE
    {
        return Err(PosixError::NotExecutable);
    }

    let coverage = installation.coverage();
    if !coverage.kernel_text()
        || !coverage.rodata()
        || !coverage.data()
        || !coverage.bss()
        || !coverage.vectors()
        || !coverage.active_stack()
        || !coverage.heap()
        || !coverage.page_frames()
        || !coverage.uart_mmio_diagnostics()
        || !coverage.scheduler_code_data()
        || !coverage.runtime_console()
        || !coverage.panic_fault_reporting()
    {
        return Err(PosixError::AccessDenied);
    }

    let permissions = installation.permissions();
    if permissions.el0_kernel_access() {
        return Err(PosixError::AccessDenied);
    }

    let input = installation.input_state();
    let effects = installation.side_effects();
    if input.descriptor_image_installed()
        || ttbr.ttbr0_written()
        || ttbr.ttbr1_written()
        || ttbr.active_root_copied()
        || installation.tcr_state() != TCR_COMPATIBILITY_RECORD_ONLY
        || installation.sctlr_state() != SCTLR_MUTATION_BLOCKED
        || installation.asid_state() != ASID_ALLOCATION_BLOCKED
        || installation.tlb_state() != TLB_INVALIDATION_BLOCKED
        || installation.barrier_state() != BARRIER_SEQUENCE_PLANNED_ONLY
        || installation.live_register_state() != LIVE_REGISTER_SEQUENCE_BLOCKED
        || installation.lower_el_eret()
        || installation.scheduler_publication()
        || installation.filesystem_syscalls()
        || effects.ttbr_mutated()
        || effects.tcr_mutated()
        || effects.mair_mutated()
        || effects.sctlr_mutated()
        || effects.active_root_copied()
        || effects.asid_allocated()
        || effects.tlb_mutated()
        || effects.live_dsb_isb()
        || effects.lower_el_eret()
        || effects.scheduler_published()
        || effects.process_table_mutated()
        || effects.descriptor_table_published()
        || effects.filesystem_mutated()
        || effects.hardware_action()
    {
        return Err(PosixError::Busy);
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
        kernel_half_descriptor_image::{
            KernelHalfDescriptorImageLeaseSource, KernelHalfDescriptorImageRequest,
            construct_kernel_half_descriptor_image,
        },
        kernel_half_reachability::{
            KernelHalfReachabilityLeaseSource, KernelHalfReachabilityRequest,
            preflight_kernel_half_reachability,
        },
        live_address_space_activation::{
            LiveAddressSpaceActivationLeaseSource, LiveAddressSpaceActivationRequest,
            preflight_live_address_space_activation,
        },
        live_descriptor_image_installation::{
            LiveDescriptorImageInstallationLeaseSource, LiveDescriptorImageInstallationRequest,
            install_live_descriptor_image_binding,
        },
        process_address_space::{
            ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource, install_process_address_space,
        },
        process_install::plan_process_image_install,
        process_page_table_materialization::{
            ProcessMaterializationRequest, ProcessPageTableMaterializationLeaseSource,
            materialize_process_page_tables,
        },
        scheduler::ProcessOwnerId,
    };

    fn fixture() -> KernelHalfDescriptorImageInstallation {
        let image =
            crate::program_loader::plan_phase8_init_image(phase8_readonly_initramfs_fixture())
                .expect("program image");
        let install_plan = plan_process_image_install(image).expect("install plan");
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(0x9900_8001).expect("address-space id"),
            Some(ProcessOwnerId::new(0x9900_8002).expect("owner id")),
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
        let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
        let activation_plan = preflight_live_address_space_activation(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            LiveAddressSpaceActivationRequest::PreflightOnly,
            &mut activation_source,
        )
        .expect("activation plan");
        let mut reachability_source = KernelHalfReachabilityLeaseSource::for_single_plan();
        let reachability_plan = preflight_kernel_half_reachability(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
            KernelHalfReachabilityRequest::PreflightOnly,
            &mut reachability_source,
        )
        .expect("reachability plan");
        let mut descriptor_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
        let descriptor_image = construct_kernel_half_descriptor_image(
            reachability_plan,
            materialization,
            KernelHalfDescriptorImageRequest::ConstructOnly,
            &mut descriptor_source,
        )
        .expect("descriptor image");
        let mut installation_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();
        install_live_descriptor_image_binding(
            Some(descriptor_image),
            Some(activation_plan),
            LiveDescriptorImageInstallationRequest::InstallModelBinding,
            &mut installation_source,
        )
        .expect("installation")
    }

    #[test_case]
    fn commits_model_activation_intent_with_copied_lineage_and_no_live_state() {
        let installation = fixture();
        let mut activation_source =
            LiveTranslationRegisterActivationLeaseSource::for_single_activation();

        let activation = commit_live_translation_register_activation(
            Some(installation),
            LiveTranslationRegisterActivationRequest::CommitModelIntent,
            &mut activation_source,
        )
        .expect("activation intent");

        assert!(activation.published());
        assert!(!activation.destroyed());
        assert_eq!(
            activation.boundary_identity(),
            LIVE_TRANSLATION_REGISTER_ACTIVATION_BOUNDARY_IDENTITY
        );
        assert_eq!(
            activation.policy_identity(),
            LIVE_TRANSLATION_REGISTER_ACTIVATION_POLICY
        );
        assert_eq!(
            activation.image_fixture_identity(),
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        );
        assert_eq!(activation.source_path(), PHASE8_INIT_PATH);
        assert_eq!(activation.source_digest(), installation.source_digest());
        assert_eq!(
            activation.address_space_id(),
            installation.address_space_id()
        );
        assert_eq!(
            activation.materialization_id(),
            installation.materialization_id()
        );
        assert_eq!(activation.entry_pc(), installation.entry_pc());
        assert_eq!(activation.initial_sp(), installation.initial_sp());
        assert_eq!(
            activation.previous_state(),
            DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE
        );
        assert_eq!(
            activation.next_state(),
            LIVE_TRANSLATION_REGISTER_ACTIVATION_STATE
        );
        assert_eq!(activation.tcr_state(), TCR_COMPATIBILITY_RECORD_ONLY);
        assert_eq!(activation.mair_state(), installation.mair_state());
        assert_eq!(activation.sctlr_state(), SCTLR_MUTATION_BLOCKED);
        assert_eq!(activation.asid_state(), ASID_ALLOCATION_BLOCKED);
        assert_eq!(activation.tlb_state(), TLB_INVALIDATION_BLOCKED);
        assert_eq!(activation.barrier_state(), BARRIER_SEQUENCE_PLANNED_ONLY);
        assert_eq!(
            activation.live_register_sequence_state(),
            LIVE_REGISTER_SEQUENCE_BLOCKED
        );
        assert_eq!(activation.lower_el_eret_state(), LOWER_EL_ERET_BLOCKED);
        assert_eq!(
            activation.runnable_publication_state(),
            RUNNABLE_PUBLICATION_BLOCKED
        );
        assert_ne!(activation.lease().token().raw(), 0);
        assert!(!activation.lease().released());
        assert_eq!(activation_source.outstanding_leases(), 1);
    }

    #[test_case]
    fn records_input_provenance_reachability_and_zero_side_effects() {
        let installation = fixture();
        let mut activation_source =
            LiveTranslationRegisterActivationLeaseSource::for_single_activation();
        let activation = commit_live_translation_register_activation(
            Some(installation),
            LiveTranslationRegisterActivationRequest::CommitModelIntent,
            &mut activation_source,
        )
        .expect("activation intent");

        let input = activation.input_state();
        assert!(input.installation_published());
        assert!(!input.installation_destroyed());
        assert!(input.below_live_registers());
        assert!(!input.descriptor_image_installed());
        assert!(!input.ttbr0_written());
        assert!(!input.ttbr1_written());
        assert!(!input.sctlr_mutated());
        assert!(!input.active_root_copied());

        let ttbr = activation.ttbr_provenance();
        assert_eq!(ttbr.ttbr0_root(), TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY);
        assert!(!ttbr.ttbr0_written());
        assert_eq!(ttbr.ttbr1_root(), TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE);
        assert!(!ttbr.ttbr1_written());
        assert!(!ttbr.active_root_copied());

        let reachability = activation.reachability();
        assert!(reachability.vbar());
        assert!(reachability.vectors());
        assert!(reachability.active_stack());
        assert!(reachability.kernel_text());
        assert!(reachability.rodata());
        assert!(reachability.data());
        assert!(reachability.bss());
        assert!(reachability.heap());
        assert!(reachability.allocator());
        assert!(reachability.uart_mmio_diagnostics());
        assert!(reachability.scheduler_code_data());
        assert!(reachability.runtime_console());
        assert!(reachability.panic_fault_reporting());

        let effects = activation.side_effects();
        assert!(!effects.ttbr_mutated());
        assert!(!effects.tcr_mutated());
        assert!(!effects.mair_mutated());
        assert!(!effects.sctlr_mutated());
        assert!(!effects.active_root_copied());
        assert!(!effects.asid_allocated());
        assert!(!effects.tlb_mutated());
        assert!(!effects.live_dsb_isb());
        assert!(!effects.lower_el_eret());
        assert!(!effects.scheduler_published());
        assert!(!effects.process_table_mutated());
        assert!(!effects.descriptor_table_published());
        assert!(!effects.filesystem_mutated());
        assert!(!effects.hardware_action());
    }

    #[test_case]
    fn rejects_deterministic_cases_without_partial_activation() {
        let installation = fixture();
        for (request, expected) in [
            (
                LiveTranslationRegisterActivationRequest::IdentityMismatch,
                PosixError::InvalidArgument,
            ),
            (
                LiveTranslationRegisterActivationRequest::LineageMismatch,
                PosixError::NotExecutable,
            ),
            (
                LiveTranslationRegisterActivationRequest::StaleRootProvenance,
                PosixError::Busy,
            ),
            (
                LiveTranslationRegisterActivationRequest::AlreadyConsumedInstallation,
                PosixError::Busy,
            ),
            (
                LiveTranslationRegisterActivationRequest::ForbiddenEl0KernelAccess,
                PosixError::AccessDenied,
            ),
            (
                LiveTranslationRegisterActivationRequest::DiagnosticReachabilityLoss,
                PosixError::AccessDenied,
            ),
            (
                LiveTranslationRegisterActivationRequest::LiveRegisterRequest,
                PosixError::NotImplemented,
            ),
            (
                LiveTranslationRegisterActivationRequest::ActiveRootCopyRequest,
                PosixError::NotImplemented,
            ),
            (
                LiveTranslationRegisterActivationRequest::LowerElLaunchRequest,
                PosixError::NotImplemented,
            ),
            (
                LiveTranslationRegisterActivationRequest::SchedulerPublicationRequest,
                PosixError::NotImplemented,
            ),
            (
                LiveTranslationRegisterActivationRequest::FilesystemRequest,
                PosixError::NotImplemented,
            ),
        ] {
            let mut activation_source =
                LiveTranslationRegisterActivationLeaseSource::for_single_activation();
            assert_eq!(
                commit_live_translation_register_activation(
                    Some(installation),
                    request,
                    &mut activation_source,
                ),
                Err(expected)
            );
            assert_eq!(activation_source.outstanding_leases(), 0);
            assert!(installation.published());
            assert!(!installation.destroyed());
        }
    }

    #[test_case]
    fn rejects_missing_destroyed_and_resource_exhaustion_without_leaks() {
        let mut installation = fixture();
        let mut activation_source =
            LiveTranslationRegisterActivationLeaseSource::for_single_activation();
        assert_eq!(
            commit_live_translation_register_activation(
                None,
                LiveTranslationRegisterActivationRequest::CommitModelIntent,
                &mut activation_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(activation_source.outstanding_leases(), 0);

        let mut activation_source =
            LiveTranslationRegisterActivationLeaseSource::with_activation_record_capacity(0);
        assert_eq!(
            commit_live_translation_register_activation(
                Some(installation),
                LiveTranslationRegisterActivationRequest::CommitModelIntent,
                &mut activation_source,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(activation_source.outstanding_leases(), 0);

        let mut installation_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();
        let _ = installation.destroy(&mut installation_source);
        let mut activation_source =
            LiveTranslationRegisterActivationLeaseSource::for_single_activation();
        assert_eq!(
            commit_live_translation_register_activation(
                Some(installation),
                LiveTranslationRegisterActivationRequest::CommitModelIntent,
                &mut activation_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(activation_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn teardown_clears_only_activation_record_and_is_idempotent() {
        let installation = fixture();
        let mut activation_source =
            LiveTranslationRegisterActivationLeaseSource::for_single_activation();
        let mut activation = commit_live_translation_register_activation(
            Some(installation),
            LiveTranslationRegisterActivationRequest::CommitModelIntent,
            &mut activation_source,
        )
        .expect("activation intent");

        let first = activation.destroy(&mut activation_source);
        assert!(first.activation_cleared());
        assert!(first.installation_input_owned());
        assert!(first.descriptor_input_owned());
        assert!(first.activation_plan_owned());
        assert!(first.materialized_root_owned());
        assert!(!first.live_state_mutated());
        assert!(!first.already_destroyed());
        assert!(!activation.published());
        assert!(activation.destroyed());
        assert_eq!(activation_source.outstanding_leases(), 0);

        let second = activation.destroy(&mut activation_source);
        assert!(!second.activation_cleared());
        assert!(second.installation_input_owned());
        assert!(second.descriptor_input_owned());
        assert!(second.activation_plan_owned());
        assert!(second.materialized_root_owned());
        assert!(!second.live_state_mutated());
        assert!(second.already_destroyed());
        assert_eq!(activation_source.outstanding_leases(), 0);
    }
}
