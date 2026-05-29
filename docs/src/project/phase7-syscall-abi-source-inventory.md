# Phase 7 Syscall ABI Source Inventory

Status: accepted as the documentation-only Phase 7.3 source inventory before
any syscall ABI contract or implementation. This document follows the accepted
[Phase 7 EL0 Trap Proof Closeout Checkpoint](phase7-el0-trap-proof-closeout-checkpoint.md).
It does not add Rust behavior, assembly behavior, syscall numbers, syscall
dispatch, copy-in/copy-out helpers, descriptor I/O, process loading, VFS,
filesystem, shell behavior, networking, SSH, QEMU reruns, Pi 5 hardware runs,
archive publishing, hardware-lock use, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

This inventory maps the source owners and missing contracts that constrain the
first stable SVC/syscall ABI. The accepted lower-EL proof has one diagnostic
SVC marker path on QEMU and Pi 5; it is evidence that the trap machinery works,
not a syscall interface.

## Source Owners

### Lower-EL Synchronous Exception Entry

- src/arch/aarch64/vectors.S owns the AArch64 vector table, saves x0 through
  x30 in the current ExceptionFrame order, reads ESR/ELR/FAR/SPSR for the
  active exception level, and calls rust_exception_handler for synchronous
  exceptions. It already distinguishes lower-AArch64 synchronous vector number
  8 from same-EL entries.
- src/arch/aarch64/exceptions.rs owns ExceptionVector, ExceptionFrame,
  exceptions::init, and rust_exception_handler. ExceptionVector::LowerAarch64Sync
  is named as lower-aarch64-sync, and ExceptionFrame::reg() exposes saved
  general registers for diagnostic handlers.
- The default synchronous exception path still prints diagnostic state and
  halts. Only proof-specific cfg(talos_boot_scenario = ...) handlers intercept
  the lower-EL SVC path.

Gap: there is no production syscall trap dispatcher, syscall trap-frame type,
recoverable process-fault policy, or general return-to-user policy. The next
contract must decide whether the first stable handler reuses ExceptionFrame
directly or wraps it in a syscall-owned frame that also records vector, ESR,
FAR, ELR, SPSR, task identity, and process-owner identity.

### Diagnostic SVC Proof Surfaces

- src/target/qemu_virt.rs::run_el0_trap_smoke and
  src/target/rpi5.rs::run_el0_trap_proof own the accepted proof payloads,
  fixed user text at 0x0000_0000_0010_0000, user stack ending at
  0x0000_0000_0020_0000, guard validation, and diagnostic SVC marker
  0x7a10.
- Their handlers check the diagnostic ESR 0x0000_0000_5400_7a10, vector
  lower-aarch64-sync, saved ELR inside the proof payload, saved user SP inside
  the proof stack, and x0 marker value before printing PASS and the proof
  classification.
- The ESR value encodes EC=0x15 for AArch64 SVC and ISS=0x7a10 for the
  diagnostic immediate, but no production code owns a stable SVC decode policy
  yet.
- scripts/qemu-el0-trap-smoke.sh and scripts/rpi5-el0-trap-proof-static-check.sh
  are proof gates. They verify the bounded diagnostic scenario and retained
  evidence, not a stable ABI.

Gap: marker 0x7a10 is proof vocabulary only. It must not become syscall number
0x7a10, a dispatch-table shape, or the numeric ABI for user programs. The next
contract must choose a stable SVC decode rule, a small syscall-number
namespace, and state whether the number lives in x8, x16, or another register.

### Syscall Number And Argument Registers

- The current proof payload uses an SVC immediate and x0 diagnostic marker to
  make evidence readable. No source file owns a stable syscall-number enum,
  syscall argument register contract, or dispatch table.
- The saved frame contains x0 through x30, so it can support the conventional
  AArch64 argument registers x0 through x5 plus a separate syscall-number
  register, but that convention is not accepted yet.

Gap: the next contract must name the first syscall-number carrier, argument
registers, clobber/preserve rules, maximum argument count, unknown-syscall
behavior, and whether the SVC immediate is ignored, reserved, or checked for a
Talos-specific marker.

### Return And Error Convention

- src/posix.rs::PosixError owns accepted errno-style names. Existing names
  include Fault / EFAULT, InvalidArgument / EINVAL, BadDescriptor / EBADF, and
  NotImplemented / ENOSYS.
- PosixError::name() is a textual diagnostic mapping only. It is not a numeric
  errno table and does not define syscall return registers.

Gap: there is no accepted numeric errno mapping, negative-return convention,
success register, secondary return register, restart policy, signal policy, or
per-thread errno storage. The next contract should keep the first ABI simple:
one return value register and one deterministic error encoding for invalid or
unimplemented syscalls, with numeric values documented before implementation.

### User-Copy Preconditions

- src/posix.rs owns the accepted user-memory validation vocabulary:
  USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, DEFAULT_USER_COPY_LIMIT,
  UserAccessKind, UserMappingPermissions, UserMapping, and
  validate_user_memory_access.
- That validation can reject null, wraparound, kernel-range, guard, unmapped,
  permission-mismatch, and length-limit cases before byte-copy helpers exist.

Gap: Talos does not yet copy bytes between user and kernel memory, pin user
ranges, handle partial copies, retry interrupted copies, or tie validation to
per-process page tables. The first syscall ABI contract should allow a proof
slice with scalar arguments only, or explicitly require a separate copy helper
task before any pointer-taking syscall is implemented.

### Descriptor-Table Interaction

- src/posix.rs::DescriptorTable owns the accepted process-local descriptor data
  model: inherited stdio handles, allocation, exact-slot allocation, lookup,
  close, dup, access checks, reserved object kinds, and deterministic
  PosixError results.
- DescriptorObjectKind::StdioInput and DescriptorObjectKind::StdioOutput name
  reserved stdio handles, but they do not bind to runtime console or TTY I/O
  behavior.
- src/runtime_console.rs, src/tty.rs, and src/diagnostic_command.rs own kernel
  diagnostic console and TTY surfaces. They are candidates for later stdio
  backing, not descriptor syscalls.

Gap: there is no read, write, close, dup, fcntl, readiness, blocking,
wait-queue, device object, VFS object, or user-buffer copy contract at the
syscall boundary. The first ABI proof should not expose descriptor I/O unless
a later task accepts descriptor object and copy semantics first.

### Process And Task Ownership

- src/scheduler.rs owns scheduler-local TaskId, Task, ProcessOwnerId,
  Task::attach_process_owner, PerCoreScheduler, ProductionSchedulerRuntime, and
  SharedSchedulerMetadata.
- A task can carry an optional process-owner placeholder, and shared metadata
  can publish that placeholder, but there is no PID allocator, process table,
  parent/child model, exit status, wait queue, credential model, process-owned
  address-space handle, or descriptor-table lifetime owner.
- Phase 6 production scheduler rules still keep scheduler mutation in
  owner-local normal control flow, not IRQ/IPI or arbitrary exception context.

Gap: a syscall handler can identify only the current scheduler task and any
attached process-owner placeholder that existing code makes available. The next
contract must not require process loading, PID lookup, wait/exit, or
descriptor-table lifetime behavior that does not exist yet.

## First Proof Slice Recommendation

The next bounded task should be phase7-syscall-abi-contract-20260529.

That contract should define a minimal stable SVC ABI before implementation:
syscall-number carrier, argument registers, return/error convention, trap-frame
ownership, unknown-syscall behavior, scalar-only first proof limits, and how
the handler reports deterministic evidence. A suitable first implementation
slice after that contract would prove dispatch for an invalid or no-op scalar
syscall without process loading, pointer arguments, descriptor I/O, VFS,
filesystem, shell, networking, SSH, or hardware requirements.

Hardware proof should remain blocked. The accepted Pi 5 lower-EL evidence
already proves the bounded trap path; a syscall ABI contract can start with
static inspection, target-independent unit tests where applicable, and a later
QEMU proof task when implementation is explicitly accepted.

## Validation

- static inspection: git status --short before edits was clean.
- static source review: inspected exception vector entry, Rust exception
  routing, QEMU and Pi 5 diagnostic EL0 trap proof surfaces, POSIX error and
  user-memory primitives, descriptor-table model, scheduler task/process-owner
  metadata, runtime-console, TTY, diagnostic command, roadmap, and decision
  log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs were not required
  because this task changes only Markdown documentation and durable worker
  state.
