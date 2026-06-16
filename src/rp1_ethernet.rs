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
pub const RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-prereq-ownership-contract-v1";
pub const RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-prereq-ownership-source-contract-20260610";
pub const RP1_ETHERNET_CLOCK_RESET_GUARD_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-clock-reset-guard-contract-v1";
pub const RP1_ETHERNET_CLOCK_RESET_OWNERSHIP_CONTRACT_TASK_ID: &str =
    "phase12-rp1-ethernet-clock-reset-ownership-contract-20260610";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-clock-reset-write-target-source-contract-v1";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-clock-reset-write-target-source-contract-20260610";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-clock-reset-write-restore-report-contract-v1";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-clk-eth-ctrl-write-target-source-contract-v1";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-clk-eth-ctrl-source-contract-20260610";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-clk-eth-ctrl-write-restore-report-contract-v1";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-source-contract-v1";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-source-contract-20260610";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-preflight-report-contract-v1";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-v1";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-20260610";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-report-contract-v1";
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
pub const RP1_ETHERNET_PREREQ_OWNERSHIP_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-prereq-ownership-candidate-local-static";
pub const RP1_ETHERNET_PREREQ_OWNERSHIP_CONTROL_CLASSIFICATION: &str =
    "no-ownership-no-ethernet-rp1-ethernet-prereq-control";
pub const RP1_ETHERNET_CLOCK_RESET_GUARD_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-clock-reset-guard-candidate-local-static";
pub const RP1_ETHERNET_CLOCK_RESET_GUARD_CONTROL_CLASSIFICATION: &str =
    "no-clock-reset-no-ethernet-rp1-ethernet-clock-reset-guard-control";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-clk-eth-tsu-ctrl-write-restore-candidate-local-static";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CONTROL_CLASSIFICATION: &str =
    "no-clock-write-no-ethernet-rp1-ethernet-write-restore-control";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-clk-eth-ctrl-write-restore-candidate-local-static";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CONTROL_CLASSIFICATION: &str =
    "no-clk-eth-ctrl-write-no-ethernet-rp1-ethernet-control";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-gpio32-phy-reset-preflight-candidate-local-static";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_CONTROL_CLASSIFICATION: &str =
    "no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-gpio32-phy-reset-write-restore-guard-candidate-local-static";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_CONTROL_CLASSIFICATION: &str =
    "no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control";
pub const RP1_ETHERNET_GEM_MID_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-gem-mid-visibility-control-output";
pub const RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-observed-sysinfo-gem-mid-discriminator-control-output";
pub const RP1_ETHERNET_OBSERVED_WINDOW_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-observed-window-gem-mid-discriminator-control-output";
pub const RP1_ETHERNET_PREREQ_OWNERSHIP_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-prereq-ownership-report-visibility-control-output";
pub const RP1_ETHERNET_CLOCK_RESET_GUARD_BOUNDARY_CLASSIFICATION: &str =
    "local-static-clock-reset-guard-report-only";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-clk-eth-tsu-ctrl-idempotent-write-restore-control-output";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-clk-eth-ctrl-idempotent-write-restore-control-output";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-gpio32-phy-reset-readonly-preflight-control-output";
pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-gpio32-phy-reset-write-restore-control-output";
pub const RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION: &str = "contract-rejected-input";

pub const RP1_ETHERNET_COMPATIBLE: &[&str] = &["raspberrypi,rp1-gem", "cdns,macb"];
pub const RP1_ETHERNET_CONTROLLER_NAME: &str = "rp1_eth";
pub const RP1_ETHERNET_RP1_BUS_BASE: u64 = 0xc0_4010_0000;
pub const RP1_ETHERNET_RP1_BUS_WINDOW_SIZE: u64 = 0x4000;
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
pub const RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW: u32 = 0x0007_0109;
pub const RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_IDNUM: u32 = 0x7;
pub const RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_REV: u32 = 0x109;
pub const RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE: &str =
    "context-only-not-broad-ethernet-mmio-readiness";
pub const RP1_ETHERNET_SELECTED_PREREQUISITE: &str =
    "rp1-ethernet-clock-reset-phy-mdio-dma-ownership-report";
pub const RP1_ETHERNET_INTERRUPT_NAME: &str = "RP1_INT_ETH";
pub const RP1_ETHERNET_INTERRUPT_NUMBER: u32 = 6;
pub const RP1_ETHERNET_CLOCK_NAMES: &[&str] = &["pclk", "hclk", "tsu_clk", "tx_clk"];
pub const RP1_ETHERNET_CLOCK_IDS: &[u32] = &[12, 12, 29, 16];
pub const RP1_ETHERNET_CLOCK_SOURCES: &[&str] = &[
    "RP1_CLK_SYS",
    "RP1_CLK_SYS",
    "RP1_CLK_ETH_TSU",
    "RP1_CLK_ETH",
];
pub const RP1_ETHERNET_CLOCK_POLICY_CLASSIFICATION: &str = "no-clock-reset-ownership";
pub const RP1_ETHERNET_SHARED_CLOCK_NAMES: &[&str] = &["pclk", "hclk"];
pub const RP1_ETHERNET_SHARED_CLOCK_SOURCE: &str = "RP1_CLK_SYS";
pub const RP1_ETHERNET_SHARED_CLOCK_ID: u32 = 12;
pub const RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_NAMES: &[&str] = &["tsu_clk", "tx_clk"];
pub const RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_SOURCES: &[&str] =
    &["RP1_CLK_ETH_TSU", "RP1_CLK_ETH"];
pub const RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_IDS: &[u32] = &[29, 16];
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_TARGET: &str =
    "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CLOCK_NAME: &str = "tsu_clk";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CLOCK_ID: u32 = 29;
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REGISTER: &str = "CLK_ETH_TSU_CTRL";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SOURCE_BLOCK: &str = "RP1 clocks@18000";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SOURCE_OFFSET: u64 = 0x018134;
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CPU_PHYSICAL_TARGET: u64 =
    RP1_ETHERNET_OBSERVED_RP1_BASE + RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SOURCE_OFFSET;
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_ALLOWED_WRITE_VALUE: &str =
    "pre-read-raw-value-only";
pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_ACCESS: &str =
    "32-bit little-endian volatile load/store";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_TARGET: &str =
    "rp1-ethernet-clk-eth-ctrl-idempotent-write-restore";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CLOCK_NAME: &str = "tx_clk";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CLOCK_ID: u32 = 16;
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REGISTER: &str = "CLK_ETH_CTRL";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_BLOCK: &str = "RP1 clocks@18000";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_OFFSET: u64 = 0x018064;
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CPU_PHYSICAL_TARGET: u64 =
    RP1_ETHERNET_OBSERVED_RP1_BASE + RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_OFFSET;
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_ALLOWED_WRITE_VALUE: &str =
    "pre-read-raw-value-only";
pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_ACCESS: &str =
    "32-bit little-endian volatile load/store";
pub const RP1_ETHERNET_RESET_CONTROLLER_POLICY_CLASSIFICATION: &str =
    "no-accepted-rp1-eth-reset-controller-target";
pub const RP1_ETHERNET_PHY_MODE: &str = "rgmii-id";
pub const RP1_ETHERNET_PHY_HANDLE: &str = "phy1";
pub const RP1_ETHERNET_PHY_NODE: &str = "ethernet-phy@1";
pub const RP1_ETHERNET_PHY_REG: u32 = 0x1;
pub const RP1_ETHERNET_PHY_RESET_GPIO: u32 = 32;
pub const RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER: &str = "rp1_gpio";
pub const RP1_ETHERNET_PHY_RESET_ROUTE: &str = "ETH_RST_N";
pub const RP1_ETHERNET_PHY_RESET_ACTIVE_LOW: bool = true;
pub const RP1_ETHERNET_PHY_RESET_LOGICAL_ASSERTION: &str =
    "Linux logical value 1 asserts reset and drives active-low ETH_RST_N physically low";
pub const RP1_ETHERNET_PHY_RESET_LOGICAL_DEASSERTION: &str =
    "Linux logical value 0 deasserts reset and drives active-low ETH_RST_N physically high";
pub const RP1_ETHERNET_PHY_RESET_DURATION_MS: u32 = 5;
pub const RP1_ETHERNET_PHY_RESET_MDIO_HOOK_RELATIONSHIP: &str = "macb_mdio_reset is installed as the MDIO bus reset hook and asserts then deasserts phy_reset_gpio";
pub const RP1_ETHERNET_GPIO32_BANK: &str = "bank1";
pub const RP1_ETHERNET_GPIO32_BANK_LOCAL_BIT: u32 = 4;
pub const RP1_ETHERNET_GPIO32_IO_BANK1_SOURCE_BASE: u64 = 0xc0_400d_4000;
pub const RP1_ETHERNET_GPIO32_IO_BANK1_OBSERVED_BASE: u64 = 0x1c_000d_4000;
pub const RP1_ETHERNET_GPIO32_STATUS_SOURCE_TARGET: u64 = 0xc0_400d_4020;
pub const RP1_ETHERNET_GPIO32_STATUS_OBSERVED_TARGET: u64 = 0x1c_000d_4020;
pub const RP1_ETHERNET_GPIO32_CTRL_SOURCE_TARGET: u64 = 0xc0_400d_4024;
pub const RP1_ETHERNET_GPIO32_CTRL_OBSERVED_TARGET: u64 = 0x1c_000d_4024;
pub const RP1_ETHERNET_GPIO32_RIO1_OUT_SOURCE_TARGET: u64 = 0xc0_400e_4000;
pub const RP1_ETHERNET_GPIO32_RIO1_OUT_OBSERVED_TARGET: u64 = 0x1c_000e_4000;
pub const RP1_ETHERNET_GPIO32_RIO1_OE_SOURCE_TARGET: u64 = 0xc0_400e_4004;
pub const RP1_ETHERNET_GPIO32_RIO1_OE_OBSERVED_TARGET: u64 = 0x1c_000e_4004;
pub const RP1_ETHERNET_GPIO32_RIO1_IN_SOURCE_TARGET: u64 = 0xc0_400e_4008;
pub const RP1_ETHERNET_GPIO32_RIO1_IN_OBSERVED_TARGET: u64 = 0x1c_000e_4008;
pub const RP1_ETHERNET_GPIO32_PAD_SOURCE_TARGET: u64 = 0xc0_400f_4014;
pub const RP1_ETHERNET_GPIO32_PAD_OBSERVED_TARGET: u64 = 0x1c_000f_4014;
pub const RP1_ETHERNET_GPIO32_WRITE_RESTORE_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_GPIO32_WRITE_RESTORE_ACCESS: &str =
    "32-bit little-endian volatile load/store only in future hardware proof";
pub const RP1_ETHERNET_GPIO32_ASSERTION_RAW_OUTPUT: &str =
    "drive GPIO32 bank1 bit 4 raw output low";
pub const RP1_ETHERNET_GPIO32_DEASSERTION_RAW_OUTPUT: &str =
    "drive GPIO32 bank1 bit 4 raw output high";
pub const RP1_ETHERNET_PHY_MDIO_POLICY_CLASSIFICATION: &str = "no-phy-reset-or-mdio-ownership";
pub const RP1_ETHERNET_DMA_DESCRIPTOR_POLICY_CLASSIFICATION: &str =
    "no-live-dma-or-descriptor-ownership";
pub const RP1_ETHERNET_CADENCE_RP1_CONFIG: &[&str] = &[
    "gigabit",
    "hardware-clock-change",
    "jumbo",
    "ptp",
    "dma-burst-length-16",
];
pub const RP1_ETHERNET_GEM_MID_DECODE_DISCRIMINATOR_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-gem-mid-blocker-reconciliation-closeout-20260610";
pub const RP1_ETHERNET_OBSERVED_WINDOW_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-observed-window-contract-20260610";

pub const RP1_ETHERNET_GEM_MID_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-source-contract/source/linux-rpi-6.12-macb.h",
];

pub const RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-source-contract.md",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-mfd.h",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-clock.h",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-cdns-macb.yaml",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c",
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

pub const RP1_ETHERNET_PREREQ_OWNERSHIP_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "Ethernet driver readiness",
    "broad Ethernet MMIO readiness",
    "RP1 MMIO writes",
    "clock/reset ownership or writes",
    "GPIO32 ownership or PHY reset assertion/deassertion",
    "MDIO transactions or PHY ownership",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_PREREQ_OWNERSHIP_RETAINED_RISKS: &[&str] = &[
    "Observed-window MACB_MID identity does not prove clocks, PHY, MDIO, DMA, interrupts, or packet behavior",
    "Source facts identify required prerequisites but not Talos ownership",
    "The prerequisite report is local/static only and is not a hardware proof",
    "Any later hardware proof is limited to report visibility unless future scope changes acceptance criteria",
];

pub const RP1_ETHERNET_CLOCK_RESET_GUARD_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "Ethernet driver readiness",
    "broad Ethernet MMIO readiness",
    "RP1 MMIO writes",
    "clock/reset writes",
    "clock/reset ownership",
    "RP1_CLK_SYS transition through pclk or hclk",
    "reset-controller ownership",
    "GPIO32 ownership or PHY reset assertion/deassertion",
    "MDIO transactions or PHY ownership",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_CLOCK_RESET_GUARD_READ_ONLY_BASELINE_REQUIREMENTS: &[&str] = &[
    "same candidate/control report path",
    "observed-window MACB_MID identity context only",
    "pclk/hclk/tsu_clk/tx_clk source-backed names and IDs",
    "pclk and hclk marked as shared RP1_CLK_SYS inputs",
    "tx_clk and tsu_clk marked as Ethernet-specific source IDs without accepted Talos register write targets",
    "no accepted rp1_eth reset-controller target in retained Pi 5 device-tree source",
    "paired no-clock-reset/no-Ethernet control withholding candidate-only clock/reset facts",
];

pub const RP1_ETHERNET_CLOCK_RESET_GUARD_WRITE_BACKED_INVARIANTS: &[&str] = &[
    "do not disable, gate, or transition RP1_CLK_SYS through pclk or hclk",
    "do not touch reset-controller state without a Pi 5 rp1_eth reset target source contract and restore semantics",
    "do not fold GPIO32 PHY reset or MDIO ownership into clock/reset ownership",
    "future writable clock work requires pre-read, post-read, restore-write, and restore-read evidence",
    "preserve non-target clock fields unless a future contract explicitly selects them",
    "include paired no-clock-reset/no-Ethernet control before any hardware proof",
    "reject inference from clock/reset ownership to PHY/MDIO, interrupts, DMA, descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition",
];

pub const RP1_ETHERNET_CLOCK_RESET_GUARD_RETAINED_RISKS: &[&str] = &[
    "Clock names and IDs are source facts, not Talos ownership",
    "pclk and hclk share RP1_CLK_SYS and require shared-clock safety before any write-backed task",
    "tx_clk and tsu_clk still need exact register targets and restore semantics before writes",
    "retained Pi 5 rp1_eth source supplies no accepted reset-controller target",
    "PHY reset remains a separate GPIO32/MDIO ownership problem",
];

pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_PRESERVED_FIELDS: &[&str] = &[
    "full 32-bit raw CLK_ETH_TSU_CTRL value",
    "CLK_CTRL_ENABLE bit 11",
    "CLK_CTRL_AUXSRC bits 9:5",
    "clock source bits starting at bit 0",
    "reserved or currently undocumented bits",
];

pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_OPERATION_SEQUENCE: &[&str] = &[
    "pre-read CLK_ETH_TSU_CTRL and retain pre_raw",
    "write pre_raw back to CLK_ETH_TSU_CTRL",
    "post-read CLK_ETH_TSU_CTRL and retain post_raw",
    "restore-write pre_raw back to CLK_ETH_TSU_CTRL",
    "restore-read CLK_ETH_TSU_CTRL and retain restore_raw",
];

pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SAFETY_INVARIANTS: &[&str] = &[
    "do not disable, gate, or transition RP1_CLK_SYS through pclk or hclk",
    "do not write CLK_ETH_CTRL, divider, select, PLL, frequency-counter, or GPCLK output-enable registers",
    "do not use reset-controller writes without a separate accepted target and restore contract",
    "do not assert or deassert GPIO32 PHY reset",
    "do not perform MDIO transactions",
    "do not infer DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition",
];

pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "Ethernet driver readiness",
    "broad Ethernet MMIO readiness",
    "unscoped RP1 MMIO writes",
    "RP1_CLK_SYS pclk/hclk write or transition",
    "CLK_ETH_CTRL write",
    "reset-controller ownership",
    "GPIO32 ownership or PHY reset assertion/deassertion",
    "MDIO transactions or PHY ownership",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_RETAINED_RISKS: &[&str] = &[
    "The selected idempotent write/restore proves only one Ethernet-private clock-manager store/readback boundary",
    "CLK_ETH_CTRL remains unselected and requires separate acceptance criteria",
    "PHY reset remains GPIO32/MDIO-owned and outside this task",
    "A future Pi 5 proof must still capture identity, TFTP, serial freshness, final identity, and restore evidence",
];

pub const RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS: &[&str] = &[
    "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored",
    "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-mismatch-restored",
    "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore-failed",
    "rp1-ethernet-clk-eth-tsu-ctrl-blocked-missing-clock-manager",
    "rp1-ethernet-clk-eth-tsu-ctrl-inconclusive-capture",
    "no-clock-write-no-ethernet-rp1-ethernet-write-restore-control",
    "staging/build-blocker",
];

pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_PRESERVED_FIELDS: &[&str] = &[
    "full 32-bit raw CLK_ETH_CTRL value",
    "CLK_CTRL_ENABLE bit 11",
    "CLK_CTRL_AUXSRC bits 9:5",
    "clock source bits starting at bit 0",
    "reserved or currently undocumented bits",
];

pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_OPERATION_SEQUENCE: &[&str] = &[
    "pre-read CLK_ETH_CTRL and retain pre_raw",
    "write pre_raw back to CLK_ETH_CTRL",
    "post-read CLK_ETH_CTRL and retain post_raw",
    "restore-write pre_raw back to CLK_ETH_CTRL",
    "restore-read CLK_ETH_CTRL and retain restore_raw",
];

pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SAFETY_INVARIANTS: &[&str] = &[
    "do not disable, gate, or transition RP1_CLK_SYS through pclk or hclk",
    "do not write CLK_ETH_TSU_CTRL, divider, select, PLL, frequency-counter, or GPCLK output-enable registers",
    "do not use reset-controller writes without a separate accepted target and restore contract",
    "do not assert or deassert GPIO32 PHY reset",
    "do not perform MDIO transactions",
    "do not infer DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition",
];

pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "Ethernet driver readiness",
    "broad Ethernet MMIO readiness",
    "unscoped RP1 MMIO writes",
    "RP1_CLK_SYS pclk/hclk write or transition",
    "CLK_ETH_TSU_CTRL same-shaped retry",
    "non-idempotent CLK_ETH_CTRL transition",
    "reset-controller ownership",
    "GPIO32 ownership or PHY reset assertion/deassertion",
    "MDIO transactions or PHY ownership",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_RETAINED_RISKS: &[&str] = &[
    "The selected idempotent write/restore proves only one Ethernet-private tx_clk clock-manager store/readback boundary",
    "Shared RP1_CLK_SYS pclk/hclk ownership remains unaccepted",
    "CLK_ETH_TSU_CTRL is a separate accepted proof and is not retried by this target",
    "PHY reset remains GPIO32/MDIO-owned and outside this task",
    "A future Pi 5 proof must still capture identity, TFTP, serial freshness, final identity, and restore evidence",
];

pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_FUTURE_CLASSIFICATIONS: &[&str] = &[
    "rp1-ethernet-clk-eth-ctrl-idempotent-write-restored",
    "rp1-ethernet-clk-eth-ctrl-idempotent-write-mismatch-restored",
    "rp1-ethernet-clk-eth-ctrl-idempotent-write-restore-failed",
    "rp1-ethernet-clk-eth-ctrl-blocked-missing-clock-manager",
    "rp1-ethernet-clk-eth-ctrl-inconclusive-capture",
    "no-clk-eth-ctrl-write-no-ethernet-rp1-ethernet-control",
    "staging/build-blocker",
];

pub const RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-source-contract.md",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-clock.h",
    "tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c",
    "tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout.md",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_ACCEPTED_INPUT_FRONTIER: &[&str] = &[
    "observed-window MACB_MID identity context only",
    "prerequisite ownership report visibility/control output",
    "CLK_ETH_TSU_CTRL write/restore proof closeout",
    "CLK_ETH_CTRL write/restore proof closeout",
    "accepted Phase 11 GPIO source/status frontiers without ownership",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_PHASE11_GPIO_CONSTRAINTS: &[&str] = &[
    "GPIO ownership remains unaccepted",
    "GPIO function changes remain unaccepted",
    "RIO OUT/OE/IN writes remain unaccepted",
    "pad writes remain unaccepted",
    "INTE/CTRL writes remain unaccepted",
    "event generation and interrupt delivery remain unaccepted",
    "GPIO write/restore authority remains unaccepted",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "GPIO ownership",
    "PHY reset assertion or deassertion",
    "MDIO transactions or PHY ownership",
    "runtime Ethernet driver readiness",
    "broad Ethernet MMIO readiness",
    "RP1 MMIO, GPIO, RIO, pad, INTE, CTRL, or clock writes",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_RETAINED_RISKS: &[&str] = &[
    "GPIO32/ETH_RST_N source facts do not prove Talos can safely drive or restore the line",
    "Phase 11 GPIO frontiers have not accepted ownership or write/restore authority for GPIO32",
    "Linux ties the reset sequence to MDIO bus reset; Talos still lacks accepted MDIO/PHY ownership",
    "A later hardware proof must still use candidate/control identity, serial freshness, TFTP delta, final identity, restore evidence, and task-owned JSON if hardware publication is selected",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_FUTURE_WRITE_RESTORE_INVARIANTS: &[&str] = &[
    "require accepted GPIO32 ownership or precise pre-state/restore contract before any write-backed task",
    "capture source-backed and hardware-visible GPIO function, RIO OUT/OE/IN, pad state, and required output-enable state before assertion",
    "preserve active-low logical-to-physical ETH_RST_N polarity handling",
    "capture pre-state, bounded assertion duration, deassertion, restore, and post-restore readback evidence",
    "include paired no-GPIO/no-Ethernet control on the same report/capture path",
    "classify assertion mismatch, deassertion mismatch, restore failure, staging/capture blocker, and source-contract blocker separately",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_PRECONDITIONS: &[&str] = &[
    "selected GPIO32 STATUS/CTRL, RIO OUT/OE/IN, and required pad reads are visible and non-sentinel",
    "GPIO32 CTRL FUNCSEL remains accepted GPIO function value 5",
    "GPIO32 CTRL OUTOVER/OEOVER/INOVER fields do not bypass the raw RIO OUT/OE path",
    "GPIO32 CTRL IRQ enable bits 20-27 and IRQRESET bit 28 do not make the task an interrupt/event task",
    "pad OUT_DISABLE/IN_ENABLE state is safe for the accepted write scope",
    "complete restore baseline exists for every register or bit the candidate would write",
    "candidate/control identity, selected-tree/TFTP, serial freshness, final identity, and restore evidence can be joined",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BASELINE_FIELDS: &[&str] = &[
    "GPIO32 STATUS raw value",
    "GPIO32 CTRL raw value",
    "RIO1 OUT raw value and bank-local bit 4",
    "RIO1 OE raw value and bank-local bit 4",
    "RIO1 IN raw value and bank-local bit 4",
    "GPIO32 pad raw value",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_OPERATION_SEQUENCE: &[&str] = &[
    "capture baseline raw values for GPIO32 STATUS, GPIO32 CTRL, RIO1 OUT, RIO1 OE, RIO1 IN, and GPIO32 pad",
    "check no-write preconditions before any store",
    "assert active-low ETH_RST_N by output-enabling GPIO32 bank1 bit 4 and driving raw output low",
    "wait source-backed 5 ms reset duration",
    "deassert ETH_RST_N by driving GPIO32 bank1 bit 4 raw output high",
    "restore every touched register or bit to captured baseline",
    "read back every touched field and classify success only when restore equals baseline",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BLOCKED_CLASSIFICATIONS: &[&str] = &[
    "rp1-ethernet-gpio32-phy-reset-blocked-sentinel-read",
    "rp1-ethernet-gpio32-phy-reset-blocked-unsafe-function",
    "rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state",
    "rp1-ethernet-gpio32-phy-reset-blocked-missing-restore-baseline",
    "rp1-ethernet-gpio32-phy-reset-inconclusive-capture",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS: &[&str] = &[
    "rp1-ethernet-gpio32-phy-reset-write-restored",
    "rp1-ethernet-gpio32-phy-reset-write-assertion-mismatch-restored",
    "rp1-ethernet-gpio32-phy-reset-write-deassertion-mismatch-restored",
    "rp1-ethernet-gpio32-phy-reset-write-restore-failed",
    "rp1-ethernet-gpio32-phy-reset-blocked-sentinel-read",
    "rp1-ethernet-gpio32-phy-reset-blocked-unsafe-function",
    "rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state",
    "rp1-ethernet-gpio32-phy-reset-blocked-missing-restore-baseline",
    "rp1-ethernet-gpio32-phy-reset-inconclusive-capture",
    "no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control",
    "staging/build-blocker",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "Ethernet driver readiness",
    "broad Ethernet MMIO readiness",
    "non-GPIO32 writes",
    "MDIO transactions or PHY ownership",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_RETAINED_RISKS: &[&str] = &[
    "The guard surface is local/static and does not prove GPIO32 can be safely driven on Pi 5",
    "A future hardware proof may still block before writing if preconditions or capture-chain joins fail",
    "MDIO/PHY register ownership remains separate from GPIO32 ETH_RST_N assertion/deassertion",
    "Ethernet driver, interrupt, DMA, descriptor, packet, networking, socket, SSH, and Phase 12.2 behavior remain unaccepted",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract.md",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-cdns-macb.yaml",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c",
    "tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/source-reference-notes.md",
];

pub const RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract.md",
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-proof-closeout.md",
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract.md",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts",
    "tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/source-reference-notes.md",
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
pub struct Rp1EthernetPrereqOwnershipSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub selected_prerequisite: &'static str,
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub rp1_bus_base: u64,
    pub rp1_bus_window_size: u64,
    pub observed_identity_target: u64,
    pub translated_comparator_target: u64,
    pub accepted_macb_mid_raw: u32,
    pub accepted_macb_mid_idnum: u32,
    pub accepted_macb_mid_rev: u32,
    pub identity_role: &'static str,
    pub interrupt_name: &'static str,
    pub interrupt_number: u32,
    pub clock_names: &'static [&'static str],
    pub clock_sources: &'static [&'static str],
    pub clock_ids: &'static [u32],
    pub clock_policy_classification: &'static str,
    pub phy_mode: &'static str,
    pub phy_handle: &'static str,
    pub phy_node: &'static str,
    pub phy_reg: u32,
    pub phy_reset_gpio: u32,
    pub phy_reset_active_low: bool,
    pub phy_reset_duration_ms: u32,
    pub phy_mdio_policy_classification: &'static str,
    pub dma_descriptor_policy_classification: &'static str,
    pub cadence_rp1_config: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetPrereqOwnershipReportKind {
    Candidate,
    NoOwnershipNoEthernetControl,
}

impl Rp1EthernetPrereqOwnershipReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoOwnershipNoEthernetControl => "no-ownership-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPrereqOwnershipReportInput {
    pub kind: Rp1EthernetPrereqOwnershipReportKind,
    pub source_contract: Option<Rp1EthernetPrereqOwnershipSourceContractEvidence>,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_rp1_mmio_writes: bool,
    pub claims_clock_reset_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPrereqOwnershipReport {
    pub kind: Rp1EthernetPrereqOwnershipReportKind,
    pub source_contract: Option<Rp1EthernetPrereqOwnershipSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPrereqOwnershipReportEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub selected_prerequisite: Option<&'static str>,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub rp1_bus_base: Option<u64>,
    pub rp1_bus_window_size: Option<u64>,
    pub observed_identity_target: Option<u64>,
    pub translated_comparator_target: Option<u64>,
    pub accepted_macb_mid_raw: Option<u32>,
    pub accepted_macb_mid_idnum: Option<u32>,
    pub accepted_macb_mid_rev: Option<u32>,
    pub identity_role: Option<&'static str>,
    pub interrupt_name: Option<&'static str>,
    pub interrupt_number: Option<u32>,
    pub clock_names: Option<&'static [&'static str]>,
    pub clock_sources: Option<&'static [&'static str]>,
    pub clock_ids: Option<&'static [u32]>,
    pub clock_policy_classification: Option<&'static str>,
    pub phy_mode: Option<&'static str>,
    pub phy_handle: Option<&'static str>,
    pub phy_node: Option<&'static str>,
    pub phy_reg: Option<u32>,
    pub phy_reset_gpio: Option<u32>,
    pub phy_reset_active_low: Option<bool>,
    pub phy_reset_duration_ms: Option<u32>,
    pub phy_mdio_policy_classification: Option<&'static str>,
    pub dma_descriptor_policy_classification: Option<&'static str>,
    pub cadence_rp1_config: Option<&'static [&'static str]>,
    pub source_evidence: Option<&'static [&'static str]>,
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_rp1_mmio_writes: bool,
    pub claims_clock_reset_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetGuardContractEvidence {
    pub guard_contract_id: &'static str,
    pub ownership_contract_task_id: &'static str,
    pub prereq_contract_id: &'static str,
    pub observed_identity_target: u64,
    pub translated_comparator_target: u64,
    pub accepted_macb_mid_raw: u32,
    pub accepted_macb_mid_idnum: u32,
    pub accepted_macb_mid_rev: u32,
    pub identity_role: &'static str,
    pub clock_names: &'static [&'static str],
    pub clock_sources: &'static [&'static str],
    pub clock_ids: &'static [u32],
    pub shared_clock_names: &'static [&'static str],
    pub shared_clock_source: &'static str,
    pub shared_clock_id: u32,
    pub ethernet_private_clock_names: &'static [&'static str],
    pub ethernet_private_clock_sources: &'static [&'static str],
    pub ethernet_private_clock_ids: &'static [u32],
    pub clock_policy_classification: &'static str,
    pub reset_controller_policy_classification: &'static str,
    pub phy_reset_gpio: u32,
    pub phy_mdio_policy_classification: &'static str,
    pub read_only_baseline_requirements: &'static [&'static str],
    pub write_backed_invariants: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetClockResetGuardReportKind {
    Candidate,
    NoClockResetNoEthernetControl,
}

impl Rp1EthernetClockResetGuardReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoClockResetNoEthernetControl => "no-clock-reset-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetGuardReportInput {
    pub kind: Rp1EthernetClockResetGuardReportKind,
    pub guard_contract: Option<Rp1EthernetClockResetGuardContractEvidence>,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_rp1_mmio_writes: bool,
    pub claims_clock_reset_writes: bool,
    pub claims_clock_reset_ownership: bool,
    pub claims_rp1_clk_sys_transition: bool,
    pub claims_reset_controller_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetGuardReport {
    pub kind: Rp1EthernetClockResetGuardReportKind,
    pub guard_contract: Option<Rp1EthernetClockResetGuardContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetGuardReportEvidence {
    pub guard_contract_id: &'static str,
    pub ownership_contract_task_id: &'static str,
    pub prereq_contract_id: &'static str,
    pub report_kind: &'static str,
    pub observed_identity_target: Option<u64>,
    pub translated_comparator_target: Option<u64>,
    pub accepted_macb_mid_raw: Option<u32>,
    pub accepted_macb_mid_idnum: Option<u32>,
    pub accepted_macb_mid_rev: Option<u32>,
    pub identity_role: Option<&'static str>,
    pub clock_names: Option<&'static [&'static str]>,
    pub clock_sources: Option<&'static [&'static str]>,
    pub clock_ids: Option<&'static [u32]>,
    pub shared_clock_names: Option<&'static [&'static str]>,
    pub shared_clock_source: Option<&'static str>,
    pub shared_clock_id: Option<u32>,
    pub ethernet_private_clock_names: Option<&'static [&'static str]>,
    pub ethernet_private_clock_sources: Option<&'static [&'static str]>,
    pub ethernet_private_clock_ids: Option<&'static [u32]>,
    pub clock_policy_classification: Option<&'static str>,
    pub reset_controller_policy_classification: Option<&'static str>,
    pub phy_reset_gpio: Option<u32>,
    pub phy_mdio_policy_classification: Option<&'static str>,
    pub read_only_baseline_requirements: Option<&'static [&'static str]>,
    pub write_backed_invariants: Option<&'static [&'static str]>,
    pub source_evidence: Option<&'static [&'static str]>,
    pub boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_rp1_mmio_writes: bool,
    pub claims_clock_reset_writes: bool,
    pub claims_clock_reset_ownership: bool,
    pub claims_rp1_clk_sys_transition: bool,
    pub claims_reset_controller_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetWriteTargetContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub target: &'static str,
    pub clock_name: &'static str,
    pub clock_id: u32,
    pub register: &'static str,
    pub source_block: &'static str,
    pub observed_rp1_base: u64,
    pub source_offset: u64,
    pub cpu_physical_target: u64,
    pub width_bits: u32,
    pub access: &'static str,
    pub allowed_write_value: &'static str,
    pub preserved_fields: &'static [&'static str],
    pub operation_sequence: &'static [&'static str],
    pub safety_invariants: &'static [&'static str],
    pub future_proof_classifications: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetClockResetWriteRestoreReportKind {
    Candidate,
    NoClockWriteNoEthernetControl,
}

impl Rp1EthernetClockResetWriteRestoreReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoClockWriteNoEthernetControl => "no-clock-write-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetWriteRestoreReportInput {
    pub kind: Rp1EthernetClockResetWriteRestoreReportKind,
    pub target_contract: Option<Rp1EthernetClockResetWriteTargetContractEvidence>,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_unscoped_rp1_mmio_writes: bool,
    pub claims_rp1_clk_sys_transition: bool,
    pub claims_clk_eth_ctrl_write: bool,
    pub claims_reset_controller_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetWriteRestoreReport {
    pub kind: Rp1EthernetClockResetWriteRestoreReportKind,
    pub target_contract: Option<Rp1EthernetClockResetWriteTargetContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClockResetWriteRestoreReportEvidence {
    pub report_contract_id: &'static str,
    pub target_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub target: Option<&'static str>,
    pub clock_name: Option<&'static str>,
    pub clock_id: Option<u32>,
    pub register: Option<&'static str>,
    pub source_block: Option<&'static str>,
    pub observed_rp1_base: Option<u64>,
    pub source_offset: Option<u64>,
    pub cpu_physical_target: Option<u64>,
    pub width_bits: Option<u32>,
    pub access: Option<&'static str>,
    pub allowed_write_value: Option<&'static str>,
    pub preserved_fields: Option<&'static [&'static str]>,
    pub operation_sequence: Option<&'static [&'static str]>,
    pub safety_invariants: Option<&'static [&'static str]>,
    pub post_eq_pre_required: Option<bool>,
    pub restore_eq_pre_required: Option<bool>,
    pub future_proof_classifications: &'static [&'static str],
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_unscoped_rp1_mmio_writes: bool,
    pub claims_rp1_clk_sys_transition: bool,
    pub claims_clk_eth_ctrl_write: bool,
    pub claims_reset_controller_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClkEthCtrlWriteRestoreReportInput {
    pub kind: Rp1EthernetClockResetWriteRestoreReportKind,
    pub target_contract: Option<Rp1EthernetClockResetWriteTargetContractEvidence>,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_unscoped_rp1_mmio_writes: bool,
    pub claims_shared_clock_write: bool,
    pub claims_tsu_same_shape_retry: bool,
    pub claims_non_idempotent_transition: bool,
    pub claims_reset_controller_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClkEthCtrlWriteRestoreReport {
    pub kind: Rp1EthernetClockResetWriteRestoreReportKind,
    pub target_contract: Option<Rp1EthernetClockResetWriteTargetContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetClkEthCtrlWriteRestoreReportEvidence {
    pub report_contract_id: &'static str,
    pub target_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub target: Option<&'static str>,
    pub clock_name: Option<&'static str>,
    pub clock_id: Option<u32>,
    pub register: Option<&'static str>,
    pub source_block: Option<&'static str>,
    pub observed_rp1_base: Option<u64>,
    pub source_offset: Option<u64>,
    pub cpu_physical_target: Option<u64>,
    pub width_bits: Option<u32>,
    pub access: Option<&'static str>,
    pub allowed_write_value: Option<&'static str>,
    pub preserved_fields: Option<&'static [&'static str]>,
    pub operation_sequence: Option<&'static [&'static str]>,
    pub safety_invariants: Option<&'static [&'static str]>,
    pub post_eq_pre_required: Option<bool>,
    pub restore_eq_pre_required: Option<bool>,
    pub future_proof_classifications: &'static [&'static str],
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_unscoped_rp1_mmio_writes: bool,
    pub claims_shared_clock_write: bool,
    pub claims_tsu_same_shape_retry: bool,
    pub claims_non_idempotent_transition: bool,
    pub claims_reset_controller_ownership: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub accepted_input_frontier: &'static [&'static str],
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub accepted_macb_mid_raw: u32,
    pub accepted_macb_mid_idnum: u32,
    pub accepted_macb_mid_rev: u32,
    pub identity_role: &'static str,
    pub phy_mode: &'static str,
    pub phy_handle: &'static str,
    pub phy_node: &'static str,
    pub phy_reg: u32,
    pub gpio_controller: &'static str,
    pub gpio_line: u32,
    pub reset_route: &'static str,
    pub active_low: bool,
    pub logical_assertion: &'static str,
    pub logical_deassertion: &'static str,
    pub reset_duration_ms: u32,
    pub mdio_reset_hook_relationship: &'static str,
    pub phase11_gpio_constraints: &'static [&'static str],
    pub future_write_restore_invariants: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32PhyResetPreflightReportKind {
    Candidate,
    NoGpioNoEthernetControl,
}

impl Rp1EthernetGpio32PhyResetPreflightReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoGpioNoEthernetControl => "no-gpio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetPreflightReportInput {
    pub kind: Rp1EthernetGpio32PhyResetPreflightReportKind,
    pub source_contract: Option<Rp1EthernetGpio32PhyResetSourceContractEvidence>,
    pub claims_gpio_ownership: bool,
    pub claims_phy_reset_assertion: bool,
    pub claims_phy_reset_deassertion: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_runtime_writes: bool,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetPreflightReport {
    pub kind: Rp1EthernetGpio32PhyResetPreflightReportKind,
    pub source_contract: Option<Rp1EthernetGpio32PhyResetSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetPreflightReportEvidence {
    pub report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub accepted_input_frontier: Option<&'static [&'static str]>,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub accepted_macb_mid_raw: Option<u32>,
    pub accepted_macb_mid_idnum: Option<u32>,
    pub accepted_macb_mid_rev: Option<u32>,
    pub identity_role: Option<&'static str>,
    pub phy_mode: Option<&'static str>,
    pub phy_handle: Option<&'static str>,
    pub phy_node: Option<&'static str>,
    pub phy_reg: Option<u32>,
    pub gpio_controller: Option<&'static str>,
    pub gpio_line: Option<u32>,
    pub reset_route: Option<&'static str>,
    pub active_low: Option<bool>,
    pub logical_assertion: Option<&'static str>,
    pub logical_deassertion: Option<&'static str>,
    pub reset_duration_ms: Option<u32>,
    pub mdio_reset_hook_relationship: Option<&'static str>,
    pub phase11_gpio_constraints: Option<&'static [&'static str]>,
    pub future_write_restore_invariants: Option<&'static [&'static str]>,
    pub source_evidence: Option<&'static [&'static str]>,
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub claims_gpio_ownership: bool,
    pub claims_phy_reset_assertion: bool,
    pub claims_phy_reset_deassertion: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_runtime_writes: bool,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence {
    pub guard_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_source_task_id: &'static str,
    pub gpio_controller: &'static str,
    pub gpio_line: u32,
    pub reset_route: &'static str,
    pub bank: &'static str,
    pub bank_local_bit: u32,
    pub io_bank1_source_base: u64,
    pub io_bank1_observed_base: u64,
    pub gpio32_status_source_target: u64,
    pub gpio32_status_observed_target: u64,
    pub gpio32_ctrl_source_target: u64,
    pub gpio32_ctrl_observed_target: u64,
    pub rio1_out_source_target: u64,
    pub rio1_out_observed_target: u64,
    pub rio1_oe_source_target: u64,
    pub rio1_oe_observed_target: u64,
    pub rio1_in_source_target: u64,
    pub rio1_in_observed_target: u64,
    pub gpio32_pad_source_target: u64,
    pub gpio32_pad_observed_target: u64,
    pub width_bits: u32,
    pub access: &'static str,
    pub active_low: bool,
    pub assertion_raw_output: &'static str,
    pub deassertion_raw_output: &'static str,
    pub reset_duration_ms: u32,
    pub no_write_preconditions: &'static [&'static str],
    pub restore_baseline_fields: &'static [&'static str],
    pub operation_sequence: &'static [&'static str],
    pub blocked_no_write_classifications: &'static [&'static str],
    pub future_proof_classifications: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind {
    Candidate,
    BlockedNoWrite,
    NoGpioWriteNoEthernetControl,
}

impl Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::BlockedNoWrite => "blocked-no-write",
            Self::NoGpioWriteNoEthernetControl => "no-gpio-write-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
    pub kind: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind,
    pub guard_contract: Option<Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence>,
    pub blocked_no_write_classification: Option<&'static str>,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_non_gpio32_write: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetWriteRestoreGuardReport {
    pub kind: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind,
    pub guard_contract: Option<Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence>,
    pub blocked_no_write_classification: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32PhyResetWriteRestoreGuardReportEvidence {
    pub report_contract_id: &'static str,
    pub guard_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_source_task_id: &'static str,
    pub report_kind: &'static str,
    pub gpio_controller: Option<&'static str>,
    pub gpio_line: Option<u32>,
    pub reset_route: Option<&'static str>,
    pub bank: Option<&'static str>,
    pub bank_local_bit: Option<u32>,
    pub io_bank1_source_base: Option<u64>,
    pub io_bank1_observed_base: Option<u64>,
    pub gpio32_status_source_target: Option<u64>,
    pub gpio32_status_observed_target: Option<u64>,
    pub gpio32_ctrl_source_target: Option<u64>,
    pub gpio32_ctrl_observed_target: Option<u64>,
    pub rio1_out_source_target: Option<u64>,
    pub rio1_out_observed_target: Option<u64>,
    pub rio1_oe_source_target: Option<u64>,
    pub rio1_oe_observed_target: Option<u64>,
    pub rio1_in_source_target: Option<u64>,
    pub rio1_in_observed_target: Option<u64>,
    pub gpio32_pad_source_target: Option<u64>,
    pub gpio32_pad_observed_target: Option<u64>,
    pub width_bits: Option<u32>,
    pub access: Option<&'static str>,
    pub active_low: Option<bool>,
    pub assertion_raw_output: Option<&'static str>,
    pub deassertion_raw_output: Option<&'static str>,
    pub reset_duration_ms: Option<u32>,
    pub no_write_preconditions: Option<&'static [&'static str]>,
    pub restore_baseline_fields: Option<&'static [&'static str]>,
    pub operation_sequence: Option<&'static [&'static str]>,
    pub blocked_no_write_classifications: &'static [&'static str],
    pub future_proof_classifications: &'static [&'static str],
    pub blocked_no_write_classification: Option<&'static str>,
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_non_gpio32_write: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetPrereqOwnershipReportError {
    CandidateMissingSourceContract,
    ControlCarriesEthernetPrereqFacts,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    Rp1MmioWritesClaim,
    ClockResetOwnershipClaim,
    Gpio32PhyResetOwnershipClaim,
    MdioPhyOwnershipClaim,
    InterruptOwnershipClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetClockResetGuardReportError {
    CandidateMissingGuardContract,
    ControlCarriesClockResetFacts,
    GuardContractIdentityMismatch,
    GuardContractTargetMismatch,
    GuardContractFieldMismatch,
    MissingSourceEvidence,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    Rp1MmioWritesClaim,
    ClockResetWritesClaim,
    ClockResetOwnershipClaim,
    Rp1ClkSysTransitionClaim,
    ResetControllerOwnershipClaim,
    Gpio32PhyResetOwnershipClaim,
    MdioPhyOwnershipClaim,
    InterruptOwnershipClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetClockResetWriteRestoreReportError {
    CandidateMissingTargetContract,
    ControlCarriesWriteTargetFacts,
    TargetContractIdentityMismatch,
    TargetContractTargetMismatch,
    TargetContractFieldMismatch,
    MissingSourceEvidence,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    UnscopedRp1MmioWritesClaim,
    Rp1ClkSysTransitionClaim,
    ClkEthCtrlWriteClaim,
    ResetControllerOwnershipClaim,
    Gpio32PhyResetOwnershipClaim,
    MdioPhyOwnershipClaim,
    InterruptOwnershipClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetClkEthCtrlWriteRestoreReportError {
    CandidateMissingTargetContract,
    ControlCarriesWriteTargetFacts,
    TargetContractIdentityMismatch,
    TargetContractTargetMismatch,
    TargetContractFieldMismatch,
    MissingSourceEvidence,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    UnscopedRp1MmioWritesClaim,
    SharedClockWriteClaim,
    TsuSameShapeRetryClaim,
    NonIdempotentTransitionClaim,
    ResetControllerOwnershipClaim,
    Gpio32PhyResetOwnershipClaim,
    MdioPhyOwnershipClaim,
    InterruptOwnershipClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32PhyResetPreflightReportError {
    CandidateMissingSourceContract,
    ControlCarriesGpioPhyResetFacts,
    SourceContractIdentityMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    GpioOwnershipClaim,
    PhyResetAssertionClaim,
    PhyResetDeassertionClaim,
    MdioPhyOwnershipClaim,
    RuntimeWritesClaim,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    InterruptOwnershipClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError {
    CandidateMissingGuardContract,
    BlockedMissingGuardContract,
    BlockedMissingClassification,
    BlockedClassificationNotAllowed,
    ControlCarriesGpioWriteFacts,
    GuardContractIdentityMismatch,
    GuardContractTargetMismatch,
    GuardContractFieldMismatch,
    MissingRestoreBaseline,
    MissingSourceEvidence,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    NonGpio32WriteClaim,
    MdioPhyOwnershipClaim,
    InterruptOwnershipClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingGuardContract => "candidate-missing-guard-contract",
            Self::BlockedMissingGuardContract => "blocked-missing-guard-contract",
            Self::BlockedMissingClassification => "blocked-missing-classification",
            Self::BlockedClassificationNotAllowed => "blocked-classification-not-allowed",
            Self::ControlCarriesGpioWriteFacts => "control-carries-gpio-write-facts",
            Self::GuardContractIdentityMismatch => "guard-contract-identity-mismatch",
            Self::GuardContractTargetMismatch => "guard-contract-target-mismatch",
            Self::GuardContractFieldMismatch => "guard-contract-field-mismatch",
            Self::MissingRestoreBaseline => "missing-restore-baseline",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::NonGpio32WriteClaim => "non-gpio32-write-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::InterruptOwnershipClaim => "interrupt-ownership-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

impl Rp1EthernetGpio32PhyResetPreflightReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesGpioPhyResetFacts => "control-carries-gpio-phy-reset-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::GpioOwnershipClaim => "gpio-ownership-claim",
            Self::PhyResetAssertionClaim => "phy-reset-assertion-claim",
            Self::PhyResetDeassertionClaim => "phy-reset-deassertion-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::RuntimeWritesClaim => "runtime-writes-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::InterruptOwnershipClaim => "interrupt-ownership-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

impl Rp1EthernetClkEthCtrlWriteRestoreReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingTargetContract => "candidate-missing-target-contract",
            Self::ControlCarriesWriteTargetFacts => "control-carries-write-target-facts",
            Self::TargetContractIdentityMismatch => "target-contract-identity-mismatch",
            Self::TargetContractTargetMismatch => "target-contract-target-mismatch",
            Self::TargetContractFieldMismatch => "target-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::UnscopedRp1MmioWritesClaim => "unscoped-rp1-mmio-writes-claim",
            Self::SharedClockWriteClaim => "shared-clock-write-claim",
            Self::TsuSameShapeRetryClaim => "clk-eth-tsu-ctrl-same-shaped-retry-claim",
            Self::NonIdempotentTransitionClaim => "non-idempotent-transition-claim",
            Self::ResetControllerOwnershipClaim => "reset-controller-ownership-claim",
            Self::Gpio32PhyResetOwnershipClaim => "gpio32-phy-reset-ownership-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::InterruptOwnershipClaim => "interrupt-ownership-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

impl Rp1EthernetClockResetWriteRestoreReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingTargetContract => "candidate-missing-target-contract",
            Self::ControlCarriesWriteTargetFacts => "control-carries-write-target-facts",
            Self::TargetContractIdentityMismatch => "target-contract-identity-mismatch",
            Self::TargetContractTargetMismatch => "target-contract-target-mismatch",
            Self::TargetContractFieldMismatch => "target-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::UnscopedRp1MmioWritesClaim => "unscoped-rp1-mmio-writes-claim",
            Self::Rp1ClkSysTransitionClaim => "rp1-clk-sys-transition-claim",
            Self::ClkEthCtrlWriteClaim => "clk-eth-ctrl-write-claim",
            Self::ResetControllerOwnershipClaim => "reset-controller-ownership-claim",
            Self::Gpio32PhyResetOwnershipClaim => "gpio32-phy-reset-ownership-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::InterruptOwnershipClaim => "interrupt-ownership-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

impl Rp1EthernetClockResetGuardReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingGuardContract => "candidate-missing-guard-contract",
            Self::ControlCarriesClockResetFacts => "control-carries-clock-reset-facts",
            Self::GuardContractIdentityMismatch => "guard-contract-identity-mismatch",
            Self::GuardContractTargetMismatch => "guard-contract-target-mismatch",
            Self::GuardContractFieldMismatch => "guard-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::Rp1MmioWritesClaim => "rp1-mmio-writes-claim",
            Self::ClockResetWritesClaim => "clock-reset-writes-claim",
            Self::ClockResetOwnershipClaim => "clock-reset-ownership-claim",
            Self::Rp1ClkSysTransitionClaim => "rp1-clk-sys-transition-claim",
            Self::ResetControllerOwnershipClaim => "reset-controller-ownership-claim",
            Self::Gpio32PhyResetOwnershipClaim => "gpio32-phy-reset-ownership-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::InterruptOwnershipClaim => "interrupt-ownership-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

impl Rp1EthernetPrereqOwnershipReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesEthernetPrereqFacts => "control-carries-ethernet-prereq-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::Rp1MmioWritesClaim => "rp1-mmio-writes-claim",
            Self::ClockResetOwnershipClaim => "clock-reset-ownership-claim",
            Self::Gpio32PhyResetOwnershipClaim => "gpio32-phy-reset-ownership-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::InterruptOwnershipClaim => "interrupt-ownership-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
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

pub const fn rp1_ethernet_prereq_ownership_source_contract_evidence()
-> Rp1EthernetPrereqOwnershipSourceContractEvidence {
    Rp1EthernetPrereqOwnershipSourceContractEvidence {
        contract_id: RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_TASK_ID,
        selected_prerequisite: RP1_ETHERNET_SELECTED_PREREQUISITE,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        rp1_bus_base: RP1_ETHERNET_RP1_BUS_BASE,
        rp1_bus_window_size: RP1_ETHERNET_RP1_BUS_WINDOW_SIZE,
        observed_identity_target: RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET,
        translated_comparator_target: RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET,
        accepted_macb_mid_raw: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW,
        accepted_macb_mid_idnum: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_IDNUM,
        accepted_macb_mid_rev: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_REV,
        identity_role: RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE,
        interrupt_name: RP1_ETHERNET_INTERRUPT_NAME,
        interrupt_number: RP1_ETHERNET_INTERRUPT_NUMBER,
        clock_names: RP1_ETHERNET_CLOCK_NAMES,
        clock_sources: RP1_ETHERNET_CLOCK_SOURCES,
        clock_ids: RP1_ETHERNET_CLOCK_IDS,
        clock_policy_classification: RP1_ETHERNET_CLOCK_POLICY_CLASSIFICATION,
        phy_mode: RP1_ETHERNET_PHY_MODE,
        phy_handle: RP1_ETHERNET_PHY_HANDLE,
        phy_node: RP1_ETHERNET_PHY_NODE,
        phy_reg: RP1_ETHERNET_PHY_REG,
        phy_reset_gpio: RP1_ETHERNET_PHY_RESET_GPIO,
        phy_reset_active_low: RP1_ETHERNET_PHY_RESET_ACTIVE_LOW,
        phy_reset_duration_ms: RP1_ETHERNET_PHY_RESET_DURATION_MS,
        phy_mdio_policy_classification: RP1_ETHERNET_PHY_MDIO_POLICY_CLASSIFICATION,
        dma_descriptor_policy_classification: RP1_ETHERNET_DMA_DESCRIPTOR_POLICY_CLASSIFICATION,
        cadence_rp1_config: RP1_ETHERNET_CADENCE_RP1_CONFIG,
        source_evidence: RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE,
    }
}

pub const fn rp1_ethernet_clock_reset_guard_contract_evidence()
-> Rp1EthernetClockResetGuardContractEvidence {
    Rp1EthernetClockResetGuardContractEvidence {
        guard_contract_id: RP1_ETHERNET_CLOCK_RESET_GUARD_CONTRACT_ID,
        ownership_contract_task_id: RP1_ETHERNET_CLOCK_RESET_OWNERSHIP_CONTRACT_TASK_ID,
        prereq_contract_id: RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID,
        observed_identity_target: RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET,
        translated_comparator_target: RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET,
        accepted_macb_mid_raw: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW,
        accepted_macb_mid_idnum: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_IDNUM,
        accepted_macb_mid_rev: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_REV,
        identity_role: RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE,
        clock_names: RP1_ETHERNET_CLOCK_NAMES,
        clock_sources: RP1_ETHERNET_CLOCK_SOURCES,
        clock_ids: RP1_ETHERNET_CLOCK_IDS,
        shared_clock_names: RP1_ETHERNET_SHARED_CLOCK_NAMES,
        shared_clock_source: RP1_ETHERNET_SHARED_CLOCK_SOURCE,
        shared_clock_id: RP1_ETHERNET_SHARED_CLOCK_ID,
        ethernet_private_clock_names: RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_NAMES,
        ethernet_private_clock_sources: RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_SOURCES,
        ethernet_private_clock_ids: RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_IDS,
        clock_policy_classification: RP1_ETHERNET_CLOCK_POLICY_CLASSIFICATION,
        reset_controller_policy_classification: RP1_ETHERNET_RESET_CONTROLLER_POLICY_CLASSIFICATION,
        phy_reset_gpio: RP1_ETHERNET_PHY_RESET_GPIO,
        phy_mdio_policy_classification: RP1_ETHERNET_PHY_MDIO_POLICY_CLASSIFICATION,
        read_only_baseline_requirements:
            RP1_ETHERNET_CLOCK_RESET_GUARD_READ_ONLY_BASELINE_REQUIREMENTS,
        write_backed_invariants: RP1_ETHERNET_CLOCK_RESET_GUARD_WRITE_BACKED_INVARIANTS,
        source_evidence: RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE,
    }
}

pub const fn rp1_ethernet_clock_reset_write_target_contract_evidence()
-> Rp1EthernetClockResetWriteTargetContractEvidence {
    Rp1EthernetClockResetWriteTargetContractEvidence {
        contract_id: RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_SOURCE_TASK_ID,
        target: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_TARGET,
        clock_name: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CLOCK_NAME,
        clock_id: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CLOCK_ID,
        register: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REGISTER,
        source_block: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SOURCE_BLOCK,
        observed_rp1_base: RP1_ETHERNET_OBSERVED_RP1_BASE,
        source_offset: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SOURCE_OFFSET,
        cpu_physical_target: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CPU_PHYSICAL_TARGET,
        width_bits: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_WIDTH_BITS,
        access: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_ACCESS,
        allowed_write_value: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_ALLOWED_WRITE_VALUE,
        preserved_fields: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_PRESERVED_FIELDS,
        operation_sequence: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_OPERATION_SEQUENCE,
        safety_invariants: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SAFETY_INVARIANTS,
        future_proof_classifications: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE,
    }
}

pub const fn rp1_ethernet_clk_eth_ctrl_write_target_contract_evidence()
-> Rp1EthernetClockResetWriteTargetContractEvidence {
    Rp1EthernetClockResetWriteTargetContractEvidence {
        contract_id: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_SOURCE_TASK_ID,
        target: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_TARGET,
        clock_name: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CLOCK_NAME,
        clock_id: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CLOCK_ID,
        register: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REGISTER,
        source_block: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_BLOCK,
        observed_rp1_base: RP1_ETHERNET_OBSERVED_RP1_BASE,
        source_offset: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_OFFSET,
        cpu_physical_target: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CPU_PHYSICAL_TARGET,
        width_bits: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_WIDTH_BITS,
        access: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_ACCESS,
        allowed_write_value: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_ALLOWED_WRITE_VALUE,
        preserved_fields: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_PRESERVED_FIELDS,
        operation_sequence: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_OPERATION_SEQUENCE,
        safety_invariants: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SAFETY_INVARIANTS,
        future_proof_classifications:
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_FUTURE_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_EVIDENCE,
    }
}

pub const fn rp1_ethernet_gpio32_phy_reset_source_contract_evidence()
-> Rp1EthernetGpio32PhyResetSourceContractEvidence {
    Rp1EthernetGpio32PhyResetSourceContractEvidence {
        contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID,
        accepted_input_frontier: RP1_ETHERNET_GPIO32_PHY_RESET_ACCEPTED_INPUT_FRONTIER,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        accepted_macb_mid_raw: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW,
        accepted_macb_mid_idnum: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_IDNUM,
        accepted_macb_mid_rev: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_REV,
        identity_role: RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE,
        phy_mode: RP1_ETHERNET_PHY_MODE,
        phy_handle: RP1_ETHERNET_PHY_HANDLE,
        phy_node: RP1_ETHERNET_PHY_NODE,
        phy_reg: RP1_ETHERNET_PHY_REG,
        gpio_controller: RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER,
        gpio_line: RP1_ETHERNET_PHY_RESET_GPIO,
        reset_route: RP1_ETHERNET_PHY_RESET_ROUTE,
        active_low: RP1_ETHERNET_PHY_RESET_ACTIVE_LOW,
        logical_assertion: RP1_ETHERNET_PHY_RESET_LOGICAL_ASSERTION,
        logical_deassertion: RP1_ETHERNET_PHY_RESET_LOGICAL_DEASSERTION,
        reset_duration_ms: RP1_ETHERNET_PHY_RESET_DURATION_MS,
        mdio_reset_hook_relationship: RP1_ETHERNET_PHY_RESET_MDIO_HOOK_RELATIONSHIP,
        phase11_gpio_constraints: RP1_ETHERNET_GPIO32_PHY_RESET_PHASE11_GPIO_CONSTRAINTS,
        future_write_restore_invariants:
            RP1_ETHERNET_GPIO32_PHY_RESET_FUTURE_WRITE_RESTORE_INVARIANTS,
        source_evidence: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_EVIDENCE,
    }
}

pub const fn rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract_evidence()
-> Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence {
    Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence {
        guard_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID,
        report_source_task_id: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_TASK_ID,
        gpio_controller: RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER,
        gpio_line: RP1_ETHERNET_PHY_RESET_GPIO,
        reset_route: RP1_ETHERNET_PHY_RESET_ROUTE,
        bank: RP1_ETHERNET_GPIO32_BANK,
        bank_local_bit: RP1_ETHERNET_GPIO32_BANK_LOCAL_BIT,
        io_bank1_source_base: RP1_ETHERNET_GPIO32_IO_BANK1_SOURCE_BASE,
        io_bank1_observed_base: RP1_ETHERNET_GPIO32_IO_BANK1_OBSERVED_BASE,
        gpio32_status_source_target: RP1_ETHERNET_GPIO32_STATUS_SOURCE_TARGET,
        gpio32_status_observed_target: RP1_ETHERNET_GPIO32_STATUS_OBSERVED_TARGET,
        gpio32_ctrl_source_target: RP1_ETHERNET_GPIO32_CTRL_SOURCE_TARGET,
        gpio32_ctrl_observed_target: RP1_ETHERNET_GPIO32_CTRL_OBSERVED_TARGET,
        rio1_out_source_target: RP1_ETHERNET_GPIO32_RIO1_OUT_SOURCE_TARGET,
        rio1_out_observed_target: RP1_ETHERNET_GPIO32_RIO1_OUT_OBSERVED_TARGET,
        rio1_oe_source_target: RP1_ETHERNET_GPIO32_RIO1_OE_SOURCE_TARGET,
        rio1_oe_observed_target: RP1_ETHERNET_GPIO32_RIO1_OE_OBSERVED_TARGET,
        rio1_in_source_target: RP1_ETHERNET_GPIO32_RIO1_IN_SOURCE_TARGET,
        rio1_in_observed_target: RP1_ETHERNET_GPIO32_RIO1_IN_OBSERVED_TARGET,
        gpio32_pad_source_target: RP1_ETHERNET_GPIO32_PAD_SOURCE_TARGET,
        gpio32_pad_observed_target: RP1_ETHERNET_GPIO32_PAD_OBSERVED_TARGET,
        width_bits: RP1_ETHERNET_GPIO32_WRITE_RESTORE_WIDTH_BITS,
        access: RP1_ETHERNET_GPIO32_WRITE_RESTORE_ACCESS,
        active_low: RP1_ETHERNET_PHY_RESET_ACTIVE_LOW,
        assertion_raw_output: RP1_ETHERNET_GPIO32_ASSERTION_RAW_OUTPUT,
        deassertion_raw_output: RP1_ETHERNET_GPIO32_DEASSERTION_RAW_OUTPUT,
        reset_duration_ms: RP1_ETHERNET_PHY_RESET_DURATION_MS,
        no_write_preconditions: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_PRECONDITIONS,
        restore_baseline_fields: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BASELINE_FIELDS,
        operation_sequence: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_OPERATION_SEQUENCE,
        blocked_no_write_classifications:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BLOCKED_CLASSIFICATIONS,
        future_proof_classifications:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_EVIDENCE,
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

pub fn build_rp1_ethernet_prereq_ownership_report(
    input: Rp1EthernetPrereqOwnershipReportInput,
) -> Result<Rp1EthernetPrereqOwnershipReport, Rp1EthernetPrereqOwnershipReportError> {
    validate_rp1_ethernet_prereq_ownership_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetPrereqOwnershipReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_prereq_ownership_source_contract(source_contract)?;
            Ok(Rp1EthernetPrereqOwnershipReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetPrereqOwnershipReportKind::Candidate, None) => {
            Err(Rp1EthernetPrereqOwnershipReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetPrereqOwnershipReportKind::NoOwnershipNoEthernetControl, None) => {
            Ok(Rp1EthernetPrereqOwnershipReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetPrereqOwnershipReportKind::NoOwnershipNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetPrereqOwnershipReportError::ControlCarriesEthernetPrereqFacts)
        }
    }
}

pub fn rp1_ethernet_prereq_ownership_report_evidence(
    report: Rp1EthernetPrereqOwnershipReport,
) -> Rp1EthernetPrereqOwnershipReportEvidence {
    match report.source_contract {
        Some(source_contract) => {
            rp1_ethernet_prereq_ownership_candidate_evidence(report.kind.name(), source_contract)
        }
        None => rp1_ethernet_prereq_ownership_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_prereq_ownership_report_evidence(
    error: Rp1EthernetPrereqOwnershipReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_rp1_ethernet_clock_reset_guard_report(
    input: Rp1EthernetClockResetGuardReportInput,
) -> Result<Rp1EthernetClockResetGuardReport, Rp1EthernetClockResetGuardReportError> {
    validate_rp1_ethernet_clock_reset_guard_rejected_claims(input)?;

    match (input.kind, input.guard_contract) {
        (Rp1EthernetClockResetGuardReportKind::Candidate, Some(guard_contract)) => {
            validate_rp1_ethernet_clock_reset_guard_contract(guard_contract)?;
            Ok(Rp1EthernetClockResetGuardReport {
                kind: input.kind,
                guard_contract: Some(guard_contract),
            })
        }
        (Rp1EthernetClockResetGuardReportKind::Candidate, None) => {
            Err(Rp1EthernetClockResetGuardReportError::CandidateMissingGuardContract)
        }
        (Rp1EthernetClockResetGuardReportKind::NoClockResetNoEthernetControl, None) => {
            Ok(Rp1EthernetClockResetGuardReport {
                kind: input.kind,
                guard_contract: None,
            })
        }
        (Rp1EthernetClockResetGuardReportKind::NoClockResetNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetClockResetGuardReportError::ControlCarriesClockResetFacts)
        }
    }
}

pub fn rp1_ethernet_clock_reset_guard_report_evidence(
    report: Rp1EthernetClockResetGuardReport,
) -> Rp1EthernetClockResetGuardReportEvidence {
    match report.guard_contract {
        Some(guard_contract) => {
            rp1_ethernet_clock_reset_guard_candidate_evidence(report.kind.name(), guard_contract)
        }
        None => rp1_ethernet_clock_reset_guard_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_clock_reset_guard_report_evidence(
    error: Rp1EthernetClockResetGuardReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_rp1_ethernet_clock_reset_write_restore_report(
    input: Rp1EthernetClockResetWriteRestoreReportInput,
) -> Result<Rp1EthernetClockResetWriteRestoreReport, Rp1EthernetClockResetWriteRestoreReportError> {
    validate_rp1_ethernet_clock_reset_write_restore_rejected_claims(input)?;

    match (input.kind, input.target_contract) {
        (Rp1EthernetClockResetWriteRestoreReportKind::Candidate, Some(target_contract)) => {
            validate_rp1_ethernet_clock_reset_write_target_contract(target_contract)?;
            Ok(Rp1EthernetClockResetWriteRestoreReport {
                kind: input.kind,
                target_contract: Some(target_contract),
            })
        }
        (Rp1EthernetClockResetWriteRestoreReportKind::Candidate, None) => {
            Err(Rp1EthernetClockResetWriteRestoreReportError::CandidateMissingTargetContract)
        }
        (Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl, None) => {
            Ok(Rp1EthernetClockResetWriteRestoreReport {
                kind: input.kind,
                target_contract: None,
            })
        }
        (Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetClockResetWriteRestoreReportError::ControlCarriesWriteTargetFacts)
        }
    }
}

pub fn rp1_ethernet_clock_reset_write_restore_report_evidence(
    report: Rp1EthernetClockResetWriteRestoreReport,
) -> Rp1EthernetClockResetWriteRestoreReportEvidence {
    match report.target_contract {
        Some(target_contract) => rp1_ethernet_clock_reset_write_restore_candidate_evidence(
            report.kind.name(),
            target_contract,
        ),
        None => rp1_ethernet_clock_reset_write_restore_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_clock_reset_write_restore_report_evidence(
    error: Rp1EthernetClockResetWriteRestoreReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
    input: Rp1EthernetClkEthCtrlWriteRestoreReportInput,
) -> Result<Rp1EthernetClkEthCtrlWriteRestoreReport, Rp1EthernetClkEthCtrlWriteRestoreReportError> {
    validate_rp1_ethernet_clk_eth_ctrl_write_restore_rejected_claims(input)?;

    match (input.kind, input.target_contract) {
        (Rp1EthernetClockResetWriteRestoreReportKind::Candidate, Some(target_contract)) => {
            validate_rp1_ethernet_clk_eth_ctrl_write_target_contract(target_contract)?;
            Ok(Rp1EthernetClkEthCtrlWriteRestoreReport {
                kind: input.kind,
                target_contract: Some(target_contract),
            })
        }
        (Rp1EthernetClockResetWriteRestoreReportKind::Candidate, None) => {
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::CandidateMissingTargetContract)
        }
        (Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl, None) => {
            Ok(Rp1EthernetClkEthCtrlWriteRestoreReport {
                kind: input.kind,
                target_contract: None,
            })
        }
        (Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::ControlCarriesWriteTargetFacts)
        }
    }
}

pub fn rp1_ethernet_clk_eth_ctrl_write_restore_report_evidence(
    report: Rp1EthernetClkEthCtrlWriteRestoreReport,
) -> Rp1EthernetClkEthCtrlWriteRestoreReportEvidence {
    match report.target_contract {
        Some(target_contract) => rp1_ethernet_clk_eth_ctrl_write_restore_candidate_evidence(
            report.kind.name(),
            target_contract,
        ),
        None => rp1_ethernet_clk_eth_ctrl_write_restore_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_clk_eth_ctrl_write_restore_report_evidence(
    error: Rp1EthernetClkEthCtrlWriteRestoreReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_rp1_ethernet_gpio32_phy_reset_preflight_report(
    input: Rp1EthernetGpio32PhyResetPreflightReportInput,
) -> Result<Rp1EthernetGpio32PhyResetPreflightReport, Rp1EthernetGpio32PhyResetPreflightReportError>
{
    validate_rp1_ethernet_gpio32_phy_reset_preflight_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetGpio32PhyResetPreflightReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_gpio32_phy_reset_source_contract(source_contract)?;
            Ok(Rp1EthernetGpio32PhyResetPreflightReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetGpio32PhyResetPreflightReportKind::Candidate, None) => {
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetGpio32PhyResetPreflightReportKind::NoGpioNoEthernetControl, None) => {
            Ok(Rp1EthernetGpio32PhyResetPreflightReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetGpio32PhyResetPreflightReportKind::NoGpioNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::ControlCarriesGpioPhyResetFacts)
        }
    }
}

pub fn rp1_ethernet_gpio32_phy_reset_preflight_report_evidence(
    report: Rp1EthernetGpio32PhyResetPreflightReport,
) -> Rp1EthernetGpio32PhyResetPreflightReportEvidence {
    match report.source_contract {
        Some(source_contract) => rp1_ethernet_gpio32_phy_reset_preflight_candidate_evidence(
            report.kind.name(),
            source_contract,
        ),
        None => rp1_ethernet_gpio32_phy_reset_preflight_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_gpio32_phy_reset_preflight_report_evidence(
    error: Rp1EthernetGpio32PhyResetPreflightReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

pub fn build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
    input: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput,
) -> Result<
    Rp1EthernetGpio32PhyResetWriteRestoreGuardReport,
    Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError,
> {
    validate_rp1_ethernet_gpio32_phy_reset_write_restore_rejected_claims(input)?;

    match (
        input.kind,
        input.guard_contract,
        input.blocked_no_write_classification,
    ) {
        (
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::Candidate,
            Some(guard_contract),
            None,
        ) => {
            validate_rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract(guard_contract)?;
            Ok(Rp1EthernetGpio32PhyResetWriteRestoreGuardReport {
                kind: input.kind,
                guard_contract: Some(guard_contract),
                blocked_no_write_classification: None,
            })
        }
        (Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::Candidate, None, _) => Err(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::CandidateMissingGuardContract,
        ),
        (Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::Candidate, Some(_), Some(_)) => Err(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::BlockedClassificationNotAllowed,
        ),
        (
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::BlockedNoWrite,
            Some(guard_contract),
            Some(blocked_no_write_classification),
        ) => {
            validate_rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract(guard_contract)?;
            validate_rp1_ethernet_gpio32_phy_reset_write_restore_blocked_classification(
                blocked_no_write_classification,
            )?;
            Ok(Rp1EthernetGpio32PhyResetWriteRestoreGuardReport {
                kind: input.kind,
                guard_contract: Some(guard_contract),
                blocked_no_write_classification: Some(blocked_no_write_classification),
            })
        }
        (Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::BlockedNoWrite, None, _) => {
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::BlockedMissingGuardContract)
        }
        (Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::BlockedNoWrite, Some(_), None) => {
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::BlockedMissingClassification)
        }
        (
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::NoGpioWriteNoEthernetControl,
            None,
            None,
        ) => Ok(Rp1EthernetGpio32PhyResetWriteRestoreGuardReport {
            kind: input.kind,
            guard_contract: None,
            blocked_no_write_classification: None,
        }),
        (
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::NoGpioWriteNoEthernetControl,
            _,
            _,
        ) => {
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::ControlCarriesGpioWriteFacts)
        }
    }
}

pub fn rp1_ethernet_gpio32_phy_reset_write_restore_guard_report_evidence(
    report: Rp1EthernetGpio32PhyResetWriteRestoreGuardReport,
) -> Rp1EthernetGpio32PhyResetWriteRestoreGuardReportEvidence {
    match report.guard_contract {
        Some(guard_contract) => {
            rp1_ethernet_gpio32_phy_reset_write_restore_guard_candidate_evidence(
                report.kind.name(),
                guard_contract,
                report.blocked_no_write_classification,
            )
        }
        None => {
            rp1_ethernet_gpio32_phy_reset_write_restore_guard_control_evidence(report.kind.name())
        }
    }
}

pub fn rejected_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report_evidence(
    error: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError,
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

fn validate_rp1_ethernet_prereq_ownership_rejected_claims(
    input: Rp1EthernetPrereqOwnershipReportInput,
) -> Result<(), Rp1EthernetPrereqOwnershipReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetPrereqOwnershipReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetPrereqOwnershipReportError::BroadMmioReadinessClaim);
    }
    if input.claims_rp1_mmio_writes {
        return Err(Rp1EthernetPrereqOwnershipReportError::Rp1MmioWritesClaim);
    }
    if input.claims_clock_reset_ownership {
        return Err(Rp1EthernetPrereqOwnershipReportError::ClockResetOwnershipClaim);
    }
    if input.claims_gpio32_phy_reset_ownership {
        return Err(Rp1EthernetPrereqOwnershipReportError::Gpio32PhyResetOwnershipClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetPrereqOwnershipReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_interrupt_ownership {
        return Err(Rp1EthernetPrereqOwnershipReportError::InterruptOwnershipClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetPrereqOwnershipReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetPrereqOwnershipReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetPrereqOwnershipReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetPrereqOwnershipReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetPrereqOwnershipReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetPrereqOwnershipReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetPrereqOwnershipReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_clock_reset_guard_rejected_claims(
    input: Rp1EthernetClockResetGuardReportInput,
) -> Result<(), Rp1EthernetClockResetGuardReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetClockResetGuardReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetClockResetGuardReportError::BroadMmioReadinessClaim);
    }
    if input.claims_rp1_mmio_writes {
        return Err(Rp1EthernetClockResetGuardReportError::Rp1MmioWritesClaim);
    }
    if input.claims_clock_reset_writes {
        return Err(Rp1EthernetClockResetGuardReportError::ClockResetWritesClaim);
    }
    if input.claims_clock_reset_ownership {
        return Err(Rp1EthernetClockResetGuardReportError::ClockResetOwnershipClaim);
    }
    if input.claims_rp1_clk_sys_transition {
        return Err(Rp1EthernetClockResetGuardReportError::Rp1ClkSysTransitionClaim);
    }
    if input.claims_reset_controller_ownership {
        return Err(Rp1EthernetClockResetGuardReportError::ResetControllerOwnershipClaim);
    }
    if input.claims_gpio32_phy_reset_ownership {
        return Err(Rp1EthernetClockResetGuardReportError::Gpio32PhyResetOwnershipClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetClockResetGuardReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_interrupt_ownership {
        return Err(Rp1EthernetClockResetGuardReportError::InterruptOwnershipClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetClockResetGuardReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetClockResetGuardReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetClockResetGuardReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetClockResetGuardReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetClockResetGuardReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetClockResetGuardReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetClockResetGuardReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_clock_reset_write_restore_rejected_claims(
    input: Rp1EthernetClockResetWriteRestoreReportInput,
) -> Result<(), Rp1EthernetClockResetWriteRestoreReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::BroadMmioReadinessClaim);
    }
    if input.claims_unscoped_rp1_mmio_writes {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::UnscopedRp1MmioWritesClaim);
    }
    if input.claims_rp1_clk_sys_transition {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::Rp1ClkSysTransitionClaim);
    }
    if input.claims_clk_eth_ctrl_write {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::ClkEthCtrlWriteClaim);
    }
    if input.claims_reset_controller_ownership {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::ResetControllerOwnershipClaim);
    }
    if input.claims_gpio32_phy_reset_ownership {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::Gpio32PhyResetOwnershipClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_interrupt_ownership {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::InterruptOwnershipClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_clk_eth_ctrl_write_restore_rejected_claims(
    input: Rp1EthernetClkEthCtrlWriteRestoreReportInput,
) -> Result<(), Rp1EthernetClkEthCtrlWriteRestoreReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::BroadMmioReadinessClaim);
    }
    if input.claims_unscoped_rp1_mmio_writes {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::UnscopedRp1MmioWritesClaim);
    }
    if input.claims_shared_clock_write {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::SharedClockWriteClaim);
    }
    if input.claims_tsu_same_shape_retry {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::TsuSameShapeRetryClaim);
    }
    if input.claims_non_idempotent_transition {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::NonIdempotentTransitionClaim);
    }
    if input.claims_reset_controller_ownership {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::ResetControllerOwnershipClaim);
    }
    if input.claims_gpio32_phy_reset_ownership {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::Gpio32PhyResetOwnershipClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_interrupt_ownership {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::InterruptOwnershipClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_phy_reset_preflight_rejected_claims(
    input: Rp1EthernetGpio32PhyResetPreflightReportInput,
) -> Result<(), Rp1EthernetGpio32PhyResetPreflightReportError> {
    if input.claims_gpio_ownership {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::GpioOwnershipClaim);
    }
    if input.claims_phy_reset_assertion {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::PhyResetAssertionClaim);
    }
    if input.claims_phy_reset_deassertion {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::PhyResetDeassertionClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_runtime_writes {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::RuntimeWritesClaim);
    }
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::BroadMmioReadinessClaim);
    }
    if input.claims_interrupt_ownership {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::InterruptOwnershipClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_phy_reset_write_restore_rejected_claims(
    input: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput,
) -> Result<(), Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError> {
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::BroadMmioReadinessClaim);
    }
    if input.claims_non_gpio32_write {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::NonGpio32WriteClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_interrupt_ownership {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::InterruptOwnershipClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::DmaDescriptorOwnershipClaim,
        );
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::PhaseTransitionClaim);
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

fn validate_rp1_ethernet_prereq_ownership_source_contract(
    evidence: Rp1EthernetPrereqOwnershipSourceContractEvidence,
) -> Result<(), Rp1EthernetPrereqOwnershipReportError> {
    if evidence.contract_id != RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID
        || evidence.source_task_id != RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_TASK_ID
        || evidence.selected_prerequisite != RP1_ETHERNET_SELECTED_PREREQUISITE
        || evidence.controller != RP1_ETHERNET_CONTROLLER_NAME
        || evidence.compatible != RP1_ETHERNET_COMPATIBLE
    {
        return Err(Rp1EthernetPrereqOwnershipReportError::SourceContractIdentityMismatch);
    }
    if evidence.rp1_bus_base != RP1_ETHERNET_RP1_BUS_BASE
        || evidence.rp1_bus_window_size != RP1_ETHERNET_RP1_BUS_WINDOW_SIZE
        || evidence.observed_identity_target
            != RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET
        || evidence.translated_comparator_target != RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET
    {
        return Err(Rp1EthernetPrereqOwnershipReportError::SourceContractTargetMismatch);
    }
    if evidence.accepted_macb_mid_raw != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW
        || evidence.accepted_macb_mid_idnum != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_IDNUM
        || evidence.accepted_macb_mid_rev != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_REV
        || evidence.identity_role != RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE
        || evidence.interrupt_name != RP1_ETHERNET_INTERRUPT_NAME
        || evidence.interrupt_number != RP1_ETHERNET_INTERRUPT_NUMBER
        || evidence.clock_names != RP1_ETHERNET_CLOCK_NAMES
        || evidence.clock_sources != RP1_ETHERNET_CLOCK_SOURCES
        || evidence.clock_ids != RP1_ETHERNET_CLOCK_IDS
        || evidence.clock_policy_classification != RP1_ETHERNET_CLOCK_POLICY_CLASSIFICATION
        || evidence.phy_mode != RP1_ETHERNET_PHY_MODE
        || evidence.phy_handle != RP1_ETHERNET_PHY_HANDLE
        || evidence.phy_node != RP1_ETHERNET_PHY_NODE
        || evidence.phy_reg != RP1_ETHERNET_PHY_REG
        || evidence.phy_reset_gpio != RP1_ETHERNET_PHY_RESET_GPIO
        || evidence.phy_reset_active_low != RP1_ETHERNET_PHY_RESET_ACTIVE_LOW
        || evidence.phy_reset_duration_ms != RP1_ETHERNET_PHY_RESET_DURATION_MS
        || evidence.phy_mdio_policy_classification != RP1_ETHERNET_PHY_MDIO_POLICY_CLASSIFICATION
        || evidence.dma_descriptor_policy_classification
            != RP1_ETHERNET_DMA_DESCRIPTOR_POLICY_CLASSIFICATION
        || evidence.cadence_rp1_config != RP1_ETHERNET_CADENCE_RP1_CONFIG
    {
        return Err(Rp1EthernetPrereqOwnershipReportError::SourceContractFieldMismatch);
    }
    if evidence.source_evidence != RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE {
        return Err(Rp1EthernetPrereqOwnershipReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn validate_rp1_ethernet_clock_reset_guard_contract(
    evidence: Rp1EthernetClockResetGuardContractEvidence,
) -> Result<(), Rp1EthernetClockResetGuardReportError> {
    if evidence.guard_contract_id != RP1_ETHERNET_CLOCK_RESET_GUARD_CONTRACT_ID
        || evidence.ownership_contract_task_id
            != RP1_ETHERNET_CLOCK_RESET_OWNERSHIP_CONTRACT_TASK_ID
        || evidence.prereq_contract_id != RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID
    {
        return Err(Rp1EthernetClockResetGuardReportError::GuardContractIdentityMismatch);
    }
    if evidence.observed_identity_target != RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET
        || evidence.translated_comparator_target != RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET
    {
        return Err(Rp1EthernetClockResetGuardReportError::GuardContractTargetMismatch);
    }
    if evidence.accepted_macb_mid_raw != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW
        || evidence.accepted_macb_mid_idnum != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_IDNUM
        || evidence.accepted_macb_mid_rev != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_REV
        || evidence.identity_role != RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE
        || evidence.clock_names != RP1_ETHERNET_CLOCK_NAMES
        || evidence.clock_sources != RP1_ETHERNET_CLOCK_SOURCES
        || evidence.clock_ids != RP1_ETHERNET_CLOCK_IDS
        || evidence.shared_clock_names != RP1_ETHERNET_SHARED_CLOCK_NAMES
        || evidence.shared_clock_source != RP1_ETHERNET_SHARED_CLOCK_SOURCE
        || evidence.shared_clock_id != RP1_ETHERNET_SHARED_CLOCK_ID
        || evidence.ethernet_private_clock_names != RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_NAMES
        || evidence.ethernet_private_clock_sources != RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_SOURCES
        || evidence.ethernet_private_clock_ids != RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_IDS
        || evidence.clock_policy_classification != RP1_ETHERNET_CLOCK_POLICY_CLASSIFICATION
        || evidence.reset_controller_policy_classification
            != RP1_ETHERNET_RESET_CONTROLLER_POLICY_CLASSIFICATION
        || evidence.phy_reset_gpio != RP1_ETHERNET_PHY_RESET_GPIO
        || evidence.phy_mdio_policy_classification != RP1_ETHERNET_PHY_MDIO_POLICY_CLASSIFICATION
        || evidence.read_only_baseline_requirements
            != RP1_ETHERNET_CLOCK_RESET_GUARD_READ_ONLY_BASELINE_REQUIREMENTS
        || evidence.write_backed_invariants
            != RP1_ETHERNET_CLOCK_RESET_GUARD_WRITE_BACKED_INVARIANTS
    {
        return Err(Rp1EthernetClockResetGuardReportError::GuardContractFieldMismatch);
    }
    if evidence.source_evidence != RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE {
        return Err(Rp1EthernetClockResetGuardReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn validate_rp1_ethernet_clock_reset_write_target_contract(
    evidence: Rp1EthernetClockResetWriteTargetContractEvidence,
) -> Result<(), Rp1EthernetClockResetWriteRestoreReportError> {
    if evidence.contract_id != RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_CONTRACT_ID
        || evidence.source_task_id != RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_SOURCE_TASK_ID
        || evidence.target != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_TARGET
    {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::TargetContractIdentityMismatch);
    }
    if evidence.clock_name != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CLOCK_NAME
        || evidence.clock_id != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CLOCK_ID
        || evidence.register != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REGISTER
        || evidence.source_block != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SOURCE_BLOCK
        || evidence.observed_rp1_base != RP1_ETHERNET_OBSERVED_RP1_BASE
        || evidence.source_offset != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SOURCE_OFFSET
        || evidence.cpu_physical_target
            != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CPU_PHYSICAL_TARGET
    {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::TargetContractTargetMismatch);
    }
    if evidence.width_bits != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_WIDTH_BITS
        || evidence.access != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_ACCESS
        || evidence.allowed_write_value
            != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_ALLOWED_WRITE_VALUE
        || evidence.preserved_fields != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_PRESERVED_FIELDS
        || evidence.operation_sequence != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_OPERATION_SEQUENCE
        || evidence.safety_invariants != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_SAFETY_INVARIANTS
        || evidence.future_proof_classifications
            != RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS
    {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::TargetContractFieldMismatch);
    }
    if evidence.source_evidence != RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE {
        return Err(Rp1EthernetClockResetWriteRestoreReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn validate_rp1_ethernet_clk_eth_ctrl_write_target_contract(
    evidence: Rp1EthernetClockResetWriteTargetContractEvidence,
) -> Result<(), Rp1EthernetClkEthCtrlWriteRestoreReportError> {
    if evidence.contract_id != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_CONTRACT_ID
        || evidence.source_task_id != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_SOURCE_TASK_ID
        || evidence.target != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_TARGET
    {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::TargetContractIdentityMismatch);
    }
    if evidence.clock_name != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CLOCK_NAME
        || evidence.clock_id != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CLOCK_ID
        || evidence.register != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REGISTER
        || evidence.source_block != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_BLOCK
        || evidence.observed_rp1_base != RP1_ETHERNET_OBSERVED_RP1_BASE
        || evidence.source_offset != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_OFFSET
        || evidence.cpu_physical_target
            != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CPU_PHYSICAL_TARGET
    {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::TargetContractTargetMismatch);
    }
    if evidence.width_bits != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_WIDTH_BITS
        || evidence.access != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_ACCESS
        || evidence.allowed_write_value
            != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_ALLOWED_WRITE_VALUE
        || evidence.preserved_fields != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_PRESERVED_FIELDS
        || evidence.operation_sequence != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_OPERATION_SEQUENCE
        || evidence.safety_invariants != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SAFETY_INVARIANTS
        || evidence.future_proof_classifications
            != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_FUTURE_CLASSIFICATIONS
    {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::TargetContractFieldMismatch);
    }
    if evidence.source_evidence != RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_SOURCE_EVIDENCE {
        return Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_phy_reset_source_contract(
    evidence: Rp1EthernetGpio32PhyResetSourceContractEvidence,
) -> Result<(), Rp1EthernetGpio32PhyResetPreflightReportError> {
    if evidence.contract_id != RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID
        || evidence.source_task_id != RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID
        || evidence.accepted_input_frontier != RP1_ETHERNET_GPIO32_PHY_RESET_ACCEPTED_INPUT_FRONTIER
        || evidence.controller != RP1_ETHERNET_CONTROLLER_NAME
        || evidence.compatible != RP1_ETHERNET_COMPATIBLE
    {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::SourceContractIdentityMismatch);
    }
    if evidence.accepted_macb_mid_raw != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW
        || evidence.accepted_macb_mid_idnum != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_IDNUM
        || evidence.accepted_macb_mid_rev != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_REV
        || evidence.identity_role != RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE
        || evidence.phy_mode != RP1_ETHERNET_PHY_MODE
        || evidence.phy_handle != RP1_ETHERNET_PHY_HANDLE
        || evidence.phy_node != RP1_ETHERNET_PHY_NODE
        || evidence.phy_reg != RP1_ETHERNET_PHY_REG
        || evidence.gpio_controller != RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER
        || evidence.gpio_line != RP1_ETHERNET_PHY_RESET_GPIO
        || evidence.reset_route != RP1_ETHERNET_PHY_RESET_ROUTE
        || evidence.active_low != RP1_ETHERNET_PHY_RESET_ACTIVE_LOW
        || evidence.logical_assertion != RP1_ETHERNET_PHY_RESET_LOGICAL_ASSERTION
        || evidence.logical_deassertion != RP1_ETHERNET_PHY_RESET_LOGICAL_DEASSERTION
        || evidence.reset_duration_ms != RP1_ETHERNET_PHY_RESET_DURATION_MS
        || evidence.mdio_reset_hook_relationship != RP1_ETHERNET_PHY_RESET_MDIO_HOOK_RELATIONSHIP
        || evidence.phase11_gpio_constraints
            != RP1_ETHERNET_GPIO32_PHY_RESET_PHASE11_GPIO_CONSTRAINTS
        || evidence.future_write_restore_invariants
            != RP1_ETHERNET_GPIO32_PHY_RESET_FUTURE_WRITE_RESTORE_INVARIANTS
    {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::SourceContractFieldMismatch);
    }
    if evidence.source_evidence != RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_EVIDENCE {
        return Err(Rp1EthernetGpio32PhyResetPreflightReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract(
    evidence: Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence,
) -> Result<(), Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError> {
    if evidence.guard_contract_id != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID
        || evidence.source_contract_id != RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID
        || evidence.source_task_id != RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID
        || evidence.report_source_task_id
            != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_TASK_ID
        || evidence.gpio_controller != RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER
        || evidence.gpio_line != RP1_ETHERNET_PHY_RESET_GPIO
        || evidence.reset_route != RP1_ETHERNET_PHY_RESET_ROUTE
    {
        return Err(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::GuardContractIdentityMismatch,
        );
    }
    if evidence.bank != RP1_ETHERNET_GPIO32_BANK
        || evidence.bank_local_bit != RP1_ETHERNET_GPIO32_BANK_LOCAL_BIT
        || evidence.io_bank1_source_base != RP1_ETHERNET_GPIO32_IO_BANK1_SOURCE_BASE
        || evidence.io_bank1_observed_base != RP1_ETHERNET_GPIO32_IO_BANK1_OBSERVED_BASE
        || evidence.gpio32_status_source_target != RP1_ETHERNET_GPIO32_STATUS_SOURCE_TARGET
        || evidence.gpio32_status_observed_target != RP1_ETHERNET_GPIO32_STATUS_OBSERVED_TARGET
        || evidence.gpio32_ctrl_source_target != RP1_ETHERNET_GPIO32_CTRL_SOURCE_TARGET
        || evidence.gpio32_ctrl_observed_target != RP1_ETHERNET_GPIO32_CTRL_OBSERVED_TARGET
        || evidence.rio1_out_source_target != RP1_ETHERNET_GPIO32_RIO1_OUT_SOURCE_TARGET
        || evidence.rio1_out_observed_target != RP1_ETHERNET_GPIO32_RIO1_OUT_OBSERVED_TARGET
        || evidence.rio1_oe_source_target != RP1_ETHERNET_GPIO32_RIO1_OE_SOURCE_TARGET
        || evidence.rio1_oe_observed_target != RP1_ETHERNET_GPIO32_RIO1_OE_OBSERVED_TARGET
        || evidence.rio1_in_source_target != RP1_ETHERNET_GPIO32_RIO1_IN_SOURCE_TARGET
        || evidence.rio1_in_observed_target != RP1_ETHERNET_GPIO32_RIO1_IN_OBSERVED_TARGET
        || evidence.gpio32_pad_source_target != RP1_ETHERNET_GPIO32_PAD_SOURCE_TARGET
        || evidence.gpio32_pad_observed_target != RP1_ETHERNET_GPIO32_PAD_OBSERVED_TARGET
    {
        return Err(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::GuardContractTargetMismatch,
        );
    }
    if evidence.width_bits != RP1_ETHERNET_GPIO32_WRITE_RESTORE_WIDTH_BITS
        || evidence.access != RP1_ETHERNET_GPIO32_WRITE_RESTORE_ACCESS
        || evidence.active_low != RP1_ETHERNET_PHY_RESET_ACTIVE_LOW
        || evidence.assertion_raw_output != RP1_ETHERNET_GPIO32_ASSERTION_RAW_OUTPUT
        || evidence.deassertion_raw_output != RP1_ETHERNET_GPIO32_DEASSERTION_RAW_OUTPUT
        || evidence.reset_duration_ms != RP1_ETHERNET_PHY_RESET_DURATION_MS
        || evidence.no_write_preconditions
            != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_PRECONDITIONS
        || evidence.operation_sequence
            != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_OPERATION_SEQUENCE
        || evidence.blocked_no_write_classifications
            != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BLOCKED_CLASSIFICATIONS
        || evidence.future_proof_classifications
            != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS
    {
        return Err(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::GuardContractFieldMismatch,
        );
    }
    if evidence.restore_baseline_fields
        != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BASELINE_FIELDS
    {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::MissingRestoreBaseline);
    }
    if evidence.source_evidence != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_EVIDENCE {
        return Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_phy_reset_write_restore_blocked_classification(
    classification: &'static str,
) -> Result<(), Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError> {
    if RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BLOCKED_CLASSIFICATIONS
        .iter()
        .any(|allowed| *allowed == classification)
    {
        Ok(())
    } else {
        Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::BlockedClassificationNotAllowed)
    }
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

fn rp1_ethernet_prereq_ownership_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetPrereqOwnershipSourceContractEvidence,
) -> Rp1EthernetPrereqOwnershipReportEvidence {
    Rp1EthernetPrereqOwnershipReportEvidence {
        contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        report_kind,
        selected_prerequisite: Some(source_contract.selected_prerequisite),
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        rp1_bus_base: Some(source_contract.rp1_bus_base),
        rp1_bus_window_size: Some(source_contract.rp1_bus_window_size),
        observed_identity_target: Some(source_contract.observed_identity_target),
        translated_comparator_target: Some(source_contract.translated_comparator_target),
        accepted_macb_mid_raw: Some(source_contract.accepted_macb_mid_raw),
        accepted_macb_mid_idnum: Some(source_contract.accepted_macb_mid_idnum),
        accepted_macb_mid_rev: Some(source_contract.accepted_macb_mid_rev),
        identity_role: Some(source_contract.identity_role),
        interrupt_name: Some(source_contract.interrupt_name),
        interrupt_number: Some(source_contract.interrupt_number),
        clock_names: Some(source_contract.clock_names),
        clock_sources: Some(source_contract.clock_sources),
        clock_ids: Some(source_contract.clock_ids),
        clock_policy_classification: Some(source_contract.clock_policy_classification),
        phy_mode: Some(source_contract.phy_mode),
        phy_handle: Some(source_contract.phy_handle),
        phy_node: Some(source_contract.phy_node),
        phy_reg: Some(source_contract.phy_reg),
        phy_reset_gpio: Some(source_contract.phy_reset_gpio),
        phy_reset_active_low: Some(source_contract.phy_reset_active_low),
        phy_reset_duration_ms: Some(source_contract.phy_reset_duration_ms),
        phy_mdio_policy_classification: Some(source_contract.phy_mdio_policy_classification),
        dma_descriptor_policy_classification: Some(
            source_contract.dma_descriptor_policy_classification,
        ),
        cadence_rp1_config: Some(source_contract.cadence_rp1_config),
        source_evidence: Some(source_contract.source_evidence),
        hardware_proof_boundary_classification:
            RP1_ETHERNET_PREREQ_OWNERSHIP_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_PREREQ_OWNERSHIP_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_PREREQ_OWNERSHIP_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_rp1_mmio_writes: false,
        claims_clock_reset_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_PREREQ_OWNERSHIP_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_prereq_ownership_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetPrereqOwnershipReportEvidence {
    Rp1EthernetPrereqOwnershipReportEvidence {
        contract_id: RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_TASK_ID,
        report_kind,
        selected_prerequisite: None,
        controller: None,
        compatible: None,
        rp1_bus_base: None,
        rp1_bus_window_size: None,
        observed_identity_target: None,
        translated_comparator_target: None,
        accepted_macb_mid_raw: None,
        accepted_macb_mid_idnum: None,
        accepted_macb_mid_rev: None,
        identity_role: None,
        interrupt_name: None,
        interrupt_number: None,
        clock_names: None,
        clock_sources: None,
        clock_ids: None,
        clock_policy_classification: None,
        phy_mode: None,
        phy_handle: None,
        phy_node: None,
        phy_reg: None,
        phy_reset_gpio: None,
        phy_reset_active_low: None,
        phy_reset_duration_ms: None,
        phy_mdio_policy_classification: None,
        dma_descriptor_policy_classification: None,
        cadence_rp1_config: None,
        source_evidence: None,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_PREREQ_OWNERSHIP_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_PREREQ_OWNERSHIP_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_PREREQ_OWNERSHIP_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_rp1_mmio_writes: false,
        claims_clock_reset_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_PREREQ_OWNERSHIP_CONTROL_CLASSIFICATION,
    }
}

fn rp1_ethernet_clock_reset_guard_candidate_evidence(
    report_kind: &'static str,
    guard_contract: Rp1EthernetClockResetGuardContractEvidence,
) -> Rp1EthernetClockResetGuardReportEvidence {
    Rp1EthernetClockResetGuardReportEvidence {
        guard_contract_id: guard_contract.guard_contract_id,
        ownership_contract_task_id: guard_contract.ownership_contract_task_id,
        prereq_contract_id: guard_contract.prereq_contract_id,
        report_kind,
        observed_identity_target: Some(guard_contract.observed_identity_target),
        translated_comparator_target: Some(guard_contract.translated_comparator_target),
        accepted_macb_mid_raw: Some(guard_contract.accepted_macb_mid_raw),
        accepted_macb_mid_idnum: Some(guard_contract.accepted_macb_mid_idnum),
        accepted_macb_mid_rev: Some(guard_contract.accepted_macb_mid_rev),
        identity_role: Some(guard_contract.identity_role),
        clock_names: Some(guard_contract.clock_names),
        clock_sources: Some(guard_contract.clock_sources),
        clock_ids: Some(guard_contract.clock_ids),
        shared_clock_names: Some(guard_contract.shared_clock_names),
        shared_clock_source: Some(guard_contract.shared_clock_source),
        shared_clock_id: Some(guard_contract.shared_clock_id),
        ethernet_private_clock_names: Some(guard_contract.ethernet_private_clock_names),
        ethernet_private_clock_sources: Some(guard_contract.ethernet_private_clock_sources),
        ethernet_private_clock_ids: Some(guard_contract.ethernet_private_clock_ids),
        clock_policy_classification: Some(guard_contract.clock_policy_classification),
        reset_controller_policy_classification: Some(
            guard_contract.reset_controller_policy_classification,
        ),
        phy_reset_gpio: Some(guard_contract.phy_reset_gpio),
        phy_mdio_policy_classification: Some(guard_contract.phy_mdio_policy_classification),
        read_only_baseline_requirements: Some(guard_contract.read_only_baseline_requirements),
        write_backed_invariants: Some(guard_contract.write_backed_invariants),
        source_evidence: Some(guard_contract.source_evidence),
        boundary_classification: RP1_ETHERNET_CLOCK_RESET_GUARD_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_CLOCK_RESET_GUARD_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_CLOCK_RESET_GUARD_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_rp1_mmio_writes: false,
        claims_clock_reset_writes: false,
        claims_clock_reset_ownership: false,
        claims_rp1_clk_sys_transition: false,
        claims_reset_controller_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_CLOCK_RESET_GUARD_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_clock_reset_guard_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetClockResetGuardReportEvidence {
    Rp1EthernetClockResetGuardReportEvidence {
        guard_contract_id: RP1_ETHERNET_CLOCK_RESET_GUARD_CONTRACT_ID,
        ownership_contract_task_id: RP1_ETHERNET_CLOCK_RESET_OWNERSHIP_CONTRACT_TASK_ID,
        prereq_contract_id: RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID,
        report_kind,
        observed_identity_target: None,
        translated_comparator_target: None,
        accepted_macb_mid_raw: None,
        accepted_macb_mid_idnum: None,
        accepted_macb_mid_rev: None,
        identity_role: None,
        clock_names: None,
        clock_sources: None,
        clock_ids: None,
        shared_clock_names: None,
        shared_clock_source: None,
        shared_clock_id: None,
        ethernet_private_clock_names: None,
        ethernet_private_clock_sources: None,
        ethernet_private_clock_ids: None,
        clock_policy_classification: None,
        reset_controller_policy_classification: None,
        phy_reset_gpio: None,
        phy_mdio_policy_classification: None,
        read_only_baseline_requirements: None,
        write_backed_invariants: None,
        source_evidence: None,
        boundary_classification: RP1_ETHERNET_CLOCK_RESET_GUARD_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_CLOCK_RESET_GUARD_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_CLOCK_RESET_GUARD_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_rp1_mmio_writes: false,
        claims_clock_reset_writes: false,
        claims_clock_reset_ownership: false,
        claims_rp1_clk_sys_transition: false,
        claims_reset_controller_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_CLOCK_RESET_GUARD_CONTROL_CLASSIFICATION,
    }
}

fn rp1_ethernet_clock_reset_write_restore_candidate_evidence(
    report_kind: &'static str,
    target_contract: Rp1EthernetClockResetWriteTargetContractEvidence,
) -> Rp1EthernetClockResetWriteRestoreReportEvidence {
    Rp1EthernetClockResetWriteRestoreReportEvidence {
        report_contract_id: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID,
        target_contract_id: target_contract.contract_id,
        source_task_id: target_contract.source_task_id,
        report_kind,
        target: Some(target_contract.target),
        clock_name: Some(target_contract.clock_name),
        clock_id: Some(target_contract.clock_id),
        register: Some(target_contract.register),
        source_block: Some(target_contract.source_block),
        observed_rp1_base: Some(target_contract.observed_rp1_base),
        source_offset: Some(target_contract.source_offset),
        cpu_physical_target: Some(target_contract.cpu_physical_target),
        width_bits: Some(target_contract.width_bits),
        access: Some(target_contract.access),
        allowed_write_value: Some(target_contract.allowed_write_value),
        preserved_fields: Some(target_contract.preserved_fields),
        operation_sequence: Some(target_contract.operation_sequence),
        safety_invariants: Some(target_contract.safety_invariants),
        post_eq_pre_required: Some(true),
        restore_eq_pre_required: Some(true),
        future_proof_classifications: target_contract.future_proof_classifications,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_unscoped_rp1_mmio_writes: false,
        claims_rp1_clk_sys_transition: false,
        claims_clk_eth_ctrl_write: false,
        claims_reset_controller_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_clock_reset_write_restore_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetClockResetWriteRestoreReportEvidence {
    Rp1EthernetClockResetWriteRestoreReportEvidence {
        report_contract_id: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID,
        target_contract_id: RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_SOURCE_TASK_ID,
        report_kind,
        target: None,
        clock_name: None,
        clock_id: None,
        register: None,
        source_block: None,
        observed_rp1_base: None,
        source_offset: None,
        cpu_physical_target: None,
        width_bits: None,
        access: None,
        allowed_write_value: None,
        preserved_fields: None,
        operation_sequence: None,
        safety_invariants: None,
        post_eq_pre_required: None,
        restore_eq_pre_required: None,
        future_proof_classifications: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_unscoped_rp1_mmio_writes: false,
        claims_rp1_clk_sys_transition: false,
        claims_clk_eth_ctrl_write: false,
        claims_reset_controller_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CONTROL_CLASSIFICATION,
    }
}

fn rp1_ethernet_clk_eth_ctrl_write_restore_candidate_evidence(
    report_kind: &'static str,
    target_contract: Rp1EthernetClockResetWriteTargetContractEvidence,
) -> Rp1EthernetClkEthCtrlWriteRestoreReportEvidence {
    Rp1EthernetClkEthCtrlWriteRestoreReportEvidence {
        report_contract_id: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REPORT_CONTRACT_ID,
        target_contract_id: target_contract.contract_id,
        source_task_id: target_contract.source_task_id,
        report_kind,
        target: Some(target_contract.target),
        clock_name: Some(target_contract.clock_name),
        clock_id: Some(target_contract.clock_id),
        register: Some(target_contract.register),
        source_block: Some(target_contract.source_block),
        observed_rp1_base: Some(target_contract.observed_rp1_base),
        source_offset: Some(target_contract.source_offset),
        cpu_physical_target: Some(target_contract.cpu_physical_target),
        width_bits: Some(target_contract.width_bits),
        access: Some(target_contract.access),
        allowed_write_value: Some(target_contract.allowed_write_value),
        preserved_fields: Some(target_contract.preserved_fields),
        operation_sequence: Some(target_contract.operation_sequence),
        safety_invariants: Some(target_contract.safety_invariants),
        post_eq_pre_required: Some(true),
        restore_eq_pre_required: Some(true),
        future_proof_classifications: target_contract.future_proof_classifications,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_unscoped_rp1_mmio_writes: false,
        claims_shared_clock_write: false,
        claims_tsu_same_shape_retry: false,
        claims_non_idempotent_transition: false,
        claims_reset_controller_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_clk_eth_ctrl_write_restore_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetClkEthCtrlWriteRestoreReportEvidence {
    Rp1EthernetClkEthCtrlWriteRestoreReportEvidence {
        report_contract_id: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REPORT_CONTRACT_ID,
        target_contract_id: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_SOURCE_TASK_ID,
        report_kind,
        target: None,
        clock_name: None,
        clock_id: None,
        register: None,
        source_block: None,
        observed_rp1_base: None,
        source_offset: None,
        cpu_physical_target: None,
        width_bits: None,
        access: None,
        allowed_write_value: None,
        preserved_fields: None,
        operation_sequence: None,
        safety_invariants: None,
        post_eq_pre_required: None,
        restore_eq_pre_required: None,
        future_proof_classifications:
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_FUTURE_CLASSIFICATIONS,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_RETAINED_RISKS,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_unscoped_rp1_mmio_writes: false,
        claims_shared_clock_write: false,
        claims_tsu_same_shape_retry: false,
        claims_non_idempotent_transition: false,
        claims_reset_controller_ownership: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CONTROL_CLASSIFICATION,
    }
}

fn rp1_ethernet_gpio32_phy_reset_preflight_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetGpio32PhyResetSourceContractEvidence,
) -> Rp1EthernetGpio32PhyResetPreflightReportEvidence {
    Rp1EthernetGpio32PhyResetPreflightReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_REPORT_CONTRACT_ID,
        source_contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        report_kind,
        accepted_input_frontier: Some(source_contract.accepted_input_frontier),
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        accepted_macb_mid_raw: Some(source_contract.accepted_macb_mid_raw),
        accepted_macb_mid_idnum: Some(source_contract.accepted_macb_mid_idnum),
        accepted_macb_mid_rev: Some(source_contract.accepted_macb_mid_rev),
        identity_role: Some(source_contract.identity_role),
        phy_mode: Some(source_contract.phy_mode),
        phy_handle: Some(source_contract.phy_handle),
        phy_node: Some(source_contract.phy_node),
        phy_reg: Some(source_contract.phy_reg),
        gpio_controller: Some(source_contract.gpio_controller),
        gpio_line: Some(source_contract.gpio_line),
        reset_route: Some(source_contract.reset_route),
        active_low: Some(source_contract.active_low),
        logical_assertion: Some(source_contract.logical_assertion),
        logical_deassertion: Some(source_contract.logical_deassertion),
        reset_duration_ms: Some(source_contract.reset_duration_ms),
        mdio_reset_hook_relationship: Some(source_contract.mdio_reset_hook_relationship),
        phase11_gpio_constraints: Some(source_contract.phase11_gpio_constraints),
        future_write_restore_invariants: Some(source_contract.future_write_restore_invariants),
        source_evidence: Some(source_contract.source_evidence),
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GPIO32_PHY_RESET_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_PHY_RESET_RETAINED_RISKS,
        claims_gpio_ownership: false,
        claims_phy_reset_assertion: false,
        claims_phy_reset_deassertion: false,
        claims_mdio_phy_ownership: false,
        claims_runtime_writes: false,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_gpio32_phy_reset_preflight_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetGpio32PhyResetPreflightReportEvidence {
    Rp1EthernetGpio32PhyResetPreflightReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_REPORT_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID,
        report_kind,
        accepted_input_frontier: None,
        controller: None,
        compatible: None,
        accepted_macb_mid_raw: None,
        accepted_macb_mid_idnum: None,
        accepted_macb_mid_rev: None,
        identity_role: None,
        phy_mode: None,
        phy_handle: None,
        phy_node: None,
        phy_reg: None,
        gpio_controller: None,
        gpio_line: None,
        reset_route: None,
        active_low: None,
        logical_assertion: None,
        logical_deassertion: None,
        reset_duration_ms: None,
        mdio_reset_hook_relationship: None,
        phase11_gpio_constraints: None,
        future_write_restore_invariants: None,
        source_evidence: None,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GPIO32_PHY_RESET_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_PHY_RESET_RETAINED_RISKS,
        claims_gpio_ownership: false,
        claims_phy_reset_assertion: false,
        claims_phy_reset_deassertion: false,
        claims_mdio_phy_ownership: false,
        claims_runtime_writes: false,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_CONTROL_CLASSIFICATION,
    }
}

fn rp1_ethernet_gpio32_phy_reset_write_restore_guard_candidate_evidence(
    report_kind: &'static str,
    guard_contract: Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence,
    blocked_no_write_classification: Option<&'static str>,
) -> Rp1EthernetGpio32PhyResetWriteRestoreGuardReportEvidence {
    Rp1EthernetGpio32PhyResetWriteRestoreGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID,
        guard_contract_id: guard_contract.guard_contract_id,
        source_contract_id: guard_contract.source_contract_id,
        source_task_id: guard_contract.source_task_id,
        report_source_task_id: guard_contract.report_source_task_id,
        report_kind,
        gpio_controller: Some(guard_contract.gpio_controller),
        gpio_line: Some(guard_contract.gpio_line),
        reset_route: Some(guard_contract.reset_route),
        bank: Some(guard_contract.bank),
        bank_local_bit: Some(guard_contract.bank_local_bit),
        io_bank1_source_base: Some(guard_contract.io_bank1_source_base),
        io_bank1_observed_base: Some(guard_contract.io_bank1_observed_base),
        gpio32_status_source_target: Some(guard_contract.gpio32_status_source_target),
        gpio32_status_observed_target: Some(guard_contract.gpio32_status_observed_target),
        gpio32_ctrl_source_target: Some(guard_contract.gpio32_ctrl_source_target),
        gpio32_ctrl_observed_target: Some(guard_contract.gpio32_ctrl_observed_target),
        rio1_out_source_target: Some(guard_contract.rio1_out_source_target),
        rio1_out_observed_target: Some(guard_contract.rio1_out_observed_target),
        rio1_oe_source_target: Some(guard_contract.rio1_oe_source_target),
        rio1_oe_observed_target: Some(guard_contract.rio1_oe_observed_target),
        rio1_in_source_target: Some(guard_contract.rio1_in_source_target),
        rio1_in_observed_target: Some(guard_contract.rio1_in_observed_target),
        gpio32_pad_source_target: Some(guard_contract.gpio32_pad_source_target),
        gpio32_pad_observed_target: Some(guard_contract.gpio32_pad_observed_target),
        width_bits: Some(guard_contract.width_bits),
        access: Some(guard_contract.access),
        active_low: Some(guard_contract.active_low),
        assertion_raw_output: Some(guard_contract.assertion_raw_output),
        deassertion_raw_output: Some(guard_contract.deassertion_raw_output),
        reset_duration_ms: Some(guard_contract.reset_duration_ms),
        no_write_preconditions: Some(guard_contract.no_write_preconditions),
        restore_baseline_fields: Some(guard_contract.restore_baseline_fields),
        operation_sequence: Some(guard_contract.operation_sequence),
        blocked_no_write_classifications: guard_contract.blocked_no_write_classifications,
        future_proof_classifications: guard_contract.future_proof_classifications,
        blocked_no_write_classification,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_RETAINED_RISKS,
        source_evidence: Some(guard_contract.source_evidence),
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_non_gpio32_write: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: blocked_no_write_classification
            .unwrap_or(RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_CANDIDATE_CLASSIFICATION),
    }
}

fn rp1_ethernet_gpio32_phy_reset_write_restore_guard_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetGpio32PhyResetWriteRestoreGuardReportEvidence {
    Rp1EthernetGpio32PhyResetWriteRestoreGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID,
        guard_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID,
        report_source_task_id: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_TASK_ID,
        report_kind,
        gpio_controller: None,
        gpio_line: None,
        reset_route: None,
        bank: None,
        bank_local_bit: None,
        io_bank1_source_base: None,
        io_bank1_observed_base: None,
        gpio32_status_source_target: None,
        gpio32_status_observed_target: None,
        gpio32_ctrl_source_target: None,
        gpio32_ctrl_observed_target: None,
        rio1_out_source_target: None,
        rio1_out_observed_target: None,
        rio1_oe_source_target: None,
        rio1_oe_observed_target: None,
        rio1_in_source_target: None,
        rio1_in_observed_target: None,
        gpio32_pad_source_target: None,
        gpio32_pad_observed_target: None,
        width_bits: None,
        access: None,
        active_low: None,
        assertion_raw_output: None,
        deassertion_raw_output: None,
        reset_duration_ms: None,
        no_write_preconditions: None,
        restore_baseline_fields: None,
        operation_sequence: None,
        blocked_no_write_classifications:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BLOCKED_CLASSIFICATIONS,
        future_proof_classifications:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS,
        blocked_no_write_classification: None,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_RETAINED_RISKS,
        source_evidence: None,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_non_gpio32_write: false,
        claims_mdio_phy_ownership: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_CONTROL_CLASSIFICATION,
    }
}

pub const RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-event-state-source-contract-v1";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-gpio32-event-state-source-contract-20260611";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-event-state-readonly-discriminator-report-v1";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_PROOF_TASK_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof-20260610";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_CLOSEOUT_TASK_ID: &str =
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout-20260610";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_COMMIT: &str =
    "0127984a1938cf050e2a6757f9f116f78976cf5e";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_V2_BLOCKER_CLASSIFICATION: &str =
    "rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-gpio32-event-state-blocked-event-state";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_CONTROL_CLASSIFICATION: &str =
    "no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-gpio32-readonly-event-state-control-output";
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_MASK: u32 = 0x0ff0_0000;
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_STATUS_RAW: u32 = 0x0abe_3300;
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_CTRL_RAW: u32 = 0x0000_0085;
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OUT_RAW: u32 = 0x0000_0010;
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OE_RAW: u32 = 0x0000_0010;
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_IN_RAW: u32 = 0x0000_0012;
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_EVENT_BITS: u32 = 0x0ab0_0000;
pub const RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_WRITES_PERFORMED: bool = false;

pub const RP1_ETHERNET_GPIO32_STATUS_SOURCE_EVENT_BIT_NAMES: &[&str] = &[
    "bit20-raw-falling",
    "bit21-raw-rising",
    "bit22-raw-low",
    "bit23-raw-high",
    "bit24-filtered-falling",
    "bit25-filtered-rising",
    "bit26-filtered-low",
    "bit27-filtered-high",
];

pub const RP1_ETHERNET_GPIO32_EVENT_STATE_CLASSIFICATIONS: &[&str] = &[
    "rp1-ethernet-gpio32-event-state-clear-precondition",
    "rp1-ethernet-gpio32-event-state-blocked-event-state",
    "rp1-ethernet-gpio32-event-state-source-unresolved-event-state",
    "rp1-ethernet-gpio32-event-state-inconclusive-capture",
    "no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control",
    "staging/build-blocker",
];

pub const RP1_ETHERNET_GPIO32_EVENT_STATE_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "event clearing",
    "GPIO, RIO, pad, or MMIO writes",
    "GPIO32 ownership",
    "PHY reset assertion or deassertion",
    "GPIO32 write/restore retry or success",
    "MDIO transactions or PHY ownership",
    "Ethernet driver readiness",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_GPIO32_EVENT_STATE_RETAINED_RISKS: &[&str] = &[
    "The v2 write/restore proof remains blocked before any write by unexpected GPIO32 event bits",
    "Read-only event-state discrimination does not authorize event clearing or GPIO32 write/restore",
    "Retained RP1 source names STATUS event bits but does not prove stale, clearable, firmware-owned, harmless, or safe-to-ignore semantics",
    "A future Pi 5 proof still needs candidate/control identity, serial freshness, TFTP delta, final identity, and restore or no-restore evidence",
];

pub const RP1_ETHERNET_GPIO32_EVENT_STATE_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-source-contract.md",
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof.md",
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout.md",
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract.md",
    "tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-core.md",
    "tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32EventStateSourceDecodingStatus {
    SourceBackedBits20To27,
    SourceUnresolved,
    CaptureChainInconclusive,
}

impl Rp1EthernetGpio32EventStateSourceDecodingStatus {
    pub const fn name(self) -> &'static str {
        match self {
            Self::SourceBackedBits20To27 => "source-backed-bits-20-27",
            Self::SourceUnresolved => "source-unresolved",
            Self::CaptureChainInconclusive => "capture-chain-inconclusive",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventStateSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub write_restore_source_contract_id: &'static str,
    pub write_restore_guard_contract_id: &'static str,
    pub v2_proof_task_id: &'static str,
    pub v2_closeout_task_id: &'static str,
    pub v2_commit: &'static str,
    pub v2_classification: &'static str,
    pub v2_writes_performed: bool,
    pub gpio_controller: &'static str,
    pub gpio_line: u32,
    pub reset_route: &'static str,
    pub bank: &'static str,
    pub bank_local_bit: u32,
    pub active_low: bool,
    pub gpio32_status_observed_target: u64,
    pub gpio32_ctrl_observed_target: u64,
    pub rio1_out_observed_target: u64,
    pub rio1_oe_observed_target: u64,
    pub rio1_in_observed_target: u64,
    pub gpio32_pad_observed_target: u64,
    pub status_event_mask: u32,
    pub source_event_bit_names: &'static [&'static str],
    pub accepted_v2_status_raw: u32,
    pub accepted_v2_ctrl_raw: u32,
    pub accepted_v2_rio1_out_raw: u32,
    pub accepted_v2_rio1_oe_raw: u32,
    pub accepted_v2_rio1_in_raw: u32,
    pub accepted_v2_event_bits: u32,
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32EventStateDiscriminatorReportKind {
    Candidate,
    NoGpioNoEthernetControl,
}

impl Rp1EthernetGpio32EventStateDiscriminatorReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoGpioNoEthernetControl => "no-gpio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventStateDiscriminatorReportInput {
    pub kind: Rp1EthernetGpio32EventStateDiscriminatorReportKind,
    pub source_contract: Option<Rp1EthernetGpio32EventStateSourceContractEvidence>,
    pub status_raw: Option<u32>,
    pub ctrl_raw: Option<u32>,
    pub rio1_out_raw: Option<u32>,
    pub rio1_oe_raw: Option<u32>,
    pub rio1_in_raw: Option<u32>,
    pub pad_raw: Option<u32>,
    pub source_decoding_status: Rp1EthernetGpio32EventStateSourceDecodingStatus,
    pub event_state_classification: &'static str,
    pub claims_event_clearing: bool,
    pub claims_gpio_rio_pad_mmio_write: bool,
    pub claims_gpio32_ownership: bool,
    pub claims_phy_reset_assertion: bool,
    pub claims_phy_reset_deassertion: bool,
    pub claims_gpio32_write_restore_retry: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventStateDiscriminatorReport {
    pub kind: Rp1EthernetGpio32EventStateDiscriminatorReportKind,
    pub source_contract: Option<Rp1EthernetGpio32EventStateSourceContractEvidence>,
    pub status_raw: Option<u32>,
    pub ctrl_raw: Option<u32>,
    pub rio1_out_raw: Option<u32>,
    pub rio1_oe_raw: Option<u32>,
    pub rio1_in_raw: Option<u32>,
    pub pad_raw: Option<u32>,
    pub source_decoding_status: Rp1EthernetGpio32EventStateSourceDecodingStatus,
    pub event_state_classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventStateDiscriminatorReportEvidence {
    pub report_contract_id: &'static str,
    pub event_state_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub write_restore_source_contract_id: &'static str,
    pub write_restore_guard_contract_id: &'static str,
    pub report_kind: &'static str,
    pub v2_proof_task_id: Option<&'static str>,
    pub v2_closeout_task_id: Option<&'static str>,
    pub v2_commit: Option<&'static str>,
    pub v2_classification: Option<&'static str>,
    pub v2_writes_performed: Option<bool>,
    pub gpio_controller: Option<&'static str>,
    pub gpio_line: Option<u32>,
    pub reset_route: Option<&'static str>,
    pub bank: Option<&'static str>,
    pub bank_local_bit: Option<u32>,
    pub active_low: Option<bool>,
    pub gpio32_status_observed_target: Option<u64>,
    pub gpio32_ctrl_observed_target: Option<u64>,
    pub rio1_out_observed_target: Option<u64>,
    pub rio1_oe_observed_target: Option<u64>,
    pub rio1_in_observed_target: Option<u64>,
    pub gpio32_pad_observed_target: Option<u64>,
    pub status_raw: Option<u32>,
    pub ctrl_raw: Option<u32>,
    pub rio1_out_raw: Option<u32>,
    pub rio1_oe_raw: Option<u32>,
    pub rio1_in_raw: Option<u32>,
    pub pad_raw: Option<u32>,
    pub status_event_mask: Option<u32>,
    pub event_bits: Option<u32>,
    pub source_event_bit_names: Option<&'static [&'static str]>,
    pub source_decoding_status: &'static str,
    pub allowed_classifications: &'static [&'static str],
    pub hardware_proof_boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub claims_event_clearing: bool,
    pub claims_gpio_rio_pad_mmio_write: bool,
    pub claims_gpio32_ownership: bool,
    pub claims_phy_reset_assertion: bool,
    pub claims_phy_reset_deassertion: bool,
    pub claims_gpio32_write_restore_retry: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_ownership: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32EventStateDiscriminatorReportError {
    CandidateMissingSourceContract,
    CandidateMissingSelectedReads,
    ControlCarriesGpioTargetFacts,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractV2LineageMismatch,
    MissingSourceEvidence,
    EventStateClassificationNotAllowed,
    EventStateClassificationMismatch,
    EventClearingClaim,
    GpioRioPadMmioWriteClaim,
    Gpio32OwnershipClaim,
    PhyResetAssertionClaim,
    PhyResetDeassertionClaim,
    Gpio32WriteRestoreRetryClaim,
    MdioPhyOwnershipClaim,
    EthernetReadinessClaim,
    InterruptOwnershipClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetGpio32EventStateDiscriminatorReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::CandidateMissingSelectedReads => "candidate-missing-selected-reads",
            Self::ControlCarriesGpioTargetFacts => "control-carries-gpio-target-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractV2LineageMismatch => "source-contract-v2-lineage-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EventStateClassificationNotAllowed => "event-state-classification-not-allowed",
            Self::EventStateClassificationMismatch => "event-state-classification-mismatch",
            Self::EventClearingClaim => "event-clearing-claim",
            Self::GpioRioPadMmioWriteClaim => "gpio-rio-pad-mmio-write-claim",
            Self::Gpio32OwnershipClaim => "gpio32-ownership-claim",
            Self::PhyResetAssertionClaim => "phy-reset-assertion-claim",
            Self::PhyResetDeassertionClaim => "phy-reset-deassertion-claim",
            Self::Gpio32WriteRestoreRetryClaim => "gpio32-write-restore-retry-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::InterruptOwnershipClaim => "interrupt-ownership-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_gpio32_event_state_source_contract_evidence()
-> Rp1EthernetGpio32EventStateSourceContractEvidence {
    Rp1EthernetGpio32EventStateSourceContractEvidence {
        contract_id: RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_EVENT_STATE_SOURCE_TASK_ID,
        write_restore_source_contract_id:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID,
        write_restore_guard_contract_id:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID,
        v2_proof_task_id: RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_PROOF_TASK_ID,
        v2_closeout_task_id: RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_CLOSEOUT_TASK_ID,
        v2_commit: RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_COMMIT,
        v2_classification: RP1_ETHERNET_GPIO32_EVENT_STATE_V2_BLOCKER_CLASSIFICATION,
        v2_writes_performed: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_WRITES_PERFORMED,
        gpio_controller: RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER,
        gpio_line: RP1_ETHERNET_PHY_RESET_GPIO,
        reset_route: RP1_ETHERNET_PHY_RESET_ROUTE,
        bank: RP1_ETHERNET_GPIO32_BANK,
        bank_local_bit: RP1_ETHERNET_GPIO32_BANK_LOCAL_BIT,
        active_low: RP1_ETHERNET_PHY_RESET_ACTIVE_LOW,
        gpio32_status_observed_target: RP1_ETHERNET_GPIO32_STATUS_OBSERVED_TARGET,
        gpio32_ctrl_observed_target: RP1_ETHERNET_GPIO32_CTRL_OBSERVED_TARGET,
        rio1_out_observed_target: RP1_ETHERNET_GPIO32_RIO1_OUT_OBSERVED_TARGET,
        rio1_oe_observed_target: RP1_ETHERNET_GPIO32_RIO1_OE_OBSERVED_TARGET,
        rio1_in_observed_target: RP1_ETHERNET_GPIO32_RIO1_IN_OBSERVED_TARGET,
        gpio32_pad_observed_target: RP1_ETHERNET_GPIO32_PAD_OBSERVED_TARGET,
        status_event_mask: RP1_ETHERNET_GPIO32_EVENT_STATE_MASK,
        source_event_bit_names: RP1_ETHERNET_GPIO32_STATUS_SOURCE_EVENT_BIT_NAMES,
        accepted_v2_status_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_STATUS_RAW,
        accepted_v2_ctrl_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_CTRL_RAW,
        accepted_v2_rio1_out_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OUT_RAW,
        accepted_v2_rio1_oe_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OE_RAW,
        accepted_v2_rio1_in_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_IN_RAW,
        accepted_v2_event_bits: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_EVENT_BITS,
        source_evidence: RP1_ETHERNET_GPIO32_EVENT_STATE_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_gpio32_event_state_discriminator_report(
    input: Rp1EthernetGpio32EventStateDiscriminatorReportInput,
) -> Result<
    Rp1EthernetGpio32EventStateDiscriminatorReport,
    Rp1EthernetGpio32EventStateDiscriminatorReportError,
> {
    validate_rp1_ethernet_gpio32_event_state_rejected_claims(input)?;
    validate_rp1_ethernet_gpio32_event_state_classification(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetGpio32EventStateDiscriminatorReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_gpio32_event_state_source_contract(source_contract)?;
            if input.source_decoding_status
                != Rp1EthernetGpio32EventStateSourceDecodingStatus::CaptureChainInconclusive
                && (input.status_raw.is_none()
                    || input.ctrl_raw.is_none()
                    || input.rio1_out_raw.is_none()
                    || input.rio1_oe_raw.is_none()
                    || input.rio1_in_raw.is_none()
                    || input.pad_raw.is_none())
            {
                return Err(
                    Rp1EthernetGpio32EventStateDiscriminatorReportError::CandidateMissingSelectedReads,
                );
            }
            Ok(Rp1EthernetGpio32EventStateDiscriminatorReport {
                kind: input.kind,
                source_contract: Some(source_contract),
                status_raw: input.status_raw,
                ctrl_raw: input.ctrl_raw,
                rio1_out_raw: input.rio1_out_raw,
                rio1_oe_raw: input.rio1_oe_raw,
                rio1_in_raw: input.rio1_in_raw,
                pad_raw: input.pad_raw,
                source_decoding_status: input.source_decoding_status,
                event_state_classification: input.event_state_classification,
            })
        }
        (Rp1EthernetGpio32EventStateDiscriminatorReportKind::Candidate, None) => {
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetGpio32EventStateDiscriminatorReportKind::NoGpioNoEthernetControl, None)
            if input.status_raw.is_none()
                && input.ctrl_raw.is_none()
                && input.rio1_out_raw.is_none()
                && input.rio1_oe_raw.is_none()
                && input.rio1_in_raw.is_none()
                && input.pad_raw.is_none()
                && input.source_decoding_status
                    == Rp1EthernetGpio32EventStateSourceDecodingStatus::CaptureChainInconclusive
                && input.event_state_classification
                    == RP1_ETHERNET_GPIO32_EVENT_STATE_CONTROL_CLASSIFICATION =>
        {
            Ok(Rp1EthernetGpio32EventStateDiscriminatorReport {
                kind: input.kind,
                source_contract: None,
                status_raw: None,
                ctrl_raw: None,
                rio1_out_raw: None,
                rio1_oe_raw: None,
                rio1_in_raw: None,
                pad_raw: None,
                source_decoding_status: input.source_decoding_status,
                event_state_classification: input.event_state_classification,
            })
        }
        (Rp1EthernetGpio32EventStateDiscriminatorReportKind::NoGpioNoEthernetControl, _) => {
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::ControlCarriesGpioTargetFacts)
        }
    }
}

pub fn rp1_ethernet_gpio32_event_state_discriminator_report_evidence(
    report: Rp1EthernetGpio32EventStateDiscriminatorReport,
) -> Rp1EthernetGpio32EventStateDiscriminatorReportEvidence {
    match report.source_contract {
        Some(source_contract) => rp1_ethernet_gpio32_event_state_candidate_evidence(
            report.kind.name(),
            source_contract,
            report.status_raw,
            report.ctrl_raw,
            report.rio1_out_raw,
            report.rio1_oe_raw,
            report.rio1_in_raw,
            report.pad_raw,
            report.source_decoding_status,
            report.event_state_classification,
        ),
        None => rp1_ethernet_gpio32_event_state_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_gpio32_event_state_discriminator_report_evidence(
    error: Rp1EthernetGpio32EventStateDiscriminatorReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_gpio32_event_state_rejected_claims(
    input: Rp1EthernetGpio32EventStateDiscriminatorReportInput,
) -> Result<(), Rp1EthernetGpio32EventStateDiscriminatorReportError> {
    if input.claims_event_clearing {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::EventClearingClaim);
    }
    if input.claims_gpio_rio_pad_mmio_write {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::GpioRioPadMmioWriteClaim);
    }
    if input.claims_gpio32_ownership {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::Gpio32OwnershipClaim);
    }
    if input.claims_phy_reset_assertion {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::PhyResetAssertionClaim);
    }
    if input.claims_phy_reset_deassertion {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::PhyResetDeassertionClaim);
    }
    if input.claims_gpio32_write_restore_retry {
        return Err(
            Rp1EthernetGpio32EventStateDiscriminatorReportError::Gpio32WriteRestoreRetryClaim,
        );
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::EthernetReadinessClaim);
    }
    if input.claims_interrupt_ownership {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::InterruptOwnershipClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(
            Rp1EthernetGpio32EventStateDiscriminatorReportError::DmaDescriptorOwnershipClaim,
        );
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_event_state_classification(
    input: Rp1EthernetGpio32EventStateDiscriminatorReportInput,
) -> Result<(), Rp1EthernetGpio32EventStateDiscriminatorReportError> {
    if !RP1_ETHERNET_GPIO32_EVENT_STATE_CLASSIFICATIONS.contains(&input.event_state_classification)
    {
        return Err(
            Rp1EthernetGpio32EventStateDiscriminatorReportError::EventStateClassificationNotAllowed,
        );
    }
    if input.kind == Rp1EthernetGpio32EventStateDiscriminatorReportKind::NoGpioNoEthernetControl {
        return Ok(());
    }

    let expected = match input.source_decoding_status {
        Rp1EthernetGpio32EventStateSourceDecodingStatus::SourceBackedBits20To27 => {
            match input.status_raw {
                Some(status_raw) if status_raw & RP1_ETHERNET_GPIO32_EVENT_STATE_MASK == 0 => {
                    "rp1-ethernet-gpio32-event-state-clear-precondition"
                }
                Some(_) => RP1_ETHERNET_GPIO32_EVENT_STATE_CANDIDATE_CLASSIFICATION,
                None => "rp1-ethernet-gpio32-event-state-inconclusive-capture",
            }
        }
        Rp1EthernetGpio32EventStateSourceDecodingStatus::SourceUnresolved => {
            "rp1-ethernet-gpio32-event-state-source-unresolved-event-state"
        }
        Rp1EthernetGpio32EventStateSourceDecodingStatus::CaptureChainInconclusive => {
            "rp1-ethernet-gpio32-event-state-inconclusive-capture"
        }
    };
    if input.event_state_classification != expected {
        return Err(
            Rp1EthernetGpio32EventStateDiscriminatorReportError::EventStateClassificationMismatch,
        );
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_event_state_source_contract(
    source_contract: Rp1EthernetGpio32EventStateSourceContractEvidence,
) -> Result<(), Rp1EthernetGpio32EventStateDiscriminatorReportError> {
    if source_contract.contract_id != RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID
        || source_contract.source_task_id != RP1_ETHERNET_GPIO32_EVENT_STATE_SOURCE_TASK_ID
        || source_contract.write_restore_source_contract_id
            != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID
        || source_contract.write_restore_guard_contract_id
            != RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID
        || source_contract.gpio_controller != RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER
        || source_contract.gpio_line != RP1_ETHERNET_PHY_RESET_GPIO
        || source_contract.reset_route != RP1_ETHERNET_PHY_RESET_ROUTE
        || source_contract.bank != RP1_ETHERNET_GPIO32_BANK
        || source_contract.bank_local_bit != RP1_ETHERNET_GPIO32_BANK_LOCAL_BIT
        || source_contract.active_low != RP1_ETHERNET_PHY_RESET_ACTIVE_LOW
    {
        return Err(
            Rp1EthernetGpio32EventStateDiscriminatorReportError::SourceContractIdentityMismatch,
        );
    }
    if source_contract.gpio32_status_observed_target != RP1_ETHERNET_GPIO32_STATUS_OBSERVED_TARGET
        || source_contract.gpio32_ctrl_observed_target != RP1_ETHERNET_GPIO32_CTRL_OBSERVED_TARGET
        || source_contract.rio1_out_observed_target != RP1_ETHERNET_GPIO32_RIO1_OUT_OBSERVED_TARGET
        || source_contract.rio1_oe_observed_target != RP1_ETHERNET_GPIO32_RIO1_OE_OBSERVED_TARGET
        || source_contract.rio1_in_observed_target != RP1_ETHERNET_GPIO32_RIO1_IN_OBSERVED_TARGET
        || source_contract.gpio32_pad_observed_target != RP1_ETHERNET_GPIO32_PAD_OBSERVED_TARGET
        || source_contract.status_event_mask != RP1_ETHERNET_GPIO32_EVENT_STATE_MASK
        || source_contract.source_event_bit_names
            != RP1_ETHERNET_GPIO32_STATUS_SOURCE_EVENT_BIT_NAMES
    {
        return Err(
            Rp1EthernetGpio32EventStateDiscriminatorReportError::SourceContractTargetMismatch,
        );
    }
    if source_contract.v2_proof_task_id
        != RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_PROOF_TASK_ID
        || source_contract.v2_closeout_task_id
            != RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_CLOSEOUT_TASK_ID
        || source_contract.v2_commit != RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_COMMIT
        || source_contract.v2_classification
            != RP1_ETHERNET_GPIO32_EVENT_STATE_V2_BLOCKER_CLASSIFICATION
        || source_contract.v2_writes_performed
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_WRITES_PERFORMED
        || source_contract.accepted_v2_status_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_STATUS_RAW
        || source_contract.accepted_v2_ctrl_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_CTRL_RAW
        || source_contract.accepted_v2_rio1_out_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OUT_RAW
        || source_contract.accepted_v2_rio1_oe_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OE_RAW
        || source_contract.accepted_v2_rio1_in_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_IN_RAW
        || source_contract.accepted_v2_event_bits
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_EVENT_BITS
    {
        return Err(
            Rp1EthernetGpio32EventStateDiscriminatorReportError::SourceContractV2LineageMismatch,
        );
    }
    if source_contract.source_evidence.is_empty() {
        return Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_gpio32_event_state_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetGpio32EventStateSourceContractEvidence,
    status_raw: Option<u32>,
    ctrl_raw: Option<u32>,
    rio1_out_raw: Option<u32>,
    rio1_oe_raw: Option<u32>,
    rio1_in_raw: Option<u32>,
    pad_raw: Option<u32>,
    source_decoding_status: Rp1EthernetGpio32EventStateSourceDecodingStatus,
    event_state_classification: &'static str,
) -> Rp1EthernetGpio32EventStateDiscriminatorReportEvidence {
    Rp1EthernetGpio32EventStateDiscriminatorReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_EVENT_STATE_REPORT_CONTRACT_ID,
        event_state_contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        write_restore_source_contract_id: source_contract.write_restore_source_contract_id,
        write_restore_guard_contract_id: source_contract.write_restore_guard_contract_id,
        report_kind,
        v2_proof_task_id: Some(source_contract.v2_proof_task_id),
        v2_closeout_task_id: Some(source_contract.v2_closeout_task_id),
        v2_commit: Some(source_contract.v2_commit),
        v2_classification: Some(source_contract.v2_classification),
        v2_writes_performed: Some(source_contract.v2_writes_performed),
        gpio_controller: Some(source_contract.gpio_controller),
        gpio_line: Some(source_contract.gpio_line),
        reset_route: Some(source_contract.reset_route),
        bank: Some(source_contract.bank),
        bank_local_bit: Some(source_contract.bank_local_bit),
        active_low: Some(source_contract.active_low),
        gpio32_status_observed_target: Some(source_contract.gpio32_status_observed_target),
        gpio32_ctrl_observed_target: Some(source_contract.gpio32_ctrl_observed_target),
        rio1_out_observed_target: Some(source_contract.rio1_out_observed_target),
        rio1_oe_observed_target: Some(source_contract.rio1_oe_observed_target),
        rio1_in_observed_target: Some(source_contract.rio1_in_observed_target),
        gpio32_pad_observed_target: Some(source_contract.gpio32_pad_observed_target),
        status_raw,
        ctrl_raw,
        rio1_out_raw,
        rio1_oe_raw,
        rio1_in_raw,
        pad_raw,
        status_event_mask: Some(source_contract.status_event_mask),
        event_bits: status_raw.map(|raw| raw & source_contract.status_event_mask),
        source_event_bit_names: Some(source_contract.source_event_bit_names),
        source_decoding_status: source_decoding_status.name(),
        allowed_classifications: RP1_ETHERNET_GPIO32_EVENT_STATE_CLASSIFICATIONS,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GPIO32_EVENT_STATE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GPIO32_EVENT_STATE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_EVENT_STATE_RETAINED_RISKS,
        source_evidence: Some(source_contract.source_evidence),
        claims_event_clearing: false,
        claims_gpio_rio_pad_mmio_write: false,
        claims_gpio32_ownership: false,
        claims_phy_reset_assertion: false,
        claims_phy_reset_deassertion: false,
        claims_gpio32_write_restore_retry: false,
        claims_mdio_phy_ownership: false,
        claims_ethernet_ready: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: event_state_classification,
    }
}

fn rp1_ethernet_gpio32_event_state_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetGpio32EventStateDiscriminatorReportEvidence {
    Rp1EthernetGpio32EventStateDiscriminatorReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_EVENT_STATE_REPORT_CONTRACT_ID,
        event_state_contract_id: RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_EVENT_STATE_SOURCE_TASK_ID,
        write_restore_source_contract_id:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID,
        write_restore_guard_contract_id:
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID,
        report_kind,
        v2_proof_task_id: None,
        v2_closeout_task_id: None,
        v2_commit: None,
        v2_classification: None,
        v2_writes_performed: None,
        gpio_controller: None,
        gpio_line: None,
        reset_route: None,
        bank: None,
        bank_local_bit: None,
        active_low: None,
        gpio32_status_observed_target: None,
        gpio32_ctrl_observed_target: None,
        rio1_out_observed_target: None,
        rio1_oe_observed_target: None,
        rio1_in_observed_target: None,
        gpio32_pad_observed_target: None,
        status_raw: None,
        ctrl_raw: None,
        rio1_out_raw: None,
        rio1_oe_raw: None,
        rio1_in_raw: None,
        pad_raw: None,
        status_event_mask: None,
        event_bits: None,
        source_event_bit_names: None,
        source_decoding_status:
            Rp1EthernetGpio32EventStateSourceDecodingStatus::CaptureChainInconclusive.name(),
        allowed_classifications: RP1_ETHERNET_GPIO32_EVENT_STATE_CLASSIFICATIONS,
        hardware_proof_boundary_classification:
            RP1_ETHERNET_GPIO32_EVENT_STATE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GPIO32_EVENT_STATE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_EVENT_STATE_RETAINED_RISKS,
        source_evidence: None,
        claims_event_clearing: false,
        claims_gpio_rio_pad_mmio_write: false,
        claims_gpio32_ownership: false,
        claims_phy_reset_assertion: false,
        claims_phy_reset_deassertion: false,
        claims_gpio32_write_restore_retry: false,
        claims_mdio_phy_ownership: false,
        claims_ethernet_ready: false,
        claims_interrupt_ownership: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GPIO32_EVENT_STATE_CONTROL_CLASSIFICATION,
    }
}

pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-event-clear-source-contract-v1";
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-gpio32-event-clear-source-contract-20260611";
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_GUARD_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-gpio32-event-clear-guard-report-contract-v1";
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-gpio32-event-clear-guard-candidate-local-static";
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_CONTROL_CLASSIFICATION: &str =
    "no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control";
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_BOUNDARY_CLASSIFICATION: &str =
    "local-static-gpio32-event-clear-guard-report-only";
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_TARGET: u64 = 0xc0_400d_6024;
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_OBSERVED_TARGET: u64 = 0x1c_000d_6024;
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_WRITE_VALUE: u32 = 0x1000_0000;
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_WRITE_VALUE_NAME: &str = "RP1_GPIO_CTRL_IRQRESET";
pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_ACCESS: &str =
    "single 32-bit little-endian volatile store in future guarded hardware proof only";

pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_PRE_READ_REQUIREMENTS: &[&str] = &[
    "candidate/control identity, serial freshness, TFTP delta, and final identity are capture-chain-current",
    "GPIO32 STATUS/CTRL, RIO1 OUT/OE/IN, and GPIO32 pad reads are present and non-sentinel",
    "GPIO32 STATUS & 0x0ff00000 == 0x0ab00000 before the selected write",
    "GPIO32 CTRL FUNCSEL remains accepted GPIO function value 5",
    "GPIO32 CTRL OUTOVER/OEOVER/INOVER do not bypass raw RIO OUT/OE handling",
    "RIO1 OUT/OE/IN and GPIO32 pad are retained for post-write comparison",
];

pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_POST_READ_REQUIREMENTS: &[&str] = &[
    "GPIO32 STATUS event bits clear or classify as persistent/source-owned without GPIO32 ownership",
    "GPIO32 CTRL non-IRQRESET fields match the pre-read value",
    "RIO1 OUT, RIO1 OE, RIO1 IN, and GPIO32 pad match pre-read values",
    "no PHY reset, MDIO/PHY, Ethernet driver, interrupt completion, DMA, packet, networking, SSH, Phase 12.2, or phase-transition claim",
];

pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_FORBIDDEN_WRITES: &[&str] = &[
    "GPIO32 STATUS",
    "GPIO32 CTRL RW/CLR/XOR aliases",
    "IO_BANK1 INTE/INTS",
    "RIO1 OUT/OE/IN",
    "GPIO32 pad",
    "clock/reset registers",
    "MDIO/PHY registers",
    "Ethernet MAC/GEM registers",
    "DMA/descriptors",
    "interrupt-controller registers",
    "non-GPIO32 registers",
];

pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-source-contract.md",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-source-contract/classification.json",
    "tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-proof-closeout.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-source-contract.md",
    "tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c",
];

pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "event clear executed by the local/static guard",
    "volatile MMIO execution by the local/static guard",
    "writes outside GPIO32 CTRL SET alias IRQRESET value 0x10000000",
    "GPIO32 CTRL RW/CLR/XOR, RIO, pad, function, or non-GPIO32 mutation",
    "GPIO32 ownership",
    "PHY reset assertion or deassertion",
    "GPIO32 write/restore retry or success",
    "MDIO transactions or PHY ownership",
    "Ethernet driver readiness",
    "interrupt ownership or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_GPIO32_EVENT_CLEAR_RETAINED_RISKS: &[&str] = &[
    "level event bits may reassert after IRQRESET if line state remains active",
    "firmware or hardware may repopulate event bits after the clear",
    "future proof may classify persistent/source-owned event state or capture/staging blockers",
    "GPIO32 write/restore ownership and PHY reset remain unaccepted",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventClearSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub event_state_contract_id: &'static str,
    pub event_state_closeout_task_id: &'static str,
    pub event_state_closeout_commit: &'static str,
    pub gpio_controller: &'static str,
    pub gpio_line: u32,
    pub reset_route: &'static str,
    pub bank: &'static str,
    pub bank_local_bit: u32,
    pub gpio32_status_observed_target: u64,
    pub gpio32_ctrl_observed_target: u64,
    pub rio1_out_observed_target: u64,
    pub rio1_oe_observed_target: u64,
    pub rio1_in_observed_target: u64,
    pub gpio32_pad_observed_target: u64,
    pub clear_source_target: u64,
    pub clear_observed_target: u64,
    pub width_bits: u32,
    pub access: &'static str,
    pub write_value: u32,
    pub write_value_name: &'static str,
    pub status_event_mask: u32,
    pub accepted_event_bits: u32,
    pub accepted_status_raw: u32,
    pub accepted_ctrl_raw: u32,
    pub accepted_rio1_out_raw: u32,
    pub accepted_rio1_oe_raw: u32,
    pub accepted_rio1_in_raw: u32,
    pub pre_read_requirements: &'static [&'static str],
    pub post_read_requirements: &'static [&'static str],
    pub forbidden_writes: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32EventClearGuardReportKind {
    Candidate,
    NoGpioNoEthernetControl,
}

impl Rp1EthernetGpio32EventClearGuardReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoGpioNoEthernetControl => "no-gpio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventClearGuardReportInput {
    pub kind: Rp1EthernetGpio32EventClearGuardReportKind,
    pub source_contract: Option<Rp1EthernetGpio32EventClearSourceContractEvidence>,
    pub claims_event_clear_executed: bool,
    pub claims_volatile_mmio_execution: bool,
    pub claims_write_outside_irqreset: bool,
    pub claims_ctrl_rw_clr_xor_write: bool,
    pub claims_rio_pad_function_mutation: bool,
    pub claims_gpio32_ownership: bool,
    pub claims_phy_reset_assertion: bool,
    pub claims_phy_reset_deassertion: bool,
    pub claims_gpio32_write_restore_retry: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventClearGuardReport {
    pub kind: Rp1EthernetGpio32EventClearGuardReportKind,
    pub source_contract: Option<Rp1EthernetGpio32EventClearSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetGpio32EventClearGuardReportEvidence {
    pub report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub event_state_contract_id: &'static str,
    pub event_state_closeout_task_id: &'static str,
    pub event_state_closeout_commit: &'static str,
    pub report_kind: &'static str,
    pub gpio_controller: Option<&'static str>,
    pub gpio_line: Option<u32>,
    pub reset_route: Option<&'static str>,
    pub bank: Option<&'static str>,
    pub bank_local_bit: Option<u32>,
    pub gpio32_status_observed_target: Option<u64>,
    pub gpio32_ctrl_observed_target: Option<u64>,
    pub rio1_out_observed_target: Option<u64>,
    pub rio1_oe_observed_target: Option<u64>,
    pub rio1_in_observed_target: Option<u64>,
    pub gpio32_pad_observed_target: Option<u64>,
    pub clear_source_target: Option<u64>,
    pub clear_observed_target: Option<u64>,
    pub width_bits: Option<u32>,
    pub access: Option<&'static str>,
    pub write_value: Option<u32>,
    pub write_value_name: Option<&'static str>,
    pub status_event_mask: Option<u32>,
    pub accepted_event_bits: Option<u32>,
    pub accepted_status_raw: Option<u32>,
    pub accepted_ctrl_raw: Option<u32>,
    pub accepted_rio1_out_raw: Option<u32>,
    pub accepted_rio1_oe_raw: Option<u32>,
    pub accepted_rio1_in_raw: Option<u32>,
    pub pre_read_requirements: Option<&'static [&'static str]>,
    pub post_read_requirements: Option<&'static [&'static str]>,
    pub forbidden_writes: Option<&'static [&'static str]>,
    pub boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub claims_event_clear_executed: bool,
    pub claims_volatile_mmio_execution: bool,
    pub claims_write_outside_irqreset: bool,
    pub claims_ctrl_rw_clr_xor_write: bool,
    pub claims_rio_pad_function_mutation: bool,
    pub claims_gpio32_ownership: bool,
    pub claims_phy_reset_assertion: bool,
    pub claims_phy_reset_deassertion: bool,
    pub claims_gpio32_write_restore_retry: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetGpio32EventClearGuardReportError {
    CandidateMissingSourceContract,
    ControlCarriesGpioTargetFacts,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    SourceContractLineageMismatch,
    MissingSourceEvidence,
    EventClearExecutedClaim,
    VolatileMmioExecutionClaim,
    WriteOutsideIrqresetClaim,
    CtrlRwClrXorWriteClaim,
    RioPadFunctionMutationClaim,
    Gpio32OwnershipClaim,
    PhyResetAssertionClaim,
    PhyResetDeassertionClaim,
    Gpio32WriteRestoreRetryClaim,
    MdioPhyOwnershipClaim,
    EthernetReadinessClaim,
    InterruptCompletionClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetGpio32EventClearGuardReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesGpioTargetFacts => "control-carries-gpio-target-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::SourceContractLineageMismatch => "source-contract-lineage-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::EventClearExecutedClaim => "event-clear-executed-claim",
            Self::VolatileMmioExecutionClaim => "volatile-mmio-execution-claim",
            Self::WriteOutsideIrqresetClaim => "write-outside-irqreset-claim",
            Self::CtrlRwClrXorWriteClaim => "ctrl-rw-clr-xor-write-claim",
            Self::RioPadFunctionMutationClaim => "rio-pad-function-mutation-claim",
            Self::Gpio32OwnershipClaim => "gpio32-ownership-claim",
            Self::PhyResetAssertionClaim => "phy-reset-assertion-claim",
            Self::PhyResetDeassertionClaim => "phy-reset-deassertion-claim",
            Self::Gpio32WriteRestoreRetryClaim => "gpio32-write-restore-retry-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_gpio32_event_clear_source_contract_evidence()
-> Rp1EthernetGpio32EventClearSourceContractEvidence {
    Rp1EthernetGpio32EventClearSourceContractEvidence {
        contract_id: RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_TASK_ID,
        event_state_contract_id: RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID,
        event_state_closeout_task_id: "phase12-rp1-ethernet-gpio32-event-state-proof-closeout-20260611",
        event_state_closeout_commit: "920327ac25db3eb37fcd60e183ae95d22a7bef5a",
        gpio_controller: RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER,
        gpio_line: RP1_ETHERNET_PHY_RESET_GPIO,
        reset_route: RP1_ETHERNET_PHY_RESET_ROUTE,
        bank: RP1_ETHERNET_GPIO32_BANK,
        bank_local_bit: RP1_ETHERNET_GPIO32_BANK_LOCAL_BIT,
        gpio32_status_observed_target: RP1_ETHERNET_GPIO32_STATUS_OBSERVED_TARGET,
        gpio32_ctrl_observed_target: RP1_ETHERNET_GPIO32_CTRL_OBSERVED_TARGET,
        rio1_out_observed_target: RP1_ETHERNET_GPIO32_RIO1_OUT_OBSERVED_TARGET,
        rio1_oe_observed_target: RP1_ETHERNET_GPIO32_RIO1_OE_OBSERVED_TARGET,
        rio1_in_observed_target: RP1_ETHERNET_GPIO32_RIO1_IN_OBSERVED_TARGET,
        gpio32_pad_observed_target: RP1_ETHERNET_GPIO32_PAD_OBSERVED_TARGET,
        clear_source_target: RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_TARGET,
        clear_observed_target: RP1_ETHERNET_GPIO32_EVENT_CLEAR_OBSERVED_TARGET,
        width_bits: RP1_ETHERNET_GPIO32_EVENT_CLEAR_WIDTH_BITS,
        access: RP1_ETHERNET_GPIO32_EVENT_CLEAR_ACCESS,
        write_value: RP1_ETHERNET_GPIO32_EVENT_CLEAR_WRITE_VALUE,
        write_value_name: RP1_ETHERNET_GPIO32_EVENT_CLEAR_WRITE_VALUE_NAME,
        status_event_mask: RP1_ETHERNET_GPIO32_EVENT_STATE_MASK,
        accepted_event_bits: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_EVENT_BITS,
        accepted_status_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_STATUS_RAW,
        accepted_ctrl_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_CTRL_RAW,
        accepted_rio1_out_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OUT_RAW,
        accepted_rio1_oe_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OE_RAW,
        accepted_rio1_in_raw: RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_IN_RAW,
        pre_read_requirements: RP1_ETHERNET_GPIO32_EVENT_CLEAR_PRE_READ_REQUIREMENTS,
        post_read_requirements: RP1_ETHERNET_GPIO32_EVENT_CLEAR_POST_READ_REQUIREMENTS,
        forbidden_writes: RP1_ETHERNET_GPIO32_EVENT_CLEAR_FORBIDDEN_WRITES,
        source_evidence: RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_gpio32_event_clear_guard_report(
    input: Rp1EthernetGpio32EventClearGuardReportInput,
) -> Result<Rp1EthernetGpio32EventClearGuardReport, Rp1EthernetGpio32EventClearGuardReportError> {
    validate_rp1_ethernet_gpio32_event_clear_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetGpio32EventClearGuardReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_gpio32_event_clear_source_contract(source_contract)?;
            Ok(Rp1EthernetGpio32EventClearGuardReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetGpio32EventClearGuardReportKind::Candidate, None) => {
            Err(Rp1EthernetGpio32EventClearGuardReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetGpio32EventClearGuardReportKind::NoGpioNoEthernetControl, None) => {
            Ok(Rp1EthernetGpio32EventClearGuardReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetGpio32EventClearGuardReportKind::NoGpioNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetGpio32EventClearGuardReportError::ControlCarriesGpioTargetFacts)
        }
    }
}

pub fn rp1_ethernet_gpio32_event_clear_guard_report_evidence(
    report: Rp1EthernetGpio32EventClearGuardReport,
) -> Rp1EthernetGpio32EventClearGuardReportEvidence {
    match report.source_contract {
        Some(source_contract) => rp1_ethernet_gpio32_event_clear_guard_candidate_evidence(
            report.kind.name(),
            source_contract,
        ),
        None => rp1_ethernet_gpio32_event_clear_guard_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_gpio32_event_clear_guard_report_evidence(
    error: Rp1EthernetGpio32EventClearGuardReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_gpio32_event_clear_rejected_claims(
    input: Rp1EthernetGpio32EventClearGuardReportInput,
) -> Result<(), Rp1EthernetGpio32EventClearGuardReportError> {
    if input.claims_event_clear_executed {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::EventClearExecutedClaim);
    }
    if input.claims_volatile_mmio_execution {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::VolatileMmioExecutionClaim);
    }
    if input.claims_write_outside_irqreset {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::WriteOutsideIrqresetClaim);
    }
    if input.claims_ctrl_rw_clr_xor_write {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::CtrlRwClrXorWriteClaim);
    }
    if input.claims_rio_pad_function_mutation {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::RioPadFunctionMutationClaim);
    }
    if input.claims_gpio32_ownership {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::Gpio32OwnershipClaim);
    }
    if input.claims_phy_reset_assertion {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::PhyResetAssertionClaim);
    }
    if input.claims_phy_reset_deassertion {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::PhyResetDeassertionClaim);
    }
    if input.claims_gpio32_write_restore_retry {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::Gpio32WriteRestoreRetryClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::EthernetReadinessClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::InterruptCompletionClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_gpio32_event_clear_source_contract(
    source_contract: Rp1EthernetGpio32EventClearSourceContractEvidence,
) -> Result<(), Rp1EthernetGpio32EventClearGuardReportError> {
    if source_contract.contract_id != RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_CONTRACT_ID
        || source_contract.source_task_id != RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_TASK_ID
        || source_contract.event_state_contract_id != RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID
        || source_contract.gpio_controller != RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER
        || source_contract.gpio_line != RP1_ETHERNET_PHY_RESET_GPIO
        || source_contract.reset_route != RP1_ETHERNET_PHY_RESET_ROUTE
        || source_contract.bank != RP1_ETHERNET_GPIO32_BANK
        || source_contract.bank_local_bit != RP1_ETHERNET_GPIO32_BANK_LOCAL_BIT
    {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::SourceContractIdentityMismatch);
    }
    if source_contract.gpio32_status_observed_target != RP1_ETHERNET_GPIO32_STATUS_OBSERVED_TARGET
        || source_contract.gpio32_ctrl_observed_target != RP1_ETHERNET_GPIO32_CTRL_OBSERVED_TARGET
        || source_contract.rio1_out_observed_target != RP1_ETHERNET_GPIO32_RIO1_OUT_OBSERVED_TARGET
        || source_contract.rio1_oe_observed_target != RP1_ETHERNET_GPIO32_RIO1_OE_OBSERVED_TARGET
        || source_contract.rio1_in_observed_target != RP1_ETHERNET_GPIO32_RIO1_IN_OBSERVED_TARGET
        || source_contract.gpio32_pad_observed_target != RP1_ETHERNET_GPIO32_PAD_OBSERVED_TARGET
        || source_contract.clear_source_target != RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_TARGET
        || source_contract.clear_observed_target != RP1_ETHERNET_GPIO32_EVENT_CLEAR_OBSERVED_TARGET
    {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::SourceContractTargetMismatch);
    }
    if source_contract.width_bits != RP1_ETHERNET_GPIO32_EVENT_CLEAR_WIDTH_BITS
        || source_contract.access != RP1_ETHERNET_GPIO32_EVENT_CLEAR_ACCESS
        || source_contract.write_value != RP1_ETHERNET_GPIO32_EVENT_CLEAR_WRITE_VALUE
        || source_contract.write_value_name != RP1_ETHERNET_GPIO32_EVENT_CLEAR_WRITE_VALUE_NAME
        || source_contract.status_event_mask != RP1_ETHERNET_GPIO32_EVENT_STATE_MASK
        || source_contract.accepted_event_bits
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_EVENT_BITS
        || source_contract.pre_read_requirements
            != RP1_ETHERNET_GPIO32_EVENT_CLEAR_PRE_READ_REQUIREMENTS
        || source_contract.post_read_requirements
            != RP1_ETHERNET_GPIO32_EVENT_CLEAR_POST_READ_REQUIREMENTS
        || source_contract.forbidden_writes != RP1_ETHERNET_GPIO32_EVENT_CLEAR_FORBIDDEN_WRITES
    {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::SourceContractFieldMismatch);
    }
    if source_contract.event_state_closeout_task_id
        != "phase12-rp1-ethernet-gpio32-event-state-proof-closeout-20260611"
        || source_contract.event_state_closeout_commit != "920327ac25db3eb37fcd60e183ae95d22a7bef5a"
        || source_contract.accepted_status_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_STATUS_RAW
        || source_contract.accepted_ctrl_raw != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_CTRL_RAW
        || source_contract.accepted_rio1_out_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OUT_RAW
        || source_contract.accepted_rio1_oe_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OE_RAW
        || source_contract.accepted_rio1_in_raw
            != RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_IN_RAW
    {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::SourceContractLineageMismatch);
    }
    if source_contract.source_evidence != RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_EVIDENCE {
        return Err(Rp1EthernetGpio32EventClearGuardReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_gpio32_event_clear_guard_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetGpio32EventClearSourceContractEvidence,
) -> Rp1EthernetGpio32EventClearGuardReportEvidence {
    Rp1EthernetGpio32EventClearGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_EVENT_CLEAR_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        event_state_contract_id: source_contract.event_state_contract_id,
        event_state_closeout_task_id: source_contract.event_state_closeout_task_id,
        event_state_closeout_commit: source_contract.event_state_closeout_commit,
        report_kind,
        gpio_controller: Some(source_contract.gpio_controller),
        gpio_line: Some(source_contract.gpio_line),
        reset_route: Some(source_contract.reset_route),
        bank: Some(source_contract.bank),
        bank_local_bit: Some(source_contract.bank_local_bit),
        gpio32_status_observed_target: Some(source_contract.gpio32_status_observed_target),
        gpio32_ctrl_observed_target: Some(source_contract.gpio32_ctrl_observed_target),
        rio1_out_observed_target: Some(source_contract.rio1_out_observed_target),
        rio1_oe_observed_target: Some(source_contract.rio1_oe_observed_target),
        rio1_in_observed_target: Some(source_contract.rio1_in_observed_target),
        gpio32_pad_observed_target: Some(source_contract.gpio32_pad_observed_target),
        clear_source_target: Some(source_contract.clear_source_target),
        clear_observed_target: Some(source_contract.clear_observed_target),
        width_bits: Some(source_contract.width_bits),
        access: Some(source_contract.access),
        write_value: Some(source_contract.write_value),
        write_value_name: Some(source_contract.write_value_name),
        status_event_mask: Some(source_contract.status_event_mask),
        accepted_event_bits: Some(source_contract.accepted_event_bits),
        accepted_status_raw: Some(source_contract.accepted_status_raw),
        accepted_ctrl_raw: Some(source_contract.accepted_ctrl_raw),
        accepted_rio1_out_raw: Some(source_contract.accepted_rio1_out_raw),
        accepted_rio1_oe_raw: Some(source_contract.accepted_rio1_oe_raw),
        accepted_rio1_in_raw: Some(source_contract.accepted_rio1_in_raw),
        pre_read_requirements: Some(source_contract.pre_read_requirements),
        post_read_requirements: Some(source_contract.post_read_requirements),
        forbidden_writes: Some(source_contract.forbidden_writes),
        boundary_classification: RP1_ETHERNET_GPIO32_EVENT_CLEAR_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GPIO32_EVENT_CLEAR_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_EVENT_CLEAR_RETAINED_RISKS,
        source_evidence: Some(source_contract.source_evidence),
        claims_event_clear_executed: false,
        claims_volatile_mmio_execution: false,
        claims_write_outside_irqreset: false,
        claims_ctrl_rw_clr_xor_write: false,
        claims_rio_pad_function_mutation: false,
        claims_gpio32_ownership: false,
        claims_phy_reset_assertion: false,
        claims_phy_reset_deassertion: false,
        claims_gpio32_write_restore_retry: false,
        claims_mdio_phy_ownership: false,
        claims_ethernet_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GPIO32_EVENT_CLEAR_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_gpio32_event_clear_guard_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetGpio32EventClearGuardReportEvidence {
    Rp1EthernetGpio32EventClearGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_GPIO32_EVENT_CLEAR_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_TASK_ID,
        event_state_contract_id: RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID,
        event_state_closeout_task_id: "phase12-rp1-ethernet-gpio32-event-state-proof-closeout-20260611",
        event_state_closeout_commit: "920327ac25db3eb37fcd60e183ae95d22a7bef5a",
        report_kind,
        gpio_controller: None,
        gpio_line: None,
        reset_route: None,
        bank: None,
        bank_local_bit: None,
        gpio32_status_observed_target: None,
        gpio32_ctrl_observed_target: None,
        rio1_out_observed_target: None,
        rio1_oe_observed_target: None,
        rio1_in_observed_target: None,
        gpio32_pad_observed_target: None,
        clear_source_target: None,
        clear_observed_target: None,
        width_bits: None,
        access: None,
        write_value: None,
        write_value_name: None,
        status_event_mask: None,
        accepted_event_bits: None,
        accepted_status_raw: None,
        accepted_ctrl_raw: None,
        accepted_rio1_out_raw: None,
        accepted_rio1_oe_raw: None,
        accepted_rio1_in_raw: None,
        pre_read_requirements: None,
        post_read_requirements: None,
        forbidden_writes: None,
        boundary_classification: RP1_ETHERNET_GPIO32_EVENT_CLEAR_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_GPIO32_EVENT_CLEAR_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_GPIO32_EVENT_CLEAR_RETAINED_RISKS,
        source_evidence: None,
        claims_event_clear_executed: false,
        claims_volatile_mmio_execution: false,
        claims_write_outside_irqreset: false,
        claims_ctrl_rw_clr_xor_write: false,
        claims_rio_pad_function_mutation: false,
        claims_gpio32_ownership: false,
        claims_phy_reset_assertion: false,
        claims_phy_reset_deassertion: false,
        claims_gpio32_write_restore_retry: false,
        claims_mdio_phy_ownership: false,
        claims_ethernet_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_GPIO32_EVENT_CLEAR_CONTROL_CLASSIFICATION,
    }
}

pub const RP1_ETHERNET_MDIO_PHY_ID_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-phy-id-source-contract-v1";
pub const RP1_ETHERNET_MDIO_PHY_ID_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611";
pub const RP1_ETHERNET_MDIO_PHY_ID_GUARD_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-phy-id-guard-report-contract-v1";
pub const RP1_ETHERNET_MDIO_PHY_ID_SELECTED_DISCRIMINATOR: &str =
    "rp1-ethernet-mdio-clause22-phy1-physid1-physid2";
pub const RP1_ETHERNET_MDIO_PHY_ID_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-mdio-phy-id-guard-candidate-local-static";
pub const RP1_ETHERNET_MDIO_PHY_ID_CONTROL_CLASSIFICATION: &str =
    "no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control";
pub const RP1_ETHERNET_MDIO_PHY_ID_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-mdio-phy-id-control-output";
pub const RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTER_NAMES: &[&str] = &["MII_PHYSID1", "MII_PHYSID2"];
pub const RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS: &[u32] = &[0x02, 0x03];
pub const RP1_ETHERNET_MDIO_PHY_ID_NCR_REGISTER: &str = "NCR";
pub const RP1_ETHERNET_MDIO_PHY_ID_NSR_REGISTER: &str = "NSR";
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_REGISTER: &str = "MAN";
pub const RP1_ETHERNET_MDIO_PHY_ID_NCR_OFFSET: u64 = 0x0000;
pub const RP1_ETHERNET_MDIO_PHY_ID_NSR_OFFSET: u64 = 0x0008;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_OFFSET: u64 = 0x0034;
pub const RP1_ETHERNET_MDIO_PHY_ID_NCR_OBSERVED_TARGET: u64 =
    RP1_ETHERNET_OBSERVED_RP1_BASE + RP1_ETHERNET_MDIO_PHY_ID_NCR_OFFSET;
pub const RP1_ETHERNET_MDIO_PHY_ID_NSR_OBSERVED_TARGET: u64 =
    RP1_ETHERNET_OBSERVED_RP1_BASE + RP1_ETHERNET_MDIO_PHY_ID_NSR_OFFSET;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_OBSERVED_TARGET: u64 =
    RP1_ETHERNET_OBSERVED_RP1_BASE + RP1_ETHERNET_MDIO_PHY_ID_MAN_OFFSET;
pub const RP1_ETHERNET_MDIO_PHY_ID_NCR_MPE_BIT: u8 = 4;
pub const RP1_ETHERNET_MDIO_PHY_ID_NSR_IDLE_BIT: u8 = 2;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_OFFSET: u8 = 0;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_SIZE: u8 = 16;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_CODE: u32 = 2;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_SOF: u32 = 1;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_READ: u32 = 2;
pub const RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_WRITE: u32 = 1;
pub const RP1_ETHERNET_MDIO_PHY_ID_PHYSID1_MAN_FRAME: u32 = 0x600a_0000;
pub const RP1_ETHERNET_MDIO_PHY_ID_PHYSID2_MAN_FRAME: u32 = 0x600e_0000;
pub const RP1_ETHERNET_MDIO_PHY_ID_TIMEOUT_POLICY: &str =
    "bounded-poll-source-derived-linux-1000000-usec-equivalent-or-tighter-talos-owned";
pub const RP1_ETHERNET_MDIO_PHY_ID_RESULT_EXTRACTION: &str = "MAN.DATA bits 15:0";
pub const RP1_ETHERNET_MDIO_PHY_ID_NO_RESTORE_EXPECTATION: &str =
    "no MAN restore write for MDIO reads and no NCR MPE write in first proof";

pub const RP1_ETHERNET_MDIO_PHY_ID_OPERATION_ORDER: &[&str] = &[
    "print candidate start marker and accepted input frontier",
    "read observed-window MACB_MID context only",
    "read NCR and require MPE bit 4 already set",
    "if MPE is clear classify source-contract-violated-blocker without writing",
    "poll NSR.IDLE bit 2 before each MAN transaction",
    "write MAN frame 0x600a0000 for Clause 22 phy1 register 0x02",
    "poll NSR.IDLE bit 2 and extract MAN.DATA bits 15:0 as physid1",
    "write MAN frame 0x600e0000 for Clause 22 phy1 register 0x03",
    "poll NSR.IDLE bit 2 and extract MAN.DATA bits 15:0 as physid2",
    "classify only the selected MDIO PHY-ID discriminator outcome",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_PRECONDITIONS: &[&str] = &[
    "observed-window MACB_MID identity remains context only",
    "NCR.MPE bit 4 must already be set before any candidate MAN write",
    "no NCR MPE write is allowed by the first proof contract",
    "initial NSR.IDLE bit 2 poll must pass before each MAN write",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_ALLOWED_CLASSIFICATIONS: &[&str] = &[
    "mdio-phy1-physid-visible",
    "mdio-phy1-physid-timeout",
    "mdio-phy1-physid-source-contract-violated-blocker",
    "precise-staging-capture-blocker",
    "no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "Ethernet driver readiness",
    "broad Ethernet MMIO readiness beyond selected observed-window targets",
    "MDIO ownership beyond selected PHY-ID discriminator",
    "NCR.MPE ownership or write permission",
    "PHY reset ownership",
    "GPIO32 ownership, event clearing, write/restore retry, or reset success",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_RETAINED_RISKS: &[&str] = &[
    "NCR.MPE may be clear and must block the first candidate without a write",
    "GPIO32 and ETH_RST_N remain unowned and may leave the PHY reset state unresolved",
    "A visible PHY ID read proves only the selected management transaction",
    "Link, MAC, DMA, interrupt, packet, socket, SSH, and Phase 12.2 readiness remain unaccepted",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-clock-reset-prereq-closeout.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout.md",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h",
];

pub const RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-mdio-mpe-enable-source-contract-20260611";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_GUARD_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-mpe-enable-guard-report-contract-v1";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_SELECTED_OPERATION: &str =
    "rp1-ethernet-mdio-mpe-enable-set-readback-restore";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-mdio-mpe-enable-guard-candidate-local-static";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_CONTROL_CLASSIFICATION: &str =
    "no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-ncr-mpe-enable-write-restore-control-output";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_REGISTER: &str = "MACB/GEM NCR";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_OFFSET: u64 = 0x0010_0000;
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_NCR_OFFSET: u64 = 0x0000;
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_OBSERVED_TARGET: u64 =
    RP1_ETHERNET_OBSERVED_RP1_BASE + RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_OFFSET;
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_WIDTH_BITS: u32 = 32;
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_MPE_BIT: u8 = 4;
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_MPE_MASK: u32 = 1 << RP1_ETHERNET_MDIO_MPE_ENABLE_MPE_BIT;
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_WRITE_RULE: &str = "pre_raw | 0x00000010";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_RESTORE_INVARIANT: &str = "restore_raw must equal pre_raw";
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_OPERATION_ORDER: &[&str] = &[
    "print candidate start marker and accepted input frontier",
    "read observed-window MACB_MID context only",
    "pre-read MACB/GEM NCR at 0x1c00100000 and retain pre_raw",
    "write pre_raw | 0x00000010 to MACB/GEM NCR",
    "read back MACB/GEM NCR and retain post_raw",
    "restore-write pre_raw to MACB/GEM NCR",
    "restore-read MACB/GEM NCR and require restore_raw equals pre_raw",
    "classify only NCR.MPE write/readback/restore ownership",
];
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_ALLOWED_CLASSIFICATIONS: &[&str] = &[
    "rp1-ethernet-mdio-mpe-enable-set-readback-restored",
    "rp1-ethernet-mdio-mpe-enable-already-set-restored",
    "rp1-ethernet-mdio-mpe-enable-readback-mismatch-restored",
    "rp1-ethernet-mdio-mpe-enable-restore-failed",
    "rp1-ethernet-mdio-mpe-enable-blocked-target-inconclusive",
    "precise-staging-capture-blocker",
    "no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control",
];
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "MAN writes",
    "PHY-ID reads",
    "broad MDIO or PHY ownership",
    "PHY reset ownership",
    "GPIO32 ownership or action",
    "Ethernet driver readiness",
    "broad Ethernet MMIO readiness beyond selected NCR target",
    "DMA or descriptor ownership",
    "interrupt completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_RETAINED_RISKS: &[&str] = &[
    "The guard surface is local/static and does not prove the NCR target is hardware-visible",
    "NCR.MPE set/readback/restore may still block on Pi 5 target visibility or restore evidence",
    "MPE ownership does not prove MAN transaction, PHY-ID, PHY reset, link, MAC, DMA, interrupt, packet, socket, SSH, or Phase 12.2 readiness",
    "Same-shaped MDIO PHY-ID retries remain closed until NCR.MPE write/restore ownership is separately accepted",
];
pub const RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-source-contract.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout.md",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-source-contract/classification.json",
    "tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub selected_discriminator: &'static str,
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub phy_handle: &'static str,
    pub phy_node: &'static str,
    pub phy_address: u32,
    pub phy_id_register_names: &'static [&'static str],
    pub phy_id_registers: &'static [u32],
    pub observed_identity_target: u64,
    pub translated_comparator_target: u64,
    pub ncr_register: &'static str,
    pub nsr_register: &'static str,
    pub man_register: &'static str,
    pub ncr_offset: u64,
    pub nsr_offset: u64,
    pub man_offset: u64,
    pub ncr_observed_target: u64,
    pub nsr_observed_target: u64,
    pub man_observed_target: u64,
    pub ncr_mpe_bit: u8,
    pub nsr_idle_bit: u8,
    pub man_data_offset: u8,
    pub man_data_size: u8,
    pub man_c22_sof: u32,
    pub man_c22_read: u32,
    pub man_c22_write: u32,
    pub man_code: u32,
    pub physid1_man_frame: u32,
    pub physid2_man_frame: u32,
    pub timeout_policy: &'static str,
    pub result_extraction: &'static str,
    pub no_restore_expectation: &'static str,
    pub preconditions: &'static [&'static str],
    pub operation_order: &'static [&'static str],
    pub allowed_classifications: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioPhyIdGuardReportKind {
    Candidate,
    NoMdioNoEthernetControl,
}

impl Rp1EthernetMdioPhyIdGuardReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoMdioNoEthernetControl => "no-mdio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdGuardReportInput {
    pub kind: Rp1EthernetMdioPhyIdGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioPhyIdSourceContractEvidence>,
    pub claims_runtime_mdio_transaction: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_mpe_write_permission: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdGuardReport {
    pub kind: Rp1EthernetMdioPhyIdGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioPhyIdSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdGuardReportEvidence {
    pub report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub selected_discriminator: Option<&'static str>,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub phy_handle: Option<&'static str>,
    pub phy_node: Option<&'static str>,
    pub phy_address: Option<u32>,
    pub phy_id_register_names: Option<&'static [&'static str]>,
    pub phy_id_registers: Option<&'static [u32]>,
    pub observed_identity_target: Option<u64>,
    pub translated_comparator_target: Option<u64>,
    pub ncr_register: Option<&'static str>,
    pub nsr_register: Option<&'static str>,
    pub man_register: Option<&'static str>,
    pub ncr_offset: Option<u64>,
    pub nsr_offset: Option<u64>,
    pub man_offset: Option<u64>,
    pub ncr_observed_target: Option<u64>,
    pub nsr_observed_target: Option<u64>,
    pub man_observed_target: Option<u64>,
    pub ncr_mpe_bit: Option<u8>,
    pub nsr_idle_bit: Option<u8>,
    pub man_data_offset: Option<u8>,
    pub man_data_size: Option<u8>,
    pub man_c22_sof: Option<u32>,
    pub man_c22_read: Option<u32>,
    pub man_c22_write: Option<u32>,
    pub man_code: Option<u32>,
    pub physid1_man_frame: Option<u32>,
    pub physid2_man_frame: Option<u32>,
    pub timeout_policy: Option<&'static str>,
    pub result_extraction: Option<&'static str>,
    pub no_restore_expectation: Option<&'static str>,
    pub preconditions: Option<&'static [&'static str]>,
    pub operation_order: Option<&'static [&'static str]>,
    pub allowed_classifications: &'static [&'static str],
    pub boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub claims_runtime_mdio_transaction: bool,
    pub claims_mdio_phy_ownership: bool,
    pub claims_mpe_write_permission: bool,
    pub claims_gpio32_phy_reset_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioPhyIdGuardReportError {
    CandidateMissingSourceContract,
    ControlCarriesMdioTargetFacts,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    RuntimeMdioTransactionClaim,
    MdioPhyOwnershipClaim,
    MpeWritePermissionClaim,
    Gpio32PhyResetOwnershipClaim,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    InterruptCompletionClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetMdioPhyIdGuardReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesMdioTargetFacts => "control-carries-mdio-target-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::RuntimeMdioTransactionClaim => "runtime-mdio-transaction-claim",
            Self::MdioPhyOwnershipClaim => "mdio-phy-ownership-claim",
            Self::MpeWritePermissionClaim => "mpe-write-permission-claim",
            Self::Gpio32PhyResetOwnershipClaim => "gpio32-phy-reset-ownership-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_mdio_phy_id_source_contract_evidence()
-> Rp1EthernetMdioPhyIdSourceContractEvidence {
    Rp1EthernetMdioPhyIdSourceContractEvidence {
        contract_id: RP1_ETHERNET_MDIO_PHY_ID_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_PHY_ID_SOURCE_TASK_ID,
        selected_discriminator: RP1_ETHERNET_MDIO_PHY_ID_SELECTED_DISCRIMINATOR,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        phy_handle: RP1_ETHERNET_PHY_HANDLE,
        phy_node: RP1_ETHERNET_PHY_NODE,
        phy_address: RP1_ETHERNET_PHY_REG,
        phy_id_register_names: RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTER_NAMES,
        phy_id_registers: RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS,
        observed_identity_target: RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET,
        translated_comparator_target: RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET,
        ncr_register: RP1_ETHERNET_MDIO_PHY_ID_NCR_REGISTER,
        nsr_register: RP1_ETHERNET_MDIO_PHY_ID_NSR_REGISTER,
        man_register: RP1_ETHERNET_MDIO_PHY_ID_MAN_REGISTER,
        ncr_offset: RP1_ETHERNET_MDIO_PHY_ID_NCR_OFFSET,
        nsr_offset: RP1_ETHERNET_MDIO_PHY_ID_NSR_OFFSET,
        man_offset: RP1_ETHERNET_MDIO_PHY_ID_MAN_OFFSET,
        ncr_observed_target: RP1_ETHERNET_MDIO_PHY_ID_NCR_OBSERVED_TARGET,
        nsr_observed_target: RP1_ETHERNET_MDIO_PHY_ID_NSR_OBSERVED_TARGET,
        man_observed_target: RP1_ETHERNET_MDIO_PHY_ID_MAN_OBSERVED_TARGET,
        ncr_mpe_bit: RP1_ETHERNET_MDIO_PHY_ID_NCR_MPE_BIT,
        nsr_idle_bit: RP1_ETHERNET_MDIO_PHY_ID_NSR_IDLE_BIT,
        man_data_offset: RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_OFFSET,
        man_data_size: RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_SIZE,
        man_c22_sof: RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_SOF,
        man_c22_read: RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_READ,
        man_c22_write: RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_WRITE,
        man_code: RP1_ETHERNET_MDIO_PHY_ID_MAN_CODE,
        physid1_man_frame: RP1_ETHERNET_MDIO_PHY_ID_PHYSID1_MAN_FRAME,
        physid2_man_frame: RP1_ETHERNET_MDIO_PHY_ID_PHYSID2_MAN_FRAME,
        timeout_policy: RP1_ETHERNET_MDIO_PHY_ID_TIMEOUT_POLICY,
        result_extraction: RP1_ETHERNET_MDIO_PHY_ID_RESULT_EXTRACTION,
        no_restore_expectation: RP1_ETHERNET_MDIO_PHY_ID_NO_RESTORE_EXPECTATION,
        preconditions: RP1_ETHERNET_MDIO_PHY_ID_PRECONDITIONS,
        operation_order: RP1_ETHERNET_MDIO_PHY_ID_OPERATION_ORDER,
        allowed_classifications: RP1_ETHERNET_MDIO_PHY_ID_ALLOWED_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_MDIO_PHY_ID_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_mdio_phy_id_guard_report(
    input: Rp1EthernetMdioPhyIdGuardReportInput,
) -> Result<Rp1EthernetMdioPhyIdGuardReport, Rp1EthernetMdioPhyIdGuardReportError> {
    validate_rp1_ethernet_mdio_phy_id_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetMdioPhyIdGuardReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_mdio_phy_id_source_contract(source_contract)?;
            Ok(Rp1EthernetMdioPhyIdGuardReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetMdioPhyIdGuardReportKind::Candidate, None) => {
            Err(Rp1EthernetMdioPhyIdGuardReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetMdioPhyIdGuardReportKind::NoMdioNoEthernetControl, None) => {
            Ok(Rp1EthernetMdioPhyIdGuardReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetMdioPhyIdGuardReportKind::NoMdioNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetMdioPhyIdGuardReportError::ControlCarriesMdioTargetFacts)
        }
    }
}

pub fn rp1_ethernet_mdio_phy_id_guard_report_evidence(
    report: Rp1EthernetMdioPhyIdGuardReport,
) -> Rp1EthernetMdioPhyIdGuardReportEvidence {
    match report.source_contract {
        Some(source_contract) => {
            rp1_ethernet_mdio_phy_id_guard_candidate_evidence(report.kind.name(), source_contract)
        }
        None => rp1_ethernet_mdio_phy_id_guard_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_mdio_phy_id_guard_report_evidence(
    error: Rp1EthernetMdioPhyIdGuardReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_mdio_phy_id_rejected_claims(
    input: Rp1EthernetMdioPhyIdGuardReportInput,
) -> Result<(), Rp1EthernetMdioPhyIdGuardReportError> {
    if input.claims_runtime_mdio_transaction {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::RuntimeMdioTransactionClaim);
    }
    if input.claims_mdio_phy_ownership {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::MdioPhyOwnershipClaim);
    }
    if input.claims_mpe_write_permission {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::MpeWritePermissionClaim);
    }
    if input.claims_gpio32_phy_reset_ownership {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::Gpio32PhyResetOwnershipClaim);
    }
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::BroadMmioReadinessClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::InterruptCompletionClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_mdio_phy_id_source_contract(
    source_contract: Rp1EthernetMdioPhyIdSourceContractEvidence,
) -> Result<(), Rp1EthernetMdioPhyIdGuardReportError> {
    if source_contract.contract_id != RP1_ETHERNET_MDIO_PHY_ID_SOURCE_CONTRACT_ID
        || source_contract.source_task_id != RP1_ETHERNET_MDIO_PHY_ID_SOURCE_TASK_ID
        || source_contract.selected_discriminator != RP1_ETHERNET_MDIO_PHY_ID_SELECTED_DISCRIMINATOR
        || source_contract.controller != RP1_ETHERNET_CONTROLLER_NAME
        || source_contract.compatible != RP1_ETHERNET_COMPATIBLE
        || source_contract.phy_handle != RP1_ETHERNET_PHY_HANDLE
        || source_contract.phy_node != RP1_ETHERNET_PHY_NODE
        || source_contract.phy_address != RP1_ETHERNET_PHY_REG
    {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::SourceContractIdentityMismatch);
    }
    if source_contract.observed_identity_target
        != RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET
        || source_contract.translated_comparator_target != RP1_ETHERNET_GEM_MID_CPU_PHYSICAL_TARGET
        || source_contract.ncr_observed_target != RP1_ETHERNET_MDIO_PHY_ID_NCR_OBSERVED_TARGET
        || source_contract.nsr_observed_target != RP1_ETHERNET_MDIO_PHY_ID_NSR_OBSERVED_TARGET
        || source_contract.man_observed_target != RP1_ETHERNET_MDIO_PHY_ID_MAN_OBSERVED_TARGET
        || source_contract.ncr_offset != RP1_ETHERNET_MDIO_PHY_ID_NCR_OFFSET
        || source_contract.nsr_offset != RP1_ETHERNET_MDIO_PHY_ID_NSR_OFFSET
        || source_contract.man_offset != RP1_ETHERNET_MDIO_PHY_ID_MAN_OFFSET
    {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::SourceContractTargetMismatch);
    }
    if source_contract.phy_id_register_names != RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTER_NAMES
        || source_contract.phy_id_registers != RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS
        || source_contract.ncr_register != RP1_ETHERNET_MDIO_PHY_ID_NCR_REGISTER
        || source_contract.nsr_register != RP1_ETHERNET_MDIO_PHY_ID_NSR_REGISTER
        || source_contract.man_register != RP1_ETHERNET_MDIO_PHY_ID_MAN_REGISTER
        || source_contract.ncr_mpe_bit != RP1_ETHERNET_MDIO_PHY_ID_NCR_MPE_BIT
        || source_contract.nsr_idle_bit != RP1_ETHERNET_MDIO_PHY_ID_NSR_IDLE_BIT
        || source_contract.man_data_offset != RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_OFFSET
        || source_contract.man_data_size != RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_SIZE
        || source_contract.man_c22_sof != RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_SOF
        || source_contract.man_c22_read != RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_READ
        || source_contract.man_c22_write != RP1_ETHERNET_MDIO_PHY_ID_MAN_C22_WRITE
        || source_contract.man_code != RP1_ETHERNET_MDIO_PHY_ID_MAN_CODE
        || source_contract.physid1_man_frame != RP1_ETHERNET_MDIO_PHY_ID_PHYSID1_MAN_FRAME
        || source_contract.physid2_man_frame != RP1_ETHERNET_MDIO_PHY_ID_PHYSID2_MAN_FRAME
        || source_contract.timeout_policy != RP1_ETHERNET_MDIO_PHY_ID_TIMEOUT_POLICY
        || source_contract.result_extraction != RP1_ETHERNET_MDIO_PHY_ID_RESULT_EXTRACTION
        || source_contract.no_restore_expectation != RP1_ETHERNET_MDIO_PHY_ID_NO_RESTORE_EXPECTATION
        || source_contract.preconditions != RP1_ETHERNET_MDIO_PHY_ID_PRECONDITIONS
        || source_contract.operation_order != RP1_ETHERNET_MDIO_PHY_ID_OPERATION_ORDER
        || source_contract.allowed_classifications
            != RP1_ETHERNET_MDIO_PHY_ID_ALLOWED_CLASSIFICATIONS
    {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::SourceContractFieldMismatch);
    }
    if source_contract.source_evidence != RP1_ETHERNET_MDIO_PHY_ID_SOURCE_EVIDENCE {
        return Err(Rp1EthernetMdioPhyIdGuardReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_mdio_phy_id_guard_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetMdioPhyIdSourceContractEvidence,
) -> Rp1EthernetMdioPhyIdGuardReportEvidence {
    Rp1EthernetMdioPhyIdGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_PHY_ID_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        report_kind,
        selected_discriminator: Some(source_contract.selected_discriminator),
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        phy_handle: Some(source_contract.phy_handle),
        phy_node: Some(source_contract.phy_node),
        phy_address: Some(source_contract.phy_address),
        phy_id_register_names: Some(source_contract.phy_id_register_names),
        phy_id_registers: Some(source_contract.phy_id_registers),
        observed_identity_target: Some(source_contract.observed_identity_target),
        translated_comparator_target: Some(source_contract.translated_comparator_target),
        ncr_register: Some(source_contract.ncr_register),
        nsr_register: Some(source_contract.nsr_register),
        man_register: Some(source_contract.man_register),
        ncr_offset: Some(source_contract.ncr_offset),
        nsr_offset: Some(source_contract.nsr_offset),
        man_offset: Some(source_contract.man_offset),
        ncr_observed_target: Some(source_contract.ncr_observed_target),
        nsr_observed_target: Some(source_contract.nsr_observed_target),
        man_observed_target: Some(source_contract.man_observed_target),
        ncr_mpe_bit: Some(source_contract.ncr_mpe_bit),
        nsr_idle_bit: Some(source_contract.nsr_idle_bit),
        man_data_offset: Some(source_contract.man_data_offset),
        man_data_size: Some(source_contract.man_data_size),
        man_c22_sof: Some(source_contract.man_c22_sof),
        man_c22_read: Some(source_contract.man_c22_read),
        man_c22_write: Some(source_contract.man_c22_write),
        man_code: Some(source_contract.man_code),
        physid1_man_frame: Some(source_contract.physid1_man_frame),
        physid2_man_frame: Some(source_contract.physid2_man_frame),
        timeout_policy: Some(source_contract.timeout_policy),
        result_extraction: Some(source_contract.result_extraction),
        no_restore_expectation: Some(source_contract.no_restore_expectation),
        preconditions: Some(source_contract.preconditions),
        operation_order: Some(source_contract.operation_order),
        allowed_classifications: RP1_ETHERNET_MDIO_PHY_ID_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_PHY_ID_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_PHY_ID_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_PHY_ID_RETAINED_RISKS,
        source_evidence: Some(source_contract.source_evidence),
        claims_runtime_mdio_transaction: false,
        claims_mdio_phy_ownership: false,
        claims_mpe_write_permission: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_PHY_ID_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_mdio_phy_id_guard_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetMdioPhyIdGuardReportEvidence {
    Rp1EthernetMdioPhyIdGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_PHY_ID_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_MDIO_PHY_ID_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_PHY_ID_SOURCE_TASK_ID,
        report_kind,
        selected_discriminator: None,
        controller: None,
        compatible: None,
        phy_handle: None,
        phy_node: None,
        phy_address: None,
        phy_id_register_names: None,
        phy_id_registers: None,
        observed_identity_target: None,
        translated_comparator_target: None,
        ncr_register: None,
        nsr_register: None,
        man_register: None,
        ncr_offset: None,
        nsr_offset: None,
        man_offset: None,
        ncr_observed_target: None,
        nsr_observed_target: None,
        man_observed_target: None,
        ncr_mpe_bit: None,
        nsr_idle_bit: None,
        man_data_offset: None,
        man_data_size: None,
        man_c22_sof: None,
        man_c22_read: None,
        man_c22_write: None,
        man_code: None,
        physid1_man_frame: None,
        physid2_man_frame: None,
        timeout_policy: None,
        result_extraction: None,
        no_restore_expectation: None,
        preconditions: None,
        operation_order: None,
        allowed_classifications: RP1_ETHERNET_MDIO_PHY_ID_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_PHY_ID_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_PHY_ID_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_PHY_ID_RETAINED_RISKS,
        source_evidence: None,
        claims_runtime_mdio_transaction: false,
        claims_mdio_phy_ownership: false,
        claims_mpe_write_permission: false,
        claims_gpio32_phy_reset_ownership: false,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_PHY_ID_CONTROL_CLASSIFICATION,
    }
}

pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-v1";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-20260611";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_GUARD_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-report-contract-v1";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SELECTED_DISCRIMINATOR: &str =
    "rp1-ethernet-mdio-after-mpe-clause22-phy1-physid1-physid2";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ACCEPTED_MPE_FRONTIER_TASK_ID: &str =
    "phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout-20260611";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ACCEPTED_MPE_FRONTIER_CLASSIFICATION: &str =
    "rp1-ethernet-mdio-mpe-enable-ownership-frontier-closed";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-mdio-phy-id-after-mpe-guard-candidate-local-static";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_CONTROL_CLASSIFICATION: &str =
    "no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-corrected-target-mdio-phy-id-after-mpe-control-output";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OBSERVED_WINDOW_BASE: u64 = 0x1c_0010_0000;
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NCR_OBSERVED_TARGET: u64 =
    RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OBSERVED_WINDOW_BASE + RP1_ETHERNET_MDIO_PHY_ID_NCR_OFFSET;
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NSR_OBSERVED_TARGET: u64 =
    RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OBSERVED_WINDOW_BASE + RP1_ETHERNET_MDIO_PHY_ID_NSR_OFFSET;
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MAN_OBSERVED_TARGET: u64 =
    RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OBSERVED_WINDOW_BASE + RP1_ETHERNET_MDIO_PHY_ID_MAN_OFFSET;
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_POLL_POLICY: &str =
    "bounded NSR.IDLE bit 2 poll before and after each MAN write";
pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MPE_PRECONDITION: &str =
    "corrected NCR.MPE bit 4 must already be set before any MAN write";

pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OPERATION_ORDER: &[&str] = &[
    "print candidate start marker and accepted input frontier",
    "read observed-window MACB_MID context at 0x1c001000fc as context only",
    "pre-read corrected NCR at 0x1c00100000",
    "if corrected NCR.MPE bit 4 is clear classify a precondition blocker without writing",
    "poll corrected NSR.IDLE bit 2 before each MAN write",
    "write corrected MAN frame 0x600a0000 for Clause 22 phy1 register 0x02",
    "poll corrected NSR.IDLE bit 2 and extract MAN.DATA bits 15:0 as physid1",
    "write corrected MAN frame 0x600e0000 for Clause 22 phy1 register 0x03",
    "poll corrected NSR.IDLE bit 2 and extract MAN.DATA bits 15:0 as physid2",
    "classify only the corrected-target after-MPE discriminator outcome",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ALLOWED_CLASSIFICATIONS: &[&str] = &[
    "mdio-phy1-physid-after-mpe-visible",
    "mdio-phy1-physid-after-mpe-timeout",
    "mdio-phy1-physid-after-mpe-precondition-blocker",
    "precise-staging-capture-blocker",
    "no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "NCR write permission or execution",
    "MAN write without corrected NCR.MPE precondition",
    "wrong 0x1c00000000-era MDIO register targets",
    "unbounded NSR.IDLE polling",
    "runtime volatile load/store evidence from local/static guard",
    "broad MDIO or PHY ownership",
    "PHY reset or GPIO32 action",
    "Ethernet driver readiness",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_RETAINED_RISKS: &[&str] = &[
    "Corrected NCR.MPE may be clear in the selected boot state and must block MAN writes",
    "Visible PHY ID reads would prove only selected management transaction visibility",
    "GPIO32 and ETH_RST_N remain unowned",
    "Link, MAC, DMA, interrupt, packet, socket, SSH, and Phase 12.2 readiness remain unaccepted",
];

pub const RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout.md",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract/classification.json",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/classification.json",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub selected_discriminator: &'static str,
    pub accepted_mpe_frontier_task_id: &'static str,
    pub accepted_mpe_frontier_classification: &'static str,
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub phy_handle: &'static str,
    pub phy_node: &'static str,
    pub phy_address: u32,
    pub macb_mid_context_target: u64,
    pub macb_mid_context_raw: u32,
    pub ncr_register: &'static str,
    pub nsr_register: &'static str,
    pub man_register: &'static str,
    pub ncr_observed_target: u64,
    pub nsr_observed_target: u64,
    pub man_observed_target: u64,
    pub phy_id_register_names: &'static [&'static str],
    pub phy_id_registers: &'static [u32],
    pub physid1_man_frame: u32,
    pub physid2_man_frame: u32,
    pub ncr_mpe_bit: u8,
    pub nsr_idle_bit: u8,
    pub poll_policy: &'static str,
    pub mpe_precondition: &'static str,
    pub man_data_offset: u8,
    pub man_data_size: u8,
    pub result_extraction: &'static str,
    pub operation_order: &'static [&'static str],
    pub allowed_classifications: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioPhyIdAfterMpeGuardReportKind {
    Candidate,
    NoMdioNoEthernetControl,
}

impl Rp1EthernetMdioPhyIdAfterMpeGuardReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoMdioNoEthernetControl => "no-mdio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
    pub kind: Rp1EthernetMdioPhyIdAfterMpeGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence>,
    pub claims_runtime_volatile_load_store: bool,
    pub claims_ncr_write: bool,
    pub claims_missing_mpe_gating: bool,
    pub claims_wrong_mdio_targets: bool,
    pub claims_unbounded_polling: bool,
    pub claims_man_write_without_mpe_precondition: bool,
    pub claims_broad_mdio_phy_ownership: bool,
    pub claims_gpio32_phy_reset_action: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdAfterMpeGuardReport {
    pub kind: Rp1EthernetMdioPhyIdAfterMpeGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioPhyIdAfterMpeGuardReportEvidence {
    pub report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub selected_discriminator: Option<&'static str>,
    pub accepted_mpe_frontier_task_id: Option<&'static str>,
    pub accepted_mpe_frontier_classification: Option<&'static str>,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub phy_handle: Option<&'static str>,
    pub phy_node: Option<&'static str>,
    pub phy_address: Option<u32>,
    pub macb_mid_context_target: Option<u64>,
    pub macb_mid_context_raw: Option<u32>,
    pub ncr_register: Option<&'static str>,
    pub nsr_register: Option<&'static str>,
    pub man_register: Option<&'static str>,
    pub ncr_observed_target: Option<u64>,
    pub nsr_observed_target: Option<u64>,
    pub man_observed_target: Option<u64>,
    pub phy_id_register_names: Option<&'static [&'static str]>,
    pub phy_id_registers: Option<&'static [u32]>,
    pub physid1_man_frame: Option<u32>,
    pub physid2_man_frame: Option<u32>,
    pub ncr_mpe_bit: Option<u8>,
    pub nsr_idle_bit: Option<u8>,
    pub poll_policy: Option<&'static str>,
    pub mpe_precondition: Option<&'static str>,
    pub man_data_offset: Option<u8>,
    pub man_data_size: Option<u8>,
    pub result_extraction: Option<&'static str>,
    pub operation_order: Option<&'static [&'static str]>,
    pub allowed_classifications: &'static [&'static str],
    pub boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub constructs_mdio_targets: bool,
    pub constructs_man_frames: bool,
    pub runtime_volatile_load_store_intent: bool,
    pub claims_runtime_volatile_load_store: bool,
    pub claims_ncr_write: bool,
    pub claims_missing_mpe_gating: bool,
    pub claims_wrong_mdio_targets: bool,
    pub claims_unbounded_polling: bool,
    pub claims_man_write_without_mpe_precondition: bool,
    pub claims_broad_mdio_phy_ownership: bool,
    pub claims_gpio32_phy_reset_action: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioPhyIdAfterMpeGuardReportError {
    CandidateMissingSourceContract,
    ControlCarriesMdioTargetFacts,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    RuntimeVolatileLoadStoreClaim,
    NcrWriteClaim,
    MissingMpeGatingClaim,
    WrongMdioTargetsClaim,
    UnboundedPollingClaim,
    ManWriteWithoutMpePreconditionClaim,
    BroadMdioPhyOwnershipClaim,
    Gpio32PhyResetActionClaim,
    EthernetReadinessClaim,
    InterruptCompletionClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetMdioPhyIdAfterMpeGuardReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesMdioTargetFacts => "control-carries-mdio-target-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::RuntimeVolatileLoadStoreClaim => "runtime-volatile-load-store-claim",
            Self::NcrWriteClaim => "ncr-write-claim",
            Self::MissingMpeGatingClaim => "missing-mpe-gating-claim",
            Self::WrongMdioTargetsClaim => "wrong-mdio-targets-claim",
            Self::UnboundedPollingClaim => "unbounded-polling-claim",
            Self::ManWriteWithoutMpePreconditionClaim => "man-write-without-mpe-precondition-claim",
            Self::BroadMdioPhyOwnershipClaim => "broad-mdio-phy-ownership-claim",
            Self::Gpio32PhyResetActionClaim => "gpio32-phy-reset-action-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_mdio_phy_id_after_mpe_source_contract_evidence()
-> Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence {
    Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence {
        contract_id: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_TASK_ID,
        selected_discriminator: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SELECTED_DISCRIMINATOR,
        accepted_mpe_frontier_task_id:
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ACCEPTED_MPE_FRONTIER_TASK_ID,
        accepted_mpe_frontier_classification:
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ACCEPTED_MPE_FRONTIER_CLASSIFICATION,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        phy_handle: RP1_ETHERNET_PHY_HANDLE,
        phy_node: RP1_ETHERNET_PHY_NODE,
        phy_address: RP1_ETHERNET_PHY_REG,
        macb_mid_context_target: RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET,
        macb_mid_context_raw: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW,
        ncr_register: RP1_ETHERNET_MDIO_PHY_ID_NCR_REGISTER,
        nsr_register: RP1_ETHERNET_MDIO_PHY_ID_NSR_REGISTER,
        man_register: RP1_ETHERNET_MDIO_PHY_ID_MAN_REGISTER,
        ncr_observed_target: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NCR_OBSERVED_TARGET,
        nsr_observed_target: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NSR_OBSERVED_TARGET,
        man_observed_target: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MAN_OBSERVED_TARGET,
        phy_id_register_names: RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTER_NAMES,
        phy_id_registers: RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS,
        physid1_man_frame: RP1_ETHERNET_MDIO_PHY_ID_PHYSID1_MAN_FRAME,
        physid2_man_frame: RP1_ETHERNET_MDIO_PHY_ID_PHYSID2_MAN_FRAME,
        ncr_mpe_bit: RP1_ETHERNET_MDIO_PHY_ID_NCR_MPE_BIT,
        nsr_idle_bit: RP1_ETHERNET_MDIO_PHY_ID_NSR_IDLE_BIT,
        poll_policy: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_POLL_POLICY,
        mpe_precondition: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MPE_PRECONDITION,
        man_data_offset: RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_OFFSET,
        man_data_size: RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_SIZE,
        result_extraction: RP1_ETHERNET_MDIO_PHY_ID_RESULT_EXTRACTION,
        operation_order: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OPERATION_ORDER,
        allowed_classifications: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ALLOWED_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
    input: Rp1EthernetMdioPhyIdAfterMpeGuardReportInput,
) -> Result<Rp1EthernetMdioPhyIdAfterMpeGuardReport, Rp1EthernetMdioPhyIdAfterMpeGuardReportError> {
    validate_rp1_ethernet_mdio_phy_id_after_mpe_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetMdioPhyIdAfterMpeGuardReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_mdio_phy_id_after_mpe_source_contract(source_contract)?;
            Ok(Rp1EthernetMdioPhyIdAfterMpeGuardReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetMdioPhyIdAfterMpeGuardReportKind::Candidate, None) => {
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetMdioPhyIdAfterMpeGuardReportKind::NoMdioNoEthernetControl, None) => {
            Ok(Rp1EthernetMdioPhyIdAfterMpeGuardReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetMdioPhyIdAfterMpeGuardReportKind::NoMdioNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::ControlCarriesMdioTargetFacts)
        }
    }
}

pub fn rp1_ethernet_mdio_phy_id_after_mpe_guard_report_evidence(
    report: Rp1EthernetMdioPhyIdAfterMpeGuardReport,
) -> Rp1EthernetMdioPhyIdAfterMpeGuardReportEvidence {
    match report.source_contract {
        Some(source_contract) => rp1_ethernet_mdio_phy_id_after_mpe_guard_candidate_evidence(
            report.kind.name(),
            source_contract,
        ),
        None => rp1_ethernet_mdio_phy_id_after_mpe_guard_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_mdio_phy_id_after_mpe_guard_report_evidence(
    error: Rp1EthernetMdioPhyIdAfterMpeGuardReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_mdio_phy_id_after_mpe_rejected_claims(
    input: Rp1EthernetMdioPhyIdAfterMpeGuardReportInput,
) -> Result<(), Rp1EthernetMdioPhyIdAfterMpeGuardReportError> {
    if input.claims_runtime_volatile_load_store {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::RuntimeVolatileLoadStoreClaim);
    }
    if input.claims_ncr_write {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::NcrWriteClaim);
    }
    if input.claims_missing_mpe_gating {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::MissingMpeGatingClaim);
    }
    if input.claims_wrong_mdio_targets {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::WrongMdioTargetsClaim);
    }
    if input.claims_unbounded_polling {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::UnboundedPollingClaim);
    }
    if input.claims_man_write_without_mpe_precondition {
        return Err(
            Rp1EthernetMdioPhyIdAfterMpeGuardReportError::ManWriteWithoutMpePreconditionClaim,
        );
    }
    if input.claims_broad_mdio_phy_ownership {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::BroadMdioPhyOwnershipClaim);
    }
    if input.claims_gpio32_phy_reset_action {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::Gpio32PhyResetActionClaim);
    }
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::EthernetReadinessClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::InterruptCompletionClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_mdio_phy_id_after_mpe_source_contract(
    source_contract: Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence,
) -> Result<(), Rp1EthernetMdioPhyIdAfterMpeGuardReportError> {
    if source_contract.contract_id != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_CONTRACT_ID
        || source_contract.source_task_id != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_TASK_ID
        || source_contract.selected_discriminator
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SELECTED_DISCRIMINATOR
        || source_contract.accepted_mpe_frontier_task_id
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ACCEPTED_MPE_FRONTIER_TASK_ID
        || source_contract.accepted_mpe_frontier_classification
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ACCEPTED_MPE_FRONTIER_CLASSIFICATION
        || source_contract.controller != RP1_ETHERNET_CONTROLLER_NAME
        || source_contract.compatible != RP1_ETHERNET_COMPATIBLE
        || source_contract.phy_handle != RP1_ETHERNET_PHY_HANDLE
        || source_contract.phy_node != RP1_ETHERNET_PHY_NODE
        || source_contract.phy_address != RP1_ETHERNET_PHY_REG
    {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SourceContractIdentityMismatch);
    }
    if source_contract.macb_mid_context_target
        != RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET
        || source_contract.macb_mid_context_raw != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW
        || source_contract.ncr_observed_target
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NCR_OBSERVED_TARGET
        || source_contract.nsr_observed_target
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NSR_OBSERVED_TARGET
        || source_contract.man_observed_target
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MAN_OBSERVED_TARGET
    {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SourceContractTargetMismatch);
    }
    if source_contract.ncr_register != RP1_ETHERNET_MDIO_PHY_ID_NCR_REGISTER
        || source_contract.nsr_register != RP1_ETHERNET_MDIO_PHY_ID_NSR_REGISTER
        || source_contract.man_register != RP1_ETHERNET_MDIO_PHY_ID_MAN_REGISTER
        || source_contract.phy_id_register_names != RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTER_NAMES
        || source_contract.phy_id_registers != RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS
        || source_contract.physid1_man_frame != RP1_ETHERNET_MDIO_PHY_ID_PHYSID1_MAN_FRAME
        || source_contract.physid2_man_frame != RP1_ETHERNET_MDIO_PHY_ID_PHYSID2_MAN_FRAME
        || source_contract.ncr_mpe_bit != RP1_ETHERNET_MDIO_PHY_ID_NCR_MPE_BIT
        || source_contract.nsr_idle_bit != RP1_ETHERNET_MDIO_PHY_ID_NSR_IDLE_BIT
        || source_contract.poll_policy != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_POLL_POLICY
        || source_contract.mpe_precondition != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MPE_PRECONDITION
        || source_contract.man_data_offset != RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_OFFSET
        || source_contract.man_data_size != RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_SIZE
        || source_contract.result_extraction != RP1_ETHERNET_MDIO_PHY_ID_RESULT_EXTRACTION
        || source_contract.operation_order != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OPERATION_ORDER
        || source_contract.allowed_classifications
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ALLOWED_CLASSIFICATIONS
    {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SourceContractFieldMismatch);
    }
    if source_contract.source_evidence != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_EVIDENCE {
        return Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_mdio_phy_id_after_mpe_guard_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence,
) -> Rp1EthernetMdioPhyIdAfterMpeGuardReportEvidence {
    Rp1EthernetMdioPhyIdAfterMpeGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        report_kind,
        selected_discriminator: Some(source_contract.selected_discriminator),
        accepted_mpe_frontier_task_id: Some(source_contract.accepted_mpe_frontier_task_id),
        accepted_mpe_frontier_classification: Some(
            source_contract.accepted_mpe_frontier_classification,
        ),
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        phy_handle: Some(source_contract.phy_handle),
        phy_node: Some(source_contract.phy_node),
        phy_address: Some(source_contract.phy_address),
        macb_mid_context_target: Some(source_contract.macb_mid_context_target),
        macb_mid_context_raw: Some(source_contract.macb_mid_context_raw),
        ncr_register: Some(source_contract.ncr_register),
        nsr_register: Some(source_contract.nsr_register),
        man_register: Some(source_contract.man_register),
        ncr_observed_target: Some(source_contract.ncr_observed_target),
        nsr_observed_target: Some(source_contract.nsr_observed_target),
        man_observed_target: Some(source_contract.man_observed_target),
        phy_id_register_names: Some(source_contract.phy_id_register_names),
        phy_id_registers: Some(source_contract.phy_id_registers),
        physid1_man_frame: Some(source_contract.physid1_man_frame),
        physid2_man_frame: Some(source_contract.physid2_man_frame),
        ncr_mpe_bit: Some(source_contract.ncr_mpe_bit),
        nsr_idle_bit: Some(source_contract.nsr_idle_bit),
        poll_policy: Some(source_contract.poll_policy),
        mpe_precondition: Some(source_contract.mpe_precondition),
        man_data_offset: Some(source_contract.man_data_offset),
        man_data_size: Some(source_contract.man_data_size),
        result_extraction: Some(source_contract.result_extraction),
        operation_order: Some(source_contract.operation_order),
        allowed_classifications: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_RETAINED_RISKS,
        source_evidence: Some(source_contract.source_evidence),
        constructs_mdio_targets: true,
        constructs_man_frames: true,
        runtime_volatile_load_store_intent: false,
        claims_runtime_volatile_load_store: false,
        claims_ncr_write: false,
        claims_missing_mpe_gating: false,
        claims_wrong_mdio_targets: false,
        claims_unbounded_polling: false,
        claims_man_write_without_mpe_precondition: false,
        claims_broad_mdio_phy_ownership: false,
        claims_gpio32_phy_reset_action: false,
        claims_ethernet_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_mdio_phy_id_after_mpe_guard_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetMdioPhyIdAfterMpeGuardReportEvidence {
    Rp1EthernetMdioPhyIdAfterMpeGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_TASK_ID,
        report_kind,
        selected_discriminator: None,
        accepted_mpe_frontier_task_id: None,
        accepted_mpe_frontier_classification: None,
        controller: None,
        compatible: None,
        phy_handle: None,
        phy_node: None,
        phy_address: None,
        macb_mid_context_target: None,
        macb_mid_context_raw: None,
        ncr_register: None,
        nsr_register: None,
        man_register: None,
        ncr_observed_target: None,
        nsr_observed_target: None,
        man_observed_target: None,
        phy_id_register_names: None,
        phy_id_registers: None,
        physid1_man_frame: None,
        physid2_man_frame: None,
        ncr_mpe_bit: None,
        nsr_idle_bit: None,
        poll_policy: None,
        mpe_precondition: None,
        man_data_offset: None,
        man_data_size: None,
        result_extraction: None,
        operation_order: None,
        allowed_classifications: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_RETAINED_RISKS,
        source_evidence: None,
        constructs_mdio_targets: false,
        constructs_man_frames: false,
        runtime_volatile_load_store_intent: false,
        claims_runtime_volatile_load_store: false,
        claims_ncr_write: false,
        claims_missing_mpe_gating: false,
        claims_wrong_mdio_targets: false,
        claims_unbounded_polling: false,
        claims_man_write_without_mpe_precondition: false,
        claims_broad_mdio_phy_ownership: false,
        claims_gpio32_phy_reset_action: false,
        claims_ethernet_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_CONTROL_CLASSIFICATION,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioMpeEnableSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub selected_operation: &'static str,
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub observed_identity_target: u64,
    pub register: &'static str,
    pub source_offset: u64,
    pub ncr_offset: u64,
    pub observed_target: u64,
    pub width_bits: u32,
    pub mpe_bit: u8,
    pub mpe_mask: u32,
    pub write_rule: &'static str,
    pub restore_invariant: &'static str,
    pub operation_order: &'static [&'static str],
    pub allowed_classifications: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioMpeEnableGuardReportKind {
    Candidate,
    NoMdioNoEthernetControl,
}

impl Rp1EthernetMdioMpeEnableGuardReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoMdioNoEthernetControl => "no-mdio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioMpeEnableGuardReportInput {
    pub kind: Rp1EthernetMdioMpeEnableGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioMpeEnableSourceContractEvidence>,
    pub claims_runtime_ncr_write_executed: bool,
    pub claims_man_write: bool,
    pub claims_phy_id_read: bool,
    pub claims_broad_mdio_phy_ownership: bool,
    pub claims_phy_reset_ownership: bool,
    pub claims_gpio32_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_interrupt_completion: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioMpeEnableGuardReport {
    pub kind: Rp1EthernetMdioMpeEnableGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioMpeEnableSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioMpeEnableGuardReportEvidence {
    pub report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub selected_operation: Option<&'static str>,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub observed_identity_target: Option<u64>,
    pub register: Option<&'static str>,
    pub source_offset: Option<u64>,
    pub ncr_offset: Option<u64>,
    pub observed_target: Option<u64>,
    pub width_bits: Option<u32>,
    pub mpe_bit: Option<u8>,
    pub mpe_mask: Option<u32>,
    pub write_rule: Option<&'static str>,
    pub restore_invariant: Option<&'static str>,
    pub operation_order: Option<&'static [&'static str]>,
    pub allowed_classifications: &'static [&'static str],
    pub boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub candidate_write_intent: bool,
    pub claims_runtime_ncr_write_executed: bool,
    pub claims_man_write: bool,
    pub claims_phy_id_read: bool,
    pub claims_broad_mdio_phy_ownership: bool,
    pub claims_phy_reset_ownership: bool,
    pub claims_gpio32_ownership: bool,
    pub claims_ethernet_ready: bool,
    pub claims_broad_mmio_ready: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_interrupt_completion: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioMpeEnableGuardReportError {
    CandidateMissingSourceContract,
    ControlCarriesNcrMpeTargetFacts,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    RuntimeNcrWriteExecutedClaim,
    ManWriteClaim,
    PhyIdReadClaim,
    BroadMdioPhyOwnershipClaim,
    PhyResetOwnershipClaim,
    Gpio32OwnershipClaim,
    EthernetReadinessClaim,
    BroadMmioReadinessClaim,
    DmaDescriptorOwnershipClaim,
    InterruptCompletionClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetMdioMpeEnableGuardReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesNcrMpeTargetFacts => "control-carries-ncr-mpe-target-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::RuntimeNcrWriteExecutedClaim => "runtime-ncr-write-executed-claim",
            Self::ManWriteClaim => "man-write-claim",
            Self::PhyIdReadClaim => "phy-id-read-claim",
            Self::BroadMdioPhyOwnershipClaim => "broad-mdio-phy-ownership-claim",
            Self::PhyResetOwnershipClaim => "phy-reset-ownership-claim",
            Self::Gpio32OwnershipClaim => "gpio32-ownership-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::BroadMmioReadinessClaim => "broad-mmio-readiness-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_mdio_mpe_enable_source_contract_evidence()
-> Rp1EthernetMdioMpeEnableSourceContractEvidence {
    Rp1EthernetMdioMpeEnableSourceContractEvidence {
        contract_id: RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_TASK_ID,
        selected_operation: RP1_ETHERNET_MDIO_MPE_ENABLE_SELECTED_OPERATION,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        observed_identity_target: RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET,
        register: RP1_ETHERNET_MDIO_MPE_ENABLE_REGISTER,
        source_offset: RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_OFFSET,
        ncr_offset: RP1_ETHERNET_MDIO_MPE_ENABLE_NCR_OFFSET,
        observed_target: RP1_ETHERNET_MDIO_MPE_ENABLE_OBSERVED_TARGET,
        width_bits: RP1_ETHERNET_MDIO_MPE_ENABLE_WIDTH_BITS,
        mpe_bit: RP1_ETHERNET_MDIO_MPE_ENABLE_MPE_BIT,
        mpe_mask: RP1_ETHERNET_MDIO_MPE_ENABLE_MPE_MASK,
        write_rule: RP1_ETHERNET_MDIO_MPE_ENABLE_WRITE_RULE,
        restore_invariant: RP1_ETHERNET_MDIO_MPE_ENABLE_RESTORE_INVARIANT,
        operation_order: RP1_ETHERNET_MDIO_MPE_ENABLE_OPERATION_ORDER,
        allowed_classifications: RP1_ETHERNET_MDIO_MPE_ENABLE_ALLOWED_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_mdio_mpe_enable_guard_report(
    input: Rp1EthernetMdioMpeEnableGuardReportInput,
) -> Result<Rp1EthernetMdioMpeEnableGuardReport, Rp1EthernetMdioMpeEnableGuardReportError> {
    validate_rp1_ethernet_mdio_mpe_enable_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetMdioMpeEnableGuardReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_mdio_mpe_enable_source_contract(source_contract)?;
            Ok(Rp1EthernetMdioMpeEnableGuardReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetMdioMpeEnableGuardReportKind::Candidate, None) => {
            Err(Rp1EthernetMdioMpeEnableGuardReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetMdioMpeEnableGuardReportKind::NoMdioNoEthernetControl, None) => {
            Ok(Rp1EthernetMdioMpeEnableGuardReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetMdioMpeEnableGuardReportKind::NoMdioNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetMdioMpeEnableGuardReportError::ControlCarriesNcrMpeTargetFacts)
        }
    }
}

pub fn rp1_ethernet_mdio_mpe_enable_guard_report_evidence(
    report: Rp1EthernetMdioMpeEnableGuardReport,
) -> Rp1EthernetMdioMpeEnableGuardReportEvidence {
    match report.source_contract {
        Some(source_contract) => rp1_ethernet_mdio_mpe_enable_guard_candidate_evidence(
            report.kind.name(),
            source_contract,
        ),
        None => rp1_ethernet_mdio_mpe_enable_guard_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_mdio_mpe_enable_guard_report_evidence(
    error: Rp1EthernetMdioMpeEnableGuardReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_mdio_mpe_enable_rejected_claims(
    input: Rp1EthernetMdioMpeEnableGuardReportInput,
) -> Result<(), Rp1EthernetMdioMpeEnableGuardReportError> {
    if input.claims_runtime_ncr_write_executed {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::RuntimeNcrWriteExecutedClaim);
    }
    if input.claims_man_write {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::ManWriteClaim);
    }
    if input.claims_phy_id_read {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::PhyIdReadClaim);
    }
    if input.claims_broad_mdio_phy_ownership {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::BroadMdioPhyOwnershipClaim);
    }
    if input.claims_phy_reset_ownership {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::PhyResetOwnershipClaim);
    }
    if input.claims_gpio32_ownership {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::Gpio32OwnershipClaim);
    }
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::EthernetReadinessClaim);
    }
    if input.claims_broad_mmio_ready {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::BroadMmioReadinessClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::InterruptCompletionClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_mdio_mpe_enable_source_contract(
    source_contract: Rp1EthernetMdioMpeEnableSourceContractEvidence,
) -> Result<(), Rp1EthernetMdioMpeEnableGuardReportError> {
    if source_contract.contract_id != RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_CONTRACT_ID
        || source_contract.source_task_id != RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_TASK_ID
        || source_contract.selected_operation != RP1_ETHERNET_MDIO_MPE_ENABLE_SELECTED_OPERATION
        || source_contract.controller != RP1_ETHERNET_CONTROLLER_NAME
        || source_contract.compatible != RP1_ETHERNET_COMPATIBLE
    {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::SourceContractIdentityMismatch);
    }
    if source_contract.observed_identity_target
        != RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET
        || source_contract.register != RP1_ETHERNET_MDIO_MPE_ENABLE_REGISTER
        || source_contract.source_offset != RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_OFFSET
        || source_contract.ncr_offset != RP1_ETHERNET_MDIO_MPE_ENABLE_NCR_OFFSET
        || source_contract.observed_target != RP1_ETHERNET_MDIO_MPE_ENABLE_OBSERVED_TARGET
    {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::SourceContractTargetMismatch);
    }
    if source_contract.width_bits != RP1_ETHERNET_MDIO_MPE_ENABLE_WIDTH_BITS
        || source_contract.mpe_bit != RP1_ETHERNET_MDIO_MPE_ENABLE_MPE_BIT
        || source_contract.mpe_mask != RP1_ETHERNET_MDIO_MPE_ENABLE_MPE_MASK
        || source_contract.write_rule != RP1_ETHERNET_MDIO_MPE_ENABLE_WRITE_RULE
        || source_contract.restore_invariant != RP1_ETHERNET_MDIO_MPE_ENABLE_RESTORE_INVARIANT
        || source_contract.operation_order != RP1_ETHERNET_MDIO_MPE_ENABLE_OPERATION_ORDER
        || source_contract.allowed_classifications
            != RP1_ETHERNET_MDIO_MPE_ENABLE_ALLOWED_CLASSIFICATIONS
    {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::SourceContractFieldMismatch);
    }
    if source_contract.source_evidence != RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_EVIDENCE {
        return Err(Rp1EthernetMdioMpeEnableGuardReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_mdio_mpe_enable_guard_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetMdioMpeEnableSourceContractEvidence,
) -> Rp1EthernetMdioMpeEnableGuardReportEvidence {
    Rp1EthernetMdioMpeEnableGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_MPE_ENABLE_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        report_kind,
        selected_operation: Some(source_contract.selected_operation),
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        observed_identity_target: Some(source_contract.observed_identity_target),
        register: Some(source_contract.register),
        source_offset: Some(source_contract.source_offset),
        ncr_offset: Some(source_contract.ncr_offset),
        observed_target: Some(source_contract.observed_target),
        width_bits: Some(source_contract.width_bits),
        mpe_bit: Some(source_contract.mpe_bit),
        mpe_mask: Some(source_contract.mpe_mask),
        write_rule: Some(source_contract.write_rule),
        restore_invariant: Some(source_contract.restore_invariant),
        operation_order: Some(source_contract.operation_order),
        allowed_classifications: RP1_ETHERNET_MDIO_MPE_ENABLE_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_MPE_ENABLE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_MPE_ENABLE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_MPE_ENABLE_RETAINED_RISKS,
        source_evidence: Some(source_contract.source_evidence),
        candidate_write_intent: true,
        claims_runtime_ncr_write_executed: false,
        claims_man_write: false,
        claims_phy_id_read: false,
        claims_broad_mdio_phy_ownership: false,
        claims_phy_reset_ownership: false,
        claims_gpio32_ownership: false,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_dma_descriptor_ownership: false,
        claims_interrupt_completion: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_MPE_ENABLE_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_mdio_mpe_enable_guard_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetMdioMpeEnableGuardReportEvidence {
    Rp1EthernetMdioMpeEnableGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_MPE_ENABLE_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_TASK_ID,
        report_kind,
        selected_operation: None,
        controller: None,
        compatible: None,
        observed_identity_target: None,
        register: None,
        source_offset: None,
        ncr_offset: None,
        observed_target: None,
        width_bits: None,
        mpe_bit: None,
        mpe_mask: None,
        write_rule: None,
        restore_invariant: None,
        operation_order: None,
        allowed_classifications: RP1_ETHERNET_MDIO_MPE_ENABLE_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_MPE_ENABLE_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_MPE_ENABLE_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_MPE_ENABLE_RETAINED_RISKS,
        source_evidence: None,
        candidate_write_intent: false,
        claims_runtime_ncr_write_executed: false,
        claims_man_write: false,
        claims_phy_id_read: false,
        claims_broad_mdio_phy_ownership: false,
        claims_phy_reset_ownership: false,
        claims_gpio32_ownership: false,
        claims_ethernet_ready: false,
        claims_broad_mmio_ready: false,
        claims_dma_descriptor_ownership: false,
        claims_interrupt_completion: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_MPE_ENABLE_CONTROL_CLASSIFICATION,
    }
}

pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-register-vector-source-contract-v1";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-mdio-register-vector-source-contract-20260611";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_GUARD_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-mdio-register-vector-guard-report-contract-v1";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_SELECTED_DISCRIMINATOR: &str =
    "rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_CANDIDATE_CLASSIFICATION: &str =
    "rp1-ethernet-mdio-register-vector-guard-candidate-local-static";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_CONTROL_CLASSIFICATION: &str =
    "no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_BOUNDARY_CLASSIFICATION: &str =
    "hardware-proof-limited-to-corrected-target-mdio-register-vector-control-output";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_PURPOSE: &str =
    "distinguish global all-ones/no-response behavior from PHY-ID-only evidence";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTER_NAMES: &[&str] = &[
    "MII_BMCR",
    "MII_BMSR",
    "MII_PHYSID1",
    "MII_PHYSID2",
    "MII_ADVERTISE / ANAR",
    "MII_LPA / ANLPAR",
];
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTERS: &[u32] =
    &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAMES: &[u32] = &[
    0x6082_0000,
    0x6086_0000,
    0x608a_0000,
    0x608e_0000,
    0x6092_0000,
    0x6096_0000,
];
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAME_CONSTRUCTION: &str =
    "(SOF 1 << 30) | (READ 2 << 28) | (PHYA 1 << 23) | (REGA register << 18) | (CODE 2 << 16)";
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_OPERATION_ORDER: &[&str] = &[
    "print candidate start marker and accepted input frontier",
    "read observed-window MACB_MID context at 0x1c001000fc as context only",
    "pre-read corrected NCR at 0x1c00100000",
    "if corrected NCR.MPE bit 4 is clear classify a precondition blocker without writing",
    "for each selected register, poll corrected NSR.IDLE bit 2 before the MAN write",
    "write the exact corrected MAN Clause 22 read frame for that register",
    "poll corrected NSR.IDLE bit 2 after the MAN write",
    "read corrected MAN and extract DATA[15:0] into the ordered six-entry vector",
    "classify only the selected register-vector discriminator outcome",
];
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_ALLOWED_CLASSIFICATIONS: &[&str] = &[
    "mdio-phy1-register-vector-visible",
    "mdio-phy1-register-vector-global-all-ones-visible",
    "mdio-phy1-register-vector-physid-only-all-ones-mixed-visible",
    "mdio-phy1-register-vector-timeout",
    "mdio-phy1-register-vector-precondition-blocker",
    "precise-staging-capture-blocker",
    "no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control",
];
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "runtime volatile load/store evidence from local/static guard",
    "NCR write permission or execution",
    "MAN write without corrected NCR.MPE precondition",
    "wrong 0x1c00000000-era MDIO register targets",
    "unbounded NSR.IDLE polling",
    "PHY absence from all-ones vector",
    "broad MDIO or PHY ownership",
    "PHY reset or GPIO32 action",
    "Ethernet driver readiness",
    "interrupt delivery, handler ownership, or completion",
    "DMA, descriptor rings, channel ownership, or transfer completion",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_RETAINED_RISKS: &[&str] = &[
    "Corrected NCR.MPE may be clear in the selected boot state and must block MAN writes",
    "A global all-ones vector does not prove PHY absence, reset state, link state, or usable Ethernet",
    "A mixed vector proves only selected MAN.DATA read visibility, not packet I/O or networking readiness",
    "GPIO32/ETH_RST_N, PHY reset state, DMA/descriptors, interrupts, sockets, SSH, and Phase 12.2 remain unaccepted",
];
pub const RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-source-contract.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof.md",
    "tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout.md",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-source-contract/classification.json",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/capture-summary.json",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h",
    "tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1StatusRawVector {
    pub bmcr: u16,
    pub bmsr: u16,
    pub physid1: u16,
    pub physid2: u16,
    pub anar: u16,
    pub anlpar: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1BmcrStatus {
    pub reset: bool,
    pub loopback: bool,
    pub speed_select_lsb_100: bool,
    pub autoneg_enable: bool,
    pub power_down: bool,
    pub isolate: bool,
    pub restart_autoneg: bool,
    pub duplex_full: bool,
    pub collision_test: bool,
    pub speed_select_msb_1000: bool,
    pub speed_label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1BmsrStatus {
    pub capable_100base_t4: bool,
    pub capable_100base_x_full: bool,
    pub capable_100base_x_half: bool,
    pub capable_10_full: bool,
    pub capable_10_half: bool,
    pub extended_status: bool,
    pub autoneg_complete: bool,
    pub remote_fault: bool,
    pub autoneg_ability: bool,
    pub link_status: bool,
    pub jabber_detect: bool,
    pub extended_capability: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1IdStatus {
    pub oui: u32,
    pub model: u8,
    pub revision: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1AdvertisementStatus {
    pub selector: u8,
    pub capable_10_half: bool,
    pub capable_10_full: bool,
    pub capable_100_tx_half: bool,
    pub capable_100_tx_full: bool,
    pub capable_100_t4: bool,
    pub pause: bool,
    pub asym_pause: bool,
    pub remote_fault: bool,
    pub acknowledge: bool,
    pub next_page: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1StatusDecode {
    pub raw: Rp1EthernetPhy1StatusRawVector,
    pub bmcr: Rp1EthernetPhy1BmcrStatus,
    pub bmsr: Rp1EthernetPhy1BmsrStatus,
    pub phy_id: Rp1EthernetPhy1IdStatus,
    pub anar: Rp1EthernetPhy1AdvertisementStatus,
    pub anlpar: Rp1EthernetPhy1AdvertisementStatus,
}

pub const fn decode_rp1_ethernet_phy1_status(
    raw: Rp1EthernetPhy1StatusRawVector,
) -> Rp1EthernetPhy1StatusDecode {
    Rp1EthernetPhy1StatusDecode {
        raw,
        bmcr: decode_rp1_ethernet_phy1_bmcr(raw.bmcr),
        bmsr: decode_rp1_ethernet_phy1_bmsr(raw.bmsr),
        phy_id: decode_rp1_ethernet_phy1_id(raw.physid1, raw.physid2),
        anar: decode_rp1_ethernet_phy1_advertisement(raw.anar),
        anlpar: decode_rp1_ethernet_phy1_advertisement(raw.anlpar),
    }
}

pub const fn decode_rp1_ethernet_phy1_bmcr(raw: u16) -> Rp1EthernetPhy1BmcrStatus {
    let speed_select_lsb_100 = raw & (1 << 13) != 0;
    let speed_select_msb_1000 = raw & (1 << 6) != 0;
    Rp1EthernetPhy1BmcrStatus {
        reset: raw & (1 << 15) != 0,
        loopback: raw & (1 << 14) != 0,
        speed_select_lsb_100,
        autoneg_enable: raw & (1 << 12) != 0,
        power_down: raw & (1 << 11) != 0,
        isolate: raw & (1 << 10) != 0,
        restart_autoneg: raw & (1 << 9) != 0,
        duplex_full: raw & (1 << 8) != 0,
        collision_test: raw & (1 << 7) != 0,
        speed_select_msb_1000,
        speed_label: match (speed_select_msb_1000, speed_select_lsb_100) {
            (false, false) => "10M",
            (false, true) => "100M",
            (true, false) => "1000M",
            (true, true) => "reserved",
        },
    }
}

pub const fn decode_rp1_ethernet_phy1_bmsr(raw: u16) -> Rp1EthernetPhy1BmsrStatus {
    Rp1EthernetPhy1BmsrStatus {
        capable_100base_t4: raw & (1 << 15) != 0,
        capable_100base_x_full: raw & (1 << 14) != 0,
        capable_100base_x_half: raw & (1 << 13) != 0,
        capable_10_full: raw & (1 << 12) != 0,
        capable_10_half: raw & (1 << 11) != 0,
        extended_status: raw & (1 << 8) != 0,
        autoneg_complete: raw & (1 << 5) != 0,
        remote_fault: raw & (1 << 4) != 0,
        autoneg_ability: raw & (1 << 3) != 0,
        link_status: raw & (1 << 2) != 0,
        jabber_detect: raw & (1 << 1) != 0,
        extended_capability: raw & 1 != 0,
    }
}

pub const fn decode_rp1_ethernet_phy1_id(physid1: u16, physid2: u16) -> Rp1EthernetPhy1IdStatus {
    Rp1EthernetPhy1IdStatus {
        oui: ((physid1 as u32) << 6) | (((physid2 as u32) >> 10) & 0x3f),
        model: ((physid2 >> 4) & 0x3f) as u8,
        revision: (physid2 & 0x0f) as u8,
    }
}

pub const fn decode_rp1_ethernet_phy1_advertisement(
    raw: u16,
) -> Rp1EthernetPhy1AdvertisementStatus {
    Rp1EthernetPhy1AdvertisementStatus {
        selector: (raw & 0x1f) as u8,
        capable_10_half: raw & (1 << 5) != 0,
        capable_10_full: raw & (1 << 6) != 0,
        capable_100_tx_half: raw & (1 << 7) != 0,
        capable_100_tx_full: raw & (1 << 8) != 0,
        capable_100_t4: raw & (1 << 9) != 0,
        pause: raw & (1 << 10) != 0,
        asym_pause: raw & (1 << 11) != 0,
        remote_fault: raw & (1 << 13) != 0,
        acknowledge: raw & (1 << 14) != 0,
        next_page: raw & (1 << 15) != 0,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioRegisterVectorSourceContractEvidence {
    pub contract_id: &'static str,
    pub source_task_id: &'static str,
    pub selected_discriminator: &'static str,
    pub purpose: &'static str,
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub phy_handle: &'static str,
    pub phy_node: &'static str,
    pub phy_address: u32,
    pub macb_mid_context_target: u64,
    pub macb_mid_context_raw: u32,
    pub ncr_register: &'static str,
    pub nsr_register: &'static str,
    pub man_register: &'static str,
    pub ncr_observed_target: u64,
    pub nsr_observed_target: u64,
    pub man_observed_target: u64,
    pub register_names: &'static [&'static str],
    pub registers: &'static [u32],
    pub man_frames: &'static [u32],
    pub man_frame_construction: &'static str,
    pub ncr_mpe_bit: u8,
    pub nsr_idle_bit: u8,
    pub poll_policy: &'static str,
    pub mpe_precondition: &'static str,
    pub man_data_offset: u8,
    pub man_data_size: u8,
    pub result_extraction: &'static str,
    pub operation_order: &'static [&'static str],
    pub allowed_classifications: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioRegisterVectorGuardReportKind {
    Candidate,
    NoMdioNoEthernetControl,
}

impl Rp1EthernetMdioRegisterVectorGuardReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoMdioNoEthernetControl => "no-mdio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioRegisterVectorGuardReportInput {
    pub kind: Rp1EthernetMdioRegisterVectorGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioRegisterVectorSourceContractEvidence>,
    pub claims_runtime_volatile_load_store: bool,
    pub claims_ncr_write: bool,
    pub claims_missing_mpe_gating: bool,
    pub claims_wrong_mdio_targets: bool,
    pub claims_unbounded_polling: bool,
    pub claims_man_write_without_mpe_precondition: bool,
    pub claims_phy_absence_from_all_ones: bool,
    pub claims_broad_mdio_phy_ownership: bool,
    pub claims_gpio32_phy_reset_action: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioRegisterVectorGuardReport {
    pub kind: Rp1EthernetMdioRegisterVectorGuardReportKind,
    pub source_contract: Option<Rp1EthernetMdioRegisterVectorSourceContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetMdioRegisterVectorGuardReportEvidence {
    pub report_contract_id: &'static str,
    pub source_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub selected_discriminator: Option<&'static str>,
    pub purpose: Option<&'static str>,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub phy_handle: Option<&'static str>,
    pub phy_node: Option<&'static str>,
    pub phy_address: Option<u32>,
    pub macb_mid_context_target: Option<u64>,
    pub macb_mid_context_raw: Option<u32>,
    pub ncr_register: Option<&'static str>,
    pub nsr_register: Option<&'static str>,
    pub man_register: Option<&'static str>,
    pub ncr_observed_target: Option<u64>,
    pub nsr_observed_target: Option<u64>,
    pub man_observed_target: Option<u64>,
    pub register_names: Option<&'static [&'static str]>,
    pub registers: Option<&'static [u32]>,
    pub man_frames: Option<&'static [u32]>,
    pub man_frame_construction: Option<&'static str>,
    pub ncr_mpe_bit: Option<u8>,
    pub nsr_idle_bit: Option<u8>,
    pub poll_policy: Option<&'static str>,
    pub mpe_precondition: Option<&'static str>,
    pub man_data_offset: Option<u8>,
    pub man_data_size: Option<u8>,
    pub result_extraction: Option<&'static str>,
    pub operation_order: Option<&'static [&'static str]>,
    pub allowed_classifications: &'static [&'static str],
    pub boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub constructs_mdio_targets: bool,
    pub constructs_man_frames: bool,
    pub runtime_volatile_load_store_intent: bool,
    pub claims_runtime_volatile_load_store: bool,
    pub claims_ncr_write: bool,
    pub claims_missing_mpe_gating: bool,
    pub claims_wrong_mdio_targets: bool,
    pub claims_unbounded_polling: bool,
    pub claims_man_write_without_mpe_precondition: bool,
    pub claims_phy_absence_from_all_ones: bool,
    pub claims_broad_mdio_phy_ownership: bool,
    pub claims_gpio32_phy_reset_action: bool,
    pub claims_ethernet_ready: bool,
    pub claims_interrupt_completion: bool,
    pub claims_dma_descriptor_ownership: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetMdioRegisterVectorGuardReportError {
    CandidateMissingSourceContract,
    ControlCarriesMdioTargetFacts,
    SourceContractIdentityMismatch,
    SourceContractTargetMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    RuntimeVolatileLoadStoreClaim,
    NcrWriteClaim,
    MissingMpeGatingClaim,
    WrongMdioTargetsClaim,
    UnboundedPollingClaim,
    ManWriteWithoutMpePreconditionClaim,
    PhyAbsenceFromAllOnesClaim,
    BroadMdioPhyOwnershipClaim,
    Gpio32PhyResetActionClaim,
    EthernetReadinessClaim,
    InterruptCompletionClaim,
    DmaDescriptorOwnershipClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetMdioRegisterVectorGuardReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesMdioTargetFacts => "control-carries-mdio-target-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractTargetMismatch => "source-contract-target-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::RuntimeVolatileLoadStoreClaim => "runtime-volatile-load-store-claim",
            Self::NcrWriteClaim => "ncr-write-claim",
            Self::MissingMpeGatingClaim => "missing-mpe-gating-claim",
            Self::WrongMdioTargetsClaim => "wrong-mdio-targets-claim",
            Self::UnboundedPollingClaim => "unbounded-polling-claim",
            Self::ManWriteWithoutMpePreconditionClaim => "man-write-without-mpe-precondition-claim",
            Self::PhyAbsenceFromAllOnesClaim => "phy-absence-from-all-ones-claim",
            Self::BroadMdioPhyOwnershipClaim => "broad-mdio-phy-ownership-claim",
            Self::Gpio32PhyResetActionClaim => "gpio32-phy-reset-action-claim",
            Self::EthernetReadinessClaim => "ethernet-readiness-claim",
            Self::InterruptCompletionClaim => "interrupt-completion-claim",
            Self::DmaDescriptorOwnershipClaim => "dma-descriptor-ownership-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_mdio_register_vector_source_contract_evidence()
-> Rp1EthernetMdioRegisterVectorSourceContractEvidence {
    Rp1EthernetMdioRegisterVectorSourceContractEvidence {
        contract_id: RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_TASK_ID,
        selected_discriminator: RP1_ETHERNET_MDIO_REGISTER_VECTOR_SELECTED_DISCRIMINATOR,
        purpose: RP1_ETHERNET_MDIO_REGISTER_VECTOR_PURPOSE,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        phy_handle: RP1_ETHERNET_PHY_HANDLE,
        phy_node: RP1_ETHERNET_PHY_NODE,
        phy_address: RP1_ETHERNET_PHY_REG,
        macb_mid_context_target: RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET,
        macb_mid_context_raw: RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW,
        ncr_register: RP1_ETHERNET_MDIO_PHY_ID_NCR_REGISTER,
        nsr_register: RP1_ETHERNET_MDIO_PHY_ID_NSR_REGISTER,
        man_register: RP1_ETHERNET_MDIO_PHY_ID_MAN_REGISTER,
        ncr_observed_target: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NCR_OBSERVED_TARGET,
        nsr_observed_target: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NSR_OBSERVED_TARGET,
        man_observed_target: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MAN_OBSERVED_TARGET,
        register_names: RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTER_NAMES,
        registers: RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTERS,
        man_frames: RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAMES,
        man_frame_construction: RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAME_CONSTRUCTION,
        ncr_mpe_bit: RP1_ETHERNET_MDIO_PHY_ID_NCR_MPE_BIT,
        nsr_idle_bit: RP1_ETHERNET_MDIO_PHY_ID_NSR_IDLE_BIT,
        poll_policy: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_POLL_POLICY,
        mpe_precondition: RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MPE_PRECONDITION,
        man_data_offset: RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_OFFSET,
        man_data_size: RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_SIZE,
        result_extraction: RP1_ETHERNET_MDIO_PHY_ID_RESULT_EXTRACTION,
        operation_order: RP1_ETHERNET_MDIO_REGISTER_VECTOR_OPERATION_ORDER,
        allowed_classifications: RP1_ETHERNET_MDIO_REGISTER_VECTOR_ALLOWED_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_mdio_register_vector_guard_report(
    input: Rp1EthernetMdioRegisterVectorGuardReportInput,
) -> Result<Rp1EthernetMdioRegisterVectorGuardReport, Rp1EthernetMdioRegisterVectorGuardReportError>
{
    validate_rp1_ethernet_mdio_register_vector_rejected_claims(input)?;

    match (input.kind, input.source_contract) {
        (Rp1EthernetMdioRegisterVectorGuardReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_mdio_register_vector_source_contract(source_contract)?;
            Ok(Rp1EthernetMdioRegisterVectorGuardReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetMdioRegisterVectorGuardReportKind::Candidate, None) => {
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetMdioRegisterVectorGuardReportKind::NoMdioNoEthernetControl, None) => {
            Ok(Rp1EthernetMdioRegisterVectorGuardReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetMdioRegisterVectorGuardReportKind::NoMdioNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::ControlCarriesMdioTargetFacts)
        }
    }
}

pub fn rp1_ethernet_mdio_register_vector_guard_report_evidence(
    report: Rp1EthernetMdioRegisterVectorGuardReport,
) -> Rp1EthernetMdioRegisterVectorGuardReportEvidence {
    match report.source_contract {
        Some(source_contract) => rp1_ethernet_mdio_register_vector_guard_candidate_evidence(
            report.kind.name(),
            source_contract,
        ),
        None => rp1_ethernet_mdio_register_vector_guard_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_mdio_register_vector_guard_report_evidence(
    error: Rp1EthernetMdioRegisterVectorGuardReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_mdio_register_vector_rejected_claims(
    input: Rp1EthernetMdioRegisterVectorGuardReportInput,
) -> Result<(), Rp1EthernetMdioRegisterVectorGuardReportError> {
    if input.claims_runtime_volatile_load_store {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::RuntimeVolatileLoadStoreClaim);
    }
    if input.claims_ncr_write {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::NcrWriteClaim);
    }
    if input.claims_missing_mpe_gating {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::MissingMpeGatingClaim);
    }
    if input.claims_wrong_mdio_targets {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::WrongMdioTargetsClaim);
    }
    if input.claims_unbounded_polling {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::UnboundedPollingClaim);
    }
    if input.claims_man_write_without_mpe_precondition {
        return Err(
            Rp1EthernetMdioRegisterVectorGuardReportError::ManWriteWithoutMpePreconditionClaim,
        );
    }
    if input.claims_phy_absence_from_all_ones {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::PhyAbsenceFromAllOnesClaim);
    }
    if input.claims_broad_mdio_phy_ownership {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::BroadMdioPhyOwnershipClaim);
    }
    if input.claims_gpio32_phy_reset_action {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::Gpio32PhyResetActionClaim);
    }
    if input.claims_ethernet_ready {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::EthernetReadinessClaim);
    }
    if input.claims_interrupt_completion {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::InterruptCompletionClaim);
    }
    if input.claims_dma_descriptor_ownership {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::DmaDescriptorOwnershipClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_mdio_register_vector_source_contract(
    source_contract: Rp1EthernetMdioRegisterVectorSourceContractEvidence,
) -> Result<(), Rp1EthernetMdioRegisterVectorGuardReportError> {
    if source_contract.contract_id != RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_CONTRACT_ID
        || source_contract.source_task_id != RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_TASK_ID
        || source_contract.selected_discriminator
            != RP1_ETHERNET_MDIO_REGISTER_VECTOR_SELECTED_DISCRIMINATOR
        || source_contract.purpose != RP1_ETHERNET_MDIO_REGISTER_VECTOR_PURPOSE
        || source_contract.controller != RP1_ETHERNET_CONTROLLER_NAME
        || source_contract.compatible != RP1_ETHERNET_COMPATIBLE
        || source_contract.phy_handle != RP1_ETHERNET_PHY_HANDLE
        || source_contract.phy_node != RP1_ETHERNET_PHY_NODE
        || source_contract.phy_address != RP1_ETHERNET_PHY_REG
    {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::SourceContractIdentityMismatch);
    }
    if source_contract.macb_mid_context_target
        != RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID_CPU_PHYSICAL_TARGET
        || source_contract.macb_mid_context_raw != RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW
        || source_contract.ncr_observed_target
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NCR_OBSERVED_TARGET
        || source_contract.nsr_observed_target
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_NSR_OBSERVED_TARGET
        || source_contract.man_observed_target
            != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MAN_OBSERVED_TARGET
    {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::SourceContractTargetMismatch);
    }
    if source_contract.ncr_register != RP1_ETHERNET_MDIO_PHY_ID_NCR_REGISTER
        || source_contract.nsr_register != RP1_ETHERNET_MDIO_PHY_ID_NSR_REGISTER
        || source_contract.man_register != RP1_ETHERNET_MDIO_PHY_ID_MAN_REGISTER
        || source_contract.register_names != RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTER_NAMES
        || source_contract.registers != RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTERS
        || source_contract.man_frames != RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAMES
        || source_contract.man_frame_construction
            != RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAME_CONSTRUCTION
        || source_contract.ncr_mpe_bit != RP1_ETHERNET_MDIO_PHY_ID_NCR_MPE_BIT
        || source_contract.nsr_idle_bit != RP1_ETHERNET_MDIO_PHY_ID_NSR_IDLE_BIT
        || source_contract.poll_policy != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_POLL_POLICY
        || source_contract.mpe_precondition != RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MPE_PRECONDITION
        || source_contract.man_data_offset != RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_OFFSET
        || source_contract.man_data_size != RP1_ETHERNET_MDIO_PHY_ID_MAN_DATA_SIZE
        || source_contract.result_extraction != RP1_ETHERNET_MDIO_PHY_ID_RESULT_EXTRACTION
        || source_contract.operation_order != RP1_ETHERNET_MDIO_REGISTER_VECTOR_OPERATION_ORDER
        || source_contract.allowed_classifications
            != RP1_ETHERNET_MDIO_REGISTER_VECTOR_ALLOWED_CLASSIFICATIONS
    {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::SourceContractFieldMismatch);
    }
    if source_contract.source_evidence != RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_EVIDENCE {
        return Err(Rp1EthernetMdioRegisterVectorGuardReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_mdio_register_vector_guard_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetMdioRegisterVectorSourceContractEvidence,
) -> Rp1EthernetMdioRegisterVectorGuardReportEvidence {
    Rp1EthernetMdioRegisterVectorGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_REGISTER_VECTOR_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: source_contract.contract_id,
        source_task_id: source_contract.source_task_id,
        report_kind,
        selected_discriminator: Some(source_contract.selected_discriminator),
        purpose: Some(source_contract.purpose),
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        phy_handle: Some(source_contract.phy_handle),
        phy_node: Some(source_contract.phy_node),
        phy_address: Some(source_contract.phy_address),
        macb_mid_context_target: Some(source_contract.macb_mid_context_target),
        macb_mid_context_raw: Some(source_contract.macb_mid_context_raw),
        ncr_register: Some(source_contract.ncr_register),
        nsr_register: Some(source_contract.nsr_register),
        man_register: Some(source_contract.man_register),
        ncr_observed_target: Some(source_contract.ncr_observed_target),
        nsr_observed_target: Some(source_contract.nsr_observed_target),
        man_observed_target: Some(source_contract.man_observed_target),
        register_names: Some(source_contract.register_names),
        registers: Some(source_contract.registers),
        man_frames: Some(source_contract.man_frames),
        man_frame_construction: Some(source_contract.man_frame_construction),
        ncr_mpe_bit: Some(source_contract.ncr_mpe_bit),
        nsr_idle_bit: Some(source_contract.nsr_idle_bit),
        poll_policy: Some(source_contract.poll_policy),
        mpe_precondition: Some(source_contract.mpe_precondition),
        man_data_offset: Some(source_contract.man_data_offset),
        man_data_size: Some(source_contract.man_data_size),
        result_extraction: Some(source_contract.result_extraction),
        operation_order: Some(source_contract.operation_order),
        allowed_classifications: RP1_ETHERNET_MDIO_REGISTER_VECTOR_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_REGISTER_VECTOR_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_REGISTER_VECTOR_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_REGISTER_VECTOR_RETAINED_RISKS,
        source_evidence: Some(source_contract.source_evidence),
        constructs_mdio_targets: true,
        constructs_man_frames: true,
        runtime_volatile_load_store_intent: false,
        claims_runtime_volatile_load_store: false,
        claims_ncr_write: false,
        claims_missing_mpe_gating: false,
        claims_wrong_mdio_targets: false,
        claims_unbounded_polling: false,
        claims_man_write_without_mpe_precondition: false,
        claims_phy_absence_from_all_ones: false,
        claims_broad_mdio_phy_ownership: false,
        claims_gpio32_phy_reset_action: false,
        claims_ethernet_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_REGISTER_VECTOR_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_mdio_register_vector_guard_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetMdioRegisterVectorGuardReportEvidence {
    Rp1EthernetMdioRegisterVectorGuardReportEvidence {
        report_contract_id: RP1_ETHERNET_MDIO_REGISTER_VECTOR_GUARD_REPORT_CONTRACT_ID,
        source_contract_id: RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_TASK_ID,
        report_kind,
        selected_discriminator: None,
        purpose: None,
        controller: None,
        compatible: None,
        phy_handle: None,
        phy_node: None,
        phy_address: None,
        macb_mid_context_target: None,
        macb_mid_context_raw: None,
        ncr_register: None,
        nsr_register: None,
        man_register: None,
        ncr_observed_target: None,
        nsr_observed_target: None,
        man_observed_target: None,
        register_names: None,
        registers: None,
        man_frames: None,
        man_frame_construction: None,
        ncr_mpe_bit: None,
        nsr_idle_bit: None,
        poll_policy: None,
        mpe_precondition: None,
        man_data_offset: None,
        man_data_size: None,
        result_extraction: None,
        operation_order: None,
        allowed_classifications: RP1_ETHERNET_MDIO_REGISTER_VECTOR_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_MDIO_REGISTER_VECTOR_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_MDIO_REGISTER_VECTOR_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_MDIO_REGISTER_VECTOR_RETAINED_RISKS,
        source_evidence: None,
        constructs_mdio_targets: false,
        constructs_man_frames: false,
        runtime_volatile_load_store_intent: false,
        claims_runtime_volatile_load_store: false,
        claims_ncr_write: false,
        claims_missing_mpe_gating: false,
        claims_wrong_mdio_targets: false,
        claims_unbounded_polling: false,
        claims_man_write_without_mpe_precondition: false,
        claims_phy_absence_from_all_ones: false,
        claims_broad_mdio_phy_ownership: false,
        claims_gpio32_phy_reset_action: false,
        claims_ethernet_ready: false,
        claims_interrupt_completion: false,
        claims_dma_descriptor_ownership: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_MDIO_REGISTER_VECTOR_CONTROL_CLASSIFICATION,
    }
}

pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_TASK_ID: &str =
    "phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract-20260616";
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REPORT_CONTRACT_ID: &str =
    "phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-contract-v1";
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_DISCRIMINATOR: &str =
    "bcm54213pe-phy1-mii-ctrl1000-stat1000-readonly-preflight";
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_CANDIDATE_CLASSIFICATION: &str =
    "bcm54213pe-readonly-preflight-report-core-candidate-local-static";
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_CONTROL_CLASSIFICATION: &str =
    "no-mdio-no-ethernet-bcm54213pe-readonly-preflight-control";
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_BOUNDARY_CLASSIFICATION: &str =
    "local-static-mii-ctrl1000-stat1000-report-core-only";
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_PURPOSE: &str =
    "encode the accepted pure Clause 22 MII_CTRL1000/MII_STAT1000 read-only preflight boundary";
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTER_NAMES: &[&str] =
    &["MII_CTRL1000", "MII_STAT1000"];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTERS: &[u32] = &[0x09, 0x0a];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_MAN_FRAMES: &[u32] =
    &[0x60a6_0000, 0x60aa_0000];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_TARGETS: &[&str] =
    &["PHY1 MII_CTRL1000 0x09", "PHY1 MII_STAT1000 0x0a"];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALREADY_SAMPLED_CONTEXT: &[&str] = &[
    "BMCR 0x1000",
    "BMSR 0x7949/0x7949",
    "ANAR 0x01e1",
    "ANLPAR 0x0000",
    "MACB_NSR 0x00000006",
];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_FORBIDDEN_TARGETS: &[&str] = &[
    "BMCR autoneg restart or write",
    "same-shaped BMCR/BMSR/ANAR/ANLPAR/MACB_NSR retry",
    "MII_BCM54XX_ISR interrupt-acknowledging read",
    "MII_BCM54XX_ECR or MII_BCM54XX_IMR interrupt configuration",
    "Broadcom AUX/shadow selector read or write/restore",
    "Clause 45/MMD EEE selector read or write/restore",
    "GPIO32 event clear or ETH_RST_N reset action",
    "PHY or MACB configuration",
];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_OPERATION_ORDER: &[&str] = &[
    "local/static report core only; perform no hardware access",
    "candidate encodes exactly PHY1 MII_CTRL1000 0x09 and MII_STAT1000 0x0a",
    "candidate may compute future Clause 22 MAN read frames as contract metadata only",
    "control withholds all MDIO, MAN, MACB, GPIO, and RP1 Ethernet target facts",
    "accepted BMCR/BMSR/ANAR/ANLPAR/MACB_NSR values remain context only",
    "select closeout before any later hardware-proof contract can be planned",
];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALLOWED_CLASSIFICATIONS: &[&str] = &[
    "bcm54213pe-readonly-preflight-report-core-candidate-local-static",
    "no-mdio-no-ethernet-bcm54213pe-readonly-preflight-control",
    "bcm54213pe-readonly-preflight-report-core-blocker",
];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REJECTED_RUNTIME_CLAIMS: &[&str] = &[
    "runtime volatile Ethernet access from local/static report core",
    "hardwareTestLock acquisition or Pi 5 hardware proof authorization",
    "GPIO32 ownership, event clear, or PHY reset action",
    "BMCR autoneg restart or any PHY register write",
    "Broadcom shadow, AUX, MMD, or selector access",
    "MII_BCM54XX_ISR/ECR/IMR interrupt ownership or interrupt-status proof",
    "broad PHY or MACB configuration",
    "link readiness",
    "packet I/O",
    "networking",
    "sockets",
    "SSH",
    "Phase 12.2 work",
    "phase transition",
];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_RETAINED_RISKS: &[&str] = &[
    "MII_CTRL1000/MII_STAT1000 report metadata does not prove link readiness while BMSR autoneg-complete remains false",
    "A later hardware proof still needs hardwareTestLock, identity evidence, TFTP/serial evidence, paired control, restore proof, and inconclusive-run triage",
    "GPIO32/ETH_RST_N reset recovery remains blocked by persistent or firmware-owned event state",
    "Broadcom selector-write surfaces remain blocked pending a separate write/restore contract",
    "Packet I/O, networking, sockets, SSH, and Phase 12.2 remain unaccepted",
];
pub const RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_EVIDENCE: &[&str] = &[
    "tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract.md",
    "tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/classification.json",
    "tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/evidence-map.json",
    "tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/linux-rpi-6.12-mii-register-readonly-preflight-excerpt.txt",
    "tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/linux-rpi-6.12-phy-device-master-slave-readonly-preflight-excerpt.txt",
    "tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/linux-rpi-6.12-phy-device-read-status-register-contract-excerpt.txt",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1GigabitPreflightRawVector {
    pub ctrl1000: u16,
    pub stat1000: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1Ctrl1000Status {
    pub advertise_1000_half: bool,
    pub advertise_1000_full: bool,
    pub prefer_master: bool,
    pub force_master: bool,
    pub master_manual_config_enable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1Stat1000Status {
    pub master_slave_resolution_failure: bool,
    pub master_slave_resolution_master: bool,
    pub local_receiver_ok: bool,
    pub remote_receiver_ok: bool,
    pub link_partner_1000_full: bool,
    pub link_partner_1000_half: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetPhy1GigabitPreflightDecode {
    pub raw: Rp1EthernetPhy1GigabitPreflightRawVector,
    pub ctrl1000: Rp1EthernetPhy1Ctrl1000Status,
    pub stat1000: Rp1EthernetPhy1Stat1000Status,
}

pub const fn decode_rp1_ethernet_phy1_gigabit_preflight(
    raw: Rp1EthernetPhy1GigabitPreflightRawVector,
) -> Rp1EthernetPhy1GigabitPreflightDecode {
    Rp1EthernetPhy1GigabitPreflightDecode {
        raw,
        ctrl1000: decode_rp1_ethernet_phy1_ctrl1000(raw.ctrl1000),
        stat1000: decode_rp1_ethernet_phy1_stat1000(raw.stat1000),
    }
}

pub const fn decode_rp1_ethernet_phy1_ctrl1000(raw: u16) -> Rp1EthernetPhy1Ctrl1000Status {
    Rp1EthernetPhy1Ctrl1000Status {
        advertise_1000_half: raw & 0x0100 != 0,
        advertise_1000_full: raw & 0x0200 != 0,
        prefer_master: raw & 0x0400 != 0,
        force_master: raw & 0x0800 != 0,
        master_manual_config_enable: raw & 0x1000 != 0,
    }
}

pub const fn decode_rp1_ethernet_phy1_stat1000(raw: u16) -> Rp1EthernetPhy1Stat1000Status {
    Rp1EthernetPhy1Stat1000Status {
        master_slave_resolution_failure: raw & 0x8000 != 0,
        master_slave_resolution_master: raw & 0x4000 != 0,
        local_receiver_ok: raw & 0x2000 != 0,
        remote_receiver_ok: raw & 0x1000 != 0,
        link_partner_1000_full: raw & 0x0800 != 0,
        link_partner_1000_half: raw & 0x0400 != 0,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence {
    pub report_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub selected_discriminator: &'static str,
    pub purpose: &'static str,
    pub selected_targets: &'static [&'static str],
    pub controller: &'static str,
    pub compatible: &'static [&'static str],
    pub phy_handle: &'static str,
    pub phy_node: &'static str,
    pub phy_address: u32,
    pub register_names: &'static [&'static str],
    pub registers: &'static [u32],
    pub man_frames: &'static [u32],
    pub man_frame_construction: &'static str,
    pub accepted_phy_id_model: &'static str,
    pub accepted_physid1: u16,
    pub accepted_physid2: u16,
    pub already_sampled_context: &'static [&'static str],
    pub forbidden_targets: &'static [&'static str],
    pub operation_order: &'static [&'static str],
    pub allowed_classifications: &'static [&'static str],
    pub source_evidence: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetBcm54213peReadonlyPreflightReportKind {
    Candidate,
    NoMdioNoEthernetControl,
}

impl Rp1EthernetBcm54213peReadonlyPreflightReportKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::NoMdioNoEthernetControl => "no-mdio-no-ethernet-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetBcm54213peReadonlyPreflightReportInput {
    pub kind: Rp1EthernetBcm54213peReadonlyPreflightReportKind,
    pub source_contract: Option<Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence>,
    pub claims_runtime_volatile_access: bool,
    pub claims_hardware_proof_authorized: bool,
    pub claims_gpio32_action: bool,
    pub claims_bmcr_or_phy_write: bool,
    pub claims_broadcom_shadow_mmd_aux_access: bool,
    pub claims_interrupt_surface_access: bool,
    pub claims_phy_or_mac_configuration: bool,
    pub claims_link_readiness: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetBcm54213peReadonlyPreflightReport {
    pub kind: Rp1EthernetBcm54213peReadonlyPreflightReportKind,
    pub source_contract: Option<Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rp1EthernetBcm54213peReadonlyPreflightReportEvidence {
    pub report_contract_id: &'static str,
    pub source_task_id: &'static str,
    pub report_kind: &'static str,
    pub selected_discriminator: Option<&'static str>,
    pub purpose: Option<&'static str>,
    pub selected_targets: Option<&'static [&'static str]>,
    pub controller: Option<&'static str>,
    pub compatible: Option<&'static [&'static str]>,
    pub phy_handle: Option<&'static str>,
    pub phy_node: Option<&'static str>,
    pub phy_address: Option<u32>,
    pub register_names: Option<&'static [&'static str]>,
    pub registers: Option<&'static [u32]>,
    pub man_frames: Option<&'static [u32]>,
    pub man_frame_construction: Option<&'static str>,
    pub accepted_phy_id_model: Option<&'static str>,
    pub accepted_physid1: Option<u16>,
    pub accepted_physid2: Option<u16>,
    pub already_sampled_context: Option<&'static [&'static str]>,
    pub forbidden_targets: Option<&'static [&'static str]>,
    pub operation_order: Option<&'static [&'static str]>,
    pub allowed_classifications: &'static [&'static str],
    pub boundary_classification: &'static str,
    pub rejected_runtime_claims: &'static [&'static str],
    pub retained_risks: &'static [&'static str],
    pub source_evidence: Option<&'static [&'static str]>,
    pub constructs_mdio_targets: bool,
    pub constructs_man_frames: bool,
    pub constructs_macb_target: bool,
    pub constructs_gpio_target: bool,
    pub runtime_volatile_access_intent: bool,
    pub claims_runtime_volatile_access: bool,
    pub claims_hardware_proof_authorized: bool,
    pub claims_gpio32_action: bool,
    pub claims_bmcr_or_phy_write: bool,
    pub claims_broadcom_shadow_mmd_aux_access: bool,
    pub claims_interrupt_surface_access: bool,
    pub claims_phy_or_mac_configuration: bool,
    pub claims_link_readiness: bool,
    pub claims_packet_io: bool,
    pub claims_networking: bool,
    pub claims_sockets: bool,
    pub claims_ssh: bool,
    pub claims_phase_12_2: bool,
    pub claims_phase_transition: bool,
    pub classification: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rp1EthernetBcm54213peReadonlyPreflightReportError {
    CandidateMissingSourceContract,
    ControlCarriesMdioTargetFacts,
    SourceContractIdentityMismatch,
    SourceContractFieldMismatch,
    MissingSourceEvidence,
    RuntimeVolatileAccessClaim,
    HardwareProofAuthorizedClaim,
    Gpio32ActionClaim,
    BmcrOrPhyWriteClaim,
    BroadcomShadowMmdAuxAccessClaim,
    InterruptSurfaceAccessClaim,
    PhyOrMacConfigurationClaim,
    LinkReadinessClaim,
    PacketIoClaim,
    NetworkingClaim,
    SocketsClaim,
    SshClaim,
    Phase122Claim,
    PhaseTransitionClaim,
}

impl Rp1EthernetBcm54213peReadonlyPreflightReportError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateMissingSourceContract => "candidate-missing-source-contract",
            Self::ControlCarriesMdioTargetFacts => "control-carries-mdio-target-facts",
            Self::SourceContractIdentityMismatch => "source-contract-identity-mismatch",
            Self::SourceContractFieldMismatch => "source-contract-field-mismatch",
            Self::MissingSourceEvidence => "missing-source-evidence",
            Self::RuntimeVolatileAccessClaim => "runtime-volatile-access-claim",
            Self::HardwareProofAuthorizedClaim => "hardware-proof-authorized-claim",
            Self::Gpio32ActionClaim => "gpio32-action-claim",
            Self::BmcrOrPhyWriteClaim => "bmcr-or-phy-write-claim",
            Self::BroadcomShadowMmdAuxAccessClaim => "broadcom-shadow-mmd-aux-access-claim",
            Self::InterruptSurfaceAccessClaim => "interrupt-surface-access-claim",
            Self::PhyOrMacConfigurationClaim => "phy-or-mac-configuration-claim",
            Self::LinkReadinessClaim => "link-readiness-claim",
            Self::PacketIoClaim => "packet-io-claim",
            Self::NetworkingClaim => "networking-claim",
            Self::SocketsClaim => "sockets-claim",
            Self::SshClaim => "ssh-claim",
            Self::Phase122Claim => "phase-12-2-claim",
            Self::PhaseTransitionClaim => "phase-transition-claim",
        }
    }
}

pub const fn rp1_ethernet_bcm54213pe_readonly_preflight_report_contract_evidence()
-> Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence {
    Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence {
        report_contract_id: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REPORT_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_TASK_ID,
        selected_discriminator: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_DISCRIMINATOR,
        purpose: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_PURPOSE,
        selected_targets: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_TARGETS,
        controller: RP1_ETHERNET_CONTROLLER_NAME,
        compatible: RP1_ETHERNET_COMPATIBLE,
        phy_handle: RP1_ETHERNET_PHY_HANDLE,
        phy_node: RP1_ETHERNET_PHY_NODE,
        phy_address: RP1_ETHERNET_PHY_REG,
        register_names: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTER_NAMES,
        registers: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTERS,
        man_frames: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_MAN_FRAMES,
        man_frame_construction: RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAME_CONSTRUCTION,
        accepted_phy_id_model: "Broadcom BCM54213PE",
        accepted_physid1: 0x600d,
        accepted_physid2: 0x84a2,
        already_sampled_context: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALREADY_SAMPLED_CONTEXT,
        forbidden_targets: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_FORBIDDEN_TARGETS,
        operation_order: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_OPERATION_ORDER,
        allowed_classifications: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALLOWED_CLASSIFICATIONS,
        source_evidence: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_EVIDENCE,
    }
}

pub fn build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
    input: Rp1EthernetBcm54213peReadonlyPreflightReportInput,
) -> Result<
    Rp1EthernetBcm54213peReadonlyPreflightReport,
    Rp1EthernetBcm54213peReadonlyPreflightReportError,
> {
    validate_rp1_ethernet_bcm54213pe_readonly_preflight_rejected_claims(input)?;
    match (input.kind, input.source_contract) {
        (Rp1EthernetBcm54213peReadonlyPreflightReportKind::Candidate, Some(source_contract)) => {
            validate_rp1_ethernet_bcm54213pe_readonly_preflight_source_contract(source_contract)?;
            Ok(Rp1EthernetBcm54213peReadonlyPreflightReport {
                kind: input.kind,
                source_contract: Some(source_contract),
            })
        }
        (Rp1EthernetBcm54213peReadonlyPreflightReportKind::Candidate, None) => {
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::CandidateMissingSourceContract)
        }
        (Rp1EthernetBcm54213peReadonlyPreflightReportKind::NoMdioNoEthernetControl, None) => {
            Ok(Rp1EthernetBcm54213peReadonlyPreflightReport {
                kind: input.kind,
                source_contract: None,
            })
        }
        (Rp1EthernetBcm54213peReadonlyPreflightReportKind::NoMdioNoEthernetControl, Some(_)) => {
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::ControlCarriesMdioTargetFacts)
        }
    }
}

pub fn rp1_ethernet_bcm54213pe_readonly_preflight_report_evidence(
    report: Rp1EthernetBcm54213peReadonlyPreflightReport,
) -> Rp1EthernetBcm54213peReadonlyPreflightReportEvidence {
    match report.source_contract {
        Some(source_contract) => rp1_ethernet_bcm54213pe_readonly_preflight_candidate_evidence(
            report.kind.name(),
            source_contract,
        ),
        None => rp1_ethernet_bcm54213pe_readonly_preflight_control_evidence(report.kind.name()),
    }
}

pub fn rejected_rp1_ethernet_bcm54213pe_readonly_preflight_report_evidence(
    error: Rp1EthernetBcm54213peReadonlyPreflightReportError,
) -> (&'static str, &'static str) {
    (RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION, error.name())
}

fn validate_rp1_ethernet_bcm54213pe_readonly_preflight_rejected_claims(
    input: Rp1EthernetBcm54213peReadonlyPreflightReportInput,
) -> Result<(), Rp1EthernetBcm54213peReadonlyPreflightReportError> {
    if input.claims_runtime_volatile_access {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::RuntimeVolatileAccessClaim);
    }
    if input.claims_hardware_proof_authorized {
        return Err(
            Rp1EthernetBcm54213peReadonlyPreflightReportError::HardwareProofAuthorizedClaim,
        );
    }
    if input.claims_gpio32_action {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::Gpio32ActionClaim);
    }
    if input.claims_bmcr_or_phy_write {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::BmcrOrPhyWriteClaim);
    }
    if input.claims_broadcom_shadow_mmd_aux_access {
        return Err(
            Rp1EthernetBcm54213peReadonlyPreflightReportError::BroadcomShadowMmdAuxAccessClaim,
        );
    }
    if input.claims_interrupt_surface_access {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::InterruptSurfaceAccessClaim);
    }
    if input.claims_phy_or_mac_configuration {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::PhyOrMacConfigurationClaim);
    }
    if input.claims_link_readiness {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::LinkReadinessClaim);
    }
    if input.claims_packet_io {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::PacketIoClaim);
    }
    if input.claims_networking {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::NetworkingClaim);
    }
    if input.claims_sockets {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::SocketsClaim);
    }
    if input.claims_ssh {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::SshClaim);
    }
    if input.claims_phase_12_2 {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::Phase122Claim);
    }
    if input.claims_phase_transition {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::PhaseTransitionClaim);
    }
    Ok(())
}

fn validate_rp1_ethernet_bcm54213pe_readonly_preflight_source_contract(
    source_contract: Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence,
) -> Result<(), Rp1EthernetBcm54213peReadonlyPreflightReportError> {
    if source_contract.report_contract_id
        != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REPORT_CONTRACT_ID
        || source_contract.source_task_id
            != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_TASK_ID
        || source_contract.selected_discriminator
            != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_DISCRIMINATOR
        || source_contract.purpose != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_PURPOSE
        || source_contract.controller != RP1_ETHERNET_CONTROLLER_NAME
        || source_contract.compatible != RP1_ETHERNET_COMPATIBLE
        || source_contract.phy_handle != RP1_ETHERNET_PHY_HANDLE
        || source_contract.phy_node != RP1_ETHERNET_PHY_NODE
        || source_contract.phy_address != RP1_ETHERNET_PHY_REG
        || source_contract.accepted_phy_id_model != "Broadcom BCM54213PE"
        || source_contract.accepted_physid1 != 0x600d
        || source_contract.accepted_physid2 != 0x84a2
    {
        return Err(
            Rp1EthernetBcm54213peReadonlyPreflightReportError::SourceContractIdentityMismatch,
        );
    }
    if source_contract.selected_targets
        != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_TARGETS
        || source_contract.register_names
            != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTER_NAMES
        || source_contract.registers != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTERS
        || source_contract.man_frames != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_MAN_FRAMES
        || source_contract.man_frame_construction
            != RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAME_CONSTRUCTION
        || source_contract.already_sampled_context
            != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALREADY_SAMPLED_CONTEXT
        || source_contract.forbidden_targets
            != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_FORBIDDEN_TARGETS
        || source_contract.operation_order
            != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_OPERATION_ORDER
        || source_contract.allowed_classifications
            != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALLOWED_CLASSIFICATIONS
    {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::SourceContractFieldMismatch);
    }
    if source_contract.source_evidence != RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_EVIDENCE
    {
        return Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::MissingSourceEvidence);
    }
    Ok(())
}

fn rp1_ethernet_bcm54213pe_readonly_preflight_candidate_evidence(
    report_kind: &'static str,
    source_contract: Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence,
) -> Rp1EthernetBcm54213peReadonlyPreflightReportEvidence {
    Rp1EthernetBcm54213peReadonlyPreflightReportEvidence {
        report_contract_id: source_contract.report_contract_id,
        source_task_id: source_contract.source_task_id,
        report_kind,
        selected_discriminator: Some(source_contract.selected_discriminator),
        purpose: Some(source_contract.purpose),
        selected_targets: Some(source_contract.selected_targets),
        controller: Some(source_contract.controller),
        compatible: Some(source_contract.compatible),
        phy_handle: Some(source_contract.phy_handle),
        phy_node: Some(source_contract.phy_node),
        phy_address: Some(source_contract.phy_address),
        register_names: Some(source_contract.register_names),
        registers: Some(source_contract.registers),
        man_frames: Some(source_contract.man_frames),
        man_frame_construction: Some(source_contract.man_frame_construction),
        accepted_phy_id_model: Some(source_contract.accepted_phy_id_model),
        accepted_physid1: Some(source_contract.accepted_physid1),
        accepted_physid2: Some(source_contract.accepted_physid2),
        already_sampled_context: Some(source_contract.already_sampled_context),
        forbidden_targets: Some(source_contract.forbidden_targets),
        operation_order: Some(source_contract.operation_order),
        allowed_classifications: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_RETAINED_RISKS,
        source_evidence: Some(source_contract.source_evidence),
        constructs_mdio_targets: true,
        constructs_man_frames: true,
        constructs_macb_target: false,
        constructs_gpio_target: false,
        runtime_volatile_access_intent: false,
        claims_runtime_volatile_access: false,
        claims_hardware_proof_authorized: false,
        claims_gpio32_action: false,
        claims_bmcr_or_phy_write: false,
        claims_broadcom_shadow_mmd_aux_access: false,
        claims_interrupt_surface_access: false,
        claims_phy_or_mac_configuration: false,
        claims_link_readiness: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_CANDIDATE_CLASSIFICATION,
    }
}

fn rp1_ethernet_bcm54213pe_readonly_preflight_control_evidence(
    report_kind: &'static str,
) -> Rp1EthernetBcm54213peReadonlyPreflightReportEvidence {
    Rp1EthernetBcm54213peReadonlyPreflightReportEvidence {
        report_contract_id: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REPORT_CONTRACT_ID,
        source_task_id: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_TASK_ID,
        report_kind,
        selected_discriminator: None,
        purpose: None,
        selected_targets: None,
        controller: None,
        compatible: None,
        phy_handle: None,
        phy_node: None,
        phy_address: None,
        register_names: None,
        registers: None,
        man_frames: None,
        man_frame_construction: None,
        accepted_phy_id_model: None,
        accepted_physid1: None,
        accepted_physid2: None,
        already_sampled_context: None,
        forbidden_targets: None,
        operation_order: None,
        allowed_classifications: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALLOWED_CLASSIFICATIONS,
        boundary_classification: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_BOUNDARY_CLASSIFICATION,
        rejected_runtime_claims: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REJECTED_RUNTIME_CLAIMS,
        retained_risks: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_RETAINED_RISKS,
        source_evidence: None,
        constructs_mdio_targets: false,
        constructs_man_frames: false,
        constructs_macb_target: false,
        constructs_gpio_target: false,
        runtime_volatile_access_intent: false,
        claims_runtime_volatile_access: false,
        claims_hardware_proof_authorized: false,
        claims_gpio32_action: false,
        claims_bmcr_or_phy_write: false,
        claims_broadcom_shadow_mmd_aux_access: false,
        claims_interrupt_surface_access: false,
        claims_phy_or_mac_configuration: false,
        claims_link_readiness: false,
        claims_packet_io: false,
        claims_networking: false,
        claims_sockets: false,
        claims_ssh: false,
        claims_phase_12_2: false,
        claims_phase_transition: false,
        classification: RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_CONTROL_CLASSIFICATION,
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

    fn accepted_prereq_ownership_input() -> Rp1EthernetPrereqOwnershipReportInput {
        Rp1EthernetPrereqOwnershipReportInput {
            kind: Rp1EthernetPrereqOwnershipReportKind::Candidate,
            source_contract: Some(rp1_ethernet_prereq_ownership_source_contract_evidence()),
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_rp1_mmio_writes: false,
            claims_clock_reset_ownership: false,
            claims_gpio32_phy_reset_ownership: false,
            claims_mdio_phy_ownership: false,
            claims_interrupt_ownership: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_ethernet_clock_reset_guard_input() -> Rp1EthernetClockResetGuardReportInput {
        Rp1EthernetClockResetGuardReportInput {
            kind: Rp1EthernetClockResetGuardReportKind::Candidate,
            guard_contract: Some(rp1_ethernet_clock_reset_guard_contract_evidence()),
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_rp1_mmio_writes: false,
            claims_clock_reset_writes: false,
            claims_clock_reset_ownership: false,
            claims_rp1_clk_sys_transition: false,
            claims_reset_controller_ownership: false,
            claims_gpio32_phy_reset_ownership: false,
            claims_mdio_phy_ownership: false,
            claims_interrupt_ownership: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_ethernet_clock_reset_write_restore_input()
    -> Rp1EthernetClockResetWriteRestoreReportInput {
        Rp1EthernetClockResetWriteRestoreReportInput {
            kind: Rp1EthernetClockResetWriteRestoreReportKind::Candidate,
            target_contract: Some(rp1_ethernet_clock_reset_write_target_contract_evidence()),
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_unscoped_rp1_mmio_writes: false,
            claims_rp1_clk_sys_transition: false,
            claims_clk_eth_ctrl_write: false,
            claims_reset_controller_ownership: false,
            claims_gpio32_phy_reset_ownership: false,
            claims_mdio_phy_ownership: false,
            claims_interrupt_ownership: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_ethernet_clk_eth_ctrl_write_restore_input()
    -> Rp1EthernetClkEthCtrlWriteRestoreReportInput {
        Rp1EthernetClkEthCtrlWriteRestoreReportInput {
            kind: Rp1EthernetClockResetWriteRestoreReportKind::Candidate,
            target_contract: Some(rp1_ethernet_clk_eth_ctrl_write_target_contract_evidence()),
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_unscoped_rp1_mmio_writes: false,
            claims_shared_clock_write: false,
            claims_tsu_same_shape_retry: false,
            claims_non_idempotent_transition: false,
            claims_reset_controller_ownership: false,
            claims_gpio32_phy_reset_ownership: false,
            claims_mdio_phy_ownership: false,
            claims_interrupt_ownership: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_gpio32_phy_reset_preflight_input() -> Rp1EthernetGpio32PhyResetPreflightReportInput
    {
        Rp1EthernetGpio32PhyResetPreflightReportInput {
            kind: Rp1EthernetGpio32PhyResetPreflightReportKind::Candidate,
            source_contract: Some(rp1_ethernet_gpio32_phy_reset_source_contract_evidence()),
            claims_gpio_ownership: false,
            claims_phy_reset_assertion: false,
            claims_phy_reset_deassertion: false,
            claims_mdio_phy_ownership: false,
            claims_runtime_writes: false,
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_interrupt_ownership: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_gpio32_phy_reset_write_restore_guard_input()
    -> Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
        Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
            kind: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::Candidate,
            guard_contract: Some(
                rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract_evidence(),
            ),
            blocked_no_write_classification: None,
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_non_gpio32_write: false,
            claims_mdio_phy_ownership: false,
            claims_interrupt_ownership: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_gpio32_event_state_discriminator_input()
    -> Rp1EthernetGpio32EventStateDiscriminatorReportInput {
        Rp1EthernetGpio32EventStateDiscriminatorReportInput {
            kind: Rp1EthernetGpio32EventStateDiscriminatorReportKind::Candidate,
            source_contract: Some(rp1_ethernet_gpio32_event_state_source_contract_evidence()),
            status_raw: Some(RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_STATUS_RAW),
            ctrl_raw: Some(RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_CTRL_RAW),
            rio1_out_raw: Some(RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OUT_RAW),
            rio1_oe_raw: Some(RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_OE_RAW),
            rio1_in_raw: Some(RP1_ETHERNET_GPIO32_EVENT_STATE_ACCEPTED_V2_RIO1_IN_RAW),
            pad_raw: Some(0),
            source_decoding_status:
                Rp1EthernetGpio32EventStateSourceDecodingStatus::SourceBackedBits20To27,
            event_state_classification: RP1_ETHERNET_GPIO32_EVENT_STATE_CANDIDATE_CLASSIFICATION,
            claims_event_clearing: false,
            claims_gpio_rio_pad_mmio_write: false,
            claims_gpio32_ownership: false,
            claims_phy_reset_assertion: false,
            claims_phy_reset_deassertion: false,
            claims_gpio32_write_restore_retry: false,
            claims_mdio_phy_ownership: false,
            claims_ethernet_ready: false,
            claims_interrupt_ownership: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_gpio32_event_clear_guard_input() -> Rp1EthernetGpio32EventClearGuardReportInput {
        Rp1EthernetGpio32EventClearGuardReportInput {
            kind: Rp1EthernetGpio32EventClearGuardReportKind::Candidate,
            source_contract: Some(rp1_ethernet_gpio32_event_clear_source_contract_evidence()),
            claims_event_clear_executed: false,
            claims_volatile_mmio_execution: false,
            claims_write_outside_irqreset: false,
            claims_ctrl_rw_clr_xor_write: false,
            claims_rio_pad_function_mutation: false,
            claims_gpio32_ownership: false,
            claims_phy_reset_assertion: false,
            claims_phy_reset_deassertion: false,
            claims_gpio32_write_restore_retry: false,
            claims_mdio_phy_ownership: false,
            claims_ethernet_ready: false,
            claims_interrupt_completion: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_mdio_phy_id_guard_input() -> Rp1EthernetMdioPhyIdGuardReportInput {
        Rp1EthernetMdioPhyIdGuardReportInput {
            kind: Rp1EthernetMdioPhyIdGuardReportKind::Candidate,
            source_contract: Some(rp1_ethernet_mdio_phy_id_source_contract_evidence()),
            claims_runtime_mdio_transaction: false,
            claims_mdio_phy_ownership: false,
            claims_mpe_write_permission: false,
            claims_gpio32_phy_reset_ownership: false,
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_interrupt_completion: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_mdio_phy_id_after_mpe_guard_input() -> Rp1EthernetMdioPhyIdAfterMpeGuardReportInput
    {
        Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
            kind: Rp1EthernetMdioPhyIdAfterMpeGuardReportKind::Candidate,
            source_contract: Some(rp1_ethernet_mdio_phy_id_after_mpe_source_contract_evidence()),
            claims_runtime_volatile_load_store: false,
            claims_ncr_write: false,
            claims_missing_mpe_gating: false,
            claims_wrong_mdio_targets: false,
            claims_unbounded_polling: false,
            claims_man_write_without_mpe_precondition: false,
            claims_broad_mdio_phy_ownership: false,
            claims_gpio32_phy_reset_action: false,
            claims_ethernet_ready: false,
            claims_interrupt_completion: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_mdio_mpe_enable_guard_input() -> Rp1EthernetMdioMpeEnableGuardReportInput {
        Rp1EthernetMdioMpeEnableGuardReportInput {
            kind: Rp1EthernetMdioMpeEnableGuardReportKind::Candidate,
            source_contract: Some(rp1_ethernet_mdio_mpe_enable_source_contract_evidence()),
            claims_runtime_ncr_write_executed: false,
            claims_man_write: false,
            claims_phy_id_read: false,
            claims_broad_mdio_phy_ownership: false,
            claims_phy_reset_ownership: false,
            claims_gpio32_ownership: false,
            claims_ethernet_ready: false,
            claims_broad_mmio_ready: false,
            claims_dma_descriptor_ownership: false,
            claims_interrupt_completion: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_mdio_register_vector_guard_input() -> Rp1EthernetMdioRegisterVectorGuardReportInput
    {
        Rp1EthernetMdioRegisterVectorGuardReportInput {
            kind: Rp1EthernetMdioRegisterVectorGuardReportKind::Candidate,
            source_contract: Some(rp1_ethernet_mdio_register_vector_source_contract_evidence()),
            claims_runtime_volatile_load_store: false,
            claims_ncr_write: false,
            claims_missing_mpe_gating: false,
            claims_wrong_mdio_targets: false,
            claims_unbounded_polling: false,
            claims_man_write_without_mpe_precondition: false,
            claims_phy_absence_from_all_ones: false,
            claims_broad_mdio_phy_ownership: false,
            claims_gpio32_phy_reset_action: false,
            claims_ethernet_ready: false,
            claims_interrupt_completion: false,
            claims_dma_descriptor_ownership: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    fn accepted_bcm54213pe_readonly_preflight_report_input()
    -> Rp1EthernetBcm54213peReadonlyPreflightReportInput {
        Rp1EthernetBcm54213peReadonlyPreflightReportInput {
            kind: Rp1EthernetBcm54213peReadonlyPreflightReportKind::Candidate,
            source_contract: Some(
                rp1_ethernet_bcm54213pe_readonly_preflight_report_contract_evidence(),
            ),
            claims_runtime_volatile_access: false,
            claims_hardware_proof_authorized: false,
            claims_gpio32_action: false,
            claims_bmcr_or_phy_write: false,
            claims_broadcom_shadow_mmd_aux_access: false,
            claims_interrupt_surface_access: false,
            claims_phy_or_mac_configuration: false,
            claims_link_readiness: false,
            claims_packet_io: false,
            claims_networking: false,
            claims_sockets: false,
            claims_ssh: false,
            claims_phase_12_2: false,
            claims_phase_transition: false,
        }
    }

    #[test_case]
    fn rp1_ethernet_clock_reset_guard_formats_candidate_report() {
        let report = build_rp1_ethernet_clock_reset_guard_report(
            accepted_ethernet_clock_reset_guard_input(),
        )
        .expect("valid clock/reset guard candidate input");
        let evidence = rp1_ethernet_clock_reset_guard_report_evidence(report);

        assert_eq!(
            evidence.guard_contract_id,
            RP1_ETHERNET_CLOCK_RESET_GUARD_CONTRACT_ID
        );
        assert_eq!(
            evidence.ownership_contract_task_id,
            RP1_ETHERNET_CLOCK_RESET_OWNERSHIP_CONTRACT_TASK_ID
        );
        assert_eq!(
            evidence.prereq_contract_id,
            RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(evidence.observed_identity_target, Some(0x1c_0010_00fc));
        assert_eq!(evidence.translated_comparator_target, Some(0x1f_0010_00fc));
        assert_eq!(evidence.accepted_macb_mid_raw, Some(0x0007_0109));
        assert_eq!(evidence.accepted_macb_mid_idnum, Some(0x7));
        assert_eq!(evidence.accepted_macb_mid_rev, Some(0x109));
        assert_eq!(
            evidence.identity_role,
            Some(RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE)
        );
        assert_eq!(evidence.clock_names, Some(RP1_ETHERNET_CLOCK_NAMES));
        assert_eq!(evidence.clock_sources, Some(RP1_ETHERNET_CLOCK_SOURCES));
        assert_eq!(evidence.clock_ids, Some(RP1_ETHERNET_CLOCK_IDS));
        assert_eq!(
            evidence.shared_clock_names,
            Some(RP1_ETHERNET_SHARED_CLOCK_NAMES)
        );
        assert_eq!(
            evidence.shared_clock_source,
            Some(RP1_ETHERNET_SHARED_CLOCK_SOURCE)
        );
        assert_eq!(evidence.shared_clock_id, Some(12));
        assert_eq!(
            evidence.ethernet_private_clock_names,
            Some(RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_NAMES)
        );
        assert_eq!(
            evidence.ethernet_private_clock_sources,
            Some(RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_SOURCES)
        );
        assert_eq!(
            evidence.ethernet_private_clock_ids,
            Some(RP1_ETHERNET_ETHERNET_PRIVATE_CLOCK_IDS)
        );
        assert_eq!(
            evidence.clock_policy_classification,
            Some(RP1_ETHERNET_CLOCK_POLICY_CLASSIFICATION)
        );
        assert_eq!(
            evidence.reset_controller_policy_classification,
            Some(RP1_ETHERNET_RESET_CONTROLLER_POLICY_CLASSIFICATION)
        );
        assert_eq!(evidence.phy_reset_gpio, Some(32));
        assert_eq!(
            evidence.phy_mdio_policy_classification,
            Some(RP1_ETHERNET_PHY_MDIO_POLICY_CLASSIFICATION)
        );
        assert_eq!(
            evidence.read_only_baseline_requirements,
            Some(RP1_ETHERNET_CLOCK_RESET_GUARD_READ_ONLY_BASELINE_REQUIREMENTS)
        );
        assert_eq!(
            evidence.write_backed_invariants,
            Some(RP1_ETHERNET_CLOCK_RESET_GUARD_WRITE_BACKED_INVARIANTS)
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE)
        );
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_CLOCK_RESET_GUARD_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_CLOCK_RESET_GUARD_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_CLOCK_RESET_GUARD_RETAINED_RISKS
        );
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_broad_mmio_ready);
        assert!(!evidence.claims_rp1_mmio_writes);
        assert!(!evidence.claims_clock_reset_writes);
        assert!(!evidence.claims_clock_reset_ownership);
        assert!(!evidence.claims_rp1_clk_sys_transition);
        assert!(!evidence.claims_reset_controller_ownership);
        assert!(!evidence.claims_gpio32_phy_reset_ownership);
        assert!(!evidence.claims_mdio_phy_ownership);
        assert!(!evidence.claims_interrupt_ownership);
        assert!(!evidence.claims_dma_descriptor_ownership);
        assert!(!evidence.claims_packet_io);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_sockets);
        assert!(!evidence.claims_ssh);
        assert!(!evidence.claims_phase_12_2);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_CLOCK_RESET_GUARD_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_clock_reset_guard_formats_paired_control() {
        let report =
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                kind: Rp1EthernetClockResetGuardReportKind::NoClockResetNoEthernetControl,
                guard_contract: None,
                ..accepted_ethernet_clock_reset_guard_input()
            })
            .expect("valid clock/reset guard control input");
        let evidence = rp1_ethernet_clock_reset_guard_report_evidence(report);

        assert_eq!(
            evidence.guard_contract_id,
            RP1_ETHERNET_CLOCK_RESET_GUARD_CONTRACT_ID
        );
        assert_eq!(
            evidence.ownership_contract_task_id,
            RP1_ETHERNET_CLOCK_RESET_OWNERSHIP_CONTRACT_TASK_ID
        );
        assert_eq!(evidence.report_kind, "no-clock-reset-no-ethernet-control");
        assert_eq!(evidence.observed_identity_target, None);
        assert_eq!(evidence.accepted_macb_mid_raw, None);
        assert_eq!(evidence.clock_names, None);
        assert_eq!(evidence.clock_sources, None);
        assert_eq!(evidence.clock_ids, None);
        assert_eq!(evidence.shared_clock_names, None);
        assert_eq!(evidence.shared_clock_source, None);
        assert_eq!(evidence.ethernet_private_clock_names, None);
        assert_eq!(evidence.ethernet_private_clock_sources, None);
        assert_eq!(evidence.reset_controller_policy_classification, None);
        assert_eq!(evidence.phy_reset_gpio, None);
        assert_eq!(evidence.read_only_baseline_requirements, None);
        assert_eq!(evidence.write_backed_invariants, None);
        assert_eq!(evidence.source_evidence, None);
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_CLOCK_RESET_GUARD_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_CLOCK_RESET_GUARD_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_CLOCK_RESET_GUARD_RETAINED_RISKS
        );
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_CLOCK_RESET_GUARD_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_clock_reset_guard_rejects_shape_bypass() {
        let input = accepted_ethernet_clock_reset_guard_input();

        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                guard_contract: None,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::CandidateMissingGuardContract)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                kind: Rp1EthernetClockResetGuardReportKind::NoClockResetNoEthernetControl,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::ControlCarriesClockResetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                guard_contract: Some(Rp1EthernetClockResetGuardContractEvidence {
                    guard_contract_id: "wrong-contract",
                    ..rp1_ethernet_clock_reset_guard_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::GuardContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                guard_contract: Some(Rp1EthernetClockResetGuardContractEvidence {
                    observed_identity_target: 0x1f_0010_00fc,
                    ..rp1_ethernet_clock_reset_guard_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::GuardContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                guard_contract: Some(Rp1EthernetClockResetGuardContractEvidence {
                    shared_clock_id: 16,
                    ..rp1_ethernet_clock_reset_guard_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::GuardContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                guard_contract: Some(Rp1EthernetClockResetGuardContractEvidence {
                    source_evidence: &[],
                    ..rp1_ethernet_clock_reset_guard_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::MissingSourceEvidence)
        );
    }

    #[test_case]
    fn rp1_ethernet_clock_reset_guard_rejects_overclaims() {
        let input = accepted_ethernet_clock_reset_guard_input();

        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_ethernet_ready: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_broad_mmio_ready: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::BroadMmioReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_rp1_mmio_writes: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::Rp1MmioWritesClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_clock_reset_writes: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::ClockResetWritesClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_clock_reset_ownership: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::ClockResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_rp1_clk_sys_transition: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::Rp1ClkSysTransitionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_reset_controller_ownership: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::ResetControllerOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_gpio32_phy_reset_ownership: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::Gpio32PhyResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_mdio_phy_ownership: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_interrupt_ownership: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::InterruptOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_dma_descriptor_ownership: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_packet_io: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_networking: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_sockets: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::SocketsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_ssh: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_phase_12_2: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::Phase122Claim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_guard_report(Rp1EthernetClockResetGuardReportInput {
                claims_phase_transition: true,
                ..input
            }),
            Err(Rp1EthernetClockResetGuardReportError::PhaseTransitionClaim)
        );
        assert_eq!(
            rejected_rp1_ethernet_clock_reset_guard_report_evidence(
                Rp1EthernetClockResetGuardReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
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
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_OBSERVED_WINDOW_RETAINED_RISKS
        );
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
    fn rp1_ethernet_prereq_ownership_formats_candidate_report() {
        let report = build_rp1_ethernet_prereq_ownership_report(accepted_prereq_ownership_input())
            .expect("valid prerequisite ownership candidate input");
        let evidence = rp1_ethernet_prereq_ownership_report_evidence(report);

        assert_eq!(
            evidence.contract_id,
            RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.selected_prerequisite,
            Some(RP1_ETHERNET_SELECTED_PREREQUISITE)
        );
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(evidence.rp1_bus_base, Some(0xc0_4010_0000));
        assert_eq!(evidence.rp1_bus_window_size, Some(0x4000));
        assert_eq!(evidence.observed_identity_target, Some(0x1c_0010_00fc));
        assert_eq!(evidence.translated_comparator_target, Some(0x1f_0010_00fc));
        assert_eq!(evidence.accepted_macb_mid_raw, Some(0x0007_0109));
        assert_eq!(evidence.accepted_macb_mid_idnum, Some(0x7));
        assert_eq!(evidence.accepted_macb_mid_rev, Some(0x109));
        assert_eq!(
            evidence.identity_role,
            Some(RP1_ETHERNET_ACCEPTED_IDENTITY_ROLE)
        );
        assert_eq!(evidence.interrupt_name, Some("RP1_INT_ETH"));
        assert_eq!(evidence.interrupt_number, Some(6));
        assert_eq!(evidence.clock_names, Some(RP1_ETHERNET_CLOCK_NAMES));
        assert_eq!(evidence.clock_sources, Some(RP1_ETHERNET_CLOCK_SOURCES));
        assert_eq!(evidence.clock_ids, Some(RP1_ETHERNET_CLOCK_IDS));
        assert_eq!(
            evidence.clock_policy_classification,
            Some("no-clock-reset-ownership")
        );
        assert_eq!(evidence.phy_mode, Some("rgmii-id"));
        assert_eq!(evidence.phy_handle, Some("phy1"));
        assert_eq!(evidence.phy_node, Some("ethernet-phy@1"));
        assert_eq!(evidence.phy_reg, Some(0x1));
        assert_eq!(evidence.phy_reset_gpio, Some(32));
        assert_eq!(evidence.phy_reset_active_low, Some(true));
        assert_eq!(evidence.phy_reset_duration_ms, Some(5));
        assert_eq!(
            evidence.phy_mdio_policy_classification,
            Some("no-phy-reset-or-mdio-ownership")
        );
        assert_eq!(
            evidence.dma_descriptor_policy_classification,
            Some("no-live-dma-or-descriptor-ownership")
        );
        assert_eq!(
            evidence.cadence_rp1_config,
            Some(RP1_ETHERNET_CADENCE_RP1_CONFIG)
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_PREREQ_OWNERSHIP_SOURCE_EVIDENCE)
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_PREREQ_OWNERSHIP_HARDWARE_PROOF_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_PREREQ_OWNERSHIP_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_PREREQ_OWNERSHIP_RETAINED_RISKS
        );
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_broad_mmio_ready);
        assert!(!evidence.claims_rp1_mmio_writes);
        assert!(!evidence.claims_clock_reset_ownership);
        assert!(!evidence.claims_gpio32_phy_reset_ownership);
        assert!(!evidence.claims_mdio_phy_ownership);
        assert!(!evidence.claims_interrupt_ownership);
        assert!(!evidence.claims_dma_descriptor_ownership);
        assert!(!evidence.claims_packet_io);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_sockets);
        assert!(!evidence.claims_ssh);
        assert!(!evidence.claims_phase_12_2);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_PREREQ_OWNERSHIP_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_prereq_ownership_formats_paired_control() {
        let report =
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                kind: Rp1EthernetPrereqOwnershipReportKind::NoOwnershipNoEthernetControl,
                source_contract: None,
                ..accepted_prereq_ownership_input()
            })
            .expect("valid prerequisite ownership control input");
        let evidence = rp1_ethernet_prereq_ownership_report_evidence(report);

        assert_eq!(
            evidence.contract_id,
            RP1_ETHERNET_PREREQ_OWNERSHIP_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-ownership-no-ethernet-control");
        assert_eq!(evidence.selected_prerequisite, None);
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.compatible, None);
        assert_eq!(evidence.rp1_bus_base, None);
        assert_eq!(evidence.observed_identity_target, None);
        assert_eq!(evidence.translated_comparator_target, None);
        assert_eq!(evidence.accepted_macb_mid_raw, None);
        assert_eq!(evidence.interrupt_number, None);
        assert_eq!(evidence.clock_names, None);
        assert_eq!(evidence.clock_ids, None);
        assert_eq!(evidence.phy_reset_gpio, None);
        assert_eq!(evidence.phy_mdio_policy_classification, None);
        assert_eq!(evidence.dma_descriptor_policy_classification, None);
        assert_eq!(evidence.source_evidence, None);
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_PREREQ_OWNERSHIP_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_PREREQ_OWNERSHIP_RETAINED_RISKS
        );
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_PREREQ_OWNERSHIP_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_prereq_ownership_rejects_shape_bypass() {
        let input = accepted_prereq_ownership_input();

        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                source_contract: None,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                kind: Rp1EthernetPrereqOwnershipReportKind::NoOwnershipNoEthernetControl,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::ControlCarriesEthernetPrereqFacts)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                source_contract: Some(Rp1EthernetPrereqOwnershipSourceContractEvidence {
                    contract_id: "wrong-contract",
                    ..rp1_ethernet_prereq_ownership_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                source_contract: Some(Rp1EthernetPrereqOwnershipSourceContractEvidence {
                    observed_identity_target: 0x1f_0010_00fc,
                    ..rp1_ethernet_prereq_ownership_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                source_contract: Some(Rp1EthernetPrereqOwnershipSourceContractEvidence {
                    phy_reset_gpio: 33,
                    ..rp1_ethernet_prereq_ownership_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                source_contract: Some(Rp1EthernetPrereqOwnershipSourceContractEvidence {
                    source_evidence: &[],
                    ..rp1_ethernet_prereq_ownership_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::MissingSourceEvidence)
        );
    }

    #[test_case]
    fn rp1_ethernet_prereq_ownership_rejects_overclaims() {
        let input = accepted_prereq_ownership_input();

        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_ethernet_ready: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_broad_mmio_ready: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::BroadMmioReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_rp1_mmio_writes: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::Rp1MmioWritesClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_clock_reset_ownership: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::ClockResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_gpio32_phy_reset_ownership: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::Gpio32PhyResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_mdio_phy_ownership: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_interrupt_ownership: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::InterruptOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_dma_descriptor_ownership: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_packet_io: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_networking: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_sockets: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::SocketsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_ssh: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_phase_12_2: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::Phase122Claim)
        );
        assert_eq!(
            build_rp1_ethernet_prereq_ownership_report(Rp1EthernetPrereqOwnershipReportInput {
                claims_phase_transition: true,
                ..input
            }),
            Err(Rp1EthernetPrereqOwnershipReportError::PhaseTransitionClaim)
        );
        assert_eq!(
            rejected_rp1_ethernet_prereq_ownership_report_evidence(
                Rp1EthernetPrereqOwnershipReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
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

    #[test_case]
    fn rp1_ethernet_clock_reset_write_restore_formats_candidate_report() {
        let report = build_rp1_ethernet_clock_reset_write_restore_report(
            accepted_ethernet_clock_reset_write_restore_input(),
        )
        .expect("valid write/restore candidate input");
        let evidence = rp1_ethernet_clock_reset_write_restore_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.target_contract_id,
            RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.target,
            Some(RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_TARGET)
        );
        assert_eq!(
            evidence.clock_name,
            Some(RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CLOCK_NAME)
        );
        assert_eq!(evidence.clock_id, Some(29));
        assert_eq!(
            evidence.register,
            Some(RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REGISTER)
        );
        assert_eq!(evidence.observed_rp1_base, Some(0x1c_0000_0000));
        assert_eq!(evidence.source_offset, Some(0x018134));
        assert_eq!(evidence.cpu_physical_target, Some(0x1c_0001_8134));
        assert_eq!(evidence.width_bits, Some(32));
        assert_eq!(
            evidence.allowed_write_value,
            Some(RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_ALLOWED_WRITE_VALUE)
        );
        assert_eq!(
            evidence.preserved_fields,
            Some(RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_PRESERVED_FIELDS)
        );
        assert_eq!(
            evidence.operation_sequence,
            Some(RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_OPERATION_SEQUENCE)
        );
        assert_eq!(evidence.post_eq_pre_required, Some(true));
        assert_eq!(evidence.restore_eq_pre_required, Some(true));
        assert_eq!(
            evidence.future_proof_classifications,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_RETAINED_RISKS
        );
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_broad_mmio_ready);
        assert!(!evidence.claims_unscoped_rp1_mmio_writes);
        assert!(!evidence.claims_rp1_clk_sys_transition);
        assert!(!evidence.claims_clk_eth_ctrl_write);
        assert!(!evidence.claims_reset_controller_ownership);
        assert!(!evidence.claims_gpio32_phy_reset_ownership);
        assert!(!evidence.claims_mdio_phy_ownership);
        assert!(!evidence.claims_interrupt_ownership);
        assert!(!evidence.claims_dma_descriptor_ownership);
        assert!(!evidence.claims_packet_io);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_sockets);
        assert!(!evidence.claims_ssh);
        assert!(!evidence.claims_phase_12_2);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_clock_reset_write_restore_formats_paired_control() {
        let report = build_rp1_ethernet_clock_reset_write_restore_report(
            Rp1EthernetClockResetWriteRestoreReportInput {
                kind: Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl,
                target_contract: None,
                ..accepted_ethernet_clock_reset_write_restore_input()
            },
        )
        .expect("valid write/restore control input");
        let evidence = rp1_ethernet_clock_reset_write_restore_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.target_contract_id,
            RP1_ETHERNET_CLOCK_RESET_WRITE_TARGET_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-clock-write-no-ethernet-control");
        assert_eq!(evidence.target, None);
        assert_eq!(evidence.clock_name, None);
        assert_eq!(evidence.clock_id, None);
        assert_eq!(evidence.register, None);
        assert_eq!(evidence.observed_rp1_base, None);
        assert_eq!(evidence.source_offset, None);
        assert_eq!(evidence.cpu_physical_target, None);
        assert_eq!(evidence.allowed_write_value, None);
        assert_eq!(evidence.preserved_fields, None);
        assert_eq!(evidence.operation_sequence, None);
        assert_eq!(evidence.safety_invariants, None);
        assert_eq!(evidence.post_eq_pre_required, None);
        assert_eq!(evidence.restore_eq_pre_required, None);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_CLOCK_RESET_WRITE_RESTORE_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_clock_reset_write_restore_rejects_shape_bypass() {
        let input = accepted_ethernet_clock_reset_write_restore_input();

        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    target_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::CandidateMissingTargetContract)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    kind:
                        Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::ControlCarriesWriteTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    target_contract: Some(Rp1EthernetClockResetWriteTargetContractEvidence {
                        contract_id: "wrong-contract",
                        ..rp1_ethernet_clock_reset_write_target_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::TargetContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    target_contract: Some(Rp1EthernetClockResetWriteTargetContractEvidence {
                        register: "CLK_ETH_CTRL",
                        ..rp1_ethernet_clock_reset_write_target_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::TargetContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    target_contract: Some(Rp1EthernetClockResetWriteTargetContractEvidence {
                        allowed_write_value: "enable-bit-toggle",
                        ..rp1_ethernet_clock_reset_write_target_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::TargetContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    target_contract: Some(Rp1EthernetClockResetWriteTargetContractEvidence {
                        source_evidence: &[],
                        ..rp1_ethernet_clock_reset_write_target_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::MissingSourceEvidence)
        );
    }

    #[test_case]
    fn rp1_ethernet_clock_reset_write_restore_rejects_forbidden_claims() {
        let input = accepted_ethernet_clock_reset_write_restore_input();

        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_rp1_clk_sys_transition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::Rp1ClkSysTransitionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_clk_eth_ctrl_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::ClkEthCtrlWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_reset_controller_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::ResetControllerOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_gpio32_phy_reset_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::Gpio32PhyResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_mdio_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_dma_descriptor_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_interrupt_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::InterruptOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_packet_io: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_networking: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_ssh: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::Phase122Claim)
        );
        assert_eq!(
            build_rp1_ethernet_clock_reset_write_restore_report(
                Rp1EthernetClockResetWriteRestoreReportInput {
                    claims_phase_transition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClockResetWriteRestoreReportError::PhaseTransitionClaim)
        );
        assert_eq!(
            rejected_rp1_ethernet_clock_reset_write_restore_report_evidence(
                Rp1EthernetClockResetWriteRestoreReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_clk_eth_ctrl_write_restore_formats_candidate_report() {
        let report = build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
            accepted_ethernet_clk_eth_ctrl_write_restore_input(),
        )
        .expect("valid CLK_ETH_CTRL write/restore candidate input");
        let evidence = rp1_ethernet_clk_eth_ctrl_write_restore_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.target_contract_id,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.target,
            Some(RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_TARGET)
        );
        assert_eq!(
            evidence.clock_name,
            Some(RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CLOCK_NAME)
        );
        assert_eq!(evidence.clock_id, Some(16));
        assert_eq!(
            evidence.register,
            Some(RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REGISTER)
        );
        assert_eq!(evidence.observed_rp1_base, Some(0x1c_0000_0000));
        assert_eq!(evidence.source_offset, Some(0x018064));
        assert_eq!(evidence.cpu_physical_target, Some(0x1c_0001_8064));
        assert_eq!(evidence.width_bits, Some(32));
        assert_eq!(
            evidence.allowed_write_value,
            Some(RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_ALLOWED_WRITE_VALUE)
        );
        assert_eq!(
            evidence.preserved_fields,
            Some(RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_PRESERVED_FIELDS)
        );
        assert_eq!(
            evidence.operation_sequence,
            Some(RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_OPERATION_SEQUENCE)
        );
        assert_eq!(evidence.post_eq_pre_required, Some(true));
        assert_eq!(evidence.restore_eq_pre_required, Some(true));
        assert_eq!(
            evidence.future_proof_classifications,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_FUTURE_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_RETAINED_RISKS
        );
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_broad_mmio_ready);
        assert!(!evidence.claims_unscoped_rp1_mmio_writes);
        assert!(!evidence.claims_shared_clock_write);
        assert!(!evidence.claims_tsu_same_shape_retry);
        assert!(!evidence.claims_non_idempotent_transition);
        assert!(!evidence.claims_reset_controller_ownership);
        assert!(!evidence.claims_gpio32_phy_reset_ownership);
        assert!(!evidence.claims_mdio_phy_ownership);
        assert!(!evidence.claims_interrupt_ownership);
        assert!(!evidence.claims_dma_descriptor_ownership);
        assert!(!evidence.claims_packet_io);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_sockets);
        assert!(!evidence.claims_ssh);
        assert!(!evidence.claims_phase_12_2);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_clk_eth_ctrl_write_restore_formats_paired_control() {
        let report = build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
            Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                kind: Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl,
                target_contract: None,
                ..accepted_ethernet_clk_eth_ctrl_write_restore_input()
            },
        )
        .expect("valid CLK_ETH_CTRL write/restore control input");
        let evidence = rp1_ethernet_clk_eth_ctrl_write_restore_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.target_contract_id,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_TARGET_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-clock-write-no-ethernet-control");
        assert_eq!(evidence.target, None);
        assert_eq!(evidence.clock_name, None);
        assert_eq!(evidence.clock_id, None);
        assert_eq!(evidence.register, None);
        assert_eq!(evidence.observed_rp1_base, None);
        assert_eq!(evidence.source_offset, None);
        assert_eq!(evidence.cpu_physical_target, None);
        assert_eq!(evidence.allowed_write_value, None);
        assert_eq!(evidence.preserved_fields, None);
        assert_eq!(evidence.operation_sequence, None);
        assert_eq!(evidence.safety_invariants, None);
        assert_eq!(evidence.post_eq_pre_required, None);
        assert_eq!(evidence.restore_eq_pre_required, None);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_CLK_ETH_CTRL_WRITE_RESTORE_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_clk_eth_ctrl_write_restore_rejects_shape_and_overclaims() {
        let input = accepted_ethernet_clk_eth_ctrl_write_restore_input();

        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    target_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::CandidateMissingTargetContract)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    kind:
                        Rp1EthernetClockResetWriteRestoreReportKind::NoClockWriteNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::ControlCarriesWriteTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    target_contract: Some(Rp1EthernetClockResetWriteTargetContractEvidence {
                        register: "CLK_ETH_TSU_CTRL",
                        ..rp1_ethernet_clk_eth_ctrl_write_target_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::TargetContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    target_contract: Some(Rp1EthernetClockResetWriteTargetContractEvidence {
                        allowed_write_value: "enable-bit-toggle",
                        ..rp1_ethernet_clk_eth_ctrl_write_target_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::TargetContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    claims_shared_clock_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::SharedClockWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    claims_tsu_same_shape_retry: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::TsuSameShapeRetryClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    claims_non_idempotent_transition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::NonIdempotentTransitionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    claims_dma_descriptor_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_clk_eth_ctrl_write_restore_report(
                Rp1EthernetClkEthCtrlWriteRestoreReportInput {
                    claims_phase_transition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetClkEthCtrlWriteRestoreReportError::PhaseTransitionClaim)
        );
        assert_eq!(
            rejected_rp1_ethernet_clk_eth_ctrl_write_restore_report_evidence(
                Rp1EthernetClkEthCtrlWriteRestoreReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_phy_reset_preflight_formats_candidate_report() {
        let report = build_rp1_ethernet_gpio32_phy_reset_preflight_report(
            accepted_gpio32_phy_reset_preflight_input(),
        )
        .expect("valid GPIO32 PHY-reset preflight candidate input");
        let evidence = rp1_ethernet_gpio32_phy_reset_preflight_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.accepted_input_frontier,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_ACCEPTED_INPUT_FRONTIER)
        );
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(
            evidence.accepted_macb_mid_raw,
            Some(RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW)
        );
        assert_eq!(evidence.phy_mode, Some(RP1_ETHERNET_PHY_MODE));
        assert_eq!(evidence.phy_handle, Some(RP1_ETHERNET_PHY_HANDLE));
        assert_eq!(evidence.phy_node, Some(RP1_ETHERNET_PHY_NODE));
        assert_eq!(evidence.phy_reg, Some(RP1_ETHERNET_PHY_REG));
        assert_eq!(
            evidence.gpio_controller,
            Some(RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER)
        );
        assert_eq!(evidence.gpio_line, Some(32));
        assert_eq!(evidence.reset_route, Some(RP1_ETHERNET_PHY_RESET_ROUTE));
        assert_eq!(evidence.active_low, Some(true));
        assert_eq!(
            evidence.logical_assertion,
            Some(RP1_ETHERNET_PHY_RESET_LOGICAL_ASSERTION)
        );
        assert_eq!(
            evidence.logical_deassertion,
            Some(RP1_ETHERNET_PHY_RESET_LOGICAL_DEASSERTION)
        );
        assert_eq!(evidence.reset_duration_ms, Some(5));
        assert_eq!(
            evidence.mdio_reset_hook_relationship,
            Some(RP1_ETHERNET_PHY_RESET_MDIO_HOOK_RELATIONSHIP)
        );
        assert_eq!(
            evidence.phase11_gpio_constraints,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_PHASE11_GPIO_CONSTRAINTS)
        );
        assert_eq!(
            evidence.future_write_restore_invariants,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_FUTURE_WRITE_RESTORE_INVARIANTS)
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_EVIDENCE)
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_GPIO32_PHY_RESET_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_GPIO32_PHY_RESET_RETAINED_RISKS
        );
        assert!(!evidence.claims_gpio_ownership);
        assert!(!evidence.claims_phy_reset_assertion);
        assert!(!evidence.claims_phy_reset_deassertion);
        assert!(!evidence.claims_mdio_phy_ownership);
        assert!(!evidence.claims_runtime_writes);
        assert!(!evidence.claims_packet_io);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_phy_reset_preflight_formats_paired_control() {
        let report = build_rp1_ethernet_gpio32_phy_reset_preflight_report(
            Rp1EthernetGpio32PhyResetPreflightReportInput {
                kind: Rp1EthernetGpio32PhyResetPreflightReportKind::NoGpioNoEthernetControl,
                source_contract: None,
                ..accepted_gpio32_phy_reset_preflight_input()
            },
        )
        .expect("valid GPIO32 PHY-reset preflight control input");
        let evidence = rp1_ethernet_gpio32_phy_reset_preflight_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_REPORT_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "no-gpio-no-ethernet-control");
        assert_eq!(evidence.accepted_input_frontier, None);
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.compatible, None);
        assert_eq!(evidence.phy_mode, None);
        assert_eq!(evidence.gpio_controller, None);
        assert_eq!(evidence.gpio_line, None);
        assert_eq!(evidence.reset_route, None);
        assert_eq!(evidence.active_low, None);
        assert_eq!(evidence.logical_assertion, None);
        assert_eq!(evidence.logical_deassertion, None);
        assert_eq!(evidence.reset_duration_ms, None);
        assert_eq!(evidence.mdio_reset_hook_relationship, None);
        assert_eq!(evidence.phase11_gpio_constraints, None);
        assert_eq!(evidence.future_write_restore_invariants, None);
        assert_eq!(evidence.source_evidence, None);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GPIO32_PHY_RESET_PREFLIGHT_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_phy_reset_preflight_rejects_shape_and_overclaims() {
        let input = accepted_gpio32_phy_reset_preflight_input();

        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    kind: Rp1EthernetGpio32PhyResetPreflightReportKind::NoGpioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::ControlCarriesGpioPhyResetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    source_contract: Some(Rp1EthernetGpio32PhyResetSourceContractEvidence {
                        gpio_line: 16,
                        ..rp1_ethernet_gpio32_phy_reset_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    source_contract: Some(Rp1EthernetGpio32PhyResetSourceContractEvidence {
                        source_evidence: &[],
                        ..rp1_ethernet_gpio32_phy_reset_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::MissingSourceEvidence)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    claims_gpio_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::GpioOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    claims_phy_reset_assertion: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::PhyResetAssertionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    claims_phy_reset_deassertion: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::PhyResetDeassertionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    claims_mdio_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    claims_runtime_writes: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::RuntimeWritesClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    claims_packet_io: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_preflight_report(
                Rp1EthernetGpio32PhyResetPreflightReportInput {
                    claims_phase_transition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetPreflightReportError::PhaseTransitionClaim)
        );
        assert_eq!(
            rejected_rp1_ethernet_gpio32_phy_reset_preflight_report_evidence(
                Rp1EthernetGpio32PhyResetPreflightReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_phy_reset_write_restore_guard_formats_candidate_report() {
        let report = build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
            accepted_gpio32_phy_reset_write_restore_guard_input(),
        )
        .expect("valid GPIO32 PHY-reset write/restore guard candidate input");
        let evidence = rp1_ethernet_gpio32_phy_reset_write_restore_guard_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.guard_contract_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_GUARD_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_SOURCE_CONTRACT_ID
        );
        assert_eq!(
            evidence.report_source_task_id,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.gpio_controller,
            Some(RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER)
        );
        assert_eq!(evidence.gpio_line, Some(32));
        assert_eq!(evidence.reset_route, Some(RP1_ETHERNET_PHY_RESET_ROUTE));
        assert_eq!(evidence.bank, Some("bank1"));
        assert_eq!(evidence.bank_local_bit, Some(4));
        assert_eq!(evidence.io_bank1_source_base, Some(0xc0_400d_4000));
        assert_eq!(evidence.io_bank1_observed_base, Some(0x1c_000d_4000));
        assert_eq!(evidence.gpio32_status_observed_target, Some(0x1c_000d_4020));
        assert_eq!(evidence.gpio32_ctrl_observed_target, Some(0x1c_000d_4024));
        assert_eq!(evidence.rio1_out_observed_target, Some(0x1c_000e_4000));
        assert_eq!(evidence.rio1_oe_observed_target, Some(0x1c_000e_4004));
        assert_eq!(evidence.rio1_in_observed_target, Some(0x1c_000e_4008));
        assert_eq!(evidence.gpio32_pad_observed_target, Some(0x1c_000f_4014));
        assert_eq!(evidence.width_bits, Some(32));
        assert_eq!(evidence.active_low, Some(true));
        assert_eq!(
            evidence.assertion_raw_output,
            Some(RP1_ETHERNET_GPIO32_ASSERTION_RAW_OUTPUT)
        );
        assert_eq!(
            evidence.deassertion_raw_output,
            Some(RP1_ETHERNET_GPIO32_DEASSERTION_RAW_OUTPUT)
        );
        assert_eq!(evidence.reset_duration_ms, Some(5));
        assert_eq!(
            evidence.no_write_preconditions,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_PRECONDITIONS)
        );
        assert_eq!(
            evidence.restore_baseline_fields,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BASELINE_FIELDS)
        );
        assert_eq!(
            evidence.operation_sequence,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_OPERATION_SEQUENCE)
        );
        assert_eq!(
            evidence.blocked_no_write_classifications,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BLOCKED_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.future_proof_classifications,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_FUTURE_CLASSIFICATIONS
        );
        assert_eq!(evidence.blocked_no_write_classification, None);
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_RETAINED_RISKS
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_SOURCE_EVIDENCE)
        );
        assert!(!evidence.claims_non_gpio32_write);
        assert!(!evidence.claims_mdio_phy_ownership);
        assert!(!evidence.claims_interrupt_ownership);
        assert!(!evidence.claims_dma_descriptor_ownership);
        assert!(!evidence.claims_phase_12_2);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_phy_reset_write_restore_guard_formats_control_and_blocked() {
        let control = build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                kind: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::NoGpioWriteNoEthernetControl,
                guard_contract: None,
                ..accepted_gpio32_phy_reset_write_restore_guard_input()
            },
        )
        .expect("valid GPIO32 PHY-reset write/restore control input");
        let control_evidence =
            rp1_ethernet_gpio32_phy_reset_write_restore_guard_report_evidence(control);

        assert_eq!(
            control_evidence.report_kind,
            "no-gpio-write-no-ethernet-control"
        );
        assert_eq!(control_evidence.gpio_controller, None);
        assert_eq!(control_evidence.gpio_line, None);
        assert_eq!(control_evidence.gpio32_ctrl_observed_target, None);
        assert_eq!(control_evidence.rio1_out_observed_target, None);
        assert_eq!(control_evidence.restore_baseline_fields, None);
        assert_eq!(control_evidence.source_evidence, None);
        assert_eq!(
            control_evidence.classification,
            RP1_ETHERNET_GPIO32_PHY_RESET_WRITE_RESTORE_CONTROL_CLASSIFICATION
        );

        let blocked_classification =
            "rp1-ethernet-gpio32-phy-reset-blocked-missing-restore-baseline";
        let blocked = build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
            Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                kind: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::BlockedNoWrite,
                blocked_no_write_classification: Some(blocked_classification),
                ..accepted_gpio32_phy_reset_write_restore_guard_input()
            },
        )
        .expect("valid GPIO32 PHY-reset blocked/no-write input");
        let blocked_evidence =
            rp1_ethernet_gpio32_phy_reset_write_restore_guard_report_evidence(blocked);

        assert_eq!(blocked_evidence.report_kind, "blocked-no-write");
        assert_eq!(
            blocked_evidence.blocked_no_write_classification,
            Some(blocked_classification)
        );
        assert_eq!(blocked_evidence.classification, blocked_classification);
        assert_eq!(
            blocked_evidence.gpio32_status_observed_target,
            Some(0x1c_000d_4020)
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_phy_reset_write_restore_guard_rejects_shape_and_overclaims() {
        let input = accepted_gpio32_phy_reset_write_restore_guard_input();

        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    guard_contract: None,
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::CandidateMissingGuardContract
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    kind: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::NoGpioWriteNoEthernetControl,
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::ControlCarriesGpioWriteFacts
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    guard_contract: Some(
                        Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence {
                            gpio_line: 33,
                            ..rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract_evidence()
                        },
                    ),
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::GuardContractIdentityMismatch
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    guard_contract: Some(
                        Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence {
                            gpio32_ctrl_observed_target: 0x1f_000d_4024,
                            ..rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract_evidence()
                        },
                    ),
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::GuardContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    guard_contract: Some(
                        Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence {
                            restore_baseline_fields: &[],
                            ..rp1_ethernet_gpio32_phy_reset_write_restore_guard_contract_evidence()
                        },
                    ),
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::MissingRestoreBaseline)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    kind: Rp1EthernetGpio32PhyResetWriteRestoreGuardReportKind::BlockedNoWrite,
                    blocked_no_write_classification: Some("rp1-ethernet-gpio32-phy-reset-write-restored"),
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::BlockedClassificationNotAllowed
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    claims_non_gpio32_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::NonGpio32WriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    claims_mdio_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    claims_interrupt_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::InterruptOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    claims_dma_descriptor_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::Phase122Claim)
        );
        assert_eq!(
            rejected_rp1_ethernet_gpio32_phy_reset_write_restore_guard_report_evidence(
                Rp1EthernetGpio32PhyResetWriteRestoreGuardReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_event_state_formats_candidate_report() {
        let report = build_rp1_ethernet_gpio32_event_state_discriminator_report(
            accepted_gpio32_event_state_discriminator_input(),
        )
        .expect("valid GPIO32 event-state discriminator candidate input");
        let evidence = rp1_ethernet_gpio32_event_state_discriminator_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_GPIO32_EVENT_STATE_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.event_state_contract_id,
            RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_GPIO32_EVENT_STATE_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.v2_proof_task_id,
            Some(RP1_ETHERNET_GPIO32_EVENT_STATE_WRITE_RESTORE_V2_PROOF_TASK_ID)
        );
        assert_eq!(
            evidence.v2_classification,
            Some(RP1_ETHERNET_GPIO32_EVENT_STATE_V2_BLOCKER_CLASSIFICATION)
        );
        assert_eq!(evidence.v2_writes_performed, Some(false));
        assert_eq!(
            evidence.gpio_controller,
            Some(RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER)
        );
        assert_eq!(evidence.gpio_line, Some(32));
        assert_eq!(evidence.reset_route, Some(RP1_ETHERNET_PHY_RESET_ROUTE));
        assert_eq!(evidence.bank, Some("bank1"));
        assert_eq!(evidence.bank_local_bit, Some(4));
        assert_eq!(evidence.active_low, Some(true));
        assert_eq!(evidence.gpio32_status_observed_target, Some(0x1c_000d_4020));
        assert_eq!(evidence.gpio32_ctrl_observed_target, Some(0x1c_000d_4024));
        assert_eq!(evidence.rio1_out_observed_target, Some(0x1c_000e_4000));
        assert_eq!(evidence.rio1_oe_observed_target, Some(0x1c_000e_4004));
        assert_eq!(evidence.rio1_in_observed_target, Some(0x1c_000e_4008));
        assert_eq!(evidence.gpio32_pad_observed_target, Some(0x1c_000f_4014));
        assert_eq!(evidence.status_raw, Some(0x0abe_3300));
        assert_eq!(evidence.ctrl_raw, Some(0x85));
        assert_eq!(evidence.rio1_out_raw, Some(0x10));
        assert_eq!(evidence.rio1_oe_raw, Some(0x10));
        assert_eq!(evidence.rio1_in_raw, Some(0x12));
        assert_eq!(evidence.event_bits, Some(0x0ab0_0000));
        assert_eq!(
            evidence.source_event_bit_names,
            Some(RP1_ETHERNET_GPIO32_STATUS_SOURCE_EVENT_BIT_NAMES)
        );
        assert_eq!(evidence.source_decoding_status, "source-backed-bits-20-27");
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GPIO32_EVENT_STATE_CANDIDATE_CLASSIFICATION
        );
        assert_eq!(
            evidence.hardware_proof_boundary_classification,
            RP1_ETHERNET_GPIO32_EVENT_STATE_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_GPIO32_EVENT_STATE_REJECTED_RUNTIME_CLAIMS
        );
        assert!(!evidence.claims_event_clearing);
        assert!(!evidence.claims_gpio_rio_pad_mmio_write);
        assert!(!evidence.claims_gpio32_ownership);
        assert!(!evidence.claims_gpio32_write_restore_retry);
        assert!(!evidence.claims_phase_transition);
    }

    #[test_case]
    fn rp1_ethernet_gpio32_event_state_formats_control_and_source_unresolved() {
        let control = build_rp1_ethernet_gpio32_event_state_discriminator_report(
            Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                kind: Rp1EthernetGpio32EventStateDiscriminatorReportKind::NoGpioNoEthernetControl,
                source_contract: None,
                status_raw: None,
                ctrl_raw: None,
                rio1_out_raw: None,
                rio1_oe_raw: None,
                rio1_in_raw: None,
                pad_raw: None,
                source_decoding_status:
                    Rp1EthernetGpio32EventStateSourceDecodingStatus::CaptureChainInconclusive,
                event_state_classification: RP1_ETHERNET_GPIO32_EVENT_STATE_CONTROL_CLASSIFICATION,
                ..accepted_gpio32_event_state_discriminator_input()
            },
        )
        .expect("valid GPIO32 event-state control input");
        let control_evidence =
            rp1_ethernet_gpio32_event_state_discriminator_report_evidence(control);

        assert_eq!(control_evidence.report_kind, "no-gpio-no-ethernet-control");
        assert_eq!(control_evidence.gpio_controller, None);
        assert_eq!(control_evidence.gpio_line, None);
        assert_eq!(control_evidence.gpio32_status_observed_target, None);
        assert_eq!(control_evidence.status_raw, None);
        assert_eq!(control_evidence.event_bits, None);
        assert_eq!(control_evidence.source_event_bit_names, None);
        assert_eq!(control_evidence.source_evidence, None);
        assert_eq!(
            control_evidence.classification,
            RP1_ETHERNET_GPIO32_EVENT_STATE_CONTROL_CLASSIFICATION
        );

        let source_unresolved = build_rp1_ethernet_gpio32_event_state_discriminator_report(
            Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                source_decoding_status:
                    Rp1EthernetGpio32EventStateSourceDecodingStatus::SourceUnresolved,
                event_state_classification:
                    "rp1-ethernet-gpio32-event-state-source-unresolved-event-state",
                ..accepted_gpio32_event_state_discriminator_input()
            },
        )
        .expect("valid source-unresolved event-state input");
        let source_unresolved_evidence =
            rp1_ethernet_gpio32_event_state_discriminator_report_evidence(source_unresolved);

        assert_eq!(
            source_unresolved_evidence.source_decoding_status,
            "source-unresolved"
        );
        assert_eq!(
            source_unresolved_evidence.classification,
            "rp1-ethernet-gpio32-event-state-source-unresolved-event-state"
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_event_state_rejects_shape_and_overclaims() {
        let input = accepted_gpio32_event_state_discriminator_input();

        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32EventStateDiscriminatorReportError::CandidateMissingSourceContract
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    kind:
                        Rp1EthernetGpio32EventStateDiscriminatorReportKind::NoGpioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::ControlCarriesGpioTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    source_contract: Some(Rp1EthernetGpio32EventStateSourceContractEvidence {
                        gpio_line: 33,
                        ..rp1_ethernet_gpio32_event_state_source_contract_evidence()
                    },),
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32EventStateDiscriminatorReportError::SourceContractIdentityMismatch
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    source_contract: Some(
                        Rp1EthernetGpio32EventStateSourceContractEvidence {
                            accepted_v2_event_bits: 0,
                            ..rp1_ethernet_gpio32_event_state_source_contract_evidence()
                        },
                    ),
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32EventStateDiscriminatorReportError::SourceContractV2LineageMismatch
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    status_raw: Some(0),
                    ..input
                }
            ),
            Err(
                Rp1EthernetGpio32EventStateDiscriminatorReportError::EventStateClassificationMismatch
            )
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    claims_event_clearing: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::EventClearingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    claims_gpio_rio_pad_mmio_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::GpioRioPadMmioWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    claims_gpio32_write_restore_retry: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::Gpio32WriteRestoreRetryClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    claims_mdio_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_state_discriminator_report(
                Rp1EthernetGpio32EventStateDiscriminatorReportInput {
                    claims_dma_descriptor_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventStateDiscriminatorReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            rejected_rp1_ethernet_gpio32_event_state_discriminator_report_evidence(
                Rp1EthernetGpio32EventStateDiscriminatorReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_event_clear_guard_formats_candidate_report() {
        let report = build_rp1_ethernet_gpio32_event_clear_guard_report(
            accepted_gpio32_event_clear_guard_input(),
        )
        .expect("valid GPIO32 event-clear guard candidate input");
        let evidence = rp1_ethernet_gpio32_event_clear_guard_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_GPIO32_EVENT_CLEAR_GUARD_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_CONTRACT_ID
        );
        assert_eq!(
            evidence.event_state_contract_id,
            RP1_ETHERNET_GPIO32_EVENT_STATE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.gpio_controller,
            Some(RP1_ETHERNET_PHY_RESET_GPIO_CONTROLLER)
        );
        assert_eq!(evidence.gpio_line, Some(32));
        assert_eq!(evidence.bank, Some("bank1"));
        assert_eq!(evidence.bank_local_bit, Some(4));
        assert_eq!(evidence.gpio32_status_observed_target, Some(0x1c_000d_4020));
        assert_eq!(evidence.gpio32_ctrl_observed_target, Some(0x1c_000d_4024));
        assert_eq!(evidence.clear_observed_target, Some(0x1c_000d_6024));
        assert_eq!(evidence.clear_source_target, Some(0xc0_400d_6024));
        assert_eq!(evidence.width_bits, Some(32));
        assert_eq!(evidence.write_value, Some(0x1000_0000));
        assert_eq!(
            evidence.write_value_name,
            Some(RP1_ETHERNET_GPIO32_EVENT_CLEAR_WRITE_VALUE_NAME)
        );
        assert_eq!(evidence.status_event_mask, Some(0x0ff0_0000));
        assert_eq!(evidence.accepted_event_bits, Some(0x0ab0_0000));
        assert_eq!(evidence.accepted_status_raw, Some(0x0abe_3300));
        assert_eq!(evidence.accepted_ctrl_raw, Some(0x85));
        assert_eq!(evidence.accepted_rio1_out_raw, Some(0x10));
        assert_eq!(evidence.accepted_rio1_oe_raw, Some(0x10));
        assert_eq!(evidence.accepted_rio1_in_raw, Some(0x12));
        assert_eq!(
            evidence.pre_read_requirements,
            Some(RP1_ETHERNET_GPIO32_EVENT_CLEAR_PRE_READ_REQUIREMENTS)
        );
        assert_eq!(
            evidence.post_read_requirements,
            Some(RP1_ETHERNET_GPIO32_EVENT_CLEAR_POST_READ_REQUIREMENTS)
        );
        assert_eq!(
            evidence.forbidden_writes,
            Some(RP1_ETHERNET_GPIO32_EVENT_CLEAR_FORBIDDEN_WRITES)
        );
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_GPIO32_EVENT_CLEAR_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_GPIO32_EVENT_CLEAR_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_GPIO32_EVENT_CLEAR_SOURCE_EVIDENCE)
        );
        assert!(!evidence.claims_event_clear_executed);
        assert!(!evidence.claims_volatile_mmio_execution);
        assert!(!evidence.claims_write_outside_irqreset);
        assert!(!evidence.claims_ctrl_rw_clr_xor_write);
        assert!(!evidence.claims_rio_pad_function_mutation);
        assert!(!evidence.claims_gpio32_ownership);
        assert!(!evidence.claims_gpio32_write_restore_retry);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GPIO32_EVENT_CLEAR_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_event_clear_guard_formats_paired_control() {
        let report = build_rp1_ethernet_gpio32_event_clear_guard_report(
            Rp1EthernetGpio32EventClearGuardReportInput {
                kind: Rp1EthernetGpio32EventClearGuardReportKind::NoGpioNoEthernetControl,
                source_contract: None,
                ..accepted_gpio32_event_clear_guard_input()
            },
        )
        .expect("valid GPIO32 event-clear guard control input");
        let evidence = rp1_ethernet_gpio32_event_clear_guard_report_evidence(report);

        assert_eq!(evidence.report_kind, "no-gpio-no-ethernet-control");
        assert_eq!(evidence.gpio_controller, None);
        assert_eq!(evidence.gpio_line, None);
        assert_eq!(evidence.gpio32_status_observed_target, None);
        assert_eq!(evidence.gpio32_ctrl_observed_target, None);
        assert_eq!(evidence.rio1_out_observed_target, None);
        assert_eq!(evidence.clear_observed_target, None);
        assert_eq!(evidence.write_value, None);
        assert_eq!(evidence.pre_read_requirements, None);
        assert_eq!(evidence.post_read_requirements, None);
        assert_eq!(evidence.forbidden_writes, None);
        assert_eq!(evidence.source_evidence, None);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_GPIO32_EVENT_CLEAR_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_gpio32_event_clear_guard_rejects_shape_and_overclaims() {
        let input = accepted_gpio32_event_clear_guard_input();

        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    kind: Rp1EthernetGpio32EventClearGuardReportKind::NoGpioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::ControlCarriesGpioTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    source_contract: Some(Rp1EthernetGpio32EventClearSourceContractEvidence {
                        gpio_line: 33,
                        ..rp1_ethernet_gpio32_event_clear_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    source_contract: Some(Rp1EthernetGpio32EventClearSourceContractEvidence {
                        clear_observed_target: 0x1c_000d_4024,
                        ..rp1_ethernet_gpio32_event_clear_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    source_contract: Some(Rp1EthernetGpio32EventClearSourceContractEvidence {
                        status_event_mask: 0x00f0_0000,
                        ..rp1_ethernet_gpio32_event_clear_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_event_clear_executed: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::EventClearExecutedClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_write_outside_irqreset: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::WriteOutsideIrqresetClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_ctrl_rw_clr_xor_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::CtrlRwClrXorWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_rio_pad_function_mutation: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::RioPadFunctionMutationClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_gpio32_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::Gpio32OwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_gpio32_write_restore_retry: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::Gpio32WriteRestoreRetryClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_mdio_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_interrupt_completion: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::InterruptCompletionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_gpio32_event_clear_guard_report(
                Rp1EthernetGpio32EventClearGuardReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetGpio32EventClearGuardReportError::Phase122Claim)
        );
        assert_eq!(
            rejected_rp1_ethernet_gpio32_event_clear_guard_report_evidence(
                Rp1EthernetGpio32EventClearGuardReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_phy_id_guard_formats_candidate_report() {
        let report =
            build_rp1_ethernet_mdio_phy_id_guard_report(accepted_mdio_phy_id_guard_input())
                .expect("valid MDIO PHY-ID guard candidate input");
        let evidence = rp1_ethernet_mdio_phy_id_guard_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_MDIO_PHY_ID_GUARD_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_MDIO_PHY_ID_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.selected_discriminator,
            Some(RP1_ETHERNET_MDIO_PHY_ID_SELECTED_DISCRIMINATOR)
        );
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(evidence.phy_handle, Some("phy1"));
        assert_eq!(evidence.phy_node, Some("ethernet-phy@1"));
        assert_eq!(evidence.phy_address, Some(1));
        assert_eq!(
            evidence.phy_id_register_names,
            Some(RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTER_NAMES)
        );
        assert_eq!(
            evidence.phy_id_registers,
            Some(RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS)
        );
        assert_eq!(evidence.observed_identity_target, Some(0x1c_0010_00fc));
        assert_eq!(evidence.translated_comparator_target, Some(0x1f_0010_00fc));
        assert_eq!(evidence.ncr_register, Some("NCR"));
        assert_eq!(evidence.nsr_register, Some("NSR"));
        assert_eq!(evidence.man_register, Some("MAN"));
        assert_eq!(evidence.ncr_offset, Some(0x0000));
        assert_eq!(evidence.nsr_offset, Some(0x0008));
        assert_eq!(evidence.man_offset, Some(0x0034));
        assert_eq!(evidence.ncr_observed_target, Some(0x1c_0000_0000));
        assert_eq!(evidence.nsr_observed_target, Some(0x1c_0000_0008));
        assert_eq!(evidence.man_observed_target, Some(0x1c_0000_0034));
        assert_eq!(evidence.ncr_mpe_bit, Some(4));
        assert_eq!(evidence.nsr_idle_bit, Some(2));
        assert_eq!(evidence.man_data_offset, Some(0));
        assert_eq!(evidence.man_data_size, Some(16));
        assert_eq!(evidence.man_c22_sof, Some(1));
        assert_eq!(evidence.man_c22_read, Some(2));
        assert_eq!(evidence.man_c22_write, Some(1));
        assert_eq!(evidence.man_code, Some(2));
        assert_eq!(evidence.physid1_man_frame, Some(0x600a_0000));
        assert_eq!(evidence.physid2_man_frame, Some(0x600e_0000));
        assert_eq!(
            evidence.preconditions,
            Some(RP1_ETHERNET_MDIO_PHY_ID_PRECONDITIONS)
        );
        assert_eq!(
            evidence.operation_order,
            Some(RP1_ETHERNET_MDIO_PHY_ID_OPERATION_ORDER)
        );
        assert_eq!(
            evidence.allowed_classifications,
            RP1_ETHERNET_MDIO_PHY_ID_ALLOWED_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_MDIO_PHY_ID_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_MDIO_PHY_ID_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_MDIO_PHY_ID_SOURCE_EVIDENCE)
        );
        assert!(!evidence.claims_runtime_mdio_transaction);
        assert!(!evidence.claims_mdio_phy_ownership);
        assert!(!evidence.claims_mpe_write_permission);
        assert!(!evidence.claims_gpio32_phy_reset_ownership);
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_PHY_ID_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_phy_id_guard_formats_paired_control() {
        let report =
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                kind: Rp1EthernetMdioPhyIdGuardReportKind::NoMdioNoEthernetControl,
                source_contract: None,
                ..accepted_mdio_phy_id_guard_input()
            })
            .expect("valid MDIO PHY-ID guard control input");
        let evidence = rp1_ethernet_mdio_phy_id_guard_report_evidence(report);

        assert_eq!(evidence.report_kind, "no-mdio-no-ethernet-control");
        assert_eq!(evidence.selected_discriminator, None);
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.phy_handle, None);
        assert_eq!(evidence.phy_address, None);
        assert_eq!(evidence.phy_id_registers, None);
        assert_eq!(evidence.observed_identity_target, None);
        assert_eq!(evidence.ncr_observed_target, None);
        assert_eq!(evidence.nsr_observed_target, None);
        assert_eq!(evidence.man_observed_target, None);
        assert_eq!(evidence.physid1_man_frame, None);
        assert_eq!(evidence.physid2_man_frame, None);
        assert_eq!(evidence.preconditions, None);
        assert_eq!(evidence.operation_order, None);
        assert_eq!(evidence.source_evidence, None);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_PHY_ID_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_phy_id_guard_rejects_shape_and_overclaims() {
        let input = accepted_mdio_phy_id_guard_input();

        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                source_contract: None,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                kind: Rp1EthernetMdioPhyIdGuardReportKind::NoMdioNoEthernetControl,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::ControlCarriesMdioTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                source_contract: Some(Rp1EthernetMdioPhyIdSourceContractEvidence {
                    phy_address: 2,
                    ..rp1_ethernet_mdio_phy_id_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                source_contract: Some(Rp1EthernetMdioPhyIdSourceContractEvidence {
                    man_observed_target: 0x1c_0000_0038,
                    ..rp1_ethernet_mdio_phy_id_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                source_contract: Some(Rp1EthernetMdioPhyIdSourceContractEvidence {
                    physid1_man_frame: 0x600e_0000,
                    ..rp1_ethernet_mdio_phy_id_source_contract_evidence()
                }),
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_runtime_mdio_transaction: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::RuntimeMdioTransactionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_mdio_phy_ownership: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::MdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_mpe_write_permission: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::MpeWritePermissionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_gpio32_phy_reset_ownership: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::Gpio32PhyResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_ethernet_ready: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_broad_mmio_ready: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::BroadMmioReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_dma_descriptor_ownership: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_guard_report(Rp1EthernetMdioPhyIdGuardReportInput {
                claims_phase_12_2: true,
                ..input
            }),
            Err(Rp1EthernetMdioPhyIdGuardReportError::Phase122Claim)
        );
        assert_eq!(
            rejected_rp1_ethernet_mdio_phy_id_guard_report_evidence(
                Rp1EthernetMdioPhyIdGuardReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_phy_id_after_mpe_guard_formats_candidate_report() {
        let report = build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
            accepted_mdio_phy_id_after_mpe_guard_input(),
        )
        .expect("valid corrected-target after-MPE guard candidate input");
        let evidence = rp1_ethernet_mdio_phy_id_after_mpe_guard_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_GUARD_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_CONTRACT_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.selected_discriminator,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SELECTED_DISCRIMINATOR)
        );
        assert_eq!(
            evidence.accepted_mpe_frontier_classification,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ACCEPTED_MPE_FRONTIER_CLASSIFICATION)
        );
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.phy_handle, Some("phy1"));
        assert_eq!(evidence.phy_node, Some("ethernet-phy@1"));
        assert_eq!(evidence.phy_address, Some(1));
        assert_eq!(evidence.macb_mid_context_target, Some(0x1c_0010_00fc));
        assert_eq!(
            evidence.macb_mid_context_raw,
            Some(RP1_ETHERNET_ACCEPTED_OBSERVED_MACB_MID_RAW)
        );
        assert_eq!(evidence.ncr_register, Some("NCR"));
        assert_eq!(evidence.nsr_register, Some("NSR"));
        assert_eq!(evidence.man_register, Some("MAN"));
        assert_eq!(evidence.ncr_observed_target, Some(0x1c_0010_0000));
        assert_eq!(evidence.nsr_observed_target, Some(0x1c_0010_0008));
        assert_eq!(evidence.man_observed_target, Some(0x1c_0010_0034));
        assert_eq!(
            evidence.phy_id_register_names,
            Some(RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTER_NAMES)
        );
        assert_eq!(
            evidence.phy_id_registers,
            Some(RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS)
        );
        assert_eq!(evidence.physid1_man_frame, Some(0x600a_0000));
        assert_eq!(evidence.physid2_man_frame, Some(0x600e_0000));
        assert_eq!(evidence.ncr_mpe_bit, Some(4));
        assert_eq!(evidence.nsr_idle_bit, Some(2));
        assert_eq!(
            evidence.poll_policy,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_POLL_POLICY)
        );
        assert_eq!(
            evidence.mpe_precondition,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MPE_PRECONDITION)
        );
        assert_eq!(evidence.result_extraction, Some("MAN.DATA bits 15:0"));
        assert_eq!(
            evidence.operation_order,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_OPERATION_ORDER)
        );
        assert_eq!(
            evidence.allowed_classifications,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_ALLOWED_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_RETAINED_RISKS
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_SOURCE_EVIDENCE)
        );
        assert!(evidence.constructs_mdio_targets);
        assert!(evidence.constructs_man_frames);
        assert!(!evidence.runtime_volatile_load_store_intent);
        assert!(!evidence.claims_ncr_write);
        assert!(!evidence.claims_missing_mpe_gating);
        assert!(!evidence.claims_wrong_mdio_targets);
        assert!(!evidence.claims_man_write_without_mpe_precondition);
        assert!(!evidence.claims_gpio32_phy_reset_action);
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_networking);
        assert!(!evidence.claims_ssh);
        assert!(!evidence.claims_phase_12_2);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_phy_id_after_mpe_guard_formats_paired_control() {
        let report = build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
            Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                kind: Rp1EthernetMdioPhyIdAfterMpeGuardReportKind::NoMdioNoEthernetControl,
                source_contract: None,
                ..accepted_mdio_phy_id_after_mpe_guard_input()
            },
        )
        .expect("valid corrected-target after-MPE control input");
        let evidence = rp1_ethernet_mdio_phy_id_after_mpe_guard_report_evidence(report);

        assert_eq!(evidence.report_kind, "no-mdio-no-ethernet-control");
        assert_eq!(evidence.selected_discriminator, None);
        assert_eq!(evidence.accepted_mpe_frontier_task_id, None);
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.phy_address, None);
        assert_eq!(evidence.macb_mid_context_target, None);
        assert_eq!(evidence.ncr_observed_target, None);
        assert_eq!(evidence.nsr_observed_target, None);
        assert_eq!(evidence.man_observed_target, None);
        assert_eq!(evidence.phy_id_registers, None);
        assert_eq!(evidence.physid1_man_frame, None);
        assert_eq!(evidence.physid2_man_frame, None);
        assert_eq!(evidence.poll_policy, None);
        assert_eq!(evidence.mpe_precondition, None);
        assert_eq!(evidence.operation_order, None);
        assert_eq!(evidence.source_evidence, None);
        assert!(!evidence.constructs_mdio_targets);
        assert!(!evidence.constructs_man_frames);
        assert!(!evidence.runtime_volatile_load_store_intent);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_phy_id_after_mpe_guard_rejects_shape_and_overclaims() {
        let input = accepted_mdio_phy_id_after_mpe_guard_input();

        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    kind: Rp1EthernetMdioPhyIdAfterMpeGuardReportKind::NoMdioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::ControlCarriesMdioTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence {
                        phy_address: 2,
                        ..rp1_ethernet_mdio_phy_id_after_mpe_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence {
                        ncr_observed_target: 0x1c_0000_0000,
                        ..rp1_ethernet_mdio_phy_id_after_mpe_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioPhyIdAfterMpeSourceContractEvidence {
                        operation_order: RP1_ETHERNET_MDIO_PHY_ID_OPERATION_ORDER,
                        ..rp1_ethernet_mdio_phy_id_after_mpe_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_runtime_volatile_load_store: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::RuntimeVolatileLoadStoreClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_ncr_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::NcrWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_missing_mpe_gating: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::MissingMpeGatingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_wrong_mdio_targets: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::WrongMdioTargetsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_unbounded_polling: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::UnboundedPollingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_man_write_without_mpe_precondition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::ManWriteWithoutMpePreconditionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_gpio32_phy_reset_action: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::Gpio32PhyResetActionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_ethernet_ready: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_networking: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_ssh: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_phy_id_after_mpe_guard_report(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioPhyIdAfterMpeGuardReportError::Phase122Claim)
        );
        assert_eq!(
            rejected_rp1_ethernet_mdio_phy_id_after_mpe_guard_report_evidence(
                Rp1EthernetMdioPhyIdAfterMpeGuardReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_mpe_enable_guard_formats_candidate_report() {
        let report =
            build_rp1_ethernet_mdio_mpe_enable_guard_report(accepted_mdio_mpe_enable_guard_input())
                .expect("valid MDIO MPE enable guard candidate input");
        let evidence = rp1_ethernet_mdio_mpe_enable_guard_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_MDIO_MPE_ENABLE_GUARD_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.selected_operation,
            Some(RP1_ETHERNET_MDIO_MPE_ENABLE_SELECTED_OPERATION)
        );
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(evidence.observed_identity_target, Some(0x1c_0010_00fc));
        assert_eq!(
            evidence.register,
            Some(RP1_ETHERNET_MDIO_MPE_ENABLE_REGISTER)
        );
        assert_eq!(evidence.source_offset, Some(0x0010_0000));
        assert_eq!(evidence.ncr_offset, Some(0));
        assert_eq!(evidence.observed_target, Some(0x1c_0010_0000));
        assert_eq!(evidence.width_bits, Some(32));
        assert_eq!(evidence.mpe_bit, Some(4));
        assert_eq!(evidence.mpe_mask, Some(0x10));
        assert_eq!(
            evidence.write_rule,
            Some(RP1_ETHERNET_MDIO_MPE_ENABLE_WRITE_RULE)
        );
        assert_eq!(
            evidence.restore_invariant,
            Some(RP1_ETHERNET_MDIO_MPE_ENABLE_RESTORE_INVARIANT)
        );
        assert_eq!(
            evidence.operation_order,
            Some(RP1_ETHERNET_MDIO_MPE_ENABLE_OPERATION_ORDER)
        );
        assert_eq!(
            evidence.allowed_classifications,
            RP1_ETHERNET_MDIO_MPE_ENABLE_ALLOWED_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_MDIO_MPE_ENABLE_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_MDIO_MPE_ENABLE_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_MDIO_MPE_ENABLE_RETAINED_RISKS
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_MDIO_MPE_ENABLE_SOURCE_EVIDENCE)
        );
        assert!(evidence.candidate_write_intent);
        assert!(!evidence.claims_runtime_ncr_write_executed);
        assert!(!evidence.claims_man_write);
        assert!(!evidence.claims_phy_id_read);
        assert!(!evidence.claims_broad_mdio_phy_ownership);
        assert!(!evidence.claims_phy_reset_ownership);
        assert!(!evidence.claims_gpio32_ownership);
        assert!(!evidence.claims_ethernet_ready);
        assert!(!evidence.claims_dma_descriptor_ownership);
        assert!(!evidence.claims_interrupt_completion);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_MPE_ENABLE_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_mpe_enable_guard_formats_paired_control() {
        let report = build_rp1_ethernet_mdio_mpe_enable_guard_report(
            Rp1EthernetMdioMpeEnableGuardReportInput {
                kind: Rp1EthernetMdioMpeEnableGuardReportKind::NoMdioNoEthernetControl,
                source_contract: None,
                ..accepted_mdio_mpe_enable_guard_input()
            },
        )
        .expect("valid MDIO MPE enable guard control input");
        let evidence = rp1_ethernet_mdio_mpe_enable_guard_report_evidence(report);

        assert_eq!(evidence.report_kind, "no-mdio-no-ethernet-control");
        assert_eq!(evidence.selected_operation, None);
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.compatible, None);
        assert_eq!(evidence.observed_identity_target, None);
        assert_eq!(evidence.register, None);
        assert_eq!(evidence.source_offset, None);
        assert_eq!(evidence.ncr_offset, None);
        assert_eq!(evidence.observed_target, None);
        assert_eq!(evidence.mpe_bit, None);
        assert_eq!(evidence.mpe_mask, None);
        assert_eq!(evidence.write_rule, None);
        assert_eq!(evidence.restore_invariant, None);
        assert_eq!(evidence.operation_order, None);
        assert_eq!(evidence.source_evidence, None);
        assert!(!evidence.candidate_write_intent);
        assert!(!evidence.claims_runtime_ncr_write_executed);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_MPE_ENABLE_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_mpe_enable_guard_rejects_shape_and_overclaims() {
        let input = accepted_mdio_mpe_enable_guard_input();

        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    kind: Rp1EthernetMdioMpeEnableGuardReportKind::NoMdioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::ControlCarriesNcrMpeTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioMpeEnableSourceContractEvidence {
                        selected_operation: "same-shaped-mdio-phy-id-retry",
                        ..rp1_ethernet_mdio_mpe_enable_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioMpeEnableSourceContractEvidence {
                        observed_target: 0x1c_0000_0000,
                        ..rp1_ethernet_mdio_mpe_enable_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioMpeEnableSourceContractEvidence {
                        mpe_mask: 0x20,
                        ..rp1_ethernet_mdio_mpe_enable_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_runtime_ncr_write_executed: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::RuntimeNcrWriteExecutedClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_man_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::ManWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_phy_id_read: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::PhyIdReadClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_broad_mdio_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::BroadMdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_phy_reset_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::PhyResetOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_gpio32_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::Gpio32OwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_ethernet_ready: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_dma_descriptor_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::DmaDescriptorOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_mpe_enable_guard_report(
                Rp1EthernetMdioMpeEnableGuardReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioMpeEnableGuardReportError::Phase122Claim)
        );
        assert_eq!(
            rejected_rp1_ethernet_mdio_mpe_enable_guard_report_evidence(
                Rp1EthernetMdioMpeEnableGuardReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_register_vector_guard_formats_candidate_report() {
        let report = build_rp1_ethernet_mdio_register_vector_guard_report(
            accepted_mdio_register_vector_guard_input(),
        )
        .expect("valid MDIO register-vector guard candidate input");
        let evidence = rp1_ethernet_mdio_register_vector_guard_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_GUARD_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_contract_id,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.selected_discriminator,
            Some(RP1_ETHERNET_MDIO_REGISTER_VECTOR_SELECTED_DISCRIMINATOR)
        );
        assert_eq!(
            evidence.purpose,
            Some(RP1_ETHERNET_MDIO_REGISTER_VECTOR_PURPOSE)
        );
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.compatible, Some(RP1_ETHERNET_COMPATIBLE));
        assert_eq!(evidence.phy_handle, Some(RP1_ETHERNET_PHY_HANDLE));
        assert_eq!(evidence.phy_node, Some(RP1_ETHERNET_PHY_NODE));
        assert_eq!(evidence.phy_address, Some(1));
        assert_eq!(evidence.macb_mid_context_target, Some(0x1c_0010_00fc));
        assert_eq!(evidence.macb_mid_context_raw, Some(0x0007_0109));
        assert_eq!(evidence.ncr_observed_target, Some(0x1c_0010_0000));
        assert_eq!(evidence.nsr_observed_target, Some(0x1c_0010_0008));
        assert_eq!(evidence.man_observed_target, Some(0x1c_0010_0034));
        assert_eq!(
            evidence.register_names,
            Some(RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTER_NAMES)
        );
        assert_eq!(
            evidence.registers,
            Some(RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTERS)
        );
        assert_eq!(
            evidence.man_frames,
            Some(RP1_ETHERNET_MDIO_REGISTER_VECTOR_MAN_FRAMES)
        );
        assert_eq!(
            evidence.man_frames.expect("candidate frames"),
            &[
                0x6082_0000,
                0x6086_0000,
                0x608a_0000,
                0x608e_0000,
                0x6092_0000,
                0x6096_0000
            ]
        );
        assert_eq!(evidence.ncr_mpe_bit, Some(4));
        assert_eq!(evidence.nsr_idle_bit, Some(2));
        assert_eq!(
            evidence.poll_policy,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_POLL_POLICY)
        );
        assert_eq!(
            evidence.mpe_precondition,
            Some(RP1_ETHERNET_MDIO_PHY_ID_AFTER_MPE_MPE_PRECONDITION)
        );
        assert_eq!(evidence.man_data_offset, Some(0));
        assert_eq!(evidence.man_data_size, Some(16));
        assert_eq!(
            evidence.allowed_classifications,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_ALLOWED_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.retained_risks,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_RETAINED_RISKS
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_MDIO_REGISTER_VECTOR_SOURCE_EVIDENCE)
        );
        assert!(evidence.constructs_mdio_targets);
        assert!(evidence.constructs_man_frames);
        assert!(!evidence.runtime_volatile_load_store_intent);
        assert!(!evidence.claims_ncr_write);
        assert!(!evidence.claims_man_write_without_mpe_precondition);
        assert!(!evidence.claims_phy_absence_from_all_ones);
        assert!(!evidence.claims_broad_mdio_phy_ownership);
        assert!(!evidence.claims_gpio32_phy_reset_action);
        assert!(!evidence.claims_phase_transition);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_register_vector_guard_formats_paired_control() {
        let report = build_rp1_ethernet_mdio_register_vector_guard_report(
            Rp1EthernetMdioRegisterVectorGuardReportInput {
                kind: Rp1EthernetMdioRegisterVectorGuardReportKind::NoMdioNoEthernetControl,
                source_contract: None,
                ..accepted_mdio_register_vector_guard_input()
            },
        )
        .expect("valid MDIO register-vector guard control input");
        let evidence = rp1_ethernet_mdio_register_vector_guard_report_evidence(report);

        assert_eq!(evidence.report_kind, "no-mdio-no-ethernet-control");
        assert_eq!(evidence.selected_discriminator, None);
        assert_eq!(evidence.purpose, None);
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.phy_address, None);
        assert_eq!(evidence.macb_mid_context_target, None);
        assert_eq!(evidence.ncr_observed_target, None);
        assert_eq!(evidence.nsr_observed_target, None);
        assert_eq!(evidence.man_observed_target, None);
        assert_eq!(evidence.register_names, None);
        assert_eq!(evidence.registers, None);
        assert_eq!(evidence.man_frames, None);
        assert_eq!(evidence.man_frame_construction, None);
        assert_eq!(evidence.poll_policy, None);
        assert_eq!(evidence.mpe_precondition, None);
        assert_eq!(evidence.operation_order, None);
        assert_eq!(evidence.source_evidence, None);
        assert!(!evidence.constructs_mdio_targets);
        assert!(!evidence.constructs_man_frames);
        assert!(!evidence.runtime_volatile_load_store_intent);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_MDIO_REGISTER_VECTOR_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_mdio_register_vector_guard_rejects_shape_and_overclaims() {
        let input = accepted_mdio_register_vector_guard_input();

        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    kind: Rp1EthernetMdioRegisterVectorGuardReportKind::NoMdioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::ControlCarriesMdioTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioRegisterVectorSourceContractEvidence {
                        phy_address: 2,
                        ..rp1_ethernet_mdio_register_vector_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioRegisterVectorSourceContractEvidence {
                        ncr_observed_target: 0x1c_0000_0000,
                        ..rp1_ethernet_mdio_register_vector_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::SourceContractTargetMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    source_contract: Some(Rp1EthernetMdioRegisterVectorSourceContractEvidence {
                        man_frames: RP1_ETHERNET_MDIO_PHY_ID_PHY_ID_REGISTERS,
                        ..rp1_ethernet_mdio_register_vector_source_contract_evidence()
                    }),
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_runtime_volatile_load_store: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::RuntimeVolatileLoadStoreClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_ncr_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::NcrWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_missing_mpe_gating: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::MissingMpeGatingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_wrong_mdio_targets: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::WrongMdioTargetsClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_unbounded_polling: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::UnboundedPollingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_man_write_without_mpe_precondition: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::ManWriteWithoutMpePreconditionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_phy_absence_from_all_ones: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::PhyAbsenceFromAllOnesClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_broad_mdio_phy_ownership: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::BroadMdioPhyOwnershipClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_gpio32_phy_reset_action: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::Gpio32PhyResetActionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_ethernet_ready: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::EthernetReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_networking: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_ssh: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_mdio_register_vector_guard_report(
                Rp1EthernetMdioRegisterVectorGuardReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetMdioRegisterVectorGuardReportError::Phase122Claim)
        );
        assert_eq!(
            rejected_rp1_ethernet_mdio_register_vector_guard_report_evidence(
                Rp1EthernetMdioRegisterVectorGuardReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_bcm54213pe_readonly_preflight_formats_candidate_report() {
        let report = build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
            accepted_bcm54213pe_readonly_preflight_report_input(),
        )
        .expect("valid BCM54213PE read-only preflight candidate input");
        let evidence = rp1_ethernet_bcm54213pe_readonly_preflight_report_evidence(report);

        assert_eq!(
            evidence.report_contract_id,
            RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REPORT_CONTRACT_ID
        );
        assert_eq!(
            evidence.source_task_id,
            RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_TASK_ID
        );
        assert_eq!(evidence.report_kind, "candidate");
        assert_eq!(
            evidence.selected_discriminator,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_DISCRIMINATOR)
        );
        assert_eq!(evidence.controller, Some(RP1_ETHERNET_CONTROLLER_NAME));
        assert_eq!(evidence.phy_handle, Some(RP1_ETHERNET_PHY_HANDLE));
        assert_eq!(evidence.phy_node, Some(RP1_ETHERNET_PHY_NODE));
        assert_eq!(evidence.phy_address, Some(1));
        assert_eq!(
            evidence.selected_targets,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SELECTED_TARGETS)
        );
        assert_eq!(
            evidence.register_names,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTER_NAMES)
        );
        assert_eq!(
            evidence.registers,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REGISTERS)
        );
        assert_eq!(
            evidence.man_frames,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_MAN_FRAMES)
        );
        assert_eq!(
            evidence.man_frames.expect("candidate frames"),
            &[0x60a6_0000, 0x60aa_0000]
        );
        assert_eq!(evidence.accepted_phy_id_model, Some("Broadcom BCM54213PE"));
        assert_eq!(evidence.accepted_physid1, Some(0x600d));
        assert_eq!(evidence.accepted_physid2, Some(0x84a2));
        assert_eq!(
            evidence.already_sampled_context,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALREADY_SAMPLED_CONTEXT)
        );
        assert_eq!(
            evidence.forbidden_targets,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_FORBIDDEN_TARGETS)
        );
        assert_eq!(
            evidence.allowed_classifications,
            RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_ALLOWED_CLASSIFICATIONS
        );
        assert_eq!(
            evidence.boundary_classification,
            RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_BOUNDARY_CLASSIFICATION
        );
        assert_eq!(
            evidence.rejected_runtime_claims,
            RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_REJECTED_RUNTIME_CLAIMS
        );
        assert_eq!(
            evidence.source_evidence,
            Some(RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_SOURCE_EVIDENCE)
        );
        assert!(evidence.constructs_mdio_targets);
        assert!(evidence.constructs_man_frames);
        assert!(!evidence.constructs_macb_target);
        assert!(!evidence.constructs_gpio_target);
        assert!(!evidence.runtime_volatile_access_intent);
        assert!(!evidence.claims_hardware_proof_authorized);
        assert!(!evidence.claims_link_readiness);
        assert!(!evidence.claims_packet_io);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_CANDIDATE_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_bcm54213pe_readonly_preflight_formats_paired_control() {
        let report = build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
            Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                kind: Rp1EthernetBcm54213peReadonlyPreflightReportKind::NoMdioNoEthernetControl,
                source_contract: None,
                ..accepted_bcm54213pe_readonly_preflight_report_input()
            },
        )
        .expect("valid BCM54213PE read-only preflight control input");
        let evidence = rp1_ethernet_bcm54213pe_readonly_preflight_report_evidence(report);

        assert_eq!(evidence.report_kind, "no-mdio-no-ethernet-control");
        assert_eq!(evidence.selected_discriminator, None);
        assert_eq!(evidence.selected_targets, None);
        assert_eq!(evidence.controller, None);
        assert_eq!(evidence.phy_address, None);
        assert_eq!(evidence.register_names, None);
        assert_eq!(evidence.registers, None);
        assert_eq!(evidence.man_frames, None);
        assert_eq!(evidence.man_frame_construction, None);
        assert_eq!(evidence.already_sampled_context, None);
        assert_eq!(evidence.forbidden_targets, None);
        assert_eq!(evidence.source_evidence, None);
        assert!(!evidence.constructs_mdio_targets);
        assert!(!evidence.constructs_man_frames);
        assert!(!evidence.constructs_macb_target);
        assert!(!evidence.constructs_gpio_target);
        assert!(!evidence.runtime_volatile_access_intent);
        assert_eq!(
            evidence.classification,
            RP1_ETHERNET_BCM54213PE_READONLY_PREFLIGHT_CONTROL_CLASSIFICATION
        );
    }

    #[test_case]
    fn rp1_ethernet_bcm54213pe_readonly_preflight_rejects_shape_and_overclaims() {
        let input = accepted_bcm54213pe_readonly_preflight_report_input();

        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    source_contract: None,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::CandidateMissingSourceContract)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    kind: Rp1EthernetBcm54213peReadonlyPreflightReportKind::NoMdioNoEthernetControl,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::ControlCarriesMdioTargetFacts)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    source_contract: Some(
                        Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence {
                            accepted_physid2: 0x84a3,
                            ..rp1_ethernet_bcm54213pe_readonly_preflight_report_contract_evidence()
                        }
                    ),
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::SourceContractIdentityMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    source_contract: Some(
                        Rp1EthernetBcm54213peReadonlyPreflightReportContractEvidence {
                            registers: RP1_ETHERNET_MDIO_REGISTER_VECTOR_REGISTERS,
                            ..rp1_ethernet_bcm54213pe_readonly_preflight_report_contract_evidence()
                        }
                    ),
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::SourceContractFieldMismatch)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_runtime_volatile_access: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::RuntimeVolatileAccessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_hardware_proof_authorized: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::HardwareProofAuthorizedClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_gpio32_action: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::Gpio32ActionClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_bmcr_or_phy_write: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::BmcrOrPhyWriteClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_broadcom_shadow_mmd_aux_access: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::BroadcomShadowMmdAuxAccessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_interrupt_surface_access: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::InterruptSurfaceAccessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_phy_or_mac_configuration: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::PhyOrMacConfigurationClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_link_readiness: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::LinkReadinessClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_packet_io: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::PacketIoClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_networking: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::NetworkingClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_ssh: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::SshClaim)
        );
        assert_eq!(
            build_rp1_ethernet_bcm54213pe_readonly_preflight_report(
                Rp1EthernetBcm54213peReadonlyPreflightReportInput {
                    claims_phase_12_2: true,
                    ..input
                }
            ),
            Err(Rp1EthernetBcm54213peReadonlyPreflightReportError::Phase122Claim)
        );
        assert_eq!(
            rejected_rp1_ethernet_bcm54213pe_readonly_preflight_report_evidence(
                Rp1EthernetBcm54213peReadonlyPreflightReportError::PhaseTransitionClaim
            ),
            (
                RP1_ETHERNET_REJECTED_INPUT_CLASSIFICATION,
                "phase-transition-claim"
            )
        );
    }

    #[test_case]
    fn rp1_ethernet_phy1_gigabit_preflight_decodes_selected_registers() {
        let decoded =
            decode_rp1_ethernet_phy1_gigabit_preflight(Rp1EthernetPhy1GigabitPreflightRawVector {
                ctrl1000: 0x1f00,
                stat1000: 0xfc00,
            });

        assert!(decoded.ctrl1000.advertise_1000_half);
        assert!(decoded.ctrl1000.advertise_1000_full);
        assert!(decoded.ctrl1000.prefer_master);
        assert!(decoded.ctrl1000.force_master);
        assert!(decoded.ctrl1000.master_manual_config_enable);
        assert!(decoded.stat1000.master_slave_resolution_failure);
        assert!(decoded.stat1000.master_slave_resolution_master);
        assert!(decoded.stat1000.local_receiver_ok);
        assert!(decoded.stat1000.remote_receiver_ok);
        assert!(decoded.stat1000.link_partner_1000_full);
        assert!(decoded.stat1000.link_partner_1000_half);
    }

    #[test_case]
    fn rp1_ethernet_phy1_status_decodes_accepted_v4_vector() {
        let decoded = decode_rp1_ethernet_phy1_status(Rp1EthernetPhy1StatusRawVector {
            bmcr: 0x1000,
            bmsr: 0x7949,
            physid1: 0x600d,
            physid2: 0x84a2,
            anar: 0x01e1,
            anlpar: 0x0000,
        });

        assert_eq!(decoded.bmcr.speed_label, "10M");
        assert!(!decoded.bmcr.reset);
        assert!(!decoded.bmcr.loopback);
        assert!(decoded.bmcr.autoneg_enable);
        assert!(!decoded.bmcr.power_down);
        assert!(!decoded.bmcr.isolate);
        assert!(!decoded.bmcr.restart_autoneg);
        assert!(!decoded.bmcr.duplex_full);
        assert!(!decoded.bmcr.speed_select_lsb_100);
        assert!(!decoded.bmcr.speed_select_msb_1000);

        assert!(decoded.bmsr.capable_100base_x_full);
        assert!(decoded.bmsr.capable_100base_x_half);
        assert!(decoded.bmsr.capable_10_full);
        assert!(decoded.bmsr.capable_10_half);
        assert!(decoded.bmsr.extended_status);
        assert!(!decoded.bmsr.autoneg_complete);
        assert!(decoded.bmsr.autoneg_ability);
        assert!(!decoded.bmsr.link_status);
        assert!(decoded.bmsr.extended_capability);

        assert_eq!(decoded.phy_id.oui, 0x180361);
        assert_eq!(decoded.phy_id.model, 0x0a);
        assert_eq!(decoded.phy_id.revision, 0x02);

        assert_eq!(decoded.anar.selector, 1);
        assert!(decoded.anar.capable_10_half);
        assert!(decoded.anar.capable_10_full);
        assert!(decoded.anar.capable_100_tx_half);
        assert!(decoded.anar.capable_100_tx_full);
        assert!(!decoded.anar.capable_100_t4);
        assert!(!decoded.anar.pause);
        assert!(!decoded.anar.asym_pause);
        assert_eq!(decoded.anlpar.selector, 0);
        assert!(!decoded.anlpar.capable_10_half);
        assert!(!decoded.anlpar.capable_10_full);
        assert!(!decoded.anlpar.capable_100_tx_half);
        assert!(!decoded.anlpar.capable_100_tx_full);
    }
}
