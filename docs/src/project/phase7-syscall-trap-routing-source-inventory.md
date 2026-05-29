# Phase 7 Syscall Trap-Routing Source Inventory

Status: accepted as the documentation-only Phase 7.3 production syscall
trap-routing source inventory after the accepted target-independent syscall
dispatch core. This document follows the accepted
[Phase 7 Syscall ABI Contract](phase7-syscall-abi-contract.md). It does not add
Rust behavior, assembly behavior, boot scenarios, QEMU runs, Pi 5 hardware
runs, archive publishing, hardware-lock use, descriptor I/O, copy-in/copy-out,
process loading, VFS, filesystem, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

This inventory maps the source owners and missing contracts for connecting
lower-AArch64 SVC traps to the accepted target-independent dispatch core
without turning the diagnostic EL0 trap proofs into a stable syscall ABI.

## Source Owners

### Lower-AArch64 SVC Detection

- src/arch/aarch64/vectors.S owns the vector slots. It assigns vector number
  8 to lower-AArch64 synchronous exceptions, saves x0 through x30 in
  ExceptionFrame order, reads ESR/ELR/FAR/SPSR for the active exception level,
  and calls rust_exception_handler for synchronous exceptions.
- src/arch/aarch64/exceptions.rs owns ExceptionVector.
  ExceptionVector::LowerAarch64Sync is the only vector accepted by the syscall
  ABI contract for production syscalls.
- ESR_ELx.EC = 0x15 identifies an AArch64 SVC exception. The first production
  routing contract should add a narrow decoder for EC and ISS instead of
  relying on proof-scenario string checks.

Gap: the default synchronous exception path is still fatal and returns !.
There is no production classifier that can choose between recoverable
lower-AArch64 svc #0 and the existing fatal exception reporter.

### SVC Immediate Validation

- src/syscall.rs owns STABLE_SVC_IMMEDIATE = 0 and
  DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE = 0x7a10.
- src/target/qemu_virt.rs and src/target/rpi5.rs own proof-only handlers that
  accept ESR ISS 0x7a10 and print diagnostic PASS/classification lines.
- The accepted ABI contract says svc #0 is the stable production boundary.
  Nonzero immediates are bad syscall traps; 0x7a10 must remain diagnostic
  vocabulary only.

Gap: no production source currently decodes ESR ISS, rejects nonzero SVC
immediates as a syscall-boundary error, or proves that 0x7a10 cannot fall into
the stable dispatch path.

### Syscall Number And Argument Capture

- src/arch/aarch64/exceptions.rs::ExceptionFrame::reg() exposes saved general
  registers by index. The accepted ABI maps x8 to the syscall number and x0
  through x5 to scalar arguments.
- src/syscall.rs::SyscallArguments owns the target-independent six-argument
  view, and syscall::dispatch() owns talos_nop success and unknown-syscall
  -ENOSYS behavior.

Gap: no routing code yet extracts x8 from ExceptionFrame::reg(8) or captures
reg(0)..reg(5) into SyscallArguments. The next contract should state exact
null-frame handling before dispatch; returning -EINVAL is only safe when the
frame can be mutated and returned to user mode.

### Return Mutation And ELR Handling

- vectors.S has a restore-and-eret path for IRQ vectors, but synchronous
  exception handling currently calls rust_exception_handler and halts after
  the Rust handler. There is no recoverable synchronous return path.
- ExceptionFrame stores saved x0 through x30, but the field is private and
  only immutable reg() access is exposed.
- QEMU retained EL0 diagnostic evidence reports the saved ELR as the payload
  address plus four after SVC execution. The syscall ABI contract preserves ELR
  unless a later process-fault policy changes it.

Gap: production syscall routing needs a mutable saved-frame API or equivalent
assembly boundary that can write the SyscallReturn x0 value, preserve the
contracted registers, and return through eret. The next contract should forbid
blind ELR double-advance and require evidence that the chosen return PC matches
the saved lower-EL SVC semantics.

### Non-Syscall Fallback

- rust_exception_handler still owns the fatal default path for same-EL
  synchronous exceptions, lower-EL non-SVC exceptions, IRQ/FIQ/SError, AArch32
  lower-EL exceptions, malformed frames, and any future process-fatal boundary.
- src/target/qemu_virt.rs::handle_el0_trap_smoke_exception and
  src/target/rpi5.rs::handle_el0_trap_proof_exception are cfg-gated proof
  handlers. They should stay proof-owned unless the next contract explicitly
  replaces them with a production syscall smoke.

Gap: the production router must keep non-syscall traps on the existing fatal
exception path and must not claim process-fatal recovery, signal delivery,
restart, per-thread errno, or user fault policy.

### Process And Scheduler Context

- src/scheduler.rs owns TaskId, ProcessOwnerId, task owner metadata, and the
  production scheduler runtime. Existing Phase 6 rules keep scheduler mutation
  in owner-local normal control flow, not arbitrary exception context.
- No source file owns PID allocation, process table lookup, process-owned page
  tables, descriptor-table lifetime binding, current-working-directory state,
  or credentials.

Gap: the first production routing slice can treat task/process identity as
optional diagnostic context only. It should not require descriptor lifetime,
process loading, wait/exit, signals, blocking I/O, or scheduler mutation from
the exception path.

## Recommended Next Contract

The next bounded task should be
phase7-syscall-trap-routing-contract-20260529.

That contract should define:

- an ESR decoder for AArch64 SVC EC and 16-bit ISS;
- the exact lower-AArch64 sync and svc #0 acceptance rule;
- the bad-trap behavior for nonzero SVC immediates, including quarantining
  diagnostic 0x7a10 outside the stable ABI;
- x8 extraction and x0-through-x5 scalar argument capture from ExceptionFrame;
- the x0 mutation boundary for SyscallReturn;
- preserved-register and ELR/SPSR rules for return-to-user;
- the non-syscall fallback to the existing fatal exception path;
- mandatory QEMU syscall smoke evidence before claiming production routing.

The smallest later implementation slice should prove svc #0 talos_nop and an
unknown syscall number through production routing in QEMU only. Descriptor I/O,
copy-in/copy-out, process loading, VFS, filesystem, shell, networking, SSH, and
Pi 5 syscall hardware proof remain blocked.

## Validation

- static inspection: git status --short before edits was clean.
- static source review: inspected vector entry and saved-frame ownership,
  Rust exception routing, QEMU and Pi 5 diagnostic EL0 trap proof handlers,
  accepted syscall dispatch vocabulary, scheduler task/process-owner metadata,
  roadmap, decision log, and task records.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
