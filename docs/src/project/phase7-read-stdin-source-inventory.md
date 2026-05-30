# Phase 7 Read And Stdin Source Inventory

Status: accepted as the documentation-only Milestone 7.4 read/stdin source
inventory after the accepted
[Phase 7 Pi 5 Dup Syscall Proof Closeout Checkpoint](phase7-pi5-dup-syscall-proof-closeout-checkpoint.md).
This task adds no Rust behavior, assembly behavior, syscall-number
allocation, read implementation, QEMU run, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, object finalization, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

This inventory maps the current source owners, accepted evidence, and missing
policy for turning the reserved fd 0/stdin descriptor vocabulary into a later
bounded talos_read contract. It separates accepted write/close/dup descriptor
behavior from unaccepted read/stdin data delivery.

## Source Owners

### Syscall Dispatch And Numbering

- `src/syscall.rs` owns stable lower-EL syscall vocabulary, SVC #0 argument
  capture, and x0 success/-errno return encoding at the target-independent
  dispatch boundary.
- Accepted numbers are `TALOS_NOP_SYSCALL = 0`, `TALOS_WRITE_SYSCALL = 1`,
  `TALOS_CLOSE_SYSCALL = 2`, and `TALOS_DUP_SYSCALL = 3`.
- `dispatch_process_descriptor()` owns the process-descriptor dispatch surface
  for talos_write, talos_close, and talos_dup when a current owner,
  ProcessDescriptorStore, user mappings, user memory, scratch buffer, and
  console backend are supplied.

Gap: no stable talos_read syscall number, `SyscallNumber` variant, argument
contract, return contract, or dispatch helper exists. The next contract should
choose the read syscall number explicitly and keep all unallocated numbers at
the accepted -ENOSYS behavior.

### Copy-Out And User-Memory Checks

- `src/posix.rs::copy_to_user()` owns target-independent all-or-nothing
  copy-out into an explicit substitute user-memory slice after complete range
  validation.
- `validate_user_memory_access()` owns the accepted user range, null-guard,
  user/kernel split, mapping, permission, wraparound, and length-limit checks.
- `UserAccessKind::Write` is the required permission kind for copy-out; read
  must not reuse copy-in permissions to write user buffers.
- `DEFAULT_USER_COPY_LIMIT` currently bounds the accepted copy helper surface.

Accepted evidence: the pointer-copy proof and copy-in/copy-out helper core
accepted `copy_to_user()` as target-independent helper behavior under explicit
substitute mappings.

Gap: no descriptor syscall uses `copy_to_user()` to deliver bytes to an EL0
buffer. No recoverable lower-EL data-abort table, partial-copy policy,
per-thread errno storage, demand paging, signal restart, or process-fatal user
fault policy is accepted.

### Process Descriptor Store And Fd 0 Lookup

- `src/posix.rs::ProcessDescriptorStore` owns current ProcessOwnerId lookup and
  maps missing current owner, unknown owner, or missing table to EBADF.
- `DescriptorTable::with_inherited_stdio()` installs fd 0, fd 1, and fd 2 in a
  bounded table for the current process-descriptor slices.
- fd 0 is `DescriptorAccess::ReadOnly` and
  `DescriptorObjectKind::StdioInput` with reference `STDIN_FD`.
- `DescriptorEntry::require_readable()` maps non-readable descriptors to
  EBADF and is the current readable-descriptor gate.

Accepted evidence: inherited fd 0 exists as read-only stdio input in descriptor
table/process descriptor tests, and talos_dup can duplicate fd 0 without
accepting read behavior. Accepted write/close/dup evidence proves lookup,
mutation, EBADF handling, and duplicate lifetime for the current
ProcessOwnerId-backed store.

Gap: no read helper resolves fd 0 through ProcessDescriptorStore, no read path
checks `require_readable()`, and no accepted object method maps
`StdioInput` to bytes. Object finalization, open-file-description reference
counts, process teardown, close-on-exec application, and descriptor inheritance
outside the bounded inherited-stdio fixture remain unaccepted.

### Runtime Console, TTY, And Stdin Surfaces

- `src/runtime_console.rs` owns runtime-console0 output and input-backend
  vocabulary. `ConsoleInputPollOutcome` distinguishes byte, no-data,
  backend-unavailable, and backend-error outcomes for diagnostic clients.
- `src/tty.rs` owns target-independent raw/canonical-lite line-discipline
  parsing, bounded polling diagnostic outcomes, and control-event vocabulary.
- `docs/src/architecture/tty-stdio.md` states that fd 0 should eventually
  attach to the readable side of a controlling TTY only after an accepted input
  source exists.
- `src/diagnostic_command.rs` consumes diagnostic TTY lines; it is not a POSIX
  read/syscall path.

Accepted evidence: QEMU and Pi 5 polling RX diagnostics proved bounded
diagnostic input through TTY/core console-input paths, but those diagnostics
are deliberately separate from descriptors and syscalls.

Gap: there is no contract tying fd 0 to runtime-console0 input, TTY canonical
line buffers, raw bytes, EOF, readiness, blocking, nonblocking mode, wait
queues, signal/control-character effects, process groups, terminal sessions,
or scheduler wakeups.

### Retained Descriptor Evidence

The retained accepted descriptor frontier before read/stdin is:

- talos_write fd 1/fd 2 to runtime-console0 through proof-owned and
  ProcessOwnerId-backed inherited stdio descriptors;
- talos_close through ProcessDescriptorStore, including write-after-close
  EBADF behavior and Pi 5 physical proof for the focused close scenario;
- talos_dup through ProcessDescriptorStore, including lowest-free fd
  allocation, -EMFILE, reserved-register -EINVAL, duplicate lifetime, and Pi 5
  physical proof for the focused dup scenario;
- talos_nop, unknown-syscall -ENOSYS, proof-only copy-probe quarantine, and
  diagnostic-marker quarantine regressions.

These evidence paths do not prove read/stdin byte delivery. They only prove
the descriptor table, mutable store, and syscall-routing substrate on which a
later read contract can build.

## Read/Stdin Policy Gaps

The following gaps must be resolved by later explicit tasks before any read
behavior is accepted:

- byte source: fixed proof input, runtime-console-backed bytes, TTY line
  buffer, or another bounded input object;
- EOF: whether zero-length success, Ctrl-D, backend-unavailable, or
  proof-buffer exhaustion means EOF for the first slice;
- blocking/readiness: whether the first read returns immediately, waits,
  reports EAGAIN, or uses bounded proof input only;
- partial reads: whether reads may return fewer bytes than requested on
  success, and what happens when the destination buffer is larger than the
  available source;
- nonblocking mode: whether descriptor flags influence read readiness or stay
  blocked until fcntl/poll work;
- restart and signals: how interrupted reads, control events, and signal
  delivery remain blocked without changing syscall results;
- user copy-out failure: exact -EFAULT behavior and no-mutation ordering when
  fd lookup succeeds but the destination range is invalid;
- object lifetime/finalization: how stdin object references survive close/dup,
  process exit, and future TTY teardown;
- physical proof: whether QEMU/substitute evidence is enough for the first
  read slice, and what later Pi 5 proof must demonstrate before accepting
  physical stdin/read behavior.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
`phase7-read-stdin-contract-20260529`, documentation-only.

That contract should define one small talos_read/stdin invariant: syscall
number, x0/x1/x2 argument roles, reserved-register behavior, success and
-errno returns, fd/error cases, copy-out failure behavior, and the exact
stdin byte-source/EOF/readiness stance for the first implementation slice. It
should keep read implementation, QEMU run, Pi 5 hardware run, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, and full POSIX descriptor
readiness blocked.

## Validation

- static inspection: `git status --short` before documentation edits was
  clean.
- static source review: inspected `src/syscall.rs`, `src/posix.rs`,
  `src/runtime_console.rs`, `src/tty.rs`,
  `docs/src/architecture/tty-stdio.md`, accepted copy helper docs, and
  accepted write/close/dup task records and closeout docs.
- static documentation diff: added this inventory, linked it from SUMMARY,
  updated roadmap current status, updated the decision log, and added the task
  record.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
