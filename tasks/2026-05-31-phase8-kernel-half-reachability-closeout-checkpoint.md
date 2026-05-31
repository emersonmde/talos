# Phase 8 Kernel-Half Reachability Closeout Checkpoint Task

Task: phase8-kernel-half-reachability-closeout-checkpoint-20260531

Status: accepted

## Scope

Documentation-only closeout for the accepted Phase 8 Milestone 8.3
kernel-half reachability preflight slice.

Changed files:

- docs/src/project/phase8-kernel-half-reachability-closeout-checkpoint.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-kernel-half-reachability-closeout-checkpoint.md

Non-goals honored: no Rust behavior change, no assembly behavior change, no
QEMU rerun, no Pi 5 hardware run, no boot archive publication, no
hardwareTestLock acquisition, no live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/
SCTLR_EL1 write, no ASID allocation, no TLB mutation, no live DSB/ISB, no
lower-EL ERET, no scheduler runnable publication, no process lifecycle, no
shell behavior, no descriptor-backed filesystem syscalls, no writable
filesystem, no networking, no SSH, no RP1/PCIe, no UART interrupt ownership,
and no DMA/cache-driver policy.

## Reviewed Evidence

- kernel-half reachability source inventory commit:
  1eda202e085d91759ad8b9e884772416516766bb.
- kernel-half reachability contract commit:
  7d95e0a7ae2041ff00a19ff89515cde9b11cb99f.
- QEMU/substitute kernel-half reachability smoke plan commit:
  c1645d5851e27b99f7aeee100782738bfdb4b093.
- kernel-half reachability core commit:
  a4294f5f6a89a44c5fcee41028e916d9655767b0.
- QEMU/substitute kernel-half reachability smoke core commit:
  edda81d340d919962b6856575451e201344abd29.
- retained QEMU/substitute smoke log:
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.

The retained smoke evidence contains the required exact classification and
PASS lines:

    qemu-kernel-half-reachability-smoke: final participants=16 expected=16 errors=0 classification=qemu-kernel-half-reachability-smoke-complete
    qemu-kernel-half-reachability-smoke: PASS

## Outcome

The checkpoint documents the accepted frontier as a target-independent
KernelHalfReachabilityPlan preflight boundary below kernel-half descriptor
image construction, live translation-register mutation, and lower-EL launch.
The accepted evidence covers copied loader/install/address-space/
materialization/launch/stack/live-activation lineage, TTBR0 materialized-root
provenance, selected TTBR1 shared privileged kernel-root policy, blocked
descriptor-image construction, required privileged kernel reachability entries,
compatibility-only TCR/MAIR records, blocked SCTLR/ASID/TLB/barrier/live
register state, deterministic no-partial rejection, idempotent plan-local
teardown, and zero live side effects.

The checkpoint records that descriptor-image construction, live TTBR/TCR/MAIR/
SCTLR mutation, ASID allocation, live TLB invalidation, live DSB/ISB,
lower-EL ERET, scheduler runnable publication, process lifecycle, broad
argv/envp/auxv/TLS ABI, descriptor-backed filesystem syscalls, Pi 5 proof,
shell behavior, writable filesystem behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache-driver policy remain blocked.

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
