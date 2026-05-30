//! Target-independent process address-space installation model.
//!
//! This module consumes a validated ProcessImageInstallPlan and produces
//! process-owned address-space metadata with explicit model leases. It does
//! not allocate physical frames, create AArch64 descriptors, switch TTBR/TCR,
//! publish scheduler state, create descriptors, build lower-EL frames, or make
//! an image runnable.

use crate::{
    posix::{PosixError, USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, UserMappingPermissions},
    process_install::{
        MAX_PROCESS_INSTALL_PAGES, PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan,
        ProcessImagePageInstallRecord,
    },
    program_loader::{LOADER_PAGE_SIZE, UserSegmentKind},
    scheduler::ProcessOwnerId,
};

pub(crate) const PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY: &str =
    "phase8-process-address-space-model-v1";
const MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE: usize = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessAddressSpaceId(u64);

impl ProcessAddressSpaceId {
    pub(crate) const fn new(id: u64) -> Option<Self> {
        if id == 0 { None } else { Some(Self(id)) }
    }

    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ModelLeaseToken(u64);

impl ModelLeaseToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn for_test_unchecked(raw: u64) -> Self {
        Self(raw)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessAddressSpaceSideEffects {
    frames_leased: usize,
    table_pages_leased: usize,
    mappings_installed: usize,
    copied_bytes: u64,
    zeroed_bytes: u64,
    rollback_releases: usize,
}

impl ProcessAddressSpaceSideEffects {
    pub(crate) const NONE: Self = Self {
        frames_leased: 0,
        table_pages_leased: 0,
        mappings_installed: 0,
        copied_bytes: 0,
        zeroed_bytes: 0,
        rollback_releases: 0,
    };

    pub(crate) const fn frames_leased(self) -> usize {
        self.frames_leased
    }

    pub(crate) const fn table_pages_leased(self) -> usize {
        self.table_pages_leased
    }

    pub(crate) const fn mappings_installed(self) -> usize {
        self.mappings_installed
    }

    pub(crate) const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub(crate) const fn zeroed_bytes(self) -> u64 {
        self.zeroed_bytes
    }

    pub(crate) const fn rollback_releases(self) -> usize {
        self.rollback_releases
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn for_test_unchecked(
        frames_leased: usize,
        table_pages_leased: usize,
        mappings_installed: usize,
        copied_bytes: u64,
        zeroed_bytes: u64,
        rollback_releases: usize,
    ) -> Self {
        Self {
            frames_leased,
            table_pages_leased,
            mappings_installed,
            copied_bytes,
            zeroed_bytes,
            rollback_releases,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessAddressSpaceLeaseSnapshot {
    pub(crate) root_leased: bool,
    pub(crate) table_pages_leased: usize,
    pub(crate) user_frames_leased: usize,
    pub(crate) mappings_installed: usize,
    pub(crate) root_releases: usize,
    pub(crate) table_page_releases: usize,
    pub(crate) user_frame_releases: usize,
    pub(crate) mapping_releases: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessAddressSpaceLeaseSource {
    root_available: bool,
    table_page_capacity: usize,
    user_frame_capacity: usize,
    mapping_capacity: usize,
    model_buffer_capacity: usize,
    copy_zero_failure_page: Option<usize>,
    next_token: u64,
    root_leased: bool,
    table_pages_leased: usize,
    user_frames_leased: usize,
    mappings_installed: usize,
    root_releases: usize,
    table_page_releases: usize,
    user_frame_releases: usize,
    mapping_releases: usize,
}

impl ProcessAddressSpaceLeaseSource {
    pub(crate) const fn with_limits(
        table_page_capacity: usize,
        user_frame_capacity: usize,
        mapping_capacity: usize,
        model_buffer_capacity: usize,
    ) -> Self {
        Self {
            root_available: true,
            table_page_capacity,
            user_frame_capacity,
            mapping_capacity,
            model_buffer_capacity,
            copy_zero_failure_page: None,
            next_token: 1,
            root_leased: false,
            table_pages_leased: 0,
            user_frames_leased: 0,
            mappings_installed: 0,
            root_releases: 0,
            table_page_releases: 0,
            user_frame_releases: 0,
            mapping_releases: 0,
        }
    }

    pub(crate) const fn for_plan(plan: ProcessImageInstallPlan) -> Self {
        Self::with_limits(
            MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE,
            plan.page_count(),
            plan.page_count(),
            plan.page_count(),
        )
    }

    pub(crate) fn deny_root(&mut self) {
        self.root_available = false;
    }

    pub(crate) fn fail_copy_zero_at_page(&mut self, page_index: usize) {
        self.copy_zero_failure_page = Some(page_index);
    }

    pub(crate) const fn snapshot(self) -> ProcessAddressSpaceLeaseSnapshot {
        ProcessAddressSpaceLeaseSnapshot {
            root_leased: self.root_leased,
            table_pages_leased: self.table_pages_leased,
            user_frames_leased: self.user_frames_leased,
            mappings_installed: self.mappings_installed,
            root_releases: self.root_releases,
            table_page_releases: self.table_page_releases,
            user_frame_releases: self.user_frame_releases,
            mapping_releases: self.mapping_releases,
        }
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        (if self.root_leased { 1 } else { 0 })
            + self.table_pages_leased
            + self.user_frames_leased
            + self.mappings_installed
    }

    fn next_token(&mut self) -> ModelLeaseToken {
        let token = ModelLeaseToken(self.next_token);
        self.next_token += 1;
        token
    }

    fn lease_root(&mut self) -> Result<ModelLeaseToken, PosixError> {
        if !self.root_available || self.root_leased {
            return Err(PosixError::NoMemory);
        }
        let token = self.next_token();
        self.root_leased = true;
        Ok(token)
    }

    fn lease_table_page(&mut self) -> Result<ModelLeaseToken, PosixError> {
        if self.table_pages_leased == self.table_page_capacity {
            return Err(PosixError::NoMemory);
        }
        let token = self.next_token();
        self.table_pages_leased += 1;
        Ok(token)
    }

    fn lease_user_frame(&mut self) -> Result<ModelLeaseToken, PosixError> {
        if self.user_frames_leased == self.user_frame_capacity {
            return Err(PosixError::NoMemory);
        }
        let token = self.next_token();
        self.user_frames_leased += 1;
        Ok(token)
    }

    fn install_mapping_slot(&mut self) -> Result<(), PosixError> {
        if self.mappings_installed == self.mapping_capacity {
            return Err(PosixError::NoMemory);
        }
        self.mappings_installed += 1;
        Ok(())
    }

    fn represent_copy_zero(&self, page_index: usize) -> Result<(), PosixError> {
        if self.model_buffer_capacity <= page_index
            || self.copy_zero_failure_page == Some(page_index)
        {
            return Err(PosixError::InvalidArgument);
        }
        Ok(())
    }

    fn release_mapping_slot(&mut self) {
        if self.mappings_installed != 0 {
            self.mappings_installed -= 1;
            self.mapping_releases += 1;
        }
    }

    fn release_user_frame(&mut self, lease: &mut UserFrameLease) {
        if !lease.released {
            lease.released = true;
            if self.user_frames_leased != 0 {
                self.user_frames_leased -= 1;
            }
            self.user_frame_releases += 1;
        }
    }

    fn release_table_page(&mut self, lease: &mut TablePageLease) {
        if !lease.released {
            lease.released = true;
            if self.table_pages_leased != 0 {
                self.table_pages_leased -= 1;
            }
            self.table_page_releases += 1;
        }
    }

    fn release_root(&mut self, root: &mut PageTableRootLease) {
        if !root.released {
            root.released = true;
            self.root_leased = false;
            self.root_releases += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PageTableRootLease {
    token: ModelLeaseToken,
    released: bool,
}

impl PageTableRootLease {
    pub(crate) const fn token(self) -> ModelLeaseToken {
        self.token
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn for_test_unchecked(token: ModelLeaseToken, released: bool) -> Self {
        Self { token, released }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TablePageLease {
    token: ModelLeaseToken,
    released: bool,
}

impl TablePageLease {
    pub(crate) const fn token(self) -> ModelLeaseToken {
        self.token
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn for_test_unchecked(token: ModelLeaseToken, released: bool) -> Self {
        Self { token, released }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UserFrameLease {
    token: ModelLeaseToken,
    virtual_page: u64,
    kind: UserSegmentKind,
    permissions: UserMappingPermissions,
    zeroed_before_copy: bool,
    copied_bytes: u64,
    zeroed_bytes: u64,
    source_page_ordinal: usize,
    released: bool,
}

impl UserFrameLease {
    pub(crate) const fn token(self) -> ModelLeaseToken {
        self.token
    }

    pub(crate) const fn virtual_page(self) -> u64 {
        self.virtual_page
    }

    pub(crate) const fn kind(self) -> UserSegmentKind {
        self.kind
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

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn for_test_unchecked(
        token: ModelLeaseToken,
        virtual_page: u64,
        kind: UserSegmentKind,
        permissions: UserMappingPermissions,
        zeroed_before_copy: bool,
        copied_bytes: u64,
        zeroed_bytes: u64,
        source_page_ordinal: usize,
        released: bool,
    ) -> Self {
        Self {
            token,
            virtual_page,
            kind,
            permissions,
            zeroed_before_copy,
            copied_bytes,
            zeroed_bytes,
            source_page_ordinal,
            released,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessUserMapping {
    virtual_start: u64,
    virtual_end: u64,
    kind: UserSegmentKind,
    permissions: UserMappingPermissions,
    copy_page_offset: u64,
    copy_file_offset: usize,
    copy_len: u64,
    zero_len: u64,
    source_page_ordinal: usize,
    el0_user_access: bool,
    write_xor_execute: bool,
    normal_memory_intent: bool,
    kernel_device_denied: bool,
}

impl ProcessUserMapping {
    pub(crate) const fn virtual_start(self) -> u64 {
        self.virtual_start
    }

    pub(crate) const fn virtual_end(self) -> u64 {
        self.virtual_end
    }

    pub(crate) const fn kind(self) -> UserSegmentKind {
        self.kind
    }

    pub(crate) const fn permissions(self) -> UserMappingPermissions {
        self.permissions
    }

    pub(crate) const fn copy_page_offset(self) -> u64 {
        self.copy_page_offset
    }

    pub(crate) const fn copy_file_offset(self) -> usize {
        self.copy_file_offset
    }

    pub(crate) const fn copy_len(self) -> u64 {
        self.copy_len
    }

    pub(crate) const fn zero_len(self) -> u64 {
        self.zero_len
    }

    pub(crate) const fn source_page_ordinal(self) -> usize {
        self.source_page_ordinal
    }

    pub(crate) const fn el0_user_access(self) -> bool {
        self.el0_user_access
    }

    pub(crate) const fn write_xor_execute(self) -> bool {
        self.write_xor_execute
    }

    pub(crate) const fn normal_memory_intent(self) -> bool {
        self.normal_memory_intent
    }

    pub(crate) const fn kernel_device_denied(self) -> bool {
        self.kernel_device_denied
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn for_test_unchecked(
        virtual_start: u64,
        virtual_end: u64,
        kind: UserSegmentKind,
        permissions: UserMappingPermissions,
        copy_page_offset: u64,
        copy_file_offset: usize,
        copy_len: u64,
        zero_len: u64,
        source_page_ordinal: usize,
        el0_user_access: bool,
        write_xor_execute: bool,
        normal_memory_intent: bool,
        kernel_device_denied: bool,
    ) -> Self {
        Self {
            virtual_start,
            virtual_end,
            kind,
            permissions,
            copy_page_offset,
            copy_file_offset,
            copy_len,
            zero_len,
            source_page_ordinal,
            el0_user_access,
            write_xor_execute,
            normal_memory_intent,
            kernel_device_denied,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessAddressSpaceTeardownReport {
    mappings_released: usize,
    user_frame_releases: usize,
    table_page_releases: usize,
    root_released: bool,
    already_destroyed: bool,
}

impl ProcessAddressSpaceTeardownReport {
    pub(crate) const fn mappings_released(self) -> usize {
        self.mappings_released
    }

    pub(crate) const fn user_frame_releases(self) -> usize {
        self.user_frame_releases
    }

    pub(crate) const fn table_page_releases(self) -> usize {
        self.table_page_releases
    }

    pub(crate) const fn root_released(self) -> bool {
        self.root_released
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessAddressSpace {
    id: ProcessAddressSpaceId,
    owner: Option<ProcessOwnerId>,
    boundary_identity: &'static str,
    root: PageTableRootLease,
    table_leases: [Option<TablePageLease>; MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE],
    table_lease_count: usize,
    user_frame_leases: [Option<UserFrameLease>; MAX_PROCESS_INSTALL_PAGES],
    user_frame_lease_count: usize,
    mappings: [Option<ProcessUserMapping>; MAX_PROCESS_INSTALL_PAGES],
    mapping_count: usize,
    side_effects: ProcessAddressSpaceSideEffects,
    published: bool,
    destroyed: bool,
}

impl ProcessAddressSpace {
    pub(crate) const fn id(self) -> ProcessAddressSpaceId {
        self.id
    }

    pub(crate) const fn owner(self) -> Option<ProcessOwnerId> {
        self.owner
    }

    pub(crate) const fn boundary_identity(self) -> &'static str {
        self.boundary_identity
    }

    pub(crate) const fn published(self) -> bool {
        self.published
    }

    pub(crate) const fn destroyed(self) -> bool {
        self.destroyed
    }

    pub(crate) const fn root(self) -> PageTableRootLease {
        self.root
    }

    pub(crate) const fn table_lease_count(self) -> usize {
        self.table_lease_count
    }

    pub(crate) const fn table_lease(self, index: usize) -> Option<TablePageLease> {
        if index >= MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE {
            None
        } else {
            self.table_leases[index]
        }
    }

    pub(crate) const fn user_frame_lease_count(self) -> usize {
        self.user_frame_lease_count
    }

    pub(crate) const fn user_frame_lease(self, index: usize) -> Option<UserFrameLease> {
        if index >= MAX_PROCESS_INSTALL_PAGES {
            None
        } else {
            self.user_frame_leases[index]
        }
    }

    pub(crate) const fn mapping_count(self) -> usize {
        self.mapping_count
    }

    pub(crate) const fn mapping(self, index: usize) -> Option<ProcessUserMapping> {
        if index >= MAX_PROCESS_INSTALL_PAGES {
            None
        } else {
            self.mappings[index]
        }
    }

    pub(crate) const fn side_effects(self) -> ProcessAddressSpaceSideEffects {
        self.side_effects
    }

    pub(crate) fn destroy(
        &mut self,
        lease_source: &mut ProcessAddressSpaceLeaseSource,
    ) -> ProcessAddressSpaceTeardownReport {
        if self.destroyed {
            return ProcessAddressSpaceTeardownReport {
                mappings_released: 0,
                user_frame_releases: 0,
                table_page_releases: 0,
                root_released: false,
                already_destroyed: true,
            };
        }

        let mut mappings_released = 0;
        while self.mapping_count != 0 {
            self.mapping_count -= 1;
            self.mappings[self.mapping_count] = None;
            lease_source.release_mapping_slot();
            mappings_released += 1;
        }

        let mut user_frame_releases = 0;
        while self.user_frame_lease_count != 0 {
            self.user_frame_lease_count -= 1;
            if let Some(mut lease) = self.user_frame_leases[self.user_frame_lease_count] {
                lease_source.release_user_frame(&mut lease);
                self.user_frame_leases[self.user_frame_lease_count] = Some(lease);
                user_frame_releases += 1;
            }
        }

        let mut table_page_releases = 0;
        while self.table_lease_count != 0 {
            self.table_lease_count -= 1;
            if let Some(mut lease) = self.table_leases[self.table_lease_count] {
                lease_source.release_table_page(&mut lease);
                self.table_leases[self.table_lease_count] = Some(lease);
                table_page_releases += 1;
            }
        }

        lease_source.release_root(&mut self.root);
        self.published = false;
        self.destroyed = true;

        ProcessAddressSpaceTeardownReport {
            mappings_released,
            user_frame_releases,
            table_page_releases,
            root_released: true,
            already_destroyed: false,
        }
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn for_test_unchecked(
        id: ProcessAddressSpaceId,
        owner: Option<ProcessOwnerId>,
        boundary_identity: &'static str,
        root: PageTableRootLease,
        table_leases: [Option<TablePageLease>; MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE],
        table_lease_count: usize,
        user_frame_leases: [Option<UserFrameLease>; MAX_PROCESS_INSTALL_PAGES],
        user_frame_lease_count: usize,
        mappings: [Option<ProcessUserMapping>; MAX_PROCESS_INSTALL_PAGES],
        mapping_count: usize,
        side_effects: ProcessAddressSpaceSideEffects,
        published: bool,
        destroyed: bool,
    ) -> Self {
        Self {
            id,
            owner,
            boundary_identity,
            root,
            table_leases,
            table_lease_count,
            user_frame_leases,
            user_frame_lease_count,
            mappings,
            mapping_count,
            side_effects,
            published,
            destroyed,
        }
    }
}

pub(crate) fn install_process_address_space(
    plan: ProcessImageInstallPlan,
    id: ProcessAddressSpaceId,
    owner: Option<ProcessOwnerId>,
    lease_source: &mut ProcessAddressSpaceLeaseSource,
) -> Result<ProcessAddressSpace, PosixError> {
    validate_plan(plan)?;

    let root = PageTableRootLease {
        token: lease_source.lease_root()?,
        released: false,
    };

    let mut partial = PartialAddressSpaceInstall {
        root,
        table_leases: [None; MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE],
        table_lease_count: 0,
        user_frame_leases: [None; MAX_PROCESS_INSTALL_PAGES],
        user_frame_lease_count: 0,
        mappings: [None; MAX_PROCESS_INSTALL_PAGES],
        mapping_count: 0,
        side_effects: ProcessAddressSpaceSideEffects::NONE,
    };

    let install_result = install_validated_plan(plan, lease_source, &mut partial);
    match install_result {
        Ok(()) => Ok(ProcessAddressSpace {
            id,
            owner,
            boundary_identity: PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
            root: partial.root,
            table_leases: partial.table_leases,
            table_lease_count: partial.table_lease_count,
            user_frame_leases: partial.user_frame_leases,
            user_frame_lease_count: partial.user_frame_lease_count,
            mappings: partial.mappings,
            mapping_count: partial.mapping_count,
            side_effects: partial.side_effects,
            published: true,
            destroyed: false,
        }),
        Err(error) => {
            rollback_partial(lease_source, &mut partial);
            Err(error)
        }
    }
}

struct PartialAddressSpaceInstall {
    root: PageTableRootLease,
    table_leases: [Option<TablePageLease>; MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE],
    table_lease_count: usize,
    user_frame_leases: [Option<UserFrameLease>; MAX_PROCESS_INSTALL_PAGES],
    user_frame_lease_count: usize,
    mappings: [Option<ProcessUserMapping>; MAX_PROCESS_INSTALL_PAGES],
    mapping_count: usize,
    side_effects: ProcessAddressSpaceSideEffects,
}

fn install_validated_plan(
    plan: ProcessImageInstallPlan,
    lease_source: &mut ProcessAddressSpaceLeaseSource,
    partial: &mut PartialAddressSpaceInstall,
) -> Result<(), PosixError> {
    while partial.table_lease_count < MODEL_TABLE_PAGE_LEASES_PER_ADDRESS_SPACE {
        partial.table_leases[partial.table_lease_count] = Some(TablePageLease {
            token: lease_source.lease_table_page()?,
            released: false,
        });
        partial.table_lease_count += 1;
        partial.side_effects.table_pages_leased += 1;
    }

    let mut page_index = 0;
    while page_index < plan.page_count() {
        let page = plan.page(page_index).ok_or(PosixError::InvalidArgument)?;
        let token = lease_source.lease_user_frame()?;
        let user_lease = UserFrameLease {
            token,
            virtual_page: page.virtual_start(),
            kind: page.kind(),
            permissions: page.permissions(),
            zeroed_before_copy: true,
            copied_bytes: page.copy_len(),
            zeroed_bytes: page.zero_len(),
            source_page_ordinal: page.index(),
            released: false,
        };
        partial.user_frame_leases[partial.user_frame_lease_count] = Some(user_lease);
        partial.user_frame_lease_count += 1;
        partial.side_effects.frames_leased += 1;
        partial.side_effects.copied_bytes += page.copy_len();
        partial.side_effects.zeroed_bytes += page.zero_len();
        lease_source.represent_copy_zero(page_index)?;

        lease_source.install_mapping_slot()?;
        partial.mappings[partial.mapping_count] = Some(ProcessUserMapping {
            virtual_start: page.virtual_start(),
            virtual_end: page.virtual_end(),
            kind: page.kind(),
            permissions: page.permissions(),
            copy_page_offset: page.copy_page_offset(),
            copy_file_offset: page.copy_file_offset(),
            copy_len: page.copy_len(),
            zero_len: page.zero_len(),
            source_page_ordinal: page.index(),
            el0_user_access: true,
            write_xor_execute: is_write_xor_execute(page.permissions()),
            normal_memory_intent: true,
            kernel_device_denied: true,
        });
        partial.mapping_count += 1;
        partial.side_effects.mappings_installed += 1;
        page_index += 1;
    }

    Ok(())
}

fn rollback_partial(
    lease_source: &mut ProcessAddressSpaceLeaseSource,
    partial: &mut PartialAddressSpaceInstall,
) {
    while partial.mapping_count != 0 {
        partial.mapping_count -= 1;
        partial.mappings[partial.mapping_count] = None;
        lease_source.release_mapping_slot();
        partial.side_effects.rollback_releases += 1;
    }

    while partial.user_frame_lease_count != 0 {
        partial.user_frame_lease_count -= 1;
        if let Some(mut lease) = partial.user_frame_leases[partial.user_frame_lease_count] {
            lease_source.release_user_frame(&mut lease);
            partial.user_frame_leases[partial.user_frame_lease_count] = Some(lease);
            partial.side_effects.rollback_releases += 1;
        }
    }

    while partial.table_lease_count != 0 {
        partial.table_lease_count -= 1;
        if let Some(mut lease) = partial.table_leases[partial.table_lease_count] {
            lease_source.release_table_page(&mut lease);
            partial.table_leases[partial.table_lease_count] = Some(lease);
            partial.side_effects.rollback_releases += 1;
        }
    }

    lease_source.release_root(&mut partial.root);
    partial.side_effects.rollback_releases += 1;
}

fn validate_plan(plan: ProcessImageInstallPlan) -> Result<(), PosixError> {
    if plan.install_boundary_identity() != PROCESS_INSTALL_BOUNDARY_IDENTITY
        || plan.page_count() == 0
        || plan.page_count() > MAX_PROCESS_INSTALL_PAGES
        || plan.side_effects().frames_allocated() != 0
        || plan.side_effects().mappings_installed() != 0
        || plan.side_effects().process_created()
        || plan.side_effects().descriptors_mutated()
        || plan.side_effects().lower_el_frame()
        || plan.side_effects().runnable()
        || !plan.lower_el_launch_blocked()
    {
        return Err(PosixError::InvalidArgument);
    }

    let mut entry_in_text = false;
    let mut previous_end = 0;
    let mut page_index = 0;
    while page_index < plan.page_count() {
        let page = plan.page(page_index).ok_or(PosixError::InvalidArgument)?;
        validate_page_record(page, page_index, previous_end)?;
        if page.kind() == UserSegmentKind::UserText
            && page.virtual_start() <= plan.entry()
            && plan.entry() < page.virtual_end()
        {
            entry_in_text = true;
        }
        previous_end = page.virtual_end();
        page_index += 1;
    }

    if !entry_in_text {
        return Err(PosixError::NotExecutable);
    }
    Ok(())
}

fn validate_page_record(
    page: ProcessImagePageInstallRecord,
    expected_index: usize,
    previous_end: u64,
) -> Result<(), PosixError> {
    if page.index() != expected_index {
        return Err(PosixError::InvalidArgument);
    }

    let expected_end = page
        .virtual_start()
        .checked_add(LOADER_PAGE_SIZE)
        .ok_or(PosixError::InvalidArgument)?;
    if page.virtual_start() >= page.virtual_end()
        || page.virtual_end() != expected_end
        || page.virtual_start() % LOADER_PAGE_SIZE != 0
    {
        return Err(PosixError::InvalidArgument);
    }

    if page.virtual_start() < USER_NULL_GUARD_END || page.virtual_end() > USER_ADDRESS_SPACE_END {
        return Err(PosixError::AccessDenied);
    }
    if expected_index != 0 && page.virtual_start() < previous_end {
        return Err(PosixError::AccessDenied);
    }

    match page.kind() {
        UserSegmentKind::UserText if page.permissions() == UserMappingPermissions::USER_TEXT => {}
        UserSegmentKind::UserData if page.permissions() == UserMappingPermissions::USER_DATA => {}
        _ => return Err(PosixError::AccessDenied),
    }
    if !is_write_xor_execute(page.permissions()) {
        return Err(PosixError::AccessDenied);
    }

    let copy_end = page
        .copy_page_offset()
        .checked_add(page.copy_len())
        .ok_or(PosixError::InvalidArgument)?;
    if copy_end > LOADER_PAGE_SIZE {
        return Err(PosixError::InvalidArgument);
    }

    let mut zero_index = 0;
    let mut zero_len = 0u64;
    let mut previous_zero_end = 0u64;
    while zero_index < page.zero_range_count() {
        let range = page
            .zero_range(zero_index)
            .ok_or(PosixError::InvalidArgument)?;
        let range_end = range
            .offset()
            .checked_add(range.len())
            .ok_or(PosixError::InvalidArgument)?;
        if range.len() == 0 || range_end > LOADER_PAGE_SIZE || range.offset() < previous_zero_end {
            return Err(PosixError::InvalidArgument);
        }
        zero_len = zero_len
            .checked_add(range.len())
            .ok_or(PosixError::InvalidArgument)?;
        previous_zero_end = range_end;
        zero_index += 1;
    }
    if page.zero_range_count() != 0 && page.zero_range(page.zero_range_count()).is_some() {
        return Err(PosixError::InvalidArgument);
    }
    if zero_len != page.zero_len() {
        return Err(PosixError::InvalidArgument);
    }

    Ok(())
}

fn is_write_xor_execute(permissions: UserMappingPermissions) -> bool {
    !(permissions.contains(UserMappingPermissions::WRITE)
        && permissions.contains(UserMappingPermissions::EXECUTE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initramfs::{PHASE8_INIT_BYTES, PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
        process_install::{
            MAX_ZERO_RANGES_PER_PAGE, PageByteRange, ProcessImagePageInstallRecord,
            ProcessInstallAction, ProcessInstallSideEffects, plan_process_image_install,
        },
        program_loader::{PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, plan_phase8_init_image},
    };

    fn install_fixture() -> ProcessImageInstallPlan {
        let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
            .expect("program image plan");
        plan_process_image_install(image).expect("process install plan")
    }

    fn address_space_id() -> ProcessAddressSpaceId {
        ProcessAddressSpaceId::new(0x8000_0001).expect("address space id")
    }

    fn owner_id() -> ProcessOwnerId {
        ProcessOwnerId::new(0x55).expect("owner id")
    }

    fn install_with_limits(
        plan: ProcessImageInstallPlan,
        lease_source: &mut ProcessAddressSpaceLeaseSource,
    ) -> Result<ProcessAddressSpace, PosixError> {
        install_process_address_space(plan, address_space_id(), Some(owner_id()), lease_source)
    }

    #[test_case]
    fn installs_process_address_space_with_preserved_permissions_and_leases() {
        let plan = install_fixture();
        let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(plan);

        let address_space =
            install_with_limits(plan, &mut lease_source).expect("process address space");

        assert_eq!(address_space.id(), address_space_id());
        assert_eq!(address_space.id().raw(), 0x8000_0001);
        assert_eq!(address_space.owner(), Some(owner_id()));
        assert_eq!(
            address_space.boundary_identity(),
            PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        );
        assert!(address_space.published());
        assert!(!address_space.destroyed());
        assert!(!address_space.root().released());
        assert_ne!(address_space.root().token().raw(), 0);
        assert_eq!(address_space.table_lease_count(), 1);
        assert!(
            !address_space
                .table_lease(0)
                .expect("table lease")
                .released()
        );
        assert_ne!(
            address_space
                .table_lease(0)
                .expect("table lease")
                .token()
                .raw(),
            0
        );
        assert_eq!(address_space.user_frame_lease_count(), plan.page_count());
        assert_eq!(address_space.mapping_count(), plan.page_count());
        assert_eq!(
            address_space.side_effects().frames_leased(),
            plan.page_count()
        );
        assert_eq!(address_space.side_effects().table_pages_leased(), 1);
        assert_eq!(
            address_space.side_effects().mappings_installed(),
            plan.page_count()
        );
        assert_eq!(address_space.side_effects().copied_bytes(), 8);
        assert_eq!(address_space.side_effects().zeroed_bytes(), 0x2ff8);
        assert_eq!(address_space.side_effects().rollback_releases(), 0);

        let text = address_space.mapping(0).expect("text mapping");
        assert_eq!(text.virtual_start(), 0x0000_0000_0001_0000);
        assert_eq!(text.virtual_end(), 0x0000_0000_0001_1000);
        assert_eq!(text.kind(), UserSegmentKind::UserText);
        assert_eq!(text.permissions(), UserMappingPermissions::USER_TEXT);
        assert!(text.el0_user_access());
        assert!(text.write_xor_execute());
        assert!(text.normal_memory_intent());
        assert!(text.kernel_device_denied());
        assert_eq!(text.copy_page_offset(), 0x100);
        assert_eq!(text.copy_file_offset(), 0x100);
        assert_eq!(text.copy_len(), 4);
        assert_eq!(text.zero_len(), 0xffc);
        assert_eq!(text.source_page_ordinal(), 0);

        let data0 = address_space.mapping(1).expect("data mapping");
        assert_eq!(data0.virtual_start(), 0x0000_0000_0002_0000);
        assert_eq!(data0.virtual_end(), 0x0000_0000_0002_1000);
        assert_eq!(data0.kind(), UserSegmentKind::UserData);
        assert_eq!(data0.permissions(), UserMappingPermissions::USER_DATA);
        assert!(data0.write_xor_execute());
        assert_eq!(data0.copy_page_offset(), 0x200);
        assert_eq!(data0.copy_file_offset(), 0x200);
        assert_eq!(data0.copy_len(), 4);
        assert_eq!(data0.zero_len(), 0xffc);
        assert_eq!(data0.source_page_ordinal(), 1);

        let data1 = address_space.user_frame_lease(2).expect("data frame");
        assert_ne!(data1.token().raw(), 0);
        assert_eq!(data1.virtual_page(), 0x0000_0000_0002_1000);
        assert_eq!(data1.kind(), UserSegmentKind::UserData);
        assert_eq!(data1.permissions(), UserMappingPermissions::USER_DATA);
        assert!(data1.zeroed_before_copy());
        assert_eq!(data1.copied_bytes(), 0);
        assert_eq!(data1.zeroed_bytes(), LOADER_PAGE_SIZE);
        assert_eq!(data1.source_page_ordinal(), 2);
        assert!(!data1.released());

        let snapshot = lease_source.snapshot();
        assert!(snapshot.root_leased);
        assert_eq!(snapshot.table_pages_leased, 1);
        assert_eq!(snapshot.user_frames_leased, plan.page_count());
        assert_eq!(snapshot.mappings_installed, plan.page_count());
        assert_eq!(
            lease_source.outstanding_leases(),
            1 + 1 + plan.page_count() + plan.page_count()
        );
    }

    #[test_case]
    fn teardown_releases_owned_leases_in_order_and_is_idempotent() {
        let plan = install_fixture();
        let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(plan);
        let mut address_space =
            install_with_limits(plan, &mut lease_source).expect("process address space");

        let first = address_space.destroy(&mut lease_source);
        assert_eq!(first.mappings_released(), plan.page_count());
        assert_eq!(first.user_frame_releases(), plan.page_count());
        assert_eq!(first.table_page_releases(), 1);
        assert!(first.root_released());
        assert!(!first.already_destroyed());
        assert!(address_space.destroyed());
        assert!(!address_space.published());
        assert_eq!(lease_source.outstanding_leases(), 0);

        let second = address_space.destroy(&mut lease_source);
        assert_eq!(second.mappings_released(), 0);
        assert_eq!(second.user_frame_releases(), 0);
        assert_eq!(second.table_page_releases(), 0);
        assert!(!second.root_released());
        assert!(second.already_destroyed());

        let snapshot = lease_source.snapshot();
        assert_eq!(snapshot.mapping_releases, plan.page_count());
        assert_eq!(snapshot.user_frame_releases, plan.page_count());
        assert_eq!(snapshot.table_page_releases, 1);
        assert_eq!(snapshot.root_releases, 1);
    }

    #[test_case]
    fn rejects_bad_install_plan_before_leasing() {
        let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
        pages[0] = install_fixture().page(0);
        let bad_plan = ProcessImageInstallPlan::for_test_unchecked(
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            PROCESS_INSTALL_BOUNDARY_IDENTITY,
            PHASE8_INIT_PATH,
            0x3892_eed2_2390_0c65,
            0x0000_0000_0001_0100,
            LOADER_PAGE_SIZE,
            2,
            pages,
            ProcessInstallSideEffects::NONE,
            true,
        );
        let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(bad_plan);

        assert_eq!(
            install_with_limits(bad_plan, &mut lease_source),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);
        assert_eq!(lease_source.snapshot().root_releases, 0);
    }

    #[test_case]
    fn rejects_null_guard_or_kernel_split_without_leasing() {
        let bad_page = ProcessImagePageInstallRecord::for_test_unchecked(
            0,
            0,
            USER_NULL_GUARD_END - LOADER_PAGE_SIZE,
            USER_NULL_GUARD_END,
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_TEXT,
            0,
            0,
            4,
            [Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4));
                MAX_ZERO_RANGES_PER_PAGE],
            1,
            LOADER_PAGE_SIZE - 4,
            ProcessInstallAction::AllocateCopyZeroMap,
        );
        let bad_plan = plan_from_page(bad_page, USER_NULL_GUARD_END - LOADER_PAGE_SIZE);
        let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(bad_plan);

        assert_eq!(
            install_with_limits(bad_plan, &mut lease_source),
            Err(PosixError::AccessDenied)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);

        let kernel_page = ProcessImagePageInstallRecord::for_test_unchecked(
            0,
            0,
            USER_ADDRESS_SPACE_END,
            USER_ADDRESS_SPACE_END + LOADER_PAGE_SIZE,
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_TEXT,
            0,
            0,
            4,
            [Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4));
                MAX_ZERO_RANGES_PER_PAGE],
            1,
            LOADER_PAGE_SIZE - 4,
            ProcessInstallAction::AllocateCopyZeroMap,
        );
        let kernel_plan = plan_from_page(kernel_page, USER_ADDRESS_SPACE_END);
        let mut kernel_source = ProcessAddressSpaceLeaseSource::for_plan(kernel_plan);
        assert_eq!(
            install_with_limits(kernel_plan, &mut kernel_source),
            Err(PosixError::AccessDenied)
        );
        assert_eq!(kernel_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rejects_overlap_and_permission_widening_without_leasing() {
        let fixture = install_fixture();
        let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
        pages[0] = fixture.page(0);
        pages[1] = Some(ProcessImagePageInstallRecord::for_test_unchecked(
            1,
            1,
            fixture.page(0).expect("text").virtual_start(),
            fixture.page(0).expect("text").virtual_end(),
            UserSegmentKind::UserData,
            UserMappingPermissions::USER_DATA,
            0,
            0,
            0,
            [None; MAX_ZERO_RANGES_PER_PAGE],
            0,
            0,
            ProcessInstallAction::AllocateCopyZeroMap,
        ));
        let overlap = ProcessImageInstallPlan::for_test_unchecked(
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            PROCESS_INSTALL_BOUNDARY_IDENTITY,
            PHASE8_INIT_BYTES,
            0x3892_eed2_2390_0c65,
            fixture.page(0).expect("text").virtual_start(),
            LOADER_PAGE_SIZE * 2,
            2,
            pages,
            ProcessInstallSideEffects::NONE,
            true,
        );
        let mut overlap_source = ProcessAddressSpaceLeaseSource::for_plan(overlap);
        assert_eq!(
            install_with_limits(overlap, &mut overlap_source),
            Err(PosixError::AccessDenied)
        );
        assert_eq!(overlap_source.outstanding_leases(), 0);

        let mut widened = fixture.page(0).expect("text page");
        widened = ProcessImagePageInstallRecord::for_test_unchecked(
            widened.index(),
            widened.segment_index(),
            widened.virtual_start(),
            widened.virtual_end(),
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_DATA,
            widened.copy_page_offset(),
            widened.copy_file_offset(),
            widened.copy_len(),
            [widened.zero_range(0), widened.zero_range(1)],
            widened.zero_range_count(),
            widened.zero_len(),
            widened.action(),
        );
        let widened_plan = plan_from_page(widened, 0x0000_0000_0001_0100);
        let mut widened_source = ProcessAddressSpaceLeaseSource::for_plan(widened_plan);
        assert_eq!(
            install_with_limits(widened_plan, &mut widened_source),
            Err(PosixError::AccessDenied)
        );
        assert_eq!(widened_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rolls_back_root_table_user_frame_mapping_and_copy_zero_failures() {
        let plan = install_fixture();

        let mut no_root = ProcessAddressSpaceLeaseSource::for_plan(plan);
        no_root.deny_root();
        assert_eq!(
            install_with_limits(plan, &mut no_root),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_root.outstanding_leases(), 0);
        assert_eq!(no_root.snapshot().root_releases, 0);

        let mut no_table = ProcessAddressSpaceLeaseSource::with_limits(
            0,
            plan.page_count(),
            plan.page_count(),
            plan.page_count(),
        );
        assert_eq!(
            install_with_limits(plan, &mut no_table),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_table.outstanding_leases(), 0);
        assert_eq!(no_table.snapshot().root_releases, 1);

        let mut no_user =
            ProcessAddressSpaceLeaseSource::with_limits(1, 1, plan.page_count(), plan.page_count());
        assert_eq!(
            install_with_limits(plan, &mut no_user),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_user.outstanding_leases(), 0);
        let no_user_snapshot = no_user.snapshot();
        assert_eq!(no_user_snapshot.user_frame_releases, 1);
        assert_eq!(no_user_snapshot.table_page_releases, 1);
        assert_eq!(no_user_snapshot.root_releases, 1);

        let mut no_mapping =
            ProcessAddressSpaceLeaseSource::with_limits(1, plan.page_count(), 1, plan.page_count());
        assert_eq!(
            install_with_limits(plan, &mut no_mapping),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_mapping.outstanding_leases(), 0);
        let no_mapping_snapshot = no_mapping.snapshot();
        assert_eq!(no_mapping_snapshot.mapping_releases, 1);
        assert_eq!(no_mapping_snapshot.user_frame_releases, 2);
        assert_eq!(no_mapping_snapshot.table_page_releases, 1);
        assert_eq!(no_mapping_snapshot.root_releases, 1);

        let mut copy_zero_failure = ProcessAddressSpaceLeaseSource::for_plan(plan);
        copy_zero_failure.fail_copy_zero_at_page(1);
        assert_eq!(
            install_with_limits(plan, &mut copy_zero_failure),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(copy_zero_failure.outstanding_leases(), 0);
        let copy_zero_snapshot = copy_zero_failure.snapshot();
        assert_eq!(copy_zero_snapshot.mapping_releases, 1);
        assert_eq!(copy_zero_snapshot.user_frame_releases, 2);
        assert_eq!(copy_zero_snapshot.table_page_releases, 1);
        assert_eq!(copy_zero_snapshot.root_releases, 1);
    }

    fn plan_from_page(page: ProcessImagePageInstallRecord, entry: u64) -> ProcessImageInstallPlan {
        let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
        pages[0] = Some(page);
        ProcessImageInstallPlan::for_test_unchecked(
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            PROCESS_INSTALL_BOUNDARY_IDENTITY,
            PHASE8_INIT_PATH,
            0x3892_eed2_2390_0c65,
            entry,
            LOADER_PAGE_SIZE,
            1,
            pages,
            ProcessInstallSideEffects::NONE,
            true,
        )
    }
}
