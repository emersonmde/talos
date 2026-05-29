# Phase 7 Pointer-Taking Syscall Contract

Status: accepted as the documentation-only Phase 7.3 pointer-taking syscall
contract after the accepted
[Phase 7 Pointer-Taking Syscall Source Inventory](phase7-pointer-taking-syscall-source-inventory.md).
It does not add Rust behavior, assembly behavior, boot scenarios, QEMU runs,
Pi 5 hardware runs, archive publishing, hardware-lock use, descriptor I/O,
runtime console or TTY integration, process loading, VFS, filesystem, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

This contract fixes the first lower-EL syscall boundary that carries a user
pointer and length into the accepted copy-in/copy-out helpers. The boundary is
proof-only and QEMU/substitute scoped. It exists to prove register argument
capture, user mapping provenance, helper invocation, return/error encoding,
and diagnostic quarantine before any descriptor-backed POSIX read/write or
path-copying surface is designed.

## Proof-Only Status

The first pointer-taking syscall is named talos_copy_probe. It is proof-only,
not a stable POSIX syscall, libc contract, descriptor I/O operation, or
filesystem API. It may be compiled only into the later
qemu_pointer_copy_smoke boot scenario and any target-independent unit-test
support needed by that implementation.

The stable SVC immediate remains svc #0. SVC immediate 0x7a10 remains the
existing diagnostic completion marker and is not a syscall number, ABI
version, selector, or success path.

talos_copy_probe uses syscall number x8 = 0x7001. That number is reserved only
for the proof-only pointer-copy smoke. Outside the accepted proof scenario, or
before the later implementation is explicitly enabled, x8 = 0x7001 must remain
unknown and return -ENOSYS like any other unaccepted syscall number.

## Register Contract

The lower-EL route obtains all arguments from the saved ExceptionFrame captured
for a lower-AArch64 synchronous svc #0 exception:

| Register | Meaning |
| --- | --- |
| x8 | syscall number; 0x7001 selects talos_copy_probe in the proof scenario only |
| x0 | user virtual start address for the user data buffer |
| x1 | byte length to copy; valid range is 0 through 32 for the first smoke |
| x2 | expected byte value for the copy-in validation, low 8 bits only |
| x3 | replacement byte value for the copy-out validation, low 8 bits only |
| x4 | reserved kernel scratch selector; must be 0 |
| x5 | reserved flags; must be 0 |

The later implementation must reject nonzero x4 or x5 with -EINVAL before
calling copy helpers. x2 and x3 are byte values; upper bits must be ignored
only after masking to u8 and must not become flags.

Zero-length calls are side-effect-free. x0 must still be a non-guard user
address accepted by UserRange::new, x1 is 0, x4 and x5 are 0, and success
returns 0. x2 and x3 are ignored for the zero-length case. A zero-length call
does not validate any mapping contents and must not make a guard, kernel, or
wrapped address acceptable.

## Copy Operation

For a nonzero success case, talos_copy_probe performs both directions in one
bounded operation:

1. Validate and copy x1 bytes from the user data buffer at x0 into a fixed
   kernel scratch buffer with copy_from_user().
2. Confirm every copied byte equals the low 8 bits of x2.
3. Fill the same scratch range with the low 8 bits of x3.
4. Copy that scratch range back to the same user buffer with copy_to_user().
5. Return x1 in x0 only after both helper calls and the expected-byte check
   succeed.

If the expected-byte check fails after a successful copy_from_user(), the
syscall returns -EINVAL and must not call copy_to_user(). This keeps malformed
QEMU proof setup distinct from user-boundary EFAULT. The later QEMU smoke may
print this as a proof-configuration failure, but it must not treat it as a
valid EFAULT observation.

The proof-specific 32-byte maximum is intentionally smaller than
DEFAULT_USER_COPY_LIMIT. Lengths larger than 32 are malformed proof setup and
return -EINVAL before helper invocation. Helper-reported InvalidArgument also
returns -EINVAL. Helper-reported Fault returns -EFAULT.

## Mapping And Backing Storage

The QEMU pointer-copy smoke must use caller-owned substitute mappings and
backing storage. The first accepted substitute data mapping is:

- UserData:
  0x0000_0000_0011_0000..0x0000_0000_0011_1000, readable and writable, not
  executable.

The mapping may sit alongside the existing QEMU UserText, UserStack, and
UserGuard ranges. The implementation must pass an explicit UserMapping slice,
user_memory_start = 0x0000_0000_0011_0000, and the matching in-kernel backing
storage slice to copy_from_user() and copy_to_user(). The helpers must not
derive authority from live page tables or from raw pointer values in x0.

The first EFAULT observation should use an address inside the existing guard
range or another unmapped/non-writable user range fixed by the later smoke
plan. The contract does not accept demand paging, copy-on-write, page-fault
recovery, fault tables, signal/restart behavior, or process-owned address
spaces.

## Return And Error Observations

talos_copy_probe returns through the existing syscall x0 convention:

| Case | Return |
| --- | ---: |
| Successful nonzero copy-in plus copy-out | requested byte length in x0 |
| Successful zero-length call for a valid non-guard user address | 0 |
| User range, mapping, permission, backing-storage, or guard failure | -EFAULT |
| Nonzero x4/x5, length greater than 32, or expected-byte mismatch | -EINVAL |
| x8 = 0x7001 outside the proof scenario, or any other unaccepted number | -ENOSYS |

The lower-EL route must preserve the accepted frame rules: x0 is the only
mutated saved register after dispatch, x1 through x5 and x8 remain preserved
in the saved frame for proof logging, and ELR/SPSR/SP_EL0 semantics remain the
ones accepted by the scalar syscall route. The handler must not infer pointer
validity from the raw x0 register; only the accepted mapping and helper
validation may authorize byte movement.

## QEMU Substitute Evidence Boundary

The next QEMU smoke plan must define exact output and retained evidence for at
least these observations:

- talos_copy_probe success with x8 = 0x7001, x0 = UserData start, x1 = 16,
  x2 = the initial byte pattern, x3 = the replacement byte pattern, x4 = 0,
  and x5 = 0, returning x0 = 16.
- user-observed data after return showing the replacement byte pattern was
  written to the substitute UserData backing storage.
- deterministic EFAULT from the same proof-only syscall number when x0 names
  the guard/unmapped range with a nonzero length.
- deterministic -ENOSYS for an unaccepted syscall number to preserve the
  scalar unknown-syscall invariant.
- diagnostic marker 0x7a10 reported as proof-only completion vocabulary and
  never dispatched as talos_copy_probe.

This evidence is QEMU/substitute only. It does not prove Pi 5 pointer-copy
hardware behavior, process address-space ownership, descriptor I/O,
filesystem-backed data, path copying, program loading, shell behavior,
networking, or SSH.

## Blocked Surfaces

This contract keeps these surfaces blocked until later explicit tasks:

- stable POSIX read/write, descriptor I/O, TTY-backed stdio, and descriptor
  table mutation through syscall entry;
- VFS/filesystem behavior, path copying, string termination policy, argv/envp
  copying, ELF loading, process tables, PID allocation, exit/wait, credentials,
  sessions, signals, syscall restart, and per-thread errno storage;
- Pi 5 pointer-copy hardware proof, hardwareTestLock acquisition, archive
  publishing, lab-controller power actions, and serial observe;
- demand paging, copy-on-write, shared memory, mmap, user DMA buffers,
  lower-EL fault-table recovery, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, shell behavior, networking, and SSH.

## Next Planning Boundary

The next bounded task should be
phase7-qemu-pointer-copy-smoke-plan-20260529. It may only plan the
QEMU/substitute smoke for the proof-only talos_copy_probe contract above:
exact boot scenario name, payload/register setup, output lines, retained log
paths, regression gates, and deferred Pi 5 hardware proof boundary.

phase7-qemu-pointer-copy-smoke-core-20260529 remains dependency-blocked until
that plan is accepted and committed. Descriptor I/O, process loading,
VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware proof
remain blocked.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: this contract fixes proof-only status, syscall
  number 0x7001, x0-through-x5 argument meanings, zero-length behavior,
  success and -EFAULT/-EINVAL/-ENOSYS observations, saved-frame preservation
  rules, QEMU substitute mapping/backing-storage provenance, diagnostic marker
  quarantine, and the next planning boundary.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
