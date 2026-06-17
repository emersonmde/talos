# Phase 10 Pi 5 Serial Command 0 Saturated-Capture Guard Core

Task id: phase10-pi5-serial-command0-saturated-capture-guard-core-20260617

Status: accepted

Classification:
command0-saturated-capture-guard-core-local-static

Evidence level: shell helper syntax check, task-owned positive and negative
fixture checks, retained-blocker replay, task-owned JSON evidence, docs build,
and diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, command0 write-delivery success,
command0 source-response retention acceptance, generated-root command-input
acceptance, storage, networking, SSH, Phase 11/12 expansion, or phase
transition was performed.

## Goal

Implement the bounded local/static guard selected by the saturated-capture
source contract: after the command0 post-write observe proof hit the saturated
serial cursor boundary, a future Pi 5 proof may use only the documented
deadline-loop direct-read fallback when it is tied to selected-kernel/TFTP
agreement, fresh pre-write state, the accepted rootinfo write, and ordered
command0 output.

## Implementation

Added scripts/rpi5-command0-saturated-capture-proof-review.sh. With no
argument it emits the command0-saturated-capture-guard-v1 evidence contract,
including terminal classifications and the selected hardware follow-up. With a
task-owned evidence JSON argument it validates the retained saturated-capture
transaction.

The helper accepts only evidence that has:

- selected-kernel/TFTP agreement for the same 208984-byte
  da591740/kernel_2712.img candidate.
- same-boot generated-root readiness with source=firmware-initramfs,
  reason=valid-artifact, ready command=0, and a visible prompt.
- a fresh pre-write direct-read boundary that has not already retained
  rootinfo, command0 dispatch/output, later-command readiness, or generated-root
  source-response output.
- accepted /serial/write text=rootinfo append_newline=true bytes=9.
- a post-write direct-read window labeled
  deadline-loop-direct-read-after-saturated-cursor from saturated cursor
  4194304.
- ordered command0 rootinfo or line marker, dispatch command=0 status=handled,
  responses=1, and ready command=1.

The helper rejects empty saturated capture, /serial/write byte acceptance alone,
prompt-only evidence, stale pre-write output, stale later-command-only output,
unordered command0 fragments, and source-response-only evidence. The retained
post-write observe proof and retained v2 direct-read blocker both remain
rejected by this guard.

## Findings

- fixed: implemented command0-saturated-capture-guard-v1 as a narrow helper
  rather than reclassifying the prior blocked direct-read or observe evidence.
- fixed: positive fixture proves the selected evaluable shape with
  selected-kernel/TFTP agreement and an ordered saturated direct-read command0
  transaction.
- fixed: negative fixtures reject empty saturated capture, write-only,
  prompt-only, stale pre-write, stale later-command-only, unordered, and
  source-response-only evidence shapes.
- fixed: retained prior blocked post-write observe and v2 direct-read evidence
  both reject under the new guard.
- deferred: serialized Pi 5 hardware proof remains dependency-gated behind
  this accepted local/static core.
- not-an-issue: no Rust source or kernel command-loop source changes were
  required by this guard/core task.
- rejected: command0 write-delivery success, command0 source-response
  retention success, generated-root command-input success, storage, networking,
  SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Source contract:
  tasks/2026-06-17-phase10-pi5-serial-command0-saturated-capture-source-contract.md.
- Helper:
  scripts/rpi5-command0-saturated-capture-proof-review.sh.
- Contract output:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-guard-core/helper-contract-output.json.
- Positive fixture and output:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-guard-core/fixtures/positive-command0-saturated-capture.json,
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-guard-core/positive-output.json.
- Negative and retained-blocker fixture summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-guard-core/fixture-results.json.
- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-guard-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-guard-core/evidence-map.json.

## Acceptance Check

- The guard/helper encodes the accepted saturated-capture contract
  mechanically: satisfied by command0-saturated-capture-guard-v1.
- Positive fixture covers the selected evaluable command0 write-delivery shape:
  satisfied.
- Negative fixtures reject empty saturated observe/direct-read, write-only,
  prompt-only, stale pre-write, stale later-command-only, unordered, and
  source-response-only shapes: satisfied.
- Retained prior blocked evidence remains rejected: satisfied by replay of the
  post-write observe and v2 direct-read blocker evidence.
- selected_next_task is
  phase10-pi5-serial-command0-saturated-capture-pi5-proof-20260617: satisfied.

## Validation

- sh -n scripts/rpi5-command0-saturated-capture-proof-review.sh: pass.
- task-owned positive fixture replay: pass.
- task-owned negative fixture replay: empty saturated capture, write-only,
  prompt-only, stale pre-write, stale later-command-only, unordered, and
  source-response-only rejected.
- retained blocked evidence replay: prior post-write observe and v2 direct-read
  blocker rejected.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-saturated-capture-pi5-proof-20260617 on the
next worker wake if dependencies remain satisfied, the repository is clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
The next task must serialize hardware access and classify only the accepted
saturated-capture command0 write-delivery proof, a precise blocker, or an
inconclusive-run triage result. It must not accept command0 source-response
retention, generated-root command-input success, storage, networking, SSH,
Phase 11/12 expansion, or a phase transition.
