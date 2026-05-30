# Phase 7 QEMU Read And Stdin Smoke Plan

Status: accepted as the documentation-only Milestone 7.4 QEMU/substitute
read/stdin smoke plan after the accepted Phase 7 read/stdin core commit
613c85a1423677a764f031328530e59b3f7998ea. It does not add Rust
behavior, assembly behavior, QEMU execution, Pi 5 hardware execution, boot
archive publication, hardware-lock acquisition, runtime-console0/TTY/hardware
stdin, process loading, VFS/filesystem behavior, shell behavior, networking,
SSH, RP1/PCIe, UART interrupt ownership, object finalization, or
DMA/cache-driver policy.

This plan makes the next implementation task mechanical: add one QEMU-only or
substitute smoke that routes talos_read through the accepted lower-AArch64
svc #0 frame path, resolves fd 0 and a duplicate of fd 0 through the current
ProcessOwnerId-backed descriptor table, copies fixed proof stdin bytes to
UserData with copy_to_user(), proves bounded short-read and EOF behavior, and
retains deterministic success and errno evidence.

## Smoke Invariant

The next implementation task must demonstrate one bounded invariant:

1. Talos builds a QEMU-only boot scenario named qemu_read_stdin_smoke.
2. The scenario creates a target-independent ProcessOwnerId-backed
   ProcessDescriptorStore with one owner and one
   DescriptorTable::with_inherited_stdio() table before entering lower
   AArch64.
3. The syscall path resolves the current owner through the same
   ProcessDescriptorStore lookup API accepted by
   phase7-read-stdin-core-20260529 and applies talos_read through
   dispatch_process_descriptor_with_fixed_stdin().
4. The implementation must not bypass current-owner lookup, mutate a separate
   proof-owned DescriptorTable, or special-case read in the QEMU harness.
5. The kernel constructs a built-in lower-EL payload from fixed in-kernel bytes
   using the accepted QEMU user address-space shape plus one substitute data
   page:
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
6. The scenario initializes fixed proof stdin with the byte string
   talos-stdin-qemu\n and zeroes the UserData destination ranges before ERET.
   It validates user ELR, user SP, SPSR/PSTATE, UserText, UserData, UserGuard,
   UserStack, process-owner id, process-owned stdio table, fixed stdin length,
   and initial fixed stdin cursor.
7. The payload performs stable svc #0 with x8 = 3, x0 = 0, and x1 through x5 =
   0. Success returns x0 = 3, proving fd 0 duplicates to the lowest free slot
   before read observations.
8. The payload performs stable svc #0 with x8 = 4, fd 0,
   x1 = 0x0000_0000_001e_0000, x2 = 5, and x3 through x5 = 0. The guard
   destination must return x0 = 0xffff_ffff_ffff_fff2, the two's-complement
   encoding of -EFAULT, must not modify UserData, and must leave the fixed
   stdin cursor at 0.
9. The payload performs stable svc #0 with x8 = 4, fd 0,
   x1 = 0x0000_0000_0011_0080, x2 = 5, x3 = 1, and x4/x5 = 0. The
   reserved-register violation must return
   0xffff_ffff_ffff_ffea, the two's-complement encoding of -EINVAL, must not
   modify UserData, and must leave the fixed stdin cursor at 0.
10. The payload performs stable svc #0 with x8 = 4, fd 1,
    x1 = 0x0000_0000_0011_0080, x2 = 5, and x3 through x5 = 0. The
    non-readable stdout descriptor must return
    0xffff_ffff_ffff_fff7, the two's-complement encoding of -EBADF, without
    modifying UserData or consuming stdin.
11. The payload performs stable svc #0 with x8 = 4, fd 99,
    x1 = 0x0000_0000_0011_0080, x2 = 5, and x3 through x5 = 0. The invalid
    descriptor must return -EBADF without modifying UserData or consuming
    stdin.
12. The payload performs stable svc #0 with x8 = 4, fd 0,
    x1 = 0x0000_0000_0011_0080, x2 = 5, and x3 through x5 = 0. Success
    returns x0 = 5, UserData at 0x0000_0000_0011_0080 contains hex
    74616c6f73 (talos), and the fixed stdin cursor advances to 5.
13. The payload performs stable svc #0 with x8 = 4, fd 3,
    x1 = 0x0000_0000_0011_00a0, x2 = 32, and x3 through x5 = 0. Success
    returns x0 = 12, proving bounded proof-buffer exhaustion through the
    duplicated stdin descriptor. UserData at 0x0000_0000_0011_00a0 contains
    hex 2d737464696e2d71656d750a (-stdin-qemu\n), and the fixed stdin cursor
    advances to 17.
14. The payload performs stable svc #0 with x8 = 4, fd 0,
    x1 = 0x0000_0000_0011_00c0, x2 = 1, and x3 through x5 = 0. EOF returns
    x0 = 0, leaves the destination unchanged, and leaves the fixed stdin cursor
    at 17.
15. The payload performs stable svc #0 with x8 = 0 and observes x0 = 0,
    preserving the talos_nop scalar syscall invariant.
16. The payload performs stable svc #0 with x8 = 17 and observes x0 =
    0xffff_ffff_ffff_ffda, the two's-complement encoding of -ENOSYS,
    preserving the unknown-syscall invariant.
17. The payload performs stable svc #0 with proof-only x8 = 0x7001 and valid
    copy-probe-looking arguments. In this read/stdin smoke scenario it must
    return -ENOSYS, proving talos_copy_probe remains quarantined outside its
    accepted proof scenarios.
18. Only after those production svc #0 observations may the payload use the
    existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
    The marker must not dispatch as talos_read, talos_dup, talos_copy_probe, or
    any stable syscall.
19. The smoke prints final classification and PASS only after current-owner
    lookup, fd 0 duplication, read success, short-read, EOF, errno,
    regression, and diagnostic-quarantine observations have been recorded.

The expected ESR for all production syscall traps is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional diagnostic
completion marker is 0x0000000054007a10. FAR_ELx is expected to be zero for
SVC paths unless QEMU reports a defined architectural value; any nonzero FAR
must be printed and justified in the implementation evidence.

## Required Output

The implementation script must retain the serial log and grep these exact
PASS/classification lines:

    qemu-read-stdin-smoke: final participants=11 expected=11 errors=0 classification=qemu-read-stdin-smoke-complete
    qemu-read-stdin-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-read-stdin-smoke: start
    qemu-read-stdin-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true process-owner=0x0000000000000001 current-owner=0x0000000000000001 descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 fixed-stdin-len=17 fixed-stdin-cursor=0
    qemu-read-stdin-smoke: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited fixed-stdin=proof-buffer
    qemu-read-stdin-smoke: syscall case=dup_stdin vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000003 lowest-free=true source-open=true
    qemu-read-stdin-smoke: syscall case=read_guard vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff2 expected=-EFAULT fixed-stdin-cursor=0 user-unchanged=true
    qemu-read-stdin-smoke: syscall case=read_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xffffffffffffffea expected=-EINVAL fixed-stdin-cursor=0 user-unchanged=true
    qemu-read-stdin-smoke: syscall case=read_fd1 vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true
    qemu-read-stdin-smoke: syscall case=read_badfd vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true
    qemu-read-stdin-smoke: syscall case=read_stdin_first vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=[x0=0x0000000000000000 x1=0x0000000000110080 x2=0x0000000000000005 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000005 fixed-stdin-cursor=5
    qemu-read-stdin-smoke: user-buffer case=read_stdin_first addr=0x0000000000110080 bytes=5 hex=74616c6f73 ok=true
    qemu-read-stdin-smoke: user-observed case=read_stdin_first x0=0x0000000000000005 ok=true
    qemu-read-stdin-smoke: syscall case=read_stdin_duplicate_remaining vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=[x0=0x0000000000000003 x1=0x00000000001100a0 x2=0x0000000000000020 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x000000000000000c fixed-stdin-cursor=17 short-read=true
    qemu-read-stdin-smoke: user-buffer case=read_stdin_duplicate_remaining addr=0x00000000001100a0 bytes=12 hex=2d737464696e2d71656d750a ok=true
    qemu-read-stdin-smoke: user-observed case=read_stdin_duplicate_remaining x0=0x000000000000000c ok=true
    qemu-read-stdin-smoke: syscall case=read_stdin_eof vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0x0000000000000000 fixed-stdin-cursor=17 user-unchanged=true eof=true
    qemu-read-stdin-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000
    qemu-read-stdin-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
    qemu-read-stdin-smoke: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false
    qemu-read-stdin-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false

The implementation may print additional descriptor-slot, copied-buffer,
saved-state, or fixed-stdin fields, but the required lines must stay stable
enough for the script gate. If implementation work needs different owner ids,
user ELR, SP, UserData address, proof stdin bytes, descriptor capacity,
descriptor numbers, fixed stdin cursor semantics, or expected return values,
it must stop for supervisor planning instead of accepting a changed smoke.

## Source Owners

The next implementation task may touch only these source owners unless it
records a narrow reason:

- build.rs and src/main.rs for adding qemu_read_stdin_smoke boot-scenario
  routing.
- src/arch/aarch64/vectors.S and src/arch/aarch64/exceptions.rs only if the
  accepted lower-AArch64 saved-frame path needs a focused extension to pass
  process-owner and fixed-stdin context into dispatch.
- src/syscall.rs only for preserving talos_read routing through
  dispatch_process_descriptor_with_fixed_stdin(), reserved-register
  validation, errno return encoding, and proof-only copy-probe quarantine.
- src/posix.rs only for focused tests or helper exposure required by this
  smoke; no broad descriptor-table refactor is in scope.
- src/target/qemu_virt.rs for QEMU harness orchestration,
  ProcessDescriptorStore::create_owner_with_inherited_stdio(), substitute
  current-owner lookup, FixedStdin proof-buffer state, UserData
  mapping/backing storage, lower-EL payload bytes, required serial output, and
  final classification.
- scripts/qemu-read-stdin-smoke.sh for retained QEMU capture and grep gates.
- Existing scalar syscall, descriptor-write, close, and dup smoke scripts only
  as regression gates, not as broadened behavior.
- Documentation and the task record needed to report evidence.

Existing runtime-console0/TTY/hardware stdin, process-loader, VFS/filesystem,
Pi 5, RP1/PCIe, UART interrupt, object-finalizer, and DMA/cache-driver owners
remain out of scope for the first QEMU read/stdin smoke.

## Diagnostic And Proof-Only Quarantine

talos_read is the only new stable descriptor syscall selected by this plan.
The implementation must keep talos_copy_probe proof-only:

- x8 = 0x7001 may dispatch to copy helpers only in its accepted proof
  scenarios, not in qemu_read_stdin_smoke.
- In qemu_read_stdin_smoke, x8 = 0x7001 must return -ENOSYS like any other
  unaccepted syscall number.
- SVC immediate 0x7a10 remains proof-owned diagnostic completion vocabulary.
  It must never become a syscall number, ABI version, selector,
  compatibility mode, descriptor operation, copy-probe operation, process
  owner selector, stdin source selector, or production success path.
- The final PASS line must require current-owner lookup, fd 0 duplication,
  read success, bounded short read, EOF, EBADF/EFAULT/EINVAL cases, scalar
  regression, copy-probe quarantine, and diagnostic-marker observations.

## Evidence Retention

The implementation task must retain:

- The QEMU serial log at
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log
  unless the accepted implementation task records an equally specific retained
  path.
- The command used to build and run qemu_read_stdin_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-read-stdin-smoke.sh.
- scripts/qemu-syscall-smoke.sh if shared syscall dispatch, lower-EL routing,
  frame mutation, syscall-number handling, or talos_nop/unknown behavior
  changes.
- scripts/qemu-descriptor-write-smoke.sh if shared descriptor dispatch,
  copy_from_user/copy_to_user helpers, runtime-console0 descriptor behavior, or
  descriptor-write routing changes.
- scripts/qemu-close-syscall-smoke.sh if ProcessDescriptorStore ownership,
  descriptor close state, or table mutation helpers change.
- scripts/qemu-dup-syscall-smoke.sh if descriptor duplication, current-owner
  lookup, descriptor capacity, or inherited stdio table setup changes.
- scripts/qemu-pointer-copy-smoke.sh if proof scenario routing,
  talos_copy_probe quarantine, copy-helper behavior, or shared user-memory
  setup changes.
- git diff --check.
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly state that no
Pi 5 hardware behavior, runtime-console0/TTY/hardware stdin behavior,
filesystem-backed data, path copying, program loading, shell behavior,
networking, SSH support, or full POSIX descriptor readiness is claimed.

## Inconclusive QEMU Handling

QEMU capture failures are not Pi 5 hardware blockers. If the script cannot
classify the run, the worker should keep hardwareTestLock untouched and triage
only QEMU/staging facts in this order:

1. Confirm the built kernel was compiled with
   TALOS_BOOT_SCENARIO=qemu_read_stdin_smoke.
2. Confirm scripts/qemu-read-stdin-smoke.sh captured a fresh retained serial
   log path.
3. Confirm the log contains qemu-read-stdin-smoke: start before looking for
   PASS.
4. Confirm the validated line names process-owned inherited stdio and fixed
   stdin length/cursor.
5. Confirm dup_stdin returns fd 3 before any read consumes the proof buffer.
6. Confirm read_guard, read_reserved, read_fd1, and read_badfd preserve
   fixed-stdin-cursor=0 and user-unchanged=true.
7. Confirm read_stdin_first copies talos, advances cursor to 5, and reports
   the user-observed x0 value.
8. Confirm read_stdin_duplicate_remaining returns 12 bytes, reports
   short-read=true, copies -stdin-qemu\n, and advances cursor to 17.
9. Confirm read_stdin_eof returns zero without modifying the destination.
10. Confirm talos_nop, unknown-syscall, and copy_probe_quarantine regression
    lines appear before the diagnostic marker completion line.
11. Compare the generated kernel path and timestamp against the build command.
12. Rerun the QEMU script once after cleaning only stale QEMU output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 read/stdin hardware proof, archive publishing, power-cycle, serial
  observe, and hardware-lock acquisition.
- runtime-console0-backed stdin, TTY canonical/raw input, hardware UART input,
  pipes, sockets, regular files, directories, VFS/filesystem lookup, and
  process-loaded stdin.
- Process loading, process-owned address spaces beyond the substitute QEMU
  owner fixture, descriptor inheritance across exec, close-on-exec, descriptor
  flags, nonblocking/readiness, line discipline, terminal sessions, foreground
  process groups, and TTY input.
- Short reads caused by interrupts, readiness changes, canonical-line
  boundaries, signals, scheduler wakeups, or hardware input.
- Signals, restart semantics, poll/select, wait queues, per-thread errno,
  demand paging, copy-on-write, shared memory, lower-EL fault-table recovery,
  path copying, argv/envp loading, libc/Rust std stdio, shell behavior,
  networking, and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Mechanically Derivable Task

The next implementation/evidence task should be
phase7-qemu-read-stdin-smoke-core-20260529. Its goal should be to implement
only the QEMU/substitute qemu_read_stdin_smoke scenario and retained log gate
defined here, preserving the accepted target-independent read/stdin core,
descriptor write/close/dup behavior, scalar syscall regressions, and proof-only
talos_copy_probe quarantine.

It must not add Pi 5 proof, boot archive publication, hardwareTestLock
acquisition, runtime-console0/TTY/hardware stdin, process loading,
VFS/filesystem behavior, path copying, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, full POSIX
descriptor readiness, or any phase transition.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this plan, linked it from SUMMARY, updated
  roadmap current status, updated the decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
