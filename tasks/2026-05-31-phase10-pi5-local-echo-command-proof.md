# Phase 10 Pi 5 Local Echo Command Proof Task

Task: phase10-pi5-local-echo-command-proof-20260531

Status: accepted

## Goal

Prove the accepted kernel-backed `echo hello` local command feature on
physical Raspberry Pi 5 serial hardware.

## Scope

Carried the accepted descriptor-backed local command loop to serialized Pi 5
hardware evidence. The proof publishes only a Pi 5 boot archive for the
kernel-backed local `echo` command path, types `echo hello` through the lab
controller serial API, prints a visible `hello` line, records descriptor-backed
input/output markers, and restores the prior accepted boot tree.

Changed files:

- build.rs
- src/target/rpi5.rs
- scripts/rpi5-local-echo-command-image.sh
- scripts/rpi5-local-echo-command-boot-tree.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase10-pi5-local-echo-command-proof.md
- tasks/evidence/2026-05-31-pi5-local-echo-command-proof/

## Accepted Frontier

Accepted: the physical Pi 5 command loop can receive a serial `echo hello`
command, dispatch the kernel-backed `echo` built-in through the
descriptor-backed local command-loop path, print the visible `hello` response,
record descriptor-backed-input=true and descriptor-backed-output=true, and
return to a ready prompt.

The Pi 5 proof harness now supports a narrow `rpi5_local_echo_command` boot
scenario. It reuses the accepted local command-loop feature and adds only proof
visibility for the physical transcript; it does not implement userspace shell
execution or filesystem command lookup.

## Deferred Surfaces

Deferred: userspace shell execution, process spawning, exec/wait/exit,
filesystem-backed command lookup, pipes, redirection, globbing,
quoting/escaping semantics, argv/envp process ABI, termios/job control,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and broad POSIX read/stdio readiness outside the accepted descriptor-backed
local command-loop path.

## Evidence

- Accepted Pi 5 serial transcript:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/serial-transcript.txt.
- Accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/proof-result-local2.txt.
- Static archive/image review:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/archive-review.txt.
- Fresh TFTP fetch evidence:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/tftp-kernel-fetch-local2.txt.
- Restore proof:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/post-snapshot-restore-status.json.

The accepted candidate identity is:

- archive sha256: 1ec5389c84e3a779ef1d98c5b664b3771947c8415fd02ee731f2cbfbafa646d4
- kernel sha256: cc80d0bb12d2f98a889ad5ec8de21119d2ba16031b4015c3b81bfcef958d5d4e
- kernel size: 98664 bytes

The retained proof summary records `typed_command=echo hello`, serial write
success for 11 bytes, two fresh 98664-byte `da591740/kernel_2712.img` TFTP
fetches, `hello` visible output, `ready-for-next prompt=true`, final
classification `pi5-local-echo-command-complete`, and
`rpi5-local-echo-command-proof: PASS`.

The first local1 run reached the PASS path but did not retain enough visible
serial bytes around the command response, so it is retained as inconclusive
capture evidence only. The accepted local2 run uses the tightened proof
visibility and a fresh candidate boot/archive identity.

## Hardware Lock

- acquired: 2026-05-31T14:34:30Z
- released: 2026-05-31T14:43:30Z
- owner task: phase10-pi5-local-echo-command-proof-20260531
- restore hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

## Validation

- git status --short before proof publication: recorded in local2 evidence.
- static archive/image inspection: `scripts/rpi5-archive-review.sh` passed for
  the local2 archive.
- serialized Pi 5 hardware boot/output: local2 retained typed command evidence,
  visible `hello`, descriptor-backed markers, ready prompt, classification,
  and PASS.
- lab-controller API/TFTP: local2 retained two fresh 98664-byte
  `da591740/kernel_2712.img` fetches.
- restore proof: local2 restored the prior accepted boot tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- staged static inspection: `git diff --cached --check` passed before commit.

## Commit

Implementation, documentation, task record, and evidence commit:
83588278519d155bc5e714ff7df086ebc5cb04af.
