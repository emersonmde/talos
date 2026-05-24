# Phase 6 Secondary-Core Controlled Workload

Task: `phase6-secondary-core-controlled-kthread-workload-20260524`

## Summary

This task adds a bounded diagnostic-only secondary-core workload on top of the accepted Phase 6.1 PSCI/per-core state path. The production scheduler remains single-core: secondary cores do not enter scheduler run queues, migrate tasks, share dispatch state, or run a general SMP scheduler.

## Implementation

- Extended `src/smp.rs` with `workload-running` and `workload-complete` lifecycle states plus a per-core workload progress counter.
- Added `run_controlled_secondary_workload`, which records deterministic progress to `64` and cleans the per-core state to the point of coherency.
- Added `TALOS_QEMU_SECONDARY_CORE_WORKLOAD_SMOKE` and `scripts/qemu-secondary-core-workload-smoke.sh`.
- Added `TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF`, `scripts/rpi5-secondary-core-workload-image.sh`, and `scripts/rpi5-secondary-core-workload-boot-tree.sh`.
- Reused the accepted secondary-core trampoline and stack-slot boundary. The workload is entered only by the focused diagnostics and is explicitly outside `scheduler.rs`.

## Evidence

- Unit tests: `cargo -Zjson-target-spec test` passed 97 no_std tests, including controlled workload lifecycle/progress coverage.
- QEMU/substitute: `scripts/qemu-secondary-core-workload-smoke.sh` passed. QEMU reported logical cores 1-3 in `workload-complete` with `progress=64 target=64 ok=true` and classification `qemu-secondary-core-controlled-workload-complete`.
- Pi 5 image/archive inspection: `scripts/rpi5-archive-review.sh target/talos-rpi5-secondary-core-workload-boot.tar.gz` passed with archive SHA256 `73e7419eef2ddc0e5ba6a4ac3756d5c0b1d0c2f5b6888b7759b9b921f6621fa7`, kernel SHA256 `a0ecfe8fef7ad4d144ed68ceefeadf325c4a5fa3ca9cb7b703f7c6e6927d8092`, and kernel size 91,288 bytes.
- Pi 5 hardware: `tasks/evidence/2026-05-24-pi5-secondary-core-workload-proof/serial-observe.json` shows cores 1-3 completed the workload with `progress=64 target=64 ok=true`, classification `pi5-secondary-core-controlled-workload-complete`, and `rpi5-secondary-core-workload: PASS`.
- TFTP/archive proof: `tasks/evidence/2026-05-24-pi5-secondary-core-workload-proof/tftp-delta-before-restore.json` shows `da591740/kernel_2712.img` fetched twice at 91,288 bytes; `publish.json` and `post-publish-status.json` show the candidate boot tree was active before restore.
- Restore proof: `restore-pre-snapshot.json` and `post-restore-status.json` show `pre-phase6-pi5-secondary-workload-20260524T202437Z` restored to the prior 82,045-byte boot tree.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 97 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` and `scripts/qemu-secondary-core-workload-smoke.sh` passed.
- image/archive inspection: `scripts/rpi5-image.sh`, `scripts/rpi5-secondary-core-workload-image.sh`, and `scripts/rpi5-archive-review.sh target/talos-rpi5-secondary-core-workload-boot.tar.gz` passed.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and `git diff --check` passed.
- static inspection: `mdbook` is unavailable in the container, so mdBook build was not run.

## Deferrals

The accepted workload is not an SMP scheduler. It does not add SMP-safe primitives, run-queue sharing, migration, load balancing, cross-core preemption, blocking I/O, userspace, syscalls, descriptors, filesystem, networking, SSH, or shell behavior.
