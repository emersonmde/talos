# Phase 11 RP1 GPIO Owned Event Discriminator Control Pi 5

Task id: phase11-rp1-gpio-owned-event-discriminator-control-pi5-20260607

Status: accepted

## Goal

Prove the paired no-MMIO/no-RP1/no-GIC control output shape on Pi 5 before any
real GPIO16 event/pending hardware proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Checked the accepted local/static control archive before publication:
  target/talos-rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-core.tar.gz.
- Published only the accepted no-MMIO/no-RP1/no-GIC GPIO16 event discriminator
  control archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed standard inconclusive-run triage after the first powered candidate
  run was rejected by non-empty pre-power serial drain evidence: candidate
  identity, fresh serial/TFTP evidence, known-good control, and candidate
  rerun.

## Non-Goals

No real RP1/GPIO/GIC/MSI-X/MMIO diagnostic, event/pending generation, GPIO14
retry, GPIO/RIO/pad/clock/reset writes, interrupt delivery, GIC IAR/EOIR
acknowledgement, ISR installation, broad GPIO driver ownership, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.3, phase transition, or real GPIO16 event discriminator behavior
acceptance.

## Classification

Accepted as no-mmio-gpio16-owned-event-discriminator-control-visible.

The accepted candidate rerun selected boot tree
a2cd628f8fed4b70b726c6659f2788762922334289f1d90eef60e61e01963e46 with
effective kernel_2712.img and a 49,480-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 49,480-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
capture retained 40 occurrences of
TALOS: rp1-gpio16-owned-event-discriminator-control.

The retained control output classification remains simulated/control. This
accepts only the no-MMIO/no-RP1/no-GIC output-shape and capture path for the
queued real GPIO16 event/source-status discriminator proof. Real GPIO16 event
generation, source-status behavior, interrupt delivery, GIC acknowledgement,
ISR/handler ownership, broad GPIO ownership, clock/reset programming,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe behavior,
Milestone 11.3, and phase transition remain unaccepted.

The capture helper restored the candidate snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-RP1/no-GIC
  control archive, including archive SHA-256, kernel SHA-256, marker string,
  and forbidden GPIO16 event MMIO strings absence.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had candidate-tied fetches and 40 control markers but was rejected by
  non-empty pre-power serial drain evidence.
- fixed: ran the required known-good control after the inconclusive first
  candidate run; it retained the production timer PASS marker and preserved the
  serial-drain rejection evidence.
- fixed: reran the selected no-MMIO/no-RP1/no-GIC control candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated GPIO16 event discriminator control output.
- deferred: the real GPIO16 event discriminator proof remains queued and must
  pass its own hardware lock, identity join, and classification gates.
- not-an-issue: no RP1 GPIO/GIC behavior is inferred from a no-MMIO simulated
  control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-control-pi5/control-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-control-pi5/known-good-control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 49,480-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 40 occurrences of
  TALOS: rp1-gpio16-owned-event-discriminator-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-gpio16-owned-event-discriminator-control-visible. The
queued real GPIO16 event discriminator proof is mechanically unblocked on a
future worker wake if hardwareTestLock remains unlocked/restored and
supervisorIntervention remains inactive.
