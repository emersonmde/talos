# Phase 10 Pi 5 Serial Capture Readiness Guard Core

Task id: phase10-pi5-serial-capture-readiness-guard-core-20260617

Status: accepted

Classification:
serial-capture-readiness-guard-core-local-static

Evidence level: static/source/task evidence inspection, shell syntax check,
task-owned local/static validator output, task-owned JSON evidence, docs build,
and diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, persistence, storage work, networking, SSH, Phase
11/12 expansion, or phase transition was performed.

## Goal

Implement the accepted serial-capture-readiness source contract as a
local/static guard before any further hardware retry.

## Implementation

scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh now emits
and enforces serial-capture-readiness-guard-v1 before the existing
command0-source-response-retention-guard-v2 transaction gate.

For the full direct_read_proof shape, the guard requires:

- boot source=firmware-initramfs reason=valid-artifact plus selected-tree,
  stable TFTP, final identity, and restore proof fields;
- readiness text containing source=firmware-initramfs, reason=valid-artifact,
  ready command=0, and a visible talos> prompt;
- readiness.fresh_after_prompt=true when the field is recorded;
- a command 0 pre-write read with fresh_after_prompt=true that has not already
  retained rootinfo, the command-0 line marker, source/reason output, dispatch
  command=0, ready command=1, or the generated manifest output;
- the existing command0-source-response-retention-guard-v2 ordered transaction:
  rootinfo or command-0 line marker, generated-root source/reason response,
  dispatch command=0 status=handled, responses=1, and ready command=1.

The existing command0-direct-read summary path remains available for the
source-response-retention guard. That shape is not used to accept serial
readiness/capture quality because it lacks readiness and pre-write boundary
fields.

## Local Static Evidence

The task-owned positive fixture
tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/serial-capture-readiness-positive.json
passes the helper and produces
tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/serial-capture-readiness-positive-review.json.

Negative controls are recorded in
tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/serial-capture-readiness-negative-results.json.
The helper rejects:

- early-firmware-only capture before generated-root readiness;
- stale later-command readiness with input-error/ready command=3;
- dispatch-only command 0 metadata without generated-root source text;
- tail-only command 0 source response missing the source prefix or command 0
  line.

## Findings

- fixed: serial readiness/capture is now checked as a distinct local/static
  guard before command0 source-response retention is evaluated.
- fixed: early-firmware-only capture is rejected before it can masquerade as a
  command0-ready window.
- fixed: stale later-command readiness, including input-error/ready command=3,
  is rejected.
- fixed: dispatch-only metadata and tail-only source response remain rejected
  under the retained command0-source-response-retention-guard-v2 gate.
- deferred: Pi 5 hardware proof remains dependency-gated behind
  phase10-pi5-serial-capture-readiness-pi5-proof-20260617.
- not-an-issue: no kernel command-loop or lab-service change was required for
  this local/static discriminator.
- rejected: hardware success, generated-root command-input success, persistence,
  storage drivers, networking, SSH, Phase 11/12 expansion, and phase
  transition.

## Changed Files

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh.
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md.
- docs/src/roadmap.md.
- tasks/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core.md.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/classification.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/evidence-map.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/serial-capture-readiness-positive.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/serial-capture-readiness-positive-review.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/early-firmware-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/stale-later-readiness-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/dispatch-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/tail-only-source-response-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/*-negative-review.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/serial-capture-readiness-negative-results.json.

## Acceptance Check

- Implemented guard/discriminator directly targets the serial readiness/capture
  blocker from the accepted closeout: satisfied.
- Task-owned fixtures include a positive evaluable command0 window and negative
  controls for early-firmware-only capture, stale retained readiness,
  dispatch-only metadata, and tail-only source response: satisfied.
- Next Pi 5 proof contract includes candidate identity, fresh serial
  cursor/boundary, TFTP delta, known-good control before candidate rerun for
  inconclusive evidence, final identity, restore evidence, and explicit terminal
  classifications: satisfied by the dependency-gated
  phase10-pi5-serial-capture-readiness-pi5-proof-20260617 task.
- Rejected claims include hardware success, generated-root command-input
  success, persistence, storage, networking, SSH, Phase 11/12 expansion, and
  phase transition: satisfied.

## Validation

- sh -n scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh:
  pass.
- task-owned local/static validator positive fixture: pass.
- task-owned local/static validator negative fixtures: early-firmware-only,
  stale-later-readiness, dispatch-only, and tail-only-source-response rejected.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-capture-readiness-pi5-proof-20260617 on the next
worker wake if dependencies remain satisfied, the repository is clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
