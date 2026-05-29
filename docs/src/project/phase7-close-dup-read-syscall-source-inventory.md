# Phase 7 Close, Dup, And Read Syscall Source Inventory

Status: accepted as the documentation-only Milestone 7.4 close/dup/read
syscall source inventory after the accepted
[Phase 7 Descriptor Close Core Closeout Checkpoint](phase7-descriptor-close-core-closeout-checkpoint.md).
This task adds no Rust behavior, assembly behavior, close/dup/read syscall
contract, close/dup/read implementation, QEMU run, Pi 5 hardware run, boot
archive publication, hardware-lock acquisition, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

This inventory maps the current source owners, accepted evidence, and gaps for
moving from process-owned descriptor-table and descriptor-close primitives to
the first user-visible close/dup/read syscall contract. It separates proven
descriptor-write and process-descriptor capabilities from unproven syscall
behavior for close, dup, and read.

## Source Owners

### Syscall Dispatch And Return Encoding

- `src/syscall.rs` owns the stable lower-EL syscall vocabulary:
  `STABLE_SVC_IMMEDIATE = 0`, syscall number in x8, arguments in x0 through
  x5, and return/-errno encoding in x0.
- `TALOS_NOP_SYSCALL = 0` and `TALOS_WRITE_SYSCALL = 1` are the only stable
  syscall numbers with accepted production behavior today. The proof-only
  `TALOS_COPY_PROBE_SYSCALL = 0x7001` remains compiled only for tests and the
  pointer-copy proof scenarios.
- `SyscallReturn::error()` maps accepted POSIX errors to negative errno values.
  The currently accepted errno subset includes EBADF, EFAULT, EINVAL, EIO,
  ENOSYS, and ENOTSUP through `errno_number()`.
- `dispatch_descriptor_write()` owns the target-independent talos_write helper
  dispatch when a caller supplies a descriptor table, user mappings, user
  memory, scratch buffer, and console backend.

Gap: no stable syscall number, `SyscallNumber` variant, argument contract, or
dispatch helper exists for close, dup, or read. The scalar `dispatch()` still
returns -ENOTSUP for talos_write without descriptor context and -ENOSYS for
every unallocated syscall number.

### Lower-EL Trap Routing

- `src/arch/aarch64/exceptions.rs::try_route_lower_aarch64_syscall()` owns the
  production lower-AArch64 SVC #0 filter, x8 extraction, x0-through-x5
  argument capture, x0 return mutation, and frame preservation.
- Existing proof scenarios route descriptor-write behavior through
  target-specific handlers after lower-EL trap recognition.

Gap: the generic production trap route still calls scalar `dispatch()` rather
than resolving a live process descriptor store. Close, dup, and read need an
explicit target-independent dispatch boundary before any QEMU or Pi 5 runtime
claim can be made.

### Process Descriptor Store And Table Operations

- `src/posix.rs::ProcessDescriptorStore` owns process-owner lookup for one
  bounded `ProcessOwnerId` to `DescriptorTable` mapping.
- `current_descriptor_table()` and `current_descriptor_table_mut()` map missing
  current owner, unknown owner, or missing table state to `PosixError::BadDescriptor`.
- `close_current_descriptor()` mutably resolves the current owner and applies
  `DescriptorTable::close()` to the current process table.
- `DescriptorTable` owns lookup, lowest-free allocation, exact-slot
  allocation, table-local close, and table-local dup.

Accepted evidence: the process descriptor table core and QEMU process
descriptor stdio smoke prove inherited fd 0/fd 1/fd 2 setup, current-owner
lookup, and talos_write fd 1/fd 2 through a `ProcessOwnerId`-backed table at
the QEMU/substitute evidence level. The descriptor close core proves only the
target-independent helper `ProcessDescriptorStore::close_current_descriptor()`
with focused unit tests.

Gap: no lower-EL close or dup syscall dispatch uses these mutable table
operations. No QEMU/substitute or Pi 5 evidence proves a user-visible close or
dup syscall, and no process teardown or object-finalizer semantics are
accepted.

### Descriptor Entries And Object Vocabulary

- `DescriptorEntry` owns access mode, flags, and object identity.
- `require_readable()` maps non-readable descriptors to EBADF, and
  `require_writable()` maps non-writable descriptors to EBADF.
- `DescriptorObjectKind::StdioInput` reserves fd 0 vocabulary, while
  `StdioOutput` backs fd 1/fd 2 descriptor writes to runtime-console0.
- `RegularFile`, `Directory`, `PipeEndpoint`, `Socket`, `Device`, and
  `OtherKernelObject` remain reserved tags only.

Gap: read needs a real source of bytes and a contract for stdin/TTY behavior.
Today there is no descriptor-facing TTY input object, read queue, EOF,
blocking/readiness state, nonblocking policy, signal interruption, restart
policy, or file/socket/pipe/device object registry.

### Copy Helpers And User Memory

- `copy_from_user()` owns all-or-nothing copy-in after full range validation.
- `copy_to_user()` owns all-or-nothing copy-out after full range validation.
- `UserMapping`, `UserMappingPermissions`, `UserAccessKind`,
  `USER_NULL_GUARD_END`, `USER_ADDRESS_SPACE_END`, and
  `DEFAULT_USER_COPY_LIMIT` own the target-independent user-memory checks.

Accepted evidence: pointer-copy proofs accept both copy directions under
explicit substitute mappings, and descriptor-write evidence accepts
copy_from_user feeding runtime-console0 writes.

Gap: read would require copy_to_user from a descriptor-backed byte source and
still lacks live process address-space lookup, page-table-backed user mapping
lookup, recoverable lower-EL data-abort policy, partial-copy behavior,
restart behavior, and per-thread errno storage.

### Runtime Console, TTY, And Stdin

- `src/runtime_console.rs` owns runtime-console0 output/input facade
  vocabulary.
- `src/tty.rs` owns line-discipline shaping and diagnostic-channel input
  vocabulary.
- `src/diagnostic_command.rs` consumes the diagnostic command stream and is
  not a POSIX descriptor syscall path.

Gap: stdout/stderr descriptor writes are accepted through runtime-console0, but
stdin/read is not. Talos has no contract tying fd 0 to TTY input bytes,
blocking semantics, readiness, EOF, control events, process groups, or
controlling-terminal behavior.

## Close, Dup, And Read Matrix

| Operation | Current source owner | Accepted evidence | Missing before syscall claim |
| --- | --- | --- | --- |
| close | `ProcessDescriptorStore::close_current_descriptor()` plus `DescriptorTable::close()` | Unit-test evidence in `tasks/2026-05-29-phase7-descriptor-close-core.md`; closeout in `docs/src/project/phase7-descriptor-close-core-closeout-checkpoint.md` | syscall number, argument decoding, lower-EL dispatch helper, return contract, QEMU close smoke, Pi 5 proof, object finalization |
| dup | `DescriptorTable::dup()` | Descriptor-table core unit tests for duplicate object identity and separate slot lifetime; close-core tests preserve duplicate after closing the original | process-owned dup syscall contract, return-fd convention, EMFILE behavior through syscall return, close-on-exec/flags policy, dup2/fcntl boundaries, QEMU/Pi 5 evidence |
| read | `DescriptorEntry::require_readable()`, `copy_to_user()`, `DescriptorObjectKind::StdioInput` vocabulary | Copy-out helper evidence from pointer-copy proofs; fd 0 inherited as read-only stdio input in descriptor-table/process-table tests | stdin/TTY byte source, read syscall number and arguments, copy_to_user integration, EOF/blocking/readiness/nonblocking/signal/restart policy, QEMU/Pi 5 evidence |

## Proven And Unproven Boundaries

Proven at the current frontier:

- lower-AArch64 stable SVC #0 routing, talos_nop, unknown-syscall -ENOSYS, and
  diagnostic marker quarantine;
- talos_write fd 1/fd 2 through proof-owned and process-owned inherited
  runtime-console0 stdio descriptors;
- copy_from_user() and copy_to_user() under explicit substitute mappings;
- ProcessDescriptorStore current-owner lookup for inherited stdio tables;
- target-independent table-local close, dup, reuse, and duplicate-lifetime
  behavior under unit tests.

Not proven:

- close, dup, or read as stable lower-EL syscalls;
- mutable process descriptor table dispatch through the production syscall
  path;
- read/stdin data delivery, EOF, blocking, readiness, signal, or restart
  behavior;
- open-file-description reference counts, object finalizers, process exit
  teardown, or close-on-exec enforcement;
- QEMU/substitute or Pi 5 physical evidence for close, dup, or read syscalls;
- process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART
  interrupt ownership, DMA/cache-driver policy, or full POSIX descriptor
  readiness.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
`phase7-close-syscall-contract-20260529`, documentation-only.

That contract should define the first close syscall only: stable syscall
number, x0 descriptor argument, unused argument requirements, success return,
EBADF cases, duplicate-descriptor interaction, and the
`ProcessDescriptorStore::close_current_descriptor()` ownership rule. It should
preserve dup, read, object finalization, QEMU/Pi 5 proof, process loading,
VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness as blocked.

## Validation

- static inspection: `git status --short` before documentation edits was
  clean.
- static source review: inspected `src/posix.rs`, `src/syscall.rs`,
  `src/arch/aarch64/exceptions.rs`, accepted descriptor syscall docs, process
  descriptor table docs, descriptor lifetime/close docs, close-core task
  record, retained QEMU process-descriptor stdio task record, roadmap, and
  decision log.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
