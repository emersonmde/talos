# Phase 6 Secondary Cacheable MMU Handoff Source Inventory

Task: phase6-secondary-cacheable-mmu-handoff-source-inventory-20260524

## Summary

This documentation-only task accepts the source inventory and handoff contract
needed before the blocked Pi 5 SMP lock hardware proof can resume. The current
Pi 5 proof is classified as
pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime: the boot CPU is
running with EL2 stage-1 translation and data cache enabled, while secondary
cores reach the proof with data cache disabled before the first generic lock
attempt.

## Accepted Contract

- Generic shared SpinLock contention on Pi 5 requires participating secondaries
  to enter the same cacheable EL2 stage-1 normal-memory regime as the boot CPU,
  or a documented narrower equivalent for every shared address touched by the
  proof.
- Required state includes accepted MAIR_EL2, TCR_EL2, TTBR0_EL2, SCTLR_EL2.M,
  SCTLR_EL2.I, and SCTLR_EL2.C, plus TLB/cache ordering through explicit
  barriers.
- The implementation should reuse the existing early boot CPU map and register
  plan rather than define a new map policy.
- Cache maintenance remains a separate early-boot sharing contract and must not
  be hidden inside the generic SpinLock API.

## Evidence

- Source/doc references:
  docs/src/project/phase6-secondary-cacheable-mmu-handoff-source-inventory.md
  inventories src/boot/rpi5.rs, src/arch/aarch64, src/memory_map, src/smp.rs,
  src/target/rpi5.rs, and memory architecture docs.
- Blocked proof linkage:
  tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/summary.md
  records boot SCTLR_EL2=0x30c51835 with cacheable MMU true and secondary
  SCTLR_EL2=0x30c50830 with cacheable MMU false before lock attempts.
- Follow-up tasks:
  phase6-secondary-cacheable-mmu-handoff-core-20260524 and
  phase6-secondary-cacheable-mmu-handoff-pi5-proof-20260524.

## Validation

- static inspection: git status --short before edits showed existing
  unaccepted Pi 5 lock-proof diagnostic changes; this task did not modify or
  revert them.
- static inspection: inspected the source files and evidence named above.
- fmt/lint/typecheck: git diff --check passed after documentation edits.
- static inspection: mdbook was unavailable in the container, so mdBook build
  was not run.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
