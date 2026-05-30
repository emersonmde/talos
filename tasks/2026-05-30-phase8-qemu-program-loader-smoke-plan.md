# Phase 8 QEMU Program Loader Smoke Plan Task

Task: phase8-qemu-program-loader-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only QEMU/substitute program-loader smoke plan after the
accepted program-loader format contract. The task defines fixture identity,
success image-plan observations, deterministic negative cases, exact
PASS/classification lines, retained evidence path, conditional regression
gates, source owners, deferred surfaces, and the next bounded implementation
recommendation.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, loader core, process creation,
exec/spawn/wait, argv/envp stack implementation, shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation/source review: used the accepted program-loader format
  contract, program-loader source inventory, read-only initramfs/VFS smoke
  plan pattern, roadmap, and ADR index.
- documentation diff: added
  docs/src/project/phase8-qemu-program-loader-smoke-plan.md, linked it from
  docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- recommendation: next bounded implementation task should be
  phase8-program-loader-core-20260530 only after supervisor planning queues it
  with explicit scope and gates.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the Milestone 8.3 QEMU/substitute program-loader smoke plan. The
accepted smoke plan uses fixture identity
phase8-program-loader-elf64-aarch64-v1, retained evidence path
tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log,
classification qemu-program-loader-smoke-complete, and PASS vocabulary. It
proves image-plan validation only; loader implementation, process address-space
installation, lower-EL launch, argv/envp stack construction, process creation,
exec/spawn/wait, shell behavior, descriptor-backed filesystem syscalls, Pi 5
hardware proof, writable filesystems, persistent storage, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
blocked.
