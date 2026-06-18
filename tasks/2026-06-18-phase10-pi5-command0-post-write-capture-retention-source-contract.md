# Phase 10 Pi 5 Command0 Post-Write Capture-Retention Source Contract

Task id: phase10-pi5-command0-post-write-capture-retention-source-contract-20260618

Status: accepted

Classification:
command0-post-write-capture-retention-source-contract-helper-core-selected

Evidence level: static/source/task evidence inspection, accepted live
write-window proof evidence inspection, task-owned JSON evidence, and diff
checks. No Pi 5 hardware run, lab mutation, boot archive publication,
hardwareTestLock acquisition, runtime feature change, source-response retention
proof, generated-root command-input success claim, storage, networking, SSH,
Phase 11/12 expansion, or phase transition was performed.

## Goal

Define the smallest objective follow-up for the remaining command0 blocker:
after a fresh live command=0 boundary and immediate rootinfo write, the retained
post-write capture contained only two bytes instead of ordered command0 output.

## Result

The next objective step is a post-write capture-retention helper/core task, not
a supervisor intervention and not another timing-only retry. The retained live
write-window evidence already shows selected-kernel/TFTP agreement, a fresh
command=0 readiness boundary, immediate rootinfo write, stable immediate/final
selected identity, and restore proof. The first failing invariant is narrower:
bounded post-write capture from that same attempt did not retain rootinfo or a
line command=0 marker, dispatch command=0 status=handled, responses=1, and
ready command=1.

The selected helper-core follow-up must therefore validate a task-owned
contract for command0 post-write capture retention. It may accept only
same-attempt evidence with selected-kernel/TFTP identity, a fresh command=0
boundary, immediate rootinfo write, bounded post-write serial capture from the
saved boundary cursor or a direct-read equivalent when the retained cursor is
saturated, ordered command0 output before command advancement beyond 1,
immediate/final selected identity, and restore proof.

The selected next task is
phase10-pi5-command0-post-write-capture-retention-helper-core-20260618.

## Findings

- fixed: separated the remaining command0 blocker from selected-kernel/TFTP,
  freshness, write timing, identity, and restore, all of which passed in the
  accepted live write-window rerun.
- fixed: selected a helper-core contract that changes the retained evidence
  boundary by requiring bounded post-write capture from the saved command=0
  boundary cursor, with a direct-read equivalent only for saturated-cursor
  capture.
- fixed: carried forward the lab serial endpoint rule that empty observe output
  from a saturated cursor is not proof of no serial output.
- fixed: rejected serial write byte acknowledgement, stale pre-write output,
  command advancement, source-response-only evidence, missing selected identity,
  and missing restore proof as acceptance surfaces.
- deferred: Pi 5 hardware proof remains dependency-gated behind the helper-core
  fixture/validator task.
- not-an-issue: no runtime or docs/src change is required to define this source
  contract because the queued helper-core task already owns implementation of
  the local/static validator.

## Evidence

- Accepted live write-window proof classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/classification.json.
- Accepted live write-window post-write read:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/serial/command0-post-write-read.with-cursor.json.
- Accepted closeout classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-closeout/classification.json.
- Live write-window discriminator:
  scripts/rpi5-command0-live-write-window-discriminator.sh.
- Serial endpoint/cursor contract:
  docs/src/project/lab-controller.md.
- Source contract classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-post-write-capture-retention-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-post-write-capture-retention-source-contract/evidence-map.json.

## Acceptance Check

- The contract identifies whether the next objective step is a helper or
  supervisor intervention using retained evidence rather than a timing-only
  retry: satisfied; helper-core selected.
- The selected helper-core contract requires same-attempt selected-kernel/TFTP
  identity, fresh command=0 boundary, immediate rootinfo write, and bounded
  post-write serial capture from the saved boundary cursor or direct-read
  equivalent: satisfied.
- The contract rejects acceptance from serial write byte acknowledgement alone,
  stale pre-write output, command advancement, source-response-only evidence,
  or missing restore proof: satisfied.
- selected_next_task is
  phase10-pi5-command0-post-write-capture-retention-helper-core-20260618:
  satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-post-write-capture-retention-helper-core-20260618 on the
next worker wake if dependencies remain satisfied. Do not accept command0 input
delivery, source-response retention, generated-root command-input success,
storage, networking, SSH, Phase 11/12 expansion, or a phase transition from
this source/static contract.
