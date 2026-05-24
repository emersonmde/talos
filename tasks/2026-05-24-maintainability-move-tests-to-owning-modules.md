# 2026-05-24 - Move Tests To Owning Modules

Status: accepted locally; pending commit

Task: `talos-maintainability-move-tests-to-owning-modules-20260524`

## Scope

Move cross-module tests and bulky inline fixtures out of `src/main.rs` so test
ownership follows the module that owns the behavior. Keep parser behavior,
assertions, and the existing custom no_std test harness unchanged.

## Ownership Summary

Before this change, `src/main.rs` owned QEMU target service checks, FDT header
and memory/chosen parser fixtures, and Pi 5 target address checks.

After this change:

- `src/device_tree/raw.rs` owns the FDT header fixture test.
- `src/device_tree/memory.rs` owns memory reservation, memory bank, and
  reserved-memory range fixture tests.
- `src/device_tree/chosen.rs` owns the chosen bootargs fixture test.
- `src/target/mod.rs` owns the target service dispatch test.
- `src/target/rpi5.rs` owns Pi 5 address constant tests.
- `src/main.rs` keeps only the crate-level smoke test alongside entry,
  panic/OOM handling, and the no_std test runner.

The test names remain unchanged and searchable. The no_std test count is
unchanged at 51.

## Validation

- Static inspection: `src/main.rs` no longer contains large `TEST_FDT`
  byte fixtures, device-tree parser tests, or Pi 5 address tests.
- Fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 51 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed with
  `talos: qemu smoke PASS`.
- Image/archive inspection: `scripts/rpi5-image.sh` produced
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- Whitespace: `git diff --check` passed.
