# Phase 10 Pi 5 Local Line-Kill Proof

Task: phase10-pi5-local-line-kill-proof-20260531
Status: accepted

## Goal

Carry the accepted Ctrl-U prompt-local line-kill feature to serialized
Raspberry Pi 5 serial hardware evidence.

## Scope

This task added only the narrow Pi 5 proof harness needed to exercise the
accepted descriptor-backed line-kill behavior on physical serial: partial
input, Ctrl-U, whole-line discard, visible line-killed response, following pwd
dispatch, visible slash output, descriptor-backed input/output markers,
next-prompt readiness, final classification, and PASS.

It does not accept POSIX signal delivery, process interruption, userspace shell
execution, process spawning, job control, terminal sessions, termios,
filesystem-backed command lookup, broad escape-sequence parsing, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Implementation

- Added the rpi5_local_line_kill boot scenario.
- Added Pi 5 line-kill image and boot-tree helper scripts.
- Reused the accepted descriptor-backed command loop and canonical-lite TTY
  Ctrl-U handling.
- Added proof-local visible replay and summary output so the physical transcript
  retains the Ctrl-U, final-line, raw-byte, control-count, and response-count
  evidence after the visible line-killed and pwd output.

Implementation commits:

- e5f7a9f11510acbf19804220ea1c22e92efd2ed0
- 52476290c3927c9dde7838c807b8ff4c45a44819
- d3e4b59c39cf35eac204a2f36245a2d1d8d64ce3

## Evidence

Accepted Pi 5 evidence:

- tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/serial-transcript-through-pass.txt
- tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/proof-result-local6.txt
- tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/proof-lines-local6.txt
- tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/tftp-kernel-fetch-local6.txt
- tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/archive-review.txt
- tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/post-snapshot-restore-status.json

The accepted local6 proof records:

- source commit: d3e4b59c39cf35eac204a2f36245a2d1d8d64ce3
- archive sha256: 89049ac8adc6871ed728587e9445c91955f4d7a1d7f128abc4721724bde741a8
- kernel sha256: f4e5d43981e049b008b750c5dd5fc37b458628f5a0cf7d8f038ca91e1d964765
- kernel size: 100272 bytes
- restored boot tree hash: a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

The retained serial transcript shows the rpi5-local-line-kill-proof scenario
start, descriptor-backed input/output markers, ready command=0, visible
talos: line-killed, visible slash output, summary evidence for partial bogus,
Ctrl-U, final line pwd, raw-bytes=10, controls=1, responses=2,
ready-for-next prompt=true, final pi5-local-line-kill-complete classification,
and rpi5-local-line-kill-proof: PASS.

## Inconclusive-Run Triage

Earlier candidate attempts reached the internal PASS path but did not retain
enough serial metadata to satisfy the hardware proof acceptance gate. Before
proof-code changes, the task recorded the standard triage sequence:

- candidate identity, fresh serial cursor, and TFTP evidence for the
  inconclusive candidate runs under local1, local3, and local5.
- restored-tree known-good controls:
  - local1-known-good-control/control-result.txt
  - local3-known-good-control/control-result.txt
- unchanged candidate reruns:
  - local2-unchanged-candidate-rerun/
  - local5-unchanged-flushed-candidate-rerun/

The final local6 candidate used only proof-local visibility changes and
supplied the accepted physical transcript.

## Hardware Lock

- owner task: phase10-pi5-local-line-kill-proof-20260531
- hardware action: serialized Pi 5 boot archive publication, power cycle,
  serial observe/write, and restore.
- restore status: local6 restored the prior accepted boot tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardwareTestLock release.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute line-kill regression:
  scripts/qemu-local-line-kill-smoke.sh --quiet passed.
- QEMU/substitute Ctrl-C line-cancel regression:
  scripts/qemu-local-line-cancel-smoke.sh --quiet passed.
- QEMU/substitute Backspace/Delete regression:
  scripts/qemu-local-line-editing-smoke.sh --quiet passed.
- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  local6 archive.
- serialized Pi 5 hardware proof: local6 retained visible line-kill, following
  pwd, classification, and PASS.
- restore proof: local6 restored the prior accepted boot tree hash.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged static inspection: git diff --cached --check passed before commit.

## Acceptance

Acceptance commit: recorded in durable supervisor state for
phase10-pi5-local-line-kill-proof-20260531 after commit creation.
