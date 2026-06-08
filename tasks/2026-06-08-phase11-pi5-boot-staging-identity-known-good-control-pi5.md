# Phase 11 Pi 5 Boot Staging Identity Known-Good Control

Task id: phase11-pi5-boot-staging-identity-known-good-control-pi5-20260608

Status: accepted

Classification: known-good-boot-staging-identity-control-ready

## Goal

Prove the repaired Pi 5 boot-staging identity procedure on a known-good
no-RP1/no-MMIO control before retrying observed GPIO14 STATUS/CTRL.

## Scope

- Acquired the hardware lock for this task only.
- Built and published the task-owned no-RP1/no-MMIO observed GPIO status
  control archive with capture nonce bti20260608T212440Z-122908e4.
- Captured a serialized Pi 5 proof bundle with selected candidate identity,
  fresh serial drain/cursor evidence, stable TFTP delta, final pre-restore
  identity, run-unique checker output, boot-staging identity checker output,
  and restore proof.
- Accepted only the staging/capture/control path; no RP1/GPIO hardware
  behavior is accepted by this task.

## Findings And Disposition

- fixed: selected candidate identity before power matched tree
  35a30932a7f8e76d8cfa657b7419ec1d5e7e8ce450c5ae898c32e957636734f1
  with effective kernel kernel_2712.img.
- fixed: pre-power serial drain reached empty-read-before-power after two
  attempts, then the saturated cursor capture used direct read as recorded by
  the capture helper.
- fixed: task-owned marker
  TALOS: rp1-observed-gpio-status-control capture-nonce=bti20260608T212440Z-122908e4
  appeared after power and was absent from the pre-power run-unique freshness
  gate.
- fixed: stable pre-restore TFTP evidence retained 13 events with two matching
  da591740/kernel_2712.img fetches of 49,072 bytes.
- fixed: final pre-restore identity still pointed at the selected tree with the
  expected fetch present at 49,072 bytes.
- fixed: restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: rpi5-proof-identity-join-run-unique-check.sh classified the bundle
  as capture-transaction-run-unique-ready with no rejection reasons.
- fixed: rpi5-boot-staging-identity-check.sh classified the bundle as
  boot-staging-identity-ready with no rejection reasons.
- not-an-issue: the control output is intentionally no-MMIO/no-RP1; it proves
  the repaired staging identity path, not GPIO14 STATUS/CTRL visibility.
- deferred: the real observed GPIO14 STATUS/CTRL retry remains a separate
  queued task.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/control-run/.
- Run-unique checker:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/run-unique-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/boot-staging-identity-check.json.
- Restore proof:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/control-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the nonce-bearing
  no-MMIO control archive.
- lab-controller API: snapshot, archive publish, power-cycle, serial, TFTP,
  final identity, and restore records were captured.
- serial hardware boot/output: passed; required task-owned nonce marker was
  visible after power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 49,072-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- repaired checker output: run-unique and boot-staging identity checkers both
  passed with no rejection reasons.
- jq empty on evidence-map, classification, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No docs/src files were touched, so mdbook was not required.

## Result

Accepted as known-good-boot-staging-identity-control-ready. This proves the
repaired boot-staging identity procedure on the no-RP1/no-MMIO control only; it
does not accept observed GPIO14 STATUS/CTRL visibility, GPIO ownership, event
generation, interrupt delivery, broad RP1 mapping, DMA/cache, networking, SSH,
Milestone 11.3, or a phase transition.
