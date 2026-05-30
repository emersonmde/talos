# Phase 8 Initial User Stack Closeout Checkpoint

Status: accepted

Task: phase8-initial-user-stack-closeout-checkpoint-20260530

## Scope

This checkpoint closes out the accepted Phase 8 Milestone 8.3 initial user
stack slice as documentation-only work. It reconciles the source inventory,
contract, QEMU/substitute smoke plan, implementation, retained QEMU/substitute
evidence, deferred surfaces, and next planning state.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- initial user stack source inventory:
  6a90e43.
- initial user stack contract:
  f64be3a.
- QEMU/substitute initial user stack smoke plan:
  5c6a975.
- initial user stack core:
  f76c07f.
- QEMU/substitute initial user stack smoke core:
  7007acf.
- retained QEMU/substitute smoke evidence:
  tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log.

The retained smoke evidence contains:

- fixture identity phase8-program-loader-elf64-aarch64-v1.
- install boundary identity phase8-process-install-plan-v1.
- address-space boundary identity phase8-process-address-space-model-v1.
- materialization boundary identity
  phase8-process-page-table-materialization-v1.
- launch-plan boundary identity phase8-initial-process-launch-plan-v1.
- stack boundary identity phase8-initial-user-stack-plan-v1.
- fixed stack top and initial SP 0x0000_8000_0000_0000 with 16-byte
  alignment.
- usable range [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000).
- guard range [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000).
- four stack-owned USER_DATA page leases, copied_bytes=0, and
  zeroed_bytes=0x4000.
- one unmapped guard page with no frame lease and no descriptor.
- minimal-empty-argc0 startup metadata with argv/envp NULL and auxv/TLS still
  blocked-pending-startup-abi.
- model-only launch-plan stack-ready binding with activation still
  blocked-no-ttbr-activation.
- idempotent teardown releasing stack leases without touching image leases.
- deterministic identity mismatch, range fault, image overlap,
  executable-stack, capacity-exhausted, already-stack-ready, and live-launch
  rejection cases.
- side-effect evidence showing ttbr-mutated=false, tcr-mutated=false,
  mair-mutated=false, sctlr-mutated=false, asid-allocated=false,
  tlb-mutated=false, lower-el-eret=false, scheduler-published=false,
  process-table-mutated=false, and descriptor-table-mutated=false.
- final classification:
  qemu-initial-user-stack-smoke-complete.
- exact PASS line:
  qemu-initial-user-stack-smoke: PASS.

## Accepted Frontier

The accepted capability is a target-independent InitialUserStackPlan for the
immutable /bin/init launch-preparation lineage. It proves fixed stack layout,
guard reservation, stack-owned USER_DATA lease accounting, zero/copy
accounting, minimal empty startup metadata, idempotent teardown, deterministic
no-partial-stack/no-partial-launch rejection, and model-only integration with
the accepted InitialProcessLaunchPlan.

This does not prove live TTBR activation, TCR/MAIR/SCTLR mutation, ASID
allocation, live TLB invalidation, lower-EL ERET, scheduler runnable
publication, process table/PID/wait/exit behavior, exec/spawn/wait, broad
argv/envp/auxv/TLS ABI, descriptor-backed filesystem syscalls, shell behavior,
Pi 5 behavior, networking, or SSH.

## Deferred Surfaces

Still blocked after this checkpoint:

- live TTBR0_EL1/TTBR1_EL1 activation and TCR/MAIR/SCTLR writes.
- ASID allocation and live TLB invalidation.
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
Likely future frontiers include live address-space activation, lower-EL launch
setup, or startup ABI expansion, but this checkpoint does not create or select
that task.
