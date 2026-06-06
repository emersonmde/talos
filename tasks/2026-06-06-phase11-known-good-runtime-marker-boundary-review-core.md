# Phase 11 Known-Good Runtime Marker-Boundary Review Core

Task id: phase11-known-good-runtime-marker-boundary-review-core-20260606

Status: accepted

## Goal

Review whether the downstream production-timer PASS marker can accept known-good
Talos runtime readiness when the retained fresh serial window omits
`TALOS: kernel_main`.

## Scope

- Inspected the retained helper/direct serial evidence from
  `phase11-known-good-runtime-serial-window-pi5-discriminator-20260606`.
- Inspected source ordering for `TALOS: kernel_main`,
  `rpi5-production-timer-preemption: PASS`, and the restored known-good
  production-timer proof path.
- Updated the narrow readiness contract docs to distinguish full-marker proof
  from downstream-marker readiness.
- Did not acquire `hardwareTestLock`, run Pi 5 hardware, publish a boot
  archive, change RP1 source, or accept RP1 candidate behavior.

## Findings And Disposition

- fixed: source inspection proves `rpi5-production-timer-preemption: PASS` is
  downstream of `kernel_main` on the restored known-good production-timer
  control. The PASS marker is emitted only after the production-timer proof's
  report predicates succeed.
- fixed: the accepted marker boundary is now
  `valid-known-good-talos-readiness-by-downstream-marker` for this restored
  known-good control when stable TFTP fetch/restore identity are already
  accepted.
- fixed: docs now preserve `TALOS: kernel_main` as the preferred direct marker
  when present, but allow the downstream PASS marker to prove readiness for the
  current restored known-good production-timer control.
- not-an-issue: the retained serial omission of `TALOS: kernel_main` is a
  serial-log completeness limitation, not a runtime-readiness blocker, because
  the same fresh serial window contains the downstream PASS marker.
- deferred: no RP1 candidate fetch, Rust entry, entry-control reachability,
  mapped/read-value, unmapped/trap, firmware-state behavior, GPIO ownership,
  interrupts, DMA/cache, storage, networking, SSH, or Milestone 11.2 behavior is
  accepted by this task.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-known-good-runtime-marker-boundary-review-core/evidence-map.json`.
- Static inspection:
  `tasks/evidence/2026-06-06-phase11-known-good-runtime-marker-boundary-review-core/static-inspection.md`.
- Serial-window discriminator evidence:
  `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/serial-readiness-observe.json`,
  `serial-observe-direct-large-after-manual.json`, and
  `tftp-delta-stable-pre-restore.json`.
- Source references: `src/main.rs`, `src/boot/rpi5.rs`, and
  `src/target/rpi5.rs`.

## Validation

- Static source/evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted with classification
`valid-known-good-talos-readiness-by-downstream-marker`.

The next task is
`phase11-known-good-runtime-marker-boundary-closeout-20260606`. RP1
entry-control candidate rerun and RP1 source work remain blocked until that
closeout reconciles this classification and records the exact next action.
