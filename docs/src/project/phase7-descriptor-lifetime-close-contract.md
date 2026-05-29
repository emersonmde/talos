# Phase 7 Descriptor Lifetime And Close Contract

Status: accepted as the documentation-only Milestone 7.4 descriptor lifetime
and close-semantics contract after the accepted
[Phase 7 Descriptor Lifetime And Close Source Inventory](phase7-descriptor-lifetime-close-source-inventory.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, close/dup/read
syscall surface, process loading, VFS/filesystem behavior, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

This contract defines the first target-independent descriptor lifetime and
close boundary for process-owned descriptor tables. It accepts table-local slot
removal and process-owned mutable lookup as the next implementation scope while
keeping syscall numbers, lower-EL close/dup/read routing, object finalization,
and full POSIX descriptor readiness blocked.

## Lifetime Model

The accepted lifetime unit for this slice is a descriptor table slot. A
descriptor slot may contain one DescriptorEntry, and that entry contains an
access mode, flags, and a DescriptorObject. Closing a descriptor removes only
the selected slot from the owning DescriptorTable and returns the removed entry
to the caller.

The following terms are intentionally distinct:

- descriptor number: the process-local table index used by callers;
- descriptor entry: the table-local access/flags/object record stored in one
  slot;
- descriptor object: the target-independent object kind plus reference value;
- open file description: future shared kernel object state, reference count,
  offset, readiness, and finalizer vocabulary that is not implemented by this
  slice.

DescriptorObject::reference() remains a stable target-independent handle, not a
reference count, pointer, inode id, pipe endpoint, socket id, MMIO address, or
device-registry entry. Duplicated descriptors may carry equal object values, but
this contract does not accept final close detection or object release behavior.

## Close Semantics

The table-local close rule is exact:

| Case | Result |
| --- | --- |
| occupied descriptor slot | slot is cleared; removed DescriptorEntry is returned |
| descriptor outside table capacity | PosixError::BadDescriptor |
| empty or already closed slot | PosixError::BadDescriptor |
| duplicate descriptor remains after closing original | duplicate stays valid and independently closeable |
| last descriptor carrying an object value is closed | table slot is removed only; no finalizer runs |

Closing fd 0, fd 1, or fd 2 is allowed at the table layer because stdio entries
are ordinary inherited descriptor slots. After fd 1 or fd 2 is closed,
descriptor-write lookup through that table must return BadDescriptor before any
runtime-console0 side effect. After fd 0 is closed, future read behavior
remains blocked because stdin/read is not accepted.

DescriptorFlags::CLOSE_ON_EXEC remains stored vocabulary only. This contract
does not define exec, close-on-exec application, descriptor-table cloning, or
descriptor inheritance across process creation.

## Process-Owned Close Boundary

The next implementation task should add only a target-independent close helper
on the process-owned descriptor-table boundary. The helper should resolve a
current ProcessOwnerId through ProcessDescriptorStore, borrow that owner's
mutable DescriptorTable, and apply DescriptorTable::close() to the selected
descriptor.

The accepted error mapping is:

- missing current owner: PosixError::BadDescriptor;
- unknown owner: PosixError::BadDescriptor;
- invalid, empty, or already closed descriptor: PosixError::BadDescriptor;
- successful close: the removed DescriptorEntry is returned to the
  target-independent caller.

The helper must not allocate a syscall number, mutate lower-EL registers,
publish a userspace ABI, call runtime-console0 directly, call target UART/MMIO
backends, run object finalizers, remove process owners, or introduce a process
exit/teardown path.

## Dup And Reuse Interaction

The existing DescriptorTable::dup() behavior remains table-local. It copies the
source entry into the lowest free slot and preserves the source object value.
That is enough for close interaction tests, but it is not a POSIX dup, dup2,
dup3, or fcntl contract.

The implementation evidence should prove:

- closing one duplicate leaves the other slot valid;
- closing the original before using the duplicate preserves the duplicate's
  object kind and reference value;
- closing both slots produces two table-local removals, not one object
  finalization;
- after close, later allocation uses the lowest available slot according to the
  existing allocation rule;
- duplicating a closed descriptor returns BadDescriptor.

No file offset sharing, append mode, nonblocking state, advisory locks,
reference-count decrement, readiness notification, or device detach behavior is
accepted.

## Evidence Required For Core

The next bounded implementation task should be
phase7-descriptor-close-core-20260529.

It should update only target-independent Rust and task documentation needed to
accept the process-owned close boundary. Focused unit tests should cover:

- process-owned close of fd 1 makes subsequent descriptor-write lookup through
  that owner return BadDescriptor;
- process-owned close of fd 2 follows the same table-local rule;
- double close through ProcessDescriptorStore remains BadDescriptor;
- invalid descriptor and missing/unknown owner remain BadDescriptor;
- descriptor reuse after close selects the lowest free slot;
- dup followed by close of the original leaves the duplicate usable;
- closing the duplicate and original never claims object finalization.

cargo fmt --all -- --check, cargo -Zjson-target-spec test, git diff --check,
and task-record inspection are sufficient for the target-independent core. QEMU
and Pi 5 evidence are not required until a later close/dup/read syscall path is
accepted.

## Deferred Surfaces

This contract keeps the following blocked: close syscall, dup syscall, read
syscall, syscall numbers and lower-EL ABI for close/dup/read, process loading,
fork/spawn/exec, descriptor inheritance across exec, close-on-exec application,
process exit teardown, wait/exit status, open-file-description reference
counting, object finalizers, VFS/filesystem lookup, regular files,
directories, pipes, sockets, device registries, stdin behavior, TTY
blocking/readiness, EOF, nonblocking flags, wait queues, signals, restart
semantics, path copying, argv/envp loading, per-thread errno storage,
libc/Rust std stdio, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, Pi 5 physical close/dup/read claims, and
full POSIX descriptor readiness.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
phase7-descriptor-close-core-20260529.

That task should implement the target-independent process-owned close helper
and focused unit tests described above. It should not add a syscall surface,
run QEMU, acquire hardwareTestLock, publish a boot archive, run Pi 5 hardware,
or advance process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, or a phase transition.

## Validation

- static inspection: reviewed the accepted descriptor lifetime and close source
  inventory, process descriptor table contract/closeout, descriptor syscall
  contract, src/posix.rs, src/syscall.rs, and retained descriptor evidence
  references.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
