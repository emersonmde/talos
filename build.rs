use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/arch/aarch64/boot.S");
    println!("cargo:rerun-if-changed=src/arch/aarch64/vectors.S");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=linker-rpi5.ld");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_CARGO_ASM_UART_PROOF");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TRANSITION_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_NORMAL_EXCEPTION_REPORT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_UNDEFINED_INSTRUCTION_REPORT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_DATA_ABORT_REPORT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TRANSLATION_FAULT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_CURRENT_SP0_SYNC_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PANIC_REPORT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FULL_PANIC_INFO_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_NESTED_PANIC_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ALLOC_OOM_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_REALLOC_GROWTH_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_VEC_GROWTH_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_STRING_GROWTH_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ALLOC_FORMAT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FALLTHROUGH_RUST_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_POST_STACK_NOP_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_POST_STACK_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ASM_INDIRECT_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ASM_TEXT_INDIRECT_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ASM_TEXT_DIRECT_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ASM_TEXT_JC_INDIRECT_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BTI_EXCEPTION_CLASSIFIER_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_DIRECT_EXCEPTION_CONTROL_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BRK_ERET_RESUME_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BRK_ELR_WRITE_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BRK_SPSR_ERET_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BRK_SPSR_HANDLER_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_RUNTIME_UART_PROBE_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_HANDOFF_UART_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ENTRY_TALOS_LINE_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_ENTRY_TALOS_LINE_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FRESH_ENTRY_CONTINUE_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FRESH_ENTRY_LABEL");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_UART_CANDIDATE_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_RUST_UART10_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_BOUNDARY_ENTRY_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC");

    let target = env::var("TARGET").expect("TARGET is set by Cargo");
    println!("cargo:rustc-check-cfg=cfg(talos_target_qemu_virt)");
    println!("cargo:rustc-check-cfg=cfg(talos_target_rpi5_bcm2712)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_exception_report_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_normal_exception_report_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_undefined_instruction_report_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_data_abort_report_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_translation_fault_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_current_sp0_sync_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_exception_return_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_panic_report_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_full_panic_info_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_nested_panic_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_alloc_oom_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_realloc_growth_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_vec_growth_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_string_growth_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_alloc_format_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_runtime_uart_probe_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_handoff_uart_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_rust_uart10_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_boundary_entry_reset_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_phase_ladder_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_phase_p0_reset_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_phase_p1_reset_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_phase_p1_short_reset_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_phase_p2_reset_diagnostic)");
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

    assemble_aarch64("src/arch/aarch64/boot.S", &boot_obj, &target);
    assemble_aarch64("src/arch/aarch64/vectors.S", &vectors_obj, &target);

    println!("cargo:rustc-link-arg={}", boot_obj.display());
    println!("cargo:rustc-link-arg={}", vectors_obj.display());
}

fn assemble_aarch64(source: &str, output: &PathBuf, target: &str) {
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
    if env::var_os("TALOS_RPI5_CARGO_ASM_UART_PROOF").is_some() {
        command.arg("-DTALOS_RPI5_CARGO_ASM_UART_PROOF");
    }
    if env::var_os("TALOS_RPI5_TRANSITION_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_TRANSITION_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_TEXT_SECTION_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_FALLTHROUGH_RUST_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_FALLTHROUGH_RUST_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_POST_STACK_NOP_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_POST_STACK_NOP_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_POST_STACK_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_POST_STACK_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ASM_INDIRECT_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ASM_INDIRECT_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ASM_TEXT_INDIRECT_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ASM_TEXT_INDIRECT_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ASM_TEXT_DIRECT_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ASM_TEXT_DIRECT_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ASM_TEXT_JC_INDIRECT_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ASM_TEXT_JC_INDIRECT_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_BTI_EXCEPTION_CLASSIFIER_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_BTI_EXCEPTION_CLASSIFIER_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_DIRECT_EXCEPTION_CONTROL_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_DIRECT_EXCEPTION_CONTROL_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_BRK_ERET_RESUME_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_BRK_ERET_RESUME_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_BRK_ELR_WRITE_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_BRK_ELR_WRITE_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_BRK_SPSR_ERET_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_BRK_SPSR_ERET_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_BRK_SPSR_HANDLER_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_BRK_SPSR_HANDLER_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_exception_report_diagnostic");
    }
    if env::var_os("TALOS_RPI5_NORMAL_EXCEPTION_REPORT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_normal_exception_report_diagnostic");
    }
    if env::var_os("TALOS_RPI5_UNDEFINED_INSTRUCTION_REPORT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_undefined_instruction_report_diagnostic");
    }
    if env::var_os("TALOS_RPI5_DATA_ABORT_REPORT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_data_abort_report_diagnostic");
    }
    if env::var_os("TALOS_RPI5_TRANSLATION_FAULT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_translation_fault_diagnostic");
    }
    if env::var_os("TALOS_RPI5_CURRENT_SP0_SYNC_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_current_sp0_sync_diagnostic");
    }
    if env::var_os("TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_exception_return_diagnostic");
    }
    if env::var_os("TALOS_RPI5_PANIC_REPORT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_panic_report_diagnostic");
    }
    if env::var_os("TALOS_RPI5_FULL_PANIC_INFO_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_full_panic_info_diagnostic");
    }
    if env::var_os("TALOS_RPI5_NESTED_PANIC_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_nested_panic_diagnostic");
    }
    if env::var_os("TALOS_RPI5_ALLOC_OOM_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_alloc_oom_diagnostic");
    }
    if env::var_os("TALOS_RPI5_REALLOC_GROWTH_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_realloc_growth_diagnostic");
    }
    if env::var_os("TALOS_RPI5_VEC_GROWTH_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_vec_growth_diagnostic");
    }
    if env::var_os("TALOS_RPI5_STRING_GROWTH_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_string_growth_diagnostic");
    }
    if env::var_os("TALOS_RPI5_ALLOC_FORMAT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_alloc_format_diagnostic");
    }
    if env::var_os("TALOS_RPI5_RUNTIME_UART_PROBE_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_runtime_uart_probe_diagnostic");
    }
    if env::var_os("TALOS_RPI5_HANDOFF_UART_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_HANDOFF_UART_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_handoff_uart_diagnostic");
    }
    if env::var_os("TALOS_RPI5_RUST_UART10_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_RUST_UART10_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_rust_uart10_diagnostic");
    }
    if env::var_os("TALOS_RPI5_BOUNDARY_ENTRY_RESET_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_boundary_entry_reset_diagnostic");
    }
    if env::var_os("TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_LADDER_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_phase_ladder_diagnostic");
    }
    if env::var_os("TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_phase_p0_reset_diagnostic");
    }
    if env::var_os("TALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_phase_p1_reset_diagnostic");
    }
    if env::var_os("TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_phase_p1_short_reset_diagnostic");
    }
    if env::var_os("TALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC");
        println!("cargo:rustc-cfg=talos_rpi5_phase_p2_reset_diagnostic");
    }
    if env::var_os("TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ENTRY_TALOS_LINE_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ENTRY_TALOS_LINE_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_ENTRY_TALOS_LINE_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_ENTRY_TALOS_LINE_RESET_DIAGNOSTIC");
    }
    if env::var_os("TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC");
        pass_fresh_entry_label(&mut command);
    }
    if env::var_os("TALOS_RPI5_FRESH_ENTRY_CONTINUE_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_FRESH_ENTRY_CONTINUE_DIAGNOSTIC");
        pass_fresh_entry_label(&mut command);
    }
    if env::var_os("TALOS_RPI5_UART_CANDIDATE_DIAGNOSTIC").is_some() {
        command.arg("-DTALOS_RPI5_UART_CANDIDATE_DIAGNOSTIC");
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

fn pass_fresh_entry_label(command: &mut Command) {
    if let Some(label) = env::var_os("TALOS_RPI5_FRESH_ENTRY_LABEL") {
        let label = label
            .into_string()
            .expect("TALOS_RPI5_FRESH_ENTRY_LABEL must be UTF-8");
        if !label
            .bytes()
            .all(|b| b.is_ascii_graphic() || b == b' ' || b == b'\r' || b == b'\n')
        {
            panic!("TALOS_RPI5_FRESH_ENTRY_LABEL must be printable ASCII");
        }
        command.arg(format!("-DTALOS_RPI5_FRESH_ENTRY_LABEL={:?}", label));
    }
}
