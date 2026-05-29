# Phase 7 Syscall ABI Contract

Status: accepted as the documentation-only Phase 7.3 syscall ABI contract
before any syscall implementation. This document follows the accepted
[Phase 7 Syscall ABI Source Inventory](phase7-syscall-abi-source-inventory.md).
It does not add Rust behavior, assembly behavior, boot scenarios, QEMU runs,
Pi 5 hardware runs, archive publishing, hardware-lock use, process loading,
descriptor I/O, VFS, filesystem, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

The contract defines the first stable SVC boundary narrowly enough that a
later implementation task can build and test dispatch without deciding full
POSIX semantics.

## Trap Entry Contract

The first production syscall path is a lower-AArch64 synchronous exception
with ESR_ELx.EC = 0x15, AArch64 SVC:

- The vector must be lower-aarch64-sync. Same-EL SVC, IRQ, FIQ, SError,
  AArch32, and abort vectors are not syscalls in this contract.
- The stable SVC immediate is 0. User code should issue svc #0 for syscalls.
- SVC immediate 0x7a10 remains diagnostic proof vocabulary only. It is not a
  syscall number, ABI version, trap class, or user-program interface.
- A lower-AArch64 SVC with a nonzero immediate is a bad syscall trap. The
  first dispatcher may report it as -EINVAL or terminate the future process if
  the trap frame is otherwise inconsistent. It must not route 0x7a10 to a
  production syscall.
- The handler must capture ESR, FAR, ELR, SPSR, vector, x0 through x30, the
  user SP, current scheduler TaskId when available, and attached
  ProcessOwnerId when available before interpreting arguments.

The syscall path may reuse ExceptionFrame register ordering internally, but the
syscall module owns the stable interpretation of saved registers and error
returns. The default fatal exception reporter is not a recoverable syscall
dispatcher.

## Register ABI

The first Talos syscall register convention is:

- x8 carries the syscall number.
- x0 through x5 carry up to six scalar arguments.
- x0 carries the return value after dispatch.
- x1 through x7 and x8 are caller-clobbered across the syscall boundary.
- x9 through x15 are caller-clobbered by normal AArch64 procedure-call
  convention and have no syscall ABI meaning in this contract.
- x16, x17, x18, x19 through x29, x30, SP_EL0, ELR, and SPSR are preserved
  unless a later process-fault or signal policy explicitly changes them.
- 64-bit unsigned register values are the transport type. Individual syscalls
  may later interpret them as signed integers, descriptor indexes, addresses,
  lengths, flags, or handles.

The first implementation must not infer pointer validity from a raw register
value. Pointer-taking syscalls require the accepted user-memory validation and
a later byte-copy helper or an explicit scalar-only proof exemption.

## Syscall Number Namespace

The first accepted syscall namespace contains only one callable success case:

| Number | Name | Arguments | Result |
| --- | --- | --- | --- |
| 0 | talos_nop | none | returns 0 |

All other syscall numbers are unknown and must return -ENOSYS when the trap
frame itself is valid. Unknown-syscall handling is part of the first proof
slice because it exercises deterministic dispatch failure without requiring
process loading, pointer arguments, descriptors, VFS, filesystem, shell,
networking, or SSH.

The namespace is intentionally not Linux-compatible yet. Later compatibility
work may map libc wrappers onto Talos numbers, but that requires a separate
ABI review before userland ports depend on it.

## Return And Error Convention

The first syscall return convention is one register:

- A nonnegative x0 value is success.
- A negative x0 value is -errno encoded in two's-complement 64-bit form.
- x1 is not a secondary return register.
- There is no per-thread errno storage, restart convention, signal delivery,
  partial-copy result, or interrupted-syscall policy in this contract.

The first numeric errno subset is:

| PosixError | errno | Syscall use |
| --- | ---: | --- |
| InvalidArgument / EINVAL | 22 | malformed trap, unsupported nonzero SVC immediate, invalid scalar flags |
| BadDescriptor / EBADF | 9 | future descriptor syscall receives an invalid descriptor |
| Fault / EFAULT | 14 | future pointer validation or copy boundary rejects user memory |
| NotImplemented / ENOSYS | 38 | unknown syscall number or accepted namespace hole |
| NotSupported / ENOTSUP | 95 | syscall exists but the object kind or mode is unsupported |

The first implementation task needs only EINVAL and ENOSYS unless it chooses
to thread the accepted error encoder through existing target-independent
PosixError values. It must not claim complete errno coverage.

## Failure Classes

The dispatcher must distinguish these classes in evidence and tests:

- Not a syscall trap: vector, exception class, or source EL is outside the
  lower-AArch64 SVC contract. This remains an exception/fault path, not a
  syscall return.
- Bad syscall trap: lower-AArch64 SVC reached the syscall boundary but the SVC
  immediate, saved frame, ELR, SP, SPSR, or available task context violates the
  contract. The first implementation may return -EINVAL only when doing so can
  preserve the user frame; otherwise it must classify a process-fatal boundary.
- Unknown syscall: x8 names no accepted syscall. Return -ENOSYS.
- Accepted scalar syscall: talos_nop with no pointer or descriptor side
  effects. Return 0.
- Deferred syscall family: descriptor I/O, process control, file paths,
  buffers, blocking waits, readiness, signals, and filesystem operations remain
  unavailable and must not be represented by stub success.

## First Proof Slice

The next implementation task should be phase7-syscall-dispatch-core-20260529.
It may implement only target-independent syscall vocabulary and deterministic
dispatch/error conversion:

- a syscall-number enum or equivalent constant owner for talos_nop = 0;
- a syscall result/error encoder that returns 0 or negative errno in x0 form;
- a dispatch function that accepts a saved scalar register view and returns
  talos_nop success or unknown-syscall -ENOSYS;
- unit tests for valid talos_nop, unknown syscall, argument preservation rules
  relevant to the pure dispatch layer, and negative errno encoding;
- optional static helpers that decode ESR EC/ISS for SVC, but only if they do
  not change runtime exception behavior.

The first proof slice must not add a boot scenario, enter EL0, modify
vectors.S, route rust_exception_handler into production syscall dispatch,
copy bytes from user memory, mutate descriptor tables through syscall entry,
load programs, expose VFS/filesystem behavior, create a shell, touch
networking/SSH, or acquire hardwareTestLock.

## Validation Gates For The Implementation Task

The following implementation task must run:

- git status --short before edits.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- git diff --check.
- mdbook build if docs/src is touched.

It must additionally run scripts/qemu-el0-trap-smoke.sh only if it touches
lower-EL trap runtime behavior, boot-scenario routing, target proof payloads,
vectors.S, arch exception routing, or diagnostic SVC marker handling. A Pi 5
hardware run is not required for a target-independent dispatch core and must
not be performed unless a later task explicitly accepts a hardware proof
boundary and acquires hardwareTestLock.

## Deferred Surfaces

This contract keeps these surfaces blocked until later explicit tasks:

- production exception-handler integration, QEMU syscall smoke, and Pi 5
  syscall hardware proof;
- pointer-taking syscalls, byte copy-in/copy-out, partial copies, restart,
  signals, process-fatal fault policy, and per-thread errno storage;
- descriptor read/write/close/dup through syscall entry, TTY-backed stdio,
  blocking I/O, readiness, wait queues, pipes, sockets, and device objects;
- process loading, ELF, argv/envp setup, PID allocation, exit/wait,
  credentials, sessions, controlling TTY, VFS, filesystem, shell, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Validation

- static inspection: git status --short before edits was clean.
- static source review: reconciled the accepted lower-EL trap proof, syscall
  ABI source inventory, exception frame/vector code, PosixError vocabulary,
  user-memory validation, descriptor-table model, scheduler task/process-owner
  placeholders, roadmap, and decision log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs were not required
  because this task changes only Markdown documentation and durable worker
  state.
