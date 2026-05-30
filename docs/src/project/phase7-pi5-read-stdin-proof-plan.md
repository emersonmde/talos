# Phase 7 Pi 5 Read And Stdin Proof Plan

Status: accepted candidate plan for
phase7-pi5-read-stdin-proof-plan-20260530.

## Scope

This plan defines the serialized Raspberry Pi 5 proof that may carry the
accepted QEMU/substitute talos_read fixed-stdin invariant to physical
hardware. It does not change Rust or assembly code, publish a boot archive,
power-cycle hardware, observe serial output, acquire hardwareTestLock, attach
fd 0 to runtime-console0/TTY/hardware stdin, add process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, object finalization,
RP1/PCIe policy, UART interrupt ownership, or DMA/cache-driver policy.

The future hardware proof may touch only the source and lab surfaces needed to
stage and run the physical proof:

- build.rs for a focused rpi5_read_stdin_proof boot-scenario cfg.
- src/boot/rpi5.rs and src/target/rpi5.rs for focused Pi 5 scenario dispatch,
  proof orchestration, physical UserData backing storage, fixed proof stdin,
  and serial output.
- src/arch/aarch64/exceptions.rs only if the Pi 5 lower-AArch64 synchronous
  SVC path needs the same bounded production routing already accepted by QEMU.
- src/syscall.rs only for using accepted talos_nop, talos_read, talos_dup,
  unknown-syscall, and proof-only talos_copy_probe quarantine behavior.
- src/posix.rs only if a narrow fixed-stdin, ProcessDescriptorStore, or
  copy_to_user helper adjustment is required by the accepted read/stdin
  contract and remains covered by unit tests.
- scripts/rpi5-read-stdin-proof-image.sh,
  scripts/rpi5-read-stdin-proof-boot-tree.sh, and a focused static/archive
  review helper if the implementation needs one.
- tasks/2026-05-30-phase7-pi5-read-stdin-proof.md and retained evidence under
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/.

Any broader source ownership requires supervisor planning before execution.

## Accepted Inputs

- Read/stdin source inventory:
  c00267891b928e53b25c8ebdbe6a6a0dc549e0ae.
- Read/stdin contract:
  49d292935b4bff2220946e9eb7fe6b60de209a26.
- Read/stdin core:
  613c85a1423677a764f031328530e59b3f7998ea.
- QEMU read/stdin smoke plan:
  e48180bf4f61dbe1cc1294614c1acec7618fcbc9.
- QEMU read/stdin smoke core:
  cb0e816d68fa63d525c04fd6fd50ecae3d1960f8.
- Read/stdin closeout checkpoint:
  62eedfdc2b5b265f9ca400ee86b391d81fbfbee4.
- Retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log.

## Physical Invariant

The future proof must demonstrate one physical invariant on serialized
Raspberry Pi 5 hardware:

1. Talos builds a focused rpi5_read_stdin_proof boot scenario from the
   accepted QEMU read/stdin smoke boundary.
2. The kernel constructs fixed lower-EL UserText, UserData, UserGuard, and
   UserStack ranges. UserData must provide physical backing storage for the
   first fd 0 destination at 0x0000_0000_0011_0080 and the duplicated-stdin
   destination at 0x0000_0000_0011_00a0, plus a guard range that helper
   validation rejects.
3. The scenario initializes fixed proof stdin with the byte string
   talos-stdin-rpi5\n and a cursor at zero.
4. The scenario initializes a four-slot current ProcessOwnerId-backed
   DescriptorTable::with_inherited_stdio() table with inherited fd 0, fd 1,
   and fd 2 descriptors. The only free descriptor before dup is fd 3.
5. Before ERET, the kernel validates user ELR, user SP, SPSR/PSTATE,
   UserText, UserData, UserGuard, UserStack, current ProcessOwnerId,
   inherited stdio descriptors, descriptor capacity, current-owner lookup,
   fixed stdin length, and fixed stdin cursor.
6. The lower-EL payload enters lower AArch64 and performs stable svc #0 with
   x8 = 3, x0 = 0, and x1 through x5 = 0. Success returns x0 = 3, proving fd
   0 duplicates to the lowest free descriptor before read observations.
7. The payload performs stable svc #0 with x8 = 4, fd 0,
   x1 = 0x0000_0000_001e_0000, x2 = 5, and x3 through x5 = 0. The guard
   destination must return x0 = 0xffff_ffff_ffff_fff2, the two's-complement
   encoding of -EFAULT, must not modify UserData, and must leave the fixed
   stdin cursor at 0.
8. The payload performs stable svc #0 with x8 = 4, fd 0,
   x1 = 0x0000_0000_0011_0080, x2 = 5, x3 = 1, and x4/x5 = 0. The
   reserved-register violation must return 0xffff_ffff_ffff_ffea, the
   two's-complement encoding of -EINVAL, must not modify UserData, and must
   leave the fixed stdin cursor at 0.
9. The payload performs stable svc #0 with x8 = 4, fd 1,
   x1 = 0x0000_0000_0011_0080, x2 = 5, and x3 through x5 = 0. The
   non-readable stdout descriptor must return 0xffff_ffff_ffff_fff7, the
   two's-complement encoding of -EBADF, without modifying UserData or
   consuming stdin.
10. The payload performs stable svc #0 with x8 = 4, fd 99,
    x1 = 0x0000_0000_0011_0080, x2 = 5, and x3 through x5 = 0. The invalid
    descriptor must return -EBADF without modifying UserData or consuming
    stdin.
11. The payload performs stable svc #0 with x8 = 4, fd 0,
    x1 = 0x0000_0000_0011_0080, x2 = 5, and x3 through x5 = 0. Success
    returns x0 = 5, UserData at 0x0000_0000_0011_0080 contains hex 74616c6f73
    (talos), and the fixed stdin cursor advances to 5.
12. The payload performs stable svc #0 with x8 = 4, fd 3,
    x1 = 0x0000_0000_0011_00a0, x2 = 32, and x3 through x5 = 0. Success
    returns x0 = 12, proving bounded proof-buffer exhaustion through the
    duplicated stdin descriptor. UserData at 0x0000_0000_0011_00a0 contains
    hex 2d737464696e2d727069350a (-stdin-rpi5\n), and the fixed stdin cursor
    advances to 17.
13. The payload performs stable svc #0 with x8 = 4, fd 0,
    x1 = 0x0000_0000_0011_00c0, x2 = 1, and x3 through x5 = 0. EOF returns
    x0 = 0, leaves the destination unchanged, and leaves the fixed stdin
    cursor at 17.
14. The payload performs stable svc #0 with x8 = 0 and observes x0 = 0,
    preserving the talos_nop scalar syscall invariant.
15. The payload performs stable svc #0 with x8 = 17 and observes x0 =
    0xffff_ffff_ffff_ffda, the two's-complement encoding of -ENOSYS,
    preserving the unknown-syscall invariant.
16. The payload performs stable svc #0 with proof-only x8 = 0x7001 and valid
    copy-probe-looking arguments. In this read/stdin proof scenario it must
    return -ENOSYS, proving talos_copy_probe remains quarantined outside its
    accepted proof scenarios.
17. Only after those production svc #0 observations may the payload use the
    existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
    The marker must not dispatch as talos_read, talos_dup, talos_copy_probe,
    or any stable syscall.
18. The proof reports the planned physical completion classification and PASS
    line only after current-owner lookup, fd 0 duplication, read success,
    bounded short-read, EOF, errno, scalar-regression, copy-probe quarantine,
    and diagnostic-marker observations have been recorded.

The expected ESR for all production syscalls is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional completion
marker is 0x0000000054007a10. FAR_ELx is expected to be zero for these SVC
paths unless the physical run reports a defined architectural value; any
nonzero FAR must be printed and justified in the task evidence.

## Required Output

The future hardware task must retain physical serial evidence with these exact
PASS/classification lines:

~~~text
rpi5-read-stdin-proof: final participants=11 expected=11 errors=0 classification=pi5-read-stdin-proof-complete
rpi5-read-stdin-proof: PASS
~~~

The retained serial log must also include these exact field names and stable
values:

~~~text
rpi5-read-stdin-proof: start
rpi5-read-stdin-proof: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true process-owner=0x0000000000000001 current-owner=0x0000000000000001 descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 fixed-stdin-len=17 fixed-stdin-cursor=0
rpi5-read-stdin-proof: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited fixed-stdin=proof-buffer
rpi5-read-stdin-proof: syscall case=dup_stdin vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000003 lowest-free=true source-open=true
rpi5-read-stdin-proof: syscall case=read_guard vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff2 expected=-EFAULT fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xffffffffffffffea expected=-EINVAL fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_fd1 vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_badfd vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_stdin_first vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=[x0=0x0000000000000000 x1=0x0000000000110080 x2=0x0000000000000005 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000005 fixed-stdin-cursor=5
rpi5-read-stdin-proof: user-buffer case=read_stdin_first addr=0x0000000000110080 bytes=5 hex=74616c6f73 ok=true
rpi5-read-stdin-proof: user-observed case=read_stdin_first x0=0x0000000000000005 ok=true
rpi5-read-stdin-proof: syscall case=read_stdin_duplicate_remaining vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=[x0=0x0000000000000003 x1=0x00000000001100a0 x2=0x0000000000000020 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x000000000000000c fixed-stdin-cursor=17 short-read=true
rpi5-read-stdin-proof: user-buffer case=read_stdin_duplicate_remaining addr=0x00000000001100a0 bytes=12 hex=2d737464696e2d727069350a ok=true
rpi5-read-stdin-proof: user-observed case=read_stdin_duplicate_remaining x0=0x000000000000000c ok=true
rpi5-read-stdin-proof: syscall case=read_stdin_eof vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0x0000000000000000 fixed-stdin-cursor=17 user-unchanged=true eof=true
rpi5-read-stdin-proof: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000
rpi5-read-stdin-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-read-stdin-proof: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false
rpi5-read-stdin-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
~~~

The accepted PASS requires current-owner lookup, fd 0 duplication, read
success, bounded short read through the duplicated stdin descriptor, EOF,
EBADF/EFAULT/EINVAL cases, scalar and unknown-syscall regressions,
copy-probe quarantine, diagnostic-marker quarantine, final classification,
and PASS lines. Firmware output, target init output, old QEMU evidence, or
marker-only printing is not enough.

## Diagnostic And Proof-Only Quarantine

talos_read is the only stable descriptor syscall selected by this plan.
talos_dup appears only to create the duplicated stdin descriptor before the
read observations. talos_copy_probe remains proof-only:

- x8 = 0x7001 may dispatch to copy helpers only in its accepted proof
  scenarios, not in rpi5_read_stdin_proof.
- In rpi5_read_stdin_proof, x8 = 0x7001 must return -ENOSYS like any other
  unaccepted syscall number.
- SVC immediate 0x7a10 remains proof-owned diagnostic completion vocabulary.
  It must never become a syscall number, ABI version, selector,
  compatibility mode, descriptor operation, copy-probe operation, process
  owner selector, stdin source selector, or production success path.
- The final PASS must fail if the marker appears before the dup, read, EOF,
  errno, scalar-regression, and copy-probe quarantine observations, or if the
  marker is dispatched through the syscall core.

## Serialized Run Requirements

The future proof task must acquire hardwareTestLock before archive publishing,
power-cycle, serial observe, candidate boot, TFTP inspection, or restoration.
The lock owner and taskId must be phase7-pi5-read-stdin-proof-20260530, and
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

A passing Pi 5 run would accept only the physical read/stdin proof invariant
listed above: stable lower-AArch64 svc #0 reaches the production syscall
dispatch core on Pi 5, talos_read x8 = 4 resolves fd 0 and duplicated fd 3
through the current ProcessOwnerId-backed descriptor table, copy_to_user()
delivers fixed proof stdin bytes into the physical UserData backing, guard
copy-out returns -EFAULT without consuming stdin, reserved arguments return
-EINVAL without mutation, non-readable and invalid descriptors return -EBADF,
bounded proof-buffer exhaustion produces a short read followed by EOF,
talos_nop and unknown-syscall behavior remains intact, and diagnostic marker
0x7a10 plus proof-only x8 = 0x7001 remain quarantined.

It would not accept runtime-console0 input, TTY canonical/raw stdin,
hardware UART input, pipes, sockets, regular files, filesystem reads,
blocking/readiness, nonblocking flags, poll/select readiness, wait queues,
signals, restart semantics, process-owned address spaces, process loading,
ELF parsing, argv/envp setup, PID allocation, exit/wait, credentials,
sessions, controlling TTY, VFS, filesystem behavior, local shell, networking,
SSH, object finalization, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

QEMU/substitute evidence remains useful for regression comparison, but only
serial hardware boot/output with candidate identity, fresh TFTP evidence,
hardwareTestLock ownership, and restoration proof can establish the Pi 5
claim.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed the accepted read/stdin
  source inventory, contract, target-independent core, QEMU smoke plan,
  retained QEMU log, read/stdin closeout, and prior Pi 5 close/dup proof-plan
  patterns.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required because this task changes
  only Markdown documentation and durable worker state.

## Next Task

The next bounded task is phase7-pi5-read-stdin-proof-20260530, scoped to
implementing and running only the serialized physical proof described here if
hardwareTestLock is unlocked/restored and dependencies remain accepted.
