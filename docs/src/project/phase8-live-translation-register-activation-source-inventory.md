# Phase 8 Live Translation-Register Activation Source Inventory

Status: accepted

Task: phase8-live-translation-register-activation-source-inventory-20260531

## Scope

This inventory maps the next Phase 8 Milestone 8.3 frontier after accepted
live descriptor-image installation closeout. It is documentation only and
authorizes no Rust behavior change, assembly behavior change, QEMU execution,
Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, active-root
descriptor copy, ASID allocation, TLB invalidation, activation DSB/ISB
sequence, lower-EL ERET, scheduler runnable publication, process lifecycle,
shell behavior, descriptor-backed filesystem syscalls, writable filesystem,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

The accepted KernelHalfDescriptorImageInstallation proves a model-only
installation-ready binding between the accepted non-installed
KernelHalfDescriptorImage and the accepted LiveAddressSpaceActivationPlan. The
exact gap for this slice is the first contract that can name a live
translation-register activation boundary while preserving kernel reachability,
fault reporting, rollback, and every lower-EL/scheduler/process blocker.

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
- kernel-half descriptor-image closeout checkpoint:
  448a95dac8fb24bc8d99c07c4fb056df7ea06d79.
- live descriptor-image installation source inventory:
  19b824a3f6b6249204b3b7ca8129c051cfefcc05.
- live descriptor-image installation contract:
  e58ecebd5a4ce339b21d79e9029ecef70cc3d109.
- live descriptor-image installation core:
  ea264b234a2a68c89dc49d91d8adfa9c266148bd.
- QEMU/substitute live descriptor-image installation smoke core:
  5ef41854f6789dc829f4c4dfc984536c7104e559.
- live descriptor-image installation closeout checkpoint:
  9d1dbec78d4fd7704ea90eb3313cdad2b6067f87.
- retained QEMU/substitute descriptor-image installation evidence:
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log.

The accepted input frontier is still below any live TTBR/TCR/MAIR/SCTLR write.
It has accepted TTBR0 materialized-root provenance, TTBR1 descriptor-image
kernel-root provenance, compatibility-only TCR/MAIR records, blocked SCTLR
mutation, blocked ASID allocation, blocked live TLBI, planned-only no-live
DSB/ISB, and zero live side effects.

## Source Owners

- src/live_descriptor_image_installation.rs owns the accepted
  KernelHalfDescriptorImageInstallation identity
  phase8-live-descriptor-image-installation-v1, policy
  model-installed-ttbr1-descriptor-image-below-live-registers-v1, copied
  TTBR0/TTBR1 provenance, installation-ready activation binding state,
  compatibility-only TCR/MAIR records, blocked SCTLR/ASID/TLB/barrier states,
  lower-EL/scheduler/filesystem blockers, idempotent teardown, and zero live
  side effects.
- src/live_address_space_activation.rs owns the accepted
  LiveAddressSpaceActivationPlan identity
  phase8-live-address-space-activation-plan-v1, TTBR0 root provenance,
  TTBR1/kernel-half blocker vocabulary, TCR/MAIR/SCTLR/ASID/TLB/barrier state
  strings, no-partial activation rejection, kernel reachability checklist, and
  commit rejections for live registers, runnable publication, and lower-EL
  launch.
- src/kernel_half_descriptor_image.rs owns the accepted TTBR1 model-owned
  kernel-root descriptor image, TTBR0 materialized-root provenance carried
  from process page-table materialization, descriptor coverage, permissions,
  rollback, and non-installed zero-side-effect evidence.
- src/kernel_half_reachability.rs owns VBAR_EL1, exception vector, active
  kernel stack, allocator, UART/MMIO diagnostic, scheduler code/data, and
  panic/fault-reporting reachability prerequisites that must remain available
  during any future activation attempt.
- src/process_page_table_materialization.rs owns the TTBR0 root lease,
  materialized process-root physical frame, activation_blocked reporting,
  rollback, and teardown evidence that any live TTBR0_EL1 source must preserve.
- src/memory_map/translation.rs owns early AArch64 descriptor and translation
  register vocabulary. It can name descriptor attributes, MAIR/TCR
  compatibility, and register sequencing constraints, but its current live
  mutation path is still the early kernel map rather than a per-process
  TTBR0/TTBR1 activation boundary.
- src/arch/aarch64/exceptions.rs and src/arch/aarch64/vectors.S own exception
  vector reachability and fault-reporting vocabulary. The activation contract
  must preserve a kernel-owned fault path before admitting any live register
  mutation.
- linker.ld and linker-rpi5.ld own kernel text, rodata, data, bss, stack, and
  vector ranges that feed descriptor-image and reachability coverage.
- src/mmio.rs, src/pl011.rs, src/runtime_console.rs, and src/tty.rs own
  UART/MMIO diagnostics and runtime console state. Live activation must keep
  device mappings device-typed and EL0-denied.
- src/scheduler.rs owns task, kernel-stack, run-queue, and dispatch state.
  Translation-register activation must stay separate from runnable
  publication and process-table mutation.

## Activation Gap Map

| Area | Accepted state | Missing activation boundary |
| --- | --- | --- |
| TTBR0 | Materialized process-root provenance is copied through activation and installation records with ttbr0_written=false. | A contract for when TTBR0_EL1 may be programmed from that provenance, including rejection of stale or destroyed roots. |
| TTBR1 | Descriptor-image kernel-root provenance is installation-ready with ttbr1_written=false and active_root_copied=false. | A contract for whether TTBR1_EL1 uses the descriptor-image root directly or first copies it into an active root. |
| TCR/MAIR | Compatibility-only records are accepted; no live mutation exists. | Exact compatibility checks, accepted values, rejection vocabulary, and whether live writes are modeled or executed in QEMU/substitute. |
| SCTLR | SCTLR mutation is blocked. | A bounded policy for whether SCTLR_EL1.M remains unchanged, is modeled only, or is admitted to live QEMU/substitute mutation with rollback evidence. |
| ASID/TLB | ASID allocation and live TLBI are blocked. | A first activation contract must either keep ASID=0/no live TLBI blocked or name deterministic ASID/TLB sequencing and failure classes. |
| Barriers | DSB/ISB sequencing is planned-only with no live execution. | Exact barrier ordering vocabulary and success/failure observations before any live activation claim. |
| Fault reporting | Kernel reachability, vectors, stack, UART/MMIO, and panic/fault reporting are accepted prerequisites. | Proof that an activation failure can still report through kernel-owned diagnostics after any admitted register sequence. |
| Rollback | Installation-local teardown clears only the binding and preserves inputs. | Rollback/teardown rules for partially prepared activation state without lowering EL or publishing a runnable. |
| Forbidden side effects | Lower-EL ERET, scheduler publication, process lifecycle, filesystem syscalls, and hardware proof are blocked. | Contract language that keeps these states blocked even if translation-register activation is modeled or partially prepared. |

## Model-Only Facts Versus Unaccepted Live State

Accepted model-only facts:

- TTBR0 provenance comes from the accepted materialized process root, including
  root token and physical frame, and has not been written to TTBR0_EL1.
- TTBR1 provenance comes from the accepted descriptor-image kernel-root
  binding and has not been written to TTBR1_EL1.
- TCR and MAIR are compatibility records only; they are not architectural
  register writes.
- SCTLR mutation, ASID allocation, live TLB invalidation, and live DSB/ISB
  remain blocked or planned-only states.
- Kernel reachability covers VBAR_EL1, exception vectors, active kernel stack,
  kernel text/data, allocator, UART/MMIO diagnostics, scheduler code/data, and
  panic/fault reporting.
- Installation teardown is local to the installation record and preserves the
  descriptor-image and activation inputs.

Unaccepted live behavior:

- programming TTBR0_EL1 or TTBR1_EL1;
- mutating TCR_EL1, MAIR_EL1, or SCTLR_EL1;
- copying a descriptor image into an active root for a live register write;
- allocating ASIDs or invalidating live TLB state;
- executing a live activation DSB/ISB sequence;
- proving post-activation instruction fetch, stack use, vector entry, or UART
  reporting on Pi 5;
- entering lower EL, publishing a scheduler runnable, mutating a process table,
  or expanding descriptor-backed filesystem/syscall behavior.

## Boundary Recommendation

The mechanically next task should be
phase8-live-translation-register-activation-contract-20260531, if queued
dependencies remain satisfied.

The contract should remain target-independent and choose a deliberately small
activation boundary. The safest bounded contract is a preparation/commit model
for live translation-register activation that:

- consumes the accepted KernelHalfDescriptorImageInstallation and copied Phase
  8 lineage;
- states TTBR0 and TTBR1 root provenance, root lifetime, and stale/destroyed
  input rejection;
- defines TCR/MAIR compatibility checks before any register mutation claim;
- keeps SCTLR, ASID, TLB, and DSB/ISB either explicitly blocked or admits only
  a named QEMU/substitute boundary with exact rollback and failure vocabulary;
- proves kernel reachability and fault-reporting prerequisites remain
  kernel-owned before and after any admitted preparation step;
- rejects lower-EL launch, scheduler publication, process lifecycle,
  descriptor-table publication, filesystem mutation, hardware proof, and
  boot-archive publication as forbidden side effects; and
- names a QEMU/substitute smoke-plan task only after it selects deterministic
  success and rejection observations.

Implementation must remain blocked until that contract and its
QEMU/substitute smoke plan are accepted.

## Deferred Surfaces

This inventory keeps these surfaces blocked:

- live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation;
- active-root descriptor copy unless explicitly accepted by a later contract;
- ASID allocation, live TLB invalidation, and live DSB/ISB activation
  sequencing;
- lower-EL ERET and architectural launch;
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

## Reviewed Materials

- docs/src/project/phase8-live-address-space-activation-source-inventory.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-kernel-half-reachability-closeout-checkpoint.md
- docs/src/project/phase8-kernel-half-descriptor-image-contract.md
- docs/src/project/phase8-kernel-half-descriptor-image-closeout-checkpoint.md
- docs/src/project/phase8-live-descriptor-image-installation-source-inventory.md
- docs/src/project/phase8-live-descriptor-image-installation-contract.md
- docs/src/project/phase8-live-descriptor-image-installation-closeout-checkpoint.md
- tasks/2026-05-31-phase8-live-descriptor-image-installation-closeout-checkpoint.md
- tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log
- src/live_descriptor_image_installation.rs
- src/live_address_space_activation.rs
- src/kernel_half_descriptor_image.rs
- src/kernel_half_reachability.rs
- src/process_page_table_materialization.rs
- src/memory_map/translation.rs
- src/arch/aarch64/exceptions.rs
- src/arch/aarch64/vectors.S
- linker.ld
- linker-rpi5.ld
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted live activation,
  kernel-half reachability, kernel-half descriptor-image, and live
  descriptor-image installation docs, task records, retained QEMU/substitute
  installation evidence, relevant source owners, linker scripts, roadmap,
  SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.
