# Phase 11 RP1 Observed GPIO16 Ownership/Event Control Pi 5 Retry

Task id: phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry-20260609

Status: accepted

Classification: no-mmio-observed-gpio16-ownership-event-control-visible

## Goal

Retry the paired no-MMIO/no-RP1/no-GIC observed GPIO16 ownership/event
preflight control proof on Pi 5 after the run-unique serial visibility
discriminator repair.

## Scope

- Acquired the hardware lock for this task only.
- Rebuilt and static-reviewed the GPIO16 no-MMIO control archive with a fresh
  task-owned capture nonce.
- Published the control archive, captured candidate identity, serial drain and
  post-power serial visibility, stable TFTP delta, final identity, checker
  output, and restore proof.
- Accepted only the control output-shape proof and capture-chain evidence; no
  GPIO/RP1/GIC/PCIe hardware behavior is accepted from this control.

## Findings And Disposition

- fixed: the retried control archive includes the task nonce
  gpio16ctlretry20260609T045148Z-5781e788 in the control marker.
- fixed: static archive review passed and found no forbidden GPIO16/RP1/GIC/
  PCIe/MMIO address strings in the no-MMIO control image.
- fixed: selected candidate identity before power matched tree
  cdb35bef8b7fbd5b68df9c76a58fbb410e20522d46aed6b77319002b0be6bd19,
  effective kernel kernel_2712.img, and expected
  da591740/kernel_2712.img size 48,744 bytes.
- fixed: TFTP evidence retained two matching served fetches for
  da591740/kernel_2712.img, each with 48,744 bytes.
- fixed: run-unique serial visibility passed: the nonce token was absent before
  power and appeared 42 times after power.
- fixed: final pre-restore identity still pointed at the selected tree, and
  restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: boot-staging identity checker classified the retained bundle as
  boot-staging-identity-ready.
- not-an-issue: the legacy v2 summary still reports
  serial-drain-not-empty-before-power; the accepted run-unique checker is the
  task-owned discriminator for saturated serial because it proves nonce absence
  before power and nonce presence after power.

No findings were removed or deferred.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/classification.json.
- Static archive review:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/control-retry-run/.
- Run-unique checker:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/run-unique-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/boot-staging-identity-check.json.
- Restore proof:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/control-retry-run/restore-snapshot.json and
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry/control-retry-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the nonce-bearing
  no-MMIO control archive.
- lab-controller API: snapshot, archive publish, power-cycle, serial, TFTP,
  final identity, and restore records were captured.
- serial hardware boot/output: passed under the run-unique discriminator;
  required nonce token was absent before power and present after power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 48,744-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- run-unique checker: passed; classification capture-transaction-run-unique-ready.
- boot-staging identity checker: passed; classification boot-staging-identity-ready.
- jq empty on classification, evidence map, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No docs/src files were touched, so mdbook is not required.

## Result

Accepted as no-mmio-observed-gpio16-ownership-event-control-visible. This
unblocks the queued real GPIO16 read-only preflight task, subject to the
hardware lock remaining unlocked/restored.

## Next Action

Next queued task phase11-rp1-observed-gpio16-ownership-event-pi5-20260609 is
mechanically unblocked on the next worker wake if hardwareTestLock remains
unlocked/restored.
