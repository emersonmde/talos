# Phase 8 Read-Only Initramfs/VFS Contract

Status: accepted as the documentation-only Milestone 8.1 read-only
initramfs/VFS contract after the accepted
[Phase 8 Filesystem And Program Loading Source Inventory](phase8-filesystem-program-loading-source-inventory.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, VFS implementation,
initramfs parser, descriptor-backed filesystem read, ELF/program loader,
argv/envp setup, process creation, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

This contract defines the first filesystem boundary Talos should implement in
Phase 8: an immutable initial filesystem content model, a small VFS vocabulary,
lookup and read semantics, and deterministic fixture expectations. It does not
accept a runtime filesystem capability; the next tasks must still add the smoke
plan, target-independent core, QEMU/substitute evidence, and closeout evidence.

## Initramfs Image Shape

The first initramfs is a read-only, kernel-supplied fixture image. It is the
same content model whether supplied as a target-independent unit-test fixture,
compiled into the QEMU proof image, or later embedded into a Pi 5 boot archive.
The transport envelope for TFTP, firmware configuration, CPIO compatibility,
compression, and archive publication remains deferred.

The content model contains:

| Field | Contract |
| --- | --- |
| image identity | stable fixture name plus a deterministic byte digest recorded by the smoke or hardware task that consumes it |
| root | exactly one root directory, addressable as `/` and as the current directory for the first process fixture |
| entries | immutable directory entries with unique byte names after accepted path normalization |
| node kind | root directory, directory, or regular file only |
| metadata | node id, kind, byte length for regular files, and read-only permissions |
| file contents | immutable byte slices; zero-length files are valid |
| ordering | deterministic entry ordering for tests and diagnostics; duplicate names are invalid fixture construction |

The first fixture must include at least:

- `/etc/banner.txt` with non-empty ASCII contents;
- `/bin/init` as a regular-file byte payload used only as data in this slice;
- `/empty` as a zero-length regular file; and
- `/dir/nested.txt` to prove multi-component lookup.

`/bin/init` is not executable in this contract. It is a regular file whose
contents may be read by VFS tests; ELF validation, segment mapping, entry-state
setup, argv/envp, process creation, exec/spawn/wait, and user-thread launch
remain blocked.

## VFS Vocabulary

The first VFS vocabulary is target-independent:

- VFS root: the immutable root directory for the active initramfs image.
- VFS node: a stable handle to one root, directory, or regular-file object.
- Directory entry: a single normalized component name mapped to a child node.
- Metadata: node kind, file length, and read-only attributes.
- Lookup cursor: traversal state over normalized path components from root or
  the first process fixture current directory.
- Open file description: a kernel object containing a regular-file node,
  read-only access, and a byte offset shared by duplicated descriptors once a
  later descriptor integration task accepts that sharing.

Device nodes, pipes, sockets, symlinks, mount points, permissions beyond
read-only, credentials, timestamps, link counts, inode persistence, writable
files, unlink/rename, directory mutation, and final object release remain
vocabulary for later contracts only.

## Path Copy And Lookup

Path bytes enter lookup through the accepted user-copy and lexical
normalization contracts:

1. A path-taking syscall or substitute helper must copy the complete path
   string from user memory before lookup. If that copy fails, the operation
   returns EFAULT and performs no VFS traversal or descriptor mutation.
2. NUL termination, counted length, and maximum copied byte count must be fixed
   by the later path-taking syscall contract. This VFS contract consumes an
   already-copied byte path.
3. `normalize_path()` remains the authority for slash folding, dot and dot-dot
   handling, NUL rejection, path length, component length, component count,
   absolute versus current-working-directory start, and trailing slash
   directory-required state.
4. The first process fixture resolves both absolute paths and
   current-working-directory-relative paths against the initramfs root. A real
   mutable current working directory, chdir, per-process root, and mount
   namespace remain deferred.
5. Lookup walks normalized components against directory entries. Every
   non-final component must resolve to a directory. A final regular file,
   directory, or missing component is handled by the operation-specific error
   table below.

Symlink expansion is not supported. If a later fixture introduces a symlink
node before a symlink contract exists, operations that encounter it must return
ENOTSUP without following it.

## Read And Open-File Semantics

The first accepted operation is descriptor-facing read from a regular file,
implemented later through an internal open-file-description fixture. It is not
yet a talos_open syscall or a filesystem-backed descriptor syscall claim.

Regular-file reads follow these rules:

| Case | Return | Effect |
| --- | ---: | --- |
| offset < file length and count > 0 | copied byte count | copies up to min(count, remaining bytes) and advances offset by copied bytes |
| offset == file length | 0 | EOF, no offset change |
| requested count == 0 | 0 | no copy and no offset change |
| invalid user destination for a nonzero copy | -EFAULT | no offset change and no partial user copy |
| descriptor does not name a readable regular-file open description | -EBADF or -ENOTSUP | no copy and no offset change |
| malformed kernel-side fixture state | -EINVAL | no copy and no offset change |

All user copies must preserve the accepted all-or-nothing copy_to_user()
behavior. File offsets advance only after the selected bytes have been
successfully copied. Reads may return a short count only at fixture EOF.
Interrupts, readiness changes, blocking, nonblocking flags, signals, scheduler
wakeups, page faults, demand paging, and process-fatal filesystem faults are
not accepted.

Directory lookup and metadata may succeed, but directory reads and directory
iteration remain separate contracts. Opening or reading a directory as a
regular file returns EISDIR. Directory listing for diagnostics may be planned
as a later smoke surface only after the smoke plan defines exact output lines.

## Error Precedence

The first read-only initramfs/VFS implementation must use this deterministic
error order when multiple failures are possible:

1. Invalid reserved arguments or malformed kernel fixture inputs: EINVAL.
2. Invalid descriptor number, missing descriptor, closed descriptor, or missing
   current descriptor namespace for a descriptor-facing operation: EBADF.
3. User path or destination copy failure for an operation that requires that
   copy: EFAULT, before lookup or file offset mutation.
4. Lexical path normalization failures: ENAMETOOLONG for path length,
   component length, or component-count overflow; EINVAL for embedded NUL or
   malformed path bytes if the copied-path contract assigns that case to
   EINVAL.
5. Empty path after copying: ENOENT.
6. Non-final component missing: ENOENT.
7. Non-final component resolves to a regular file: ENOTDIR.
8. Final component missing: ENOENT.
9. Trailing slash or directory-required path resolves to a regular file:
   ENOTDIR.
10. Operation requires a regular file but final node is a directory: EISDIR.
11. Operation is recognized but intentionally unsupported in this slice:
    ENOTSUP.

Unsupported write flags, filesystem mutation, executable loading, directory
iteration, symlink traversal, mount traversal, persistent-storage operations,
and device-backed operations return ENOTSUP when they reach this boundary.

## Fixture And Smoke Expectations

The QEMU/substitute smoke plan must define one deterministic scenario using the
same fixture content model as the unit tests. The plan should specify exact
PASS/classification lines and evidence paths before any QEMU execution occurs.

The smoke should prove, at minimum:

- lookup of `/etc/banner.txt` reaches a regular file with the expected length;
- reading `/etc/banner.txt` returns the expected bytes and advances the offset;
- reading `/empty` returns 0 without mutation;
- lookup of `/dir/nested.txt` proves multi-component traversal;
- missing file, not-directory, is-directory, name-too-long, and unsupported
  operation cases return the expected errno; and
- existing scalar syscall, descriptor-write, close, dup, and read/stdin smoke
  frontiers are not regressed if the accepted smoke plan requires those gates.

The first Pi 5 hardware proof is blocked. A later hardware plan must define
candidate image identity, initramfs/archive identity, TFTP or embedded-image
evidence, fresh serial cursor, restore proof, inconclusive-run triage, and
exact PASS/classification lines before hardwareTestLock may be acquired.

## Deferred Surfaces

This contract keeps the following blocked: ELF/program loading, executable
permission checks, segment validation, zero-fill, argv/envp setup, process
creation, PID allocation, exec/spawn/wait, process-owned address-space
installation, mutable current working directory, chdir, per-process root,
mount namespaces, writable filesystems, persistent storage, directory
iteration, open syscall ABI, close-on-exec enforcement, descriptor inheritance
across exec, open-file-description final release, symlinks, device nodes,
pipes, sockets, nonblocking I/O, poll/select readiness, scheduler wait queues,
signals, per-thread errno storage, runtime-console0/TTY/hardware stdin,
libc/Rust std filesystem support, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, boot archive publication,
and Pi 5 hardware proof.

## Recommended Next Tasks

The next mechanically unblocked Milestone 8.1 task should be
`phase8-readonly-initramfs-vfs-smoke-plan-20260530`.

That task should define the exact QEMU/substitute smoke fixture, expected
lookup/read/error observations, PASS/classification lines, retained log paths,
and regression gates. It should not implement the VFS core, run QEMU, publish
a boot archive, acquire hardwareTestLock, or run Pi 5 hardware.

The dependency-gated target-independent core task should be
`phase8-readonly-initramfs-vfs-core-20260530` after the contract and smoke plan
are accepted. That task should implement only the read-only initramfs/VFS object
model, lookup, regular-file reads, and focused unit tests described here. It
must not claim QEMU/substitute runtime evidence or physical Pi 5 behavior until
later explicit tasks accept those gates.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation review: reviewed the accepted Phase 8 source inventory,
  roadmap, ADR index, accepted POSIX path/error vocabulary, read/stdin
  contract, process descriptor table contract, and `src/posix.rs` ownership
  markers for errors, path normalization, and descriptor object kinds.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required because this task changes only
  Markdown documentation and durable worker state.
