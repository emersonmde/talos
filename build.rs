use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/arch/aarch64/boot.S");
    println!("cargo:rerun-if-changed=linker.ld");

    let target = env::var("TARGET").expect("TARGET is set by Cargo");
    if !target.starts_with("aarch64") {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let boot_obj = out_dir.join("boot.o");

    let status = Command::new("clang")
        .args([
            "-target",
            "aarch64-none-elf",
            "-ffreestanding",
            "-fno-stack-protector",
            "-c",
            "src/arch/aarch64/boot.S",
            "-o",
        ])
        .arg(&boot_obj)
        .status()
        .expect("failed to run clang for AArch64 boot assembly");

    if !status.success() {
        panic!("AArch64 boot assembly failed");
    }

    println!("cargo:rustc-link-arg={}", boot_obj.display());
}
