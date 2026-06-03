# Talos Full-System Review Cycle 1

Task: talos-full-review-cycle-1-20260603
Status: accepted
Started: 2026-06-03T07:00:00Z
Completed: 2026-06-03T07:28:00Z

## Scope

Reviewed the repository after the subsystem review/refactor tasks for
cross-subsystem coupling, stale abstractions, dead code, docs/test drift,
validation fragility, script hygiene, and remaining fake-feature surfaces.

No new Talos OS feature, hardware proof, Pi 5 boot publication, networking,
RP1/PCIe, UART interrupt ownership, or DMA/cache policy was added.

## Findings

- Fixed: every QEMU smoke script and the Cargo test runner assumed
  `qemu-system-aarch64` was available on `PATH`. In this workspace the pinned
  QEMU 9.2.0 binary lives outside normal `PATH`, so a full `cargo test` failed
  before executing the no_std test harness unless the caller remembered to
  export a local PATH override. The QEMU scripts now source
  `scripts/qemu-tool.sh`, which resolves `QEMU_SYSTEM_AARCH64`, then `QEMU`,
  then `qemu-system-aarch64` from `PATH`, and fails with an actionable message.
- Fixed: README and the testing strategy now document the shared QEMU binary
  resolution contract so local gates and the custom Cargo runner use the same
  setup rule.
- Not an issue: a naive `sh -n scripts/*.sh` pass reports a syntax error on
  Bash-only scripts that intentionally use arrays, `/dev/tcp`, and Bash read
  flags. A shebang-aware syntax pass over all scripts is the correct static
  gate and passes after the QEMU helper refactor.
- Not an issue: `docs/src/SUMMARY.md` references every project/architecture
  markdown page; the only markdown file outside the summary is the summary
  itself.
- Not an issue: ignored generated artifacts under `target/`, `book/`, and
  numeric lab cursor scratch files remain excluded by `.gitignore`; this review
  did not perform destructive local cleanup of ignored scratch state.
- Deferred: `cargo clippy` is not available for the pinned nightly toolchain in
  this environment. This is not a listed acceptance gate for this task; future
  toolchain maintenance can decide whether to add the component as a standard
  gate.

## Changes

- Added `scripts/qemu-tool.sh` as the shared QEMU binary resolver.
- Updated all direct QEMU smoke scripts and `scripts/qemu-runner.sh` to source
  the resolver and invoke `$qemu_tool` instead of a hard-coded binary name.
- Updated README and `docs/src/project/testing-strategy.md` with the
  `QEMU_SYSTEM_AARCH64` override rule.

## Validation

- Static inspection: reviewed source, docs, build.rs scenario/cfg registration,
  script shebangs, QEMU script call sites, docs summary links, ignored
  generated-artifact rules, recent review task records, and fake/kernel-backed
  shell references with `rg`, `find`, `sed`, and `git diff`.
- Pre-fix evidence: `cargo -Zjson-target-spec test --quiet` failed because
  `scripts/qemu-runner.sh` executed `qemu-system-aarch64` from `PATH` only.
- Script syntax: shebang-aware syntax check passed for 143 scripts.
- Tool resolution: `QEMU_SYSTEM_AARCH64=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin/qemu-system-aarch64 sh -c '. scripts/qemu-tool.sh; printf "%s\\n" "$qemu_tool"'` resolved the pinned workspace QEMU binary; a PATH-only negative check produced the expected actionable failure message.
- Unit tests/QEMU runner: `cargo -Zjson-target-spec test --quiet` passed with
  366 no_std tests using `QEMU_SYSTEM_AARCH64` instead of a PATH override.
- QEMU/substitute smoke: `./scripts/qemu-smoke.sh` passed with
  `talos: qemu smoke PASS` using `QEMU_SYSTEM_AARCH64`.
- QEMU/substitute smoke: `./scripts/qemu-readonly-initramfs-vfs-smoke.sh`
  passed with `qemu-readonly-initramfs-vfs-smoke: PASS` using
  `QEMU_SYSTEM_AARCH64`.
- fmt: `cargo fmt --all -- --check` passed.
- fmt/lint/typecheck: `cargo -Zjson-target-spec check --quiet` passed.
- target check: `cargo -Zjson-target-spec check --quiet --target
  targets/aarch64-talos-rpi5-bcm2712.json` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed; mdbook warned
  that the search index is large.
- diff hygiene: `git diff --check` passed.
- hardwareTestLock remained unlocked/restored and unused; no hardware run was
  performed.

## Remaining Risks

- The QEMU scripts still duplicate most launch arguments. The new resolver
  removes the most fragile cross-script dependency without broadening into a
  launch-argument generator; a future script review can decide whether central
  QEMU argument templates are worth the churn.
- `src/scheduler.rs` remains large, as already recorded by the scheduler
  subsystem review. This full cycle did not find a stable split that should be
  made before the Phase 8 process/VFS/userspace work clarifies ownership.
- Kernel-backed shell fixtures remain retained only as regression/control
  surfaces. No fake command behavior was expanded.

Implementation commit: pending.
Final acceptance/state commit: recorded in durable state.
