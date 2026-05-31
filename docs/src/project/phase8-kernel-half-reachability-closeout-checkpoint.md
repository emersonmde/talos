# Phase 8 Kernel-Half Reachability Closeout Checkpoint

Status: accepted

Task: phase8-kernel-half-reachability-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the accepted Phase 8 Milestone 8.3 kernel-half
reachability preflight slice as documentation-only work. It reconciles the
source inventory, contract, QEMU/substitute smoke plan, implementation,
retained QEMU/substitute evidence, deferred surfaces, residual risks, and next
planning state.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- kernel-half reachability source inventory:
  1eda202.
- kernel-half reachability contract:
  7d95e0a.
- QEMU/substitute kernel-half reachability smoke plan:
  c1645d5.
- kernel-half reachability core:
  a4294f5.
- QEMU/substitute kernel-half reachability smoke core:
  edda81d.
- retained QEMU/substitute smoke evidence:
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.

The retained smoke evidence contains:

- fixture identity phase8-program-loader-elf64-aarch64-v1.
- install boundary identity phase8-process-install-plan-v1.
- address-space boundary identity phase8-process-address-space-model-v1.
- materialization boundary identity
  phase8-process-page-table-materialization-v1.
- launch boundary identity phase8-initial-process-launch-plan-v1.
- stack boundary identity phase8-initial-user-stack-plan-v1.
- activation boundary identity phase8-live-address-space-activation-plan-v1.
- kernel-half boundary identity phase8-kernel-half-reachability-plan-v1.
- kernel-half policy
  preflight-ttbr1-shared-kernel-root-reachability-v1.
- TTBR0 root provenance from the materialized process root lease with
  ttbr0-written=false.
- TTBR1 shared privileged kernel-root policy with descriptor-image
  construction blocked.
- required kernel text, rodata, data, bss, vectors, active stack, heap,
  page-frame, UART/MMIO diagnostic, scheduler, and panic/fault reachability.
- privileged-only kernel text execution, non-executable data, no normal-memory
  device mapping, and no EL0 kernel access.
- split TCR compatibility-record-only state, normal/device MAIR
  compatibility-record-only state, and SCTLR mutation-blocked state.
- blocked ASID allocation, live TLB invalidation, planned-only DSB/ISB, and
  live-register sequence states.
- side-effect evidence showing no TTBR/TCR/MAIR/SCTLR mutation, descriptor
  image installation, ASID allocation, TLB mutation, live DSB/ISB, lower-EL
  ERET, scheduler publication, process-table mutation, or descriptor-table
  mutation.
- idempotent plan-local teardown while accepted input records remain owned by
  their existing teardown paths.
- deterministic identity mismatch, missing kernel range, missing diagnostic
  fault-reporting, forbidden EL0 access, bad device attribute intent,
  live-register request, descriptor-image request, scheduler-publication
  request, lower-EL launch request, and resource-exhaustion rejection cases.
- final classification:
  qemu-kernel-half-reachability-smoke-complete.
- exact PASS line:
  qemu-kernel-half-reachability-smoke: PASS.

## Accepted Frontier

The accepted capability is a target-independent KernelHalfReachabilityPlan
preflight record for immutable /bin/init. It proves copied
loader/install/address-space/materialization/launch/stack/live-activation
lineage, TTBR0 materialized-root provenance without live TTBR0_EL1 mutation,
selected TTBR1 shared privileged kernel-root policy, blocked kernel-half
descriptor-image construction, required privileged kernel reachability records,
compatibility-only TCR/MAIR observations, blocked SCTLR/ASID/TLB/barrier/live
register state, idempotent plan-local teardown, deterministic no-partial
rejection, and zero live side effects.

This does not prove kernel-half descriptor-image construction, live TTBR0_EL1
or TTBR1_EL1 programming, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID
allocation, live TLB invalidation, live DSB/ISB activation sequencing,
lower-EL ERET, scheduler runnable publication, process table/PID/wait/exit
behavior, exec/spawn/wait, broad argv/envp/auxv/TLS ABI, descriptor-backed
filesystem syscalls, shell behavior, Pi 5 behavior, networking, or SSH.

## Deferred Surfaces

Still blocked after this checkpoint:

- kernel-half descriptor-image construction and installation.
- live TTBR0_EL1/TTBR1_EL1 activation and TCR/MAIR/SCTLR writes.
- ASID allocation and live TLB invalidation.
- live DSB/ISB activation sequencing.
- lower-EL ERET and architectural register writes.
- scheduler runnable publication and process table mutation.
- PID allocation, process lifecycle, wait/exit, exec/spawn, and descriptor
  inheritance semantics.
- broad argv/envp/auxv/TLS startup ABI and libc-compatible startup framing.
- descriptor-backed filesystem syscalls, writable filesystem state, and shell
  behavior.
- Pi 5 hardware proof, boot archive publication, and hardwareTestLock use.
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Next Planning State

No explicit queued follow-up task remains after this checkpoint. Supervisor
planning is required before the worker may promote another Phase 8.3 task.
Likely future frontiers include kernel-half descriptor-image construction,
live translation-register activation, lower-EL launch setup, scheduler
runnable publication, or startup ABI expansion, but this checkpoint does not
create or select that task.
