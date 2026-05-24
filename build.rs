use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/arch/aarch64/boot.S");
    println!("cargo:rerun-if-changed=src/arch/aarch64/vectors.S");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=linker-rpi5.ld");
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
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_PAGE_FRAME_REUSE_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_HEAP_EXPANSION_POLICY_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_QEMU_CONTEXT_SWITCH_SMOKE");
    println!("cargo:rerun-if-env-changed=TALOS_QEMU_SCHEDULER_YIELD_SMOKE");
    println!("cargo:rerun-if-env-changed=TALOS_QEMU_TIMER_PREEMPTION_SMOKE");
    println!("cargo:rerun-if-env-changed=TALOS_QEMU_POLLING_TTY_RX_DIAGNOSTIC");

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
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_page_frame_reuse_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_heap_expansion_policy_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_timer_irq_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_timer_preemption_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_qemu_context_switch_smoke)");
    println!("cargo:rustc-check-cfg=cfg(talos_qemu_scheduler_yield_smoke)");
    println!("cargo:rustc-check-cfg=cfg(talos_qemu_timer_preemption_smoke)");
    println!("cargo:rustc-check-cfg=cfg(talos_qemu_polling_tty_rx_diagnostic)");
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
    if env::var_os("TALOS_RPI5_PAGE_FRAME_REUSE_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_page_frame_reuse_diagnostic");
    }
    if env::var_os("TALOS_RPI5_HEAP_EXPANSION_POLICY_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_heap_expansion_policy_diagnostic");
    }
    if env::var_os("TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_timer_irq_diagnostic");
    }
    if env::var_os("TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_timer_preemption_diagnostic");
    }
    if env::var_os("TALOS_QEMU_CONTEXT_SWITCH_SMOKE").is_some() {
        println!("cargo:rustc-cfg=talos_qemu_context_switch_smoke");
    }
    if env::var_os("TALOS_QEMU_SCHEDULER_YIELD_SMOKE").is_some() {
        println!("cargo:rustc-cfg=talos_qemu_scheduler_yield_smoke");
    }
    if env::var_os("TALOS_QEMU_TIMER_PREEMPTION_SMOKE").is_some() {
        println!("cargo:rustc-cfg=talos_qemu_timer_preemption_smoke");
    }
    if env::var_os("TALOS_QEMU_POLLING_TTY_RX_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_qemu_polling_tty_rx_diagnostic");
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
