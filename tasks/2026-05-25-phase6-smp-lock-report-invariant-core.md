# Phase 6 SMP Lock Report Invariant Core

Task: `phase6-smp-lock-report-invariant-core-20260525`

Status: accepted.

## Scope

This task implements the narrow report-state correction identified by the
accepted evidence hygiene and report inventory. It does not change the generic
`SpinLock<T>` contract and does not run Pi 5 hardware.

## Implementation

- Added `PerCoreState::republish_identity`, which refreshes context, MPIDR,
  affinity, and stack pointer fields without resetting lifecycle or workload
  progress.
- The Pi 5 secondary entry path now republishes those identity fields after
  the secondary cacheable-MMU handoff and before `handoff-ready` publication
  for the SMP lock proof.
- The QEMU SMP lock contention smoke now checks the final counter and each
  per-core report identity/progress line, not only the final PASS marker.

## Validation

- static inspection: inspected `src/smp.rs`, `src/target/rpi5.rs`, and
  `scripts/qemu-smp-lock-contention-smoke.sh` for the post-handoff identity
  publication and final-report assertions.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 103 no_std tests,
  including
  `republish_identity_refreshes_identity_without_resetting_progress`.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed and
  reported `counter=192 expected=192 participants=3 errors=0`,
  `classification=qemu-smp-lock-contention-complete`, and per-core
  `ok=true` reports for logical cores 1, 2, and 3.
- image/archive inspection:
  `scripts/rpi5-smp-lock-cache-coherence-image.sh` built
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-smp-lock-cache-coherence.img`.
- image/archive inspection:
  `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`
  passed with archive SHA256
  `73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`,
  kernel SHA256
  `e28596b5f259775c4c239c3e18b57e3d61d24ff453aa3c762c879e38075f7278`,
  kernel size 96,824 bytes, `text_offset=0`, `flags=12`, and
  `loader_diagnostic=false`.
- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in the container.
- hardware: no hardware commands were run for this task.

## Acceptance

Accepted as the narrow report-invariant core implementation. The corrected Pi
5 archive is ready for the separately queued serialized hardware proof task;
this task makes no Pi 5 hardware acceptance claim.
