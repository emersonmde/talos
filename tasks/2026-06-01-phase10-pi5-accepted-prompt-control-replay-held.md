# Phase 10 Pi 5 Accepted Prompt Control Replay Held

Task: phase10-pi5-accepted-prompt-control-replay-held-20260601
Status: accepted-prompt-control-responsive

## Goal

Replay an already accepted prompt-capable Pi 5 local-interactivity feature while
holding the selected archive active until the lab records selected-control TFTP
fetch evidence before any help-command candidate rerun.

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

Accepted local3 evidence:

- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/selected-control.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/archive-review.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/archive-sha256.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/archive-kernel-sha256.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/pre-status.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/pre-serial-peek.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/pre-tftp-tail.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/publish.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/power-cycle.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/tftp-kernel-fetch-held-control.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/serial-write-literal-echo.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/serial-transcript.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/serial-key-lines.txt
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/tftp-delta-final-before-restore.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/rollback.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/post-rollback-status.json
- tasks/evidence/2026-06-01-pi5-accepted-prompt-control-replay-held/local3-held-literal-echo-control/control-result.txt

Earlier local1/local2 attempts are retained under the same evidence root. They
were not used as acceptance evidence: local1 wrote after the proof had timed
out, and local2 did not retain selected-control fetch bytes.

## Result

Classification: accepted-prompt-control-responsive.

The selected accepted literal-echo archive matched its recorded sha256 and
passed static archive review. The lab published it successfully, and the fixed
Weathertop port 8 power cycle returned ok=true. While the selected archive was
active, retained TFTP evidence shows da591740/kernel_2712.img served with
100352 bytes, matching the selected accepted control kernel size.

Fresh serial evidence shows the selected control reached the descriptor-backed
local command loop, accepted echo local serial works, printed visible local
serial works, returned to talos>, emitted pi5-local-literal-echo-complete, and
ended with rpi5-local-literal-echo-proof: PASS.

Rollback restored the pre-run boot tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Hardware Lock

- owner task: phase10-pi5-accepted-prompt-control-replay-held-20260601
- local3 lock acquired: recorded in durable supervisor state at
  2026-06-01T05:32:17Z
- local3 lock released: recorded in durable supervisor state at
  2026-06-01T05:32:44Z
- restore status: PASS; post-rollback tree hash matched the pre-run tree hash.

## Validation

- static selected-control review: accepted literal echo proof identity recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- serialized Pi 5 corrected control replay: local3 retained selected-control
  TFTP fetch, literal echo command response, next-prompt readiness,
  classification, and PASS.
- post-run restore proof: pre/post boot tree hashes matched.
- static inspection: git diff --check passed for this task's files.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

The prompt-control discriminator is satisfied. The help-command proof can be
promoted by the worker on a later wake without a supervisor flip, provided the
help candidate is rerun unchanged first and the hardware lock is unlocked and
restored.
