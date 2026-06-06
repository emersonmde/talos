# Phase 11 RP1 Post-Handoff Marker Reset Capture Recheck Closeout

Task id: phase11-rp1-post-handoff-marker-reset-capture-recheck-closeout-20260606

Status: accepted

## Goal

Close out the repaired capture-invariant Pi 5 recheck for the post-handoff
marker/reset discriminator without promoting RP1 MMIO work from reset
side-effect-only evidence.

## Scope

- Reconciled the accepted marker/reset source/static core, the earlier
  staging-capture-blocked hardware discriminator, the repaired capture
  invariant, and the serialized Pi 5 recheck evidence.
- Recorded findings with disposition.
- Updated the Phase 11 RP1/PCIe map contract and roadmap with the accepted
  boundary.
- Did not run hardware, publish a boot archive, acquire the hardware lock,
  change kernel/RP1 source, or promote the RP1 UART0 FR-read task.

## Final Classification

Classification: reset-side-effect-accepted-marker-visibility-blocked.

The repaired capture-invariant run proves that the selected
target/talos-rpi5-post-handoff-marker-reset-core.tar.gz candidate was staged
and fetched before restore. Preflight and final pre-restore identity matched
tree 37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2,
effective kernel kernel_2712.img, and a 51,736-byte
da591740/kernel_2712.img. Stable same-cursor TFTP evidence from cursor
4111814 to 4118569 retained 65 events, including 10 served fetches of that
candidate kernel.

The fresh serial window from cursor 4113931 retained 19,625 bytes over 90
seconds and showed repeated Raspberry Pi firmware NETWORK boot/fetch cycles.
It did not contain TALOS: kernel_main, TALOS: rust_entry, or the unique
rpi5-rp1-post-handoff-marker-reset marker text. The repeated candidate fetches
therefore accept the PSCI reset-loop side effect for the selected
post-handoff-marker candidate, but they do not accept visible post-handoff
serial observability or the marker path's serial output.

This closeout does not accept RP1 UART0 FR-read readiness. The queued RP1
UART0 FR-read refresh remains blocked because it depends on visible
post-handoff serial observability, not reset-side-effect-only evidence.

## Findings And Disposition

- fixed: the repaired capture-invariant proof tied candidate identity to
  selected boot tree 37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2,
  effective kernel kernel_2712.img, 51,736-byte da591740/kernel_2712.img,
  fresh serial cursor 4113931, and fresh TFTP cursor 4111814.
- fixed: stable same-cursor pre-restore TFTP evidence proved 10 served
  candidate kernel fetches before restore, replacing the earlier
  staging-capture-blocked classification for this marker/reset candidate.
- fixed: restore hygiene was retained; the proof restored snapshot
  phase11-post-handoff-marker-reset-capture-recheck-pre-20260606T0852Z and
  post-restore status returned tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: reset side-effect evidence is now accepted for the selected
  marker/reset candidate because the repeated candidate fetch sequence is
  consistent with rust_entry reaching the PSCI SYSTEM_RESET path.
- deferred: visible post-handoff serial observability remains blocked because
  the fresh serial window did not show TALOS: kernel_main, TALOS: rust_entry,
  or rpi5-rp1-post-handoff-marker-reset.
- deferred: the marker-path serial output boundary remains unresolved; the
  smallest next discriminator is a supervisor-planned source/hardware step that
  can separate UART10 marker visibility from reset-side-effect-only
  reachability without touching RP1 MMIO.
- deferred: RP1 UART0 FR-read readiness, RP1 mapped/read-value, RP1
  unmapped/trap, and firmware-state behavior remain unaccepted.
- not-an-issue: known-good control and candidate rerun were not required by the
  recheck task because the first repaired capture-invariant run produced
  candidate-tied TFTP fetches and reset-loop side-effect evidence.
- not-an-issue: GPIO ownership, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, and phase transition behavior
  are outside this closeout and remain unaccepted.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-closeout/evidence-map.json.
- Source/static core task:
  tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-core.md.
- Earlier Pi 5 discriminator closeout:
  tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-closeout.md.
- Repaired capture-invariant harness/core task:
  tasks/2026-06-06-phase11-pi5-capture-invariant-harness-core.md.
- Pi 5 recheck task:
  tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5.md.
- Recheck classification:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/classification.json.
- Recheck evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/evidence-map.json.
- Relevant commits: a2d4c6add9e2ec7e91ba9dcd82c549c82ea01807,
  6b41f0d4798a03b4de8380e7095a9f375b17eb4d, and
  5c86ae2060b1aac4a7b6a4475e47d4b2de8e8019.

## Validation

- static evidence inspection: completed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as reset-side-effect-accepted-marker-visibility-blocked.

This closeout accepts candidate fetch and PSCI reset-loop side-effect evidence
for the selected no-RP1-MMIO marker/reset candidate only. It does not accept
visible post-handoff serial observability, RP1 UART0 FR-read readiness, RP1
mapped/read-value, RP1 unmapped/trap, firmware-state behavior, GPIO ownership,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition.

No later task is mechanically unblocked in the existing queue. Supervisor
planning is required for the next bounded post-handoff marker-visibility
discriminator before returning to the RP1 UART0 flag-register read.
