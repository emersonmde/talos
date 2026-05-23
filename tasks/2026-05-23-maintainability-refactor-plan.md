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
