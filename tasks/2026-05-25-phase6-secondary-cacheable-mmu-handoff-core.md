# Phase 6 Secondary Cacheable MMU Handoff Core

Task: phase6-secondary-cacheable-mmu-handoff-core-20260524

## Summary

This task implements the minimal secondary-core cacheable EL2 stage-1 handoff
needed before the Pi 5 SMP lock proof can be rerun as a valid hardware proof.
It does not run hardware, broaden scheduler ownership, change `SpinLock<T>`,
or add lower-EL, filesystem, network, shell, RP1/PCIe, or DMA policy.

The boot CPU now publishes its active EL2 stage-1/cache regime before issuing
Pi 5 PSCI `CPU_ON` calls for the lock proof. Each secondary loads that
published plan, installs MAIR_EL2/TCR_EL2/TTBR0_EL2, invalidates EL2
translations and caches with barriers, enables SCTLR_EL2.M/I/C, then publishes
the resulting SCTLR_EL2 through the existing lock diagnostic channel before
first generic lock access.

## Implementation Notes

- `src/arch/aarch64/mod.rs` adds `El2Stage1CacheRegime`,
  `current_el2_stage1_cache_regime()`, and
  `install_el2_stage1_cache_regime()` for the register-programming boundary.
- `src/target/rpi5.rs` stores the boot CPU handoff plan in cache-maintained
  atomics scoped to `talos_rpi5_smp_lock_cache_coherence_proof`.
- Pi 5 secondary lock-proof entry parks before generic `SpinLock<T>` access
  if the handoff plan is unavailable or cacheable-MMU enablement fails.
- Existing wait/report diagnostics keep naming boot CPU SCTLR/cacheable state
  and now report the secondary SCTLR after the handoff attempt.
- The generic lock remains free of hidden cache maintenance or MMU setup.

## Validation

- static inspection: `git status --short` before edits showed existing
  unaccepted Pi 5 lock-proof diagnostic changes from the blocked proof task;
  this task preserved and built on them without reverting evidence.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 102 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed with
  classification `qemu-smp-lock-contention-complete`.
- image inspection: `scripts/rpi5-smp-lock-cache-coherence-image.sh`
  generated
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-smp-lock-cache-coherence.img`,
  size 96,792 bytes, SHA256
  `acc334beb5bc82555d6d4c3309d3e24b0b669593768cb9d01e479bc40e350e40`.
- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.

## Evidence

- Source boundaries:
  `src/arch/aarch64/mod.rs`, `src/target/rpi5.rs`, and existing diagnostic
  scaffolding from `phase6-pi5-smp-lock-cache-coherence-proof-20260524`.
- Blocked proof linkage:
  `tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/summary.md`
  classified the previous physical proof setup as
  `pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime`.
- Architecture update: `docs/src/architecture/memory.md`.
- Decision log: `docs/src/decisions/README.md`.

## Acceptance

Accepted after local static, unit, QEMU/substitute, image-generation, and diff
gates passed. The follow-up Pi 5 proof task must provide the hardware evidence;
this task makes no hardware claim.
