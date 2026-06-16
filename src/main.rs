#![no_std]
#![no_main]
#![cfg_attr(
    all(not(test), talos_target_rpi5_bcm2712),
    feature(alloc_error_handler)
)]
#![cfg_attr(
    all(
        talos_target_rpi5_bcm2712,
        any(
            talos_boot_scenario = "rpi5_timer_preemption",
            talos_boot_scenario = "rpi5_diagnostic_command_channel",
            talos_boot_scenario = "rpi5_local_serial_command_loop",
            talos_boot_scenario = "rpi5_local_echo_command",
            talos_boot_scenario = "rpi5_local_literal_echo",
            talos_boot_scenario = "rpi5_local_help_command",
            talos_boot_scenario = "rpi5_local_pwd_command",
            talos_boot_scenario = "rpi5_local_ls_root",
            talos_boot_scenario = "rpi5_local_ls_bin",
            talos_boot_scenario = "rpi5_local_cat_banner",
            talos_boot_scenario = "rpi5_local_cat_cwd",
            talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
            talos_boot_scenario = "rpi5_local_ls_cwd",
            talos_boot_scenario = "rpi5_local_line_editing",
            talos_boot_scenario = "rpi5_local_line_cancel",
            talos_boot_scenario = "rpi5_local_line_kill",
            talos_boot_scenario = "rpi5_psci_secondary_core_alive",
            talos_boot_scenario = "rpi5_secondary_core_workload",
            talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
            talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
            talos_boot_scenario = "rpi5_secondary_scheduler_service_loop",
            talos_boot_scenario = "rpi5_shared_runqueue_migration",
            talos_boot_scenario = "rpi5_load_balancing_proof",
            talos_boot_scenario = "rpi5_multicore_preemption_proof",
            talos_boot_scenario = "rpi5_production_timer_preemption_proof",
            talos_boot_scenario = "rpi5_el0_trap_proof",
            talos_boot_scenario = "rpi5_syscall_proof",
            talos_boot_scenario = "rpi5_pointer_copy_proof",
            talos_boot_scenario = "rpi5_rp1_entry_control",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_read",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_read_delayed_marker",
            talos_boot_scenario = "rpi5_rp1_final_preload_marker_hold",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_read_hold_control",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_shaped_no_mmio_marker",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_result",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio14_status_read",
            talos_boot_scenario = "rpi5_rp1_gpio14_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_interrupt_routing_msix_cfg_read",
            talos_boot_scenario = "rpi5_rp1_interrupt_routing_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read",
            talos_boot_scenario = "rpi5_rp1_gic_visible_route_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_read",
            talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_manager_status_read",
            talos_boot_scenario = "rpi5_rp1_clock_manager_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore",
            talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle",
            talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read",
            talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read",
            talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
            talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate",
            talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_state_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_state_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_clear_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_clear_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_after_mpe_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_after_mpe_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_mpe_enable_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_mpe_enable_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_phy1_status_diagnostic_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_phy1_status_diagnostic_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_macb_nsr_link_readonly_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_macb_nsr_link_readonly_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_phy1_autoneg_restart_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_phy1_autoneg_restart_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_post_physical_link_status_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_post_physical_link_status_no_mdio_macb_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_kernel_entry_serial_beacon",
            talos_boot_scenario = "rpi5_rp1_ethernet_bootinfo_report_serial_visibility_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bootinfo_report_serial_visibility_earliest_only_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_staging_sentinel_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_staging_sentinel_control",
            talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read",
            talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read",
            talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
            talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
            talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
            talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
            talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_handoff_reset",
            talos_boot_scenario = "rpi5_rp1_post_handoff_marker_reset",
            talos_boot_scenario = "rpi5_rust_entry_uart10_marker_loop",
        )
    ),
    allow(dead_code, unused_imports, unused_variables, unreachable_code)
)]
#![cfg_attr(
    all(
        not(test),
        any(
            talos_boot_scenario = "qemu_polling_tty_rx",
            talos_boot_scenario = "qemu_diagnostic_command_channel",
            talos_boot_scenario = "qemu_secondary_core_workload",
            talos_boot_scenario = "qemu_smp_lock_contention",
            talos_boot_scenario = "qemu_per_core_scheduler_ownership",
            talos_boot_scenario = "qemu_cross_core_ipi_delivery",
            talos_boot_scenario = "qemu_remote_wakeup_request",
            talos_boot_scenario = "qemu_production_secondary_dispatch",
            talos_boot_scenario = "qemu_shared_scheduler_metadata",
            talos_boot_scenario = "qemu_shared_runqueue_migration",
            talos_boot_scenario = "qemu_load_balancing_smoke",
            talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
            talos_boot_scenario = "qemu_multicore_preemption_smoke",
            talos_boot_scenario = "qemu_production_timer_preemption_smoke",
            talos_boot_scenario = "qemu_el0_trap_smoke",
            talos_boot_scenario = "qemu_syscall_smoke",
            talos_boot_scenario = "qemu_pointer_copy_smoke",
            talos_boot_scenario = "qemu_local_serial_command_loop",
            talos_boot_scenario = "qemu_local_shell_distinct_stderr_routing",
            talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke",
            talos_boot_scenario = "qemu_open_read_syscall_surface_smoke",
            talos_boot_scenario = "qemu_program_loader_smoke",
            talos_boot_scenario = "qemu_program_loader_from_vfs_smoke",
            talos_boot_scenario = "qemu_process_install_smoke",
            talos_boot_scenario = "qemu_process_address_space_smoke",
            talos_boot_scenario = "qemu_process_page_table_materialization_smoke",
            talos_boot_scenario = "qemu_initial_process_launch_smoke",
            talos_boot_scenario = "qemu_initial_userspace_process_launch_smoke",
            talos_boot_scenario = "qemu_initial_user_stack_smoke",
            talos_boot_scenario = "qemu_live_address_space_activation_smoke",
            talos_boot_scenario = "qemu_kernel_half_reachability_smoke",
            talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke",
            talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke",
            talos_boot_scenario = "qemu_live_translation_register_activation_smoke",
            talos_boot_scenario = "qemu_descriptor_write_smoke"
        )
    ),
    allow(dead_code, unused_imports, unused_variables, unreachable_code)
)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
extern crate alloc;

mod allocator;
mod arch;
mod boot;
mod device_tree;
#[cfg_attr(
    all(
        not(test),
        not(any(
            talos_boot_scenario = "qemu_diagnostic_command_channel",
            talos_boot_scenario = "rpi5_diagnostic_command_channel"
        ))
    ),
    allow(dead_code)
)]
mod diagnostic_command;
mod diagnostics;
#[cfg_attr(not(test), allow(dead_code))]
mod dma_cache;
mod early_format;
#[cfg_attr(not(test), allow(dead_code))]
mod initial_process_launch;
#[cfg_attr(not(test), allow(dead_code))]
mod initial_user_stack;
#[cfg_attr(not(test), allow(dead_code))]
mod initramfs;
#[cfg_attr(not(test), allow(dead_code))]
mod kernel_half_descriptor_image;
#[cfg_attr(not(test), allow(dead_code))]
mod kernel_half_reachability;
#[cfg_attr(not(test), allow(dead_code))]
mod live_address_space_activation;
#[cfg_attr(not(test), allow(dead_code))]
mod live_descriptor_image_installation;
#[cfg_attr(not(test), allow(dead_code))]
mod live_translation_register_activation;
#[cfg_attr(not(test), allow(dead_code))]
mod local_command_loop;
mod memory_map;
mod mmio;
mod pl011;
#[cfg_attr(not(test), allow(dead_code))]
mod posix;
#[cfg_attr(not(test), allow(dead_code))]
mod process_address_space;
#[cfg_attr(not(test), allow(dead_code))]
mod process_install;
#[cfg_attr(not(test), allow(dead_code))]
mod process_page_table_materialization;
#[cfg_attr(not(test), allow(dead_code))]
mod program_loader;
#[cfg_attr(not(test), allow(dead_code))]
mod rp1_ethernet;
mod runtime_console;
// Phase 6.1 accepts per-core ownership before boot-time hardware use.
#[cfg_attr(not(test), allow(dead_code))]
mod smp;
// Phase 6.2 accepts narrow SMP-safe primitives before scheduler sharing.
#[cfg_attr(not(test), allow(dead_code))]
mod smp_sync;
// Phase 4.3 accepts scheduler data structures before wiring boot-time use.
#[cfg_attr(not(test), allow(dead_code))]
mod scheduler;
#[cfg_attr(not(test), allow(dead_code))]
mod syscall;
mod target;
#[cfg_attr(
    not(any(
        talos_boot_scenario = "qemu_polling_tty_rx",
        talos_boot_scenario = "qemu_diagnostic_command_channel",
        talos_boot_scenario = "qemu_local_serial_command_loop",
        talos_boot_scenario = "rpi5_diagnostic_command_channel",
        talos_boot_scenario = "rpi5_local_serial_command_loop",
        talos_boot_scenario = "rpi5_local_echo_command",
        talos_boot_scenario = "rpi5_local_literal_echo",
        talos_boot_scenario = "rpi5_local_help_command",
        talos_boot_scenario = "rpi5_local_pwd_command",
        talos_boot_scenario = "rpi5_local_ls_root",
        talos_boot_scenario = "rpi5_local_ls_bin",
        talos_boot_scenario = "rpi5_local_cat_banner",
        talos_boot_scenario = "rpi5_local_cat_cwd",
        talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
        talos_boot_scenario = "rpi5_local_ls_cwd",
        talos_boot_scenario = "rpi5_local_line_editing",
        talos_boot_scenario = "rpi5_local_line_cancel",
        talos_boot_scenario = "rpi5_local_line_kill"
    )),
    allow(dead_code)
)]
mod tty;

use core::panic::PanicInfo;
#[cfg(talos_target_rpi5_bcm2712)]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) struct PanicInProgress(AtomicUsize);

use boot::BootInfo;

#[cfg_attr(not(test), global_allocator)]
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub(crate) static KERNEL_GLOBAL_ALLOCATOR: allocator::BumpAllocator =
    allocator::BumpAllocator::new();

#[cfg(talos_target_rpi5_bcm2712)]
impl PanicInProgress {
    const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    fn enter(&self) -> bool {
        self.0
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) static PANIC_IN_PROGRESS: PanicInProgress = PanicInProgress::new();

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    target::console::write_static("talos: alloc error: size=");
    target::console::write_hex_u64(layout.size() as u64);
    target::console::write_static(" align=");
    target::console::write_hex_u64(layout.align() as u64);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_entry(dtb_pa: usize) -> ! {
    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_handoff_reset"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_handoff_reset_diagnostic();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_post_handoff_marker_reset"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::RustEntry);
        target::rpi5::run_rp1_post_handoff_marker_reset_diagnostic();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rust_entry_uart10_marker_loop"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rust_entry_uart10_marker_loop();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_uart0_fr_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_uart0_fr_read_diagnostic();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_uart0_fr_read_delayed_marker"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_uart0_fr_read_delayed_marker_diagnostic();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_final_preload_marker_hold"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_final_preload_marker_hold();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_uart0_fr_read_hold_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_uart0_fr_read_hold_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_uart0_fr_shaped_no_mmio_marker"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_uart0_fr_shaped_no_mmio_marker();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_result"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_uart0_fr_tail_stable_result();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_uart0_fr_tail_stable_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio14_status_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio14_status_read_diagnostic();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio14_status_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio14_status_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_interrupt_routing_msix_cfg_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_interrupt_routing_msix_cfg_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_interrupt_routing_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_interrupt_routing_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gic_visible_route_status_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gic_visible_route_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gic_visible_route_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio_bank_source_status_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio_bank_source_status_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_manager_status_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_manager_status_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_manager_status_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_manager_status_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_adc_ctrl_write_restore();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_adc_ctrl_write_restore_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_adc_ctrl_enable_toggle();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_adc_window_coherence_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_adc_window_coherence_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_sysinfo_clock_sentinel_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_sysinfo_clock_sentinel_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_reset_dependency_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_clock_reset_dependency_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_dma_cache_small_diagnostic_visibility_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_dma_cache_small_diagnostic_visibility_no_plan_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gem_mid_visibility_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gem_mid_visibility_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gem_mid_decode_discriminator_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_observed_window_discriminator_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_observed_window_discriminator_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_prereq_ownership_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_prereq_ownership_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_clock_reset_readonly_baseline_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_clock_reset_write_restore_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_clock_reset_write_restore_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_clk_eth_ctrl_write_restore_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_phy_reset_preflight_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_phy_reset_preflight_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_phy_reset_write_restore_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_state_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_event_state_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_state_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_event_state_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_clear_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_event_clear_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_event_clear_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_gpio32_event_clear_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_phy_id_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_phy_id_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_after_mpe_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_phy_id_after_mpe_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_phy_id_after_mpe_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_phy_id_after_mpe_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_mpe_enable_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_mpe_enable_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_mpe_enable_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_mpe_enable_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_register_vector_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_register_vector_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_phy1_status_diagnostic_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_phy1_status_diagnostic_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_phy1_status_diagnostic_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_phy1_status_diagnostic_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_phy1_bmsr_double_sample_link_readiness_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_macb_nsr_link_readonly_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_macb_nsr_link_readonly_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_macb_nsr_link_readonly_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_macb_nsr_link_readonly_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_phy1_autoneg_restart_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_phy1_autoneg_restart_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_phy1_autoneg_restart_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_phy1_autoneg_restart_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_post_physical_link_status_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_post_physical_link_status_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_post_physical_link_status_no_mdio_macb_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_post_physical_link_status_no_mdio_macb_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_readonly_preflight_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_readonly_preflight_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_readonly_preflight_v2_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_readonly_preflight_v2_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_no_mdio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_no_mdio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_kernel_entry_serial_beacon"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_kernel_entry_serial_beacon();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_staging_sentinel_candidate"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_register_vector_staging_sentinel_candidate();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_ethernet_mdio_register_vector_staging_sentinel_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_ethernet_mdio_register_vector_staging_sentinel_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_pcie2_host_link_status_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_pcie2_host_link_status_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_endpoint_config_identity_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_endpoint_config_identity_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_bridge_config_preflight_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_bridge_config_preflight_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_bridge_setup_state_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_bridge_setup_state_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_observed_aperture_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_observed_aperture_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_observed_aperture_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_observed_gpio_status_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_observed_gpio_status_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio14_ownership_route_preflight_read();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio14_ownership_route_preflight_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio16_owned_event_discriminator();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::run_rp1_gpio16_owned_event_discriminator_no_mmio_control();
    }

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_boot_scenario = "rpi5_rp1_entry_control"
    ))]
    {
        let _ = dtb_pa;
        target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::RustEntry);
        target::rpi5::run_rp1_entry_control_diagnostic();
    }

    #[cfg(not(all(
        talos_target_rpi5_bcm2712,
        any(
            talos_boot_scenario = "rpi5_rp1_entry_control",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_read",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_read_delayed_marker",
            talos_boot_scenario = "rpi5_rp1_final_preload_marker_hold",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_read_hold_control",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_shaped_no_mmio_marker",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_result",
            talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio14_status_read",
            talos_boot_scenario = "rpi5_rp1_gpio14_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_interrupt_routing_msix_cfg_read",
            talos_boot_scenario = "rpi5_rp1_interrupt_routing_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read",
            talos_boot_scenario = "rpi5_rp1_gic_visible_route_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_read",
            talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_manager_status_read",
            talos_boot_scenario = "rpi5_rp1_clock_manager_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
            talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle",
            talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read",
            talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read",
            talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
            talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate",
            talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_candidate",
            talos_boot_scenario = "rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_no_mdio_control",
            talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read",
            talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read",
            talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read",
            talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
            talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
            talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
            talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
            talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control",
            talos_boot_scenario = "rpi5_rp1_handoff_reset",
            talos_boot_scenario = "rpi5_rp1_post_handoff_marker_reset",
            talos_boot_scenario = "rpi5_rp1_ethernet_kernel_entry_serial_beacon",
            talos_boot_scenario = "rpi5_rust_entry_uart10_marker_loop"
        )
    )))]
    {
        #[cfg(talos_target_rpi5_bcm2712)]
        target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::RustEntry);

        let boot_info = BootInfo::from_aarch64_x0(dtb_pa);

        #[cfg(talos_target_rpi5_bcm2712)]
        target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::BootInfoParsed);

        target::init(&boot_info);

        #[cfg(talos_target_rpi5_bcm2712)]
        target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::TargetInit);

        arch::aarch64::exceptions::init();

        #[cfg(talos_target_rpi5_bcm2712)]
        target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::ExceptionsReady);

        #[cfg(test)]
        {
            test_main();
            target::qemu::exit_success();
        }

        #[cfg(not(test))]
        kernel_main(&boot_info)
    }
}

#[cfg(not(test))]
fn kernel_main(boot_info: &BootInfo) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    boot::rpi5::kernel_main(boot_info);

    #[cfg(not(talos_target_rpi5_bcm2712))]
    {
        println!();
        println!(
            "Talos {} booting on {}",
            env!("CARGO_PKG_VERSION"),
            boot_info.target.name()
        );

        println!(
            "boot-info: dtb_pa={:#018x} core={} el={} target={}",
            boot_info.dtb_pa,
            boot_info.primary_core,
            boot_info.exception_level,
            boot_info.target.name()
        );
        let services = target::services(boot_info);
        println!(
            "target-services: uart={} timer={} irq={} dtb={:#018x?}",
            services.uart.name(),
            services.timer.name(),
            services.interrupt_controller.name(),
            services.device_tree.physical_address()
        );
        println!("mmio-regions: {}", services.mmio_map.regions().len());
        if boot_info.target == target::TargetKind::QemuVirt && boot_info.exception_level == 2 {
            #[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
            {
                if target::qemu_virt::run_smp_lock_contention_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
            {
                if target::qemu_virt::run_per_core_scheduler_ownership_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
            {
                if target::qemu_virt::run_cross_core_ipi_delivery_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
            {
                if target::qemu_virt::run_remote_wakeup_request_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
            {
                if target::qemu_virt::run_production_secondary_dispatch_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
            {
                if target::qemu_virt::run_shared_scheduler_metadata_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_shared_runqueue_migration")]
            {
                if target::qemu_virt::run_shared_runqueue_migration_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_load_balancing_smoke")]
            {
                if target::qemu_virt::run_load_balancing_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
            {
                if target::qemu_virt::run_secondary_scheduler_service_loop_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
            {
                if target::qemu_virt::run_multicore_preemption_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
            {
                if target::qemu_virt::run_production_timer_preemption_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_el0_trap_smoke")]
            {
                target::qemu_virt::run_el0_trap_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
            {
                target::qemu_virt::run_syscall_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
            {
                target::qemu_virt::run_pointer_copy_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_process_descriptor_stdio_smoke")]
            {
                target::qemu_virt::run_process_descriptor_stdio_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
            {
                target::qemu_virt::run_close_syscall_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
            {
                target::qemu_virt::run_dup_syscall_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
            {
                target::qemu_virt::run_read_stdin_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
            {
                if target::qemu_virt::run_readonly_initramfs_vfs_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_open_read_syscall_surface_smoke")]
            {
                if target::qemu_virt::run_open_read_syscall_surface_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
            {
                if target::qemu_virt::run_program_loader_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_program_loader_from_vfs_smoke")]
            {
                if target::qemu_virt::run_program_loader_from_vfs_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
            {
                if target::qemu_virt::run_process_install_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
            {
                if target::qemu_virt::run_process_address_space_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
            {
                if target::qemu_virt::run_process_page_table_materialization_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_initial_process_launch_smoke")]
            {
                if target::qemu_virt::run_initial_process_launch_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_initial_userspace_process_launch_smoke")]
            target::qemu_virt::run_initial_userspace_process_launch_smoke();

            #[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
            {
                if target::qemu_virt::run_initial_user_stack_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
            {
                if target::qemu_virt::run_live_address_space_activation_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
            {
                if target::qemu_virt::run_kernel_half_reachability_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke")]
            {
                if target::qemu_virt::run_kernel_half_descriptor_image_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
            {
                if target::qemu_virt::run_live_descriptor_image_installation_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_live_translation_register_activation_smoke")]
            {
                if target::qemu_virt::run_live_translation_register_activation_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(all(
                talos_boot_scenario = "qemu_descriptor_write_smoke",
                not(talos_boot_scenario = "qemu_process_descriptor_stdio_smoke"),
                not(talos_boot_scenario = "qemu_close_syscall_smoke"),
                not(talos_boot_scenario = "qemu_dup_syscall_smoke"),
                not(talos_boot_scenario = "qemu_read_stdin_smoke")
            ))]
            {
                target::qemu_virt::run_descriptor_write_smoke();
            }

            #[cfg(talos_boot_scenario = "qemu_secondary_core_workload")]
            {
                if target::qemu_virt::run_secondary_core_workload_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_diagnostic_command_channel")]
            {
                if target::qemu_virt::run_diagnostic_command_channel_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_local_serial_command_loop")]
            {
                if target::qemu_virt::run_local_serial_command_loop_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_polling_tty_rx")]
            {
                if target::qemu_virt::run_polling_tty_rx_diagnostic() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_boot_scenario = "qemu_timer_preemption")]
            {
                if target::qemu_virt::run_el2_timer_preemption_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(all(
                not(talos_boot_scenario = "qemu_timer_preemption"),
                talos_boot_scenario = "qemu_scheduler_yield"
            ))]
            {
                if target::qemu_virt::run_el2_scheduler_yield_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(all(
                not(talos_boot_scenario = "qemu_timer_preemption"),
                not(talos_boot_scenario = "qemu_scheduler_yield"),
                talos_boot_scenario = "qemu_context_switch"
            ))]
            {
                if target::qemu_virt::run_el2_context_switch_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(not(any(
                talos_boot_scenario = "qemu_diagnostic_command_channel",
                talos_boot_scenario = "qemu_local_serial_command_loop",
                talos_boot_scenario = "qemu_polling_tty_rx",
                talos_boot_scenario = "qemu_timer_preemption",
                talos_boot_scenario = "qemu_scheduler_yield",
                talos_boot_scenario = "qemu_context_switch",
                talos_boot_scenario = "qemu_secondary_core_workload",
                talos_boot_scenario = "qemu_smp_lock_contention",
                talos_boot_scenario = "qemu_per_core_scheduler_ownership",
                talos_boot_scenario = "qemu_cross_core_ipi_delivery",
                talos_boot_scenario = "qemu_remote_wakeup_request",
                talos_boot_scenario = "qemu_production_secondary_dispatch",
                talos_boot_scenario = "qemu_shared_scheduler_metadata",
                talos_boot_scenario = "qemu_shared_runqueue_migration",
                talos_boot_scenario = "qemu_load_balancing_smoke",
                talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
                talos_boot_scenario = "qemu_multicore_preemption_smoke",
                talos_boot_scenario = "qemu_production_timer_preemption_smoke",
                talos_boot_scenario = "qemu_el0_trap_smoke",
                talos_boot_scenario = "qemu_syscall_smoke",
                talos_boot_scenario = "qemu_pointer_copy_smoke",
                talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke",
                talos_boot_scenario = "qemu_open_read_syscall_surface_smoke",
                talos_boot_scenario = "qemu_program_loader_smoke",
                talos_boot_scenario = "qemu_program_loader_from_vfs_smoke",
                talos_boot_scenario = "qemu_process_install_smoke",
                talos_boot_scenario = "qemu_process_address_space_smoke",
                talos_boot_scenario = "qemu_process_page_table_materialization_smoke",
                talos_boot_scenario = "qemu_initial_process_launch_smoke",
                talos_boot_scenario = "qemu_initial_userspace_process_launch_smoke",
                talos_boot_scenario = "qemu_initial_user_stack_smoke",
                talos_boot_scenario = "qemu_live_address_space_activation_smoke",
                talos_boot_scenario = "qemu_kernel_half_reachability_smoke",
                talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke",
                talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke",
                talos_boot_scenario = "qemu_live_translation_register_activation_smoke",
                talos_boot_scenario = "qemu_descriptor_write_smoke"
            )))]
            if target::qemu_virt::run_el2_timer_irq_smoke() {
                target::qemu::exit_success();
            }
            #[cfg(not(any(
                talos_boot_scenario = "qemu_diagnostic_command_channel",
                talos_boot_scenario = "qemu_local_serial_command_loop",
                talos_boot_scenario = "qemu_timer_preemption",
                talos_boot_scenario = "qemu_scheduler_yield",
                talos_boot_scenario = "qemu_context_switch",
                talos_boot_scenario = "qemu_secondary_core_workload",
                talos_boot_scenario = "qemu_smp_lock_contention",
                talos_boot_scenario = "qemu_per_core_scheduler_ownership",
                talos_boot_scenario = "qemu_cross_core_ipi_delivery",
                talos_boot_scenario = "qemu_remote_wakeup_request",
                talos_boot_scenario = "qemu_production_secondary_dispatch",
                talos_boot_scenario = "qemu_shared_scheduler_metadata",
                talos_boot_scenario = "qemu_shared_runqueue_migration",
                talos_boot_scenario = "qemu_load_balancing_smoke",
                talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
                talos_boot_scenario = "qemu_multicore_preemption_smoke",
                talos_boot_scenario = "qemu_production_timer_preemption_smoke",
                talos_boot_scenario = "qemu_el0_trap_smoke",
                talos_boot_scenario = "qemu_syscall_smoke",
                talos_boot_scenario = "qemu_pointer_copy_smoke",
                talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke",
                talos_boot_scenario = "qemu_open_read_syscall_surface_smoke",
                talos_boot_scenario = "qemu_program_loader_smoke",
                talos_boot_scenario = "qemu_program_loader_from_vfs_smoke",
                talos_boot_scenario = "qemu_process_install_smoke",
                talos_boot_scenario = "qemu_process_address_space_smoke",
                talos_boot_scenario = "qemu_process_page_table_materialization_smoke",
                talos_boot_scenario = "qemu_initial_process_launch_smoke",
                talos_boot_scenario = "qemu_initial_userspace_process_launch_smoke",
                talos_boot_scenario = "qemu_initial_user_stack_smoke",
                talos_boot_scenario = "qemu_live_address_space_activation_smoke",
                talos_boot_scenario = "qemu_kernel_half_reachability_smoke",
                talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke",
                talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke",
                talos_boot_scenario = "qemu_live_translation_register_activation_smoke",
                talos_boot_scenario = "qemu_descriptor_write_smoke"
            )))]
            target::qemu::exit_failure();
        }
        println!("talos: hello from {}", boot_info.target.name());
        println!("talos: qemu smoke PASS");
        match boot_info.target {
            target::TargetKind::QemuVirt => target::qemu::exit_success(),
            target::TargetKind::Rpi5Bcm2712 => arch::aarch64::halt(),
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    {
        target::console::write_static("\nTALOS: panic handler entered\n");

        if PANIC_IN_PROGRESS.enter() {
            target::console::write_static("\nTALOS: nested panic\n");
            target::rpi5::wait_uart10_empty_early_phase();
            arch::aarch64::halt()
        }

        println!("talos panic: {}", info);
        target::rpi5::wait_uart10_empty_early_phase();
        arch::aarch64::halt()
    }

    #[cfg(not(talos_target_rpi5_bcm2712))]
    {
        println!();
        println!("talos panic: {}", info);

        #[cfg(test)]
        target::qemu::exit_failure();

        #[cfg(not(test))]
        arch::aarch64::halt()
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        target::console::write_static(core::any::type_name::<T>());
        target::console::write_static(" ... ");
        self();
        println!("ok");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!();
    target::console::write_static("running ");
    target::console::write_dec_usize(tests.len());
    target::console::write_static(" talos no_std tests\n");
    for test in tests {
        test.run();
    }
    target::console::write_static("test result: ok. ");
    target::console::write_dec_usize(tests.len());
    target::console::write_static(" passed\n");
}

#[cfg(test)]
#[test_case]
fn smoke_test_runs() {
    assert_eq!(2 + 2, 4);
}
