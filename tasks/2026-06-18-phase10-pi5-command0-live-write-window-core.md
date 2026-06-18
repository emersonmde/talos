# Phase 10 Pi 5 Command0 Live Write Window Core

Task id: phase10-pi5-command0-live-write-window-core-20260618

Status: accepted

Classification: command0-live-write-window-core-local-static

Evidence level: static source/task/evidence inspection, shell syntax check,
task-owned positive and negative fixture replay, retained blocked
readiness-timeout-boundary rerun replay, task-owned JSON evidence, docs build,
and diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, command0 input delivery acceptance,
source-response retention proof, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Define the smallest local/static discriminator that only accepts command0
delivery when rootinfo is written immediately after a live fresh command=0
readiness boundary. The contract must reject retained output that already
advanced past command=0 before the write.

## Implementation

Added scripts/rpi5-command0-live-write-window-discriminator.sh. The helper
implements command0-live-write-window-v1 and accepts only an evidence shape
where:

- readiness is same-boot firmware-initramfs valid-artifact output with
  ready command=0, a visible talos> prompt, a retained cursor, and no timeout,
  line command=0, dispatch command=0, responses=1, or ready command=1 output
  before rootinfo is written;
- live_write_window ties the write to the readiness cursor with
  boundary_was_live=true, write_issued_after_boundary=true,
  write_immediate_after_boundary=true, write_waited_for_timeout=false, no
  pre-write drain after the boundary, and max_boundary_to_write_ms <= 5000;
- post-write evidence uses the same cursor;
- /serial/write accepts text=rootinfo, append_newline=true, bytes=9;
- post-write evidence is ordered as rootinfo or line command=0, dispatch
  command=0 status=handled, responses=1, and ready command=1, with no command=2
  or later advancement before command0 delivery.

This core does not accept command0 input delivery. It selects the serialized
Pi 5 live write-window proof only after local fixture replay passes.

## Findings

- fixed: stated the first-principles invariant: rootinfo must be written
  immediately after a live fresh command=0 readiness boundary and observed as
  command=0 before timeout advancement.
- fixed: added a task-owned discriminator helper and fixtures for positive
  live write-window delivery, stale retained command0 boundary, delayed write,
  unordered delivery, and the accepted blocked readiness-timeout-boundary rerun.
- fixed: classified the retained readiness-timeout-boundary rerun as
  retained output already advanced past the live command0 window before write.
- not-an-issue: selected-kernel/TFTP and selected identity evidence from the
  blocked readiness-timeout-boundary proof remain useful hardware context, but
  they cannot prove command0 delivery without the live write-window boundary.
- deferred: the serialized Pi 5 proof remains a separate task under
  hardwareTestLock.
- rejected: command0 input delivery acceptance, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Evidence

- Accepted readiness-timeout-boundary closeout:
  tasks/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-closeout.md.
- Accepted readiness-timeout-boundary proof:
  tasks/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof.md.
- Helper: scripts/rpi5-command0-live-write-window-discriminator.sh.
- Proof contract:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-core/proof-contract.json.
- Classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-core/evidence-map.json.
- Fixture replay summary:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-core/results/fixture-replay-summary.json.
- Retained blocked rerun replay:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-core/results/negative-retained-readiness-timeout-boundary-rerun.out.json.

## Acceptance Check

- Live write-window contract cannot be satisfied by retained output already
  advanced past command=0 before rootinfo is written: satisfied.
- Retained readiness-timeout-boundary rerun is rejected because its pre-write
  boundary was stale/timeout-advanced: satisfied.
- Positive fixture covers fresh command=0 boundary, immediate rootinfo write,
  ordered command0 dispatch/responses, and ready command=1: satisfied.
- selected_next_task is
  phase10-pi5-command0-live-write-window-pi5-proof-20260618: satisfied.
- Rejected claims include command0 input delivery acceptance, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- static source/task/evidence inspection: pass.
- shell syntax check for
  scripts/rpi5-command0-live-write-window-discriminator.sh: pass.
- positive live write-window fixture replay: pass.
- stale boundary fixture replay: pass, rejected.
- delayed write fixture replay: pass, rejected.
- unordered delivery fixture replay: pass, rejected.
- retained readiness-timeout-boundary rerun replay: pass, rejected as retained
  output already advanced past the live command0 window before write.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-live-write-window-pi5-proof-20260618 on the next
worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has
no conflicting uncommitted changes. Do not run hardware from this local/static
discriminator.
