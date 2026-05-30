# Phase 8 Process Address-Space Source Inventory Task

Task: phase8-process-address-space-source-inventory-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 source inventory after the accepted
process-install closeout. The task mapped source owners, accepted inputs, and
gaps for turning a metadata-only ProcessImageInstallPlan into a future
process-owned address-space installation boundary.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no frame allocation, no physical byte copy, no page-table
mutation, no TTBR/TCR switching, no lower-EL launch, no argv/envp, no
exec/spawn/wait, no shell, no descriptor-backed filesystem syscall, no
writable filesystem, no persistent storage, no networking, no SSH, no
RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Evidence

- inventory document:
  docs/src/project/phase8-process-address-space-source-inventory.md.
- reviewed docs:
  - docs/src/project/phase8-process-install-source-inventory.md
  - docs/src/project/phase8-process-install-contract.md
  - docs/src/project/phase8-qemu-process-install-smoke-plan.md
  - docs/src/project/phase8-process-install-closeout-checkpoint.md
  - docs/src/project/phase8-program-loader-source-inventory.md
  - docs/src/project/phase8-program-loader-format-contract.md
  - docs/src/project/phase7-el0-trap-address-space-contract.md
  - docs/src/project/phase7-copyin-copyout-helper-contract.md
- reviewed source owners:
  src/process_install.rs, src/program_loader.rs, src/posix.rs,
  src/scheduler.rs, src/arch/aarch64/exceptions.rs, src/arch/aarch64/mod.rs,
  src/memory_map/page_frames.rs, src/memory_map/translation.rs,
  src/target/qemu_virt.rs, and src/target/rpi5.rs.
- reviewed task/evidence records:
  - tasks/2026-05-30-phase8-process-install-core.md
  - tasks/2026-05-30-phase8-qemu-process-install-smoke-core.md
  - tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log
- gap summary: missing process address-space identity/lifetime, user-frame
  lease/release policy, page-table root ownership, user leaf descriptor
  policy, mapping order, rollback, teardown, TTBR/TCR switching policy, and
  address-space mutation evidence vocabulary.
- next bounded task recommendation:
  phase8-process-address-space-contract-20260530.

## Outcome

The inventory separates accepted metadata-only process-install behavior from
unaccepted physical mutation. ProcessImageInstallPlan now has a documented
source-owner map and gap list for a later address-space mutator, but no
process-owned address-space object, physical frame lease, page-table root,
mapping, lower-EL launch, process creation, descriptor inheritance, hardware,
networking, or SSH capability is accepted by this task.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
