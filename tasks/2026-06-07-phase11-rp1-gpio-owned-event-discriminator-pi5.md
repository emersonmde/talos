# Phase 11 RP1 GPIO Owned Event Discriminator Pi 5

Task id: phase11-rp1-gpio-owned-event-discriminator-pi5-20260607

Status: accepted

## Goal

Run the accepted real Talos-owned RP1 GPIO16 event/pending discriminator on Pi
5 after the paired control proof, accepting only the bounded event/source-status
result or a precise blocker.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 hardware work.
- Published only the accepted real GPIO16 owned event discriminator candidate
  archive:
  target/talos-rpi5-rp1-gpio16-owned-event-discriminator-core.tar.gz.
- Retained static archive identity, candidate publication identity, fresh
  serial/TFTP cursors, serial capture, stable pre-restore TFTP evidence, final
  pre-restore identity, restore evidence, and v2 identity-join records.
- Performed standard inconclusive-run triage after the first powered candidate
  run was rejected by non-empty pre-power serial drain evidence: candidate
  identity, fresh serial/TFTP evidence, known-good control, and candidate rerun.

## Non-Goals

No interrupt delivery acceptance, GIC IAR/EOIR acknowledgement, ISR
installation, broad GPIO driver ownership, GPIO14 retry, unplanned pin-control,
pad, RIO, clock/reset writes, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe enumeration, Milestone 11.3, phase transition, or GPIO event
success acceptance beyond the selected contract classification.

## Classification

Accepted as gpio16-owned-event-preflight-blocked-pin-function.

The accepted candidate rerun selected boot tree
348b127402b41ca3115ed09aa2e55cc2dce837dc04a7e4770f0143bd17e4c61c with
effective kernel_2712.img and a 52,056-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain reached
empty, stable pre-restore TFTP retained two served 52,056-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
capture retained 38 occurrences of
TALOS: rp1-gpio16-owned-event-discriminator-result.

The retained result markers report the accepted preflight blocker
classification=gpio16-owned-event-preflight-blocked-pin-function. The GPIO16
preflight reported fsel 13 / unknown function, so the diagnostic skipped all
accepted action writes and restore writes. This accepts only the source-backed
hardware-visible GPIO16 preflight blocker. It does not accept GPIO16 event
generation, interrupt pending generation, interrupt delivery, GIC
acknowledgement, ISR/handler ownership, broad GPIO ownership, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
behavior, Milestone 11.3, or phase transition.

The capture helper restored the candidate snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  diagnostic work.
- fixed: retained static archive identity for the accepted real GPIO16
  discriminator archive, including archive SHA-256, kernel SHA-256, marker
  string, and forbidden control string absence.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had candidate-tied fetches and real result markers but was rejected by
  non-empty pre-power serial drain evidence.
- fixed: ran the required known-good control after the inconclusive first
  candidate run, then reran the same real candidate without code changes.
- fixed: accepted the identity-joined candidate rerun as a precise GPIO16
  pin-function preflight blocker with action-skipped=true and no GPIO16
  mutation/restore writes attempted.
- deferred: a future supervisor-planned task must choose a different
  discriminator or source-backed ownership/function strategy before attempting
  event generation again.
- not-an-issue: the accepted result is a blocker classification from the
  contract, not a failed interrupt-delivery proof.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/real-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/real-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/known-good-control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 52,056-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 38 occurrences of
  TALOS: rp1-gpio16-owned-event-discriminator-result were retained.
- inconclusive-run triage: completed; first candidate run and known-good
  control both recorded serial-drain rejection, then the candidate rerun passed
  identity join.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as gpio16-owned-event-preflight-blocked-pin-function. The queued
closeout task is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
