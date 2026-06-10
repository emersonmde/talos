pub const RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gem-mid-source-contract-20260609";
pub const RP1_ETHERNET_GEM_MID_DIAGNOSTIC_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gem-mid-diagnostic-report-contract-v1";
pub const RP1_ETHERNET_GEM_MID_CANDIDATE_CLASSIFICATION: &str =
    "local-static-rp1-ethernet-gem-mid-candidate";
pub const RP1_ETHERNET_GEM_MID_CONTROL_CLASSIFICATION: &str =
    "no-ethernet-no-mmio-rp1-ethernet-gem-mid-control";
pub const RP1_ETHERNET_GEM_MID_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-gem-mid-visibility-control-output";
pub const RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION: &str = "contract-rejected-input";

pub const RP1_ETHERNET_COMPATIBLE: &[&str] = &["raspberrypi,rp1-gem", "cdns,macb"];
pub const RP1_ETHERNET_CONTROLLER_NAME: &str = "rp1_eth";
pub const RP1_ETHERNET_RP1_BUS_BASE: u64 = 0xc0_4010_0000;
pub const RP1_ETHERNET_CPU_PHYSICAL_BASE: u64 = 0x1f_0010_0000;
pub const RP1_ETHERNET_GEM_MID_REGISTER: &str = "MACB_MID";
pub const RP1_ETHERNET_GEM_MID_OFFSET: u64 = 0x00fc;
pub const RP1_ETHERNET_GEM_MID_RP1_BUS_TARGET: u64 = 0xc0_4010_00fc;
pub const RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET: u64 = 0x1f_0010_00fc;
pub const RP1_ETHERNET_GEM_MID_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_GEM_MID_ENDIANNESS: &str = "little-endian";
pub const RP1_ETHERNET_GEM_MID_ACCESS: &str = "read-only volatile load";
pub const RP1_ETHERNET_GEM_MID_IDNUM_OFFSET: u8 = 16;
pub const RP1_ETHERNET_GEM_MID_IDNUM_SIZE: u8 = 12;
pub const RP1_ETHERNET_GEM_MID_REV_OFFSET: u8 = 0;
pub const RP1_ETHERNET_GEM_MID_REV_SIZE: u8 = 16;

pub const RP1_ETHERNET_GEM_MID_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-source-contract/source/linux-rpi-6.12-macb.h",
];

pub const RP1_ETHERNET_GEM_MID_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "Ethernet driver readiness",
    "live broad RP1 Ethernet MMIO readiness",
    "RP1 MMIO writes",
    "RP1 DMA programming",
    "descriptor rings",
    "DMA ownership",
    "transfer completion",
    "interrupt completion",
    "clock/reset ownership",
    "PHY reset ownership",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_GEM_MID_RETAINED_RISKS: &[&str] = &[
    "The source-translated CPU physical GEM MID address is not hardware-proven",
    "Endpoint config identity, bridge setup, and outbound window behavior remain retained Phase 11 risks",
    "Clock/reset and PHY reset ownership remain unaccepted",
    "Packet path, descriptors, DMA, and interrupts remain outside this contract",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGemMidSourceContractEvidence {
    pub contract_id: &'static str,
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub rp1_bus_base: u64,
    pub cpu_physical_base: u64,
    pub register: &'static str,
    pub offset: u64,
    pub rp1_bus_target: u64,
    pub cpu_physical_target: u64,
    pub width_bits: u32,
    pub endianness: &'static str,
    pub access: &'static str,
    pub idnum_offset: u8,
    pub idnum_size: u8,
    pub rev_offset: u8,
    pub rev_size: u8,
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGemMidDiagnosticReportKind {
    Candidate,
    NoEthernetNoMmioControl,
}

impl Rp1EthernetGemMidDiagnosticReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoEthernetNoMmioControl => "no-ethernet-no-mmio-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGemMidDiagnosticReportInput {
    pub kind: Rp1EthernetGemMidDiagnosticReportKind,
    pub source_contract: Option<Rp1EthernetGemMidSourceContractEvidence>,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_rp1_mmio_dma_programming: bool,
    pub claims_descriptor_rings: bool,
    pub claims_dma_ownership: bool,
    pub claims_transfer_completion: bool,
    pub claims_interrupt_completion: bool,
    pub claims_clock_reset_ownership: bool,
    pub claims_phy_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGemMidDiagnosticReport {
    pub kind: Rp1EthernetGemMidDiagnosticReportKind,
    pub source_contract: Option<Rp1EthernetGemMidSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGemMidDiagnosticReportEvidence {
    pub diagnostic_report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub report_kind: &'static str,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub register: Option<&'static str>,
    pub offset: Option<u64>,
    pub rp1_bus_base: Option<u64>,
    pub cpu_physical_base: Option<u64>,
    pub rp1_bus_target: Option<u64>,
    pub cpu_physical_target: Option<u64>,
    pub width_bits: Option<u32>,
    pub endianness: Option<&'static str>,
    pub access: Option<&'static str>,
    pub idnum_offset: Option<u8>,
    pub idnum_size: Option<u8>,
    pub rev_offset: Option<u8>,
    pub rev_size: Option<u8>,
    pub source_evidence: Option<&'static [&'static str]>,
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_rp1_mmio_dma_programming: bool,
    pub claims_descriptor_rings: bool,
    pub claims_dma_ownership: bool,
    pub claims_transfer_completion: bool,
    pub claims_interrupt_completion: bool,
    pub claims_clock_reset_ownership: bool,
    pub claims_phy_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGemMidDiagnosticReportError {
    CandidateMissingSourceContract,
    ControlCarriesEthernetMmioTarget,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    Rp1MmioDmaProgrammingClaim,
    DescriptorRingsClaim,
    DmaOwnershipClaim,
    TransferCompletionClaim,
    InterruptCompletionClaim,
    ClockResetOwnershipClaim,
    PhyOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetGemMidDiagnosticReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesEthernetMmioTarget => "control-carries-ethernet-mmio-target",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::Rp1MmioDmaProgrammingClaim => "rp1-mmio-dma-programming-claim",
            Self::DescriptorRingsClaim => "descriptor-rings-claim",
            Self::DmaOwnershipClaim => "dma-ownership-claim",
            Self::TransferCompletionClaim => "transfer-completion-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::ClockResetOwnershipClaim => "clock-reset-ownership-claim",
            Self::PhyOwnershipClaim => "phy-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_gem_mid_source_contract_evidence()
-> Rp1EthernetGemMidSourceContractEvidence {
    Rp1EthernetGemMidSourceContractEvidence {
        contract_id: RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        rp1_bus_base: RP1_ETHERNET_RP1_BUS_BASE,
        cpu_physical_base: RP1_ETHERNET_CPU_PHYSICAL_BASE,
        register: RP1_ETHERNET_GEM_MID_REGISTER,
        offset: RP1_ETHERNET_GEM_MID_OFFSET,
        rp1_bus_target: RP1_ETHERNET_GEM_MID_RP1_BUS_TARGET,
        cpu_physical_target: RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET,
        width_bits: RP1_ETHERNET_GEM_MID_WIDTH_BITS,
        endianness: RP1_ETHERNET_GEM_MID_ENDIANNESS,
        access: RP1_ETHERNET_GEM_MID_ACCESS,
        idnum_offset: RP1_ETHERNET_GEM_MID_IDNUM_OFFSET,
        idnum_size: RP1_ETHERNET_GEM_MID_IDNUM_SIZE,
        rev_offset: RP1_ETHERNET_GEM_MID_REV_OFFSET,
        rev_size: RP1_ETHERNET_GEM_MID_REV_SIZE,
        source_evidence: RP1_ETHERNET_GEM_MID_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_gem_mid_diagnostic_report(
    input: Rp1EthernetGemMidDiagnosticReportInput,
) -> Result<Rp1EthernetGemMidDiagnosticReport, Rp1EthernetGemMidDiagnosticReportError> {
    validate_rp1_ethernet_gem_mid_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetGemMidDiagnosticReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_gem_mid_source_contract(source_contract)?;
            Ok(Rp1EthernetGemMidDiagnosticReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetGemMidDiagnosticReportKind::Candidate, None) => {
            Err(Rp1EthernetGemMidDiagnosticReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetGemMidDiagnosticReportKind::NoEthernetNoMmioControl, None) => {
            Ok(Rp1EthernetGemMidDiagnosticReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetGemMidDiagnosticReportKind::NoEthernetNoMmioControl, Some(_)) => {
            Err(Rp1EthernetGemMidDiagnosticReportError::ControlCarriesEthernetMmioTarget)
        }
    }
}

pub fn rp1_ethernet_gem_mid_diagnostic_report_evidence(
    report: Rp1EthernetGemMidDiagnosticReport,
) -> Rp1EthernetGemMidDiagnosticReportEvidence {
    match report.source_contract {
        Some(source_contract) => {
            rp1_ethernet_gem_mid_candidate_evidence(report.kind.name(), source_contract)
        }
        None => rp1_ethernet_gem_mid_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_gem_mid_diagnostic_report_evidence(
    error: Rp1EthernetGemMidDiagnosticReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_gem_mid_rejected_claims(
    input: Rp1EthernetGemMidDiagnosticReportInput,
) -> Result<(), Rp1EthernetGemMidDiagnosticReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetGemMidDiagnosticReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetGemMidDiagnosticReportError::BroadMmioReadinessClaim);
    }
    if input.claims_rp1_mmio_dma_programming {
        return Err(Rp1EthernetGemMidDiagnosticReportError::Rp1MmioDmaProgrammingClaim);
    }
    if input.claims_descriptor_rings {
        return Err(Rp1EthernetGemMidDiagnosticReportError::DescriptorRingsClaim);
    }
    if input.claims_dma_ownership {
        return Err(Rp1EthernetGemMidDiagnosticReportError::DmaOwnershipClaim);
    }
    if input.claims_transfer_completion {
        return Err(Rp1EthernetGemMidDiagnosticReportError::TransferCompletionClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetGemMidDiagnosticReportError::InterruptCompletionClaim);
    }
    if input.claims_clock_reset_ownership {
        return Err(Rp1EthernetGemMidDiagnosticReportError::ClockResetOwnershipClaim);
    }
    if input.claims_phy_ownership {
        return Err(Rp1EthernetGemMidDiagnosticReportError::PhyOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetGemMidDiagnosticReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetGemMidDiagnosticReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetGemMidDiagnosticReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetGemMidDiagnosticReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetGemMidDiagnosticReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetGemMidDiagnosticReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_gem_mid_source_contract(
    evidence: Rp1EthernetGemMidSourceContractEvidence,
) -> Result<(), Rp1EthernetGemMidDiagnosticReportError> {
    if evidence.contract_id != RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID
        || evidence.controller != RP1_ETHERNET_CONTROLLER_NAME
        || evidence.compatible != RP1_ETHERNET_COMPATIBLE
    {
        return Err(Rp1EthernetGemMidDiagnosticReportError::SourceContractIdentityMismatch);
    }
    if evidence.rp1_bus_base != RP1_ETHERNET_RP1_BUS_BASE
        || evidence.cpu_physical_base != RP1_ETHERNET_CPU_PHYSICAL_BASE
        || evidence.register != RP1_ETHERNET_GEM_MID_REGISTER
        || evidence.offset != RP1_ETHERNET_GEM_MID_OFFSET
        || evidence.rp1_bus_target != RP1_ETHERNET_GEM_MID_RP1_BUS_TARGET
        || evidence.cpu_physical_target != RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET
    {
        return Err(Rp1EthernetGemMidDiagnosticReportError::SourceContractTargetMismatch);
    }
    if evidence.width_bits != RP1_ETHERNET_GEM_MID_WIDTH_BITS
        || evidence.endianness != RP1_ETHERNET_GEM_MID_ENDIANNESS
        || evidence.access != RP1_ETHERNET_GEM_MID_ACCESS
        || evidence.idnum_offset != RP1_ETHERNET_GEM_MID_IDNUM_OFFSET
        || evidence.idnum_size != RP1_ETHERNET_GEM_MID_IDNUM_SIZE
        || evidence.rev_offset != RP1_ETHERNET_GEM_MID_REV_OFFSET
        || evidence.rev_size != RP1_ETHERNET_GEM_MID_REV_SIZE
    {
        return Err(Rp1EthernetGemMidDiagnosticReportError::SourceContractFieldMismatch);
    }
    if evidence.source_evidence != RP1_ETHERNET_GEM_MID_SOURCE_EVIDENCE {
        return Err(Rp1EthernetGemMidDiagnosticReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_gem_mid_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetGemMidSourceContractEvidence,
) -> Rp1EthernetGemMidDiagnosticReportEvidence {
    Rp1EthernetGemMidDiagnosticReportEvidence {
        diagnostic_report_contract_id: RP1_ETHERNET_GEM_MID_DIAGNOSTIC_REPORT_CONTRACT_ID,
        source_contract_id: source_contract.contract_id,
        report_kind,
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        register: Some(source_contract.register),
        offset: Some(source_contract.offset),
        rp1_bus_base: Some(source_contract.rp1_bus_base),
        cpu_physical_base: Some(source_contract.cpu_physical_base),
        rp1_bus_target: Some(source_contract.rp1_bus_target),
        cpu_physical_target: Some(source_contract.cpu_physical_target),
        width_bits: Some(source_contract.width_bits),
        endianness: Some(source_contract.endianness),
        access: Some(source_contract.access),
        idnum_offset: Some(source_contract.idnum_offset),
        idnum_size: Some(source_contract.idnum_size),
        rev_offset: Some(source_contract.rev_offset),
        rev_size: Some(source_contract.rev_size),
        source_evidence: Some(source_contract.source_evidence),
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GEM_MID_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GEM_MID_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GEM_MID_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_rp1_mmio_dma_programming: false,
        claims_descriptor_rings: false,
        claims_dma_ownership: false,
        claims_transfer_completion: false,
        claims_interrupt_completion: false,
        claims_clock_reset_ownership: false,
        claims_phy_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GEM_MID_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_gem_mid_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetGemMidDiagnosticReportEvidence {
    Rp1EthernetGemMidDiagnosticReportEvidence {
        diagnostic_report_contract_id: RP1_ETHERNET_GEM_MID_DIAGNOSTIC_REPORT_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID,
        report_kind,
        controller: None,
        compatible: None,
        register: None,
        offset: None,
        rp1_bus_base: None,
        cpu_physical_base: None,
        rp1_bus_target: None,
        cpu_physical_target: None,
        width_bits: None,
        endianness: None,
        access: None,
        idnum_offset: None,
        idnum_size: None,
        rev_offset: None,
        rev_size: None,
        source_evidence: None,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GEM_MID_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GEM_MID_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GEM_MID_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_rp1_mmio_dma_programming: false,
        claims_descriptor_rings: false,
        claims_dma_ownership: false,
        claims_transfer_completion: false,
        claims_interrupt_completion: false,
        claims_clock_reset_ownership: false,
        claims_phy_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GEM_MID_CONTROL_CLASSIFICATION,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepted_candidate_input() -> Rp1EthernetGemMidDiagnosticReportInput {
        Rp1EthernetGemMidDiagnosticReportInput {
            kind: Rp1EthernetGemMidDiagnosticReportKind::Candidate,
            source_contract: Some(rp1_ethernet_gem_mid_source_contract_evidence()),
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_rp1_mmio_dma_programming: false,
            claims_descriptor_rings: false,
            claims_dma_ownership: false,
            claims_transfer_completion: false,
            claims_interrupt_completion: false,
            claims_clock_reset_ownership: false,
            claims_phy_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_report_formats_candidate_contract() {
        let report = build_rp1_ethernet_gem_mid_diagnostic_report(accepted_candidate_input())
            .expect("valid GEM MID candidate input");
        let evidence = rp1_ethernet_gem_mid_diagnostic_report_evidence(report);

        assert_eq!(
            evidence.diagnostic_report_contract_id,
            RP1_ETHERNET_GEM_MID_DIAGNOSTIC_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(evidence.register, Some(RP1_ETHERNET_GEM_MID_REGISTER));
        assert_eq!(evidence.offset, Some(0x00fc));
        assert_eq!(evidence.rp1_bus_base, Some(0xc0_4010_0000));
        assert_eq!(evidence.cpu_physical_base, Some(0x1f_0010_0000));
        assert_eq!(evidence.rp1_bus_target, Some(0xc0_4010_00fc));
        assert_eq!(evidence.cpu_physical_target, Some(0x1f_0010_00fc));
        assert_eq!(evidence.width_bits, Some(32));
        assert_eq!(evidence.endianness, Some("little-endian"));
        assert_eq!(evidence.access, Some("read-only volatile load"));
        assert_eq!(evidence.idnum_offset, Some(16));
        assert_eq!(evidence.idnum_size, Some(12));
        assert_eq!(evidence.rev_offset, Some(0));
        assert_eq!(evidence.rev_size, Some(16));
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_GEM_MID_SOURCE_EVIDENCE)
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_GEM_MID_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_GEM_MID_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(evidence.retained_risks, RP1_ETHERNET_GEM_MID_RETAINED_RISKS);
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_broad_mmio_ready);
        assert!(!evidence.claims_rp1_mmio_dma_programming);
        assert!(!evidence.claims_descriptor_rings);
        assert!(!evidence.claims_dma_ownership);
        assert!(!evidence.claims_transfer_completion);
        assert!(!evidence.claims_interrupt_completion);
        assert!(!evidence.claims_clock_reset_ownership);
        assert!(!evidence.claims_phy_ownership);
        assert!(!evidence.claims_packet_io);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_sockets);
        assert!(!evidence.claims_ssh);
        assert!(!evidence.claims_phase_12_2);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GEM_MID_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_report_formats_no_mmio_control() {
        let report =
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                kind: Rp1EthernetGemMidDiagnosticReportKind::NoEthernetNoMmioControl,
                source_contract: None,
                ..accepted_candidate_input()
            })
            .expect("valid GEM MID control input");
        let evidence = rp1_ethernet_gem_mid_diagnostic_report_evidence(report);

        assert_eq!(
            evidence.diagnostic_report_contract_id,
            RP1_ETHERNET_GEM_MID_DIAGNOSTIC_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-ethernet-no-mmio-control");
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.compatible, None);
        assert_eq!(evidence.register, None);
        assert_eq!(evidence.offset, None);
        assert_eq!(evidence.rp1_bus_base, None);
        assert_eq!(evidence.cpu_physical_base, None);
        assert_eq!(evidence.rp1_bus_target, None);
        assert_eq!(evidence.cpu_physical_target, None);
        assert_eq!(evidence.width_bits, None);
        assert_eq!(evidence.source_evidence, None);
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_GEM_MID_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_GEM_MID_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(evidence.retained_risks, RP1_ETHERNET_GEM_MID_RETAINED_RISKS);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GEM_MID_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_report_rejects_shape_and_contract_bypass() {
        let input = accepted_candidate_input();

        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                source_contract: None,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                kind: Rp1EthernetGemMidDiagnosticReportKind::NoEthernetNoMmioControl,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::ControlCarriesEthernetMmioTarget)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                source_contract: Some(Rp1EthernetGemMidSourceContractEvidence {
                    contract_id: "wrong-contract",
                    ..rp1_ethernet_gem_mid_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                source_contract: Some(Rp1EthernetGemMidSourceContractEvidence {
                    cpu_physical_target: 0x1f_0010_0000,
                    ..rp1_ethernet_gem_mid_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                source_contract: Some(Rp1EthernetGemMidSourceContractEvidence {
                    width_bits: 64,
                    ..rp1_ethernet_gem_mid_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                source_contract: Some(Rp1EthernetGemMidSourceContractEvidence {
                    source_evidence: &[],
                    ..rp1_ethernet_gem_mid_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::MissingSourceEvidence)
        );
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_report_rejects_overclaims() {
        let input = accepted_candidate_input();

        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_ethernet_ready: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_broad_mmio_ready: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::BroadMmioReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_rp1_mmio_dma_programming: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::Rp1MmioDmaProgrammingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_descriptor_rings: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::DescriptorRingsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_dma_ownership: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::DmaOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_transfer_completion: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::TransferCompletionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_interrupt_completion: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::InterruptCompletionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_clock_reset_ownership: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::ClockResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_phy_ownership: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::PhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_packet_io: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_networking: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_sockets: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::SocketsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_ssh: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_phase_12_2: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::Phase122Claim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_diagnostic_report(Rp1EthernetGemMidDiagnosticReportInput {
                claims_phase_transition: true,
                ..input
            }),
            Err(Rp1EthernetGemMidDiagnosticReportError::PhaseTransitionClaim)
        );
        assert_eq!(
            rejected_rp1_ethernet_gem_mid_diagnostic_report_evidence(
                Rp1EthernetGemMidDiagnosticReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }
}
