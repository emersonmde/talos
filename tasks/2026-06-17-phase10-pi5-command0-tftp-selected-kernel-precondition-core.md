# Phase 10 Pi 5 Command0 TFTP Selected-Kernel Precondition Core

Task id: phase10-pi5-command0-tftp-selected-kernel-precondition-core-20260617

Status: accepted

Classification:
command0-tftp-selected-kernel-precondition-core-local-static

Evidence level: static source/task inspection, shell syntax check, local/static
proof-helper fixtures, task-owned JSON evidence, docs build, and diff checks.
No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, command-loop feature expansion, storage,
networking, SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Implement the selected local/static preflight guard that prevents command0 Pi 5
proofs from proceeding when the lab-selected kernel and TFTP-served kernel
bytes disagree.

## Implementation

Updated scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh
to add selected-kernel-tftp-precondition-v1 before the existing serial capture
and command0 transaction gates can accept a full direct_read_proof.

The new guard requires:

- expected_fetch is da591740/kernel_2712.img;
- expected_kernel_2712_size matches direct_read_proof.boot.kernel_2712_size;
- the same-power-cycle TFTP cursor advances and the retained delta is stable;
- every retained selected-kernel TFTP fetch has the expected byte count;
- final pre-restore identity still exposes kernel_2712.img at that byte count;
- restore proof is present and ok.

This keeps command0-write-delivery-guard-v1 and
command0-source-response-retention-guard-v2 as later transaction gates. A
selected-kernel/TFTP precondition pass does not accept command0 write delivery,
command0 source-response retention, generated-root command-input success, or a
phase transition.

## Local Static Evidence

Positive fixture:
tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/selected-kernel-tftp-positive.json.

Positive review:
tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/selected-kernel-tftp-positive-review.json.

Negative results:
tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/selected-kernel-tftp-negative-results.txt.

The positive fixture proves that a selected 208984-byte kernel_2712.img with
two matching same-power-cycle da591740/kernel_2712.img TFTP fetches can pass
the precondition before command0 behavior is evaluated.

Negative controls reject:

- no fresh TFTP;
- baseline-sized 104136-byte TFTP fetches under candidate identity;
- final pre-restore selected-kernel identity mismatch;
- stale serial-only evidence with no selected-kernel/TFTP precondition;
- restore failure;
- the retained known mismatch where the selected candidate expected 208984
  bytes but the same-cursor TFTP evidence served 104136 bytes.

## Findings

- fixed: the proof-review helper now names
  selected-kernel-tftp-precondition-v1 and requires it for full
  direct_read_proof validation.
- fixed: local/static positive evidence proves selected-kernel/TFTP byte
  agreement can be accepted before command0 behavior is evaluated.
- fixed: local/static negative fixtures reject no fresh TFTP, baseline-sized
  TFTP under candidate identity, final identity mismatch, stale serial-only
  evidence, restore failure, and the known 208984-expected versus
  104136-served mismatch.
- not-an-issue: command0-write-delivery-guard-v1 and
  command0-source-response-retention-guard-v2 remain separate transaction
  gates; this task does not weaken either one.
- deferred: lab/boot-root reconciliation remains deferred until a serialized
  precondition proof shows the mismatch persists with this guard.
- rejected: command0 write-delivery success, command0 source-response retention
  success, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Changed Files

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh.
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md.
- docs/src/roadmap.md.
- tasks/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core.md.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/classification.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/evidence-map.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/selected-kernel-tftp-positive.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/selected-kernel-tftp-positive-review.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/no-fresh-tftp-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/baseline-sized-tftp-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/final-identity-mismatch-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/stale-serial-only-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/restore-failure-negative.json.
- tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/selected-kernel-tftp-negative-results.txt.

## Acceptance Check

- Implementation matches the accepted source contract: satisfied; only the
  selected proof-helper surface, task-owned evidence, task record, and docs
  were changed.
- Local/static validation rejects the known 208984-expected versus
  104136-served mismatch shape: satisfied by known-mismatch rejection.
- Positive fixture/static evidence proves selected-kernel/TFTP-served byte
  agreement can be accepted before command0 proof continues: satisfied.
- Negative controls reject no fresh TFTP, baseline-sized TFTP under candidate
  identity, final identity mismatch, stale serial-only evidence, and restore
  failure: satisfied.
- Task-owned JSON records findings with disposition: satisfied.
- Hardware follow-up authorization: selected_next_task is
  phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof-20260617.

## Validation

- sh -n scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh:
  pass.
- task-owned local/static validator positive fixture: pass.
- task-owned local/static validator negative fixtures: no-fresh-tftp,
  baseline-sized-tftp, final-identity-mismatch, stale-serial-only,
  restore-failure, and known-mismatch rejected.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof-20260617 on
the next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not retry command0 write-delivery behavior directly from this
local/static core.
