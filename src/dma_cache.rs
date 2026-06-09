use crate::memory_map::EarlyPageFrameSpan;

pub const DMA_CACHE_SUBSTRATE_CONTRACT_ID: &str = "phase11-rp1-dma-cache-substrate-contract-v1";
pub const DMA_CACHE_SYNC_PLAN_CONTRACT_ID: &str = "phase11-rp1-dma-cache-sync-plan-contract-v1";
pub const DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID: &str =
    "phase11-rp1-dma-cache-maintenance-sequence-contract-v1";
pub const DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID: &str =
    "phase11-rp1-dma-cache-maintenance-executor-contract-v1";
pub const DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID: &str =
    "phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1";
pub const DMA_CACHE_SMALL_DIAGNOSTIC_SOURCE_CONTRACT_ID: &str =
    "phase11-rp1-dma-cache-small-diagnostic-source-contract-20260609";
pub const DMA_CACHE_SMALL_DIAGNOSTIC_PLAN_CONTRACT_ID: &str =
    "phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1";
pub const DMA_CACHE_SMALL_DIAGNOSTIC_VISIBILITY_REPORT_CONTRACT_ID: &str =
    "phase11-rp1-dma-cache-small-diagnostic-visibility-report-contract-v1";
pub const DMA_CACHE_SOURCE_INVENTORY_ID: &str = "phase11-rp1-dma-cache-source-inventory-20260609";
pub const DMA_LOCAL_STATIC_CLASSIFICATION: &str = "local-static-dma-cache-contract-visible";
pub const DMA_SYNC_PLAN_LOCAL_STATIC_CLASSIFICATION: &str =
    "local-static-dma-cache-sync-plan-visible";
pub const DMA_MAINTENANCE_SEQUENCE_LOCAL_STATIC_CLASSIFICATION: &str =
    "local-static-dma-cache-maintenance-sequence-visible";
pub const DMA_MAINTENANCE_EXECUTOR_RUNTIME_CLASSIFICATION: &str =
    "runtime-execution-dma-cache-maintenance-executor-visible";
pub const DMA_DRIVER_DIAGNOSTIC_ENVELOPE_LOCAL_STATIC_CLASSIFICATION: &str =
    "local-static-dma-cache-driver-diagnostic-envelope-visible";
pub const DMA_SMALL_DIAGNOSTIC_PLAN_LOCAL_STATIC_CLASSIFICATION: &str =
    "local-static-rp1-dma-small-diagnostic-plan-visible";
pub const DMA_SMALL_DIAGNOSTIC_VISIBILITY_CANDIDATE_CLASSIFICATION: &str =
    "local-static-rp1-dma-small-diagnostic-plan-visibility-candidate";
pub const DMA_SMALL_DIAGNOSTIC_VISIBILITY_CONTROL_CLASSIFICATION: &str =
    "no-plan-rp1-dma-small-diagnostic-visibility-control";
pub const DMA_SMALL_DIAGNOSTIC_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-plan-visibility-control-output";
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

pub const DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "driver-dma-completion",
    "rp1-mmio-writes",
    "dma-channel-programming",
    "descriptor-rings",
    "interrupt-completion",
    "ethernet-storage-networking-ssh",
    "hardware-validation",
    "milestone-11-3-completion",
];

pub const DMA_DRIVER_DIAGNOSTIC_UNRESOLVED_GAPS: &[&str] = &[
    "rp1-dma-channel-ownership",
    "descriptor-ring-layout-and-ownership",
    "transfer-completion-and-interrupt-policy",
    "iommu-runtime-policy",
    "dma-safe-allocation-and-pinning",
    "hardware-proof",
    "device-specific-consumer",
];

pub const DMA_SMALL_DIAGNOSTIC_VISIBILITY_REJECTED_HARDWARE_CLAIMS: &[&str] = &[
    "Pi 5 hardware validation by this local/static task",
    "boot archive publication by this local/static task",
    "hardwareTestLock acquisition by this local/static task",
    "RP1 MMIO writes",
    "RP1 DMA channel ownership",
    "DMA channel programming",
    "descriptor-ring construction",
    "descriptor-ring ownership",
    "transfer completion",
    "interrupt completion",
    "hardware or device completion",
    "Ethernet readiness",
    "storage readiness",
    "networking",
    "SSH",
    "cache-coherent driver policy",
    "non-cacheable DMA mapping policy",
    "IOMMU-backed runtime policy",
    "DMA-safe allocation or pinning beyond accepted descriptor validation",
    "Milestone 12 progress",
    "Milestone 11.3 completion by implication",
    "phase transition",
];

pub const DMA_SMALL_DIAGNOSTIC_VISIBILITY_RETAINED_RISKS: &[&str] = &[
    "No RP1 DMA channel ownership has been accepted",
    "No descriptor-ring layout or ownership has been accepted",
    "No transfer-completion or interrupt policy exists",
    "IOMMU/runtime policy remains source-unassigned",
    "DMA-safe allocation and pinning remain unaccepted beyond descriptor validation",
    "No live hardware DMA proof has been accepted",
    "No device-specific consumer has been selected",
    "Ethernet, storage, networking, and SSH remain explicitly out of scope",
];

pub const RP1_RAM_WINDOW_SOURCE: &str = "rp1-dma-ranges-ram-window";
pub const RP1_RAM_WINDOW_BASE: u64 = 0x10_0000_0000;
pub const RP1_RAM_WINDOW_CPU_BASE: u64 = 0x0;
pub const RP1_RAM_WINDOW_SIZE: u64 = 0x10_0000_0000;

pub const RP1_PERIPHERAL_WINDOW_SOURCE: &str = "rp1-dma-ranges-peripheral-window";
pub const RP1_PERIPHERAL_WINDOW_BASE: u64 = 0xc0_4000_0000;
pub const RP1_PERIPHERAL_WINDOW_CPU_BASE: u64 = 0x1f_0000_0000;
pub const RP1_PERIPHERAL_WINDOW_SIZE: u64 = 0x0041_0000;

pub const RP1_DMA_CONTROLLER_COMPATIBLE: &str = "snps,axi-dma-1.01a";
pub const RP1_DMA_CONTROLLER_BUS_BASE: u64 = 0xc0_4018_8000;
pub const RP1_DMA_CONTROLLER_CPU_BASE: u64 = 0x1f_0018_8000;
pub const RP1_DMA_CHANNEL_COUNT: u32 = 8;
pub const RP1_DMA_TARGET_COUNT: u32 = 64;
pub const RP1_DMA_INTERRUPT_NAME: &str = "RP1_INT_DMA";
pub const RP1_DMA_CLOCK_NAMES: &[&str] = &["RP1_CLK_DMA", "RP1_CLK_SYS"];

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheMaintenanceExecutorError {
    ContractIdentityMismatch,
    NonAcceptedSequenceClassification,
    WrongCacheabilityIommuIdentity,
    MissingRejectedRuntimeClaimsIdentity,
    ZeroLineCoverage,
    LineRangeMismatch,
    RangeOverflow,
    UnsupportedOperationVocabulary,
    UnsupportedInstructionVocabulary,
    UnsupportedBarrierVocabulary,
}

impl DmaCacheMaintenanceExecutorError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ContractIdentityMismatch => "contract-identity-mismatch",
            Self::NonAcceptedSequenceClassification => "non-accepted-sequence-classification",
            Self::WrongCacheabilityIommuIdentity => "wrong-cacheability-iommu-identity",
            Self::MissingRejectedRuntimeClaimsIdentity => {
                "missing-rejected-runtime-claims-identity"
            }
            Self::ZeroLineCoverage => "zero-line-coverage",
            Self::LineRangeMismatch => "line-range-mismatch",
            Self::RangeOverflow => "range-overflow",
            Self::UnsupportedOperationVocabulary => "unsupported-operation-vocabulary",
            Self::UnsupportedInstructionVocabulary => "unsupported-instruction-vocabulary",
            Self::UnsupportedBarrierVocabulary => "unsupported-barrier-vocabulary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheMaintenanceExecutorEvidence {
    pub executor_contract_id: &'static str,
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
    pub prerequisite_rejected_runtime_claims: &'static [&'static str],
    pub rejected_runtime_claims: &'static [&'static str],
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheDriverDiagnosticEnvelopeInput {
    pub executor_evidence: DmaCacheMaintenanceExecutorEvidence,
    pub claims_driver_dma_completion: bool,
    pub claims_hardware_device_completion: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheDriverDiagnosticEnvelope {
    pub executor_evidence: DmaCacheMaintenanceExecutorEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheDriverDiagnosticEnvelopeEvidence {
    pub driver_diagnostic_envelope_contract_id: &'static str,
    pub executor_contract_id: &'static str,
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
    pub prerequisite_rejected_runtime_claims: &'static [&'static str],
    pub executor_rejected_runtime_claims: &'static [&'static str],
    pub unresolved_dma_diagnostic_gaps: &'static [&'static str],
    pub claims_driver_dma_completion: bool,
    pub claims_hardware_device_completion: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheDriverDiagnosticEnvelopeError {
    DriverDmaCompletionClaim,
    HardwareDeviceCompletionClaim,
    MissingPrerequisiteIdentity,
    NonAcceptedExecutorClassification,
    MissingRejectedRuntimeClaimsIdentity,
    ZeroLineCoverage,
    LineRangeMismatch,
    RangeOverflow,
    UnsupportedCacheabilityClaim,
    UnsupportedIommuClaim,
}

impl DmaCacheDriverDiagnosticEnvelopeError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::DriverDmaCompletionClaim => "driver-dma-completion-claim",
            Self::HardwareDeviceCompletionClaim => "hardware-device-completion-claim",
            Self::MissingPrerequisiteIdentity => "missing-prerequisite-identity",
            Self::NonAcceptedExecutorClassification => "non-accepted-executor-classification",
            Self::MissingRejectedRuntimeClaimsIdentity => {
                "missing-rejected-runtime-claims-identity"
            }
            Self::ZeroLineCoverage => "zero-line-coverage",
            Self::LineRangeMismatch => "line-range-mismatch",
            Self::RangeOverflow => "range-overflow",
            Self::UnsupportedCacheabilityClaim => "unsupported-cacheability-claim",
            Self::UnsupportedIommuClaim => "unsupported-iommu-claim",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1DmaControllerSourceFacts {
    pub compatible: &'static str,
    pub rp1_bus_base: u64,
    pub cpu_physical_base: u64,
    pub channel_count: u32,
    pub target_count: u32,
    pub interrupt_name: &'static str,
    pub clock_names: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSmallDiagnosticPlanInput {
    pub envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence,
    pub controller_source: Rp1DmaControllerSourceFacts,
    pub claims_rp1_channel_ownership: bool,
    pub claims_descriptor_ring_ready: bool,
    pub claims_transfer_completion: bool,
    pub claims_interrupt_completion: bool,
    pub claims_hardware_device_completion: bool,
    pub claims_ethernet_ready: bool,
    pub claims_storage_ready: bool,
    pub claims_networking: bool,
    pub claims_ssh: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSmallDiagnosticPlan {
    pub envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence,
    pub controller_source: Rp1DmaControllerSourceFacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSmallDiagnosticPlanEvidence {
    pub small_diagnostic_plan_contract_id: &'static str,
    pub driver_diagnostic_envelope_contract_id: &'static str,
    pub executor_contract_id: &'static str,
    pub maintenance_sequence_contract_id: &'static str,
    pub sync_plan_contract_id: &'static str,
    pub descriptor_contract_id: &'static str,
    pub descriptor_source_inventory_id: &'static str,
    pub rp1_dma_compatible: &'static str,
    pub rp1_dma_controller_rp1_bus_base: u64,
    pub rp1_dma_controller_cpu_physical_base: u64,
    pub rp1_dma_channel_count: u32,
    pub rp1_dma_target_count: u32,
    pub rp1_dma_interrupt_name: &'static str,
    pub rp1_dma_clock_names: &'static [&'static str],
    pub cpu_physical: u64,
    pub cpu_visible: u64,
    pub rp1_bus_address: u64,
    pub descriptor_length: u64,
    pub cache_line_source: &'static str,
    pub cache_line_size: u64,
    pub line_aligned_cpu_start: u64,
    pub covered_length: u64,
    pub line_count: u64,
    pub direction: &'static str,
    pub cacheability: &'static str,
    pub owner_transition: &'static str,
    pub iommu_classification: &'static str,
    pub prerequisite_rejected_runtime_claims: &'static [&'static str],
    pub executor_rejected_runtime_claims: &'static [&'static str],
    pub unresolved_dma_diagnostic_gaps: &'static [&'static str],
    pub claims_rp1_channel_ownership: bool,
    pub claims_descriptor_ring_ready: bool,
    pub claims_transfer_completion: bool,
    pub claims_interrupt_completion: bool,
    pub claims_hardware_device_completion: bool,
    pub claims_ethernet_ready: bool,
    pub claims_storage_ready: bool,
    pub claims_networking: bool,
    pub claims_ssh: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheSmallDiagnosticPlanError {
    NonAcceptedEnvelopeClassification,
    MissingPrerequisiteIdentity,
    MissingRejectedCompletionClaims,
    MissingUnresolvedDiagnosticGaps,
    UnsupportedCacheabilityClaim,
    UnsupportedIommuClaim,
    ZeroChannelCount,
    InvalidTranslatedControllerBase,
    Rp1ChannelOwnershipClaim,
    DescriptorRingReadinessClaim,
    TransferCompletionClaim,
    InterruptCompletionClaim,
    HardwareDeviceCompletionClaim,
    EthernetReadinessClaim,
    StorageReadinessClaim,
    NetworkingClaim,
    SshClaim,
}

impl DmaCacheSmallDiagnosticPlanError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::NonAcceptedEnvelopeClassification => "non-accepted-envelope-classification",
            Self::MissingPrerequisiteIdentity => "missing-prerequisite-identity",
            Self::MissingRejectedCompletionClaims => "missing-rejected-completion-claims",
            Self::MissingUnresolvedDiagnosticGaps => "missing-unresolved-diagnostic-gaps",
            Self::UnsupportedCacheabilityClaim => "unsupported-cacheability-claim",
            Self::UnsupportedIommuClaim => "unsupported-iommu-claim",
            Self::ZeroChannelCount => "zero-channel-count",
            Self::InvalidTranslatedControllerBase => "invalid-translated-controller-base",
            Self::Rp1ChannelOwnershipClaim => "rp1-channel-ownership-claim",
            Self::DescriptorRingReadinessClaim => "descriptor-ring-readiness-claim",
            Self::TransferCompletionClaim => "transfer-completion-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::HardwareDeviceCompletionClaim => "hardware-device-completion-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::StorageReadinessClaim => "storage-readiness-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SshClaim => "ssh-claim",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheSmallDiagnosticVisibilityReportKind {
    Candidate,
    NoPlanControl,
}

impl DmaCacheSmallDiagnosticVisibilityReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoPlanControl => "no-plan-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSmallDiagnosticVisibilityReportInput {
    pub kind: DmaCacheSmallDiagnosticVisibilityReportKind,
    pub plan_evidence: Option<DmaCacheSmallDiagnosticPlanEvidence>,
    pub claims_rp1_mmio_writes: bool,
    pub claims_rp1_channel_ownership: bool,
    pub claims_dma_channel_programming: bool,
    pub claims_descriptor_ring_ready: bool,
    pub claims_transfer_completion: bool,
    pub claims_interrupt_completion: bool,
    pub claims_hardware_device_completion: bool,
    pub claims_ethernet_ready: bool,
    pub claims_storage_ready: bool,
    pub claims_networking: bool,
    pub claims_ssh: bool,
    pub claims_milestone_11_3_completion: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSmallDiagnosticVisibilityReport {
    pub kind: DmaCacheSmallDiagnosticVisibilityReportKind,
    pub plan_evidence: Option<DmaCacheSmallDiagnosticPlanEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaCacheSmallDiagnosticVisibilityReportEvidence {
    pub visibility_report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub report_kind: &'static str,
    pub small_diagnostic_plan_contract_id: Option<&'static str>,
    pub driver_diagnostic_envelope_contract_id: Option<&'static str>,
    pub executor_contract_id: Option<&'static str>,
    pub maintenance_sequence_contract_id: Option<&'static str>,
    pub sync_plan_contract_id: Option<&'static str>,
    pub descriptor_contract_id: Option<&'static str>,
    pub descriptor_source_inventory_id: Option<&'static str>,
    pub rp1_dma_compatible: Option<&'static str>,
    pub rp1_dma_controller_rp1_bus_base: Option<u64>,
    pub rp1_dma_controller_cpu_physical_base: Option<u64>,
    pub rp1_dma_channel_count: Option<u32>,
    pub rp1_dma_target_count: Option<u32>,
    pub rp1_dma_interrupt_name: Option<&'static str>,
    pub rp1_dma_clock_names: Option<&'static [&'static str]>,
    pub cpu_physical: Option<u64>,
    pub cpu_visible: Option<u64>,
    pub rp1_bus_address: Option<u64>,
    pub descriptor_length: Option<u64>,
    pub cache_line_source: Option<&'static str>,
    pub cache_line_size: Option<u64>,
    pub line_aligned_cpu_start: Option<u64>,
    pub covered_length: Option<u64>,
    pub line_count: Option<u64>,
    pub direction: Option<&'static str>,
    pub cacheability: Option<&'static str>,
    pub owner_transition: Option<&'static str>,
    pub iommu_classification: Option<&'static str>,
    pub prerequisite_rejected_runtime_claims: Option<&'static [&'static str]>,
    pub executor_rejected_runtime_claims: Option<&'static [&'static str]>,
    pub unresolved_dma_diagnostic_gaps: Option<&'static [&'static str]>,
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_hardware_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub claims_rp1_mmio_writes: bool,
    pub claims_rp1_channel_ownership: bool,
    pub claims_dma_channel_programming: bool,
    pub claims_descriptor_ring_ready: bool,
    pub claims_transfer_completion: bool,
    pub claims_interrupt_completion: bool,
    pub claims_hardware_device_completion: bool,
    pub claims_ethernet_ready: bool,
    pub claims_storage_ready: bool,
    pub claims_networking: bool,
    pub claims_ssh: bool,
    pub claims_milestone_11_3_completion: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaCacheSmallDiagnosticVisibilityReportError {
    CandidateMissingPlanEvidence,
    ControlCarriesPlanEvidence,
    NonAcceptedPlanClassification,
    MissingPrerequisiteIdentity,
    MissingRejectedRuntimeClaims,
    MissingUnresolvedDiagnosticGaps,
    UnsupportedCacheabilityClaim,
    UnsupportedIommuClaim,
    Rp1MmioWriteClaim,
    Rp1ChannelOwnershipClaim,
    DmaChannelProgrammingClaim,
    DescriptorRingReadinessClaim,
    TransferCompletionClaim,
    InterruptCompletionClaim,
    HardwareDeviceCompletionClaim,
    EthernetReadinessClaim,
    StorageReadinessClaim,
    NetworkingClaim,
    SshClaim,
    Milestone113CompletionClaim,
    PhaseTransitionClaim,
}

impl DmaCacheSmallDiagnosticVisibilityReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingPlanEvidence => "candidate-missing-plan-evidence",
            Self::ControlCarriesPlanEvidence => "control-carries-plan-evidence",
            Self::NonAcceptedPlanClassification => "non-accepted-plan-classification",
            Self::MissingPrerequisiteIdentity => "missing-prerequisite-identity",
            Self::MissingRejectedRuntimeClaims => "missing-rejected-runtime-claims",
            Self::MissingUnresolvedDiagnosticGaps => "missing-unresolved-diagnostic-gaps",
            Self::UnsupportedCacheabilityClaim => "unsupported-cacheability-claim",
            Self::UnsupportedIommuClaim => "unsupported-iommu-claim",
            Self::Rp1MmioWriteClaim => "rp1-mmio-write-claim",
            Self::Rp1ChannelOwnershipClaim => "rp1-channel-ownership-claim",
            Self::DmaChannelProgrammingClaim => "dma-channel-programming-claim",
            Self::DescriptorRingReadinessClaim => "descriptor-ring-readiness-claim",
            Self::TransferCompletionClaim => "transfer-completion-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::HardwareDeviceCompletionClaim => "hardware-device-completion-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::StorageReadinessClaim => "storage-readiness-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SshClaim => "ssh-claim",
            Self::Milestone113CompletionClaim => "milestone-11-3-completion-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
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

pub fn execute_dma_cache_maintenance_sequence(
    evidence: DmaCacheMaintenanceSequenceEvidence,
) -> Result<DmaCacheMaintenanceExecutorEvidence, DmaCacheMaintenanceExecutorError> {
    let instruction = validate_accepted_maintenance_sequence_evidence(evidence)?;
    let barrier = validate_accepted_maintenance_barrier(evidence)?;
    let mut line = 0;

    while line < evidence.line_count {
        let offset = line
            .checked_mul(evidence.cache_line_size)
            .ok_or(DmaCacheMaintenanceExecutorError::RangeOverflow)?;
        let address = evidence
            .line_aligned_cpu_start
            .checked_add(offset)
            .ok_or(DmaCacheMaintenanceExecutorError::RangeOverflow)?;
        dispatch_dma_cache_maintenance_instruction(instruction, address);
        line += 1;
    }

    dispatch_dma_cache_maintenance_barrier(barrier);
    Ok(dma_cache_maintenance_executor_evidence(evidence))
}

pub fn dma_cache_maintenance_executor_evidence(
    evidence: DmaCacheMaintenanceSequenceEvidence,
) -> DmaCacheMaintenanceExecutorEvidence {
    DmaCacheMaintenanceExecutorEvidence {
        executor_contract_id: DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID,
        maintenance_sequence_contract_id: evidence.maintenance_sequence_contract_id,
        sync_plan_contract_id: evidence.sync_plan_contract_id,
        descriptor_contract_id: evidence.descriptor_contract_id,
        descriptor_source_inventory_id: evidence.descriptor_source_inventory_id,
        operation: evidence.operation,
        instruction: evidence.instruction,
        instruction_mnemonic: evidence.instruction_mnemonic,
        barrier: evidence.barrier,
        barrier_mnemonic: evidence.barrier_mnemonic,
        cache_line_source: evidence.cache_line_source,
        cache_line_size: evidence.cache_line_size,
        line_aligned_cpu_start: evidence.line_aligned_cpu_start,
        covered_length: evidence.covered_length,
        line_count: evidence.line_count,
        cpu_physical: evidence.cpu_physical,
        cpu_visible: evidence.cpu_visible,
        rp1_bus_address: evidence.rp1_bus_address,
        descriptor_length: evidence.descriptor_length,
        direction: evidence.direction,
        cacheability: evidence.cacheability,
        owner_transition: evidence.owner_transition,
        iommu_classification: evidence.iommu_classification,
        prerequisite_rejected_runtime_claims: evidence.rejected_runtime_claims,
        rejected_runtime_claims: DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS,
        classification: DMA_MAINTENANCE_EXECUTOR_RUNTIME_CLASSIFICATION,
    }
}

pub fn rejected_dma_cache_maintenance_executor_evidence(
    error: DmaCacheMaintenanceExecutorError,
) -> (&'static str, &'static str) {
    (DMA_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_dma_cache_driver_diagnostic_envelope(
    input: DmaCacheDriverDiagnosticEnvelopeInput,
) -> Result<DmaCacheDriverDiagnosticEnvelope, DmaCacheDriverDiagnosticEnvelopeError> {
    if input.claims_driver_dma_completion {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::DriverDmaCompletionClaim);
    }
    if input.claims_hardware_device_completion {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::HardwareDeviceCompletionClaim);
    }
    validate_driver_diagnostic_executor_evidence(input.executor_evidence)?;

    Ok(DmaCacheDriverDiagnosticEnvelope {
        executor_evidence: input.executor_evidence,
    })
}

pub fn dma_cache_driver_diagnostic_envelope_evidence(
    envelope: DmaCacheDriverDiagnosticEnvelope,
) -> DmaCacheDriverDiagnosticEnvelopeEvidence {
    let executor = envelope.executor_evidence;
    DmaCacheDriverDiagnosticEnvelopeEvidence {
        driver_diagnostic_envelope_contract_id: DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID,
        executor_contract_id: executor.executor_contract_id,
        maintenance_sequence_contract_id: executor.maintenance_sequence_contract_id,
        sync_plan_contract_id: executor.sync_plan_contract_id,
        descriptor_contract_id: executor.descriptor_contract_id,
        descriptor_source_inventory_id: executor.descriptor_source_inventory_id,
        operation: executor.operation,
        instruction: executor.instruction,
        instruction_mnemonic: executor.instruction_mnemonic,
        barrier: executor.barrier,
        barrier_mnemonic: executor.barrier_mnemonic,
        cache_line_source: executor.cache_line_source,
        cache_line_size: executor.cache_line_size,
        line_aligned_cpu_start: executor.line_aligned_cpu_start,
        covered_length: executor.covered_length,
        line_count: executor.line_count,
        cpu_physical: executor.cpu_physical,
        cpu_visible: executor.cpu_visible,
        rp1_bus_address: executor.rp1_bus_address,
        descriptor_length: executor.descriptor_length,
        direction: executor.direction,
        cacheability: executor.cacheability,
        owner_transition: executor.owner_transition,
        iommu_classification: executor.iommu_classification,
        prerequisite_rejected_runtime_claims: executor.prerequisite_rejected_runtime_claims,
        executor_rejected_runtime_claims: executor.rejected_runtime_claims,
        unresolved_dma_diagnostic_gaps: DMA_DRIVER_DIAGNOSTIC_UNRESOLVED_GAPS,
        claims_driver_dma_completion: false,
        claims_hardware_device_completion: false,
        classification: DMA_DRIVER_DIAGNOSTIC_ENVELOPE_LOCAL_STATIC_CLASSIFICATION,
    }
}

pub fn rejected_dma_cache_driver_diagnostic_envelope_evidence(
    error: DmaCacheDriverDiagnosticEnvelopeError,
) -> (&'static str, &'static str) {
    (DMA_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub const fn rp1_dma_controller_source_facts() -> Rp1DmaControllerSourceFacts {
    Rp1DmaControllerSourceFacts {
        compatible: RP1_DMA_CONTROLLER_COMPATIBLE,
        rp1_bus_base: RP1_DMA_CONTROLLER_BUS_BASE,
        cpu_physical_base: RP1_DMA_CONTROLLER_CPU_BASE,
        channel_count: RP1_DMA_CHANNEL_COUNT,
        target_count: RP1_DMA_TARGET_COUNT,
        interrupt_name: RP1_DMA_INTERRUPT_NAME,
        clock_names: RP1_DMA_CLOCK_NAMES,
    }
}

pub fn build_dma_cache_small_diagnostic_plan(
    input: DmaCacheSmallDiagnosticPlanInput,
) -> Result<DmaCacheSmallDiagnosticPlan, DmaCacheSmallDiagnosticPlanError> {
    validate_small_diagnostic_rejected_claims(input)?;
    validate_small_diagnostic_envelope_evidence(input.envelope_evidence)?;
    validate_rp1_dma_controller_source(input.controller_source)?;

    Ok(DmaCacheSmallDiagnosticPlan {
        envelope_evidence: input.envelope_evidence,
        controller_source: input.controller_source,
    })
}

pub fn dma_cache_small_diagnostic_plan_evidence(
    plan: DmaCacheSmallDiagnosticPlan,
) -> DmaCacheSmallDiagnosticPlanEvidence {
    let envelope = plan.envelope_evidence;
    let controller = plan.controller_source;
    DmaCacheSmallDiagnosticPlanEvidence {
        small_diagnostic_plan_contract_id: DMA_CACHE_SMALL_DIAGNOSTIC_PLAN_CONTRACT_ID,
        driver_diagnostic_envelope_contract_id: envelope.driver_diagnostic_envelope_contract_id,
        executor_contract_id: envelope.executor_contract_id,
        maintenance_sequence_contract_id: envelope.maintenance_sequence_contract_id,
        sync_plan_contract_id: envelope.sync_plan_contract_id,
        descriptor_contract_id: envelope.descriptor_contract_id,
        descriptor_source_inventory_id: envelope.descriptor_source_inventory_id,
        rp1_dma_compatible: controller.compatible,
        rp1_dma_controller_rp1_bus_base: controller.rp1_bus_base,
        rp1_dma_controller_cpu_physical_base: controller.cpu_physical_base,
        rp1_dma_channel_count: controller.channel_count,
        rp1_dma_target_count: controller.target_count,
        rp1_dma_interrupt_name: controller.interrupt_name,
        rp1_dma_clock_names: controller.clock_names,
        cpu_physical: envelope.cpu_physical,
        cpu_visible: envelope.cpu_visible,
        rp1_bus_address: envelope.rp1_bus_address,
        descriptor_length: envelope.descriptor_length,
        cache_line_source: envelope.cache_line_source,
        cache_line_size: envelope.cache_line_size,
        line_aligned_cpu_start: envelope.line_aligned_cpu_start,
        covered_length: envelope.covered_length,
        line_count: envelope.line_count,
        direction: envelope.direction,
        cacheability: envelope.cacheability,
        owner_transition: envelope.owner_transition,
        iommu_classification: envelope.iommu_classification,
        prerequisite_rejected_runtime_claims: envelope.prerequisite_rejected_runtime_claims,
        executor_rejected_runtime_claims: envelope.executor_rejected_runtime_claims,
        unresolved_dma_diagnostic_gaps: envelope.unresolved_dma_diagnostic_gaps,
        claims_rp1_channel_ownership: false,
        claims_descriptor_ring_ready: false,
        claims_transfer_completion: false,
        claims_interrupt_completion: false,
        claims_hardware_device_completion: false,
        claims_ethernet_ready: false,
        claims_storage_ready: false,
        claims_networking: false,
        claims_ssh: false,
        classification: DMA_SMALL_DIAGNOSTIC_PLAN_LOCAL_STATIC_CLASSIFICATION,
    }
}

pub fn rejected_dma_cache_small_diagnostic_plan_evidence(
    error: DmaCacheSmallDiagnosticPlanError,
) -> (&'static str, &'static str) {
    (DMA_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_dma_cache_small_diagnostic_visibility_report(
    input: DmaCacheSmallDiagnosticVisibilityReportInput,
) -> Result<DmaCacheSmallDiagnosticVisibilityReport, DmaCacheSmallDiagnosticVisibilityReportError> {
    validate_small_diagnostic_visibility_rejected_claims(input)?;

    match (input.kind, input.plan_evidence) {
        (DmaCacheSmallDiagnosticVisibilityReportKind::Candidate, Some(plan_evidence)) => {
            validate_small_diagnostic_visibility_plan_evidence(plan_evidence)?;
            Ok(DmaCacheSmallDiagnosticVisibilityReport {
                kind: input.kind,
                plan_evidence: Some(plan_evidence),
            })
        }
        (DmaCacheSmallDiagnosticVisibilityReportKind::Candidate, None) => {
            Err(DmaCacheSmallDiagnosticVisibilityReportError::CandidateMissingPlanEvidence)
        }
        (DmaCacheSmallDiagnosticVisibilityReportKind::NoPlanControl, None) => {
            Ok(DmaCacheSmallDiagnosticVisibilityReport {
                kind: input.kind,
                plan_evidence: None,
            })
        }
        (DmaCacheSmallDiagnosticVisibilityReportKind::NoPlanControl, Some(_)) => {
            Err(DmaCacheSmallDiagnosticVisibilityReportError::ControlCarriesPlanEvidence)
        }
    }
}

pub fn dma_cache_small_diagnostic_visibility_report_evidence(
    report: DmaCacheSmallDiagnosticVisibilityReport,
) -> DmaCacheSmallDiagnosticVisibilityReportEvidence {
    match report.plan_evidence {
        Some(plan) => {
            dma_cache_small_diagnostic_visibility_candidate_evidence(report.kind.name(), plan)
        }
        None => dma_cache_small_diagnostic_visibility_control_evidence(report.kind.name()),
    }
}

pub fn rejected_dma_cache_small_diagnostic_visibility_report_evidence(
    error: DmaCacheSmallDiagnosticVisibilityReportError,
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
    if evidence.covered_length == 0 {
        return Err(DmaCacheMaintenanceSequenceError::ZeroCoveredLength);
    }
    evidence
        .line_aligned_cpu_start
        .checked_add(evidence.covered_length)
        .ok_or(DmaCacheMaintenanceSequenceError::RangeOverflow)?;
    if evidence.cache_line_source != BCM2712_CACHE_LINE_SOURCE
        || evidence.cache_line_size != BCM2712_DMA_CACHE_LINE_SIZE
        || evidence.line_aligned_cpu_start & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
        || evidence.covered_length & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
    {
        return Err(DmaCacheMaintenanceSequenceError::CacheLineMismatch);
    }
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

fn validate_accepted_maintenance_sequence_evidence(
    evidence: DmaCacheMaintenanceSequenceEvidence,
) -> Result<DmaCacheMaintenanceInstruction, DmaCacheMaintenanceExecutorError> {
    if evidence.maintenance_sequence_contract_id != DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID
        || evidence.sync_plan_contract_id != DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        || evidence.descriptor_contract_id != DMA_CACHE_SUBSTRATE_CONTRACT_ID
        || evidence.descriptor_source_inventory_id != DMA_CACHE_SOURCE_INVENTORY_ID
    {
        return Err(DmaCacheMaintenanceExecutorError::ContractIdentityMismatch);
    }
    if evidence.classification != DMA_MAINTENANCE_SEQUENCE_LOCAL_STATIC_CLASSIFICATION {
        return Err(DmaCacheMaintenanceExecutorError::NonAcceptedSequenceClassification);
    }
    if evidence.cacheability != DmaCacheability::CacheableRequiresMaintenance.name()
        || evidence.iommu_classification != RP1_DMA_SOURCE_UNASSIGNED_IOMMU
    {
        return Err(DmaCacheMaintenanceExecutorError::WrongCacheabilityIommuIdentity);
    }
    if evidence.rejected_runtime_claims != DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS {
        return Err(DmaCacheMaintenanceExecutorError::MissingRejectedRuntimeClaimsIdentity);
    }
    validate_maintenance_sequence_line_coverage(evidence)?;
    validate_maintenance_sequence_operation_identity(evidence)
}

fn validate_accepted_maintenance_barrier(
    evidence: DmaCacheMaintenanceSequenceEvidence,
) -> Result<DmaCacheMaintenanceBarrier, DmaCacheMaintenanceExecutorError> {
    if evidence.barrier != DmaCacheMaintenanceBarrier::DataSynchronizationBarrierSy.name()
        || evidence.barrier_mnemonic
            != DmaCacheMaintenanceBarrier::DataSynchronizationBarrierSy.instruction()
    {
        return Err(DmaCacheMaintenanceExecutorError::UnsupportedBarrierVocabulary);
    }
    Ok(DmaCacheMaintenanceBarrier::DataSynchronizationBarrierSy)
}

fn validate_maintenance_sequence_line_coverage(
    evidence: DmaCacheMaintenanceSequenceEvidence,
) -> Result<(), DmaCacheMaintenanceExecutorError> {
    if evidence.cache_line_source != BCM2712_CACHE_LINE_SOURCE
        || evidence.cache_line_size != BCM2712_DMA_CACHE_LINE_SIZE
        || evidence.line_aligned_cpu_start & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
        || evidence.covered_length & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
    {
        return Err(DmaCacheMaintenanceExecutorError::LineRangeMismatch);
    }
    if evidence.line_count == 0 || evidence.covered_length == 0 {
        return Err(DmaCacheMaintenanceExecutorError::ZeroLineCoverage);
    }
    let expected_covered_length = evidence
        .line_count
        .checked_mul(evidence.cache_line_size)
        .ok_or(DmaCacheMaintenanceExecutorError::RangeOverflow)?;
    if expected_covered_length != evidence.covered_length {
        return Err(DmaCacheMaintenanceExecutorError::LineRangeMismatch);
    }
    evidence
        .line_aligned_cpu_start
        .checked_add(evidence.covered_length)
        .ok_or(DmaCacheMaintenanceExecutorError::RangeOverflow)?;
    Ok(())
}

fn validate_maintenance_sequence_operation_identity(
    evidence: DmaCacheMaintenanceSequenceEvidence,
) -> Result<DmaCacheMaintenanceInstruction, DmaCacheMaintenanceExecutorError> {
    match (
        evidence.operation,
        evidence.instruction,
        evidence.instruction_mnemonic,
        evidence.direction,
        evidence.owner_transition,
    ) {
        (
            "clean-to-point-of-coherency",
            "clean-by-virtual-address-to-poc",
            "dc cvac",
            "to-device",
            "cpu-to-device",
        ) => Ok(DmaCacheMaintenanceInstruction::CleanByVirtualAddressToPoC),
        (
            "invalidate-from-point-of-coherency",
            "invalidate-by-virtual-address-from-poc",
            "dc ivac",
            "from-device",
            "device-to-cpu",
        ) => Ok(DmaCacheMaintenanceInstruction::InvalidateByVirtualAddressFromPoC),
        (
            "clean-invalidate-to-point-of-coherency",
            "clean-invalidate-by-virtual-address-to-poc",
            "dc civac",
            "bidirectional",
            "shared-cpu-device",
        ) => Ok(DmaCacheMaintenanceInstruction::CleanInvalidateByVirtualAddressToPoC),
        (
            "clean-to-point-of-coherency"
            | "invalidate-from-point-of-coherency"
            | "clean-invalidate-to-point-of-coherency",
            _,
            _,
            _,
            _,
        ) => Err(DmaCacheMaintenanceExecutorError::UnsupportedInstructionVocabulary),
        _ => Err(DmaCacheMaintenanceExecutorError::UnsupportedOperationVocabulary),
    }
}

fn validate_driver_diagnostic_executor_evidence(
    evidence: DmaCacheMaintenanceExecutorEvidence,
) -> Result<(), DmaCacheDriverDiagnosticEnvelopeError> {
    if evidence.executor_contract_id != DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID
        || evidence.maintenance_sequence_contract_id != DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID
        || evidence.sync_plan_contract_id != DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        || evidence.descriptor_contract_id != DMA_CACHE_SUBSTRATE_CONTRACT_ID
        || evidence.descriptor_source_inventory_id != DMA_CACHE_SOURCE_INVENTORY_ID
    {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::MissingPrerequisiteIdentity);
    }
    if evidence.classification != DMA_MAINTENANCE_EXECUTOR_RUNTIME_CLASSIFICATION {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::NonAcceptedExecutorClassification);
    }
    if evidence.cacheability != DmaCacheability::CacheableRequiresMaintenance.name() {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::UnsupportedCacheabilityClaim);
    }
    if evidence.iommu_classification != RP1_DMA_SOURCE_UNASSIGNED_IOMMU {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::UnsupportedIommuClaim);
    }
    if evidence.prerequisite_rejected_runtime_claims != DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS
        || evidence.rejected_runtime_claims != DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS
    {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::MissingRejectedRuntimeClaimsIdentity);
    }
    validate_driver_diagnostic_line_coverage(evidence)
}

fn validate_driver_diagnostic_line_coverage(
    evidence: DmaCacheMaintenanceExecutorEvidence,
) -> Result<(), DmaCacheDriverDiagnosticEnvelopeError> {
    if evidence.cache_line_source != BCM2712_CACHE_LINE_SOURCE
        || evidence.cache_line_size != BCM2712_DMA_CACHE_LINE_SIZE
        || evidence.line_aligned_cpu_start & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
        || evidence.covered_length & (BCM2712_DMA_CACHE_LINE_SIZE - 1) != 0
    {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::LineRangeMismatch);
    }
    if evidence.line_count == 0 || evidence.covered_length == 0 {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::ZeroLineCoverage);
    }
    let expected_covered_length = evidence
        .line_count
        .checked_mul(evidence.cache_line_size)
        .ok_or(DmaCacheDriverDiagnosticEnvelopeError::RangeOverflow)?;
    if expected_covered_length != evidence.covered_length {
        return Err(DmaCacheDriverDiagnosticEnvelopeError::LineRangeMismatch);
    }
    evidence
        .line_aligned_cpu_start
        .checked_add(evidence.covered_length)
        .ok_or(DmaCacheDriverDiagnosticEnvelopeError::RangeOverflow)?;
    Ok(())
}

fn validate_small_diagnostic_rejected_claims(
    input: DmaCacheSmallDiagnosticPlanInput,
) -> Result<(), DmaCacheSmallDiagnosticPlanError> {
    if input.claims_rp1_channel_ownership {
        return Err(DmaCacheSmallDiagnosticPlanError::Rp1ChannelOwnershipClaim);
    }
    if input.claims_descriptor_ring_ready {
        return Err(DmaCacheSmallDiagnosticPlanError::DescriptorRingReadinessClaim);
    }
    if input.claims_transfer_completion {
        return Err(DmaCacheSmallDiagnosticPlanError::TransferCompletionClaim);
    }
    if input.claims_interrupt_completion {
        return Err(DmaCacheSmallDiagnosticPlanError::InterruptCompletionClaim);
    }
    if input.claims_hardware_device_completion {
        return Err(DmaCacheSmallDiagnosticPlanError::HardwareDeviceCompletionClaim);
    }
    if input.claims_ethernet_ready {
        return Err(DmaCacheSmallDiagnosticPlanError::EthernetReadinessClaim);
    }
    if input.claims_storage_ready {
        return Err(DmaCacheSmallDiagnosticPlanError::StorageReadinessClaim);
    }
    if input.claims_networking {
        return Err(DmaCacheSmallDiagnosticPlanError::NetworkingClaim);
    }
    if input.claims_ssh {
        return Err(DmaCacheSmallDiagnosticPlanError::SshClaim);
    }
    Ok(())
}

fn validate_small_diagnostic_envelope_evidence(
    evidence: DmaCacheDriverDiagnosticEnvelopeEvidence,
) -> Result<(), DmaCacheSmallDiagnosticPlanError> {
    if evidence.driver_diagnostic_envelope_contract_id
        != DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID
        || evidence.executor_contract_id != DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID
        || evidence.maintenance_sequence_contract_id != DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID
        || evidence.sync_plan_contract_id != DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        || evidence.descriptor_contract_id != DMA_CACHE_SUBSTRATE_CONTRACT_ID
        || evidence.descriptor_source_inventory_id != DMA_CACHE_SOURCE_INVENTORY_ID
    {
        return Err(DmaCacheSmallDiagnosticPlanError::MissingPrerequisiteIdentity);
    }
    if evidence.classification != DMA_DRIVER_DIAGNOSTIC_ENVELOPE_LOCAL_STATIC_CLASSIFICATION {
        return Err(DmaCacheSmallDiagnosticPlanError::NonAcceptedEnvelopeClassification);
    }
    if evidence.cacheability != DmaCacheability::CacheableRequiresMaintenance.name() {
        return Err(DmaCacheSmallDiagnosticPlanError::UnsupportedCacheabilityClaim);
    }
    if evidence.iommu_classification != RP1_DMA_SOURCE_UNASSIGNED_IOMMU {
        return Err(DmaCacheSmallDiagnosticPlanError::UnsupportedIommuClaim);
    }
    if evidence.claims_driver_dma_completion
        || evidence.claims_hardware_device_completion
        || evidence.executor_rejected_runtime_claims
            != DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS
    {
        return Err(DmaCacheSmallDiagnosticPlanError::MissingRejectedCompletionClaims);
    }
    if evidence.prerequisite_rejected_runtime_claims != DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS {
        return Err(DmaCacheSmallDiagnosticPlanError::MissingPrerequisiteIdentity);
    }
    if evidence.unresolved_dma_diagnostic_gaps != DMA_DRIVER_DIAGNOSTIC_UNRESOLVED_GAPS {
        return Err(DmaCacheSmallDiagnosticPlanError::MissingUnresolvedDiagnosticGaps);
    }
    Ok(())
}

fn validate_rp1_dma_controller_source(
    source: Rp1DmaControllerSourceFacts,
) -> Result<(), DmaCacheSmallDiagnosticPlanError> {
    if source.channel_count == 0 {
        return Err(DmaCacheSmallDiagnosticPlanError::ZeroChannelCount);
    }
    if source.compatible != RP1_DMA_CONTROLLER_COMPATIBLE
        || source.target_count != RP1_DMA_TARGET_COUNT
        || source.interrupt_name != RP1_DMA_INTERRUPT_NAME
        || source.clock_names != RP1_DMA_CLOCK_NAMES
    {
        return Err(DmaCacheSmallDiagnosticPlanError::InvalidTranslatedControllerBase);
    }
    let translated_base = source
        .rp1_bus_base
        .checked_sub(RP1_PERIPHERAL_WINDOW_BASE)
        .and_then(|offset| RP1_PERIPHERAL_WINDOW_CPU_BASE.checked_add(offset))
        .ok_or(DmaCacheSmallDiagnosticPlanError::InvalidTranslatedControllerBase)?;
    let translated_end = translated_base
        .checked_add(1)
        .ok_or(DmaCacheSmallDiagnosticPlanError::InvalidTranslatedControllerBase)?;
    let window_end = RP1_PERIPHERAL_WINDOW_CPU_BASE
        .checked_add(RP1_PERIPHERAL_WINDOW_SIZE)
        .ok_or(DmaCacheSmallDiagnosticPlanError::InvalidTranslatedControllerBase)?;
    if source.rp1_bus_base != RP1_DMA_CONTROLLER_BUS_BASE
        || source.cpu_physical_base != RP1_DMA_CONTROLLER_CPU_BASE
        || translated_base != source.cpu_physical_base
        || translated_end > window_end
    {
        return Err(DmaCacheSmallDiagnosticPlanError::InvalidTranslatedControllerBase);
    }
    Ok(())
}

fn validate_small_diagnostic_visibility_rejected_claims(
    input: DmaCacheSmallDiagnosticVisibilityReportInput,
) -> Result<(), DmaCacheSmallDiagnosticVisibilityReportError> {
    if input.claims_rp1_mmio_writes {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::Rp1MmioWriteClaim);
    }
    if input.claims_rp1_channel_ownership {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::Rp1ChannelOwnershipClaim);
    }
    if input.claims_dma_channel_programming {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::DmaChannelProgrammingClaim);
    }
    if input.claims_descriptor_ring_ready {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::DescriptorRingReadinessClaim);
    }
    if input.claims_transfer_completion {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::TransferCompletionClaim);
    }
    if input.claims_interrupt_completion {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::InterruptCompletionClaim);
    }
    if input.claims_hardware_device_completion {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::HardwareDeviceCompletionClaim);
    }
    if input.claims_ethernet_ready {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::EthernetReadinessClaim);
    }
    if input.claims_storage_ready {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::StorageReadinessClaim);
    }
    if input.claims_networking {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::NetworkingClaim);
    }
    if input.claims_ssh {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::SshClaim);
    }
    if input.claims_milestone_11_3_completion {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::Milestone113CompletionClaim);
    }
    if input.claims_phase_transition {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_small_diagnostic_visibility_plan_evidence(
    evidence: DmaCacheSmallDiagnosticPlanEvidence,
) -> Result<(), DmaCacheSmallDiagnosticVisibilityReportError> {
    if evidence.classification != DMA_SMALL_DIAGNOSTIC_PLAN_LOCAL_STATIC_CLASSIFICATION {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::NonAcceptedPlanClassification);
    }
    if evidence.small_diagnostic_plan_contract_id != DMA_CACHE_SMALL_DIAGNOSTIC_PLAN_CONTRACT_ID
        || evidence.driver_diagnostic_envelope_contract_id
            != DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID
        || evidence.executor_contract_id != DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID
        || evidence.maintenance_sequence_contract_id != DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID
        || evidence.sync_plan_contract_id != DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        || evidence.descriptor_contract_id != DMA_CACHE_SUBSTRATE_CONTRACT_ID
        || evidence.descriptor_source_inventory_id != DMA_CACHE_SOURCE_INVENTORY_ID
    {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::MissingPrerequisiteIdentity);
    }
    if evidence.cacheability != DmaCacheability::CacheableRequiresMaintenance.name() {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::UnsupportedCacheabilityClaim);
    }
    if evidence.iommu_classification != RP1_DMA_SOURCE_UNASSIGNED_IOMMU {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::UnsupportedIommuClaim);
    }
    if evidence.prerequisite_rejected_runtime_claims != DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS
        || evidence.executor_rejected_runtime_claims
            != DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS
    {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::MissingRejectedRuntimeClaims);
    }
    if evidence.unresolved_dma_diagnostic_gaps != DMA_DRIVER_DIAGNOSTIC_UNRESOLVED_GAPS {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::MissingUnresolvedDiagnosticGaps);
    }
    if evidence.claims_rp1_channel_ownership
        || evidence.claims_descriptor_ring_ready
        || evidence.claims_transfer_completion
        || evidence.claims_interrupt_completion
        || evidence.claims_hardware_device_completion
        || evidence.claims_ethernet_ready
        || evidence.claims_storage_ready
        || evidence.claims_networking
        || evidence.claims_ssh
    {
        return Err(DmaCacheSmallDiagnosticVisibilityReportError::MissingRejectedRuntimeClaims);
    }
    Ok(())
}

fn dma_cache_small_diagnostic_visibility_candidate_evidence(
    report_kind: &'static str,
    plan: DmaCacheSmallDiagnosticPlanEvidence,
) -> DmaCacheSmallDiagnosticVisibilityReportEvidence {
    DmaCacheSmallDiagnosticVisibilityReportEvidence {
        visibility_report_contract_id: DMA_CACHE_SMALL_DIAGNOSTIC_VISIBILITY_REPORT_CONTRACT_ID,
        source_contract_id: DMA_CACHE_SMALL_DIAGNOSTIC_SOURCE_CONTRACT_ID,
        report_kind,
        small_diagnostic_plan_contract_id: Some(plan.small_diagnostic_plan_contract_id),
        driver_diagnostic_envelope_contract_id: Some(plan.driver_diagnostic_envelope_contract_id),
        executor_contract_id: Some(plan.executor_contract_id),
        maintenance_sequence_contract_id: Some(plan.maintenance_sequence_contract_id),
        sync_plan_contract_id: Some(plan.sync_plan_contract_id),
        descriptor_contract_id: Some(plan.descriptor_contract_id),
        descriptor_source_inventory_id: Some(plan.descriptor_source_inventory_id),
        rp1_dma_compatible: Some(plan.rp1_dma_compatible),
        rp1_dma_controller_rp1_bus_base: Some(plan.rp1_dma_controller_rp1_bus_base),
        rp1_dma_controller_cpu_physical_base: Some(plan.rp1_dma_controller_cpu_physical_base),
        rp1_dma_channel_count: Some(plan.rp1_dma_channel_count),
        rp1_dma_target_count: Some(plan.rp1_dma_target_count),
        rp1_dma_interrupt_name: Some(plan.rp1_dma_interrupt_name),
        rp1_dma_clock_names: Some(plan.rp1_dma_clock_names),
        cpu_physical: Some(plan.cpu_physical),
        cpu_visible: Some(plan.cpu_visible),
        rp1_bus_address: Some(plan.rp1_bus_address),
        descriptor_length: Some(plan.descriptor_length),
        cache_line_source: Some(plan.cache_line_source),
        cache_line_size: Some(plan.cache_line_size),
        line_aligned_cpu_start: Some(plan.line_aligned_cpu_start),
        covered_length: Some(plan.covered_length),
        line_count: Some(plan.line_count),
        direction: Some(plan.direction),
        cacheability: Some(plan.cacheability),
        owner_transition: Some(plan.owner_transition),
        iommu_classification: Some(plan.iommu_classification),
        prerequisite_rejected_runtime_claims: Some(plan.prerequisite_rejected_runtime_claims),
        executor_rejected_runtime_claims: Some(plan.executor_rejected_runtime_claims),
        unresolved_dma_diagnostic_gaps: Some(plan.unresolved_dma_diagnostic_gaps),
        hardware_proof_boundary_classification:
            DMA_SMALL_DIAGNOSTIC_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_hardware_claims: DMA_SMALL_DIAGNOSTIC_VISIBILITY_REJECTED_HARDWARE_CLAIMS,
        retained_risks: DMA_SMALL_DIAGNOSTIC_VISIBILITY_RETAINED_RISKS,
        claims_rp1_mmio_writes: false,
        claims_rp1_channel_ownership: false,
        claims_dma_channel_programming: false,
        claims_descriptor_ring_ready: false,
        claims_transfer_completion: false,
        claims_interrupt_completion: false,
        claims_hardware_device_completion: false,
        claims_ethernet_ready: false,
        claims_storage_ready: false,
        claims_networking: false,
        claims_ssh: false,
        claims_milestone_11_3_completion: false,
        claims_phase_transition: false,
        classification: DMA_SMALL_DIAGNOSTIC_VISIBILITY_CANDIDATE_CLASSIFICATION,
    }
}

fn dma_cache_small_diagnostic_visibility_control_evidence(
    report_kind: &'static str,
) -> DmaCacheSmallDiagnosticVisibilityReportEvidence {
    DmaCacheSmallDiagnosticVisibilityReportEvidence {
        visibility_report_contract_id: DMA_CACHE_SMALL_DIAGNOSTIC_VISIBILITY_REPORT_CONTRACT_ID,
        source_contract_id: DMA_CACHE_SMALL_DIAGNOSTIC_SOURCE_CONTRACT_ID,
        report_kind,
        small_diagnostic_plan_contract_id: None,
        driver_diagnostic_envelope_contract_id: None,
        executor_contract_id: None,
        maintenance_sequence_contract_id: None,
        sync_plan_contract_id: None,
        descriptor_contract_id: None,
        descriptor_source_inventory_id: None,
        rp1_dma_compatible: None,
        rp1_dma_controller_rp1_bus_base: None,
        rp1_dma_controller_cpu_physical_base: None,
        rp1_dma_channel_count: None,
        rp1_dma_target_count: None,
        rp1_dma_interrupt_name: None,
        rp1_dma_clock_names: None,
        cpu_physical: None,
        cpu_visible: None,
        rp1_bus_address: None,
        descriptor_length: None,
        cache_line_source: None,
        cache_line_size: None,
        line_aligned_cpu_start: None,
        covered_length: None,
        line_count: None,
        direction: None,
        cacheability: None,
        owner_transition: None,
        iommu_classification: None,
        prerequisite_rejected_runtime_claims: None,
        executor_rejected_runtime_claims: None,
        unresolved_dma_diagnostic_gaps: None,
        hardware_proof_boundary_classification:
            DMA_SMALL_DIAGNOSTIC_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_hardware_claims: DMA_SMALL_DIAGNOSTIC_VISIBILITY_REJECTED_HARDWARE_CLAIMS,
        retained_risks: DMA_SMALL_DIAGNOSTIC_VISIBILITY_RETAINED_RISKS,
        claims_rp1_mmio_writes: false,
        claims_rp1_channel_ownership: false,
        claims_dma_channel_programming: false,
        claims_descriptor_ring_ready: false,
        claims_transfer_completion: false,
        claims_interrupt_completion: false,
        claims_hardware_device_completion: false,
        claims_ethernet_ready: false,
        claims_storage_ready: false,
        claims_networking: false,
        claims_ssh: false,
        claims_milestone_11_3_completion: false,
        claims_phase_transition: false,
        classification: DMA_SMALL_DIAGNOSTIC_VISIBILITY_CONTROL_CLASSIFICATION,
    }
}

#[cfg(target_arch = "aarch64")]
fn dispatch_dma_cache_maintenance_instruction(
    instruction: DmaCacheMaintenanceInstruction,
    address: u64,
) {
    let addr = address as usize;
    unsafe {
        match instruction {
            DmaCacheMaintenanceInstruction::CleanByVirtualAddressToPoC => {
                core::arch::asm!("dc cvac, {addr}", addr = in(reg) addr, options(nostack, preserves_flags));
            }
            DmaCacheMaintenanceInstruction::InvalidateByVirtualAddressFromPoC => {
                core::arch::asm!("dc ivac, {addr}", addr = in(reg) addr, options(nostack, preserves_flags));
            }
            DmaCacheMaintenanceInstruction::CleanInvalidateByVirtualAddressToPoC => {
                core::arch::asm!("dc civac, {addr}", addr = in(reg) addr, options(nostack, preserves_flags));
            }
        }
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn dispatch_dma_cache_maintenance_instruction(
    _instruction: DmaCacheMaintenanceInstruction,
    _address: u64,
) {
}

#[cfg(target_arch = "aarch64")]
fn dispatch_dma_cache_maintenance_barrier(_barrier: DmaCacheMaintenanceBarrier) {
    unsafe {
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn dispatch_dma_cache_maintenance_barrier(_barrier: DmaCacheMaintenanceBarrier) {}

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

    fn accepted_maintenance_sequence_evidence(
        direction: DmaDirection,
        boundary: DmaCacheSyncBoundary,
    ) -> DmaCacheMaintenanceSequenceEvidence {
        let sequence =
            derive_dma_cache_maintenance_sequence(accepted_sync_plan_evidence(direction, boundary))
                .expect("valid sequence");
        dma_cache_maintenance_sequence_evidence(sequence)
    }

    fn accepted_executor_evidence(
        direction: DmaDirection,
        boundary: DmaCacheSyncBoundary,
    ) -> DmaCacheMaintenanceExecutorEvidence {
        execute_dma_cache_maintenance_sequence(accepted_maintenance_sequence_evidence(
            direction, boundary,
        ))
        .expect("valid executor input")
    }

    fn accepted_driver_diagnostic_envelope_input() -> DmaCacheDriverDiagnosticEnvelopeInput {
        DmaCacheDriverDiagnosticEnvelopeInput {
            executor_evidence: accepted_executor_evidence(
                DmaDirection::ToDevice,
                DmaCacheSyncBoundary::BeforeDeviceOwnership,
            ),
            claims_driver_dma_completion: false,
            claims_hardware_device_completion: false,
        }
    }

    fn accepted_driver_diagnostic_envelope_evidence() -> DmaCacheDriverDiagnosticEnvelopeEvidence {
        let envelope =
            build_dma_cache_driver_diagnostic_envelope(accepted_driver_diagnostic_envelope_input())
                .expect("valid diagnostic envelope input");
        dma_cache_driver_diagnostic_envelope_evidence(envelope)
    }

    fn accepted_small_diagnostic_plan_input() -> DmaCacheSmallDiagnosticPlanInput {
        DmaCacheSmallDiagnosticPlanInput {
            envelope_evidence: accepted_driver_diagnostic_envelope_evidence(),
            controller_source: rp1_dma_controller_source_facts(),
            claims_rp1_channel_ownership: false,
            claims_descriptor_ring_ready: false,
            claims_transfer_completion: false,
            claims_interrupt_completion: false,
            claims_hardware_device_completion: false,
            claims_ethernet_ready: false,
            claims_storage_ready: false,
            claims_networking: false,
            claims_ssh: false,
        }
    }

    fn accepted_small_diagnostic_plan_evidence() -> DmaCacheSmallDiagnosticPlanEvidence {
        let plan = build_dma_cache_small_diagnostic_plan(accepted_small_diagnostic_plan_input())
            .expect("valid small diagnostic plan input");
        dma_cache_small_diagnostic_plan_evidence(plan)
    }

    fn accepted_visibility_candidate_input() -> DmaCacheSmallDiagnosticVisibilityReportInput {
        DmaCacheSmallDiagnosticVisibilityReportInput {
            kind: DmaCacheSmallDiagnosticVisibilityReportKind::Candidate,
            plan_evidence: Some(accepted_small_diagnostic_plan_evidence()),
            claims_rp1_mmio_writes: false,
            claims_rp1_channel_ownership: false,
            claims_dma_channel_programming: false,
            claims_descriptor_ring_ready: false,
            claims_transfer_completion: false,
            claims_interrupt_completion: false,
            claims_hardware_device_completion: false,
            claims_ethernet_ready: false,
            claims_storage_ready: false,
            claims_networking: false,
            claims_ssh: false,
            claims_milestone_11_3_completion: false,
            claims_phase_transition: false,
        }
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
    fn maintenance_executor_dispatches_clean_sequence_and_formats_evidence() {
        let evidence = accepted_maintenance_sequence_evidence(
            DmaDirection::ToDevice,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        );

        let execution =
            execute_dma_cache_maintenance_sequence(evidence).expect("valid executor input");

        assert_eq!(
            execution.executor_contract_id,
            DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID
        );
        assert_eq!(
            execution.maintenance_sequence_contract_id,
            DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID
        );
        assert_eq!(
            execution.sync_plan_contract_id,
            DMA_CACHE_SYNC_PLAN_CONTRACT_ID
        );
        assert_eq!(
            execution.descriptor_contract_id,
            DMA_CACHE_SUBSTRATE_CONTRACT_ID
        );
        assert_eq!(
            execution.descriptor_source_inventory_id,
            DMA_CACHE_SOURCE_INVENTORY_ID
        );
        assert_eq!(execution.operation, "clean-to-point-of-coherency");
        assert_eq!(execution.instruction, "clean-by-virtual-address-to-poc");
        assert_eq!(execution.instruction_mnemonic, "dc cvac");
        assert_eq!(execution.barrier, "data-synchronization-barrier-sy");
        assert_eq!(execution.barrier_mnemonic, "dsb sy");
        assert_eq!(execution.cache_line_source, BCM2712_CACHE_LINE_SOURCE);
        assert_eq!(execution.cache_line_size, 64);
        assert_eq!(execution.line_aligned_cpu_start, 0x2f02_0000);
        assert_eq!(execution.covered_length, 0x2000);
        assert_eq!(execution.line_count, 128);
        assert_eq!(execution.cpu_physical, 0x2f02_0000);
        assert_eq!(execution.cpu_visible, 0x2f02_0000);
        assert_eq!(execution.rp1_bus_address, 0x10_2f02_0000);
        assert_eq!(execution.descriptor_length, 0x2000);
        assert_eq!(execution.direction, "to-device");
        assert_eq!(execution.cacheability, "cacheable-requires-maintenance");
        assert_eq!(execution.owner_transition, "cpu-to-device");
        assert_eq!(
            execution.iommu_classification,
            RP1_DMA_SOURCE_UNASSIGNED_IOMMU
        );
        assert_eq!(
            execution.prerequisite_rejected_runtime_claims,
            DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            execution.rejected_runtime_claims,
            DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            execution.classification,
            DMA_MAINTENANCE_EXECUTOR_RUNTIME_CLASSIFICATION
        );
    }

    #[test_case]
    fn maintenance_executor_accepts_invalidate_and_clean_invalidate_dispatch_vocabularies() {
        let invalidate =
            execute_dma_cache_maintenance_sequence(accepted_maintenance_sequence_evidence(
                DmaDirection::FromDevice,
                DmaCacheSyncBoundary::AfterDeviceOwnership,
            ))
            .expect("valid invalidate executor input");
        let clean_invalidate =
            execute_dma_cache_maintenance_sequence(accepted_maintenance_sequence_evidence(
                DmaDirection::Bidirectional,
                DmaCacheSyncBoundary::SharedSynchronizationBoundary,
            ))
            .expect("valid clean+invalidate executor input");

        assert_eq!(invalidate.operation, "invalidate-from-point-of-coherency");
        assert_eq!(invalidate.instruction_mnemonic, "dc ivac");
        assert_eq!(
            clean_invalidate.operation,
            "clean-invalidate-to-point-of-coherency"
        );
        assert_eq!(clean_invalidate.instruction_mnemonic, "dc civac");
        assert_eq!(clean_invalidate.barrier_mnemonic, "dsb sy");
    }

    #[test_case]
    fn maintenance_executor_rejects_contract_classification_and_identity_bypass() {
        let evidence = accepted_maintenance_sequence_evidence(
            DmaDirection::ToDevice,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        );

        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                maintenance_sequence_contract_id: "wrong-maintenance-sequence-contract",
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::ContractIdentityMismatch)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                classification: DMA_REJECTED_INPUT_CLASSIFICATION,
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::NonAcceptedSequenceClassification)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                cacheability: "non-cacheable-mapping-unaccepted",
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::WrongCacheabilityIommuIdentity)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                iommu_classification: "unknown-iommu-unaccepted",
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::WrongCacheabilityIommuIdentity)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                rejected_runtime_claims: &["executed-driver-buffer-cache-maintenance"],
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::MissingRejectedRuntimeClaimsIdentity)
        );
    }

    #[test_case]
    fn maintenance_executor_rejects_line_coverage_overflow_and_range_mismatch() {
        let evidence = accepted_maintenance_sequence_evidence(
            DmaDirection::ToDevice,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        );

        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                line_count: 0,
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::ZeroLineCoverage)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                covered_length: 0,
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::ZeroLineCoverage)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                covered_length: 64,
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::LineRangeMismatch)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                cache_line_size: 32,
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::LineRangeMismatch)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                line_aligned_cpu_start: u64::MAX - 0x3f,
                covered_length: 0x40,
                line_count: 1,
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::RangeOverflow)
        );
    }

    #[test_case]
    fn maintenance_executor_rejects_unsupported_operation_instruction_and_barrier() {
        let evidence = accepted_maintenance_sequence_evidence(
            DmaDirection::ToDevice,
            DmaCacheSyncBoundary::BeforeDeviceOwnership,
        );

        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                operation: "unsupported-operation",
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::UnsupportedOperationVocabulary)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                instruction_mnemonic: "dc zva",
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::UnsupportedInstructionVocabulary)
        );
        assert_eq!(
            execute_dma_cache_maintenance_sequence(DmaCacheMaintenanceSequenceEvidence {
                barrier_mnemonic: "dsb ish",
                ..evidence
            }),
            Err(DmaCacheMaintenanceExecutorError::UnsupportedBarrierVocabulary)
        );
        assert_eq!(
            rejected_dma_cache_maintenance_executor_evidence(
                DmaCacheMaintenanceExecutorError::UnsupportedBarrierVocabulary
            ),
            (
                DMA_REJECTED_INPUT_CLASSIFICATION,
                "unsupported-barrier-vocabulary"
            )
        );
    }

    #[test_case]
    fn driver_diagnostic_envelope_formats_accepted_executor_evidence() {
        let envelope =
            build_dma_cache_driver_diagnostic_envelope(accepted_driver_diagnostic_envelope_input())
                .expect("valid diagnostic envelope input");
        let evidence = dma_cache_driver_diagnostic_envelope_evidence(envelope);

        assert_eq!(
            evidence.driver_diagnostic_envelope_contract_id,
            DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID
        );
        assert_eq!(
            evidence.executor_contract_id,
            DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID
        );
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
            evidence.prerequisite_rejected_runtime_claims,
            DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.executor_rejected_runtime_claims,
            DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.unresolved_dma_diagnostic_gaps,
            DMA_DRIVER_DIAGNOSTIC_UNRESOLVED_GAPS
        );
        assert!(!evidence.claims_driver_dma_completion);
        assert!(!evidence.claims_hardware_device_completion);
        assert_eq!(
            evidence.classification,
            DMA_DRIVER_DIAGNOSTIC_ENVELOPE_LOCAL_STATIC_CLASSIFICATION
        );
    }

    #[test_case]
    fn driver_diagnostic_envelope_rejects_non_accepted_executor_and_missing_prerequisites() {
        let input = accepted_driver_diagnostic_envelope_input();

        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    classification: DMA_REJECTED_INPUT_CLASSIFICATION,
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::NonAcceptedExecutorClassification)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    descriptor_contract_id: "wrong-descriptor-contract",
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::MissingPrerequisiteIdentity)
        );
    }

    #[test_case]
    fn driver_diagnostic_envelope_rejects_line_coverage_overflow_and_completion_claims() {
        let input = accepted_driver_diagnostic_envelope_input();

        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    line_count: 0,
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::ZeroLineCoverage)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    covered_length: 64,
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::LineRangeMismatch)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    line_aligned_cpu_start: u64::MAX - 0x3f,
                    covered_length: 0x40,
                    line_count: 1,
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::RangeOverflow)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                claims_driver_dma_completion: true,
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::DriverDmaCompletionClaim)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                claims_hardware_device_completion: true,
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::HardwareDeviceCompletionClaim)
        );
    }

    #[test_case]
    fn driver_diagnostic_envelope_rejects_unsupported_claims_and_missing_rejected_claims() {
        let input = accepted_driver_diagnostic_envelope_input();

        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    cacheability: "non-cacheable-mapping-unaccepted",
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::UnsupportedCacheabilityClaim)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    iommu_classification: "unknown-iommu-unaccepted",
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::UnsupportedIommuClaim)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    prerequisite_rejected_runtime_claims: &[
                        "executed-driver-buffer-cache-maintenance"
                    ],
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::MissingRejectedRuntimeClaimsIdentity)
        );
        assert_eq!(
            build_dma_cache_driver_diagnostic_envelope(DmaCacheDriverDiagnosticEnvelopeInput {
                executor_evidence: DmaCacheMaintenanceExecutorEvidence {
                    rejected_runtime_claims: &["driver-dma-completion"],
                    ..input.executor_evidence
                },
                ..input
            }),
            Err(DmaCacheDriverDiagnosticEnvelopeError::MissingRejectedRuntimeClaimsIdentity)
        );
        assert_eq!(
            rejected_dma_cache_driver_diagnostic_envelope_evidence(
                DmaCacheDriverDiagnosticEnvelopeError::HardwareDeviceCompletionClaim
            ),
            (
                DMA_REJECTED_INPUT_CLASSIFICATION,
                "hardware-device-completion-claim"
            )
        );
    }

    #[test_case]
    fn small_diagnostic_plan_formats_accepted_envelope_and_rp1_dma_source_facts() {
        let plan = build_dma_cache_small_diagnostic_plan(accepted_small_diagnostic_plan_input())
            .expect("valid small diagnostic plan input");
        let evidence = dma_cache_small_diagnostic_plan_evidence(plan);

        assert_eq!(
            evidence.small_diagnostic_plan_contract_id,
            DMA_CACHE_SMALL_DIAGNOSTIC_PLAN_CONTRACT_ID
        );
        assert_eq!(
            evidence.driver_diagnostic_envelope_contract_id,
            DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID
        );
        assert_eq!(
            evidence.executor_contract_id,
            DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID
        );
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
        assert_eq!(evidence.rp1_dma_compatible, RP1_DMA_CONTROLLER_COMPATIBLE);
        assert_eq!(
            evidence.rp1_dma_controller_rp1_bus_base,
            RP1_DMA_CONTROLLER_BUS_BASE
        );
        assert_eq!(
            evidence.rp1_dma_controller_cpu_physical_base,
            RP1_DMA_CONTROLLER_CPU_BASE
        );
        assert_eq!(evidence.rp1_dma_channel_count, 8);
        assert_eq!(evidence.rp1_dma_target_count, 64);
        assert_eq!(evidence.rp1_dma_interrupt_name, RP1_DMA_INTERRUPT_NAME);
        assert_eq!(evidence.rp1_dma_clock_names, RP1_DMA_CLOCK_NAMES);
        assert_eq!(evidence.cpu_physical, 0x2f02_0000);
        assert_eq!(evidence.cpu_visible, 0x2f02_0000);
        assert_eq!(evidence.rp1_bus_address, 0x10_2f02_0000);
        assert_eq!(evidence.descriptor_length, 0x2000);
        assert_eq!(evidence.cache_line_source, BCM2712_CACHE_LINE_SOURCE);
        assert_eq!(evidence.cache_line_size, 64);
        assert_eq!(evidence.line_aligned_cpu_start, 0x2f02_0000);
        assert_eq!(evidence.covered_length, 0x2000);
        assert_eq!(evidence.line_count, 128);
        assert_eq!(evidence.direction, "to-device");
        assert_eq!(evidence.cacheability, "cacheable-requires-maintenance");
        assert_eq!(evidence.owner_transition, "cpu-to-device");
        assert_eq!(
            evidence.iommu_classification,
            RP1_DMA_SOURCE_UNASSIGNED_IOMMU
        );
        assert_eq!(
            evidence.prerequisite_rejected_runtime_claims,
            DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.executor_rejected_runtime_claims,
            DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.unresolved_dma_diagnostic_gaps,
            DMA_DRIVER_DIAGNOSTIC_UNRESOLVED_GAPS
        );
        assert!(!evidence.claims_rp1_channel_ownership);
        assert!(!evidence.claims_descriptor_ring_ready);
        assert!(!evidence.claims_transfer_completion);
        assert!(!evidence.claims_interrupt_completion);
        assert!(!evidence.claims_hardware_device_completion);
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_storage_ready);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_ssh);
        assert_eq!(
            evidence.classification,
            DMA_SMALL_DIAGNOSTIC_PLAN_LOCAL_STATIC_CLASSIFICATION
        );
    }

    #[test_case]
    fn small_diagnostic_plan_rejects_non_accepted_envelope_and_missing_prerequisites() {
        let input = accepted_small_diagnostic_plan_input();

        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence {
                    classification: DMA_REJECTED_INPUT_CLASSIFICATION,
                    ..input.envelope_evidence
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::NonAcceptedEnvelopeClassification)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence {
                    executor_contract_id: "wrong-executor-contract",
                    ..input.envelope_evidence
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::MissingPrerequisiteIdentity)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence {
                    unresolved_dma_diagnostic_gaps: &["rp1-dma-channel-ownership"],
                    ..input.envelope_evidence
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::MissingUnresolvedDiagnosticGaps)
        );
    }

    #[test_case]
    fn small_diagnostic_plan_rejects_controller_source_and_policy_claims() {
        let input = accepted_small_diagnostic_plan_input();

        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                controller_source: Rp1DmaControllerSourceFacts {
                    channel_count: 0,
                    ..input.controller_source
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::ZeroChannelCount)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                controller_source: Rp1DmaControllerSourceFacts {
                    cpu_physical_base: RP1_DMA_CONTROLLER_CPU_BASE + 0x1000,
                    ..input.controller_source
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::InvalidTranslatedControllerBase)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence {
                    cacheability: "coherent-hardware-unaccepted",
                    ..input.envelope_evidence
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::UnsupportedCacheabilityClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence {
                    iommu_classification: "unknown-iommu-unaccepted",
                    ..input.envelope_evidence
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::UnsupportedIommuClaim)
        );
    }

    #[test_case]
    fn small_diagnostic_plan_rejects_completion_and_driver_readiness_claims() {
        let input = accepted_small_diagnostic_plan_input();

        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                envelope_evidence: DmaCacheDriverDiagnosticEnvelopeEvidence {
                    claims_driver_dma_completion: true,
                    ..input.envelope_evidence
                },
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::MissingRejectedCompletionClaims)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_rp1_channel_ownership: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::Rp1ChannelOwnershipClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_descriptor_ring_ready: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::DescriptorRingReadinessClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_transfer_completion: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::TransferCompletionClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_interrupt_completion: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::InterruptCompletionClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_hardware_device_completion: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::HardwareDeviceCompletionClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_ethernet_ready: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_storage_ready: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::StorageReadinessClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_networking: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::NetworkingClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_plan(DmaCacheSmallDiagnosticPlanInput {
                claims_ssh: true,
                ..input
            }),
            Err(DmaCacheSmallDiagnosticPlanError::SshClaim)
        );
        assert_eq!(
            rejected_dma_cache_small_diagnostic_plan_evidence(
                DmaCacheSmallDiagnosticPlanError::DescriptorRingReadinessClaim
            ),
            (
                DMA_REJECTED_INPUT_CLASSIFICATION,
                "descriptor-ring-readiness-claim"
            )
        );
    }

    #[test_case]
    fn small_diagnostic_visibility_report_formats_candidate_plan_evidence() {
        let report = build_dma_cache_small_diagnostic_visibility_report(
            accepted_visibility_candidate_input(),
        )
        .expect("valid visibility candidate input");
        let evidence = dma_cache_small_diagnostic_visibility_report_evidence(report);

        assert_eq!(
            evidence.visibility_report_contract_id,
            DMA_CACHE_SMALL_DIAGNOSTIC_VISIBILITY_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            DMA_CACHE_SMALL_DIAGNOSTIC_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.small_diagnostic_plan_contract_id,
            Some(DMA_CACHE_SMALL_DIAGNOSTIC_PLAN_CONTRACT_ID)
        );
        assert_eq!(
            evidence.driver_diagnostic_envelope_contract_id,
            Some(DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID)
        );
        assert_eq!(
            evidence.executor_contract_id,
            Some(DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID)
        );
        assert_eq!(
            evidence.maintenance_sequence_contract_id,
            Some(DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID)
        );
        assert_eq!(
            evidence.sync_plan_contract_id,
            Some(DMA_CACHE_SYNC_PLAN_CONTRACT_ID)
        );
        assert_eq!(
            evidence.descriptor_contract_id,
            Some(DMA_CACHE_SUBSTRATE_CONTRACT_ID)
        );
        assert_eq!(
            evidence.descriptor_source_inventory_id,
            Some(DMA_CACHE_SOURCE_INVENTORY_ID)
        );
        assert_eq!(
            evidence.rp1_dma_compatible,
            Some(RP1_DMA_CONTROLLER_COMPATIBLE)
        );
        assert_eq!(
            evidence.rp1_dma_controller_rp1_bus_base,
            Some(RP1_DMA_CONTROLLER_BUS_BASE)
        );
        assert_eq!(
            evidence.rp1_dma_controller_cpu_physical_base,
            Some(RP1_DMA_CONTROLLER_CPU_BASE)
        );
        assert_eq!(evidence.rp1_dma_channel_count, Some(8));
        assert_eq!(evidence.rp1_dma_target_count, Some(64));
        assert_eq!(
            evidence.rp1_dma_interrupt_name,
            Some(RP1_DMA_INTERRUPT_NAME)
        );
        assert_eq!(evidence.rp1_dma_clock_names, Some(RP1_DMA_CLOCK_NAMES));
        assert_eq!(evidence.cpu_physical, Some(0x2f02_0000));
        assert_eq!(evidence.cpu_visible, Some(0x2f02_0000));
        assert_eq!(evidence.rp1_bus_address, Some(0x10_2f02_0000));
        assert_eq!(evidence.descriptor_length, Some(0x2000));
        assert_eq!(evidence.cache_line_source, Some(BCM2712_CACHE_LINE_SOURCE));
        assert_eq!(evidence.cache_line_size, Some(64));
        assert_eq!(evidence.line_aligned_cpu_start, Some(0x2f02_0000));
        assert_eq!(evidence.covered_length, Some(0x2000));
        assert_eq!(evidence.line_count, Some(128));
        assert_eq!(evidence.direction, Some("to-device"));
        assert_eq!(
            evidence.cacheability,
            Some("cacheable-requires-maintenance")
        );
        assert_eq!(evidence.owner_transition, Some("cpu-to-device"));
        assert_eq!(
            evidence.iommu_classification,
            Some(RP1_DMA_SOURCE_UNASSIGNED_IOMMU)
        );
        assert_eq!(
            evidence.prerequisite_rejected_runtime_claims,
            Some(DMA_SYNC_PLAN_REJECTED_RUNTIME_CLAIMS)
        );
        assert_eq!(
            evidence.executor_rejected_runtime_claims,
            Some(DMA_MAINTENANCE_EXECUTOR_REJECTED_RUNTIME_CLAIMS)
        );
        assert_eq!(
            evidence.unresolved_dma_diagnostic_gaps,
            Some(DMA_DRIVER_DIAGNOSTIC_UNRESOLVED_GAPS)
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            DMA_SMALL_DIAGNOSTIC_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_hardware_claims,
            DMA_SMALL_DIAGNOSTIC_VISIBILITY_REJECTED_HARDWARE_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            DMA_SMALL_DIAGNOSTIC_VISIBILITY_RETAINED_RISKS
        );
        assert!(!evidence.claims_rp1_mmio_writes);
        assert!(!evidence.claims_rp1_channel_ownership);
        assert!(!evidence.claims_dma_channel_programming);
        assert!(!evidence.claims_descriptor_ring_ready);
        assert!(!evidence.claims_transfer_completion);
        assert!(!evidence.claims_interrupt_completion);
        assert!(!evidence.claims_hardware_device_completion);
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_storage_ready);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_ssh);
        assert!(!evidence.claims_milestone_11_3_completion);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            DMA_SMALL_DIAGNOSTIC_VISIBILITY_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn small_diagnostic_visibility_report_formats_no_plan_control() {
        let report = build_dma_cache_small_diagnostic_visibility_report(
            DmaCacheSmallDiagnosticVisibilityReportInput {
                kind: DmaCacheSmallDiagnosticVisibilityReportKind::NoPlanControl,
                plan_evidence: None,
                ..accepted_visibility_candidate_input()
            },
        )
        .expect("valid no-plan control input");
        let evidence = dma_cache_small_diagnostic_visibility_report_evidence(report);

        assert_eq!(
            evidence.visibility_report_contract_id,
            DMA_CACHE_SMALL_DIAGNOSTIC_VISIBILITY_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            DMA_CACHE_SMALL_DIAGNOSTIC_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-plan-control");
        assert_eq!(evidence.small_diagnostic_plan_contract_id, None);
        assert_eq!(evidence.driver_diagnostic_envelope_contract_id, None);
        assert_eq!(evidence.executor_contract_id, None);
        assert_eq!(evidence.maintenance_sequence_contract_id, None);
        assert_eq!(evidence.sync_plan_contract_id, None);
        assert_eq!(evidence.descriptor_contract_id, None);
        assert_eq!(evidence.descriptor_source_inventory_id, None);
        assert_eq!(evidence.rp1_dma_compatible, None);
        assert_eq!(evidence.rp1_dma_controller_rp1_bus_base, None);
        assert_eq!(evidence.rp1_dma_controller_cpu_physical_base, None);
        assert_eq!(evidence.rp1_dma_channel_count, None);
        assert_eq!(evidence.rp1_dma_target_count, None);
        assert_eq!(evidence.rp1_dma_interrupt_name, None);
        assert_eq!(evidence.rp1_dma_clock_names, None);
        assert_eq!(evidence.cpu_physical, None);
        assert_eq!(evidence.cpu_visible, None);
        assert_eq!(evidence.rp1_bus_address, None);
        assert_eq!(evidence.descriptor_length, None);
        assert_eq!(evidence.cache_line_source, None);
        assert_eq!(evidence.cache_line_size, None);
        assert_eq!(evidence.line_aligned_cpu_start, None);
        assert_eq!(evidence.covered_length, None);
        assert_eq!(evidence.line_count, None);
        assert_eq!(evidence.direction, None);
        assert_eq!(evidence.cacheability, None);
        assert_eq!(evidence.owner_transition, None);
        assert_eq!(evidence.iommu_classification, None);
        assert_eq!(evidence.prerequisite_rejected_runtime_claims, None);
        assert_eq!(evidence.executor_rejected_runtime_claims, None);
        assert_eq!(evidence.unresolved_dma_diagnostic_gaps, None);
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            DMA_SMALL_DIAGNOSTIC_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_hardware_claims,
            DMA_SMALL_DIAGNOSTIC_VISIBILITY_REJECTED_HARDWARE_CLAIMS
        );
        assert_eq!(
            evidence.classification,
            DMA_SMALL_DIAGNOSTIC_VISIBILITY_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn small_diagnostic_visibility_report_rejects_shape_and_plan_bypass() {
        let input = accepted_visibility_candidate_input();

        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    plan_evidence: None,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::CandidateMissingPlanEvidence)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    kind: DmaCacheSmallDiagnosticVisibilityReportKind::NoPlanControl,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::ControlCarriesPlanEvidence)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    plan_evidence: Some(DmaCacheSmallDiagnosticPlanEvidence {
                        classification: DMA_REJECTED_INPUT_CLASSIFICATION,
                        ..accepted_small_diagnostic_plan_evidence()
                    }),
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::NonAcceptedPlanClassification)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    plan_evidence: Some(DmaCacheSmallDiagnosticPlanEvidence {
                        executor_contract_id: "wrong-executor-contract",
                        ..accepted_small_diagnostic_plan_evidence()
                    }),
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::MissingPrerequisiteIdentity)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    plan_evidence: Some(DmaCacheSmallDiagnosticPlanEvidence {
                        prerequisite_rejected_runtime_claims: &[
                            "executed-driver-buffer-cache-maintenance"
                        ],
                        ..accepted_small_diagnostic_plan_evidence()
                    }),
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::MissingRejectedRuntimeClaims)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    plan_evidence: Some(DmaCacheSmallDiagnosticPlanEvidence {
                        unresolved_dma_diagnostic_gaps: &["rp1-dma-channel-ownership"],
                        ..accepted_small_diagnostic_plan_evidence()
                    }),
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::MissingUnresolvedDiagnosticGaps)
        );
    }

    #[test_case]
    fn small_diagnostic_visibility_report_rejects_overclaims() {
        let input = accepted_visibility_candidate_input();

        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_rp1_mmio_writes: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::Rp1MmioWriteClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_rp1_channel_ownership: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::Rp1ChannelOwnershipClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_dma_channel_programming: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::DmaChannelProgrammingClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_descriptor_ring_ready: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::DescriptorRingReadinessClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_transfer_completion: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::TransferCompletionClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_interrupt_completion: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::InterruptCompletionClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_hardware_device_completion: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::HardwareDeviceCompletionClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_ethernet_ready: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_storage_ready: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::StorageReadinessClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_networking: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::NetworkingClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_ssh: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::SshClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_milestone_11_3_completion: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::Milestone113CompletionClaim)
        );
        assert_eq!(
            build_dma_cache_small_diagnostic_visibility_report(
                DmaCacheSmallDiagnosticVisibilityReportInput {
                    claims_phase_transition: true,
                    ..input
                }
            ),
            Err(DmaCacheSmallDiagnosticVisibilityReportError::PhaseTransitionClaim)
        );
        assert_eq!(
            rejected_dma_cache_small_diagnostic_visibility_report_evidence(
                DmaCacheSmallDiagnosticVisibilityReportError::PhaseTransitionClaim
            ),
            (DMA_REJECTED_INPUT_CLASSIFICATION, "phase-transition-claim")
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
