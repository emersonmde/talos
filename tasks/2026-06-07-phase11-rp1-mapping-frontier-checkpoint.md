# Phase 11 RP1 Mapping Frontier Checkpoint

Task id: phase11-rp1-mapping-frontier-checkpoint-20260607

Status: accepted

## Goal

Close out the accepted RP1 UART0 FR mapped/read-value boundary and explicitly
authorize the move from Milestone 11.1 to the first Milestone 11.2 slice.

## Scope

- Inspected the accepted tail-stable source/static core evidence.
- Inspected the accepted tail-stable no-MMIO control proof and closeout
  evidence.
- Inspected the accepted tail-stable RP1 UART0 FR Pi 5 proof and closeout
  evidence.
- Updated the roadmap and RP1/PCIe map contract only to record the Milestone
  11.1 frontier and the next Milestone 11.2 source-contract task.

## Non-Goals

No source/runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 constant change, GPIO or pin-control write,
clock/reset programming, interrupt enablement or handling, DMA/cache policy,
storage, generated-root, networking, SSH, broader PCIe enumeration, or
Milestone 11.2 implementation.

## Classification

milestone-11-1-frontier-accepted.

The accepted Milestone 11.1 frontier is the read-only RP1 UART0 FR single-load
diagnostic at 0x1f00030018. The accepted source/static core committed at
c3700b21166100468e2131d35c221d88d1f1612e proves the RP1 candidate performs
exactly one contracted 32-bit volatile load from that address and the paired
no-MMIO control performs zero RP1 loads.

The accepted no-MMIO control proof and closeout committed at
b5878af296576c6b930426b2b6db208eaeec515c and
9b2d46059c0ebe99327a189da1894b2f11d0e9e9 prove the repeated tail-stable
output shape is capturable on Pi 5 without RP1 MMIO.

The accepted RP1 Pi 5 proof and closeout committed at
427acda2f1b6cf95a0e56a01196596e5be9cd97d and
01f09f9c9853badcc6c7e72d58492a2b49600c70 prove the selected RP1 candidate
passed the v2 identity join with stable TFTP, final pre-restore identity,
restore proof, and 1,498 repeated tail-stable markers:

    TALOS: fr-tail-stable-result contract=phase11-rp1-pcie-map-contract-v1 target=rp1-uart0-fr-read address=0x1f00030018 width=32 raw=0xdeaddead classification=mapped/read-value

Accepted claims are limited to that read-only RP1 UART0 FR diagnostic boundary
and the proof-chain evidence tying it to the selected candidate. GPIO and
pin-control ownership, RP1 clocks/resets, interrupt routing or handling,
DMA/cache behavior, storage, generated-root, networking, SSH, broader PCIe
enumeration, and Milestone 11.2 implementation remain unaccepted.

The next mechanically unblocked task after this checkpoint commit is
phase11-rp1-irq-clock-gpio-source-contract-20260607.

## Findings And Disposition

- fixed: recorded the accepted Milestone 11.1 frontier as the read-only RP1
  UART0 FR single-load diagnostic at 0x1f00030018.
- fixed: tied the accepted hardware result to v2 candidate identity, stable
  TFTP, final pre-restore identity, restore proof, and repeated tail-stable
  mapped/read-value markers with raw 0xdeaddead.
- fixed: updated the roadmap and RP1/PCIe map contract so the next task is the
  Milestone 11.2 source-contract review, not an implicit implementation or
  hardware run.
- deferred: GPIO/pin-control ownership, clocks/resets, interrupts, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration, and
  Milestone 11.2 implementation remain for later accepted tasks.
- not-an-issue: no new runtime code or hardware evidence is required for this
  checkpoint because it reconciles already accepted committed evidence.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-mapping-frontier-checkpoint/evidence-map.json.
- Static reconciliation:
  tasks/evidence/2026-06-07-phase11-rp1-mapping-frontier-checkpoint/static-reconciliation.md.
- Source/static core evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/evidence-map.json.
- No-MMIO control evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/evidence-map.json.
- No-MMIO control closeout evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout/evidence-map.json.
- RP1 tail-stable Pi 5 proof evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/evidence-map.json.
- RP1 tail-stable closeout evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-closeout/evidence-map.json.

## Validation

- static inspection of accepted task records and evidence maps: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as milestone-11-1-frontier-accepted. The next mechanically unblocked
task is phase11-rp1-irq-clock-gpio-source-contract-20260607.
