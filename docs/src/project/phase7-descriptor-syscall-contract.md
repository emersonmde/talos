# Phase 7 Descriptor Syscall Contract

Status: accepted as the documentation-only Phase 7.3 descriptor syscall
contract after the accepted
[Phase 7 Descriptor Syscall Source Inventory](phase7-descriptor-syscall-source-inventory.md).
This task does not add Rust behavior, assembly behavior, QEMU runs, Pi 5
hardware runs, boot archives, hardware-lock use, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

This contract defines the first stable descriptor syscall slice: a bounded
stdout/stderr write operation backed by runtime-console0. It deliberately
does not contract stdin/read, close, dup, pipes, regular files, path copying,
blocking, readiness, partial writes, process-owned descriptor tables, or full
POSIX compatibility.

## Syscall Boundary

The first descriptor syscall is talos_write:

| Field | Contract |
| --- | --- |
| SVC immediate | svc #0, the accepted stable lower-AArch64 syscall trap |
| Syscall number | x8 = 1 |
| Name | talos_write |
| x0 | descriptor number |
| x1 | user virtual start address for bytes to write |
| x2 | byte length |
| x3 through x5 | reserved, must be zero |
| Return x0 | nonnegative byte count, or negative errno |

talos_nop remains syscall number 0. The proof-only talos_copy_probe number
0x7001 remains quarantined for accepted pointer-copy proof scenarios only; it
is not a descriptor operation, selector, compatibility mode, or write-like
shortcut.

The syscall trap preconditions, register capture, ELR/SPSR preservation, and
unknown-syscall -ENOSYS behavior remain the ones accepted by the syscall ABI
and trap-routing contracts. This contract only adds the descriptor operation
selected after a valid lower-AArch64 svc #0 reaches target-independent
dispatch.

## Descriptor Semantics

The only accepted descriptors for this first slice are inherited stdout and
stderr:

| Descriptor | Required object | Required access | Backing |
| ---: | --- | --- | --- |
| 1 | DescriptorObjectKind::StdioOutput | write-capable | runtime-console0 |
| 2 | DescriptorObjectKind::StdioOutput | write-capable | runtime-console0 |

The implementation task may use a focused QEMU/substitute descriptor table
initialized with DescriptorTable::with_inherited_stdio() until a later
process-owned descriptor-table contract exists. It must not claim a live
process descriptor table, open-file description lifetime, inherited descriptor
state across exec, or per-process stdio ownership.

Descriptor lookup failure, descriptor numbers other than 1 or 2, read-only
descriptors, closed descriptors, or non-stdio objects return -EBADF. If a
future test fixture constructs a write-capable non-stdio object, this contract
does not accept routing it to runtime-console0; it must return -ENOTSUP or
remain unimplemented until a separate object-kind contract exists.

## User Buffer And Copy Rules

talos_write treats x1 and x2 as a user byte range:

- x2 = 0 is a successful no-op and returns 0 after descriptor validation and
  reserved-register validation. It does not call runtime-console0.
- Nonzero lengths must be at most DEFAULT_USER_COPY_LIMIT.
- The full user range must pass copy_from_user() with UserAccessKind::Read
  before any runtime-console0 write side effect occurs.
- Null-guard, kernel-range, wraparound, unmapped, unreadable, or backing-store
  failures return -EFAULT.
- The first implementation may use explicit QEMU/substitute user mappings and
  backing storage. It must not infer authority from raw pointer values or
  claim page-table-backed process memory ownership.

There is no accepted partial-copy result. If validation fails, no console
bytes are written. If copy succeeds, the implementation attempts the complete
copied byte slice as one bounded write.

## Runtime-Console Behavior

The only accepted backing object is runtime-console0 through the
src/runtime_console.rs facade. Descriptor writes must not call target
UART/MMIO backends directly.

On success, talos_write returns exactly the requested byte length in x0. The
first slice does not accept short successful writes. If runtime-console0
reports a backend write failure, the implementation task must add or select a
documented errno mapping before acceptance; the preferred mapping is
PosixError::Io to EIO = 5. Until that mapping is implemented, backend failure
must not be reported as successful bytes written.

The contract does not define terminal line discipline, echo, canonical/raw
mode, blocking, readiness, nonblocking flags, EOF, signal interruption,
restart semantics, scheduler sleep/wakeup, or input polling. Those remain
future stdin/read and TTY descriptor work.

## Return And Error Contract

The descriptor write syscall returns through the existing syscall x0
convention:

| Case | Return |
| --- | ---: |
| fd 1 or 2, valid readable user range, complete runtime-console write | requested byte length |
| zero-length write to fd 1 or 2 with reserved registers zero | 0 |
| invalid, closed, read-only, or unsupported descriptor | -EBADF |
| valid descriptor but unsupported object kind or backend contract | -ENOTSUP |
| invalid user range, permissions, wraparound, or backing storage | -EFAULT |
| nonzero x3, x4, or x5, or length above the accepted limit | -EINVAL |
| runtime-console backend write failure after copy succeeds | -EIO once encoded; never success |
| any unaccepted syscall number | -ENOSYS |

The later implementation may extend the accepted errno encoder only as needed
for this slice. It must keep existing talos_nop, unknown-syscall, and
proof-only pointer-copy behavior intact.

## Proof And Validation Boundary

The next bounded task should be a QEMU descriptor-write smoke plan or a
target-independent descriptor-write implementation core, depending on the
supervisor's decomposition. The useful first evidence should prove:

- x8 = 1, fd 1 writes a user buffer through descriptor lookup,
  copy_from_user(), and runtime-console0, returning the byte count.
- fd 2 follows the same write-capable stdio-output path.
- fd 0, an invalid descriptor, and an unsupported object fail without console
  side effects.
- a guard-range or otherwise invalid user pointer returns -EFAULT.
- nonzero reserved registers return -EINVAL.
- talos_nop, unknown-syscall -ENOSYS, and proof-only talos_copy_probe
  quarantine remain unchanged.

QEMU/substitute evidence is sufficient for the first implementation slice if
the code touches only target-independent dispatch, descriptor-table lookup,
copy helpers, and runtime-console facade tests. A later Pi 5 proof is required
before claiming physical descriptor-backed user writes from lower EL.

## Deferred Surfaces

This contract keeps the following blocked: stdin/read, close, dup, pipes,
regular files, directories, sockets, devices, path copying, VFS/filesystem,
process loading, process-owned address spaces, inherited descriptor lifetime
across process creation, close-on-exec, descriptor flags beyond validation,
blocking/readiness, wait queues, signals, restart semantics, libc/Rust std
stdio, shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and any phase transition.

## Validation

- static inspection: reviewed the accepted descriptor syscall source
  inventory, syscall ABI/trap-routing contracts, pointer-copy contract,
  src/syscall.rs, src/posix.rs, and src/runtime_console.rs.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
