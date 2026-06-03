# Phase 8 Process Install Fixture Digest Regression

Task ID: phase8-process-install-fixture-digest-regression-20260603

Status: accepted

## Scope

- Restore the repo-wide no_std test gate after the accepted `/bin/init`
  fixture changed during initial userspace launch work.
- Keep the accepted current `/bin/init` fixture behavior intact.
- Do not expand shell behavior, process lifecycle, writable filesystems,
  networking, SSH, RP1/PCIe, DMA/cache policy, or Pi 5 hardware claims.
- Do not acquire hardwareTestLock or publish boot archives.

## Findings

- fixed: `process_install::tests::derives_metadata_only_install_plan_from_fixture`
  still asserted the old stable source digest `0x3892eed223900c65`
  after commit `b879d1a` changed the fixture text instruction from `ret` to
  `svc #0` for the accepted initial userspace launch signal. The current
  accepted `/bin/init` digest is `0xf4a6cc15f4d94461`.
- fixed: `program_loader` carried the same stale fixture digest constant, so
  its fixture identity test would also fail once the no_std harness reached it.
- fixed: process address-space, process page-table materialization, and the
  QEMU process-install smoke helper had stale source-digest literals in
  test-only or smoke-only fixture constructors for the same accepted
  `/bin/init` bytes.
- not-an-issue: the program loader and process-install implementations already
  derived the digest from the current initramfs fixture; the failure was stale
  test/smoke expectations, not a loader, VFS, or install planner regression.
- deferred: older roadmap and decision-log entries that record historical
  evidence with the old digest remain historical records and were not rewritten
  by this regression-gate task.

## Implementation

- Updated current fixture digest assertions and test/smoke constructors to
  `0xf4a6cc15f4d94461` in:
  - `src/program_loader.rs`
  - `src/process_install.rs`
  - `src/process_address_space.rs`
  - `src/process_page_table_materialization.rs`
  - `src/target/qemu_virt.rs`
- Added a local `PHASE8_INIT_DIGEST` constant in the process-install test module
  so the metadata assertion and negative-plan helper share one current fixture
  identity.

## Evidence

- Failing-test root cause:
  - Focused no_std test run first reproduced
    `process_install::tests::derives_metadata_only_install_plan_from_fixture`
    with actual digest `0xf4a6cc15f4d94461` and stale expected digest
    `0x3892eed223900c65`.
  - `git show b879d1a -- src/initramfs.rs` shows the accepted fixture changed
    the text bytes from `ret` (`c0 03 5f d6`) to `svc #0`
    (`01 42 0f d4`).
- Static inspection:
  - `rg` over `src/` found no remaining `0x3892_eed2_2390_0c65` source
    literals after the fix.
- Hardware:
  - Not run. This task made no physical claim and did not acquire
    hardwareTestLock.

## Validation

- `cargo fmt --all -- --check`: passed.
- `cargo -Zjson-target-spec test --quiet`: passed repo-wide no_std test gate.
- `cargo -Zjson-target-spec check --quiet`: passed for the default
  `aarch64-talos-virt` target.
- `git diff --check`: passed.
- `/home/node/.cargo/bin/mdbook build`: passed; warning only: large search
  index.
- `git diff --cached --check`: passed before commit.

## Acceptance

Accepted at: 2026-06-03T11:45Z

Commit: recorded in durable supervisor state after commit.
