# Phase 10 Pi 5 Generated-Root Command-Input Direct-Read Harness Core

Task id: phase10-pi5-generated-root-command-input-direct-read-harness-core-20260617

Status: accepted

Classification:
direct-read-command-input-harness-core-local-static

Evidence level: static source/task evidence inspection, shell syntax check,
local/static generated-root command-input proof-helper review, positive and
negative direct-read evidence validator fixtures, task-owned JSON evidence,
docs build, and diff checks. No Pi 5 hardware run, boot archive publication,
lab mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial live
capture, runtime feature expansion, persistence, storage, networking, SSH,
Phase 11/12 expansion, or phase transition was performed.

## Goal

Implement the local/static helper for the accepted
direct-read-after-saturated-cursor-command-input-v1 contract so the next Pi 5
hardware proof can evaluate command-indexed direct-read evidence mechanically.

## Implementation

Added
scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh.

The helper reviews the retained generated-root command-input archive and keeps
the existing source gates:

1. the candidate boot archive passes the generated-root boot-transport archive
   review;
2. root and da591740-prefixed initramfs_2712 artifacts exist and match;
3. the generated-root artifact contains /generated/manifest.txt and
   Talos generated-root external artifact A;
4. kernel strings retain firmware-initramfs, valid-artifact, rootinfo,
   cat /generated/manifest.txt, ready command, dispatch, status, responses,
   ready-for-next, and PASS markers.

The helper emits the direct-read contract selected by the source task. The
next hardware proof must retain same-boot source=firmware-initramfs
reason=valid-artifact, selected-tree identity, stable TFTP evidence, final
pre-restore identity, and restore proof. Within that boot it must:

1. perform a command 0 pre-write freshness read after ready command=0 and a
   visible talos> prompt;
2. write rootinfo with append_newline=true;
3. retain a command 0 direct-read window with rootinfo, source evidence,
   dispatch command=0 status=handled responses=1, and ready command=1;
4. perform a command 1 pre-write freshness read after ready command=1;
5. write cat /generated/manifest.txt with append_newline=true;
6. retain a command 1 direct-read window with the command text,
   Talos generated-root external artifact A, dispatch command=1
   status=handled responses=1, and ready command=2, ready-for-next
   prompt=true, or PASS.

The helper also accepts an optional direct-read-evidence.json argument. That
validator rejects prompt-only, /serial/write-only, stale pre-write direct-read,
missing-dispatch, missing source gate, and missing TFTP/final-identity/restore
evidence. Task-owned fixtures prove the positive shape passes and the four
explicit negative shapes are rejected.

## Findings

- fixed: added a dedicated direct-read helper instead of reusing the old
  observe-only helper contract.
- fixed: generated-root artifact content and firmware-initramfs valid-artifact
  source gates remain mechanically checked before hardware.
- fixed: command 0 and command 1 direct-read windows are command-indexed and
  require immediate pre-write freshness reads.
- fixed: prompt-only and /serial/write-only evidence are mechanically rejected
  by the helper's optional evidence validator.
- fixed: stale pre-write direct-read output is rejected so a previous response
  cannot satisfy a later command window.
- fixed: missing dispatch command=1 status=handled responses=1 evidence is
  rejected even when command text and manifest output are present.
- deferred: serialized Pi 5 hardware publication and command-input acceptance
  are owned by the selected follow-up task.
- rejected: Pi 5 command-input success, persistence, writable filesystem,
  SD/USB/block storage, networking, SSH, Phase 11/12 expansion, and phase
  transition claims in this local/static task.

## Evidence

- Accepted direct-read source contract:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-source-contract.md.
- Helper:
  scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh.
- Direct-read helper review:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core/direct-read-harness-review.json.
- Positive validator fixture:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core/direct-read-validator-positive.json.
- Positive validator review:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core/direct-read-validator-positive-review.json.
- Negative validator results:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core/direct-read-validator-negative-results.json.
- Classification JSON:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core/evidence-map.json.

## Validation

- sh -n scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh:
  pass.
- local/static generated-root command-input proof-helper review on the
  retained compile-only archive: pass.
- positive direct-read evidence validator fixture: pass.
- negative validator fixtures for prompt-only, /serial/write-only, stale
  pre-write direct-read, and missing dispatch: rejected as expected.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Implementation matches the accepted direct-read source contract exactly:
  satisfied.
- Local/static review output records command-indexed pre-write and post-write
  freshness requirements for rootinfo and cat /generated/manifest.txt:
  satisfied.
- The helper rejects prompt-only, /serial/write-only, stale direct-read, and
  missing-dispatch evidence: satisfied by task-owned negative fixtures.
- The generated-root external artifact content and firmware-initramfs source
  gates remain mechanically checked before hardware: satisfied.
- selected_next_task is the direct-read Pi 5 proof: satisfied.

## Next Action

Promote
phase10-pi5-generated-root-command-input-direct-read-pi5-proof-20260617 on the
next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not infer persistence, storage, networking, SSH, Phase 11/12
expansion, or a phase transition from this local/static task.
