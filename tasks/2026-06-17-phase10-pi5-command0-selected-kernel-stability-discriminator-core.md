# Phase 10 Pi 5 Command0 Selected-Kernel Stability Discriminator Core

Task id: phase10-pi5-command0-selected-kernel-stability-discriminator-core-20260617

Status: accepted

Classification:
selected-kernel-stability-discriminator-core-local-static

Evidence level: local/static helper implementation, shell syntax check,
task-owned positive/negative fixture replay, retained regression replay,
task-owned JSON evidence, docs build, and diff checks. No hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, command0
write, source-response-retention proof, generated-root command-input
acceptance, storage, networking, SSH, Phase 11/12 expansion, or phase
transition was performed.

## Goal

Create a local/static discriminator that accepts selected-kernel stability only
when post-publish identity, same-power-cycle TFTP bytes, final pre-restore
identity, and restore proof all satisfy the selected contract.

## Implementation

Added scripts/rpi5-selected-kernel-stability-discriminator.sh. The helper
normalizes selected-kernel stability evidence from task-owned fixture shape,
selected_kernel_tftp_precondition-v1 style evidence, or retained task
classification JSON where the required fields exist. It emits a JSON result and
exits 0 only when all of these boundaries pass:

1. post-publish identity exposes the selected tree, effective kernel_2712.img,
   selected da591740/kernel_2712.img, and the selected kernel byte count;
2. the TFTP cursor is present and advances;
3. the same-power-cycle TFTP delta is stable, has at least one selected-kernel
   fetch, and every selected-kernel fetch byte count matches the selected
   kernel;
4. final pre-restore identity still exposes the selected tree and selected
   kernel byte count;
5. restore proof is present and ok.

The helper selects
phase10-pi5-command0-selected-kernel-stability-pi5-proof-20260617 only for an
accepted local/static result. Rejected outputs keep selected_next_task null and
preserve the first failing invariant.

## Findings

- fixed: added a directly paired selected-kernel stability helper instead of
  reusing command0 transaction helpers as an implicit publication/TFTP gate.
- fixed: positive fixture accepts only the full selected-kernel stability shape
  with post-publish identity, fresh stable TFTP bytes, final identity, and
  restore proof.
- fixed: negative fixtures reject no fresh TFTP, final identity mismatch, stale
  cursor/cursor-boundary ambiguity, restore failure, and baseline-served TFTP
  regression.
- fixed: retained saturated-capture regression evidence remains rejected by the
  helper and therefore cannot unblock command0 write delivery or
  source-response retention.
- not-an-issue: command0 write-delivery, source-response retention, and
  generated-root command-input helpers remain downstream gates only after this
  selected-kernel stability prerequisite is reproven.
- rejected: command0 write-delivery success, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Evidence

- Helper:
  scripts/rpi5-selected-kernel-stability-discriminator.sh.
- Positive fixture:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/fixtures/positive-selected-kernel-stability.json.
- Negative fixtures:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/fixtures/negative-no-fresh-tftp.json,
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/fixtures/negative-final-identity-mismatch.json,
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/fixtures/negative-stale-cursor.json,
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/fixtures/negative-restore-failure.json, and
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/fixtures/negative-baseline-served-regression.json.
- Fixture replay summary:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/results/fixture-replay-summary.json.
- Retained saturated-capture regression replay:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/results/retained-saturated-capture-regression.result.json.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/evidence-map.json.

## Acceptance Check

- Local/static discriminator accepts only evidence where post-publish selected
  identity, same-power-cycle TFTP bytes, final identity, and restore satisfy
  the selected contract: satisfied.
- Local/static discriminator rejects the retained saturated-capture
  baseline-served regression evidence: satisfied.
- Fixtures cover no fresh TFTP, final identity mismatch, stale cursor/cursor
  boundary ambiguity, and restore failure: satisfied.
- selected_next_task is
  phase10-pi5-command0-selected-kernel-stability-pi5-proof-20260617:
  satisfied.
- Rejected claims remain explicit for command0 write delivery, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- sh -n scripts/rpi5-selected-kernel-stability-discriminator.sh: pass.
- task-owned positive/negative fixture replay: pass.
- retained saturated-capture regression replay: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-selected-kernel-stability-pi5-proof-20260617 on the
next worker wake if dependencies remain satisfied and hardwareTestLock is
unlocked/restored. Do not accept command0 write-delivery, source-response
retention, generated-root command-input success, storage, networking, SSH,
Phase 11/12 expansion, or phase transition from this local/static core.
