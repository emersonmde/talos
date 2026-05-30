# Phase 8 Program Loader Closeout Checkpoint

Status: accepted for phase8-program-loader-closeout-checkpoint-20260530.

## Scope

- Reconciled the accepted program-loader source inventory, format contract,
  QEMU/substitute smoke plan, target-independent core, retained
  QEMU/substitute evidence, validation gates, deferred surfaces, and residual
  risks.
- Added docs/src/project/phase8-program-loader-closeout-checkpoint.md and
  linked it from docs/src/SUMMARY.md.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md.
- Did not change Rust or assembly behavior, rerun QEMU, run Pi 5 hardware,
  publish a boot archive, or acquire hardwareTestLock.

## Accepted Reconciliation

- Source inventory commit: a4cb53483914622ae90529b16ab814639e14e45d.
- Format contract commit: d6020818bc3d9163b26590858ab7d225e2d62563.
- QEMU/substitute smoke plan commit:
  71b26c62e0f4502278a2c8d6bb63f48cd519c33b.
- Target-independent core commit:
  38b3a09ad4e1be353950ad75880b119e7e0b534e.
- QEMU/substitute smoke core commit:
  2ff02f29962804c8579a324baf41e232c203fc08.
- Retained smoke evidence:
  tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log.
- PASS/classification:
  qemu-program-loader-smoke: final participants=8 expected=8 errors=0
  classification=qemu-program-loader-smoke-complete
  qemu-program-loader-smoke: PASS.

## Deferred Surfaces

Process address-space installation, user-frame allocation, page-table
mutation, segment materialization, lower-EL launch, initial stack, argv/envp,
descriptor inheritance across exec, process creation, exec/spawn/wait, shell
behavior, descriptor-backed filesystem syscalls, Pi 5 hardware proof, boot
archive publication, writable filesystems, persistent storage, networking,
SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
blocked.

## Next Recommendation

Recommend phase8-process-install-source-inventory-20260530 as the next bounded
documentation-only Phase 8 task. It should inventory process-owned
address-space installation, user-frame allocation, segment mapping, zero-fill,
initial lower-EL frame, user stack policy, descriptor inheritance, and
scheduler/process ownership using the accepted image-plan-only loader output
as input, without accepting implementation, QEMU execution, hardware, shell,
networking, or driver surfaces.

## Evidence

- static inspection: git status --short before edits was clean.
- static evidence review: inspected the accepted loader source inventory,
  format contract, QEMU/substitute smoke plan, core task record, smoke task
  record, roadmap, ADR index, and retained evidence path.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Next Action

Accepted and committed. The already queued
phase8-process-install-source-inventory-20260530 task is mechanically
unblocked only because this checkpoint explicitly recommends it; promote it on
the next worker wake if dependencies remain satisfied and the working tree has
no relevant conflicts.
