# Phase 11 Known-Good Runtime Direct-Cursor Closeout

Task id: phase11-known-good-runtime-direct-cursor-closeout-20260605

Status: accepted

## Goal

Close out the repaired direct-cursor known-good runtime evidence and decide
whether RP1 entry-control candidate rerun is unblocked.

## Scope

- Reconciled the accepted lineage/cursor repair and the serialized direct-cursor
  Pi 5 recheck evidence.
- Distinguished proof semantics, known-good fetch visibility, direct-cursor
  evidence quality, known-good runtime readiness, and unresolved blockers.
- Preserved the Phase 11 Milestone 11.1 boundary.

## Non-Goals Honored

No runtime/kernel/RP1 source change, boot archive publication, hardware run,
hardwareTestLock acquisition, GPIO ownership, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2 work, or phase
transition was performed. This task does not accept RP1 candidate fetch, Rust
entry, entry-control reachability, mapped/read-value, unmapped/trap, or
firmware-state behavior.

## Findings And Disposition

- fixed: known-good fetch visibility is accepted through the repaired
  direct-cursor evidence path. Fresh TFTP cursor 4096953 produced stable
  pre-restore evidence with two served 104,136-byte da591740/kernel_2712.img
  fetches.
- fixed: direct-cursor evidence quality is accepted for this proof boundary.
  The blank-cursor caveat was repaired before the hardware run, and both stable
  pre-restore TFTP captures agreed.
- fixed: boot identity and restore hygiene are retained. Pre-run, pre-restore,
  and post-restore state all reported the restored known-good tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective_kernel=kernel_2712.img, and the pre-run snapshot was restored.
- deferred: valid known-good Talos runtime readiness is not accepted. Serial
  from fresh cursor 4096040 did not contain TALOS: kernel_main, talos>, or
  rpi5-production-timer-preemption: PASS.
- deferred: RP1 entry-control candidate rerun remains blocked because the
  required dependency, accepted valid-known-good-talos-readiness, is absent.
- removed: no hardware rerun, source change, boot publication, alternate
  capture path, extra wait stack, RP1 constant, MMIO read, GPIO work, interrupt
  work, DMA/cache work, storage work, generated-root work, networking, SSH,
  broader PCIe work, Milestone 11.2 work, or phase transition was added.
- not-an-issue: the direct-cursor Pi 5 recheck is completed with blocker
  evidence instead of accepted runtime readiness; this closeout records that as
  the accepted classification boundary.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-closeout/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-closeout/static-evidence-inspection.md.
- Lineage/cursor repair task:
  tasks/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair.md.
- Direct-cursor Pi 5 recheck task:
  tasks/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck.md.
- Direct-cursor classification:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck/classification.json.
- Diff hygiene:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-closeout/git-diff-check.log.
- Docs validation:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-closeout/mdbook-build.log.

## Validation

- static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed after docs updates.
- git diff --cached --check before commit: passed.

## Result

Accepted closeout classification:
known-good-direct-cursor-fetch-runtime-readiness-blocked.

Direct-cursor evidence proves known-good fetch visibility and restores the
hardware state cleanly, but valid known-good Talos runtime readiness remains
blocked after confirmed kernel_2712.img fetch. The queued RP1 entry-control
candidate rerun must not be promoted. Supervisor planning is required for the
next bounded boot/runtime readiness repair or discriminator before any RP1
candidate/source work.
