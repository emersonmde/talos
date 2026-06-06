# Phase 11 Pi 5 Capture Invariant Harness Core

Task id: phase11-pi5-capture-invariant-harness-core-20260606

Status: accepted

## Goal

Make the Pi 5 proof workflow explicit enough that the next post-handoff
marker/reset recheck can classify staging, capture, serial, and marker/reset
evidence without inferring RP1 MMIO behavior.

## Scope

- Inspected the latest staging/capture blocker evidence from the post-handoff
  marker/reset discriminator and closeout.
- Added a bounded serial-window helper that accumulates serial output from a
  fresh cursor until a deadline instead of stopping at the first settled
  firmware burst.
- Added a capture-invariant proof-bundle helper that records boot identity,
  fresh cursors, bounded serial output, stable pre-restore TFTP evidence,
  final pre-restore identity, restore evidence, and an annotated summary.
- Updated the lab-controller proof contract with the helper usage.
- Did not run hardware, publish a boot archive, acquire the hardware lock, or
  change kernel/RP1 diagnostic source.

## Final Classification

Classification: ready-for-post-handoff-marker-reset-capture-recheck.

The accepted helper workflow makes a zero-event TFTP delta meaningful only
when it is produced by the stable same-cursor helper before restore. It keeps
Raspberry Pi firmware NETWORK serial distinct from Talos entry/runtime markers,
captures final pre-restore status/files when the run is inconclusive, and
summarizes staging-publication-mismatch, tftp-capture-logging-blindness,
serial-only-firmware-reboot, post-handoff marker visibility, and reset-side
effect candidates without accepting RP1 mapped/unmapped behavior.

## Findings And Disposition

- fixed: the prior marker/reset Pi 5 discriminator retained the right evidence
  files, but the workflow was still hand-stitched; the new
  scripts/rpi5-capture-invariant-proof-bundle.sh helper records the proof
  bundle deterministically for the next recheck.
- fixed: a single serial observe can settle after firmware output, so
  scripts/rpi5-observe-serial-window.sh now loops until the requested deadline
  while accumulating output from the fresh cursor.
- fixed: zero-event TFTP evidence is only summarized as meaningful when it is
  stable under scripts/rpi5-wait-tftp-delta.sh and recorded before restore.
- fixed: the helper records selected tree identity, effective kernel, expected
  fetch path, expected fetch byte count, final pre-restore identity, and
  post-restore tree hash in capture-invariant-summary.json.
- fixed: a dry-run validation caught jq's reserved label token; the helper now
  uses proof_label internally and the passing dry-run record is retained.
- deferred: the helper does not itself acquire the hardware lock or publish the
  candidate archive; the next Pi 5 recheck task must do that under its own
  acceptance criteria.
- deferred: reset-side-effect acceptance remains a closeout decision based on
  hardware evidence; the helper only reports repeated expected fetch and
  firmware-network observations as a candidate classification.
- not-an-issue: no kernel/RP1 source change is required for this harness task.
- not-an-issue: firmware NETWORK output remains proof of reboot/serial wiring,
  not proof of Talos entry, marker visibility, candidate fetch, or RP1
  mapped/unmapped behavior.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-pi5-capture-invariant-harness-core/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-pi5-capture-invariant-harness-core/evidence-map.json.
- Dry-run proof-bundle output:
  tasks/evidence/2026-06-06-phase11-pi5-capture-invariant-harness-core/capture-invariant-proof-bundle-dry-run.json.
- Validation logs:
  tasks/evidence/2026-06-06-phase11-pi5-capture-invariant-harness-core/.

## Validation

- static inspection of latest staging/capture blocker evidence and
  lab-controller proof contract: completed.
- bash -n on changed shell scripts: passed.
- helper dry-run/no-hardware mode: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src/project/lab-controller.md changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as ready-for-post-handoff-marker-reset-capture-recheck.

This task accepts proof-harness and capture semantics only. It does not accept
post-handoff serial observability, reset-side-effect evidence, marker-path
hang/fault evidence, RP1 UART0 FR-read readiness, RP1 mapped/read-value,
RP1 unmapped/trap, firmware-state behavior, GPIO ownership, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, or a phase transition.

The next queued Pi 5 recheck is mechanically unblocked only if this task is
committed and hardwareTestLock remains unlocked/restored.
