# Phase 10 Pi 5 Serial Command 0 Write Delivery Guard Core

Task id: phase10-pi5-serial-command0-write-delivery-guard-core-20260617

Status: accepted

Classification:
serial-command0-write-delivery-guard-core-local-static

Evidence level: static source/task inspection, shell syntax check, local/static
proof-helper fixtures, task-owned JSON evidence, docs build, and diff checks.
No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, kernel command-loop source change, persistence,
storage work, networking, SSH, Phase 11/12 expansion, or phase transition was
performed.

## Goal

Implement the bounded local/static discriminator selected by the command 0
write-delivery source contract.

## Implementation

Updated scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh
to add command0-write-delivery-guard-v1 as a distinct evidence gate after
serial-capture-readiness-guard-v1 and before any later
command0-source-response-retention-guard-v2 claim.

The new guard accepts only an ordered command 0 write-delivery transaction:

- same-boot firmware-initramfs valid-artifact readiness with ready command=0
  and a visible prompt;
- fresh command 0 pre-write read after that prompt;
- accepted 9-byte /serial/write text=rootinfo append_newline=true;
- post-write retained rootinfo or command 0 line evidence;
- dispatch command=0 status=handled;
- responses=1;
- ready command=1 after dispatch.

The guard deliberately does not require the generated-root source response.
That response remains the separate source-response-retention gate. A write
delivery pass therefore authorizes only the serialized Pi 5 write-delivery
proof; it does not accept generated-root command input.

## Local Static Evidence

Positive fixture:
tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/command0-write-delivery-positive.json.

Positive review:
tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/command0-write-delivery-positive-review.json.

Negative results:
tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/command0-write-delivery-negative-results.txt.

The helper rejects:

- write-accepted-only evidence;
- prompt-only evidence;
- dispatch-only metadata when delivery evidence is required;
- unordered command 0 line/dispatch/ready output;
- stale later-command readiness;
- source-response-only evidence without command 0 write-delivery evidence.

## Findings

- fixed: command 0 write delivery is now checked separately from command 0
  source-response retention.
- fixed: task-owned fixtures prove the positive ordered line/dispatch/ready
  shape and reject write-accepted-only, prompt-only, dispatch-only, unordered,
  stale-readiness, and source-response-only shapes.
- fixed: the proof-review output now names
  command0-write-delivery-guard-v1 and selects
  phase10-pi5-serial-command0-write-delivery-pi5-proof-20260617.
- not-an-issue: kernel command-loop source already emits the required
  line/dispatch/ready markers; this task did not need a target source change.
- deferred: Pi 5 hardware proof remains dependency-gated behind this accepted
  local/static core.
- rejected: command0 source-response retention success, generated-root
  command-input success, storage, networking, SSH, Phase 11/12 expansion, and
  phase transition.

## Changed Files

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh.
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md.
- docs/src/roadmap.md.
- tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core.md.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/classification.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/evidence-map.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/command0-write-delivery-positive.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/command0-write-delivery-positive-review.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/write-accepted-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/prompt-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/dispatch-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/unordered-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/stale-readiness-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/source-response-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/command0-write-delivery-negative-results.txt.

## Acceptance Check

- Selected discriminator is implemented without broadening the user-visible
  shell feature surface: satisfied.
- Local/static validation rejects weak shapes: satisfied by task-owned negative
  fixtures for write-accepted-only, prompt-only, dispatch-only, unordered,
  stale-readiness, and source-response-only evidence.
- Proof surface can classify accepted, blocked, or inconclusive write-delivery
  outcomes before code changes: satisfied by command0-write-delivery-guard-v1
  terminal classifications in the helper review output.
- selected_next_task is
  phase10-pi5-serial-command0-write-delivery-pi5-proof-20260617: satisfied.
- Rejected claims include command0 source-response retention success,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition: satisfied.

## Validation

- sh -n scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh:
  pass.
- task-owned local/static validator positive fixture: pass.
- task-owned local/static validator negative fixtures: write-accepted-only,
  prompt-only, dispatch-only, unordered, stale-readiness, and
  source-response-only rejected.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-write-delivery-pi5-proof-20260617 on the
next worker wake if dependencies remain satisfied, the repository is clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
The next task must serialize hardware access and classify only command 0 write
delivery or the first failing invariant; it must not accept command0
source-response retention or generated-root command-input success.
