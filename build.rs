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
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_RUST_ENTRY_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_MINIMAL_FORMAT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_DYNAMIC_FORMAT_FALLBACK_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FMT_SINK_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FMT_STATIC_SINK_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FMT_SINK_DIRECT_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_FALLTHROUGH_RUST_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_POST_STACK_NOP_DIAGNOSTIC");
    println!("cargo:rerun-if-env-changed=TALOS_RPI5_POST_STACK_RESET_DIAGNOSTIC");

    let target = env::var("TARGET").expect("TARGET is set by Cargo");
    println!("cargo:rustc-check-cfg=cfg(talos_target_qemu_virt)");
    println!("cargo:rustc-check-cfg=cfg(talos_target_rpi5_bcm2712)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_rust_entry_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_minimal_format_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_exception_report_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_dynamic_format_fallback_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_fmt_sink_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_fmt_static_sink_diagnostic)");
    println!("cargo:rustc-check-cfg=cfg(talos_rpi5_fmt_sink_direct_diagnostic)");
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
    if env::var_os("TALOS_RPI5_RUST_ENTRY_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_rust_entry_diagnostic");
    }
    if env::var_os("TALOS_RPI5_MINIMAL_FORMAT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_minimal_format_diagnostic");
    }
    if env::var_os("TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_exception_report_diagnostic");
    }
    if env::var_os("TALOS_RPI5_DYNAMIC_FORMAT_FALLBACK_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_dynamic_format_fallback_diagnostic");
    }
    if env::var_os("TALOS_RPI5_FMT_SINK_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_fmt_sink_diagnostic");
    }
    if env::var_os("TALOS_RPI5_FMT_STATIC_SINK_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_fmt_static_sink_diagnostic");
    }
    if env::var_os("TALOS_RPI5_FMT_SINK_DIRECT_DIAGNOSTIC").is_some() {
        println!("cargo:rustc-cfg=talos_rpi5_fmt_sink_direct_diagnostic");
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
