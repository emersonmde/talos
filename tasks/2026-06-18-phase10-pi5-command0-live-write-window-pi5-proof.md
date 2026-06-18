# Phase 10 Pi 5 Command0 Live Write-Window Pi 5 Proof

Task id: phase10-pi5-command0-live-write-window-pi5-proof-20260618

Status: accepted

Classification:
command0-live-write-window-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock, lab
controller API identity/status evidence, TFTP delta evidence, direct serial
hardware read/write output, baseline restore proof, task-owned JSON evidence,
docs build, and diff checks.

## Goal

Run one serialized Pi 5 proof that binds a fresh command=0 readiness boundary
to an immediate rootinfo write and records whether ordered command0 delivery
passes with stable selected identity before timeout advancement.

## Result

Command0 input delivery remains blocked. The accepted direct-read rerun retained
an empty pre-power serial drain, selected post-publish identity, two stable
208984-byte da591740/kernel_2712.img TFTP fetches, a fresh command=0 readiness
boundary, an immediate rootinfo write, immediate/final pre-restore selected
identity, and baseline restore proof. The post-write serial read retained only
two bytes and no ordered rootinfo, dispatch command=0 status=handled,
responses=1, or ready command=1 evidence.

The first candidate used the cursor-based observe path while the lab serial
cursor was saturated at 4194304. It was treated as an inconclusive capture
attempt, not terminal evidence. The accepted rerun switched to the direct-read
path after an empty pre-power serial drain, which made the selected-kernel/TFTP
and live boundary gates evaluable.

This task accepts only the hardware proof classification. It rejects command0
input delivery acceptance, source-response retention, generated-root
command-input success, storage, networking, SSH, Phase 11/12 expansion, and
phase transition.

## Findings

- fixed: reran after the saturated-cursor candidate with a direct serial read
  path gated by an empty pre-power drain.
- fixed: retained selected post-publish, immediate post-command, final
  pre-restore, TFTP, and restore evidence for the terminal rerun.
- fixed: normalized the lab serial write response with the sent text and
  append_newline payload before replaying the discriminator; the raw API write
  result remains retained.
- not-an-issue: selected-kernel/TFTP identity stayed stable in the accepted
  rerun; the failing invariant is ordered command0 delivery after the live
  write.
- deferred: source-response retention remains gated on the follow-up closeout
  because command0 input delivery is still unaccepted.

## Evidence

- Inconclusive cursor-saturated candidate:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-20260618T041911Z/classification.json.
- Accepted rerun classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/classification.json.
- Accepted rerun evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/evidence-map.json.
- Accepted rerun pre-power drain:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/serial/pre-power-read-drain-summary.json.
- Accepted rerun TFTP delta:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/tftp/tftp-delta-stable-pre-restore.json.
- Accepted rerun readiness summary:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/serial/readiness-summary.json.
- Accepted rerun live write-window contract:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/serial/live-write-window.json.
- Accepted rerun serial write:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/serial/command0-write-normalized.json.
- Accepted rerun post-write read:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/serial/command0-post-write-read.with-cursor.json.
- Accepted rerun immediate identity:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/immediate-post-command-status.json.
- Accepted rerun final pre-restore identity:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/final-pre-restore-status.json.
- Accepted rerun restore proof:
  tasks/evidence/2026-06-18-phase10-pi5-command0-live-write-window-pi5-proof/candidate-live-write-window-direct-read-rerun-20260618T042226Z/restore/post-restore-status.json.

## Acceptance Check

- HardwareTestLock acquired before lab mutation and released after restore:
  satisfied by state history and retained restore evidence.
- Selected-kernel/TFTP precondition passes before command0 input delivery:
  satisfied by the accepted rerun with two selected 208984-byte fetches.
- Fresh command=0 boundary, immediate rootinfo write, and stable selected
  identity retained in the same attempt: satisfied.
- Ordered command0 delivery accepted only with rootinfo/dispatch/responses/ready
  evidence: satisfied by rejection; no input-delivery acceptance was made.
- selected_next_task is the closeout task: satisfied.

## Validation

- Pi 5 serialized hardware proof under hardwareTestLock: pass, blocked
  classification.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-live-write-window-closeout-20260618 on the next
worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has
no conflicting uncommitted changes.
