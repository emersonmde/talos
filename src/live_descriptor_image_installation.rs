//! Model-only live descriptor-image installation binding.
//!
//! This module consumes an accepted KernelHalfDescriptorImage and
//! LiveAddressSpaceActivationPlan, then publishes an inspectable installation
//! record below live translation-register activation. It does not write
//! TTBR0_EL1/TTBR1_EL1, mutate TCR_EL1/MAIR_EL1/SCTLR_EL1, allocate an ASID,
//! invalidate live TLB state, execute DSB/ISB activation sequencing, publish
//! a scheduler runnable, mutate process/descriptor tables, or touch hardware.

use crate::{
    kernel_half_descriptor_image::{
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY, KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY,
        KernelHalfDescriptorCoverage, KernelHalfDescriptorImage,
        KernelHalfDescriptorPermissionPolicy,
    },
    kernel_half_reachability::{
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY, KERNEL_HALF_REACHABILITY_POLICY,
    },
    live_address_space_activation::{
        ASID_ALLOCATION_BLOCKED, BARRIER_SEQUENCE_PLANNED_ONLY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY, LIVE_REGISTER_SEQUENCE_BLOCKED,
        LiveAddressSpaceActivationPlan, SCTLR_MUTATION_BLOCKED, TCR_COMPATIBILITY_RECORD_ONLY,
        TLB_INVALIDATION_BLOCKED,
    },
    posix::PosixError,
    process_page_table_materialization::PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
    program_loader::PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
};

pub(crate) const LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY: &str =
    "phase8-live-descriptor-image-installation-v1";
pub(crate) const LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY: &str =
    "model-installed-ttbr1-descriptor-image-below-live-registers-v1";
pub(crate) const TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE: &str =
    "descriptor-image-kernel-root-provenance";
pub(crate) const DESCRIPTOR_IMAGE_NOT_INSTALLED_STATE: &str = "non-installed-descriptor-image";
pub(crate) const DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE: &str =
    "installation-ready-activation-binding";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveDescriptorImageInstallationRequest {
    InstallModelBinding,
    IdentityMismatch,
    LineageMismatch,
    AlreadyInstalledInput,
    ForbiddenEl0Access,
    DiagnosticReachabilityLoss,
    LiveRegisterRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveDescriptorImageInstallationToken(u64);

impl LiveDescriptorImageInstallationToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveDescriptorImageInstallationLeaseSnapshot {
    pub(crate) installation_records: usize,
    pub(crate) installation_record_releases: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveDescriptorImageInstallationLeaseSource {
    installation_record_capacity: usize,
    next_token: u64,
    installation_records: usize,
    installation_record_releases: usize,
}

impl LiveDescriptorImageInstallationLeaseSource {
    pub(crate) const fn with_installation_record_capacity(
        installation_record_capacity: usize,
    ) -> Self {
        Self {
            installation_record_capacity,
            next_token: 1,
            installation_records: 0,
            installation_record_releases: 0,
        }
    }

    pub(crate) const fn for_single_installation() -> Self {
        Self::with_installation_record_capacity(1)
    }

    pub(crate) const fn snapshot(self) -> LiveDescriptorImageInstallationLeaseSnapshot {
        LiveDescriptorImageInstallationLeaseSnapshot {
            installation_records: self.installation_records,
            installation_record_releases: self.installation_record_releases,
        }
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        self.installation_records
    }

    fn lease_installation_record(
        &mut self,
    ) -> Result<LiveDescriptorImageInstallationLease, PosixError> {
        if self.installation_records == self.installation_record_capacity {
            return Err(PosixError::NoMemory);
        }
        let lease = LiveDescriptorImageInstallationLease {
            token: LiveDescriptorImageInstallationToken(self.next_token),
            released: false,
        };
        self.next_token += 1;
        self.installation_records += 1;
        Ok(lease)
    }

    fn release_installation_record(&mut self, lease: &mut LiveDescriptorImageInstallationLease) {
        if !lease.released {
            lease.released = true;
            if self.installation_records != 0 {
                self.installation_records -= 1;
            }
            self.installation_record_releases += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveDescriptorImageInstallationLease {
    token: LiveDescriptorImageInstallationToken,
    released: bool,
}

impl LiveDescriptorImageInstallationLease {
    pub(crate) const fn token(self) -> LiveDescriptorImageInstallationToken {
        self.token
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveDescriptorImageInstallationInputState {
    descriptor_published: bool,
    descriptor_destroyed: bool,
    descriptor_installed: bool,
    descriptor_image_installed: bool,
    ttbr1_written: bool,
    activation_published: bool,
    activation_destroyed: bool,
    activation_model_only: bool,
}

impl LiveDescriptorImageInstallationInputState {
    pub(crate) const fn descriptor_published(self) -> bool {
        self.descriptor_published
    }

    pub(crate) const fn descriptor_installed(self) -> bool {
        self.descriptor_installed
    }

    pub(crate) const fn descriptor_image_installed(self) -> bool {
        self.descriptor_image_installed
    }

    pub(crate) const fn ttbr1_written(self) -> bool {
        self.ttbr1_written
    }

    pub(crate) const fn activation_published(self) -> bool {
        self.activation_published
    }

    pub(crate) const fn activation_model_only(self) -> bool {
        self.activation_model_only
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveDescriptorImageInstallationTtbrProvenance {
    ttbr0_root: &'static str,
    ttbr0_root_token: u64,
    ttbr0_root_physical_frame: u64,
    ttbr0_written: bool,
    ttbr1_root: &'static str,
    ttbr1_written: bool,
    active_root_copied: bool,
}

impl LiveDescriptorImageInstallationTtbrProvenance {
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
pub(crate) struct LiveDescriptorImageInstallationSideEffects {
    ttbr_mutated: bool,
    tcr_mutated: bool,
    mair_mutated: bool,
    sctlr_mutated: bool,
    active_root_copied: bool,
    descriptor_table_published: bool,
    asid_allocated: bool,
    tlb_mutated: bool,
    live_dsb_isb: bool,
    lower_el_eret: bool,
    scheduler_published: bool,
    process_table_mutated: bool,
    filesystem_mutated: bool,
    hardware_action: bool,
}

impl LiveDescriptorImageInstallationSideEffects {
    pub(crate) const NONE: Self = Self {
        ttbr_mutated: false,
        tcr_mutated: false,
        mair_mutated: false,
        sctlr_mutated: false,
        active_root_copied: false,
        descriptor_table_published: false,
        asid_allocated: false,
        tlb_mutated: false,
        live_dsb_isb: false,
        lower_el_eret: false,
        scheduler_published: false,
        process_table_mutated: false,
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

    pub(crate) const fn descriptor_table_published(self) -> bool {
        self.descriptor_table_published
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
            && !self.descriptor_table_published
            && !self.asid_allocated
            && !self.tlb_mutated
            && !self.live_dsb_isb
            && !self.lower_el_eret
            && !self.scheduler_published
            && !self.process_table_mutated
            && !self.filesystem_mutated
            && !self.hardware_action
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveDescriptorImageInstallationTeardownReport {
    installation_cleared: bool,
    descriptor_input_owned: bool,
    activation_input_owned: bool,
    live_state_mutated: bool,
    already_destroyed: bool,
}

impl LiveDescriptorImageInstallationTeardownReport {
    pub(crate) const fn installation_cleared(self) -> bool {
        self.installation_cleared
    }

    pub(crate) const fn descriptor_input_owned(self) -> bool {
        self.descriptor_input_owned
    }

    pub(crate) const fn activation_input_owned(self) -> bool {
        self.activation_input_owned
    }

    pub(crate) const fn live_state_mutated(self) -> bool {
        self.live_state_mutated
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorImageInstallation {
    boundary_identity: &'static str,
    policy_identity: &'static str,
    image_fixture_identity: &'static str,
    install_boundary_identity: &'static str,
    address_space_boundary_identity: &'static str,
    materialization_boundary_identity: &'static str,
    launch_boundary_identity: &'static str,
    stack_boundary_identity: &'static str,
    activation_boundary_identity: &'static str,
    reachability_boundary_identity: &'static str,
    descriptor_image_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    address_space_id: u64,
    materialization_id: u64,
    entry_pc: u64,
    initial_sp: u64,
    lease: LiveDescriptorImageInstallationLease,
    input_state: LiveDescriptorImageInstallationInputState,
    ttbr_provenance: LiveDescriptorImageInstallationTtbrProvenance,
    coverage: KernelHalfDescriptorCoverage,
    permissions: KernelHalfDescriptorPermissionPolicy,
    previous_state: &'static str,
    next_state: &'static str,
    live_register_state: &'static str,
    tcr_state: &'static str,
    mair_state: &'static str,
    sctlr_state: &'static str,
    asid_state: &'static str,
    tlb_state: &'static str,
    barrier_state: &'static str,
    lower_el_eret: bool,
    scheduler_publication: bool,
    filesystem_syscalls: bool,
    side_effects: LiveDescriptorImageInstallationSideEffects,
    published: bool,
    destroyed: bool,
}

impl KernelHalfDescriptorImageInstallation {
    pub(crate) const fn boundary_identity(self) -> &'static str {
        self.boundary_identity
    }

    pub(crate) const fn policy_identity(self) -> &'static str {
        self.policy_identity
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

    pub(crate) const fn launch_boundary_identity(self) -> &'static str {
        self.launch_boundary_identity
    }

    pub(crate) const fn stack_boundary_identity(self) -> &'static str {
        self.stack_boundary_identity
    }

    pub(crate) const fn descriptor_image_boundary_identity(self) -> &'static str {
        self.descriptor_image_boundary_identity
    }

    pub(crate) const fn activation_boundary_identity(self) -> &'static str {
        self.activation_boundary_identity
    }

    pub(crate) const fn reachability_boundary_identity(self) -> &'static str {
        self.reachability_boundary_identity
    }

    pub(crate) const fn materialization_boundary_identity(self) -> &'static str {
        self.materialization_boundary_identity
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

    pub(crate) const fn lease(self) -> LiveDescriptorImageInstallationLease {
        self.lease
    }

    pub(crate) const fn input_state(self) -> LiveDescriptorImageInstallationInputState {
        self.input_state
    }

    pub(crate) const fn ttbr_provenance(self) -> LiveDescriptorImageInstallationTtbrProvenance {
        self.ttbr_provenance
    }

    pub(crate) const fn coverage(self) -> KernelHalfDescriptorCoverage {
        self.coverage
    }

    pub(crate) const fn permissions(self) -> KernelHalfDescriptorPermissionPolicy {
        self.permissions
    }

    pub(crate) const fn previous_state(self) -> &'static str {
        self.previous_state
    }

    pub(crate) const fn next_state(self) -> &'static str {
        self.next_state
    }

    pub(crate) const fn live_register_state(self) -> &'static str {
        self.live_register_state
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

    pub(crate) const fn lower_el_eret(self) -> bool {
        self.lower_el_eret
    }

    pub(crate) const fn scheduler_publication(self) -> bool {
        self.scheduler_publication
    }

    pub(crate) const fn filesystem_syscalls(self) -> bool {
        self.filesystem_syscalls
    }

    pub(crate) const fn side_effects(self) -> LiveDescriptorImageInstallationSideEffects {
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
        lease_source: &mut LiveDescriptorImageInstallationLeaseSource,
    ) -> LiveDescriptorImageInstallationTeardownReport {
        if self.destroyed {
            return LiveDescriptorImageInstallationTeardownReport {
                installation_cleared: false,
                descriptor_input_owned: true,
                activation_input_owned: true,
                live_state_mutated: !self.side_effects.is_empty(),
                already_destroyed: true,
            };
        }

        lease_source.release_installation_record(&mut self.lease);
        self.published = false;
        self.destroyed = true;

        LiveDescriptorImageInstallationTeardownReport {
            installation_cleared: true,
            descriptor_input_owned: true,
            activation_input_owned: true,
            live_state_mutated: !self.side_effects.is_empty(),
            already_destroyed: false,
        }
    }
}

pub(crate) fn install_live_descriptor_image_binding(
    descriptor_image: Option<KernelHalfDescriptorImage>,
    activation_plan: Option<LiveAddressSpaceActivationPlan>,
    request: LiveDescriptorImageInstallationRequest,
    lease_source: &mut LiveDescriptorImageInstallationLeaseSource,
) -> Result<KernelHalfDescriptorImageInstallation, PosixError> {
    match request {
        LiveDescriptorImageInstallationRequest::InstallModelBinding => {}
        LiveDescriptorImageInstallationRequest::IdentityMismatch => {
            return Err(PosixError::InvalidArgument);
        }
        LiveDescriptorImageInstallationRequest::LineageMismatch => {
            return Err(PosixError::NotExecutable);
        }
        LiveDescriptorImageInstallationRequest::AlreadyInstalledInput => {
            return Err(PosixError::Busy);
        }
        LiveDescriptorImageInstallationRequest::ForbiddenEl0Access
        | LiveDescriptorImageInstallationRequest::DiagnosticReachabilityLoss => {
            return Err(PosixError::AccessDenied);
        }
        LiveDescriptorImageInstallationRequest::LiveRegisterRequest => {
            return Err(PosixError::NotImplemented);
        }
    }

    let descriptor_image = descriptor_image.ok_or(PosixError::InvalidArgument)?;
    let activation_plan = activation_plan.ok_or(PosixError::InvalidArgument)?;
    validate_inputs(descriptor_image, activation_plan)?;

    let lease = lease_source.lease_installation_record()?;
    Ok(KernelHalfDescriptorImageInstallation {
        boundary_identity: LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY,
        policy_identity: LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY,
        image_fixture_identity: descriptor_image.image_fixture_identity(),
        install_boundary_identity: activation_plan.install_boundary_identity(),
        address_space_boundary_identity: activation_plan.address_space_boundary_identity(),
        materialization_boundary_identity: descriptor_image.materialization_boundary_identity(),
        launch_boundary_identity: activation_plan.launch_boundary_identity(),
        stack_boundary_identity: activation_plan.stack_boundary_identity(),
        activation_boundary_identity: activation_plan.boundary_identity(),
        reachability_boundary_identity: descriptor_image.reachability_boundary_identity(),
        descriptor_image_boundary_identity: descriptor_image.boundary_identity(),
        source_path: descriptor_image.source_path(),
        source_digest: descriptor_image.source_digest(),
        address_space_id: descriptor_image.address_space_id(),
        materialization_id: descriptor_image.materialization_id(),
        entry_pc: descriptor_image.entry_pc(),
        initial_sp: descriptor_image.initial_sp(),
        lease,
        input_state: LiveDescriptorImageInstallationInputState {
            descriptor_published: descriptor_image.published(),
            descriptor_destroyed: descriptor_image.destroyed(),
            descriptor_installed: false,
            descriptor_image_installed: descriptor_image
                .side_effects()
                .descriptor_image_installed(),
            ttbr1_written: descriptor_image.ttbr1_written(),
            activation_published: activation_plan.published(),
            activation_destroyed: activation_plan.destroyed(),
            activation_model_only: activation_plan.live_register_sequence_state()
                == LIVE_REGISTER_SEQUENCE_BLOCKED,
        },
        ttbr_provenance: LiveDescriptorImageInstallationTtbrProvenance {
            ttbr0_root: descriptor_image.ttbr0_root(),
            ttbr0_root_token: descriptor_image.ttbr0_root_token(),
            ttbr0_root_physical_frame: descriptor_image.ttbr0_root_physical_frame(),
            ttbr0_written: descriptor_image.ttbr0_written(),
            ttbr1_root: TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE,
            ttbr1_written: false,
            active_root_copied: false,
        },
        coverage: descriptor_image.coverage(),
        permissions: descriptor_image.permissions(),
        previous_state: DESCRIPTOR_IMAGE_NOT_INSTALLED_STATE,
        next_state: DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE,
        live_register_state: LIVE_REGISTER_SEQUENCE_BLOCKED,
        tcr_state: TCR_COMPATIBILITY_RECORD_ONLY,
        mair_state: activation_plan.mair_state(),
        sctlr_state: SCTLR_MUTATION_BLOCKED,
        asid_state: ASID_ALLOCATION_BLOCKED,
        tlb_state: TLB_INVALIDATION_BLOCKED,
        barrier_state: BARRIER_SEQUENCE_PLANNED_ONLY,
        lower_el_eret: false,
        scheduler_publication: false,
        filesystem_syscalls: false,
        side_effects: LiveDescriptorImageInstallationSideEffects::NONE,
        published: true,
        destroyed: false,
    })
}

fn validate_inputs(
    descriptor_image: KernelHalfDescriptorImage,
    activation_plan: LiveAddressSpaceActivationPlan,
) -> Result<(), PosixError> {
    if descriptor_image.boundary_identity() != KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY
        || descriptor_image.policy_identity() != KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY
        || descriptor_image.reachability_boundary_identity()
            != KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        || descriptor_image.reachability_policy_identity() != KERNEL_HALF_REACHABILITY_POLICY
        || descriptor_image.image_fixture_identity() != PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        || descriptor_image.materialization_boundary_identity()
            != PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        || activation_plan.boundary_identity() != LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        || !descriptor_image.published()
        || descriptor_image.destroyed()
        || !activation_plan.published()
        || activation_plan.destroyed()
    {
        return Err(PosixError::InvalidArgument);
    }

    if descriptor_image.source_path() != activation_plan.source_path()
        || descriptor_image.source_digest() != activation_plan.source_digest()
        || descriptor_image.address_space_id() != activation_plan.address_space_id()
        || descriptor_image.materialization_id() != activation_plan.materialization_id()
        || descriptor_image.entry_pc() != activation_plan.entry_pc()
        || descriptor_image.initial_sp() != activation_plan.initial_sp()
        || descriptor_image.ttbr0_root_token() != activation_plan.root_provenance().root_token()
        || descriptor_image.ttbr0_root_physical_frame()
            != activation_plan.root_provenance().root_physical_frame()
    {
        return Err(PosixError::NotExecutable);
    }

    let coverage = descriptor_image.coverage();
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

    let permissions = descriptor_image.permissions();
    if !permissions.text_exec_privileged_only()
        || permissions.rodata_write()
        || permissions.data_exec()
        || permissions.device_normal_memory()
        || permissions.el0_kernel_access()
        || permissions.wx_normal_memory()
    {
        return Err(PosixError::AccessDenied);
    }

    let descriptor_effects = descriptor_image.side_effects();
    let activation_effects = activation_plan.side_effects();
    if descriptor_image.ttbr1_written()
        || descriptor_effects.ttbr_mutated()
        || descriptor_effects.tcr_mutated()
        || descriptor_effects.mair_mutated()
        || descriptor_effects.sctlr_mutated()
        || descriptor_effects.descriptor_image_installed()
        || descriptor_effects.asid_allocated()
        || descriptor_effects.tlb_mutated()
        || descriptor_effects.live_dsb_isb()
        || descriptor_effects.lower_el_eret()
        || descriptor_effects.scheduler_published()
        || descriptor_effects.process_table_mutated()
        || descriptor_effects.descriptor_table_mutated()
        || activation_plan.root_provenance().ttbr0_written()
        || activation_effects.ttbr_mutated()
        || activation_effects.tcr_mutated()
        || activation_effects.mair_mutated()
        || activation_effects.sctlr_mutated()
        || activation_effects.asid_allocated()
        || activation_effects.tlb_mutated()
        || activation_effects.live_dsb_isb()
        || activation_effects.lower_el_eret()
        || activation_effects.scheduler_published()
        || activation_effects.process_table_mutated()
        || activation_effects.descriptor_table_mutated()
        || activation_plan.live_register_sequence_state() != LIVE_REGISTER_SEQUENCE_BLOCKED
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

    fn fixture() -> (
        KernelHalfDescriptorImage,
        LiveAddressSpaceActivationPlan,
        KernelHalfDescriptorImageLeaseSource,
        LiveAddressSpaceActivationLeaseSource,
    ) {
        let image =
            crate::program_loader::plan_phase8_init_image(phase8_readonly_initramfs_fixture())
                .expect("program image");
        let install_plan = plan_process_image_install(image).expect("install plan");
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(0x8800_8001).expect("address-space id"),
            Some(ProcessOwnerId::new(0x8800_8002).expect("owner id")),
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

        (
            descriptor_image,
            activation_plan,
            descriptor_source,
            activation_source,
        )
    }

    #[test_case]
    fn installs_model_binding_with_copied_lineage_and_no_live_state() {
        let (descriptor_image, activation_plan, descriptor_source, activation_source) = fixture();
        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();

        let installation = install_live_descriptor_image_binding(
            Some(descriptor_image),
            Some(activation_plan),
            LiveDescriptorImageInstallationRequest::InstallModelBinding,
            &mut install_source,
        )
        .expect("installation binding");

        assert!(installation.published());
        assert!(!installation.destroyed());
        assert_eq!(
            installation.boundary_identity(),
            LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY
        );
        assert_eq!(
            installation.policy_identity(),
            LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY
        );
        assert_eq!(
            installation.image_fixture_identity(),
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        );
        assert_eq!(
            installation.descriptor_image_boundary_identity(),
            KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY
        );
        assert_eq!(
            installation.activation_boundary_identity(),
            LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        );
        assert_eq!(
            installation.reachability_boundary_identity(),
            KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        );
        assert_eq!(
            installation.materialization_boundary_identity(),
            PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        );
        assert_eq!(installation.source_path(), PHASE8_INIT_PATH);
        assert_eq!(
            installation.source_digest(),
            descriptor_image.source_digest()
        );
        assert_eq!(
            installation.address_space_id(),
            descriptor_image.address_space_id()
        );
        assert_eq!(
            installation.materialization_id(),
            descriptor_image.materialization_id()
        );
        assert_eq!(installation.entry_pc(), descriptor_image.entry_pc());
        assert_eq!(installation.initial_sp(), descriptor_image.initial_sp());
        assert_eq!(
            installation.previous_state(),
            DESCRIPTOR_IMAGE_NOT_INSTALLED_STATE
        );
        assert_eq!(
            installation.next_state(),
            DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE
        );
        assert_eq!(
            installation.live_register_state(),
            LIVE_REGISTER_SEQUENCE_BLOCKED
        );
        assert_eq!(installation.tcr_state(), TCR_COMPATIBILITY_RECORD_ONLY);
        assert_eq!(installation.mair_state(), activation_plan.mair_state());
        assert_eq!(installation.sctlr_state(), SCTLR_MUTATION_BLOCKED);
        assert_eq!(installation.asid_state(), ASID_ALLOCATION_BLOCKED);
        assert_eq!(installation.tlb_state(), TLB_INVALIDATION_BLOCKED);
        assert_eq!(installation.barrier_state(), BARRIER_SEQUENCE_PLANNED_ONLY);
        assert!(!installation.lower_el_eret());
        assert!(!installation.scheduler_publication());
        assert!(!installation.filesystem_syscalls());
        assert_ne!(installation.lease().token().raw(), 0);
        assert!(!installation.lease().released());
        assert_eq!(install_source.outstanding_leases(), 1);
        assert_eq!(descriptor_source.outstanding_leases(), 16);
        assert_eq!(activation_source.outstanding_leases(), 1);

        let input = installation.input_state();
        assert!(input.descriptor_published());
        assert!(!input.descriptor_installed());
        assert!(!input.descriptor_image_installed());
        assert!(!input.ttbr1_written());
        assert!(input.activation_published());
        assert!(input.activation_model_only());

        let ttbr = installation.ttbr_provenance();
        assert_eq!(ttbr.ttbr0_root(), descriptor_image.ttbr0_root());
        assert!(!ttbr.ttbr0_written());
        assert_eq!(ttbr.ttbr1_root(), TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE);
        assert!(!ttbr.ttbr1_written());
        assert!(!ttbr.active_root_copied());
    }

    #[test_case]
    fn preserves_coverage_permissions_and_zero_side_effect_accounting() {
        let (descriptor_image, activation_plan, _descriptor_source, _activation_source) = fixture();
        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();

        let installation = install_live_descriptor_image_binding(
            Some(descriptor_image),
            Some(activation_plan),
            LiveDescriptorImageInstallationRequest::InstallModelBinding,
            &mut install_source,
        )
        .expect("installation binding");

        let coverage = installation.coverage();
        assert!(coverage.kernel_text());
        assert!(coverage.rodata());
        assert!(coverage.data());
        assert!(coverage.bss());
        assert!(coverage.vectors());
        assert!(coverage.active_stack());
        assert!(coverage.heap());
        assert!(coverage.page_frames());
        assert!(coverage.uart_mmio_diagnostics());
        assert!(coverage.scheduler_code_data());
        assert!(coverage.runtime_console());
        assert!(coverage.panic_fault_reporting());

        let permissions = installation.permissions();
        assert!(permissions.text_exec_privileged_only());
        assert!(!permissions.rodata_write());
        assert!(!permissions.data_exec());
        assert!(!permissions.device_normal_memory());
        assert!(!permissions.el0_kernel_access());
        assert!(!permissions.wx_normal_memory());

        let effects = installation.side_effects();
        assert!(!effects.ttbr_mutated());
        assert!(!effects.tcr_mutated());
        assert!(!effects.mair_mutated());
        assert!(!effects.sctlr_mutated());
        assert!(!effects.active_root_copied());
        assert!(!effects.descriptor_table_published());
        assert!(!effects.asid_allocated());
        assert!(!effects.tlb_mutated());
        assert!(!effects.live_dsb_isb());
        assert!(!effects.lower_el_eret());
        assert!(!effects.scheduler_published());
        assert!(!effects.process_table_mutated());
        assert!(!effects.filesystem_mutated());
        assert!(!effects.hardware_action());
    }

    #[test_case]
    fn rejects_deterministic_cases_without_partial_installation() {
        let (descriptor_image, activation_plan, _descriptor_source, _activation_source) = fixture();
        for (request, expected) in [
            (
                LiveDescriptorImageInstallationRequest::IdentityMismatch,
                PosixError::InvalidArgument,
            ),
            (
                LiveDescriptorImageInstallationRequest::LineageMismatch,
                PosixError::NotExecutable,
            ),
            (
                LiveDescriptorImageInstallationRequest::AlreadyInstalledInput,
                PosixError::Busy,
            ),
            (
                LiveDescriptorImageInstallationRequest::ForbiddenEl0Access,
                PosixError::AccessDenied,
            ),
            (
                LiveDescriptorImageInstallationRequest::DiagnosticReachabilityLoss,
                PosixError::AccessDenied,
            ),
            (
                LiveDescriptorImageInstallationRequest::LiveRegisterRequest,
                PosixError::NotImplemented,
            ),
        ] {
            let mut install_source =
                LiveDescriptorImageInstallationLeaseSource::for_single_installation();
            assert_eq!(
                install_live_descriptor_image_binding(
                    Some(descriptor_image),
                    Some(activation_plan),
                    request,
                    &mut install_source,
                ),
                Err(expected)
            );
            assert_eq!(install_source.outstanding_leases(), 0);
            assert!(!descriptor_image.side_effects().descriptor_image_installed());
            assert!(activation_plan.published());
            assert!(!activation_plan.side_effects().scheduler_published());
        }
    }

    #[test_case]
    fn rejects_missing_destroyed_and_resource_exhaustion_without_leaks() {
        let (
            mut descriptor_image,
            mut activation_plan,
            mut descriptor_source,
            mut activation_source,
        ) = fixture();

        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();
        assert_eq!(
            install_live_descriptor_image_binding(
                None,
                Some(activation_plan),
                LiveDescriptorImageInstallationRequest::InstallModelBinding,
                &mut install_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(install_source.outstanding_leases(), 0);

        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::with_installation_record_capacity(0);
        assert_eq!(
            install_live_descriptor_image_binding(
                Some(descriptor_image),
                Some(activation_plan),
                LiveDescriptorImageInstallationRequest::InstallModelBinding,
                &mut install_source,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(install_source.outstanding_leases(), 0);

        let _ = descriptor_image.destroy(&mut descriptor_source);
        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();
        assert_eq!(
            install_live_descriptor_image_binding(
                Some(descriptor_image),
                Some(activation_plan),
                LiveDescriptorImageInstallationRequest::InstallModelBinding,
                &mut install_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(install_source.outstanding_leases(), 0);

        let (descriptor_image, activation_plan_fresh, _descriptor_source, _activation_source) =
            fixture();
        let _ = activation_plan.destroy(&mut activation_source);
        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();
        assert_eq!(
            install_live_descriptor_image_binding(
                Some(descriptor_image),
                Some(activation_plan),
                LiveDescriptorImageInstallationRequest::InstallModelBinding,
                &mut install_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(install_source.outstanding_leases(), 0);

        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();
        install_live_descriptor_image_binding(
            Some(descriptor_image),
            Some(activation_plan_fresh),
            LiveDescriptorImageInstallationRequest::InstallModelBinding,
            &mut install_source,
        )
        .expect("fresh installation still works");
    }

    #[test_case]
    fn teardown_clears_only_installation_record_and_is_idempotent() {
        let (descriptor_image, activation_plan, descriptor_source, activation_source) = fixture();
        let mut install_source =
            LiveDescriptorImageInstallationLeaseSource::for_single_installation();
        let mut installation = install_live_descriptor_image_binding(
            Some(descriptor_image),
            Some(activation_plan),
            LiveDescriptorImageInstallationRequest::InstallModelBinding,
            &mut install_source,
        )
        .expect("installation binding");

        let first = installation.destroy(&mut install_source);
        assert!(first.installation_cleared());
        assert!(first.descriptor_input_owned());
        assert!(first.activation_input_owned());
        assert!(!first.live_state_mutated());
        assert!(!first.already_destroyed());
        assert!(!installation.published());
        assert!(installation.destroyed());
        assert_eq!(install_source.outstanding_leases(), 0);
        assert_eq!(install_source.snapshot().installation_record_releases, 1);
        assert_eq!(descriptor_source.outstanding_leases(), 16);
        assert_eq!(activation_source.outstanding_leases(), 1);

        let second = installation.destroy(&mut install_source);
        assert!(!second.installation_cleared());
        assert!(second.descriptor_input_owned());
        assert!(second.activation_input_owned());
        assert!(!second.live_state_mutated());
        assert!(second.already_destroyed());
        assert_eq!(install_source.snapshot().installation_record_releases, 1);
    }
}
