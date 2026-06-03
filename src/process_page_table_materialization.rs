//! Non-activating process page-table materialization evidence.
//!
//! This module consumes the accepted Phase 8 loader, process-install, and
//! process address-space records and produces owned descriptor/user-frame
//! evidence only. It does not write TTBR/TCR/MAIR/SCTLR, invalidate live TLBs,
//! publish scheduler state, create lower-EL frames, or make a process runnable.

use crate::{
    memory_map::{
        EARLY_PAGE_SIZE, EARLY_TRANSLATION_NORMAL_ATTR_INDEX, STAGE1_DESC_AF,
        STAGE1_DESC_ATTR_INDEX_SHIFT, STAGE1_DESC_PXN, STAGE1_DESC_SH_INNER, STAGE1_DESC_TABLE,
        STAGE1_DESC_UXN, STAGE1_DESC_VALID, STAGE1_TABLE_ADDR_MASK,
    },
    posix::{PosixError, USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, UserMappingPermissions},
    process_address_space::{
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace, ProcessUserMapping,
        UserFrameLease,
    },
    process_install::{
        MAX_PROCESS_INSTALL_PAGES, PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan,
    },
    program_loader::{
        LOADER_PAGE_SIZE, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan, UserSegmentKind,
    },
};

pub(crate) const PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY: &str =
    "phase8-process-page-table-materialization-v1";
pub(crate) const PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY: &str =
    "activation-blocked-no-kernel-half";

const MATERIALIZATION_TABLE_PAGE_COUNT: usize = 3;
const MATERIALIZATION_PAGE_ENTRIES: usize = 512;
const DEFAULT_MATERIALIZATION_FRAME_BASE: u64 = 0x0000_0000_4000_0000;
const STAGE1_DESC_AP_EL0_RW: u64 = 0b01 << 6;
const STAGE1_DESC_AP_EL0_RO: u64 = 0b11 << 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProcessMaterializationRequest {
    DescriptorImageOnly,
    RunnableLowerElState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MaterializationLeaseToken(u64);

impl MaterializationLeaseToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessPageTableMaterializationLeaseSnapshot {
    pub(crate) root_pages_leased: usize,
    pub(crate) table_pages_leased: usize,
    pub(crate) user_frames_leased: usize,
    pub(crate) descriptor_slots_installed: usize,
    pub(crate) root_page_releases: usize,
    pub(crate) table_page_releases: usize,
    pub(crate) user_frame_releases: usize,
    pub(crate) descriptor_slot_releases: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessPageTableMaterializationLeaseSource {
    root_page_capacity: usize,
    table_page_capacity: usize,
    user_frame_capacity: usize,
    descriptor_slot_capacity: usize,
    next_token: u64,
    next_physical_frame: u64,
    root_pages_leased: usize,
    table_pages_leased: usize,
    user_frames_leased: usize,
    descriptor_slots_installed: usize,
    root_page_releases: usize,
    table_page_releases: usize,
    user_frame_releases: usize,
    descriptor_slot_releases: usize,
    fail_population_page: Option<usize>,
}

impl ProcessPageTableMaterializationLeaseSource {
    pub(crate) const fn with_limits(
        table_page_capacity: usize,
        user_frame_capacity: usize,
        descriptor_slot_capacity: usize,
    ) -> Self {
        Self {
            root_page_capacity: 1,
            table_page_capacity,
            user_frame_capacity,
            descriptor_slot_capacity,
            next_token: 1,
            next_physical_frame: DEFAULT_MATERIALIZATION_FRAME_BASE,
            root_pages_leased: 0,
            table_pages_leased: 0,
            user_frames_leased: 0,
            descriptor_slots_installed: 0,
            root_page_releases: 0,
            table_page_releases: 0,
            user_frame_releases: 0,
            descriptor_slot_releases: 0,
            fail_population_page: None,
        }
    }

    pub(crate) const fn for_address_space(address_space: ProcessAddressSpace) -> Self {
        Self::with_limits(
            MATERIALIZATION_TABLE_PAGE_COUNT,
            address_space.user_frame_lease_count(),
            address_space.mapping_count(),
        )
    }

    pub(crate) fn deny_root(&mut self) {
        self.root_page_capacity = 0;
    }

    pub(crate) fn fail_population_at_page(&mut self, page_index: usize) {
        self.fail_population_page = Some(page_index);
    }

    pub(crate) const fn snapshot(self) -> ProcessPageTableMaterializationLeaseSnapshot {
        ProcessPageTableMaterializationLeaseSnapshot {
            root_pages_leased: self.root_pages_leased,
            table_pages_leased: self.table_pages_leased,
            user_frames_leased: self.user_frames_leased,
            descriptor_slots_installed: self.descriptor_slots_installed,
            root_page_releases: self.root_page_releases,
            table_page_releases: self.table_page_releases,
            user_frame_releases: self.user_frame_releases,
            descriptor_slot_releases: self.descriptor_slot_releases,
        }
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        self.root_pages_leased
            + self.table_pages_leased
            + self.user_frames_leased
            + self.descriptor_slots_installed
    }

    fn next_token(&mut self) -> MaterializationLeaseToken {
        let token = MaterializationLeaseToken(self.next_token);
        self.next_token += 1;
        token
    }

    fn lease_root_page(&mut self) -> Result<MaterializedRootPageLease, PosixError> {
        if self.root_pages_leased == self.root_page_capacity {
            return Err(PosixError::NoMemory);
        }
        self.root_pages_leased += 1;
        Ok(MaterializedRootPageLease {
            token: self.next_token(),
            physical_frame: self.next_frame()?,
            released: false,
        })
    }

    fn lease_table_page(&mut self, level: u8) -> Result<MaterializedTablePageLease, PosixError> {
        if self.table_pages_leased == self.table_page_capacity {
            return Err(PosixError::NoMemory);
        }
        self.table_pages_leased += 1;
        Ok(MaterializedTablePageLease {
            token: self.next_token(),
            physical_frame: self.next_frame()?,
            level,
            released: false,
        })
    }

    fn lease_user_frame(
        &mut self,
        mapping_index: usize,
        lease: UserFrameLease,
    ) -> Result<MaterializedUserFrameLease, PosixError> {
        if self.user_frames_leased == self.user_frame_capacity {
            return Err(PosixError::NoMemory);
        }
        if self.fail_population_page == Some(mapping_index) {
            return Err(PosixError::InvalidArgument);
        }
        self.user_frames_leased += 1;
        Ok(MaterializedUserFrameLease {
            token: self.next_token(),
            model_token: lease.token(),
            virtual_page: lease.virtual_page(),
            physical_frame: self.next_frame()?,
            kind: lease.kind(),
            permissions: lease.permissions(),
            zeroed_before_copy: lease.zeroed_before_copy(),
            copied_bytes: lease.copied_bytes(),
            zeroed_bytes: lease.zeroed_bytes(),
            source_page_ordinal: lease.source_page_ordinal(),
            scrub_required: true,
            released: false,
        })
    }

    fn install_descriptor_slot(&mut self) -> Result<(), PosixError> {
        if self.descriptor_slots_installed == self.descriptor_slot_capacity {
            return Err(PosixError::NoMemory);
        }
        self.descriptor_slots_installed += 1;
        Ok(())
    }

    fn release_descriptor_slot(&mut self) {
        if self.descriptor_slots_installed != 0 {
            self.descriptor_slots_installed -= 1;
            self.descriptor_slot_releases += 1;
        }
    }

    fn release_user_frame(&mut self, lease: &mut MaterializedUserFrameLease) {
        if !lease.released {
            lease.released = true;
            if self.user_frames_leased != 0 {
                self.user_frames_leased -= 1;
            }
            self.user_frame_releases += 1;
        }
    }

    fn release_table_page(&mut self, lease: &mut MaterializedTablePageLease) {
        if !lease.released {
            lease.released = true;
            if self.table_pages_leased != 0 {
                self.table_pages_leased -= 1;
            }
            self.table_page_releases += 1;
        }
    }

    fn release_root_page(&mut self, lease: &mut MaterializedRootPageLease) {
        if !lease.released {
            lease.released = true;
            if self.root_pages_leased != 0 {
                self.root_pages_leased -= 1;
            }
            self.root_page_releases += 1;
        }
    }

    fn next_frame(&mut self) -> Result<u64, PosixError> {
        let frame = self.next_physical_frame;
        self.next_physical_frame = self
            .next_physical_frame
            .checked_add(EARLY_PAGE_SIZE)
            .ok_or(PosixError::NoMemory)?;
        Ok(frame)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MaterializedRootPageLease {
    token: MaterializationLeaseToken,
    physical_frame: u64,
    released: bool,
}

impl MaterializedRootPageLease {
    pub(crate) const fn token(self) -> MaterializationLeaseToken {
        self.token
    }

    pub(crate) const fn physical_frame(self) -> u64 {
        self.physical_frame
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MaterializedTablePageLease {
    token: MaterializationLeaseToken,
    physical_frame: u64,
    level: u8,
    released: bool,
}

impl MaterializedTablePageLease {
    pub(crate) const fn token(self) -> MaterializationLeaseToken {
        self.token
    }

    pub(crate) const fn physical_frame(self) -> u64 {
        self.physical_frame
    }

    pub(crate) const fn level(self) -> u8 {
        self.level
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MaterializedUserFrameLease {
    token: MaterializationLeaseToken,
    model_token: crate::process_address_space::ModelLeaseToken,
    virtual_page: u64,
    physical_frame: u64,
    kind: crate::program_loader::UserSegmentKind,
    permissions: UserMappingPermissions,
    zeroed_before_copy: bool,
    copied_bytes: u64,
    zeroed_bytes: u64,
    source_page_ordinal: usize,
    scrub_required: bool,
    released: bool,
}

impl MaterializedUserFrameLease {
    pub(crate) const fn token(self) -> MaterializationLeaseToken {
        self.token
    }

    pub(crate) const fn model_token(self) -> crate::process_address_space::ModelLeaseToken {
        self.model_token
    }

    pub(crate) const fn virtual_page(self) -> u64 {
        self.virtual_page
    }

    pub(crate) const fn physical_frame(self) -> u64 {
        self.physical_frame
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn kind(self) -> crate::program_loader::UserSegmentKind {
        self.kind
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn permissions(self) -> UserMappingPermissions {
        self.permissions
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
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

    pub(crate) const fn scrub_required(self) -> bool {
        self.scrub_required
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessPageDescriptorRecord {
    mapping_index: usize,
    kind: crate::program_loader::UserSegmentKind,
    virtual_page: u64,
    physical_frame: u64,
    l0_slot: usize,
    l1_slot: usize,
    l2_slot: usize,
    l3_slot: usize,
    descriptor_value: u64,
    user_access: bool,
    privileged_execute_never: bool,
    user_execute_never: bool,
    normal_inner_shareable: bool,
    writable: bool,
    executable: bool,
    write_xor_execute: bool,
}

impl ProcessPageDescriptorRecord {
    pub(crate) const fn mapping_index(self) -> usize {
        self.mapping_index
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn kind(self) -> crate::program_loader::UserSegmentKind {
        self.kind
    }

    pub(crate) const fn virtual_page(self) -> u64 {
        self.virtual_page
    }

    pub(crate) const fn physical_frame(self) -> u64 {
        self.physical_frame
    }

    pub(crate) const fn l3_slot(self) -> usize {
        self.l3_slot
    }

    pub(crate) const fn descriptor_value(self) -> u64 {
        self.descriptor_value
    }

    pub(crate) const fn privileged_execute_never(self) -> bool {
        self.privileged_execute_never
    }

    pub(crate) const fn user_execute_never(self) -> bool {
        self.user_execute_never
    }

    #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
    pub(crate) const fn normal_inner_shareable(self) -> bool {
        self.normal_inner_shareable
    }

    pub(crate) const fn writable(self) -> bool {
        self.writable
    }

    pub(crate) const fn executable(self) -> bool {
        self.executable
    }

    pub(crate) const fn write_xor_execute(self) -> bool {
        self.write_xor_execute
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessPageTableMaterializationSideEffects {
    root_pages_leased: usize,
    table_pages_leased: usize,
    user_frames_leased: usize,
    user_frames_populated: usize,
    descriptors_installed: usize,
    copied_bytes: u64,
    zeroed_bytes: u64,
    rollback_releases: usize,
    teardown_releases: usize,
    activation_blocked: bool,
}

impl ProcessPageTableMaterializationSideEffects {
    const NONE: Self = Self {
        root_pages_leased: 0,
        table_pages_leased: 0,
        user_frames_leased: 0,
        user_frames_populated: 0,
        descriptors_installed: 0,
        copied_bytes: 0,
        zeroed_bytes: 0,
        rollback_releases: 0,
        teardown_releases: 0,
        activation_blocked: true,
    };

    pub(crate) const fn root_pages_leased(self) -> usize {
        self.root_pages_leased
    }

    pub(crate) const fn table_pages_leased(self) -> usize {
        self.table_pages_leased
    }

    pub(crate) const fn user_frames_leased(self) -> usize {
        self.user_frames_leased
    }

    pub(crate) const fn user_frames_populated(self) -> usize {
        self.user_frames_populated
    }

    pub(crate) const fn descriptors_installed(self) -> usize {
        self.descriptors_installed
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

    pub(crate) const fn teardown_releases(self) -> usize {
        self.teardown_releases
    }

    pub(crate) const fn activation_blocked(self) -> bool {
        self.activation_blocked
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessPageTableMaterializationTeardownReport {
    descriptors_cleared: usize,
    table_pages_released: usize,
    user_frames_released: usize,
    root_released: bool,
    already_destroyed: bool,
}

impl ProcessPageTableMaterializationTeardownReport {
    pub(crate) const fn descriptors_cleared(self) -> usize {
        self.descriptors_cleared
    }

    pub(crate) const fn table_pages_released(self) -> usize {
        self.table_pages_released
    }

    pub(crate) const fn user_frames_released(self) -> usize {
        self.user_frames_released
    }

    pub(crate) const fn root_released(self) -> bool {
        self.root_released
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessPageTableMaterialization {
    id: u64,
    boundary_identity: &'static str,
    kernel_mapping_policy: &'static str,
    source_digest: u64,
    root: MaterializedRootPageLease,
    table_pages: [Option<MaterializedTablePageLease>; MATERIALIZATION_TABLE_PAGE_COUNT],
    table_page_count: usize,
    user_frames: [Option<MaterializedUserFrameLease>; MAX_PROCESS_INSTALL_PAGES],
    user_frame_count: usize,
    descriptors: [Option<ProcessPageDescriptorRecord>; MAX_PROCESS_INSTALL_PAGES],
    descriptor_count: usize,
    side_effects: ProcessPageTableMaterializationSideEffects,
    published: bool,
    destroyed: bool,
}

impl ProcessPageTableMaterialization {
    pub(crate) const fn id(self) -> u64 {
        self.id
    }

    pub(crate) const fn boundary_identity(self) -> &'static str {
        self.boundary_identity
    }

    pub(crate) const fn kernel_mapping_policy(self) -> &'static str {
        self.kernel_mapping_policy
    }

    pub(crate) const fn source_digest(self) -> u64 {
        self.source_digest
    }

    pub(crate) const fn root(self) -> MaterializedRootPageLease {
        self.root
    }

    pub(crate) const fn table_page_count(self) -> usize {
        self.table_page_count
    }

    pub(crate) const fn table_page(self, index: usize) -> Option<MaterializedTablePageLease> {
        if index >= MATERIALIZATION_TABLE_PAGE_COUNT {
            None
        } else {
            self.table_pages[index]
        }
    }

    pub(crate) const fn user_frame_count(self) -> usize {
        self.user_frame_count
    }

    pub(crate) const fn user_frame(self, index: usize) -> Option<MaterializedUserFrameLease> {
        if index >= MAX_PROCESS_INSTALL_PAGES {
            None
        } else {
            self.user_frames[index]
        }
    }

    pub(crate) const fn descriptor_count(self) -> usize {
        self.descriptor_count
    }

    pub(crate) const fn descriptor(self, index: usize) -> Option<ProcessPageDescriptorRecord> {
        if index >= MAX_PROCESS_INSTALL_PAGES {
            None
        } else {
            self.descriptors[index]
        }
    }

    pub(crate) const fn side_effects(self) -> ProcessPageTableMaterializationSideEffects {
        self.side_effects
    }

    pub(crate) const fn published(self) -> bool {
        self.published
    }

    pub(crate) const fn destroyed(self) -> bool {
        self.destroyed
    }

    pub(crate) const fn activation_blocked(self) -> bool {
        self.side_effects.activation_blocked
    }

    #[cfg(any(
        talos_boot_scenario = "qemu_initial_process_launch_smoke",
        talos_boot_scenario = "qemu_initial_user_stack_smoke",
        talos_boot_scenario = "qemu_live_address_space_activation_smoke"
    ))]
    pub(crate) fn for_test_missing_descriptor(mut self, index: usize) -> Self {
        if index < MAX_PROCESS_INSTALL_PAGES {
            self.descriptors[index] = None;
        }
        self
    }

    pub(crate) fn destroy(
        &mut self,
        lease_source: &mut ProcessPageTableMaterializationLeaseSource,
    ) -> ProcessPageTableMaterializationTeardownReport {
        if self.destroyed {
            return ProcessPageTableMaterializationTeardownReport {
                descriptors_cleared: 0,
                table_pages_released: 0,
                user_frames_released: 0,
                root_released: false,
                already_destroyed: true,
            };
        }

        let mut descriptors_cleared = 0;
        while self.descriptor_count != 0 {
            self.descriptor_count -= 1;
            self.descriptors[self.descriptor_count] = None;
            lease_source.release_descriptor_slot();
            descriptors_cleared += 1;
        }

        let mut table_pages_released = 0;
        while self.table_page_count != 0 {
            self.table_page_count -= 1;
            if let Some(mut lease) = self.table_pages[self.table_page_count] {
                lease_source.release_table_page(&mut lease);
                self.table_pages[self.table_page_count] = Some(lease);
                table_pages_released += 1;
            }
        }

        let mut user_frames_released = 0;
        while self.user_frame_count != 0 {
            self.user_frame_count -= 1;
            if let Some(mut lease) = self.user_frames[self.user_frame_count] {
                lease_source.release_user_frame(&mut lease);
                self.user_frames[self.user_frame_count] = Some(lease);
                user_frames_released += 1;
            }
        }

        lease_source.release_root_page(&mut self.root);
        self.side_effects.teardown_releases =
            descriptors_cleared + table_pages_released + user_frames_released + 1;
        self.published = false;
        self.destroyed = true;

        ProcessPageTableMaterializationTeardownReport {
            descriptors_cleared,
            table_pages_released,
            user_frames_released,
            root_released: true,
            already_destroyed: false,
        }
    }
}

pub(crate) fn materialize_process_page_tables(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
    request: ProcessMaterializationRequest,
    lease_source: &mut ProcessPageTableMaterializationLeaseSource,
) -> Result<ProcessPageTableMaterialization, PosixError> {
    if request == ProcessMaterializationRequest::RunnableLowerElState {
        return Err(PosixError::NotImplemented);
    }

    validate_inputs(image, install_plan, address_space)?;
    let topology = ProcessTableTopology::from_address_space(address_space)?;

    let root = lease_source.lease_root_page()?;
    let mut partial = PartialMaterialization {
        root,
        table_pages: [None; MATERIALIZATION_TABLE_PAGE_COUNT],
        table_page_count: 0,
        user_frames: [None; MAX_PROCESS_INSTALL_PAGES],
        user_frame_count: 0,
        descriptors: [None; MAX_PROCESS_INSTALL_PAGES],
        descriptor_count: 0,
        side_effects: ProcessPageTableMaterializationSideEffects::NONE,
    };

    let result =
        materialize_validated_address_space(address_space, topology, lease_source, &mut partial);
    match result {
        Ok(()) => Ok(ProcessPageTableMaterialization {
            id: materialization_id(address_space),
            boundary_identity: PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
            kernel_mapping_policy: PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY,
            source_digest: image.source_digest(),
            root: partial.root,
            table_pages: partial.table_pages,
            table_page_count: partial.table_page_count,
            user_frames: partial.user_frames,
            user_frame_count: partial.user_frame_count,
            descriptors: partial.descriptors,
            descriptor_count: partial.descriptor_count,
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

struct PartialMaterialization {
    root: MaterializedRootPageLease,
    table_pages: [Option<MaterializedTablePageLease>; MATERIALIZATION_TABLE_PAGE_COUNT],
    table_page_count: usize,
    user_frames: [Option<MaterializedUserFrameLease>; MAX_PROCESS_INSTALL_PAGES],
    user_frame_count: usize,
    descriptors: [Option<ProcessPageDescriptorRecord>; MAX_PROCESS_INSTALL_PAGES],
    descriptor_count: usize,
    side_effects: ProcessPageTableMaterializationSideEffects,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProcessTableTopology {
    l0_slot: usize,
    l1_slot: usize,
    l2_slot: usize,
}

impl ProcessTableTopology {
    fn from_address_space(address_space: ProcessAddressSpace) -> Result<Self, PosixError> {
        let first = address_space
            .mapping(0)
            .ok_or(PosixError::InvalidArgument)?;
        let topology = Self {
            l0_slot: table_slot(first.virtual_start(), 39)?,
            l1_slot: table_slot(first.virtual_start(), 30)?,
            l2_slot: table_slot(first.virtual_start(), 21)?,
        };
        let mut index = 0;
        while index < address_space.mapping_count() {
            let mapping = address_space
                .mapping(index)
                .ok_or(PosixError::InvalidArgument)?;
            if table_slot(mapping.virtual_start(), 39)? != topology.l0_slot
                || table_slot(mapping.virtual_start(), 30)? != topology.l1_slot
                || table_slot(mapping.virtual_start(), 21)? != topology.l2_slot
            {
                return Err(PosixError::NotSupported);
            }
            index += 1;
        }
        Ok(topology)
    }
}

fn materialize_validated_address_space(
    address_space: ProcessAddressSpace,
    topology: ProcessTableTopology,
    lease_source: &mut ProcessPageTableMaterializationLeaseSource,
    partial: &mut PartialMaterialization,
) -> Result<(), PosixError> {
    let mut level = 1u8;
    while partial.table_page_count < MATERIALIZATION_TABLE_PAGE_COUNT {
        partial.table_pages[partial.table_page_count] = Some(lease_source.lease_table_page(level)?);
        partial.table_page_count += 1;
        partial.side_effects.table_pages_leased += 1;
        level += 1;
    }

    let mut index = 0;
    while index < address_space.mapping_count() {
        let mapping = address_space
            .mapping(index)
            .ok_or(PosixError::InvalidArgument)?;
        validate_mapping(mapping)?;
        let model_lease = address_space
            .user_frame_lease(index)
            .ok_or(PosixError::InvalidArgument)?;
        validate_user_frame_lease(mapping, model_lease)?;
        let user_frame = lease_source.lease_user_frame(index, model_lease)?;
        partial.side_effects.user_frames_leased += 1;
        partial.side_effects.copied_bytes += user_frame.copied_bytes();
        partial.side_effects.zeroed_bytes += user_frame.zeroed_bytes();
        partial.side_effects.user_frames_populated += 1;
        partial.user_frames[partial.user_frame_count] = Some(user_frame);
        partial.user_frame_count += 1;

        let descriptor = descriptor_record(index, mapping, user_frame, topology)?;
        lease_source.install_descriptor_slot()?;
        partial.descriptors[partial.descriptor_count] = Some(descriptor);
        partial.descriptor_count += 1;
        partial.side_effects.descriptors_installed += 1;
        index += 1;
    }

    partial.side_effects.root_pages_leased = 1;
    Ok(())
}

fn rollback_partial(
    lease_source: &mut ProcessPageTableMaterializationLeaseSource,
    partial: &mut PartialMaterialization,
) {
    while partial.descriptor_count != 0 {
        partial.descriptor_count -= 1;
        partial.descriptors[partial.descriptor_count] = None;
        lease_source.release_descriptor_slot();
        partial.side_effects.rollback_releases += 1;
    }

    while partial.user_frame_count != 0 {
        partial.user_frame_count -= 1;
        if let Some(mut lease) = partial.user_frames[partial.user_frame_count] {
            lease_source.release_user_frame(&mut lease);
            partial.user_frames[partial.user_frame_count] = Some(lease);
            partial.side_effects.rollback_releases += 1;
        }
    }

    while partial.table_page_count != 0 {
        partial.table_page_count -= 1;
        if let Some(mut lease) = partial.table_pages[partial.table_page_count] {
            lease_source.release_table_page(&mut lease);
            partial.table_pages[partial.table_page_count] = Some(lease);
            partial.side_effects.rollback_releases += 1;
        }
    }

    lease_source.release_root_page(&mut partial.root);
    partial.side_effects.rollback_releases += 1;
}

fn validate_inputs(
    image: ProgramImagePlan,
    install_plan: ProcessImageInstallPlan,
    address_space: ProcessAddressSpace,
) -> Result<(), PosixError> {
    if image.fixture_identity() != PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        || install_plan.fixture_identity() != image.fixture_identity()
        || install_plan.install_boundary_identity() != PROCESS_INSTALL_BOUNDARY_IDENTITY
        || install_plan.source_path() != image.source_path()
        || install_plan.source_digest() != image.source_digest()
        || install_plan.entry() != image.entry()
        || install_plan.memory_footprint() != image.memory_footprint()
        || !install_plan.lower_el_launch_blocked()
        || address_space.boundary_identity() != PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        || !address_space.published()
        || address_space.destroyed()
        || address_space.root().released()
        || address_space.table_lease_count() == 0
        || address_space.user_frame_lease_count() != install_plan.page_count()
        || address_space.mapping_count() != install_plan.page_count()
    {
        return Err(PosixError::InvalidArgument);
    }

    let mut index = 0;
    while index < install_plan.page_count() {
        let page = install_plan
            .page(index)
            .ok_or(PosixError::InvalidArgument)?;
        let mapping = address_space
            .mapping(index)
            .ok_or(PosixError::InvalidArgument)?;
        let lease = address_space
            .user_frame_lease(index)
            .ok_or(PosixError::InvalidArgument)?;
        if mapping.virtual_start() != page.virtual_start()
            || mapping.virtual_end() != page.virtual_end()
            || mapping.kind() != page.kind()
            || mapping.permissions() != page.permissions()
            || mapping.copy_page_offset() != page.copy_page_offset()
            || mapping.copy_file_offset() != page.copy_file_offset()
            || mapping.copy_len() != page.copy_len()
            || mapping.zero_len() != page.zero_len()
            || mapping.source_page_ordinal() != page.index()
            || lease.virtual_page() != page.virtual_start()
            || lease.kind() != page.kind()
            || lease.permissions() != page.permissions()
            || lease.copied_bytes() != page.copy_len()
            || lease.zeroed_bytes() != page.zero_len()
            || lease.source_page_ordinal() != page.index()
            || lease.released()
        {
            return Err(PosixError::InvalidArgument);
        }
        index += 1;
    }

    Ok(())
}

fn validate_mapping(mapping: ProcessUserMapping) -> Result<(), PosixError> {
    if mapping.virtual_start() >= mapping.virtual_end()
        || mapping.virtual_start() % LOADER_PAGE_SIZE != 0
        || mapping.virtual_end() != mapping.virtual_start() + LOADER_PAGE_SIZE
    {
        return Err(PosixError::InvalidArgument);
    }
    if mapping.virtual_start() < USER_NULL_GUARD_END
        || mapping.virtual_end() > USER_ADDRESS_SPACE_END
    {
        return Err(PosixError::AccessDenied);
    }
    if !mapping.el0_user_access()
        || !mapping.write_xor_execute()
        || !mapping.normal_memory_intent()
        || !mapping.kernel_device_denied()
    {
        return Err(PosixError::AccessDenied);
    }
    match mapping.kind() {
        UserSegmentKind::UserText if mapping.permissions() == UserMappingPermissions::USER_TEXT => {
            Ok(())
        }
        UserSegmentKind::UserData
            if matches!(
                mapping.permissions(),
                UserMappingPermissions::USER_DATA | UserMappingPermissions::READ
            ) =>
        {
            Ok(())
        }
        _ => Err(PosixError::AccessDenied),
    }
}

fn validate_user_frame_lease(
    mapping: ProcessUserMapping,
    lease: UserFrameLease,
) -> Result<(), PosixError> {
    let total_bytes = lease
        .copied_bytes()
        .checked_add(lease.zeroed_bytes())
        .ok_or(PosixError::InvalidArgument)?;
    if lease.virtual_page() != mapping.virtual_start()
        || lease.kind() != mapping.kind()
        || lease.permissions() != mapping.permissions()
        || !lease.zeroed_before_copy()
        || lease.source_page_ordinal() != mapping.source_page_ordinal()
        || lease.copied_bytes() != mapping.copy_len()
        || lease.zeroed_bytes() != mapping.zero_len()
        || total_bytes > LOADER_PAGE_SIZE
        || lease.released()
    {
        return Err(PosixError::InvalidArgument);
    }
    Ok(())
}

fn descriptor_record(
    mapping_index: usize,
    mapping: ProcessUserMapping,
    frame: MaterializedUserFrameLease,
    topology: ProcessTableTopology,
) -> Result<ProcessPageDescriptorRecord, PosixError> {
    let writable = mapping
        .permissions()
        .contains(UserMappingPermissions::WRITE);
    let executable = mapping
        .permissions()
        .contains(UserMappingPermissions::EXECUTE);
    if writable && executable {
        return Err(PosixError::AccessDenied);
    }
    let descriptor_value = page_descriptor(frame.physical_frame(), mapping.permissions())
        .ok_or(PosixError::InvalidArgument)?;
    Ok(ProcessPageDescriptorRecord {
        mapping_index,
        kind: mapping.kind(),
        virtual_page: mapping.virtual_start(),
        physical_frame: frame.physical_frame(),
        l0_slot: topology.l0_slot,
        l1_slot: topology.l1_slot,
        l2_slot: topology.l2_slot,
        l3_slot: table_slot(mapping.virtual_start(), 12)?,
        descriptor_value,
        user_access: true,
        privileged_execute_never: true,
        user_execute_never: !executable,
        normal_inner_shareable: true,
        writable,
        executable,
        write_xor_execute: !(writable && executable),
    })
}

fn page_descriptor(physical_frame: u64, permissions: UserMappingPermissions) -> Option<u64> {
    if physical_frame % EARLY_PAGE_SIZE != 0 {
        return None;
    }
    let ap = if permissions.contains(UserMappingPermissions::WRITE) {
        STAGE1_DESC_AP_EL0_RW
    } else {
        STAGE1_DESC_AP_EL0_RO
    };
    let uxn = if permissions.contains(UserMappingPermissions::EXECUTE) {
        0
    } else {
        STAGE1_DESC_UXN
    };
    Some(
        (physical_frame & STAGE1_TABLE_ADDR_MASK)
            | STAGE1_DESC_TABLE
            | STAGE1_DESC_VALID
            | (EARLY_TRANSLATION_NORMAL_ATTR_INDEX << STAGE1_DESC_ATTR_INDEX_SHIFT)
            | ap
            | STAGE1_DESC_SH_INNER
            | STAGE1_DESC_AF
            | STAGE1_DESC_PXN
            | uxn,
    )
}

fn table_slot(virtual_address: u64, shift: u64) -> Result<usize, PosixError> {
    Ok(((virtual_address >> shift) & (MATERIALIZATION_PAGE_ENTRIES as u64 - 1)) as usize)
}

fn materialization_id(address_space: ProcessAddressSpace) -> u64 {
    0x8_3000_0000 | address_space.id().raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initramfs::{PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
        process_address_space::{
            ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource, install_process_address_space,
        },
        process_install::{
            PageByteRange, ProcessImagePageInstallRecord, ProcessInstallAction,
            ProcessInstallSideEffects, plan_process_image_install,
        },
        program_loader::{
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, PlannedUserSegment, UserSegmentKind,
            plan_phase8_init_image,
        },
        scheduler::ProcessOwnerId,
    };

    fn image_fixture() -> ProgramImagePlan {
        plan_phase8_init_image(phase8_readonly_initramfs_fixture()).expect("program image")
    }

    fn install_fixture() -> ProcessImageInstallPlan {
        plan_process_image_install(image_fixture()).expect("install plan")
    }

    fn address_space_fixture(
        plan: ProcessImageInstallPlan,
    ) -> (ProcessAddressSpace, ProcessAddressSpaceLeaseSource) {
        let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(plan);
        let address_space = install_process_address_space(
            plan,
            ProcessAddressSpaceId::new(0x8000_0001).expect("address space id"),
            Some(ProcessOwnerId::new(0x55).expect("owner id")),
            &mut lease_source,
        )
        .expect("address space");
        (address_space, lease_source)
    }

    fn materialize_fixture() -> (
        ProcessPageTableMaterialization,
        ProcessPageTableMaterializationLeaseSource,
    ) {
        let image = image_fixture();
        let plan = install_fixture();
        let (address_space, _) = address_space_fixture(plan);
        let mut lease_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
        let materialization = materialize_process_page_tables(
            image,
            plan,
            address_space,
            ProcessMaterializationRequest::DescriptorImageOnly,
            &mut lease_source,
        )
        .expect("materialization");
        (materialization, lease_source)
    }

    fn readonly_data_plan() -> (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessAddressSpaceLeaseSource,
    ) {
        let text = ProgramImagePlan::for_test_unchecked(
            PHASE8_INIT_PATH,
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            0x204,
            0xf4a6_cc15_f4d9_4461,
            0x0000_0000_0001_0100,
            2,
            [
                Some(PlannedUserSegment::for_test_unchecked(
                    UserSegmentKind::UserText,
                    UserMappingPermissions::USER_TEXT,
                    0x0000_0000_0001_0100,
                    0x0000_0000_0001_0104,
                    0x0000_0000_0001_0000,
                    0x0000_0000_0001_1000,
                    0x100,
                    4,
                    0x0000_0000_0001_0104,
                    0x0000_0000_0001_0104,
                )),
                Some(PlannedUserSegment::for_test_unchecked(
                    UserSegmentKind::UserData,
                    UserMappingPermissions::READ,
                    0x0000_0000_0002_0000,
                    0x0000_0000_0002_0004,
                    0x0000_0000_0002_0000,
                    0x0000_0000_0002_1000,
                    0x200,
                    4,
                    0x0000_0000_0002_0004,
                    0x0000_0000_0002_0004,
                )),
                None,
                None,
            ],
            0x0000_0000_0001_0000,
            0x0000_0000_0002_1000,
            LOADER_PAGE_SIZE * 2,
        );
        let plan = plan_process_image_install(text).expect("readonly data install plan");
        let (address_space, lease_source) = address_space_fixture(plan);
        (text, plan, address_space, lease_source)
    }

    #[test_case]
    fn materializes_descriptor_image_with_preserved_permissions() {
        let (materialization, lease_source) = materialize_fixture();

        assert_eq!(
            materialization.boundary_identity(),
            PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        );
        assert_eq!(
            materialization.kernel_mapping_policy(),
            PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY
        );
        assert_eq!(materialization.source_digest(), 0xf4a6_cc15_f4d9_4461);
        assert_eq!(materialization.id(), 0x8_b000_0001);
        assert!(materialization.published());
        assert!(!materialization.destroyed());
        assert!(materialization.activation_blocked());
        assert_eq!(
            materialization.root().physical_frame(),
            DEFAULT_MATERIALIZATION_FRAME_BASE
        );
        assert!(!materialization.root().released());
        assert_ne!(materialization.root().token().raw(), 0);
        assert_eq!(
            materialization.table_page_count(),
            MATERIALIZATION_TABLE_PAGE_COUNT
        );
        assert_eq!(materialization.table_page(0).expect("l1 table").level(), 1);
        assert_eq!(
            materialization
                .table_page(0)
                .expect("l1 table")
                .physical_frame(),
            DEFAULT_MATERIALIZATION_FRAME_BASE + EARLY_PAGE_SIZE
        );
        assert!(!materialization.table_page(0).expect("l1 table").released());
        assert_ne!(
            materialization
                .table_page(0)
                .expect("l1 table")
                .token()
                .raw(),
            0
        );
        assert_eq!(materialization.user_frame_count(), 3);
        assert_eq!(materialization.descriptor_count(), 3);
        assert_eq!(materialization.side_effects().root_pages_leased(), 1);
        assert_eq!(
            materialization.side_effects().table_pages_leased(),
            MATERIALIZATION_TABLE_PAGE_COUNT
        );
        assert_eq!(materialization.side_effects().user_frames_leased(), 3);
        assert_eq!(materialization.side_effects().user_frames_populated(), 3);
        assert_eq!(materialization.side_effects().descriptors_installed(), 3);
        assert_eq!(materialization.side_effects().copied_bytes(), 8);
        assert_eq!(materialization.side_effects().zeroed_bytes(), 0x2ff8);
        assert_eq!(materialization.side_effects().rollback_releases(), 0);
        assert_eq!(materialization.side_effects().teardown_releases(), 0);
        assert!(materialization.side_effects().activation_blocked());

        let text_frame = materialization.user_frame(0).expect("text frame");
        assert_eq!(text_frame.virtual_page(), 0x0000_0000_0001_0000);
        assert_eq!(text_frame.copied_bytes(), 4);
        assert_eq!(text_frame.zeroed_bytes(), 0xffc);
        assert_eq!(text_frame.source_page_ordinal(), 0);
        assert!(text_frame.scrub_required());
        assert!(!text_frame.released());
        assert_ne!(text_frame.token().raw(), 0);
        assert_ne!(text_frame.model_token().raw(), 0);

        let text = materialization.descriptor(0).expect("text descriptor");
        assert_eq!(text.mapping_index(), 0);
        assert_eq!(text.virtual_page(), 0x0000_0000_0001_0000);
        assert_eq!(text.physical_frame(), text_frame.physical_frame());
        assert_eq!(text.l3_slot(), 0x10);
        assert!(text.privileged_execute_never());
        assert!(!text.user_execute_never());
        assert!(!text.writable());
        assert!(text.executable());
        assert!(text.write_xor_execute());
        assert_eq!(
            text.descriptor_value() & STAGE1_DESC_VALID,
            STAGE1_DESC_VALID
        );
        assert_eq!(
            text.descriptor_value() & STAGE1_DESC_TABLE,
            STAGE1_DESC_TABLE
        );
        assert_eq!(text.descriptor_value() & STAGE1_DESC_PXN, STAGE1_DESC_PXN);
        assert_eq!(text.descriptor_value() & STAGE1_DESC_UXN, 0);

        let data = materialization.descriptor(1).expect("data descriptor");
        assert_eq!(data.l3_slot(), 0x20);
        assert!(data.privileged_execute_never());
        assert!(data.user_execute_never());
        assert!(data.writable());
        assert!(!data.executable());
        assert!(data.write_xor_execute());
        assert_eq!(data.descriptor_value() & STAGE1_DESC_UXN, STAGE1_DESC_UXN);

        let snapshot = lease_source.snapshot();
        assert_eq!(snapshot.root_pages_leased, 1);
        assert_eq!(
            snapshot.table_pages_leased,
            MATERIALIZATION_TABLE_PAGE_COUNT
        );
        assert_eq!(snapshot.user_frames_leased, 3);
        assert_eq!(snapshot.descriptor_slots_installed, 3);
        assert_eq!(lease_source.outstanding_leases(), 10);
    }

    #[test_case]
    fn teardown_clears_descriptors_and_releases_leases_once() {
        let (mut materialization, mut lease_source) = materialize_fixture();

        let first = materialization.destroy(&mut lease_source);
        assert_eq!(first.descriptors_cleared(), 3);
        assert_eq!(
            first.table_pages_released(),
            MATERIALIZATION_TABLE_PAGE_COUNT
        );
        assert_eq!(first.user_frames_released(), 3);
        assert!(first.root_released());
        assert!(!first.already_destroyed());
        assert!(!materialization.published());
        assert!(materialization.destroyed());
        assert_eq!(lease_source.outstanding_leases(), 0);

        let second = materialization.destroy(&mut lease_source);
        assert_eq!(second.descriptors_cleared(), 0);
        assert_eq!(second.table_pages_released(), 0);
        assert_eq!(second.user_frames_released(), 0);
        assert!(!second.root_released());
        assert!(second.already_destroyed());
        assert_eq!(lease_source.outstanding_leases(), 0);

        let snapshot = lease_source.snapshot();
        assert_eq!(snapshot.descriptor_slot_releases, 3);
        assert_eq!(snapshot.user_frame_releases, 3);
        assert_eq!(
            snapshot.table_page_releases,
            MATERIALIZATION_TABLE_PAGE_COUNT
        );
        assert_eq!(snapshot.root_page_releases, 1);
    }

    #[test_case]
    fn materializes_readonly_data_descriptors_as_el0_read_only_non_executable() {
        let (image, plan, address_space, _) = readonly_data_plan();
        let mut lease_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);

        let materialization = materialize_process_page_tables(
            image,
            plan,
            address_space,
            ProcessMaterializationRequest::DescriptorImageOnly,
            &mut lease_source,
        )
        .expect("readonly data materialization");

        let rodata = materialization
            .descriptor(1)
            .expect("readonly data descriptor");
        assert_eq!(rodata.mapping_index(), 1);
        assert_eq!(rodata.virtual_page(), 0x0000_0000_0002_0000);
        assert!(rodata.user_access);
        assert!(rodata.privileged_execute_never());
        assert!(rodata.user_execute_never());
        assert!(!rodata.writable());
        assert!(!rodata.executable());
        assert!(rodata.write_xor_execute());
        assert_eq!(rodata.descriptor_value() & STAGE1_DESC_UXN, STAGE1_DESC_UXN);
        assert_eq!(
            rodata.descriptor_value() & STAGE1_DESC_AP_EL0_RO,
            STAGE1_DESC_AP_EL0_RO
        );
    }

    #[test_case]
    fn rejects_activation_request_before_leasing() {
        let image = image_fixture();
        let plan = install_fixture();
        let (address_space, _) = address_space_fixture(plan);
        let mut lease_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);

        assert_eq!(
            materialize_process_page_tables(
                image,
                plan,
                address_space,
                ProcessMaterializationRequest::RunnableLowerElState,
                &mut lease_source,
            ),
            Err(PosixError::NotImplemented)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rejects_bad_address_space_or_install_mismatch_before_leasing() {
        let image = image_fixture();
        let plan = install_fixture();
        let (mut address_space, mut address_space_leases) = address_space_fixture(plan);
        let _ = address_space.destroy(&mut address_space_leases);
        let mut lease_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
        assert_eq!(
            materialize_process_page_tables(
                image,
                plan,
                address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut lease_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);

        let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
        pages[0] = plan.page(0);
        let bad_plan = ProcessImageInstallPlan::for_test_unchecked(
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            PROCESS_INSTALL_BOUNDARY_IDENTITY,
            PHASE8_INIT_PATH,
            0xdead_beef,
            plan.entry(),
            plan.memory_footprint(),
            1,
            pages,
            ProcessInstallSideEffects::NONE,
            true,
        );
        let (fresh_address_space, _) = address_space_fixture(plan);
        let mut bad_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(fresh_address_space);
        assert_eq!(
            materialize_process_page_tables(
                image,
                bad_plan,
                fresh_address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut bad_source,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(bad_source.outstanding_leases(), 0);
    }

    #[test_case]
    fn rolls_back_resource_exhaustion_and_population_failures() {
        let image = image_fixture();
        let plan = install_fixture();
        let (address_space, _) = address_space_fixture(plan);

        let mut no_root =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
        no_root.deny_root();
        assert_eq!(
            materialize_process_page_tables(
                image,
                plan,
                address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut no_root,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_root.outstanding_leases(), 0);
        assert_eq!(no_root.snapshot().root_page_releases, 0);

        let mut no_tables = ProcessPageTableMaterializationLeaseSource::with_limits(1, 3, 3);
        assert_eq!(
            materialize_process_page_tables(
                image,
                plan,
                address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut no_tables,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_tables.outstanding_leases(), 0);
        assert_eq!(no_tables.snapshot().root_page_releases, 1);
        assert_eq!(no_tables.snapshot().table_page_releases, 1);

        let mut no_frames = ProcessPageTableMaterializationLeaseSource::with_limits(
            MATERIALIZATION_TABLE_PAGE_COUNT,
            1,
            3,
        );
        assert_eq!(
            materialize_process_page_tables(
                image,
                plan,
                address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut no_frames,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_frames.outstanding_leases(), 0);
        assert_eq!(no_frames.snapshot().descriptor_slot_releases, 1);
        assert_eq!(no_frames.snapshot().user_frame_releases, 1);
        assert_eq!(
            no_frames.snapshot().table_page_releases,
            MATERIALIZATION_TABLE_PAGE_COUNT
        );
        assert_eq!(no_frames.snapshot().root_page_releases, 1);

        let mut no_descriptors = ProcessPageTableMaterializationLeaseSource::with_limits(
            MATERIALIZATION_TABLE_PAGE_COUNT,
            3,
            1,
        );
        assert_eq!(
            materialize_process_page_tables(
                image,
                plan,
                address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut no_descriptors,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(no_descriptors.outstanding_leases(), 0);
        assert_eq!(no_descriptors.snapshot().descriptor_slot_releases, 1);
        assert_eq!(no_descriptors.snapshot().user_frame_releases, 2);

        let mut copy_zero_failure =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
        copy_zero_failure.fail_population_at_page(1);
        assert_eq!(
            materialize_process_page_tables(
                image,
                plan,
                address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut copy_zero_failure,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(copy_zero_failure.outstanding_leases(), 0);
        assert_eq!(copy_zero_failure.snapshot().descriptor_slot_releases, 1);
        assert_eq!(copy_zero_failure.snapshot().user_frame_releases, 1);
    }

    #[test_case]
    fn rejects_unsupported_multiple_l2_topology_without_leasing() {
        let text = ProcessImagePageInstallRecord::for_test_unchecked(
            0,
            0,
            USER_NULL_GUARD_END,
            USER_NULL_GUARD_END + LOADER_PAGE_SIZE,
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_TEXT,
            0,
            0,
            4,
            [
                Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4)),
                None,
            ],
            1,
            LOADER_PAGE_SIZE - 4,
            ProcessInstallAction::AllocateCopyZeroMap,
        );
        let data = ProcessImagePageInstallRecord::for_test_unchecked(
            1,
            1,
            USER_NULL_GUARD_END + 0x20_0000,
            USER_NULL_GUARD_END + 0x20_0000 + LOADER_PAGE_SIZE,
            UserSegmentKind::UserData,
            UserMappingPermissions::USER_DATA,
            0,
            4,
            4,
            [
                Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4)),
                None,
            ],
            1,
            LOADER_PAGE_SIZE - 4,
            ProcessInstallAction::AllocateCopyZeroMap,
        );
        let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
        pages[0] = Some(text);
        pages[1] = Some(data);
        let plan = ProcessImageInstallPlan::for_test_unchecked(
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            PROCESS_INSTALL_BOUNDARY_IDENTITY,
            PHASE8_INIT_PATH,
            0xf4a6_cc15_f4d9_4461,
            image_fixture().entry(),
            image_fixture().memory_footprint(),
            2,
            pages,
            ProcessInstallSideEffects::NONE,
            true,
        );
        let (address_space, _) = address_space_fixture(plan);
        let mut lease_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);

        assert_eq!(
            materialize_process_page_tables(
                image_fixture(),
                plan,
                address_space,
                ProcessMaterializationRequest::DescriptorImageOnly,
                &mut lease_source,
            ),
            Err(PosixError::NotSupported)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);
    }
}
