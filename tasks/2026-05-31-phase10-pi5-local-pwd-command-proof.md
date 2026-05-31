# Phase 10 Pi 5 Local Pwd Command Proof Task

Task: phase10-pi5-local-pwd-command-proof-20260531

Status: accepted

## Goal

Prove the accepted kernel-backed `pwd` local command feature on physical
Raspberry Pi 5 serial hardware.

## Scope

Carried the accepted descriptor-backed local `pwd` command to serialized Pi 5
hardware evidence. The proof publishes only a Pi 5 boot archive for the
kernel-backed local `pwd` command path, types `pwd` through the lab
controller serial API, prints visible `/`, records descriptor-backed
input/output markers, and restores the prior accepted boot tree.

Changed files:

- build.rs
- src/target/rpi5.rs
- scripts/rpi5-local-pwd-command-image.sh
- scripts/rpi5-local-pwd-command-boot-tree.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase10-pi5-local-pwd-command-proof.md
- tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/

## Accepted Frontier

Accepted: the physical Pi 5 command loop can receive a serial `pwd` command,
dispatch the kernel-backed `pwd` built-in through the descriptor-backed local
command-loop path, print visible `/` from the root-only current-directory
placeholder, record descriptor-backed-input=true and
descriptor-backed-output=true, and return to a ready prompt.

The Pi 5 proof harness now supports a narrow `rpi5_local_pwd_command` boot
scenario. It reuses the accepted local command-loop feature and adds only proof
visibility for the physical transcript; it does not implement `cd`, VFS path
lookup, userspace shell execution, or filesystem command lookup.

## Deferred Surfaces

Deferred: `cd`, path traversal and normalization, VFS lookup, directory
listing, userspace shell execution, process spawning, exec/wait/exit,
filesystem-backed command lookup, pipes, redirection, globbing,
quoting/escaping semantics, argv/envp process ABI, termios/job control,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and broad POSIX read/stdio readiness outside the accepted descriptor-backed
local command-loop path.

## Evidence

- Accepted Pi 5 serial transcript:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/serial-transcript.txt.
- Accepted normalized serial transcript:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/serial-transcript-normalized.txt.
- Accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/proof-result-local2.txt.
- Static archive/image review:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/archive-review.txt.
- Fresh TFTP fetch evidence:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/tftp-kernel-fetch-local2.txt.
- Restore proof:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/post-snapshot-restore-status.json.
- Inconclusive first candidate and known-good control:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local1-clean-candidate/
  and
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local1-known-good-control/.

The accepted candidate identity is:

- source commit: 215bf0bca780a8c50f01977e778cdadd34d20238
- archive sha256: 6754773f7511b5c06b2fab5d1bb954212921ced5df1876f9c9c9d257dd2db5ae
- kernel sha256: 1a31c94a569aa52ceb339b035ab35478f37adb5ef9ec2057b2cff8ab03327c4d
- kernel size: 98816 bytes

The retained proof summary records `typed_command=pwd`, serial write success,
two fresh 98816-byte `da591740/kernel_2712.img` TFTP fetches, visible `/`
output, descriptor-backed input/output markers, `ready-for-next prompt=true`,
final classification `pi5-local-pwd-command-complete`, and
`rpi5-local-pwd-command-proof: PASS`.

The first local1 run fetched the candidate kernel but did not retain the prompt
before the script restored the prior boot tree. Before any code changes, triage
recorded candidate identity, fresh serial cursor, TFTP delta, a known-good
control boot of the restored prior accepted tree, and then reran the unchanged
candidate as local2. The known-good control reached its accepted production
timer preemption PASS path, proving the lab serial/TFTP path before the
unchanged candidate rerun.

## Hardware Lock

- acquired: 2026-05-31T16:04:55Z
- released: recorded in talos-supervisor-state.json when this task was accepted
- owner task: phase10-pi5-local-pwd-command-proof-20260531
- restore hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

## Validation

- pre-run git status and candidate identity: recorded in local1/local2 evidence.
- static archive/image inspection: `scripts/rpi5-archive-review.sh` passed for
  the local2 archive.
- serialized Pi 5 hardware boot/output: local2 retained typed command evidence,
  visible `/`, descriptor-backed markers, ready prompt, classification, and
  PASS.
- lab-controller API/TFTP: local2 retained two fresh 98816-byte
  `da591740/kernel_2712.img` fetches.
- restore proof: local2 restored the prior accepted boot tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute regression: `scripts/qemu-local-pwd-command-smoke.sh --quiet`
  passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- staged static inspection: `git diff --cached --check` passed before commit.

## Commit

Proof harness implementation commit:
215bf0bca780a8c50f01977e778cdadd34d20238.

Implementation, documentation, task record, and evidence commit:
recorded in talos-supervisor-state.json after commit creation.
