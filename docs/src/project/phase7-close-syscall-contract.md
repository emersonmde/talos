# Phase 7 Close Syscall Contract

Status: accepted as the documentation-only Milestone 7.4 close syscall
contract after the accepted
[Phase 7 Close, Dup, And Read Syscall Source Inventory](phase7-close-dup-read-syscall-source-inventory.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, dup/read syscall
contract, process loading, VFS/filesystem behavior, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

This contract defines the first user-visible descriptor close boundary through
the current process-owned descriptor table. It accepts only a close syscall
that removes one descriptor slot from the current owner's
ProcessDescriptorStore table and returns through the existing x0
success/-errno convention.

## Syscall Boundary

The first close syscall is talos_close:

| Field | Contract |
| --- | --- |
| SVC immediate | svc #0, the accepted stable lower-AArch64 syscall trap |
| Syscall number | x8 = 2 |
| Name | talos_close |
| x0 | descriptor number |
| x1 through x5 | reserved, must be zero |
| Return x0 | 0 on success, or negative errno |

talos_nop remains syscall number 0. talos_write remains syscall number 1.
The proof-only talos_copy_probe number 0x7001 remains quarantined for accepted
pointer-copy proof scenarios only; it is not a descriptor operation,
compatibility mode, close selector, or errno transport.

The syscall trap preconditions, register capture, ELR/SPSR preservation, and
unknown-syscall -ENOSYS behavior remain the accepted syscall ABI and
trap-routing contracts. This contract only adds the descriptor operation
selected after a valid lower-AArch64 svc #0 reaches target-independent
dispatch with a process descriptor store available.

## Process-Owned Descriptor Lookup

talos_close must resolve the current process-owned descriptor table before
mutating any slot. The accepted ownership rule is:

1. Resolve the current ProcessOwnerId from the caller-provided current-task or
   substitute context.
2. Resolve that owner through ProcessDescriptorStore.
3. Mutably borrow the owner's DescriptorTable.
4. Apply ProcessDescriptorStore::close_current_descriptor() to the descriptor
   number from x0.

Missing current owner, unknown owner, missing descriptor table, invalid
descriptor, empty descriptor, or already closed descriptor all return -EBADF.
The first implementation may use the same target-independent owner/store
fixture shape accepted by the QEMU process descriptor stdio smoke. It must not
claim process loading, PID allocation, cross-CPU process registries, process
exit teardown, or final object release.

## Close Semantics

The successful close rule is table-local:

| Case | Return | Effect |
| --- | ---: | --- |
| occupied descriptor slot | 0 | selected slot is cleared |
| descriptor outside table capacity | -EBADF | table unchanged |
| empty or already closed slot | -EBADF | table unchanged |
| missing current owner or unknown owner | -EBADF | store unchanged |
| any reserved x1 through x5 nonzero | -EINVAL | table unchanged |

The removed DescriptorEntry is internal evidence for the helper boundary. It
is not returned to userspace. Userspace observes only x0 = 0 on success.

Closing fd 0, fd 1, or fd 2 is allowed when the slot is occupied. Stdio
descriptors are ordinary inherited table slots in this slice. After closing
fd 1 or fd 2, later talos_write lookup through the same process descriptor
store must return -EBADF before any runtime-console0 side effect. After
closing fd 0, future read behavior remains blocked because stdin/read is not
accepted.

## Duplicate Interaction

The existing DescriptorTable::dup() behavior remains table-local and outside
this syscall contract, but close must preserve its already accepted lifetime
interaction:

- closing one descriptor slot does not remove another slot that carries an
  equal DescriptorObject value;
- closing the original while a duplicate remains leaves the duplicate valid;
- closing both slots is two independent table-local removals;
- no open-file-description reference count, final close detection, device
  detach, offset update, or object finalizer is accepted.

The implementation task should add focused unit tests around close syscall
dispatch only if they are required to prove the syscall boundary. Existing
target-independent close/dup helper tests remain the authority for table-local
duplicate lifetime until a later dup syscall contract exists.

## Return And Error Contract

talos_close returns through the existing syscall x0 convention:

| Case | Return |
| --- | ---: |
| valid current owner and occupied descriptor | 0 |
| descriptor outside capacity, empty, already closed, or invalid | -EBADF |
| missing current owner, unknown owner, or missing table | -EBADF |
| nonzero x1, x2, x3, x4, or x5 | -EINVAL |
| descriptor number does not fit usize | -EBADF |
| any unaccepted syscall number | -ENOSYS |

The implementation must preserve accepted talos_nop, talos_write,
unknown-syscall, descriptor-write fd/error, and proof-only pointer-copy
quarantine behavior. It must not reinterpret close failures as process-fatal
exceptions, per-thread errno storage, signal delivery, restart behavior, or
diagnostic marker completion.

## Proof And Validation Boundary

The next bounded task should be phase7-close-syscall-core-20260529.

That implementation task should add the smallest target-independent close
syscall dispatch surface using ProcessDescriptorStore::close_current_descriptor().
It should fix the syscall number and SyscallNumber variant, validate x1
through x5 as reserved zero, convert x0 to a descriptor index, call the
process-owned close helper, and return x0 = 0 or negative errno. Focused
unit tests should prove:

- close of fd 1 through the process-owned syscall helper makes later
  descriptor-write lookup through that owner return BadDescriptor;
- close of fd 2 follows the same table-local rule;
- double close returns BadDescriptor;
- invalid descriptor, missing owner, and unknown owner return BadDescriptor;
- nonzero reserved arguments return InvalidArgument without closing;
- talos_nop, talos_write, unknown-syscall, and proof-only copy-probe
  quarantine behavior remain unchanged.

Static inspection, cargo fmt, no_std unit tests, git diff --check, and mdbook
build are sufficient for the target-independent core. A later QEMU close
syscall smoke is required before claiming lower-AArch64 runtime evidence, and
a later serialized Pi 5 proof is required before claiming physical close
behavior.

## Deferred Surfaces

This contract keeps the following blocked: dup syscall, read syscall,
stdin/read object model, descriptor duplication ABI, QEMU close/dup/read
smoke, Pi 5 physical close/dup/read proof, boot archive publication, process
loading, fork/spawn/exec, descriptor inheritance across exec, close-on-exec
application, process exit teardown, wait/exit status, open-file-description
reference counting, object finalizers, VFS/filesystem lookup, regular files,
directories, pipes, sockets, device registries, TTY blocking/readiness, EOF,
nonblocking flags, wait queues, signals, restart semantics, path copying,
argv/envp loading, per-thread errno storage, libc/Rust std stdio, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
phase7-close-syscall-core-20260529.

That task should implement only the target-independent close syscall dispatch
surface and focused tests described above. It should not run QEMU, acquire
hardwareTestLock, publish a boot archive, run Pi 5 hardware, implement dup or
read, or advance process loading, VFS/filesystem, shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or a phase
transition.

## Validation

- static inspection: reviewed the accepted close/dup/read syscall source
  inventory, descriptor lifetime and close contract, descriptor close core
  closeout, process descriptor table contract/smoke evidence, src/posix.rs,
  and src/syscall.rs.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
