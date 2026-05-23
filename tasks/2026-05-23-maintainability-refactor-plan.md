# 2026-05-23 Maintainability Refactor Plan

## Audit Summary

This audit was triggered before more Phase 3 feature work because the current
Talos source layout is carrying too much implementation history in a few files.
No kernel code changed in this task.

Pre-audit git status --short showed an existing README.md modification that is
outside this task and was left untouched.

Largest Rust files at audit time:

| File | Bytes | Lines | Current responsibility mix |
| --- | ---: | ---: | --- |
| src/main.rs | 114127 | 2758 | Entry flow, normal Pi 5 boot orchestration, report formatting, hardware diagnostics, panic/allocation handlers, and FDT tests. |
| src/memory_map.rs | 51807 | 1590 | Memory-bank policy, page-frame seed/reservation/ownership, translation-table layout/population/register planning, descriptor helpers, and tests. |
| src/target/rpi5.rs | 41945 | 1252 | Pi 5 target services, UART/MMIO constants, early phase output, firmware console, and many hardware diagnostics. |
| src/device_tree.rs | 28120 | 795 | FDT public data types, unsafe raw cursor parsing, /chosen, reservation block, /memory, and /reserved-memory interpretation. |
| src/arch/aarch64/exceptions.rs | 10036 | 315 | Exception entry/reporting. |
| src/allocator.rs | 9429 | 300 | Bootstrap bump allocator. |
| src/arch/aarch64/mod.rs | 6763 | 243 | AArch64 MMU/cache/register helpers and test exit. |

## Hotspots

src/main.rs is the first cleanup target. kernel_main currently contains the
normal Pi 5 boot sequence and the memory/MMU/cache/allocator progression inside
a long nested Option chain. The deepest section around memory selection,
page-frame seed, bootstrap reservation, translation-table layout/population,
MMU enable, instruction-cache enable, data-cache enable, allocator init, and
post-allocator reports spans hundreds of lines and repeatedly embeds:

- cfg exclusions for talos_rpi5_vec_growth_diagnostic,
  talos_rpi5_string_growth_diagnostic, and talos_rpi5_alloc_format_diagnostic.
- diagnostic escapes such as talos_rpi5_translation_fault_diagnostic.
- immediate report writes mixed with state transitions.
- allocator smoke and diagnostic dispatch mixed into the accepted normal boot path.

The same file also holds many Pi 5-only report writer helpers (write_rpi5_*)
and diagnostic implementations from roughly the post-boot-report section
onward. That makes the entry flow hard to review because behavior,
presentation, and temporary hardware probes share one module.

src/target/rpi5.rs has a similar split problem. The stable target surface
(services, MMIO ranges, firmware console, UART constants) is adjacent to long
early-phase and diagnostic routines. The char-by-char early phase output is
legitimate early-boot code, but its current placement makes the target module
look like a diagnostic archive rather than a board support boundary.

src/memory_map.rs should become several memory modules. The current file mixes
four responsibilities: choosing a conservative usable low-memory span, naming
page-frame ownership partitions, constructing early translation-table metadata,
and populating raw AArch64 descriptors. These are related, but they now have
different review and test surfaces.

src/device_tree.rs should split raw FDT walking from Talos-specific memory
interpretation. Header/block validation, structure-token walking, and cell
decoding are lower-level parser concerns; /memory, reservation-block,
/reserved-memory, and /chosen outputs are policy-facing data extraction.

The scripts/ directory contains many one-off Pi 5 diagnostic image/tree
wrappers. They are useful evidence tools, but future refactors should avoid
adding more wrappers that duplicate environment setup or image staging logic.

## Diagnostic Disposition

The cleanup policy is deletion-first: a diagnostic stays only if it is still a
boot-process probe, a hardware regression gate, or the clearest way to validate
an explicit task. If a probe only preserves knowledge already accepted in
hardware evidence, the next cleanup task must document that fact in the relevant
architecture/decision note or encode it in a real test before deleting the
function, script wrapper, cfg flag, and build.rs plumbing.

| Family | Examples | Classification | Cleanup requirement |
| --- | --- | --- | --- |
| Normal boot image and format gates | scripts/rpi5-image.sh, scripts/rpi5-boot-tree.sh, scripts/rpi5-boot-img.sh, scripts/rpi5-format-guard-check.sh, scripts/rpi5-archive-review.sh | keep-as-boot/regression diagnostic | Keep. These are validation gates, not stale probes. |
| Allocator and alloc-crate diagnostics | talos_rpi5_alloc_oom_diagnostic, talos_rpi5_realloc_growth_diagnostic, talos_rpi5_vec_growth_diagnostic, talos_rpi5_string_growth_diagnostic, talos_rpi5_alloc_format_diagnostic and matching scripts | promote-to-feature | Convert into allocator/page-frame/heap policy tests or explicit diagnostic modules. Delete one-off image padding knobs after the accepted behavior is covered by tests or task records. |
| Exception/fault report diagnostics | talos_rpi5_normal_exception_report_diagnostic, talos_rpi5_undefined_instruction_report_diagnostic, talos_rpi5_data_abort_report_diagnostic, talos_rpi5_translation_fault_diagnostic, talos_rpi5_current_sp0_sync_diagnostic and matching scripts | keep-as-boot/regression diagnostic | Keep only the deliberate exception/fault cases that exercise active reporting paths. Move them out of src/main.rs. Delete duplicate or superseded wrappers after preserving the accepted ESR/class/status facts in docs/tests. |
| Panic diagnostics | talos_rpi5_panic_report_diagnostic, talos_rpi5_full_panic_info_diagnostic, talos_rpi5_nested_panic_diagnostic and matching scripts | keep-as-boot/regression diagnostic | Keep a minimal panic and nested-panic regression path. Remove older full-info variants if the normal panic report already proves the same output contract. |
| Early serial and handoff probes | talos_rpi5_runtime_uart_probe_diagnostic, talos_rpi5_handoff_uart_diagnostic, talos_rpi5_rust_uart10_diagnostic, rpi5-uart-*-proof*.sh, rpi5-entry-*-diagnostic*.sh, fresh-entry label/continue/reset helpers | document-and-delete | The project has accepted firmware-console, UART10, rust_entry, and readable Talos-origin serial facts. Capture any still-useful address/firmware facts in early-serial docs, then delete stale probes and proof scripts. Keep only one fresh-entry/readable-output control if the lab workflow still needs it as a regression discriminator. |
| Entry boundary, stack, text, and assembly reset probes | transition/text/vector/boot-near/boot-far/fallthrough/post-stack/asm-direct/asm-indirect/asm-to-rust/BTI/direct-exception/BRK reset cfgs and scripts | document-and-delete | These were bring-up boundary finders for first-light and exception-return work. Preserve accepted boundaries in decisions/architecture notes, then remove the cfgs, assembly conditionals, build.rs env plumbing, and scripts unless one is explicitly promoted as a current regression gate. |
| println, format-sink, rodata, and function-pointer probes | talos_rpi5_minimal_format_diagnostic, talos_rpi5_dynamic_format_fallback_diagnostic, talos_rpi5_fmt_*_diagnostic, talos_rpi5_fnptr_reset_diagnostic, talos_rpi5_println_phase_diagnostic, talos_rpi5_rodata_address_diagnostic, talos_rpi5_static_format_boundary_diagnostic | delete-as-stale | The accepted post-data-cache println boundary and UART polling fix are now the real feature. Record the learned formatter/rodata boundary in early-serial docs if missing, then delete these probes. |
| Phase ladder/reset probes | talos_rpi5_phase_ladder_diagnostic, talos_rpi5_phase_p0/p1/p1_short/p2 diagnostics, CPACR/BSS/stack/stack-to-text/stack-to-rust/continue scripts | document-and-delete | These bracketed the path into Rust. Current normal boot and rust_entry evidence supersede them. Delete after the accepted boot-stage sequence is documented. |
| Loader, armstub, EFI, Circle, and alternative boot experiments | rpi5-loader-diagnostic*, rpi5-armstub-diagnostic*, rpi5-efi-diagnostic*, rpi5-circle-loader-diagnostic-tree.sh, prefixed loader/armstub scripts | delete-as-stale | These are not on the accepted normal firmware handoff path. Delete unless a future supervisor task explicitly reopens alternative boot research. |

## Ordered Refactor Tasks

1. phase3-main-entry-diagnostics-refactor-20260523

   Primary write scope: src/main.rs, new entry/report/diagnostic modules, and
   diagnostic/build-script cleanup for the families above. This task must not
   simply move stale diagnostics into nicer modules.

   Proposed module layout:

   - src/boot/entry.rs or src/runtime_boot.rs: short kernel_main driver and
     named boot-step functions.
   - src/boot/rpi5_reports.rs: normal Pi 5 boot report formatting helpers.
   - src/diagnostics/rpi5_alloc.rs: alloc, realloc, Vec, String, and
     alloc-format diagnostics.
   - src/diagnostics/rpi5_boot.rs: reset, println, format, translation-fault,
     and panic/exception diagnostic entry points.

   Code-quality targets:

   - shrink src/main.rs substantially, with a target below 900 lines after the
     first refactor;
   - make kernel_main read as an ordered boot pipeline rather than a deeply
     nested if-let tree;
   - delete stale diagnostic functions, scripts, cfg flags, and build.rs env
     plumbing after recording accepted facts;
   - move only still-justified diagnostic dispatch behind named cfg-gated
     functions;
   - keep accepted normal boot log order unchanged.

2. phase3-memory-fdt-module-refactor-20260523

   Primary write scope: src/memory_map.rs, src/device_tree.rs, and new
   src/memory_map/ and src/device_tree/ module files. This task should not edit
   the entry/diagnostic modules except for import/call-site fallout.

   Proposed memory layout:

   - src/memory_map/mod.rs: public re-exports and high-level contract.
   - src/memory_map/layout.rs: kernel/FDT range inputs and low-tail candidate
     selection.
   - src/memory_map/page_frames.rs: seed, bootstrap reservation, ownership
     contract, and allocator-owned span metadata.
   - src/memory_map/translation.rs: table layout, population counts, raw
     descriptor helpers, and register/cache plans.

   Proposed FDT layout:

   - src/device_tree/mod.rs: public DeviceTree facade and re-exports.
   - src/device_tree/raw.rs: FDT header, block validation, cursor, token
     walking, alignment, and big-endian cell helpers.
   - src/device_tree/memory.rs: reservation block, /memory, and
     /reserved-memory interpretation.
   - src/device_tree/chosen.rs: /chosen properties such as bootargs.

   Code-quality targets:

   - keep each new memory/FDT module under roughly 700 lines unless tests justify
     the size;
   - keep public type names stable unless a rename removes real ambiguity;
   - add focused boundary tests for parser/policy splits exposed by the move;
   - preserve accepted ownership values and normal boot output.

3. phase3-maintainability-review-checkpoint-20260523

   Primary write scope: task records, supervisor state, and docs only unless the
   review finds a small cleanup that must happen before feature work resumes.

   Review targets:

   - record the largest Rust files after refactor;
   - confirm src/main.rs is no longer the catch-all orchestration module;
   - confirm Pi 5 diagnostics are cfg-gated outside the normal boot flow;
   - confirm memory/FDT responsibilities are visible from module paths;
   - either insert one more bounded cleanup task or mark the next Phase 3
     feature task ready.

## Behavior-Preservation Risks

The riskiest refactor is the entry/diagnostic split because moving code around
kernel_main can accidentally reorder accepted serial lines or change which
cfg-gated diagnostic path suppresses normal reports. The refactor must preserve
the accepted post-data-cache/allocator/String/translation-table/page-frame/DTB
report order and must not remove any accepted diagnostic capability without a
replacement path.

The memory/FDT split is lower boot-output risk but higher semantic risk. Moving
raw FDT parsing helpers away from memory policy must not change reservation
counting, truncation behavior, low-tail candidate selection, page-frame
ownership partitions, table slots, descriptor counts, or register plans.

## Validation Gates

For no-behavior-change Rust refactors:

- git status --short before and after;
- cargo fmt --all -- --check;
- cargo -Zjson-target-spec test;
- scripts/qemu-smoke.sh;
- scripts/rpi5-image.sh;
- scripts/rpi5-format-guard-check.sh;
- git diff --check;
- mdbook build if available when docs are touched.

Serialized Pi 5 hardware evidence is required only if a refactor changes normal
Pi 5 boot output, accepted line order, diagnostic image behavior that is being
accepted, or hardware-facing register/MMU/cache behavior. Pure module moves with
unchanged output should record a no-normal-boot-output-change rationale instead
of consuming the hardware lock.

## Audit Validation

- git status --short before audit changes: README.md was already modified and
  was not touched by this task.
- git status --short after audit changes: README.md remained modified outside
  this task, and this task added tasks/2026-05-23-maintainability-refactor-plan.md.
- git diff --check passed.
- mdbook build was not run because mdbook is unavailable in the container.
- Rust formatting and tests were not required for this audit because no Rust
  files changed.
- Hardware validation was not required because no boot behavior changed.

## Refactor Progress

### phase3-main-entry-diagnostics-refactor-20260523

First cleanup pass removed the stale rust-entry reset, format-sink,
println-phase, rodata-address, static-format-boundary, and
stack-to-rust/asm-to-rust reset diagnostic paths classified as
document-and-delete or delete-as-stale. The pass deleted their standalone image
scripts, removed their build.rs cfg/env plumbing, removed stale env unsets from
retained exception/panic scripts, and deleted the corresponding src/main.rs and
src/target/rpi5.rs implementations.

Before/after line counts for this pass:

| File | Before | After |
| --- | ---: | ---: |
| src/main.rs | 2758 | 2307 |
| src/target/rpi5.rs | 1252 | 812 |
| build.rs | 438 | 354 |

Validation for the first cleanup pass:

- cargo fmt --all -- --check passed.
- cargo -Zjson-target-spec test passed: 39 no_std tests.
- scripts/qemu-smoke.sh passed.
- scripts/rpi5-image.sh passed and produced target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img.
- scripts/rpi5-format-guard-check.sh passed.
- Representative retained diagnostic build passed:
  scripts/rpi5-panic-report-diagnostic-image.sh.
- git diff --check passed.
- No Pi 5 hardware run was required because no accepted normal boot output,
  entry register programming, MMU/cache programming, or allocator behavior was
  intentionally changed.

Second extraction pass moved the retained normal Pi 5 boot path, boot-report
formatting helpers, allocator diagnostics/bootstrap allocation smoke, and
exception/fault/panic diagnostic dispatch out of src/main.rs.

Final module layout for this task:

- src/boot/rpi5.rs owns the normal Pi 5 boot pipeline and calls named report
  and diagnostic helpers.
- src/boot/rpi5_reports.rs owns the normal Pi 5 report formatting helpers and
  linker-symbol layout report.
- src/diagnostics/rpi5.rs owns retained allocator, translation-fault,
  exception/fault, and panic diagnostic bodies.
- src/main.rs keeps rust_entry, panic/OOM handling, QEMU smoke entry, tests,
  and top-level orchestration only.

Final line counts after the extraction pass:

| File | Lines | Bytes |
| --- | ---: | ---: |
| src/main.rs | 612 | 22637 |
| src/boot/rpi5.rs | 548 | 27955 |
| src/boot/rpi5_reports.rs | 543 | 22180 |
| src/diagnostics/rpi5.rs | 643 | 23107 |
| src/target/rpi5.rs | 812 | 28272 |
| build.rs | 354 | 19459 |

Validation after the module split:

- cargo fmt --all -- --check passed.
- cargo -Zjson-target-spec test passed: 39 no_std tests.
- scripts/qemu-smoke.sh passed.
- scripts/rpi5-image.sh passed and produced target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img.
- scripts/rpi5-format-guard-check.sh passed.
- Representative retained diagnostic builds passed:
  scripts/rpi5-panic-report-diagnostic-image.sh,
  scripts/rpi5-normal-exception-report-diagnostic-image.sh,
  scripts/rpi5-translation-fault-diagnostic-image.sh, and
  scripts/rpi5-alloc-oom-diagnostic-image.sh.
- git diff --check passed.
- No Pi 5 hardware run was required because the refactor moved code without
  intentionally changing accepted normal boot output order, entry register
  programming, MMU/cache programming, allocator policy, or diagnostic cfg
  behavior.

### phase3-memory-fdt-module-refactor-20260523

The memory and FDT code split keeps the previous public memory_map::... and
device_tree::... call sites stable through module-level re-exports while moving
implementation responsibilities into named files.

Final module layout for this task:

- src/memory_map/mod.rs owns public re-exports and the high-level module
  boundary.
- src/memory_map/layout.rs owns kernel/FDT range inputs and conservative
  low-tail usable-memory candidate selection.
- src/memory_map/page_frames.rs owns page-frame seed, bootstrap reservation,
  no-free bootstrap allocator plan, and current ownership-contract metadata.
- src/memory_map/translation.rs owns translation-table layout, descriptor
  population, register/cache enable plans, and descriptor helper tests.
- src/memory_map/common.rs owns small range/alignment helpers shared by the
  layout, page-frame, and translation modules.
- src/device_tree/mod.rs owns the DeviceTree facade and public FDT re-exports.
- src/device_tree/raw.rs owns FDT header reads, structure-block cursoring,
  token constants, string lookup, block validation, and raw cell/string helpers.
- src/device_tree/memory.rs owns reservation-block, /memory, and
  /reserved-memory interpretation.
- src/device_tree/chosen.rs owns /chosen property lookup and bootargs decoding.

Before/after line counts for the extracted files:

| File | Before | After |
| --- | ---: | ---: |
| src/memory_map.rs | 1590 | removed |
| src/memory_map/mod.rs | new | 41 |
| src/memory_map/layout.rs | new | 330 |
| src/memory_map/page_frames.rs | new | 589 |
| src/memory_map/translation.rs | new | 639 |
| src/memory_map/common.rs | new | 70 |
| src/device_tree.rs | 795 | removed |
| src/device_tree/mod.rs | new | 28 |
| src/device_tree/raw.rs | new | 246 |
| src/device_tree/memory.rs | new | 543 |
| src/device_tree/chosen.rs | new | 65 |

Boundary tests added during this task:

- talos::device_tree::raw::tests::raw_fdt_helpers_bound_alignment_blocks_and_strings
- talos::device_tree::memory::tests::memory_cell_decoder_accepts_only_one_or_two_complete_cells

Validation after the memory/FDT split:

- cargo fmt --all -- --check passed.
- cargo -Zjson-target-spec test passed: 41 no_std tests.
- scripts/qemu-smoke.sh passed.
- scripts/rpi5-image.sh passed and produced
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img.
- scripts/rpi5-format-guard-check.sh passed.
- git diff --check passed.
- mdbook build was not run because mdbook is unavailable in the container.
- No Pi 5 hardware run was required because the refactor only moved code,
  preserved public call sites through re-exports, and did not intentionally
  change accepted normal boot output, memory ownership values, translation
  table layout/population, MMU/cache programming, allocator behavior, or FDT
  interpretation policy.

### phase3-maintainability-review-checkpoint-20260523

The review accepts the module-boundary work for main/boot reports, retained
diagnostics, memory-map responsibilities, and FDT parsing/policy boundaries,
but it does not clear the maintainability intervention yet. The next worker
must run one more deletion-focused cleanup before feature work resumes.

Largest Rust files after the refactors:

| File | Lines | Review note |
| --- | ---: | --- |
| src/target/rpi5.rs | 812 | Still mixes stable Pi 5 target services with early UART/handoff/phase-ladder reset diagnostics. |
| src/diagnostics/rpi5.rs | 643 | Retained current allocator, exception/fault, panic, and translation-fault diagnostics are isolated from the normal boot pipeline. |
| src/memory_map/translation.rs | 639 | Translation-table layout/population code and tests are cohesive and below the review target. |
| src/main.rs | 624 | Top-level entry, panic/OOM handling, QEMU smoke, and tests remain; normal Pi 5 boot orchestration moved out. |
| src/memory_map/page_frames.rs | 589 | Page-frame seed/reservation/ownership contract is cohesive and below the review target. |
| src/boot/rpi5.rs | 548 | Normal Pi 5 boot pipeline is still dense but no longer buried in src/main.rs. |
| src/device_tree/memory.rs | 543 | FDT memory policy extraction is separate from raw token/cell walking. |
| src/boot/rpi5_reports.rs | 543 | Normal report formatting helpers are isolated from boot-state transitions. |

Review findings:

- src/main.rs is no longer the catch-all orchestration module.
- Normal Pi 5 reports, retained diagnostic bodies, memory-map logic, and FDT
  parser/policy boundaries are visible from module paths.
- The current memory/FDT split should be preserved: raw FDT cursor/cell/string
  helpers belong in src/device_tree/raw.rs, Talos memory interpretation belongs
  in src/device_tree/memory.rs, and page-frame ownership should not drift back
  into translation-table descriptor code.
- Future workers must not add new one-off Pi 5 bring-up probes to src/main.rs
  or src/target/rpi5.rs. A diagnostic must either be a current regression gate,
  a bounded task diagnostic, or a documented fact with the stale code deleted.
- The cleanup sequence is not done because the repository still contains many
  stale bring-up scripts and cfgs from the audit's document-and-delete and
  delete-as-stale families. Examples include loader/armstub/EFI experiments,
  UART proof trees, entry/transition/vector/text reset probes, direct assembly
  reset probes, and the still-wired runtime UART, handoff UART, rust-uart10,
  boundary-entry reset, and phase-ladder target helpers.

Queued follow-up cleanup:

- phase3-target-rpi5-diagnostic-deletion-20260523 owns src/target/rpi5.rs,
  build.rs cfg/env plumbing, and stale scripts/rpi5-* diagnostic/proof wrappers
  that are not current validation gates.
- It must preserve only the accepted normal boot image/format gates and
  currently useful allocator, exception/fault, panic, and translation-fault
  diagnostic wrappers.
- It must document any remaining early serial or boot-boundary facts before
  deleting stale helper code, cfgs, and scripts.
- It must not implement page-frame free/reuse, heap expansion, high-memory/DMA,
  lower-EL/userspace, Phase 4 work, or normal boot-output changes.

Validation for this review checkpoint:

- Static inspection confirmed the module boundaries above.
- Static script/cfg inventory found remaining stale diagnostic/proof wrappers
  and target helpers, so another cleanup task is required.
- cargo fmt --all -- --check passed.
- cargo -Zjson-target-spec test passed: 41 no_std tests.
- scripts/qemu-smoke.sh passed.
- scripts/rpi5-image.sh passed and produced
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img.
- scripts/rpi5-format-guard-check.sh passed.
- git diff --check passed.
- mdbook build was not run because mdbook is unavailable in the container.
- No Pi 5 hardware run was required because this checkpoint changed docs and
  durable task state only; it introduced no runtime or boot-image behavior.
- Matthew notification remains required. Because this isolated cron run has no
  direct Telegram delivery surface, the supervisor state keeps a pending
  delivery note with the exact remaining cleanup work.

### phase3-target-rpi5-diagnostic-deletion-20260523

This deletion pass completed the maintainability follow-up from the review
checkpoint. It removed stale Pi 5 bring-up wrapper scripts for the
loader/armstub/EFI alternatives, UART proof trees, entry/fresh-entry/candidate
serial probes, transition/text/vector/fallthrough/post-stack boundary probes,
direct assembly reset/BRK/BTI exception classifier probes, boundary-entry
reset, and phase-ladder reset families. Those families were historical
bring-up experiments; accepted facts from them remain in the early-serial and
decision-log history rather than as active wrappers.

Before/after cleanup inventory:

| Item | Before | After |
| --- | ---: | ---: |
| src/target/rpi5.rs | 812 lines | 458 lines |
| build.rs | 354 lines | 136 lines |
| stale rpi5 script wrappers deleted | 0 | 82 |
| retained rpi5 scripts | many one-off probes plus gates | 25 current gate/diagnostic helpers |

Retained wrapper families:

- Normal Pi 5 image, boot tree/image, boot ramdisk tree, format guard, archive
  review, TFTP cursor, and TFTP wait helpers.
- Allocator and alloc-crate diagnostics: alloc OOM, realloc growth, Vec
  growth, String growth, and alloc format.
- Exception/fault diagnostics: exception report, normal exception report,
  exception return, undefined instruction, data abort, current SP0 sync, and
  translation fault.
- Panic diagnostics: panic report, full panic info, nested panic, and the
  nested-panic boot tree.

Code cleanup summary:

- src/target/rpi5.rs now contains stable Pi 5 target services, UART constants,
  early phase output, relocation helpers, and TargetServices construction; the
  stale runtime UART, handoff UART, rust-uart10, boundary-entry reset, and
  phase-ladder diagnostic entry points were deleted.
- build.rs now advertises and consumes only current allocator,
  exception/fault, panic, and translation-fault diagnostic env/cfg flags.
- Retained diagnostic wrappers no longer unset or compose stale bring-up env
  flags. The alloc-format wrapper now builds directly through the active
  alloc-format cfg instead of depending on a fresh-entry side marker.
- src/arch/aarch64/boot.S still contains dormant historical conditional
  assembly blocks, but no remaining build.rs or script path advertises or
  enables those stale flags. A future cleanup can remove the unreachable
  assembly bodies if it is worth the diff risk; this task removed the active
  repository surface that future workers would run.

Validation for this deletion pass:

- cargo fmt --all -- --check passed.
- cargo -Zjson-target-spec test passed: 41 no_std tests.
- scripts/qemu-smoke.sh passed.
- scripts/rpi5-image.sh passed and produced
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img.
- scripts/rpi5-format-guard-check.sh passed.
- Representative retained diagnostic image builds passed:
  scripts/rpi5-panic-report-diagnostic-image.sh,
  scripts/rpi5-normal-exception-report-diagnostic-image.sh,
  scripts/rpi5-translation-fault-diagnostic-image.sh,
  scripts/rpi5-alloc-oom-diagnostic-image.sh, and
  scripts/rpi5-alloc-format-diagnostic-image.sh.
- git diff --check passed.
- mdbook build was not run because mdbook is unavailable in the container.
- No Pi 5 hardware run was required because normal boot output, MMU/cache
  programming, allocator policy, FDT interpretation, and hardware-facing normal
  boot behavior were not intentionally changed.
