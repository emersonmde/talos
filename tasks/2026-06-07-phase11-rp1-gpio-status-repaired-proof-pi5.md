# Phase 11 RP1 GPIO Status Repaired Proof

Task id: phase11-rp1-gpio-status-repaired-proof-pi5-20260607

Status: accepted

## Goal

Run one repaired-proof Pi 5 discriminator for the accepted read-only RP1 GPIO14
STATUS diagnostic after the repaired no-MMIO control proof passed.

## Scope

- Acquired hardwareTestLock before publishing the GPIO14 STATUS diagnostic
  archive.
- Published only the accepted diagnostic archive:
  target/talos-rpi5-rp1-gpio14-status-read-core.tar.gz.
- Captured hardware evidence through pi5-capture-transaction-v2: candidate
  identity, serial window, stable same-cursor TFTP evidence, final pre-restore
  identity, restore proof, and identity-join output.
- Performed standard triage after the first proof was compromised by
  non-empty pre-power serial drain: retained the rejected first run, ran a
  known-good control, then reran the selected GPIO14 STATUS candidate.

## Non-Goals

No uncontracted GPIO/pin-control writes, pad writes, clock/reset programming,
interrupt enablement or handling, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
transition. This proof accepts only the read-only GPIO14 STATUS diagnostic
boundary.

## Classification

Accepted as gpio14-status-result-identity-joined.

The accepted candidate rerun selected boot tree
cb7827b07a3822370fc610dfd18a8ab580cea31a47c4559e41a242975976f83a with
effective kernel_2712.img and 46,336-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons. The capture retained two
selected 46,336-byte TFTP fetches, final selected-tree identity, and 390
occurrences of TALOS: gpio14-status-result. The repeated result lines reported
contract phase11-rp1-irq-clock-gpio-contract-v1, target
rp1-gpio14-status-read, address 0x1f000d0070, width 32, raw 0xdeaddead, and
classification=diagnostic-result-visible.

The lab was restored to the pre-run known-good tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired hardwareTestLock before archive publication and retained
  final restore evidence before release.
- fixed: retained static archive identity for the accepted GPIO14 STATUS
  diagnostic archive before publication.
- fixed: retained the first diagnostic proof as compromised evidence. Prior
  no-MMIO control output was still streaming, so pre-power serial drain did not
  reach empty; this run is not used for acceptance.
- fixed: ran the required known-good control after the compromised first proof.
  It retained rpi5-production-timer-preemption: PASS output and reset runtime
  serial state for the candidate rerun.
- fixed: reran the selected GPIO14 STATUS diagnostic candidate; the rerun
  passed the v2 identity join and retained repeated diagnostic result output.
- deferred: interrupt delivery, GPIO ownership, clock/reset programming,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
  11.3, and phase transition remain unaccepted.
- not-an-issue: the accepted diagnostic reads status only and does not claim
  ownership of GPIO14 or alter pin control.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-repaired-proof-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-repaired-proof-pi5/classification.json.
- Compromised first proof:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-repaired-proof-pi5/proof-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-repaired-proof-pi5/known-good-control-run/.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-repaired-proof-pi5/proof-rerun-after-kg/.

## Validation

- static archive identity check: passed against accepted diagnostic evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted
  candidate rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two selected
  46,336-byte candidate fetches were retained.
- serial hardware boot/output: passed; 390 occurrences of
  TALOS: gpio14-status-result were retained with raw 0xdeaddead.
- known-good control and candidate rerun after compromised evidence: run and
  retained.
- restore proof: passed; final lab status returned to the pre-run tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as gpio14-status-result-identity-joined. The queued closeout task is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored.
