# Talos Boot Scenario Cfg Routing Cleanup

## Task

- Title: Talos boot scenario cfg routing cleanup
- Owner: worker
- Date: 2026-05-26
- Milestone: repository health precursor before Phase 6.3 shared run-queue core
- Scope: build cfg, script routing, and existing boot scenario selection only

## Goal

Replace the broad set of one-off diagnostic, smoke, and proof cfg knobs with a
single checked boot-scenario selector before more scheduler work adds new proof
surfaces.

## Problem Statement

Talos had accumulated many public environment and cfg names for historical
diagnostics, smoke tests, and hardware proofs. That made the active build
surface look like a collection of temporary probes rather than a maintainable
open-source kernel interface. The cleanup is not required by the shared
run-queue algorithm itself, but it is a bounded repository-health precursor:
future Phase 6.3 proofs should add one scenario value, not another top-level cfg
namespace.

## Invariant

Before and after the cleanup, each retained QEMU or Pi 5 script must select the
same boot behavior it selected previously. The public selector surface should be
one environment variable, `TALOS_BOOT_SCENARIO`, mapped by `build.rs` into
checked `talos_boot_scenario` values and the minimum assembly defines needed for
early boot routing.

## Alternatives Considered

- Narrow shared run-queue core: leave cfg routing alone and implement only the
  next scheduler slice. This avoids touching unrelated files, but preserves the
  probe-style public build surface and makes future proof additions worse.
- Separate precursor cleanup: convert existing scripts and code to one scenario
  selector, validate representative default, QEMU, and test paths, then commit
  before shared run-queue implementation resumes. This addresses the repository
  hygiene issue without mixing it into scheduler behavior.

The second approach was chosen because Matthew explicitly called out the cfg
sprawl as below the bar for a public repository, and because the work is
behavior-preserving routing rather than scheduler feature implementation.

## Work Performed

- Consolidated the previous `TALOS_*_DIAGNOSTIC`, `TALOS_*_SMOKE`, and
  `TALOS_*_PROOF` environment/cfg surface into `TALOS_BOOT_SCENARIO`.
- Updated `build.rs` to register and validate one checked cfg namespace:
  `talos_boot_scenario = "..."`.
- Kept only a few internal assembly defines for early SMP and exception-vector
  routing where Rust cfg values are not directly available.
- Updated QEMU and Pi 5 scripts to pass `TALOS_BOOT_SCENARIO` values.
- Removed old diagnostic, smoke, and proof cfg names from active `build.rs`,
  `src/`, and script routing.

## Discriminator

The cleanup is accepted only if a default build, at least one non-default
scenario build, the QEMU timer IRQ smoke script, and the no_std test suite all
pass without old active cfg names reappearing in build or script routing.

## Evidence

- `cargo fmt --check`
- `cargo -Zjson-target-spec build`
- `TALOS_BOOT_SCENARIO=qemu_remote_wake_to_local_runnable cargo -Zjson-target-spec build`
- `./scripts/qemu-timer-irq-smoke.sh`
- `cargo -Zjson-target-spec test`: 134 passed
- `git diff --check`
- `rg` found no old active diagnostic/smoke/proof cfg names in `build.rs`,
  `src/`, or scripts. Historical docs were intentionally left unchanged.

## Result

Accepted as a bounded cleanup precursor. Shared run-queue implementation should
resume only as its own task after supervisor ready-marking, using the new
`TALOS_BOOT_SCENARIO` selector for any retained or new proof scenario.
