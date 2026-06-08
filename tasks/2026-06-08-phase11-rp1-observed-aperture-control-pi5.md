# Phase 11 RP1 Observed Aperture Control Pi 5

Task id: phase11-rp1-observed-aperture-control-pi5-20260608

Status: accepted

Classification: no-mmio-observed-aperture-control-visible

## Goal

Prove the observed-aperture no-MMIO control output shape on the real Pi 5
before the real observed-aperture candidate.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-PCIe/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-observed-aperture-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first candidate
  capture: candidate identity, fresh serial/TFTP evidence, known-good control,
  and an unchanged candidate rerun.

## Non-Goals

No real observed-aperture read, endpoint config retry, same-shaped 0x1f RP1
read rerun, same-shaped bridge/setup rerun, BAR discovery or programming,
bridge setup writes, PERST/link-control changes, GPIO/pad/clock/reset writes,
interrupt enablement or delivery, GIC acknowledgement, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Classification

Accepted as no-mmio-observed-aperture-control-visible.

The accepted unchanged candidate rerun selected boot tree
5e646c783c95672121fc135a41acc5e0c7e87424ea4694b8e952fa5c8b22ece2 with
effective kernel_2712.img and a 47,344-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
47,344-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 72 task-owned occurrences of
TALOS: rp1-observed-aperture-control and the terminal classification
no-mmio-observed-aperture-control-visible. The control report preserves the
accepted contract fields with source-rp1-bus-address, observed-cpu-physical-address,
and register-offset set to not-constructed; reports raw=0x90 and
raw-is-pl011-fr-shaped=true; preserves the retained bridge/setup mismatch
fields; and emits no real observed-aperture result marker.

This accepts only the no-MMIO/no-PCIe/no-RP1/no-GIC output shape and capture
path for the queued real observed-aperture proof. Real 0x1c00030018 reads,
live RP1 aperture visibility, endpoint ownership, broad RP1 mapping,
interrupt delivery, GPIO/clock ownership, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, and phase transition remain unaccepted.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO control
  archive, including archive SHA-256, kernel SHA-256, kernel size, and control
  marker.
- fixed: retained the first candidate capture as capture-staging-blocked
  evidence; it had serial marker output, but pre-power serial drain was not
  empty and stable pre-restore TFTP had no expected candidate fetches, so no
  decisive control classification was taken from that run.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate evidence. It passed the v2 identity join with two
  served 104,136-byte known-good kernel fetches and retained PASS output.
- fixed: reran the unchanged no-MMIO observed-aperture control candidate after
  the known-good control; the rerun passed the v2 identity join and retained
  repeated observed-aperture control output.
- deferred: the real observed-aperture proof remains queued and must pass its
  own hardware lock, identity join, restore, and classification gates.
- not-an-issue: no real RP1 aperture, endpoint, interrupt, GPIO, clock/reset,
  or DMA behavior is inferred from a no-MMIO simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-control-pi5/control-rerun-after-kg/.
- Initial candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-control-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,344-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 72 task-owned occurrences of
  TALOS: rp1-observed-aperture-control were retained.
- known-good control and unchanged candidate rerun after inconclusive evidence:
  run and retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-observed-aperture-control-visible. The queued real
observed-aperture proof is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
