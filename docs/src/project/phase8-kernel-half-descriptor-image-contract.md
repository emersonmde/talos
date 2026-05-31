# Phase 8 Kernel-Half Descriptor-Image Contract

Status: accepted documentation-only contract for
phase8-kernel-half-descriptor-image-contract-20260531.

This contract follows the accepted
[Phase 8 Kernel-Half Descriptor-Image Source Inventory](phase8-kernel-half-descriptor-image-source-inventory.md).
It selects the first kernel-half descriptor-image boundary after accepted
kernel-half reachability preflight. It adds no Rust behavior, assembly
behavior, QEMU execution, Pi 5 hardware run, boot archive publication,
hardware-lock acquisition, live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1,
SCTLR_EL1, ASID, TLBI, DSB, or ISB mutation, lower-EL ERET, scheduler
runnable publication, process lifecycle, shell behavior, descriptor-backed
filesystem syscalls, writable filesystem, persistent storage, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Selected Boundary

The next implementation boundary should be a non-installed
KernelHalfDescriptorImage record. It consumes the accepted
KernelHalfReachabilityPlan and its copied Phase 8 input lineage, then produces
an inspectable TTBR1 shared privileged kernel-root descriptor-image intent. It
does not write TTBR1_EL1, copy descriptors into an active root, change live
TCR_EL1 or MAIR_EL1, invalidate live TLB state, publish scheduler state,
create a runnable process, or ERET to lower EL.

Accepted inputs:

- KernelHalfReachabilityPlan from src/kernel_half_reachability.rs, including
  boundary identity phase8-kernel-half-reachability-plan-v1, policy
  preflight-ttbr1-shared-kernel-root-reachability-v1, copied loader/install/
  address-space/materialization/launch/stack/activation lineage, required
  reachability entries, privileged-only permission expectations, and current
  blocked-no-kernel-half-descriptor-image state.
- ProcessPageTableMaterialization from
  src/process_page_table_materialization.rs, including TTBR0 materialized-root
  provenance and descriptor-image precedent for owned image construction,
  rollback, idempotent teardown, and activation-blocked reporting.
- Linker-owned kernel ranges from linker.ld and linker-rpi5.ld, including
  kernel start, text, vectors, rodata, data, bss, heap, stack, image end, and
  kernel end symbols.
- AArch64 stage-1 descriptor vocabulary from src/memory_map/translation.rs,
  including valid table/page/block descriptor bits, normal/device attribute
  index intent, AF, shareability, UXN, PXN, and table address masks.
- Memory ownership, exception-vector, UART/MMIO diagnostic, runtime console,
  scheduler, panic/fault-reporting, and live activation records named by the
  source inventory.

Accepted output:

- one KernelHalfDescriptorImage record whose boundary identity should be
  phase8-kernel-half-descriptor-image-v1;
- selected policy identity
  ttbr1-shared-privileged-kernel-root-descriptor-image-v1;
- copied input lineage for the accepted kernel-half reachability plan,
  materialized TTBR0 root provenance, loader/install/address-space/
  materialization/launch/stack/activation records, and linker/source-owner
  range evidence;
- an owned non-installed TTBR1 kernel-root image intent with root/table lease
  records, descriptor records, and rollback/teardown state;
- coverage records for kernel text, rodata, data, bss, vectors, active kernel
  stack, heap, page-frame metadata, UART/MMIO diagnostics, scheduler
  code/data, runtime console, panic/fault reporting, and exception-vector
  reachability;
- descriptor attribute records for privileged executable text/vectors,
  privileged read-only data, privileged non-executable writable data/stack/
  heap/scheduler state, and device MMIO diagnostics; and
- side-effect observations proving zero live TTBR/TCR/MAIR/SCTLR/TLB/barrier,
  lower-EL, scheduler, process-table, and descriptor-table effects.

The selected boundary deliberately remains below live address-space
activation. It produces inspectable records and model-owned image leases only;
it must not make the descriptor image architecturally active.

## Descriptor Policy

Kernel-half descriptor-image construction must preserve the accepted
privileged-only reachability policy:

| Mapping class | Required descriptor intent |
| --- | --- |
| Kernel text and vectors | Normal memory, privileged executable, EL0 access denied, writable denied, UXN clear only where execution is required, PXN behavior recorded for privileged execution policy. |
| Kernel rodata | Normal memory, privileged read-only, EL0 access denied, writable denied, executable denied. |
| Kernel data, bss, stacks, heap, page-frame metadata, scheduler state, runtime console state, and panic/fault data | Normal memory, privileged read-write, EL0 access denied, executable denied. |
| UART/MMIO diagnostics | Device memory, privileged read/write as required for diagnostics, EL0 access denied, executable denied, never represented as normal memory. |

Every descriptor record must carry a source range, virtual range, descriptor
kind, access policy, memory attribute intent, shareability intent, AF state,
UXN/PXN state, EL0-deny observation, owner, and source evidence label. The
first implementation may use page or block records only where alignment,
coverage, and attribute uniformity make the representation exact. It must
reject any silent widening, range truncation, or attribute coalescing that
would hide a policy difference.

The descriptor image must keep TTBR0 and TTBR1 responsibilities separate.
User mappings remain provenance-only TTBR0 input from the accepted
ProcessPageTableMaterialization. Kernel mappings belong to the new TTBR1
shared privileged kernel-root image. No accepted descriptor record may give
EL0 read, write, or execute access to kernel-half mappings.

## Ownership And Rollback

Descriptor-image construction must use explicit model-owned root/table leases.
The first implementation may use deterministic target-independent fixtures or
future allocator-backed records, but the published contract is the same:

- no root or table lease is borrowed from live early translation tables;
- no descriptor image is published until all inputs, ranges, attributes,
  table leases, descriptor records, and coverage requirements have passed;
- failure releases every acquired root/table lease and returns no partial
  KernelHalfDescriptorImage;
- teardown is idempotent, clears descriptor records, releases model-owned
  leases, marks the image unpublished, and reports already-destroyed state on
  later teardown calls; and
- input records retain their original ownership and are never mutated by
  descriptor-image construction.

The record must distinguish model ownership from future hardware ownership.
It may describe a TTBR1-compatible root, but it must not transfer that root to
TTBR1_EL1, a process table, a scheduler task, or a live activation record.

## Error Matrix

| Condition | Required error or blocker |
| --- | --- |
| Missing, destroyed, wrong-identity, or internally inconsistent KernelHalfReachabilityPlan | EINVAL |
| Accepted lineage does not match loader/install/address-space/materialization/launch/stack/activation provenance | EINVAL |
| Required kernel text, rodata, data, bss, vector, stack, heap, page-frame, UART/MMIO, scheduler, runtime-console, panic/fault, or exception-vector coverage is missing | EINVAL |
| Duplicate, overlapping, unaligned, overflowed, out-of-kernel, or unsupported source/virtual range | EINVAL |
| Kernel mapping grants EL0 access, writable text, executable data, executable device memory, or W+X normal memory | EACCES |
| Device MMIO is requested with normal-memory attributes or normal memory is requested with device attributes | EACCES |
| Missing root/table lease capacity or descriptor-record capacity | ENOMEM |
| Unsupported exact table topology for this first descriptor-image boundary | ENOTSUP |
| Caller asks for live TTBR/TCR/MAIR/SCTLR mutation, ASID allocation, TLB invalidation, activation DSB/ISB, lower-EL launch, scheduler publication, process lifecycle mutation, or descriptor-table publication | ENOSYS |

Errors must be deterministic and must leave no visible partial image, no
descriptor publication, no live-register side effect, no lower-EL launch, and
no scheduler/process-table side effect.

## Side-Effect Boundary

The descriptor image is inspectable evidence only. Successful construction may
record compatibility observations for future activation, including TTBR1 root
intent, TCR/MAIR compatibility labels, and blocker replacement from
blocked-no-kernel-half-descriptor-image to descriptor-image-ready. Those
observations are not live register writes.

The following remain explicitly blocked:

- live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation;
- ASID allocation, live TLB invalidation, and live DSB/ISB activation
  sequencing;
- lower-EL ERET and architectural saved-frame installation;
- scheduler runnable publication, process-table mutation, PID allocation,
  wait/exit, exec/spawn, descriptor inheritance, and descriptor-table
  publication;
- startup ABI expansion, argv/envp/auxv/TLS, signals, copy-on-write, demand
  paging, and guard-fault recovery;
- descriptor-backed filesystem syscall expansion, shell behavior, writable
  filesystem state, and persistent storage;
- Pi 5 hardware proof, boot archive publication, hardware-lock acquisition,
  TFTP/serial evidence, and physical serial claims; and
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Next Evidence Boundary

The mechanically next documentation-only task should be
phase8-qemu-kernel-half-descriptor-image-smoke-plan-20260531, if queued
dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
descriptor-image boundary:

- exact scenario or substitute command identity;
- retained evidence path under
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/;
- boundary identity phase8-kernel-half-descriptor-image-v1;
- policy identity ttbr1-shared-privileged-kernel-root-descriptor-image-v1;
- classification line and PASS line;
- success observations for accepted input lineage, kernel range coverage,
  descriptor attributes, root/table lease ownership, no-partial-image
  construction, idempotent teardown, and zero live activation side effects;
- deterministic rejection observations for malformed lineage, missing
  reachability coverage, forbidden EL0 access, bad device attributes,
  duplicate/overlapping ranges, resource exhaustion, unsupported topology, and
  live activation requests; and
- conditional regression gates for kernel-half reachability, live activation,
  and process page-table materialization smoke evidence if shared owners are
  touched.

Implementation remains blocked until this contract and the QEMU/substitute
smoke plan are both accepted. Pi 5 hardware proof remains blocked until a
later explicit hardware-proof plan exists.

## Reviewed Inputs

- docs/src/project/phase8-kernel-half-descriptor-image-source-inventory.md
- docs/src/project/phase8-kernel-half-reachability-closeout-checkpoint.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-qemu-kernel-half-reachability-smoke-plan.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md
- tasks/2026-05-31-phase8-kernel-half-descriptor-image-source-inventory.md
- tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log
- src/kernel_half_reachability.rs
- src/process_page_table_materialization.rs
- src/memory_map/translation.rs
- src/memory_map/layout.rs
- src/memory_map/page_frames.rs
- src/arch/aarch64/exceptions.rs
- src/arch/aarch64/vectors.S
- src/mmio.rs
- src/pl011.rs
- src/runtime_console.rs
- src/scheduler.rs
- linker.ld
- linker-rpi5.ld
- docs/src/architecture/memory.md
- docs/src/architecture/lower-el-userspace.md
- docs/src/architecture/exceptions.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted descriptor-image
  source inventory, kernel-half reachability closeout/contract/smoke plan,
  retained reachability smoke evidence, process page-table materialization
  precedent, live activation contract and closeout, source owners for
  kernel-half reachability, page-table materialization, translation
  descriptors, linker ranges, memory ownership, exception/vector,
  UART/MMIO/runtime-console, scheduler, and architecture notes.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: hardwareTestLock remained unlocked/restored and unused; no Pi 5
  archive publication, power cycle, TFTP action, or serial observation was
  performed by this documentation-only contract.
