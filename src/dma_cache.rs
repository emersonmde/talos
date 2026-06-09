use crate::memory_map::EarlyPageFrameSpan;

pub const DMA_CACHE_SUBSTRATE_CONTRACT_ID: &str = "phase11-rp1-dma-cache-substrate-contract-v1";
pub const DMA_CACHE_SOURCE_INVENTORY_ID: &str = "phase11-rp1-dma-cache-source-inventory-20260609";
pub const DMA_LOCAL_STATIC_CLASSIFICATION: &str = "local-static-dma-cache-contract-visible";
pub const DMA_REJECTED_INPUT_CLASSIFICATION: &str = "contract-rejected-input";
pub const DMA_STAGING_BLOCKER_CLASSIFICATION: &str = "staging/build-blocker";
pub const RP1_DMA_SOURCE_UNASSIGNED_IOMMU: &str = "source-unassigned-rp1-dma";

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
