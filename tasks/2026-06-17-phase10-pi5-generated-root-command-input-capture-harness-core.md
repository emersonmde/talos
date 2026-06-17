# Phase 10 Pi 5 Generated-Root Command-Input Capture Harness Core

Task id: phase10-pi5-generated-root-command-input-capture-harness-core-20260617

Status: accepted

Classification:
command-input-capture-harness-core-local-static

Evidence level: static source/task evidence inspection, shell syntax check,
local/static proof-helper review, task-owned JSON evidence, docs build, and
diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, TFTP/serial live capture, runtime
feature expansion, persistence, storage, networking, SSH, Phase 11/12 expansion,
or phase transition was performed.

## Goal

Make the next generated-root command-input hardware proof decisive at the first
failing invariant: post-prompt /serial/write accepted bytes must become
shell-visible command text in retained serial, or the proof must classify why
they did not.

## Implementation

Updated scripts/rpi5-generated-root-command-input-proof-review.sh so its JSON
contract records the generated-root proof harness command order:

1. wait for same-boot source=firmware-initramfs reason=valid-artifact,
   rpi5-generated-root-boot-transport-proof: ready command=0, and a visible
   talos> prompt;
2. write rootinfo with append_newline=true, then observe from the saved command
   0 cursor until the rootinfo response and ready command=1 are retained;
3. save the command 1 post-prompt cursor, write cat /generated/manifest.txt
   with append_newline=true, then observe from that cursor until the retained
   command text, Talos generated-root external artifact A, dispatch command=1
   status=handled responses=1, and ready command=2 or final PASS evidence is
   retained.

The helper also records the capture strategy and allowed terminal
classifications for the serialized Pi 5 follow-up. Direct /serial/read fallback
is now diagnostic only: it may support triage when observe/cursor evidence is
saturated or unavailable, but it cannot replace command-indexed retained
evidence for command-input acceptance.

No runtime source changed. The next task owns boot publication and hardware
proof.

## Findings

- fixed: the proof contract now matches the generated-root harness source,
  which expects rootinfo at command 0 and cat /generated/manifest.txt at
  command 1.
- fixed: the proof contract now requires command-indexed saved cursors and
  /serial/observe windows rather than accepting direct-read-only retained
  output as command-input proof.
- fixed: allowed terminal classifications are explicit for accepted command
  input, missing write ingress, saturated observe/cursor evidence, command 0
  prelude blocker, command 1 manifest blocker, unexpected boot identity, and
  restore blocker.
- not-an-issue: newline termination remains append_newline=true; the blocked
  Pi 5 proof already recorded a 28-byte write for cat /generated/manifest.txt,
  matching the command plus newline.
- not-an-issue: external artifact content remains present in the reviewed boot
  archive; /generated/manifest.txt and Talos generated-root external artifact A
  are still checked locally before hardware.
- deferred: a runtime input-path source fix is not justified until the
  command-indexed two-step hardware proof fails with retained evidence.
- rejected: Pi 5 command-input success, hardware publication, persistence,
  writable filesystem, SD/USB/block storage, networking, SSH, Phase 11/12
  expansion, and phase transition claims in this local/static task.

## Evidence

- Accepted command-input closeout:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-closeout.md.
- Blocked Pi 5 command-input proof:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-pi5-proof.md.
- Prior proof core:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-proof-core.md.
- Updated helper:
  scripts/rpi5-generated-root-command-input-proof-review.sh.
- Capture-harness review JSON:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-core/capture-harness-review.json.
- Classification JSON:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-core/evidence-map.json.

## Validation

- static source/task evidence inspection: pass.
- sh -n scripts/rpi5-generated-root-command-input-proof-review.sh: pass.
- local/static proof-helper review on the retained compile-only archive: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- First failing invariant is stated: satisfied.
- At least two harness/input hypotheses are evaluated: satisfied in
  classification JSON with five dispositioned hypotheses.
- Helper change is narrow and preserves hardware proof identity, serial/TFTP,
  known-good control, candidate rerun, and restore requirements: satisfied.
- Terminal classification is allowed: satisfied with
  command-input-capture-harness-core-local-static.
- Task record and task-owned JSON evidence select the serialized Pi 5
  capture-harness proof: satisfied.

## Next Action

Promote
phase10-pi5-generated-root-command-input-capture-harness-pi5-proof-20260617 on
the next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not infer persistence, storage, networking, SSH, Phase 11/12
expansion, or a phase transition from this local/static task.
