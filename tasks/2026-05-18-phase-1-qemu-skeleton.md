# Phase 1 QEMU Skeleton

## Task

- Title: Phase 1 Rust kernel scaffold and QEMU virt smoke test
- Owner: Seldon / background implementation agent
- Date: 2026-05-18
- Milestone: Phase 1.1 and Phase 1.2
- Scope: Initial Rust no_std kernel package, build tooling, QEMU virt boot path, and documentation updates.

## Goal

Create the first runnable Talos kernel scaffold:

- Rust no_std package.
- Pinned toolchain.
- Custom AArch64 target.
- Linker script and boot assembly.
- Panic path.
- QEMU virt serial output.
- Basic runner/test path if feasible in the first pass.

The first target is QEMU virt, not Raspberry Pi 5 emulation. Pi 5 hardware validation waits for the serial cable and lab serial API.

## Constraints

- Do not hardcode Pi 5 peripheral offsets as the long-term hardware strategy.
- Preserve the architecture shape needed to consume the Pi 5 firmware-provided DTB from x0 later.
- Keep target-specific code isolated so QEMU virt and Raspberry Pi 5 can diverge cleanly.
- Use Daedalus as a reference for working Rust/kernel patterns, but do not copy stale Pi 4 hardware assumptions.
- Do not use the lab mutating endpoints for this task unless explicitly needed; Phase 1 is emulator-first.

## Expected Outputs

- Buildable kernel scaffold.
- QEMU smoke test or documented blocker.
- Updated README/docs with exact commands.
- Task notes describing what changed and what remains for Pi 5 first-light.

## Evidence To Capture

- Build command and output summary.
- QEMU command and serial output summary.
- Any assumptions about target JSON, linker layout, or boot ABI.

## Work Performed

- Added a Rust 2024 no_std kernel package with pinned nightly toolchain.
- Added custom AArch64 target JSON files for talos-aarch64-virt and the later
  talos-rpi5-bcm2712 path.
- Added Cargo build-std configuration, an AArch64 linker script, and a build
  script that assembles the early boot stub.
- Added minimal AArch64 assembly that preserves x0, clears BSS, sets the boot
  stack, and enters Rust.
- Added early BootInfo with preserved dtb_pa, exception level, core ID, and
  target kind.
- Added QEMU virt target code with PL011 serial output and semihosting
  pass/fail exit.
- Added a custom no_std test runner and one smoke test.
- Added scripts/qemu-runner.sh for cargo run / cargo test and
  scripts/qemu-smoke.sh for a clear boot-log smoke gate.
- Updated README and architecture docs with exact commands and layout notes.

## Evidence

Verification completed in this task run:

~~~bash
cargo fmt --check
cargo -Zjson-target-spec build
./scripts/qemu-smoke.sh
cargo -Zjson-target-spec test
mdbook build
~~~

Command results:

- cargo fmt --check: passed after formatting the new Rust files.
- cargo -Zjson-target-spec build: passed; produced
  target/aarch64-talos-virt/debug/talos and
  target/talos-aarch64-virt.map.
- ./scripts/qemu-smoke.sh: passed on system QEMU 7.2.22.
- cargo -Zjson-target-spec test: passed; QEMU ran 1 no_std smoke test.
- mdbook build: passed and rebuilt book/.

QEMU serial evidence:

~~~text
Talos 0.1.0 booting on talos-aarch64-virt
boot-info: dtb_pa=0x0000000048000000 core=0 el=1 target=talos-aarch64-virt
talos: hello from qemu virt
talos: qemu smoke PASS
~~~

Target and linker assumptions:

- Current nightly Cargo requires -Zjson-target-spec for checked-in JSON target
  specs.
- AArch64 uses LLVM code-model = "large"; the older kernel spelling is not
  accepted by this toolchain.
- QEMU virt boots the objcopied arm64 Image, not the ELF directly. The boot
  section includes the 64-byte arm64 Image header.
- The QEMU image is linked at 0x4020_0000 so it does not overlap QEMU's
  generated low-memory DTB handoff area; x0 is preserved as BootInfo::dtb_pa.
- FP/SIMD is enabled before Rust because debug core formatting and
  precondition paths can emit SIMD instructions.

## Result

Phase 1.1/1.2 scaffold is complete for QEMU virt. The repository now has a
buildable no_std Rust kernel, target split, linker layout, boot assembly, serial
hello path, semihosting QEMU exit, and a working custom no_std test runner.

## Follow-up

- Replace the Pi 5 target stub with the real firmware handoff path once serial
  hardware is available through the lab controller.
- Add exception vectors before MMU and deliberate-fault diagnostics.
- Add an allocator using the reserved linker heap once Phase 3 begins.
