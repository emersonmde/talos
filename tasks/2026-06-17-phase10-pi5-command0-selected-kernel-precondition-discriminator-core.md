# Phase 10 Pi 5 Command0 Selected-Kernel Precondition Discriminator Core

Task id: phase10-pi5-command0-selected-kernel-precondition-discriminator-core-20260617

Status: accepted

Classification:
selected-kernel-precondition-discriminator-core-local-static

Evidence level: static source/task inspection, shell syntax check,
local/static fixture replay, retained lab-boundary regression replay,
task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware run,
boot archive publication, lab mutation, hardwareTestLock acquisition,
command0 retry, source-response-retention proof, generated-root command-input
acceptance, storage, networking, SSH, Phase 11/12 expansion, or phase
transition was performed.

## Goal

Encode the selected-kernel/TFTP precondition contract that must pass before
future command0 lab-boundary evidence can be accepted again.

## Implementation

Added scripts/rpi5-selected-kernel-precondition-discriminator.sh. The helper
implements selected-kernel-tftp-precondition-lab-boundary-v1 and accepts only a
no-command-write selected-kernel precondition proof with:

- da591740/kernel_2712.img as the selected TFTP fetch;
- post-publish selected tree identity and effective kernel_2712.img;
- selected kernel_2712.img byte count present before power;
- fresh stable same-power-cycle TFTP cursor delta;
- every retained TFTP fetch byte count matching the selected kernel byte
  count;
- final pre-restore selected tree identity and effective kernel_2712.img;
- restore proof;
- no command write evidence and no single-run durable-unblocker claim.

This discriminator is deliberately narrower than command0 input delivery. It
does not accept /serial/write delivery, source-response retention,
generated-root command-input success, storage, networking, SSH, Phase 11/12
expansion, or phase transition.

## Local Static Evidence

Fixture replay summary:
tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/results/fixture-replay-summary.json.

Positive fixture:
tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/fixtures/positive-selected-kernel-precondition.json.

Negative fixtures reject:

- retained lab-boundary regression where the candidate expected 208984 bytes
  but same-power-cycle TFTP served 104136-byte baseline kernels;
- stale or no-fresh-TFTP evidence;
- final pre-restore identity mismatch;
- command-write-present evidence;
- single-run durable-unblocker claims.

## Findings

- fixed: added the local/static precondition discriminator helper.
- fixed: positive selected-kernel evidence passes only when post-publish
  identity, same-power-cycle TFTP bytes, final identity, restore, and
  no-command-write boundary are all present.
- fixed: retained lab-boundary regression evidence is rejected on the original
  selected-kernel/TFTP invariant.
- fixed: negative fixtures reject stale/no-fresh TFTP, final identity mismatch,
  command-write-present evidence, and single-run durable-unblocker claims.
- deferred: Pi 5 precondition proof remains a separate serialized hardware
  task under hardwareTestLock.
- rejected: command0 input delivery acceptance, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Changed Files

- scripts/rpi5-selected-kernel-precondition-discriminator.sh.
- docs/src/roadmap.md.
- tasks/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core.md.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/classification.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/evidence-map.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/fixtures/positive-selected-kernel-precondition.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/fixtures/negative-baseline-served-regression.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/fixtures/negative-no-fresh-tftp.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/fixtures/negative-final-identity-mismatch.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/fixtures/negative-command-write-present.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/fixtures/negative-single-run-durable-unblocker.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-discriminator-core/results/fixture-replay-summary.json.

## Acceptance Check

- Local/static discriminator accepts only evidence satisfying the
  selected-kernel/TFTP precondition contract selected by the checkpoint:
  satisfied.
- Fixtures cover positive selected-kernel evidence and negative
  baseline-served, stale/no-fresh-TFTP, final-identity-mismatch,
  command-write-present, and single-run-durable-unblocker cases: satisfied.
- Retained lab-boundary regression evidence is rejected on the original
  selected-kernel/TFTP invariant: satisfied.
- selected_next_task is
  phase10-pi5-command0-selected-kernel-precondition-pi5-proof-20260617 only if
  local/static validation passes: satisfied.
- Rejected claims remain explicit: command0 input delivery, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- sh -n scripts/rpi5-selected-kernel-precondition-discriminator.sh: pass.
- task-owned positive fixture replay: pass.
- task-owned negative fixture replay: pass.
- retained lab-boundary regression replay: pass, rejected on
  same-power-cycle TFTP served bytes not matching the selected kernel.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-selected-kernel-precondition-pi5-proof-20260617 on the
next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has
no conflicting uncommitted changes. Do not retry command0 from this
local/static core.
