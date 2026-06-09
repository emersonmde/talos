# Phase 11 RP1 Observed GPIO14 Ownership/Route Control Pi 5

Task id: phase11-rp1-observed-gpio-ownership-route-control-pi5-20260608

Status: accepted

Classification: no-mmio-observed-gpio14-ownership-route-control-visible

## Goal

Prove the paired no-MMIO/no-RP1/no-GIC observed GPIO14 ownership/route
preflight control output shape on Pi 5 before the real read-only preflight.

## Scope

- Acquired the hardware lock for this task only.
- Added a task-owned optional capture nonce to the GPIO14 ownership/route
  preflight control marker, and taught the control image/review scripts to
  validate that nonce when supplied.
- Built and published the no-MMIO control archive with capture nonce
  orctrl20260609T001146Z-6b2be481.
- Captured a serialized Pi 5 proof bundle with selected candidate identity,
  fresh serial/TFTP cursors, stable TFTP delta, final pre-restore identity,
  v3/run-unique/boot-staging checker output, marker-visible control output,
  and restore proof.
- Accepted only the control capture path. No GPIO/RP1/GIC/PCIe hardware
  behavior is accepted by this task.

## Non-Goals

No real GPIO/RP1/GIC/PCIe/clock MMIO claim, GPIO write, event generation,
interrupt delivery, endpoint config retry, bridge setup write, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: the control image now supports a task-owned capture nonce so the
  saturated serial cursor path can use the accepted run-unique freshness
  discriminator.
- fixed: static archive review passed for the nonce-bearing no-MMIO control
  archive and retained no-forbidden-MMIO evidence.
- fixed: selected candidate identity before power matched tree
  88d0cb665e847257dee487caab5c30e525a6c764c96c8974f99c3d0b1fd72c35 with
  effective kernel kernel_2712.img and expected fetch
  da591740/kernel_2712.img at 48,528 bytes.
- fixed: pre-power serial drain saturated without an empty read, but v3
  freshness passed because the required capture nonce was absent before power
  and appeared 91 times after power.
- fixed: stable pre-restore TFTP evidence retained 13 events with two matching
  da591740/kernel_2712.img fetches of 48,528 bytes.
- fixed: final pre-restore identity still pointed at the selected tree with the
  expected fetch present at 48,528 bytes.
- fixed: restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: rpi5-proof-identity-join-v3-check.sh classified the bundle as
  capture-transaction-v3-ready with no rejection reasons.
- fixed: rpi5-proof-identity-join-run-unique-check.sh classified the bundle as
  capture-transaction-run-unique-ready with no rejection reasons.
- fixed: rpi5-boot-staging-identity-check.sh classified the bundle as
  boot-staging-identity-ready with no rejection reasons.
- not-an-issue: GET / on the deployed lab API returned the documented 404
  semantics; candidate identity was retained through /status and /boot/files.
- deferred: the real read-only GPIO14 ownership/route preflight remains a
  separate queued task.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/control-run/.
- V3 checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/v3-check.json.
- Run-unique checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/run-unique-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/boot-staging-identity-check.json.
- Restore proof:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5/control-run/post-restore-status.json.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 423 no_std
  tests.
- shell syntax: bash -n passed for the touched control image/review scripts.
- image/archive inspection: static review passed for the nonce-bearing
  no-MMIO control archive.
- lab-controller API: snapshot, archive publish, power-cycle, serial, TFTP,
  final identity, and restore records were captured.
- serial hardware boot/output: passed; the run-unique nonce marker was absent
  before power and present after power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 48,528-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- capture/staging checker output: v3, run-unique, and boot-staging identity
  checkers all passed with no rejection reasons.
- jq empty on evidence-map, classification, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.

No docs/src files were touched, so mdbook was not required.

## Result

Accepted as no-mmio-observed-gpio14-ownership-route-control-visible. This
accepts only the no-MMIO control output/capture path. It does not accept GPIO
ownership, event generation, interrupt delivery, broad RP1 mapping, DMA/cache,
networking, SSH, Milestone 11.3, or a phase transition.

## Next Action

The next queued worker task is
phase11-rp1-observed-gpio-ownership-route-pi5-20260608, mechanically unblocked
if this task is committed and hardwareTestLock remains unlocked/restored.
