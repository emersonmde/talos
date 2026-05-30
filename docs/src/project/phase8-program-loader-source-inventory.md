# Phase 8 Program Loader Source Inventory

Status: accepted source inventory for
phase8-program-loader-source-inventory-20260530.

## Scope

This documentation-only inventory follows the accepted
[Phase 8 Read-Only Initramfs/VFS Closeout Checkpoint](phase8-readonly-initramfs-vfs-closeout-checkpoint.md).
It maps the source owners, accepted inputs, missing contracts, and next bounded
task for loading an executable image from the accepted read-only initramfs/VFS
fixture.

It adds no Rust or assembly behavior, runs no QEMU scenario, performs no
Raspberry Pi 5 hardware action, publishes no boot archive, and acquires no
hardwareTestLock. It does not accept an ELF parser, executable /bin/init,
user page mapping, process creation, exec/spawn/wait, argv/envp setup, shell,
networking, SSH, writable filesystem, persistent storage, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Accepted Loader Inputs

The loader can now rely on one input boundary: the accepted read-only
initramfs/VFS fixture can expose immutable regular-file bytes through
target-independent lookup and read helpers.

- `src/initramfs.rs::PHASE8_INIT_PATH` names `/bin/init` and
  `PHASE8_INIT_BYTES` provides its current data payload.
- `ReadOnlyInitramfs::lookup_default()` and `open_regular_file()` prove that a
  normalized path can resolve to a regular-file object or deterministic
  POSIX-style error.
- `ReadOnlyInitramfs::read_regular_file()` and `read_descriptor()` prove
  all-or-nothing byte reads into accepted user-copy substitute mappings.
- The accepted QEMU/substitute smoke retained the fixture identity
  `phase8-readonly-initramfs-vfs-v1` and proved lookup/read/error behavior in
  `tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log`.

This is not yet a production executable source. `/bin/init` is still data only:
no executable format has been accepted, no descriptor-backed open/read syscall
can feed a loader, and no process image can be installed.

## Source Owners

### Executable Bytes

- `src/initramfs.rs` owns the only accepted filesystem bytes. The loader can
  later consume a regular-file byte slice or bounded byte reader derived from
  `ReadOnlyInitramfs`, but the current APIs are fixture-owned and
  target-independent.
- `src/posix.rs::normalize_path()` and `DEFAULT_PATH_LIMITS` remain the path
  authority before any loader path lookup.
- `src/target/qemu_virt.rs::run_readonly_initramfs_vfs_smoke()` owns the
  current QEMU/substitute proof convention for fixture identity and file-read
  evidence.

Gap: no loader-owned byte source trait or image object exists. A later
contract must decide whether the first loader consumes a whole regular file,
a bounded reader, or an already materialized byte slice. It must keep
descriptor-backed production open/read syscalls deferred unless a separate
filesystem syscall task accepts them.

### Image Format And Validation

- No source module currently owns ELF parsing, executable image headers,
  program-header iteration, interpreter handling, relocation policy, or image
  digest reporting.
- `src/posix.rs::PosixError` already names `ENOEXEC`, `EACCES`, `ENOMEM`,
  `EFAULT`, `EINVAL`, `ENOENT`, `ENOTDIR`, `EISDIR`, `ENAMETOOLONG`, and
  `ENOTSUP`, but `src/syscall.rs::errno_number()` maps only the accepted
  syscall subset today.
- Existing built-in lower-EL proof payloads in `src/target/qemu_virt.rs`,
  `src/target/rpi5.rs`, and `src/arch/aarch64/vectors.S` are not loaded
  executable images.

Gap: the next contract must choose the first executable format policy. The
mechanically smallest durable choice is a narrow static ELF64/AArch64 subset
unless that contract proves a smaller temporary format is safer. It must define
magic/class/data/machine/type checks, loadable program-header limits, alignment,
overflow handling, file-size versus memory-size rules, unsupported dynamic
interpreter rejection, and deterministic error mapping.

### Segment And User-Memory Mapping

- `src/posix.rs::UserMapping`, `UserRange`, `UserMappingPermissions`, and
  `validate_user_memory_access()` own the accepted target-independent user
  range and permission vocabulary.
- `docs/src/project/phase7-el0-trap-address-space-contract.md` owns the
  lower-EL vocabulary for `UserText`, `UserData`, `UserHeap`, `UserStack`,
  `UserGuard`, kernel-only mappings, entry ELR validation, and stack alignment.
- `src/memory_map/translation.rs` and `src/memory_map/layout.rs` own current
  kernel translation/layout helpers, not process-owned page tables.
- `src/target/qemu_virt.rs` and `src/target/rpi5.rs` own scenario-local EL0
  proof mappings, static payload pages, and proof stacks.

Gap: no process address-space object, user-frame allocator, page-table
installer, segment permission mapper, zero-fill owner, or teardown path exists.
The contract must separate pure validation from later memory installation and
must define accepted user VA ranges, page alignment, W^X permissions,
read-only text, writable data, BSS zero-fill, and rejection when a segment
overlaps guard, stack, kernel, MMIO, or another segment.

### Entry State And User Stack

- `src/arch/aarch64/exceptions.rs::ExceptionFrame` can record lower-AArch64
  trap state, and `try_route_lower_aarch64_syscall()` owns stable SVC routing.
- `src/scheduler.rs::ContextFrame` owns kernel-task context fields. It is not a
  user-thread initial state.
- The Phase 7 EL0/address-space contract requires a validated user ELR, a
  writable 16-byte-aligned user SP, and process-fatal handling for user
  instruction/data/stack faults.

Gap: no initial user register frame, user stack builder, argv/envp layout,
auxiliary vector policy, TLS policy, or failure-unwind rule exists. The next
contract should state that argv/envp are deferred or set to a minimal empty
layout, and it must keep shell and libc startup compatibility out of scope.

### Process Ownership And Install Boundary

- `src/scheduler.rs::Task`, `TaskId`, and `ProcessOwnerId` preserve a
  scheduler task versus future POSIX process-owner distinction.
- `Task::attach_process_owner()` and `Task::process_owner()` can carry a
  `ProcessOwnerId` placeholder, but not a process table, PID, executable image,
  address-space pointer, exit status, or wait target.
- `src/posix.rs::ProcessDescriptorStore` owns inherited-stdio descriptor
  tables for proof scenarios.

Gap: no process object owns the loaded address space, descriptor table,
current working directory, root directory, credentials, parent/child relation,
or exit state. The next contract must define loader outputs as an image
validation/mapping result and explicitly leave process creation,
exec/spawn/wait, PID allocation, and scheduler installation to later tasks.

### Descriptor Inheritance

- `src/posix.rs::DescriptorTable`, `DescriptorEntry`, `DescriptorFlags`, and
  `DescriptorObjectKind` already name inherited stdio, regular files,
  directories, close-on-exec, and other object classes.
- Accepted Phase 7 descriptor work proves inherited stdio, write, close, dup,
  and fixed proof-stdin read boundaries. It does not make filesystem-backed
  descriptors production-ready.
- `src/initramfs.rs::read_descriptor()` can read a regular-file object through
  a target-independent descriptor fixture, but that helper is not wired to
  production syscall dispatch.

Gap: the loader contract must decide whether it needs descriptor inheritance
only as an input to a later process object or whether close-on-exec policy is
deferred entirely. It must not accept open-file-description final release,
file-backed descriptor syscalls, directory descriptors, seek, or current
working directory mutation.

### Error Mapping And Observability

- `src/posix.rs::PosixError` names the error vocabulary the loader can reuse.
- `src/syscall.rs::SyscallReturn` can encode accepted errno values only for
  the current syscall subset; several loader-relevant errors are not mapped
  through syscall return values yet.
- Existing QEMU/Pi 5 proof scenarios use exact PASS/classification lines for
  bounded claims.

Gap: the next contract must map loader failures before implementation. At a
minimum, missing path and traversal errors should come from VFS lookup, invalid
or unsupported executable images should be deterministic `ENOEXEC` or
`ENOTSUP` cases, permission/range violations should be deterministic `EACCES`
or `EINVAL` cases, and allocation failures should be `ENOMEM`. It must also
define the QEMU/substitute observations a later smoke would print.

## Missing Contract Map

| Area | Accepted input | Missing before implementation |
| --- | --- | --- |
| Executable bytes | Read-only initramfs/VFS fixture regular-file data and retained QEMU/substitute proof. | Loader-owned byte-source shape, whole-file limit, digest/identity reporting, and whether `/bin/init` is the first test path. |
| Executable format | Lower-EL payload mechanics and POSIX error vocabulary. | ELF64/AArch64 or smaller early format policy, header validation, program-header matrix, unsupported dynamic-linker handling, and deterministic rejects. |
| Segment mapping | User range/permission vocabulary and scenario-local EL0 proof pages. | Process-owned address-space object, page allocation, page-table installation, segment permission policy, W^X, overlap checks, and teardown. |
| Zero-fill | No accepted loader owner. | `p_memsz > p_filesz` policy, BSS zeroing, overflow checks, and out-of-memory behavior. |
| Entry state | Lower-EL trap/return invariants and exception-frame capture. | Entry-point validation, initial SPSR/ELR/SP/register state, empty argv/envp or deferred argv/envp policy, and user stack layout. |
| Process install | `ProcessOwnerId` placeholder and scheduler task vocabulary. | Process table/PID policy, current-process lookup, image ownership, scheduler handoff, exec/spawn/wait, and failure reporting. |
| Descriptor inheritance | Inherited-stdio descriptor store and close-on-exec vocabulary. | Exec inheritance rules, close-on-exec enforcement, filesystem-backed descriptors, open-file lifetime, and final release. |
| Loader errors | POSIX error names and current syscall return encoding. | Loader-specific error matrix and any additional errno-number mappings required by a later public syscall boundary. |
| Evidence | QEMU/substitute PASS/classification conventions. | Loader fixture image identity, success and negative output lines, retained log path, and hardware proof deferral rules. |

## Recommended Next Task

The next bounded task should be
`phase8-program-loader-format-contract-20260530`, documentation-only under
Milestone 8.3.

That contract should define:

- the first executable image policy, preferably a narrow static ELF64/AArch64
  subset unless a smaller temporary format is explicitly justified;
- header and program-header validation, loadable segment permissions,
  alignment, user-address ranges, zero-fill, entry-point validation, and
  deterministic rejection cases;
- how the accepted read-only initramfs/VFS regular file provides image bytes
  without accepting descriptor-backed production open/read syscalls;
- which process/address-space/stack/descriptor inputs are required versus
  deferred; and
- the next bounded task, likely a QEMU/substitute loader smoke plan if the
  format contract fully closes the implementation prerequisites.

The contract must keep process creation, exec/spawn/wait, argv/envp stack
implementation, shell behavior, Pi 5 hardware proof, writable filesystems,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy blocked until later explicit tasks accept their gates.

## Validation

- static inspection: git status --short before edits was clean.
- static source/documentation review: inspected the accepted read-only
  initramfs/VFS closeout, Phase 8 source inventory and contract, Phase 7
  EL0/address-space and process-descriptor contracts, src/initramfs.rs,
  src/posix.rs, src/syscall.rs, src/scheduler.rs, src/memory_map/layout.rs,
  src/memory_map/translation.rs, src/arch/aarch64/exceptions.rs,
  src/target/qemu_virt.rs, src/target/rpi5.rs, roadmap, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this inventory.
