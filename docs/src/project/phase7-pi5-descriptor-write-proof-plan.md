# Phase 7 Pi 5 Descriptor-Write Proof Plan

Status: accepted candidate plan for
phase7-pi5-descriptor-write-proof-plan-20260529.

## Scope

This plan defines the serialized Raspberry Pi 5 proof that may carry the
accepted QEMU/substitute descriptor-write invariant to physical hardware. It
does not change Rust or assembly code, publish a boot archive, power-cycle
hardware, observe serial output, acquire hardwareTestLock, add stdin/read,
close, dup, process loading, VFS/filesystem behavior, path copying, shell
behavior, networking, SSH, RP1/PCIe policy, UART interrupt ownership, or
DMA/cache-driver policy.

The future hardware proof may touch only the source and lab surfaces needed to
stage and run the physical proof:

- build.rs for a focused rpi5_descriptor_write_proof boot-scenario cfg.
- src/boot/rpi5.rs and src/target/rpi5.rs for focused Pi 5 scenario dispatch,
  proof orchestration, physical UserData backing storage, runtime-console0
  observation, and serial output.
- src/arch/aarch64/exceptions.rs only if the Pi 5 lower-AArch64 synchronous
  SVC path needs the same bounded production routing already accepted by QEMU.
- src/syscall.rs only for using the accepted talos_write route and preserving
  talos_copy_probe quarantine outside proof scenarios.
- src/posix.rs only if a narrow descriptor or copy-helper adjustment is
  required by the accepted descriptor-write contract and remains covered by
  unit tests.
- src/runtime_console.rs only for a focused observation hook if the accepted
  runtime-console0 abstraction cannot already retain the required output.
- scripts/rpi5-descriptor-write-proof-image.sh,
  scripts/rpi5-descriptor-write-proof-boot-tree.sh, and a focused
  static/archive review helper if the implementation needs one.
- tasks/2026-05-29-phase7-pi5-descriptor-write-proof.md and retained evidence
  under tasks/evidence/2026-05-29-pi5-descriptor-write-proof/.

Any broader source ownership requires supervisor planning before execution.

## Accepted Inputs

- Descriptor syscall source inventory:
  96dda33fbca64ed71c6d8ea76d21e4fd030463c4.
- Descriptor syscall contract:
  23429329540dfa87ebc13a5086829173400791ea.
- QEMU descriptor-write smoke plan:
  dd338a284f8c9ba47c36b0735ade498664ff439f.
- Descriptor-write core:
  e462f45ff98fe5196900c2c5ce8783a997349568.
- QEMU descriptor-write smoke core:
  26c36ffaada05e4ba598144c44f49210534b233a.
- Descriptor-write closeout checkpoint:
  d00b1939ed49266b107d5d130a64e6851a5f628a.
- Retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.

## Physical Invariant

The future proof must demonstrate one physical invariant on serialized
Raspberry Pi 5 hardware:

1. Talos builds a focused rpi5_descriptor_write_proof boot scenario from the
   accepted QEMU descriptor-write smoke boundary.
2. The kernel constructs fixed lower-EL UserText, UserData, UserGuard, and
   UserStack ranges. UserData must provide physical backing storage for the
   stdout bytes at 0x0000_0000_0011_0000 and stderr bytes at
   0x0000_0000_0011_0040, plus a guard range that helper validation rejects.
3. The scenario initializes inherited fd 1 and fd 2 descriptors backed by
   runtime-console0 and keeps fd 0 non-writable for this write proof.
4. Before ERET, the kernel validates user ELR, user SP, SPSR/PSTATE, UserText,
   UserData, UserGuard, UserStack, inherited stdio descriptors, and
   runtime-console0 availability.
5. The lower-EL payload enters lower AArch64 and performs stable svc #0 with
   x8 = 1, x0 = 1, x1 = 0x0000_0000_0011_0000, x2 = 18, and x3 through x5 =
   0. Success returns x0 = 18 and runtime-console0 observes exactly the stdout
   bytes 74616c6f732d7374646f75742d727069350a.
6. The payload performs stable svc #0 with x8 = 1, x0 = 2,
   x1 = 0x0000_0000_0011_0040, x2 = 18, and x3 through x5 = 0. Success
   returns x0 = 18 and runtime-console0 observes exactly the stderr bytes
   74616c6f732d7374646572722d727069350a.
7. The payload performs stable svc #0 with x8 = 1 and fd 0 using the stdout
   byte range. It must return x0 = 0xffff_ffff_ffff_fff7, the
   two's-complement encoding of -EBADF, and must not add console bytes.
8. The payload performs stable svc #0 with x8 = 1 and fd 99 using the stdout
   byte range. It must return -EBADF and must not add console bytes.
9. The payload performs stable svc #0 with x8 = 1, fd 1,
   x1 = 0x0000_0000_001e_0000, x2 = 18, and x3 through x5 = 0. The guard
   address must return x0 = 0xffff_ffff_ffff_fff2, the two's-complement
   encoding of -EFAULT, and must not add console bytes.
10. The payload performs stable svc #0 with x8 = 1, fd 1, the stdout byte
    range, x2 = 18, x3 = 1, and x4/x5 = 0. It must return
    0xffff_ffff_ffff_ffea, the two's-complement encoding of -EINVAL, and must
    not add console bytes.
11. The payload performs stable svc #0 with x8 = 0 and observes x0 = 0,
    preserving the talos_nop scalar syscall invariant.
12. The payload performs stable svc #0 with x8 = 17 and observes x0 =
    0xffff_ffff_ffff_ffda, the two's-complement encoding of -ENOSYS,
    preserving the unknown-syscall invariant.
13. The payload performs stable svc #0 with proof-only x8 = 0x7001 and valid
    copy-probe-looking arguments. In this descriptor-write proof scenario it
    must return -ENOSYS, proving talos_copy_probe remains quarantined outside
    its accepted proof scenarios.
14. Only after those production svc #0 observations may the payload use the
    existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
    The marker must not dispatch as talos_write, talos_copy_probe, or any
    stable syscall.
15. The proof reports the planned physical completion classification and PASS
    line only after success, errno, regression, and diagnostic-quarantine
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
rpi5-descriptor-write-proof: final participants=8 expected=8 errors=0 classification=pi5-descriptor-write-proof-complete
rpi5-descriptor-write-proof: PASS
~~~

The retained serial log must also include these exact field names and stable
values:

~~~text
rpi5-descriptor-write-proof: start
rpi5-descriptor-write-proof: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true descriptor-table=inherited-stdio runtime-console=runtime-console0
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
~~~

The accepted PASS requires both runtime-console success observations, the
fd/error cases, scalar and unknown-syscall regressions, copy-probe quarantine,
diagnostic-marker quarantine, final classification, and PASS lines. Firmware
output, target init output, old QEMU evidence, or marker-only printing is not
enough.

## Diagnostic And Proof-Only Quarantine

talos_write is the only stable descriptor syscall selected by this plan.
talos_copy_probe remains proof-only:

- x8 = 0x7001 may dispatch to copy helpers only in its accepted proof
  scenarios, not in rpi5_descriptor_write_proof.
- In rpi5_descriptor_write_proof, x8 = 0x7001 must return -ENOSYS like any
  other unaccepted syscall number.
- SVC immediate 0x7a10 remains proof-owned diagnostic completion vocabulary.
  It must never become a syscall number, ABI version, selector,
  compatibility mode, descriptor operation, copy-probe operation, or
  production success path.
- The final PASS must fail if the marker appears before the descriptor-write
  success, errno, scalar-regression, and copy-probe quarantine observations, or
  if the marker is dispatched through the syscall core.

## Serialized Run Requirements

The future proof task must acquire hardwareTestLock before archive publishing,
power-cycle, serial observe, candidate boot, TFTP inspection, or restoration.
The lock owner and taskId must be phase7-pi5-descriptor-write-proof-20260529,
and the lock must be released after completion, failure, or pause.

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

A passing Pi 5 run would accept only the physical descriptor-write proof
invariant listed above: stable lower-AArch64 svc #0 reaches the production
syscall dispatch core on Pi 5, talos_write x8 = 1 writes fd 1 and fd 2 through
copy_from_user(), inherited stdio descriptors, and runtime-console0, fd and
pointer failures return the accepted negative errno values without extra
console bytes, talos_nop and unknown-syscall behavior remains intact, and
diagnostic marker 0x7a10 plus proof-only x8 = 0x7001 remain quarantined.

It would not accept stdin/read, close, dup, stable full POSIX descriptor
semantics, process-owned descriptor tables, process-owned address spaces,
blocking/readiness, partial writes, per-thread errno storage, restart
semantics, signals, resumable user faults, process loading, ELF parsing,
argv/envp setup, PID allocation, exit/wait, credentials, sessions,
controlling TTY, VFS, filesystem behavior, local shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

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
  observe, hardware-lock acquisition, or physical descriptor-write claim is
  made by this plan.

## Next Boundary

The next mechanically derivable task is
phase7-pi5-descriptor-write-proof-20260529, provided this plan is accepted
and hardwareTestLock is unlocked. It must acquire the lock before any hardware
action and must run exactly the serialized proof described here.
