# Phase 8 Live Address-Space Activation Source Inventory Task

Task: phase8-live-address-space-activation-source-inventory-20260530

Status: accepted

## Scope

Completed a documentation-only source inventory for the first live
address-space activation boundary after the accepted Phase 8 model lineage:
ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace,
ProcessPageTableMaterialization, InitialProcessLaunchPlan, and
InitialUserStackPlan.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no live TLB
invalidation, no lower-EL ERET, no scheduler runnable publication, no process
lifecycle, no broad argv/envp/auxv/TLS ABI, no descriptor-backed filesystem
syscalls, no shell, no writable filesystem, no networking, no SSH, no
RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Evidence

- inventory document:
  docs/src/project/phase8-live-address-space-activation-source-inventory.md.
- reviewed source/docs:
  - docs/src/project/phase8-initial-user-stack-closeout-checkpoint.md.
  - docs/src/project/phase8-initial-user-stack-source-inventory.md.
  - docs/src/project/phase8-initial-user-stack-contract.md.
  - docs/src/project/phase8-process-page-table-materialization-contract.md.
  - docs/src/project/phase8-initial-process-launch-contract.md.
  - tasks/2026-05-30-phase8-initial-user-stack-core.md.
  - tasks/2026-05-30-phase8-qemu-initial-user-stack-smoke-core.md.
  - tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log.
  - tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log.
  - tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log.
  - src/program_loader.rs.
  - src/process_install.rs.
  - src/process_address_space.rs.
  - src/process_page_table_materialization.rs.
  - src/initial_process_launch.rs.
  - src/initial_user_stack.rs.
  - src/posix.rs.
  - src/memory_map/translation.rs.
  - src/arch/aarch64/mod.rs.
  - src/arch/aarch64/exceptions.rs.
  - src/scheduler.rs.
  - src/target/qemu_virt.rs.
  - src/target/rpi5.rs.
  - docs/src/architecture/lower-el-userspace.md.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.
- accepted input commits reviewed:
  - ProgramImagePlan core:
    38b3a09ad4e1be353950ad75880b119e7e0b534e.
  - ProcessImageInstallPlan core:
    49a54d91ef7920f74c97ca403a5075ce5f8d84a1.
  - ProcessAddressSpace core:
    06a5f4ed8e426afd01b77382c070a76d572d7c12.
  - ProcessPageTableMaterialization core:
    54d519e6ef629b9298bcfd2dea6fb9552fb86747.
  - InitialProcessLaunchPlan core:
    a57b0678126b1cb95c444e3e09fecfe8bea227f9.
  - InitialUserStackPlan core:
    f76c07f264efd1fc570b678af71e8a26ada155fa.
  - Initial user stack closeout:
    56096a174a73baadf581b8330e70759d38911dca.
- inventory result: the missing boundary is live translation activation below
  lower-EL ERET. The next contract must decide root provenance, TTBR0_EL1/
  TTBR1_EL1 policy, TCR_EL1/MAIR_EL1 compatibility, ASID/TLB behavior,
  barrier ordering, kernel reachability, fault reporting, rollback/teardown,
  and how activation state changes without runnable publication.
- recommended next task:
  phase8-live-address-space-activation-contract-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted Phase 8 docs, task
  records, retained QEMU/substitute evidence, source owners, lower-EL
  architecture notes, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as a documentation-only inventory. It recommends the queued
phase8-live-address-space-activation-contract-20260530 task as the next
bounded documentation-only step. Live register mutation, lower-EL ERET,
scheduler publication, process lifecycle, filesystem syscalls, hardware proof,
networking, and SSH remain blocked.

Commit: recorded in durable supervisor state after acceptance.
