# Phase 10 Pi 5 Local Line-Editing Proof Task

Task: phase10-pi5-local-line-editing-proof-20260531

Status: accepted

## Goal

Carry the accepted Backspace/Delete local line-editing feature to serialized
Raspberry Pi 5 serial hardware evidence.

## Scope

The task added a narrow Pi 5 proof harness scenario for the already accepted
local command-loop line-editing behavior. The scenario changes proof labeling
and telemetry only: it expects a corrected `pwd` line, prints Backspace/Delete
counts, and reports `pi5-local-line-editing-complete` after the command loop
returns to the next prompt.

No new command semantics, userspace shell execution, process spawning,
filesystem lookup, termios/history/cursor behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy is accepted here.

Changed files committed before hardware attempt:

- build.rs
- src/boot/rpi5.rs
- src/main.rs
- src/pl011.rs
- src/runtime_console.rs
- src/target/rpi5.rs
- scripts/rpi5-local-line-editing-image.sh
- scripts/rpi5-local-line-editing-boot-tree.sh

Implementation commit:
d7b788caa1f93cf10575fcb9c650e923e660f307.

## Validation Before Hardware

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 343 tests.
- QEMU/substitute retained current evidence was rerun:
  `scripts/qemu-local-line-editing-smoke.sh --quiet` passed and retained
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- static image/archive inspection passed:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local1-clean-candidate/archive-review.txt.

## Inconclusive Hardware Attempt

Candidate local1 was published under hardwareTestLock after recording fresh
serial and TFTP cursors.

Candidate identity:

- source commit: d7b788caa1f93cf10575fcb9c650e923e660f307
- archive sha256:
  1bde053105052508d32ebd1c9fa8faa959a2b9959ff1b40721dc3859e668adda
- kernel sha256:
  f2b38552e791d7fb23aa8553694d30e914abfa80e8e3bf2b4c03f0547cb97f7e
- kernel size: 98944 bytes

Retained local1 evidence:

- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local1-clean-candidate/
- The published boot tree status recorded the candidate 98944-byte kernel and
  tree hash b908f00cb97ce378b322cbbf7691af28ba2836cedc9a1cac661ae9ce4b8bd3a1.
- TFTP evidence shows repeated fresh 98944-byte
  `da591740/kernel_2712.img` fetches.
- Serial evidence from the fresh cursor did not produce the expected
  `rpi5-local-line-editing-proof` prompt or PASS path, so the run is
  inconclusive and not accepted.

Per the hardware policy, no code changes were made after the inconclusive
candidate run. Triage recorded candidate identity, fresh serial cursor, TFTP
delta, and a restored-tree known-good control attempt before any further
implementation work.

## Control, Rerun, And Restore

The pre-candidate boot tree was restored from snapshot
`pre-pi5-local-line-editing-local1-20260531T173845Z`.

Restored tree evidence:

- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local1-known-good-control/restore-pre-snapshot.json
- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local1-known-good-control/post-control-status.json
- restored tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
- restored kernel size: 104136 bytes

The first restored-tree control fetched the restored 104136-byte kernel, but
retained serial observation did not reach a Talos PASS path. No code changed
after that inconclusive result.

The resumed hardware triage then reran the restored accepted boot tree and
reached the accepted production timer preemption PASS path:

- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-known-good-control/control-result.txt
- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-known-good-control/serial-transcript.txt

After that conclusive known-good control, the worker reran the unchanged
candidate archive from implementation commit
d7b788caa1f93cf10575fcb9c650e923e660f307. The rerun typed
`pwx` + Backspace + `d` + Enter through physical serial input. The accepted
PASS window records the line-editing proof prompt, one Backspace, visible `/`
output, descriptor-backed input/output markers, next prompt readiness, final
`pi5-local-line-editing-complete` classification, and
`rpi5-local-line-editing-proof: PASS`.

Accepted rerun evidence:

- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/serial-transcript-through-pass.txt
- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/proof-result-local2.txt
- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/tftp-kernel-fetch-local2.txt
- tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/post-restore-status.json

The unchanged rerun retained two fresh 98944-byte
`da591740/kernel_2712.img` TFTP fetches and restored the prior accepted boot
tree hash:
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
The full raw serial transcript also contains later reset-loop output after the
accepted PASS; `serial-transcript-through-pass.txt` is the bounded acceptance
window.

## Accepted Frontier

Talos now has serialized Pi 5 hardware evidence that the descriptor-backed
serial command loop accepts Backspace before Enter dispatch, removes the
previous editable byte, dispatches the corrected kernel-backed `pwd` command,
prints visible `/` through descriptor-backed stdout, and returns to a ready
`talos>` prompt.

This does not accept userspace shell execution, process spawning, filesystem
lookup, `cd`, path traversal, termios, history, cursor addressing, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Final Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 343 tests.
- QEMU/substitute regression:
  `scripts/qemu-local-line-editing-smoke.sh --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed with the existing large search-index
  warning.
- validation summary:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/validation/validation-summary.txt.
