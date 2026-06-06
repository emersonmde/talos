# Static Evidence Inspection

Task id: phase11-pi5-capture-transaction-v2-closeout-20260606

## Inputs

- tasks/2026-06-06-phase11-pi5-capture-transaction-forensics-core.md
- tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/classification.json
- tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/evidence-map.json
- tasks/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5.md
- tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/classification.json
- tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/evidence-map.json

## Findings

- fixed: the forensics core accepted capture-transaction-v2-ready and
  documented why retained f274ff7 serial remains capture-staging-blocked
  without an empty pre-power /serial/read drain and selected-tree TFTP/final
  identity.
- fixed: the sentinel task accepted no-mmio-sentinel-identity-joined from a
  clean Pi 5 rerun under the v2 contract.
- fixed: sentinel evidence joins selected tree
  101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47,
  effective kernel kernel_2712.img, selected fetch
  da591740/kernel_2712.img, 45,816-byte candidate fetches, an empty
  pre-power serial drain, fresh hold-marker serial, final pre-restore identity,
  and restore proof.
- fixed: the v2 proof-chain boundary is now ready for the already queued RP1
  UART0 FR-read hold-control v2 proof task.
- removed: the compromised first sentinel attempt and retained f274ff7
  saturated serial output are not decisive RP1 behavior evidence.
- deferred: RP1 UART0 FR mapped/read-value, bus-fault/trap, and
  firmware-state behavior remain unaccepted until the queued RP1 proof passes
  the same v2 identity join with the selected hold-control candidate.
- not-an-issue: this closeout is static and does not need a hardware run or
  hardwareTestLock acquisition.

## Classification

proof-chain-ready-for-rp1-fr-read-v2.
