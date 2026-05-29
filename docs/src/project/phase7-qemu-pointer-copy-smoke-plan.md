# Phase 7 QEMU Pointer-Copy Smoke Plan

Status: accepted as the documentation-only Phase 7.3 QEMU pointer-copy smoke
plan after the accepted
[Phase 7 Pointer-Taking Syscall Contract](phase7-pointer-taking-syscall-contract.md).
It does not add Rust behavior, assembly behavior, boot scenarios, QEMU runs,
Pi 5 hardware runs, archive publishing, hardware-lock use, descriptor I/O,
runtime console or TTY integration, process loading, VFS, filesystem, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

This plan makes the next implementation task mechanical: add one
QEMU/substitute-only smoke that routes the proof-only talos_copy_probe syscall
through the accepted lower-AArch64 svc #0 frame path, invokes the accepted
copy_from_user() and copy_to_user() helpers with explicit substitute mappings
and backing storage, and retains deterministic success and EFAULT evidence.

## Smoke Invariant

The next implementation task must demonstrate one bounded invariant:

1. Talos builds a QEMU-only boot scenario named qemu_pointer_copy_smoke.
2. The kernel constructs a built-in lower-EL payload from fixed in-kernel bytes
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
3. The scenario initializes the first 16 bytes of UserData backing storage to
   0x2a and validates user ELR, user SP, SPSR/PSTATE, UserText, UserData,
   UserGuard, and UserStack before ERET.
4. The payload enters lower AArch64 and performs stable svc #0 with
   x8 = 0x7001, x0 = 0x0000_0000_0011_0000, x1 = 16, x2 = 0x2a, x3 = 0xa5,
   x4 = 0, and x5 = 0. Success returns x0 = 16 and the kernel-observed
   UserData backing storage contains 16 bytes of 0xa5 after copy_to_user().
5. The payload performs a second stable svc #0 with x8 = 0x7001,
   x0 = 0x0000_0000_001e_0000, x1 = 16, x2 = 0x2a, x3 = 0xa5, x4 = 0, and
   x5 = 0. The guard address must return x0 = 0xffff_ffff_ffff_fff2, the
   two's-complement encoding of -EFAULT.
6. The payload performs a stable svc #0 with an unaccepted syscall number
   x8 = 17 and observes x0 = 0xffff_ffff_ffff_ffda, the two's-complement
   encoding of -ENOSYS, preserving the scalar unknown-syscall invariant.
7. Only after those production svc #0 observations may the payload use the
   existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
   The marker must not dispatch as talos_copy_probe or any stable syscall.
8. The smoke prints final classification and PASS only after the success,
   EFAULT, unknown-syscall, and diagnostic-quarantine observations have been
   recorded.

The expected ESR for all production syscall traps is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional diagnostic
completion marker is 0x0000000054007a10. FAR_ELx is expected to be zero for
SVC paths unless QEMU reports a defined architectural value; any nonzero FAR
must be printed and justified in the implementation evidence.

## Required Output

The implementation script must retain the serial log and grep these exact
PASS/classification lines:

    qemu-pointer-copy-smoke: final participants=3 expected=3 errors=0 classification=qemu-pointer-copy-smoke-complete
    qemu-pointer-copy-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-pointer-copy-smoke: start
    qemu-pointer-copy-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true
    qemu-pointer-copy-smoke: syscall case=copy_probe_success vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x0000000000110000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000010
    qemu-pointer-copy-smoke: user-observed case=copy_probe_success x0=0x0000000000000010 data=0xa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5 ok=true
    qemu-pointer-copy-smoke: syscall case=copy_probe_efault vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x00000000001e0000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0xfffffffffffffff2 expected=-EFAULT
    qemu-pointer-copy-smoke: user-observed case=copy_probe_efault x0=0xfffffffffffffff2 ok=true
    qemu-pointer-copy-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
    qemu-pointer-copy-smoke: user-observed case=unknown x0=0xffffffffffffffda ok=true
    qemu-pointer-copy-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false

The implementation may print additional source-owner, mapping, scratch-buffer,
or saved-state fields, but these required lines must stay stable enough for
the script gate. If implementation work needs different user ELR, SP, UserData
address, byte values, length, or expected return values, it must stop for
supervisor planning instead of accepting a changed smoke.

## Source Owners

The next implementation task may touch only these source owners unless it
records a narrow reason:

- build.rs and src/main.rs for adding qemu_pointer_copy_smoke boot-scenario
  routing.
- src/arch/aarch64/vectors.S and src/arch/aarch64/exceptions.rs only if the
  accepted lower-AArch64 saved-frame path needs a focused extension to pass
  proof-owned user mapping/backing-storage context into dispatch.
- src/syscall.rs for adding the proof-only x8 = 0x7001 talos_copy_probe route
  in the QEMU smoke scenario, preserving x8 = 0x7001 as -ENOSYS elsewhere.
- src/posix.rs only if minor target-independent helper API adjustments are
  required by the accepted contract and remain covered by existing helper
  tests.
- src/target/qemu_virt.rs for QEMU harness orchestration, substitute UserData
  mapping/backing storage, lower-EL payload bytes, required serial output, and
  final classification.
- scripts/qemu-pointer-copy-smoke.sh for retained QEMU capture and grep gates.
- Existing scalar syscall and diagnostic EL0 trap smoke scripts only as
  regression gates, not as broadened behavior.
- Documentation and the task record needed to report evidence.

Existing descriptor-table, VFS/filesystem, runtime-console/TTY, process-loader,
Pi 5, RP1/PCIe, UART interrupt, and DMA/cache-driver owners remain out of
scope for the first implementation.

## Diagnostic And Proof-Only Quarantine

talos_copy_probe remains proof-only. The implementation must make the
qemu_pointer_copy_smoke scenario the only place where x8 = 0x7001 dispatches
to copy helpers. Outside that accepted scenario, x8 = 0x7001 must return
-ENOSYS like any other unaccepted syscall number.

SVC immediate 0x7a10 remains proof-owned diagnostic completion vocabulary. It
must never become a syscall number, ABI version, selector, compatibility mode,
copy-probe operation, or production success path. The final PASS line must
require the copy success, copy EFAULT, and unknown-syscall observations before
the diagnostic marker can count.

## Evidence Retention

The implementation task must retain:

- The QEMU serial log at
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log
  unless the accepted implementation task records an equally specific retained
  path.
- The command used to build and run qemu_pointer_copy_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-pointer-copy-smoke.sh.
- scripts/qemu-syscall-smoke.sh if shared syscall dispatch, lower-EL routing,
  frame mutation, or syscall-number handling changes.
- scripts/qemu-el0-trap-smoke.sh if vector routing, diagnostic marker handling,
  boot-scenario routing, lower-EL payload setup, or exception fallback changes.
- git diff --check.
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly state that no
Pi 5 hardware behavior, process-owned address space, descriptor I/O,
filesystem-backed data, path copying, program loading, shell behavior,
networking, or SSH support is claimed.

## Inconclusive QEMU Handling

QEMU capture failures are not Pi 5 hardware blockers. If the script cannot
classify the run, the worker should keep hardwareTestLock untouched and triage
only QEMU/staging facts in this order:

1. Confirm the built kernel was compiled with
   TALOS_BOOT_SCENARIO=qemu_pointer_copy_smoke.
2. Confirm scripts/qemu-pointer-copy-smoke.sh captured a fresh retained serial
   log path.
3. Confirm the log contains qemu-pointer-copy-smoke: start before looking for
   PASS.
4. Confirm the validated mapping line names the accepted UserData and
   UserGuard ranges.
5. Confirm the copy_probe_success line appears before the user-observed
   replacement data line.
6. Confirm the copy_probe_efault and unknown-syscall lines appear before the
   diagnostic marker completion line.
7. Compare the generated kernel path and timestamp against the build command.
8. Rerun the QEMU script once after cleaning only stale QEMU output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 pointer-copy hardware proof, archive publishing, power-cycle, serial
  observe, and hardware-lock acquisition.
- Descriptor read/write syscalls, TTY-backed stdio, process loading,
  process-owned address spaces, VFS/filesystem behavior, path copying,
  shell behavior, networking, and SSH.
- Stable public POSIX API status for talos_copy_probe or any descriptor-backed
  copy operation.
- Demand paging, copy-on-write, shared memory, user DMA buffers, mmap,
  lower-EL fault-table recovery, signals, restart semantics, and per-thread
  errno storage.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Mechanically Derivable Task

The next implementation task should be
phase7-qemu-pointer-copy-smoke-core-20260529. Its goal should be to implement
only the QEMU/substitute qemu_pointer_copy_smoke scenario, proof-only
talos_copy_probe dispatch, accepted copy-helper invocation, deterministic
success and EFAULT observations, unknown-syscall regression observation,
diagnostic-marker quarantine, script gate, and retained QEMU evidence defined
in this plan.

It must not add Pi 5 proof, descriptor I/O, runtime console or TTY behavior,
process loading, VFS/filesystem behavior, path copying, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this plan, linked it from SUMMARY, updated
  roadmap current status, updated the decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
