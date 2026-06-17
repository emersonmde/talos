# Phase 10 Pi 5 Serial Command 0 Post-Write Observe Helper Core

Task id: phase10-pi5-serial-command0-post-write-observe-helper-core-20260617

Status: accepted

Classification:
command0-post-write-observe-helper-core-local-static

Evidence level: shell helper syntax check, task-owned positive and negative
fixture checks, task-owned JSON evidence, and diff checks. No Rust source,
docs/src files, Pi 5 hardware run, lab mutation, boot archive publication,
hardwareTestLock acquisition, command0 source-response retention acceptance,
generated-root command-input acceptance, storage, networking, SSH, Phase 11/12
expansion, or phase transition was performed.

## Goal

Implement the helper/core discriminator selected by the accepted post-write
observe contract: after a visible command=0 prompt and accepted rootinfo write,
the Pi 5 proof must retain command0 output with a cursor-bound
POST /serial/observe window before command0 write-delivery can be accepted.

## Implementation

Added scripts/rpi5-command0-post-write-observe-proof-review.sh. With no
argument it emits the command0-post-write-observe-guard-v1 proof contract,
including terminal classifications for accepted, blocked, and inconclusive
outcomes. With a task-owned evidence JSON argument it validates the retained
post-write observe transaction.

The helper accepts only evidence that has:

- same-boot generated-root readiness and visible command=0 prompt.
- a fresh pre-write boundary that has not already retained rootinfo, command0
  dispatch, responses=1, or ready command=1.
- accepted /serial/write text=rootinfo append_newline=true bytes=9.
- cursor-bound /serial/observe output after the saved pre-write cursor that
  retains rootinfo or the command0 line marker, dispatch command=0
  status=handled, responses=1, and ready command=1 in order.

The helper rejects /serial/write byte acceptance alone, empty observe windows,
stale pre-write output, unordered command0 output, stale later-command
readiness, and source-response-only evidence.

For the selected Pi 5 proof follow-up, the helper contract carries forward the
accepted v2 candidate identity so the hardware dependency is explicit:
selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212,
expected fetch da591740/kernel_2712.img, and expected selected kernel size
208984 bytes. The proof must record the fresh command0 prompt/pre-write cursor,
post-write /serial/observe from that cursor, same-power-cycle selected-kernel
TFTP byte agreement, final pre-restore identity, and restore proof before
hardwareTestLock release.

## Findings

- fixed: implemented command0-post-write-observe-guard-v1 as a narrow helper
  rather than changing kernel behavior or retrying a direct-read transaction.
- fixed: positive fixture proves the accepted shape requires ordered command0
  line/rootinfo, dispatch, response count, and ready command=1 after the saved
  cursor.
- fixed: negative fixtures reject write-only, empty observe, stale pre-write,
  unordered, stale later-readiness, and source-response-only evidence shapes.
- fixed: accepted helper evidence names the selected Pi 5 proof candidate
  identity, pre-write cursor/observe requirements, selected-kernel/TFTP
  requirements, final identity requirements, and restore requirements.
- deferred: Pi 5 hardware proof remains selected for the next task.
- not-an-issue: no docs/src or Rust source changes were required by this
  helper/core task.

## Evidence

- Helper: scripts/rpi5-command0-post-write-observe-proof-review.sh.
- Contract output:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-helper-core/helper-contract-output.json.
- Positive fixture and output:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-helper-core/fixtures/positive-command0-post-write-observe.json,
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-helper-core/positive-output.json.
- Negative fixture summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-helper-core/fixture-results.json.
- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-helper-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-helper-core/evidence-map.json.

## Acceptance Check

- Proof-helper output names the new command0 post-write observe guard and
  terminal classifications: satisfied.
- Local/static fixtures accept only ordered command0/rootinfo, dispatch
  command=0 status=handled, responses=1, and ready command=1 after the saved
  pre-write cursor: satisfied.
- Negative fixtures reject write-only, empty observe, stale pre-write,
  unordered, stale later-command readiness, and source-response-only evidence:
  satisfied.
- selected_next_task is
  phase10-pi5-serial-command0-post-write-observe-pi5-proof-20260617:
  satisfied.
- Rejected claims remain explicit: satisfied.
- Accepted helper/core evidence names exact candidate identity and Pi 5 proof
  requirements needed by the queued proof dependency: satisfied.

## Validation

- sh -n on changed shell helper: pass.
- task-owned local/static positive and negative fixture checks: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet: not run; no Rust source or shared
  validator behavior touched.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; no docs/src files touched.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-post-write-observe-pi5-proof-20260617 on
the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention remains inactive, and the repository
has no conflicting uncommitted changes. Do not accept command0 source-response
retention, generated-root command-input success, storage, networking, SSH,
Phase 11/12 expansion, or a phase transition from this helper/core task.
