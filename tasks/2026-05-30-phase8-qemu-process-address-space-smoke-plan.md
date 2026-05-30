# Phase 8 QEMU Process Address-Space Smoke Plan Task

Task: phase8-qemu-process-address-space-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only QEMU/substitute smoke plan after the accepted process
address-space contract. The task defines fixture identity, target-independent
ProcessAddressSpace success observations, deterministic no-partial-install and
no-leak rejection cases, teardown observations, exact PASS/classification
lines, retained evidence path, conditional regression gates, source owners,
deferred surfaces, and the next bounded implementation recommendation.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, process-address-space core
implementation, hardware page-table mutation, TTBR/TCR switching, lower-EL
launch, argv/envp construction, exec/spawn/wait, shell, descriptor-backed
filesystem syscalls, writable filesystem, persistent storage, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation/source review: used the accepted process address-space
  contract, process address-space source inventory, process-install smoke-plan
  pattern, roadmap, and ADR index.
- documentation diff: added
  docs/src/project/phase8-qemu-process-address-space-smoke-plan.md, linked it
  from docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- output/evidence specification: retained path
  tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log;
  classification qemu-process-address-space-smoke-complete; PASS line
  qemu-process-address-space-smoke: PASS.
- blocked surfaces: implementation, hardware, AArch64 descriptor
  construction, TTBR/TCR switching, lower-EL launch, argv/envp, scheduler
  handoff, descriptor-backed filesystem syscalls, shell, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked.
- recommendation: next bounded implementation task should be
  phase8-process-address-space-core-20260530 only because supervisor planning
  has already queued it with explicit scope and gates.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the Milestone 8.3 QEMU/substitute process address-space smoke
plan. The accepted smoke plan uses loader fixture identity
phase8-program-loader-elf64-aarch64-v1, install boundary identity
phase8-process-install-plan-v1, address-space boundary identity
phase8-process-address-space-model-v1, retained evidence path
tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log,
classification qemu-process-address-space-smoke-complete, and PASS
vocabulary. It proves target-independent ProcessAddressSpace model
installation only; hardware page tables, TTBR/TCR switching, lower-EL launch,
argv/envp stack construction, process creation, exec/spawn/wait, shell
behavior, descriptor-backed filesystem syscalls, Pi 5 hardware proof,
writable filesystems, persistent storage, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache-driver policy remain blocked.
