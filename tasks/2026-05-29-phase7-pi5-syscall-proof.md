# Phase 7 Pi 5 Syscall Proof

Task: phase7-pi5-syscall-proof-20260529
Status: accepted

## Scope

This task carried the accepted QEMU production syscall routing invariant to
serialized Raspberry Pi 5 hardware. It added only the focused
rpi5_syscall_proof boot scenario, Pi 5 lower-AArch64 svc #0 routing, image and
boot-tree helpers, retained local evidence, and retained lab evidence needed
for stable talos_nop and unknown-syscall return observations.

It did not add descriptor I/O, copy-in/copy-out byte helpers, pointer-taking
syscalls, process loading, VFS, filesystem behavior, shell behavior,
networking, SSH, RP1/PCIe work, UART interrupt ownership, or DMA/cache-driver
policy.

## Candidate Identity

- Candidate source commit: 9d702d7e1a9ca8f3e1ab71da5f25297a8f34410c.
- Candidate image: target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-syscall-proof.img.
- Kernel SHA256: 6c0d4c040e0d10e4bc8ec9400e4596278daed2615b55ebd2665c10fc77ac8a6f.
- Kernel size: 101408 bytes.
- Candidate archive: target/talos-rpi5-syscall-proof-boot.tar.gz.
- Archive SHA256: d864125d2c99a16ce28641a3fccb0aacabdcc305030f1462ec09dcec5d61e073.
- Static archive review: tasks/evidence/2026-05-29-pi5-syscall-proof/local1-candidate/archive-review.txt.

## Hardware Lock

- Owner task id: phase7-pi5-syscall-proof-20260529.
- Acquired: 2026-05-29T05:16:00Z.
- Released: after local3 candidate rerun evidence and restore.
- Restore snapshot: pre-pi5-syscall-proof-local1-20260529.
- Pre-run tree hash: a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- Post-restore tree hash: a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Hardware Evidence

Evidence directory: tasks/evidence/2026-05-29-pi5-syscall-proof/.

- local1-candidate: fresh TFTP evidence showed repeated
  da591740/kernel_2712.img serves at 101408 bytes, but the retained serial
  observe windows contained only firmware/network-boot output. Result:
  inconclusive; no code changes followed.
- local2b-known-good-control-rerun: restored accepted control tree reached
  TALOS: asm_start, talos: boot start,
  classification=pi5-production-timer-preemption-complete, and
  rpi5-production-timer-preemption: PASS from a fresh serial cursor. TFTP
  served da591740/kernel_2712.img at 104136 bytes.
- local3-candidate-rerun: unchanged candidate archive rerun from fresh serial
  and TFTP cursors reached the required physical syscall proof lines:

~~~text
rpi5-syscall-proof: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000000
rpi5-syscall-proof: user-observed case=talos_nop x0=0x0000000000000000 ok=true
rpi5-syscall-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-syscall-proof: user-observed case=unknown x0=0xffffffffffffffda ok=true
rpi5-syscall-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
rpi5-syscall-proof: final participants=2 expected=2 errors=0 classification=pi5-syscall-proof-complete
rpi5-syscall-proof: PASS
~~~

Retained proof lines:
tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt.

TFTP proof:
tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.

Restore proof:
tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-post-restore-status.json.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 198 tests.
- QEMU/substitute: scripts/qemu-syscall-smoke.sh passed; retained log:
  tasks/evidence/2026-05-29-pi5-syscall-proof/qemu-syscall-smoke.log.
- image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-syscall-proof-boot.tar.gz passed.
- serialized hardware boot/output: local3 candidate rerun accepted with
  classification=pi5-syscall-proof-complete and rpi5-syscall-proof: PASS.
- restore proof: post-restore tree hash matched pre-run tree hash.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Boundary

Accepted: lower-AArch64 stable svc #0 reaches the production syscall dispatch
core on Pi 5; talos_nop returns x0 = 0 to lower EL; unknown syscall number 17
returns x0 = -ENOSYS to lower EL; diagnostic marker 0x7a10 remains outside the
production syscall path.

Still blocked: descriptor read/write/close/dup, copy-in/copy-out byte helpers,
pointer-taking syscalls, partial copies, restart semantics, signals, resumable
user faults, per-thread errno storage, process loading, ELF parsing, argv/envp
setup, PID allocation, exit/wait, credentials, sessions, controlling TTY, VFS,
filesystem behavior, local shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy.
