# Phase 11 RP1 GPIO Status No-MMIO Control Pi 5

Task id: phase11-rp1-gpio-status-no-mmio-control-pi5-20260607

Status: accepted

## Goal

Run the serialized Pi 5 no-MMIO control for the source-contracted GPIO14
STATUS output shape before any real RP1 GPIO14 STATUS hardware proof.

## Scope

- Acquired hardwareTestLock for the serialized Pi 5 control runs.
- Checked the accepted local/static control archive before publication:
  target/talos-rpi5-rp1-gpio14-status-no-mmio-control-core.tar.gz.
- Published only the accepted no-MMIO control archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed standard inconclusive-run triage after the first run was rejected
  by non-empty pre-power serial drain evidence: candidate identity, fresh
  serial/TFTP evidence, known-good control, and candidate rerun.

## Non-Goals

No real RP1 GPIO14 STATUS load, uncontracted RP1 MMIO, GPIO ownership, pinmux
change, pad write, clock/reset programming, interrupt enable/handling,
DMA/cache, Ethernet, networking, SSH, storage, generated-root, broader PCIe
enumeration, phase transition, or real diagnostic behavior acceptance.

## Classification

Accepted as no-mmio-gpio-status-control-visible.

The accepted candidate rerun selected boot tree
99b1f8b2d7295da435bc82236fe0192f9dd9aade64e877508335cc2014d356fc with
effective kernel_2712.img and a 46,160-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 46,160-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
capture retained 795 occurrences of TALOS: gpio14-status-control.

The capture helper restored the candidate snapshot after the rerun; the worker
then performed a final restore to pre-run snapshot pre-gpio14-no-mmio-kg-021825,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

This accepts only the no-MMIO GPIO14 status control output-shape visibility and
proof-chain readiness for the queued real diagnostic proof. Real RP1 GPIO14
STATUS behavior, interrupt delivery, clock/reset programming, GPIO ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe behavior, and
phase transition remain unaccepted.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO control
  archive, including archive SHA-256, marker string, and forbidden RP1 GPIO
  diagnostic string absence.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had candidate-tied fetches and 794 control markers but was rejected by
  non-empty pre-power serial drain evidence.
- fixed: ran the required known-good control after the inconclusive first
  candidate run; it retained the production timer PASS marker and preserved the
  serial-drain rejection evidence.
- fixed: reran the selected candidate after the known-good control; the rerun
  passed the v2 identity join and retained repeated GPIO14 status control
  output.
- fixed: performed an explicit final restore to the pre-run boot tree after
  the rerun helper restored its candidate snapshot.
- deferred: the real RP1 GPIO14 STATUS diagnostic proof remains queued and must
  pass its own hardware lock, identity join, and classification gates.
- not-an-issue: no RP1 MMIO behavior is inferred from a no-MMIO simulated
  control.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-no-mmio-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-no-mmio-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-no-mmio-control-pi5/control-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-no-mmio-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-no-mmio-control-pi5/known-good-control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 46,160-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 795 occurrences of
  TALOS: gpio14-status-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-gpio-status-control-visible. The queued real RP1 GPIO14
STATUS diagnostic proof is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
