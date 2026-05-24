# 2026-05-24 - Flatten Pi 5 Boot Pipeline

Status: accepted locally; pending commit

Task: `talos-maintainability-flatten-pi5-boot-pipeline-20260524`

## Scope

Refactor `src/boot/rpi5.rs` so the Pi 5 boot path reads as named phases
instead of a single deeply nested orchestration body. Preserve normal boot
policy, serial ordering, retained diagnostics, allocator behavior, MMU/cache
setup, and hardware-facing output.

## Structure Summary

Before this change, `kernel_main` owned the full DTB, memory-candidate,
page-frame reservation, translation-table, MMU/cache, allocator, diagnostic,
and post-allocator reporting flow in one 500-line function with nested
`if let` blocks.

After this change, `kernel_main` is a short ordered phase list:

```text
report_boot_identity
plan_boot_memory
enable_translation_and_caches
init_bootstrap_allocator
report_post_allocator_memory
report_dtb_memory_banks
run_exception_fault_panic_diagnostics
```

The owning helpers split DTB reporting/scans, memory candidate selection,
bootstrap page reservation, translation-table planning, MMU/cache enabling,
allocator initialization, and post-allocator reports. Failure paths still
emit the same explicit serial-visible `unavailable` lines through
`report_unavailable`.

## Output Contract

Normal Pi 5 serial line ordering is intended to be preserved. This task did
not change memory/MMU/cache policy, allocator policy, diagnostic routing, or
the supported Pi 5 boot image format. The repeated Vec/String/alloc-format
diagnostic report-suppression predicate is now named once as
`suppress_growth_diagnostic_boot_reports`.

## Validation

- Static inspection: `src/boot/rpi5.rs` now has a short `kernel_main` and
  named phase helpers for DTB, memory planning, MMU/cache, allocator, and
  reports.
- Fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 51 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed with
  `talos: qemu smoke PASS`.
- Image/archive inspection: `scripts/rpi5-image.sh` produced
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- Fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` passed with
  `Pi 5 formatted early-console build PASS`.
- Representative retained diagnostics: panic report, normal exception report,
  translation fault, alloc OOM, page-frame reuse, and heap expansion policy
  image scripts all built `kernel_2712.img` successfully.
- Whitespace: `git diff --check` passed.
