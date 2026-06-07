# Phase 11 RP1 Mapping Frontier Static Reconciliation

Task id: phase11-rp1-mapping-frontier-checkpoint-20260607

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/evidence-map.json
- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/evidence-map.json
- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout/evidence-map.json
- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/evidence-map.json
- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-closeout.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-closeout/evidence-map.json

## Reconciliation

The source/static core accepts the paired diagnostic shape: the RP1 candidate
branches directly from rust_entry, performs exactly one 32-bit volatile load
from 0x1f00030018, and repeats the read result only after the load returns.
The paired control branches from the same early path, constructs no RP1 FR
address, performs zero RP1 loads, and repeats the same output shape as
simulated/control.

The no-MMIO control proof accepts the capture path for that repeated output
shape on Pi 5: selected control candidate identity, empty pre-power serial
drain, v2 identity join, stable same-cursor TFTP, final selected-tree identity,
restore proof, and 1,771 TALOS: fr-tail-stable-control occurrences.

The RP1 result proof accepts the read-only diagnostic boundary: selected RP1
candidate identity, empty pre-power serial drain, v2 identity join, stable
same-cursor TFTP, final selected-tree identity, restore proof, and 1,498
TALOS: fr-tail-stable-result occurrences carrying raw 0xdeaddead and
classification=mapped/read-value.

## Accepted Claims

- The read-only RP1 UART0 FR single-load diagnostic at 0x1f00030018 is
  accepted as mapped-read-value-tail-stable.
- The accepted proof is tied to the selected candidate by v2 identity, stable
  TFTP, final pre-restore identity, restore proof, and repeated tail-stable
  mapped/read-value markers.
- Milestone 11.1 has a narrow accepted RP1 mapping frontier sufficient to move
  to the first Milestone 11.2 source-contract task.

## Unaccepted Surfaces

- GPIO or pin-control ownership.
- RP1 clocks/resets.
- Interrupt routing or handling.
- DMA/cache behavior.
- Storage, generated-root, networking, or SSH.
- Broader PCIe enumeration.
- Milestone 11.2 implementation or phase transition.

## Next Task

phase11-rp1-irq-clock-gpio-source-contract-20260607 is the next mechanically
unblocked task after this checkpoint commit. It must remain a source-contract
task and must not perform runtime implementation or a hardware run.
