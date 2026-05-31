# Phase 8 Live Descriptor-Image Installation Contract

Status: accepted documentation-only contract for
phase8-live-descriptor-image-installation-contract-20260531.

This contract follows the accepted
[Phase 8 Live Descriptor-Image Installation Source Inventory](phase8-live-descriptor-image-installation-source-inventory.md).
It selects the first target-independent installation-ready binding between
the accepted non-installed KernelHalfDescriptorImage and the accepted
LiveAddressSpaceActivationPlan. It adds no Rust behavior, assembly behavior,
QEMU execution, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/
SCTLR_EL1 mutation, ASID allocation, live TLB invalidation, activation
DSB/ISB sequencing, lower-EL ERET, scheduler runnable publication, process
lifecycle, shell behavior, descriptor-backed filesystem syscalls, writable
filesystem, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Selected Boundary

The next implementation boundary should be a
KernelHalfDescriptorImageInstallation record, or an equivalent activation
extension record with the same externally inspectable contract. It consumes
one accepted KernelHalfDescriptorImage and one accepted
LiveAddressSpaceActivationPlan, verifies their copied Phase 8 lineage, and
produces a model-level installation-ready binding below TTBR activation.

This selected boundary is not live architectural installation. It does not
write TTBR1_EL1, copy descriptors into the currently active translation root,
change TCR_EL1 or MAIR_EL1, enable SCTLR_EL1.M, allocate an ASID, invalidate
live TLB state, run activation barriers, publish a scheduler task, mutate a
process table, expose descriptor-backed filesystem syscalls, or ERET to lower
EL.

Accepted inputs:

- KernelHalfDescriptorImage from src/kernel_half_descriptor_image.rs,
  including boundary identity phase8-kernel-half-descriptor-image-v1, policy
  identity ttbr1-shared-privileged-kernel-root-descriptor-image-v1,
  published=true, installed=false, descriptor_image_installed=false,
  ttbr1-written=false, copied TTBR0 materialized-root provenance,
  model-owned TTBR1 root/table leases, descriptor coverage records,
  permission policy, compatibility-only TCR/MAIR state, blocked live-register
  state, idempotent teardown state, and zero side-effect counters.
- LiveAddressSpaceActivationPlan from src/live_address_space_activation.rs,
  including boundary identity phase8-live-address-space-activation-plan-v1,
  selected policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1, copied loader/
  install/address-space/materialization/launch/stack lineage, TTBR0 root
  provenance, model-only activation state, TTBR1/kernel-half blocker
  vocabulary, kernel reachability checklist, fault-reporting prerequisites,
  deterministic no-partial activation behavior, and zero live side effects.
- KernelHalfReachabilityPlan from src/kernel_half_reachability.rs, as copied
  by the descriptor image, including TTBR1 shared privileged kernel-root
  policy, privileged-only reachability requirements, device attribute intent,
  exception-vector and UART/MMIO diagnostic requirements, and blocked live
  activation states.
- ProcessPageTableMaterialization provenance from
  src/process_page_table_materialization.rs, including the TTBR0
  materialized-root lease and descriptor-image precedent for ownership,
  rollback, and teardown.
- Linker-owned kernel range evidence from linker.ld and linker-rpi5.ld, plus
  descriptor, register, exception-vector, UART/MMIO, runtime-console,
  scheduler, and fault-reporting vocabulary named by the source inventory.

Accepted output:

- one installation-ready binding whose boundary identity should be
  phase8-live-descriptor-image-installation-v1;
- selected policy identity
  model-installed-ttbr1-descriptor-image-below-live-registers-v1;
- copied identities for descriptor image, activation plan, reachability plan,
  TTBR0 materialized root, loader/install/address-space/materialization/
  launch/stack records, source digest, entry point, and initial SP;
- copied kernel-half descriptor-image coverage and permission observations for
  text, vectors, rodata, data, bss, active stack, heap, page-frame metadata,
  UART/MMIO diagnostics, scheduler state, runtime console, panic/fault
  reporting, and exception-vector reachability;
- a model-owned installed binding state that changes only installation record
  state from non-installed descriptor-image evidence to
  installation-ready-activation-binding;
- explicit live state proving TTBR1_EL1 has not been written, TTBR0_EL1 is
  not rewritten, TCR_EL1/MAIR_EL1/SCTLR_EL1 remain record-only or blocked,
  ASID/TLB/barrier activation remains blocked, and lower-EL launch remains
  blocked; and
- side-effect observations proving no live register, live table mutation,
  scheduler, process-table, descriptor-table, filesystem, QEMU hardware, or
  Pi 5 hardware side effect occurred.

## Installation Invariant

Before the binding may be published, these facts must be true and
inspectable:

- the descriptor image and activation plan are both published, not destroyed,
  and share the same copied Phase 8 lineage;
- the descriptor image is still non-installed as hardware state:
  installed=false, descriptor_image_installed=false, and ttbr1-written=false;
- TTBR0 materialized-root provenance in the activation plan and descriptor
  image matches exactly and is not rewritten by installation;
- the descriptor image policy remains a TTBR1 shared privileged kernel-root
  policy with EL0 access denied for every kernel-half mapping;
- kernel text, vectors, rodata, data, bss, active stack, heap, allocator
  metadata, scheduler state, runtime console, UART/MMIO diagnostics, and
  panic/fault reporting remain covered by the accepted descriptor policy;
- VBAR_EL1, exception vectors, active kernel stack, UART/MMIO diagnostics,
  and panic/fault reporting remain kernel-owned prerequisites and do not
  depend on user mappings;
- device mappings retain device attributes and are not reclassified as normal
  memory;
- TCR_EL1 and MAIR_EL1 values remain compatibility records only;
- ASID allocation, live TLBI, activation DSB/ISB, and SCTLR_EL1 mutation
  remain blocked; and
- no scheduler runnable state, process-table entry, descriptor table,
  filesystem syscall behavior, or lower-EL saved-frame state becomes
  publishable through this binding.

The installed binding may be used by a later activation contract as model
evidence that the accepted TTBR1 descriptor image is the selected kernel-half
candidate. It must not be used as evidence that architectural translation
state has changed.

## Policy Fields

The record must expose stable field names so a later QEMU/substitute smoke can
retain evidence without interpreting private implementation details:

| Field | Required value for this boundary |
| --- | --- |
| boundary identity | phase8-live-descriptor-image-installation-v1 |
| selected policy | model-installed-ttbr1-descriptor-image-below-live-registers-v1 |
| descriptor image identity | phase8-kernel-half-descriptor-image-v1 |
| descriptor image policy | ttbr1-shared-privileged-kernel-root-descriptor-image-v1 |
| activation plan identity | phase8-live-address-space-activation-plan-v1 |
| TTBR0 provenance | copied materialized process root; no rewrite |
| TTBR1 provenance | copied descriptor-image kernel root; no register write |
| installation state | installation-ready-activation-binding |
| live register state | blocked-no-live-register-sequence |
| live table state | no-active-root-copy-or-mutation |
| TCR_EL1 state | compatibility-record-only |
| MAIR_EL1 state | compatibility-record-only |
| SCTLR_EL1 state | mutation-blocked |
| ASID state | blocked-no-asid-allocation |
| TLB state | blocked-no-live-tlbi |
| barriers | planned-only-no-live-dsb-isb |
| lower-EL launch | blocked-no-lower-el-eret |
| runnable publication | blocked-no-runnable-publication |
| side effects | zero-live-activation-side-effects |

## Deterministic Errors And Blockers

Errors must be deterministic and leave no visible partial installation state.

| Condition | Required result |
| --- | --- |
| Missing, destroyed, unpublished, wrong-identity, or internally inconsistent descriptor image or activation plan | EINVAL |
| Descriptor image and activation plan copied lineage, TTBR0 provenance, source digest, entry point, initial SP, reachability identity, or policy identity disagree | ENOEXEC |
| Descriptor image is already hardware-installed, already marked installed by another activation binding, or records nonzero live TTBR/TCR/MAIR/SCTLR/TLB/barrier side effects | EBUSY |
| Kernel coverage is missing, silently widened/truncated, overlaps user/device/null-guard space incorrectly, or grants EL0 access to kernel-half mappings | EACCES |
| UART/MMIO diagnostics lose device attributes, fault reporting loses kernel-owned vector/stack/UART prerequisites, or panic/fault reporting would depend on user mappings | EACCES |
| Installation record capacity, copied coverage capacity, or deterministic fixture capacity is exhausted | ENOMEM |
| Caller asks for live TTBR0_EL1/TTBR1_EL1 write, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, live TLBI, activation DSB/ISB, lower-EL ERET, scheduler publication, process-table mutation, descriptor-table publication, filesystem syscall behavior, Pi 5 proof, boot archive publication, or hardware-lock use | ENOSYS |

The binding should preserve explicit blocker vocabulary:

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

These blockers are contract facts, not implementation failures.

## Ownership, Rollback, And Teardown

Published installation state is owned by the installation record. It may refer
to the accepted descriptor image and activation plan, but it must not destroy,
release, or mutate their existing owned roots, tables, frames, ranges, launch
records, stack records, or teardown state.

Construction is all-or-nothing:

1. no installation record is published until input identities, copied lineage,
   kernel coverage, permission policy, diagnostic prerequisites, and
   side-effect counters are consistent;
2. failure returns no partial installed binding and leaves the descriptor
   image installed=false and descriptor_image_installed=false;
3. failure leaves LiveAddressSpaceActivationPlan model-only and
   live-register blocked;
4. rollback releases only installation-record-local capacity;
5. teardown is idempotent and clears only the installation binding state;
6. teardown does not release descriptor-image root/table leases or activation
   plan records; and
7. teardown records no live translation, scheduler, process, descriptor-table,
   filesystem, or hardware side effects.

The first implementation may use target-independent model records only. A
future contract must separately admit any transfer from model-owned installed
binding to architectural TTBR1_EL1 ownership.

## Smoke And Implementation Gates

The mechanically next documentation-only task should be
phase8-qemu-live-descriptor-image-installation-smoke-plan-20260531, if queued
dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
installation-ready binding:

- scenario identity qemu_live_descriptor_image_installation_smoke;
- retained evidence path under
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/;
- boundary identity phase8-live-descriptor-image-installation-v1;
- selected policy
  model-installed-ttbr1-descriptor-image-below-live-registers-v1;
- classification qemu-live-descriptor-image-installation-smoke-complete and
  PASS vocabulary;
- success observations for copied descriptor-image and activation identities,
  copied TTBR0 provenance, copied TTBR1 descriptor-image root provenance,
  reachability/coverage/permission preservation, installation-ready binding,
  rollback/teardown, and zero live side effects;
- deterministic rejection observations for stale or destroyed inputs, lineage
  mismatch, already-installed input, forbidden EL0 kernel access, diagnostic
  reachability loss, capacity exhaustion, and live-register requests; and
- conditional regression gates for live activation, kernel-half reachability,
  kernel-half descriptor-image, and process page-table materialization smoke
  evidence if shared owners are touched.

The later implementation task must at minimum run cargo fmt, the Rust test
suite, git diff --check, mdbook build if docs are touched, and git diff
--cached --check before commit. It should run the accepted QEMU/substitute
smoke once that script exists. Pi 5 hardware evidence, boot archive
publication, lower-EL user execution, scheduler runnable publication, and
filesystem syscall behavior remain blocked until separately planned and
accepted.

## Deferred Surfaces

This contract explicitly defers:

- live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB,
  and ISB mutation;
- active-root descriptor copy, architectural TTBR1 installation, live kernel
  reachability proof, and live activation fault recovery;
- lower-EL ERET, SP_EL0/ELR_EL1/SPSR_EL1 architectural writes, trap return,
  and loaded /bin/init execution;
- scheduler runnable publication, current-process state, process table/PID
  allocation, parent/child lifecycle, exit, wait, exec, spawn, signals, and
  credentials;
- argv/envp/auxv/TLS expansion, libc startup, dynamic stack growth,
  guard-fault recovery, copy-on-write, and demand paging;
- descriptor inheritance, close-on-exec, cwd/root, descriptor-backed
  filesystem syscalls, writable filesystem state, and shell behavior;
- Pi 5 hardware proof, boot archive publication, hardwareTestLock
  acquisition, TFTP/serial evidence, and physical serial claims; and
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Reviewed Inputs

- docs/src/project/phase8-live-descriptor-image-installation-source-inventory.md
- docs/src/project/phase8-kernel-half-descriptor-image-closeout-checkpoint.md
- docs/src/project/phase8-kernel-half-descriptor-image-contract.md
- docs/src/project/phase8-qemu-kernel-half-descriptor-image-smoke-plan.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- tasks/2026-05-31-phase8-live-descriptor-image-installation-source-inventory.md
- tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log
- src/kernel_half_descriptor_image.rs
- src/live_address_space_activation.rs
- src/kernel_half_reachability.rs
- src/process_page_table_materialization.rs
- src/memory_map/translation.rs
- linker.ld
- linker-rpi5.ld
- docs/src/architecture/memory.md
- docs/src/architecture/lower-el-userspace.md
- docs/src/architecture/exceptions.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted live
  descriptor-image installation source inventory, kernel-half descriptor-image
  closeout/contract/smoke plan, retained descriptor-image smoke evidence,
  kernel-half reachability contract, live address-space activation contract
  and closeout, process page-table materialization contract, source owners for
  descriptor-image, live activation, reachability, TTBR0 materialization,
  translation descriptors, linker ranges, exception/vector, UART/MMIO/runtime
  console, scheduler, and architecture notes.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: hardwareTestLock remained unlocked/restored and unused; no Pi 5
  archive publication, power cycle, TFTP action, or serial observation was
  performed by this documentation-only contract.
