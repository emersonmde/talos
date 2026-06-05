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
    println!("cargo:rerun-if-changed={GENERATED_ROOT_MANIFEST}");

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

fn generate_initramfs_manifest(out_dir: &PathBuf) {
    let manifest = fs::read_to_string(GENERATED_ROOT_MANIFEST)
        .unwrap_or_else(|error| panic!("failed to read {GENERATED_ROOT_MANIFEST}: {error}"));
    let entry = parse_generated_manifest(&manifest);
    let digest = generated_root_digest(entry.path.as_bytes(), entry.contents.as_bytes());
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
        entry.path.as_bytes()
    )
    .expect("write generated root path");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_DIR_NAME: &[u8] = &{:?};",
        entry.directory.as_bytes()
    )
    .expect("write generated root dir");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_FILE_NAME: &[u8] = &{:?};",
        entry.file_name.as_bytes()
    )
    .expect("write generated root file name");
    writeln!(
        file,
        "pub(crate) const GENERATED_ROOT_FILE_BYTES: &[u8] = &{:?};",
        entry.contents.as_bytes()
    )
    .expect("write generated root bytes");
}

struct GeneratedManifestEntry {
    path: String,
    directory: String,
    file_name: String,
    contents: String,
}

fn parse_generated_manifest(manifest: &str) -> GeneratedManifestEntry {
    let mut path = None;
    let mut contents = None;

    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = trimmed
            .split_once('=')
            .unwrap_or_else(|| panic!("invalid generated manifest line: {trimmed}"));
        match key {
            "path" => path = Some(value.to_string()),
            "contents" => contents = Some(unescape_manifest_value(value)),
            _ => panic!("unsupported generated manifest key: {key}"),
        }
    }

    let path = path.expect("generated manifest path is required");
    let contents = contents.expect("generated manifest contents are required");
    validate_generated_manifest_entry(&path, &contents);
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

    GeneratedManifestEntry {
        path,
        directory,
        file_name,
        contents,
    }
}

fn validate_generated_manifest_entry(path: &str, contents: &str) {
    if !path.starts_with('/') || path == "/" {
        panic!("generated manifest path must be absolute and non-root: {path}");
    }
    if path.as_bytes().contains(&0) || contents.as_bytes().contains(&0) {
        panic!("generated manifest path and contents must not contain NUL bytes");
    }
    if path.contains("//") || path.split('/').any(|part| part == "." || part == "..") {
        panic!("generated manifest path must be normalized inside root: {path}");
    }
    if contents.len() > 4096 {
        panic!("generated manifest file exceeds first-slice 4096-byte limit");
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

fn generated_root_digest(path: &[u8], contents: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

    let mut hash = FNV_OFFSET;
    for byte in GENERATED_ROOT_IDENTITY.as_bytes() {
        hash = fnv_step(hash, *byte);
    }
    hash = fnv_step(hash, 0);
    for byte in path {
        hash = fnv_step(hash, *byte);
    }
    hash = fnv_step(hash, 0);
    for byte in contents {
        hash = fnv_step(hash, *byte);
    }
    hash
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
