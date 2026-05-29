# Phase 7 File Descriptor Table Source Inventory

Status: accepted as the documentation-only Milestone 7.4 file descriptor table
source inventory after the accepted
[Phase 7 Syscall ABI and Dispatch Closeout Checkpoint](phase7-syscall-abi-dispatch-closeout-checkpoint.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

This inventory maps the source owners, accepted contracts, retained evidence,
and missing contracts for turning the proof-owned inherited-stdio descriptor
slice into a process-owned file descriptor table milestone. It preserves the
Phase 7.3 boundary: lower-AArch64 svc #0, talos_write fd 1/fd 2, copy helpers,
and runtime-console0 evidence are accepted; full POSIX descriptor lifetime,
stdin/read, close, dup, process loading, filesystem, shell, networking, and SSH
remain deferred.

## Source Owners

### Descriptor Table Data Model

- `src/posix.rs::DescriptorTable` owns the target-independent table shape,
  fixed-capacity storage, empty construction, inherited stdio construction,
  lookup, lowest-free allocation, exact-slot allocation, close, and dup.
- `src/posix.rs::DescriptorEntry` owns descriptor access mode, flags, and the
  referenced `DescriptorObject`.
- `src/posix.rs::DescriptorObjectKind` reserves stdio input/output, regular
  file, directory, pipe endpoint, socket, device, and other-kernel-object
  vocabulary without implementing those backing subsystems.
- `STDIN_FD`, `STDOUT_FD`, and `STDERR_FD` fix descriptor numbers 0, 1, and 2
  for inherited stdio setup.

Accepted contract: the Phase 7.1 descriptor-table contract and core accept
target-independent allocation, lookup, close, dup, access checks, TTY-only
checks, object-kind tagging, and deterministic `PosixError` results. They do
not attach the table to a process, syscall frame, runtime console, TTY, VFS,
pipe, socket, device registry, scheduler owner, or hardware target.

Gap: Milestone 7.4 still needs a process-owned descriptor table contract that
states who owns a table, how inherited stdio is installed, how the current
process descriptor table is found from a syscall path, and which close/dup/read
semantics are accepted first.

### Syscall Dispatch And Descriptor Writes

- `src/syscall.rs` owns stable syscall numbers, `SyscallArguments`,
  `SyscallReturn`, errno encoding, target-independent `dispatch()`, and
  `dispatch_descriptor_write()`.
- `src/syscall.rs::TALOS_WRITE_SYSCALL` fixes `x8 = 1` for the accepted
  `talos_write` boundary.
- `src/arch/aarch64/exceptions.rs::try_route_lower_aarch64_syscall()` owns
  production lower-AArch64 svc #0 detection, x8 extraction, x0-through-x5
  capture, x0 mutation, and frame preservation.

Accepted contract: Milestone 7.3 accepts lower-AArch64 svc #0, x8 syscall
numbers, x0-through-x5 arguments, x0 return/-errno encoding, talos_nop,
unknown-syscall -ENOSYS, proof-only talos_copy_probe quarantine, and talos_write
fd 1/fd 2 through proof-owned inherited stdio descriptors.

Gap: the stable syscall path still uses proof-owned descriptor tables in the
QEMU and Pi 5 descriptor-write scenarios. It does not yet resolve a current
process, borrow or mutate that process's descriptor table, implement close or
dup through syscalls, route stdin/read, or define descriptor inheritance across
program loading.

### Copy Helpers And User Memory

- `src/posix.rs::copy_from_user()` and `copy_to_user()` own all-or-nothing
  byte movement after full-range validation.
- `UserMapping`, `UserMappingPermissions`, `UserAccessKind`,
  `USER_NULL_GUARD_END`, `USER_ADDRESS_SPACE_END`, and
  `DEFAULT_USER_COPY_LIMIT` own the accepted user-memory vocabulary.
- The pointer-copy and descriptor-write QEMU/Pi 5 proofs own explicit
  substitute mappings and backing storage for their retained evidence.

Accepted contract: copy helpers are target-independent and map invalid user
range, permission, wraparound, mapping, or backing-storage failures to
recoverable `EFAULT` before side effects. Descriptor-write evidence proves the
helpers can feed runtime-console0 writes through lower-EL syscall routing.

Gap: no live process address-space owner, page-table-backed user mapping
lookup, pinned user buffer, resumable lower-EL data-abort recovery, partial
write/restart policy, per-thread errno storage, argv/envp setup, or path/string
copying policy is accepted.

### Runtime Console, TTY, And Stdio Backing

- `src/runtime_console.rs` owns runtime-console0 output and input facade
  vocabulary.
- `src/tty.rs` owns accepted TTY line discipline input shaping, echo buffer,
  and control-event vocabulary.
- `src/diagnostic_command.rs` owns diagnostic command flow over TTY and
  runtime-console surfaces. Those calls remain kernel diagnostic behavior, not
  POSIX descriptor syscalls.
- `src/target/qemu_virt.rs` and `src/target/rpi5.rs` own proof scenarios and
  target backends for retained QEMU and Pi 5 descriptor-write evidence.

Accepted contract: fd 1 and fd 2 descriptor writes route to runtime-console0
through the runtime console facade in the accepted descriptor-write proof
slice. The proof does not make target UART/MMIO backends descriptor objects.

Gap: stdin/read lacks descriptor-facing TTY input, blocking/readiness, EOF,
nonblocking, signal interruption, scheduler wait/wakeup, controlling TTY, and
line-discipline ownership contracts. Runtime-console0 still lacks a
process-owned object registry and open-file-description lifetime model.

### Scheduler, Task, And Process Ownership

- `src/scheduler.rs::TaskId`, `Task`, and `ProcessOwnerId` own the accepted
  scheduler task and placeholder process-owner vocabulary.
- `Task::attach_process_owner()` and `Task::process_owner()` provide an
  explicit future extension point for process ownership.
- `SchedulerTaskSnapshot` preserves process-owner metadata in scheduler
  snapshots.

Accepted contract: scheduler ownership is CPU-local and explicit. ProcessOwnerId
is a placeholder identity that can be attached to tasks without accepting a
process table, PID allocator, address-space owner, or descriptor table owner.

Gap: there is no process table, PID allocator, current-process lookup from the
syscall handler, descriptor table inside a process object, fork/spawn/exec,
exit/wait, credentials, sessions, controlling TTY, process-owned address
spaces, or descriptor inheritance policy.

### VFS, Filesystems, Devices, Pipes, And Sockets

- `DescriptorObjectKind` reserves object-kind tags for regular files,
  directories, pipes, sockets, and devices.
- `src/posix.rs` already owns path normalization primitives and POSIX error
  vocabulary useful for future VFS work.
- Phase 8 remains the planned filesystem and program-loading phase; Phase 12
  remains networking/SSH.

Gap: Milestone 7.4 should not implement VFS lookup, filesystem-backed files,
pipe buffers, sockets, device registries, blocking readiness, poll/select,
socket-backed descriptors, RP1/PCIe, UART interrupt ownership, or DMA/cache
driver policy. Those object kinds remain reserved vocabulary only.

## Retained Evidence Anchors

- Descriptor table contract:
  `docs/src/project/phase7-descriptor-table-contract.md`.
- Descriptor table core task:
  `tasks/2026-05-28-phase7-descriptor-table-core.md`.
- Syscall ABI and dispatch closeout:
  `docs/src/project/phase7-syscall-abi-dispatch-closeout-checkpoint.md`.
- QEMU descriptor-write smoke:
  `tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log`.
- Pi 5 descriptor-write proof:
  `tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt`.
- Pi 5 descriptor-write restore proof:
  `tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-post-restore-status.json`.

These evidence anchors accept only the bounded proof-owned inherited stdio
descriptor-write frontier. They are input evidence for Milestone 7.4 planning,
not proof of process-owned descriptor lifetime.

## Missing Contracts

Milestone 7.4 needs explicit contracts before implementation can safely grow:

- process-owned descriptor table ownership, creation, attachment, and lookup;
- inherited stdio object lifetime and runtime-console0/TTY handle identity;
- current process and current address-space lookup at the syscall boundary;
- close semantics, including double close, object lifetime, and future
  reference-count behavior;
- dup semantics, including lowest-free allocation, shared underlying object,
  flags, and unsupported dup2/fcntl boundaries;
- stdin/read semantics, including TTY input, EOF, blocking/readiness,
  nonblocking, wait queues, and signal/restart policy;
- descriptor inheritance across future spawn/exec and close-on-exec behavior;
- object registries for files, directories, pipes, sockets, and devices;
- validation gates for QEMU/substitute and later Pi 5 physical claims.

The first Milestone 7.4 implementation should not guess at all of these at
once. It should contract the process-owned descriptor table boundary first,
then implement the smallest target-independent core under that contract.

## Recommended Next Task

The next bounded task should be
`phase7-process-descriptor-table-contract-20260529`, documentation-only under
Milestone 7.4.

That contract should define:

- the process-owned descriptor-table owner and lifetime vocabulary;
- how inherited fd 0, fd 1, and fd 2 are installed for the first user process;
- how the syscall path will locate a current process descriptor table without
  inventing PID, fork, spawn, exec, VFS, shell, or networking behavior;
- which target-independent table operations are accepted for the first
  process-owned slice;
- which error cases remain stable from the accepted descriptor table core; and
- which QEMU/substitute evidence would later prove the boundary.

The contract should keep stdin/read, close/dup syscalls, VFS/filesystem, path
copying, program loading, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor claims blocked
until later explicit tasks accept their contracts and validation gates.

## Validation

- static inspection: `git status --short` before documentation edits was clean.
- static source review: inspected `src/posix.rs`, `src/syscall.rs`,
  `src/runtime_console.rs`, `src/scheduler.rs`, accepted descriptor-table and
  descriptor-write docs, retained QEMU/Pi 5 evidence references, roadmap, and
  decision log.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
