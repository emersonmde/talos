# Phase 7 QEMU Syscall Smoke Plan

Status: accepted as the documentation-only Phase 7.3 QEMU syscall smoke
plan after the accepted
[Phase 7 Syscall Trap-Routing Contract](phase7-syscall-trap-routing-contract.md).
This plan does not add Rust behavior, assembly behavior, boot scenarios, QEMU
runs, Pi 5 hardware runs, archive publishing, hardware-lock use, descriptor
I/O, copy-in/copy-out, process loading, VFS, filesystem, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

The purpose of this plan is to make the next implementation task mechanical:
add one QEMU-only production syscall routing smoke that proves lower-AArch64
svc #0 reaches the accepted target-independent dispatch core and returns the
accepted x0 values.

## Smoke Invariant

The first QEMU production syscall smoke must demonstrate one bounded invariant:

1. Talos builds a QEMU-only boot scenario named qemu_syscall_smoke.
2. The kernel constructs one built-in lower-EL payload from fixed in-kernel
   bytes using the accepted QEMU EL0 trap smoke address-space shape:
   - UserText:
     0x0000_0000_0010_0000..0x0000_0000_0010_1000, readable and executable,
     not writable.
   - UserStack:
     0x0000_0000_001f_0000..0x0000_0000_0020_0000, readable and writable,
     not executable.
   - UserGuard:
     0x0000_0000_001e_0000..0x0000_0000_001f_0000, unmapped or no access.
3. The implementation validates user ELR, user SP, SPSR/PSTATE, and mappings
   through the accepted user-memory validation primitives before ERET.
4. The payload enters lower AArch64 and performs a stable production svc #0
   with x8 = 0, expecting x0 = 0 after return.
5. After observing that return in lower EL, the payload performs a second
   stable production svc #0 with x8 = 17, expecting x0 =
   0xffffffffffffffda, the two's-complement encoding of -ENOSYS.
6. After observing the unknown-syscall return in lower EL, the payload may use
   the existing diagnostic SVC marker 0x7a10 as a proof-only completion trap.
   That marker must be printed as non-production and not dispatched as a
   syscall.
7. The smoke prints final classification and PASS only after both production
   syscall return observations and the diagnostic-marker quarantine check have
   been recorded.

The expected ESR for the two production syscalls is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional completion
marker is 0x0000000054007a10. FAR_ELx is expected to be zero for these SVC
paths unless QEMU reports a defined architectural value; any nonzero FAR must
be printed and justified in the task evidence.

## Required Output

The implementation script must retain the serial log and grep these exact
PASS/classification lines:

    qemu-syscall-smoke: final participants=2 expected=2 errors=0 classification=qemu-syscall-smoke-complete
    qemu-syscall-smoke: PASS

The serial log must also include these exact field names so evidence review
can distinguish production syscall dispatch from diagnostic marker proof:

    qemu-syscall-smoke: start
    qemu-syscall-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 spsr=0x00000000000003c0 guard-blocked=true
    qemu-syscall-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000000
    qemu-syscall-smoke: user-observed case=talos_nop x0=0x0000000000000000 ok=true
    qemu-syscall-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
    qemu-syscall-smoke: user-observed case=unknown x0=0xffffffffffffffda ok=true
    qemu-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false

The implementation may print additional source-owner or saved-state fields,
but these required lines must remain stable enough for the script gate. If the
implementation chooses different user ELR or SP values inside the fixed
ranges, the implementation task must update this plan or stop for supervisor
planning before accepting.

## Source Owners

The next implementation task may touch only these source owners unless it
records a narrow reason:

- build.rs and src/main.rs for adding qemu_syscall_smoke boot-scenario routing.
- src/arch/aarch64/vectors.S and src/arch/aarch64/exceptions.rs for mutable
  lower-AArch64 synchronous saved-frame handling, ESR/SVC decoding, x8
  extraction, x0-through-x5 argument capture, x0 return mutation, and ERET.
- src/syscall.rs only for using the accepted stable constants and dispatch API,
  not for adding a broader namespace.
- A small architecture or target module for built-in EL0 payload bytes,
  validated user frame construction, and lower-EL return observation.
- src/target/qemu_virt.rs only for QEMU harness orchestration, scenario state,
  required serial output, and final classification.
- scripts/qemu-syscall-smoke.sh for the retained QEMU capture and grep gate.
- Documentation and the task record needed to report evidence.

Existing descriptor-table, VFS/filesystem, runtime console, Pi 5, RP1/PCIe,
UART interrupt, and DMA/cache-driver owners remain out of scope for the first
implementation.

## Diagnostic Proof Preservation

The implementation must preserve or explicitly quarantine the accepted
qemu_el0_trap_smoke diagnostic marker proof:

- If vector routing, target proof payloads, boot scenarios, diagnostic marker
  handling, or exception fallback behavior changes, the implementation must run
  scripts/qemu-el0-trap-smoke.sh or an accepted replacement that proves the
  qemu-el0-trap-smoke classification and PASS lines.
- SVC immediate 0x7a10 may be used only as proof-owned completion vocabulary in
  qemu_syscall_smoke. It must never be treated as a stable syscall immediate,
  syscall number, ABI version, compatibility mode, or production success path.
- The qemu_syscall_smoke final PASS must require the two production svc #0
  return observations before any diagnostic-marker completion can count.

## Evidence Retention

The implementation task must retain:

- The QEMU serial log containing the production syscall case lines,
  user-observed return lines, diagnostic-marker quarantine line, final
  classification, and PASS.
- The command used to build and run the scenario.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-el0-trap-smoke.sh unless the implementation does not touch any
  diagnostic proof, vector, payload, boot-scenario, marker, or exception
  fallback surface.
- scripts/qemu-syscall-smoke.sh.
- git diff --check.
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly say that no Pi 5
hardware behavior, descriptor I/O, copy-in/copy-out, process loading,
filesystem, shell, networking, or SSH support is claimed.

## Inconclusive QEMU Handling

QEMU capture failures are not Pi 5 hardware blockers. If the script cannot
classify the run, the worker should keep the hardware lock untouched and triage
only QEMU/staging facts in this order:

1. Confirm the built kernel was compiled with
   TALOS_BOOT_SCENARIO=qemu_syscall_smoke.
2. Confirm scripts/qemu-runner.sh or the smoke script captured a fresh serial
   log path.
3. Confirm the log contains qemu-syscall-smoke: start before looking for PASS.
4. Confirm both production syscall case lines appear before the diagnostic
   marker completion line.
5. Compare the generated kernel path and timestamp against the build command.
6. Rerun the QEMU script once after cleaning only stale QEMU output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 production syscall hardware proof, archive publishing, power-cycle,
  serial observe, and hardware-lock acquisition.
- Descriptor I/O, byte copy-in/copy-out, pointer-taking syscalls, process
  loader, process table, process exit/wait, signals, and resumable user faults.
- VFS, filesystem, ELF parsing, stdio TTY integration, shell behavior,
  networking, and SSH.
- Demand paging, copy-on-write, shared memory, user DMA buffers, and memory
  mapped files.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Mechanically Derivable Task

The next implementation task should be
phase7-qemu-syscall-smoke-core-20260529. Its goal should be to implement the
QEMU-only qemu_syscall_smoke boot scenario, production svc #0 routing into the
accepted target-independent dispatch core, user-observed talos_nop and unknown
return evidence, diagnostic-marker quarantine, script gate, and retained QEMU
evidence defined in this plan.

It must not add Pi 5 proof, descriptor I/O, byte copy-in/copy-out,
pointer-taking syscalls, process loading, filesystem behavior, shell behavior,
networking, or SSH.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this plan, linked it from SUMMARY, updated
  roadmap current status, updated the decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
