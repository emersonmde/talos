use crate::memory_map::EarlyPageFrameSpan;

pub const DMA_CACHE_SUBSTRATE_CONTRACT_ID: &str = "phase11-rp1-dma-cache-substrate-contract-v1";
pub const DMA_CACHE_SYNC_PLAN_CONTRACT_ID: &str = "phase11-rp1-dma-cache-sync-plan-contract-v1";
pub const DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID: &str =
    "phase11-rp1-dma-cache-maintenance-sequence-contract-v1";
pub const DMA_CACHE_SOURCE_INVENTORY_ID: &str = "phase11-rp1-dma-cache-source-inventory-20260609";
pub const DMA_LOCAL_STATIC_CLASSIFICATION: &str = "local-static-dma-cache-contract-visible";
pub const DMA_SYNC_PLAN_LOCAL_STATIC_CLASSIFICATION: &str =
    "local-static-dma-cache-sync-plan-visible";
pub const DMA_MAINTENANCE_SEQUENCE_LOCAL_STATIC_CLASSIFICATION: &str =
    "local-static-dma-cache-maintenance-sequence-visible";
pub const DMA_REJECTED_INPUT_CLASSIFICATION: &str = "contract-rejected-input";
pub const DMA_STAGING_BLOCKER_CLASSIFICATION: &str = "staging/build-blocker";
pub const RP1_DMA_SOURCE_UNASSIGNED_IOMMU: &str = "source-unassigned-rp1-dma";
pub const BCM2712_CACHE_LINE_SOURCE: &str = "bcm2712-dcache-l2-cache-line-size";
pub const BCM2712_DMA_CACHE_LINE_SIZE: u64 = 64;

pub const DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "executed-driver-buffer-cache-maintenance",
    "live-barrier-ordering",
    "rp1-mmio-writes",
    "dma-channel-programming",
    "descriptor-rings",
    "ethernet-storage-networking-ssh",
    "milestone-11-3-completion",
];

pub const RP1_RAM_WINDOW_SOURCE: &str = "rp1-dma-ranges-ram-window";
pub const RP1_RAM_WINDOW_BASE: u64 = 0x10_0000_0000;
pub const RP1_RAM_WINDOW_CPU_BASE: u64 = 0x0;
pub const RP1_RAM_WINDOW_SIZE: u64 = 0x10_0000_0000;

pub const RP1_PERIPHERAL_WINDOW_SOURCE: &str = "rp1-dma-ranges-peripheral-window";
pub const RP1_PERIPHERAL_WINDOW_BASE: u64 = 0xc0_4000_0000;
pub const RP1_PERIPHERAL_WINDOW_CPU_BASE: u64 = 0x1f_0000_0000;
pub const RP1_PERIPHERAL_WINDOW_SIZE: u64 = 0x0041_0000;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaDirection {
    ToDevice,
    FromDevice,
    Bidirectional,
}

impl DmaDirection {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ToDevice => "to-device",
            Self::FromDevice => "from-device",
            Self::Bidirectional => "bidirectional",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheability {
    CacheableRequiresMaintenance,
    CoherentHardwareUnaccepted,
    NonCacheableMappingUnaccepted,
}

impl DmaCacheability {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CacheableRequiresMaintenance => "cacheable-requires-maintenance",
            Self::CoherentHardwareUnaccepted => "coherent-hardware-unaccepted",
            Self::NonCacheableMappingUnaccepted => "non-cacheable-mapping-unaccepted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaAddressPath {
    Rp1RamWindow,
    Rp1PeripheralWindow,
}

impl DmaAddressPath {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Rp1RamWindow => "rp1-ram-window",
            Self::Rp1PeripheralWindow => "rp1-peripheral-window",
        }
    }

    pub const fn source_range(self) -> &'static str {
        match self {
            Self::Rp1RamWindow => RP1_RAM_WINDOW_SOURCE,
            Self::Rp1PeripheralWindow => RP1_PERIPHERAL_WINDOW_SOURCE,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaBufferOwner {
    CpuOwned,
    DeviceOwned,
    SharedSynchronizationBoundary,
}

impl DmaBufferOwner {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CpuOwned => "cpu-owned",
            Self::DeviceOwned => "device-owned",
            Self::SharedSynchronizationBoundary => "shared-synchronization-boundary",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaIommuClassification {
    SourceUnassignedRp1Dma,
    Iommu5DisplayCameraOnlyUnaccepted,
    UnknownUnaccepted,
}

impl DmaIommuClassification {
    pub const fn name(self) -> &'static str {
        match self {
            Self::SourceUnassignedRp1Dma => RP1_DMA_SOURCE_UNASSIGNED_IOMMU,
            Self::Iommu5DisplayCameraOnlyUnaccepted => "iommu5-display-camera-only-unaccepted",
            Self::UnknownUnaccepted => "unknown-iommu-unaccepted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaBufferRequest {
    pub cpu_physical: u64,
    pub cpu_visible: u64,
    pub length: u64,
    pub alignment: u64,
    pub direction: DmaDirection,
    pub cacheability: DmaCacheability,
    pub address_path: DmaAddressPath,
    pub owner: DmaBufferOwner,
    pub iommu: DmaIommuClassification,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaBufferDescriptor {
    pub cpu_physical: u64,
    pub cpu_visible: u64,
    pub rp1_bus_address: u64,
    pub length: u64,
    pub alignment: u64,
    pub direction: DmaDirection,
    pub cacheability: DmaCacheability,
    pub address_path: DmaAddressPath,
    pub owner: DmaBufferOwner,
    pub iommu: DmaIommuClassification,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaValidationError {
    ZeroLength,
    InvalidAlignment,
    UnalignedCpuAddress,
    UnalignedLength,
    AddressOverflow,
    OwnershipSpanContainment,
    ForbiddenHighMemory,
    ForbiddenReservedMemory,
    TranslationRange,
    TranslationOverflow,
    ForbiddenCacheabilityClaim,
    ForbiddenIommuClaim,
}

impl DmaValidationError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ZeroLength => "zero-length",
            Self::InvalidAlignment => "invalid-alignment",
            Self::UnalignedCpuAddress => "unaligned-cpu-address",
            Self::UnalignedLength => "unaligned-length",
            Self::AddressOverflow => "address-overflow",
            Self::OwnershipSpanContainment => "ownership-span-containment",
            Self::ForbiddenHighMemory => "forbidden-high-memory",
            Self::ForbiddenReservedMemory => "forbidden-reserved-memory",
            Self::TranslationRange => "translation-range",
            Self::TranslationOverflow => "translation-overflow",
            Self::ForbiddenCacheabilityClaim => "forbidden-cacheability-claim",
            Self::ForbiddenIommuClaim => "forbidden-iommu-claim",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaValidationResults {
    pub alignment: bool,
    pub ownership_span_containment: bool,
    pub translation_range: bool,
    pub forbidden_claims: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaDescriptorEvidence {
    pub contract_id: &'static str,
    pub source_inventory_id: &'static str,
    pub address_path: &'static str,
    pub address_path_source: &'static str,
    pub cpu_physical: u64,
    pub cpu_visible: u64,
    pub rp1_bus_address: u64,
    pub length: u64,
    pub alignment: u64,
    pub direction: &'static str,
    pub cacheability: &'static str,
    pub owner: &'static str,
    pub iommu_classification: &'static str,
    pub validation: DmaValidationResults,
    pub classification: &'static str,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheSyncBoundary {
    BeforeDeviceOwnership,
    AfterDeviceOwnership,
    SharedSynchronizationBoundary,
}

impl DmaCacheSyncBoundary {
    pub const fn name(self) -> &'static str {
        match self {
            Self::BeforeDeviceOwnership => "before-device-ownership",
            Self::AfterDeviceOwnership => "after-device-ownership",
            Self::SharedSynchronizationBoundary => "shared-synchronization-boundary",
        }
    }

    pub const fn owner_transition(self) -> &'static str {
        match self {
            Self::BeforeDeviceOwnership => "cpu-to-device",
            Self::AfterDeviceOwnership => "device-to-cpu",
            Self::SharedSynchronizationBoundary => "shared-cpu-device",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheSyncOperation {
    CleanToPointOfCoherency,
    InvalidateFromPointOfCoherency,
    CleanInvalidateToPointOfCoherency,
}

impl DmaCacheSyncOperation {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CleanToPointOfCoherency => "clean-to-point-of-coherency",
            Self::InvalidateFromPointOfCoherency => "invalidate-from-point-of-coherency",
            Self::CleanInvalidateToPointOfCoherency => "clean-invalidate-to-point-of-coherency",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheSyncPlanError {
    ZeroLength,
    RangeOverflow,
    UnsupportedCacheabilityClaim,
    UnsupportedIommuClaim,
    UnsupportedDirectionBoundary,
    NonAcceptedDescriptorClassification,
    DescriptorEvidenceMismatch,
}

impl DmaCacheSyncPlanError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ZeroLength => "zero-length",
            Self::RangeOverflow => "range-overflow",
            Self::UnsupportedCacheabilityClaim => "unsupported-cacheability-claim",
            Self::UnsupportedIommuClaim => "unsupported-iommu-claim",
            Self::UnsupportedDirectionBoundary => "unsupported-direction-boundary",
            Self::NonAcceptedDescriptorClassification => "non-accepted-descriptor-classification",
            Self::DescriptorEvidenceMismatch => "descriptor-evidence-mismatch",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSyncPlan {
    pub descriptor: DmaBufferDescriptor,
    pub boundary: DmaCacheSyncBoundary,
    pub operation: DmaCacheSyncOperation,
    pub cache_line_size: u64,
    pub line_aligned_cpu_start: u64,
    pub covered_length: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSyncPlanEvidence {
    pub sync_plan_contract_id: &'static str,
    pub descriptor_contract_id: &'static str,
    pub descriptor_source_inventory_id: &'static str,
    pub sync_boundary: &'static str,
    pub operation: &'static str,
    pub cache_line_source: &'static str,
    pub cache_line_size: u64,
    pub line_aligned_cpu_start: u64,
    pub covered_length: u64,
    pub cpu_physical: u64,
    pub cpu_visible: u64,
    pub rp1_bus_address: u64,
    pub descriptor_length: u64,
    pub direction: &'static str,
    pub cacheability: &'static str,
    pub owner_transition: &'static str,
    pub iommu_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub classification: &'static str,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheMaintenanceInstruction {
    CleanByVirtualAddressToPoC,
    InvalidateByVirtualAddressFromPoC,
    CleanInvalidateByVirtualAddressToPoC,
}

impl DmaCacheMaintenanceInstruction {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CleanByVirtualAddressToPoC => "clean-by-virtual-address-to-poc",
            Self::InvalidateByVirtualAddressFromPoC => "invalidate-by-virtual-address-from-poc",
            Self::CleanInvalidateByVirtualAddressToPoC => {
                "clean-invalidate-by-virtual-address-to-poc"
            }
        }
    }

    pub const fn instruction(self) -> &'static str {
        match self {
            Self::CleanByVirtualAddressToPoC => "dc cvac",
            Self::InvalidateByVirtualAddressFromPoC => "dc ivac",
            Self::CleanInvalidateByVirtualAddressToPoC => "dc civac",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheMaintenanceBarrier {
    DataSynchronizationBarrierSy,
}

impl DmaCacheMaintenanceBarrier {
    pub const fn name(self) -> &'static str {
        match self {
            Self::DataSynchronizationBarrierSy => "data-synchronization-barrier-sy",
        }
    }

    pub const fn instruction(self) -> &'static str {
        match self {
            Self::DataSynchronizationBarrierSy => "dsb sy",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheMaintenanceSequenceError {
    ZeroCoveredLength,
    CacheLineMismatch,
    RangeOverflow,
    NonAcceptedSyncPlanClassification,
    DescriptorEvidenceMismatch,
    UnsupportedRuntimeClaims,
    UnsupportedSyncOperation,
}

impl DmaCacheMaintenanceSequenceError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ZeroCoveredLength => "zero-covered-length",
            Self::CacheLineMismatch => "cache-line-mismatch",
            Self::RangeOverflow => "range-overflow",
            Self::NonAcceptedSyncPlanClassification => "non-accepted-sync-plan-classification",
            Self::DescriptorEvidenceMismatch => "descriptor-evidence-mismatch",
            Self::UnsupportedRuntimeClaims => "unsupported-runtime-claims",
            Self::UnsupportedSyncOperation => "unsupported-sync-operation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheMaintenanceSequence {
    pub sync_plan: DmaCacheSyncPlanEvidence,
    pub instruction: DmaCacheMaintenanceInstruction,
    pub barrier: DmaCacheMaintenanceBarrier,
    pub line_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheMaintenanceSequenceEvidence {
    pub maintenance_sequence_contract_id: &'static str,
    pub sync_plan_contract_id: &'static str,
    pub descriptor_contract_id: &'static str,
    pub descriptor_source_inventory_id: &'static str,
    pub operation: &'static str,
    pub instruction: &'static str,
    pub instruction_mnemonic: &'static str,
    pub barrier: &'static str,
    pub barrier_mnemonic: &'static str,
    pub cache_line_source: &'static str,
    pub cache_line_size: u64,
    pub line_aligned_cpu_start: u64,
    pub covered_length: u64,
    pub line_count: u64,
    pub cpu_physical: u64,
    pub cpu_visible: u64,
    pub rp1_bus_address: u64,
    pub descriptor_length: u64,
    pub direction: &'static str,
    pub cacheability: &'static str,
    pub owner_transition: &'static str,
    pub iommu_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub classification: &'static str,
}

pub fn validate_dma_buffer_descriptor(
    request: DmaBufferRequest,
    accepted_owned_span: EarlyPageFrameSpan,
) -> Result<DmaBufferDescriptor, DmaValidationError> {
    validate_alignment(request.cpu_physical, request.length, request.alignment)?;
    validate_claims(request.cacheability, request.iommu)?;
    validate_owned_span(request.cpu_physical, request.length, accepted_owned_span)?;
    let rp1_bus_address =
        translate_rp1_bus_address(request.address_path, request.cpu_physical, request.length)?;

    Ok(DmaBufferDescriptor {
        cpu_physical: request.cpu_physical,
        cpu_visible: request.cpu_visible,
        rp1_bus_address,
        length: request.length,
        alignment: request.alignment,
        direction: request.direction,
        cacheability: request.cacheability,
        address_path: request.address_path,
        owner: request.owner,
        iommu: request.iommu,
    })
}

pub fn dma_descriptor_evidence(descriptor: DmaBufferDescriptor) -> DmaDescriptorEvidence {
    DmaDescriptorEvidence {
        contract_id: DMA_CACHE_SUBSTRATE_CONTRACT_ID,
        source_inventory_id: DMA_CACHE_SOURCE_INVENTORY_ID,
        address_path: descriptor.address_path.name(),
        address_path_source: descriptor.address_path.source_range(),
        cpu_physical: descriptor.cpu_physical,
        cpu_visible: descriptor.cpu_visible,
        rp1_bus_address: descriptor.rp1_bus_address,
        length: descriptor.length,
        alignment: descriptor.alignment,
        direction: descriptor.direction.name(),
        cacheability: descriptor.cacheability.name(),
        owner: descriptor.owner.name(),
        iommu_classification: descriptor.iommu.name(),
        validation: DmaValidationResults {
            alignment: true,
            ownership_span_containment: true,
            translation_range: true,
            forbidden_claims: true,
        },
        classification: DMA_LOCAL_STATIC_CLASSIFICATION,
    }
}

pub fn rejected_dma_input_evidence(error: DmaValidationError) -> (&'static str, &'static str) {
    (DMA_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn plan_dma_cache_sync(
    descriptor: DmaBufferDescriptor,
    descriptor_evidence: DmaDescriptorEvidence,
    boundary: DmaCacheSyncBoundary,
) -> Result<DmaCacheSyncPlan, DmaCacheSyncPlanError> {
    validate_accepted_descriptor_evidence(descriptor, descriptor_evidence)?;
    if descriptor.length == 0 {
        return Err(DmaCacheSyncPlanError::ZeroLength);
    }
    if descriptor.cacheability != DmaCacheability::CacheableRequiresMaintenance {
        return Err(DmaCacheSyncPlanError::UnsupportedCacheabilityClaim);
    }
    if descriptor.iommu != DmaIommuClassification::SourceUnassignedRp1Dma {
        return Err(DmaCacheSyncPlanError::UnsupportedIommuClaim);
    }

    let operation = derive_dma_cache_sync_operation(descriptor.direction, boundary)?;
    let line_aligned_cpu_start = align_down(descriptor.cpu_visible, BCM2712_DMA_CACHE_LINE_SIZE);
    let descriptor_end = descriptor
        .cpu_visible
        .checked_add(descriptor.length)
        .ok_or(DmaCacheSyncPlanError::RangeOverflow)?;
    let line_aligned_cpu_end = align_up(descriptor_end, BCM2712_DMA_CACHE_LINE_SIZE)?;
    let covered_length = line_aligned_cpu_end
        .checked_sub(line_aligned_cpu_start)
        .ok_or(DmaCacheSyncPlanError::RangeOverflow)?;

    Ok(DmaCacheSyncPlan {
        descriptor,
        boundary,
        operation,
        cache_line_size: BCM2712_DMA_CACHE_LINE_SIZE,
        line_aligned_cpu_start,
        covered_length,
    })
}

pub fn derive_dma_cache_sync_operation(
    direction: DmaDirection,
    boundary: DmaCacheSyncBoundary,
) -> Result<DmaCacheSyncOperation, DmaCacheSyncPlanError> {
    match (direction, boundary) {
        (DmaDirection::ToDevice, DmaCacheSyncBoundary::BeforeDeviceOwnership) => {
            Ok(DmaCacheSyncOperation::CleanToPointOfCoherency)
        }
        (DmaDirection::FromDevice, DmaCacheSyncBoundary::AfterDeviceOwnership) => {
            Ok(DmaCacheSyncOperation::InvalidateFromPointOfCoherency)
        }
        (DmaDirection::Bidirectional, DmaCacheSyncBoundary::SharedSynchronizationBoundary) => {
            Ok(DmaCacheSyncOperation::CleanInvalidateToPointOfCoherency)
        }
        _ => Err(DmaCacheSyncPlanError::UnsupportedDirectionBoundary),
    }
}

pub fn dma_cache_sync_plan_evidence(plan: DmaCacheSyncPlan) -> DmaCacheSyncPlanEvidence {
    DmaCacheSyncPlanEvidence {
        sync_plan_contract_id: DMA_CACHE_SYNC_PLAN_CONTRACT_ID,
        descriptor_contract_id: DMA_CACHE_SUBSTRATE_CONTRACT_ID,
        descriptor_source_inventory_id: DMA_CACHE_SOURCE_INVENTORY_ID,
        sync_boundary: plan.boundary.name(),
        operation: plan.operation.name(),
        cache_line_source: BCM2712_CACHE_LINE_SOURCE,
        cache_line_size: plan.cache_line_size,
        line_aligned_cpu_start: plan.line_aligned_cpu_start,
        covered_length: plan.covered_length,
        cpu_physical: plan.descriptor.cpu_physical,
        cpu_visible: plan.descriptor.cpu_visible,
        rp1_bus_address: plan.descriptor.rp1_bus_address,
        descriptor_length: plan.descriptor.length,
        direction: plan.descriptor.direction.name(),
        cacheability: plan.descriptor.cacheability.name(),
        owner_transition: plan.boundary.owner_transition(),
        iommu_classification: plan.descriptor.iommu.name(),
        rejected_runtime_claims: DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS,
        classification: DMA_SYNC_PLAN_LOCAL_STATIC_CLASSIFICATION,
    }
}

pub fn rejected_dma_cache_sync_plan_evidence(
    error: DmaCacheSyncPlanError,
) -> (&'static str, &'static str) {
    (DMA_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn derive_dma_cache_maintenance_sequence(
    sync_plan_evidence: DmaCacheSyncPlanEvidence,
) -> Result<DmaCacheMaintenanceSequence, DmaCacheMaintenanceSequenceError> {
    validate_accepted_sync_plan_evidence(sync_plan_evidence)?;
    let instruction = derive_dma_cache_maintenance_instruction(sync_plan_evidence.operation)?;
    let line_count = sync_plan_evidence
        .covered_length
        .checked_div(sync_plan_evidence.cache_line_size)
        .ok_or(DmaCacheMaintenanceSequenceError::CacheLineMismatch)?;

    Ok(DmaCacheMaintenanceSequence {
        sync_plan: sync_plan_evidence,
        instruction,
        barrier: DmaCacheMaintenanceBarrier::DataSynchronizationBarrierSy,
        line_count,
    })
}

pub fn dma_cache_maintenance_sequence_evidence(
    sequence: DmaCacheMaintenanceSequence,
) -> DmaCacheMaintenanceSequenceEvidence {
    DmaCacheMaintenanceSequenceEvidence {
        maintenance_sequence_contract_id: DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID,
        sync_plan_contract_id: sequence.sync_plan.sync_plan_contract_id,
        descriptor_contract_id: sequence.sync_plan.descriptor_contract_id,
        descriptor_source_inventory_id: sequence.sync_plan.descriptor_source_inventory_id,
        operation: sequence.sync_plan.operation,
        instruction: sequence.instruction.name(),
        instruction_mnemonic: sequence.instruction.instruction(),
        barrier: sequence.barrier.name(),
        barrier_mnemonic: sequence.barrier.instruction(),
        cache_line_source: sequence.sync_plan.cache_line_source,
        cache_line_size: sequence.sync_plan.cache_line_size,
        line_aligned_cpu_start: sequence.sync_plan.line_aligned_cpu_start,
        covered_length: sequence.sync_plan.covered_length,
        line_count: sequence.line_count,
        cpu_physical: sequence.sync_plan.cpu_physical,
        cpu_visible: sequence.sync_plan.cpu_visible,
        rp1_bus_address: sequence.sync_plan.rp1_bus_address,
        descriptor_length: sequence.sync_plan.descriptor_length,
        direction: sequence.sync_plan.direction,
        cacheability: sequence.sync_plan.cacheability,
        owner_transition: sequence.sync_plan.owner_transition,
        iommu_classification: sequence.sync_plan.iommu_classification,
        rejected_runtime_claims: sequence.sync_plan.rejected_runtime_claims,
        classification: DMA_MAINTENANCE_SEQUENCE_LOCAL_STATIC_CLASSIFICATION,
    }
}

pub fn rejected_dma_cache_maintenance_sequence_evidence(
    error: DmaCacheMaintenanceSequenceError,
) -> (&'static str, &'static str) {
    (DMA_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn translate_rp1_bus_address(
    path: DmaAddressPath,
    cpu_physical: u64,
    length: u64,
) -> Result<u64, DmaValidationError> {
    if length == 0 {
        return Err(DmaValidationError::ZeroLength);
    }
    let cpu_end = cpu_physical
        .checked_add(length)
        .ok_or(DmaValidationError::TranslationOverflow)?;

    let (cpu_base, rp1_base, window_size) = match path {
        DmaAddressPath::Rp1RamWindow => (
            RP1_RAM_WINDOW_CPU_BASE,
            RP1_RAM_WINDOW_BASE,
            RP1_RAM_WINDOW_SIZE,
        ),
        DmaAddressPath::Rp1PeripheralWindow => (
            RP1_PERIPHERAL_WINDOW_CPU_BASE,
            RP1_PERIPHERAL_WINDOW_BASE,
            RP1_PERIPHERAL_WINDOW_SIZE,
        ),
    };
    let window_end = cpu_base
        .checked_add(window_size)
        .ok_or(DmaValidationError::TranslationOverflow)?;
    if cpu_physical < cpu_base || cpu_end > window_end {
        return Err(DmaValidationError::TranslationRange);
    }

    let offset = cpu_physical
        .checked_sub(cpu_base)
        .ok_or(DmaValidationError::TranslationRange)?;
    rp1_base
        .checked_add(offset)
        .ok_or(DmaValidationError::TranslationOverflow)
}

fn validate_alignment(
    cpu_physical: u64,
    length: u64,
    alignment: u64,
) -> Result<(), DmaValidationError> {
    if length == 0 {
        return Err(DmaValidationError::ZeroLength);
    }
    if alignment == 0 || !alignment.is_power_of_two() {
        return Err(DmaValidationError::InvalidAlignment);
    }
    if cpu_physical & (alignment - 1) != 0 {
        return Err(DmaValidationError::UnalignedCpuAddress);
    }
    if length & (alignment - 1) != 0 {
        return Err(DmaValidationError::UnalignedLength);
    }
    Ok(())
}

fn validate_owned_span(
    cpu_physical: u64,
    length: u64,
    accepted_owned_span: EarlyPageFrameSpan,
) -> Result<(), DmaValidationError> {
    let cpu_end = cpu_physical
        .checked_add(length)
        .ok_or(DmaValidationError::AddressOverflow)?;

    if cpu_physical >= 0x4000_0000 {
        return Err(DmaValidationError::ForbiddenHighMemory);
    }
    if cpu_physical < accepted_owned_span.start {
        return Err(DmaValidationError::ForbiddenReservedMemory);
    }
    if cpu_physical >= accepted_owned_span.end || cpu_end > accepted_owned_span.end {
        return Err(DmaValidationError::OwnershipSpanContainment);
    }
    Ok(())
}

fn validate_claims(
    cacheability: DmaCacheability,
    iommu: DmaIommuClassification,
) -> Result<(), DmaValidationError> {
    if cacheability != DmaCacheability::CacheableRequiresMaintenance {
        return Err(DmaValidationError::ForbiddenCacheabilityClaim);
    }
    if iommu != DmaIommuClassification::SourceUnassignedRp1Dma {
        return Err(DmaValidationError::ForbiddenIommuClaim);
    }
    Ok(())
}

fn validate_accepted_descriptor_evidence(
    descriptor: DmaBufferDescriptor,
    evidence: DmaDescriptorEvidence,
) -> Result<(), DmaCacheSyncPlanError> {
    if evidence.contract_id != DMA_CACHE_SUBSTRATE_CONTRACT_ID
        || evidence.source_inventory_id != DMA_CACHE_SOURCE_INVENTORY_ID
        || evidence.classification != DMA_LOCAL_STATIC_CLASSIFICATION
    {
        return Err(DmaCacheSyncPlanError::NonAcceptedDescriptorClassification);
    }
    if evidence.cpu_physical != descriptor.cpu_physical
        || evidence.cpu_visible != descriptor.cpu_visible
        || evidence.rp1_bus_address != descriptor.rp1_bus_address
        || evidence.length != descriptor.length
        || evidence.direction != descriptor.direction.name()
        || evidence.cacheability != descriptor.cacheability.name()
        || evidence.owner != descriptor.owner.name()
        || evidence.iommu_classification != descriptor.iommu.name()
    {
        return Err(DmaCacheSyncPlanError::DescriptorEvidenceMismatch);
    }
    Ok(())
}

fn validate_accepted_sync_plan_evidence(
    evidence: DmaCacheSyncPlanEvidence,
) -> Result<(), DmaCacheMaintenanceSequenceError> {
    if evidence.sync_plan_contract_id != DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        || evidence.classification != DMA_SYNC_PLAN_LOCAL_STATIC_CLASSIFICATION
    {
        return Err(DmaCacheMaintenanceSequenceError::NonAcceptedSyncPlanClassification);
    }
    if evidence.descriptor_contract_id != DMA_CACHE_SUBSTRATE_CONTRACT_ID
        || evidence.descriptor_source_inventory_id != DMA_CACHE_SOURCE_INVENTORY_ID
        || evidence.cacheability != DmaCacheability::CacheableRequiresMaintenance.name()
        || evidence.iommu_classification != RP1_DMA_SOURCE_UNASSIGNED_IOMMU
    {
        return Err(DmaCacheMaintenanceSequenceError::DescriptorEvidenceMismatch);
    }
    if evidence.rejected_runtime_claims != DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS {
        return Err(DmaCacheMaintenanceSequenceError::UnsupportedRuntimeClaims);
    }
    if evidence.cache_line_source != BCM2712_CACHE_LINE_SOURCE
        || evidence.cache_line_size != BCM2712_DMA_CACHE_LINE_SIZE
        || evidence.line_aligned_cpu_start & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
        || evidence.covered_length & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
    {
        return Err(DmaCacheMaintenanceSequenceError::CacheLineMismatch);
    }
    if evidence.covered_length == 0 {
        return Err(DmaCacheMaintenanceSequenceError::ZeroCoveredLength);
    }
    evidence
        .line_aligned_cpu_start
        .checked_add(evidence.covered_length)
        .ok_or(DmaCacheMaintenanceSequenceError::RangeOverflow)?;
    validate_sync_plan_operation_identity(evidence)?;
    Ok(())
}

fn derive_dma_cache_maintenance_instruction(
    operation: &'static str,
) -> Result<DmaCacheMaintenanceInstruction, DmaCacheMaintenanceSequenceError> {
    match operation {
        "clean-to-point-of-coherency" => {
            Ok(DmaCacheMaintenanceInstruction::CleanByVirtualAddressToPoC)
        }
        "invalidate-from-point-of-coherency" => {
            Ok(DmaCacheMaintenanceInstruction::InvalidateByVirtualAddressFromPoC)
        }
        "clean-invalidate-to-point-of-coherency" => {
            Ok(DmaCacheMaintenanceInstruction::CleanInvalidateByVirtualAddressToPoC)
        }
        _ => Err(DmaCacheMaintenanceSequenceError::UnsupportedSyncOperation),
    }
}

fn validate_sync_plan_operation_identity(
    evidence: DmaCacheSyncPlanEvidence,
) -> Result<(), DmaCacheMaintenanceSequenceError> {
    match (
        evidence.operation,
        evidence.direction,
        evidence.owner_transition,
    ) {
        ("clean-to-point-of-coherency", "to-device", "cpu-to-device")
        | ("invalidate-from-point-of-coherency", "from-device", "device-to-cpu")
        | ("clean-invalidate-to-point-of-coherency", "bidirectional", "shared-cpu-device") => {
            Ok(())
        }
        ("clean-to-point-of-coherency", _, _)
        | ("invalidate-from-point-of-coherency", _, _)
        | ("clean-invalidate-to-point-of-coherency", _, _) => {
            Err(DmaCacheMaintenanceSequenceError::DescriptorEvidenceMismatch)
        }
        _ => Err(DmaCacheMaintenanceSequenceError::UnsupportedSyncOperation),
    }
}

const fn align_down(value: u64, alignment: u64) -> u64 {
    value & !(alignment - 1)
}

fn align_up(value: u64, alignment: u64) -> Result<u64, DmaCacheSyncPlanError> {
    let mask = alignment - 1;
    value
        .checked_add(mask)
        .map(|aligned| aligned & !mask)
        .ok_or(DmaCacheSyncPlanError::RangeOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_map::{EARLY_PAGE_SIZE, EarlyPageFrameSpan};

    fn accepted_owned_span() -> EarlyPageFrameSpan {
        EarlyPageFrameSpan {
            start: 0x2f01_0000,
            end: 0x3fc0_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x10bf0,
        }
    }

    fn valid_request() -> DmaBufferRequest {
        DmaBufferRequest {
            cpu_physical: 0x2f02_0000,
            cpu_visible: 0x2f02_0000,
            length: 0x2000,
            alignment: EARLY_PAGE_SIZE,
            direction: DmaDirection::ToDevice,
            cacheability: DmaCacheability::CacheableRequiresMaintenance,
            address_path: DmaAddressPath::Rp1RamWindow,
            owner: DmaBufferOwner::CpuOwned,
            iommu: DmaIommuClassification::SourceUnassignedRp1Dma,
        }
    }

    fn accepted_descriptor(direction: DmaDirection) -> DmaBufferDescriptor {
        let request = DmaBufferRequest {
            direction,
            ..valid_request()
        };
        validate_dma_buffer_descriptor(request, accepted_owned_span()).expect("valid")
    }

    fn accepted_sync_plan_evidence(
        direction: DmaDirection,
        boundary: DmaCacheSyncBoundary,
    ) -> DmaCacheSyncPlanEvidence {
        let descriptor = accepted_descriptor(direction);
        let plan = plan_dma_cache_sync(descriptor, dma_descriptor_evidence(descriptor), boundary)
            .expect("valid sync plan");
        dma_cache_sync_plan_evidence(plan)
    }

    #[test_case]
    fn valid_rp1_ram_window_descriptor_emits_local_static_evidence() {
        let descriptor =
            validate_dma_buffer_descriptor(valid_request(), accepted_owned_span()).expect("valid");

        assert_eq!(DMA_STAGING_BLOCKER_CLASSIFICATION, "staging/build-blocker");
        assert_eq!(descriptor.cpu_physical, 0x2f02_0000);
        assert_eq!(descriptor.cpu_visible, 0x2f02_0000);
        assert_eq!(descriptor.rp1_bus_address, 0x10_2f02_0000);
        assert_eq!(descriptor.length, 0x2000);
        assert_eq!(descriptor.alignment, EARLY_PAGE_SIZE);
        assert_eq!(descriptor.direction, DmaDirection::ToDevice);
        assert_eq!(
            descriptor.cacheability,
            DmaCacheability::CacheableRequiresMaintenance
        );
        assert_eq!(descriptor.address_path, DmaAddressPath::Rp1RamWindow);
        assert_eq!(descriptor.owner, DmaBufferOwner::CpuOwned);
        assert_eq!(
            descriptor.iommu,
            DmaIommuClassification::SourceUnassignedRp1Dma
        );

        let evidence = dma_descriptor_evidence(descriptor);
        assert_eq!(evidence.contract_id, DMA_CACHE_SUBSTRATE_CONTRACT_ID);
        assert_eq!(evidence.source_inventory_id, DMA_CACHE_SOURCE_INVENTORY_ID);
        assert_eq!(evidence.address_path, "rp1-ram-window");
        assert_eq!(evidence.address_path_source, RP1_RAM_WINDOW_SOURCE);
        assert_eq!(evidence.cpu_physical, 0x2f02_0000);
        assert_eq!(evidence.cpu_visible, 0x2f02_0000);
        assert_eq!(evidence.rp1_bus_address, 0x10_2f02_0000);
        assert_eq!(evidence.length, 0x2000);
        assert_eq!(evidence.alignment, EARLY_PAGE_SIZE);
        assert_eq!(evidence.direction, "to-device");
        assert_eq!(evidence.cacheability, "cacheable-requires-maintenance");
        assert_eq!(evidence.owner, "cpu-owned");
        assert_eq!(
            evidence.iommu_classification,
            RP1_DMA_SOURCE_UNASSIGNED_IOMMU
        );
        assert_eq!(
            evidence.validation,
            DmaValidationResults {
                alignment: true,
                ownership_span_containment: true,
                translation_range: true,
                forbidden_claims: true,
            }
        );
        assert_eq!(evidence.classification, DMA_LOCAL_STATIC_CLASSIFICATION);
    }

    #[test_case]
    fn sync_plan_derives_to_device_before_ownership_clean() {
        let descriptor = accepted_descriptor(DmaDirection::ToDevice);
        let plan = plan_dma_cache_sync(
            descriptor,
            dma_descriptor_evidence(descriptor),
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        )
        .expect("valid sync plan");

        assert_eq!(
            plan.operation,
            DmaCacheSyncOperation::CleanToPointOfCoherency
        );
        assert_eq!(plan.cache_line_size, BCM2712_DMA_CACHE_LINE_SIZE);
        assert_eq!(plan.line_aligned_cpu_start, 0x2f02_0000);
        assert_eq!(plan.covered_length, 0x2000);

        let evidence = dma_cache_sync_plan_evidence(plan);
        assert_eq!(
            evidence.sync_plan_contract_id,
            DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        );
        assert_eq!(
            evidence.descriptor_contract_id,
            DMA_CACHE_SUBSTRATE_CONTRACT_ID
        );
        assert_eq!(
            evidence.descriptor_source_inventory_id,
            DMA_CACHE_SOURCE_INVENTORY_ID
        );
        assert_eq!(evidence.sync_boundary, "before-device-ownership");
        assert_eq!(evidence.operation, "clean-to-point-of-coherency");
        assert_eq!(evidence.cache_line_source, BCM2712_CACHE_LINE_SOURCE);
        assert_eq!(evidence.cache_line_size, 64);
        assert_eq!(evidence.line_aligned_cpu_start, 0x2f02_0000);
        assert_eq!(evidence.covered_length, 0x2000);
        assert_eq!(evidence.cpu_physical, 0x2f02_0000);
        assert_eq!(evidence.cpu_visible, 0x2f02_0000);
        assert_eq!(evidence.rp1_bus_address, 0x10_2f02_0000);
        assert_eq!(evidence.descriptor_length, 0x2000);
        assert_eq!(evidence.direction, "to-device");
        assert_eq!(evidence.cacheability, "cacheable-requires-maintenance");
        assert_eq!(evidence.owner_transition, "cpu-to-device");
        assert_eq!(
            evidence.iommu_classification,
            RP1_DMA_SOURCE_UNASSIGNED_IOMMU
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.classification,
            DMA_SYNC_PLAN_LOCAL_STATIC_CLASSIFICATION
        );
    }

    #[test_case]
    fn sync_plan_derives_from_device_after_ownership_invalidate() {
        let descriptor = accepted_descriptor(DmaDirection::FromDevice);
        let plan = plan_dma_cache_sync(
            descriptor,
            dma_descriptor_evidence(descriptor),
            DmaCacheSyncBoundary::AfterDeviceOwnership,
        )
        .expect("valid sync plan");

        assert_eq!(
            plan.operation,
            DmaCacheSyncOperation::InvalidateFromPointOfCoherency
        );
        let evidence = dma_cache_sync_plan_evidence(plan);
        assert_eq!(evidence.sync_boundary, "after-device-ownership");
        assert_eq!(evidence.operation, "invalidate-from-point-of-coherency");
        assert_eq!(evidence.direction, "from-device");
        assert_eq!(evidence.owner_transition, "device-to-cpu");
    }

    #[test_case]
    fn sync_plan_derives_bidirectional_shared_clean_invalidate() {
        let descriptor = accepted_descriptor(DmaDirection::Bidirectional);
        let plan = plan_dma_cache_sync(
            descriptor,
            dma_descriptor_evidence(descriptor),
            DmaCacheSyncBoundary::SharedSynchronizationBoundary,
        )
        .expect("valid sync plan");

        assert_eq!(
            plan.operation,
            DmaCacheSyncOperation::CleanInvalidateToPointOfCoherency
        );
        let evidence = dma_cache_sync_plan_evidence(plan);
        assert_eq!(evidence.sync_boundary, "shared-synchronization-boundary");
        assert_eq!(evidence.operation, "clean-invalidate-to-point-of-coherency");
        assert_eq!(evidence.direction, "bidirectional");
        assert_eq!(evidence.owner_transition, "shared-cpu-device");
    }

    #[test_case]
    fn sync_plan_covers_unaligned_cpu_visible_range_with_cache_lines() {
        let descriptor = DmaBufferDescriptor {
            cpu_visible: 0x2f02_0001,
            length: 63,
            ..accepted_descriptor(DmaDirection::ToDevice)
        };
        let mut evidence = dma_descriptor_evidence(descriptor);
        evidence.cpu_visible = 0x2f02_0001;
        evidence.length = 63;

        let plan = plan_dma_cache_sync(
            descriptor,
            evidence,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        )
        .expect("valid sync plan");

        assert_eq!(plan.line_aligned_cpu_start, 0x2f02_0000);
        assert_eq!(plan.covered_length, 64);
    }

    #[test_case]
    fn sync_plan_rejects_overflow_unsupported_claims_and_boundary_pairs() {
        let descriptor = accepted_descriptor(DmaDirection::ToDevice);
        assert_eq!(
            plan_dma_cache_sync(
                DmaBufferDescriptor {
                    cpu_visible: u64::MAX - 0x10,
                    length: 0x20,
                    ..descriptor
                },
                DmaDescriptorEvidence {
                    cpu_visible: u64::MAX - 0x10,
                    length: 0x20,
                    ..dma_descriptor_evidence(descriptor)
                },
                DmaCacheSyncBoundary::BeforeDeviceOwnership,
            ),
            Err(DmaCacheSyncPlanError::RangeOverflow)
        );
        assert_eq!(
            plan_dma_cache_sync(
                DmaBufferDescriptor {
                    cacheability: DmaCacheability::NonCacheableMappingUnaccepted,
                    ..descriptor
                },
                DmaDescriptorEvidence {
                    cacheability: "non-cacheable-mapping-unaccepted",
                    ..dma_descriptor_evidence(descriptor)
                },
                DmaCacheSyncBoundary::BeforeDeviceOwnership,
            ),
            Err(DmaCacheSyncPlanError::UnsupportedCacheabilityClaim)
        );
        assert_eq!(
            plan_dma_cache_sync(
                DmaBufferDescriptor {
                    iommu: DmaIommuClassification::UnknownUnaccepted,
                    ..descriptor
                },
                DmaDescriptorEvidence {
                    iommu_classification: "unknown-iommu-unaccepted",
                    ..dma_descriptor_evidence(descriptor)
                },
                DmaCacheSyncBoundary::BeforeDeviceOwnership,
            ),
            Err(DmaCacheSyncPlanError::UnsupportedIommuClaim)
        );
        assert_eq!(
            plan_dma_cache_sync(
                descriptor,
                dma_descriptor_evidence(descriptor),
                DmaCacheSyncBoundary::AfterDeviceOwnership,
            ),
            Err(DmaCacheSyncPlanError::UnsupportedDirectionBoundary)
        );
        assert_eq!(
            rejected_dma_cache_sync_plan_evidence(
                DmaCacheSyncPlanError::UnsupportedDirectionBoundary
            ),
            (
                DMA_REJECTED_INPUT_CLASSIFICATION,
                "unsupported-direction-boundary"
            )
        );
    }

    #[test_case]
    fn sync_plan_rejects_non_accepted_descriptor_classification_inputs() {
        let descriptor = accepted_descriptor(DmaDirection::ToDevice);
        assert_eq!(
            plan_dma_cache_sync(
                DmaBufferDescriptor {
                    length: 0,
                    ..descriptor
                },
                DmaDescriptorEvidence {
                    length: 0,
                    ..dma_descriptor_evidence(descriptor)
                },
                DmaCacheSyncBoundary::BeforeDeviceOwnership,
            ),
            Err(DmaCacheSyncPlanError::ZeroLength)
        );
        assert_eq!(
            plan_dma_cache_sync(
                descriptor,
                DmaDescriptorEvidence {
                    classification: DMA_REJECTED_INPUT_CLASSIFICATION,
                    ..dma_descriptor_evidence(descriptor)
                },
                DmaCacheSyncBoundary::BeforeDeviceOwnership,
            ),
            Err(DmaCacheSyncPlanError::NonAcceptedDescriptorClassification)
        );
        assert_eq!(
            plan_dma_cache_sync(
                descriptor,
                DmaDescriptorEvidence {
                    rp1_bus_address: 0,
                    ..dma_descriptor_evidence(descriptor)
                },
                DmaCacheSyncBoundary::BeforeDeviceOwnership,
            ),
            Err(DmaCacheSyncPlanError::DescriptorEvidenceMismatch)
        );
    }

    #[test_case]
    fn maintenance_sequence_derives_clean_instruction_barrier_and_evidence() {
        let sync_evidence = accepted_sync_plan_evidence(
            DmaDirection::ToDevice,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        );
        let sequence =
            derive_dma_cache_maintenance_sequence(sync_evidence).expect("valid sequence");

        assert_eq!(
            sequence.instruction,
            DmaCacheMaintenanceInstruction::CleanByVirtualAddressToPoC
        );
        assert_eq!(
            sequence.barrier,
            DmaCacheMaintenanceBarrier::DataSynchronizationBarrierSy
        );
        assert_eq!(sequence.line_count, 128);

        let evidence = dma_cache_maintenance_sequence_evidence(sequence);
        assert_eq!(
            evidence.maintenance_sequence_contract_id,
            DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID
        );
        assert_eq!(
            evidence.sync_plan_contract_id,
            DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        );
        assert_eq!(
            evidence.descriptor_contract_id,
            DMA_CACHE_SUBSTRATE_CONTRACT_ID
        );
        assert_eq!(
            evidence.descriptor_source_inventory_id,
            DMA_CACHE_SOURCE_INVENTORY_ID
        );
        assert_eq!(evidence.operation, "clean-to-point-of-coherency");
        assert_eq!(evidence.instruction, "clean-by-virtual-address-to-poc");
        assert_eq!(evidence.instruction_mnemonic, "dc cvac");
        assert_eq!(evidence.barrier, "data-synchronization-barrier-sy");
        assert_eq!(evidence.barrier_mnemonic, "dsb sy");
        assert_eq!(evidence.cache_line_source, BCM2712_CACHE_LINE_SOURCE);
        assert_eq!(evidence.cache_line_size, 64);
        assert_eq!(evidence.line_aligned_cpu_start, 0x2f02_0000);
        assert_eq!(evidence.covered_length, 0x2000);
        assert_eq!(evidence.line_count, 128);
        assert_eq!(evidence.cpu_physical, 0x2f02_0000);
        assert_eq!(evidence.cpu_visible, 0x2f02_0000);
        assert_eq!(evidence.rp1_bus_address, 0x10_2f02_0000);
        assert_eq!(evidence.descriptor_length, 0x2000);
        assert_eq!(evidence.direction, "to-device");
        assert_eq!(evidence.cacheability, "cacheable-requires-maintenance");
        assert_eq!(evidence.owner_transition, "cpu-to-device");
        assert_eq!(
            evidence.iommu_classification,
            RP1_DMA_SOURCE_UNASSIGNED_IOMMU
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.classification,
            DMA_MAINTENANCE_SEQUENCE_LOCAL_STATIC_CLASSIFICATION
        );
    }

    #[test_case]
    fn maintenance_sequence_derives_invalidate_and_clean_invalidate_instructions() {
        let invalidate = derive_dma_cache_maintenance_sequence(accepted_sync_plan_evidence(
            DmaDirection::FromDevice,
            DmaCacheSyncBoundary::AfterDeviceOwnership,
        ))
        .expect("valid invalidate sequence");
        let clean_invalidate = derive_dma_cache_maintenance_sequence(accepted_sync_plan_evidence(
            DmaDirection::Bidirectional,
            DmaCacheSyncBoundary::SharedSynchronizationBoundary,
        ))
        .expect("valid clean+invalidate sequence");

        assert_eq!(
            dma_cache_maintenance_sequence_evidence(invalidate).instruction_mnemonic,
            "dc ivac"
        );
        assert_eq!(
            dma_cache_maintenance_sequence_evidence(clean_invalidate).instruction_mnemonic,
            "dc civac"
        );
    }

    #[test_case]
    fn maintenance_sequence_rejects_zero_mismatch_overflow_and_runtime_claims() {
        let evidence = accepted_sync_plan_evidence(
            DmaDirection::ToDevice,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        );

        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                covered_length: 0,
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::ZeroCoveredLength)
        );
        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                cache_line_size: 32,
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::CacheLineMismatch)
        );
        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                line_aligned_cpu_start: 0x2f02_0001,
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::CacheLineMismatch)
        );
        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                line_aligned_cpu_start: u64::MAX - 0x10,
                covered_length: 0x40,
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::RangeOverflow)
        );
        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                rejected_runtime_claims: &["executed-driver-buffer-cache-maintenance"],
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::UnsupportedRuntimeClaims)
        );
    }

    #[test_case]
    fn maintenance_sequence_rejects_non_accepted_sync_plan_and_descriptor_mismatch() {
        let evidence = accepted_sync_plan_evidence(
            DmaDirection::ToDevice,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        );

        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                classification: DMA_REJECTED_INPUT_CLASSIFICATION,
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::NonAcceptedSyncPlanClassification)
        );
        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                descriptor_contract_id: "wrong-descriptor-contract",
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::DescriptorEvidenceMismatch)
        );
        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                direction: "from-device",
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::DescriptorEvidenceMismatch)
        );
        assert_eq!(
            derive_dma_cache_maintenance_sequence(DmaCacheSyncPlanEvidence {
                operation: "unsupported-operation",
                ..evidence
            }),
            Err(DmaCacheMaintenanceSequenceError::UnsupportedSyncOperation)
        );
        assert_eq!(
            rejected_dma_cache_maintenance_sequence_evidence(
                DmaCacheMaintenanceSequenceError::UnsupportedRuntimeClaims
            ),
            (
                DMA_REJECTED_INPUT_CLASSIFICATION,
                "unsupported-runtime-claims"
            )
        );
    }

    #[test_case]
    fn validator_rejects_unaligned_descriptor() {
        let request = DmaBufferRequest {
            cpu_physical: 0x2f02_0001,
            ..valid_request()
        };

        assert_eq!(
            validate_dma_buffer_descriptor(request, accepted_owned_span()),
            Err(DmaValidationError::UnalignedCpuAddress)
        );
    }

    #[test_case]
    fn validator_rejects_owned_span_escape() {
        let request = DmaBufferRequest {
            cpu_physical: 0x3fbf_f000,
            length: 0x2000,
            ..valid_request()
        };

        assert_eq!(
            validate_dma_buffer_descriptor(request, accepted_owned_span()),
            Err(DmaValidationError::OwnershipSpanContainment)
        );
    }

    #[test_case]
    fn validator_rejects_forbidden_reserved_and_high_memory_inputs() {
        let reserved = DmaBufferRequest {
            cpu_physical: 0x2f00_0000,
            ..valid_request()
        };
        let high_memory = DmaBufferRequest {
            cpu_physical: 0x1_0000_0000,
            ..valid_request()
        };

        assert_eq!(
            validate_dma_buffer_descriptor(reserved, accepted_owned_span()),
            Err(DmaValidationError::ForbiddenReservedMemory)
        );
        assert_eq!(
            validate_dma_buffer_descriptor(high_memory, accepted_owned_span()),
            Err(DmaValidationError::ForbiddenHighMemory)
        );
    }

    #[test_case]
    fn translation_helpers_cover_ram_peripheral_range_and_overflow() {
        assert_eq!(
            translate_rp1_bus_address(DmaAddressPath::Rp1RamWindow, 0x2f02_0000, 0x1000),
            Ok(0x10_2f02_0000)
        );
        assert_eq!(
            translate_rp1_bus_address(DmaAddressPath::Rp1PeripheralWindow, 0x1f_000d_0000, 4),
            Ok(0xc0_400d_0000)
        );
        assert_eq!(
            translate_rp1_bus_address(DmaAddressPath::Rp1PeripheralWindow, 0x1f_0040_fff0, 0x20),
            Err(DmaValidationError::TranslationRange)
        );
        assert_eq!(
            translate_rp1_bus_address(DmaAddressPath::Rp1RamWindow, u64::MAX - 0x10, 0x20),
            Err(DmaValidationError::TranslationOverflow)
        );
    }

    #[test_case]
    fn validator_rejects_forbidden_cacheability_and_iommu_claims() {
        let coherent = DmaBufferRequest {
            cacheability: DmaCacheability::CoherentHardwareUnaccepted,
            ..valid_request()
        };
        let iommu = DmaBufferRequest {
            iommu: DmaIommuClassification::Iommu5DisplayCameraOnlyUnaccepted,
            ..valid_request()
        };

        assert_eq!(
            validate_dma_buffer_descriptor(coherent, accepted_owned_span()),
            Err(DmaValidationError::ForbiddenCacheabilityClaim)
        );
        assert_eq!(
            validate_dma_buffer_descriptor(iommu, accepted_owned_span()),
            Err(DmaValidationError::ForbiddenIommuClaim)
        );
        assert_eq!(
            rejected_dma_input_evidence(DmaValidationError::ForbiddenIommuClaim),
            (DMA_REJECTED_INPUT_CLASSIFICATION, "forbidden-iommu-claim")
        );
    }
}
