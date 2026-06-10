# Phase 12 Pi 5 Runtime Readiness Evidence Retention Core

Task id: phase12-pi5-runtime-readiness-evidence-retention-core-20260610

## Goal

Repair the runtime-readiness evidence-retention path so a primary helper
artifact from a restored known-good Pi 5 run cannot be overwritten by follow-up
reads before any GPIO32 retry.

## Scope

- Consumed the accepted serial endpoint follow-up classification
  `serial-endpoints-readable-current-device-buffer-empty`.
- Inspected the known-good bounded runtime-readiness proof blocker, closeout,
  serial endpoint follow-up, runtime-readiness helper, and lab-controller proof
  contract.
- Added a source/script retention wrapper for the runtime-readiness helper.
- Added a local guard that demonstrates same-run-label overwrite attempts are
  rejected before the helper can replace the retained primary artifact.
- Updated lab-controller docs to require retained-primary helper artifacts for
  accepted known-good runtime-readiness evidence.
- Did not run hardware, publish a boot archive, acquire the hardware lock,
  power cycle, write serial input, retry GPIO32, or change Talos runtime code.

## Evidence Summary

- `scripts/rpi5-retain-runtime-readiness-primary.sh` writes helper output to
  `<run-label>-runtime-readiness-primary.json`, derives
  `<run-label>-runtime-readiness-primary-summary.json` from that retained
  primary JSON, writes `<run-label>-runtime-readiness-primary.status`, and
  refuses to overwrite any of those paths.
- The wrapper preserves the helper exit code while retaining JSON and status,
  so failed readiness attempts still leave durable primary evidence.
- Follow-up direct-read or endpoint discriminator artifacts are explicitly
  outside the primary path and cannot satisfy or erase the primary readiness
  gate.
- `scripts/rpi5-runtime-readiness-retention-guard.sh` uses a fixture helper
  to prove the first primary artifact remains `fixture-call-1`, a second
  same-label run exits 3, and the helper invocation count stays 1.
- The lab-controller proof contract now points hardware proofs at the retained
  primary wrapper instead of a mutable latest-style `runtime-readiness.json`
  path.

## Findings

- fixed: primary runtime-readiness helper output now has a run-label-qualified
  artifact path and refuses same-label overwrites.
- fixed: derived summaries now identify and consume the retained primary
  artifact rather than a mutable helper output path.
- fixed: follow-up direct-read or endpoint discriminator evidence is documented
  as separately named evidence that cannot replace the primary readiness gate.
- fixed: the focused local guard reproduces and prevents the overwrite class by
  rejecting a second same-label retention attempt before helper invocation.
- not-an-issue: the underlying runtime-readiness helper marker and saturated
  cursor contracts did not need to change for this task.
- deferred: a serialized Pi 5 known-good runtime-readiness v2 proof remains
  required before GPIO32 write/restore v2 can be reconsidered.

## Classification

`pi5-runtime-readiness-primary-retention-core-accepted`

This task accepts a local/static source and evidence-workflow repair. It does
not accept `valid-known-good-talos-readiness`, GPIO32 authorization, PHY
reset behavior, Ethernet behavior, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## Validation

- static inspection of accepted serial endpoint follow-up, known-good
  runtime-readiness proof blocker, closeout, and touched scripts/docs:
  completed
- shell syntax: `sh -n scripts/rpi5-observe-runtime-readiness.sh scripts/rpi5-retain-runtime-readiness-primary.sh scripts/rpi5-runtime-readiness-retention-guard.sh`
  passed
- focused local retention guard:
  `TMPDIR=/opt/strider/openclaw/current/workspace/tmp scripts/rpi5-runtime-readiness-retention-guard.sh`
  passed
- `jq empty` on task-owned JSON: passed
- `git diff --check`: passed
- `/home/node/.cargo/bin/mdbook build`: passed
- `git diff --cached --check`: passed

## Evidence

- `tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-core/classification.json`
- `tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-core/evidence-map.json`
- `tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-core/retention-guard-output.json`
- `tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-core/sh-n-runtime-readiness-retention.log`

Next action: promote
phase12-pi5-runtime-readiness-evidence-retention-closeout-20260610 after this
task is accepted and committed. Do not run hardware or promote GPIO32 from
this task.
