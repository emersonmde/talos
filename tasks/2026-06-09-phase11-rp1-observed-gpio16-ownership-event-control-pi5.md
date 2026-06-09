# Phase 11 RP1 Observed GPIO16 Ownership/Event Control Pi 5

Task id: phase11-rp1-observed-gpio16-ownership-event-control-pi5-20260609

Status: completed-blocker

Classification: capture-staging-blocked

## Goal

Prove the paired no-MMIO/no-RP1/no-GIC observed GPIO16 ownership/event
preflight control output shape on Pi 5 before the real preflight.

## Scope

- Acquired the hardware lock for this task only.
- Added the task-owned capture nonce to the GPIO16 no-MMIO control marker so
  the accepted run-unique freshness checker can distinguish this boot from
  retained saturated serial.
- Built and static-reviewed only the GPIO16 no-MMIO control archive.
- Published the task-owned control archive, captured candidate identity, serial
  drain/window, TFTP delta, final identity, checker output, and restore proof.
- Accepted no GPIO/RP1/GIC/PCIe hardware behavior from this control run.

## Findings And Disposition

- fixed: the GPIO16 no-MMIO control marker now includes
  TALOS_CAPTURE_NONCE when provided, matching the run-unique freshness
  procedure required for Pi 5 capture tasks.
- fixed: static archive review now accepts an optional --capture-nonce gate and
  fails if the nonce-bearing marker is absent from the control image.
- fixed: candidate identity before power matched selected tree
  56d2c8171b5424a77358c4732238161bcd12f68739a54993e9af9d00cc1996fb with
  effective kernel kernel_2712.img and expected
  da591740/kernel_2712.img size 48,744 bytes.
- fixed: TFTP delta was stable and retained two matching
  da591740/kernel_2712.img fetches of 48,744 bytes.
- fixed: final pre-restore identity still pointed at the selected tree, and
  restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: boot-staging identity checker classified the retained bundle as
  boot-staging-identity-ready with no rejection reasons.
- deferred: the no-MMIO control output was not accepted because the serial
  freshness checker rejected the run; the required GPIO16 control marker was
  absent after power.
- not-an-issue: the no-MMIO control archive constructs no forbidden RP1, GPIO,
  RIO, pad, MSI-X/PCIe/MIP, GIC, clock/reset, DMA, or other MMIO address.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/control-run/.
- Run-unique checker:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/run-unique-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/boot-staging-identity-check.json.
- Restore proof:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/control-run/restore-snapshot.json and
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/control-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the nonce-bearing
  no-MMIO control archive.
- lab-controller API: snapshot, archive publish, power-cycle, serial, TFTP,
  final identity, and restore records were captured.
- serial hardware boot/output: blocked; serial capture used saturated direct
  read, the nonce-bearing marker was absent before power and absent after
  power, and run-unique rejected the capture with
  required-marker-not-present-after-power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 48,744-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- run-unique checker: blocked; classification capture-staging-blocked.
- boot-staging identity checker: passed; classification
  boot-staging-identity-ready.
- jq empty on classification, evidence map, checker, and retained JSON
  artifacts: passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- bash -n on touched shell scripts: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No docs/src files were touched, so mdbook was not required.

## Result

Completed as a committed blocker: capture-staging-blocked. This does not accept
the GPIO16 no-MMIO control output shape on Pi 5 and does not unblock the real
GPIO16 preflight. It accepts only the retained staging/TFTP/final identity and
restore evidence, plus the local nonce/control-marker adjustment.

## Next Action

No mechanically unblocked worker-owned GPIO16 real preflight task remains,
because the queued real task depends on an accepted decisive no-MMIO control
proof. Supervisor planning is required to decide the next bounded capture or
implementation step.
