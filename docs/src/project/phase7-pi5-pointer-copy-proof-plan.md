# Phase 7 Pi 5 Pointer-Copy Proof Plan

Status: accepted candidate plan for
phase7-pi5-pointer-copy-proof-plan-20260529.

## Scope

This plan defines the serialized Raspberry Pi 5 proof that may carry the
accepted QEMU/substitute pointer-copy syscall invariant to physical hardware.
It does not change Rust or assembly code, publish a boot archive, power-cycle
hardware, observe serial output, acquire hardwareTestLock, add descriptor I/O,
runtime console or TTY integration, process loading, VFS/filesystem behavior,
path copying, shell behavior, networking, SSH, RP1/PCIe policy, UART
interrupt ownership, or DMA/cache-driver policy.

The future hardware proof may touch only the source and lab surfaces needed to
stage and run the physical proof:

- build.rs for a focused rpi5_pointer_copy_proof boot-scenario cfg.
- src/boot/rpi5.rs and src/target/rpi5.rs for focused Pi 5 scenario dispatch,
  proof orchestration, physical UserData backing storage, and serial output.
- src/arch/aarch64/exceptions.rs only if the Pi 5 lower-AArch64 synchronous
  SVC path needs the same bounded production routing already accepted by QEMU.
- src/syscall.rs only for using the accepted proof-only talos_copy_probe route
  and preserving x8 = 0x7001 as -ENOSYS outside the proof scenario.
- src/posix.rs only if a narrow helper adjustment is required by the accepted
  copy-helper contract and remains covered by unit tests.
- scripts/rpi5-pointer-copy-proof-image.sh,
  scripts/rpi5-pointer-copy-proof-boot-tree.sh, and a focused static/archive
  review helper if the implementation needs one.
- tasks/2026-05-29-phase7-pi5-pointer-copy-proof.md and retained evidence
  under tasks/evidence/2026-05-29-pi5-pointer-copy-proof/.

Any broader source ownership requires supervisor planning before execution.

## Accepted Inputs

- Pointer-taking syscall contract:
  ddefb045443010a3de0dd89a046454df93f192c2.
- QEMU pointer-copy smoke plan:
  75414541efb936f467ce57e270b1701edcba9b3d.
- QEMU pointer-copy smoke core:
  10c23e00e04173fa9b8af987273b047d2dd4e2e3.
- Pointer-copy closeout checkpoint:
  a30883bc5b4458850fe369b4558c27dc97736258.
- Retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.

## Physical Invariant

The future proof must demonstrate one physical invariant on serialized
Raspberry Pi 5 hardware:

1. Talos builds a focused rpi5_pointer_copy_proof boot scenario from the
   accepted QEMU pointer-copy smoke boundary.
2. The kernel constructs fixed lower-EL UserText, UserData, UserGuard, and
   UserStack ranges. UserData must provide physical backing storage initialized
   to 16 bytes of 0x2a, plus a guard range that helper validation rejects.
3. Before ERET, the kernel validates user ELR, user SP, SPSR/PSTATE, UserText,
   UserData, UserGuard, and UserStack.
4. The lower-EL payload enters lower AArch64 and performs stable svc #0 with
   x8 = 0x7001, x0 = 0x0000_0000_0011_0000, x1 = 16, x2 = 0x2a,
   x3 = 0xa5, x4 = 0, and x5 = 0. Success returns x0 = 16 and the kernel
   observes the UserData backing storage changed to 16 bytes of 0xa5.
5. The payload performs stable svc #0 with x8 = 0x7001,
   x0 = 0x0000_0000_001e_0000, x1 = 16, x2 = 0x2a, x3 = 0xa5, x4 = 0,
   and x5 = 0. The guard address must return x0 = 0xffff_ffff_ffff_fff2,
   the two's-complement encoding of -EFAULT.
6. The payload performs stable svc #0 with x8 = 17 and observes
   x0 = 0xffff_ffff_ffff_ffda, the two's-complement encoding of -ENOSYS.
7. Only after those production svc #0 observations may the payload use the
   existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
   That marker must not dispatch as talos_copy_probe or any stable syscall.
8. The proof reports the planned physical completion classification and PASS
   line only after success, EFAULT, unknown-syscall, and diagnostic-quarantine
   observations have been recorded.

The expected ESR for all production syscalls is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional completion
marker is 0x0000000054007a10. FAR_ELx is expected to be zero for these SVC
paths unless the physical run reports a defined architectural value; any
nonzero FAR must be printed and justified in the task evidence.

## Required Output

The future hardware task must retain physical serial evidence with these exact
PASS/classification lines:

~~~text
rpi5-pointer-copy-proof: final participants=3 expected=3 errors=0 classification=pi5-pointer-copy-proof-complete
rpi5-pointer-copy-proof: PASS
~~~

The retained serial log must also include these exact field names and stable
values:

~~~text
rpi5-pointer-copy-proof: start
rpi5-pointer-copy-proof: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true
rpi5-pointer-copy-proof: syscall case=copy_probe_success vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x0000000000110000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000010
rpi5-pointer-copy-proof: user-observed case=copy_probe_success x0=0x0000000000000010 data=0xa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5 ok=true
rpi5-pointer-copy-proof: syscall case=copy_probe_efault vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x00000000001e0000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0xfffffffffffffff2 expected=-EFAULT
rpi5-pointer-copy-proof: user-observed case=copy_probe_efault x0=0xfffffffffffffff2 ok=true
rpi5-pointer-copy-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-pointer-copy-proof: user-observed case=unknown x0=0xffffffffffffffda ok=true
rpi5-pointer-copy-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
~~~

The accepted PASS requires the success, EFAULT, unknown-syscall, diagnostic
marker, final classification, and PASS lines. Firmware output, target init
output, old QEMU evidence, or marker-only printing is not enough.

## Diagnostic And Proof-Only Quarantine

talos_copy_probe remains proof-only. The future proof may route x8 = 0x7001
only in the focused rpi5_pointer_copy_proof scenario. Outside that scenario,
x8 = 0x7001 must continue to return -ENOSYS like any other unaccepted syscall
number.

SVC immediate 0x7a10 remains proof vocabulary. It is not a stable syscall
immediate, syscall number, ABI version, compatibility mode, production
dispatch selector, copy-probe operation, or production success path. The final
PASS must fail if the marker appears before the copy success, copy EFAULT, and
unknown-syscall observations, or if the marker is dispatched through the
syscall core.

## Serialized Run Requirements

The future proof task must acquire hardwareTestLock before archive publishing,
power-cycle, serial observe, candidate boot, TFTP inspection, or restoration.
The lock owner and taskId must be phase7-pi5-pointer-copy-proof-20260529, and
the lock must be released after completion, failure, or pause.

Before publishing the candidate, the future task must retain:

- candidate source commit and git status --short.
- focused image path, kernel SHA256, kernel size, archive path, archive SHA256,
  and scripts/rpi5-archive-review.sh output or an accepted focused
  replacement.
- pre-run boot-tree snapshot or restore handle.
- fresh serial cursor captured before the candidate run.
- TFTP cursor captured before the candidate run.

After publishing and power-cycling, the future task must retain:

- TFTP delta showing a fresh da591740/kernel_2712.img fetch for the candidate.
- serial evidence from the fresh cursor, using repeated observe windows if one
  window reaches only firmware or early bootloader output.
- final classification and PASS, or a classified failure/inconclusive result.
- restoration record and post-restore status for the prior accepted boot tree.

The accepted proof may not depend on old serial scrollback or a stale TFTP
event. Candidate identity must be tied to both archive/kernel digests and the
fresh TFTP fetch.

## Inconclusive-Run Triage

If any candidate hardware run is inconclusive, no code changes are allowed
until this triage sequence is recorded:

1. Candidate identity: commit, git status, archive SHA256, kernel SHA256,
   kernel size, and archive-review output.
2. Fresh serial cursor: prove the observe window starts after the candidate
   publish/power-cycle point.
3. TFTP delta: prove the Pi fetched the candidate kernel_2712.img after the
   candidate publish.
4. Known-good control: restore and run an accepted known-good Pi 5 proof or
   baseline to verify lab health, unless the failure is already a clear Talos
   proof failure with complete candidate fetch and serial evidence.
5. Candidate rerun: republish or rerun the same candidate with fresh serial
   and TFTP cursors before changing code.

Only after that sequence may the worker classify the result as a code issue
and change implementation. Failed hardware boots are evidence, not incidents.
The lab should be restored to the pre-run tree unless restoration itself is
the classified failure.

## Evidence Boundary

A passing Pi 5 run would accept only the physical pointer-copy proof invariant
listed above: stable lower-AArch64 svc #0 reaches the production syscall
dispatch core on Pi 5, proof-only talos_copy_probe returns x0 = 16 after
copying 0x2a bytes in and 0xa5 bytes out, a guard-range request returns
-EFAULT, unknown syscall number 17 returns -ENOSYS, and diagnostic marker
0x7a10 stays quarantined outside production dispatch.

It would not accept descriptor read/write/close/dup, stable public POSIX status
for talos_copy_probe, process-owned address spaces, partial copies, restart
semantics, signals, resumable user faults, per-thread errno storage, process
loading, ELF parsing, argv/envp setup, PID allocation, exit/wait,
credentials, sessions, controlling TTY, VFS, filesystem behavior, local shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

QEMU/substitute evidence remains useful for regression comparison, but only
serial hardware boot/output with candidate identity, fresh TFTP evidence,
hardwareTestLock ownership, and restoration proof can establish the Pi 5
claim.

## Validation

- static inspection: git status --short before edits must be clean or record
  unrelated changes.
- static inspection: git diff --check must pass.
- documentation: mdbook build must pass for this plan.
- hardware: no Pi 5 hardware run, archive publication, power-cycle, serial
  observe, hardware-lock acquisition, or physical pointer-copy claim is made
  by this plan.

## Next Boundary

The next mechanically derivable task is phase7-pi5-pointer-copy-proof-20260529,
provided this plan is accepted and hardwareTestLock is unlocked. It must
acquire the lock before any hardware action and must run exactly the serialized
proof described here.
