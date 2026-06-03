# Talos Entry, Boot, and Target Review

Task: talos-review-entry-boot-targets-20260603
Status: accepted

## Scope

Reviewed entry, boot, target routing, Raspberry Pi 5/QEMU proof harness cfg
boundaries, and target-specific build scenario behavior.

## Findings

- Fixed: Pi 5 local-command proof helper cfg was narrower than the shared
  command-loop runner. The base rpi5_local_serial_command_loop build and
  derived scenarios such as rpi5_local_help_command compiled the runner but not
  every replay helper referenced by that runner. Broadened those helpers to the
  shared command-loop scenario so the base and derived Pi 5 local proof builds
  compile consistently.
- Fixed: src/main.rs top-level dead-code/unused cfg lists named only a subset
  of the Pi 5 local command-loop proof scenarios. Expanded the list to match
  the scenario names in build.rs, reducing stale proof-routing clutter and
  making the target-specific boundary explicit at the entry module.
- Not an issue: src/boot/rpi5.rs routes local command-loop proofs through
  rpi5_local_serial_command_loop. build.rs explicitly implies that base
  scenario for the narrower local command proof scenarios, so duplicate
  per-feature boot dispatch blocks are unnecessary.
- Deferred: the non-RPi5 kernel_main path still owns a large QEMU scenario
  dispatcher in src/main.rs. It is behavior-sensitive and did not produce a
  failing gate in this review. A later full-system or QEMU-routing refactor
  should move it behind a boot/target-owned dispatcher without changing smoke
  behavior.

## Changes

- src/target/rpi5.rs now compiles shared local-command replay helpers under
  rpi5_local_serial_command_loop, matching the build-time implication model.
- src/main.rs now lists the full currently scheduled Pi 5 local command-loop
  scenario family in the root cfg allowances.

No runtime feature behavior, hardware boot behavior, filesystem behavior,
userspace behavior, networking, RP1/PCIe, UART interrupt ownership, DMA, or
cache policy changed.

## Validation

- Static inspection: reviewed src/main.rs, src/boot/*, src/target/*,
  src/diagnostics/*, and build.rs scenario routing with rg/sed.
- RPi5 target checks: cargo -Zjson-target-spec check --target
  targets/aarch64-talos-rpi5-bcm2712.json --quiet passed for
  rpi5_local_serial_command_loop, rpi5_local_help_command,
  rpi5_local_ls_root, rpi5_local_ls_bin, rpi5_local_cat_banner,
  rpi5_local_cat_cwd, rpi5_local_cd_fixed_dirs, rpi5_local_ls_cwd,
  rpi5_local_pwd_command, rpi5_local_echo_command,
  rpi5_local_literal_echo, rpi5_local_line_editing,
  rpi5_local_line_cancel, and rpi5_local_line_kill.
- QEMU target checks: cargo -Zjson-target-spec check --target
  targets/aarch64-talos-virt.json --quiet passed for
  qemu_local_serial_command_loop, qemu_local_cat_cwd, and
  qemu_descriptor_write_smoke.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- static diff hygiene: git diff --check passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- hardwareTestLock remained unlocked/restored and unused; no hardware claim was
  made.

## Remaining Risks

The QEMU dispatcher remains large in src/main.rs; this review records it as a
deferred maintainability risk rather than moving hundreds of scenario lines in
the same patch as the Pi 5 cfg correctness fix.
