# Phase 10 Pi 5 Command0 Selected-Kernel Paired Sentinel Core

Task id: phase10-pi5-command0-selected-kernel-paired-sentinel-core-20260617

Status: accepted

Classification:
selected-kernel-paired-sentinel-core-local-static

Evidence level: local/static helper implementation, shell syntax check,
task-owned positive/negative fixture replay, retained selected-kernel
stability regression replay, task-owned JSON evidence, docs build, and diff
checks. No hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, command0 write, source-response-retention proof,
generated-root command-input acceptance, storage, networking, SSH,
Phase 11/12 expansion, or phase transition was performed.

## Goal

Create the local/static selected-kernel paired sentinel discriminator selected
by the recurrence checkpoint before any hardware proof or command0 retry.

## Implementation

Added scripts/rpi5-selected-kernel-paired-sentinel-discriminator.sh. The helper
accepts only selected-kernel-paired-sentinel-publication-boundary-v1 evidence
with both candidate and control runs present. Each run must satisfy:

1. no command write evidence is present;
2. post-publish identity exposes the selected tree, effective kernel_2712.img,
   selected da591740/kernel_2712.img, and selected kernel byte count;
3. the same-power-cycle TFTP cursor advances, is stable, has at least one
   selected-kernel fetch, and every selected-kernel fetch byte count matches
   the selected kernel;
4. final pre-restore identity still exposes the selected tree and selected
   kernel byte count;
5. restore proof is present; and
6. candidate and control selected identities are distinct.

The helper selects
phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof-20260617 only
for an accepted local/static result. Rejected outputs keep selected_next_task
null and preserve the first failing invariant.

## Findings

- fixed: added a paired no-command-write discriminator instead of reusing the
  single-run selected-kernel stability helper as a durable unblocker.
- fixed: positive fixture accepts only paired candidate/control evidence with
  selected post-publish identity, matching same-power-cycle TFTP bytes, final
  selected identity, restore proof, and distinct identities.
- fixed: negative fixtures reject retained baseline-served selected-kernel
  evidence, single-run-only evidence, and command-write-present evidence.
- fixed: retained selected-kernel stability proof replay is rejected on the
  original same-power-cycle TFTP byte invariant.
- deferred: serialized Pi 5 hardware proof remains dependency-gated behind
  phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof-20260617.
- rejected: command0 write-delivery success, source-response retention,
  generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Helper:
  scripts/rpi5-selected-kernel-paired-sentinel-discriminator.sh.
- Positive fixture:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/fixtures/positive-paired-sentinel.json.
- Negative fixtures:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/fixtures/negative-baseline-served-regression.json,
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/fixtures/negative-single-run-only.json, and
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/fixtures/negative-command-write-present.json.
- Fixture replay summary:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/results/fixture-replay-summary.json.
- Retained selected-kernel stability regression replay:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/results/retained-selected-kernel-stability-regression.result.json.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-core/evidence-map.json.

## Acceptance Check

- The local/static discriminator rejects retained baseline-served
  selected-kernel evidence and accepts only selected post-publish identity plus
  matching same-power-cycle TFTP served bytes plus final selected identity plus
  restore proof: satisfied.
- Helper/script changes are narrowly tied to the selected discriminator and
  include retained evidence/classification output: satisfied.
- Hardware follow-up is authorized as
  phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof-20260617:
  satisfied.
- Rejected claims include command0 write-delivery success, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- sh -n scripts/rpi5-selected-kernel-paired-sentinel-discriminator.sh: pass.
- task-owned paired sentinel positive/negative fixture replay: pass.
- retained selected-kernel stability regression replay: pass/rejected on
  same-power-cycle TFTP byte invariant.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof-20260617
on the next worker wake if dependencies remain satisfied and hardwareTestLock
is unlocked/restored. Do not accept command0 write-delivery,
source-response retention, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition from this
local/static core.
