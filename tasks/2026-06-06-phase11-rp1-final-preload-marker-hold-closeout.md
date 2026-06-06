# Phase 11 RP1 Final-Preload-Marker Hold Closeout

Task id: phase11-rp1-final-preload-marker-hold-closeout-20260606

Status: accepted

## Goal

Reconcile the final-preload-marker hold source/static and Pi 5 discriminator
evidence into the exact marker visibility boundary and next RP1 planning
frontier.

## Scope

- Reviewed the accepted no-RP1-MMIO final-preload-marker hold core evidence.
- Reviewed the serialized Pi 5 marker-visibility discriminator evidence.
- Recorded findings with disposition.
- Stated the final classification and exact accepted/unaccepted claims.
- Updated the Phase 11 RP1/PCIe map contract proof status and roadmap frontier.
- Identified the next planning frontier without creating a new task, promoting
  hardware work, or causing a phase transition.

## Non-Goals

No hardware run, boot archive publication, hardware lock acquisition,
source/runtime changes beyond docs/evidence hygiene, RP1 UART0 FR volatile
load, GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.2, or phase transition. This closeout does not
convert marker visibility into RP1 mapped/read-value or unmapped/trap
acceptance.

## Classification

final-preload-hold-marker-visible.

The accepted source/static candidate archive SHA-256 was
07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287. Static
inspection proves the selected rpi5_rp1_final_preload_marker_hold path emits
the delayed-marker FR-read-shaped start, pre-MMIO, before-RP1-read, bounded
repeated pre-load, and final pre-load marker strings, flushes UART10, and then
loops forever on TALOS: fr-final-preload-hold-loop. That selected path does
not call read_rp1_reg_u32, construct or use 0x1f_0003_0018, or execute the RP1
UART0 FR volatile load.

The Pi 5 discriminator published that accepted no-RP1-MMIO candidate as tree
101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47, selected
kernel_2712.img, and exposed the expected 45,816-byte
da591740/kernel_2712.img. Stable same-cursor TFTP evidence retained 13 events,
including two selected candidate kernel fetches. Direct serial read from the
saturated 4194304 cursor retained 57,040 bytes with 1,628 occurrences of
TALOS: fr-final-preload-hold-loop.

The direct-read window did not retain the earlier final pre-load marker. The
accepted boundary is therefore hold-marker visibility from the selected
no-RP1-MMIO candidate after candidate-tied fetch evidence, plus restore
hygiene. It does not accept RP1 mapped/read-value behavior, RP1 unmapped/trap
behavior, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or a phase
transition.

## Findings And Disposition

- fixed: reconciled the source/static no-RP1-MMIO hold candidate with the
  serialized Pi 5 marker-visibility discriminator evidence.
- fixed: retained source/static proof that the selected path reaches the
  delayed-marker reporting path and final pre-load marker before entering the
  unique hold loop.
- fixed: retained source/static proof that the selected hold path does not call
  read_rp1_reg_u32, construct/use 0x1f_0003_0018, or execute the RP1 UART0 FR
  volatile load.
- fixed: retained candidate archive SHA-256, staged tree, effective kernel,
  selected fetch path, selected fetch size, and kernel SHA-256.
- fixed: retained stable same-cursor TFTP evidence with two selected candidate
  kernel fetches before restore.
- fixed: retained repaired saturated-cursor direct serial evidence with 1,628
  visible occurrences of TALOS: fr-final-preload-hold-loop.
- fixed: retained restore proof to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
  hardware-lock release/restored state.
- removed: no final pre-load marker visibility claim is made because the
  decisive direct-read window retained only the later hold marker.
- removed: no RP1 mapped/read-value claim is made because this candidate
  intentionally does not execute the RP1 UART0 FR volatile load.
- removed: no RP1 unmapped/trap claim is made because this candidate
  intentionally does not execute the RP1 UART0 FR volatile load.
- deferred: returning to an actual RP1 UART0 FR read requires supervisor
  planning for a non-repetitive bounded source/static candidate and serialized
  Pi 5 proof with explicit acceptance gates.
- not-an-issue: this closeout performs no new hardware run and does not acquire
  the hardware lock.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-closeout/evidence-map.json.
- Source/static core task:
  tasks/2026-06-06-phase11-rp1-final-preload-marker-hold-core.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator.md.
- Pi 5 discriminator classification:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/classification.json.
- Pi 5 discriminator evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/evidence-map.json.

## Validation

- static evidence inspection: completed for core and Pi 5 discriminator records.
- git diff --check: passed.
- mdbook build: passed because docs/src changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as final-preload-hold-marker-visible.

The next bounded frontier is supervisor planning for a non-repetitive RP1 UART0
FR-read task that uses this accepted hold-marker visibility boundary as the
control point. The next actual hardware proof must have explicit queued
acceptance criteria and must not auto-promote from this closeout.
