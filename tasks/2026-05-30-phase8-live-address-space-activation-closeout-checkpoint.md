# Phase 8 Live Address-Space Activation Closeout Checkpoint Task

Task: phase8-live-address-space-activation-closeout-checkpoint-20260530

Status: accepted

## Scope

Documentation-only closeout for the accepted Phase 8 Milestone 8.3 live
address-space activation preflight slice.

Changed files:

- docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-live-address-space-activation-closeout-checkpoint.md

Non-goals honored: no Rust behavior change, no assembly behavior change, no
QEMU rerun, no Pi 5 hardware run, no boot archive publication, no
hardwareTestLock acquisition, no TTBR/TCR/MAIR/SCTLR write, no ASID
allocation, no TLB mutation, no lower-EL ERET, no scheduler runnable
publication, no process lifecycle, no shell behavior, no descriptor-backed
filesystem syscalls, no writable filesystem, no networking, no SSH, no
RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Reviewed Evidence

- live address-space activation source inventory commit:
  7f645691ea423bdc38e9bf04a27f75ce967984a5.
- live address-space activation contract commit:
  89c624b95f39739e67168c4e6465a61ee18f345d.
- QEMU/substitute live address-space activation smoke plan commit:
  b8f403dded6820062843c5772f3f4f8079b20b3b.
- live address-space activation core commit:
  129337734011004297da0b2768a3a802063c3293.
- QEMU/substitute live address-space activation smoke core commit:
  1c441c301387ed75e24db7f9788301126f1f5a72.
- retained QEMU/substitute smoke log:
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log.

The retained smoke evidence contains the required exact classification and
PASS lines:

    qemu-live-address-space-activation-smoke: final participants=15 expected=15 errors=0 classification=qemu-live-address-space-activation-smoke-complete
    qemu-live-address-space-activation-smoke: PASS

## Outcome

The checkpoint documents the accepted frontier as a target-independent
LiveAddressSpaceActivationPlan preflight boundary below live register mutation
and lower-EL launch. The accepted evidence covers copied loader/install/
address-space/materialization/launch/stack lineage, activation identity,
activation policy, TTBR0 root provenance without TTBR0 writes, blocked
TTBR1/kernel-half policy, compatibility-only TCR/MAIR state, blocked SCTLR/
ASID/TLB/barrier/live-register state, kernel reachability prerequisites,
model-only launch binding, idempotent plan-local teardown, deterministic
no-partial-activation rejection, no-runnable-publication, and zero live side
effects.

The checkpoint records that live TTBR/TCR/MAIR/SCTLR mutation, accepted
kernel-half mapping, ASID allocation, live TLB invalidation, lower-EL ERET,
scheduler runnable publication, process lifecycle, broad argv/envp/auxv/TLS
ABI, descriptor-backed filesystem syscalls, Pi 5 proof, shell behavior,
writable filesystem behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked.

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
