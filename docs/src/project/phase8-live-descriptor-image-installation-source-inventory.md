# Phase 8 Live Descriptor-Image Installation Source Inventory

Status: accepted

Task: phase8-live-descriptor-image-installation-source-inventory-20260531

## Scope

This inventory maps the next Phase 8 Milestone 8.3 frontier after accepted
kernel-half descriptor-image closeout. It is documentation only and authorizes
no Rust behavior change, assembly behavior change, QEMU execution, Pi 5
hardware run, boot archive publication, hardwareTestLock acquisition, live
TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB
mutation, activation DSB/ISB sequence, lower-EL ERET, scheduler runnable
publication, process lifecycle, shell behavior, descriptor-backed filesystem
syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

The accepted KernelHalfDescriptorImage proves that Talos can construct an
inspectable, model-owned TTBR1 shared privileged kernel-root descriptor image
for immutable /bin/init. That evidence is still explicitly non-installed:
installed=false, descriptor-image-installed=false, ttbr1-written=false, and all
live translation side effects remain zero. The exact gap for this slice is the
installation boundary that can bind that descriptor image to the live
activation lineage without yet writing architectural translation registers or
launching lower EL.

## Accepted Inputs

Accepted artifacts and evidence reviewed for this inventory:

- live address-space activation source inventory:
  7f645691ea423bdc38e9bf04a27f75ce967984a5.
- live address-space activation contract:
  89c624b95f39739e67168c4e6465a61ee18f345d.
- live address-space activation core:
  129337734011004297da0b2768a3a802063c3293.
- QEMU/substitute live address-space activation smoke core:
  1c441c301387ed75e24db7f9788301126f1f5a72.
- kernel-half reachability closeout checkpoint:
  e2b91b87f12199838571f4e46277c09f8f998068.
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
- kernel-half descriptor-image closeout checkpoint:
  448a95dac8fb24bc8d99c07c4fb056df7ea06d79.
- retained QEMU/substitute descriptor-image smoke evidence:
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log.

The accepted input frontier is still below any live TTBR/TCR/MAIR/SCTLR write.
It has a concrete TTBR1 descriptor image, but no source yet owns the transition
from non-installed image evidence to an activation-owned installed-image record.

## Source Owners

- src/kernel_half_descriptor_image.rs owns the accepted
  KernelHalfDescriptorImage identity phase8-kernel-half-descriptor-image-v1,
  policy ttbr1-shared-privileged-kernel-root-descriptor-image-v1, TTBR0
  materialized-root provenance, model-owned TTBR1 root/table leases, descriptor
  coverage records, permission policy, compatibility-only TCR/MAIR state,
  blocked SCTLR/ASID/TLB/barrier/live-register state, idempotent teardown, and
  zero side-effect counters. It deliberately reports installed=false and
  descriptor_image_installed=false.
- src/live_address_space_activation.rs owns the accepted
  LiveAddressSpaceActivationPlan identity
  phase8-live-address-space-activation-plan-v1, model-only activation state,
  TTBR0 provenance, TTBR1/kernel-half blocker vocabulary, kernel reachability
  prerequisites, deterministic no-partial activation behavior, and commit
  rejections for live registers, runnable publication, and lower-EL launch.
  It is the natural consumer for an installation-ready kernel-half image, but
  it currently has no field or transition that can accept one.
- src/kernel_half_reachability.rs owns the policy bridge from activation
  preflight to TTBR1 shared privileged kernel-root reachability. It proves the
  source reachability requirements that descriptor-image installation must
  preserve, but it does not own install state.
- src/process_page_table_materialization.rs owns TTBR0 materialized process
  root provenance, descriptor-image precedent, activation_blocked reporting,
  rollback, and teardown. Installation must preserve this TTBR0/TTBR1 split
  instead of merging user and kernel mappings into one ambiguous root.
- src/memory_map/translation.rs owns early AArch64 descriptor and translation
  register vocabulary. It can supply naming for table roots, descriptor
  attributes, MAIR/TCR compatibility, and future register sequencing, but its
  current production mutation path is still the early kernel map, not a
  per-process TTBR1 installation mechanism.
- linker.ld and linker-rpi5.ld own the linked kernel range symbols that back
  the descriptor image. Installation must not silently widen, truncate, or
  reinterpret those ranges.
- src/arch/aarch64/exceptions.rs and src/arch/aarch64/vectors.S own exception
  vector reachability and fault-reporting vocabulary. Installation cannot rely
  on a user mapping to report its own failure.
- src/mmio.rs, src/pl011.rs, src/runtime_console.rs, and src/tty.rs own current
  UART/MMIO diagnostics and runtime console state. Installation must preserve
  device attribute intent and EL0 denial.
- src/scheduler.rs owns task, kernel-stack, run-queue, and dispatch state.
  Descriptor-image installation must stay separate from runnable publication
  and process-table mutation.

## Installation Gap Map

| Area | Accepted state | Missing installation boundary |
| --- | --- | --- |
| Descriptor image | KernelHalfDescriptorImage constructs an owned TTBR1 descriptor image and proves installed=false. | A named record that marks the image installation-ready or activation-installed without writing TTBR1_EL1. |
| Activation handoff | LiveAddressSpaceActivationPlan blocks TTBR1/kernel-half installation with model-only activation state. | A consumer relationship from activation preflight to the descriptor image, plus copied identity checks and no-stale-input rejection. |
| Ownership | Descriptor image owns root/table leases and teardown; activation owns plan-local records. | Ownership rules for an installed image record, including who may tear it down and how teardown interacts with activation plan lifetime. |
| Fault reporting | Reachability and descriptor-image records require vectors/UART/panic paths. | Installation must prove those diagnostics remain kernel-owned before admitting a live-register sequence later. |
| Rollback | Descriptor-image construction rolls back failed construction. | Installation must roll back failed binding without marking the image installed or leaking activation-owned state. |
| Register state | TCR/MAIR/SCTLR/ASID/TLB/barrier state remains blocked or compatibility-only. | Installation must keep live TTBR/TCR/MAIR/SCTLR mutation blocked unless a later contract explicitly admits it. |
| Evidence | QEMU/substitute smoke proves non-installed descriptor-image construction. | A future smoke must distinguish installation-ready state from the already accepted non-installed construction evidence. |

## Boundary Recommendation

The next boundary should remain target-independent and preflight-installation
only. It may perform a narrow model-level descriptor-image installation into
the activation lineage, but it must stay below TTBR activation.

The recommended contract should define a KernelHalfDescriptorImageInstallation
record or equivalent activation-extension record that:

- consumes one accepted KernelHalfDescriptorImage, one
  LiveAddressSpaceActivationPlan, and the copied Phase 8 lineage;
- verifies the descriptor-image boundary identity, policy identity, TTBR0
  provenance, reachability identity, source digest, entry point, initial SP,
  kernel coverage, permission policy, compatibility states, and zero live side
  effects;
- changes only model state from non-installed descriptor-image evidence to an
  installation-ready activation binding;
- records that TTBR1_EL1 has not been written, the live register sequence is
  still blocked, ASID/TLB/barrier activation is still blocked, and lower-EL
  ERET and scheduler publication remain blocked;
- defines rollback and teardown for the installed binding without destroying
  input descriptor-image or activation records unless their existing teardown
  owners request that separately; and
- rejects stale, destroyed, wrong-identity, partial, mismatched, or live
  activation requests deterministically with no partial installation state.

This recommendation is not a claim that live translation registers can be
mutated. It is the smallest objective step that turns accepted non-installed
descriptor-image evidence into a contractible installation handoff while
preserving the current no-live-side-effect invariant.

## Invariants And Blockers

Any later contract must preserve these invariants:

- TTBR0 materialized-root provenance remains copied from the accepted process
  page-table materialization and is not rewritten.
- TTBR1 kernel-root provenance may point at the accepted descriptor image only
  as model-owned installation state until a later task accepts live register
  mutation.
- Kernel text, rodata, data, bss, vectors, active stack, heap, page-frame
  metadata, UART/MMIO diagnostics, scheduler state, runtime console, and
  panic/fault reporting remain privileged-only and covered by the accepted
  descriptor policy.
- Device mappings remain device memory, not normal memory, and EL0 access is
  denied for all kernel-half mappings.
- VBAR_EL1, exception vectors, active kernel stack, UART/MMIO diagnostics, and
  panic/fault reporting remain available before any live activation attempt.
- No ASID allocation, live TLBI, activation DSB/ISB, lower-EL ERET, scheduler
  publication, process-table mutation, descriptor-table mutation, filesystem
  syscall behavior, or Pi 5 hardware proof is implied by installation.

Remaining blockers:

- blocked-no-live-register-sequence;
- blocked-no-asid-allocation;
- blocked-no-live-tlbi;
- blocked-no-live-dsb-isb;
- blocked-no-lower-el-eret;
- blocked-no-runnable-publication;
- blocked-no-process-lifecycle;
- blocked-no-startup-abi-expansion;
- blocked-no-filesystem-syscalls; and
- blocked-no-pi5-hardware-proof.

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
phase8-live-descriptor-image-installation-contract-20260531, if queued
dependencies remain satisfied. The accepted closeout and current source owners
make the contract boundary objective: a target-independent installation-ready
binding between the accepted non-installed KernelHalfDescriptorImage and the
accepted LiveAddressSpaceActivationPlan, below live translation-register
mutation and lower-EL launch.

Implementation should remain blocked until that contract and its
QEMU/substitute smoke plan are accepted.

## Reviewed Materials

- docs/src/project/phase8-live-address-space-activation-source-inventory.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-kernel-half-reachability-closeout-checkpoint.md
- docs/src/project/phase8-kernel-half-descriptor-image-source-inventory.md
- docs/src/project/phase8-kernel-half-descriptor-image-contract.md
- docs/src/project/phase8-kernel-half-descriptor-image-closeout-checkpoint.md
- tasks/2026-05-31-phase8-kernel-half-descriptor-image-closeout-checkpoint.md
- tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log
- src/kernel_half_descriptor_image.rs
- src/live_address_space_activation.rs
- src/kernel_half_reachability.rs
- src/process_page_table_materialization.rs
- src/memory_map/translation.rs
- linker.ld
- linker-rpi5.ld
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted live activation,
  kernel-half reachability, kernel-half descriptor-image docs and task records,
  retained QEMU/substitute descriptor-image evidence, relevant source owners,
  linker scripts, roadmap, SUMMARY, and ADR index.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed by this
  documentation-only inventory.
