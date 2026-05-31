# Phase 10 Pi 5 Local Line-Cancel Proof

Task: phase10-pi5-local-line-cancel-proof-20260531
Status: accepted

## Goal

Carry the accepted Ctrl-C local line-cancel feature to serialized Raspberry Pi
5 serial hardware evidence.

## Scope

This task added only the narrow Pi 5 proof harness support needed to exercise
the accepted descriptor-backed line-cancel behavior on physical serial:
partial input, Ctrl-C, a visible cancellation response, a fresh prompt,
following pwd dispatch, visible slash output, descriptor-backed input/output
markers, next-prompt readiness, final classification, and PASS.

It does not accept POSIX signal delivery, process interruption, userspace shell
execution, process spawning, job control, terminal sessions, termios,
filesystem-backed command lookup, broad escape-sequence parsing, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Implementation

- Added the rpi5_local_line_cancel boot scenario and proof labels.
- Reused the accepted descriptor-backed command loop and canonical-lite TTY
  Ctrl-C handling.
- Added proof-local visible replay for talos: line-canceled, matching the
  existing Pi 5 proof replay pattern for command responses.
- Added Pi 5 line-cancel image and boot-tree helper scripts.

Implementation commits:

- cb2a362712f9e8b803d4cd4e0aa98593bde2b49e
- 15dbf99226a231d1ef80663e90f233be4aea7cff

## Evidence

Accepted Pi 5 evidence:

- tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/serial-transcript-through-pass.txt
- tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/proof-result-local3.txt
- tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/proof-lines-local3.txt
- tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/tftp-kernel-fetch-local3.txt
- tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/archive-review.txt
- tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/post-snapshot-restore-status.json

The accepted local3 proof records:

- source commit: 15dbf99226a231d1ef80663e90f233be4aea7cff
- archive sha256:
  39791d161a9466fee248ca83a8175415db6f1089f40dbb8272ddbff41bbae854
- kernel sha256:
  8e47209f4248ea04fdfe005892a4ec2f346cb52d2535c8fdb98d3fd3345c6f75
- kernel size: 99552 bytes
- published boot tree hash:
  73024a9208246c0f4f55e122db913f7db582ee6753962ecd8131b4155b93155d
- restored boot tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

The retained serial transcript shows the rpi5-local-line-cancel-proof
scenario start, descriptor-backed input/output markers, ready command=0,
partial input canceled with control telemetry, visible talos: line-canceled,
ready command=1, following pwd, visible slash output, ready-for-next
prompt=true, final pi5-local-line-cancel-complete classification, and
rpi5-local-line-cancel-proof: PASS.

## Inconclusive-Run Triage

The first candidate attempt did not produce complete acceptance evidence before
the capture loop became stale. Before changing proof code, the task recorded:

- candidate identity and archive/image review in
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local1-clean-candidate/
- fresh serial cursor and TFTP evidence from the candidate attempt.
- restored accepted boot-tree control PASS in
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local1-known-good-control/control-result.txt.
- unchanged candidate rerun evidence in
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local2-unchanged-candidate-rerun/.

The local2 unchanged rerun reached the internal PASS path and restored the boot
tree, but its retained transcript did not preserve the explicit
talos: line-canceled response required by this task. The follow-up local3
candidate used only a proof-local visibility adjustment and supplied the
accepted visible cancellation transcript.

## Hardware Lock

- owner task: phase10-pi5-local-line-cancel-proof-20260531
- hardware action: serialized Pi 5 boot archive publication, power cycle,
  serial observe/write, and restore.
- restore status: local3 restored the prior accepted boot tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardwareTestLock release.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute regression: scripts/qemu-local-line-cancel-smoke.sh --quiet
  passed.
- QEMU/substitute Backspace/Delete regression:
  scripts/qemu-local-line-editing-smoke.sh --quiet passed.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  local3 archive.
- serialized Pi 5 hardware proof: local3 retained visible cancel, following
  pwd, classification, and PASS.
- restore proof: local3 restored the prior accepted boot tree hash.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged static inspection: git diff --cached --check passed before commit.

## Acceptance

Acceptance commit: recorded in durable supervisor state for
phase10-pi5-local-line-cancel-proof-20260531 after commit creation.
