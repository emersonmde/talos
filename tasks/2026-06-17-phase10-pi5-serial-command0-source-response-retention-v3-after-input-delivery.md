# Phase 10 Pi 5 Serial Command0 Source-Response Retention V3 After Input Delivery

Task id: phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery-20260617

Status: accepted

Classification:
command0-source-response-retention-v3-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock, lab
controller API identity/status evidence, TFTP delta evidence, direct serial
hardware output, baseline restore proof, task-owned JSON evidence, docs build,
and diff checks.

## Goal

Evaluate command0 source-response retention only after accepted command0 input
delivery. The feature under test remains Pi 5 serial shell command input
against the firmware-initramfs generated-root artifact; command0 write delivery
is a precondition, not the success claim for this task.

## Result

Source-response retention remains blocked. The candidate retained the accepted
command0 input-delivery shape again: selected 208984-byte
`kernel_2712.img`, stable same-power-cycle TFTP serves of
`da591740/kernel_2712.img`, firmware-initramfs valid-artifact readiness,
9-byte `rootinfo\n` write acceptance, ordered command0 line evidence,
`dispatch command=0 status=handled responses=1`, and `ready command=1`.
Immediate/final identity stayed on the selected tree and the pre-run snapshot
restore returned the Pi 5 lab to the 104136-byte baseline.

The retained command0 response did not include the required
`source=firmware-initramfs reason=valid-artifact` text for the same selected
candidate boundary. The direct-read response starts mid-source-response at
`path=/generated/manifest.txt exec-path=/generated/status7` before the line,
dispatch, response-count, and ready markers. That is enough to preserve
command0 input delivery, but not enough to accept source-response retention or
generated-root command-input success.

selected_next_task is null and planningNeeded=true.

## Findings

- fixed: reran the source-response retention proof only after command0 input
  delivery was accepted by the timeout-stable command-index closeout.
- fixed: retained selected-kernel/TFTP identity, direct serial command0 output,
  final selected identity, and baseline restore proof under hardwareTestLock.
- blocked: source-response retention is still missing because the retained
  command0 output begins after the required `source=firmware-initramfs
  reason=valid-artifact` fragments.
- not-an-issue: command0 write delivery itself remained accepted in this run.
- rejected: generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery/candidate-source-response-retention-v3-20260618T080234Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery/candidate-source-response-retention-v3-20260618T080234Z/evidence-map.json.
- Readiness summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery/candidate-source-response-retention-v3-20260618T080234Z/serial/readiness-summary.json.
- Command0 direct-read summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery/candidate-source-response-retention-v3-20260618T080234Z/serial/command0-direct-read-summary.json.
- Command0 retained text:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery/candidate-source-response-retention-v3-20260618T080234Z/serial/command0-direct-read-text.txt.
- TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery/candidate-source-response-retention-v3-20260618T080234Z/tftp/tftp-delta-after-command.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-v3-after-input-delivery/candidate-source-response-retention-v3-20260618T080234Z/restore/post-restore-status.json.

## Acceptance Check

- Accepted source-response retention requires command0 input-delivery evidence
  plus retained response evidence for the same selected candidate boundary:
  blocked; input delivery passed, response source fragments were not retained.
- Blocked proof records the precise first failing invariant without claiming
  generated-root command-input success: satisfied.
- selected_next_task is null with planningNeeded=true: satisfied.
- Rejected claims include generated-root command-input success, storage,
  networking, SSH, Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- Pi 5 serialized hardware proof under hardwareTestLock: pass, terminal blocked
  classification.
- Candidate identity via lab API before power: pass.
- Fresh serial readiness and command0 direct-read evidence: pass.
- `/serial/write` result evidence: pass, 9 bytes written.
- Stable selected-candidate TFTP delta: pass, selected 208984-byte
  `da591740/kernel_2712.img` served.
- Final pre-restore boot identity and post-run baseline restore proof: pass.
- Task-owned classifier output: pass, blocked classification.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before another source-response retention retry,
generated-root command-input success claim, storage, networking, SSH, Phase
11/12 expansion, phase transition, or same-shaped command0 capture retry.
