# Phase 10 Pi 5 Accepted Prompt Control Replay

Task: phase10-pi5-accepted-prompt-control-replay-20260601
Status: accepted-prompt-control-inconclusive

## Goal

Replay an already accepted prompt-capable Pi 5 local-interactivity feature as
the control that proves the lab can still reach a Talos prompt before any
help-command candidate rerun.

## Scope

This task selected the already accepted bounded literal echo Pi 5 proof as the
control:

- accepted task: phase10-pi5-local-literal-echo-proof-20260531
- accepted commit: 29b8b7d6afbc57dc156db593c376aef36640ebb1
- accepted archive: target/talos-rpi5-local-literal-echo-local3.tar.gz
- accepted archive sha256:
  7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- accepted kernel sha256:
  63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826

The help-command candidate remained quarantined and was not published or run.
No Talos runtime, proof harness, lab-controller, roadmap, or ADR changes were
made by this task.

## Evidence

Retained evidence:

- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/selected-control.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/archive-review.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/archive-sha256.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/pre-status.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/pre-serial-peek.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/pre-tftp-tail.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/publish.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/power-cycle.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/serial-observe-after-power.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/tftp-delta-final-before-restore.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/rollback.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/post-rollback-status.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/serial-observe-late-after-restore.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/tftp-delta-late-after-restore.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/control-result.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay/local1-literal-echo-control/file-mtimes.txt

## Result

Classification: accepted-prompt-control-inconclusive.

The selected literal-echo archive matched the accepted sha256 and passed static
archive review. The lab published it successfully, and the fixed Weathertop port
8 power cycle returned ok=true. Fresh serial captured only early firmware/RP1
bytes from cursor 3828692. The first TFTP delta before rollback had zero events.

Rollback then restored the pre-run boot tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. A later
TFTP query from the same cursor showed 13 TFTP events, including
kernel_2712.img, but those events occurred after rollback and therefore reflect
the restored tree's file sizes rather than proving that the selected accepted
literal-echo control booted. No Talos prompt, descriptor-backed markers,
literal echo response, complete classification, or PASS vocabulary appeared in
fresh serial bytes.

## Hardware Lock

- owner task: phase10-pi5-accepted-prompt-control-replay-20260601
- hardware lock acquired: recorded in durable supervisor state at
  2026-06-01T03:54:02Z
- hardware lock released: recorded in durable supervisor state at
  2026-06-01T03:57:10Z
- restore status: PASS; post-rollback tree hash matched the pre-run tree hash.

## Validation

- static selected-control review: accepted literal echo proof identity recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- serialized Pi 5 control replay: retained, but inconclusive; power serial was
  fresh, while selected-control TFTP fetch was not captured before rollback.
- post-run restore proof: pre/post boot tree hashes matched.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

Set planningNeeded=true for supervisor planning. The help-command proof remains
dependency-blocked and the help/runtime code remains quarantined. A follow-up
control replay should keep the selected archive published until a TFTP delta is
observed or a longer explicit timeout expires before rollback; this task should
not be treated as prompt-responsive evidence.
