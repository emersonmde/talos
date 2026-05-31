# Phase 10 Pi 5 Local Literal Echo Proof

Task: phase10-pi5-local-literal-echo-proof-20260531
Status: accepted

## Goal

Carry the accepted bounded literal echo tail feature to serialized Raspberry Pi
5 serial hardware evidence.

## Scope

This task added only the narrow Pi 5 proof harness needed to exercise the
accepted descriptor-backed literal echo behavior on physical serial: type
`echo local serial works`, press Enter, dispatch through fd0/runtime-console0
canonical-lite input, print visible `local serial works` through
descriptor-backed stdout, return to a ready prompt, emit the final
classification, and retain exact PASS evidence.

It does not accept broad shell parsing, quoting, escaping, globbing, argv/envp
process ABI, userspace shell execution, process spawning, filesystem-backed
command lookup, terminal/session behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Implementation

- Added the `rpi5_local_literal_echo` boot scenario as a proof-only extension
  of the existing descriptor-backed local serial command loop.
- Added Pi 5 literal echo image and boot-tree helper scripts.
- Added proof-local visible replay and a summary line for the literal input so
  the physical transcript retains the command text, final line, raw byte count,
  control count, response count, visible output, ready prompt, classification,
  and PASS.
- Kept the accepted literal echo command semantics unchanged from the
  QEMU/substitute core.

## Evidence

Accepted Pi 5 evidence:

- tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/serial-transcript-through-pass.txt
- tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/proof-result-local3.txt
- tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/proof-lines-local3.txt
- tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/tftp-kernel-fetch-local3.txt
- tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/archive-review.txt
- tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/restore-proof.txt
- tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/post-snapshot-restore-status.json

The accepted local3 proof records:

- archive sha256: 7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5
- kernel sha256: 63d16359baa5e3c8631f582014a3bcd14f78b64023717bb1546dcc9e9e4c3826
- kernel size: 100352 bytes
- restored boot tree hash: a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

The retained serial transcript shows the rpi5-local-literal-echo-proof scenario
start, descriptor-backed input/output markers, ready command=0, visible
`local serial works`, summary evidence for input `echo local serial works`,
final line `echo local serial works`, raw-bytes=24, controls=0,
responses=1, ready-for-next prompt=true, final
pi5-local-literal-echo-complete classification, and
rpi5-local-literal-echo-proof: PASS.

## Inconclusive-Run Triage

The initial local1 candidate reached the internal PASS path but did not retain
the visible input command text required by the hardware proof acceptance gate.
Before proof-code changes, the task recorded the standard triage sequence:

- candidate identity, fresh serial cursor, and TFTP evidence under
  local1-candidate.
- restored-tree known-good controls:
  - local1-known-good-control-rerun/control-result.txt
- unchanged candidate rerun:
  - local2-unchanged-candidate-rerun/

The final local3 candidate used only proof-local visibility changes and
supplied the accepted physical transcript.

## Hardware Lock

- owner task: phase10-pi5-local-literal-echo-proof-20260531
- hardware lock acquired: 2026-05-31T22:06:13Z
- hardware action: serialized Pi 5 boot archive publication, power cycle,
  serial observe/write, TFTP evidence collection, and boot-tree restore.
- restore status: local3 restored the prior accepted boot tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardwareTestLock release.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 348 tests.
- QEMU/substitute literal echo feature gate:
  `scripts/qemu-local-literal-echo-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regressions:
  `scripts/qemu-local-echo-command-smoke.sh --quiet`,
  `scripts/qemu-local-pwd-command-smoke.sh --quiet`,
  `scripts/qemu-local-line-editing-smoke.sh --quiet`,
  `scripts/qemu-local-line-cancel-smoke.sh --quiet`, and
  `scripts/qemu-local-line-kill-smoke.sh --quiet` passed.
- static archive/image review: `scripts/rpi5-archive-review.sh` passed for the
  local3 archive.
- serialized Pi 5 hardware proof: local3 retained visible literal echo output,
  input summary, classification, and PASS.
- restore proof: local3 restored the prior accepted boot tree hash.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- staged static inspection: `git diff --cached --check` passed before commit.

Acceptance commit: recorded in durable supervisor state after commit creation.
