# Talos Obsolete Bloat Removal Sweep

Task ID: talos-obsolete-bloat-removal-sweep-20260527
Status: accepted

## Goal

Remove every obsolete diagnostic, smoke, proof, gate, cfg, script, and current
documentation surface classified as remove-now by
tasks/2026-05-27-talos-obsolete-bloat-full-inventory.md.

## Scope

- Delete obsolete cfg-gated kernel code, build.rs scenarios, scripts, docs
  references, task/evidence hooks, and stale proof classifications identified
  as remove-now.
- Promote any retained behavioral check into a real unit/QEMU/host test or
  narrowly named active proof path.
- Preserve accepted evidence summaries, classifications, and artifact digests
  in historical task records.
- Verify no stale references to removed cfgs, scripts, scenarios, PASS
  strings, or diagnostic labels remain in active code, scripts, or current
  docs.

## Removal Summary

Removed 20 executable scripts and 18 boot scenarios:

- Historical QEMU secondary-core discriminator:
  scripts/qemu-secondary-core-discriminator.sh,
  qemu_secondary_core_discriminator, and
  run_secondary_core_discriminator.
- Old Pi 5 exception report probes:
  scripts/rpi5-exception-report-diagnostic-image.sh,
  scripts/rpi5-normal-exception-report-diagnostic-image.sh,
  scripts/rpi5-undefined-instruction-report-diagnostic-image.sh,
  scripts/rpi5-data-abort-report-diagnostic-image.sh,
  scripts/rpi5-current-sp0-sync-diagnostic-image.sh,
  rpi5_exception_report, rpi5_normal_exception_report,
  rpi5_undefined_instruction_report, rpi5_data_abort_report, and
  rpi5_current_sp0_sync.
- Old Pi 5 exception-return and translation-fault probes:
  scripts/rpi5-exception-return-diagnostic-image.sh,
  scripts/rpi5-translation-fault-diagnostic-image.sh,
  scripts/rpi5-translation-fault-diagnostic-tree.sh,
  rpi5_exception_return, rpi5_translation_fault, and matching assembly/Rust
  exception-return branches.
- Old Pi 5 panic probes:
  scripts/rpi5-panic-report-diagnostic-image.sh,
  scripts/rpi5-full-panic-info-diagnostic-image.sh,
  scripts/rpi5-nested-panic-diagnostic-image.sh,
  scripts/rpi5-nested-panic-diagnostic-tree.sh,
  rpi5_panic_report, rpi5_full_panic_info, and rpi5_nested_panic.
- Old Pi 5 allocator/container probes:
  scripts/rpi5-alloc-oom-diagnostic-image.sh,
  scripts/rpi5-realloc-growth-diagnostic-image.sh,
  scripts/rpi5-vec-growth-diagnostic-image.sh,
  scripts/rpi5-string-growth-diagnostic-image.sh,
  scripts/rpi5-alloc-format-diagnostic-image.sh,
  scripts/rpi5-page-frame-reuse-diagnostic-image.sh,
  scripts/rpi5-heap-expansion-policy-diagnostic-image.sh,
  rpi5_alloc_oom, rpi5_realloc_growth, rpi5_vec_growth,
  rpi5_string_growth, rpi5_alloc_format, rpi5_page_frame_reuse, and
  rpi5_heap_expansion_policy.

No remove-now surface was promoted or reclassified. Deferred items remain
limited to accepted evidence records, current Phase 4/5 Pi 5 proof scripts,
generic Pi 5 hardware helpers, and planned Phase 6.3 multi-core preemption
proof surfaces.

## Code Changes

- build.rs now registers 31 active boot scenarios, down from 49.
- src/main.rs no longer dispatches the QEMU secondary-core discriminator or
  old Pi 5 allocator/exception/panic diagnostic scenarios.
- src/target/qemu_virt.rs removes the historical discriminator function while
  retaining active secondary-core workload and scheduler/SMP proof paths.
- src/diagnostics/rpi5.rs retains the bootstrap allocator smoke used by the
  normal Pi 5 boot path and removes one-off allocator, exception, panic, and
  translation-fault diagnostics.
- src/boot/rpi5.rs removes diagnostic-only boot-report suppression and
  translation-fault bypass routing.
- src/arch/aarch64/vectors.S and src/arch/aarch64/exceptions.rs keep the
  normal framed exception path and remove one-off exception-report and
  exception-return branches.

## Validation

- static inspection: git status --short before edits passed; worktree was
  clean.
- static inspection: remove-now names were absent from build.rs, src/, and
  scripts after the sweep.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 147 no_std tests.
- QEMU/substitute: scripts/qemu-smoke.sh passed.
- QEMU/substitute: scripts/qemu-timer-preemption-smoke.sh passed.
- QEMU/substitute: scripts/qemu-secondary-scheduler-service-loop-smoke.sh
  passed with classification=qemu-secondary-scheduler-service-loop-complete.
- QEMU/substitute: scripts/qemu-shared-runqueue-migration-smoke.sh passed with
  classification=qemu-shared-runqueue-migration-complete.
- QEMU/substitute: scripts/qemu-load-balancing-smoke.sh passed with
  classification=qemu-load-balancing-smoke-complete.
- static stale-reference check: rg over build.rs, src/, and scripts found no
  removed scenario, script, cfg, or function names.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Result

Accepted as a comprehensive removal sweep. No hardware claim changed.
