# Phase 10 Pi 5 Prompt Control After Capture Recovery

Task: phase10-pi5-prompt-control-after-capture-recovery-20260601
Status: accepted-prompt-responsive

## Goal

After independent evidence proved the Pi 5 lab serial capture path was live
again, replay an already accepted prompt-capable Talos control before resuming
the paused `ls /` hardware proof.

## Scope

This task selected the already accepted bounded literal-echo Pi 5 proof as the
known-good prompt control:

- accepted task: phase10-pi5-accepted-prompt-control-replay-held-20260601
- accepted commit: 9f02d5436b5b7d00f21a023f6bd90d725d8ccf34
- accepted archive: target/talos-rpi5-local-literal-echo-local3.tar.gz
- accepted archive sha256:
  7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- accepted kernel sha256:
  63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826

The `ls /` candidate remained paused and was not published or rerun. No Talos
source, proof script, command-loop behavior, archive construction, boot routing,
lab-controller code, roadmap, or ADR file was changed by this task.

## Evidence

Evidence directory:

- tasks/evidence/2026-06-01-pi5-prompt-control-after-capture-recovery/local1-held-literal-echo-control/

Key files:

- selected-control.txt
- archive-review.txt
- archive-sha256.txt
- archive-kernel-sha256.txt
- archive-kernel-size.txt
- pre-status.json
- pre-snapshots.json
- pre-serial-peek.json
- pre-tftp-tail.json
- serial-cursor-before.txt
- tftp-cursor-before.txt
- pre-run-snapshot.json
- publish.json
- post-publish-status.json
- power-cycle.json
- tftp-delta-final-before-restore.json
- tftp-kernel-fetch-held-control.txt
- serial-write-literal-echo.json
- serial-observe-after-write-final.json
- serial-transcript.txt
- serial-transcript-clean.txt
- serial-key-lines.txt
- restore-snapshot.json
- post-restore-status.json
- control-result.txt

Accepted capture-live prerequisite:

- tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/local1-production-timer-control/discriminator-result.txt

## Result

Classification: accepted-prompt-responsive.

The selected accepted literal-echo archive matched its recorded sha256 and
passed static archive review. The lab published it successfully, and the fixed
Weathertop port 8 power cycle returned ok=true. Retained TFTP evidence shows
`da591740/kernel_2712.img` served with 104136 bytes for the selected control
after the fresh pre-run TFTP cursor 3993667.

Fresh serial evidence advanced from cursor 3841582 to 3848207. The retained
transcript shows the control reached the descriptor-backed local command loop,
accepted the serial command `echo local serial works`, printed `local serial
works`, returned to `talos>`, emitted
`classification=pi5-local-literal-echo-complete`, and ended with
`rpi5-local-literal-echo-proof: PASS`.

The pre-run boot tree hash was
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
post-restore boot tree hash matched it, so the temporary control archive was
removed from the active boot tree.

This unblocks phase10-pi5-local-ls-root-proof-20260601 for a later worker wake,
provided the hardware lock remains unlocked/restored and the queued task is
promoted mechanically from its paused dependency-blocked state.

## Hardware Lock

- owner task: phase10-pi5-prompt-control-after-capture-recovery-20260601
- acquired: 2026-06-01T10:08:25Z
- released: 2026-06-01T10:09:34Z
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state.

## Validation

- static selected-control review: accepted literal echo proof identity recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- lab-controller API: pre-run status, snapshot inventory, serial cursor, TFTP
  cursor, publish, power cycle, TFTP delta, serial observe, and restore evidence
  retained.
- serialized Pi 5 known-good prompt control: retained prompt-level literal echo
  command response, next-prompt readiness, classification, and PASS.
- post-run restore proof: pre/post boot tree hashes matched.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.
- accepted commit: recorded in durable supervisor state after commit.

## Dirty Worktree Note

The repository had pre-existing unstaged `ls /` proof work outside this task
when this task started: `build.rs`, `src/target/rpi5.rs`,
`scripts/rpi5-local-ls-root-boot-tree.sh`,
`scripts/rpi5-local-ls-root-image.sh`,
`tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md`, and
`tasks/evidence/2026-06-01-pi5-local-ls-root-proof/`. Those files were not
staged or committed by this prompt-control task.
