# Phase 10 RPi5 DTB Memory Scan Progress Core

Task: phase10-rpi5-dtb-memory-scan-progress-core-20260602
Status: accepted

## Goal

Add the smallest RPi5 boot-path progress reporting needed to classify the cd
fixed-directories candidate stop between "TALOS: dtb memory scan start" and the
local serial command-loop prompt.

## Static Review

The accepted candidate discriminator narrowed the blocker to a fresh cd
candidate that entered Talos and emitted "TALOS: dtb memory scan start", then
failed to retain "TALOS: dtb memory scan done", memory-plan output, cache
transition output, command-loop proof entry, prompt, cd transcript,
classification, or PASS.

The reviewed path is scan_memory_banks() through plan_boot_memory(),
enable_translation_and_caches(), and run_local_serial_command_loop_proof().
Existing output already had DTB scan start/done and MMU/I-cache/D-cache phase
lines, but unavailable memory-plan returns and local command-loop proof
entry/readiness did not have a compact raw UART boundary suitable for fresh Pi 5
evidence classification.

Detailed evidence: tasks/evidence/2026-06-02-rpi5-dtb-memory-scan-progress-core/static-review.txt.

## Implementation

- src/boot/rpi5.rs now reports DTB memory scan success and unavailable outcomes
  separately.
- src/boot/rpi5.rs now emits bounded raw UART markers for memory-plan start,
  memory-plan unavailable, memory-plan done with/without translation layout,
  cache-transition start, cache-transition unavailable, and cache-transition
  done.
- src/target/rpi5.rs now emits bounded raw UART markers for local command-loop
  proof entry, descriptor-backed IO readiness, and final prompt readiness.

These are diagnostic/proof-progress markers only. They do not change cd
semantics, cwd behavior, descriptor-backed filesystem behavior, userspace shell
execution, process lifecycle, networking, RP1/PCIe, UART interrupt ownership,
or DMA/cache policy.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed, 352 tests.
- QEMU/substitute cd feature:
  scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet passed.
- QEMU/substitute command-loop regression:
  scripts/qemu-local-serial-command-loop-smoke.sh --quiet passed.
- RPi5 image/archive inspection:
  scripts/rpi5-local-cd-fixed-dirs-boot-tree.sh rebuilt
  target/talos-rpi5-local-cd-fixed-dirs-dtb-progress-core.tar.gz, and
  scripts/rpi5-archive-review.sh passed with kernel size 110008 bytes.
- Image marker inspection retained the new progress marker strings in the
  rebuilt kernel image.
- Static diff hygiene: git diff --check passed.

Retained validation logs are under
tasks/evidence/2026-06-02-rpi5-dtb-memory-scan-progress-core/.

## Next Action

The original Pi 5 cd fixed-directories proof remains blocked. The next
mechanically queued task is the serialized Pi 5 dtb-scan-progress proof, which
can use these markers to classify the earliest fresh boundary or accept the cd
feature only if it retains the full pwd/cd transcript and PASS.
