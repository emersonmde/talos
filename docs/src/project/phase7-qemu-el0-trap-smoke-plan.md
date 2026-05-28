# Phase 7 QEMU EL0 Trap Smoke Plan

Status: accepted as the documentation-only Phase 7.2 QEMU EL0 trap smoke
plan. This follows the accepted
[Phase 7 EL0 Trap and Address-Space Contract](phase7-el0-trap-address-space-contract.md)
and the target-independent user-memory permission core. It does not add Rust
behavior, assembly behavior, boot scenarios, QEMU runs, Pi 5 hardware runs,
archive publishing, hardware-lock use, a syscall ABI, VFS, filesystem, program
loader, descriptor I/O, networking, SSH, shell behavior, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

The plan exists to make the first lower-EL implementation task mechanical: a
future worker should be able to add exactly one QEMU-only built-in payload
proof without interpreting broader Phase 7 direction.

## Smoke Invariant

The first QEMU lower-EL proof must demonstrate one bounded invariant:

1. Talos builds a QEMU-only boot scenario named qemu_el0_trap_smoke.
2. The kernel constructs one built-in user payload from fixed in-kernel bytes.
3. The payload is mapped only in the accepted user range:
   - UserText:
     0x0000_0000_0010_0000..0x0000_0000_0010_1000, readable and executable,
     not writable.
   - UserStack:
     0x0000_0000_001f_0000..0x0000_0000_0020_0000, readable and writable,
     not executable.
   - UserGuard:
     0x0000_0000_001e_0000..0x0000_0000_001f_0000, unmapped or no access.
4. The implementation validates the user ELR, user SP, SPSR/PSTATE, and
   mappings through the accepted user-memory validation primitives before ERET.
5. The payload enters lower EL, executes only a diagnostic SVC marker
   0x7a10, and traps back to the kernel. This marker is not a syscall number
   and must not add a syscall dispatch table or return-value ABI.
6. The trap path records the saved user state, classifies the event as the
   planned diagnostic lower-EL trap, and halts or returns to the QEMU harness
   only after printing the required final lines.

The expected ESR for the marker is 0x0000000054007a10
(EC=0x15, AArch64 SVC, ISS=0x7a10). FAR_ELx is expected to be zero for
this SVC path unless QEMU reports a defined architectural value; any nonzero
FAR must be printed and justified in the task evidence.

## Required Output

The implementation script must retain the serial log and grep these exact
PASS/classification lines:

    qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete
    qemu-el0-trap-smoke: PASS

The serial log must also include one saved-state line with these exact field
names so evidence review can distinguish a real lower-EL trap from marker-only
output:

    qemu-el0-trap-smoke: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=<hex> elr=<hex> sp=<hex> spsr=<hex> marker=0x7a10

The implementation task may choose the concrete instruction address and stack
top inside the fixed ranges above, but the task evidence must state the chosen
values and explain why the printed ELR/SP are inside UserText/UserStack.

## Source Owners

The next implementation task may touch only these source owners unless it
records a narrow reason:

- build.rs and src/main.rs for boot-scenario routing.
- src/arch/aarch64/vectors.S and src/arch/aarch64/exceptions.rs for the
  lower-AArch64 synchronous vector path, saved user frame fields, and bounded
  trap classification.
- A small architecture or target module for the built-in EL0 payload bytes,
  validated user frame construction, and ERET handoff.
- src/memory_map/ only for the minimum QEMU translation-table or permission
  helper needed to represent the fixed UserText/UserStack/UserGuard mappings.
- src/target/qemu_virt.rs only for QEMU harness orchestration and final
  output.
- scripts/qemu-el0-trap-smoke.sh for the retained QEMU capture and grep gate.
- Documentation and the task record needed to report evidence.

Existing scheduler, descriptor-table, VFS/filesystem, runtime console, Pi 5,
RP1/PCIe, UART interrupt, and DMA/cache-driver owners remain out of scope for
the first implementation.

## Evidence Retention

The implementation task must retain:

- The QEMU serial log containing the saved-state, final classification, and
  PASS lines.
- The command used to build/run the scenario.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-el0-trap-smoke.sh.
- git diff --check.
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly say that no Pi 5
hardware behavior is claimed.

## Inconclusive QEMU Handling

QEMU capture failures are not Pi 5 hardware blockers. If the script cannot
classify the run, the worker should keep the hardware lock untouched and triage
only QEMU/staging facts in this order:

1. Confirm the built kernel was compiled with
   TALOS_BOOT_SCENARIO=qemu_el0_trap_smoke.
2. Confirm scripts/qemu-runner.sh captured a fresh serial log path.
3. Confirm the log contains the scenario start line before looking for PASS.
4. Compare the generated kernel path and timestamp against the build command.
5. Rerun the QEMU script once after cleaning only stale QEMU output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 lower-EL hardware proof, archive publishing, power-cycle, serial observe,
  and hardware-lock acquisition.
- General syscall ABI, syscall table, numeric errno return registers, restart
  conventions, and per-thread errno storage.
- Process table, scheduler process ownership beyond a diagnostic placeholder,
  process exit/wait, signals, and resumable user faults.
- VFS, filesystem, loader, ELF parsing, descriptor I/O, stdio TTY integration,
  shell behavior, networking, and SSH.
- Demand paging, copy-on-write, shared memory, user DMA buffers, and memory
  mapped files.

## Next Mechanically Derivable Task

The next implementation task should be
phase7-qemu-el0-trap-smoke-core-20260528. Its goal should be to implement the
QEMU-only boot scenario, built-in payload mapping, validated ERET handoff,
lower-EL synchronous trap capture, script gate, and retained QEMU evidence
defined in this plan. It must not add Pi 5 proof, a syscall ABI, process
loading, filesystem behavior, shell behavior, networking, or SSH.
