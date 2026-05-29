# Phase 7 Pi 5 Pointer-Copy Proof

Task: phase7-pi5-pointer-copy-proof-20260529
Status: accepted

## Scope

This task carried the accepted QEMU/substitute pointer-copy syscall boundary to
serialized Raspberry Pi 5 hardware. It added only the focused
rpi5_pointer_copy_proof boot scenario, Pi 5 lower-AArch64 svc #0 proof-only
talos_copy_probe routing, image and boot-tree helpers, retained local evidence,
and retained lab evidence needed for the success-copy, guard-range -EFAULT, and
unknown-syscall -ENOSYS observations.

It did not add descriptor-backed read/write, close, dup, pipe, stdio, runtime
console or TTY integration, process loading, VFS/filesystem behavior, path
copying, shell behavior, networking, SSH, RP1/PCIe work, UART interrupt
ownership, DMA/cache-driver policy, demand paging, copy-on-write,
signal/restart semantics, or a stable public talos_copy_probe claim.

## Candidate Identity

- Implementation commit: f67595b892125a8d03f5190103b6af886d3c1ffd.
- Candidate image:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-pointer-copy-proof.img.
- Kernel SHA256:
  99890a3520fc1351c00250551409974cba82b802a47b40b18ac683234c1fa23b.
- Kernel size: 106408 bytes.
- Candidate archive: target/talos-rpi5-pointer-copy-proof-boot.tar.gz.
- Archive SHA256:
  195e196bb785292847da7e98f32ef4e15b08caa7d2bdd850a1240682a1c68dd9.
- Static archive review:
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local1-candidate/archive-review.txt.

## Hardware Lock

- Owner task id: phase7-pi5-pointer-copy-proof-20260529.
- Acquired before local1 archive publication and power-cycle.
- Released after local3 unchanged candidate rerun evidence and restore.
- Restore snapshot: pre-pi5-pointer-copy-proof-local1-20260529.
- Pre-run tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- Post-restore tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Hardware Evidence

Evidence directory:
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/.

- local1-candidate: fresh TFTP evidence showed
  da591740/kernel_2712.img served at 106408 bytes, but retained serial
  observe windows contained only firmware/network-boot output. Result:
  inconclusive; no code changes followed.
- local2b-known-good-control-rerun: restored accepted control tree reached
  TALOS: asm_start, talos: boot start,
  classification=pi5-production-timer-preemption-complete, and
  rpi5-production-timer-preemption: PASS from a fresh serial cursor.
- local3-candidate-rerun: unchanged candidate archive rerun from fresh serial
  and TFTP cursors reached the required physical pointer-copy proof lines:

~~~text
rpi5-pointer-copy-proof: syscall case=copy_probe_success vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x0000000000110000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000010
rpi5-pointer-copy-proof: user-observed case=copy_probe_success x0=0x0000000000000010 data=0xa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5 ok=true
rpi5-pointer-copy-proof: syscall case=copy_probe_efault vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x00000000001e0000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0xfffffffffffffff2 expected=-EFAULT
rpi5-pointer-copy-proof: user-observed case=copy_probe_efault x0=0xfffffffffffffff2 ok=true
rpi5-pointer-copy-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-pointer-copy-proof: user-observed case=unknown x0=0xffffffffffffffda ok=true
rpi5-pointer-copy-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
rpi5-pointer-copy-proof: final participants=3 expected=3 errors=0 classification=pi5-pointer-copy-proof-complete
rpi5-pointer-copy-proof: PASS
~~~

Retained proof lines:
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt.

TFTP proof:
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.

Restore proof:
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-post-restore-status.json.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 208 tests.
- QEMU/substitute: scripts/qemu-pointer-copy-smoke.sh passed.
- QEMU/substitute regression: scripts/qemu-syscall-smoke.sh passed.
- image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-pointer-copy-proof-boot.tar.gz passed.
- serialized hardware boot/output: local3 candidate rerun accepted with
  classification=pi5-pointer-copy-proof-complete and
  rpi5-pointer-copy-proof: PASS.
- restore proof: post-restore tree hash matched pre-run tree hash.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Boundary

Accepted: lower-AArch64 stable svc #0 reaches the proof-only talos_copy_probe
path on Pi 5; x8 = 0x7001 is routed only in the focused proof scenario; a
16-byte success case returns x0 = 16 and rewrites UserData from 0x2a to 0xa5;
the guard-range request returns -EFAULT; unknown syscall number 17 returns
-ENOSYS; diagnostic marker 0x7a10 remains outside the production syscall path.

Still blocked: descriptor read/write/close/dup, pipe, stdio, runtime
console/TTY integration, process-owned address spaces, partial copies, restart
semantics, signals, resumable user faults, per-thread errno storage, process
loading, ELF parsing, argv/envp setup, PID allocation, exit/wait, credentials,
sessions, controlling TTY, VFS, filesystem behavior, local shell, networking,
SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and stable
POSIX descriptor claims.
