# Phase 10 Pi 5 Command0 Final Identity Regression Discriminator Core

Task id: phase10-pi5-command0-final-identity-regression-discriminator-core-20260618

Status: accepted

Classification:
command0-final-identity-regression-discriminator-core-local-static

Evidence level: static source/task/evidence consistency review, shell syntax
check, task-owned positive/negative fixture replay, retained regression replay,
task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware run,
boot archive publication, lab mutation, hardwareTestLock acquisition, command0
retry, source-response retention proof, generated-root command-input success,
storage, networking, SSH, Phase 11/12 expansion, or phase transition was
performed.

## Goal

Define the smallest discriminator for the final pre-restore identity regression
that followed otherwise passing selected-kernel/TFTP and command0 serial
delivery evidence.

## Implementation

Added scripts/rpi5-command0-final-identity-regression-discriminator.sh. The
helper implements command0-final-identity-regression-v1 and accepts only an
evidence shape where:

- selected-kernel/TFTP precondition passes before command0 evaluation;
- ordered prearmed command0 serial delivery passes;
- lab identity is sampled immediately after command0 and before any restore or
  cleanup side effect;
- immediate post-command0 identity still matches the selected tree and
  208984-byte da591740/kernel_2712.img;
- final pre-restore identity still matches that selected tree and kernel byte
  count;
- restore proof is present.

This core does not accept command0 input delivery. It selects the next
serialized Pi 5 proof only after proving the local contract and fixture replay.

## Findings

- fixed: stated the first-principles invariant: the selected candidate identity
  must remain selected from post-publish through immediate post-command0 status
  until explicit restore.
- fixed: identified the evidence gap that allowed selected pre-command TFTP and
  ordered command0 serial delivery to coexist with final baseline identity: the
  proof did not sample lab identity immediately after command0 and before
  restore or cleanup ambiguity.
- fixed: added a local/static discriminator helper and fixtures that require the
  immediate post-command identity sample and visible evidence ordering.
- fixed: retained after-precondition regression evidence is rejected as missing
  the immediate post-command identity sample before restore/cleanup.
- deferred: the serialized Pi 5 proof remains a separate task under
  hardwareTestLock.
- rejected: command0 input delivery acceptance, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Contract

The next Pi 5 proof must record:

- candidate identity and expected kernel_2712.img byte count before power;
- fresh serial cursor and TFTP cursor before power;
- selected-kernel/TFTP delta before command0 evaluation;
- ordered command0 serial delivery;
- immediate post-command0 lab identity before any restore or cleanup side
  effect;
- final pre-restore identity;
- restore request/result and post-restore baseline identity.

Allowed terminal classifications are:

- command0-final-identity-stable-input-delivery-accepted;
- command0-final-identity-regressed-after-command0;
- command0-final-identity-missing-immediate-post-command-sample;
- command0-final-identity-command0-delivery-blocked;
- command0-final-identity-precondition-blocked;
- command0-final-identity-inconclusive-triage-required.

## Evidence

- Accepted after-precondition closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-closeout.md.
- Accepted after-precondition Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof.md.
- Helper:
  scripts/rpi5-command0-final-identity-regression-discriminator.sh.
- Proof contract:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-discriminator-core/proof-contract.json.
- Classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-discriminator-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-discriminator-core/evidence-map.json.
- Fixture replay summary:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-discriminator-core/results/fixture-replay-summary.json.

## Acceptance Check

- Core states the first-principles invariant: satisfied.
- Core identifies the exact evidence gap: satisfied.
- Core updates the next proof contract so it samples identity immediately after
  command0 and before restore/cleanup, with ordering visible in JSON:
  satisfied.
- Core defines allowed terminal classifications and the next selected task:
  satisfied.
- Core does not accept command0 input delivery, source-response retention,
  generated-root command-input success, or phase transition: satisfied.
- selected_next_task is
  phase10-pi5-command0-final-identity-regression-pi5-proof-20260618:
  satisfied.

## Validation

- task/source/evidence consistency review: pass.
- bash -n scripts/rpi5-command0-final-identity-regression-discriminator.sh:
  pass.
- task-owned positive fixture replay: pass.
- task-owned negative fixture replay: pass.
- retained regression replay: pass, rejected on missing immediate post-command
  identity sample before restore/cleanup.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-final-identity-regression-pi5-proof-20260618 on the
next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has no
conflicting uncommitted changes.
