# Phase 10 Pi 5 Command0 Post-Write Capture-Retention Helper Core

Task id: phase10-pi5-command0-post-write-capture-retention-helper-core-20260618

Status: accepted

Classification:
command0-post-write-capture-retention-helper-core-local-static

Evidence level: local/static helper implementation, shell syntax check, positive
and negative fixture replay, task-owned JSON evidence, and diff checks. No Pi 5
hardware run, lab mutation, boot archive publication, hardwareTestLock
acquisition, runtime shell feature expansion, source-response retention proof,
generated-root command-input success claim, storage, networking, SSH, Phase
11/12 expansion, or phase transition was performed.

## Goal

Implement the local/static helper and validator surface for the
post-write capture-retention proof selected by the accepted source contract.

## Result

Added
`scripts/rpi5-command0-post-write-capture-retention-discriminator.sh`.
The helper accepts only same-attempt evidence with selected-kernel/TFTP
identity, a fresh command=0 readiness boundary, immediate rootinfo serial
write tied to the saved boundary cursor, bounded post-write capture from that
cursor or direct-read equivalent, ordered command0 output before command
advancement beyond 1, immediate/final selected identity, and restore proof.

Fixture replay passed 10/10 expectations. The two positive fixtures cover
saved-cursor observe and saturated-cursor direct-read equivalent evidence. The
negative fixtures reject byte-ack-only evidence, empty/two-byte post-write
capture, stale pre-write output, unordered output, timeout/command advancement,
source-response-only evidence, missing selected identity, and missing restore
proof.

The selected next task is
phase10-pi5-command0-post-write-capture-retention-pi5-proof-20260618.

## Findings

- fixed: added a dedicated post-write capture-retention discriminator instead
  of reusing the live-write helper that cannot distinguish byte acknowledgement
  from retained post-write command0 output.
- fixed: encoded terminal classifications for accepted, blocked, and
  inconclusive proof outcomes.
- fixed: replayed saved-cursor and direct-read-equivalent positive fixtures.
- fixed: replayed negative fixtures for byte-ack-only, two-byte/empty capture,
  stale output, unordered output, timeout/advancement, source-response-only,
  missing selected identity, and missing restore proof.
- deferred: serialized Pi 5 proof remains dependency-gated behind this
  accepted helper-core task.
- not-an-issue: no docs/src update was required because the roadmap frontier
  and selected next task already matched the accepted source contract.

## Evidence

- Helper:
  scripts/rpi5-command0-post-write-capture-retention-discriminator.sh.
- Proof contract:
  tasks/evidence/2026-06-18-phase10-pi5-command0-post-write-capture-retention-helper-core/proof-contract.json.
- Fixture replay summary:
  tasks/evidence/2026-06-18-phase10-pi5-command0-post-write-capture-retention-helper-core/results/fixture-replay-summary.json.
- Fixture replay log:
  tasks/evidence/2026-06-18-phase10-pi5-command0-post-write-capture-retention-helper-core/results/fixture-replay-results.txt.
- Classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-post-write-capture-retention-helper-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-post-write-capture-retention-helper-core/evidence-map.json.

## Acceptance Check

- Local/static fixture replay accepts only ordered command0 delivery after the
  selected fresh-boundary/write contract: satisfied by positive saved-cursor
  and direct-read-equivalent fixtures.
- Negative fixtures reject byte-ack-only, empty/two-byte post-write capture,
  stale output, unordered output, timeout/command advancement,
  source-response-only, missing selected identity, and missing restore proof:
  satisfied.
- The helper emits explicit accepted, blocked, and inconclusive terminal
  classifications: satisfied.
- selected_next_task is
  phase10-pi5-command0-post-write-capture-retention-pi5-proof-20260618:
  satisfied.

## Validation

- shell syntax check for changed shell script: pass.
- positive and negative fixture replay for helper/validator: pass, 10/10.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-post-write-capture-retention-pi5-proof-20260618 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and the repository
has no conflicting uncommitted changes. Do not claim command0 input delivery,
source-response retention, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition from this
local/static helper-core task.
