# Phase 7 Pointer-Taking Syscall Source Inventory

Status: accepted as the documentation-only Phase 7.3 pointer-taking syscall
source inventory after the accepted target-independent copy-in/copy-out helper
closeout. This document follows the accepted
[Phase 7 Copy-In/Copy-Out Helper Closeout Checkpoint](phase7-copyin-copyout-helper-closeout-checkpoint.md).
It does not add Rust behavior, assembly behavior, boot scenarios, QEMU runs,
Pi 5 hardware runs, archive publishing, hardware-lock use, descriptor I/O,
runtime console or TTY integration, process loading, VFS, filesystem, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

This inventory maps source owners and gaps for the first lower-EL syscall that
passes a user pointer into the accepted copy helper boundary. The later
contract must decide whether that syscall remains proof-only or becomes a
stable POSIX-facing surface before any implementation allocates a syscall
number or changes boot scenarios.

## Source Owners

### Frame Argument Extraction

- src/arch/aarch64/exceptions.rs::try_route_lower_aarch64_syscall owns the
  recoverable lower-AArch64 svc #0 route. It accepts only
  ExceptionVector::LowerAarch64Sync, ESR EC 0x15, and stable svc #0.
- ExceptionFrame::reg() exposes saved x0 through x30. The current route
  captures x0 through x5 into syscall::SyscallArguments and reads x8 as the
  syscall number before dispatch.
- The current route mutates only saved x0 with SyscallReturn and preserves the
  existing saved ELR/SPSR semantics supplied by the vector frame.

Gap: there is no pointer-taking syscall contract that assigns argument roles
such as user pointer, length, operation selector, expected byte pattern, or
kernel buffer ownership. The next contract must state exact x0-through-x5
meaning, zero-length behavior, and null-frame handling before implementation.

### Syscall Number And Dispatch

- src/syscall.rs owns STABLE_SVC_IMMEDIATE = 0, the diagnostic marker
  0x7a10 quarantine, SyscallNumber, SyscallArguments, SyscallReturn, errno
  encoding, and syscall::dispatch().
- The accepted stable syscall namespace currently contains only talos_nop = 0.
  Unknown syscall numbers return -ENOSYS through x0.
- src/arch/aarch64/exceptions.rs calls syscall::dispatch() from the lower-EL
  route and writes only the returned x0 value back to the saved frame.

Gap: the dispatcher has no pointer-taking syscall variant and no way to pass a
validated user-memory view or kernel buffer into syscall-specific code. The
next contract must either allocate a temporary proof-only syscall number with
clear quarantine, or reserve a stable number with a compatibility story and
explicit deferred POSIX surfaces.

### User-Memory Mapping Provenance

- src/posix.rs owns UserRange, UserMapping, UserMappingPermissions,
  UserAccessKind, DEFAULT_USER_COPY_LIMIT, validate_user_memory_access(),
  copy_from_user(), and copy_to_user().
- src/target/qemu_virt.rs::run_syscall_smoke and
  src/target/rpi5.rs::run_syscall_proof already build fixed UserText and
  UserStack mapping arrays for lower-EL proof payload setup.
- The accepted copy helpers currently require a caller-provided mapping slice,
  user backing-storage start address, and backing-storage slice. They are
  target-independent and do not inspect live page tables.

Gap: no production process address-space object owns user mappings or backing
storage for a syscall handler. A QEMU pointer-copy smoke can use a fixed
boot-scenario-owned user data page and mapping array, but the contract must
name that as substitute/proof evidence and must not imply demand paging,
copy-on-write, page-fault recovery, process address spaces, or filesystem
buffers.

### Copy Helper Calls

- src/posix.rs::copy_from_user validates a complete readable user range before
  copying bytes into a kernel destination and returns the requested length on
  success.
- src/posix.rs::copy_to_user validates a complete writable user range before
  copying bytes from a kernel source and returns the requested length on
  success.
- User-boundary failures return PosixError::Fault. Malformed kernel-side helper
  use, such as a short kernel buffer/source, returns PosixError::InvalidArgument.
- The helper tests prove all-or-nothing behavior for validation failures before
  destination mutation.

Gap: no syscall path invokes these helpers yet. The next contract must decide
the first pointer-copy operation shape, required kernel scratch buffer size,
whether both copy-in and copy-out are exercised in one syscall, and how helper
errors map back to x0 without mutating unrelated saved registers.

### Return And Error Encoding

- src/syscall.rs::SyscallReturn encodes accepted PosixError values as negative
  errno in x0. PosixError::Fault maps to -EFAULT, InvalidArgument maps to
  -EINVAL, and NotImplemented maps to -ENOSYS.
- src/arch/aarch64/exceptions.rs::RoutedSyscall records raw_number,
  arguments, and return_x0 for proof logging.
- The QEMU and Pi 5 syscall proof handlers print stable talos_nop and
  unknown-syscall return observations from the routed value and from the
  user-observed x0 after return.

Gap: no accepted output line or return contract exists for pointer-copy
success or EFAULT. The next contract must define exact success returns,
EFAULT observations for bad pointers or permissions, and whether EINVAL is
observable for malformed proof configuration only.

### QEMU Smoke Ownership

- scripts/qemu-syscall-smoke.sh owns the current QEMU/substitute syscall
  smoke gate and retained serial-log path conventions.
- src/target/qemu_virt.rs owns qemu_syscall_smoke payload bytes, fixed EL0
  tables, fixed user text/stack mappings, and qemu-syscall-smoke
  classification/PASS output.
- docs/src/project/phase7-qemu-syscall-smoke-plan.md and
  docs/src/project/phase7-syscall-routing-closeout-checkpoint.md style
  retained evidence and exact expected output for syscall smokes.

Gap: there is no qemu_pointer_copy_smoke scenario, script, retained evidence
directory, expected output vocabulary, or user data mapping. The next smoke
plan should not be promoted until a contract fixes the syscall number,
arguments, success and EFAULT observations, diagnostic quarantine, and whether
the evidence remains QEMU/substitute only.

### Diagnostic Surface Quarantine

- src/syscall.rs keeps DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE = 0x7a10 outside the
  stable syscall ABI.
- src/target/qemu_virt.rs and src/target/rpi5.rs still own the existing
  proof-only diagnostic marker handling for EL0 trap and scalar syscall proof
  scenarios.
- The accepted copy helpers are not diagnostic by themselves; only a later
  boot-scenario syscall smoke would be diagnostic/proof-only.

Gap: the next contract must state whether the pointer-taking syscall is a
temporary proof-only syscall or a stable surface. If proof-only, its syscall
number, output lines, and scripts must be quarantined like the existing smoke
scenarios and must not become descriptor I/O, filesystem I/O, or user-visible
POSIX API.

## Recommended Next Contract

The next bounded task should be
phase7-pointer-taking-syscall-contract-20260529, created by supervisor
planning before any implementation or QEMU smoke plan is promoted.

That contract should define:

- proof-only versus stable syscall status and the exact syscall number;
- x0-through-x5 argument roles, including user pointer, length, and any
  operation selector or expected data value;
- the caller-owned user mapping/backing-storage model for QEMU substitute
  evidence;
- whether the first smoke exercises copy-in, copy-out, or both;
- success return values and -EFAULT/-EINVAL/-ENOSYS behavior;
- preserved-register and ELR/SPSR rules inherited from scalar syscall routing;
- diagnostic marker 0x7a10 quarantine and proof-only output naming;
- exact QEMU smoke evidence paths and PASS/classification lines;
- the explicit rule that descriptor I/O, VFS/filesystem behavior, process
  loading, shell, networking, SSH, and Pi 5 pointer-copy hardware proof remain
  later tasks.

phase7-qemu-pointer-copy-smoke-plan-20260529 remains dependency-blocked until
that contract is accepted. Descriptor I/O and filesystem behavior remain out
of scope until the pointer-taking syscall boundary has an accepted contract and
QEMU evidence plan.

## Validation

- static inspection: git status --short before edits was clean.
- static source review: inspected lower-AArch64 syscall frame extraction and
  return mutation, target-independent syscall dispatch and errno encoding,
  accepted copy-in/copy-out helper ownership and tests, QEMU/Pi 5 syscall proof
  handlers, QEMU smoke script ownership, roadmap, decision log, and task
  records.
- static documentation diff summary: added this inventory document; updated
  docs/src/roadmap.md, docs/src/decisions/README.md, docs/src/SUMMARY.md; added
  tasks/2026-05-29-phase7-pointer-taking-syscall-source-inventory.md.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
