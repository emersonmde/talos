# Phase 11 RP1 Observed Aperture Static Reconciliation

Task id: phase11-rp1-observed-aperture-closeout-20260608

Classification: observed-aperture-rp1-uart0-fr-visible-frontier-closed

## Reconciled Inputs

- Source contract:
  tasks/2026-06-08-phase11-rp1-observed-aperture-source-contract.md.
- Local/static core:
  tasks/2026-06-08-phase11-rp1-observed-aperture-core.md.
- Control proof:
  tasks/2026-06-08-phase11-rp1-observed-aperture-control-pi5.md.
- Real proof:
  tasks/2026-06-08-phase11-rp1-observed-aperture-pi5.md.
- Control classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-control-pi5/classification.json.
- Real classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/classification.json.

## Static Reconciliation

- Source contract selected exactly one real operation: a 32-bit volatile read
  from observed CPU physical address 0x1c00030018, the retained RP1 UART0
  PL011 flag-register offset after the accepted bridge/setup 0x1f mismatch.
- Local/static core implemented the real and control candidates with the
  accepted report shape, classification vocabulary, and archive review gates.
- Control Pi 5 proof accepted only the no-MMIO/no-PCIe/no-RP1/no-GIC output
  shape as no-mmio-observed-aperture-control-visible, with two 47,344-byte
  TFTP fetches, 72 control markers, and restore to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- Real Pi 5 proof accepted only the selected one-read observed aperture as
  observed-aperture-rp1-uart0-fr-visible, with two 47,664-byte TFTP fetches,
  69 result markers, raw=0x187, raw-is-pl011-fr-shaped=true, and restore to
  tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Accepted Claims

- The selected observed CPU physical address 0x1c00030018 returned a
  non-sentinel, non-zero, non-all-ones 32-bit value on Pi 5.
- The returned value raw=0x187 is PL011-FR-shaped under the local PL011 FR
  mask.
- The accepted evidence chain ties the selected candidate artifact, serial
  output, TFTP fetches, final pre-restore identity, and restore proof into one
  Pi 5 run.
- The no-MMIO/no-PCIe/no-RP1/no-GIC control output shape and capture path are
  visible on Pi 5.

## Retained Risks And Rejected Claims

- Endpoint ownership, broad RP1 mapping, UART ownership, interrupt delivery,
  GPIO/clock ownership, DMA/cache, storage, generated-root, networking, SSH,
  Milestone 11.3, and phase transition remain unaccepted.
- Same-shaped endpoint config identity, bridge/setup-state, 0x1f RP1
  peripheral, and 0x1c observed-aperture hardware reruns are not progress
  unless a future supervisor task supplies a different discriminator or new
  acceptance criteria.

## Next Action

No worker-owned follow-up task is created by this closeout. Supervisor
planning is required for the next Milestone 11.2 frontier if work should
continue.
