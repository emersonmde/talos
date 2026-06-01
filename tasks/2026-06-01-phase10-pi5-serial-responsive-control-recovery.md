# Phase 10 Pi 5 Serial Responsive Control Recovery

Task: phase10-pi5-serial-responsive-control-recovery-20260601
Status: accepted-blocked-control

## Goal

Recover or prove a responsive known-good Pi 5 serial prompt/control before
resuming the paused ls-root hardware proof.

## Scope

This task used the already accepted held literal-echo prompt control:

- control task: phase10-pi5-accepted-prompt-control-replay-held-20260601
- accepted control commit: 9f02d5436b5b7d00f21a023f6bd90d725d8ccf34
- accepted literal-echo proof commit:
  29b8b7d6afbc57dc156db593c376aef36640ebb1
- archive: target/talos-rpi5-local-literal-echo-local3.tar.gz
- archive sha256:
  7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- kernel sha256:
  63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826
- kernel size: 100352 bytes

No ls-root runtime/proof code, command-loop behavior, archive scripts, boot
layout, or target routing was changed, and the ls-root candidate was not rerun.

## Evidence

Blocked-control local1 evidence:

- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/selected-control.txt
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/archive-review.txt
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/archive-sha256.txt
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/archive-kernel-sha256.txt
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/archive-kernel-size.txt
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/pre-run-snapshot.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/pre-serial-peek.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/publish.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/power-cycle.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/tftp-delta-after-power.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/serial-write-literal-echo.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/serial-observe-through-pass.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/serial-transcript.txt
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/post-restore-serial-peek-20k.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/tftp-delta-final-before-restore.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/restore-snapshot.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/post-restore-status.json
- tasks/evidence/2026-06-01-pi5-serial-responsive-control-recovery/local1-held-literal-echo-control/control-result.txt

## Result

Classification: accepted-blocked-control.

The lab published the selected accepted literal-echo archive and power-cycled
the Pi 5. Fresh TFTP evidence from cursor 3990965 advanced to 3992316 and
served `da591740/kernel_2712.img` at 100352 bytes, matching the selected
control kernel size.

The serial path did not produce fresh retained bytes for the control run:
`serial-observe-through-pass.json` records cursor_start=3840874,
cursor_end=3840874, and bytes=0 after the power cycle and command write.
`post-restore-serial-peek-20k.json` still reports cursor=3840874, confirming
the retained serial cursor did not advance even though TFTP and boot staging
changed normally.

This blocks the paused ls-root hardware proof dependency. The failure is not
evidence against ls-root semantics and does not justify ls-root runtime or proof
harness changes.

## Hypothesis Review

- Serial capture: primary suspect. The retained serial cursor stayed fixed at
  3840874 across power-cycle, command write, and post-restore peek.
- Boot staging: less likely for this control because publish succeeded and TFTP
  served the selected 100352-byte control kernel.
- Power-cycle: less likely because the lab API accepted the power cycle and the
  selected boot tree was fetched afterward.
- TFTP: healthy for this run; fresh events were captured for the selected
  control archive.
- Talos runtime: not proven unhealthy by this task. The same archive previously
  produced accepted prompt-responsive evidence, and this run lacks fresh serial
  bytes rather than showing a fresh Talos failure.

Smallest next discriminator: perform a documented serial-capture recovery action
for the lab collector/service or an independent serial capture path, then prove
the cursor advances on a known-good prompt-capable control before resuming the
paused ls-root proof.

## Hardware Lock

- owner task: phase10-pi5-serial-responsive-control-recovery-20260601
- acquired: 2026-06-01T08:21:37Z
- released: recorded in durable supervisor state after restore.
- restore status: PASS; post-restore tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Validation

- static selected-control review: accepted literal-echo prompt control identity
  recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed.
- serialized Pi 5 known-good control run: blocked-control evidence retained;
  fresh TFTP fetch succeeded, but serial cursor did not advance.
- post-run restore proof: named boot snapshot restore returned the pre-run tree
  hash.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

Keep phase10-pi5-local-ls-root-proof-20260601 paused. Supervisor planning
should choose the next serial-capture recovery discriminator. Do not change
ls-root code or rerun the ls-root candidate until a known-good prompt-capable
control produces fresh retained serial output again.
