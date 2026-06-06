# Phase 11 RP1 UART0 FR-Read Delayed-Marker Closeout

Task id: phase11-rp1-uart0-fr-read-delayed-marker-closeout-20260606

Status: accepted

## Goal

Reconcile the delayed-marker source/static and Pi 5 discriminator evidence into
the exact accepted RP1 UART0 FR volatile-read boundary and next planning
frontier.

## Scope

- Reviewed the accepted delayed-marker source/static core evidence.
- Reviewed the serialized Pi 5 discriminator evidence.
- Recorded findings with disposition.
- Stated the final classification and exact accepted/unaccepted claims.
- Updated the Phase 11 RP1/PCIe map contract proof status.
- Identified the next planning frontier without creating a new task or phase
  transition.

## Non-Goals

No hardware run, boot archive publication, hardware lock acquisition,
source/runtime changes beyond docs/evidence hygiene, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, or phase transition. This closeout does not convert marker-only or
no-return evidence into RP1 mapped/read-value acceptance.

## Classification

`candidate-fetch-without-final-preload-marker`.

The accepted source/static candidate archive SHA-256 was
`90452242f872eb085c9fe7963c02ad67556694326daebd7d199caf4ed5f597f4`. Static
inspection proves the selected path emits bounded delayed pre-load markers and
one final pre-load marker before exactly one 32-bit volatile load from
`0x1f_0003_0018`; post-load contract, raw-value, mapped/read-value, and PASS
output occur only after that load returns.

The Pi 5 discriminator published that accepted candidate shape as tree
`e9cd5c4a9571cab464ee76c046a7c4a2f42ba9cf75bb91f55de931dba16a3e2a`, selected
`kernel_2712.img`, and exposed the expected 46,152-byte
`da591740/kernel_2712.img`. The first candidate run and required candidate
rerun each retained stable TFTP evidence with two selected candidate kernel
fetches. Repaired saturated-cursor direct serial windows retained firmware
NETWORK output but did not show `TALOS: fr-delayed-preload-loop`, the final
pre-load marker, post-load value output, mapped/read-value classification, or
trap/panic output.

The closeout accepts only the source/static candidate shape, candidate
publication/fetch evidence, the exact
`candidate-fetch-without-final-preload-marker` classification, and restore
hygiene. It does not accept RP1 mapped/read-value behavior, RP1 unmapped/trap
behavior, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or a phase
transition.

## Findings And Disposition

- fixed: reconciled the delayed-marker source/static candidate with the Pi 5
  discriminator evidence.
- fixed: retained exact source/static proof that the selected path emits final
  pre-load evidence before one contracted `0x1f_0003_0018` volatile load.
- fixed: retained candidate archive SHA-256, staged tree, effective kernel,
  selected fetch path, and selected fetch size.
- fixed: retained stable same-cursor TFTP evidence for the first candidate run
  and candidate rerun.
- fixed: retained repaired saturated-cursor serial summaries for the first
  candidate run, known-good control, and candidate rerun.
- fixed: retained restore proof to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` and
  hardware-lock release/restored state.
- removed: no RP1 mapped/read-value claim is made because no final pre-load
  marker or post-load value output was visible.
- removed: no RP1 unmapped/trap claim is made because no final pre-load marker
  or attributable trap/panic output was visible.
- deferred: the next discriminator must be non-repetitive and
  supervisor-planned around post-handoff or serial-marker visibility before
  returning to the FR-read candidate.
- not-an-issue: this closeout performs no new hardware run and does not acquire
  the hardware lock.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-closeout/evidence-map.json.
- Source/static core task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator.md.
- Pi 5 discriminator classification:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/classification.json.
- Pi 5 discriminator evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/evidence-map.json.

## Validation

- static evidence inspection: completed for core and Pi 5 discriminator records.
- git diff --check: passed.
- mdbook build: passed because docs/src changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as `candidate-fetch-without-final-preload-marker`.

Because evidence is blocked before the final pre-load marker, this closeout
does not promote another same-shaped FR-read rerun. Supervisor planning is
required for a non-repetitive post-handoff or serial-marker visibility
discriminator before returning to the RP1 UART0 flag-register read.
