# Phase 7 EL0 Trap and Address-Space Contract

Status: accepted as the documentation-only Phase 7.2 EL0 trap-return and
user address-space contract. This document follows the accepted
[Phase 7 EL0 Address-Space Source Inventory](phase7-el0-address-space-source-inventory.md).
It does not add Rust behavior, assembly behavior, boot scenarios, QEMU runs,
Pi 5 hardware runs, archive publishing, hardware-lock use, EL0 entry,
SVC/syscall numeric ABI, VFS, filesystem, program loader, descriptor I/O,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The contract defines the first invariants that implementation tasks must obey
before Talos enters lower ELs or accepts user memory. It deliberately keeps the
first implementation slice target-independent: user range and permission
validation may be implemented and unit-tested before any translation-register,
exception-vector, or scheduler/process runtime behavior changes.

## Address-Space Invariants

The first userspace contract uses 48-bit virtual-address vocabulary and keeps
the user and kernel regions separate by policy, even though the accepted kernel
still runs on a broad EL2 identity map:

- The canonical user range is the low half below 0x0000_8000_0000_0000.
- The first null guard is 0x0000_0000_0000_0000..0x0000_0000_0001_0000.
  No user mapping may cover it.
- User code, data, heap, stack, and guard mappings must live inside the
  canonical user range and outside the null guard.
- Kernel text, data, stacks, heap, bootstrap tables, DTB data, UART, GIC-local,
  BCM2712 MMIO, and future RP1/PCIe windows are kernel-only. They may be
  mapped while a user task runs only with permissions that deny EL0 read,
  write, and execute access.
- MMIO is never user-accessible in this contract.
- Shared user/kernel mappings, shared memory objects, memory-mapped files,
  demand paging, copy-on-write, high-memory user frames, and DMA-visible user
  buffers remain deferred.

The first implementation may choose a smaller accepted user window inside the
canonical range for a built-in proof payload, but it must not choose a larger
or overlapping range without updating this contract.

## Mapping Vocabulary

Future address-space code should use these names consistently:

- UserText: user-readable and user-executable, not user-writable.
- UserData: user-readable and user-writable, never executable.
- UserHeap: user-readable and user-writable, never executable; growth policy
  remains deferred.
- UserStack: user-readable and user-writable, never executable; the stack
  grows down and must have an adjacent guard gap.
- UserGuard: deliberately unmapped or no-access space used to catch null,
  stack overflow, and layout mistakes.
- KernelMapping: supervisor-only code, data, stack, heap, device, or bootstrap
  mapping reachable by the kernel while handling traps.

The accepted early page-frame ownership vocabulary may describe future frame
sources, but it does not yet allocate process page tables or user frames.
Implementation tasks must keep bootstrap-reserved frames and translation-table
frames protected from user mappings.

## Lower-EL Trap and Return Invariants

Lower-EL entry and trap handling must preserve the existing scheduler
task/process split:

- A scheduler TaskId remains scheduler-local and is not a PID.
- A future process owns the address space and descriptor table. The scheduler
  may attach a process-owner placeholder, but it is not a process table.
- Scheduler mutation remains owner-local normal control flow. Lower-EL trap
  handlers may capture bounded trap state and classify the event, but they
  must not switch another CPU's current task or consume remote scheduler state.
- Initial user return state must include a validated user ELR, user stack
  pointer, SPSR/PSTATE value, and general-register frame.
- ELR must point at UserText; the user stack pointer must point inside a
  writable UserStack mapping and remain 16-byte aligned at public ABI
  boundaries.
- ERET to lower EL may happen only from an explicitly validated user frame.
  Current same-EL IRQ ERET evidence is not enough to construct that frame.
- Returning from a lower-EL trap must preserve the captured user frame until a
  later policy either resumes the task, reports a recoverable boundary error,
  or terminates the future process.

The first saved user-trap frame must be able to record x0 through x30, user
SP, ELR, SPSR, ESR, FAR, vector class, and the scheduler task/process-owner
identity available at the trap boundary. It may reuse the current
ExceptionFrame register ordering where practical, but the current fatal same-EL
diagnostic report is not itself the lower-EL frame contract.

## User Fault Classes

Talos classifies user faults separately from kernel faults:

- User instruction abort: executing unmapped, non-executable, or kernel-only
  memory.
- User data abort on read: reading unmapped, guard, kernel-only, or
  insufficient-permission memory.
- User data abort on write: writing unmapped, guard, read-only,
  executable-only, or kernel-only memory.
- User stack fault: using a stack pointer outside the accepted UserStack
  mapping, crossing a guard gap, or violating required alignment.
- Bad trap-return state: invalid user ELR, invalid user SP, non-canonical
  address, kernel address, or unsupported PSTATE.
- Unsupported lower-EL synchronous trap: a trap class that has no accepted
  recovery or syscall policy yet.

Until signals, demand paging, process exit, and wait semantics exist, a fault
caused by a running user task is fatal to that future process. It is not a
kernel panic unless kernel invariants are also corrupted. A fault observed
while validating a user pointer for a kernel service should map to
PosixError::Fault / EFAULT when the service has a POSIX-facing error boundary
and no side effect has been committed.

## Copy-In and Copy-Out Preconditions

The first copy-in/copy-out work must validate user ranges before byte-copy
helpers exist. A valid user memory request must satisfy all of these
preconditions:

- The start address is canonical, in the accepted user range, and outside the
  null guard.
- start + len does not wrap and does not exceed the accepted user range.
- The range is fully covered by user mappings for the requested access kind.
- Read operations require user-readable mappings.
- Write operations require user-writable mappings.
- Execute checks require UserText; copy helpers must not treat execute
  permission as read or write permission.
- The range does not touch UserGuard, unmapped space, kernel mappings, MMIO,
  bootstrap table pages, kernel stacks, or DTB data.
- The requested length is within a bounded kernel limit chosen by the
  implementation task.

Zero-length operations may be accepted only when the caller's later ABI
defines them as side-effect-free. They still must not be used to smuggle a
kernel pointer through an unchecked API.

Invalid ranges map to the accepted PosixError::Fault / EFAULT vocabulary where
a POSIX-facing boundary exists. Numeric errno values, syscall return
registers, restart conventions, partial-copy semantics, and per-thread errno
storage remain deferred.

## Evidence Levels

Target-independent user-memory helper evidence is sufficient when an
implementation changes only pure range, permission, or mapping metadata logic.
That evidence must include cargo fmt --all -- --check,
cargo -Zjson-target-spec test, git diff --check, and mdbook build if
documentation changes. The tests must cover null, wraparound, kernel-range,
guard-page, unmapped, read/write/execute mismatch, and length-limit cases.

QEMU lower-EL trap proof is required before Talos claims working lower-EL
entry or return. A QEMU proof must use an explicit boot scenario or script,
run a built-in payload, take a controlled trap back to the kernel, report the
captured user ELR/SPSR/FAR or equivalent saved state, and end with a named
PASS/classification line. QEMU proof does not imply Pi 5 hardware behavior.

Pi 5 lower-EL proof remains a later serialized hardware task. It must acquire
hardwareTestLock, record candidate commit and image/archive identity, capture
fresh serial and TFTP evidence, report PASS or a blocker classification, and
restore the pre-run boot state according to the lab-controller policy.

## Blocked Surfaces

This contract keeps these surfaces blocked until later explicit tasks:

- SVC/syscall numeric ABI, syscall dispatch table, return registers, and errno
  number mapping;
- VFS, filesystem, program loading, descriptor I/O, pipes, sockets, readiness,
  blocking I/O, shell behavior, and filesystem-backed commands;
- PID allocation, process tables, parent/child storage, exit status, wait,
  signals, credentials, sessions, and controlling TTY;
- TTBR/TCR/SCTLR implementation changes, actual EL0 entry, trap-return
  assembly, and process page-table switching;
- networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
  policy, and high-memory user-frame ownership.

## Next Implementation Boundary

The next bounded task is phase7-user-memory-permission-core-20260528. It may
implement only target-independent user virtual range, mapping permission,
access kind, and copy-boundary validation primitives matching this contract.
It must not copy bytes from live user memory, enter EL0, change translation
registers or vector assembly, add a syscall ABI, create a process table,
integrate descriptors, touch VFS/filesystems, or claim QEMU/Pi 5 behavior.

## Validation

- static inspection: git status --short was clean before documentation edits.
- documentation: this contract names address-space invariants, lower-EL
  trap/return invariants, user fault classes, copy-in/copy-out preconditions,
  evidence levels, blocked surfaces, and the next implementation boundary.
- whitespace inspection: git diff --check passed.
- documentation build: mdbook build passed.
- Rust fmt/tests, QEMU, and Pi 5 hardware runs were not required because this
  task changes only Markdown documentation and durable worker state.
