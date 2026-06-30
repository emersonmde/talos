# Phase 12 SSH Live TCP TFTP Capture Boundary Reconciliation V12

Task id: phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12-20260630

Status: accepted after commit.

Classification: tftp-capture-helper-repair-ready.

Evidence level: static/helper inspection, retained v11 JSON inspection, read-only
lab-controller TFTP cursor replay, targeted helper replay, task-owned JSON
evidence, docs build, and diff checks.

## Goal

Reconcile the v11 runtime-marker preflight TFTP capture boundary without
hardware, then either repair the helper contract or preserve a precise blocker.

## Scope Performed

- Inspected the retained v11 evidence from
  phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11-20260630.
- Replayed the retained v11 TFTP cursor 4693472 through GET /tftp/logs without
  publishing artifacts, restoring snapshots, power-cycling the Pi 5, packet-I/O,
  OpenSSH/generated-root retry, or generated-root work.
- Repaired scripts/rpi5-wait-tftp-delta.sh so event-containing responses may
  still complete after stable samples, but empty responses must wait until the
  configured timeout before returning stable-zero-timeout.
- Retained task-owned evidence showing the repaired helper observes the v11
  cursor's delayed events and no longer accepts a fresh empty tail after only a
  few stable samples.

## Terminal Classification

tftp-capture-helper-repair-ready.

The first missing v11 fact remains selected-fetch proof in the original
pre-restore window, but the stable-zero helper result is no longer durable
no-request evidence. Replaying v11 cursor 4693472 now returns cursor_end
4694823 with 13 parsed events at the v11 boot time, including two
da591740/kernel_2712.img serves. Because the lab endpoint computes byte labels
from the current served root, the replay is not used to reclassify selected-byte
identity. It proves the old helper accepted empty stability too early.

The corrected capture algorithm is:

- keep polling from the same cursor until event-containing output is stable, then
  return success;
- if the output remains empty, wait until the configured timeout before returning
  stable-zero-timeout and failure.

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12-20260630.

planningNeeded: false.

## Findings

- fixed: v11 zero-event pre-restore result is recorded as not durable
  no-request evidence after read-only cursor replay exposed delayed events.
- fixed: scripts/rpi5-wait-tftp-delta.sh no longer exits on stable empty
  samples before the capture window elapses.
- not-an-issue: post-restore replay byte labels are current served-root
  metadata, so they are retained as timing/cursor evidence only.
- deferred: packet-I/O and OpenSSH/generated-root retry remain blocked until the
  selected v12 hardware preflight proves selected fetch and runtime markers
  under the repaired contract.
- removed: same-shaped stable-zero helper acceptance as a candidate-capture-ready
  blocker shortcut.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12/tftp-capture-boundary-reconciliation-v12-20260630T092407Z/.
- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12/tftp-capture-boundary-reconciliation-v12-20260630T092407Z/evidence-map.json.
- Read-only v11 cursor replay:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12/tftp-capture-boundary-reconciliation-v12-20260630T092407Z/v11-cursor-4693472-current-replay-summary.json.
- Repaired helper replay against retained v11 cursor:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12/tftp-capture-boundary-reconciliation-v12-20260630T092407Z/validation/repaired-helper-retained-v11-cursor.stdout.json.
- Repaired helper empty-tail timeout check:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12/tftp-capture-boundary-reconciliation-v12-20260630T092407Z/validation/repaired-helper-empty-tail-timeout.stdout.json.

## Redaction Review

Task summary JSON avoids packet payloads, SSH material, key material, boot
artifact bytes, private user data, and generated-root data. Raw TFTP replay
artifacts remain task-owned lab evidence and may include local lab endpoint
fields.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no conflicting tracked Talos changes before task-owned changes.
- jq empty on retained v11 JSON evidence: pass.
- Read-only GET /tftp/logs cursor replay: pass; cursor 4693472 now returns 13
  parsed events.
- bash/sh syntax for scripts/rpi5-wait-tftp-delta.sh: pass.
- Targeted helper replay: pass; retained v11 cursor exits 0 with 13 events, and
  a fresh empty-tail check exits 1 after the configured 4-second timeout with
  reason stable-zero-timeout.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
