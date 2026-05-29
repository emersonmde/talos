# Phase 7 Dup Syscall Contract

Status: accepted as the documentation-only Milestone 7.4 dup syscall
contract after the accepted
[Phase 7 Pi 5 Close Syscall Proof Closeout Checkpoint](phase7-pi5-close-syscall-proof-closeout-checkpoint.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, read syscall
contract, process loading, VFS/filesystem behavior, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, object finalization, or
DMA/cache-driver policy.

This contract defines the first user-visible descriptor duplication boundary
through the current process-owned descriptor table. It accepts only a dup
syscall that copies one existing descriptor entry into the lowest available
slot in the current owner's ProcessDescriptorStore table and returns the new
descriptor number through the existing x0 success/-errno convention.

## Syscall Boundary

The first dup syscall is talos_dup:

| Field | Contract |
| --- | --- |
| SVC immediate | svc #0, the accepted stable lower-AArch64 syscall trap |
| Syscall number | x8 = 3 |
| Name | talos_dup |
| x0 | source descriptor number |
| x1 through x5 | reserved, must be zero |
| Return x0 | new descriptor number on success, or negative errno |

talos_nop remains syscall number 0. talos_write remains syscall number 1.
talos_close remains syscall number 2. The proof-only talos_copy_probe number
0x7001 remains quarantined for accepted pointer-copy proof scenarios only; it
is not a descriptor operation, compatibility mode, dup selector, or errno
transport.

The syscall trap preconditions, register capture, ELR/SPSR preservation, and
unknown-syscall -ENOSYS behavior remain the accepted syscall ABI and
trap-routing contracts. This contract only adds the descriptor operation
selected after a valid lower-AArch64 svc #0 reaches target-independent
dispatch with a process descriptor store available.

## Process-Owned Descriptor Lookup

talos_dup must resolve the current process-owned descriptor table before
allocating any destination slot. The accepted ownership rule is:

1. Resolve the current ProcessOwnerId from the caller-provided current-task or
   substitute context.
2. Resolve that owner through ProcessDescriptorStore.
3. Mutably borrow the owner's DescriptorTable.
4. Apply the table-local DescriptorTable::dup() rule to the source descriptor
   number from x0.

Missing current owner, unknown owner, missing descriptor table, invalid source
descriptor, empty source descriptor, or already closed source descriptor all
return -EBADF. The first implementation may use the same target-independent
owner/store fixture shape accepted by the QEMU process descriptor stdio and
close syscall smokes. It must not claim process loading, PID allocation,
cross-CPU process registries, process exit teardown, or final object release.

## Dup Semantics

The successful dup rule is table-local:

| Case | Return | Effect |
| --- | ---: | --- |
| occupied source descriptor and free destination slot exists | new descriptor number | copies the source DescriptorEntry into the lowest free slot |
| source descriptor outside table capacity | -EBADF | table unchanged |
| source descriptor empty or already closed | -EBADF | table unchanged |
| missing current owner or unknown owner | -EBADF | store unchanged |
| no free destination slot | -EMFILE | table unchanged |
| any reserved x1 through x5 nonzero | -EINVAL | table unchanged |

The new descriptor carries the same DescriptorEntry value as the source,
including access mode, flags, DescriptorObject kind, and DescriptorObject
reference. DescriptorFlags::CLOSE_ON_EXEC is copied as stored vocabulary only;
this contract does not define exec, close-on-exec application, descriptor-table
cloning, or descriptor inheritance across process creation.

Duplicating fd 0, fd 1, or fd 2 is allowed when the source slot is occupied.
Stdio descriptors are ordinary inherited table slots in this slice. A duplicate
of fd 1 or fd 2 may later be used by accepted descriptor-write dispatch because
it carries the same writable runtime-console0 object value. A duplicate of fd 0
does not accept read behavior because stdin/read is still blocked.

## Close And Reuse Interaction

talos_dup preserves the accepted table-local close/dup interaction:

- duplicating a descriptor does not consume, close, or mutate the source slot;
- closing the source after dup leaves the duplicate valid;
- closing the duplicate after dup leaves the source valid;
- after close, later dup or allocation uses the lowest available slot according
  to the existing DescriptorTable allocation rule;
- no open-file-description reference count, file offset sharing, final close
  detection, device detach, or object finalizer is accepted.

This contract intentionally does not define dup2, dup3, fcntl(F_DUPFD),
close-on-exec flag changes, exact destination selection, file-offset sharing,
append mode, nonblocking state, advisory locks, readiness notification, or
process-fork inheritance.

## Return And Error Contract

talos_dup returns through the existing syscall x0 convention:

| Case | Return |
| --- | ---: |
| valid current owner, occupied source descriptor, and free slot | new descriptor number |
| source outside capacity, empty, already closed, or invalid | -EBADF |
| missing current owner, unknown owner, or missing table | -EBADF |
| table has no free destination slot | -EMFILE |
| nonzero x1, x2, x3, x4, or x5 | -EINVAL |
| source descriptor number does not fit usize | -EBADF |
| any unaccepted syscall number | -ENOSYS |

EMFILE is required for the syscall return even though the current
target-independent PosixError vocabulary does not yet expose a stable errno
number for it. The implementation task must add only the minimum errno
vocabulary needed to encode -EMFILE for table-full dup and preserve existing
errno encodings for talos_nop, talos_write, talos_close, unknown syscall,
descriptor-write, and proof-only pointer-copy quarantine behavior.

## Proof And Validation Boundary

The next bounded task should be phase7-dup-syscall-core-20260529.

That implementation task should add the smallest target-independent dup syscall
dispatch surface using the current ProcessOwnerId-backed
ProcessDescriptorStore and DescriptorTable::dup(). It should fix the syscall
number and SyscallNumber variant, validate x1 through x5 as reserved zero,
convert x0 to a descriptor index, duplicate the source entry into the lowest
free slot, and return the new descriptor number or negative errno. Focused unit
tests should prove:

- dup of fd 1 returns the lowest free descriptor and preserves later
  descriptor-write lookup through the duplicate;
- dup of fd 2 follows the same table-local rule;
- dup of fd 0 preserves readable stdin object vocabulary without accepting
  read behavior;
- invalid, empty, already closed, missing-owner, and unknown-owner cases return
  BadDescriptor/-EBADF;
- table-full dup returns TooManyOpenFiles/-EMFILE and leaves the table
  unchanged;
- nonzero reserved arguments return InvalidArgument/-EINVAL without
  duplicating;
- talos_nop, talos_write, talos_close, unknown-syscall, and proof-only
  copy-probe quarantine behavior remain unchanged.

Static inspection, cargo fmt, no_std unit tests, git diff --check, and mdbook
build are sufficient for the target-independent core. A later QEMU dup syscall
smoke is required before claiming lower-AArch64 runtime evidence, and a later
serialized Pi 5 proof is required before claiming physical dup behavior.

## Deferred Surfaces

This contract keeps the following blocked: read syscall, stdin/read object
policy, QEMU dup/read smoke, Pi 5 physical dup/read proof, boot archive
publication, process loading, fork/spawn/exec, descriptor inheritance across
exec, close-on-exec application, dup2/dup3/fcntl, process exit teardown,
wait/exit status, open-file-description reference counting, object finalizers,
file-offset sharing, VFS/filesystem lookup, regular files, directories, pipes,
sockets, device registries, TTY blocking/readiness, EOF, nonblocking flags,
wait queues, signals, restart semantics, path copying, argv/envp loading,
per-thread errno storage, libc/Rust std stdio, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
descriptor readiness.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
phase7-dup-syscall-core-20260529.

That task should implement only the target-independent dup syscall dispatch
surface and focused tests described above. It should not run QEMU, acquire
hardwareTestLock, publish a boot archive, run Pi 5 hardware, implement read,
or advance process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, or a phase transition.

## Validation

- static inspection: reviewed the accepted close/dup/read syscall source
  inventory, descriptor lifetime and close contract, close syscall contract and
  close proof closeout, src/posix.rs, and src/syscall.rs.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
