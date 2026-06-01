# Phase 10 Pi 5 Serial Write Ingress Control Proof

Task: phase10-pi5-serial-write-ingress-control-proof-20260601
Status: accepted-input-responsive

## Goal

Carry the accepted prompt-live serial-write ingress control to serialized
Raspberry Pi 5 evidence before resuming the paused `ls /` hardware proof.

## Scope

This task selected the already accepted literal-echo Pi 5 control archive as
the hardware equivalent of the prompt-live local/QEMU ingress control:

- accepted local control task:
  phase10-local-serial-write-ingress-control-core-20260601
- selected archive: `target/talos-rpi5-local-literal-echo-local3.tar.gz`
- archive sha256:
  `7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5`
- kernel sha256:
  `63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826`
- kernel size: 100352 bytes

No Talos runtime code, proof code, boot scripts, target routing,
command-loop behavior, lab-controller code, roadmap, ADR, or paused ls-root
proof work was changed. The paused `ls /` candidate was not published or
rerun.

## Evidence

Accepted local2 evidence:

- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/proof-result.txt`
- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/serial-prompt-transcript.txt`
- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/serial-write-literal-echo-response.json`
- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/serial-response-transcript.txt`
- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/tftp-delta-before-restore.json`
- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/archive-review.txt`
- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/restore-snapshot.json`
- `tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local2-clean-prompt-live-literal-echo-control/post-restore-status.json`

Superseded local1 evidence is retained under
`tasks/evidence/2026-06-01-pi5-serial-write-ingress-control-proof/local1-prompt-live-literal-echo-control/`.
It restored the boot tree, but its prompt/write cursor pairing was noisy after
an interrupted orchestration attempt, so it is not acceptance evidence.

## Result

Classification: accepted-input-responsive.

The selected control archive matched the previously accepted literal-echo
archive and passed static archive review. The lab published it successfully,
changing the active tree from
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` to
`b742bba067ca68395007583f57408c755484318195d2af96551d6dfa56f92465`.
Fresh TFTP evidence from cursor 4011230 includes
`da591740/kernel_2712.img` served at 100352 bytes.

Fresh serial evidence reached
`rpi5-local-literal-echo-proof: ready command=0` and a visible `talos>`
prompt. The worker saved cursor 3888843 after that prompt, then wrote
`echo local serial works` through the lab serial endpoint. The post-write
observe from cursor 3888843 retained 554 bytes, including visible
`local serial works`, the descriptor-backed literal-echo summary,
`ready-for-next prompt=true`, final
`classification=pi5-local-literal-echo-complete`, and
`rpi5-local-literal-echo-proof: PASS`.

This proves the prompt-live control was still servicing serial input after the
visible prompt boundary. Exactly one unchanged rerun of
`phase10-pi5-local-ls-root-proof-20260601` is now unblocked, provided the
hardware lock remains unlocked/restored on the next worker wake.

## Restore

The local2 pre-run snapshot was
`phase10-pi5-serial-write-ingress-control-pre2-20260601T175219Z`. The
post-restore tree hash matched the pre-run tree hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

## Hardware Lock

- owner task: phase10-pi5-serial-write-ingress-control-proof-20260601
- acquired: 2026-06-01T17:43:08Z
- released: 2026-06-01T17:53:10Z
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state

## Validation

- static selected-control review: accepted literal-echo prompt control identity
  recorded; no `ls /` candidate was published or rerun.
- static archive/image review: `scripts/rpi5-archive-review.sh` passed for
  the selected control archive.
- candidate identity via lab-controller API: pre-publish and post-publish
  status plus boot-file evidence retained; post-publish active boot tree
  changed to the selected control archive.
- serialized Pi 5 hardware proof: local2 retained prompt-live serial evidence,
  post-prompt serial write response, descriptor-backed command output,
  next-prompt readiness, final classification, and PASS.
- TFTP delta: local2 retained selected-control fetch evidence for
  `da591740/kernel_2712.img` at 100352 bytes before restore.
- post-run restore proof: pre/post boot tree hashes matched.
- static worktree review: no Talos source/proof-code/lab-controller changes
  were made by this proof; pre-existing paused ls-root proof work remained
  unstaged.
- static inspection: `git diff --check` passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: `git diff --cached --check` passed before commit.

## Dirty Worktree Note

The repository had pre-existing unstaged ls-root proof work outside this task
when this proof started: `build.rs`, `src/target/rpi5.rs`,
`scripts/rpi5-local-ls-root-boot-tree.sh`,
`scripts/rpi5-local-ls-root-image.sh`,
`tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md`, and
`tasks/evidence/2026-06-01-pi5-local-ls-root-proof/`. Those files were not
staged or committed by this task.

## Next Action

Promote exactly one unchanged `phase10-pi5-local-ls-root-proof-20260601`
rerun on the next worker wake if hardwareTestLock remains unlocked/restored.
