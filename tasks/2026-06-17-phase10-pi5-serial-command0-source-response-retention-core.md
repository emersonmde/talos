# Phase 10 Pi 5 Serial Command 0 Source Response Retention Core

Task id: phase10-pi5-serial-command0-source-response-retention-core-20260617

Status: accepted

Classification:
serial-command0-source-response-retention-core-local-static

Evidence level: static source/task inspection, shell syntax check, local/static
proof-helper review, positive and negative command-0 source-response retention
fixtures, task-owned JSON evidence, docs build, and diff checks. No Pi 5
hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, kernel command-loop source change, persistence, storage work,
networking, SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Implement the selected source-contract remediation for command-0
source-response retention without changing the accepted feature boundary. The
feature under test remains Pi 5 serial shell command input against the
firmware-initramfs generated-root artifact.

## Implementation

Updated scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh
from its previous command0-write-to-next-ready guard to
command0-source-response-retention-guard-v2.

The helper now accepts task-owned command0-direct-read summary evidence as a
local/static validator input, in addition to the existing full
direct_read_proof shape. For command 0, it requires this ordered transaction:

- rootinfo, or the retained command-0 line hex marker;
- talos: generated-root source=firmware-initramfs;
- reason=valid-artifact;
- dispatch command=0 status=handled;
- responses=1;
- rpi5-generated-root-boot-transport-proof: ready command=1.

The selected source contract did not prove that kernel command-loop or Pi 5
target proof source needed to change. No kernel source was edited. This task
therefore keeps the remediation on the proof/capture/validation surface.

## Local Static Evidence

- Positive fixture:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-positive.json.
- Positive review:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-positive-review.json.
- Negative results:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-negative-results.json.

The positive fixture passes the new guard and selects
phase10-pi5-serial-command0-source-response-retention-pi5-proof-20260617 as the
next serialized hardware follow-up.

Negative controls all reject:

- tail-only: matches the previously retained failure shape, starting after the
  generated-root source prefix while still retaining line/dispatch/ready;
- dispatch-only: retains command text, dispatch, responses=1, and ready command
  1 but omits source=firmware-initramfs reason=valid-artifact;
- unordered: retains the source line after dispatch, proving ordered retention
  remains required.

## Findings

- fixed: command0-direct-read summary evidence can now be checked locally for
  the exact source-response retention invariant before any hardware rerun.
- fixed: the accepted previous tail-only retained text is mechanically rejected
  as insufficient even though it includes command-0 line, dispatch, responses=1,
  and ready command=1.
- fixed: dispatch-only and unordered source-response evidence are rejected.
- fixed: the helper records the command0-source-response-retention-guard-v2
  required ordered fragments and rejected evidence shapes.
- not-an-issue: kernel command-loop and target proof source already generate
  line, source response, dispatch, response count, and ready markers in the
  required order; the retained failure was in proof/capture retention.
- deferred: Pi 5 hardware proof remains dependency-gated behind this accepted
  local/static core.
- rejected: prompt-only, write-only, stale, dispatch-only, unordered,
  tail-only, persistence, storage drivers, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Changed Files

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh.
- docs/src/roadmap.md.
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md.
- tasks/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core.md.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/classification.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/evidence-map.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-positive.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-positive-review.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-tail-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-dispatch-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-unordered-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/command0-retention-negative-results.json.

## Acceptance Check

- Implementation matches the accepted source contract: satisfied; only the
  selected proof/helper surface and task-owned evidence/docs were changed.
- Local/static evidence proves the selected command-0 source-response retention
  guard: satisfied by command0-retention-positive-review.json.
- Negative controls reject stale/fragmentary shapes: satisfied for tail-only,
  dispatch-only, and unordered command-0 evidence; the helper retains earlier
  prompt-only, write-only, stale, dispatch-only, source-gate, TFTP, final
  identity, and restore rejections.
- Task-owned JSON records findings with disposition: satisfied in
  classification.json and evidence-map.json.
- Hardware follow-up authorization: selected_next_task is
  phase10-pi5-serial-command0-source-response-retention-pi5-proof-20260617.

## Validation

- sh -n scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh:
  pass.
- task-owned local/static validator positive fixture: pass.
- task-owned local/static validator negative fixtures: tail-only rejected,
  dispatch-only rejected, unordered rejected.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-source-response-retention-pi5-proof-20260617
on the next worker wake if dependencies remain satisfied, the repository is
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. The next task must serialize hardware access and retain command-0
source-response evidence under command0-source-response-retention-guard-v2.
