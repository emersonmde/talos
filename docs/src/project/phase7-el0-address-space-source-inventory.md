# Phase 7 EL0 Address-Space Source Inventory

Status: accepted as a Phase 7.2 source inventory before any lower-EL,
user-address-space, syscall, copy-in/copy-out, VFS, filesystem, program-loader,
networking, SSH, or shell implementation. This document changes documentation
only. It does not add Rust behavior, assembly behavior, boot scenarios, QEMU
runs, Pi 5 hardware runs, archive publishing, or hardware-lock usage.

This inventory follows the accepted Phase 7.1 POSIX baseline closeout. It maps
the accepted exception, memory, scheduler, POSIX, descriptor, and validation
surfaces that constrain the first EL0 trap-return and user address-space
contract.

## Accepted Surfaces

### Exception Vectors And Saved Frames

- `src/arch/aarch64/vectors.S` installs one 16-entry AArch64 vector table
  for the current exception level and funnels entries through
  `__exception_entry_frame`.
- The vector entry path saves general registers x0 through x30 into the
  `ExceptionFrame` layout before calling Rust handlers. The IRQ path passes
  vector, ELR, SPSR, and the saved-frame pointer to `rust_irq_handler`, then
  restores the interrupted frame and returns with `ERET`.
- The synchronous exception path currently reports ESR, ELR, FAR, vector, SPSR,
  and saved register context through `rust_exception_handler` on Pi 5, then
  halts. It is a diagnostic same-kernel path, not a recoverable user-fault
  policy.
- `src/arch/aarch64/exceptions.rs` already names lower-AArch64 vector classes
  in `ExceptionVector`, but the accepted production path does not yet route
  EL0 exceptions into process-aware recovery or syscall dispatch.
- `exceptions::init` writes `VBAR_EL1`, `VBAR_EL2`, or `VBAR_EL3`
  according to `CurrentEL`. The accepted Pi 5 runtime still installs and runs
  the kernel path at EL2.

### Same-EL ERET Diagnostics

- The accepted Phase 4 timer and preemption evidence proves current-EL IRQ
  save/restore and `ERET` return for kernel execution.
- The accepted scheduler architecture explicitly keeps cooperative
  kernel-thread context switching separate from asynchronous exception-frame
  switching. Phase 6 production timer/preemption records bounded IRQ state and
  services scheduler work from owner-local normal control flow after interrupt
  return.
- These facts are useful constraints for trap return, but they do not define
  lower-EL entry state, user SPSR/ELR construction, user stack selection,
  process-fault recovery, or syscall ABI.

### EL2 Translation Setup

- `src/memory_map/translation.rs` owns the accepted early EL2 stage-1 table
  skeleton. It reserves four 4 KiB table pages, uses `TTBR0_EL2`, configures
  MAIR/TCR for a 48-bit EL2 regime, maps low DRAM
  `0x0..0x4000_0000` as normal identity blocks, and maps the BCM2712
  `0x10_7c00_0000..0x10_8000_0000` MMIO window as device identity blocks.
- Device blocks are PXN/UXN, but the low-DRAM identity map is a broad kernel
  bring-up map. It does not encode a user/kernel split, per-process page
  tables, user-accessible descriptors, guarded user stacks, or kernel-only
  permissions.
- `src/arch/aarch64/mod.rs::enable_el2_mmu_from_plan` programs
  `MAIR_EL2`, `TCR_EL2`, `TTBR0_EL2`, invalidates EL2 TLBs, and sets
  `SCTLR_EL2.M` from the accepted plan. It does not switch TTBRs, prepare an
  EL1 or EL0 regime, or create process address spaces.

### Page-Frame Ownership

- `src/memory_map/page_frames.rs` defines the accepted early page-frame
  ownership contract: bootstrap-reserved frames, translation-table frames,
  bootstrap allocator-owned frames, deferred memory outside the conservative
  low-tail window, and a small reuse allocator for accepted early memory work.
- That ownership model protects bootstrap tables and early kernel allocation,
  but it is not a long-lived physical-memory manager, process page allocator,
  copy-on-write policy, high-memory policy, DMA/cache policy, or user mapping
  lifecycle.
- The next contract may use the accepted ownership vocabulary to describe
  future user-stack, user-code, guard, and address-space frame sources, but it
  must not imply those sources are implemented.

### Scheduler Task And Process Separation

- `src/scheduler.rs` schedules tasks. `TaskId` is scheduler-local, and
  `Task` currently owns kernel stack and cooperative context state.
- `ProcessOwnerId` and `Task::attach_process_owner` are accepted extension
  points only. There is no PID allocator, process table, parent/child storage,
  exit status, wait queue, signal model, credentials, session, controlling TTY,
  or process-owned address-space handle.
- `SharedSchedulerMetadata` may publish an optional process-owner
  placeholder, but it remains advisory scheduler metadata, not a global process
  registry or authority for address-space lifetime.
- Phase 6 production scheduler rules still require owner-local normal control
  flow for scheduler mutation. A lower-EL trap contract must preserve that
  boundary and avoid switching or migrating another CPU's current task from an
  exception handler.

### POSIX Error Vocabulary

- `src/posix.rs` owns the accepted Phase 7.1 target-independent
  `PosixError` vocabulary. `PosixError::Fault` maps to `EFAULT` and is
  the right existing name for invalid userspace pointers at a later syscall or
  copy boundary.
- The vocabulary is internal and target-independent. Talos has not yet accepted
  numeric errno values, syscall return registers, restart conventions,
  signal-on-fault behavior, or per-thread errno storage.

### Descriptor-Table Ownership

- `src/posix.rs::DescriptorTable` is the accepted process-local descriptor
  data model. It covers table entries, inherited stdio handles, allocation,
  lookup, close, dup, access checks, reserved object kinds, and deterministic
  `PosixError` results.
- Descriptor entries are future process-owned resources. They do not yet carry
  runtime I/O, VFS objects, readiness state, blocking policy, scheduler wait
  queues, user buffers, or copy-in/copy-out behavior.
- The lower-EL and user-address-space contract must treat descriptor I/O as
  blocked until later tasks define syscall ABI, user memory copying, and object
  backends.

### Retained Validation Gates

- Target-independent Rust slices should continue to use
  `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`,
  `git diff --check`, and `mdbook build` when docs are touched.
- Lower-EL runtime proof tasks must explicitly add QEMU gates. The existing
  base and scheduler QEMU scripts remain retained evidence surfaces, but they
  do not prove EL0 entry or user fault handling by implication.
- Pi 5 hardware evidence remains serialized behind `hardwareTestLock` and
  task-specific acceptance criteria. This inventory makes no hardware claim.

## Diagnostic-Only Or Kernel-Only Surfaces

These surfaces must not be treated as accepted lower-EL, syscall,
copy-in/copy-out, process-isolation, or userspace contracts:

- Current same-EL exception reports and old fault diagnostics: useful for ESR,
  FAR, ELR, SPSR, and frame reporting, but they halt instead of recovering a
  user task.
- Current IRQ `ERET` return: accepted for interrupted kernel execution, not
  for constructing initial user SPSR/ELR or user stack state.
- The broad EL2 identity map: accepted for early kernel bring-up only, not
  user/kernel isolation or permission enforcement.
- Bootstrap page-frame and heap policies: accepted early ownership and
  allocation tools, not process page-table allocation or user mapping policy.
- Scheduler proof scenarios and diagnostic boot modes: evidence surfaces for
  named invariants, not production process or syscall APIs.
- Runtime console, TTY, and diagnostic command channel: kernel diagnostics and
  future stdio backing surfaces, not shell, program loader, descriptor I/O, or
  syscall interfaces.
- Descriptor-table target-independent tests: table semantics only, not proof
  of file, device, pipe, socket, or user-buffer I/O.

## Implementation Gaps For Phase 7.2

- Address-space policy: user/kernel virtual split, canonical user range,
  kernel mapping while user code runs, guard gaps, and permission vocabulary.
- Mapping lifecycle: user code, data, heap, stack, guard, and shared mappings;
  physical-frame source, ownership, teardown, and error behavior.
- Translation regime: TTBR/TCR/SCTLR policy for lower-EL execution, including
  whether Talos keeps EL2 as the kernel level for the first user payload.
- Trap return: initial user register-frame shape, user SPSR/ELR construction,
  stack pointer selection, `ERET` rules, and return-to-kernel invariants.
- Lower-EL exception policy: synchronous fault classes, fatal versus
  recoverable outcomes, bad instruction fetch, bad stack access, and how the
  current task/process is identified after a trap.
- User memory boundary: range validation, read/write/execute permissions,
  null, wraparound, kernel-range, guard-page, unmapped, and length-limit
  behavior before byte-copy helpers exist.
- POSIX mapping: invalid user pointers should map to the accepted `EFAULT`
  vocabulary where applicable, without inventing syscall return numbers yet.
- Proof strategy: target-independent helper tests first, QEMU lower-EL trap
  proof later, and serialized Pi 5 proof only when a task explicitly requires
  hardware evidence.

## Recommended Follow-Up

The next bounded task should be
`phase7-el0-trap-address-space-contract-20260528`.

That task should define the first EL0 trap-return and user address-space
contract: address-space invariants, user/kernel mapping policy, lower-EL
trap/return invariants, user fault classes, copy-in/copy-out preconditions,
and evidence levels. It should remain documentation-only unless a narrow
compile-time constant or example is explicitly justified.

It must keep implementation, SVC/syscall numeric ABI, VFS, filesystem, program
loading, descriptor I/O, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache-driver policy blocked until later explicit
tasks.

## Validation

- static inspection: git status --short was clean before edits.
- static review: inspected accepted Phase 7.1 closeout, lower-EL readiness,
  exception vectors and handlers, AArch64 control-register helpers, early EL2
  translation setup, page-frame ownership, scheduler task/process separation,
  POSIX error and descriptor-table cores, roadmap, and decision log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs were not required
  because this task changes only Markdown documentation and durable worker
  state.
