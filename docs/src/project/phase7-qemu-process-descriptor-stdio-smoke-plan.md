# Phase 7 QEMU Process Descriptor Stdio Smoke Plan

Status: accepted as the documentation-only Milestone 7.4 QEMU/substitute
process descriptor stdio smoke plan after the accepted process descriptor
table core commit a30944d53aefd58ca89a7d197d12bae0790beb73.
It does not add Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware execution, boot archive publication, hardware-lock acquisition,
close/dup/read syscalls, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

This plan makes the next implementation task mechanical: add one QEMU-only
or substitute smoke that proves lower-AArch64 talos_write fd 1 and fd 2 route
through a ProcessOwnerId-backed process-owned inherited stdio table, not the
earlier proof-owned ad hoc inherited stdio table. The task must carry forward
the accepted descriptor-write success, fd/error, scalar syscall, copy-probe
quarantine, and diagnostic-marker quarantine observations.

## Smoke Invariant

The next implementation task must demonstrate one bounded invariant:

1. Talos builds a QEMU-only boot scenario named
   qemu_process_descriptor_stdio_smoke.
2. The scenario creates a target-independent ProcessOwnerId-backed
   ProcessDescriptorStore with one owner and one
   DescriptorTable::with_inherited_stdio() table before entering lower
   AArch64.
3. The syscall path resolves the current owner through the same
   ProcessDescriptorStore lookup API accepted by
   phase7-process-descriptor-table-core-20260529 and borrows that owner table
   for talos_write dispatch.
4. The implementation must not construct a separate proof-owned
   DescriptorTable for talos_write after the process-owned store exists. The
   retained evidence must identify the process owner, the current-owner lookup,
   the process-owned inherited stdio table, and runtime-console0.
5. The kernel constructs a built-in lower-EL payload from fixed in-kernel
   bytes using the accepted QEMU user address-space shape plus one substitute
   data page:
   - UserText:
     0x0000_0000_0010_0000..0x0000_0000_0010_1000, readable and executable,
     not writable.
   - UserData:
     0x0000_0000_0011_0000..0x0000_0000_0011_1000, readable and writable,
     not executable.
   - UserGuard:
     0x0000_0000_001e_0000..0x0000_0000_001f_0000, unmapped or no access.
   - UserStack:
     0x0000_0000_001f_0000..0x0000_0000_0020_0000, readable and writable,
     not executable.
6. The scenario initializes UserData with two fixed byte strings:
   stdout bytes at 0x0000_0000_0011_0000 and stderr bytes at
   0x0000_0000_0011_0040. It validates user ELR, user SP, SPSR/PSTATE,
   UserText, UserData, UserGuard, UserStack, process-owner id, and
   process-owned stdio table before ERET.
7. The payload enters lower AArch64 and performs stable svc #0 with x8 = 1,
   x0 = 1, x1 = 0x0000_0000_0011_0000, x2 = 18, and x3 through x5 = 0.
   Success returns x0 = 18 and runtime-console0 observes exactly those stdout
   bytes through the process-owned descriptor table.
8. The payload performs stable svc #0 with x8 = 1, x0 = 2,
   x1 = 0x0000_0000_0011_0040, x2 = 18, and x3 through x5 = 0. Success
   returns x0 = 18 and runtime-console0 observes exactly those stderr bytes
   through the process-owned descriptor table.
9. The payload performs stable svc #0 with x8 = 1 and fd 0 using the stdout
   byte range. It must return x0 = 0xffff_ffff_ffff_fff7, the two's-complement
   encoding of -EBADF, and must not add console bytes.
10. The payload performs stable svc #0 with x8 = 1 and fd 99 using the stdout
    byte range. It must return -EBADF and must not add console bytes.
11. The payload performs stable svc #0 with x8 = 1, fd 1,
    x1 = 0x0000_0000_001e_0000, x2 = 18, and x3 through x5 = 0. The guard
    address must return x0 = 0xffff_ffff_ffff_fff2, the two's-complement
    encoding of -EFAULT, and must not add console bytes.
12. The payload performs stable svc #0 with x8 = 1, fd 1, the stdout byte
    range, x2 = 18, x3 = 1, and x4/x5 = 0. It must return
    0xffff_ffff_ffff_ffea, the two's-complement encoding of -EINVAL, and must
    not add console bytes.
13. The payload performs stable svc #0 with x8 = 0 and observes x0 = 0,
    preserving the talos_nop scalar syscall invariant.
14. The payload performs stable svc #0 with x8 = 17 and observes x0 =
    0xffff_ffff_ffff_ffda, the two's-complement encoding of -ENOSYS,
    preserving the unknown-syscall invariant.
15. The payload performs stable svc #0 with proof-only x8 = 0x7001 and valid
    copy-probe-looking arguments. In this process descriptor stdio smoke
    scenario it must return -ENOSYS, proving talos_copy_probe remains
    quarantined outside its accepted proof scenarios.
16. Only after those production svc #0 observations may the payload use the
    existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
    The marker must not dispatch as talos_write, talos_copy_probe, or any
    stable syscall.
17. The smoke prints final classification and PASS only after process-owner
    lookup, success, errno, regression, and diagnostic-quarantine observations
    have been recorded.

The expected ESR for all production syscall traps is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional diagnostic
completion marker is 0x0000000054007a10. FAR_ELx is expected to be zero for
SVC paths unless QEMU reports a defined architectural value; any nonzero FAR
must be printed and justified in the implementation evidence.

## Required Output

The implementation script must retain the serial log and grep these exact
PASS/classification lines:

    qemu-process-descriptor-stdio-smoke: final participants=8 expected=8 errors=0 classification=qemu-process-descriptor-stdio-smoke-complete
    qemu-process-descriptor-stdio-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-process-descriptor-stdio-smoke: start
    qemu-process-descriptor-stdio-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true process-owner=0x0000000000000001 current-owner=0x0000000000000001 descriptor-table=process-owned-inherited-stdio runtime-console=runtime-console0
    qemu-process-descriptor-stdio-smoke: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited runtime-console=runtime-console0
    qemu-process-descriptor-stdio-smoke: syscall case=write_stdout vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000001 x1=0x0000000000110000 x2=0x0000000000000012 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000012
    qemu-process-descriptor-stdio-smoke: runtime-console case=write_stdout device=runtime-console0 bytes=18 hex=74616c6f732d7374646f75742d71656d750a ok=true
    qemu-process-descriptor-stdio-smoke: user-observed case=write_stdout x0=0x0000000000000012 ok=true
    qemu-process-descriptor-stdio-smoke: syscall case=write_stderr vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000002 x1=0x0000000000110040 x2=0x0000000000000012 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000012
    qemu-process-descriptor-stdio-smoke: runtime-console case=write_stderr device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d71656d750a ok=true
    qemu-process-descriptor-stdio-smoke: user-observed case=write_stderr x0=0x0000000000000012 ok=true
    qemu-process-descriptor-stdio-smoke: syscall case=write_fd0 vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
    qemu-process-descriptor-stdio-smoke: syscall case=write_badfd vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
    qemu-process-descriptor-stdio-smoke: syscall case=write_efault vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff2 expected=-EFAULT console-unchanged=true
    qemu-process-descriptor-stdio-smoke: syscall case=write_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xffffffffffffffea expected=-EINVAL console-unchanged=true
    qemu-process-descriptor-stdio-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000
    qemu-process-descriptor-stdio-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
    qemu-process-descriptor-stdio-smoke: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false
    qemu-process-descriptor-stdio-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false

The implementation may print additional source-owner, saved-state,
descriptor-entry, copied-buffer, or console-capture fields, but the required
lines must stay stable enough for the script gate. If implementation work
needs different owner ids, user ELR, SP, UserData address, byte strings,
lengths, descriptor numbers, or expected return values, it must stop for
supervisor planning instead of accepting a changed smoke.

## Source Owners

The next implementation task may touch only these source owners unless it
records a narrow reason:

- build.rs and src/main.rs for adding qemu_process_descriptor_stdio_smoke
  boot-scenario routing.
- src/arch/aarch64/vectors.S and src/arch/aarch64/exceptions.rs only if the
  accepted lower-AArch64 saved-frame path needs a focused extension to pass
  process-owner context into dispatch.
- src/syscall.rs only for routing talos_write through a caller-provided
  process-owned descriptor table while preserving reserved-register
  validation, errno return encoding, and proof-only copy-probe quarantine.
- src/posix.rs only for ProcessDescriptorStore lookup helpers, descriptor
  table borrow plumbing, or focused tests required by this smoke.
- src/runtime_console.rs only for a narrow capture hook needed to observe
  runtime-console0 output through the accepted abstraction.
- src/scheduler.rs only for a narrow current ProcessOwnerId fixture or metadata
  accessor required by the accepted contract.
- src/target/qemu_virt.rs for QEMU harness orchestration,
  ProcessDescriptorStore::create_owner_with_inherited_stdio(), substitute
  current-owner lookup, UserData mapping/backing storage, lower-EL payload
  bytes, required serial output, and final classification.
- scripts/qemu-process-descriptor-stdio-smoke.sh for retained QEMU capture and
  grep gates.
- Existing scalar syscall, pointer-copy, and descriptor-write smoke scripts
  only as regression gates, not as broadened behavior.
- Documentation and the task record needed to report evidence.

Existing stdin/read, close, dup, process-loader, VFS/filesystem, Pi 5,
RP1/PCIe, UART interrupt, and DMA/cache-driver owners remain out of scope for
the first process-owned descriptor-table smoke.

## Diagnostic And Proof-Only Quarantine

talos_write remains the only stable descriptor syscall selected by this plan.
The implementation must keep talos_copy_probe proof-only:

- x8 = 0x7001 may dispatch to copy helpers only in its accepted proof
  scenarios, not in qemu_process_descriptor_stdio_smoke.
- In qemu_process_descriptor_stdio_smoke, x8 = 0x7001 must return -ENOSYS like
  any other unaccepted syscall number.
- SVC immediate 0x7a10 remains proof-owned diagnostic completion vocabulary.
  It must never become a syscall number, ABI version, selector,
  compatibility mode, descriptor operation, copy-probe operation, process
  owner selector, or production success path.
- The final PASS line must require process-owner lookup, descriptor-write
  success, errno, scalar-regression, copy-probe quarantine, and
  diagnostic-marker observations.

## Evidence Retention

The implementation task must retain:

- The QEMU serial log at
  tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log
  unless the accepted implementation task records an equally specific retained
  path.
- The command used to build and run qemu_process_descriptor_stdio_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-process-descriptor-stdio-smoke.sh.
- scripts/qemu-descriptor-write-smoke.sh as the proof-owned descriptor-write
  regression.
- scripts/qemu-syscall-smoke.sh if shared syscall dispatch, lower-EL routing,
  frame mutation, syscall-number handling, or talos_nop/unknown behavior
  changes.
- scripts/qemu-pointer-copy-smoke.sh if proof scenario routing,
  talos_copy_probe quarantine, copy-helper behavior, or shared user-memory
  setup changes.
- git diff --check.
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly state that no
Pi 5 hardware behavior, stdin/read behavior, close/dup behavior,
filesystem-backed data, path copying, program loading, shell behavior,
networking, or SSH support is claimed.

## Inconclusive QEMU Handling

QEMU capture failures are not Pi 5 hardware blockers. If the script cannot
classify the run, the worker should keep hardwareTestLock untouched and triage
only QEMU/staging facts in this order:

1. Confirm the built kernel was compiled with
   TALOS_BOOT_SCENARIO=qemu_process_descriptor_stdio_smoke.
2. Confirm scripts/qemu-process-descriptor-stdio-smoke.sh captured a fresh
   retained serial log path.
3. Confirm the log contains qemu-process-descriptor-stdio-smoke: start before
   looking for PASS.
4. Confirm the validated line names process-owned inherited stdio,
   current-owner lookup, process-owner id, and runtime-console0.
5. Confirm write_stdout and write_stderr success lines contain
   descriptor-owner=0x0000000000000001 before their runtime-console
   observation lines.
6. Confirm the fd 0, bad fd, guard-range, and reserved-register failures
   appear with console-unchanged=true before the diagnostic marker completion
   line.
7. Confirm talos_nop, unknown-syscall, and copy_probe_quarantine regression
   lines appear before the diagnostic marker completion line.
8. Compare the generated kernel path and timestamp against the build command.
9. Rerun the QEMU script once after cleaning only stale QEMU output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 process-descriptor stdio proof, archive publishing, power-cycle, serial
  observe, and hardware-lock acquisition.
- stdin/read, close, dup, descriptor inheritance across process creation,
  close-on-exec, descriptor flags, nonblocking/readiness, line discipline, and
  TTY input.
- PID allocation, fork/spawn/exec, process loading, process-owned address
  spaces, VFS/filesystem behavior, path copying, shell behavior, networking,
  and SSH.
- Short writes, partial-copy/restart semantics, signals, per-thread errno,
  demand paging, copy-on-write, shared memory, and lower-EL fault-table
  recovery.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Mechanically Derivable Task

The next implementation task should be
phase7-qemu-process-descriptor-stdio-smoke-core-20260529. Its goal should be
to implement only the QEMU/substitute process-owned inherited stdio smoke,
retaining exact lower-AArch64 talos_write fd 1/fd 2, fd/error, scalar
syscall, copy-probe quarantine, and diagnostic-marker observations.

It must not add Pi 5 proof, stdin/read, close, dup, process loading,
VFS/filesystem behavior, path copying, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or any phase
transition.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this plan, linked it from SUMMARY, updated
  roadmap current status, updated the decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
