# Lower-EL and Userspace Mapping Readiness

Talos currently runs the accepted Raspberry Pi 5 kernel path at EL2 with one
early stage-1 translation regime. The map is a kernel bring-up map, not a
userspace contract.

## Current EL2 Map

The accepted early table skeleton uses `TTBR0_EL2` and 4 KiB table pages from
the fixed bootstrap reservation. It maps:

- low DRAM `0x0..0x40000000` with 2 MiB normal-memory identity blocks,
- and the BCM2712 local-peripheral window `0x107c000000..0x1080000000` with
  2 MiB device identity blocks.

The matching register plan is still the early EL2 plan:

```text
talos: translation control plan: el=0x2 mair=0x4ff tcr=0x53510 ttbr0=0x2f000000 sctlr_set=0x1 va_bits=0x30 pa_bits=0x30 kind=el2-stage1-4k-no-enable
talos: translation enabled: el=0x2 sctlr=0x30c50831 ttbr0=0x2f000000 kind=el2-stage1-4k-enabled
```

This proves that the early kernel can keep running with EL2 stage-1 translation
enabled, normal memory cacheable, and required MMIO reachable as device memory.
Historical proof-only diagnostics also showed the same-EL exception path can
report BRK, undefined instruction, data-abort, and translation-fault state. The
old Pi 5 diagnostic wrappers for those proofs were retired after their accepted
evidence was summarized.

## What The Map Does Not Prove

The current identity map does not prove a safe lower-EL execution environment.
It does not establish:

- a user/kernel virtual address split,
- user-accessible page descriptors or per-process page tables,
- kernel-only permissions for kernel text, data, stacks, heap, bootstrap tables,
  DTB data, UART, GIC-local, or other MMIO mappings,
- lower-EL vector routing, trap return, or production fault recovery,
- user stack, heap, code, data, guard, or shared-memory mappings,
- a syscall ABI,
- or safe copy-in/copy-out and invalid-user-memory behavior.

No code should treat the accepted EL2 identity map as permission to enter EL0 or
to run untrusted payloads. The map is intentionally broad enough for early
kernel bring-up and intentionally too broad for userspace isolation.

## Phase 7.2 Source Inventory Boundary

The Phase 7.2 EL0/address-space source inventory is accepted as documentation
only. It names the current exception-vector and saved-frame surfaces, same-EL
`ERET` diagnostics, broad EL2 identity map, early page-frame ownership,
scheduler task/process separation, `PosixError::Fault` / `EFAULT`
vocabulary, descriptor-table ownership, retained gates, and implementation
gaps that constrain the next contract.

That inventory does not reduce the lower-EL gate. Current same-EL exception
reports, current IRQ `ERET`, broad EL2 identity mappings, bootstrap
page-frame ownership, scheduler diagnostics, runtime console, TTY, diagnostic
commands, and descriptor-table unit tests remain diagnostic-only or
kernel-only. They are not lower-EL, syscall, copy-in/copy-out,
process-isolation, or userspace contracts.

## Minimum Prerequisites Before EL0 Work

The Phase 7.1 POSIX baseline now accepts target-independent errno/path and
descriptor-table cores. That does not reduce the lower-EL gate. Before Talos
can accept EL0 or syscall work, it still needs explicit design and validation
for:

- the address-space shape: kernel half, user range, guard gaps, and whether the
  kernel remains globally mapped while a user task runs;
- descriptor permissions: privileged-only kernel pages, executable user text,
  non-executable user data/stacks, and no user access to MMIO;
- TTBR/TCR/SCTLR policy for the lower-EL regime, including which exception
  level owns process address spaces;
- exception routing from lower ELs, including saved user register frames,
  `ERET` return rules, and fatal-versus-recoverable fault policy;
- user stack and heap mapping creation from accepted frame ownership, without
  using deferred high-memory or DMA ownership;
- an SVC/syscall ABI with stable argument, return-value, and error conventions;
- copy-in/copy-out helpers that reject invalid user pointers without corrupting
  kernel state;
- and tests or hardware evidence for bad user pointers, bad instruction fetches,
  bad stack accesses, and successful trap return.

The accepted follow-up contract defines the first address-space invariants,
lower-EL trap/return invariants, user-fault classes, copy-in/copy-out
preconditions, and evidence levels before Rust or assembly implementation
changes.

## Phase 7.2 Trap And Address-Space Contract

The Phase 7.2 EL0 trap and address-space contract is accepted as
documentation only. It defines a 48-bit vocabulary with a canonical user range
below 0x0000_8000_0000_0000, a null guard at
0x0000_0000_0000_0000..0x0000_0000_0001_0000, user text/data/heap/stack/guard
mapping names, and kernel mappings that may be present while a user task runs
only if they deny EL0 read, write, and execute access.

The contract also requires validated user ELR, user stack pointer, SPSR, and
general-register frame state before any ERET to lower EL. User trap frames
must be able to record x0 through x30, user SP, ELR, SPSR, ESR, FAR, vector
class, and available task/process-owner identity. Current same-EL diagnostics
remain useful source material, not accepted lower-EL recovery.

User fault classes are now named for instruction abort, read data abort, write
data abort, stack fault, bad trap-return state, and unsupported lower-EL
synchronous traps. Invalid userspace pointers at a POSIX-facing copy boundary
map to PosixError::Fault / EFAULT when no side effect has been committed. The
first implementation remains limited to target-independent range and
permission validation; EL0 entry, trap-return assembly, translation-register
changes, syscall ABI, VFS/filesystem work, program loading, descriptor I/O,
shell behavior, QEMU proof, and Pi 5 proof remain separate explicit tasks.

## Phase 7.2 QEMU EL0 Trap Smoke Proof

The accepted QEMU EL0 trap smoke plan defined the first lower-EL proof as a
QEMU-only built-in payload. The accepted implementation maps fixed UserText,
UserStack, and UserGuard ranges inside the accepted user range, validates the
user ELR/SP/SPSR/mappings before ERET, executes only diagnostic SVC marker
0x7a10, and traps back through the lower-AArch64 synchronous vector.

The retained QEMU evidence is:

```text
tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt
```

The log includes the saved-state field names and the exact final lines:
qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete
and qemu-el0-trap-smoke: PASS. The saved trap state reports the lower-AArch64
synchronous vector, marker 0x7a10, ELR 0x0000000000100004 after the SVC, and
SP 0x0000000000200000 at the top of the fixed UserStack range.

This evidence remains QEMU/substitute only. It does not acquire the Pi 5
hardware lock or claim physical lower-EL behavior. General syscall ABI,
process loading, descriptor I/O, filesystem, shell, networking, and Pi 5 proof
remain deferred. The next lower-EL hardware step must first be a serialized Pi
5 proof plan that names archive identity, serial cursor, TFTP delta,
known-good control, candidate rerun, restoration, retained evidence, and
hardwareTestLock ownership.

Phase 4 interrupt/timer/preemption tasks may rely on the current EL2 kernel map
only for kernel execution. They must not assume process isolation, physical
lower-EL recovery, or production user memory validation.

## Validation Boundary

This readiness note is documentation only. It changes no Rust code, linker
layout, boot image, translation table contents, MAIR/TCR/TTBR/SCTLR programming,
exception-vector code, allocator policy, or normal Pi 5 boot output. No
hardware run is required for the note itself.
