# Phase 8 Kernel-Half Descriptor-Image Source Inventory

Status: accepted

Task: phase8-kernel-half-descriptor-image-source-inventory-20260531

## Scope

This inventory maps the next Phase 8 Milestone 8.3 frontier after the accepted
kernel-half reachability closeout. It is documentation only and authorizes no
Rust behavior change, assembly behavior change, QEMU execution, Pi 5 hardware
run, boot archive publication, hardwareTestLock acquisition, live
TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB
mutation, activation DSB/ISB sequence, lower-EL ERET, scheduler runnable
publication, process lifecycle, shell behavior, descriptor-backed filesystem
syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

The accepted KernelHalfReachabilityPlan proves copied Phase 8 input lineage,
TTBR0 materialized-root provenance, a selected TTBR1 shared privileged
kernel-root policy, required privileged reachability records, compatibility-only
TCR/MAIR observations, blocked live activation surfaces, deterministic
rejections, idempotent plan-local teardown, and zero live side effects. Its
exact remaining blocker is kernel-half descriptor-image construction:
blocked-no-kernel-half-descriptor-image.

## Accepted Inputs

Accepted artifacts and evidence reviewed for this inventory:

- kernel-half reachability source inventory:
  1eda202e085d91759ad8b9e884772416516766bb.
- kernel-half reachability contract:
  7d95e0a7ae2041ff00a19ff89515cde9b11cb99f.
- QEMU/substitute kernel-half reachability smoke plan:
  c1645d5851e27b99f7aeee100782738bfdb4b093.
- kernel-half reachability core:
  a4294f5f6a89a44c5fcee41028e916d9655767b0.
- QEMU/substitute kernel-half reachability smoke core:
  edda81d340d919962b6856575451e201344abd29.
- kernel-half reachability closeout checkpoint:
  e2b91b87f12199838571f4e46277c09f8f998068.
- retained QEMU/substitute smoke evidence:
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.

The accepted input frontier is still below any live TTBR/TCR/MAIR/SCTLR write.
It selects process-user mappings in TTBR0_EL1 and a future TTBR1_EL1 shared
privileged kernel root, but no source yet builds, owns, validates, or tears down
that kernel-root descriptor image.

## Source Owners

- src/kernel_half_reachability.rs owns the accepted
  KernelHalfReachabilityPlan identity
  phase8-kernel-half-reachability-plan-v1, policy
  preflight-ttbr1-shared-kernel-root-reachability-v1, and current blocker
  blocked-no-kernel-half-descriptor-image. It records required kernel text,
  rodata, data, bss, vector, active-stack, heap, page-frame, UART/MMIO
  diagnostic, scheduler, and panic/fault reachability entries, plus
  privileged-only permission expectations. It does not allocate table pages,
  produce descriptor entries, or install a TTBR1 root.
- src/process_page_table_materialization.rs owns the closest accepted
  architecture-record precedent. It consumes ProgramImagePlan,
  ProcessImageInstallPlan, and ProcessAddressSpace records, leases a model
  root/table/user-frame set, emits ProcessPageDescriptorRecord entries for
  user mappings, proves rollback/teardown, and keeps
  kernel_mapping_policy=activation-blocked-no-kernel-half. Its descriptors are
  for user pages only and intentionally do not model kernel-half entries.
- src/memory_map/translation.rs owns current stage-1 descriptor vocabulary for
  early EL2 bring-up: table descriptors, 2 MiB normal/device block descriptors,
  MAIR normal/device attribute indices, inner-shareable/access-flag bits, PXN,
  UXN, table address masks, and the early translation register plan. Its
  production mutation path is still the current early kernel map, not a
  process-owned TTBR1 descriptor-image builder.
- linker.ld and linker-rpi5.ld own the current linked kernel ranges:
  __kernel_start, .text.boot, .vectors, .text, .rodata, .data,
  __kernel_image_end, .bss, __heap_start, __heap_end, __stack_bottom,
  __stack_top, and __kernel_end. They define source ranges, not the virtual
  half, descriptor permissions, or table ownership for a future shared kernel
  root.
- src/memory_map/layout.rs and src/memory_map/page_frames.rs own current
  conservative low-tail, bootstrap-reservation, and page-frame ownership
  vocabulary. They can inform future descriptor-image table-page sourcing and
  rollback rules, but no accepted slice transfers frames to a TTBR1 kernel-root
  image.
- src/arch/aarch64/exceptions.rs and src/arch/aarch64/vectors.S own exception
  vectors and saved-frame vocabulary. Descriptor-image construction must keep
  VBAR_EL1/vector reachability explicit, but this inventory does not change
  vector installation or lower-EL return behavior.
- src/mmio.rs, src/pl011.rs, src/runtime_console.rs, and src/tty.rs own current
  UART/MMIO diagnostic paths. A future descriptor image must distinguish device
  attributes from normal memory and keep EL0 access forbidden, but no device
  mapping is added here.
- src/scheduler.rs owns scheduler code/data, kernel stacks, tasks, run queues,
  and dispatch state. Descriptor-image construction must keep scheduler
  reachability separate from runnable publication and process-table mutation.
- src/live_address_space_activation.rs owns activation preflight records,
  copied TTBR0 root provenance, compatibility-only TCR/MAIR vocabulary, and
  blocked live side effects. The descriptor-image frontier must feed that
  lineage without writing live registers.

## Descriptor-Image Gap Map

| Area | Accepted state | Missing descriptor-image boundary |
| --- | --- | --- |
| Boundary identity | KernelHalfReachabilityPlan selects the TTBR1 shared privileged kernel-root policy. | A named non-installed descriptor-image record for that policy. |
| Input lineage | Loader/install/address-space/materialization/launch/stack/activation records are copied and checked. | A descriptor-image input record tying those accepted records to kernel range coverage. |
| Kernel ranges | Linker symbols and reachability booleans name required kernel areas. | Page- or block-aligned coverage records for text, rodata, data, bss, vectors, stack, heap, page-frame metadata, scheduler, and fault-reporting code/data. |
| Descriptor attributes | Permission policy requires privileged-only text execution, non-executable data, no normal-memory device mapping, and no EL0 kernel access. | Concrete descriptor attribute vocabulary for kernel text, read-only data, writable data, stacks/heap, and device MMIO. |
| Device mappings | UART/MMIO diagnostics are required for panic/fault reporting. | Device-nGnRE mapping intent, EL0-deny behavior, and deterministic rejection of device-as-normal-memory intent. |
| Table ownership | Process page-table materialization has model root/table/user-frame leases and rollback. | TTBR1 kernel-root/table lease records, no-partial rollback, idempotent teardown, and no transfer to live TTBR1. |
| Register state | TCR/MAIR are compatibility-only, SCTLR/ASID/TLB/barriers are blocked or planned-only. | Explicit proof that descriptor-image construction emits records only and leaves all live register/TLB/barrier state untouched. |
| Activation handoff | LiveAddressSpaceActivationPlan copies TTBR0 root provenance and blocks TTBR1/kernel-half installation. | A future input for activation that can replace blocked-no-kernel-half-descriptor-image without performing activation itself. |

## Candidate Boundary

The smallest objective next boundary is a documentation-only contract for a
non-installed KernelHalfDescriptorImage record associated with the accepted
KernelHalfReachabilityPlan. The boundary should:

- consume the accepted KernelHalfReachabilityPlan plus its copied Phase 8
  input lineage;
- preserve TTBR0 materialized-root provenance while constructing only a TTBR1
  shared privileged kernel-root descriptor image;
- describe kernel text, rodata, data, bss, vectors, active stack, heap,
  page-frame metadata, UART/MMIO diagnostics, scheduler code/data, and
  panic/fault-reporting coverage;
- define descriptor attribute expectations for privileged executable text,
  privileged non-executable writable data, privileged read-only data, device
  MMIO, no EL0 read/write/execute access, PXN/UXN behavior, shareability, AF,
  and normal/device MAIR attribute index intent;
- provide deterministic errors for missing accepted lineage, missing kernel
  ranges, missing diagnostic/fault-reporting coverage, forbidden EL0 access,
  bad device attributes, exhausted table leases, duplicate/overlapping ranges,
  forbidden live-register request, scheduler-publication request, and lower-EL
  launch request;
- guarantee all-or-nothing construction, no partial image publication,
  idempotent teardown, and preserved ownership of input records; and
- prove zero live TTBR/TCR/MAIR/SCTLR/TLB/barrier/lower-EL/scheduler/process/
  descriptor-table side effects.

This is deliberately below live activation. It should produce inspectable
records and model leases only; no accepted task should write TTBR1_EL1 or make
the kernel-half image architecturally active until a later contract selects
that sequence.

## Deferred Surfaces

This inventory keeps these surfaces blocked:

- live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation;
- ASID allocation, live TLB invalidation, and live DSB/ISB activation
  sequencing;
- lower-EL ERET and architectural register writes;
- scheduler runnable publication, process-table mutation, PID allocation,
  wait/exit, exec/spawn, and descriptor inheritance semantics;
- startup ABI expansion, argv/envp/auxv/TLS, libc framing, signal stacks,
  guard-fault recovery, copy-on-write, and demand paging;
- descriptor-backed filesystem syscalls, cwd/root, shell behavior, writable
  filesystem state, and persistent storage;
- Pi 5 hardware proof, boot archive publication, hardwareTestLock acquisition,
  TFTP/serial evidence, and physical serial claims;
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Recommendation

The mechanically next task should be
phase8-kernel-half-descriptor-image-contract-20260531, if queued dependencies
remain satisfied. The accepted closeout and current source owners make the
contract boundary objective: a non-installed KernelHalfDescriptorImage record
for the selected TTBR1 shared privileged kernel-root policy, below any live
register mutation or lower-EL launch.

Implementation should remain blocked until that contract and its
QEMU/substitute smoke plan are accepted.

## Reviewed Materials

- docs/src/project/phase8-kernel-half-reachability-source-inventory.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-qemu-kernel-half-reachability-smoke-plan.md
- docs/src/project/phase8-kernel-half-reachability-closeout-checkpoint.md
- tasks/2026-05-31-phase8-kernel-half-reachability-core.md
- tasks/2026-05-31-phase8-qemu-kernel-half-reachability-smoke-core.md
- tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log
- docs/src/project/phase8-process-page-table-materialization-source-inventory.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- src/kernel_half_reachability.rs
- src/process_page_table_materialization.rs
- src/memory_map/translation.rs
- src/memory_map/layout.rs
- linker.ld
- linker-rpi5.ld
- docs/src/architecture/memory.md
- docs/src/architecture/lower-el-userspace.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted kernel-half
  reachability docs, task records, retained QEMU/substitute evidence, process
  page-table materialization precedent, AArch64 translation descriptors,
  linker-owned kernel ranges, memory architecture notes, lower-EL architecture
  notes, roadmap, SUMMARY, and ADR index.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed by this
  documentation-only inventory.
