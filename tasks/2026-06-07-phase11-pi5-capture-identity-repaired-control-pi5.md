# Phase 11 Pi 5 Capture Identity Repaired Control

Task id: phase11-pi5-capture-identity-repaired-control-pi5-20260607

Status: accepted

## Goal

Use one serialized Pi 5 no-MMIO control run to prove the repaired/audited
capture identity path before retrying any real GPIO14 STATUS diagnostic.

## Scope

- Acquired hardwareTestLock before publishing the no-MMIO control archive.
- Published only the accepted GPIO14 no-MMIO control archive:
  target/talos-rpi5-rp1-gpio14-status-no-mmio-control-core.tar.gz.
- Captured hardware evidence through pi5-capture-transaction-v2: clean
  pre-power serial drain, direct-read serial window, stable same-cursor TFTP
  delta, final pre-restore identity, restore proof, and identity-join output.
- Performed standard triage after the first control attempt was compromised:
  retained the rejected first run, ran a known-good control, then reran the
  selected no-MMIO control candidate.

## Non-Goals

No real RP1 GPIO14 STATUS load, GPIO/pin-control write, clock/reset
programming, interrupt enablement, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
transition. Control visibility is not evidence of real RP1 GPIO/status
behavior.

## Classification

Accepted as repaired-capture-control-visible.

The accepted candidate rerun selected boot tree
99b1f8b2d7295da435bc82236fe0192f9dd9aade64e877508335cc2014d356fc with
effective kernel_2712.img and 46,160-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons. The capture retained two
selected 46,160-byte TFTP fetches, final selected-tree identity, and 795
occurrences of TALOS: gpio14-status-control.

The lab was restored to the pre-run known-good tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired hardwareTestLock before archive publication and retained
  final restore evidence before release.
- fixed: retained static archive identity for the accepted no-MMIO GPIO14
  control archive before publication.
- fixed: retained the first control attempt as compromised evidence. A manual
  restore overlapped the still-running capture transaction, so the final
  pre-restore identity matched the restored known-good tree; this run is not
  used for acceptance.
- fixed: ran the required known-good control after the compromised first
  attempt; it passed the v2 identity join and retained the production timer
  PASS marker.
- fixed: reran the selected no-MMIO control candidate; the rerun passed the
  v2 identity join and retained repeated GPIO14 status control output.
- deferred: the real GPIO14 STATUS diagnostic proof remains queued and must
  pass its own hardware lock, identity join, and classification gates.
- not-an-issue: no real RP1 GPIO/status behavior is inferred from no-MMIO
  control visibility.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-repaired-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-repaired-control-pi5/classification.json.
- Compromised first control attempt:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-repaired-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-repaired-control-pi5/known-good-control-run/.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-repaired-control-pi5/control-rerun-after-kg/.

## Validation

- static archive identity check: passed against accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 46,160-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 795 occurrences of
  TALOS: gpio14-status-control were retained.
- known-good control and candidate rerun after compromised evidence: run and
  retained.
- restore proof: passed; final lab status returned to the pre-run tree.
- git diff --check: passed.
- mdbook build: not run; no docs/src files touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as repaired-capture-control-visible. The queued real RP1 GPIO14
STATUS repaired proof is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
