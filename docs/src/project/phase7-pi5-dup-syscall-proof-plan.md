# Phase 7 Pi 5 Dup Syscall Proof Plan

Status: accepted candidate plan for
phase7-pi5-dup-syscall-proof-plan-20260529.

## Scope

This plan defines the serialized Raspberry Pi 5 proof that may carry the
accepted QEMU/substitute talos_dup invariant to physical hardware. It does not
change Rust or assembly code, publish a boot archive, power-cycle hardware,
observe serial output, acquire hardwareTestLock, add read syscall behavior,
process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
object finalization, RP1/PCIe policy, UART interrupt ownership, or
DMA/cache-driver policy.

The future hardware proof may touch only the source and lab surfaces needed to
stage and run the physical proof:

- build.rs for a focused rpi5_dup_syscall_proof boot-scenario cfg.
- src/boot/rpi5.rs and src/target/rpi5.rs for focused Pi 5 scenario dispatch,
  proof orchestration, physical UserData backing storage, runtime-console0
  observation, and serial output.
- src/arch/aarch64/exceptions.rs only if the Pi 5 lower-AArch64 synchronous
  SVC path needs the same bounded production routing already accepted by QEMU.
- src/syscall.rs only for using accepted talos_nop, talos_write, talos_close,
  talos_dup, unknown-syscall, and proof-only talos_copy_probe quarantine
  behavior.
- src/posix.rs only if a narrow descriptor-table or process-owner helper
  adjustment is required by the accepted dup syscall contract and remains
  covered by unit tests.
- src/runtime_console.rs only for a focused observation hook if the accepted
  runtime-console0 abstraction cannot already retain the required output.
- scripts/rpi5-dup-syscall-proof-image.sh,
  scripts/rpi5-dup-syscall-proof-boot-tree.sh, and a focused static/archive
  review helper if the implementation needs one.
- tasks/2026-05-29-phase7-pi5-dup-syscall-proof.md and retained evidence under
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/.

Any broader source ownership requires supervisor planning before execution.

## Accepted Inputs

- Dup syscall contract:
  041ca2f449afc9bd7889497720702b4f4f849bc3.
- Dup syscall core:
  2c30e4446f6611edb2bea1b75f226a6e919bf310.
- QEMU dup syscall smoke plan:
  37401fb7d9ff4924acd8a9ed072db1ec3441b261.
- QEMU dup syscall smoke core:
  5cce637bab95b227f5a98aba99b9104d2a017751.
- Dup syscall closeout checkpoint:
  84e2306b7e0d1397e3ea002d71c32f6d57556595.
- Retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.

## Physical Invariant

The future proof must demonstrate one physical invariant on serialized
Raspberry Pi 5 hardware:

1. Talos builds a focused rpi5_dup_syscall_proof boot scenario from the
   accepted QEMU dup syscall smoke boundary.
2. The kernel constructs fixed lower-EL UserText, UserData, UserGuard, and
   UserStack ranges. UserData must provide physical backing storage for the
   source stdout bytes at 0x0000_0000_0011_0000 and duplicate stdout bytes at
   0x0000_0000_0011_0040, plus a guard range that helper validation rejects.
3. The scenario initializes a four-slot current ProcessOwnerId-backed
   DescriptorTable::with_inherited_stdio() table with inherited fd 0, fd 1,
   and fd 2 descriptors. The only free descriptor before dup is fd 3.
4. Before ERET, the kernel validates user ELR, user SP, SPSR/PSTATE, UserText,
   UserData, UserGuard, UserStack, current ProcessOwnerId, inherited stdio
   descriptors, descriptor capacity, current-owner lookup, and runtime-console0
   availability.
5. The lower-EL payload enters lower AArch64 and performs stable svc #0 with
   x8 = 3, x0 = 1, and x1 through x5 = 0. Success returns x0 = 3, the lowest
   free descriptor, and leaves fd 1 occupied.
6. The payload performs stable svc #0 with x8 = 3, x0 = 2, and x1 through x5
   = 0. Because fd 3 is now occupied and the four-slot table is full, it must
   return x0 = 0xffff_ffff_ffff_ffe8, the two's-complement encoding of
   -EMFILE, and must leave the table unchanged.
7. The payload performs stable svc #0 with x8 = 3, x0 = 1, x1 = 1, and x2
   through x5 = 0. The reserved-register violation must return
   0xffff_ffff_ffff_ffea, the two's-complement encoding of -EINVAL, and must
   leave fd 1 and fd 3 unchanged.
8. The payload performs stable svc #0 with x8 = 1, x0 = 1,
   x1 = 0x0000_0000_0011_0000, x2 = 19, and x3 through x5 = 0. Success must
   return x0 = 19 and runtime-console0 must observe exactly the source bytes
   74616c6f732d6475702d7372632d727069350a.
9. The payload performs stable svc #0 with x8 = 1, x0 = 3,
   x1 = 0x0000_0000_0011_0040, x2 = 19, and x3 through x5 = 0. Success must
   return x0 = 19 and runtime-console0 must observe exactly the duplicate
   bytes 74616c6f732d6475702d6e65772d727069350a, proving descriptor-write
   dispatch uses the copied descriptor entry.
10. The payload performs stable svc #0 with x8 = 2, x0 = 1, and x1 through x5
    = 0. Success returns x0 = 0 and closes only the source stdout descriptor.
11. The payload performs stable svc #0 with x8 = 1, x0 = 1,
    x1 = 0x0000_0000_0011_0000, x2 = 19, and x3 through x5 = 0. It must
    return x0 = 0xffff_ffff_ffff_fff7, the two's-complement encoding of
    -EBADF, and must not add runtime-console0 bytes.
12. The payload performs stable svc #0 with x8 = 1, x0 = 3,
    x1 = 0x0000_0000_0011_0040, x2 = 19, and x3 through x5 = 0. Success must
    return x0 = 19 and runtime-console0 must observe the duplicate bytes
    again, proving close(fd 1) did not close fd 3.
13. The payload performs stable svc #0 with x8 = 2, x0 = 3, and x1 through x5
    = 0. Success returns x0 = 0 and closes only the duplicate descriptor.
14. The payload performs stable svc #0 with x8 = 1, x0 = 3,
    x1 = 0x0000_0000_0011_0040, x2 = 19, and x3 through x5 = 0. It must
    return -EBADF and must not add runtime-console0 bytes.
15. The payload performs stable svc #0 with x8 = 3, x0 = 1, and x1 through x5
    = 0 after fd 1 is already closed. It must return -EBADF and leave the
    table unchanged.
16. The payload performs stable svc #0 with x8 = 0 and observes x0 = 0,
    preserving the talos_nop scalar syscall invariant.
17. The payload performs stable svc #0 with x8 = 17 and observes x0 =
    0xffff_ffff_ffff_ffda, the two's-complement encoding of -ENOSYS,
    preserving the unknown-syscall invariant.
18. The payload performs stable svc #0 with proof-only x8 = 0x7001 and valid
    copy-probe-looking arguments. In this dup proof scenario it must return
    -ENOSYS, proving talos_copy_probe remains quarantined outside its accepted
    proof scenarios.
19. Only after those production svc #0 observations may the payload use the
    existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
    The marker must not dispatch as talos_dup, talos_close, talos_write,
    talos_copy_probe, or any stable syscall.
20. The proof reports the planned physical completion classification and PASS
    line only after current-owner lookup, dup success, table-full EMFILE,
    reserved-register EINVAL, writes through source and duplicate,
    close-one-descriptor independence, EBADF, regression, and
    diagnostic-quarantine observations have been recorded.

The expected ESR for all production syscalls is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional completion
marker is 0x0000000054007a10. FAR_ELx is expected to be zero for these SVC
paths unless the physical run reports a defined architectural value; any
nonzero FAR must be printed and justified in the task evidence.

## Required Output

The future hardware task must retain physical serial evidence with these exact
PASS/classification lines:

~~~text
rpi5-dup-syscall-proof: final participants=14 expected=14 errors=0 classification=pi5-dup-syscall-proof-complete
rpi5-dup-syscall-proof: PASS
~~~

The retained serial log must also include these exact field names and stable
values:

~~~text
rpi5-dup-syscall-proof: start
rpi5-dup-syscall-proof: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true process-owner=0x0000000000000001 current-owner=0x0000000000000001 descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 runtime-console=runtime-console0
rpi5-dup-syscall-proof: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited runtime-console=runtime-console0
rpi5-dup-syscall-proof: syscall case=dup_stdout vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 args=[x0=0x0000000000000001 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000003 lowest-free=true source-open=true
rpi5-dup-syscall-proof: syscall case=dup_stderr_full vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 return-x0=0xffffffffffffffe8 expected=-EMFILE table-unchanged=true
rpi5-dup-syscall-proof: syscall case=dup_stdout_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 return-x0=0xffffffffffffffea expected=-EINVAL table-unchanged=true
rpi5-dup-syscall-proof: syscall case=write_stdout_source vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000001 x1=0x0000000000110000 x2=0x0000000000000013 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000013
rpi5-dup-syscall-proof: runtime-console case=write_stdout_source device=runtime-console0 bytes=19 hex=74616c6f732d6475702d7372632d727069350a ok=true
rpi5-dup-syscall-proof: syscall case=write_stdout_duplicate vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000003 x1=0x0000000000110040 x2=0x0000000000000013 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000013
rpi5-dup-syscall-proof: runtime-console case=write_stdout_duplicate device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d727069350a ok=true
rpi5-dup-syscall-proof: syscall case=close_stdout_source vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 args=[x0=0x0000000000000001 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000000
rpi5-dup-syscall-proof: syscall case=write_stdout_source_after_close vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
rpi5-dup-syscall-proof: syscall case=write_duplicate_after_source_close vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000003 x1=0x0000000000110040 x2=0x0000000000000013 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000013
rpi5-dup-syscall-proof: runtime-console case=write_duplicate_after_source_close device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d727069350a ok=true
rpi5-dup-syscall-proof: syscall case=close_stdout_duplicate vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 args=[x0=0x0000000000000003 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000000
rpi5-dup-syscall-proof: syscall case=write_duplicate_after_duplicate_close vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
rpi5-dup-syscall-proof: syscall case=dup_closed_source vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 return-x0=0xfffffffffffffff7 expected=-EBADF table-unchanged=true
rpi5-dup-syscall-proof: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000
rpi5-dup-syscall-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-dup-syscall-proof: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false
rpi5-dup-syscall-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
~~~

The accepted PASS requires current-owner lookup, dup success, table-full
EMFILE, reserved-register EINVAL, writes through source and duplicate stdout
descriptors, close-one-descriptor independence, closed-descriptor EBADF
observations, scalar and unknown-syscall regressions, copy-probe quarantine,
diagnostic-marker quarantine, final classification, and PASS lines. Firmware
output, target init output, old QEMU evidence, or marker-only printing is not
enough.

## Diagnostic And Proof-Only Quarantine

talos_dup is the only new stable descriptor syscall selected by this plan.
talos_write and talos_close appear only to prove duplicated descriptor state
and independent descriptor lifetime. talos_copy_probe remains proof-only:

- x8 = 0x7001 may dispatch to copy helpers only in its accepted proof
  scenarios, not in rpi5_dup_syscall_proof.
- In rpi5_dup_syscall_proof, x8 = 0x7001 must return -ENOSYS like any other
  unaccepted syscall number.
- SVC immediate 0x7a10 remains proof-owned diagnostic completion vocabulary.
  It must never become a syscall number, ABI version, selector,
  compatibility mode, descriptor operation, copy-probe operation, process
  owner selector, or production success path.
- The final PASS must fail if the marker appears before the dup, write,
  close, EBADF, scalar-regression, and copy-probe quarantine observations, or
  if the marker is dispatched through the syscall core.

## Serialized Run Requirements

The future proof task must acquire hardwareTestLock before archive publishing,
power-cycle, serial observe, candidate boot, TFTP inspection, or restoration.
The lock owner and taskId must be phase7-pi5-dup-syscall-proof-20260529, and
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

A passing Pi 5 run would accept only the physical dup syscall proof invariant
listed above: stable lower-AArch64 svc #0 reaches the production syscall
dispatch core on Pi 5, talos_dup x8 = 3 duplicates fd 1 to fd 3 through the
current ProcessOwnerId-backed descriptor table, full tables return -EMFILE,
reserved registers return -EINVAL without mutation, writes through both source
and duplicate descriptors reach runtime-console0, close(fd 1) preserves fd 3,
closed descriptors and dup(closed fd 1) return -EBADF, talos_nop and
unknown-syscall behavior remains intact, and diagnostic marker 0x7a10 plus
proof-only x8 = 0x7001 remain quarantined.

Read behavior, stdin/read object modeling, process loading, VFS/filesystem,
shell, networking, SSH, object finalization, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, dup2/fcntl, and full POSIX descriptor
readiness remain blocked.

## Recommended Next Task

If accepted, the next bounded task is phase7-pi5-dup-syscall-proof-20260529.
That task should implement and run only the serialized physical proof defined
here, under hardwareTestLock, then restore the prior accepted boot tree.
