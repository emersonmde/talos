# Phase 10 Pi 5 Command0 Timeout-Stable Command Index Core

Task id: phase10-pi5-command0-timeout-stable-command-index-core-20260618

Status: accepted

Classification:
command0-timeout-stable-command-index-core-local-source

Evidence level: source implementation, local unit tests, QEMU command-loop
smoke, task-owned JSON evidence, docs build, and diff checks. No Pi 5
hardware run, lab mutation, boot archive publication, hardwareTestLock
acquisition, source-response retention proof, generated-root command-input
success claim, storage, networking, SSH, Phase 11/12 expansion, or phase
transition was performed.

## Goal

Make an empty no-data timeout before the command0 write non-terminal for the
Pi 5 generated-root command-input proof, while preserving normal descriptor
backed command dispatch and a bounded proof-visible failure mode.

## Result

The Pi 5 local serial command-loop proof now treats an empty no-data timeout at
command=0 as a bounded hold instead of a terminal command0 dispatch. The hold
does not advance the proof command index and emits a proof-visible
`timeout-hold command=0` line that includes the hold count, limit, status,
raw-byte count, pending marker, and `timeout-stable-command-index` source. If
the write never arrives, the fourth hold emits `timeout-hold-exhausted` and
fails the proof rather than hanging silently.

Normal complete command input is unchanged: completed lines still dispatch
through descriptor-backed stdin/stdout and advance the command index exactly
once. Non-command0 timeouts also remain terminal cycle results.

The selected next task is
phase10-pi5-command0-timeout-stable-command-index-pi5-proof-20260618.

## Findings

- fixed: changed the Pi 5 proof loop from unconditional `for` indexing to
  explicit command-index advancement so command0 can remain pending across
  empty no-data timeouts.
- fixed: added `LocalCommandCycleResult::is_no_data_timeout()` so the hold
  path is keyed to the exact empty timeout shape: no line bytes, no raw input,
  and `input-error timeout`.
- fixed: added proof-visible timeout-hold and timeout-hold-exhausted markers
  for the later serialized hardware task to distinguish a held command0
  readiness window from stale pre-write output.
- fixed: added a hard command0 hold limit to avoid an unbounded silent wait in
  proof paths.
- not-an-issue: generic command parsing and normal descriptor-backed dispatch
  remain unchanged; completed input still advances once and QEMU command-loop
  smoke still passes.
- deferred: serialized Pi 5 command0 delivery evidence remains owned by the
  queued hardware proof task.

## Evidence

- Source changes:
  - src/local_command_loop.rs
  - src/target/rpi5.rs
- Classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-core/evidence-map.json.

## Acceptance Check

- Empty no-data timeout before command0 write is not a terminal command0
  dispatch and does not advance the Pi 5 proof command index: satisfied by the
  command0 timeout-hold path and explicit index advancement only after
  non-held cycles.
- Implementation records proof-visible output/source evidence for the later
  Pi 5 task to distinguish a held command0 readiness window from stale
  pre-write output: satisfied by `timeout-hold command=0 ... pending=true
  source=timeout-stable-command-index`.
- Normal complete command input still dispatches through descriptor-backed
  stdin/stdout and advances exactly once: satisfied by unchanged generic
  dispatch and passing QEMU serial command-loop smoke.
- Timeout handling remains bounded and testable and does not introduce an
  unbounded silent hang: satisfied by `COMMAND0_TIMEOUT_HOLD_LIMIT=4` and the
  local no-data-timeout unit assertion.
- selected_next_task is
  phase10-pi5-command0-timeout-stable-command-index-pi5-proof-20260618:
  satisfied.

## Validation

- cargo fmt --all -- --check: pass after rustfmt.
- cargo -Zjson-target-spec test --quiet: pass.
- QEMU/substitute local serial command loop smoke:
  scripts/qemu-local-serial-command-loop-smoke.sh pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-timeout-stable-command-index-pi5-proof-20260618 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and the repository
has no conflicting uncommitted changes. Do not claim command0 input delivery,
source-response retention, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition from this
local/source task.
