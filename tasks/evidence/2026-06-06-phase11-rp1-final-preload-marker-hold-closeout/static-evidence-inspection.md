# Static Evidence Inspection

Task: phase11-rp1-final-preload-marker-hold-closeout-20260606

## Inputs Inspected

- Source/static core task:
  tasks/2026-06-06-phase11-rp1-final-preload-marker-hold-core.md.
- Source/static evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/evidence-map.json.
- Source/static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/static-inspection.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator.md.
- Pi 5 discriminator classification:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/classification.json.
- Pi 5 discriminator evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/evidence-map.json.
- Pi 5 discriminator validation summary:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/validation-summary.txt.

## Reconciled Boundary

The source/static core accepted a non-published no-RP1-MMIO
final-preload-marker hold candidate. Static inspection proves the selected
path branches from rust_entry into run_rp1_final_preload_marker_hold,
preserves the delayed-marker FR-read-shaped start, pre-MMIO, and
before-RP1-read UART10 lines, emits 32 bounded
TALOS: fr-delayed-preload-loop markers, emits
rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker, flushes UART10,
and then loops forever on TALOS: fr-final-preload-hold-loop.

The selected path does not call read_rp1_reg_u32, does not include that symbol,
does not construct or use 0x1f_0003_0018, and does not execute the RP1 UART0 FR
volatile load. The only retained ldr w10, [x9, #0x18] instructions in the
reviewed hold disassembly are UART10 PL011 FR polling with x9 =
0x10_7d00_1000.

The serialized Pi 5 discriminator published the accepted archive
target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz with SHA-256
07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287. The
candidate tree
101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47 selected
kernel_2712.img and exposed the expected 45,816-byte
da591740/kernel_2712.img.

Hardware evidence accepts marker visibility for the selected candidate. Stable
same-cursor TFTP evidence before restore retained 13 events and two selected
candidate kernel fetches. The repaired saturated-cursor direct serial read
retained 57,040 bytes and 1,628 occurrences of
TALOS: fr-final-preload-hold-loop. The direct-read window did not retain the
earlier final pre-load marker, so final-preload marker visibility itself
remains unaccepted.

The restore boundary is retained: post-restore tree hash was
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and the
hardware lock was released/restored by the discriminator task.

## Findings And Disposition

- fixed: reconciled the source/static no-RP1-MMIO hold candidate with the
  serialized Pi 5 marker-visibility discriminator evidence.
- fixed: retained exact source/static proof that the selected path reaches the
  final pre-load marker and then enters the unique hold loop without executing
  RP1 UART0 FR MMIO.
- fixed: retained candidate identity, archive SHA-256, staged tree, effective
  kernel, selected fetch path, selected fetch size, and kernel SHA-256.
- fixed: retained stable same-cursor TFTP fetch evidence before restore.
- fixed: retained repaired saturated-cursor direct serial evidence for the
  visible hold marker.
- fixed: retained restore proof and lock-release/restored state from the
  discriminator.
- removed: no final pre-load marker visibility claim is made because the
  direct-read window retained the later hold marker but not the earlier final
  marker.
- removed: no RP1 mapped/read-value claim is made because the selected
  candidate intentionally avoids the RP1 UART0 FR volatile load.
- removed: no RP1 unmapped/trap claim is made because the selected candidate
  intentionally avoids the RP1 UART0 FR volatile load.
- deferred: the next actual RP1 UART0 FR-read discriminator must be
  supervisor-planned with explicit source/static and serialized Pi 5 acceptance
  criteria.
- not-an-issue: this closeout performs no new hardware run and does not acquire
  the hardware lock.

## Accepted Claims

- The source/static final-preload-marker hold candidate shape is accepted.
- The selected Pi 5 candidate archive was published and fetched.
- The Pi 5 hardware classification is exactly final-preload-hold-marker-visible.
- The selected no-RP1-MMIO candidate's unique hold marker is visible on Pi 5.
- Restore hygiene and hardware-lock release/restored state are accepted.

## Unaccepted Claims

Visible final pre-load marker output before the hold loop, RP1 mapped/read-value
behavior, RP1 unmapped/trap behavior, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remain unaccepted.

## Next Planning Frontier

Because the no-RP1-MMIO hold marker is visible after candidate-tied fetch, the
next bounded Phase 11 frontier may return to an actual RP1 UART0 FR read only
through supervisor planning. That task needs explicit source/static and
serialized Pi 5 acceptance gates; no hardware FR-read run is mechanically
promoted by this closeout.
