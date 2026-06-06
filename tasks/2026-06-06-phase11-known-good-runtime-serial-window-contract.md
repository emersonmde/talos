# Phase 11 Known-Good Runtime Serial-Window Contract

Task id: phase11-known-good-runtime-serial-window-contract-20260606

Status: accepted

## Goal

Repair or classify the no-hardware serial-observation contract for the restored known-good runtime-readiness proof before any RP1 candidate rerun.

## Scope

- Compared prior accepted runtime-ready known-good evidence with the latest direct-cursor fetch-without-readiness evidence.
- Inspected lab-controller serial cursor/observe semantics and the readiness helper.
- Repaired only the readiness helper and proof-contract docs so the next hardware proof observes the full bounded serial window across firmware quiet gaps.
- Kept RP1 candidate/source work, candidate rerun, GPIO, interrupts, DMA/cache, storage, networking, SSH, broader PCIe, and Milestone 11.2 blocked.

## Findings And Disposition

- fixed: the previous helper made one `/serial/observe` call with a 1000 ms settle window. The latest blocker evidence retained only 708 bytes through `RP1 FW: load 0`, while prior known-good control evidence shows Talos output can arrive later in the same boot window.
- fixed: `scripts/rpi5-observe-runtime-readiness.sh` now loops until the requested deadline, advances the serial cursor between observe calls, accumulates text from the original fresh cursor, and records `observe_contract=deadline-loop-accumulated-from-fresh-cursor`.
- fixed: lab-controller, roadmap, and Phase 11 RP1/PCIe contract docs now require the deadline-loop helper for known-good runtime readiness.
- not-an-issue: stable TFTP evidence from the direct-cursor run remains accepted as fetch visibility for the restored 104,136-byte `da591740/kernel_2712.img`; this task does not alter TFTP semantics.
- deferred: valid known-good Talos runtime readiness is still unaccepted until a serialized Pi 5 discriminator runs with the repaired serial-window helper and retained restore evidence.

## Evidence

- Evidence map: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-contract/evidence-map.json`.
- Static inspection: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-contract/static-inspection.md`.
- Readiness helper dry run: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-contract/script-dry-run.log`.
- Validation logs: `shell-syntax-check.log`, `git-diff-check.log`, `mdbook-build.log`, and `git-diff-cached-check.log`.

## Validation

- static source/script/docs/evidence inspection: passed.
- shell syntax check: passed.
- shell/script dry run: passed with a fake lab API response sequence; the helper accumulated a firmware burst plus later readiness markers and exited successfully.
- cargo fmt/test: not run; no Rust source, runtime, boot, target routing, or readiness-marker code was touched.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted with classification `ready-for-serial-window-discriminator`.

The next queued task may run exactly one serialized known-good Pi 5 serial-window discriminator if the hardware lock is unlocked/restored. RP1 entry-control candidate rerun and source work remain blocked until the serial-window discriminator and closeout accept `valid-known-good-talos-readiness`.
