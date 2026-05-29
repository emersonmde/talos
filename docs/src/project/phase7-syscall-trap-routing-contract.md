# Phase 7 Syscall Trap-Routing Contract

Status: accepted as the documentation-only Phase 7.3 production syscall
trap-routing contract after the accepted
[Phase 7 Syscall Trap-Routing Source Inventory](phase7-syscall-trap-routing-source-inventory.md).
This contract does not add Rust behavior, assembly behavior, boot scenarios,
QEMU runs, Pi 5 hardware runs, archive publishing, hardware-lock use,
descriptor I/O, copy-in/copy-out, process loading, VFS, filesystem, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The purpose of this contract is to make the next implementation task
mechanical: route only valid lower-AArch64 svc #0 traps through the accepted
target-independent syscall dispatch core while preserving the diagnostic EL0
trap proofs as proof-owned surfaces.

## Routing Preconditions

A trap is eligible for production syscall routing only when all of these
conditions hold:

- The vector is ExceptionVector::LowerAarch64Sync.
- ESR_ELx.EC is 0x15, the AArch64 SVC exception class.
- ESR_ELx.ISS[15:0] is 0, the accepted stable SVC immediate.
- The saved frame is non-null and contains x0 through x30 in ExceptionFrame
  order.
- ELR and SPSR describe a returnable lower-AArch64 context. The first
  implementation may treat malformed or unavailable return state as a fatal
  process boundary rather than attempting a recoverable syscall return.

No same-EL exception, IRQ, FIQ, SError, AArch32 exception, lower-EL abort,
undefined instruction, BRK, or malformed frame is a production syscall in this
contract.

The routing implementation must decode ESR explicitly. It must not infer a
syscall from boot-scenario names, target-specific diagnostic handlers, output
strings, or the presence of an EL0 proof payload.

## Register Inputs

For an eligible trap:

- x8 is the raw syscall number.
- x0 through x5 are copied into SyscallArguments in register order.
- x6, x7, and x8 are caller-clobbered by this boundary and are not syscall
  arguments in the first contract.
- x9 through x15 have no syscall ABI meaning.
- x16, x17, x18, x19 through x29, x30, SP_EL0, ELR, and SPSR are preserved
  unless a later accepted process-fault, signal, restart, or loader contract
  changes that rule.

The first implementation may add a mutable saved-frame API or an equivalent
assembly handoff to support return mutation, but it must keep the stable
register interpretation in src/syscall.rs.

## Dispatch And Return Mutation

The production router calls the accepted target-independent syscall dispatch
core with:

- raw_number = saved x8;
- arguments = saved x0 through x5.

It then writes only the dispatch return value to saved x0 before returning to
lower EL. talos_nop returns 0. Unknown syscall numbers return -ENOSYS encoded
as a two's-complement u64 in x0.

The router must not mutate descriptor tables, scheduler queues, process
tables, page tables, VFS state, filesystem state, runtime console state, or
network state. It must not copy user memory or interpret raw register values
as user pointers.

## ELR And SPSR Rules

The first production syscall return path preserves the ELR value captured by
the exception frame. The accepted lower-EL proof evidence already reports the
saved ELR after SVC execution as the payload address plus four, so the router
must not blindly add another four bytes.

SPSR is preserved. The first implementation must not change the lower-EL mode,
interrupt mask, single-step state, condition flags, or user SP policy while
returning from a successful or unknown-syscall dispatch.

If implementation evidence shows that a chosen assembly boundary captures ELR
before architectural SVC advancement, the implementation task must stop and
update the contract or task state before accepting. It must not silently
introduce an ELR double-advance or no-advance ambiguity.

## Failure Classes

The implementation and QEMU smoke evidence must distinguish these classes:

- Not a syscall trap: vector or ESR exception class is outside the
  lower-AArch64 SVC precondition. Continue to the existing fatal exception
  path.
- Bad syscall trap: lower-AArch64 SVC reached the boundary but the SVC
  immediate is nonzero or the saved frame/return state is not safely
  mutable. Nonzero SVC immediate may return -EINVAL only when the router can
  safely mutate x0 and return to lower EL; otherwise it remains fatal.
- Unknown syscall: valid svc #0 frame with an x8 value outside the accepted
  namespace. Return -ENOSYS in x0.
- Accepted scalar syscall: valid svc #0 talos_nop. Return 0 in x0.

This contract does not define process-fatal recovery, signal delivery,
per-thread errno storage, restartable syscalls, partial-copy results,
blocking behavior, wait/exit behavior, or descriptor lifetime effects.

## Diagnostic Proof Quarantine

SVC immediate 0x7a10 remains diagnostic proof vocabulary only:

- qemu_el0_trap_smoke owns the QEMU diagnostic EL0 marker proof and its
  classification=qemu-el0-trap-smoke-complete output.
- rpi5_el0_trap_proof owns the Pi 5 diagnostic EL0 marker proof and its
  classification=pi5-el0-trap-proof-complete output.
- Production syscall routing must not treat 0x7a10 as a syscall number, ABI
  version, trap selector, compatibility mode, or success path.

The next implementation task may preserve the diagnostic handlers unchanged or
quarantine them behind proof-only cfg gates while adding a separate production
syscall smoke. If it touches vector routing, target proof payloads, boot
scenarios, diagnostic marker handling, or exception fallback behavior, it must
run the accepted diagnostic QEMU EL0 trap smoke or an explicitly accepted
replacement that proves the same quarantine.

## Required QEMU Syscall Smoke

Before Talos claims production syscall trap routing, a QEMU-only smoke must
exercise the runtime exception path with a lower-AArch64 payload that performs:

- svc #0 with x8 = 0, expecting x0 = 0 after return;
- svc #0 with an unknown x8 value, expecting x0 = -ENOSYS after return;
- a final deterministic PASS/classification line from kernel-controlled serial
  output.

The smoke must retain the serial log under tasks/evidence and name the exact
classification/PASS lines in the task record. The smoke must state that QEMU
evidence does not prove Pi 5 hardware syscall routing.

## Next Implementation Task

The next bounded implementation task should be
phase7-qemu-syscall-smoke-plan-20260529 before any runtime routing changes.
That plan must define the exact QEMU payload, expected return values, retained
log path, classification/PASS lines, diagnostic-proof preservation or
quarantine evidence, and local gates for the later core implementation.

Descriptor I/O, byte copy-in/copy-out, pointer-taking syscalls, process
loading, VFS, filesystem, shell, networking, SSH, Pi 5 syscall hardware proof,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
blocked.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this contract, linked it from SUMMARY,
  updated roadmap current status, updated the decision log, and added the
  task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
