# Phase 10 Pi 5 Command0 Readiness Timeout Boundary Discriminator Core

Task id: phase10-pi5-command0-readiness-timeout-boundary-discriminator-core-20260618

Status: accepted

Classification:
command0-readiness-timeout-boundary-discriminator-core-local-static

Evidence level: static source/task/evidence inspection, shell syntax check,
task-owned positive/negative fixture replay, retained final-identity proof
replay, task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
source-response retention proof, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Define the smallest local/static discriminator that separates true command0
rootinfo delivery from a wait path that has already advanced the generated-root
command loop to command=1 or later before rootinfo is observed.

## Implementation

Added scripts/rpi5-command0-readiness-timeout-boundary-discriminator.sh. The
helper implements command0-readiness-timeout-boundary-v1 and accepts only an
evidence shape where:

- readiness is same-boot firmware-initramfs valid-artifact output with
  ready command=0 and a visible talos> prompt;
- the saved pre-write boundary is fresh and has not already retained rootinfo,
  command0 dispatch, responses=1, ready command=1, or command=1+ timeout
  output;
- /serial/write accepts text=rootinfo, append_newline=true, bytes=9;
- post-write evidence is ordered as rootinfo or line command=0, dispatch
  command=0 status=handled, responses=1, and ready command=1;
- post-write evidence contains no line or dispatch for command=1 or later.

This core does not accept command0 input delivery. It selects the serialized
Pi 5 readiness-timeout-boundary proof only after the local contract and fixture
replay pass.

## Findings

- fixed: stated the first-principles invariant: rootinfo must be written and
  observed at command=0 from a fresh readiness boundary before timeout
  advancement to command=1 or later.
- fixed: added a task-owned discriminator helper and fixtures for positive
  command0 delivery, command=1 timeout advancement, command=4 timeout
  advancement, stale pre-write output, and retained final-identity proof replay.
- fixed: classified the accepted final-identity proof shape as
  readiness-wait-timeout-advanced-to-command4, not command0 input delivery.
- not-an-issue: selected-kernel/TFTP and final selected identity evidence from
  the final-identity proof remain useful context, but they cannot compensate for
  missing command0-boundary serial ordering.
- deferred: the serialized Pi 5 proof remains a separate task under
  hardwareTestLock.
- rejected: command0 input delivery acceptance, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Contract

The next Pi 5 proof must record:

- candidate identity, fresh serial cursor, and selected-kernel/TFTP evidence
  before command0 evaluation;
- the exact fresh command=0 readiness boundary used for the write;
- append_newline=true rootinfo write evidence tied to that saved boundary;
- post-write serial evidence from that boundary proving rootinfo at command=0,
  dispatch command=0 status=handled, responses=1, and ready command=1;
- rejection of command=1 or later timeout-advanced output as command0 delivery;
- immediate and final pre-restore selected identity plus restore proof.

Allowed terminal classifications are:

- command0-readiness-timeout-boundary-accepted;
- command0-readiness-timeout-boundary-blocked;
- command0-readiness-timeout-boundary-inconclusive-triage-required.

## Evidence

- Accepted final-identity proof:
  tasks/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof.md.
- Accepted final-identity closeout:
  tasks/2026-06-18-phase10-pi5-command0-final-identity-regression-closeout.md.
- Helper:
  scripts/rpi5-command0-readiness-timeout-boundary-discriminator.sh.
- Proof contract:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-discriminator-core/proof-contract.json.
- Classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-discriminator-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-discriminator-core/evidence-map.json.
- Fixture replay summary:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-discriminator-core/results/fixture-replay-summary.json.
- Retained final-identity replay:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-discriminator-core/results/negative-retained-final-identity-command4.out.json.

## Acceptance Check

- First-principles invariant is named: satisfied.
- Findings are recorded with disposition: satisfied.
- Stale retained output, command=1-or-later timeout-advanced output, and the
  accepted command=4 final-identity proof shape are rejected as command0
  delivery: satisfied.
- Positive fixture accepts only ordered command0 rootinfo/dispatch/responses=1
  and ready command=1 after the saved fresh command=0 boundary: satisfied.
- selected_next_task is
  phase10-pi5-command0-readiness-timeout-boundary-pi5-proof-20260618:
  satisfied.
- Command0 input delivery, source-response retention, generated-root
  command-input success, storage, networking, SSH, Phase 11/12 expansion, and
  phase transition remain rejected: satisfied.

## Validation

- static source/task/evidence inspection: pass.
- bash -n scripts/rpi5-command0-readiness-timeout-boundary-discriminator.sh:
  pass.
- positive command=0 fixture replay: pass.
- negative command=1 timeout-advanced fixture replay: pass.
- negative command=4 timeout-advanced fixture replay: pass.
- stale pre-write fixture replay: pass.
- retained final-identity proof replay: pass, rejected as
  readiness-wait-timeout-advanced-to-command4.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-readiness-timeout-boundary-pi5-proof-20260618 on
the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has no
conflicting uncommitted changes. Do not run hardware from this task.
