# Phase 7 QEMU Dup Syscall Smoke Plan

Status: accepted as the documentation-only Milestone 7.4 QEMU/substitute dup
syscall smoke plan after the accepted dup syscall core commit
2c30e4446f6611edb2bea1b75f226a6e919bf310. It does not add Rust behavior,
assembly behavior, QEMU execution, Pi 5 hardware execution, boot archive
publication, hardware-lock acquisition, read behavior, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, object finalization, or DMA/cache-driver policy.

This plan makes the next implementation task mechanical: add one QEMU-only or
substitute smoke that proves lower-AArch64 talos_dup duplicates stdout through
the current ProcessOwnerId-backed process-owned descriptor table, writes
through both the source and duplicate, closes one descriptor without closing
the other, and preserves deterministic EBADF, EMFILE, EINVAL, talos_nop,
unknown-syscall, close, descriptor-write, and diagnostic-quarantine behavior.

## Smoke Invariant

The next implementation task must demonstrate one bounded invariant:

1. Talos builds a QEMU-only boot scenario named qemu_dup_syscall_smoke.
2. The scenario creates a target-independent ProcessOwnerId-backed
   ProcessDescriptorStore with one owner and one four-slot
   DescriptorTable::with_inherited_stdio() table before entering lower
   AArch64.
3. The syscall path resolves the current owner through the same
   ProcessDescriptorStore lookup API accepted by
   phase7-dup-syscall-core-20260529 and applies talos_dup through
   ProcessDescriptorStore::dup_current_descriptor().
4. The implementation must not bypass current-owner lookup, mutate a separate
   proof-owned DescriptorTable, or special-case dup in the QEMU harness.
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
   source-write bytes at 0x0000_0000_0011_0000 and duplicate-write bytes at
   0x0000_0000_0011_0040. It validates user ELR, user SP, SPSR/PSTATE,
   UserText, UserData, UserGuard, UserStack, process-owner id, current-owner
   id, process-owned stdio table, descriptor capacity, and runtime-console0
   capture before ERET.
7. The payload performs stable svc #0 with x8 = 3, x0 = 1, and x1 through x5
   = 0. Success returns x0 = 3, the lowest free descriptor in the four-slot
   inherited stdio table, and leaves fd 1 occupied.
8. The payload performs stable svc #0 with x8 = 3, x0 = 2, and x1 through x5
   = 0. Because fd 3 is now occupied and the table is full, it returns
   x0 = 0xffff_ffff_ffff_ffe8, the two's-complement encoding of -EMFILE, and
   leaves the table unchanged.
9. The payload performs stable svc #0 with x8 = 3, x0 = 1, x1 = 1, and x2
   through x5 = 0. The reserved-register violation must return
   0xffff_ffff_ffff_ffea, the two's-complement encoding of -EINVAL, and must
   leave fd 1 and fd 3 unchanged.
10. The payload performs stable svc #0 with x8 = 1, x0 = 1,
    x1 = 0x0000_0000_0011_0000, x2 = 19, and x3 through x5 = 0. Success
    returns x0 = 19 and runtime-console0 observes exactly those source bytes.
11. The payload performs stable svc #0 with x8 = 1, x0 = 3,
    x1 = 0x0000_0000_0011_0040, x2 = 19, and x3 through x5 = 0. Success
    returns x0 = 19 and runtime-console0 observes exactly those duplicate
    bytes, proving descriptor-write dispatch uses the copied descriptor entry.
12. The payload performs stable svc #0 with x8 = 2, x0 = 1, and x1 through x5
    = 0. Success returns x0 = 0 and closes only the source stdout descriptor.
13. The payload performs stable svc #0 with x8 = 1, x0 = 1,
    x1 = 0x0000_0000_0011_0000, x2 = 19, and x3 through x5 = 0. The closed
    source descriptor must return x0 = 0xffff_ffff_ffff_fff7, the
    two's-complement encoding of -EBADF, and must not add console bytes.
14. The payload performs stable svc #0 with x8 = 1, x0 = 3,
    x1 = 0x0000_0000_0011_0040, x2 = 19, and x3 through x5 = 0. Success
    returns x0 = 19 and runtime-console0 observes the duplicate bytes again,
    proving close(fd 1) did not close fd 3.
15. The payload performs stable svc #0 with x8 = 2, x0 = 3, and x1 through x5
    = 0. Success returns x0 = 0 and closes only the duplicate descriptor.
16. The payload performs stable svc #0 with x8 = 1, x0 = 3,
    x1 = 0x0000_0000_0011_0040, x2 = 19, and x3 through x5 = 0. The closed
    duplicate descriptor must return -EBADF and must not add console bytes.
17. The payload performs stable svc #0 with x8 = 3, x0 = 1, and x1 through x5
    = 0. Duplicating the already closed source descriptor must return -EBADF
    and leave the table unchanged.
18. The payload performs stable svc #0 with x8 = 0 and observes x0 = 0,
    preserving the talos_nop scalar syscall invariant.
19. The payload performs stable svc #0 with x8 = 17 and observes x0 =
    0xffff_ffff_ffff_ffda, the two's-complement encoding of -ENOSYS,
    preserving the unknown-syscall invariant.
20. The payload performs stable svc #0 with proof-only x8 = 0x7001 and valid
    copy-probe-looking arguments. In this dup smoke scenario it must return
    -ENOSYS, proving talos_copy_probe remains quarantined outside its accepted
    proof scenarios.
21. Only after those production svc #0 observations may the payload use the
    existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
    The marker must not dispatch as talos_dup, talos_close, talos_write,
    talos_copy_probe, or any stable syscall.
22. The smoke prints final classification and PASS only after current-owner
    lookup, dup success, table-full EMFILE, reserved-register EINVAL, writes
    through source and duplicate, close-one-descriptor independence, EBADF,
    regression, and diagnostic-quarantine observations have been recorded.

The expected ESR for all production syscall traps is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional diagnostic
completion marker is 0x0000000054007a10. FAR_ELx is expected to be zero for
SVC paths unless QEMU reports a defined architectural value; any nonzero FAR
must be printed and justified in the implementation evidence.

## Required Output

The implementation script must retain the serial log and grep these exact
PASS/classification lines:

    qemu-dup-syscall-smoke: final participants=14 expected=14 errors=0 classification=qemu-dup-syscall-smoke-complete
    qemu-dup-syscall-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-dup-syscall-smoke: start
    qemu-dup-syscall-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true process-owner=0x0000000000000001 current-owner=0x0000000000000001 descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 runtime-console=runtime-console0
    qemu-dup-syscall-smoke: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited runtime-console=runtime-console0
    qemu-dup-syscall-smoke: syscall case=dup_stdout vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 args=[x0=0x0000000000000001 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000003 lowest-free=true source-open=true
    qemu-dup-syscall-smoke: syscall case=dup_stderr_full vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 return-x0=0xffffffffffffffe8 expected=-EMFILE table-unchanged=true
    qemu-dup-syscall-smoke: syscall case=dup_stdout_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 return-x0=0xffffffffffffffea expected=-EINVAL table-unchanged=true
    qemu-dup-syscall-smoke: syscall case=write_stdout_source vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000001 x1=0x0000000000110000 x2=0x0000000000000013 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000013
    qemu-dup-syscall-smoke: runtime-console case=write_stdout_source device=runtime-console0 bytes=19 hex=74616c6f732d6475702d7372632d71656d750a ok=true
    qemu-dup-syscall-smoke: syscall case=write_stdout_duplicate vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000003 x1=0x0000000000110040 x2=0x0000000000000013 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000013
    qemu-dup-syscall-smoke: runtime-console case=write_stdout_duplicate device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d71656d750a ok=true
    qemu-dup-syscall-smoke: syscall case=close_stdout_source vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 args=[x0=0x0000000000000001 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000000
    qemu-dup-syscall-smoke: syscall case=write_stdout_source_after_close vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
    qemu-dup-syscall-smoke: syscall case=write_duplicate_after_source_close vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=[x0=0x0000000000000003 x1=0x0000000000110040 x2=0x0000000000000013 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000013
    qemu-dup-syscall-smoke: runtime-console case=write_duplicate_after_source_close device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d71656d750a ok=true
    qemu-dup-syscall-smoke: syscall case=close_stdout_duplicate vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 args=[x0=0x0000000000000003 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000000
    qemu-dup-syscall-smoke: syscall case=write_duplicate_after_duplicate_close vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true
    qemu-dup-syscall-smoke: syscall case=dup_closed_source vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 return-x0=0xfffffffffffffff7 expected=-EBADF table-unchanged=true
    qemu-dup-syscall-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000
    qemu-dup-syscall-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
    qemu-dup-syscall-smoke: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false
    qemu-dup-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false

The implementation may print additional descriptor-slot, table-full, copied
entry, saved-state, or console-capture fields, but the required lines must
stay stable enough for the script gate. If implementation work needs different
owner ids, user ELR, SP, UserData address, byte strings, descriptor capacity,
descriptor numbers, or expected return values, it must stop for supervisor
planning instead of accepting a changed smoke.

## Source Owners

The next implementation task may touch only these source owners unless it
records a narrow reason:

- build.rs and src/main.rs for adding qemu_dup_syscall_smoke boot-scenario
  routing.
- src/arch/aarch64/vectors.S and src/arch/aarch64/exceptions.rs only if the
  accepted lower-AArch64 saved-frame path needs a focused extension to pass
  process-owner context into dispatch.
- src/syscall.rs only for preserving talos_dup routing through
  dispatch_process_descriptor(), reserved-register validation, errno return
  encoding, and proof-only copy-probe quarantine.
- src/posix.rs only for focused tests or helper exposure required by this
  smoke; no broad descriptor-table refactor is in scope.
- src/runtime_console.rs only for a narrow capture hook needed to observe
  runtime-console0 output through the accepted abstraction.
- src/target/qemu_virt.rs for QEMU harness orchestration,
  ProcessDescriptorStore::create_owner_with_inherited_stdio(), substitute
  current-owner lookup, UserData mapping/backing storage, lower-EL payload
  bytes, required serial output, and final classification.
- scripts/qemu-dup-syscall-smoke.sh for retained QEMU capture and grep gates.
- Existing scalar syscall, descriptor-write, and close smoke scripts only as
  regression gates, not as broadened behavior.
- Documentation and the task record needed to report evidence.

Existing stdin/read, process-loader, VFS/filesystem, Pi 5, RP1/PCIe, UART
interrupt, object-finalizer, and DMA/cache-driver owners remain out of scope
for the first QEMU dup syscall smoke.

## Diagnostic And Proof-Only Quarantine

talos_dup is the only new stable descriptor syscall selected by this plan. The
implementation must keep talos_copy_probe proof-only:

- x8 = 0x7001 may dispatch to copy helpers only in its accepted proof
  scenarios, not in qemu_dup_syscall_smoke.
- In qemu_dup_syscall_smoke, x8 = 0x7001 must return -ENOSYS like any other
  unaccepted syscall number.
- SVC immediate 0x7a10 remains proof-owned diagnostic completion vocabulary.
  It must never become a syscall number, ABI version, selector,
  compatibility mode, descriptor operation, copy-probe operation, process
  owner selector, or production success path.
- The final PASS line must require current-owner lookup, dup success,
  table-full EMFILE, reserved-register EINVAL, descriptor-write through both
  source and duplicate, close-one-descriptor preservation, EBADF, scalar
  regression, copy-probe quarantine, and diagnostic-marker observations.

## Evidence Retention

The implementation task must retain:

- The QEMU serial log at
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log
  unless the accepted implementation task records an equally specific retained
  path.
- The command used to build and run qemu_dup_syscall_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-dup-syscall-smoke.sh.
- scripts/qemu-descriptor-write-smoke.sh as the descriptor-write regression.
- scripts/qemu-close-syscall-smoke.sh as the close/lifetime regression.
- scripts/qemu-syscall-smoke.sh if shared syscall dispatch, lower-EL routing,
  frame mutation, syscall-number handling, or talos_nop/unknown behavior
  changes.
- git diff --check.
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly state that no
Pi 5 hardware behavior, read behavior, filesystem-backed data, path copying,
program loading, shell behavior, networking, or SSH support is claimed.

## Inconclusive QEMU Handling

QEMU capture failures are not Pi 5 hardware blockers. If the script cannot
classify the run, the worker should keep hardwareTestLock untouched and triage
only QEMU/staging facts in this order:

1. Confirm the built kernel was compiled with
   TALOS_BOOT_SCENARIO=qemu_dup_syscall_smoke.
2. Confirm the retained log is fresh and belongs to the current candidate.
3. Confirm talos_dup still routes through dispatch_process_descriptor() and
   ProcessDescriptorStore::dup_current_descriptor().
4. Confirm current-owner lookup is present before dup, write, and close
   dispatch.
5. Confirm DescriptorTable capacity is the planned four-slot table so the
   second dup deterministically returns -EMFILE.
6. Confirm the descriptor-write, close, and scalar syscall regressions still
   classify.

If those facts do not distinguish the failure, the worker should record the
inconclusive evidence and stop for supervisor planning instead of broadening
the smoke.

## Recommended Next Task

The next bounded task should be
phase7-qemu-dup-syscall-smoke-core-20260529. It should implement only the
QEMU/substitute dup syscall smoke defined here, retain the required log, run
the listed regression gates, and keep Pi 5 physical proof, read behavior,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
and full POSIX descriptor readiness blocked.
