pub const RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gem-mid-source-contract-20260609";
pub const RP1_ETHERNET_GEM_MID_DIAGNOSTIC_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gem-mid-diagnostic-report-contract-v1";
pub const RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gem-mid-decode-discriminator-contract-v1";
pub const RP1_ETHERNET_OBSERVED_WINDOW_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-observed-window-contract-v1";
pub const RP1_ETHERNET_OBSERVED_WINDOW_DISCRIMINATOR_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-observed-window-discriminator-contract-v1";
pub const RP1_ETHERNET_GEM_MID_CANDIDATE_CLASSIFICATION: &str =
    "local-static-rp1-ethernet-gem-mid-candidate";
pub const RP1_ETHERNET_GEM_MID_CONTROL_CLASSIFICATION: &str =
    "no-ethernet-no-mmio-rp1-ethernet-gem-mid-control";
pub const RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTROL_CLASSIFICATION: &str =
    "no-mmio-no-ethernet-rp1-ethernet-gem-mid-decode-discriminator-control";
pub const RP1_ETHERNET_OBSERVED_WINDOW_CANDIDATE_CLASSIFICATION: &str =
    "local-static-rp1-ethernet-observed-window-gem-mid-candidate";
pub const RP1_ETHERNET_OBSERVED_WINDOW_CONTROL_CLASSIFICATION: &str =
    "no-mmio-no-ethernet-rp1-ethernet-observed-window-control";
pub const RP1_ETHERNET_GEM_MID_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-gem-mid-visibility-control-output";
pub const RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-observed-sysinfo-gem-mid-discriminator-control-output";
pub const RP1_ETHERNET_OBSERVED_WINDOW_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-observed-window-gem-mid-discriminator-control-output";
pub const RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION: &str = "contract-rejected-input";

pub const RP1_ETHERNET_COMPATIBLE: &[&str] = &["raspberrypi,rp1-gem", "cdns,macb"];
pub const RP1_ETHERNET_CONTROLLER_NAME: &str = "rp1_eth";
pub const RP1_ETHERNET_RP1_BUS_BASE: u64 = 0xc0_4010_0000;
pub const RP1_ETHERNET_CPU_PHYSICAL_BASE: u64 = 0x1f_0010_0000;
pub const RP1_ETHERNET_GEM_MID_REGISTER: &str = "MACB_MID";
pub const RP1_ETHERNET_GEM_MID_OFFSET: u64 = 0x00fc;
pub const RP1_ETHERNET_GEM_MID_RP1_BUS_TARGET: u64 = 0xc0_4010_00fc;
pub const RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET: u64 = 0x1f_0010_00fc;
pub const RP1_ETHERNET_OBSERVED_RP1_BASE: u64 = 0x1c_0000_0000;
pub const RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_SOURCE_OFFSET: u64 = 0x0010_00fc;
pub const RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET: u64 =
    RP1_ETHERNET_OBSERVED_RP1_BASE + RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_SOURCE_OFFSET;
pub const RP1_ETHERNET_GEM_MID_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_GEM_MID_ENDIANNESS: &str = "little-endian";
pub const RP1_ETHERNET_GEM_MID_ACCESS: &str = "read-only volatile load";
pub const RP1_ETHERNET_GEM_MID_IDNUM_OFFSET: u8 = 16;
pub const RP1_ETHERNET_GEM_MID_IDNUM_SIZE: u8 = 12;
pub const RP1_ETHERNET_GEM_MID_REV_OFFSET: u8 = 0;
pub const RP1_ETHERNET_GEM_MID_REV_SIZE: u8 = 16;
pub const RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_REGISTER: &str = "SYSINFO_CHIP_ID";
pub const RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_CPU_PHYSICAL_TARGET: u64 = 0x1c_0000_0000;
pub const RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_EXPECTED: u32 = 0x2000_1927;
pub const RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_ACCESS: &str = "read-only volatile load";
pub const RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-gem-mid-blocker-reconciliation-closeout-20260610";
pub const RP1_ETHERNET_OBSERVED_WINDOW_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-observed-window-contract-20260610";

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

pub const RP1_ETHERNET_OBSERVED_WINDOW_RETAINED_RISKS: &[&str] = &[
    "The observed-window MACB_MID target may still return a sentinel or fault",
    "PCI/RP1 bridge or address-window ownership remains unaccepted",
    "Ethernet clock/reset and PHY/MDIO ownership remain unaccepted",
    "Future hardware proof requires capture-chain-v4-style candidate/control evidence",
];

pub const RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_EXPECTED_CLASSIFICATIONS: &[&str] = &[
    "observed-rp1-positive-control-gem-mid-0x1f-window-sentinel",
    "observed-rp1-positive-control-and-gem-mid-visible",
    "observed-rp1-positive-control-sentinel",
    "staging/build-blocker",
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
pub struct Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence {
    pub register: &'static str,
    pub cpu_physical_target: u64,
    pub width_bits: u32,
    pub expected_value: u32,
    pub access: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGemMidDecodeDiscriminatorReportKind {
    Candidate,
    NoMmioNoEthernetControl,
}

impl Rp1EthernetGemMidDecodeDiscriminatorReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoMmioNoEthernetControl => "no-mmio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGemMidDecodeDiscriminatorReportInput {
    pub kind: Rp1EthernetGemMidDecodeDiscriminatorReportKind,
    pub source_contract: Option<Rp1EthernetGemMidSourceContractEvidence>,
    pub observed_sysinfo_positive_control:
        Option<Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence>,
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
pub struct Rp1EthernetGemMidDecodeDiscriminatorReport {
    pub kind: Rp1EthernetGemMidDecodeDiscriminatorReportKind,
    pub source_contract: Option<Rp1EthernetGemMidSourceContractEvidence>,
    pub observed_sysinfo_positive_control:
        Option<Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGemMidDecodeDiscriminatorReportEvidence {
    pub discriminator_contract_id: &'static str,
    pub selected_by_task_id: &'static str,
    pub source_contract_id: &'static str,
    pub report_kind: &'static str,
    pub same_run_required: bool,
    pub changed_from_gem_mid_only_proof: bool,
    pub observed_positive_control_register: Option<&'static str>,
    pub observed_positive_control_cpu_physical_target: Option<u64>,
    pub observed_positive_control_width_bits: Option<u32>,
    pub observed_positive_control_expected_value: Option<u32>,
    pub observed_positive_control_access: Option<&'static str>,
    pub ethernet_controller: Option<&'static str>,
    pub ethernet_compatible: Option<&'static [&'static str]>,
    pub ethernet_register: Option<&'static str>,
    pub ethernet_offset: Option<u64>,
    pub ethernet_rp1_bus_target: Option<u64>,
    pub ethernet_cpu_physical_target: Option<u64>,
    pub ethernet_width_bits: Option<u32>,
    pub expected_candidate_classifications: &'static [&'static str],
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
pub enum Rp1EthernetObservedWindowDiscriminatorReportKind {
    Candidate,
    NoMmioNoEthernetControl,
}

impl Rp1EthernetObservedWindowDiscriminatorReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoMmioNoEthernetControl => "no-mmio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetObservedWindowDiscriminatorReportInput {
    pub kind: Rp1EthernetObservedWindowDiscriminatorReportKind,
    pub source_contract: Option<Rp1EthernetGemMidSourceContractEvidence>,
    pub observed_sysinfo_positive_control:
        Option<Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence>,
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
pub struct Rp1EthernetObservedWindowDiscriminatorReport {
    pub kind: Rp1EthernetObservedWindowDiscriminatorReportKind,
    pub source_contract: Option<Rp1EthernetGemMidSourceContractEvidence>,
    pub observed_sysinfo_positive_control:
        Option<Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetObservedWindowDiscriminatorReportEvidence {
    pub observed_window_contract_id: &'static str,
    pub discriminator_contract_id: &'static str,
    pub selected_by_task_id: &'static str,
    pub source_contract_id: &'static str,
    pub report_kind: &'static str,
    pub same_run_required: bool,
    pub material_difference_from_translated_window: bool,
    pub observed_positive_control_register: Option<&'static str>,
    pub observed_positive_control_cpu_physical_target: Option<u64>,
    pub observed_positive_control_width_bits: Option<u32>,
    pub observed_positive_control_expected_value: Option<u32>,
    pub observed_positive_control_access: Option<&'static str>,
    pub ethernet_controller: Option<&'static str>,
    pub ethernet_compatible: Option<&'static [&'static str]>,
    pub ethernet_register: Option<&'static str>,
    pub ethernet_register_offset: Option<u64>,
    pub source_offset_from_observed_rp1_base: Option<u64>,
    pub observed_rp1_base: Option<u64>,
    pub observed_window_cpu_physical_target: Option<u64>,
    pub translated_window_comparator_cpu_physical_target: Option<u64>,
    pub translated_window_comparator_role: Option<&'static str>,
    pub ethernet_width_bits: Option<u32>,
    pub ethernet_endianness: Option<&'static str>,
    pub ethernet_access: Option<&'static str>,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGemMidDecodeDiscriminatorReportError {
    CandidateMissingSourceContract,
    CandidateMissingObservedPositiveControl,
    ControlCarriesEthernetMmioTarget,
    ControlCarriesObservedPositiveControl,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    ObservedPositiveControlMismatch,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetObservedWindowDiscriminatorReportError {
    CandidateMissingSourceContract,
    CandidateMissingObservedPositiveControl,
    ControlCarriesEthernetMmioTarget,
    ControlCarriesObservedPositiveControl,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    ObservedPositiveControlMismatch,
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

impl Rp1EthernetObservedWindowDiscriminatorReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::CandidateMissingObservedPositiveControl => {
                "candidate-missing-observed-positive-control"
            }
            Self::ControlCarriesEthernetMmioTarget => "control-carries-ethernet-mmio-target",
            Self::ControlCarriesObservedPositiveControl => {
                "control-carries-observed-positive-control"
            }
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::ObservedPositiveControlMismatch => "observed-positive-control-mismatch",
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

impl Rp1EthernetGemMidDecodeDiscriminatorReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::CandidateMissingObservedPositiveControl => {
                "candidate-missing-observed-positive-control"
            }
            Self::ControlCarriesEthernetMmioTarget => "control-carries-ethernet-mmio-target",
            Self::ControlCarriesObservedPositiveControl => {
                "control-carries-observed-positive-control"
            }
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::ObservedPositiveControlMismatch => "observed-positive-control-mismatch",
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

pub const fn rp1_ethernet_gem_mid_observed_sysinfo_positive_control_evidence()
-> Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence {
    Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence {
        register: RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_REGISTER,
        cpu_physical_target: RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_CPU_PHYSICAL_TARGET,
        width_bits: RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_WIDTH_BITS,
        expected_value: RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_EXPECTED,
        access: RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_ACCESS,
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

pub fn build_rp1_ethernet_gem_mid_decode_discriminator_report(
    input: Rp1EthernetGemMidDecodeDiscriminatorReportInput,
) -> Result<
    Rp1EthernetGemMidDecodeDiscriminatorReport,
    Rp1EthernetGemMidDecodeDiscriminatorReportError,
> {
    validate_rp1_ethernet_gem_mid_decode_discriminator_rejected_claims(input)?;

    match (
        input.kind,
        input.source_contract,
        input.observed_sysinfo_positive_control,
    ) {
        (
            Rp1EthernetGemMidDecodeDiscriminatorReportKind::Candidate,
            Some(source_contract),
            Some(observed_sysinfo_positive_control),
        ) => {
            validate_rp1_ethernet_gem_mid_source_contract_for_decode_discriminator(
                source_contract,
            )?;
            validate_rp1_ethernet_gem_mid_observed_sysinfo_positive_control(
                observed_sysinfo_positive_control,
            )?;
            Ok(Rp1EthernetGemMidDecodeDiscriminatorReport {
                kind: input.kind,
                source_contract: Some(source_contract),
                observed_sysinfo_positive_control: Some(observed_sysinfo_positive_control),
            })
        }
        (Rp1EthernetGemMidDecodeDiscriminatorReportKind::Candidate, None, _) => Err(
            Rp1EthernetGemMidDecodeDiscriminatorReportError::CandidateMissingSourceContract,
        ),
        (Rp1EthernetGemMidDecodeDiscriminatorReportKind::Candidate, Some(_), None) => Err(
            Rp1EthernetGemMidDecodeDiscriminatorReportError::CandidateMissingObservedPositiveControl,
        ),
        (Rp1EthernetGemMidDecodeDiscriminatorReportKind::NoMmioNoEthernetControl, None, None) => {
            Ok(Rp1EthernetGemMidDecodeDiscriminatorReport {
                kind: input.kind,
                source_contract: None,
                observed_sysinfo_positive_control: None,
            })
        }
        (
            Rp1EthernetGemMidDecodeDiscriminatorReportKind::NoMmioNoEthernetControl,
            Some(_),
            _,
        ) => Err(
            Rp1EthernetGemMidDecodeDiscriminatorReportError::ControlCarriesEthernetMmioTarget,
        ),
        (
            Rp1EthernetGemMidDecodeDiscriminatorReportKind::NoMmioNoEthernetControl,
            None,
            Some(_),
        ) => Err(
            Rp1EthernetGemMidDecodeDiscriminatorReportError::ControlCarriesObservedPositiveControl,
        ),
    }
}

pub fn rp1_ethernet_gem_mid_decode_discriminator_report_evidence(
    report: Rp1EthernetGemMidDecodeDiscriminatorReport,
) -> Rp1EthernetGemMidDecodeDiscriminatorReportEvidence {
    match (
        report.source_contract,
        report.observed_sysinfo_positive_control,
    ) {
        (Some(source_contract), Some(observed_sysinfo_positive_control)) => {
            rp1_ethernet_gem_mid_decode_discriminator_candidate_evidence(
                report.kind.name(),
                source_contract,
                observed_sysinfo_positive_control,
            )
        }
        (None, None) => {
            rp1_ethernet_gem_mid_decode_discriminator_control_evidence(report.kind.name())
        }
        _ => unreachable!("decode discriminator reports are validated at construction"),
    }
}

pub fn rejected_rp1_ethernet_gem_mid_decode_discriminator_report_evidence(
    error: Rp1EthernetGemMidDecodeDiscriminatorReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_rp1_ethernet_observed_window_discriminator_report(
    input: Rp1EthernetObservedWindowDiscriminatorReportInput,
) -> Result<
    Rp1EthernetObservedWindowDiscriminatorReport,
    Rp1EthernetObservedWindowDiscriminatorReportError,
> {
    validate_rp1_ethernet_observed_window_discriminator_rejected_claims(input)?;

    match (
        input.kind,
        input.source_contract,
        input.observed_sysinfo_positive_control,
    ) {
        (
            Rp1EthernetObservedWindowDiscriminatorReportKind::Candidate,
            Some(source_contract),
            Some(observed_sysinfo_positive_control),
        ) => {
            validate_rp1_ethernet_gem_mid_source_contract_for_observed_window_discriminator(
                source_contract,
            )?;
            validate_rp1_ethernet_gem_mid_observed_sysinfo_positive_control_for_observed_window(
                observed_sysinfo_positive_control,
            )?;
            Ok(Rp1EthernetObservedWindowDiscriminatorReport {
                kind: input.kind,
                source_contract: Some(source_contract),
                observed_sysinfo_positive_control: Some(observed_sysinfo_positive_control),
            })
        }
        (Rp1EthernetObservedWindowDiscriminatorReportKind::Candidate, None, _) => Err(
            Rp1EthernetObservedWindowDiscriminatorReportError::CandidateMissingSourceContract,
        ),
        (Rp1EthernetObservedWindowDiscriminatorReportKind::Candidate, Some(_), None) => Err(
            Rp1EthernetObservedWindowDiscriminatorReportError::CandidateMissingObservedPositiveControl,
        ),
        (Rp1EthernetObservedWindowDiscriminatorReportKind::NoMmioNoEthernetControl, None, None) => {
            Ok(Rp1EthernetObservedWindowDiscriminatorReport {
                kind: input.kind,
                source_contract: None,
                observed_sysinfo_positive_control: None,
            })
        }
        (
            Rp1EthernetObservedWindowDiscriminatorReportKind::NoMmioNoEthernetControl,
            Some(_),
            _,
        ) => Err(
            Rp1EthernetObservedWindowDiscriminatorReportError::ControlCarriesEthernetMmioTarget,
        ),
        (
            Rp1EthernetObservedWindowDiscriminatorReportKind::NoMmioNoEthernetControl,
            None,
            Some(_),
        ) => Err(
            Rp1EthernetObservedWindowDiscriminatorReportError::ControlCarriesObservedPositiveControl,
        ),
    }
}

pub fn rp1_ethernet_observed_window_discriminator_report_evidence(
    report: Rp1EthernetObservedWindowDiscriminatorReport,
) -> Rp1EthernetObservedWindowDiscriminatorReportEvidence {
    match (
        report.source_contract,
        report.observed_sysinfo_positive_control,
    ) {
        (Some(source_contract), Some(observed_sysinfo_positive_control)) => {
            rp1_ethernet_observed_window_discriminator_candidate_evidence(
                report.kind.name(),
                source_contract,
                observed_sysinfo_positive_control,
            )
        }
        (None, None) => {
            rp1_ethernet_observed_window_discriminator_control_evidence(report.kind.name())
        }
        _ => unreachable!("observed-window discriminator reports are validated at construction"),
    }
}

pub fn rejected_rp1_ethernet_observed_window_discriminator_report_evidence(
    error: Rp1EthernetObservedWindowDiscriminatorReportError,
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

fn validate_rp1_ethernet_gem_mid_decode_discriminator_rejected_claims(
    input: Rp1EthernetGemMidDecodeDiscriminatorReportInput,
) -> Result<(), Rp1EthernetGemMidDecodeDiscriminatorReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::BroadMmioReadinessClaim);
    }
    if input.claims_rp1_mmio_dma_programming {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::Rp1MmioDmaProgrammingClaim);
    }
    if input.claims_descriptor_rings {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::DescriptorRingsClaim);
    }
    if input.claims_dma_ownership {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::DmaOwnershipClaim);
    }
    if input.claims_transfer_completion {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::TransferCompletionClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::InterruptCompletionClaim);
    }
    if input.claims_clock_reset_ownership {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::ClockResetOwnershipClaim);
    }
    if input.claims_phy_ownership {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::PhyOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_observed_window_discriminator_rejected_claims(
    input: Rp1EthernetObservedWindowDiscriminatorReportInput,
) -> Result<(), Rp1EthernetObservedWindowDiscriminatorReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::BroadMmioReadinessClaim);
    }
    if input.claims_rp1_mmio_dma_programming {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::Rp1MmioDmaProgrammingClaim);
    }
    if input.claims_descriptor_rings {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::DescriptorRingsClaim);
    }
    if input.claims_dma_ownership {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::DmaOwnershipClaim);
    }
    if input.claims_transfer_completion {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::TransferCompletionClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::InterruptCompletionClaim);
    }
    if input.claims_clock_reset_ownership {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::ClockResetOwnershipClaim);
    }
    if input.claims_phy_ownership {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::PhyOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetObservedWindowDiscriminatorReportError::PhaseTransitionClaim);
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

fn validate_rp1_ethernet_gem_mid_source_contract_for_decode_discriminator(
    evidence: Rp1EthernetGemMidSourceContractEvidence,
) -> Result<(), Rp1EthernetGemMidDecodeDiscriminatorReportError> {
    validate_rp1_ethernet_gem_mid_source_contract(evidence).map_err(|error| match error {
        Rp1EthernetGemMidDiagnosticReportError::SourceContractIdentityMismatch => {
            Rp1EthernetGemMidDecodeDiscriminatorReportError::SourceContractIdentityMismatch
        }
        Rp1EthernetGemMidDiagnosticReportError::SourceContractTargetMismatch => {
            Rp1EthernetGemMidDecodeDiscriminatorReportError::SourceContractTargetMismatch
        }
        Rp1EthernetGemMidDiagnosticReportError::SourceContractFieldMismatch => {
            Rp1EthernetGemMidDecodeDiscriminatorReportError::SourceContractFieldMismatch
        }
        Rp1EthernetGemMidDiagnosticReportError::MissingSourceEvidence => {
            Rp1EthernetGemMidDecodeDiscriminatorReportError::MissingSourceEvidence
        }
        _ => unreachable!("source contract validation only returns source-contract errors"),
    })
}

fn validate_rp1_ethernet_gem_mid_source_contract_for_observed_window_discriminator(
    evidence: Rp1EthernetGemMidSourceContractEvidence,
) -> Result<(), Rp1EthernetObservedWindowDiscriminatorReportError> {
    validate_rp1_ethernet_gem_mid_source_contract(evidence).map_err(|error| match error {
        Rp1EthernetGemMidDiagnosticReportError::SourceContractIdentityMismatch => {
            Rp1EthernetObservedWindowDiscriminatorReportError::SourceContractIdentityMismatch
        }
        Rp1EthernetGemMidDiagnosticReportError::SourceContractTargetMismatch => {
            Rp1EthernetObservedWindowDiscriminatorReportError::SourceContractTargetMismatch
        }
        Rp1EthernetGemMidDiagnosticReportError::SourceContractFieldMismatch => {
            Rp1EthernetObservedWindowDiscriminatorReportError::SourceContractFieldMismatch
        }
        Rp1EthernetGemMidDiagnosticReportError::MissingSourceEvidence => {
            Rp1EthernetObservedWindowDiscriminatorReportError::MissingSourceEvidence
        }
        _ => unreachable!("source contract validation only returns source-contract errors"),
    })
}

fn validate_rp1_ethernet_gem_mid_observed_sysinfo_positive_control(
    evidence: Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence,
) -> Result<(), Rp1EthernetGemMidDecodeDiscriminatorReportError> {
    if evidence.register != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_REGISTER
        || evidence.cpu_physical_target != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_CPU_PHYSICAL_TARGET
        || evidence.width_bits != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_WIDTH_BITS
        || evidence.expected_value != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_EXPECTED
        || evidence.access != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_ACCESS
    {
        return Err(
            Rp1EthernetGemMidDecodeDiscriminatorReportError::ObservedPositiveControlMismatch,
        );
    }
    Ok(())
}

fn validate_rp1_ethernet_gem_mid_observed_sysinfo_positive_control_for_observed_window(
    evidence: Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence,
) -> Result<(), Rp1EthernetObservedWindowDiscriminatorReportError> {
    if evidence.register != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_REGISTER
        || evidence.cpu_physical_target != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_CPU_PHYSICAL_TARGET
        || evidence.width_bits != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_WIDTH_BITS
        || evidence.expected_value != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_EXPECTED
        || evidence.access != RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_ACCESS
    {
        return Err(
            Rp1EthernetObservedWindowDiscriminatorReportError::ObservedPositiveControlMismatch,
        );
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

fn rp1_ethernet_gem_mid_decode_discriminator_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetGemMidSourceContractEvidence,
    observed_sysinfo_positive_control: Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence,
) -> Rp1EthernetGemMidDecodeDiscriminatorReportEvidence {
    Rp1EthernetGemMidDecodeDiscriminatorReportEvidence {
        discriminator_contract_id: RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTRACT_ID,
        selected_by_task_id: RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_SOURCE_TASK_ID,
        source_contract_id: source_contract.contract_id,
        report_kind,
        same_run_required: true,
        changed_from_gem_mid_only_proof: true,
        observed_positive_control_register: Some(observed_sysinfo_positive_control.register),
        observed_positive_control_cpu_physical_target: Some(
            observed_sysinfo_positive_control.cpu_physical_target,
        ),
        observed_positive_control_width_bits: Some(observed_sysinfo_positive_control.width_bits),
        observed_positive_control_expected_value: Some(
            observed_sysinfo_positive_control.expected_value,
        ),
        observed_positive_control_access: Some(observed_sysinfo_positive_control.access),
        ethernet_controller: Some(source_contract.controller),
        ethernet_compatible: Some(source_contract.compatible),
        ethernet_register: Some(source_contract.register),
        ethernet_offset: Some(source_contract.offset),
        ethernet_rp1_bus_target: Some(source_contract.rp1_bus_target),
        ethernet_cpu_physical_target: Some(source_contract.cpu_physical_target),
        ethernet_width_bits: Some(source_contract.width_bits),
        expected_candidate_classifications:
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_EXPECTED_CLASSIFICATIONS,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
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
        classification: RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_EXPECTED_CLASSIFICATIONS[0],
    }
}

fn rp1_ethernet_gem_mid_decode_discriminator_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetGemMidDecodeDiscriminatorReportEvidence {
    Rp1EthernetGemMidDecodeDiscriminatorReportEvidence {
        discriminator_contract_id: RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTRACT_ID,
        selected_by_task_id: RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_SOURCE_TASK_ID,
        source_contract_id: RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID,
        report_kind,
        same_run_required: true,
        changed_from_gem_mid_only_proof: true,
        observed_positive_control_register: None,
        observed_positive_control_cpu_physical_target: None,
        observed_positive_control_width_bits: None,
        observed_positive_control_expected_value: None,
        observed_positive_control_access: None,
        ethernet_controller: None,
        ethernet_compatible: None,
        ethernet_register: None,
        ethernet_offset: None,
        ethernet_rp1_bus_target: None,
        ethernet_cpu_physical_target: None,
        ethernet_width_bits: None,
        expected_candidate_classifications:
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_EXPECTED_CLASSIFICATIONS,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
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
        classification: RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTROL_CLASSIFICATION,
    }
}

fn rp1_ethernet_observed_window_discriminator_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetGemMidSourceContractEvidence,
    observed_sysinfo_positive_control: Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence,
) -> Rp1EthernetObservedWindowDiscriminatorReportEvidence {
    Rp1EthernetObservedWindowDiscriminatorReportEvidence {
        observed_window_contract_id: RP1_ETHERNET_OBSERVED_WINDOW_CONTRACT_ID,
        discriminator_contract_id: RP1_ETHERNET_OBSERVED_WINDOW_DISCRIMINATOR_CONTRACT_ID,
        selected_by_task_id: RP1_ETHERNET_OBSERVED_WINDOW_SOURCE_TASK_ID,
        source_contract_id: source_contract.contract_id,
        report_kind,
        same_run_required: true,
        material_difference_from_translated_window: true,
        observed_positive_control_register: Some(observed_sysinfo_positive_control.register),
        observed_positive_control_cpu_physical_target: Some(
            observed_sysinfo_positive_control.cpu_physical_target,
        ),
        observed_positive_control_width_bits: Some(observed_sysinfo_positive_control.width_bits),
        observed_positive_control_expected_value: Some(
            observed_sysinfo_positive_control.expected_value,
        ),
        observed_positive_control_access: Some(observed_sysinfo_positive_control.access),
        ethernet_controller: Some(source_contract.controller),
        ethernet_compatible: Some(source_contract.compatible),
        ethernet_register: Some(source_contract.register),
        ethernet_register_offset: Some(source_contract.offset),
        source_offset_from_observed_rp1_base: Some(
            RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_SOURCE_OFFSET,
        ),
        observed_rp1_base: Some(RP1_ETHERNET_OBSERVED_RP1_BASE),
        observed_window_cpu_physical_target: Some(
            RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET,
        ),
        translated_window_comparator_cpu_physical_target: Some(source_contract.cpu_physical_target),
        translated_window_comparator_role: Some("comparator-sentinel-only"),
        ethernet_width_bits: Some(source_contract.width_bits),
        ethernet_endianness: Some(source_contract.endianness),
        ethernet_access: Some(source_contract.access),
        source_evidence: Some(source_contract.source_evidence),
        hardware_proof_boundary_classification:
            RP1_ETHERNET_OBSERVED_WINDOW_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GEM_MID_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_OBSERVED_WINDOW_RETAINED_RISKS,
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
        classification: RP1_ETHERNET_OBSERVED_WINDOW_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_observed_window_discriminator_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetObservedWindowDiscriminatorReportEvidence {
    Rp1EthernetObservedWindowDiscriminatorReportEvidence {
        observed_window_contract_id: RP1_ETHERNET_OBSERVED_WINDOW_CONTRACT_ID,
        discriminator_contract_id: RP1_ETHERNET_OBSERVED_WINDOW_DISCRIMINATOR_CONTRACT_ID,
        selected_by_task_id: RP1_ETHERNET_OBSERVED_WINDOW_SOURCE_TASK_ID,
        source_contract_id: RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID,
        report_kind,
        same_run_required: true,
        material_difference_from_translated_window: true,
        observed_positive_control_register: None,
        observed_positive_control_cpu_physical_target: None,
        observed_positive_control_width_bits: None,
        observed_positive_control_expected_value: None,
        observed_positive_control_access: None,
        ethernet_controller: None,
        ethernet_compatible: None,
        ethernet_register: None,
        ethernet_register_offset: None,
        source_offset_from_observed_rp1_base: None,
        observed_rp1_base: None,
        observed_window_cpu_physical_target: None,
        translated_window_comparator_cpu_physical_target: None,
        translated_window_comparator_role: None,
        ethernet_width_bits: None,
        ethernet_endianness: None,
        ethernet_access: None,
        source_evidence: None,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_OBSERVED_WINDOW_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GEM_MID_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_OBSERVED_WINDOW_RETAINED_RISKS,
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
        classification: RP1_ETHERNET_OBSERVED_WINDOW_CONTROL_CLASSIFICATION,
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

    fn accepted_decode_discriminator_input() -> Rp1EthernetGemMidDecodeDiscriminatorReportInput {
        Rp1EthernetGemMidDecodeDiscriminatorReportInput {
            kind: Rp1EthernetGemMidDecodeDiscriminatorReportKind::Candidate,
            source_contract: Some(rp1_ethernet_gem_mid_source_contract_evidence()),
            observed_sysinfo_positive_control: Some(
                rp1_ethernet_gem_mid_observed_sysinfo_positive_control_evidence(),
            ),
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

    fn accepted_observed_window_input() -> Rp1EthernetObservedWindowDiscriminatorReportInput {
        Rp1EthernetObservedWindowDiscriminatorReportInput {
            kind: Rp1EthernetObservedWindowDiscriminatorReportKind::Candidate,
            source_contract: Some(rp1_ethernet_gem_mid_source_contract_evidence()),
            observed_sysinfo_positive_control: Some(
                rp1_ethernet_gem_mid_observed_sysinfo_positive_control_evidence(),
            ),
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
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_OBSERVED_WINDOW_RETAINED_RISKS
        );
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GEM_MID_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_decode_discriminator_formats_candidate_shape() {
        let report = build_rp1_ethernet_gem_mid_decode_discriminator_report(
            accepted_decode_discriminator_input(),
        )
        .expect("valid GEM MID decode discriminator candidate input");
        let evidence = rp1_ethernet_gem_mid_decode_discriminator_report_evidence(report);

        assert_eq!(
            evidence.discriminator_contract_id,
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTRACT_ID
        );
        assert_eq!(
            evidence.selected_by_task_id,
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_SOURCE_TASK_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert!(evidence.same_run_required);
        assert!(evidence.changed_from_gem_mid_only_proof);
        assert_eq!(
            evidence.observed_positive_control_register,
            Some(RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_REGISTER)
        );
        assert_eq!(
            evidence.observed_positive_control_cpu_physical_target,
            Some(0x1c_0000_0000)
        );
        assert_eq!(evidence.observed_positive_control_width_bits, Some(32));
        assert_eq!(
            evidence.observed_positive_control_expected_value,
            Some(0x2000_1927)
        );
        assert_eq!(
            evidence.observed_positive_control_access,
            Some("read-only volatile load")
        );
        assert_eq!(
            evidence.ethernet_controller,
            Some(RP1_ETHERNET_CONTROLLER_NAME)
        );
        assert_eq!(evidence.ethernet_compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(
            evidence.ethernet_register,
            Some(RP1_ETHERNET_GEM_MID_REGISTER)
        );
        assert_eq!(evidence.ethernet_offset, Some(0x00fc));
        assert_eq!(evidence.ethernet_rp1_bus_target, Some(0xc0_4010_00fc));
        assert_eq!(evidence.ethernet_cpu_physical_target, Some(0x1f_0010_00fc));
        assert_eq!(evidence.ethernet_width_bits, Some(32));
        assert_eq!(
            evidence.expected_candidate_classifications,
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_EXPECTED_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION
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
            "observed-rp1-positive-control-gem-mid-0x1f-window-sentinel"
        );
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_decode_discriminator_formats_no_mmio_no_ethernet_control() {
        let report = build_rp1_ethernet_gem_mid_decode_discriminator_report(
            Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                kind: Rp1EthernetGemMidDecodeDiscriminatorReportKind::NoMmioNoEthernetControl,
                source_contract: None,
                observed_sysinfo_positive_control: None,
                ..accepted_decode_discriminator_input()
            },
        )
        .expect("valid GEM MID decode discriminator control input");
        let evidence = rp1_ethernet_gem_mid_decode_discriminator_report_evidence(report);

        assert_eq!(
            evidence.discriminator_contract_id,
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-mmio-no-ethernet-control");
        assert!(evidence.same_run_required);
        assert!(evidence.changed_from_gem_mid_only_proof);
        assert_eq!(evidence.observed_positive_control_register, None);
        assert_eq!(evidence.observed_positive_control_cpu_physical_target, None);
        assert_eq!(evidence.observed_positive_control_width_bits, None);
        assert_eq!(evidence.observed_positive_control_expected_value, None);
        assert_eq!(evidence.observed_positive_control_access, None);
        assert_eq!(evidence.ethernet_controller, None);
        assert_eq!(evidence.ethernet_compatible, None);
        assert_eq!(evidence.ethernet_register, None);
        assert_eq!(evidence.ethernet_offset, None);
        assert_eq!(evidence.ethernet_rp1_bus_target, None);
        assert_eq!(evidence.ethernet_cpu_physical_target, None);
        assert_eq!(evidence.ethernet_width_bits, None);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_observed_window_discriminator_formats_candidate_shape() {
        let report = build_rp1_ethernet_observed_window_discriminator_report(
            accepted_observed_window_input(),
        )
        .expect("valid observed-window discriminator candidate input");
        let evidence = rp1_ethernet_observed_window_discriminator_report_evidence(report);

        assert_eq!(
            evidence.observed_window_contract_id,
            RP1_ETHERNET_OBSERVED_WINDOW_CONTRACT_ID
        );
        assert_eq!(
            evidence.discriminator_contract_id,
            RP1_ETHERNET_OBSERVED_WINDOW_DISCRIMINATOR_CONTRACT_ID
        );
        assert_eq!(
            evidence.selected_by_task_id,
            RP1_ETHERNET_OBSERVED_WINDOW_SOURCE_TASK_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_GEM_MID_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert!(evidence.same_run_required);
        assert!(evidence.material_difference_from_translated_window);
        assert_eq!(
            evidence.observed_positive_control_register,
            Some(RP1_ETHERNET_OBSERVED_SYSINFO_CHIP_ID_REGISTER)
        );
        assert_eq!(
            evidence.observed_positive_control_cpu_physical_target,
            Some(0x1c_0000_0000)
        );
        assert_eq!(
            evidence.observed_positive_control_expected_value,
            Some(0x2000_1927)
        );
        assert_eq!(
            evidence.ethernet_controller,
            Some(RP1_ETHERNET_CONTROLLER_NAME)
        );
        assert_eq!(evidence.ethernet_compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(
            evidence.ethernet_register,
            Some(RP1_ETHERNET_GEM_MID_REGISTER)
        );
        assert_eq!(evidence.ethernet_register_offset, Some(0x00fc));
        assert_eq!(
            evidence.source_offset_from_observed_rp1_base,
            Some(0x0010_00fc)
        );
        assert_eq!(evidence.observed_rp1_base, Some(0x1c_0000_0000));
        assert_eq!(
            evidence.observed_window_cpu_physical_target,
            Some(0x1c_0010_00fc)
        );
        assert_eq!(
            evidence.translated_window_comparator_cpu_physical_target,
            Some(0x1f_0010_00fc)
        );
        assert_eq!(
            evidence.translated_window_comparator_role,
            Some("comparator-sentinel-only")
        );
        assert_eq!(evidence.ethernet_width_bits, Some(32));
        assert_eq!(evidence.ethernet_endianness, Some("little-endian"));
        assert_eq!(evidence.ethernet_access, Some("read-only volatile load"));
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_GEM_MID_SOURCE_EVIDENCE)
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_OBSERVED_WINDOW_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION
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
            RP1_ETHERNET_OBSERVED_WINDOW_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_observed_window_discriminator_formats_no_mmio_no_ethernet_control() {
        let report = build_rp1_ethernet_observed_window_discriminator_report(
            Rp1EthernetObservedWindowDiscriminatorReportInput {
                kind: Rp1EthernetObservedWindowDiscriminatorReportKind::NoMmioNoEthernetControl,
                source_contract: None,
                observed_sysinfo_positive_control: None,
                ..accepted_observed_window_input()
            },
        )
        .expect("valid observed-window discriminator control input");
        let evidence = rp1_ethernet_observed_window_discriminator_report_evidence(report);

        assert_eq!(
            evidence.discriminator_contract_id,
            RP1_ETHERNET_OBSERVED_WINDOW_DISCRIMINATOR_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-mmio-no-ethernet-control");
        assert!(evidence.same_run_required);
        assert!(evidence.material_difference_from_translated_window);
        assert_eq!(evidence.observed_positive_control_register, None);
        assert_eq!(evidence.observed_positive_control_cpu_physical_target, None);
        assert_eq!(evidence.ethernet_controller, None);
        assert_eq!(evidence.ethernet_register, None);
        assert_eq!(evidence.ethernet_register_offset, None);
        assert_eq!(evidence.source_offset_from_observed_rp1_base, None);
        assert_eq!(evidence.observed_rp1_base, None);
        assert_eq!(evidence.observed_window_cpu_physical_target, None);
        assert_eq!(
            evidence.translated_window_comparator_cpu_physical_target,
            None
        );
        assert_eq!(evidence.translated_window_comparator_role, None);
        assert_eq!(evidence.ethernet_width_bits, None);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_OBSERVED_WINDOW_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_observed_window_discriminator_rejects_shape_bypass() {
        let input = accepted_observed_window_input();

        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    observed_sysinfo_positive_control: None,
                    ..input
                }
            ),
            Err(
                Rp1EthernetObservedWindowDiscriminatorReportError::CandidateMissingObservedPositiveControl
            )
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    kind: Rp1EthernetObservedWindowDiscriminatorReportKind::NoMmioNoEthernetControl,
                    ..input
                }
            ),
            Err(
                Rp1EthernetObservedWindowDiscriminatorReportError::ControlCarriesEthernetMmioTarget
            )
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    source_contract: Some(Rp1EthernetGemMidSourceContractEvidence {
                        cpu_physical_target: 0x1f_0010_0000,
                        ..rp1_ethernet_gem_mid_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    observed_sysinfo_positive_control: Some(
                        Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence {
                            expected_value: 0,
                            ..rp1_ethernet_gem_mid_observed_sysinfo_positive_control_evidence()
                        }
                    ),
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::ObservedPositiveControlMismatch)
        );
        assert_eq!(
            rejected_rp1_ethernet_observed_window_discriminator_report_evidence(
                Rp1EthernetObservedWindowDiscriminatorReportError::ObservedPositiveControlMismatch
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "observed-positive-control-mismatch"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_observed_window_discriminator_rejects_overclaims() {
        let input = accepted_observed_window_input();

        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_ethernet_ready: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_broad_mmio_ready: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::BroadMmioReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_rp1_mmio_dma_programming: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::Rp1MmioDmaProgrammingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_descriptor_rings: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::DescriptorRingsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_dma_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::DmaOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_transfer_completion: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::TransferCompletionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_interrupt_completion: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::InterruptCompletionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_clock_reset_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::ClockResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::PhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_packet_io: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_networking: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_sockets: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::SocketsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_ssh: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::Phase122Claim)
        );
        assert_eq!(
            build_rp1_ethernet_observed_window_discriminator_report(
                Rp1EthernetObservedWindowDiscriminatorReportInput {
                    claims_phase_transition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetObservedWindowDiscriminatorReportError::PhaseTransitionClaim)
        );
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_decode_discriminator_rejects_shape_bypass() {
        let input = accepted_decode_discriminator_input();

        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    observed_sysinfo_positive_control: None,
                    ..input
                }
            ),
            Err(
                Rp1EthernetGemMidDecodeDiscriminatorReportError::CandidateMissingObservedPositiveControl
            )
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    kind: Rp1EthernetGemMidDecodeDiscriminatorReportKind::NoMmioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::ControlCarriesEthernetMmioTarget)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    source_contract: Some(Rp1EthernetGemMidSourceContractEvidence {
                        cpu_physical_target: 0x1f_0010_0000,
                        ..rp1_ethernet_gem_mid_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    observed_sysinfo_positive_control: Some(
                        Rp1EthernetGemMidObservedSysinfoPositiveControlEvidence {
                            cpu_physical_target: 0x1f_0000_0000,
                            ..rp1_ethernet_gem_mid_observed_sysinfo_positive_control_evidence()
                        }
                    ),
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::ObservedPositiveControlMismatch)
        );
        assert_eq!(
            rejected_rp1_ethernet_gem_mid_decode_discriminator_report_evidence(
                Rp1EthernetGemMidDecodeDiscriminatorReportError::ObservedPositiveControlMismatch
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "observed-positive-control-mismatch"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_gem_mid_decode_discriminator_rejects_overclaims() {
        let input = accepted_decode_discriminator_input();

        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_ethernet_ready: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_broad_mmio_ready: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::BroadMmioReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_rp1_mmio_dma_programming: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::Rp1MmioDmaProgrammingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_descriptor_rings: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::DescriptorRingsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_dma_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::DmaOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_transfer_completion: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::TransferCompletionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_interrupt_completion: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::InterruptCompletionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_clock_reset_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::ClockResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::PhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_packet_io: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_networking: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_sockets: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::SocketsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_ssh: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::Phase122Claim)
        );
        assert_eq!(
            build_rp1_ethernet_gem_mid_decode_discriminator_report(
                Rp1EthernetGemMidDecodeDiscriminatorReportInput {
                    claims_phase_transition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGemMidDecodeDiscriminatorReportError::PhaseTransitionClaim)
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
