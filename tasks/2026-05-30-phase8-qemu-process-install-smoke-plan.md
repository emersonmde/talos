# Phase 8 QEMU Process Install Smoke Plan Task

Task: phase8-qemu-process-install-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only QEMU/substitute smoke plan after the accepted
process-install contract. The task defines fixture identity, metadata-only
ProcessImageInstallPlan success observations, deterministic no-partial-install
rejection cases, exact PASS/classification lines, retained evidence path,
conditional regression gates, source owners, deferred surfaces, and the next
bounded implementation recommendation.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, process-install core implementation,
process address-space mutation, user-frame allocation, page-table installation,
lower-EL launch, argv/envp construction, exec/spawn/wait, shell,
descriptor-backed filesystem syscall, writable filesystem, persistent storage,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation/source review: used the accepted process-install
  contract, process-install source inventory, program-loader smoke-plan
  pattern, roadmap, and ADR index.
- documentation diff: added
  docs/src/project/phase8-qemu-process-install-smoke-plan.md, linked it from
  docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- output/evidence specification: retained path
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log;
  classification qemu-process-install-smoke-complete; PASS line
  qemu-process-install-smoke: PASS.
- blocked surfaces: implementation, hardware, physical page allocation,
  page-table mutation, lower-EL launch, argv/envp, scheduler handoff,
  descriptor-backed filesystem syscalls, shell, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy remain blocked.
- recommendation: next bounded implementation task should be
  phase8-process-install-core-20260530 only because supervisor planning has
  already queued it with explicit scope and gates.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the Milestone 8.3 QEMU/substitute process-install smoke plan. The
accepted smoke plan uses loader fixture identity
phase8-program-loader-elf64-aarch64-v1, install boundary identity
phase8-process-install-plan-v1, retained evidence path
tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log,
classification qemu-process-install-smoke-complete, and PASS vocabulary. It
proves metadata-only install-plan derivation only; process address-space
installation, physical user frames, page-table mutation, lower-EL launch,
argv/envp stack construction, process creation, exec/spawn/wait, shell
behavior, descriptor-backed filesystem syscalls, Pi 5 hardware proof,
writable filesystems, persistent storage, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache-driver policy remain blocked.
