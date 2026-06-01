# Phase 10 Pi 5 Netboot Settle Window Control

Task: phase10-pi5-netboot-settle-window-control-20260601
Status: accepted-staging-healthy

## Goal

Rerun the accepted prompt-capable literal-echo control with a deliberately long
post-power-cycle settle window to determine whether the prior no-TFTP staging
result was caused by observing too early.

## Scope

This task used the already accepted prompt-capable control archive:

- accepted archive: target/talos-rpi5-local-literal-echo-local3.tar.gz
- archive sha256:
  7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- kernel path: da591740/kernel_2712.img
- kernel sha256:
  63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826
- kernel size: 100352 bytes

No Talos runtime code, proof code, boot script, target routing,
command-loop behavior, lab-controller code, roadmap, ADR, or paused `ls /`
proof work was changed. The `ls /` candidate was not published or rerun.

## Evidence

Accepted evidence directory:

- tasks/evidence/2026-06-01-pi5-netboot-settle-window-control/local2-long-settle-literal-echo-control/

Key accepted files:

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
- serial-observe-loop-*.json
- serial-observe-long-settle.json
- serial-long-settle.txt
- tftp-delta-before-restore.json
- tftp-kernel-fetch-control.txt
- restore-snapshot.json
- post-restore-status.json
- settle-result.txt

## Result

Classification: accepted-staging-healthy.

The selected accepted literal-echo control archive matched its recorded sha256
and passed static archive review. The lab published it successfully with boot
tree hash b742bba067ca68395007583f57408c755484318195d2af96551d6dfa56f92465,
configured kernel `kernel_2712.img`, and effective kernel
`kernel_2712.img`. The published tree differed from the pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and
`post-publish-boot-files.json` records the expected Pi 5 archive contents,
including root and `da591740/` mirrored kernel files.

The fixed-port power cycle returned ok=true. Fresh serial evidence advanced
from cursor 3858516 to 3859737 and retained Raspberry Pi firmware/RP1 boot
text followed by Talos output and a fresh `talos>` prompt. Fresh TFTP evidence
from cursor 4005826 advanced to 4007177 with 13 events, including
`da591740/kernel_2712.img` fetches at 100352 bytes. The prompt appeared after
41 seconds, so the long-settle gate ended early under the task's explicit
prompt/PASS exception.

This run proves the active staged tree can be fetched after a longer settle
window. The next queued command-response control discriminator is mechanically
unblocked. The paused phase10-pi5-local-ls-root-proof-20260601 remains
unpublished and unrereun.

## Restore

The pre-run snapshot name was
`phase10-pi5-netboot-settle-control-pre2-20260601T155641Z`. The post-restore
boot tree hash matched the pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, so the
temporary accepted-control archive was removed from the active boot tree.

## Hardware Lock

- owner task: phase10-pi5-netboot-settle-window-control-20260601
- acquired: 2026-06-01T15:51:18Z
- released: 2026-06-01T15:58:39Z
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state

## Validation

- static selected-control review: accepted literal-echo prompt control identity
  recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- candidate identity via lab-controller API: pre-publish and post-publish
  status plus boot-file evidence retained; the post-publish active boot tree
  changed and selected `kernel_2712.img`.
- serialized Pi 5 accepted prompt control run: fixed-port power cycle produced
  fresh firmware/RP1 serial bytes, fresh TFTP activity, Talos output, and a
  fresh `talos>` prompt.
- long-settle serial observation: prompt appeared after 41 seconds, satisfying
  the explicit early-completion exception.
- TFTP delta: captured before restore from fresh cursor 4005826; 13 events,
  including `da591740/kernel_2712.img` fetches at 100352 bytes.
- post-run restore proof: pre/post boot tree hashes matched.
- static worktree review: no Talos source/proof-code/lab-controller changes
  were made by this discriminator; pre-existing paused `ls /` proof work
  remained unstaged.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

Promote phase10-pi5-serial-command-response-control-discriminator-20260601 on
the next worker wake if hardwareTestLock remains unlocked/restored. Keep
phase10-pi5-local-ls-root-proof-20260601 paused until command-response evidence
accepts one unchanged `ls /` candidate rerun.

## Dirty Worktree Note

The repository had pre-existing unstaged `ls /` proof work outside this task
when this discriminator started: `build.rs`, `src/target/rpi5.rs`,
`scripts/rpi5-local-ls-root-boot-tree.sh`,
`scripts/rpi5-local-ls-root-image.sh`,
`tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md`, and
`tasks/evidence/2026-06-01-pi5-local-ls-root-proof/`. Those files were not
staged or committed by this task.
