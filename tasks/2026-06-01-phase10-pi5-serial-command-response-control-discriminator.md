# Phase 10 Pi 5 Serial Command Response Control Discriminator

Task: phase10-pi5-serial-command-response-control-discriminator-20260601
Status: accepted-input-blocked

## Goal

After the long-settle control proved netboot staging healthy, verify whether a
fresh accepted prompt-capable control also accepts post-prompt serial input and
produces command-loop output before resuming the paused ls-root hardware proof.

## Scope

This task used the already accepted prompt-capable literal-echo control archive:

- accepted archive: target/talos-rpi5-local-literal-echo-local3.tar.gz
- archive sha256:
  7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- kernel path: da591740/kernel_2712.img
- kernel sha256:
  63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826
- kernel size: 100352 bytes

No Talos runtime code, proof code, boot scripts, target routing,
command-loop behavior, lab-controller code, roadmap, ADR, or paused ls-root
proof work was changed. The ls-root candidate was not published or rerun.

## Evidence

Accepted evidence directory:

- tasks/evidence/2026-06-01-pi5-serial-command-response-control-discriminator/local1-literal-echo-control/

Key retained files:

- selected-control.txt
- archive-review.txt
- archive-sha256.txt
- archive-kernel-sha256.txt
- archive-kernel-size.txt
- pre-status.json
- pre-boot-files.json
- pre-run-snapshot-name.txt
- pre-run-snapshot.json
- publish.json
- post-publish-status.json
- post-publish-boot-files.json
- pre-serial-peek.json
- serial-cursor-before.txt
- tftp-cursor-before.txt
- power-cycle.json
- serial-observe-prompt-*.json
- serial-prompt-transcript.txt
- tftp-delta-before-input.json
- raw-enter-write-request.json
- raw-enter-write-response.json
- raw-enter-observe.json
- raw-enter-observe.txt
- raw-enter-classification.txt
- reacquire-power-cycle.json
- reacquire-serial-prompt-transcript.txt
- reacquire-tftp-delta-before-input.json
- split-command-write-request.json
- split-command-write-response.json
- split-command-observe-before-newline.json
- split-newline-write-request.json
- split-newline-write-response.json
- split-newline-observe-after-command.json
- tftp-delta-before-restore.json
- restore-snapshot.json
- post-restore-status.json
- discriminator-result.txt

## Result

Classification: accepted-input-blocked.

The lab published the selected accepted literal-echo control archive
successfully. The active boot tree changed from
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 to
b742bba067ca68395007583f57408c755484318195d2af96551d6dfa56f92465 with
configured and effective kernel kernel_2712.img.

Fresh serial output from the first control boot reached a talos> prompt on
observe loop 9. Fresh TFTP evidence from cursor 4007177 advanced to 4009879 and
included da591740/kernel_2712.img fetches at 100352 bytes. After saving a
post-prompt cursor, the raw Enter probe wrote a newline with
append_newline=false; the write endpoint returned ok, but the post-write
observe retained zero response bytes through cursor 3863862, so raw Enter was
classified raw-enter-inconclusive.

Per task scope, the worker reacquired a fresh prompt/control run before the
split command/newline probe. The second control boot reached a talos> prompt on
observe loop 9, and fresh TFTP evidence again included
da591740/kernel_2712.img fetches at 100352 bytes. The split probe wrote
echo local serial works without newline, then a separate raw newline with
append_newline=false. Both write endpoint responses returned ok, but the
post-command observe retained zero bytes from the saved command cursor; there
was no local serial works output and no next-prompt response from the input.

This accepts the task under its input-blocked outcome: the selected accepted
control fetched over TFTP and reached a fresh prompt, but both raw Enter and
split command-plus-raw-newline input forms produced no post-cursor response
bytes. The paused phase10-pi5-local-ls-root-proof-20260601 remains unpublished,
unrerun, and dependency-blocked. Supervisor planning is needed for the next
decisive discriminator, likely focused on whether lab serial writes reach the
Pi UART after the proof harness reports its prompt/FAIL boundary or whether a
new prompt-control artifact needs to keep the command loop alive after the
initial proof command window.

## Restore

The pre-run snapshot name was
phase10-pi5-command-response-control-pre-20260601T161707Z. The post-restore
boot tree hash matched the pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, so the
temporary accepted-control archive was removed from the active boot tree.

## Hardware Lock

- owner task: phase10-pi5-serial-command-response-control-discriminator-20260601
- acquired: 2026-06-01T16:11:56Z
- released: 2026-06-01T16:18:57Z
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state

## Validation

- static selected-control review: accepted literal-echo prompt control identity
  recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- candidate identity via lab-controller API: pre-publish and post-publish
  status plus boot-file evidence retained; the post-publish active boot tree
  changed and selected kernel_2712.img.
- serialized Pi 5 accepted prompt control run: two fixed-port power cycles
  produced fresh firmware/RP1 serial bytes, fresh TFTP activity, Talos output,
  and fresh talos> prompts.
- post-prompt serial input probes: raw Enter, split command text, and split
  raw newline write endpoint responses returned ok, but post-write observes
  retained zero response bytes from the saved cursors.
- TFTP delta: captured before restore from fresh cursor 4007177; events
  included da591740/kernel_2712.img fetches at 100352 bytes.
- post-run restore proof: pre/post boot tree hashes matched.
- static worktree review: no Talos source/proof-code/lab-controller changes
  were made by this discriminator; pre-existing paused ls-root proof work
  remained unstaged.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

Do not promote or rerun phase10-pi5-local-ls-root-proof-20260601. Set
planningNeeded=true for supervisor planning around the serial input blocker.
The next task should discriminate lab serial-write ingress versus an artifact
design issue where the accepted proof harness reaches a prompt but no longer
services post-proof commands.

## Dirty Worktree Note

The repository had pre-existing unstaged ls-root proof work outside this task
when this discriminator started: build.rs, src/target/rpi5.rs,
scripts/rpi5-local-ls-root-boot-tree.sh,
scripts/rpi5-local-ls-root-image.sh,
tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md, and
tasks/evidence/2026-06-01-pi5-local-ls-root-proof/. Those files were not
staged or committed by this task.
