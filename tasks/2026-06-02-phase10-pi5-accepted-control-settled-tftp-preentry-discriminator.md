# Phase 10 Pi 5 Accepted-Control Settled TFTP Pre-Entry Discriminator

Task: phase10-pi5-accepted-control-settled-tftp-preentry-discriminator-20260602
Status: accepted-prompt-responsive

## Goal

Re-run the already accepted prompt-capable literal-echo Pi 5 control with
settled same-cursor TFTP collection before any cat-banner proof strategy change
or unchanged cat-banner rerun.

## Scope

This task selected the accepted literal-echo control archive from
`phase10-pi5-serial-write-ingress-control-proof-20260601`:

- selected archive: `target/talos-rpi5-local-literal-echo-local3.tar.gz`
- selected control commit: `0aceae6708ac57b027184c436603bc24308e5d92`
- archive sha256:
  `7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5`
- kernel sha256:
  `63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826`

No Talos runtime code, proof harness code, command-loop behavior,
cat-banner behavior, parser behavior, filesystem/syscall behavior,
userspace execution, lab-controller code, roadmap, or ADR was changed. The
cat-banner candidate was not rerun.

## TFTP Cursor Method

Each hardware attempt captured the TFTP cursor before power-cycle by omitting
`cursor` from `/tftp/logs` and retaining `tftp.cursor_end`. The accepted
local3 run used cursor `4040952` and collected the same cursor repeatedly
before restore.

Accepted local3 TFTP evidence:

- pre-run tail:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/tftp-cursor-start.json`
- settle loop summary:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/tftp-settle-loop-summary.txt`
- final before-restore delta:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/tftp-delta-before-restore.json`

The settled local3 loop returned 13 events on all 12 same-cursor queries,
with `cursor_end=4042303`. Expected fresh boot-file requests included
`da591740/config.txt`, `da591740/kernel_2712.img` at 100352 bytes,
`da591740/bcm2712-rpi-5-b.dtb`, overlays, and `da591740//cmdline.txt`.
This resolves the immediate-empty TFTP artifact from the prior discriminator
as collection timing rather than lack of a network boot fetch.

## Evidence

Accepted local3 evidence:

- result:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/discriminator-result.txt`
- serial response transcript:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/serial-transcript.txt`
- serial write request/response:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/serial-write-literal-echo-request.json`
  and
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/serial-write-literal-echo-response.json`
- archive review:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/archive-review.txt`
- pre/post status and restore:
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/pre-status.json`,
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/restore-snapshot.json`, and
  `tasks/evidence/2026-06-02-pi5-accepted-control-settled-tftp-preentry-discriminator/local3-delayed-write-accepted-literal-echo-control/post-restore-status.json`

The accepted local3 run classified `accepted-prompt-responsive`. It used a
single delayed serial write 28 seconds after the fixed-port power-cycle, after
local2 showed the prompt proof can timeout if the worker waits to observe the
prompt before writing. The local3 serial transcript retained the visible
`local serial works` response, `talos>` next prompt, ready-for-next evidence,
`classification=pi5-local-literal-echo-complete`, and
`rpi5-local-literal-echo-proof: PASS`. The serial observation did not retain
the full boot log because observation began after the delayed write, but the
same local3 run retained fresh settled TFTP fetches for the selected control
archive and post-restore status. Local2 retained a fresh prompt/input-timeout
failure with settled TFTP, and local4 documents that an over-large
`settle_ms=60000` observe request is rejected by the lab API.

## Attempts

- local1: aborted before publish/power because the worker used `GET /`, which
  returned 404 in the deployed lab API. Hardware lock was released; no candidate
  was staged.
- local2: settled TFTP collection worked, but prompt/write orchestration waited
  until the control timed out, yielding `input-error` and `FAIL`. This is not
  acceptance evidence.
- local3: accepted prompt-responsive evidence with fresh settled TFTP and a
  visible literal-echo response/PASS.
- local4: attempted to capture boot and write in one concurrent observe, but
  `settle_ms=60000` was rejected with HTTP 400. TFTP remained healthy; this is
  not acceptance evidence.

## Restore

The accepted local3 pre-run tree hash was
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
Post-restore status returned to the same tree hash with
`effective_kernel=kernel_2712.img`. The hardware lock was released and marked
restored in durable supervisor state.

## Validation

- image/archive inspection: `scripts/rpi5-archive-review.sh` passed for the
  selected accepted literal-echo control archive.
- lab-controller API: pre/post status, snapshots, publish, boot files,
  fixed-port power-cycle, serial write/observe, settled same-cursor TFTP, and
  restore artifacts are retained.
- serial hardware boot/output: local3 retained fresh selected-control command
  response output and PASS after a serial write; local2 retained the prompt
  timeout failure that motivated the delayed-write retry.
- TFTP evidence: local3 retained settled same-cursor fresh boot-file requests
  before restore.
- restore proof: local3 pre/post boot tree hashes matched.
- static inspection: `git diff --check` passed.
- documentation: mdBook was not required because mdBook docs were not touched.
- staged static inspection: `git diff --cached --check` passed before commit.

## Next Action

The unchanged cat-banner rerun
`phase10-pi5-local-cat-banner-unchanged-rerun-after-control-20260602` is now
mechanically unblocked on the next worker wake if hardwareTestLock remains
unlocked/restored.
