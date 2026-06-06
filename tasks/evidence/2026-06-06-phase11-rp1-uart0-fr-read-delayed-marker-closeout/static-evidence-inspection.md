# Static Evidence Inspection

Task: phase11-rp1-uart0-fr-read-delayed-marker-closeout-20260606

## Inputs Inspected

- Source/static core task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core.md.
- Source/static evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/evidence-map.json.
- Source/static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/static-inspection.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator.md.
- Pi 5 discriminator classification:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/classification.json.
- Pi 5 discriminator evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/evidence-map.json.
- Pi 5 discriminator validation summary:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/validation-summary.txt.

## Reconciled Boundary

The source/static core accepted a non-published delayed-marker RP1 UART0 FR
candidate. Static inspection proves the selected path preserves the
FR-shaped UART10 reporting path, emits 32 bounded
`TALOS: fr-delayed-preload-loop` markers, emits
`rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker`, flushes
UART10, then performs exactly one 32-bit volatile load from
`0x1f_0003_0018`. Post-load contract, raw-value, mapped/read-value, and PASS
output are control-dependent on that load returning.

The serialized Pi 5 discriminator published the accepted archive
`target/talos-rpi5-rp1-uart0-fr-read-delayed-marker-core.tar.gz` with
SHA-256 `90452242f872eb085c9fe7963c02ad67556694326daebd7d199caf4ed5f597f4`.
The candidate tree
`e9cd5c4a9571cab464ee76c046a7c4a2f42ba9cf75bb91f55de931dba16a3e2a`
selected `kernel_2712.img` and exposed the expected 46,152-byte
`da591740/kernel_2712.img`.

Hardware evidence is limited to candidate fetch without the final pre-load
marker. The first candidate run retained stable same-cursor TFTP evidence with
13 events and two selected candidate kernel fetches. The repaired
saturated-cursor direct serial window retained 5,039 bytes with firmware
NETWORK output, but no `TALOS: fr-delayed-preload-loop`, final pre-load
marker, mapped/read-value output, or trap/panic output. Required triage then
ran a known-good control and candidate rerun. The candidate rerun again
retained stable TFTP evidence with two selected 46,152-byte candidate kernel
fetches and a 4,479-byte serial window with firmware NETWORK output, but still
no delayed-loop, final pre-load, mapped/read-value, or trap/panic output.

The restore boundary is retained: post-restore tree hash was
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, and the
hardware lock was released/restored by the discriminator task.

## Findings And Disposition

- fixed: reconciled the source/static candidate evidence with the serialized
  Pi 5 discriminator evidence.
- fixed: retained exact source/static proof that the final pre-load marker and
  one contracted `0x1f_0003_0018` volatile load are ordered on the selected
  path.
- fixed: retained candidate identity, archive SHA-256, staged tree, effective
  kernel, selected fetch path, and selected fetch size.
- fixed: retained stable same-cursor TFTP fetch evidence for the first
  candidate run and candidate rerun.
- fixed: retained repaired saturated-cursor serial summaries for the first
  candidate run, known-good control, and candidate rerun.
- fixed: retained restore proof and lock-release/restored state from the
  discriminator.
- removed: no RP1 mapped/read-value claim is made because no final pre-load
  marker or post-load value output was visible.
- removed: no RP1 unmapped/trap claim is made because no final pre-load marker
  or attributable trap/panic output was visible.
- deferred: the next discriminator must be non-repetitive and
  supervisor-planned around post-handoff/serial-marker visibility instead of
  another same-shaped FR-read rerun.
- not-an-issue: the known-good control in this specific discriminator also
  lacked `TALOS: kernel_main` in the bounded retained serial window; the
  task still completed required inconclusive-run triage and accepted only the
  candidate-fetch boundary.

## Accepted Claims

- The delayed-marker source/static candidate shape is accepted.
- The selected Pi 5 candidate archive was published and fetched.
- The Pi 5 hardware classification is exactly
  `candidate-fetch-without-final-preload-marker`.
- Restore hygiene and hardware-lock release/restored state are accepted.

## Unaccepted Claims

Visible final pre-load marker output, post-load RP1 UART0 FR value output,
RP1 mapped/read-value behavior, RP1 unmapped/trap behavior, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, and phase transition remain unaccepted.

## Next Planning Frontier

Because evidence is blocked before the final pre-load marker, the next
bounded Phase 11 task must be supervisor-planned and non-repetitive. Do not
run another same-shaped delayed-marker FR-read rerun from this closeout.
