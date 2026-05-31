# Phase 8 Live Translation-Register Activation Contract

Status: accepted documentation-only contract for
phase8-live-translation-register-activation-contract-20260531.

This contract follows the accepted
[Phase 8 Live Translation-Register Activation Source Inventory](phase8-live-translation-register-activation-source-inventory.md).
It selects a target-independent activation-commit model below architectural
translation-register mutation. It adds no Rust behavior, assembly behavior,
QEMU execution, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/
SCTLR_EL1 write, active-root descriptor copy, ASID allocation, live TLB
invalidation, activation DSB/ISB execution, lower-EL ERET, scheduler runnable
publication, process lifecycle, shell behavior, descriptor-backed filesystem
syscalls, writable filesystem, persistent storage, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

## Selected Boundary

The next implementation boundary should be a
LiveTranslationRegisterActivation model record, or an equivalent extension to
the accepted installation record with the same externally inspectable contract.
It consumes one accepted KernelHalfDescriptorImageInstallation, verifies copied
Phase 8 lineage and register-compatibility facts, and produces a deterministic
activation-commit intent that remains model/substitute-only.

This selected boundary is not a live architectural register sequence. It does
not write TTBR0_EL1 or TTBR1_EL1, change TCR_EL1 or MAIR_EL1, mutate
SCTLR_EL1.M, copy descriptor images into the currently active root, allocate an
ASID, invalidate live TLB state, execute activation barriers, publish a
scheduler task, mutate process or descriptor-table state, expose filesystem
syscalls, or ERET to lower EL.

Accepted inputs:

- KernelHalfDescriptorImageInstallation from
  src/live_descriptor_image_installation.rs, including boundary identity
  phase8-live-descriptor-image-installation-v1, selected policy
  model-installed-ttbr1-descriptor-image-below-live-registers-v1, copied
  descriptor-image and activation identities, TTBR0 materialized-root
  provenance, TTBR1 descriptor-image kernel-root provenance,
  compatibility-only TCR/MAIR records, blocked SCTLR/ASID/TLB/barrier states,
  idempotent teardown, and zero live side effects.
- LiveAddressSpaceActivationPlan from src/live_address_space_activation.rs,
  including boundary identity phase8-live-address-space-activation-plan-v1,
  selected policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1, copied loader/
  install/address-space/materialization/launch/stack lineage, TTBR0 root
  provenance, kernel reachability checklist, fault-reporting prerequisites,
  deterministic no-partial behavior, and zero live side effects.
- KernelHalfDescriptorImage and KernelHalfReachabilityPlan records from
  src/kernel_half_descriptor_image.rs and src/kernel_half_reachability.rs,
  including TTBR1 privileged kernel-root coverage, permission policy, linker
  range coverage, VBAR/vector/stack/UART/MMIO diagnostic reachability,
  scheduler code/data reachability, and panic/fault-reporting prerequisites.
- ProcessPageTableMaterialization provenance from
  src/process_page_table_materialization.rs, including the TTBR0
  materialized-root lease, root physical frame, rollback state, and destroyed
  input rejection precedent.
- AArch64 translation, descriptor, exception-vector, linker, UART/MMIO,
  runtime-console, scheduler, and fault-reporting vocabulary named by the
  source inventory.

Accepted output:

- one activation model record whose boundary identity should be
  phase8-live-translation-register-activation-v1;
- selected policy identity
  model-ttbr0-ttbr1-activation-commit-below-live-registers-v1;
- copied identities for installation, descriptor image, activation plan,
  reachability plan, TTBR0 materialized root, TTBR1 descriptor-image root,
  loader/install/address-space/materialization/launch/stack records, source
  digest, entry point, and initial SP;
- compatibility observations for TTBR0 root lifetime, TTBR1 kernel-root
  coverage, TCR_EL1 split/granule/cacheability/shareability vocabulary, and
  MAIR_EL1 normal-memory/device attribute vocabulary;
- a blocked SCTLR_EL1 mutation policy, blocked ASID allocation, blocked live
  TLBI, and planned-only DSB/ISB ordering vocabulary;
- explicit kernel reachability and fault-reporting preservation observations
  for VBAR_EL1, exception vectors, active kernel stack, kernel text/data,
  allocator state, UART/MMIO diagnostics, scheduler code/data, runtime
  console, and panic/fault reporting; and
- side-effect observations proving no live register write, active-root copy,
  ASID/TLB/barrier execution, lower-EL launch, scheduler publication, process
  mutation, descriptor-table mutation, filesystem mutation, QEMU hardware
  mutation, Pi 5 hardware action, boot archive publication, or hardware-lock
  use occurred.

## Activation Invariant

Before the activation model record may be published, these facts must be true
and inspectable:

- the installation record is published, not destroyed, and still below live
  translation-register ownership;
- the copied Phase 8 lineage, source digest, entry point, initial SP, TTBR0
  provenance, TTBR1 descriptor-image provenance, and reachability identity
  agree across the installation, activation, descriptor-image, and
  materialization records;
- TTBR0 materialized-root provenance remains owned by the accepted process
  page-table materialization and has not been written to TTBR0_EL1;
- TTBR1 descriptor-image provenance remains model-owned and has not been
  written to TTBR1_EL1 or copied into an active root;
- TCR_EL1 and MAIR_EL1 remain compatibility records only;
- SCTLR_EL1.M mutation, ASID allocation, live TLBI, and activation DSB/ISB
  execution remain blocked;
- kernel text, vectors, rodata, data, bss, active stack, heap, allocator
  metadata, scheduler state, runtime console, UART/MMIO diagnostics, and
  panic/fault reporting remain covered by kernel-owned prerequisites;
- device mappings retain device attributes and remain EL0-denied;
- activation failure can be classified without depending on user mappings; and
- no scheduler runnable state, process-table entry, descriptor table,
  filesystem syscall behavior, lower-EL saved frame, QEMU live mutation, or
  hardware proof becomes publishable through this record.

The first implementation must expose an activation intent, not activation
execution. Requests to perform the architectural register sequence must return
ENOSYS until a later supervisor-planned contract admits live mutation with
scaled gates.

## Policy Fields

The record must expose stable field names so a later QEMU/substitute smoke can
retain evidence without interpreting private implementation details:

| Field | Required value for this boundary |
| --- | --- |
| boundary identity | phase8-live-translation-register-activation-v1 |
| selected policy | model-ttbr0-ttbr1-activation-commit-below-live-registers-v1 |
| installation identity | phase8-live-descriptor-image-installation-v1 |
| activation plan identity | phase8-live-address-space-activation-plan-v1 |
| descriptor image identity | phase8-kernel-half-descriptor-image-v1 |
| TTBR0 provenance | copied materialized process root; no register write |
| TTBR1 provenance | copied descriptor-image kernel root; no register write |
| TCR_EL1 state | compatibility-record-only |
| MAIR_EL1 state | compatibility-record-only |
| SCTLR_EL1 state | mutation-blocked |
| ASID state | blocked-no-asid-allocation |
| TLB state | blocked-no-live-tlbi |
| barriers | planned-only-no-live-dsb-isb |
| active root | no-active-root-copy-or-mutation |
| fault reporting | kernel-owned-vector-diagnostics-preserved |
| activation state | model-only-activation-commit-intent |
| lower-EL launch | blocked-no-lower-el-eret |
| runnable publication | blocked-no-runnable-publication |
| side effects | zero-live-activation-side-effects |

## Deterministic Errors And Blockers

Errors must be deterministic and leave no visible partial activation state.

| Condition | Required result |
| --- | --- |
| Missing, destroyed, unpublished, wrong-identity, or internally inconsistent installation, descriptor-image, activation, reachability, or materialization record | EINVAL |
| Copied lineage, TTBR0 provenance, TTBR1 provenance, source digest, entry point, initial SP, reachability identity, descriptor policy, or activation policy disagree | ENOEXEC |
| Installation is already consumed by a live activation, records nonzero live side effects, or its underlying roots are stale/destroyed | EBUSY |
| Kernel coverage is missing, silently widened/truncated, overlaps user/device/null-guard space incorrectly, or grants EL0 access to kernel-half mappings | EACCES |
| UART/MMIO diagnostics lose device attributes, fault reporting loses kernel-owned vector/stack/UART prerequisites, or panic/fault reporting would depend on user mappings | EACCES |
| Activation record capacity, copied compatibility-record capacity, or deterministic fixture capacity is exhausted | ENOMEM |
| Caller asks for live TTBR0_EL1/TTBR1_EL1 write, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, active-root copy, ASID allocation, live TLBI, activation DSB/ISB execution, lower-EL ERET, scheduler publication, process-table mutation, descriptor-table publication, filesystem syscall behavior, Pi 5 proof, boot archive publication, or hardware-lock use | ENOSYS |

The model record should preserve explicit blocker vocabulary:

- blocked-no-live-register-sequence;
- blocked-no-active-root-copy;
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

Published activation model state is owned by the activation record. It may
refer to the accepted installation, descriptor image, activation plan, and
materialized roots, but it must not destroy, release, rewrite, or transfer
their owned roots, tables, frames, ranges, launch records, stack records, or
teardown state.

Construction is all-or-nothing:

1. no activation model record is published until input identities, copied
   lineage, root provenance, compatibility records, reachability prerequisites,
   and side-effect counters are consistent;
2. failure returns no partial activation intent and leaves the installation
   record below live registers;
3. failure leaves TTBR0_EL1 and TTBR1_EL1 unwritten, TCR_EL1 and MAIR_EL1
   record-only, and SCTLR_EL1 mutation blocked;
4. failure records no ASID, TLBI, DSB, ISB, active-root, scheduler, process,
   descriptor-table, filesystem, QEMU hardware, or Pi 5 hardware side effect;
5. rollback releases only activation-record-local capacity;
6. teardown is idempotent and clears only the activation model record; and
7. teardown does not release descriptor-image root/table leases,
   materialized-root leases, activation plan records, launch records, or stack
   records.

The first implementation may use target-independent model records and
QEMU/substitute fixture observations only. A future contract must separately
admit any live transfer to architectural translation-register ownership.

## Smoke And Implementation Gates

The mechanically next documentation-only task should be
phase8-qemu-live-translation-register-activation-smoke-plan-20260531, if
queued dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
model/substitute-only activation boundary:

- scenario identity qemu_live_translation_register_activation_smoke;
- retained evidence path under
  tasks/evidence/2026-05-31-qemu-live-translation-register-activation-smoke-core/;
- boundary identity phase8-live-translation-register-activation-v1;
- selected policy
  model-ttbr0-ttbr1-activation-commit-below-live-registers-v1;
- classification qemu-live-translation-register-activation-smoke-complete and
  PASS vocabulary;
- success observations for copied installation identity, copied TTBR0/TTBR1
  provenance, TCR/MAIR compatibility records, blocked SCTLR/ASID/TLB/barrier
  states, active-root nonmutation, kernel reachability/fault-reporting
  preservation, rollback/teardown, and zero live side effects;
- deterministic rejection observations for stale or destroyed inputs, lineage
  mismatch, already-consumed installation, forbidden EL0 kernel access,
  diagnostic reachability loss, live-register request, active-root-copy
  request, lower-EL launch request, scheduler publication request, filesystem
  request, and capacity exhaustion; and
- conditional regression gates for live descriptor-image installation,
  kernel-half descriptor-image, kernel-half reachability, live activation, and
  process page-table materialization smoke evidence if shared owners are
  touched.

The later implementation task must at minimum run cargo fmt, the Rust test
suite, git diff --check, and git diff --cached --check before commit. It
should run the accepted QEMU/substitute smoke once that script exists, and run
mdbook build if docs are touched. Pi 5 hardware evidence, boot archive
publication, lower-EL user execution, scheduler runnable publication,
filesystem syscall behavior, and live register mutation remain blocked until
separately planned and accepted.

## Deferred Surfaces

This contract explicitly defers:

- live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB,
  and ISB mutation;
- active-root descriptor copy, architectural TTBR0/TTBR1 ownership transfer,
  live kernel reachability proof, and live activation fault recovery;
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

## Validation

- static inspection: git status --short before edits was clean in the Talos
  repo.
- static documentation/source inspection: reviewed the accepted live
  translation-register activation source inventory, adjacent live activation,
  kernel-half reachability, kernel-half descriptor-image, and live
  descriptor-image installation contracts/closeouts, retained QEMU/substitute
  descriptor-image installation evidence, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.
