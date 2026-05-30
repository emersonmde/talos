# Phase 8 Live Address-Space Activation Source Inventory

Status: accepted

Task: phase8-live-address-space-activation-source-inventory-20260530

## Scope

This inventory maps the next Phase 8 Milestone 8.3 frontier after accepted
loader, process-install, process address-space, page-table materialization,
initial process launch, and initial user stack evidence. It is documentation
only and authorizes no live TTBR switch, lower-EL launch, scheduler
publication, hardware proof, or QEMU execution.

The accepted program-loader chain can now validate immutable /bin/init, derive
an install plan, model a process address space, materialize a non-activating
AArch64 descriptor image, produce a launch-preparation record, and attach a
model-only initial user stack. The exact gap is that none of those accepted
records defines the live activation policy that would make the materialized
root current in the CPU translation regime while preserving kernel
reachability, exception handling, MMIO access, rollback, and future process
switching.

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
- initial user stack closeout checkpoint:
  56096a174a73baadf581b8330e70759d38911dca.

## Current Owners

- src/program_loader.rs owns ProgramImagePlan, PlannedUserSegment,
  UserSegmentKind, immutable /bin/init fixture identity, digest, entry point,
  UserText/UserData permissions, file-copy ranges, and BSS zero-fill ranges.
  It does not allocate frames, produce page-table roots, activate
  translations, or launch lower-EL code.
- src/process_install.rs owns ProcessImageInstallPlan and page-install
  metadata. It preserves accepted segment permissions and page copy/zero-fill
  records while lower_el_launch_blocked=true. It has no live translation or
  lower-EL frame side effect.
- src/process_address_space.rs owns the target-independent
  ProcessAddressSpace model, ProcessAddressSpaceId, ProcessOwnerId linkage,
  model root/table/user-frame leases, mapping order, publication state,
  rollback, and idempotent teardown. Its root and table leases are model
  ownership records, not CPU-visible TTBR roots.
- src/process_page_table_materialization.rs owns
  ProcessPageTableMaterialization, materialized root/table/user-frame leases,
  AArch64 descriptor records, descriptor values, copy/zero byte accounting,
  activation_blocked=true, kernel_mapping_policy
  activation-blocked-no-kernel-half, rollback, and idempotent teardown. It
  deliberately does not define a TTBR0_EL1 or TTBR1_EL1 load, a kernel-half
  sharing policy, ASID/TLB behavior, or lower-EL launch state.
- src/initial_process_launch.rs owns InitialProcessLaunchPlan, entry
  provenance, saved-frame intent, blocked-no-ttbr-activation state, and zero
  live side-effect counters for TTBR/TCR/MAIR/SCTLR, ASID, TLB,
  lower-EL ERET, scheduler publication, process table mutation, and descriptor
  table mutation.
- src/initial_user_stack.rs owns InitialUserStackPlan,
  phase8-initial-user-stack-plan-v1, fixed stack top
  0x0000_8000_0000_0000, usable and guard ranges, stack-owned USER_DATA page
  leases, minimal-empty-argc0 startup metadata, launch-plan stack-ready
  binding, and no-partial-launch rejection. It keeps activation_state at
  blocked-no-ttbr-activation and writes no architectural register.
- src/posix.rs owns the accepted user range, null guard, user mapping
  permissions, copy-in/copy-out, DescriptorTable, and ProcessDescriptorStore
  vocabulary. It can validate user ranges and data permissions, but it does
  not select a live translation regime or context-switch policy.
- src/memory_map/translation.rs owns early stage-1 table and descriptor
  helpers for the current kernel bring-up map. Its current accepted use is the
  EL2 kernel identity map and materialization descriptor constants. It does
  not yet own a per-process activation root, kernel-half sharing policy, ASID
  policy, or EL1 activation sequence for /bin/init.
- src/arch/aarch64/mod.rs and src/arch/aarch64/exceptions.rs own AArch64
  register helpers, exception vectors, saved-frame vocabulary, lower-EL trap
  observations, and proof-local TTBR/TCR/MAIR/SCTLR code paths used by earlier
  QEMU/Pi 5 diagnostics. Those are source material only for this slice.
- src/scheduler.rs owns TaskId, ProcessOwnerId, Task, KernelStack,
  ContextFrame, run queues, and production dispatch state. It does not own a
  live ProcessAddressSpace binding, current ASID, activated TTBR root,
  process table entry, exit/wait state, or runnable publication for /bin/init.
- src/target/qemu_virt.rs owns retained QEMU/substitute smoke producers for
  the accepted model boundaries plus proof-local lower-EL diagnostic tables.
  Those proof tables are not a reusable Phase 8 activation contract.
- src/target/rpi5.rs owns serialized Pi 5 proof scenarios and proof-local
  lower-EL observations from Phase 7. Hardware evidence remains blocked until
  a later explicit Pi 5 proof plan.
- docs/src/architecture/lower-el-userspace.md owns the accepted lower-EL
  vocabulary: user range, kernel/user split, validated ELR/SP/SPSR state,
  user fault classes, copy-in/copy-out preconditions, and the warning that the
  current broad EL2 identity map is not a userspace-isolation contract.

## Activation Gap Map

| Area | Accepted input | Missing activation contract |
| --- | --- | --- |
| Translation root | ProcessPageTableMaterialization owns an inspectable root/table descriptor image. | Which root is loaded into TTBR0_EL1 and/or TTBR1_EL1, whether the kernel half is shared/replicated/split, and how the root remains owned after activation. |
| Register regime | Earlier diagnostics contain proof-local TTBR/TCR/MAIR/SCTLR writes. | Production EL1 activation values, ordering, barriers, TLB invalidation, failure reporting, and whether activation is one-shot initial /bin/init only or structured for later context switches. |
| ASID and TLB | Accepted side effects report asid_allocated=false and tlb_mutated=false. | ASID allocation/reuse, TTBR ASID field policy, TLBI scope, stale entry prevention, and teardown behavior for active address spaces. |
| Kernel reachability | Materialization records kernel_mapping_policy=activation-blocked-no-kernel-half. | VBAR_EL1, kernel text/data, kernel stack, heap, UART/MMIO, scheduler, and exception-vector reachability after user root activation. |
| Exception/fault reporting | Lower-EL architecture notes and diagnostics can report traps in proof-local paths. | The activation contract must state how synchronous faults during or after activation are observable without relying on a user mapping that may be broken. |
| Saved-frame intent | InitialProcessLaunchPlan and InitialUserStackPlan provide entry and SP provenance without register writes. | Which fields become architectural ELR_EL1, SP_EL0, SPSR_EL1, x0..x5, DAIF, and PSTATE inputs, and which remain blocked until lower-EL launch setup. |
| Scheduler binding | Scheduler has Task/ProcessOwnerId placeholders; launch plans reject runnable publication. | How a future runnable record will bind a task, process owner, active root/ASID, kernel stack, saved frame, and lifecycle state. |
| Rollback and teardown | Address-space, materialization, and stack records have idempotent teardown below activation. | Whether activation is reversible, how teardown behaves for the active address space, and what can be rolled back if register mutation fails mid-sequence. |
| Evidence | Existing QEMU/substitute smokes prove model boundaries and zero live side effects. | A retained smoke must prove an activation plan/record boundary without claiming QEMU Pi 5 fidelity or live hardware execution unless later explicitly scoped. |

## Boundary Separation

The live activation frontier must stay separate from several adjacent but
unaccepted surfaces:

- TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLB, DSB, and ISB
  policy belongs to the activation contract.
- lower-EL ERET, architectural SP_EL0/ELR_EL1/SPSR_EL1 writes, and live trap
  return behavior should remain a later lower-EL launch contract unless the
  supervisor explicitly scopes them into activation.
- scheduler runnable publication, process table/PID/current-process state,
  parent/child lifecycle, exit, wait, signals, and credentials remain later
  process lifecycle work.
- broad argv/envp/auxv/TLS ABI, libc startup, dynamic stack growth,
  guard-fault recovery, copy-on-write, and demand paging remain later startup
  ABI and VM work.
- descriptor inheritance, close-on-exec, cwd/root, descriptor-backed
  filesystem syscalls, writable filesystem state, shell behavior, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  outside this milestone slice.
- Pi 5 hardware proof, boot archive publication, TFTP/serial evidence, and
  hardwareTestLock acquisition remain blocked until a later explicit hardware
  proof plan exists.

## Smallest Target-Independent Boundary

The smallest objective follow-up is a documentation-only contract for
phase8-live-address-space-activation-contract-20260530.

That contract should define a live address-space activation plan or record
below lower-EL ERET. The record should consume the accepted materialization,
launch, and stack records and decide:

- exact root provenance and compatibility checks for
  phase8-process-page-table-materialization-v1;
- whether activation uses TTBR0_EL1 only with an existing kernel regime,
  TTBR1_EL1 kernel-half sharing, or another explicitly documented first-slice
  policy;
- required TCR_EL1 and MAIR_EL1 compatibility with the accepted descriptor
  image and normal/device memory attributes;
- ASID ownership, allocation state, and TLB invalidation requirements, even if
  the first implementation keeps ASID fixed or activation still plan-only;
- ordering for DSB/ISB/TLBI around any future register mutation;
- kernel reachability invariants for VBAR_EL1, exception vectors, active
  kernel stack, UART/MMIO diagnostics, scheduler code/data, and panic/fault
  reporting;
- rollback, teardown, and no-partial-activation behavior;
- how saved-frame intent, initial SP, and activation state are updated without
  permitting lower-EL ERET or runnable publication; and
- exact QEMU/substitute smoke vocabulary for success and deterministic
  rejection, including zero lower-EL/scheduler/process lifecycle side effects.

Implementation should remain blocked until that contract and the
QEMU/substitute smoke plan are accepted.

## Deferred Surfaces

The live address-space activation inventory keeps these surfaces blocked:

- lower-EL ERET and live execution of /bin/init;
- scheduler runnable publication, current-process state, process table/PID
  allocation, process lifecycle, exit, wait, exec, and spawn;
- broad argv/envp/auxv/TLS ABI, libc startup framing, signal stacks,
  guard-fault recovery, copy-on-write, and demand paging;
- descriptor-backed filesystem syscalls, descriptor inheritance semantics,
  cwd/root, shell behavior, writable filesystem state, and persistent storage;
- Pi 5 hardware proof, boot archive publication, hardwareTestLock acquisition,
  TFTP/serial evidence, and physical serial claims;
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Recommendation

The mechanically next task should be
phase8-live-address-space-activation-contract-20260530, if queued
dependencies remain satisfied. The accepted inventory identifies live
translation activation as the first missing boundary after the accepted
model-only /bin/init lineage: the kernel can describe and inspect the loaded
program and its stack, but no accepted contract yet says how that descriptor
image becomes the active EL1 translation context while preserving kernel
reachability and future fault reporting.

## Reviewed Materials

- docs/src/project/phase8-initial-user-stack-closeout-checkpoint.md
- docs/src/project/phase8-initial-user-stack-source-inventory.md
- docs/src/project/phase8-initial-user-stack-contract.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-initial-process-launch-contract.md
- tasks/2026-05-30-phase8-initial-user-stack-core.md
- tasks/2026-05-30-phase8-qemu-initial-user-stack-smoke-core.md
- tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log
- tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log
- tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/process_page_table_materialization.rs
- src/initial_process_launch.rs
- src/initial_user_stack.rs
- src/posix.rs
- src/memory_map/translation.rs
- src/arch/aarch64/mod.rs
- src/arch/aarch64/exceptions.rs
- src/scheduler.rs
- src/target/qemu_virt.rs
- src/target/rpi5.rs
- docs/src/architecture/lower-el-userspace.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted Phase 8 loader,
  install, address-space, materialization, launch, and stack docs/task
  records/evidence; source owners for loader/install/address-space/
  materialization/launch/stack/POSIX/translation/AArch64/scheduler/target
  evidence; lower-EL architecture notes; roadmap; SUMMARY; and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
