# Phase 10 Pi 5 Local Help Command Proof Task

Task: phase10-pi5-local-help-command-proof-20260531

Status: accepted

## Goal

Carry the accepted serial `help` command feature to serialized Raspberry Pi 5
serial hardware evidence.

## Scope

This task published only the accepted help-command proof archive, typed
`help` over the serial console, retained the visible help response, and
restored the pre-run accepted boot tree. The help implementation remains
kernel-backed and prompt-local.

The accepted hardware proof is local6:

- tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/serial-transcript-through-pass.txt
- tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/proof-result-local6.txt
- tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/tftp-kernel-fetch-local6.txt

Earlier local3/local4 attempts are retained as inconclusive evidence. They
served the help kernel but missed proof bytes because the serial observe/write
ordering did not keep observation active before writing `help`. local6 fixed
only the proof procedure by starting serial observe before the write; it did not
change runtime help semantics.

Changed files:

- build.rs
- src/target/rpi5.rs
- scripts/rpi5-local-help-command-image.sh
- scripts/rpi5-local-help-command-boot-tree.sh
- docs/src/roadmap.md
- docs/src/project/lab-controller.md
- tasks/2026-05-31-phase10-pi5-local-help-command-proof.md
- tasks/evidence/2026-05-31-pi5-local-help-command-proof/

## Accepted Frontier

Accepted: on Raspberry Pi 5 hardware, the descriptor-backed serial command loop
can read `help` from fd0/runtime-console0, dispatch the kernel-backed help
built-in, print accurate help text through descriptor-backed stdout, and return
to a ready `talos>` prompt.

The retained local6 transcript shows:

- `talos: ok help`
- `talos: commands help status stdio pwd echo`
- `talos: echo forms echo hello; echo local serial works`
- `talos: editing backspace delete ctrl-c ctrl-u`
- descriptor-backed proof markers for dispatch, response count, raw bytes, and
  editing counters
- `pi5-local-help-command-complete`
- `rpi5-local-help-command-proof: PASS`

## Deferred Surfaces

Deferred: broad shell tokenization, quoting, escaping, globbing, environment
expansion, variables, command substitution, multiline input, pipes,
redirection, userspace shell execution, process spawning, external command
lookup, filesystem-backed commands, cd/path traversal, directory listing,
writable filesystem state, broad POSIX read and stdio readiness, terminal
sessions, termios, job control, cursor addressing, screen repainting, arrow
keys, networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
policy, and paused Phase 8 proof-only work.

## Evidence

- Static archive/image review:
  tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/archive-review.txt.
- Archive/kernel identity:
  tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/archive-and-kernel-sha256.txt.
- Fresh TFTP candidate fetch evidence:
  tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/tftp-kernel-fetch-local6.txt.
- Retained Pi 5 serial transcript:
  tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/serial-transcript-through-pass.txt.
- Key proof lines:
  tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/serial-key-lines.txt.
- Restore proof:
  tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/post-restore-status.json.

local6 summary:

```text
classification=accepted-pi5-help-responsive
archive_sha256=dc9d53623c55e19b3781ee504c5f04bf37a3367b19cf0f6305a1d6b366c0467f
kernel_sha256=ae855e8de0d0a93befee417df55b6137e8b00074cccade9ee6a4cc9da74d41f8
pre_tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
post_tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
result=PASS
```

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute feature gate:
  `scripts/qemu-local-help-command-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regressions:
  `scripts/qemu-local-literal-echo-smoke.sh --quiet`,
  `scripts/qemu-local-pwd-command-smoke.sh --quiet`,
  `scripts/qemu-local-line-editing-smoke.sh --quiet`,
  `scripts/qemu-local-line-cancel-smoke.sh --quiet`, and
  `scripts/qemu-local-line-kill-smoke.sh --quiet` passed.
- static archive/image review passed for local6.
- serialized Pi 5 hardware proof passed in local6.
- post-proof restore proof shows the pre-run boot tree hash restored.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- pre-commit static inspection: `git diff --cached --check` passed.

Acceptance commit: recorded in durable supervisor state after commit creation.
