# Phase 8 Filesystem And Program Loading Source Inventory

Status: accepted source inventory for
phase8-filesystem-program-loading-source-inventory-20260530.

## Scope

This documentation-only inventory starts Phase 8 after the accepted
[Phase 7 Final Closeout Checkpoint](phase7-final-closeout-checkpoint.md)
set the durable recommendation flag. It maps the source owners, accepted input
contracts, missing contracts, and smallest next task for the first
filesystem/program-loading slice.

It adds no Rust or assembly behavior, runs no QEMU scenario, performs no
Raspberry Pi 5 hardware action, publishes no boot archive, and acquires no
hardwareTestLock. It does not accept VFS lookup, filesystem-backed I/O,
program loading, ELF parsing, argv/envp setup, process creation, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
or any Phase 8 runtime capability.

## Source Owners

### POSIX Paths And Errors

- `src/posix.rs::PosixError` owns the accepted errno-style vocabulary used by
  filesystem and loader contracts: ENOENT, ENOEXEC, EBADF, EFAULT, EACCES,
  ENOMEM, ENOTDIR, EISDIR, EINVAL, ENAMETOOLONG, ENOSYS, ENOTEMPTY, and
  ENOTSUP are already named, while numeric syscall mappings exist only for the
  accepted syscall subset.
- `src/posix.rs::normalize_path()`, `NormalizedPath`, `PathStart`, and
  `PathLimits` own target-independent lexical path normalization. They accept
  slash-separated byte paths, absolute versus current-working-directory start,
  dot and dot-dot handling, directory-required trailing slash vocabulary,
  length limits, component limits, and NUL rejection.
- `DEFAULT_PATH_LIMITS` fixes the current local defaults: 4096-byte paths,
  255-byte components, and 64 normalized components.

Accepted input contract: path normalization is pure and does not perform VFS
lookup, mount traversal, symlink expansion, permission checks, filesystem I/O,
or current-working-directory storage.

Gap: Phase 8 needs a contract for copying path strings from user memory,
terminating versus counted path inputs, current-working-directory/root handles,
lookup error precedence, trailing-slash behavior during object traversal,
symlink deferral, and how lexical paths feed VFS lookup without duplicating
normalization rules.

### User Memory And Address-Space Inputs

- `src/posix.rs::UserMapping`, `UserRange`, `UserMappingPermissions`,
  `UserAccessKind`, `validate_user_memory_access()`, `copy_from_user()`,
  and `copy_to_user()` own the accepted all-or-nothing user-copy helpers.
- `USER_NULL_GUARD_END`, `USER_ADDRESS_SPACE_END`, and
  `DEFAULT_USER_COPY_LIMIT` own the current user-range guard and copy limit
  vocabulary.
- The QEMU and Pi 5 syscall/copy proofs provide substitute backing stores for
  user-memory evidence, not live process address spaces.

Accepted input contract: user-copy helpers validate complete ranges before
side effects and map rejected user ranges to EFAULT.

Gap: Phase 8 needs contracts for process-owned address-space lookup, loader
mapping ownership, page-table-backed user buffers, argv/envp memory layout,
path-string copy helpers, partial read/write policy once filesystem objects
exist, and whether loader faults are process-fatal or recoverable syscall
errors.

### Descriptor Tables And Runtime I/O

- `src/posix.rs::DescriptorTable`, `DescriptorEntry`,
  `DescriptorObject`, `DescriptorObjectKind`, `DescriptorFlags`,
  `ProcessDescriptorOwner`, and `ProcessDescriptorStore` own the accepted
  fixed-capacity inherited-stdio descriptor frontier.
- `DescriptorObjectKind` already reserves vocabulary for regular files,
  directories, pipe endpoints, sockets, devices, and other kernel objects.
- `src/syscall.rs::dispatch_process_descriptor()` and
  `dispatch_process_descriptor_with_fixed_stdin()` own the accepted
  process-owner-backed write, close, dup, and fixed-proof-stdin read paths.

Accepted input contract: stdout/stderr writes, close, dup, and fixed-proof
stdin reads are accepted only for focused target-independent, QEMU/substitute,
and Pi 5 proof frontiers.

Gap: Phase 8 needs contracts for open file descriptions, object lifetime,
reference counting, descriptor inheritance across exec, close-on-exec,
filesystem-backed read semantics, directory handles, file offsets,
seekability, final release, and how VFS objects become descriptor objects.

### Process Identity And Loader Ownership

- `src/scheduler.rs::TaskId`, `Task`, and `ProcessOwnerId` own the
  accepted separation between scheduler tasks and future POSIX process owners.
- `Task::attach_process_owner()`, `Task::process_owner()`, and scheduler
  snapshots preserve a process-owner placeholder without accepting a process
  table or PID allocator.
- The Phase 7 POSIX baseline defines process, PID, parent/child, spawn, exec,
  exit, wait, zombie, and reaped vocabulary without implementing those
  mechanisms.

Accepted input contract: a process owner may be attached to a task and used to
look up a descriptor table in proof scenarios. It is not a PID, process-table
entry, or loader-owned address-space object.

Gap: Phase 8 needs contracts for the first process object, PID or stable owner
identity, current-process lookup from syscall and loader paths, initial process
creation, address-space ownership, user-thread entry state, exit/failure
reporting for loader errors, and descriptor inheritance when a process image is
installed.

### Executable Images And Program Loading

- No source module currently owns an ELF parser, executable image abstraction,
  program-header validation, user image mapper, argv/envp stack builder, or
  exec/spawn entry point.
- Existing lower-EL QEMU and Pi 5 proof payloads in `src/target/qemu_virt.rs`,
  `src/target/rpi5.rs`, and `src/arch/aarch64/vectors.S` are built-in
  proof payloads, not loaded programs.
- `src/memory_map/translation.rs`, `src/memory_map/layout.rs`, and the
  accepted EL0/user-memory contracts provide vocabulary that a later loader can
  use, but they do not currently own process image construction.

Accepted input contract: Phase 7 proves lower-EL trap/syscall mechanics with
built-in payloads only.

Gap: Phase 8 needs contracts for executable format selection, ELF acceptance
rules or an explicit non-ELF early format, segment permissions, zero-fill,
alignment, entry-point validation, user stack layout, argv/envp placement,
auxiliary vector deferral, interpreter/dynamic-linker deferral, and how a
loaded image becomes the current process program.

### VFS, Filesystem Images, And Boot/Test Scenarios

- No source module currently owns VFS nodes, dentries, inodes, mount roots,
  initramfs archives, regular-file contents, directory iteration, device nodes,
  or filesystem image publication.
- `src/target/qemu_virt.rs` and `src/target/rpi5.rs` own the existing
  focused proof scenarios and PASS/classification output conventions.
- The lab-controller/TFTP evidence discipline from Phase 7 remains the standard
  for later physical claims, but this inventory performs no hardware action.

Accepted input contract: QEMU/substitute tests should carry pure filesystem
and loader contracts first; serialized Pi 5 runs should be reserved for the
smallest physical claim that cannot be proven locally.

Gap: Phase 8 needs contracts for a read-only initial filesystem image,
directory metadata, lookup by normalized path, regular-file byte reads,
error mapping, deterministic test fixtures, QEMU smoke ownership, archive
identity, and later Pi 5 restore requirements. Writable filesystems,
persistent storage, block devices, RP1/PCIe, DMA/cache policy, shell-driven
commands, and networking remain deferred.

## Missing Contract Map

| Area | Already accepted input | Missing before implementation |
| --- | --- | --- |
| VFS/filesystem objects | POSIX path/error vocabulary and descriptor object-kind tags. | Node/inode/open-file-description model, root/current-directory handles, directory versus regular-file behavior, lookup errors, file offsets, and object lifetime. |
| Path copying | User-copy helpers and lexical path normalization. | NUL-terminated or counted user path policy, maximum copied length, EFAULT ordering, ENOENT/EINVAL precedence, and CWD/root resolution. |
| Program image format | Lower-EL entry/trap mechanics and user-memory permission vocabulary. | ELF or early image format contract, segment validation, page permissions, zero-fill, entry-point checks, and loader error mapping. |
| Address-space setup | User range/mapping validation and accepted lower-EL proof mappings. | Process-owned page-table/image layout, stack placement, argv/envp storage, guard regions, and teardown on loader failure. |
| Descriptor inheritance | ProcessDescriptorStore, inherited stdio, close, dup, read/write proofs. | open/close-on-exec inheritance policy, file-backed descriptors, offset sharing, final release, and exec preservation rules. |
| Process identity | ProcessOwnerId placeholder attached to scheduler tasks. | Process table, initial process creation, current-process lookup, PID policy, parent/child deferral, and process-fatal loader failure handling. |
| Boot/test scenarios | QEMU/Pi 5 proof conventions and hardware evidence discipline. | Deterministic initramfs fixture, QEMU filesystem smoke, later QEMU loader smoke, archive identity, and Pi 5 proof/restore requirements. |

## Recommended Next Task

The next bounded task should be
`phase8-readonly-initramfs-vfs-contract-20260530`, documentation-only under
Milestone 8.1.

That contract should define:

- the read-only initial filesystem image shape and how it is supplied to QEMU
  and, later, Pi 5 boot artifacts;
- VFS object vocabulary for root, directories, regular files, metadata,
  lookup, open file descriptions, and descriptor-facing reads;
- how accepted lexical paths and user path copies feed lookup;
- deterministic errors for not-found, not-directory, is-directory,
  name-too-long, unsupported operation, bad descriptor, and fault cases;
- the smallest QEMU/substitute smoke that lists or reads fixed fixture files;
  and
- which loader surfaces remain blocked until a later program-loading contract.

The contract must keep ELF/program loading, argv/envp setup, process creation,
exec/spawn/wait, shell, writable filesystems, persistent storage, networking,
SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and Pi 5
hardware proof blocked until later explicit tasks accept their contracts and
validation gates.

## Validation

- static inspection: git status --short before edits was clean.
- static source review: inspected src/posix.rs, src/syscall.rs,
  src/scheduler.rs, src/runtime_console.rs, src/tty.rs,
  src/arch/aarch64/exceptions.rs, src/target/qemu_virt.rs,
  src/target/rpi5.rs, and accepted Phase 7 closeout/source-inventory docs.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this inventory.
