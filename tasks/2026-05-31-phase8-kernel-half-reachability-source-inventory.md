# Phase 8 Kernel-Half Reachability Source Inventory Task

Task: phase8-kernel-half-reachability-source-inventory-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 inventory of the kernel-half reachability gap
left by the accepted live address-space activation closeout. No Rust behavior,
assembly behavior, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, live TTBR/TCR/MAIR/SCTLR mutation,
ASID allocation, TLB mutation, lower-EL ERET, scheduler runnable publication,
process lifecycle, shell behavior, descriptor-backed filesystem syscalls,
writable filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy was added.

Changed files:

- docs/src/project/phase8-kernel-half-reachability-source-inventory.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-kernel-half-reachability-source-inventory.md

## Outcome

The inventory maps accepted Phase 8 loader/install/address-space/
materialization/launch/stack/activation artifacts and retained evidence to the
remaining blocked-no-accepted-kernel-half-map frontier.

It identifies current owners for kernel image layout, kernel text/rodata/data/
bss, vectors, active stack, heap/page-frame allocator, UART/MMIO diagnostics,
scheduler state, early translation helpers, process materialization descriptor
images, and AArch64 register vocabulary.

It separates candidate kernel-half reachability policies from live activation:

- TTBR1_EL1 shared kernel root;
- replicated kernel-half descriptors in each process root;
- explicitly blocked preflight record before descriptor construction.

The recommended next bounded task is the already queued documentation-only
phase8-kernel-half-reachability-contract-20260531, if dependencies remain
satisfied. Live register mutation, lower-EL ERET, scheduler publication,
process lifecycle, startup ABI expansion, descriptor-backed filesystem
syscalls, Pi 5 proof, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked.

## Evidence

- Inventory document:
  docs/src/project/phase8-kernel-half-reachability-source-inventory.md.
- Reviewed commits:
  program-loader core 38b3a09ad4e1be353950ad75880b119e7e0b534e;
  process-install core 49a54d91ef7920f74c97ca403a5075ce5f8d84a1;
  process address-space core 06a5f4ed8e426afd01b77382c070a76d572d7c12;
  page-table materialization core 54d519e6ef629b9298bcfd2dea6fb9552fb86747;
  initial process launch core a57b0678126b1cb95c444e3e09fecfe8bea227f9;
  initial user stack core f76c07f264efd1fc570b678af71e8a26ada155fa;
  live activation core 129337734011004297da0b2768a3a802063c3293;
  live activation QEMU/substitute smoke core
  1c441c301387ed75e24db7f9788301126f1f5a72; and live activation closeout
  5f4589db87b821775b671c7630bb19c9905d497c.
- Retained evidence:
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log.
- Recommendation: phase8-kernel-half-reachability-contract-20260531 as the
  next bounded documentation-only task if dependencies remain satisfied.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted Phase 8 docs,
  task records, retained evidence, linker layout, allocator/page-frame,
  translation, AArch64 exception/vector, UART/MMIO diagnostic, scheduler,
  target evidence, architecture, roadmap, SUMMARY, and ADR sources relevant to
  kernel-half reachability.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
