# Phase 11 Known-Good Runtime Serial-Window Closeout

Task id: phase11-known-good-runtime-serial-window-closeout-20260606

Status: accepted

## Goal

Close out the known-good serial-window runtime-readiness chain before any RP1 entry-control candidate rerun.

## Scope

- Reconciled the accepted no-hardware serial-window contract with the serialized Pi 5 discriminator evidence.
- Preserved the distinction between known-good fetch visibility, valid Talos runtime readiness, and RP1 candidate/source behavior.
- Kept runtime/kernel/RP1 source changes, boot archive publication, hardware runs, hardware lock acquisition, GPIO ownership, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase transition out of scope.

## Findings And Disposition

- fixed: the closeout evidence map now links the contract repair, hardware discriminator, restore proof, validation records, and commit records needed to understand the serial-window proof boundary.
- not-an-issue: the known-good TFTP fetch path remains accepted. The serialized discriminator stable pre-restore delta included two `da591740/kernel_2712.img` fetches for the 104,136-byte known-good image, and the post-run restore evidence kept `effective_kernel=kernel_2712.img`.
- deferred: `valid-known-good-talos-readiness` remains unaccepted. The retained fresh serial output reached `rpi5-production-timer-preemption: PASS`, but both helper and direct-large observations omitted the required `TALOS: kernel_main` marker.
- deferred: RP1 entry-control candidate rerun and RP1 source work remain blocked because their queued dependency requires this closeout to accept `valid-known-good-talos-readiness`.
- deferred: the smallest next discriminator is a supervisor-planned serial-log completeness/marker-boundary review that explains how the fresh serial window can contain a later production-timer PASS while omitting the earlier `TALOS: kernel_main` marker, or explicitly revises the accepted readiness marker boundary.

## Evidence

- Closeout evidence map: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-closeout/evidence-map.json`.
- Static evidence inspection: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-closeout/static-evidence-inspection.md`.
- Contract evidence: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-contract/evidence-map.json`.
- Discriminator evidence: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/evidence-map.json`.
- Discriminator classification: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/classification.json`.

## Validation

- static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted with classification `boot-runtime-readiness-blocked`.

This closeout accepts the repaired serial-window proof semantics and known-good fetch visibility, but it does not accept valid known-good Talos runtime readiness. The queued RP1 entry-control candidate rerun remains mechanically blocked. Supervisor planning is required for the next bounded serial-log completeness/marker-boundary discriminator before RP1 candidate/source work resumes.
