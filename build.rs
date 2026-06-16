use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

struct BootScenario {
    value: &'static str,
    implied_values: &'static [&'static str],
    asm_defines: &'static [&'static str],
}

const QEMU_SMP_ASM: &str = "TALOS_QEMU_SMP_BOOT_SCENARIO";
const RPI5_SMP_ASM: &str = "TALOS_RPI5_SMP_BOOT_SCENARIO";
const RPI5_EL0_TRAP_PROOF_ASM: &str = "TALOS_RPI5_EL0_TRAP_PROOF_SCENARIO";
const RPI5_SYSCALL_PROOF_ASM: &str = "TALOS_RPI5_SYSCALL_PROOF_SCENARIO";
const QEMU_SYSCALL_SMOKE_ASM: &str = "TALOS_QEMU_SYSCALL_SMOKE_SCENARIO";
const GENERATED_ROOT_MANIFEST: &str = "userland/generated-root.manifest";
const GENERATED_INITRAMFS_RUST: &str = "generated_initramfs.rs";
const GENERATED_ROOT_IDENTITY: &str = "phase10-generated-root-manifest-v1";
const GENERATED_ROOT_EXEC_ELF_LEN: usize = 0x204;
const GENERATED_ROOT_EXEC_TEXT_OFFSET: usize = 0x100;

const BOOT_SCENARIOS: &[BootScenario] = &[
    BootScenario {
        value: "rpi5_timer_irq",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_timer_preemption",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_uart10_polling_rx",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_diagnostic_command_channel",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_serial_command_loop",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_echo_command",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_literal_echo",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_help_command",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_ls_root",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_ls_bin",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_cat_banner",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_cat_cwd",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_cd_fixed_dirs",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_ls_cwd",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_pwd_command",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_line_editing",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_line_cancel",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_local_line_kill",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_generated_root_boot_transport",
        implied_values: &["rpi5_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_uart0_fr_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_uart0_fr_read_delayed_marker",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_final_preload_marker_hold",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_uart0_fr_read_hold_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_uart0_fr_shaped_no_mmio_marker",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_uart0_fr_tail_stable_result",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_uart0_fr_tail_stable_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio14_status_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio14_status_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_interrupt_routing_msix_cfg_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_interrupt_routing_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gic_visible_route_status_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gic_visible_route_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio_bank_source_status_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio_bank_source_status_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_manager_status_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_manager_status_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_adc_ctrl_write_restore",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_adc_ctrl_enable_toggle",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_adc_window_coherence_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_adc_window_coherence_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_sysinfo_clock_sentinel_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_reset_dependency_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_clock_reset_dependency_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_prereq_ownership_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_event_state_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_event_state_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_event_clear_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_gpio32_event_clear_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_phy_id_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_phy_id_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_phy_id_after_mpe_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_phy_id_after_mpe_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_mpe_enable_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_mpe_enable_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_register_vector_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_register_vector_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_phy1_status_diagnostic_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_phy1_status_diagnostic_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_macb_nsr_link_readonly_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_macb_nsr_link_readonly_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_phy1_autoneg_restart_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_phy1_autoneg_restart_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_post_physical_link_status_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_post_physical_link_status_no_mdio_macb_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_no_mdio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_kernel_entry_serial_beacon",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_register_vector_staging_sentinel_candidate",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_ethernet_mdio_register_vector_staging_sentinel_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_pcie2_host_link_status_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_pcie2_host_link_status_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_endpoint_config_identity_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_endpoint_config_identity_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_bridge_config_preflight_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_bridge_config_preflight_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_bridge_setup_state_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_bridge_setup_state_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_observed_aperture_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_observed_aperture_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_observed_gpio_status_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_observed_gpio_status_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio14_ownership_route_preflight_read",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio16_owned_event_discriminator",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_entry_control",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_handoff_reset",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rp1_post_handoff_marker_reset",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_rust_entry_uart10_marker_loop",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_psci_secondary_core_alive",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_secondary_core_workload",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_smp_lock_cache_coherence",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_cross_core_ipi_delivery",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_remote_wakeup_request",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_remote_wake_to_local_runnable",
        implied_values: &["rpi5_remote_wakeup_request"],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_production_secondary_dispatch",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_shared_scheduler_metadata",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_shared_runqueue_migration",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_load_balancing_proof",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "rpi5_multicore_preemption_proof",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_production_timer_preemption_proof",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "rpi5_el0_trap_proof",
        implied_values: &[],
        asm_defines: &[RPI5_EL0_TRAP_PROOF_ASM],
    },
    BootScenario {
        value: "rpi5_syscall_proof",
        implied_values: &[],
        asm_defines: &[RPI5_SYSCALL_PROOF_ASM],
    },
    BootScenario {
        value: "rpi5_pointer_copy_proof",
        implied_values: &["rpi5_syscall_proof"],
        asm_defines: &[RPI5_SYSCALL_PROOF_ASM],
    },
    BootScenario {
        value: "rpi5_descriptor_write_proof",
        implied_values: &["rpi5_syscall_proof"],
        asm_defines: &[RPI5_SYSCALL_PROOF_ASM],
    },
    BootScenario {
        value: "rpi5_close_syscall_proof",
        implied_values: &["rpi5_syscall_proof"],
        asm_defines: &[RPI5_SYSCALL_PROOF_ASM],
    },
    BootScenario {
        value: "rpi5_dup_syscall_proof",
        implied_values: &["rpi5_syscall_proof"],
        asm_defines: &[RPI5_SYSCALL_PROOF_ASM],
    },
    BootScenario {
        value: "rpi5_read_stdin_proof",
        implied_values: &["rpi5_syscall_proof"],
        asm_defines: &[RPI5_SYSCALL_PROOF_ASM],
    },
    BootScenario {
        value: "rpi5_secondary_scheduler_service_loop",
        implied_values: &[],
        asm_defines: &[RPI5_SMP_ASM],
    },
    BootScenario {
        value: "qemu_context_switch",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_scheduler_yield",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_timer_preemption",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_polling_tty_rx",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_diagnostic_command_channel",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_serial_command_loop",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_command_stdin_descriptor",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_echo_command",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_literal_echo",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_help_command",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_pwd_command",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_ls_root",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_ls_bin",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_cat_banner",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_cat_cwd",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_vfs_exec",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_literal_argv",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_path_lookup",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdout",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdin",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stderr",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_distinct_stderr_routing",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdout_to_stderr_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_dev_null_stdout_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_dev_null_stderr_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_dev_null_stdin_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_readonly_regular_file_stdin_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdout_regular_file_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdout_regular_file_append_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdout_regular_file_append_create_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_explicit_fd1_regular_file_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdout_arbitrary_tmp_output_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stderr_arbitrary_tmp_output_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_combined_stdin_stdout_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_pipeline_consumer_output_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_pipeline_producer_file_redirection_away",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_background_vfs_exec_lifecycle",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_jobs_accounting_list",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_multiple_background_jobs",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_background_jobs_stale_entry_policy",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_generated_userland_manifest",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stderr_regular_file_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stderr_regular_file_append_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stderr_regular_file_append_create_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stderr_to_stdout_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stdout_close_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_stderr_close_redirection",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_minimal_stdout_to_stdin_pipeline",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_pipeline_stderr_not_piped",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_pipeline_stderr_dup_to_stdout",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_pipeline_stdout_redirect_away",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_shell_waitpid",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_cd_fixed_dirs",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_ls_cwd",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_line_editing",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_line_cancel",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_local_line_kill",
        implied_values: &["qemu_local_serial_command_loop"],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_secondary_core_workload",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_smp_lock_contention",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_per_core_scheduler_ownership",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_cross_core_ipi_delivery",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_remote_wakeup_request",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_remote_wake_to_local_runnable",
        implied_values: &["qemu_remote_wakeup_request"],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_production_secondary_dispatch",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_shared_scheduler_metadata",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_shared_runqueue_migration",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_load_balancing_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_secondary_scheduler_service_loop",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_multicore_preemption_smoke",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_production_timer_preemption_smoke",
        implied_values: &[],
        asm_defines: &[QEMU_SMP_ASM],
    },
    BootScenario {
        value: "qemu_el0_trap_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_syscall_smoke",
        implied_values: &[],
        asm_defines: &[QEMU_SYSCALL_SMOKE_ASM],
    },
    BootScenario {
        value: "qemu_pointer_copy_smoke",
        implied_values: &[],
        asm_defines: &[QEMU_SYSCALL_SMOKE_ASM],
    },
    BootScenario {
        value: "qemu_descriptor_write_smoke",
        implied_values: &[],
        asm_defines: &[QEMU_SYSCALL_SMOKE_ASM],
    },
    BootScenario {
        value: "qemu_process_descriptor_stdio_smoke",
        implied_values: &["qemu_descriptor_write_smoke"],
        asm_defines: &[QEMU_SYSCALL_SMOKE_ASM],
    },
    BootScenario {
        value: "qemu_close_syscall_smoke",
        implied_values: &["qemu_descriptor_write_smoke"],
        asm_defines: &[QEMU_SYSCALL_SMOKE_ASM],
    },
    BootScenario {
        value: "qemu_dup_syscall_smoke",
        implied_values: &["qemu_descriptor_write_smoke"],
        asm_defines: &[QEMU_SYSCALL_SMOKE_ASM],
    },
    BootScenario {
        value: "qemu_read_stdin_smoke",
        implied_values: &["qemu_descriptor_write_smoke"],
        asm_defines: &[QEMU_SYSCALL_SMOKE_ASM],
    },
    BootScenario {
        value: "qemu_readonly_initramfs_vfs_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_open_read_syscall_surface_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_program_loader_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_program_loader_from_vfs_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_process_install_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_process_address_space_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_process_page_table_materialization_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_initial_process_launch_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_initial_userspace_process_launch_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_initial_user_stack_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_live_address_space_activation_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_kernel_half_reachability_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_kernel_half_descriptor_image_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_live_descriptor_image_installation_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
    BootScenario {
        value: "qemu_live_translation_register_activation_smoke",
        implied_values: &[],
        asm_defines: &[],
    },
];

fn main() {
    println!("cargo:rerun-if-changed=src/arch/aarch64/boot.S");
    println!("cargo:rerun-if-changed=src/arch/aarch64/vectors.S");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=linker-rpi5.ld");
    println!("cargo:rerun-if-env-changed=TALOS_BOOT_SCENARIO");
    println!("cargo:rerun-if-env-changed=TALOS_CAPTURE_NONCE");
    println!("cargo:rerun-if-changed={GENERATED_ROOT_MANIFEST}");
    if let Ok(nonce) = env::var("TALOS_CAPTURE_NONCE") {
        validate_capture_nonce(&nonce);
        println!("cargo:rustc-env=TALOS_CAPTURE_NONCE={nonce}");
    }

    let target = env::var("TARGET").expect("TARGET is set by Cargo");
    let scenario = selected_boot_scenario();
    register_check_cfgs();
    if let Some(scenario) = scenario {
        emit_scenario_cfg(scenario.value);
        for value in scenario.implied_values {
            emit_scenario_cfg(value);
        }
    }

    if target.contains("rpi5") || target.contains("bcm2712") {
        println!("cargo:rustc-cfg=talos_target_rpi5_bcm2712");
    } else {
        println!("cargo:rustc-cfg=talos_target_qemu_virt");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    generate_initramfs_manifest(&out_dir);

    if !target.starts_with("aarch64") {
        return;
    }

    let boot_obj = out_dir.join("boot.o");
    let vectors_obj = out_dir.join("vectors.o");

    assemble_aarch64("src/arch/aarch64/boot.S", &boot_obj, &target, scenario);
    assemble_aarch64(
        "src/arch/aarch64/vectors.S",
        &vectors_obj,
        &target,
        scenario,
    );

    println!("cargo:rustc-link-arg={}", boot_obj.display());
    println!("cargo:rustc-link-arg={}", vectors_obj.display());
}

fn selected_boot_scenario() -> Option<&'static BootScenario> {
    let value = env::var("TALOS_BOOT_SCENARIO").ok()?;
    Some(
        BOOT_SCENARIOS
            .iter()
            .find(|scenario| scenario.value == value)
            .unwrap_or_else(|| panic!("unsupported TALOS_BOOT_SCENARIO: {value}")),
    )
}

fn register_check_cfgs() {
    println!("cargo:rustc-check-cfg=cfg(talos_target_qemu_virt)");
    println!("cargo:rustc-check-cfg=cfg(talos_target_rpi5_bcm2712)");

    let values = BOOT_SCENARIOS
        .iter()
        .map(|scenario| format!(r#""{}""#, scenario.value))
        .collect::<Vec<_>>()
        .join(", ");
    println!("cargo:rustc-check-cfg=cfg(talos_boot_scenario, values({values}))");
}

fn emit_scenario_cfg(value: &str) {
    println!("cargo:rustc-cfg=talos_boot_scenario=\"{value}\"");
}

fn validate_capture_nonce(nonce: &str) {
    if nonce.len() > 64 {
        panic!("TALOS_CAPTURE_NONCE must be 64 characters or fewer");
    }
    if !nonce
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b':' | b'-'))
    {
        panic!("TALOS_CAPTURE_NONCE may contain only A-Z, a-z, 0-9, _, ., :, and -");
    }
}

fn generate_initramfs_manifest(out_dir: &PathBuf) {
    let manifest = fs::read_to_string(GENERATED_ROOT_MANIFEST)
        .unwrap_or_else(|error| panic!("failed to read {GENERATED_ROOT_MANIFEST}: {error}"));
    let manifest = parse_generated_manifest(&manifest);
    let executable_bytes = build_generated_root_exit_elf_bytes(manifest.executable.exit_status);
    let digest = generated_root_digest(
        manifest.file.path.as_bytes(),
        manifest.file.contents.as_bytes(),
        manifest.executable.path.as_bytes(),
        &executable_bytes,
    );
    let output = out_dir.join(GENERATED_INITRAMFS_RUST);
    let mut file = fs::File::create(&output)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", output.display()));

    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_IDENTITY: &str = {:?};",
        GENERATED_ROOT_IDENTITY
    )
    .expect("write generated root identity");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_SOURCE: &str = {:?};",
        GENERATED_ROOT_MANIFEST
    )
    .expect("write generated root source");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_DIGEST: u64 = {digest:#018x};"
    )
    .expect("write generated root digest");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_FILE_PATH: &[u8] = &{:?};",
        manifest.file.path.as_bytes()
    )
    .expect("write generated root path");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_DIR_NAME: &[u8] = &{:?};",
        manifest.file.directory.as_bytes()
    )
    .expect("write generated root dir");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_FILE_NAME: &[u8] = &{:?};",
        manifest.file.file_name.as_bytes()
    )
    .expect("write generated root file name");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_FILE_BYTES: &[u8] = &{:?};",
        manifest.file.contents.as_bytes()
    )
    .expect("write generated root bytes");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_EXEC_PATH: &[u8] = &{:?};",
        manifest.executable.path.as_bytes()
    )
    .expect("write generated root executable path");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_EXEC_NAME: &[u8] = &{:?};",
        manifest.executable.file_name.as_bytes()
    )
    .expect("write generated root executable file name");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_EXEC_EXIT_STATUS: u64 = {:#018x};",
        manifest.executable.exit_status
    )
    .expect("write generated root executable exit status");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_EXEC_BYTES: &[u8] = &{:?};",
        executable_bytes
    )
    .expect("write generated root executable bytes");
}

struct GeneratedRootManifest {
    file: GeneratedManifestFile,
    executable: GeneratedManifestExecutable,
}

struct GeneratedManifestFile {
    path: String,
    directory: String,
    file_name: String,
    contents: String,
}

struct GeneratedManifestExecutable {
    path: String,
    file_name: String,
    exit_status: u64,
}

fn parse_generated_manifest(manifest: &str) -> GeneratedRootManifest {
    let mut file_path = None;
    let mut file_contents = None;
    let mut exec_path = None;
    let mut exec_exit_status = None;

    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = trimmed
            .split_once('=')
            .unwrap_or_else(|| panic!("invalid generated manifest line: {trimmed}"));
        match key {
            "file.path" => file_path = Some(value.to_string()),
            "file.contents" => file_contents = Some(unescape_manifest_value(value)),
            "exec.path" => exec_path = Some(value.to_string()),
            "exec.exit_status" => {
                exec_exit_status = Some(
                    value
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("invalid generated executable status: {value}")),
                )
            }
            _ => panic!("unsupported generated manifest key: {key}"),
        }
    }

    let file_path = file_path.expect("generated manifest file.path is required");
    let file_contents = file_contents.expect("generated manifest file.contents is required");
    validate_generated_manifest_file(&file_path, &file_contents);
    let (file_directory, file_name) = split_generated_manifest_path(&file_path);

    let exec_path = exec_path.expect("generated manifest exec.path is required");
    let exec_exit_status =
        exec_exit_status.expect("generated manifest exec.exit_status is required");
    validate_generated_manifest_executable(&exec_path, exec_exit_status);
    let (exec_directory, exec_name) = split_generated_manifest_path(&exec_path);

    if file_directory != exec_directory {
        panic!("generated manifest first slice requires file and executable in one directory");
    }
    if file_name == exec_name {
        panic!("generated manifest file and executable names must differ");
    }

    GeneratedRootManifest {
        file: GeneratedManifestFile {
            path: file_path,
            directory: file_directory,
            file_name,
            contents: file_contents,
        },
        executable: GeneratedManifestExecutable {
            path: exec_path,
            file_name: exec_name,
            exit_status: exec_exit_status,
        },
    }
}

fn split_generated_manifest_path(path: &str) -> (String, String) {
    let (directory, file_name) = path
        .strip_prefix('/')
        .expect("validated generated manifest path is absolute")
        .split_once('/')
        .unwrap_or_else(|| panic!("generated manifest path must include one directory: {path}"));
    let directory = directory.to_string();
    let file_name = file_name.to_string();
    if directory.is_empty() || file_name.is_empty() || file_name.contains('/') {
        panic!("generated manifest path must be /directory/file: {path}");
    }

    (directory, file_name)
}

fn validate_generated_manifest_path(path: &str) {
    if !path.starts_with('/') || path == "/" {
        panic!("generated manifest path must be absolute and non-root: {path}");
    }
    if path.as_bytes().contains(&0) {
        panic!("generated manifest path must not contain NUL bytes");
    }
    if path.contains("//") || path.split('/').any(|part| part == "." || part == "..") {
        panic!("generated manifest path must be normalized inside root: {path}");
    }
}

fn validate_generated_manifest_file(path: &str, contents: &str) {
    validate_generated_manifest_path(path);
    if contents.as_bytes().contains(&0) {
        panic!("generated manifest contents must not contain NUL bytes");
    }
    if contents.len() > 4096 {
        panic!("generated manifest file exceeds first-slice 4096-byte limit");
    }
}

fn validate_generated_manifest_executable(path: &str, exit_status: u64) {
    validate_generated_manifest_path(path);
    if exit_status > 0xffff {
        panic!("generated executable exit status exceeds first-slice 16-bit status limit");
    }
}

fn unescape_manifest_value(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        match chars.next() {
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some('t') => output.push('\t'),
            Some('\\') => output.push('\\'),
            Some(other) => panic!("unsupported generated manifest escape: \\{other}"),
            None => panic!("trailing generated manifest escape"),
        }
    }
    output
}

fn generated_root_digest(
    file_path: &[u8],
    file_contents: &[u8],
    exec_path: &[u8],
    exec_contents: &[u8],
) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

    let mut hash = FNV_OFFSET;
    for byte in GENERATED_ROOT_IDENTITY.as_bytes() {
        hash = fnv_step(hash, *byte);
    }
    hash = fnv_step(hash, 0);
    for byte in file_path {
        hash = fnv_step(hash, *byte);
    }
    hash = fnv_step(hash, 0);
    for byte in file_contents {
        hash = fnv_step(hash, *byte);
    }
    hash = fnv_step(hash, 0);
    for byte in exec_path {
        hash = fnv_step(hash, *byte);
    }
    hash = fnv_step(hash, 0);
    for byte in exec_contents {
        hash = fnv_step(hash, *byte);
    }
    hash
}

fn build_generated_root_exit_elf_bytes(exit_status: u64) -> Vec<u8> {
    const EHDR_LEN: usize = 64;
    const PHENT_LEN: usize = 56;
    const DATA_OFFSET: usize = 0x200;
    const TEXT_VADDR: u64 = 0x0000_0000_0001_0100;
    const DATA_VADDR: u64 = 0x0000_0000_0002_0200;
    const ENTRY: u64 = TEXT_VADDR;
    const PF_X: u32 = 1;
    const PF_W: u32 = 2;
    const PF_R: u32 = 4;
    const PAGE_ALIGN: u64 = 0x1000;

    let mut bytes = vec![0u8; GENERATED_ROOT_EXEC_ELF_LEN];
    bytes[0] = 0x7f;
    bytes[1] = b'E';
    bytes[2] = b'L';
    bytes[3] = b'F';
    bytes[4] = 2;
    bytes[5] = 1;
    bytes[6] = 1;

    write_le_u16(&mut bytes, 16, 2);
    write_le_u16(&mut bytes, 18, 183);
    write_le_u32(&mut bytes, 20, 1);
    write_le_u64(&mut bytes, 24, ENTRY);
    write_le_u64(&mut bytes, 32, EHDR_LEN as u64);
    write_le_u16(&mut bytes, 52, EHDR_LEN as u16);
    write_le_u16(&mut bytes, 54, PHENT_LEN as u16);
    write_le_u16(&mut bytes, 56, 2);

    write_load_phdr(
        &mut bytes,
        EHDR_LEN,
        PF_R | PF_X,
        GENERATED_ROOT_EXEC_TEXT_OFFSET as u64,
        TEXT_VADDR,
        8,
        8,
        PAGE_ALIGN,
    );
    write_load_phdr(
        &mut bytes,
        EHDR_LEN + PHENT_LEN,
        PF_R | PF_W,
        DATA_OFFSET as u64,
        DATA_VADDR,
        4,
        0x1004,
        PAGE_ALIGN,
    );

    let exit_status = (exit_status & 0xffff) as u32;
    let movz_x0 = 0xd280_0000u32 | (exit_status << 5);
    write_le_u32(&mut bytes, GENERATED_ROOT_EXEC_TEXT_OFFSET, movz_x0);
    write_le_u32(&mut bytes, GENERATED_ROOT_EXEC_TEXT_OFFSET + 4, 0xd40f_4201);
    bytes[DATA_OFFSET] = b'D';
    bytes[DATA_OFFSET + 1] = b'A';
    bytes[DATA_OFFSET + 2] = b'T';
    bytes[DATA_OFFSET + 3] = b'A';

    bytes
}

fn write_load_phdr(
    bytes: &mut [u8],
    offset: usize,
    flags: u32,
    file_offset: u64,
    virtual_address: u64,
    file_size: u64,
    memory_size: u64,
    alignment: u64,
) {
    write_le_u32(bytes, offset, 1);
    write_le_u32(bytes, offset + 4, flags);
    write_le_u64(bytes, offset + 8, file_offset);
    write_le_u64(bytes, offset + 16, virtual_address);
    write_le_u64(bytes, offset + 24, virtual_address);
    write_le_u64(bytes, offset + 32, file_size);
    write_le_u64(bytes, offset + 40, memory_size);
    write_le_u64(bytes, offset + 48, alignment);
}

fn write_le_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset] = value as u8;
    bytes[offset + 1] = (value >> 8) as u8;
}

fn write_le_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset] = value as u8;
    bytes[offset + 1] = (value >> 8) as u8;
    bytes[offset + 2] = (value >> 16) as u8;
    bytes[offset + 3] = (value >> 24) as u8;
}

fn write_le_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset] = value as u8;
    bytes[offset + 1] = (value >> 8) as u8;
    bytes[offset + 2] = (value >> 16) as u8;
    bytes[offset + 3] = (value >> 24) as u8;
    bytes[offset + 4] = (value >> 32) as u8;
    bytes[offset + 5] = (value >> 40) as u8;
    bytes[offset + 6] = (value >> 48) as u8;
    bytes[offset + 7] = (value >> 56) as u8;
}

fn fnv_step(hash: u64, byte: u8) -> u64 {
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    (hash ^ byte as u64).wrapping_mul(FNV_PRIME)
}

fn assemble_aarch64(source: &str, output: &PathBuf, target: &str, scenario: Option<&BootScenario>) {
    let mut command = Command::new("clang");
    command.args([
        "-target",
        "aarch64-none-elf",
        "-ffreestanding",
        "-fno-stack-protector",
        "-c",
    ]);

    if target.contains("rpi5") || target.contains("bcm2712") {
        command.arg("-DTALOS_TARGET_RPI5_BCM2712");
    }
    if let Some(scenario) = scenario {
        for define in scenario.asm_defines {
            command.arg(format!("-D{define}"));
        }
    }

    let status = command
        .arg(source)
        .arg("-o")
        .arg(output)
        .status()
        .expect("failed to run clang for AArch64 assembly");

    if !status.success() {
        panic!("AArch64 assembly failed for {source}");
    }
}
