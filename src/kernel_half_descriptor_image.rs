//! Target-independent kernel-half descriptor-image construction.
//!
//! This module consumes the accepted kernel-half reachability preflight and
//! materialized TTBR0 provenance, then emits an inspectable, non-installed
//! TTBR1 kernel-root descriptor-image record. It does not write translation
//! registers, invalidate live TLB state, allocate an ASID, publish scheduler
//! state, mutate process/descriptor tables, or enter lower EL.

use crate::{
    kernel_half_reachability::{
        KERNEL_HALF_DESCRIPTOR_IMAGE_BLOCKED, KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_POLICY, KernelHalfReachabilityPlan,
    },
    posix::PosixError,
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessPageTableMaterialization,
    },
};

pub(crate) const KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY: &str =
    "phase8-kernel-half-descriptor-image-v1";
pub(crate) const KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY: &str =
    "ttbr1-shared-privileged-kernel-root-descriptor-image-v1";
pub(crate) const TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY: &str =
    "materialized-process-root-provenance";
pub(crate) const TTBR1_OWNED_KERNEL_ROOT_IMAGE: &str = "owned-kernel-root-image";
pub(crate) const KERNEL_HALF_DESCRIPTOR_IMAGE_READY: &str = "descriptor-image-ready";
pub(crate) const KERNEL_HALF_DESCRIPTOR_ROOT_LEASE_OWNER: &str = "model-owned";
pub(crate) const KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_OWNER: &str = "model-owned";
pub(crate) const KERNEL_HALF_DESCRIPTOR_RECORD_COUNT: usize = 12;
const KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KernelHalfDescriptorImageRequest {
    ConstructOnly,
    BadReachabilityPlan,
    LineageMismatch,
    MissingKernelCoverage,
    ForbiddenEl0Access,
    WritableText,
    ExecutableData,
    BadDeviceAttributeIntent,
    OverlappingRange,
    ResourceExhaustion,
    UnsupportedTopology,
    LiveActivationRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorImageToken(u64);

impl KernelHalfDescriptorImageToken {
    pub(crate) const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorImageLeaseSnapshot {
    pub(crate) root_leases: usize,
    pub(crate) table_leases: usize,
    pub(crate) descriptor_records: usize,
    pub(crate) root_releases: usize,
    pub(crate) table_releases: usize,
    pub(crate) descriptor_record_releases: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorImageLeaseSource {
    root_capacity: usize,
    table_capacity: usize,
    descriptor_record_capacity: usize,
    next_token: u64,
    root_leases: usize,
    table_leases: usize,
    descriptor_records: usize,
    root_releases: usize,
    table_releases: usize,
    descriptor_record_releases: usize,
}

impl KernelHalfDescriptorImageLeaseSource {
    pub(crate) const fn with_limits(
        root_capacity: usize,
        table_capacity: usize,
        descriptor_record_capacity: usize,
    ) -> Self {
        Self {
            root_capacity,
            table_capacity,
            descriptor_record_capacity,
            next_token: 1,
            root_leases: 0,
            table_leases: 0,
            descriptor_records: 0,
            root_releases: 0,
            table_releases: 0,
            descriptor_record_releases: 0,
        }
    }

    pub(crate) const fn for_descriptor_image() -> Self {
        Self::with_limits(
            1,
            KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT,
            KERNEL_HALF_DESCRIPTOR_RECORD_COUNT,
        )
    }

    pub(crate) const fn snapshot(self) -> KernelHalfDescriptorImageLeaseSnapshot {
        KernelHalfDescriptorImageLeaseSnapshot {
            root_leases: self.root_leases,
            table_leases: self.table_leases,
            descriptor_records: self.descriptor_records,
            root_releases: self.root_releases,
            table_releases: self.table_releases,
            descriptor_record_releases: self.descriptor_record_releases,
        }
    }

    pub(crate) const fn outstanding_leases(self) -> usize {
        self.root_leases + self.table_leases + self.descriptor_records
    }

    fn next_token(&mut self) -> KernelHalfDescriptorImageToken {
        let token = KernelHalfDescriptorImageToken(self.next_token);
        self.next_token += 1;
        token
    }

    fn lease_root(&mut self) -> Result<KernelHalfDescriptorRootLease, PosixError> {
        if self.root_leases == self.root_capacity {
            return Err(PosixError::NoMemory);
        }
        self.root_leases += 1;
        Ok(KernelHalfDescriptorRootLease {
            token: self.next_token(),
            owner: KERNEL_HALF_DESCRIPTOR_ROOT_LEASE_OWNER,
            released: false,
        })
    }

    fn lease_table(&mut self, level: u8) -> Result<KernelHalfDescriptorTableLease, PosixError> {
        if self.table_leases == self.table_capacity {
            return Err(PosixError::NoMemory);
        }
        self.table_leases += 1;
        Ok(KernelHalfDescriptorTableLease {
            token: self.next_token(),
            owner: KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_OWNER,
            level,
            released: false,
        })
    }

    fn install_descriptor_record(&mut self) -> Result<(), PosixError> {
        if self.descriptor_records == self.descriptor_record_capacity {
            return Err(PosixError::NoMemory);
        }
        self.descriptor_records += 1;
        Ok(())
    }

    fn release_descriptor_record(&mut self) {
        if self.descriptor_records != 0 {
            self.descriptor_records -= 1;
            self.descriptor_record_releases += 1;
        }
    }

    fn release_table(&mut self, lease: &mut KernelHalfDescriptorTableLease) {
        if !lease.released {
            lease.released = true;
            if self.table_leases != 0 {
                self.table_leases -= 1;
            }
            self.table_releases += 1;
        }
    }

    fn release_root(&mut self, lease: &mut KernelHalfDescriptorRootLease) {
        if !lease.released {
            lease.released = true;
            if self.root_leases != 0 {
                self.root_leases -= 1;
            }
            self.root_releases += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorRootLease {
    token: KernelHalfDescriptorImageToken,
    owner: &'static str,
    released: bool,
}

impl KernelHalfDescriptorRootLease {
    pub(crate) const fn token(self) -> KernelHalfDescriptorImageToken {
        self.token
    }

    pub(crate) const fn owner(self) -> &'static str {
        self.owner
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorTableLease {
    token: KernelHalfDescriptorImageToken,
    owner: &'static str,
    level: u8,
    released: bool,
}

impl KernelHalfDescriptorTableLease {
    pub(crate) const fn token(self) -> KernelHalfDescriptorImageToken {
        self.token
    }

    pub(crate) const fn owner(self) -> &'static str {
        self.owner
    }

    pub(crate) const fn level(self) -> u8 {
        self.level
    }

    pub(crate) const fn released(self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KernelHalfDescriptorMemoryKind {
    NormalExecutable,
    NormalReadOnly,
    NormalWritable,
    DeviceMmio,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorRecord {
    label: &'static str,
    virtual_start: u64,
    virtual_end: u64,
    kind: KernelHalfDescriptorMemoryKind,
    normal_memory: bool,
    device_memory: bool,
    inner_shareable: bool,
    access_flag: bool,
    privileged_only: bool,
    writable: bool,
    executable: bool,
    user_access: bool,
    owner: &'static str,
    evidence: &'static str,
}

impl KernelHalfDescriptorRecord {
    pub(crate) const fn label(self) -> &'static str {
        self.label
    }

    pub(crate) const fn kind(self) -> KernelHalfDescriptorMemoryKind {
        self.kind
    }

    pub(crate) const fn privileged_only(self) -> bool {
        self.privileged_only
    }

    pub(crate) const fn writable(self) -> bool {
        self.writable
    }

    pub(crate) const fn executable(self) -> bool {
        self.executable
    }

    pub(crate) const fn user_access(self) -> bool {
        self.user_access
    }

    pub(crate) const fn normal_memory(self) -> bool {
        self.normal_memory
    }

    pub(crate) const fn device_memory(self) -> bool {
        self.device_memory
    }

    pub(crate) const fn inner_shareable(self) -> bool {
        self.inner_shareable
    }

    pub(crate) const fn access_flag(self) -> bool {
        self.access_flag
    }

    pub(crate) const fn owner(self) -> &'static str {
        self.owner
    }

    const fn valid(self) -> bool {
        self.virtual_start < self.virtual_end
            && self.privileged_only
            && !self.user_access
            && self.access_flag
            && ((self.normal_memory && !self.device_memory)
                || (!self.normal_memory && self.device_memory))
            && match self.kind {
                KernelHalfDescriptorMemoryKind::NormalExecutable => {
                    self.normal_memory && self.executable && !self.writable
                }
                KernelHalfDescriptorMemoryKind::NormalReadOnly => {
                    self.normal_memory && !self.executable && !self.writable
                }
                KernelHalfDescriptorMemoryKind::NormalWritable => {
                    self.normal_memory && !self.executable && self.writable
                }
                KernelHalfDescriptorMemoryKind::DeviceMmio => {
                    self.device_memory && !self.executable && self.writable
                }
            }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorCoverage {
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
    runtime_console: bool,
    panic_fault_reporting: bool,
}

impl KernelHalfDescriptorCoverage {
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
        runtime_console: true,
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

    pub(crate) const fn runtime_console(self) -> bool {
        self.runtime_console
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
            && self.runtime_console
            && self.panic_fault_reporting
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorPermissionPolicy {
    text_exec_privileged_only: bool,
    rodata_write: bool,
    data_exec: bool,
    device_normal_memory: bool,
    el0_kernel_access: bool,
    wx_normal_memory: bool,
}

impl KernelHalfDescriptorPermissionPolicy {
    pub(crate) const REQUIRED: Self = Self {
        text_exec_privileged_only: true,
        rodata_write: false,
        data_exec: false,
        device_normal_memory: false,
        el0_kernel_access: false,
        wx_normal_memory: false,
    };

    pub(crate) const fn text_exec_privileged_only(self) -> bool {
        self.text_exec_privileged_only
    }

    pub(crate) const fn rodata_write(self) -> bool {
        self.rodata_write
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

    pub(crate) const fn wx_normal_memory(self) -> bool {
        self.wx_normal_memory
    }

    const fn valid(self) -> bool {
        self.text_exec_privileged_only
            && !self.rodata_write
            && !self.data_exec
            && !self.device_normal_memory
            && !self.el0_kernel_access
            && !self.wx_normal_memory
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorSideEffects {
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

impl KernelHalfDescriptorSideEffects {
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
pub(crate) struct KernelHalfDescriptorImageTeardownReport {
    descriptors_cleared: usize,
    root_released: bool,
    tables_released: usize,
    input_records_owned: bool,
    already_destroyed: bool,
}

impl KernelHalfDescriptorImageTeardownReport {
    pub(crate) const fn descriptors_cleared(self) -> usize {
        self.descriptors_cleared
    }

    pub(crate) const fn root_released(self) -> bool {
        self.root_released
    }

    pub(crate) const fn tables_released(self) -> usize {
        self.tables_released
    }

    pub(crate) const fn input_records_owned(self) -> bool {
        self.input_records_owned
    }

    pub(crate) const fn already_destroyed(self) -> bool {
        self.already_destroyed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelHalfDescriptorImage {
    boundary_identity: &'static str,
    policy_identity: &'static str,
    reachability_boundary_identity: &'static str,
    reachability_policy_identity: &'static str,
    image_fixture_identity: &'static str,
    materialization_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    address_space_id: u64,
    materialization_id: u64,
    entry_pc: u64,
    initial_sp: u64,
    ttbr0_root: &'static str,
    ttbr0_root_token: u64,
    ttbr0_root_physical_frame: u64,
    ttbr0_written: bool,
    ttbr1_root: &'static str,
    ttbr1_written: bool,
    descriptor_image_state: &'static str,
    root_lease: KernelHalfDescriptorRootLease,
    table_leases:
        [Option<KernelHalfDescriptorTableLease>; KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT],
    table_lease_count: usize,
    descriptor_records: [Option<KernelHalfDescriptorRecord>; KERNEL_HALF_DESCRIPTOR_RECORD_COUNT],
    descriptor_record_count: usize,
    coverage: KernelHalfDescriptorCoverage,
    permissions: KernelHalfDescriptorPermissionPolicy,
    tcr_state: &'static str,
    mair_state: &'static str,
    sctlr_state: &'static str,
    asid_state: &'static str,
    tlb_state: &'static str,
    barrier_state: &'static str,
    live_register_sequence_state: &'static str,
    lower_el_eret_state: &'static str,
    scheduler_publication_state: &'static str,
    side_effects: KernelHalfDescriptorSideEffects,
    published: bool,
    destroyed: bool,
}

impl KernelHalfDescriptorImage {
    pub(crate) const fn boundary_identity(self) -> &'static str {
        self.boundary_identity
    }

    pub(crate) const fn policy_identity(self) -> &'static str {
        self.policy_identity
    }

    pub(crate) const fn reachability_boundary_identity(self) -> &'static str {
        self.reachability_boundary_identity
    }

    pub(crate) const fn reachability_policy_identity(self) -> &'static str {
        self.reachability_policy_identity
    }

    pub(crate) const fn image_fixture_identity(self) -> &'static str {
        self.image_fixture_identity
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

    pub(crate) const fn ttbr1_root(self) -> &'static str {
        self.ttbr1_root
    }

    pub(crate) const fn ttbr1_written(self) -> bool {
        self.ttbr1_written
    }

    pub(crate) const fn descriptor_image_state(self) -> &'static str {
        self.descriptor_image_state
    }

    pub(crate) const fn root_lease(self) -> KernelHalfDescriptorRootLease {
        self.root_lease
    }

    pub(crate) const fn table_lease_count(self) -> usize {
        self.table_lease_count
    }

    pub(crate) const fn table_lease(self, index: usize) -> Option<KernelHalfDescriptorTableLease> {
        if index >= KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT {
            None
        } else {
            self.table_leases[index]
        }
    }

    pub(crate) const fn descriptor_record_count(self) -> usize {
        self.descriptor_record_count
    }

    pub(crate) const fn descriptor_record(
        self,
        index: usize,
    ) -> Option<KernelHalfDescriptorRecord> {
        if index >= KERNEL_HALF_DESCRIPTOR_RECORD_COUNT {
            None
        } else {
            self.descriptor_records[index]
        }
    }

    pub(crate) const fn coverage(self) -> KernelHalfDescriptorCoverage {
        self.coverage
    }

    pub(crate) const fn permissions(self) -> KernelHalfDescriptorPermissionPolicy {
        self.permissions
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

    pub(crate) const fn scheduler_publication_state(self) -> &'static str {
        self.scheduler_publication_state
    }

    pub(crate) const fn side_effects(self) -> KernelHalfDescriptorSideEffects {
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
        lease_source: &mut KernelHalfDescriptorImageLeaseSource,
    ) -> KernelHalfDescriptorImageTeardownReport {
        if self.destroyed {
            return KernelHalfDescriptorImageTeardownReport {
                descriptors_cleared: 0,
                root_released: false,
                tables_released: 0,
                input_records_owned: true,
                already_destroyed: true,
            };
        }

        let descriptor_count = self.descriptor_record_count;
        let mut index = 0;
        while index < descriptor_count {
            self.descriptor_records[index] = None;
            lease_source.release_descriptor_record();
            index += 1;
        }
        self.descriptor_record_count = 0;

        let mut tables_released = 0;
        let mut table_index = 0;
        while table_index < self.table_lease_count {
            if let Some(mut lease) = self.table_leases[table_index] {
                lease_source.release_table(&mut lease);
                self.table_leases[table_index] = Some(lease);
                tables_released += 1;
            }
            table_index += 1;
        }

        let mut root = self.root_lease;
        lease_source.release_root(&mut root);
        self.root_lease = root;
        self.published = false;
        self.destroyed = true;

        KernelHalfDescriptorImageTeardownReport {
            descriptors_cleared: descriptor_count,
            root_released: true,
            tables_released,
            input_records_owned: true,
            already_destroyed: false,
        }
    }
}

pub(crate) fn construct_kernel_half_descriptor_image(
    reachability_plan: KernelHalfReachabilityPlan,
    materialization: ProcessPageTableMaterialization,
    request: KernelHalfDescriptorImageRequest,
    lease_source: &mut KernelHalfDescriptorImageLeaseSource,
) -> Result<KernelHalfDescriptorImage, PosixError> {
    match request {
        KernelHalfDescriptorImageRequest::ConstructOnly => {}
        KernelHalfDescriptorImageRequest::BadReachabilityPlan
        | KernelHalfDescriptorImageRequest::LineageMismatch
        | KernelHalfDescriptorImageRequest::MissingKernelCoverage
        | KernelHalfDescriptorImageRequest::OverlappingRange => {
            return Err(PosixError::InvalidArgument);
        }
        KernelHalfDescriptorImageRequest::ForbiddenEl0Access
        | KernelHalfDescriptorImageRequest::WritableText
        | KernelHalfDescriptorImageRequest::ExecutableData
        | KernelHalfDescriptorImageRequest::BadDeviceAttributeIntent => {
            return Err(PosixError::AccessDenied);
        }
        KernelHalfDescriptorImageRequest::ResourceExhaustion => {
            return Err(PosixError::NoMemory);
        }
        KernelHalfDescriptorImageRequest::UnsupportedTopology => {
            return Err(PosixError::NotSupported);
        }
        KernelHalfDescriptorImageRequest::LiveActivationRequest => {
            return Err(PosixError::NotImplemented);
        }
    }

    validate_inputs(reachability_plan, materialization)?;
    let coverage = KernelHalfDescriptorCoverage::REQUIRED;
    if !coverage.all_required() {
        return Err(PosixError::InvalidArgument);
    }
    let permissions = KernelHalfDescriptorPermissionPolicy::REQUIRED;
    if !permissions.valid() {
        return Err(PosixError::AccessDenied);
    }

    let root_lease = lease_source.lease_root()?;
    let mut image = KernelHalfDescriptorImage {
        boundary_identity: KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY,
        policy_identity: KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY,
        reachability_boundary_identity: reachability_plan.boundary_identity(),
        reachability_policy_identity: reachability_plan.policy_identity(),
        image_fixture_identity: reachability_plan.image_fixture_identity(),
        materialization_boundary_identity: materialization.boundary_identity(),
        source_path: reachability_plan.source_path(),
        source_digest: reachability_plan.source_digest(),
        address_space_id: reachability_plan.address_space_id(),
        materialization_id: reachability_plan.materialization_id(),
        entry_pc: reachability_plan.entry_pc(),
        initial_sp: reachability_plan.initial_sp(),
        ttbr0_root: TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY,
        ttbr0_root_token: materialization.root().token().raw(),
        ttbr0_root_physical_frame: materialization.root().physical_frame(),
        ttbr0_written: false,
        ttbr1_root: TTBR1_OWNED_KERNEL_ROOT_IMAGE,
        ttbr1_written: false,
        descriptor_image_state: KERNEL_HALF_DESCRIPTOR_IMAGE_READY,
        root_lease,
        table_leases: [None; KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT],
        table_lease_count: 0,
        descriptor_records: [None; KERNEL_HALF_DESCRIPTOR_RECORD_COUNT],
        descriptor_record_count: 0,
        coverage,
        permissions,
        tcr_state: reachability_plan.tcr_state(),
        mair_state: reachability_plan.mair_state(),
        sctlr_state: reachability_plan.sctlr_state(),
        asid_state: reachability_plan.asid_state(),
        tlb_state: reachability_plan.tlb_state(),
        barrier_state: reachability_plan.barrier_state(),
        live_register_sequence_state: reachability_plan.live_register_sequence_state(),
        lower_el_eret_state: reachability_plan.lower_el_eret_state(),
        scheduler_publication_state: reachability_plan.runnable_publication_state(),
        side_effects: KernelHalfDescriptorSideEffects::NONE,
        published: false,
        destroyed: false,
    };

    let build_result = populate_image_records(&mut image, lease_source);
    if let Err(error) = build_result {
        rollback_unpublished_image(&mut image, lease_source);
        return Err(error);
    }

    image.published = true;
    Ok(image)
}

fn validate_inputs(
    reachability_plan: KernelHalfReachabilityPlan,
    materialization: ProcessPageTableMaterialization,
) -> Result<(), PosixError> {
    if reachability_plan.boundary_identity() != KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        || reachability_plan.policy_identity() != KERNEL_HALF_REACHABILITY_POLICY
        || !reachability_plan.published()
        || reachability_plan.destroyed()
        || reachability_plan.root_policy().descriptor_image()
            != KERNEL_HALF_DESCRIPTOR_IMAGE_BLOCKED
        || reachability_plan.root_policy().ttbr1_written()
        || materialization.boundary_identity()
            != PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        || !materialization.published()
        || materialization.destroyed()
        || !materialization.activation_blocked()
        || materialization.id() != reachability_plan.materialization_id()
        || materialization.source_digest() != reachability_plan.source_digest()
        || materialization.root().token().raw()
            != reachability_plan.root_policy().ttbr0_root_token()
        || materialization.root().physical_frame()
            != reachability_plan.root_policy().ttbr0_root_physical_frame()
    {
        return Err(PosixError::InvalidArgument);
    }

    let reachability = reachability_plan.reachability();
    if !reachability.kernel_text()
        || !reachability.rodata()
        || !reachability.data()
        || !reachability.bss()
        || !reachability.vectors()
        || !reachability.active_stack()
        || !reachability.heap()
        || !reachability.page_frames()
        || !reachability.uart_mmio_diagnostics()
        || !reachability.scheduler_code_data()
        || !reachability.panic_fault_reporting()
    {
        return Err(PosixError::InvalidArgument);
    }

    let permissions = reachability_plan.permission_policy();
    if !permissions.text_exec_privileged_only()
        || permissions.data_exec()
        || permissions.device_normal_memory()
        || permissions.el0_kernel_access()
    {
        return Err(PosixError::AccessDenied);
    }

    let effects = reachability_plan.side_effects();
    if effects.ttbr_mutated()
        || effects.tcr_mutated()
        || effects.mair_mutated()
        || effects.sctlr_mutated()
        || effects.descriptor_image_installed()
        || effects.asid_allocated()
        || effects.tlb_mutated()
        || effects.live_dsb_isb()
        || effects.lower_el_eret()
        || effects.scheduler_published()
        || effects.process_table_mutated()
        || effects.descriptor_table_mutated()
    {
        return Err(PosixError::NotImplemented);
    }

    Ok(())
}

fn populate_image_records(
    image: &mut KernelHalfDescriptorImage,
    lease_source: &mut KernelHalfDescriptorImageLeaseSource,
) -> Result<(), PosixError> {
    let mut index = 0;
    while index < KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT {
        let lease = lease_source.lease_table((index + 1) as u8)?;
        image.table_leases[index] = Some(lease);
        image.table_lease_count += 1;
        index += 1;
    }

    for record in required_descriptor_records() {
        if !record.valid() {
            return Err(PosixError::InvalidArgument);
        }
        lease_source.install_descriptor_record()?;
        image.descriptor_records[image.descriptor_record_count] = Some(record);
        image.descriptor_record_count += 1;
    }
    Ok(())
}

fn rollback_unpublished_image(
    image: &mut KernelHalfDescriptorImage,
    lease_source: &mut KernelHalfDescriptorImageLeaseSource,
) {
    while image.descriptor_record_count != 0 {
        image.descriptor_record_count -= 1;
        image.descriptor_records[image.descriptor_record_count] = None;
        lease_source.release_descriptor_record();
    }

    while image.table_lease_count != 0 {
        image.table_lease_count -= 1;
        if let Some(mut lease) = image.table_leases[image.table_lease_count] {
            lease_source.release_table(&mut lease);
            image.table_leases[image.table_lease_count] = Some(lease);
        }
    }

    let mut root = image.root_lease;
    lease_source.release_root(&mut root);
    image.root_lease = root;
    image.published = false;
    image.destroyed = true;
}

const fn normal_exec(label: &'static str, index: u64) -> KernelHalfDescriptorRecord {
    descriptor(
        label,
        index,
        KernelHalfDescriptorMemoryKind::NormalExecutable,
        false,
        true,
        true,
    )
}

const fn normal_ro(label: &'static str, index: u64) -> KernelHalfDescriptorRecord {
    descriptor(
        label,
        index,
        KernelHalfDescriptorMemoryKind::NormalReadOnly,
        false,
        false,
        true,
    )
}

const fn normal_rw(label: &'static str, index: u64) -> KernelHalfDescriptorRecord {
    descriptor(
        label,
        index,
        KernelHalfDescriptorMemoryKind::NormalWritable,
        true,
        false,
        true,
    )
}

const fn device(label: &'static str, index: u64) -> KernelHalfDescriptorRecord {
    descriptor(
        label,
        index,
        KernelHalfDescriptorMemoryKind::DeviceMmio,
        true,
        false,
        false,
    )
}

const fn descriptor(
    label: &'static str,
    index: u64,
    kind: KernelHalfDescriptorMemoryKind,
    writable: bool,
    executable: bool,
    normal_memory: bool,
) -> KernelHalfDescriptorRecord {
    let start = 0xffff_0000_0000_0000 + (index << 21);
    KernelHalfDescriptorRecord {
        label,
        virtual_start: start,
        virtual_end: start + (1 << 21),
        kind,
        normal_memory,
        device_memory: !normal_memory,
        inner_shareable: normal_memory,
        access_flag: true,
        privileged_only: true,
        writable,
        executable,
        user_access: false,
        owner: "kernel-half-descriptor-image",
        evidence: "phase8-kernel-half-descriptor-image-contract",
    }
}

const fn required_descriptor_records()
-> [KernelHalfDescriptorRecord; KERNEL_HALF_DESCRIPTOR_RECORD_COUNT] {
    [
        normal_exec("kernel-text", 0),
        normal_ro("rodata", 1),
        normal_rw("data", 2),
        normal_rw("bss", 3),
        normal_exec("vectors", 4),
        normal_rw("active-stack", 5),
        normal_rw("heap", 6),
        normal_rw("page-frames", 7),
        device("uart-mmio-diagnostics", 8),
        normal_rw("scheduler-code-data", 9),
        normal_rw("runtime-console", 10),
        normal_rw("panic-fault-reporting", 11),
    ]
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
        kernel_half_reachability::{
            KernelHalfReachabilityLeaseSource, KernelHalfReachabilityRequest,
            preflight_kernel_half_reachability,
        },
        live_address_space_activation::{
            ASID_ALLOCATION_BLOCKED, BARRIER_SEQUENCE_PLANNED_ONLY,
            LiveAddressSpaceActivationLeaseSource, LiveAddressSpaceActivationRequest,
            SCTLR_MUTATION_BLOCKED, TLB_INVALIDATION_BLOCKED,
            preflight_live_address_space_activation,
        },
        process_address_space::{
            ProcessAddressSpace, ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource,
            install_process_address_space,
        },
        process_install::{ProcessImageInstallPlan, plan_process_image_install},
        process_page_table_materialization::{
            ProcessMaterializationRequest, ProcessPageTableMaterializationLeaseSource,
            materialize_process_page_tables,
        },
        program_loader::{ProgramImagePlan, plan_phase8_init_image},
        scheduler::ProcessOwnerId,
    };

    fn fixture() -> (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        KernelHalfReachabilityPlan,
    ) {
        let image =
            plan_phase8_init_image(phase8_readonly_initramfs_fixture()).expect("program image");
        let install_plan = plan_process_image_install(image).expect("install plan");
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(0x8800_7001).expect("address-space id"),
            Some(ProcessOwnerId::new(0x8800_7002).expect("owner id")),
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

        (
            image,
            install_plan,
            address_space,
            materialization,
            reachability_plan,
        )
    }

    #[test_case]
    fn constructs_non_installed_descriptor_image_with_copied_lineage() {
        let (image, _install_plan, address_space, materialization, reachability_plan) = fixture();
        let mut lease_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();

        let descriptor_image = construct_kernel_half_descriptor_image(
            reachability_plan,
            materialization,
            KernelHalfDescriptorImageRequest::ConstructOnly,
            &mut lease_source,
        )
        .expect("descriptor image");

        assert_eq!(
            descriptor_image.boundary_identity(),
            KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY
        );
        assert_eq!(
            descriptor_image.policy_identity(),
            KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY
        );
        assert_eq!(
            descriptor_image.reachability_boundary_identity(),
            KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        );
        assert_eq!(
            descriptor_image.reachability_policy_identity(),
            KERNEL_HALF_REACHABILITY_POLICY
        );
        assert_eq!(
            descriptor_image.image_fixture_identity(),
            reachability_plan.image_fixture_identity()
        );
        assert_eq!(
            descriptor_image.materialization_boundary_identity(),
            PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        );
        assert_eq!(descriptor_image.source_path(), PHASE8_INIT_PATH);
        assert_eq!(descriptor_image.source_digest(), image.source_digest());
        assert_eq!(
            descriptor_image.address_space_id(),
            address_space.id().raw()
        );
        assert_eq!(descriptor_image.materialization_id(), materialization.id());
        assert_eq!(descriptor_image.entry_pc(), image.entry());
        assert_eq!(
            descriptor_image.initial_sp(),
            reachability_plan.initial_sp()
        );
        assert!(descriptor_image.published());
        assert!(!descriptor_image.destroyed());
    }

    #[test_case]
    fn records_ttbr1_image_intent_without_live_side_effects() {
        let (_image, _install_plan, _address_space, materialization, reachability_plan) = fixture();
        let mut lease_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();

        let descriptor_image = construct_kernel_half_descriptor_image(
            reachability_plan,
            materialization,
            KernelHalfDescriptorImageRequest::ConstructOnly,
            &mut lease_source,
        )
        .expect("descriptor image");

        assert_eq!(
            descriptor_image.ttbr0_root(),
            TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY
        );
        assert_eq!(
            descriptor_image.ttbr0_root_token(),
            materialization.root().token().raw()
        );
        assert_eq!(
            descriptor_image.ttbr0_root_physical_frame(),
            materialization.root().physical_frame()
        );
        assert!(!descriptor_image.ttbr0_written());
        assert_eq!(descriptor_image.ttbr1_root(), TTBR1_OWNED_KERNEL_ROOT_IMAGE);
        assert!(!descriptor_image.ttbr1_written());
        assert_eq!(
            descriptor_image.descriptor_image_state(),
            KERNEL_HALF_DESCRIPTOR_IMAGE_READY
        );
        assert_eq!(
            descriptor_image.root_lease().owner(),
            KERNEL_HALF_DESCRIPTOR_ROOT_LEASE_OWNER
        );
        assert_ne!(descriptor_image.root_lease().token().raw(), 0);
        assert!(!descriptor_image.root_lease().released());
        assert_eq!(
            descriptor_image.table_lease_count(),
            KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT
        );
        let mut table_index = 0;
        while table_index < descriptor_image.table_lease_count() {
            let table = descriptor_image
                .table_lease(table_index)
                .expect("table lease");
            assert_ne!(table.token().raw(), 0);
            assert_eq!(table.owner(), KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_OWNER);
            assert_eq!(table.level(), (table_index + 1) as u8);
            assert!(!table.released());
            table_index += 1;
        }
        assert_eq!(
            descriptor_image.descriptor_record_count(),
            KERNEL_HALF_DESCRIPTOR_RECORD_COUNT
        );
        assert_eq!(lease_source.outstanding_leases(), 16);

        let effects = descriptor_image.side_effects();
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
    fn records_required_coverage_permissions_attributes_and_blockers() {
        let (_image, _install_plan, _address_space, materialization, reachability_plan) = fixture();
        let mut lease_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();

        let descriptor_image = construct_kernel_half_descriptor_image(
            reachability_plan,
            materialization,
            KernelHalfDescriptorImageRequest::ConstructOnly,
            &mut lease_source,
        )
        .expect("descriptor image");

        let coverage = descriptor_image.coverage();
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

        let permissions = descriptor_image.permissions();
        assert!(permissions.text_exec_privileged_only());
        assert!(!permissions.rodata_write());
        assert!(!permissions.data_exec());
        assert!(!permissions.device_normal_memory());
        assert!(!permissions.el0_kernel_access());
        assert!(!permissions.wx_normal_memory());

        assert_eq!(descriptor_image.tcr_state(), reachability_plan.tcr_state());
        assert_eq!(
            descriptor_image.mair_state(),
            reachability_plan.mair_state()
        );
        assert_eq!(descriptor_image.sctlr_state(), SCTLR_MUTATION_BLOCKED);
        assert_eq!(descriptor_image.asid_state(), ASID_ALLOCATION_BLOCKED);
        assert_eq!(descriptor_image.tlb_state(), TLB_INVALIDATION_BLOCKED);
        assert_eq!(
            descriptor_image.barrier_state(),
            BARRIER_SEQUENCE_PLANNED_ONLY
        );
        assert_eq!(
            descriptor_image.live_register_sequence_state(),
            reachability_plan.live_register_sequence_state()
        );
        assert_eq!(
            descriptor_image.lower_el_eret_state(),
            reachability_plan.lower_el_eret_state()
        );
        assert_eq!(
            descriptor_image.scheduler_publication_state(),
            reachability_plan.runnable_publication_state()
        );

        let mut index = 0;
        while index < descriptor_image.descriptor_record_count() {
            let record = descriptor_image
                .descriptor_record(index)
                .expect("descriptor record");
            assert!(record.valid());
            assert!(record.privileged_only());
            assert!(!record.user_access());
            assert!(record.access_flag());
            assert_eq!(record.owner(), "kernel-half-descriptor-image");
            index += 1;
        }
        let text = descriptor_image.descriptor_record(0).expect("text");
        assert_eq!(text.label(), "kernel-text");
        assert_eq!(
            text.kind(),
            KernelHalfDescriptorMemoryKind::NormalExecutable
        );
        assert!(text.normal_memory());
        assert!(text.inner_shareable());
        assert!(text.executable());
        assert!(!text.writable());

        let mmio = descriptor_image.descriptor_record(8).expect("mmio");
        assert_eq!(mmio.kind(), KernelHalfDescriptorMemoryKind::DeviceMmio);
        assert!(mmio.device_memory());
        assert!(!mmio.normal_memory());
        assert!(!mmio.inner_shareable());
        assert!(!mmio.executable());
        assert!(mmio.writable());
    }

    #[test_case]
    fn rejects_blocked_requests_without_partial_image() {
        let (_image, _install_plan, _address_space, materialization, reachability_plan) = fixture();

        for (request, expected) in [
            (
                KernelHalfDescriptorImageRequest::BadReachabilityPlan,
                PosixError::InvalidArgument,
            ),
            (
                KernelHalfDescriptorImageRequest::LineageMismatch,
                PosixError::InvalidArgument,
            ),
            (
                KernelHalfDescriptorImageRequest::MissingKernelCoverage,
                PosixError::InvalidArgument,
            ),
            (
                KernelHalfDescriptorImageRequest::ForbiddenEl0Access,
                PosixError::AccessDenied,
            ),
            (
                KernelHalfDescriptorImageRequest::WritableText,
                PosixError::AccessDenied,
            ),
            (
                KernelHalfDescriptorImageRequest::ExecutableData,
                PosixError::AccessDenied,
            ),
            (
                KernelHalfDescriptorImageRequest::BadDeviceAttributeIntent,
                PosixError::AccessDenied,
            ),
            (
                KernelHalfDescriptorImageRequest::OverlappingRange,
                PosixError::InvalidArgument,
            ),
            (
                KernelHalfDescriptorImageRequest::ResourceExhaustion,
                PosixError::NoMemory,
            ),
            (
                KernelHalfDescriptorImageRequest::UnsupportedTopology,
                PosixError::NotSupported,
            ),
            (
                KernelHalfDescriptorImageRequest::LiveActivationRequest,
                PosixError::NotImplemented,
            ),
        ] {
            let mut lease_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
            assert_eq!(
                construct_kernel_half_descriptor_image(
                    reachability_plan,
                    materialization,
                    request,
                    &mut lease_source,
                ),
                Err(expected)
            );
            assert_eq!(lease_source.outstanding_leases(), 0);
        }
    }

    #[test_case]
    fn rolls_back_lease_shortage_without_partial_image() {
        let (_image, _install_plan, _address_space, materialization, reachability_plan) = fixture();
        let mut lease_source = KernelHalfDescriptorImageLeaseSource::with_limits(1, 1, 1);

        assert_eq!(
            construct_kernel_half_descriptor_image(
                reachability_plan,
                materialization,
                KernelHalfDescriptorImageRequest::ConstructOnly,
                &mut lease_source,
            ),
            Err(PosixError::NoMemory)
        );
        assert_eq!(lease_source.outstanding_leases(), 0);
        assert_eq!(lease_source.snapshot().root_releases, 1);
        assert_eq!(lease_source.snapshot().table_releases, 1);
        assert_eq!(lease_source.snapshot().descriptor_record_releases, 0);
    }

    #[test_case]
    fn teardown_releases_model_owned_records_and_is_idempotent() {
        let (_image, _install_plan, _address_space, materialization, reachability_plan) = fixture();
        let mut lease_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
        let mut descriptor_image = construct_kernel_half_descriptor_image(
            reachability_plan,
            materialization,
            KernelHalfDescriptorImageRequest::ConstructOnly,
            &mut lease_source,
        )
        .expect("descriptor image");

        let first = descriptor_image.destroy(&mut lease_source);
        assert_eq!(
            first.descriptors_cleared(),
            KERNEL_HALF_DESCRIPTOR_RECORD_COUNT
        );
        assert!(first.root_released());
        assert_eq!(
            first.tables_released(),
            KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT
        );
        assert!(first.input_records_owned());
        assert!(!first.already_destroyed());
        assert!(!descriptor_image.published());
        assert!(descriptor_image.destroyed());
        assert_eq!(lease_source.outstanding_leases(), 0);

        let second = descriptor_image.destroy(&mut lease_source);
        assert_eq!(second.descriptors_cleared(), 0);
        assert!(!second.root_released());
        assert_eq!(second.tables_released(), 0);
        assert!(second.input_records_owned());
        assert!(second.already_destroyed());
        assert_eq!(lease_source.snapshot().root_releases, 1);
        assert_eq!(
            lease_source.snapshot().table_releases,
            KERNEL_HALF_DESCRIPTOR_TABLE_LEASE_COUNT
        );
        assert_eq!(
            lease_source.snapshot().descriptor_record_releases,
            KERNEL_HALF_DESCRIPTOR_RECORD_COUNT
        );
    }
}
