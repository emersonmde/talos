# Phase 8 Kernel-Half Reachability Source Inventory

Status: accepted

Task: phase8-kernel-half-reachability-source-inventory-20260531

## Scope

This inventory maps the accepted kernel-half reachability gap left by the
Phase 8 live address-space activation closeout. It is documentation only and
authorizes no Rust or assembly behavior change, QEMU execution, live
TTBR/TCR/MAIR/SCTLR mutation, lower-EL launch, scheduler publication, Pi 5
hardware proof, boot archive publication, or hardware-lock acquisition.

The accepted LiveAddressSpaceActivationPlan can copy the loader/install/
address-space/materialization/launch/stack lineage and prove that activation
preflight has no live side effects. Its exact blocker is still
blocked-no-accepted-kernel-half-map: no accepted policy says how kernel text,
rodata, data, bss, stacks, heap, vectors, UART/MMIO diagnostics, scheduler
state, and fault reporting remain reachable when a process root eventually
becomes active.

## Accepted Inputs

Accepted artifacts and evidence reviewed for this inventory:

- ProgramImagePlan:
  phase8-program-loader-core-20260530 accepted at
  38b3a09ad4e1be353950ad75880b119e7e0b534e, with fixture identity
  phase8-program-loader-elf64-aarch64-v1 and digest 0x3892eed223900c65.
- retained program-loader QEMU/substitute evidence:
  tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log.
- ProcessImageInstallPlan:
  phase8-process-install-core-20260530 accepted at
  49a54d91ef7920f74c97ca403a5075ce5f8d84a1.
- retained process-install QEMU/substitute evidence:
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log.
- ProcessAddressSpace:
  phase8-process-address-space-core-20260530 accepted at
  06a5f4ed8e426afd01b77382c070a76d572d7c12.
- retained process address-space QEMU/substitute evidence:
  tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log.
- ProcessPageTableMaterialization:
  phase8-process-page-table-materialization-core-20260530 accepted at
  54d519e6ef629b9298bcfd2dea6fb9552fb86747.
- retained page-table materialization QEMU/substitute evidence:
  tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log.
- InitialProcessLaunchPlan:
  phase8-initial-process-launch-core-20260530 accepted at
  a57b0678126b1cb95c444e3e09fecfe8bea227f9.
- retained initial process launch QEMU/substitute evidence:
  tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log.
- InitialUserStackPlan:
  phase8-initial-user-stack-core-20260530 accepted at
  f76c07f264efd1fc570b678af71e8a26ada155fa.
- retained initial user stack QEMU/substitute evidence:
  tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log.
- LiveAddressSpaceActivationPlan:
  phase8-live-address-space-activation-core-20260530 accepted at
  129337734011004297da0b2768a3a802063c3293.
- retained live address-space activation QEMU/substitute evidence:
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log.
- live address-space activation closeout checkpoint:
  5f4589db87b821775b671c7630bb19c9905d497c.

## Current Owners

- linker.ld and linker-rpi5.ld own the linked kernel image layout:
  __kernel_start, .text.boot, .vectors, .text, .rodata, .data,
  __kernel_image_end, .bss, __heap_start, __heap_end, __stack_bottom,
  __stack_top, and __kernel_end. They do not currently describe a per-process
  kernel-half virtual mapping contract.
- src/boot/rpi5_reports.rs reports the kernel, heap, and stack ranges plus
  post-allocator translation-table layout on Pi 5. Those reports are evidence
  and diagnostics, not an activation-time reachability policy.
- src/allocator.rs and src/memory_map/page_frames.rs own bootstrap heap and
  early page-frame allocation vocabulary. They do not yet define kernel-half
  mappings, active-address-space teardown, or active-root ownership.
- src/memory_map/layout.rs and src/memory_map/translation.rs own early
  stage-1 layout and descriptor helpers for the kernel bring-up map. The
  current accepted production use remains the EL2 kernel identity map and
  target-independent user descriptor records, not a shared EL1 kernel-half
  root.
- src/arch/aarch64/exceptions.rs and src/arch/aarch64/vectors.S own VBAR
  setup, the vector table, lower-AArch64 synchronous vector paths, and saved
  exception-frame vocabulary. They do not yet prove that the vector table and
  fatal-report path remain reachable under a process translation root.
- src/mmio.rs, src/pl011.rs, src/runtime_console.rs, and src/tty.rs own the
  current UART/MMIO diagnostic and console surfaces. They are required
  reachability inputs for future fault reporting, but no accepted kernel-half
  map currently binds their MMIO descriptors into a process activation root.
- src/scheduler.rs owns TaskId, ProcessOwnerId, Task, KernelStack,
  ContextFrame, run queues, and production dispatch state. It has no accepted
  live ProcessAddressSpace binding, current ASID, active root, or runnable
  publication record for /bin/init.
- src/process_page_table_materialization.rs owns an inspectable user
  descriptor image with kernel_mapping_policy=activation-blocked-no-kernel-half.
  Its root/table/user-frame leases deliberately stop below a kernel-half map,
  TTBR load, ASID allocation, TLBI, and activation rollback.
- src/live_address_space_activation.rs owns the current preflight boundary.
  It records a KernelReachabilityChecklist and blocked TTBR1/kernel-half
  policy, but the checklist is a prerequisite record only; it is not a
  descriptor image or architecture-owned map.
- src/target/qemu_virt.rs owns QEMU/substitute evidence producers for the
  accepted Phase 8 model boundaries plus proof-local lower-EL diagnostic
  tables. The proof-local tables are not a reusable kernel-half reachability
  contract.
- src/target/rpi5.rs owns serialized Pi 5 proof scenarios and proof-local
  lower-EL observations from earlier phases. Hardware evidence remains
  blocked until a later explicit Pi 5 proof plan.

## Kernel-Half Gap Map

| Area | Accepted state | Missing kernel-half policy |
| --- | --- | --- |
| Kernel image sections | Linker scripts name text, vectors, rodata, data, bss, heap, stack, and kernel-end symbols. | Which virtual range and descriptor permissions keep those ranges reachable while denying EL0 access. |
| Active kernel stack | Current boot and scheduler paths have kernel stacks and diagnostics can report stack ranges. | How the active stack is mapped across a process-root switch and whether future per-task stacks share the same kernel-half policy. |
| Heap and allocator | Bootstrap allocator and page-frame seed evidence exist. | Whether heap/page-frame metadata is globally mapped, replicated, or explicitly forbidden during first-slice activation. |
| VBAR and vectors | VBAR setup and vector-table relocation are accepted for current EL paths. | How VBAR_EL1 and exception vectors remain executable and readable after user TTBR/root activation. |
| UART/MMIO diagnostics | Runtime console and PL011/MMIO owners can report faults today. | Which device mappings are retained for panic/fault reporting, and with what device attributes and EL0 permissions. |
| Scheduler state | Scheduler owns tasks, stacks, run queues, and context frames. | How scheduler code/data and future current-process metadata remain reachable without accepting runnable publication. |
| User process root | Materialization owns only user descriptors and records blocked kernel mapping policy. | Whether the first accepted policy uses TTBR1_EL1 shared kernel root, replicated kernel-half descriptors, or a preflight-only blocked record. |
| AArch64 register vocabulary | Live activation records TTBR/TCR/MAIR/SCTLR/ASID/TLB states as blocked or compatibility-only. | Exact TCR split, TTBR0/TTBR1 root ownership, MAIR attribute index compatibility, ASID scope, and TLBI/barrier requirements for kernel reachability. |
| Fault reporting | Current exception and panic paths can print under the current kernel map. | How synchronous activation faults are reported if the process root or kernel-half policy is incomplete. |

## Candidate First-Slice Policies

Three policies are plausible source material for the next contract:

- TTBR1_EL1 shared kernel root: keep process user mappings in TTBR0_EL1 and
  a separately owned kernel-half root in TTBR1_EL1. This best matches a normal
  user/kernel split, but the contract must define TCR split fields, VA range,
  kernel descriptor provenance, MMIO attributes, ASID interaction, and
  teardown ownership before implementation.
- Replicated kernel-half descriptors: copy a minimal kernel/MMIO/vector
  reachability set into each process root. This can make the first model
  easier to inspect, but risks hiding the future sharing and context-switch
  rules unless the contract states replication is a temporary first-slice
  policy.
- Explicitly blocked preflight record: keep LiveAddressSpaceActivationPlan
  blocked and add a richer KernelHalfReachabilityPlan that proves all required
  inputs, selected policy, and rejection behavior without building
  descriptors. This is the smallest safe boundary if the next slice should
  remove ambiguity before touching page-table construction.

The current source inventory does not choose a live register sequence. It
identifies the contract boundary needed to replace
blocked-no-accepted-kernel-half-map with an accepted, inspectable
kernel-half-reachability policy.

## Boundary Separation

Kernel-half reachability must stay separate from adjacent unaccepted surfaces:

- Live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB,
  and ISB mutation remain blocked until a later implementation task has an
  accepted contract and smoke plan.
- Lower-EL ERET, SP_EL0/ELR_EL1/SPSR_EL1 writes, and live trap return remain
  lower-EL launch work.
- Scheduler runnable publication, process table/PID/current-process state,
  exit, wait, exec, spawn, signals, and credentials remain process lifecycle
  work.
- Broad argv/envp/auxv/TLS ABI, libc startup, dynamic stack growth,
  copy-on-write, demand paging, and guard-fault recovery remain later startup
  ABI and VM work.
- Descriptor-backed filesystem syscalls, descriptor inheritance, cwd/root,
  shell behavior, writable filesystem state, persistent storage, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  outside this slice.
- Pi 5 hardware proof, boot archive publication, TFTP/serial evidence, and
  hardwareTestLock acquisition remain blocked until a later explicit hardware
  proof plan exists.

## Smallest Next Boundary

The smallest objective follow-up is a documentation-only contract for
phase8-kernel-half-reachability-contract-20260531.

That contract should select a KernelHalfReachabilityPlan, kernel-half
descriptor-image policy, or explicitly blocked preflight boundary that:

- consumes the accepted live activation closeout and Phase 8 loader/install/
  address-space/materialization/launch/stack lineage;
- names the selected first-slice policy for kernel text/rodata/data/bss,
  vectors, active kernel stack, heap/page-frame allocator, UART/MMIO
  diagnostics, scheduler state, and panic/fault reporting;
- defines descriptor permissions for kernel-only executable/data pages,
  device mappings, and no EL0 access;
- states whether TTBR1_EL1 shared kernel root, replicated descriptors, or a
  preflight-only blocked record is the first accepted boundary;
- records TCR_EL1, MAIR_EL1, TTBR0_EL1/TTBR1_EL1, ASID, TLB, and barrier
  compatibility vocabulary without performing live mutation;
- defines deterministic rejection/no-partial behavior for missing kernel
  ranges, missing MMIO diagnostics, incompatible user materialization, and
  forbidden live register or lower-EL requests; and
- names the QEMU/substitute smoke plan needed before implementation.

Implementation should remain blocked until that contract and the
QEMU/substitute smoke plan are accepted.

## Deferred Surfaces

This inventory keeps these surfaces blocked:

- live TTBR/TCR/MAIR/SCTLR mutation, ASID allocation, live TLB invalidation,
  and live DSB/ISB activation sequencing;
- lower-EL ERET and live execution of /bin/init;
- scheduler runnable publication, current-process state, process table/PID
  allocation, process lifecycle, exit, wait, exec, and spawn;
- broad argv/envp/auxv/TLS ABI, libc startup framing, signal stacks,
  guard-fault recovery, copy-on-write, and demand paging;
- descriptor-backed filesystem syscalls, descriptor inheritance semantics,
  cwd/root, shell behavior, writable filesystem state, and persistent storage;
- Pi 5 hardware proof, boot archive publication, hardwareTestLock
  acquisition, TFTP/serial evidence, and physical serial claims;
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Recommendation

The mechanically next task should be
phase8-kernel-half-reachability-contract-20260531, if queued dependencies
remain satisfied. The accepted live activation closeout identifies
blocked-no-accepted-kernel-half-map as the first blocker after the
model-only activation preflight; the next contract can remove ambiguity
without accepting live register mutation, lower-EL ERET, scheduler
publication, or hardware proof.

## Reviewed Materials

- docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md
- docs/src/project/phase8-live-address-space-activation-source-inventory.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md
- tasks/2026-05-30-phase8-live-address-space-activation-core.md
- tasks/2026-05-30-phase8-qemu-live-address-space-activation-smoke-core.md
- tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-initial-user-stack-contract.md
- linker.ld
- linker-rpi5.ld
- src/allocator.rs
- src/boot/rpi5_reports.rs
- src/live_address_space_activation.rs
- src/memory_map/layout.rs
- src/memory_map/page_frames.rs
- src/memory_map/translation.rs
- src/mmio.rs
- src/pl011.rs
- src/runtime_console.rs
- src/scheduler.rs
- src/tty.rs
- src/arch/aarch64/exceptions.rs
- src/arch/aarch64/vectors.S
- src/target/qemu_virt.rs
- src/target/rpi5.rs
- docs/src/architecture/memory.md
- docs/src/architecture/lower-el-userspace.md
- docs/src/architecture/exceptions.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted Phase 8 activation
  docs/task records/evidence; source owners for linker layout, allocator,
  page frames, translation, AArch64 exceptions/vectors, UART/MMIO diagnostics,
  scheduler, live activation, and target evidence producers; architecture
  notes; roadmap; SUMMARY; and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
