# Phase 8 Initial User Stack Closeout Checkpoint Task

Task: phase8-initial-user-stack-closeout-checkpoint-20260530

Status: accepted

## Scope

Documentation-only closeout for the accepted Phase 8 Milestone 8.3 initial
user stack slice.

Changed files:

- docs/src/project/phase8-initial-user-stack-closeout-checkpoint.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-initial-user-stack-closeout-checkpoint.md

Non-goals honored: no Rust behavior change, no assembly behavior change, no
QEMU rerun, no Pi 5 hardware run, no boot archive publication, no
hardwareTestLock acquisition, no TTBR/TCR/MAIR/SCTLR write, no ASID
allocation, no TLB mutation, no lower-EL ERET, no scheduler runnable
publication, no process lifecycle, no shell behavior, no descriptor-backed
filesystem syscalls, no writable filesystem, no networking, no SSH, no
RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Reviewed Evidence

- initial user stack source inventory commit:
  6a90e4317258450ce7c732117e9de5a776034a01.
- initial user stack contract commit:
  f64be3aa2bcd2133a6cc31b610959afe0471846b.
- QEMU/substitute initial user stack smoke plan commit:
  5c6a975574fb7840f0e463c676a34d2a2d13bef1.
- initial user stack core commit:
  f76c07f264efd1fc570b678af71e8a26ada155fa.
- QEMU/substitute initial user stack smoke core commit:
  7007acf1ac821fcd84643e36e6b68c203adcda39.
- retained QEMU/substitute smoke log:
  tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log.

The retained smoke evidence contains the required exact classification and
PASS lines:

    qemu-initial-user-stack-smoke: final participants=13 expected=13 errors=0 classification=qemu-initial-user-stack-smoke-complete
    qemu-initial-user-stack-smoke: PASS

## Outcome

The checkpoint documents the accepted frontier as a target-independent
InitialUserStackPlan below live activation. The accepted evidence covers fixed
stack layout, guard reservation, stack-owned USER_DATA lease accounting,
zero/copy accounting, minimal empty startup metadata, idempotent teardown,
deterministic no-partial-stack/no-partial-launch rejection, zero live-launch
side effects, and model-only launch-plan stack-ready binding.

The checkpoint records that live TTBR activation, lower-EL ERET, scheduler
runnable publication, process lifecycle, broad argv/envp/auxv/TLS ABI,
descriptor-backed filesystem syscalls, Pi 5 proof, shell behavior, writable
filesystem behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked.

No explicit queued follow-up task remains after this checkpoint, so durable
state should set planningNeeded=true for supervisor planning.

## Validation

- static inspection: git status --short before edits was clean except durable
  supervisor state promotion outside the Talos repo.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed.

## Commit

Recorded in durable supervisor state after acceptance.
