# Phase 10 Pi 5 TFTP Fetch Freshness Control Discriminator

Task: phase10-pi5-tftp-fetch-freshness-control-discriminator-20260602
Status: blocked-no-fresh-tftp

## Goal

Decide whether the Pi 5 lab can still produce fresh TFTP boot-file fetch
evidence for an already accepted prompt-capable local-interactivity control
before any cat-banner proof strategy changes or candidate rerun.

## Scope

This task used the accepted literal-echo prompt-capable control archive from
`phase10-pi5-serial-write-ingress-control-proof-20260601`:

- selected archive: `target/talos-rpi5-local-literal-echo-local3.tar.gz`
- selected control commit: `0aceae6708ac57b027184c436603bc24308e5d92`
- archive sha256:
  `7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5`
- kernel sha256:
  `63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826`

No Talos runtime code, proof harnesses, cat-banner behavior, parser behavior,
filesystem/syscall behavior, userspace execution, lab-controller code,
roadmap, or ADRs were changed. The cat-banner candidate was not rerun.

## TFTP Cursor Method

The run captured the pre-power TFTP cursor by calling `/tftp/logs` with the
cursor omitted and `limit=1`, then used the returned `tftp.cursor_end` as the
delta cursor:

- pre-run tail artifact:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/tftp-cursor-start.json`
- cursor: `4038250`
- before-restore delta artifact:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/tftp-delta-before-restore.json`

The before-restore delta used `/tftp/logs?cursor=4038250&limit=2000&max_bytes=1048576`.
It returned `cursor_start=4038250`, `cursor_end=4038250`, `events=[]`, and
`lines=[]`.

## Evidence

Local1 evidence:

- summary:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/discriminator-result.txt`
- archive review:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/archive-review.txt`
- pre-run status:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/pre-status.json`
- post-publish status:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/post-publish-status.json`
- power-cycle:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/power-cycle.json`
- serial transcript:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/serial-transcript.txt`
- no-response artifact:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/no-response-preentry-artifact.txt`
- post-restore status:
  `tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/local1-accepted-literal-echo-control/post-restore-status.json`

The lab published the selected accepted literal-echo archive and the boot tree
changed from pre-run hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` to
published hash
`b742bba067ca68395007583f57408c755484318195d2af96551d6dfa56f92465`.
The fixed-port Weathertop port 8 power cycle returned `ok=true`.

Fresh serial from cursor `3996319` captured 708 bytes of Raspberry Pi
firmware/RP1 reboot output, proving a power-cycle/serial-reboot event from the
fresh cursor. The transcript did not show a Talos prompt, accepted literal echo
response, or `rpi5-local-literal-echo-proof: PASS`. Per task scope, no serial
write was attempted without a prompt.

The decisive blocker is the before-restore TFTP delta: it returned zero fresh
events and zero expected boot-file requests from the pre-run cursor. Therefore
this run satisfies only the `blocked-no-fresh-tftp` path and does not unblock
the unchanged cat-banner rerun.

## Restore

The pre-run snapshot was
`phase10-pi5-tftp-fetch-freshness-control-pre-20260602T0354Z`. The boot tree
was restored after the run. The post-restore tree hash was
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
matching the pre-run hash.

## Hardware Lock

- owner task: phase10-pi5-tftp-fetch-freshness-control-discriminator-20260602
- acquired: 2026-06-02T03:54:36Z
- released: 2026-06-02T03:54:43Z
- release status: hardwareTestLock released/unlocked/restored in durable
  supervisor state

## Validation

- image/archive inspection: `scripts/rpi5-archive-review.sh` passed for the
  selected accepted literal-echo control archive.
- lab-controller API: pre/post status, snapshots, publish, boot files, power
  cycle, serial, TFTP, and restore artifacts are retained.
- serial hardware boot/output: fresh firmware/RP1 reboot output was retained
  from cursor `3996319`; no Talos prompt or command response was retained.
- TFTP delta: before-restore `/tftp/logs?cursor=4038250` returned zero fresh
  events, so the task is classified `blocked-no-fresh-tftp`.
- restore proof: post-restore lab API status returned to the pre-run tree hash.
- static inspection: `git diff --check` passed.
- documentation: mdBook was not required because mdBook docs were not touched.
- staged static inspection: `git diff --cached --check` passed before commit.

## Supervisor Intervention Checklist

The 2026-06-02 supervisor intervention checklist is retained at:

`tasks/evidence/2026-06-02-pi5-tftp-fetch-freshness-control-discriminator/intervention-checklist.md`

It restates the first-principles no-fresh-TFTP problem, the expected
network-boot/TFTP cursor invariant, contradicting evidence from the two
prompt-baseline control attempts and this discriminator, unproven boot-source
and logging assumptions, qualitatively different next approaches, the smallest
decisive discriminator, and the quarantine plan forbidding Talos runtime,
proof-harness, candidate archive, marker, wait, or acceptance-criteria changes.
A later replay of saved cursor `4038250` returned the expected 03:54:59-03:55:01 TFTP boot-file request sequence. Treat the immediate zero-delta artifact as a TFTP log collection timing problem unless a future settled cursor query remains empty and live-tail health is also proven.

## Next Action

Supervisor planning is required. Do not rerun the unchanged cat-banner
candidate or change cat proof strategy until the lab/control no-fresh-TFTP
blocker has an explicit next discriminator or recovery task.
