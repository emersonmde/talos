# Phase 8 Initial Process Launch Closeout Checkpoint

Status: accepted

Task: phase8-initial-process-launch-closeout-checkpoint-20260530

## Scope

This checkpoint closes out the accepted Phase 8 Milestone 8.3 initial process
launch-preparation slice as documentation-only work. It reconciles the source
inventory, contract, QEMU/substitute smoke plan, implementation, retained
QEMU/substitute evidence, deferred surfaces, and next planning state.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- initial process launch source inventory:
  ec03c37.
- initial process launch contract:
  fce7a11.
- QEMU/substitute initial process launch smoke plan:
  d353b88.
- initial process launch core:
  a57b067.
- QEMU/substitute initial process launch smoke core:
  a2a5f0b.
- retained QEMU/substitute smoke evidence:
  tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log.

The retained smoke evidence contains:

- fixture identity phase8-program-loader-elf64-aarch64-v1.
- install boundary identity phase8-process-install-plan-v1.
- address-space boundary identity phase8-process-address-space-model-v1.
- materialization boundary identity
  phase8-process-page-table-materialization-v1.
- launch-plan boundary identity phase8-initial-process-launch-plan-v1.
- final classification:
  qemu-initial-process-launch-smoke-complete.
- exact PASS line:
  qemu-initial-process-launch-smoke: PASS.
- success evidence for one InitialProcessLaunchPlan with entry copied from the
  accepted ProgramImagePlan lineage.
- entry provenance evidence through the accepted install plan,
  ProcessAddressSpace UserText mapping, and EL0-executable UserText
  descriptor.
- blocked-missing-initial-user-stack and blocked-no-ttbr-activation state.
- saved-frame intent for ELR, SP_EL0, SPSR, x0 through x5, DAIF, and
  address-space token state without architectural register writes.
- side-effect evidence showing ttbr-mutated=false, tcr-mutated=false,
  mair-mutated=false, sctlr-mutated=false, asid-allocated=false,
  tlb-mutated=false, lower-el-eret=false, scheduler-published=false,
  process-table-mutated=false, and descriptor-table-mutated=false.
- ENOSYS commit-to-runnable rejection with no-partial-launch=true and
  no-runnable-publication=true.
- deterministic no-partial-launch/no-runnable-publication rejections for
  identity mismatch, entry mismatch, missing UserText descriptor, forbidden
  entry range, destroyed input, activation request, stack-required launch, and
  scheduler publication request.

## Accepted Frontier

The accepted capability is a target-independent InitialProcessLaunchPlan for
immutable /bin/init. It proves launch-preparation identity, entry provenance,
blocked initial-user-stack state, blocked TTBR activation state, saved-frame
intent, zero launch side effects, and QEMU/substitute deterministic rejection
evidence below any runnable lower-EL process.

This does not prove initial user stack construction, argv/envp/auxv/TLS layout,
TTBR/TCR/MAIR/SCTLR writes, ASID allocation, live TLB invalidation, lower-EL
ERET, scheduler runnable publication, process table/PID/wait/exit behavior,
descriptor inheritance, exec/spawn/wait, shell behavior, filesystem syscalls,
Pi 5 behavior, networking, or SSH.

## Next Planning State

No explicit queued follow-up task remains after this checkpoint. Supervisor
planning is required before the worker may promote another Phase 8.3 task. The
likely frontier is a bounded documentation-only inventory or contract for the
next real launch prerequisite, such as initial user stack construction, live
address-space activation, or lower-EL launch setup, but this checkpoint does
not create that task.
