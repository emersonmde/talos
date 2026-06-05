# Phase 11 Known-Good Runtime Lineage And Cursor Repair

Task id: phase11-known-good-runtime-lineage-and-cursor-repair-20260605

Status: accepted

## Goal

Repair the known-good runtime-readiness evidence lineage after observed kernel
fetch but missing Talos readiness, without using hardware.

## Scope

- Traced the restored known-good boot tree, effective kernel, selected
  `kernel_2712.img` identity, prior accepted runtime readiness, and current
  readiness marker contract.
- Fixed the reusable cursor helpers so a missing or blank TFTP cursor cannot
  silently become the next hardware task's direct-cursor evidence.
- Preserved the Phase 11 Milestone 11.1 boundary: this task does not accept
  runtime readiness or any RP1 candidate behavior.

## Non-Goals Honored

No hardware lock acquisition, power cycle, boot archive publication, boot
restore, RP1 source/runtime change, candidate rerun, GPIO ownership,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2 work, or phase transition was performed. This task does not
accept known-good runtime readiness, RP1 candidate fetch, Rust entry,
entry-control reachability, mapped/read-value, or unmapped/trap behavior.

## Findings And Disposition

- fixed: lineage for the restored control is explicit. The restored tree hash
  is `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
  `configured_kernel=kernel_2712.img`, `effective_kernel=kernel_2712.img`, and
  `da591740/kernel_2712.img` is the 104,136-byte known-good image.
- fixed: observed fetch lineage is retained. The direct replay from retained
  fresh TFTP cursor `4095602` produced stable 13-event deltas with two served
  104,136-byte `da591740/kernel_2712.img` fetches.
- fixed: prior accepted runtime readiness on the same restored tree remains
  represented by the Phase 11 entry-proof known-good control, which reached
  `TALOS: kernel_main` and `rpi5-production-timer-preemption: PASS` after a
  104,136-byte known-good kernel fetch.
- fixed: `scripts/rpi5-tftp-cursor.sh` now fails if the lab API response does
  not contain a numeric `.tftp.cursor_end` or fallback `.cursor_end`, and
  `scripts/rpi5-wait-tftp-delta.sh` rejects blank or non-numeric cursors before
  querying `/tftp/logs`.
- deferred: the latest runtime-readiness discriminator still did not reach
  `TALOS: kernel_main`, `talos>`, or
  `rpi5-production-timer-preemption: PASS`; known-good runtime readiness remains
  unaccepted until a serialized direct-cursor hardware recheck proves it.
- removed: no alternate capture path, extra wait stack, candidate rerun, boot
  publication, source change, or phase transition was added.
- not-an-issue: the active readiness contract still requires a 75-second,
  1000 ms settle, 65536-byte serial observation with `TALOS: kernel_main` plus
  `rpi5-production-timer-preemption: PASS`; this task only repaired evidence
  lineage and cursor hygiene.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair/lineage-map.json`.
- Static source/doc/evidence inspection:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair/static-source-doc-evidence-inspection.md`.
- Cursor-caveat disposition:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair/cursor-caveat-disposition.md`.
- Shell syntax checks:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair/sh-n-tftp-helpers.log`.
- Diff hygiene:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair/git-diff-check.log`.
- Staged diff hygiene:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair/git-diff-cached-check.log`.

## Validation

- static source/doc/evidence inspection: passed.
- shell syntax checks: `sh -n scripts/rpi5-tftp-cursor.sh` and
  `sh -n scripts/rpi5-wait-tftp-delta.sh` passed.
- cargo fmt/test: not run; no Rust runtime/source files were touched.
- /home/node/.cargo/bin/mdbook build: not run; no `docs/src` files were
  touched and the proof rule did not change.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted lineage and cursor repair. The restored known-good tree is expected to
reach the accepted Talos readiness markers based on prior same-tree control
evidence, but the latest run did not prove readiness. The next mechanically
checkable path is exactly the queued serialized known-good direct-cursor Pi 5
recheck, using `scripts/rpi5-tftp-cursor.sh` for the authoritative fresh cursor
and `scripts/rpi5-wait-tftp-delta.sh` for direct stable pre-restore TFTP
evidence before restore.

No RP1 entry-control candidate rerun, RP1 source/runtime change, GPIO,
interrupt, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition is unblocked by this task.
