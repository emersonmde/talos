# Phase 6 Secondary-Core Bring-Up Source Inventory

Task: `phase6-secondary-core-bringup-source-inventory-and-contract-20260524`

Status: accepted and committed as the Phase 6.1 planning contract.

## Goal

Create a source-backed secondary-core bring-up inventory and contract before
any SMP implementation, hardware run, scheduler migration, or lock work starts.

## Accepted Contract

- PSCI is the default secondary-core bring-up mechanism.
- Pi 5 uses the SMC conduit from the Raspberry Pi Linux `/psci` node.
- The first Pi 5 CPU identity mapping should start from MPIDR affinity values
  matching the Linux DTS CPU node `reg` values: `0x000`, `0x100`, `0x200`,
  and `0x300`.
- Each secondary core must prove core identity, exclusive stack ownership,
  per-core state registration, and controlled handoff before scheduler work.
- Spin-table and mailbox paths remain fallback research, not the default path.

## Evidence

- Source inventory and contract:
  `docs/src/project/phase6-secondary-core-bringup-source-inventory.md`.
- Repository source inspection:
  `docs/src/roadmap.md`, `docs/src/project/reference-notes.md`,
  `src/arch/aarch64/boot.S`, `src/boot/mod.rs`, `src/arch/aarch64/mod.rs`,
  `src/target/mod.rs`, `src/target/qemu_virt.rs`, and `src/target/rpi5.rs`.
- Primary/advisory source inspection: Raspberry Pi Linux `bcm2712.dtsi` from
  `rpi-6.12.y` and QEMU 9.2.0 generated `virt` DTB strings for
  `virt,gic-version=2,virtualization=on -cpu cortex-a76 -smp 4`.
- QEMU DTB artifact:
  `target/tmp/phase6-secondary-core-source-inventory/qemu-virt-smp4.dtb`.

## Validation

- static inspection: `git status --short` was clean before documentation edits.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.
- Rust fmt/tests were not required because no Rust files changed.

## Next Task

`phase6-qemu-secondary-core-bringup-discriminator-20260524` is the next
explicit bounded task. It must remain QEMU/substitute and source-backed before
per-core stacks/state or Pi 5 PSCI hardware proof work starts.
