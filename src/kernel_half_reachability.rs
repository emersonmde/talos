//! Target-independent kernel-half reachability preflight.
//!
//! This module consumes the accepted Phase 8 loader, install, address-space,
//! materialization, launch, stack, and live-activation records and emits an
//! inspectable kernel-half policy record only. It does not build or install a
//! kernel-half descriptor image, mutate translation registers, invalidate live
//! TLB state, publish scheduler state, or enter lower EL.

use crate::{
    initial_process_launch::{INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY, InitialProcessLaunchPlan},
    initial_user_stack::{INITIAL_USER_STACK_BOUNDARY_IDENTITY, InitialUserStackPlan},
    live_address_space_activation::{
        ACTIVATION_PREFLIGHT_READY, ASID_ALLOCATION_BLOCKED, BARRIER_SEQUENCE_PLANNED_ONLY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY, LIVE_REGISTER_SEQUENCE_BLOCKED,
        LiveAddressSpaceActivationPlan, SCTLR_MUTATION_BLOCKED, TLB_INVALIDATION_BLOCKED,
        TTBR0_ROOT_PROVENANCE,
    },
    posix::PosixError,
    process_address_space::{PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace},
    process_install::{PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan},
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessPageTableMaterialization,
    },
    program_loader::{PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan},
};

pub(crate) const KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY: &str =
    "phase8-kernel-half-reachability-plan-v1";
pub(crate) const KERNEL_HALF_REACHABILITY_POLICY: &str =
    "preflight-ttbr1-shared-kernel-root-reachability-v1";
pub(crate) const TTBR1_SHARED_KERNEL_ROOT_POLICY: &str = "shared-privileged-kernel-root";
pub(crate) const KERNEL_HALF_DESCRIPTOR_IMAGE_BLOCKED: &str =
    "blocked-no-kernel-half-descriptor-image";
pub(crate) const SPLIT_TCR_COMPATIBILITY_RECORD_ONLY: &str = "split-compatibility-record-only";
pub(crate) const NORMAL_DEVICE_MAIR_COMPATIBILITY_RECORD_ONLY: &str =
    "normal-device-compatibility-record-only";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KernelHalfReachabilityRequest {
    PreflightOnly,
    MissingKernelRange,
    MissingDiagnosticFaultReporting,
    ForbiddenEl0Access,
    BadDeviceAttributeIntent,
    LiveRegisterSequence,
    DescriptorImage,
    PublishSchedulerRunnable,
    LowerElLaunch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfPlanToken(u64);

impl KernelHalfPlanToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfReachabilityLeaseSnapshot {
    pub(crate) plan_records_leased: usize,
    pub(crate) plan_record_releases: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfReachabilityLeaseSource {
    plan_record_capacity: usize,
    next_token: u64,
    plan_records_leased: usize,
    plan_record_releases: usize,
}

impl KernelHalfReachabilityLeaseSource {
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

    pub(crate) const fn snapshot(self) -> KernelHalfReachabilityLeaseSnapshot {
        KernelHalfReachabilityLeaseSnapshot {
            plan_records_leased: self.plan_records_leased,
            plan_record_releases: self.plan_record_releases,
        }
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        self.plan_records_leased
    }

    fn lease_plan_record(&mut self) -> Result<KernelHalfPlanToken, PosixError> {
        if self.plan_records_leased == self.plan_record_capacity {
            return Err(PosixError::NoMemory);
        }
        let token = KernelHalfPlanToken(self.next_token);
        self.next_token += 1;
        self.plan_records_leased += 1;
        Ok(token)
    }

    fn release_plan_record(&mut self, token: &mut KernelHalfReachabilityPlanLease) {
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
pub(crate) struct KernelHalfReachabilityPlanLease {
    token: KernelHalfPlanToken,
    released: bool,
}

impl KernelHalfReachabilityPlanLease {
    pub(crate) const fn token(self) -> KernelHalfPlanToken {
        self.token
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfRootPolicy {
    ttbr0_root: &'static str,
    ttbr0_root_token: u64,
    ttbr0_root_physical_frame: u64,
    ttbr0_written: bool,
    ttbr1_policy: &'static str,
    ttbr1_written: bool,
    descriptor_image: &'static str,
}

impl KernelHalfRootPolicy {
    pub(crate) const fn ttbr0_root(self) -> &'static str {
        self.ttbr0_root
    }

    pub(crate) const fn ttbr0_root_token(self) -> u64 {
        self.ttbr0_root_token
    }

    pub(crate) const fn ttbr0_root_physical_frame(self) -> u64 {
        self.ttbr0_root_physical_frame
    }

    pub(crate) const fn ttbr0_written(self) -> bool {
        self.ttbr0_written
    }

    pub(crate) const fn ttbr1_policy(self) -> &'static str {
        self.ttbr1_policy
    }

    pub(crate) const fn ttbr1_written(self) -> bool {
        self.ttbr1_written
    }

    pub(crate) const fn descriptor_image(self) -> &'static str {
        self.descriptor_image
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelReachabilityEntries {
    kernel_text: bool,
    rodata: bool,
    data: bool,
    bss: bool,
    vectors: bool,
    active_stack: bool,
    heap: bool,
    page_frames: bool,
    uart_mmio_diagnostics: bool,
    scheduler_code_data: bool,
    panic_fault_reporting: bool,
}

impl KernelReachabilityEntries {
    pub(crate) const REQUIRED: Self = Self {
        kernel_text: true,
        rodata: true,
        data: true,
        bss: true,
        vectors: true,
        active_stack: true,
        heap: true,
        page_frames: true,
        uart_mmio_diagnostics: true,
        scheduler_code_data: true,
        panic_fault_reporting: true,
    };

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

    pub(crate) const fn vectors(self) -> bool {
        self.vectors
    }

    pub(crate) const fn active_stack(self) -> bool {
        self.active_stack
    }

    pub(crate) const fn heap(self) -> bool {
        self.heap
    }

    pub(crate) const fn page_frames(self) -> bool {
        self.page_frames
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
        self.kernel_text
            && self.rodata
            && self.data
            && self.bss
            && self.vectors
            && self.active_stack
            && self.heap
            && self.page_frames
            && self.uart_mmio_diagnostics
            && self.scheduler_code_data
            && self.panic_fault_reporting
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfPermissionPolicy {
    text_exec_privileged_only: bool,
    data_exec: bool,
    device_normal_memory: bool,
    el0_kernel_access: bool,
}

impl KernelHalfPermissionPolicy {
    pub(crate) const REQUIRED: Self = Self {
        text_exec_privileged_only: true,
        data_exec: false,
        device_normal_memory: false,
        el0_kernel_access: false,
    };

    pub(crate) const fn text_exec_privileged_only(self) -> bool {
        self.text_exec_privileged_only
    }

    pub(crate) const fn data_exec(self) -> bool {
        self.data_exec
    }

    pub(crate) const fn device_normal_memory(self) -> bool {
        self.device_normal_memory
    }

    pub(crate) const fn el0_kernel_access(self) -> bool {
        self.el0_kernel_access
    }

    const fn valid(self) -> bool {
        self.text_exec_privileged_only
            && !self.data_exec
            && !self.device_normal_memory
            && !self.el0_kernel_access
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfSideEffects {
    ttbr_mutated: bool,
    tcr_mutated: bool,
    mair_mutated: bool,
    sctlr_mutated: bool,
    descriptor_image_installed: bool,
    asid_allocated: bool,
    tlb_mutated: bool,
    live_dsb_isb: bool,
    lower_el_eret: bool,
    scheduler_published: bool,
    process_table_mutated: bool,
    descriptor_table_mutated: bool,
}

impl KernelHalfSideEffects {
    pub(crate) const NONE: Self = Self {
        ttbr_mutated: false,
        tcr_mutated: false,
        mair_mutated: false,
        sctlr_mutated: false,
        descriptor_image_installed: false,
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

    pub(crate) const fn descriptor_image_installed(self) -> bool {
        self.descriptor_image_installed
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
pub(crate) struct KernelHalfReachabilityTeardownReport {
    plan_record_released: bool,
    input_records_owned: bool,
    descriptor_image_installed: bool,
    already_destroyed: bool,
}

impl KernelHalfReachabilityTeardownReport {
    pub(crate) const fn plan_record_released(self) -> bool {
        self.plan_record_released
    }

    pub(crate) const fn input_records_owned(self) -> bool {
        self.input_records_owned
    }

    pub(crate) const fn descriptor_image_installed(self) -> bool {
        self.descriptor_image_installed
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfReachabilityPlan {
    boundary_identity: &'static str,
    policy_identity: &'static str,
    image_fixture_identity: &'static str,
    install_boundary_identity: &'static str,
    address_space_boundary_identity: &'static str,
    materialization_boundary_identity: &'static str,
    launch_boundary_identity: &'static str,
    stack_boundary_identity: &'static str,
    activation_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    address_space_id: u64,
    materialization_id: u64,
    entry_pc: u64,
    initial_sp: u64,
    plan_lease: KernelHalfReachabilityPlanLease,
    root_policy: KernelHalfRootPolicy,
    reachability: KernelReachabilityEntries,
    permission_policy: KernelHalfPermissionPolicy,
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
    side_effects: KernelHalfSideEffects,
    published: bool,
    destroyed: bool,
}

impl KernelHalfReachabilityPlan {
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

    pub(crate) const fn materialization_boundary_identity(self) -> &'static str {
        self.materialization_boundary_identity
    }

    pub(crate) const fn launch_boundary_identity(self) -> &'static str {
        self.launch_boundary_identity
    }

    pub(crate) const fn stack_boundary_identity(self) -> &'static str {
        self.stack_boundary_identity
    }

    pub(crate) const fn activation_boundary_identity(self) -> &'static str {
        self.activation_boundary_identity
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

    pub(crate) const fn plan_lease(self) -> KernelHalfReachabilityPlanLease {
        self.plan_lease
    }

    pub(crate) const fn root_policy(self) -> KernelHalfRootPolicy {
        self.root_policy
    }

    pub(crate) const fn reachability(self) -> KernelReachabilityEntries {
        self.reachability
    }

    pub(crate) const fn permission_policy(self) -> KernelHalfPermissionPolicy {
        self.permission_policy
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

    pub(crate) const fn side_effects(self) -> KernelHalfSideEffects {
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
        lease_source: &mut KernelHalfReachabilityLeaseSource,
    ) -> KernelHalfReachabilityTeardownReport {
        if self.destroyed {
            return KernelHalfReachabilityTeardownReport {
                plan_record_released: false,
                input_records_owned: true,
                descriptor_image_installed: false,
                already_destroyed: true,
            };
        }

        lease_source.release_plan_record(&mut self.plan_lease);
        self.published = false;
        self.destroyed = true;

        KernelHalfReachabilityTeardownReport {
            plan_record_released: true,
            input_records_owned: true,
            descriptor_image_installed: false,
            already_destroyed: false,
        }
    }
}

pub(crate) fn preflight_kernel_half_reachability(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    launch_plan: InitialProcessLaunchPlan,
    stack_plan: InitialUserStackPlan,
    activation_plan: LiveAddressSpaceActivationPlan,
    request: KernelHalfReachabilityRequest,
    lease_source: &mut KernelHalfReachabilityLeaseSource,
) -> Result<KernelHalfReachabilityPlan, PosixError> {
    match request {
        KernelHalfReachabilityRequest::PreflightOnly => {}
        KernelHalfReachabilityRequest::MissingKernelRange => return Err(PosixError::AccessDenied),
        KernelHalfReachabilityRequest::MissingDiagnosticFaultReporting => {
            return Err(PosixError::NotImplemented);
        }
        KernelHalfReachabilityRequest::ForbiddenEl0Access => return Err(PosixError::AccessDenied),
        KernelHalfReachabilityRequest::BadDeviceAttributeIntent => {
            return Err(PosixError::AccessDenied);
        }
        KernelHalfReachabilityRequest::LiveRegisterSequence
        | KernelHalfReachabilityRequest::DescriptorImage
        | KernelHalfReachabilityRequest::PublishSchedulerRunnable
        | KernelHalfReachabilityRequest::LowerElLaunch => return Err(PosixError::NotImplemented),
    }

    validate_lineage(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
    )?;
    let reachability = KernelReachabilityEntries::REQUIRED;
    if !reachability.all_required() {
        return Err(PosixError::NotImplemented);
    }
    let permission_policy = KernelHalfPermissionPolicy::REQUIRED;
    if !permission_policy.valid() {
        return Err(PosixError::AccessDenied);
    }

    let activation_root = activation_plan.root_provenance();
    let plan_lease = KernelHalfReachabilityPlanLease {
        token: lease_source.lease_plan_record()?,
        released: false,
    };

    Ok(KernelHalfReachabilityPlan {
        boundary_identity: KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        policy_identity: KERNEL_HALF_REACHABILITY_POLICY,
        image_fixture_identity: image.fixture_identity(),
        install_boundary_identity: install_plan.install_boundary_identity(),
        address_space_boundary_identity: address_space.boundary_identity(),
        materialization_boundary_identity: materialization.boundary_identity(),
        launch_boundary_identity: launch_plan.boundary_identity(),
        stack_boundary_identity: stack_plan.boundary_identity(),
        activation_boundary_identity: activation_plan.boundary_identity(),
        source_path: image.source_path(),
        source_digest: image.source_digest(),
        address_space_id: address_space.id().raw(),
        materialization_id: materialization.id(),
        entry_pc: image.entry(),
        initial_sp: stack_plan.layout().initial_sp(),
        plan_lease,
        root_policy: KernelHalfRootPolicy {
            ttbr0_root: TTBR0_ROOT_PROVENANCE,
            ttbr0_root_token: activation_root.root_token(),
            ttbr0_root_physical_frame: activation_root.root_physical_frame(),
            ttbr0_written: false,
            ttbr1_policy: TTBR1_SHARED_KERNEL_ROOT_POLICY,
            ttbr1_written: false,
            descriptor_image: KERNEL_HALF_DESCRIPTOR_IMAGE_BLOCKED,
        },
        reachability,
        permission_policy,
        tcr_state: SPLIT_TCR_COMPATIBILITY_RECORD_ONLY,
        mair_state: NORMAL_DEVICE_MAIR_COMPATIBILITY_RECORD_ONLY,
        sctlr_state: SCTLR_MUTATION_BLOCKED,
        asid_state: ASID_ALLOCATION_BLOCKED,
        tlb_state: TLB_INVALIDATION_BLOCKED,
        barrier_state: BARRIER_SEQUENCE_PLANNED_ONLY,
        live_register_sequence_state: LIVE_REGISTER_SEQUENCE_BLOCKED,
        lower_el_eret_state: crate::live_address_space_activation::LOWER_EL_ERET_BLOCKED,
        runnable_publication_state:
            crate::live_address_space_activation::RUNNABLE_PUBLICATION_BLOCKED,
        process_lifecycle_state: crate::live_address_space_activation::PROCESS_LIFECYCLE_BLOCKED,
        startup_abi_state: crate::live_address_space_activation::STARTUP_ABI_EXPANSION_BLOCKED,
        filesystem_syscall_state: crate::live_address_space_activation::FILESYSTEM_SYSCALLS_BLOCKED,
        pi5_hardware_proof_state: crate::live_address_space_activation::PI5_HARDWARE_PROOF_BLOCKED,
        side_effects: KernelHalfSideEffects::NONE,
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
    activation_plan: LiveAddressSpaceActivationPlan,
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
        || materialization.boundary_identity()
            != PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        || !materialization.published()
        || materialization.destroyed()
        || !materialization.activation_blocked()
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
        || activation_plan.boundary_identity() != LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        || activation_plan.image_fixture_identity() != image.fixture_identity()
        || activation_plan.install_boundary_identity() != install_plan.install_boundary_identity()
        || activation_plan.address_space_boundary_identity() != address_space.boundary_identity()
        || activation_plan.materialization_boundary_identity()
            != materialization.boundary_identity()
        || activation_plan.launch_boundary_identity() != launch_plan.boundary_identity()
        || activation_plan.stack_boundary_identity() != stack_plan.boundary_identity()
        || activation_plan.source_path() != image.source_path()
        || activation_plan.source_digest() != image.source_digest()
        || activation_plan.address_space_id() != address_space.id().raw()
        || activation_plan.materialization_id() != materialization.id()
        || activation_plan.entry_pc() != image.entry()
        || activation_plan.initial_sp() != stack_plan.layout().initial_sp()
        || activation_plan.launch_activation_state() != ACTIVATION_PREFLIGHT_READY
        || !activation_plan.published()
        || activation_plan.destroyed()
        || activation_plan.side_effects().ttbr_mutated()
        || activation_plan.side_effects().tcr_mutated()
        || activation_plan.side_effects().mair_mutated()
        || activation_plan.side_effects().sctlr_mutated()
        || activation_plan.side_effects().asid_allocated()
        || activation_plan.side_effects().tlb_mutated()
        || activation_plan.side_effects().live_dsb_isb()
        || activation_plan.side_effects().lower_el_eret()
        || activation_plan.side_effects().scheduler_published()
        || activation_plan.side_effects().process_table_mutated()
        || activation_plan.side_effects().descriptor_table_mutated()
    {
        return Err(PosixError::InvalidArgument);
    }

    let root = activation_plan.root_provenance();
    if root.state() != TTBR0_ROOT_PROVENANCE || root.ttbr0_written() {
        return Err(PosixError::NotExecutable);
    }

    let activation_reachability = activation_plan.kernel_reachability();
    if !activation_reachability.vbar_el1()
        || !activation_reachability.exception_vectors()
        || !activation_reachability.active_kernel_stack()
        || !activation_reachability.kernel_text_data()
        || !activation_reachability.allocator()
        || !activation_reachability.uart_mmio_diagnostics()
        || !activation_reachability.scheduler_code_data()
        || !activation_reachability.panic_fault_reporting()
    {
        return Err(PosixError::NotImplemented);
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
        live_address_space_activation::{
            LiveAddressSpaceActivationLeaseSource, LiveAddressSpaceActivationRequest,
            preflight_live_address_space_activation,
        },
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
        LiveAddressSpaceActivationPlan,
    ) {
        let image =
            plan_phase8_init_image(phase8_readonly_initramfs_fixture()).expect("program image");
        let install_plan = plan_process_image_install(image).expect("install plan");
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(0x8800_5001).expect("address-space id"),
            Some(ProcessOwnerId::new(0x8800_5002).expect("owner id")),
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

        (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
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
    fn builds_preflight_plan_with_copied_input_lineage() {
        let (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        ) = fixture();
        let mut lease_source = KernelHalfReachabilityLeaseSource::for_single_plan();

        let plan = preflight_kernel_half_reachability(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
            KernelHalfReachabilityRequest::PreflightOnly,
            &mut lease_source,
        )
        .expect("kernel-half reachability plan");

        assert_eq!(
            plan.boundary_identity(),
            KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        );
        assert_eq!(plan.policy_identity(), KERNEL_HALF_REACHABILITY_POLICY);
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
        assert_eq!(
            plan.activation_boundary_identity(),
            LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        );
        assert_eq!(plan.source_path(), PHASE8_INIT_PATH);
        assert_eq!(plan.source_digest(), image.source_digest());
        assert_eq!(plan.address_space_id(), address_space.id().raw());
        assert_eq!(plan.materialization_id(), materialization.id());
        assert_eq!(plan.entry_pc(), image.entry());
        assert_eq!(plan.initial_sp(), stack_plan.layout().initial_sp());
        assert!(plan.published());
        assert!(!plan.destroyed());
        assert_ne!(plan.plan_lease().token().raw(), 0);
        assert!(!plan.plan_lease().released());
        assert_eq!(lease_source.outstanding_leases(), 1);
    }

    #[test_case]
    fn records_kernel_half_policy_without_live_side_effects() {
        let (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        ) = fixture();
        let mut lease_source = KernelHalfReachabilityLeaseSource::for_single_plan();

        let plan = preflight_kernel_half_reachability(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
            KernelHalfReachabilityRequest::PreflightOnly,
            &mut lease_source,
        )
        .expect("kernel-half reachability plan");

        let root = plan.root_policy();
        assert_eq!(root.ttbr0_root(), TTBR0_ROOT_PROVENANCE);
        assert_eq!(
            root.ttbr0_root_token(),
            activation_plan.root_provenance().root_token()
        );
        assert_eq!(
            root.ttbr0_root_physical_frame(),
            activation_plan.root_provenance().root_physical_frame()
        );
        assert!(!root.ttbr0_written());
        assert_eq!(root.ttbr1_policy(), TTBR1_SHARED_KERNEL_ROOT_POLICY);
        assert!(!root.ttbr1_written());
        assert_eq!(
            root.descriptor_image(),
            KERNEL_HALF_DESCRIPTOR_IMAGE_BLOCKED
        );

        let reachability = plan.reachability();
        assert!(reachability.kernel_text());
        assert!(reachability.rodata());
        assert!(reachability.data());
        assert!(reachability.bss());
        assert!(reachability.vectors());
        assert!(reachability.active_stack());
        assert!(reachability.heap());
        assert!(reachability.page_frames());
        assert!(reachability.uart_mmio_diagnostics());
        assert!(reachability.scheduler_code_data());
        assert!(reachability.panic_fault_reporting());

        let permissions = plan.permission_policy();
        assert!(permissions.text_exec_privileged_only());
        assert!(!permissions.data_exec());
        assert!(!permissions.device_normal_memory());
        assert!(!permissions.el0_kernel_access());

        assert_eq!(plan.tcr_state(), SPLIT_TCR_COMPATIBILITY_RECORD_ONLY);
        assert_eq!(
            plan.mair_state(),
            NORMAL_DEVICE_MAIR_COMPATIBILITY_RECORD_ONLY
        );
        assert_eq!(plan.sctlr_state(), SCTLR_MUTATION_BLOCKED);
        assert_eq!(plan.asid_state(), ASID_ALLOCATION_BLOCKED);
        assert_eq!(plan.tlb_state(), TLB_INVALIDATION_BLOCKED);
        assert_eq!(plan.barrier_state(), BARRIER_SEQUENCE_PLANNED_ONLY);
        assert_eq!(
            plan.live_register_sequence_state(),
            LIVE_REGISTER_SEQUENCE_BLOCKED
        );
        assert_eq!(
            plan.lower_el_eret_state(),
            crate::live_address_space_activation::LOWER_EL_ERET_BLOCKED
        );
        assert_eq!(
            plan.runnable_publication_state(),
            crate::live_address_space_activation::RUNNABLE_PUBLICATION_BLOCKED
        );
        assert_eq!(
            plan.process_lifecycle_state(),
            crate::live_address_space_activation::PROCESS_LIFECYCLE_BLOCKED
        );
        assert_eq!(
            plan.startup_abi_state(),
            crate::live_address_space_activation::STARTUP_ABI_EXPANSION_BLOCKED
        );
        assert_eq!(
            plan.filesystem_syscall_state(),
            crate::live_address_space_activation::FILESYSTEM_SYSCALLS_BLOCKED
        );
        assert_eq!(
            plan.pi5_hardware_proof_state(),
            crate::live_address_space_activation::PI5_HARDWARE_PROOF_BLOCKED
        );

        let effects = plan.side_effects();
        assert!(!effects.ttbr_mutated());
        assert!(!effects.tcr_mutated());
        assert!(!effects.mair_mutated());
        assert!(!effects.sctlr_mutated());
        assert!(!effects.descriptor_image_installed());
        assert!(!effects.asid_allocated());
        assert!(!effects.tlb_mutated());
        assert!(!effects.live_dsb_isb());
        assert!(!effects.lower_el_eret());
        assert!(!effects.scheduler_published());
        assert!(!effects.process_table_mutated());
        assert!(!effects.descriptor_table_mutated());
    }

    #[test_case]
    fn rejects_blocked_requests_without_partial_plan() {
        let (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        ) = fixture();

        for (request, expected) in [
            (
                KernelHalfReachabilityRequest::MissingKernelRange,
                PosixError::AccessDenied,
            ),
            (
                KernelHalfReachabilityRequest::MissingDiagnosticFaultReporting,
                PosixError::NotImplemented,
            ),
            (
                KernelHalfReachabilityRequest::ForbiddenEl0Access,
                PosixError::AccessDenied,
            ),
            (
                KernelHalfReachabilityRequest::BadDeviceAttributeIntent,
                PosixError::AccessDenied,
            ),
            (
                KernelHalfReachabilityRequest::LiveRegisterSequence,
                PosixError::NotImplemented,
            ),
            (
                KernelHalfReachabilityRequest::DescriptorImage,
                PosixError::NotImplemented,
            ),
            (
                KernelHalfReachabilityRequest::PublishSchedulerRunnable,
                PosixError::NotImplemented,
            ),
            (
                KernelHalfReachabilityRequest::LowerElLaunch,
                PosixError::NotImplemented,
            ),
        ] {
            let mut lease_source = KernelHalfReachabilityLeaseSource::for_single_plan();
            assert_eq!(
                preflight_kernel_half_reachability(
                    image,
                    install_plan,
                    address_space,
                    materialization,
                    launch_plan,
                    stack_plan,
                    activation_plan,
                    request,
                    &mut lease_source,
                ),
                Err(expected)
            );
            assert_eq!(lease_source.outstanding_leases(), 0);
        }
    }

    #[test_case]
    fn teardown_releases_only_plan_local_lease_and_is_idempotent() {
        let (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        ) = fixture();
        let mut lease_source = KernelHalfReachabilityLeaseSource::for_single_plan();
        let mut plan = preflight_kernel_half_reachability(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
            KernelHalfReachabilityRequest::PreflightOnly,
            &mut lease_source,
        )
        .expect("kernel-half reachability plan");

        let first = plan.destroy(&mut lease_source);
        assert!(first.plan_record_released());
        assert!(first.input_records_owned());
        assert!(!first.descriptor_image_installed());
        assert!(!first.already_destroyed());
        assert!(!plan.published());
        assert!(plan.destroyed());
        assert_eq!(lease_source.outstanding_leases(), 0);
        assert_eq!(lease_source.snapshot().plan_record_releases, 1);

        let second = plan.destroy(&mut lease_source);
        assert!(!second.plan_record_released());
        assert!(second.input_records_owned());
        assert!(!second.descriptor_image_installed());
        assert!(second.already_destroyed());
        assert_eq!(lease_source.snapshot().plan_record_releases, 1);
    }

    #[test_case]
    fn rejects_identity_and_lineage_disagreements_before_publication() {
        let (
            _image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        ) = fixture();
        let mut lease_source = KernelHalfReachabilityLeaseSource::for_single_plan();

        assert_eq!(
            preflight_kernel_half_reachability(
                image_with_identity("wrong-fixture"),
                install_plan,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                activation_plan,
                KernelHalfReachabilityRequest::PreflightOnly,
                &mut lease_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);

        let (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        ) = fixture();
        let bad_install = install_with_entry(install_plan, image.entry() + 4);
        let mut lease_source = KernelHalfReachabilityLeaseSource::for_single_plan();
        assert_eq!(
            preflight_kernel_half_reachability(
                image,
                bad_install,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                activation_plan,
                KernelHalfReachabilityRequest::PreflightOnly,
                &mut lease_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rolls_back_resource_exhaustion_without_partial_plan() {
        let (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        ) = fixture();
        let mut lease_source = KernelHalfReachabilityLeaseSource::with_plan_record_capacity(0);

        assert_eq!(
            preflight_kernel_half_reachability(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                activation_plan,
                KernelHalfReachabilityRequest::PreflightOnly,
                &mut lease_source,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);
        assert_eq!(lease_source.snapshot().plan_record_releases, 0);
    }
}
