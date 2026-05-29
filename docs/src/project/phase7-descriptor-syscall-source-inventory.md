# Phase 7 Descriptor Syscall Source Inventory

Status: accepted as the documentation-only Phase 7.3 descriptor syscall source
inventory after the accepted
[Phase 7 Pi 5 Pointer-Copy Proof Closeout Checkpoint](phase7-pi5-pointer-copy-proof-closeout-checkpoint.md).
This task does not add Rust behavior, assembly behavior, descriptor syscall
contracts, QEMU runs, Pi 5 hardware runs, boot archives, hardware-lock use,
process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

This inventory maps the source owners and gaps for the first descriptor syscall
contract after Talos has accepted syscall routing, copy helpers, and the
proof-only pointer-copy boundary. It separates the temporary talos_copy_probe
proof surface from future stable descriptor syscalls.

## Source Owners

### Descriptor Table And Descriptor Objects

- src/posix.rs::DescriptorTable owns the target-independent descriptor table
  data model. It currently supports empty and inherited-stdio construction,
  lowest-free allocation, exact-slot allocation, lookup, close, and dup.
- src/posix.rs::DescriptorEntry owns per-descriptor access mode, descriptor
  flags, and object identity. require_readable(), require_writable(), and
  require_tty() already map access and kind failures to EBADF or ENOTTY.
- DescriptorObjectKind::StdioInput and DescriptorObjectKind::StdioOutput
  reserve stdin, stdout, and stderr object vocabulary, while RegularFile,
  Directory, PipeEndpoint, Socket, Device, and OtherKernelObject remain tags
  for later contracts.
- STDIN_FD, STDOUT_FD, and STDERR_FD fix descriptor numbers 0, 1, and 2 for
  inherited stdio table setup.

Gap: the descriptor table is not attached to a live process, current task,
syscall frame, runtime-console object, TTY object, VFS object, pipe, socket, or
device registry. It also has no open-file-description reference counts,
blocking/readiness state, descriptor syscall number allocation, signed file
descriptor decoding, dup2/fcntl policy, close-on-exec behavior, or stable
userspace ABI.

### Syscall Numbering, Argument Capture, And Return Encoding

- src/syscall.rs owns STABLE_SVC_IMMEDIATE = 0, x8 syscall-number vocabulary,
  x0-through-x5 scalar arguments through SyscallArguments, negative x0 errno
  returns through SyscallReturn, and the accepted errno subset currently
  including EINVAL, EBADF, EFAULT, ENOSYS, and ENOTSUP.
- src/syscall.rs::dispatch() currently accepts only talos_nop = 0 in the
  stable target-independent dispatch path. Unknown syscall numbers return
  -ENOSYS.
- src/arch/aarch64/exceptions.rs::try_route_lower_aarch64_syscall() owns the
  production lower-AArch64 svc #0 detection, x8 extraction, x0-through-x5
  capture, x0 return mutation, and preservation of the rest of the saved frame.
- The proof-only TALOS_COPY_PROBE_SYSCALL = 0x7001 and dispatch_copy_probe()
  are compiled only for tests and the accepted pointer-copy proof scenarios.
  They are not descriptor syscall precedent.

Gap: no stable descriptor syscall number is allocated for read, write, close,
dup, or any stdio operation. The next contract must choose one descriptor slice,
define exact register roles, choose the stable syscall number, and state how
usize lengths, negative or out-of-range descriptor numbers, and kernel-side
malformed inputs map to x0 returns.

### Copy Helpers And User Buffer Provenance

- src/posix.rs::copy_from_user() and copy_to_user() own the accepted
  target-independent byte-copy helper behavior. They validate the complete
  user range before side effects, use UserAccessKind::Read for copy-in and
  UserAccessKind::Write for copy-out, return the requested length on success,
  and map null guard, kernel range, wraparound, mapping, permission, and
  backing-storage failures to EFAULT.
- UserMapping, UserMappingPermissions, USER_ADDRESS_SPACE_END,
  USER_NULL_GUARD_END, and DEFAULT_USER_COPY_LIMIT own the current
  user-memory vocabulary and validation limits.
- The accepted pointer-copy QEMU and Pi 5 proofs pass explicit substitute
  mappings and backing storage into these helpers. That is proof evidence, not
  process-owned address-space support.

Gap: descriptor syscalls still lack a current process address-space owner,
page-table-backed user mapping lookup, pinned user buffers, path/string
copying policy, partial-write restart policy, per-thread errno storage, and
recoverable lower-EL data-abort/fault-table policy. A first descriptor write
contract can reuse explicit QEMU substitute mappings later, but it must not
claim live process memory ownership until a separate task accepts it.

### Runtime Console, TTY, And Stdio Backing

- src/runtime_console.rs::RuntimeConsole owns the internal output facade for
  runtime-console0. write_default_console_output() returns a structured
  ConsoleWriteOutcome with complete byte counts or backend failure status.
- runtime_console::poll_default_console_input() and ConsoleInputPollOutcome
  define the polling input facade shape, including byte available, no data,
  backend unavailable, and backend error outcomes.
- src/tty.rs::TtyLineDiscipline owns the accepted canonical-lite/raw TTY input
  shaping, echo buffer, control-event vocabulary, and PollingTtyRxOutcome
  names.
- src/diagnostic_command.rs uses the TTY and runtime-console surfaces for
  diagnostic command flow. Those calls are kernel diagnostic behavior, not
  descriptor syscalls.
- src/target/qemu.rs, src/target/qemu_virt.rs, and src/target/rpi5.rs own
  target-specific console backends. Descriptor syscalls should not call those
  target backends directly; they should route through a descriptor-owned
  kernel object boundary once contracted.

Gap: stdout/stderr descriptors are not yet bound to a runtime-console object,
stdin is not bound to a TTY input object, and there is no descriptor-facing
readiness, blocking, EOF, short read/write, nonblocking flag, wait queue,
signal interruption, or scheduler sleep/wakeup contract. Read is therefore a
larger slice than a bounded stdout/stderr write contract.

### Process, Task, And Ownership Boundaries

- src/scheduler.rs::TaskId, Task, and ProcessOwnerId own the accepted
  scheduler task and placeholder process-owner vocabulary.
- Task::attach_process_owner(), Task::process_owner(), and
  ProductionSchedulerRuntime show where later process ownership can attach to
  scheduler state.
- The accepted production scheduler rules keep owner-local scheduler mutation
  in normal control flow, not arbitrary IRQ/IPI or exception context.

Gap: there is no PID allocator, process table, current-process lookup from the
syscall handler, descriptor-table lifetime owner attached to a process, fork,
spawn, exec, exit/wait, credential model, controlling TTY, or process-owned
address-space handle. The first descriptor syscall contract must either use a
focused QEMU substitute descriptor table/backing object or explicitly wait for
process-owned descriptor state.

### Evidence And Script Ownership

- scripts/qemu-syscall-smoke.sh, scripts/qemu-pointer-copy-smoke.sh, and the
  retained logs under tasks/evidence/2026-05-29-* show the existing
  QEMU/substitute evidence style for lower-EL syscall routing and copy helper
  behavior.
- docs/src/project/phase7-descriptor-table-contract.md and
  tasks/2026-05-28-phase7-descriptor-table-core.md record the accepted
  descriptor-table data-model contract and core implementation evidence.
- docs/src/project/phase7-copyin-copyout-helper-contract.md and
  tasks/2026-05-29-phase7-copyin-copyout-helper-core.md record the accepted
  helper contract and unit-tested implementation behavior.

Gap: no descriptor syscall smoke script exists, and no retained QEMU evidence
currently proves descriptor-backed user buffers, stdout/stderr writes, close,
dup, or stdin reads through the lower-EL syscall path.

## Recommended First Contract Slice

The next bounded task should be phase7-descriptor-syscall-contract-20260529,
scoped specifically to a stdout/stderr descriptor write contract. That contract
should define one stable write-style syscall boundary for fd 1 and fd 2, using
a user pointer and length copied with the accepted copy_from_user helper,
descriptor-table lookup and write-access checks, runtime-console0 as the only
contracted backing object, and x0 returning the accepted byte count or a
negative errno.

The recommended first slice should keep stdin/read, close, dup, pipes, regular
files, VFS/filesystem paths, process loading, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
blocking/readiness, signals, restart semantics, and live process-owned address
spaces deferred. Read should wait until the descriptor-facing TTY input,
blocking/readiness, EOF, and scheduler wait semantics are contracted.

The proof-only talos_copy_probe path remains quarantined. It must not become
the descriptor write syscall number, operation selector, stable ABI name, or
runtime-console shortcut.

## Validation

- static inspection: git status --short before documentation edits showed a
  pre-existing docs/src/roadmap.md working-tree edit; this task preserved and
  accommodated it.
- static source review: inspected descriptor-table core, syscall dispatch and
  lower-AArch64 frame routing, copy helpers, runtime-console and TTY surfaces,
  scheduler task/process-owner vocabulary, accepted descriptor/copy/pointer
  docs, roadmap, and decision log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
