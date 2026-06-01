# Phase 10 Pi 5 Netboot Staging Control Discriminator

Task: phase10-pi5-netboot-staging-control-discriminator-20260601
Status: accepted-netboot-staging-blocked

## Goal

Determine whether the Pi 5 lab is fetching the staged active boot tree after a
publish/restore cycle before any further `ls /` candidate rerun.

## Scope

This task selected the already accepted bounded literal-echo Pi 5 proof as the
prompt-capable control:

- accepted prerequisite task:
  phase10-pi5-serial-input-ingress-discriminator-20260601
- accepted prerequisite commit: 7921cd79072a3a9afae972773bebce6c081629ca
- accepted archive: target/talos-rpi5-local-literal-echo-local3.tar.gz
- archive sha256:
  7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- kernel sha256:
  63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826
- kernel size: 100352 bytes

No Talos runtime code, proof code, boot script, target routing,
command-loop behavior, lab-controller code, roadmap, ADR, or paused `ls /`
proof work was changed. The `ls /` candidate was not published or rerun.

## Evidence

Evidence directory:

- tasks/evidence/2026-06-01-pi5-netboot-staging-control-discriminator/local1-literal-echo-staging-control/

Key files:

- selected-control.txt
- archive-review.txt
- archive-sha256.txt
- archive-kernel-sha256.txt
- archive-kernel-size.txt
- pre-status.json
- pre-boot-files.json
- pre-snapshots.json
- pre-run-snapshot-name.txt
- pre-run-snapshot.json
- publish.json
- post-publish-status.json
- post-publish-boot-files.json
- pre-serial-peek.json
- serial-cursor-before.txt
- pre-tftp-tail.json
- tftp-cursor-before.txt
- power-cycle.json
- serial-observe-after-power.json
- serial-after-power.txt
- tftp-delta-before-restore.json
- tftp-kernel-fetch-control.txt
- restore-snapshot.json
- post-restore-status.json
- staging-result.txt

## Result

Classification: accepted-netboot-staging-blocked.

The selected accepted literal-echo control archive matched its recorded sha256
and passed static archive review. The lab published it successfully with boot
tree hash b742bba067ca68395007583f57408c755484318195d2af96551d6dfa56f92465,
configured kernel `kernel_2712.img`, and effective kernel
`kernel_2712.img`. The published tree differed from the pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and
`post-publish-boot-files.json` records the expected Pi 5 archive contents,
including root and `da591740/` mirrored kernel files.

The fixed Weathertop port 8 power cycle returned ok=true. Fresh serial evidence
advanced from cursor 3857220 to 3857868 and retained Raspberry Pi firmware/RP1
boot text, including `RPi: BOOTSYS`, `BOOTMODE`, and `RP1_BOOT`. Fresh TFTP
evidence from cursor 4003124 did not advance and retained zero events, so the
published accepted control was not fetched in this run and no Talos
kernel-entry or `talos>` prompt evidence appeared.

No documented named known-good boot snapshot was available in the task source
or project docs, so no extra known-good snapshot control was run. The next
queued serial command-response discriminator remains mechanically blocked
because this task did not accept staging as healthy. The paused
phase10-pi5-local-ls-root-proof-20260601 remains unpublished and unrereun, and
supervisor planning is needed for the next bounded lab/netboot recovery step.

## Restore

The pre-run snapshot name was
`phase10-pi5-netboot-staging-control-pre-20260601T1424Z`. The post-restore
boot tree hash matched the pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, so the
temporary accepted-control archive was removed from the active boot tree.

## Hardware Lock

- owner task: phase10-pi5-netboot-staging-control-discriminator-20260601
- acquired: 2026-06-01T14:23:00Z
- released: 2026-06-01T14:26:00Z
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state.

## Validation

- static selected-control review: accepted literal-echo prompt control identity
  recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- candidate identity via lab-controller API: pre-publish and post-publish
  status plus boot-file evidence retained; the post-publish active boot tree
  changed and selected `kernel_2712.img`.
- serialized Pi 5 accepted prompt control run: fixed-port power cycle produced
  fresh firmware/RP1 serial bytes, but no fresh TFTP delta and no Talos prompt.
- TFTP delta: captured before restore from fresh cursor 4003124; zero events.
- post-run restore proof: pre/post boot tree hashes matched.
- static worktree review: no Talos source/proof-code/lab-controller changes
  were made by this discriminator; pre-existing paused `ls /` proof work
  remained unstaged.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

Keep phase10-pi5-local-ls-root-proof-20260601 paused. Supervisor planning is
needed because the lab produced fresh firmware serial output but did not show
fresh TFTP fetches for the published active control tree.

## Dirty Worktree Note

The repository had pre-existing unstaged `ls /` proof work outside this task
when this discriminator started: `build.rs`, `src/target/rpi5.rs`,
`scripts/rpi5-local-ls-root-boot-tree.sh`,
`scripts/rpi5-local-ls-root-image.sh`,
`tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md`, and
`tasks/evidence/2026-06-01-pi5-local-ls-root-proof/`. Those files were not
staged or committed by this task.
