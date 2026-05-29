# Phase 7 Pi 5 Descriptor-Write Proof

Task: phase7-pi5-descriptor-write-proof-20260529
Status: accepted

## Scope

This task carried the accepted QEMU/substitute descriptor-write syscall
boundary to serialized Raspberry Pi 5 hardware. It added only the focused
rpi5_descriptor_write_proof boot scenario, Pi 5 lower-AArch64 svc #0
descriptor-write proof routing, image and boot-tree helpers, retained local
evidence, and retained lab evidence needed for stdout/stderr writes, fd/error
cases, scalar regressions, and diagnostic-surface quarantine.

It did not add stdin/read, close, dup, pipe, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, live process-owned address spaces,
blocking/readiness, signals, restart semantics, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, or a full POSIX descriptor claim.

## Candidate Identity

- Implementation commit: 83b17d5695c3bd69ae39cd3cc1e74bf7d5fcd168.
- Candidate image:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-descriptor-write-proof.img.
- Kernel SHA256:
  f791c78e85dc72fe2c3db01b84f823e97bea1275faac0669ce0647bc402365f7.
- Kernel size: 108136 bytes.
- Candidate archive: target/talos-rpi5-descriptor-write-proof-boot.tar.gz.
- Archive SHA256:
  1152cf838d6e4c7d36f1276b627c37275964600ac9e6d5090641408e4806ea6f.
- Static archive review:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local1-candidate/archive-review.txt.
- git status before candidate build recorded pre-existing docs/src/roadmap.md
  edits and the newly created evidence directory; no uncommitted Rust, assembly,
  or script source was part of the candidate.

## Hardware Lock

- Owner task id: phase7-pi5-descriptor-write-proof-20260529.
- Acquired before local1 archive publication and power-cycle.
- Released after local3 unchanged candidate rerun evidence and restore.
- Restore snapshot: pre-pi5-descriptor-write-proof-local1-20260529.
- Pre-run tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- Post-restore tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Hardware Evidence

Evidence directory:
tasks/evidence/2026-05-29-pi5-descriptor-write-proof/.

- local1-candidate: fresh TFTP evidence showed
  da591740/kernel_2712.img served at 108136 bytes, but retained serial observe
  windows contained only null/blank bytes. Result: inconclusive; no code
  changes followed.
- local2-known-good-control: restored accepted control tree reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS from a fresh serial cursor.
- local3-candidate-rerun: unchanged candidate archive rerun from fresh serial
  and TFTP cursors reached the required physical descriptor-write proof lines:

~~~text
rpi5-descriptor-write-proof: syscall case=write_stdout vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000001 x1=0x0000000000110000 x2=0x0000000000000012 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000012
rpi5-descriptor-write-proof: runtime-console case=write_stdout device=runtime-console0 bytes=18 hex=74616c6f732d7374646f75742d727069350a ok=true
rpi5-descriptor-write-proof: user-observed case=write_stdout x0=0x0000000000000012 ok=true
rpi5-descriptor-write-proof: syscall case=write_stderr vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000002 x1=0x0000000000110040 x2=0x0000000000000012 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000012
rpi5-descriptor-write-proof: runtime-console case=write_stderr device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d727069350a ok=true
rpi5-descriptor-write-proof: user-observed case=write_stderr x0=0x0000000000000012 ok=true
rpi5-descriptor-write-proof: syscall case=write_fd0 vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
rpi5-descriptor-write-proof: syscall case=write_badfd vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
rpi5-descriptor-write-proof: syscall case=write_efault vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff2 expected=-EFAULT console-unchanged=true
rpi5-descriptor-write-proof: syscall case=write_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xffffffffffffffea expected=-EINVAL console-unchanged=true
rpi5-descriptor-write-proof: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000
rpi5-descriptor-write-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-descriptor-write-proof: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false
rpi5-descriptor-write-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
rpi5-descriptor-write-proof: final participants=8 expected=8 errors=0 classification=pi5-descriptor-write-proof-complete
rpi5-descriptor-write-proof: PASS
~~~

Retained proof lines:
tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt.

TFTP proof:
tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.

Restore proof:
tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-post-restore-status.json.

## Validation

- static inspection: git status --short before edits showed only a
  pre-existing docs/src/roadmap.md edit.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 218 tests.
- QEMU/substitute regression: scripts/qemu-syscall-smoke.sh passed.
- QEMU/substitute regression: scripts/qemu-pointer-copy-smoke.sh passed.
- QEMU/substitute: scripts/qemu-descriptor-write-smoke.sh passed.
- image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-descriptor-write-proof-boot.tar.gz passed.
- serialized hardware boot/output: local3 candidate rerun accepted with
  classification=pi5-descriptor-write-proof-complete and
  rpi5-descriptor-write-proof: PASS.
- restore proof: post-restore tree hash matched pre-run tree hash.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Boundary

Accepted: lower-AArch64 stable svc #0 reaches the descriptor-write path on Pi
5; talos_write x8 = 1 writes fd 1 and fd 2 through copy_from_user(), inherited
stdio descriptors, and runtime-console0; fd 0/fd 99, guard, and reserved
argument failures return the accepted errno values without extra console bytes;
talos_nop and unknown-syscall behavior remains intact; x8 = 0x7001 and
diagnostic marker 0x7a10 remain quarantined.

Still blocked: stdin/read, close, dup, pipe, full POSIX descriptor semantics,
process-owned descriptor tables, process-owned address spaces,
blocking/readiness, partial writes, per-thread errno storage, restart
semantics, signals, resumable user faults, process loading, ELF parsing,
argv/envp setup, PID allocation, exit/wait, credentials, sessions, controlling
TTY, VFS, filesystem behavior, local shell, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache-driver policy.
