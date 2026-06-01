# Phase 10 Pi 5 Serial Input Ingress Discriminator

Task: phase10-pi5-serial-input-ingress-discriminator-20260601
Status: accepted-capture-staging-blocked

## Goal

Determine whether a prompt-visible accepted Pi 5 Talos control can receive
serial input and produce post-prompt command-loop output before any ls-root
candidate rerun.

## Scope

This task selected the already accepted bounded literal-echo Pi 5 proof as the
known-good prompt-capable control:

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
behavior, lab-controller code, roadmap, ADR, or ls-root candidate was changed
or rerun by this discriminator.

## Evidence

Evidence directory:

- tasks/evidence/2026-06-01-pi5-serial-input-ingress-discriminator/

Key files:

- selected-control.txt
- archive-review.txt
- archive-sha256.txt
- archive-kernel-sha256.txt
- archive-kernel-size.txt
- pre-status.json
- pre-run-snapshot.json
- local1-raw-enter-control/pre-serial-peek.json
- local1-raw-enter-control/pre-tftp-tail.json
- local1-raw-enter-control/publish.json
- local1-raw-enter-control/post-publish-status.json
- local1-raw-enter-control/power-cycle.json
- local1-raw-enter-control/serial-observe-through-prompt.json
- local1-raw-enter-control/serial-through-prompt.txt
- local1-raw-enter-control/tftp-delta-before-restore.json
- tftp-delta-final-before-restore.json
- restore-snapshot.json
- post-restore-status.json
- discriminator-result.txt

Accepted prompt-control stability blocker reference:

- tasks/evidence/2026-06-01-pi5-prompt-control-stability-discriminator/local1-held-literal-echo-control/control-result.txt

## Result

Classification: accepted-capture-staging-blocked.

The selected accepted literal-echo control archive matched its recorded sha256
and passed static archive review. The lab published it successfully with boot
tree hash b742bba067ca68395007583f57408c755484318195d2af96551d6dfa56f92465,
configured kernel kernel_2712.img, and effective kernel kernel_2712.img. The
fixed Weathertop port 8 power cycle returned ok=true.

Fresh serial evidence advanced from cursor 3856512 to 3857220 and retained a
new Raspberry Pi firmware/RP1 boot prefix, but no Talos kernel-entry bytes,
no rpi5-local-literal-echo-proof start marker, and no talos> prompt. Fresh TFTP
evidence from cursor 4001773 did not advance and retained zero events, so the
accepted control was not proven to fetch the staged kernel in this run.

Because no fresh prompt was reached, the raw Enter and split command/raw-newline
input probes were not attempted. This task therefore does not classify serial
input ingress. It classifies the prerequisite prompt/staging path as blocked
for this run and leaves the paused ls-root proof unpublished and unrereun.

## Restore

The pre-run boot tree hash was
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
post-restore boot tree hash matched it, so the temporary control archive was
removed from the active boot tree.

## Hardware Lock

- owner task: phase10-pi5-serial-input-ingress-discriminator-20260601
- acquired: recorded in durable supervisor state
- released: recorded in durable supervisor state after restore
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state

## Validation

- static selected-control review: accepted literal-echo prompt control identity
  recorded.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  accepted local3 literal-echo archive.
- static worktree review: no Talos source/proof-code/lab-controller changes
  were made by this discriminator; pre-existing paused ls-root proof work
  remained unstaged.
- serialized Pi 5 accepted prompt control run: capture/staging-blocked evidence
  retained; fixed-port power cycle produced fresh firmware/RP1 serial bytes,
  but no fresh TFTP delta and no fresh Talos prompt.
- post-run restore proof: pre/post boot tree hashes matched.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Recommendation

Keep phase10-pi5-local-ls-root-proof-20260601 paused. Supervisor planning is
needed for the next decisive discriminator because the selected prompt-capable
control was not fetched over TFTP and did not reach a fresh Talos prompt in
this run. Do not change or rerun ls-root until a known-good prompt-capable
control again produces fresh command-response evidence, or the supervisor
explicitly plans a different bounded recovery step.

## Dirty Worktree Note

The repository had pre-existing unstaged ls-root proof work outside this task
when this discriminator started: build.rs, src/target/rpi5.rs,
scripts/rpi5-local-ls-root-boot-tree.sh,
scripts/rpi5-local-ls-root-image.sh,
tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md, and
tasks/evidence/2026-06-01-pi5-local-ls-root-proof/. Those files were not staged
or committed by this task.
