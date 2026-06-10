# Phase 12 Pi 5 Runtime Readiness Evidence Retention Closeout

Task id: phase12-pi5-runtime-readiness-evidence-retention-closeout-20260610

## Goal

Close out the runtime-readiness evidence-retention repair and decide whether a
bounded known-good Pi 5 readiness proof can be retried under the changed
artifact contract.

## Scope

- Consumed the accepted evidence-retention core commit
  884d760a066b84c6ea279b7c671873c2d5cba184.
- Inspected the core task record, classification/evidence JSON, retained-primary
  helper scripts, lab-controller contract text, and commit history.
- Confirmed the closeout can select the bounded known-good runtime-readiness v2
  proof as a changed-discriminator hardware retry.
- Did not run hardware, publish a boot archive, acquire the hardware lock,
  power cycle, write serial input, retry GPIO32, or change Talos runtime code.

## Evidence Summary

- The accepted core added
  `scripts/rpi5-retain-runtime-readiness-primary.sh`, which writes a
  run-label-qualified primary helper artifact, summary, and status file.
- The retained-primary wrapper refuses existing primary, summary, or status
  paths before invoking the helper and again before publishing temporary output.
- The derived summary records
  `derived_from_retained_primary_artifact: true` and names the retained primary
  JSON and status artifacts.
- The accepted guard
  `scripts/rpi5-runtime-readiness-retention-guard.sh` demonstrates that a
  second same-label retention attempt exits 3, leaves the first primary JSON and
  summary at `fixture-call-1`, and does not invoke the helper again.
- Lab-controller docs now require accepted known-good runtime-readiness proofs
  to retain the primary helper JSON through the wrapper; follow-up direct-read
  or endpoint checks must use separately named artifacts and cannot replace the
  primary readiness gate.

## Findings

- fixed: the closeout accepts the retained-primary runtime-readiness artifact
  contract from the core.
- fixed: same-run-label overwrite prevention is demonstrated by committed local
  guard evidence before any hardware retry.
- fixed: derived summaries consume the retained primary artifact and cannot be
  treated as a substitute for overwritten raw helper JSON.
- fixed: the next bounded readiness proof has a changed discriminator because it
  must use immutable primary artifacts rather than the prior mutable helper path.
- deferred: valid known-good Talos readiness remains unaccepted until the v2 Pi
  5 proof runs under hardwareTestLock and satisfies the repaired contract.
- deferred: GPIO32 write/restore v2 remains blocked until the v2 readiness
  closeout accepts `valid-known-good-talos-readiness`.
- not-an-issue: no further code or docs/src changes were required in this
  closeout; the accepted core already changed the helper contract and docs.

## Classification

pi5-runtime-readiness-primary-retention-closeout-accepted

This closeout accepts the local/static retained-primary artifact contract. It
does not accept `valid-known-good-talos-readiness`, GPIO32 write/restore
authorization, PHY reset behavior, Ethernet driver readiness, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

The selected next action is
phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof-20260610 because
the accepted retention contract changes the hardware-readiness discriminator
from a mutable raw helper path to an immutable run-label-qualified primary
artifact path.

Rejected claims:

- valid known-good Talos readiness
- GPIO32 write/restore v2 authorization
- PHY reset behavior
- Ethernet driver behavior
- packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition

## Validation

- static inspection of core task record, classification/evidence JSON, touched
  helper scripts, lab-controller docs, and commit history: completed
- jq empty on task-owned JSON: passed
- git diff --check: passed
- /home/node/.cargo/bin/mdbook build: not run; no docs/src files were touched by
  this closeout
- git diff --cached --check: passed

## Evidence

- tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-closeout/classification.json
- tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-closeout/evidence-map.json
- Source core record:
  tasks/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-core.md
- Source core classification:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-evidence-retention-core/classification.json

Next action: mechanically promote
phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof-20260610 on the
next worker wake if hardwareTestLock remains unlocked and supervisorIntervention
is inactive. Do not promote GPIO32 v2 from this closeout.
