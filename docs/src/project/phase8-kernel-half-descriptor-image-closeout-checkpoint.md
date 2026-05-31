# Phase 8 Kernel-Half Descriptor-Image Closeout Checkpoint

Status: accepted

Task: phase8-kernel-half-descriptor-image-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the accepted Phase 8 Milestone 8.3 kernel-half
descriptor-image construction slice as documentation-only work. It reconciles
the source inventory, contract, QEMU/substitute smoke plan, implementation,
retained QEMU/substitute evidence, deferred surfaces, residual risks, and next
planning state.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- kernel-half descriptor-image source inventory:
  6cafdd8fc7673955adab8a91f9195e1a4a4da770.
- kernel-half descriptor-image contract:
  a3bc1610975027f5377c276ef45de345d6bbc83b.
- QEMU/substitute kernel-half descriptor-image smoke plan:
  ddaebb3897da014cadc3d4c76ddea79bd9709ba5.
- kernel-half descriptor-image core:
  3e0e83652e07581447cdb7f06c54a454e052ab83.
- QEMU/substitute kernel-half descriptor-image smoke core:
  424c1f3f754462f8496feeba6684bf3a4ae6738a.
- retained QEMU/substitute smoke evidence:
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log.

The retained smoke evidence contains:

- fixture identity phase8-program-loader-elf64-aarch64-v1.
- install boundary identity phase8-process-install-plan-v1.
- address-space boundary identity phase8-process-address-space-model-v1.
- materialization boundary identity
  phase8-process-page-table-materialization-v1.
- launch boundary identity phase8-initial-process-launch-plan-v1.
- stack boundary identity phase8-initial-user-stack-plan-v1.
- activation boundary identity phase8-live-address-space-activation-plan-v1.
- reachability boundary identity phase8-kernel-half-reachability-plan-v1.
- descriptor-image boundary identity phase8-kernel-half-descriptor-image-v1.
- descriptor-image policy
  ttbr1-shared-privileged-kernel-root-descriptor-image-v1.
- successful KernelHalfDescriptorImage construction with published=true and
  installed=false.
- TTBR0 materialized-root provenance with ttbr0-written=false.
- model-owned TTBR1 kernel-root image intent with ttbr1-written=false and
  descriptor-image-installed=false.
- required kernel text, rodata, data, bss, vectors, active stack, heap,
  page-frame, UART/MMIO diagnostic, scheduler, runtime console, and
  panic/fault-reporting coverage.
- privileged-only kernel text execution, read-only rodata, non-executable
  writable data, no normal-memory device mapping, no EL0 kernel access, and
  no W+X normal-memory policy.
- normal-memory inner-shareable and device-nGnRE attribute intent, AF state,
  denied user access, and exact-coverage evidence.
- model-owned root/table leases, no live-table borrow, preserved input-record
  ownership, and rollback-ready state.
- compatibility-only TCR/MAIR records and blocked SCTLR mutation.
- blocked ASID allocation, blocked live TLBI, planned-only no-live DSB/ISB,
  blocked live-register sequence, lower-EL ERET=false, and scheduler
  publication=false.
- side-effect evidence showing no TTBR/TCR/MAIR/SCTLR mutation, no descriptor
  image installation, no ASID allocation, no TLB mutation, no live DSB/ISB,
  no lower-EL ERET, no scheduler publication, no process-table mutation, and
  no descriptor-table mutation.
- idempotent teardown that clears descriptors, releases root/table leases,
  marks the image unpublished, preserves input records, and reports
  already-destroyed state on the second teardown.
- deterministic bad reachability plan, lineage mismatch, missing kernel
  coverage, forbidden EL0 access, writable text, executable data, bad device
  attribute intent, overlapping range, resource exhaustion, unsupported
  topology, and live activation request rejection cases with no partial image
  and no leaked leases.
- final classification:
  qemu-kernel-half-descriptor-image-smoke-complete.
- exact PASS line:
  qemu-kernel-half-descriptor-image-smoke: PASS.

## Accepted Frontier

The accepted capability is a target-independent, non-installed
KernelHalfDescriptorImage construction boundary for immutable /bin/init. It
consumes copied loader/install/address-space/materialization/launch/stack/
activation/reachability lineage, preserves TTBR0 materialized-root provenance,
builds a model-owned TTBR1 shared privileged kernel-root descriptor-image
intent, records required kernel coverage and privileged-only normal/device
descriptor attributes, owns root/table leases, rolls back failures without a
partial image, supports idempotent teardown, and proves zero live side
effects.

This does not prove descriptor-image installation, live TTBR0_EL1 or
TTBR1_EL1 programming, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation,
live TLB invalidation, live DSB/ISB activation sequencing, lower-EL ERET,
scheduler runnable publication, process table/PID/wait/exit behavior,
exec/spawn/wait, broad argv/envp/auxv/TLS ABI, descriptor-backed filesystem
syscalls, shell behavior, Pi 5 behavior, networking, or SSH.

## Deferred Surfaces

Still blocked after this checkpoint:

- descriptor-image installation into a live TTBR1 kernel root.
- live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation.
- ASID allocation and live TLB invalidation.
- live DSB/ISB activation sequencing.
- lower-EL ERET and architectural register writes.
- scheduler runnable publication and process table mutation.
- PID allocation, process lifecycle, wait/exit, exec/spawn, and descriptor
  inheritance semantics.
- broad argv/envp/auxv/TLS startup ABI and libc-compatible startup framing.
- descriptor-backed filesystem syscalls, writable filesystem state, and shell
  behavior.
- Pi 5 hardware proof, boot archive publication, and hardwareTestLock use.
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Next Planning State

No explicit queued follow-up task remains after this checkpoint. Supervisor
planning is required before the worker may promote another Phase 8.3 task.
Likely future frontiers include live descriptor-image installation,
translation-register activation, lower-EL launch setup, scheduler runnable
publication, or startup ABI expansion, but this checkpoint does not create or
select that task.
