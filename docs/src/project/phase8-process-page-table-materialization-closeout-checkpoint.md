# Phase 8 Process Page-Table Materialization Closeout Checkpoint

Status: accepted

Task: phase8-process-page-table-materialization-closeout-checkpoint-20260530

## Scope

This checkpoint closes out the accepted Phase 8 Milestone 8.3 process
page-table materialization slice as documentation-only work. It reconciles the
source inventory, contract, QEMU/substitute smoke plan, implementation,
retained QEMU/substitute evidence, deferred surfaces, and next planning state.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- process page-table materialization source inventory:
  8de8472.
- process page-table materialization contract:
  7d8c0ce.
- QEMU/substitute process page-table materialization smoke plan:
  4ee01e8.
- process page-table materialization core:
  54d519e.
- QEMU/substitute process page-table materialization smoke core:
  6169783.
- retained QEMU/substitute smoke evidence:
  tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log.

The retained smoke evidence contains:

- fixture identity phase8-program-loader-elf64-aarch64-v1.
- install boundary identity phase8-process-install-plan-v1.
- address-space boundary identity phase8-process-address-space-model-v1.
- materialization boundary identity
  phase8-process-page-table-materialization-v1.
- final classification:
  qemu-process-page-table-materialization-smoke-complete.
- exact PASS line:
  qemu-process-page-table-materialization-smoke: PASS.
- success evidence for one non-activating materialization with one root page,
  three table pages, three user frames, three descriptors,
  activation-blocked=true, and
  kernel-mapping-policy=activation-blocked-no-kernel-half.
- UserText/UserData descriptor evidence preserving AP, PXN/UXN, AF,
  normal-inner-shareable attributes, and W^X.
- side-effect evidence showing ttbr-mutated=false, tlb-mutated=false,
  scheduler-published=false, lower-el-frame=false, and runnable=false.
- teardown evidence for first-release and second already-destroyed behavior.
- deterministic no-partial-materialization/no-leak rejections for bad
  address-space, forbidden range, permission widening, resource exhaustion,
  unsupported topology, copy/zero mismatch, and activation request.

## Accepted Frontier

The accepted capability is a non-activating AArch64 descriptor-image and
user-frame materialization record for immutable /bin/init. It proves material
lease ownership, ordered EL0 descriptor records, byte copy/zero accounting,
permission preservation, rollback, and idempotent teardown below TTBR
activation.

This does not prove TTBR/TCR/MAIR/SCTLR writes, live ASID/TLB sequencing,
kernel-half mapping activation, lower-EL launch, argv/envp, process creation,
exec/spawn/wait, descriptor inheritance, shell behavior, filesystem syscalls,
Pi 5 behavior, networking, or SSH.

## Next Planning State

No explicit queued follow-up task remains after this checkpoint. Supervisor
planning is required before the worker may promote another Phase 8.3 task. The
likely frontier is a bounded documentation-only inventory or contract for the
next real launch prerequisite, such as TTBR activation, initial user stack, or
lower-EL launch setup, but this checkpoint does not create that task.
