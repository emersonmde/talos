# Phase 8 Live Address-Space Activation Closeout Checkpoint

Status: accepted

Task: phase8-live-address-space-activation-closeout-checkpoint-20260530

## Scope

This checkpoint closes out the accepted Phase 8 Milestone 8.3 live
address-space activation preflight slice as documentation-only work. It
reconciles the source inventory, contract, QEMU/substitute smoke plan,
implementation, retained QEMU/substitute evidence, deferred surfaces, and next
planning state.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- live address-space activation source inventory:
  7f64569.
- live address-space activation contract:
  89c624b.
- QEMU/substitute live address-space activation smoke plan:
  b8f403d.
- live address-space activation core:
  1293377.
- QEMU/substitute live address-space activation smoke core:
  1c441c3.
- retained QEMU/substitute smoke evidence:
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log.

The retained smoke evidence contains:

- fixture identity phase8-program-loader-elf64-aarch64-v1.
- install boundary identity phase8-process-install-plan-v1.
- address-space boundary identity phase8-process-address-space-model-v1.
- materialization boundary identity
  phase8-process-page-table-materialization-v1.
- launch-plan boundary identity phase8-initial-process-launch-plan-v1.
- stack boundary identity phase8-initial-user-stack-plan-v1.
- activation boundary identity phase8-live-address-space-activation-plan-v1.
- activation policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1.
- TTBR0 root provenance from the materialized process root lease with
  ttbr0-written=false.
- TTBR1/kernel-half policy blocked-no-accepted-kernel-half-map.
- TCR and MAIR compatibility-record-only state and SCTLR mutation-blocked
  state.
- ASID, TLB, barrier, and live-register sequence blockers.
- kernel reachability prerequisites for VBAR_EL1, exception vectors, active
  kernel stack, kernel text/data, allocator, UART/MMIO diagnostics, scheduler
  code/data, and panic/fault reporting.
- model-only launch binding from blocked-no-ttbr-activation to
  model-only-activation-preflight-ready.
- idempotent plan-local teardown that leaves materialization, launch, stack,
  and image ownership with their existing teardown paths.
- deterministic identity mismatch, entry/stack/descriptor disagreement,
  forbidden-range, missing-kernel-reachability, live-register,
  scheduler-publication, lower-EL launch, and resource-exhaustion rejection
  cases.
- side-effect evidence showing ttbr-mutated=false, tcr-mutated=false,
  mair-mutated=false, sctlr-mutated=false, asid-allocated=false,
  tlb-mutated=false, live-dsb-isb=false, lower-el-eret=false,
  scheduler-published=false, process-table-mutated=false, and
  descriptor-table-mutated=false.
- final classification:
  qemu-live-address-space-activation-smoke-complete.
- exact PASS line:
  qemu-live-address-space-activation-smoke: PASS.

## Accepted Frontier

The accepted capability is a target-independent
LiveAddressSpaceActivationPlan preflight record for immutable /bin/init. It
proves copied loader/install/address-space/materialization/launch/stack
lineage, TTBR0 root provenance without live TTBR0_EL1 mutation, explicit
TTBR1/kernel-half blocking, compatibility-only TCR/MAIR records, blocked
SCTLR/ASID/TLB/barrier/live-register state, required kernel-reachability
prerequisites, model-only launch binding, idempotent plan-local teardown,
deterministic no-partial-activation rejection, and no-runnable-publication
evidence.

This does not prove live TTBR0_EL1/TTBR1_EL1 programming, TCR_EL1/MAIR_EL1/
SCTLR_EL1 mutation, ASID allocation, live TLB invalidation, live DSB/ISB
activation sequencing, lower-EL ERET, scheduler runnable publication, process
table/PID/wait/exit behavior, exec/spawn/wait, broad argv/envp/auxv/TLS ABI,
descriptor-backed filesystem syscalls, shell behavior, Pi 5 behavior,
networking, or SSH.

## Deferred Surfaces

Still blocked after this checkpoint:

- live TTBR0_EL1/TTBR1_EL1 activation and TCR/MAIR/SCTLR writes.
- accepted kernel-half mapping and live kernel reachability under a switched
  address space.
- ASID allocation and live TLB invalidation.
- live DSB/ISB activation sequencing.
- lower-EL ERET and architectural register writes.
- scheduler runnable publication and process table mutation.
- PID allocation, process lifecycle, wait/exit, exec/spawn, and descriptor
  inheritance semantics.
- broad argv/envp/auxv/TLS startup ABI and libc-compatible startup framing.
- descriptor-backed filesystem syscalls, writable filesystem state, and
  shell behavior.
- Pi 5 hardware proof, boot archive publication, and hardwareTestLock use.
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Next Planning State

No explicit queued follow-up task remains after this checkpoint. Supervisor
planning is required before the worker may promote another Phase 8.3 task.
Likely future frontiers include accepted kernel-half mapping for live address
spaces, lower-EL launch setup, scheduler runnable publication, or startup ABI
expansion, but this checkpoint does not create or select that task.
