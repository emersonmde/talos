# Talos Memory, MMU, and Allocator Review

Task: talos-review-memory-mmu-allocator-20260603
Status: accepted

## Scope

Reviewed the early memory-map helpers, page-frame/bootstrap allocator
contracts, MMU translation table planning/population, descriptor encoders,
allocator initialization, MMIO map surface, and RPi5/QEMU cache-regime handoff
paths.

## Findings

- Fixed: BumpAllocator::init_from_plan trusted the supplied
  EarlyBootstrapAllocatorPlan start/end without checking the recorded size,
  page size, page count, or alignment contract. A malformed plan could publish a
  heap that did not match the page-frame allocator's ownership contract. The
  initializer now rejects inconsistent bounds, sizes, counts, and alignment.
- Fixed: bump allocation alignment overflow was reported as InvalidAlignment
  because the allocator collapsed align_up arithmetic overflow into the same
  Option failure as a bad alignment value. The fallible path now distinguishes
  InvalidAlignment from AddressOverflow.
- Fixed: early page-frame reuse metadata accepted any nonempty out-of-managed
  range, even if the recorded metadata range was too small or unaligned for the
  supplied free-list slice. The constructor now validates metadata size and u64
  alignment before publishing allocator state.
- Fixed: early translation population accepted an arbitrary set of page-aligned
  table pointers, so callers could accidentally overlap tables or point outside
  the reserved bootstrap table span. Population/register planning now requires
  the canonical contiguous root/L1/low-L2/MMIO-L2 layout produced by
  early_translation_table_layout.
- Fixed: table and block descriptor encoders silently masked addresses that
  exceeded the descriptor address field. They now reject out-of-range physical
  addresses instead of truncating them.
- Not an issue: MMIO map definitions are currently static target inventory
  data. The review did not find a runtime mutability or ownership issue in
  src/mmio.rs; future validation can add overlap checks when MMIO regions start
  feeding mapping policy.
- Not an issue: the RPi5 secondary cacheable-MMU handoff path copies the current
  EL2 regime and applies M/I/C bits for secondary bring-up. It remains a
  target/SMP boundary concern; this task did not change cache policy or
  secondary-core behavior.

## Changes

- src/allocator.rs validates allocator plan consistency and adds regression
  tests for malformed plans and alignment-overflow accounting.
- src/memory_map/page_frames.rs validates reuse-allocator metadata range size
  and alignment, with regression coverage.
- src/memory_map/translation.rs validates contiguous reserved table layout and
  descriptor address bounds, and updates the table-population unit test to use a
  contiguous aligned test arena.

No hardware behavior, userspace behavior, filesystem behavior, networking,
RP1/PCIe, UART interrupt ownership, DMA policy, or new feature surface was
added.

## Validation

- Static inspection: reviewed src/memory_map/*, src/allocator.rs, src/mmio.rs,
  src/arch/aarch64/mod.rs MMU/cache helpers, target MMIO inventories, and RPi5
  secondary cacheable-MMU handoff call sites with rg/sed.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 357 no_std
  tests.
- target checks: cargo -Zjson-target-spec check --quiet passed.
- RPi5 target check: TALOS_BOOT_SCENARIO=rpi5_local_serial_command_loop cargo
  -Zjson-target-spec check --target targets/aarch64-talos-rpi5-bcm2712.json
  --quiet passed.
- QEMU target check: TALOS_BOOT_SCENARIO=qemu_local_serial_command_loop cargo
  -Zjson-target-spec check --quiet passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed after adding this
  task record.
- static diff hygiene: git diff --check and git diff --cached --check passed.
- hardwareTestLock remained unlocked/restored and unused; no hardware claim was
  made.

## Remaining Risks

- MMIO region overlap/coverage validation is deferred until MMIO map inventory
  feeds an accepted mapping policy; adding it now would create unused policy
  surface without changing behavior.
- Cache policy remains target-specific and proof-driven. This review tightened
  model-side MMU/allocator contracts but did not change accepted cache enable or
  secondary handoff semantics.

Accepted commit: recorded in durable state for
talos-review-memory-mmu-allocator-20260603.
