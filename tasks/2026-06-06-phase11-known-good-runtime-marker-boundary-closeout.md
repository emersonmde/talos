# Phase 11 Known-Good Runtime Marker-Boundary Closeout

Task id: phase11-known-good-runtime-marker-boundary-closeout-20260606

Status: accepted

## Goal

Close out the known-good runtime marker-boundary review and record the exact
readiness boundary before any RP1 entry-control candidate rerun.

## Scope

- Reconciled the prior serial-window closeout, the accepted marker-boundary
  review core, retained serial/TFTP evidence, restore state, docs, deferred
  work, and risks.
- Accepted known-good Talos runtime readiness only for the restored known-good
  production-timer control under the downstream
  `rpi5-production-timer-preemption: PASS` marker boundary.
- Kept runtime/RP1 source changes, boot archive publication, Pi 5 hardware,
  hardware lock acquisition, RP1 candidate rerun, GPIO ownership, interrupts,
  DMA/cache, storage, generated-root work, networking, SSH, broader PCIe,
  Milestone 11.2, and phase transition out of scope.

## Findings And Disposition

- fixed: the exact closeout classification is
  `valid-known-good-talos-readiness`. It is derived from the accepted core
  classification `valid-known-good-talos-readiness-by-downstream-marker`.
- fixed: the accepted boundary is the restored known-good 104,136-byte
  `da591740/kernel_2712.img` production-timer control with stable TFTP
  fetch/restore identity and a fresh serial window containing
  `rpi5-production-timer-preemption: PASS`.
- fixed: the closeout evidence map links the serial-window closeout,
  marker-boundary review, retained serial/TFTP evidence, validation logs, and
  prior commit records.
- not-an-issue: the missing `TALOS: kernel_main` text in the retained serial
  window is treated as serial-window completeness loss, not as a runtime
  readiness blocker for this known-good control, because source order proves the
  PASS marker is downstream of `kernel_main`.
- deferred: this does not accept RP1 candidate fetch, Rust entry,
  entry-control reachability, mapped/read-value, unmapped/trap, firmware-state
  behavior, GPIO ownership, interrupts, DMA/cache, storage, generated-root work,
  networking, SSH, broader PCIe, or Milestone 11.2 behavior.
- deferred: the queued serial-completeness hardware discriminator is not
  mechanically eligible from this closeout because readiness is already accepted
  by the downstream marker boundary.

## Evidence

- Closeout evidence map:
  `tasks/evidence/2026-06-06-phase11-known-good-runtime-marker-boundary-closeout/evidence-map.json`.
- Static evidence inspection:
  `tasks/evidence/2026-06-06-phase11-known-good-runtime-marker-boundary-closeout/static-evidence-inspection.md`.
- Prior serial-window closeout:
  `tasks/2026-06-06-phase11-known-good-runtime-serial-window-closeout.md` and
  `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-closeout/evidence-map.json`.
- Marker-boundary review core:
  `tasks/2026-06-06-phase11-known-good-runtime-marker-boundary-review-core.md`
  and
  `tasks/evidence/2026-06-06-phase11-known-good-runtime-marker-boundary-review-core/evidence-map.json`.

## Validation

- Static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted with classification `valid-known-good-talos-readiness`.

The existing queued `phase11-rp1-entry-control-candidate-rerun-20260605` is the
only RP1 candidate task made mechanically eligible by this closeout, subject to
its own hardware lock and validation gates. No RP1 candidate fetch, Rust entry,
entry-control reachability, mapped/read-value, unmapped/trap, firmware-state,
GPIO, interrupt, DMA/cache, storage, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition is accepted here.
