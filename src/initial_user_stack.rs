//! Target-independent initial user stack construction model.
//!
//! This module consumes accepted Phase 8 image/install/address-space,
//! non-activating materialization, and initial launch records and emits an
//! inspectable stack plan only. It does not write architectural registers,
//! switch translation tables, allocate a PID, publish scheduler state, or make
//! /bin/init runnable.

use crate::{
    initial_process_launch::{
        INITIAL_ACTIVATION_BLOCKED, INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_SP_BLOCKED, InitialProcessLaunchPlan, InitialProcessLaunchSideEffects,
    },
    posix::{PosixError, USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, UserMappingPermissions},
    process_address_space::{PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace},
    process_install::{PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan},
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessPageTableMaterialization,
    },
    program_loader::{
        LOADER_PAGE_SIZE, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan, UserSegmentKind,
    },
};

pub(crate) const INITIAL_USER_STACK_BOUNDARY_IDENTITY: &str = "phase8-initial-user-stack-plan-v1";
pub(crate) const INITIAL_USER_STACK_READY: &str = "model-only-initial-user-stack-ready";
pub(crate) const INITIAL_USER_STACK_STARTUP_PAYLOAD_STATE: &str = "minimal-empty-argc0";
pub(crate) const STARTUP_ABI_BLOCKED: &str = "blocked-pending-startup-abi";
pub(crate) const INITIAL_USER_STACK_USABLE_PAGES: usize = 4;
pub(crate) const INITIAL_USER_STACK_GUARD_PAGES: usize = 1;
pub(crate) const INITIAL_USER_STACK_PAGE_COUNT: usize = INITIAL_USER_STACK_USABLE_PAGES;
pub(crate) const INITIAL_USER_STACK_USABLE_BYTES: u64 =
    LOADER_PAGE_SIZE * INITIAL_USER_STACK_USABLE_PAGES as u64;
pub(crate) const INITIAL_USER_STACK_GUARD_BYTES: u64 =
    LOADER_PAGE_SIZE * INITIAL_USER_STACK_GUARD_PAGES as u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitialUserStackRequest {
    PlanOnly,
    LiveLaunch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitialUserStackCommitTarget {
    Runnable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackCommitRejection {
    error: PosixError,
    no_partial_launch: bool,
    no_runnable_publication: bool,
}

impl InitialUserStackCommitRejection {
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
pub(crate) struct InitialUserStackLeaseToken(u64);

impl InitialUserStackLeaseToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackLeaseSnapshot {
    pub(crate) stack_frames_leased: usize,
    pub(crate) stack_frame_releases: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackLeaseSource {
    stack_frame_capacity: usize,
    next_token: u64,
    stack_frames_leased: usize,
    stack_frame_releases: usize,
    #[cfg(test)]
    layout_override: Option<InitialUserStackLayout>,
    #[cfg(test)]
    permission_override: Option<UserMappingPermissions>,
}

impl InitialUserStackLeaseSource {
    pub(crate) const fn with_stack_frame_capacity(stack_frame_capacity: usize) -> Self {
        Self {
            stack_frame_capacity,
            next_token: 1,
            stack_frames_leased: 0,
            stack_frame_releases: 0,
            #[cfg(test)]
            layout_override: None,
            #[cfg(test)]
            permission_override: None,
        }
    }

    pub(crate) const fn for_initial_stack() -> Self {
        Self::with_stack_frame_capacity(INITIAL_USER_STACK_USABLE_PAGES)
    }

    pub(crate) const fn snapshot(self) -> InitialUserStackLeaseSnapshot {
        InitialUserStackLeaseSnapshot {
            stack_frames_leased: self.stack_frames_leased,
            stack_frame_releases: self.stack_frame_releases,
        }
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        self.stack_frames_leased
    }

    #[cfg(test)]
    fn override_layout(&mut self, layout: InitialUserStackLayout) {
        self.layout_override = Some(layout);
    }

    #[cfg(test)]
    fn override_permissions(&mut self, permissions: UserMappingPermissions) {
        self.permission_override = Some(permissions);
    }

    fn next_token(&mut self) -> InitialUserStackLeaseToken {
        let token = InitialUserStackLeaseToken(self.next_token);
        self.next_token += 1;
        token
    }

    fn lease_stack_frame(&mut self) -> Result<InitialUserStackLeaseToken, PosixError> {
        if self.stack_frames_leased == self.stack_frame_capacity {
            return Err(PosixError::NoMemory);
        }
        self.stack_frames_leased += 1;
        Ok(self.next_token())
    }

    fn release_stack_frame(&mut self, lease: &mut InitialUserStackPageLease) {
        if !lease.released {
            lease.released = true;
            if self.stack_frames_leased != 0 {
                self.stack_frames_leased -= 1;
            }
            self.stack_frame_releases += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackLayout {
    stack_top: u64,
    initial_sp: u64,
    usable_start: u64,
    usable_end: u64,
    guard_start: u64,
    guard_end: u64,
    page_size: u64,
    usable_pages: usize,
    guard_pages: usize,
    permissions: UserMappingPermissions,
}

impl InitialUserStackLayout {
    pub(crate) const fn stack_top(self) -> u64 {
        self.stack_top
    }

    pub(crate) const fn initial_sp(self) -> u64 {
        self.initial_sp
    }

    pub(crate) const fn usable_start(self) -> u64 {
        self.usable_start
    }

    pub(crate) const fn usable_end(self) -> u64 {
        self.usable_end
    }

    pub(crate) const fn guard_start(self) -> u64 {
        self.guard_start
    }

    pub(crate) const fn guard_end(self) -> u64 {
        self.guard_end
    }

    pub(crate) const fn page_size(self) -> u64 {
        self.page_size
    }

    pub(crate) const fn usable_pages(self) -> usize {
        self.usable_pages
    }

    pub(crate) const fn guard_pages(self) -> usize {
        self.guard_pages
    }

    pub(crate) const fn permissions(self) -> UserMappingPermissions {
        self.permissions
    }

    pub(crate) const fn sp_aligned_16(self) -> bool {
        self.initial_sp & 0xf == 0
    }

    #[cfg(test)]
    const fn for_test_unchecked(
        stack_top: u64,
        initial_sp: u64,
        usable_start: u64,
        usable_end: u64,
        guard_start: u64,
        guard_end: u64,
        page_size: u64,
        usable_pages: usize,
        guard_pages: usize,
        permissions: UserMappingPermissions,
    ) -> Self {
        Self {
            stack_top,
            initial_sp,
            usable_start,
            usable_end,
            guard_start,
            guard_end,
            page_size,
            usable_pages,
            guard_pages,
            permissions,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackPageLease {
    token: InitialUserStackLeaseToken,
    virtual_page: u64,
    permissions: UserMappingPermissions,
    zeroed_before_copy: bool,
    copied_bytes: u64,
    zeroed_bytes: u64,
    source_page_ordinal: usize,
    released: bool,
}

impl InitialUserStackPageLease {
    pub(crate) const fn token(self) -> InitialUserStackLeaseToken {
        self.token
    }

    pub(crate) const fn virtual_page(self) -> u64 {
        self.virtual_page
    }

    pub(crate) const fn permissions(self) -> UserMappingPermissions {
        self.permissions
    }

    pub(crate) const fn zeroed_before_copy(self) -> bool {
        self.zeroed_before_copy
    }

    pub(crate) const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub(crate) const fn zeroed_bytes(self) -> u64 {
        self.zeroed_bytes
    }

    pub(crate) const fn source_page_ordinal(self) -> usize {
        self.source_page_ordinal
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackStartupPayload {
    state: &'static str,
    argc: usize,
    argv_null: bool,
    envp_null: bool,
    auxv_state: &'static str,
    tls_state: &'static str,
    copied_startup_bytes: u64,
}

impl InitialUserStackStartupPayload {
    pub(crate) const fn state(self) -> &'static str {
        self.state
    }

    pub(crate) const fn argc(self) -> usize {
        self.argc
    }

    pub(crate) const fn argv_null(self) -> bool {
        self.argv_null
    }

    pub(crate) const fn envp_null(self) -> bool {
        self.envp_null
    }

    pub(crate) const fn auxv_state(self) -> &'static str {
        self.auxv_state
    }

    pub(crate) const fn tls_state(self) -> &'static str {
        self.tls_state
    }

    pub(crate) const fn copied_startup_bytes(self) -> u64 {
        self.copied_startup_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackLaunchBinding {
    user_sp_state: &'static str,
    saved_frame_sp_el0: u64,
    startup_payload_state: &'static str,
    activation_state: &'static str,
    no_partial_launch: bool,
    no_runnable_publication: bool,
    side_effects: InitialProcessLaunchSideEffects,
}

impl InitialUserStackLaunchBinding {
    pub(crate) const fn user_sp_state(self) -> &'static str {
        self.user_sp_state
    }

    pub(crate) const fn saved_frame_sp_el0(self) -> u64 {
        self.saved_frame_sp_el0
    }

    pub(crate) const fn startup_payload_state(self) -> &'static str {
        self.startup_payload_state
    }

    pub(crate) const fn activation_state(self) -> &'static str {
        self.activation_state
    }

    pub(crate) const fn no_partial_launch(self) -> bool {
        self.no_partial_launch
    }

    pub(crate) const fn no_runnable_publication(self) -> bool {
        self.no_runnable_publication
    }

    pub(crate) const fn side_effects(self) -> InitialProcessLaunchSideEffects {
        self.side_effects
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackTeardownReport {
    stack_frame_releases: usize,
    image_leases_untouched: bool,
    already_destroyed: bool,
}

impl InitialUserStackTeardownReport {
    pub(crate) const fn stack_frame_releases(self) -> usize {
        self.stack_frame_releases
    }

    pub(crate) const fn image_leases_untouched(self) -> bool {
        self.image_leases_untouched
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitialUserStackPlan {
    boundary_identity: &'static str,
    image_fixture_identity: &'static str,
    install_boundary_identity: &'static str,
    address_space_boundary_identity: &'static str,
    materialization_boundary_identity: &'static str,
    launch_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    address_space_id: u64,
    materialization_id: u64,
    entry_pc: u64,
    layout: InitialUserStackLayout,
    page_leases: [Option<InitialUserStackPageLease>; INITIAL_USER_STACK_PAGE_COUNT],
    page_lease_count: usize,
    guard_pages_reserved: usize,
    total_copied_bytes: u64,
    total_zeroed_bytes: u64,
    startup_payload: InitialUserStackStartupPayload,
    launch_binding: InitialUserStackLaunchBinding,
    published: bool,
    destroyed: bool,
}

impl InitialUserStackPlan {
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

    pub(crate) const fn launch_boundary_identity(self) -> &'static str {
        self.launch_boundary_identity
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

    pub(crate) const fn layout(self) -> InitialUserStackLayout {
        self.layout
    }

    pub(crate) const fn page_lease_count(self) -> usize {
        self.page_lease_count
    }

    pub(crate) const fn page_lease(self, index: usize) -> Option<InitialUserStackPageLease> {
        if index >= INITIAL_USER_STACK_PAGE_COUNT {
            None
        } else {
            self.page_leases[index]
        }
    }

    pub(crate) const fn guard_pages_reserved(self) -> usize {
        self.guard_pages_reserved
    }

    pub(crate) const fn total_copied_bytes(self) -> u64 {
        self.total_copied_bytes
    }

    pub(crate) const fn total_zeroed_bytes(self) -> u64 {
        self.total_zeroed_bytes
    }

    pub(crate) const fn startup_payload(self) -> InitialUserStackStartupPayload {
        self.startup_payload
    }

    pub(crate) const fn launch_binding(self) -> InitialUserStackLaunchBinding {
        self.launch_binding
    }

    pub(crate) const fn published(self) -> bool {
        self.published
    }

    pub(crate) const fn destroyed(self) -> bool {
        self.destroyed
    }

    pub(crate) const fn commit_request(
        self,
        _target: InitialUserStackCommitTarget,
    ) -> Result<(), InitialUserStackCommitRejection> {
        Err(InitialUserStackCommitRejection {
            error: PosixError::NotImplemented,
            no_partial_launch: true,
            no_runnable_publication: true,
        })
    }

    pub(crate) fn destroy(
        &mut self,
        lease_source: &mut InitialUserStackLeaseSource,
    ) -> InitialUserStackTeardownReport {
        if self.destroyed {
            return InitialUserStackTeardownReport {
                stack_frame_releases: 0,
                image_leases_untouched: true,
                already_destroyed: true,
            };
        }

        let mut stack_frame_releases = 0;
        while self.page_lease_count != 0 {
            self.page_lease_count -= 1;
            if let Some(mut lease) = self.page_leases[self.page_lease_count] {
                lease_source.release_stack_frame(&mut lease);
                self.page_leases[self.page_lease_count] = Some(lease);
                stack_frame_releases += 1;
            }
        }
        self.published = false;
        self.destroyed = true;

        InitialUserStackTeardownReport {
            stack_frame_releases,
            image_leases_untouched: true,
            already_destroyed: false,
        }
    }
}

pub(crate) fn plan_initial_user_stack(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    launch_plan: InitialProcessLaunchPlan,
    request: InitialUserStackRequest,
    lease_source: &mut InitialUserStackLeaseSource,
) -> Result<InitialUserStackPlan, PosixError> {
    if request != InitialUserStackRequest::PlanOnly {
        return Err(PosixError::NotImplemented);
    }

    validate_lineage(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
    )?;

    let mut layout = default_layout()?;
    #[cfg(test)]
    if let Some(override_layout) = lease_source.layout_override {
        layout = override_layout;
    }
    #[cfg(test)]
    if let Some(permissions) = lease_source.permission_override {
        layout.permissions = permissions;
    }

    validate_layout(layout)?;
    validate_no_overlap(image, install_plan, address_space, materialization, layout)?;

    let mut partial = PartialInitialUserStack {
        page_leases: [None; INITIAL_USER_STACK_PAGE_COUNT],
        page_lease_count: 0,
        total_copied_bytes: 0,
        total_zeroed_bytes: 0,
    };

    let result = lease_stack_pages(layout, lease_source, &mut partial);
    match result {
        Ok(()) => Ok(InitialUserStackPlan {
            boundary_identity: INITIAL_USER_STACK_BOUNDARY_IDENTITY,
            image_fixture_identity: image.fixture_identity(),
            install_boundary_identity: install_plan.install_boundary_identity(),
            address_space_boundary_identity: address_space.boundary_identity(),
            materialization_boundary_identity: materialization.boundary_identity(),
            launch_boundary_identity: launch_plan.boundary_identity(),
            source_path: image.source_path(),
            source_digest: image.source_digest(),
            address_space_id: address_space.id().raw(),
            materialization_id: materialization.id(),
            entry_pc: image.entry(),
            layout,
            page_leases: partial.page_leases,
            page_lease_count: partial.page_lease_count,
            guard_pages_reserved: INITIAL_USER_STACK_GUARD_PAGES,
            total_copied_bytes: partial.total_copied_bytes,
            total_zeroed_bytes: partial.total_zeroed_bytes,
            startup_payload: InitialUserStackStartupPayload {
                state: INITIAL_USER_STACK_STARTUP_PAYLOAD_STATE,
                argc: 0,
                argv_null: true,
                envp_null: true,
                auxv_state: STARTUP_ABI_BLOCKED,
                tls_state: STARTUP_ABI_BLOCKED,
                copied_startup_bytes: 0,
            },
            launch_binding: InitialUserStackLaunchBinding {
                user_sp_state: INITIAL_USER_STACK_READY,
                saved_frame_sp_el0: layout.initial_sp(),
                startup_payload_state: INITIAL_USER_STACK_STARTUP_PAYLOAD_STATE,
                activation_state: INITIAL_ACTIVATION_BLOCKED,
                no_partial_launch: true,
                no_runnable_publication: true,
                side_effects: InitialProcessLaunchSideEffects::NONE,
            },
            published: true,
            destroyed: false,
        }),
        Err(error) => {
            rollback_partial(lease_source, &mut partial);
            Err(error)
        }
    }
}

struct PartialInitialUserStack {
    page_leases: [Option<InitialUserStackPageLease>; INITIAL_USER_STACK_PAGE_COUNT],
    page_lease_count: usize,
    total_copied_bytes: u64,
    total_zeroed_bytes: u64,
}

fn default_layout() -> Result<InitialUserStackLayout, PosixError> {
    let usable_end = USER_ADDRESS_SPACE_END;
    let usable_start = usable_end
        .checked_sub(INITIAL_USER_STACK_USABLE_BYTES)
        .ok_or(PosixError::Fault)?;
    let guard_start = usable_start
        .checked_sub(INITIAL_USER_STACK_GUARD_BYTES)
        .ok_or(PosixError::Fault)?;
    Ok(InitialUserStackLayout {
        stack_top: USER_ADDRESS_SPACE_END,
        initial_sp: USER_ADDRESS_SPACE_END,
        usable_start,
        usable_end,
        guard_start,
        guard_end: usable_start,
        page_size: LOADER_PAGE_SIZE,
        usable_pages: INITIAL_USER_STACK_USABLE_PAGES,
        guard_pages: INITIAL_USER_STACK_GUARD_PAGES,
        permissions: UserMappingPermissions::USER_DATA,
    })
}

fn validate_lineage(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    launch_plan: InitialProcessLaunchPlan,
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
        || !launch_plan.published()
    {
        return Err(PosixError::InvalidArgument);
    }

    if launch_plan.user_sp_state() != INITIAL_USER_SP_BLOCKED
        || launch_plan.saved_frame_intent().sp_el0_state() != INITIAL_USER_SP_BLOCKED
        || launch_plan.activation_state() != INITIAL_ACTIVATION_BLOCKED
    {
        return Err(PosixError::InvalidArgument);
    }
    if launch_plan.saved_frame_intent().elr() != image.entry()
        || launch_plan.saved_frame_intent().address_space_id() != address_space.id().raw()
        || launch_plan.saved_frame_intent().materialization_id() != materialization.id()
    {
        return Err(PosixError::NotExecutable);
    }
    if launch_plan.side_effects() != InitialProcessLaunchSideEffects::NONE {
        return Err(PosixError::InvalidArgument);
    }
    validate_entry_provenance(image, install_plan, address_space, materialization)
}

fn validate_entry_provenance(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
) -> Result<(), PosixError> {
    let entry_page = image.entry() & !(LOADER_PAGE_SIZE - 1);
    let mut mapping_index = 0;
    while mapping_index < address_space.mapping_count() {
        let mapping = address_space
            .mapping(mapping_index)
            .ok_or(PosixError::InvalidArgument)?;
        if mapping.virtual_start() <= image.entry() && image.entry() < mapping.virtual_end() {
            if mapping.kind() != UserSegmentKind::UserText
                || !mapping
                    .permissions()
                    .contains(UserMappingPermissions::EXECUTE)
                || mapping.virtual_start() != entry_page
            {
                return Err(PosixError::NotExecutable);
            }
            let page = install_plan
                .page(mapping_index)
                .ok_or(PosixError::InvalidArgument)?;
            if page.kind() != UserSegmentKind::UserText
                || !page.permissions().contains(UserMappingPermissions::EXECUTE)
                || page.virtual_start() != mapping.virtual_start()
                || page.virtual_end() != mapping.virtual_end()
            {
                return Err(PosixError::NotExecutable);
            }
            let descriptor = materialization
                .descriptor(mapping_index)
                .ok_or(PosixError::NotExecutable)?;
            if descriptor.virtual_page() != mapping.virtual_start()
                || !descriptor.executable()
                || descriptor.user_execute_never()
            {
                return Err(PosixError::NotExecutable);
            }
            return Ok(());
        }
        mapping_index += 1;
    }
    Err(PosixError::NotExecutable)
}

fn validate_layout(layout: InitialUserStackLayout) -> Result<(), PosixError> {
    if layout.page_size != LOADER_PAGE_SIZE
        || layout.usable_pages != INITIAL_USER_STACK_USABLE_PAGES
        || layout.guard_pages != INITIAL_USER_STACK_GUARD_PAGES
        || layout.stack_top != USER_ADDRESS_SPACE_END
        || layout.initial_sp != layout.stack_top
        || !layout.sp_aligned_16()
        || layout.usable_start % LOADER_PAGE_SIZE != 0
        || layout.usable_end % LOADER_PAGE_SIZE != 0
        || layout.guard_start % LOADER_PAGE_SIZE != 0
        || layout.guard_end % LOADER_PAGE_SIZE != 0
    {
        return Err(PosixError::Fault);
    }
    if layout.permissions != UserMappingPermissions::USER_DATA
        || layout.permissions.contains(UserMappingPermissions::EXECUTE)
    {
        return Err(PosixError::AccessDenied);
    }
    if layout
        .usable_start
        .checked_add(INITIAL_USER_STACK_USABLE_BYTES)
        .ok_or(PosixError::Fault)?
        != layout.usable_end
        || layout
            .guard_start
            .checked_add(INITIAL_USER_STACK_GUARD_BYTES)
            .ok_or(PosixError::Fault)?
            != layout.guard_end
        || layout.guard_end != layout.usable_start
        || layout.usable_end != USER_ADDRESS_SPACE_END
        || layout.guard_start < USER_NULL_GUARD_END
        || layout.usable_start < USER_NULL_GUARD_END
        || layout.usable_end > USER_ADDRESS_SPACE_END
    {
        return Err(PosixError::Fault);
    }
    Ok(())
}

fn validate_no_overlap(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    materialization: ProcessPageTableMaterialization,
    layout: InitialUserStackLayout,
) -> Result<(), PosixError> {
    let mut segment_index = 0;
    while segment_index < image.segment_count() {
        let segment = image
            .segment(segment_index)
            .ok_or(PosixError::InvalidArgument)?;
        if ranges_overlap(
            layout.guard_start(),
            layout.usable_end(),
            segment.rounded_start(),
            segment.rounded_end(),
        ) {
            return Err(PosixError::AccessDenied);
        }
        segment_index += 1;
    }

    let mut page_index = 0;
    while page_index < install_plan.page_count() {
        let page = install_plan
            .page(page_index)
            .ok_or(PosixError::InvalidArgument)?;
        if ranges_overlap(
            layout.guard_start(),
            layout.usable_end(),
            page.virtual_start(),
            page.virtual_end(),
        ) {
            return Err(PosixError::AccessDenied);
        }
        page_index += 1;
    }

    let mut mapping_index = 0;
    while mapping_index < address_space.mapping_count() {
        let mapping = address_space
            .mapping(mapping_index)
            .ok_or(PosixError::InvalidArgument)?;
        if ranges_overlap(
            layout.guard_start(),
            layout.usable_end(),
            mapping.virtual_start(),
            mapping.virtual_end(),
        ) {
            return Err(PosixError::AccessDenied);
        }
        mapping_index += 1;
    }

    let mut descriptor_index = 0;
    while descriptor_index < materialization.descriptor_count() {
        let descriptor = materialization
            .descriptor(descriptor_index)
            .ok_or(PosixError::InvalidArgument)?;
        let descriptor_end = descriptor
            .virtual_page()
            .checked_add(LOADER_PAGE_SIZE)
            .ok_or(PosixError::InvalidArgument)?;
        if ranges_overlap(
            layout.guard_start(),
            layout.usable_end(),
            descriptor.virtual_page(),
            descriptor_end,
        ) {
            return Err(PosixError::AccessDenied);
        }
        descriptor_index += 1;
    }

    Ok(())
}

fn ranges_overlap(left_start: u64, left_end: u64, right_start: u64, right_end: u64) -> bool {
    left_start < right_end && right_start < left_end
}

fn lease_stack_pages(
    layout: InitialUserStackLayout,
    lease_source: &mut InitialUserStackLeaseSource,
    partial: &mut PartialInitialUserStack,
) -> Result<(), PosixError> {
    let mut page_index = 0;
    while page_index < INITIAL_USER_STACK_USABLE_PAGES {
        let virtual_page = layout.usable_start + page_index as u64 * LOADER_PAGE_SIZE;
        let token = lease_source.lease_stack_frame()?;
        let lease = InitialUserStackPageLease {
            token,
            virtual_page,
            permissions: layout.permissions,
            zeroed_before_copy: true,
            copied_bytes: 0,
            zeroed_bytes: LOADER_PAGE_SIZE,
            source_page_ordinal: page_index,
            released: false,
        };
        partial.page_leases[partial.page_lease_count] = Some(lease);
        partial.page_lease_count += 1;
        partial.total_zeroed_bytes += LOADER_PAGE_SIZE;
        page_index += 1;
    }
    Ok(())
}

fn rollback_partial(
    lease_source: &mut InitialUserStackLeaseSource,
    partial: &mut PartialInitialUserStack,
) {
    while partial.page_lease_count != 0 {
        partial.page_lease_count -= 1;
        if let Some(mut lease) = partial.page_leases[partial.page_lease_count] {
            lease_source.release_stack_frame(&mut lease);
            partial.page_leases[partial.page_lease_count] = Some(lease);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initial_process_launch::{InitialProcessLaunchRequest, prepare_initial_process_launch},
        initramfs::{PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
        process_address_space::{
            ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource, install_process_address_space,
        },
        process_install::plan_process_image_install,
        process_page_table_materialization::{
            ProcessMaterializationRequest, ProcessPageTableMaterializationLeaseSource,
            materialize_process_page_tables,
        },
        program_loader::plan_phase8_init_image,
        scheduler::ProcessOwnerId,
    };

    fn fixture() -> (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        InitialProcessLaunchPlan,
    ) {
        let image =
            plan_phase8_init_image(phase8_readonly_initramfs_fixture()).expect("program image");
        let install_plan = plan_process_image_install(image).expect("install plan");
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(0x8400_3001).expect("address-space id"),
            Some(ProcessOwnerId::new(0x8400_3002).expect("owner id")),
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
        (
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
        )
    }

    fn plan_fixture() -> (InitialUserStackPlan, InitialUserStackLeaseSource) {
        let (image, install_plan, address_space, materialization, launch_plan) = fixture();
        let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
        let plan = plan_initial_user_stack(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            InitialUserStackRequest::PlanOnly,
            &mut stack_source,
        )
        .expect("stack plan");
        (plan, stack_source)
    }

    fn image_with_stack_overlap(image: ProgramImagePlan) -> ProgramImagePlan {
        let mut segments = [None; crate::program_loader::MAX_LOAD_SEGMENTS];
        let mut index = 0;
        while index < image.segment_count() {
            segments[index] = image.segment(index);
            index += 1;
        }
        segments[index] = Some(
            crate::program_loader::PlannedUserSegment::for_test_unchecked(
                UserSegmentKind::UserData,
                UserMappingPermissions::USER_DATA,
                0x0000_7fff_ffff_c000,
                0x0000_8000_0000_0000,
                0x0000_7fff_ffff_c000,
                0x0000_8000_0000_0000,
                0,
                0,
                0x0000_7fff_ffff_c000,
                0x0000_8000_0000_0000,
            ),
        );
        ProgramImagePlan::for_test_unchecked(
            image.source_path(),
            image.fixture_identity(),
            image.source_len(),
            image.source_digest(),
            image.entry(),
            image.segment_count() + 1,
            segments,
            image.memory_start(),
            image.memory_end(),
            image.memory_footprint(),
        )
    }

    #[test_case]
    fn builds_initial_stack_plan_with_fixed_layout_and_accounting() {
        let (plan, stack_source) = plan_fixture();

        assert_eq!(
            plan.boundary_identity(),
            INITIAL_USER_STACK_BOUNDARY_IDENTITY
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
        assert_eq!(plan.source_path(), PHASE8_INIT_PATH);
        assert_ne!(plan.source_digest(), 0);
        assert_ne!(plan.address_space_id(), 0);
        assert_ne!(plan.materialization_id(), 0);
        assert_eq!(plan.entry_pc(), 0x0000_0000_0001_0100);
        assert!(plan.published());
        assert!(!plan.destroyed());

        let layout = plan.layout();
        assert_eq!(layout.stack_top(), USER_ADDRESS_SPACE_END);
        assert_eq!(layout.initial_sp(), USER_ADDRESS_SPACE_END);
        assert!(layout.sp_aligned_16());
        assert_eq!(layout.usable_start(), 0x0000_7fff_ffff_c000);
        assert_eq!(layout.usable_end(), 0x0000_8000_0000_0000);
        assert_eq!(layout.guard_start(), 0x0000_7fff_ffff_b000);
        assert_eq!(layout.guard_end(), 0x0000_7fff_ffff_c000);
        assert_eq!(layout.page_size(), LOADER_PAGE_SIZE);
        assert_eq!(layout.usable_pages(), INITIAL_USER_STACK_USABLE_PAGES);
        assert_eq!(layout.guard_pages(), INITIAL_USER_STACK_GUARD_PAGES);
        assert_eq!(layout.permissions(), UserMappingPermissions::USER_DATA);

        assert_eq!(plan.page_lease_count(), INITIAL_USER_STACK_USABLE_PAGES);
        assert_eq!(plan.guard_pages_reserved(), INITIAL_USER_STACK_GUARD_PAGES);
        assert_eq!(plan.total_copied_bytes(), 0);
        assert_eq!(plan.total_zeroed_bytes(), INITIAL_USER_STACK_USABLE_BYTES);
        assert_eq!(
            stack_source.outstanding_leases(),
            INITIAL_USER_STACK_USABLE_PAGES
        );
        assert_eq!(
            stack_source.snapshot().stack_frames_leased,
            INITIAL_USER_STACK_USABLE_PAGES
        );

        let mut index = 0;
        while index < plan.page_lease_count() {
            let lease = plan.page_lease(index).expect("stack page lease");
            assert_ne!(lease.token().raw(), 0);
            assert_eq!(
                lease.virtual_page(),
                layout.usable_start() + index as u64 * LOADER_PAGE_SIZE
            );
            assert_eq!(lease.permissions(), UserMappingPermissions::USER_DATA);
            assert!(lease.zeroed_before_copy());
            assert_eq!(lease.copied_bytes(), 0);
            assert_eq!(lease.zeroed_bytes(), LOADER_PAGE_SIZE);
            assert_eq!(lease.source_page_ordinal(), index);
            assert!(!lease.released());
            index += 1;
        }
    }

    #[test_case]
    fn records_empty_startup_payload_and_launch_binding_without_live_side_effects() {
        let (plan, _) = plan_fixture();
        let payload = plan.startup_payload();
        assert_eq!(payload.state(), INITIAL_USER_STACK_STARTUP_PAYLOAD_STATE);
        assert_eq!(payload.argc(), 0);
        assert!(payload.argv_null());
        assert!(payload.envp_null());
        assert_eq!(payload.auxv_state(), STARTUP_ABI_BLOCKED);
        assert_eq!(payload.tls_state(), STARTUP_ABI_BLOCKED);
        assert_eq!(payload.copied_startup_bytes(), 0);

        let binding = plan.launch_binding();
        assert_eq!(binding.user_sp_state(), INITIAL_USER_STACK_READY);
        assert_eq!(binding.saved_frame_sp_el0(), plan.layout().initial_sp());
        assert_eq!(
            binding.startup_payload_state(),
            INITIAL_USER_STACK_STARTUP_PAYLOAD_STATE
        );
        assert_eq!(binding.activation_state(), INITIAL_ACTIVATION_BLOCKED);
        assert!(binding.no_partial_launch());
        assert!(binding.no_runnable_publication());

        let side_effects = binding.side_effects();
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
    fn teardown_releases_only_stack_owned_leases_and_is_idempotent() {
        let (mut plan, mut stack_source) = plan_fixture();

        let first = plan.destroy(&mut stack_source);
        assert_eq!(
            first.stack_frame_releases(),
            INITIAL_USER_STACK_USABLE_PAGES
        );
        assert!(first.image_leases_untouched());
        assert!(!first.already_destroyed());
        assert!(plan.destroyed());
        assert!(!plan.published());
        assert_eq!(stack_source.outstanding_leases(), 0);
        assert_eq!(
            stack_source.snapshot().stack_frame_releases,
            INITIAL_USER_STACK_USABLE_PAGES
        );

        let second = plan.destroy(&mut stack_source);
        assert_eq!(second.stack_frame_releases(), 0);
        assert!(second.image_leases_untouched());
        assert!(second.already_destroyed());
    }

    #[test_case]
    fn rejects_live_launch_requests_without_partial_launch_or_publication() {
        let (image, install_plan, address_space, materialization, launch_plan) = fixture();
        let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();

        assert_eq!(
            plan_initial_user_stack(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                InitialUserStackRequest::LiveLaunch,
                &mut stack_source,
            ),
            Err(PosixError::NotImplemented)
        );
        assert_eq!(stack_source.outstanding_leases(), 0);

        let (plan, _) = plan_fixture();
        let rejection = plan
            .commit_request(InitialUserStackCommitTarget::Runnable)
            .expect_err("runnable publication remains blocked");
        assert_eq!(rejection.error(), PosixError::NotImplemented);
        assert!(rejection.no_partial_launch());
        assert!(rejection.no_runnable_publication());
        assert!(!plan.launch_binding().side_effects().scheduler_published());
    }

    #[test_case]
    fn rejects_already_stack_ready_launch_input_without_leasing() {
        let (image, install_plan, address_space, materialization, launch_plan) = fixture();
        let stack_ready_launch = launch_plan.for_test_with_user_sp_state(INITIAL_USER_STACK_READY);
        let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();

        assert_eq!(
            plan_initial_user_stack(
                image,
                install_plan,
                address_space,
                materialization,
                stack_ready_launch,
                InitialUserStackRequest::PlanOnly,
                &mut stack_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(stack_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rejects_bad_stack_range_and_executable_permissions_without_leasing() {
        let (image, install_plan, address_space, materialization, launch_plan) = fixture();
        let mut bad_range_source = InitialUserStackLeaseSource::for_initial_stack();
        bad_range_source.override_layout(InitialUserStackLayout::for_test_unchecked(
            USER_ADDRESS_SPACE_END,
            USER_ADDRESS_SPACE_END - 8,
            USER_NULL_GUARD_END - LOADER_PAGE_SIZE,
            USER_NULL_GUARD_END,
            USER_NULL_GUARD_END - 2 * LOADER_PAGE_SIZE,
            USER_NULL_GUARD_END - LOADER_PAGE_SIZE,
            LOADER_PAGE_SIZE,
            INITIAL_USER_STACK_USABLE_PAGES,
            INITIAL_USER_STACK_GUARD_PAGES,
            UserMappingPermissions::USER_DATA,
        ));

        assert_eq!(
            plan_initial_user_stack(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                InitialUserStackRequest::PlanOnly,
                &mut bad_range_source,
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(bad_range_source.outstanding_leases(), 0);

        let mut bad_permission_source = InitialUserStackLeaseSource::for_initial_stack();
        bad_permission_source.override_permissions(UserMappingPermissions::USER_TEXT);
        assert_eq!(
            plan_initial_user_stack(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                InitialUserStackRequest::PlanOnly,
                &mut bad_permission_source,
            ),
            Err(PosixError::AccessDenied)
        );
        assert_eq!(bad_permission_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rejects_image_overlap_without_leasing() {
        let (image, install_plan, address_space, materialization, launch_plan) = fixture();
        let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();

        assert_eq!(
            plan_initial_user_stack(
                image_with_stack_overlap(image),
                install_plan,
                address_space,
                materialization,
                launch_plan,
                InitialUserStackRequest::PlanOnly,
                &mut stack_source,
            ),
            Err(PosixError::AccessDenied)
        );
        assert_eq!(stack_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn capacity_exhaustion_releases_partial_stack_leases() {
        let (image, install_plan, address_space, materialization, launch_plan) = fixture();
        let mut stack_source = InitialUserStackLeaseSource::with_stack_frame_capacity(2);

        assert_eq!(
            plan_initial_user_stack(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                InitialUserStackRequest::PlanOnly,
                &mut stack_source,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(stack_source.outstanding_leases(), 0);
        assert_eq!(stack_source.snapshot().stack_frame_releases, 2);
    }
}
