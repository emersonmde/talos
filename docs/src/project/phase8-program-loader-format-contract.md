# Phase 8 Program Loader Format Contract

Status: accepted documentation-only contract for
phase8-program-loader-format-contract-20260530.

## Scope

This contract follows the accepted
[Phase 8 Program Loader Source Inventory](phase8-program-loader-source-inventory.md).
It defines the first executable image subset, validation matrix, segment
permission policy, zero-fill behavior, entry-point rules, and loader error
mapping Talos should implement before any loader core exists.

It adds no Rust or assembly behavior, runs no QEMU scenario, performs no
Raspberry Pi 5 hardware action, publishes no boot archive, and acquires no
hardwareTestLock. It does not accept process creation, exec/spawn/wait,
argv/envp stack implementation, shell behavior, writable filesystem,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Accepted Executable Subset

The first program-loader implementation must accept only a narrow static ELF64
AArch64 executable image. A smaller private format would reduce parser surface,
but it would also create a second executable contract that must be replaced
before normal userland work. The durable minimum is therefore the smallest ELF
subset that can load one static /bin/init fixture from the accepted read-only
initramfs/VFS regular-file boundary.

Accepted ELF identity:

| Field | Accepted value |
| --- | --- |
| magic | 0x7f, E, L, F |
| class | ELFCLASS64 |
| data | ELFDATA2LSB |
| version | EV_CURRENT |
| osabi | System V or unspecified, treated identically for this slice |
| type | ET_EXEC |
| machine | EM_AARCH64 |
| entry | canonical user virtual address inside loaded executable text |
| headers | ELF header plus program headers entirely inside the regular-file bytes |
| program headers | at least one PT_LOAD; all non-PT_LOAD headers ignored unless explicitly rejected below |

The first loader must reject dynamic and compatibility features:

- ET_DYN, ET_REL, ET_CORE, non-AArch64 machines, 32-bit ELF, big-endian ELF,
  unsupported ELF versions, malformed header sizes, and wrapped header ranges;
- PT_INTERP, PT_DYNAMIC, relocation-bearing images, shared libraries,
  position-independent executable requirements, and any image that needs a
  dynamic linker;
- TLS, notes, GNU property, stack-exec, build-id, and other metadata as
  loader inputs. They may be ignored only when they do not change the accepted
  load semantics and do not require side effects;
- section headers, symbol tables, debug tables, and string tables as runtime
  authority. The first loader is program-header driven only.

## Header Validation

Validation is pure and precedes any address-space mutation. The loader consumes
immutable regular-file bytes obtained from the accepted read-only initramfs/VFS
fixture, but it does not accept descriptor-backed production open/read syscalls.

The loader must check:

1. The regular-file byte length is nonzero and at most the first loader image
   limit selected by the implementation task.
2. The ELF header range is present and all fixed identity fields match the
   accepted subset.
3. e_ehsize, e_phentsize, e_phoff, and e_phnum describe a program header table
   fully contained in the file without integer overflow.
4. At least one PT_LOAD segment exists.
5. Every PT_LOAD segment has p_filesz <= p_memsz, nonzero p_memsz, and file
   and memory ranges that do not wrap.
6. PT_LOAD virtual ranges are page-compatible, sorted or otherwise checked for
   overlap, and entirely inside accepted user address space.
7. Unsupported mandatory program-header types are rejected before any segment
   install. Optional ignored headers must not affect the load result.

Validation output is a loader-owned image plan. It records the source path,
source byte length or digest, entry point, ordered loadable segments, computed
memory ranges, permissions, zero-fill ranges, and deterministic failure reason.
It is not a process object and does not own scheduler state.

## Segment Policy

The first segment vocabulary uses the accepted Phase 7 user-memory names:
UserText, UserData, UserHeap, UserStack, UserGuard, and KernelMapping.

PT_LOAD segment flags map as follows:

| ELF flags | Accepted user mapping |
| --- | --- |
| R+X, not W | UserText: readable and executable, never writable |
| R+W, not X | UserData: readable and writable, never executable |
| R only | UserData-style read-only data mapping, not executable |
| W without R | rejected as invalid for the first loader |
| W+X | rejected to preserve W^X |
| X without R | rejected until executable-only mapping policy exists |

Every loadable segment must satisfy these invariants:

- The virtual address range is canonical, below
  0x0000_8000_0000_0000, and outside the null guard
  0x0000_0000_0000_0000..0x0000_0000_0001_0000.
- It does not overlap UserGuard, future UserStack, kernel mappings, MMIO,
  bootstrap tables, kernel stacks, DTB data, or another segment.
- It does not require user access to kernel-only physical memory.
- It is aligned so a later page-table installer can apply the requested
  permissions without granting broader write or execute access than the image
  plan allows.
- If the implementation must round to page boundaries, the rounded area must
  still remain inside the accepted user range and must not merge incompatible
  permissions.

The first loader may choose one fixed low userspace window for fixture images
as long as that window is stricter than the Phase 7 canonical user range. It
must not grow the accepted range or allow kernel addresses without a new
contract.

## Zero-Fill And File Bytes

For each PT_LOAD segment:

- bytes [p_offset, p_offset + p_filesz) are copied from the immutable
  regular-file bytes into the planned user mapping;
- bytes [p_vaddr + p_filesz, p_vaddr + p_memsz) are zero-filled;
- p_memsz == p_filesz has no zero-fill range;
- p_filesz == 0 is valid only for a non-executable writable data/BSS segment
  with nonzero p_memsz;
- zero-fill must be explicit in the image plan so a later implementation can
  prove no stale kernel or fixture bytes leak into user memory.

The loader must reject any segment whose file byte range falls outside the
regular file, whose memory range wraps, whose zero-fill range crosses a mapping
boundary that cannot preserve permissions, or whose combined image exceeds the
implementation task's bounded memory budget.

## Entry State Boundary

The entry point is accepted only when it:

- is a canonical user virtual address outside the null guard;
- lies inside a loaded UserText range;
- satisfies AArch64 instruction alignment;
- does not point into a writable or non-executable segment; and
- can be represented in the future lower-EL frame as the initial ELR.

This contract does not build the initial user register frame. It requires only
that a later process-loader boundary have enough validated data to create one:
entry ELR, initial SP policy, SPSR/PSTATE policy, x0/x1 argument convention,
and image metadata.

The first argv/envp policy remains deferred. A later task may choose an empty
argv/envp stack, a kernel-provided /bin/init argv, or another minimal layout,
but this format contract does not accept stack bytes, auxiliary vectors, TLS,
libc startup compatibility, or shell argument parsing.

## Process And Descriptor Boundary

The loader validates an image and produces an image plan. It does not create a
process, attach a ProcessOwnerId, allocate a PID, install a descriptor table,
set current working directory or root, mutate scheduler tasks, or perform
exec/spawn/wait.

Required inputs for a later process-install task:

- a validated image plan from this loader contract;
- a process-owned address-space installer that can allocate user frames, copy
  file bytes, zero BSS, install page permissions, and unwind on failure;
- an initial user stack policy;
- descriptor inheritance policy, including whether close-on-exec is enforced
  in the first process boundary; and
- a scheduler handoff policy that distinguishes TaskId from future PID/process
  identity.

Descriptor-backed filesystem reads remain out of scope. The accepted read-only
initramfs/VFS regular file may supply bytes directly to the loader through a
kernel fixture boundary; production open, read, directory iteration, seek,
close-on-exec finalization, and open-file-description release must be accepted
by later tasks.

## Error Mapping

The first loader error vocabulary is deterministic and POSIX-shaped, but it is
not yet a public exec syscall ABI. If a later syscall boundary exposes these
errors, it must first add any missing numeric errno mappings.

| Failure class | Error |
| --- | --- |
| VFS lookup reports missing path | ENOENT |
| Lookup crosses non-directory component | ENOTDIR |
| Selected path resolves to a directory | EISDIR |
| User-supplied path copy fails before lookup | EFAULT |
| Path normalization limit failure | ENAMETOOLONG or EINVAL, matching the path contract |
| ELF magic/class/data/version/type/machine mismatch | ENOEXEC |
| Malformed ELF/header/program-header range or overflow | ENOEXEC |
| Dynamic/interpreter/relocation feature required | ENOTSUP |
| Segment maps outside accepted user range or overlaps blocked ranges | EACCES |
| Invalid segment flags, W+X, or unsupported permissions | EACCES |
| Entry point outside loaded executable text | ENOEXEC |
| Image exceeds file, segment, or memory budget | ENOMEM when allocation-sized, otherwise ENOEXEC for malformed image |
| Kernel-side malformed loader request | EINVAL |

No partial image install is observable on failure. If a later implementation
allocates frames or builds page tables while realizing an accepted image plan,
failure unwind and teardown are responsibilities of that implementation task,
not this format contract.

## Evidence And Next Task

The next bounded task should be
phase8-qemu-program-loader-smoke-plan-20260530, documentation-only under
Milestone 8.3, if the durable queue dependencies remain satisfied.

That smoke plan should define:

- the exact fixture image identity, likely replacing the current
  not-executable-yet /bin/init bytes only in a later implementation task;
- success observations for parsing a narrow static ELF64/AArch64 fixture,
  entry validation, segment permission classification, and zero-fill planning;
- deterministic negative cases for malformed magic, unsupported dynamic
  interpreter, W+X segment, out-of-user-range segment, overlap, bad entry, and
  file-range overflow;
- retained QEMU/substitute evidence path, PASS line, and classification line;
  and
- conditional regression gates for user-memory, descriptor/read-only VFS, and
  lower-EL/syscall behavior only when a later implementation touches those
  shared owners.

Pi 5 hardware proof remains blocked until after loader implementation and
QEMU/substitute evidence justify a separate serialized physical task.

## Blocked Surfaces

These surfaces remain blocked until later explicit tasks accept their own
contracts and validation gates:

- loader Rust implementation, ELF parser code, image-plan data structures, and
  fixture executable bytes;
- process address-space installation, user frame allocation, page-table
  mutation, teardown, and lower-EL launch of a loaded image;
- argv/envp stack builder, auxiliary vector, TLS, libc startup, and shell
  argument behavior;
- process creation, exec/spawn/wait, PID allocation, parent/child ownership,
  exit status, signals, and credentials;
- descriptor-backed filesystem syscalls, directory iteration, seek, current
  working directory mutation, close-on-exec enforcement, and final release;
- writable filesystems, persistent storage, networking, SSH, RP1/PCIe, UART
  interrupt ownership, DMA/cache-driver policy, and Pi 5 physical proof.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: used the accepted program-loader source
  inventory, read-only initramfs/VFS closeout, Phase 7 EL0/address-space
  contract, copy-in/copy-out contract, POSIX baseline loader vocabulary,
  src/initramfs.rs, src/posix.rs, src/syscall.rs, roadmap, and ADR index.
- static documentation diff: added this contract and task record, linked the
  contract from SUMMARY, and updated roadmap and decisions.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this documentation-only
  contract.
