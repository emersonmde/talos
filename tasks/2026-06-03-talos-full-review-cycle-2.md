# Talos Full-System Review Cycle 2

Task: talos-full-review-cycle-2-20260603
Status: accepted
Started: 2026-06-03T07:19:30Z
Completed: 2026-06-03T07:45:00Z

## Scope

Ran the second full-system senior engineering review after the subsystem review
campaign and full-system review cycle 1. The pass focused on remaining
cross-subsystem coupling, script/test fragility, stale documentation, and
whether any retained diagnostic or fake-command surfaces were being expanded
instead of kept as regression controls.

No Talos OS feature, Pi 5 publication, hardware run, networking, RP1/PCIe,
UART interrupt ownership, DMA/cache policy, userspace launch, or shell-command
expansion was added.

## Findings

- Fixed: the nographic QEMU smoke scripts still duplicated the same Cargo
  build, objcopy, and QEMU virt launch stanza across dozens of scripts. Cycle 1
  already found one failure in this duplicated area when the Cargo test runner
  and scripts resolved QEMU inconsistently. The duplicated launch stanzas also
  made release-profile and SMP script behavior harder to review mechanically.
  Added `scripts/qemu-nographic-smoke-lib.sh` and migrated the simple
  nographic smoke scripts to call the shared helper while keeping each script's
  scenario-specific assertions and evidence-copy rules local.
- Fixed: README and the testing strategy documented QEMU binary resolution but
  not the new shared nographic smoke helper boundary. Updated both docs so
  future smoke scripts know where build/image/run setup belongs.
- Not an issue: socket-driven QEMU scripts such as diagnostic command-channel
  and local serial command-loop smokes still have bespoke QEMU launch code.
  They interact with QEMU while it is running, manage TCP serial sockets, and
  need different cleanup/connection logic, so pulling them into the nographic
  helper would hide important control flow.
- Not an issue: `scripts/qemu-runner.sh` remains separate from the nographic
  smoke helper. It is Cargo's test runner, takes an already-built kernel ELF,
  and has no smoke-log assertion or retained-evidence responsibilities.
- Not an issue: retained kernel-backed local command loop fixtures remain
  regression/control surfaces only. This review did not add command behavior or
  treat fake commands as POSIX progress.

## Changes

- Added `scripts/qemu-nographic-smoke-lib.sh` with two sourced helpers:
  `talos_qemu_prepare_image` for Cargo build plus objcopy, and
  `talos_qemu_run_nographic` for QEMU virt nographic execution.
- Migrated 38 simple nographic QEMU smoke scripts to the helper, preserving
  their selected boot scenario, debug/release profile, SMP setting, machine
  string, log path, greps, and retained-evidence copying.
- Updated README and `docs/src/project/testing-strategy.md` with the helper
  ownership rule.

## Validation

- Static inspection: reviewed roadmap/test docs, cycle 1 findings, QEMU script
  call sites, remaining direct QEMU invocations, retained fake-command surfaces,
  and script diff shape with `rg`, `sed`, and `git diff`.
- Script syntax: shebang-aware syntax check passed for all scripts.
- QEMU/substitute: `./scripts/qemu-smoke.sh` passed with
  `talos: qemu smoke PASS`, exercising the default no-scenario/no-SMP helper
  path.
- QEMU/substitute: `./scripts/qemu-readonly-initramfs-vfs-smoke.sh` passed
  with `qemu-readonly-initramfs-vfs-smoke: PASS`, exercising an EL2
  retained-evidence script.
- QEMU/substitute: `./scripts/qemu-cross-core-ipi-delivery-smoke.sh` passed
  with `qemu-cross-core-ipi-delivery: PASS`, exercising the SMP helper path.
- QEMU/substitute: `./scripts/qemu-production-secondary-dispatch-smoke.sh`
  passed with `qemu-production-secondary-dispatch: PASS`.
- QEMU/substitute: `./scripts/qemu-secondary-scheduler-service-loop-smoke.sh`
  passed with `qemu-secondary-scheduler-service-loop: PASS`, exercising the
  release-profile helper path.
- fmt: `cargo fmt --all -- --check` passed.
- fmt/lint/typecheck: `cargo -Zjson-target-spec check --quiet` passed.
- target check: `cargo -Zjson-target-spec check --quiet --target
  targets/aarch64-talos-rpi5-bcm2712.json` passed.
- Unit tests/QEMU runner: `cargo -Zjson-target-spec test --quiet` passed with
  the shared QEMU resolver environment.
- Docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- Diff hygiene: `git diff --check` passed.
- hardwareTestLock remained unlocked/restored and unused; no hardware run was
  performed.

## Remaining Risks

- Socket-driven QEMU command-channel scripts still duplicate some launch
  arguments, but their live interaction and cleanup semantics are meaningfully
  different from the simple nographic scripts. A future script-specific review
  can extract a socket helper if that duplication causes a real failure.
- The next POSIX-backed feature path should use the cleaned smoke helper for
  QEMU evidence, but should not add more kernel-backed command-loop fixtures as
  feature progress.

Implementation commit: 20dd708af263f7871c26bc4b705c4c7f523d9eb6.
Final acceptance/state commit: recorded in durable state after acceptance.
