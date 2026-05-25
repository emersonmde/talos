# Phase 6 SMP Lock Proof Scaffolding Quarantine

Task: `phase6-smp-lock-proof-scaffolding-quarantine-20260525`

Status: accepted.

## Scope

This task removes temporary SMP lock entry-discriminator scaffolding after the
final physical Pi 5 SMP lock/cache-coherence proof was accepted. It does not
change lock behavior, scheduler behavior, or hardware acceptance criteria.

## Cleanup

- Removed the
  `TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_ENTRY_DISCRIMINATOR` build input,
  assembly define propagation, Rust `cfg`, and all matching production
  references.
- Removed early serial marker emissions for the entry-discriminator path from
  `src/arch/aarch64/boot.S`, `src/main.rs`, `src/boot/rpi5.rs`, and
  `src/target/rpi5.rs`.
- Removed the temporary scripts:
  `scripts/rpi5-smp-lock-cache-coherence-entry-discriminator-image.sh` and
  `scripts/rpi5-smp-lock-cache-coherence-entry-discriminator-boot-tree.sh`.
- Preserved accepted task records and evidence directories, including the
  entry-discriminator history in
  `tasks/2026-05-24-phase6-pi5-smp-lock-cache-coherence-proof.md`.

## Evidence Hygiene

- Before edits: `git status --short` showed a clean Talos worktree.
- Production-path search after cleanup found no remaining
  `ENTRY_DISCRIMINATOR`, `entry_discriminator`, `entry-discriminator`,
  `kernel-main-dispatch`, or entry-discriminator script references in
  `build.rs`, `src/`, or `scripts/`.
- After cleanup, `git status --short` showed only the intended tracked
  source/script/doc changes and no untracked placeholder evidence files.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 103 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed with
  `counter=192 expected=192 participants=3 errors=0` and per-core
  `ok=true` reports for logical cores 1, 2, and 3.
- image/archive inspection:
  `scripts/rpi5-smp-lock-cache-coherence-image.sh` built the retained Pi 5
  proof image.
- image/archive inspection:
  `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`
  passed with archive SHA256
  `73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`,
  kernel size 96,824 bytes, `text_offset=0`, and `flags=12`.
- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in the container.
- hardware: no hardware commands were run for this cleanup task.

## Acceptance

Accepted as scaffolding quarantine for the Milestone 6.2 SMP lock proof. The
temporary entry-discriminator path is no longer part of production builds, and
the accepted final proof records still provide the reproducible archive and
hardware evidence.
