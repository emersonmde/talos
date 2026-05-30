# Phase 7 Read And Stdin Contract

Status: accepted as the documentation-only Milestone 7.4 read/stdin
contract after the accepted
[Phase 7 Read And Stdin Source Inventory](phase7-read-stdin-source-inventory.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, read implementation,
process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
object finalization, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

This contract defines the first bounded user-visible read boundary through the
current process-owned descriptor table. It accepts only a target-independent
talos_read contract for fd 0/stdin and duplicates of the inherited StdioInput
descriptor, backed by fixed proof input supplied to the later core task. It
does not attach fd 0 to runtime-console0 input, a TTY line buffer, filesystem
input, pipes, sockets, or hardware UART input.

## Syscall Boundary

The first read syscall is talos_read:

| Field | Contract |
| --- | --- |
| SVC immediate | svc #0, the accepted stable lower-AArch64 syscall trap |
| Syscall number | x8 = 4 |
| Name | talos_read |
| x0 | descriptor number |
| x1 | destination user pointer |
| x2 | requested byte count |
| x3 through x5 | reserved, must be zero |
| Return x0 | bytes copied on success, 0 at bounded EOF, or negative errno |

talos_nop remains syscall number 0. talos_write remains syscall number 1.
talos_close remains syscall number 2. talos_dup remains syscall number 3. The
proof-only talos_copy_probe number 0x7001 remains quarantined for accepted
pointer-copy proof scenarios only; it is not a read selector, compatibility
mode, errno transport, or stdin source.

The syscall trap preconditions, register capture, ELR/SPSR preservation, and
unknown-syscall -ENOSYS behavior remain the accepted syscall ABI and
trap-routing contracts. This contract only adds the descriptor operation
selected after a valid lower-AArch64 svc #0 reaches target-independent
dispatch with a process descriptor store, user mappings, user memory, a
kernel scratch buffer, and fixed proof stdin state available.

## Process-Owned Descriptor Lookup

talos_read must resolve the current process-owned descriptor table before
copying to userspace or consuming any proof input. The accepted ownership rule
is:

1. Reject nonzero x3, x4, or x5 with -EINVAL and no mutation.
2. Convert x0 to a descriptor index; an unrepresentable descriptor is -EBADF.
3. Resolve the current ProcessOwnerId from the caller-provided current-task or
   substitute context.
4. Resolve that owner through ProcessDescriptorStore.
5. Read the DescriptorEntry from the owner's DescriptorTable.
6. Require DescriptorAccess readability.
7. Require DescriptorObjectKind::StdioInput for the first accepted slice.

Missing current owner, unknown owner, missing descriptor table, invalid
descriptor, empty descriptor, already closed descriptor, or non-readable
descriptor all return -EBADF. A readable descriptor whose object kind is not
StdioInput returns -ENOTSUP. The first implementation may use the same
target-independent owner/store fixture shape accepted by the write, close, and
dup syscall smokes. It must not claim process loading, PID allocation,
cross-CPU process registries, process exit teardown, or final object release.

## Fixed Proof Stdin Source

The first talos_read slice uses fixed proof input, not runtime-console0 or TTY
input. The later core task should supply a bounded stdin proof buffer and a
cursor alongside the process descriptor dispatch helper. Reads from fd 0 or a
duplicate of fd 0 consume from that shared proof cursor only after the
destination copy succeeds.

The readiness and EOF stance is deterministic:

| Source state | Return | Effect |
| --- | ---: | --- |
| remaining proof bytes and requested count > 0 | copied byte count | copies up to min(requested count, remaining bytes) and advances cursor |
| no remaining proof bytes | 0 | EOF; no user copy and no cursor change |
| requested count == 0 | 0 | no user copy and no cursor change |
| proof stdin state unavailable | -ENOTSUP | no descriptor or user-memory mutation |

Partial reads are accepted only as bounded proof-buffer exhaustion: if fewer
bytes remain than x2 requests, the read returns the available byte count after
copying those bytes. Short reads caused by interrupts, readiness changes,
canonical-line boundaries, signals, scheduler wakeups, or hardware input are
not accepted.

The first slice is always immediately ready when fixed proof bytes remain. It
does not define blocking, wait queues, EAGAIN, nonblocking descriptor flags,
poll/select readiness, terminal sessions, foreground process groups, Ctrl-D,
Ctrl-C, or signal restart.

## Copy-Out Contract

talos_read copies bytes to the x1 destination with the accepted copy_to_user
helper and UserAccessKind::Write. The copy is all-or-nothing for the selected
byte count:

| Case | Return | Effect |
| --- | ---: | --- |
| destination range is mapped writable user memory | byte count | selected proof bytes are copied and cursor advances |
| destination pointer is null, guarded, kernel, unmapped, non-writable, wrapping, or outside substitute backing | -EFAULT | no proof cursor advance and user memory unchanged |
| requested count exceeds DEFAULT_USER_COPY_LIMIT | -EFAULT | no proof cursor advance and user memory unchanged |
| kernel scratch buffer is smaller than selected byte count | -EINVAL | no proof cursor advance and user memory unchanged |

For requested count 0 or EOF, no copy_to_user call is required and x1 is not
used. For requested count greater than 0, the implementation must validate and
copy exactly the byte count it will return before advancing the proof cursor.
There is no partial user copy on EFAULT, no recoverable lower-EL data-abort
table, no per-thread errno storage, no demand paging, and no process-fatal
fault policy in this slice.

## Return And Error Contract

talos_read returns through the existing syscall x0 convention:

| Case | Return |
| --- | ---: |
| valid StdioInput descriptor, bytes available, valid destination | copied byte count |
| valid StdioInput descriptor and bounded EOF | 0 |
| requested count is 0 | 0 |
| fd outside capacity, empty, already closed, invalid, or non-readable | -EBADF |
| fd is readable but not StdioInput | -ENOTSUP |
| missing current owner, unknown owner, or missing table | -EBADF |
| invalid destination for a nonzero successful copy | -EFAULT |
| requested count exceeds DEFAULT_USER_COPY_LIMIT | -EFAULT |
| nonzero x3, x4, or x5 | -EINVAL |
| malformed kernel-side scratch/source state | -EINVAL or -ENOTSUP as applicable |
| any unaccepted syscall number | -ENOSYS |

The implementation must preserve accepted talos_nop, talos_write,
talos_close, talos_dup, unknown-syscall, descriptor-write fd/error,
close/dup lifetime, proof-only copy-probe quarantine, and diagnostic-marker
quarantine behavior. It must not reinterpret read failures as process-fatal
exceptions, per-thread errno storage, signal delivery, restart behavior, or
diagnostic marker completion.

## Proof And Validation Boundary

The next bounded task should be phase7-read-stdin-core-20260529.

That implementation task should add the smallest target-independent read
syscall dispatch surface using the current ProcessOwnerId-backed
ProcessDescriptorStore, DescriptorEntry::require_readable(), StdioInput object
matching, fixed proof stdin state, and copy_to_user(). It should fix the
syscall number and SyscallNumber variant, validate x3 through x5 as reserved
zero, convert x0 to a descriptor index, interpret x1/x2 as destination/count,
and return copied byte count, 0 EOF, or negative errno.

Focused unit tests should prove:

- read of fd 0 copies fixed proof bytes into writable UserData and advances
  the proof cursor only after a successful copy;
- reads may return a short count at proof-buffer exhaustion and then 0 at EOF;
- read of a duplicate of fd 0 shares the same StdioInput proof source;
- requested count 0 returns 0 without consuming proof input;
- fd 1, fd 2, invalid, closed, missing-owner, and unknown-owner cases return
  -EBADF without copying or consuming input;
- nonzero x3 through x5 return -EINVAL without copying or consuming input;
- invalid destination ranges and over-limit requests return -EFAULT without
  consuming input;
- readable non-StdioInput descriptors return -ENOTSUP if such a fixture is
  present;
- talos_nop, talos_write, talos_close, talos_dup, unknown-syscall, and
  proof-only copy-probe quarantine behavior remain unchanged.

Static inspection, cargo fmt, no_std unit tests, git diff --check, and mdbook
build are sufficient for the target-independent core. A later QEMU
read/stdin smoke is required before claiming lower-AArch64 runtime evidence,
and a later serialized Pi 5 proof is required before claiming physical
read/stdin behavior.

## Deferred Surfaces

This contract keeps the following blocked: QEMU read/stdin smoke, Pi 5
physical read/stdin proof, boot archive publication, runtime-console-backed
stdin, TTY canonical/raw stdin reads, hardware UART input, pipes, sockets,
regular files, directories, VFS/filesystem lookup, process loading,
fork/spawn/exec, descriptor inheritance across exec, close-on-exec
application, process exit teardown, wait/exit status, open-file-description
reference counting, object finalizers, file offsets, nonblocking flags,
poll/select readiness, wait queues, signals, restart semantics, path copying,
argv/envp loading, per-thread errno storage, libc/Rust std stdio, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
phase7-read-stdin-core-20260529.

That task should implement only the target-independent talos_read/stdin
dispatch surface and focused tests described above. It should not run QEMU,
acquire hardwareTestLock, publish a boot archive, run Pi 5 hardware, attach
stdin to runtime-console0 or TTY input, implement filesystem/pipes/sockets, or
advance process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, or a phase transition.

## Validation

- static inspection: reviewed the accepted read/stdin source inventory,
  accepted close and dup syscall contracts, src/syscall.rs, src/posix.rs,
  src/runtime_console.rs, src/tty.rs, and accepted write/close/dup task
  records and closeout docs.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
