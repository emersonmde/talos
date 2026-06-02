use std::env;
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
        value: "qemu_program_loader_smoke",
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

    if !target.starts_with("aarch64") {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));
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
