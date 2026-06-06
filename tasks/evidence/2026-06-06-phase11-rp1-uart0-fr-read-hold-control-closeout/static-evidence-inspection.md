# Static Evidence Inspection

Task id: phase11-rp1-uart0-fr-read-hold-control-closeout-20260606

Evidence level: static evidence inspection of committed task, source/static,
hardware discriminator, lab-status, TFTP, serial, restore, validation, and
contract records.

## Inspected Records

- tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/static-inspection.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/evidence-map.json
- tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/evidence-map.json
- docs/src/project/phase11-rp1-pcie-map-contract.md

## Findings

- fixed: core evidence accepts the source/static candidate only. It proves the
  direct rust_entry branch, UART10 pre-read control marker, exactly one
  contracted 32-bit volatile load from 0x1f00030018, and post-read contract
  reporting if the read returns.
- fixed: core archive identity is retained as
  e9ab45b6dd15e4e80395302a116fb8aa751d699c5b679e5b9cee22077059a9b2 with a
  46,320-byte kernel image, contract id phase11-rp1-pcie-map-contract-v1,
  target rp1-uart0-fr-read, and width 32-bit.
- fixed: Pi 5 discriminator evidence retains hardware-lock acquisition,
  selected archive identity check, publication identity, serial capture, stable
  same-cursor TFTP, required triage attempts, restore proof, validation logs,
  and accepted commit b09842f1a6070215e6c3dc8966d3920f88948aff.
- fixed: publication identity was locally coherent: lab tree
  ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0,
  effective kernel kernel_2712.img, and expected 46,320-byte
  da591740/kernel_2712.img.
- fixed: main serial output retained 222,783 bytes and 5,582
  TALOS: fr-hold-control-post-read-loop occurrences, but stable same-cursor
  TFTP for that run recorded zero selected-candidate 46,320-byte fetches.
- fixed: triage evidence did not recover a candidate-tied capture. The
  known-good control had stable zero-event TFTP, and the candidate rerun again
  staged the 46,320-byte kernel but had stable zero-event TFTP plus empty
  serial observe output.
- fixed: lab restore returned to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardware-lock release.
- removed: no RP1 mapped/read-value, bus-fault/trap, pre-read-control-visible,
  or candidate-fetch-without-control-marker claim is carried forward from
  serial text that was not tied to selected-candidate TFTP fetch evidence.
- deferred: a future hardware classifier must first repair the capture/staging
  tie so selected-candidate fetch, serial cursor, and marker/read/trap output
  are retained for the same run.
- not-an-issue: docs/src/project/phase11-rp1-pcie-map-contract.md already
  records the hold-control discriminator classification and unaccepted
  surfaces, so this closeout does not require a docs/src edit.

## Classification

capture-staging-blocked.

## Accepted Boundary

The accepted boundary is limited to the source/static hold-control FR-read
candidate, hardware-locked publication/capture attempts, restore hygiene, and
the capture-staging-blocked classification. RP1 UART0 FR mapped/read-value
behavior, bus-fault/trap behavior, pre-read-control-visible-without-read-result,
candidate-fetch-without-control-marker, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remain unaccepted.

## Next Frontier

Supervisor planning is required for a non-repetitive capture/staging repair
task. This closeout rejects another same-shaped hardware FR-read rerun as the
next step because the current blocker is not the FR load shape; it is the
inability to tie selected-candidate fetch, serial cursor, and observed serial
bytes strongly enough in one run.
