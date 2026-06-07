# Phase 11 RP1 UART0 FR Tail-Stable Control Closeout

Task id: phase11-rp1-uart0-fr-tail-stable-control-closeout-20260606

Status: accepted

## Goal

Reconcile the tail-stable no-MMIO control proof and decide whether the queued
RP1 UART0 FR tail-stable result proof is mechanically unblocked.

## Scope

- Inspected the accepted tail-stable result core evidence.
- Inspected the accepted no-MMIO Pi 5 control task record and evidence.
- Reconciled the v2 identity join, serial marker retention, TFTP, final
  pre-restore identity, and restore evidence from the accepted control rerun.
- Updated the Phase 11 RP1/PCIe contract and roadmap boundary.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition, source
runtime changes, new RP1 constants, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition. This closeout does not accept RP1 UART0 FR mapped/read-value,
bus-fault/trap, or firmware-state behavior.

## Classification

tail-stable-control-visible.

The accepted source/static core committed at
c3700b21166100468e2131d35c221d88d1f1612e produced two paired candidates. The
RP1 candidate keeps exactly one contracted volatile 32-bit load from
0x1f00030018 and repeats TALOS: fr-tail-stable-result only after a returned
load. The no-MMIO control candidate constructs no RP1 FR address, performs no
RP1 volatile load, and repeats TALOS: fr-tail-stable-control with
classification=simulated/control.

The accepted Pi 5 control committed at
b5878af296576c6b930426b2b6db208eaeec515c selected tree
b4b780193281538a643aec3c17898ae59204c335f32452b90cf08b0cb8e10161 with
effective kernel_2712.img and a 45,728-byte da591740/kernel_2712.img. The v2
identity join passed with an empty pre-power /serial/read drain, stable
same-cursor TFTP retaining two selected-candidate fetches, final pre-restore
selected-tree identity, and restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The saturated direct-read serial window retained 1,771 occurrences of
TALOS: fr-tail-stable-control. That accepts the no-MMIO simulated/control
tail-stable output shape as capturable on Pi 5, and it mechanically unblocks
the queued RP1 tail-stable result proof under hardwareTestLock and
supervisorIntervention rules.

Accepted claims are limited to the local/static paired discriminator shape, the
selected no-MMIO control archive identity, the v2 identity-joined Pi 5 control
run, repeated no-MMIO control marker retention, stable TFTP/final identity, and
restore hygiene. RP1 UART0 FR mapped/read-value behavior, bus-fault/trap
behavior, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase
transition remain unaccepted until the separate RP1 proof passes its own
classification gates.

## Findings And Disposition

- fixed: reconciled the accepted no-MMIO Pi 5 control into the Phase 11
  contract and roadmap boundary.
- fixed: confirmed that the decisive control rerun passed the
  pi5-capture-transaction-v2 identity join with no rejection reasons.
- fixed: accepted only the selected-candidate control rerun, not the earlier
  staging mismatch or non-empty pre-power-drain candidate/control attempts, as
  decisive feature evidence.
- fixed: preserved the paired source/static boundary: one RP1 load in the
  queued RP1 result candidate and zero RP1 loads in the accepted no-MMIO
  control.
- deferred: RP1 UART0 FR mapped/read-value or trap behavior remains blocked on
  the queued tail-stable RP1 result proof.
- removed: no RP1 register semantics are inferred from the no-MMIO
  simulated/control marker.
- not-an-issue: the lab-published tree hash differs from the local archive
  boot-tree identity while the selected kernel bytes and archive identity match
  the accepted control evidence.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout/evidence-map.json.
- Source/static core task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core.md.
- Source/static core evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/evidence-map.json.
- No-MMIO control task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5.md.
- No-MMIO control evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/evidence-map.json.

## Validation

- static inspection of control task record/evidence: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as tail-stable-control-visible. The queued
phase11-rp1-uart0-fr-tail-stable-result-pi5-20260606 proof is mechanically
unblocked on a future worker wake if hardwareTestLock remains unlocked/restored
and supervisorIntervention remains inactive.
