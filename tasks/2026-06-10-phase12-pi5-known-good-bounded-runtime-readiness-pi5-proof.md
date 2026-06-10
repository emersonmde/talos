# Phase 12 Pi 5 Known-Good Bounded Runtime Readiness Proof

Task id: phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof-20260610

## Goal

Run one serialized restored-known-good Pi 5 control through the repaired bounded
runtime-readiness helper and decide whether known-good Talos serial readiness
has recovered.

## Scope

- Used the accepted runtime-readiness helper saturation closeout as the
  authorization boundary.
- Acquired the hardware lock before the power cycle and released it after final
  status/files, readiness-helper output, and TFTP evidence were retained.
- Captured one shared run label:
  `known-good-runtime-readiness-20260610T215810Z`.
- Captured pre-power and final `GET /status`, `GET /boot/files`, snapshots,
  fresh serial/TFTP cursors, power-cycle response, repaired helper output,
  stable TFTP delta, and final TFTP tail.
- Did not publish or restore a boot archive and did not run GPIO32
  write/restore.

## Evidence Summary

- Pre-power and final boot identity matched the restored known-good tree:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- Pre-power and final effective/configured kernel were `kernel_2712.img`.
- The restored `da591740/kernel_2712.img` file was present with 104,136 bytes.
- Stable TFTP delta from cursor `4420008` contained 13 events, including two
  served `da591740/kernel_2712.img` fetches with 104,136 bytes.
- The primary helper summary generated immediately after the power cycle started
  from saturated serial cursor `4194304`, selected
  `deadline-loop-direct-read-after-saturated-cursor`, read 7,046 bytes, and
  saw `rpi5-production-timer-preemption: PASS` but not
  `TALOS: kernel_main`.
- The raw primary helper JSON was accidentally overwritten by a later follow-up
  direct read from the same saturated cursor; that follow-up read returned zero
  bytes. The derived primary summary is retained, but this run does not satisfy
  the raw-helper evidence gate for accepting readiness.

## Findings

- fixed: confirmed the repaired helper selected direct `/serial/read` instead
  of accepting an empty cursor-based observe window at the 4 MiB cursor
  saturation boundary.
- fixed: captured same-run identity across pre-power status/files, fresh
  cursors, power-cycle response, runtime-readiness helper output, stable TFTP
  delta, final status/files, and lock release.
- not-an-issue: boot publication was not the remaining blocker; pre/final
  identity matched and TFTP served the expected known-good kernel.
- deferred: the first helper summary showed the downstream production-timer PASS
  marker, but the raw helper JSON was overwritten by a follow-up read; this
  proof therefore records an evidence-retention blocker instead of accepting
  known-good runtime-readiness.
- deferred: GPIO32 write/restore v2 remains unauthorized until a later accepted
  closeout establishes valid known-good Talos readiness under the repaired
  helper contract.

## Classification

`known-good-readiness-raw-helper-artifact-overwritten`

This proof does not accept `valid-known-good-talos-readiness`. It records a
precise evidence-retention blocker: TFTP and boot identity recovered, and the
primary helper summary saw the downstream PASS marker, but the raw helper JSON
required to accept readiness was overwritten by a follow-up helper read.

Rejected claims:

- valid known-good Talos readiness
- GPIO32 write/restore v2 authorization
- PHY reset behavior
- Ethernet driver behavior
- packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition

## Validation

- serialized Pi 5 hardware proof through lab-controller API: completed
- repaired runtime-readiness helper:
  `scripts/rpi5-observe-runtime-readiness.sh 4194304 75 1000 65536` exited 1;
  the primary derived summary classified
  `known-good-fetch-observed-without-talos-readiness`, and the overwritten
  follow-up raw JSON classified `saturated-cursor-capture-blocked`
- stable TFTP delta:
  `scripts/rpi5-wait-tftp-delta.sh 4420008 90 3` exited 0 with stable evidence
- `jq empty` on task-owned JSON: passed
- `git diff --check`: passed
- `/home/node/.cargo/bin/mdbook build`: not required; no `docs/src` files
  touched
- `git diff --cached --check`: passed

## Evidence

- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/classification.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/evidence-map.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/pre-power-status.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/pre-power-boot-files.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/fresh-cursors.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/power-cycle-response.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/runtime-readiness.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/runtime-readiness-primary-summary.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/tftp-delta.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/final-status.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/final-boot-files.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/final-tftp-tail.json`

Next action: run the explicit closeout task. Do not promote GPIO32 v2 from this
proof; this run did not accept valid known-good Talos readiness.
