# Phase 10 Pi 5 Prompt Control Stability Discriminator

Task: phase10-pi5-prompt-control-stability-discriminator-20260601
Status: accepted-blocked-control

## Goal

Before resuming the paused ls-root hardware proof, re-prove that an already
accepted prompt-capable Pi 5 control still reaches the Talos prompt and command
response with fresh serial and TFTP evidence.

## Scope

This task selected the already accepted bounded literal-echo Pi 5 proof as the
known-good prompt control:

- accepted prerequisite task:
  phase10-pi5-prompt-control-after-capture-recovery-20260601
- accepted prerequisite commit: b4dfbba5a7dc3ee7719111c133bc0003e8f6ffc1
- accepted archive: target/talos-rpi5-local-literal-echo-local3.tar.gz
- archive sha256:
  7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- kernel sha256:
  63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826
- kernel size: 100352 bytes

No Talos runtime code, proof code, boot script, target routing, command-loop
behavior, lab-controller code, roadmap, ADR, or ls-root candidate was changed or
rerun by this discriminator.

## Evidence

Evidence directory:

- tasks/evidence/2026-06-01-pi5-prompt-control-stability-discriminator/local1-held-literal-echo-control/

Key files:

- selected-control.txt
- archive-review.txt
- archive-sha256.txt
- archive-kernel-sha256.txt
- archive-kernel-size.txt
- pre-run-snapshot.json
- pre-serial-peek.json
- pre-tftp-tail.json
- serial-cursor-before.txt
- tftp-cursor-before.txt
- publish.json
- power-cycle.json
- tftp-delta-before-restore.json
- serial-observe-before-write.json
- serial-before-write.txt
- serial-write-literal-echo.json
- serial-observe-after-write-final.json
- serial-observe-after-write-retry.json
- serial-transcript-clean.txt
- serial-key-lines.txt
- tftp-delta-final-before-restore.json
- tftp-kernel-fetch-held-control.txt
- final-restore-snapshot-after-command.json
- final-post-restore-status-after-command.json
- control-result.txt

Accepted prompt-responsive prerequisite:

- tasks/evidence/2026-06-01-pi5-prompt-control-after-capture-recovery/local1-held-literal-echo-control/control-result.txt

## Result

Classification: accepted-blocked-control.

The selected accepted literal-echo control archive matched its recorded sha256
and passed static archive review. The lab published it successfully and the
fixed Weathertop port 8 power cycle returned ok=true. Fresh TFTP evidence from
cursor 3999071 advanced to 4000422 and served
da591740/kernel_2712.img at 100352 bytes for the selected control.

Fresh serial evidence advanced from cursor 3849734 to 3855804 and retained a
new Talos boot through the descriptor-backed local command loop. The transcript
shows:

- rpi5-local-literal-echo-proof: start ... descriptor-backed-input=true descriptor-backed-output=true
- rpi5-local-literal-echo-proof: ready command=0
- talos>

After the fresh prompt was observed, the lab serial write endpoint accepted
echo local serial works and reported 24 bytes written. A follow-up observe from
cursor 3855804 retained zero response bytes: no local serial works, no next
prompt after the command, no classification=pi5-local-literal-echo-complete,
and no rpi5-local-literal-echo-proof: PASS.

This is a prompt-visible but command-response-blocked control, not evidence
against the paused ls-root feature semantics. The ls-root candidate remained
unpublished and unrereun.

## Restore

The pre-run boot tree hash was
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The final
post-restore boot tree hash matched it, so the temporary control archive was
removed from the active boot tree.

## Hardware Lock

- owner task: phase10-pi5-prompt-control-stability-discriminator-20260601
- acquired: 2026-06-01T11:22:00Z
- released: recorded in durable supervisor state after restore.
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state.

## Validation

- static selected-control review: accepted literal-echo prompt control identity
  recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- static worktree review: no Talos source/proof-code changes were made by this
  discriminator; pre-existing paused ls-root proof work remained unstaged.
- serialized Pi 5 known-good control run: blocked-control evidence retained;
  fresh TFTP fetch succeeded, a fresh Talos prompt appeared, but command write
  produced zero response bytes.
- TFTP evidence: delta captured before restore and final delta retained.
- post-run restore proof: pre/post boot tree hashes matched.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

Keep phase10-pi5-local-ls-root-proof-20260601 paused. Supervisor planning should
choose the next decisive discriminator for the serial input/command-response
blocker. Do not change or rerun ls-root until a known-good prompt-capable control
produces fresh command-response evidence again, or the supervisor explicitly
plans a different bounded recovery step.

## Dirty Worktree Note

The repository had pre-existing unstaged ls-root proof work outside this task
when this discriminator started: build.rs, src/target/rpi5.rs,
scripts/rpi5-local-ls-root-boot-tree.sh,
scripts/rpi5-local-ls-root-image.sh,
tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md, and
tasks/evidence/2026-06-01-pi5-local-ls-root-proof/. Those files were not staged
or committed by this task.
