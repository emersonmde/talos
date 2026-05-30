//! Target-independent initial process launch preparation.
//!
//! This module consumes accepted Phase 8 image/install/address-space and
//! non-activating materialization records and emits an inspectable launch
//! preparation record only. It does not write architectural registers, switch
//! translation tables, allocate a PID, publish scheduler state, build a user
//! stack, or enter lower EL.

use crate::{
    posix::{PosixError, USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, UserMappingPermissions},
    process_address_space::{PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace},
    process_install::{PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan},
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessPageDescriptorRecord,
        ProcessPageTableMaterialization,
    },
    program_loader::{
        LOADER_PAGE_SIZE, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan, UserSegmentKind,
    },
};

pub(crate) const INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY: &str =
    "phase8-initial-process-launch-plan-v1";
pub(crate) const INITIAL_USER_SP_BLOCKED: &str = "blocked-missing-initial-user-stack";
pub(crate) const INITIAL_ACTIVATION_BLOCKED: &str = "blocked-no-ttbr-activation";
pub(crate) const INITIAL_SPSR_BLOCKED: &str = "blocked-pending-lower-el-pstate-policy";
pub(crate) const INITIAL_X0_X5_BLOCKED: &str = "blocked-pending-startup-abi";
pub(crate) const INITIAL_DAIF_BLOCKED: &str = "blocked-pending-interrupt-mask-policy";
pub(crate) const INITIAL_ADDRESS_SPACE_TOKEN_STATE: &str = "model-only";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitialProcessLaunchRequest {
    PreparePlanOnly,
    ActivateAddressSpace,
    StackRequiredLaunch,
    PublishSchedulerRunnable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitialProcessLaunchCommitTarget {
    Runnable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialProcessLaunchCommitRejection {
    error: PosixError,
    no_partial_launch: bool,
    no_runnable_publication: bool,
}

impl InitialProcessLaunchCommitRejection {
    pub(crate) const fn error(self) -> PosixError {
        self.error
    }

    pub(crate) const fn no_partial_launch(self) -> bool {
        self.no_partial_launch
    }

    pub(crate) const fn no_runnable_publication(self) -> bool {
        self.no_runnable_publication
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialSavedFrameIntent {
    elr: u64,
    sp_el0_state: &'static str,
    spsr_state: &'static str,
    x0_x5_state: &'static str,
    daif_state: &'static str,
    address_space_id: u64,
    materialization_id: u64,
    address_space_token_state: &'static str,
}

impl InitialSavedFrameIntent {
    pub(crate) const fn elr(self) -> u64 {
        self.elr
    }

    pub(crate) const fn sp_el0_state(self) -> &'static str {
        self.sp_el0_state
    }

    pub(crate) const fn spsr_state(self) -> &'static str {
        self.spsr_state
    }

    pub(crate) const fn x0_x5_state(self) -> &'static str {
        self.x0_x5_state
    }

    pub(crate) const fn daif_state(self) -> &'static str {
        self.daif_state
    }

    pub(crate) const fn address_space_id(self) -> u64 {
        self.address_space_id
    }

    pub(crate) const fn materialization_id(self) -> u64 {
        self.materialization_id
    }

    pub(crate) const fn address_space_token_state(self) -> &'static str {
        self.address_space_token_state
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialProcessLaunchSideEffects {
    ttbr_mutated: bool,
    tcr_mutated: bool,
    mair_mutated: bool,
    sctlr_mutated: bool,
    asid_allocated: bool,
    tlb_mutated: bool,
    lower_el_eret: bool,
    scheduler_published: bool,
    process_table_mutated: bool,
    descriptor_table_mutated: bool,
}

impl InitialProcessLaunchSideEffects {
    pub(crate) const NONE: Self = Self {
        ttbr_mutated: false,
        tcr_mutated: false,
        mair_mutated: false,
        sctlr_mutated: false,
        asid_allocated: false,
        tlb_mutated: false,
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
pub(crate) struct InitialProcessLaunchPlan {
    boundary_identity: &'static str,
    image_fixture_identity: &'static str,
    install_boundary_identity: &'static str,
    address_space_boundary_identity: &'static str,
    materialization_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    address_space_id: u64,
    materialization_id: u64,
    entry_pc: u64,
    user_sp_state: &'static str,
    activation_state: &'static str,
    saved_frame_intent: InitialSavedFrameIntent,
    side_effects: InitialProcessLaunchSideEffects,
    published: bool,
}

impl InitialProcessLaunchPlan {
    pub(crate) const fn boundary_identity(self) -> &'static str {
        self.boundary_identity
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

    pub(crate) const fn user_sp_state(self) -> &'static str {
        self.user_sp_state
    }

    pub(crate) const fn activation_state(self) -> &'static str {
        self.activation_state
    }

    pub(crate) const fn saved_frame_intent(self) -> InitialSavedFrameIntent {
        self.saved_frame_intent
    }

    pub(crate) const fn side_effects(self) -> InitialProcessLaunchSideEffects {
        self.side_effects
    }

    pub(crate) const fn published(self) -> bool {
        self.published
    }

    pub(crate) const fn commit_request(
        self,
        _target: InitialProcessLaunchCommitTarget,
    ) -> Result<(), InitialProcessLaunchCommitRejection> {
        Err(InitialProcessLaunchCommitRejection {
            error: PosixError::NotImplemented,
            no_partial_launch: true,
            no_runnable_publication: true,
        })
    }

    #[cfg(any(test, talos_boot_scenario = "qemu_initial_user_stack_smoke"))]
    pub(crate) const fn for_test_with_user_sp_state(mut self, state: &'static str) -> Self {
        self.user_sp_state = state;
        self.saved_frame_intent.sp_el0_state = state;
        self
    }
}

pub(crate) fn prepare_initial_process_launch(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    request: InitialProcessLaunchRequest,
) -> Result<InitialProcessLaunchPlan, PosixError> {
    if request != InitialProcessLaunchRequest::PreparePlanOnly {
        return Err(PosixError::NotImplemented);
    }

    validate_identity_lineage(image, install_plan, address_space, materialization)?;
    validate_entry_range(image.entry())?;
    let mapping = entry_mapping(address_space, image.entry())?;
    let descriptor = entry_descriptor(materialization, mapping.virtual_start())?;
    validate_entry_provenance(image, install_plan, mapping, descriptor)?;

    Ok(InitialProcessLaunchPlan {
        boundary_identity: INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        image_fixture_identity: image.fixture_identity(),
        install_boundary_identity: install_plan.install_boundary_identity(),
        address_space_boundary_identity: address_space.boundary_identity(),
        materialization_boundary_identity: materialization.boundary_identity(),
        source_path: image.source_path(),
        source_digest: image.source_digest(),
        address_space_id: address_space.id().raw(),
        materialization_id: materialization.id(),
        entry_pc: image.entry(),
        user_sp_state: INITIAL_USER_SP_BLOCKED,
        activation_state: INITIAL_ACTIVATION_BLOCKED,
        saved_frame_intent: InitialSavedFrameIntent {
            elr: image.entry(),
            sp_el0_state: INITIAL_USER_SP_BLOCKED,
            spsr_state: INITIAL_SPSR_BLOCKED,
            x0_x5_state: INITIAL_X0_X5_BLOCKED,
            daif_state: INITIAL_DAIF_BLOCKED,
            address_space_id: address_space.id().raw(),
            materialization_id: materialization.id(),
            address_space_token_state: INITIAL_ADDRESS_SPACE_TOKEN_STATE,
        },
        side_effects: InitialProcessLaunchSideEffects::NONE,
        published: true,
    })
}

fn validate_identity_lineage(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
) -> Result<(), PosixError> {
    if image.fixture_identity() != PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        || install_plan.fixture_identity() != image.fixture_identity()
        || install_plan.install_boundary_identity() != PROCESS_INSTALL_BOUNDARY_IDENTITY
        || install_plan.source_path() != image.source_path()
        || install_plan.source_digest() != image.source_digest()
        || install_plan.memory_footprint() != image.memory_footprint()
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
        || materialization.descriptor_count() != address_space.mapping_count()
        || materialization.user_frame_count() != address_space.user_frame_lease_count()
    {
        return Err(PosixError::InvalidArgument);
    }
    if install_plan.entry() != image.entry() {
        return Err(PosixError::NotExecutable);
    }
    Ok(())
}

fn validate_entry_range(entry: u64) -> Result<(), PosixError> {
    let instruction_end = entry.checked_add(4).ok_or(PosixError::AccessDenied)?;
    if entry < USER_NULL_GUARD_END || instruction_end > USER_ADDRESS_SPACE_END || entry & 0x3 != 0 {
        return Err(PosixError::AccessDenied);
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

fn entry_descriptor(
    materialization: ProcessPageTableMaterialization,
    virtual_page: u64,
) -> Result<ProcessPageDescriptorRecord, PosixError> {
    let mut index = 0;
    while index < materialization.descriptor_count() {
        if let Some(descriptor) = materialization.descriptor(index)
            && descriptor.virtual_page() == virtual_page
        {
            return Ok(descriptor);
        }
        index += 1;
    }
    Err(PosixError::NotExecutable)
}

fn validate_entry_provenance(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    mapping: crate::process_address_space::ProcessUserMapping,
    descriptor: ProcessPageDescriptorRecord,
) -> Result<(), PosixError> {
    let entry_page = image.entry() & !(LOADER_PAGE_SIZE - 1);
    if mapping.kind() != UserSegmentKind::UserText
        || !mapping
            .permissions()
            .contains(UserMappingPermissions::EXECUTE)
        || mapping.virtual_start() != entry_page
        || descriptor.virtual_page() != mapping.virtual_start()
        || !descriptor.executable()
        || descriptor.user_execute_never()
    {
        return Err(PosixError::NotExecutable);
    }

    let mut index = 0;
    while index < install_plan.page_count() {
        let page = install_plan
            .page(index)
            .ok_or(PosixError::InvalidArgument)?;
        if page.virtual_start() <= image.entry() && image.entry() < page.virtual_end() {
            return if page.kind() == UserSegmentKind::UserText
                && page.permissions().contains(UserMappingPermissions::EXECUTE)
                && page.virtual_start() == mapping.virtual_start()
                && page.virtual_end() == mapping.virtual_end()
            {
                Ok(())
            } else {
                Err(PosixError::NotExecutable)
            };
        }
        index += 1;
    }
    Err(PosixError::NotExecutable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
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

    fn image_fixture() -> ProgramImagePlan {
        plan_phase8_init_image(phase8_readonly_initramfs_fixture()).expect("program image")
    }

    fn install_fixture(image: ProgramImagePlan) -> ProcessImageInstallPlan {
        plan_process_image_install(image).expect("install plan")
    }

    fn fixture() -> (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
    ) {
        let image = image_fixture();
        let install_plan = install_fixture(image);
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(0x8300_3001).expect("address-space id"),
            Some(ProcessOwnerId::new(0x8300_3002).expect("owner id")),
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
        (image, install_plan, address_space, materialization)
    }

    fn image_with_identity(identity: &'static str) -> ProgramImagePlan {
        let image = image_fixture();
        ProgramImagePlan::for_test_unchecked(
            image.source_path(),
            identity,
            image.source_len(),
            image.source_digest(),
            image.entry(),
            image.segment_count(),
            image_segments(image),
            image.memory_start(),
            image.memory_end(),
            image.memory_footprint(),
        )
    }

    fn image_with_entry(entry: u64) -> ProgramImagePlan {
        let image = image_fixture();
        ProgramImagePlan::for_test_unchecked(
            image.source_path(),
            image.fixture_identity(),
            image.source_len(),
            image.source_digest(),
            entry,
            image.segment_count(),
            image_segments(image),
            image.memory_start(),
            image.memory_end(),
            image.memory_footprint(),
        )
    }

    fn install_with_entry(
        install_plan: ProcessImageInstallPlan,
        entry: u64,
    ) -> ProcessImageInstallPlan {
        ProcessImageInstallPlan::for_test_unchecked(
            install_plan.fixture_identity(),
            install_plan.install_boundary_identity(),
            install_plan.source_path(),
            install_plan.source_digest(),
            entry,
            install_plan.memory_footprint(),
            install_plan.page_count(),
            install_pages(install_plan),
            ProcessInstallSideEffects::NONE,
            install_plan.lower_el_launch_blocked(),
        )
    }

    fn image_segments(
        image: ProgramImagePlan,
    ) -> [Option<crate::program_loader::PlannedUserSegment>; MAX_LOAD_SEGMENTS] {
        let mut segments = [None; MAX_LOAD_SEGMENTS];
        let mut index = 0;
        while index < image.segment_count() {
            segments[index] = image.segment(index);
            index += 1;
        }
        segments
    }

    fn install_pages(
        install_plan: ProcessImageInstallPlan,
    ) -> [Option<crate::process_install::ProcessImagePageInstallRecord>;
        crate::process_install::MAX_PROCESS_INSTALL_PAGES] {
        let mut pages = [None; crate::process_install::MAX_PROCESS_INSTALL_PAGES];
        let mut index = 0;
        while index < install_plan.page_count() {
            pages[index] = install_plan.page(index);
            index += 1;
        }
        pages
    }

    #[test_case]
    fn prepares_launch_plan_without_runnable_side_effects() {
        let (image, install_plan, address_space, materialization) = fixture();

        let plan = prepare_initial_process_launch(
            image,
            install_plan,
            address_space,
            materialization,
            InitialProcessLaunchRequest::PreparePlanOnly,
        )
        .expect("launch plan");

        assert_eq!(
            plan.boundary_identity(),
            INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
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
        assert_eq!(plan.source_path(), PHASE8_INIT_PATH);
        assert_eq!(plan.source_digest(), image.source_digest());
        assert_eq!(plan.address_space_id(), address_space.id().raw());
        assert_eq!(plan.materialization_id(), materialization.id());
        assert_eq!(plan.entry_pc(), image.entry());
        assert_eq!(plan.user_sp_state(), INITIAL_USER_SP_BLOCKED);
        assert_eq!(plan.activation_state(), INITIAL_ACTIVATION_BLOCKED);
        assert!(plan.published());

        let frame = plan.saved_frame_intent();
        assert_eq!(frame.elr(), image.entry());
        assert_eq!(frame.sp_el0_state(), INITIAL_USER_SP_BLOCKED);
        assert_eq!(frame.spsr_state(), INITIAL_SPSR_BLOCKED);
        assert_eq!(frame.x0_x5_state(), INITIAL_X0_X5_BLOCKED);
        assert_eq!(frame.daif_state(), INITIAL_DAIF_BLOCKED);
        assert_eq!(frame.address_space_id(), address_space.id().raw());
        assert_eq!(frame.materialization_id(), materialization.id());
        assert_eq!(
            frame.address_space_token_state(),
            INITIAL_ADDRESS_SPACE_TOKEN_STATE
        );

        let side_effects = plan.side_effects();
        assert!(!side_effects.ttbr_mutated());
        assert!(!side_effects.tcr_mutated());
        assert!(!side_effects.mair_mutated());
        assert!(!side_effects.sctlr_mutated());
        assert!(!side_effects.asid_allocated());
        assert!(!side_effects.tlb_mutated());
        assert!(!side_effects.lower_el_eret());
        assert!(!side_effects.scheduler_published());
        assert!(!side_effects.process_table_mutated());
        assert!(!side_effects.descriptor_table_mutated());
    }

    #[test_case]
    fn rejects_runnable_commit_without_partial_launch_or_publication() {
        let (image, install_plan, address_space, materialization) = fixture();
        let plan = prepare_initial_process_launch(
            image,
            install_plan,
            address_space,
            materialization,
            InitialProcessLaunchRequest::PreparePlanOnly,
        )
        .expect("launch plan");

        let rejection = plan
            .commit_request(InitialProcessLaunchCommitTarget::Runnable)
            .expect_err("commit must remain blocked");

        assert_eq!(rejection.error(), PosixError::NotImplemented);
        assert!(rejection.no_partial_launch());
        assert!(rejection.no_runnable_publication());
        assert!(!plan.side_effects().scheduler_published());
    }

    #[test_case]
    fn rejects_bad_identity_before_publication() {
        let (_, install_plan, address_space, materialization) = fixture();

        assert_eq!(
            prepare_initial_process_launch(
                image_with_identity("wrong-fixture"),
                install_plan,
                address_space,
                materialization,
                InitialProcessLaunchRequest::PreparePlanOnly,
            ),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn rejects_entry_mismatch_as_not_executable() {
        let (image, install_plan, address_space, materialization) = fixture();
        let bad_install = install_with_entry(install_plan, image.entry() + 4);

        assert_eq!(
            prepare_initial_process_launch(
                image,
                bad_install,
                address_space,
                materialization,
                InitialProcessLaunchRequest::PreparePlanOnly,
            ),
            Err(PosixError::NotExecutable)
        );
    }

    #[test_case]
    fn rejects_forbidden_entry_range() {
        let (_, install_plan, address_space, materialization) = fixture();
        let bad_image = image_with_entry(USER_NULL_GUARD_END - 4);
        let bad_install = install_with_entry(install_plan, bad_image.entry());

        assert_eq!(
            prepare_initial_process_launch(
                bad_image,
                bad_install,
                address_space,
                materialization,
                InitialProcessLaunchRequest::PreparePlanOnly,
            ),
            Err(PosixError::AccessDenied)
        );
    }

    #[test_case]
    fn rejects_destroyed_inputs_without_partial_launch() {
        let (image, install_plan, mut address_space, materialization) = fixture();
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let report = address_space.destroy(&mut address_source);
        assert!(report.root_released());

        assert_eq!(
            prepare_initial_process_launch(
                image,
                install_plan,
                address_space,
                materialization,
                InitialProcessLaunchRequest::PreparePlanOnly,
            ),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn rejects_activation_stack_and_scheduler_requests() {
        let (image, install_plan, address_space, materialization) = fixture();

        for request in [
            InitialProcessLaunchRequest::ActivateAddressSpace,
            InitialProcessLaunchRequest::StackRequiredLaunch,
            InitialProcessLaunchRequest::PublishSchedulerRunnable,
        ] {
            assert_eq!(
                prepare_initial_process_launch(
                    image,
                    install_plan,
                    address_space,
                    materialization,
                    request,
                ),
                Err(PosixError::NotImplemented)
            );
        }
    }
}
