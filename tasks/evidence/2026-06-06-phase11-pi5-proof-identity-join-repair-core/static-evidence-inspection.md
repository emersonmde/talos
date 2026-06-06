# Static Evidence Inspection

Task id: phase11-pi5-proof-identity-join-repair-core-20260606

## Sources Reviewed

- tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator.md.
- tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-closeout.md.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/candidate-run/.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-closeout/static-evidence-inspection.md.
- scripts/rpi5-capture-invariant-proof-bundle.sh.
- scripts/rpi5-observe-serial-window.sh.
- scripts/rpi5-wait-tftp-delta.sh.
- docs/src/project/lab-controller.md.
- docs/src/project/phase11-rp1-pcie-map-contract.md.

## Findings

- fixed: the existing capture-invariant helper recorded preflight identity,
  serial window, stable TFTP, final pre-restore identity, and restore evidence,
  but did not mechanically reject TFTP fetches whose byte count differed from
  the selected candidate.
- fixed: the hold-control candidate-run preflight selected tree
  `ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0`,
  effective kernel `kernel_2712.img`, expected fetch
  `da591740/kernel_2712.img`, and expected byte count 46,320.
- fixed: the same run retained a direct-read serial window from saturated
  cursor 4,194,304 with 222,783 bytes and post-read-loop marker output, but the
  stable TFTP delta contained two 104,136-byte `da591740/kernel_2712.img`
  fetches instead of the selected 46,320-byte candidate.
- fixed: the final pre-restore identity was the restored tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
  not the selected candidate tree.
- fixed: `scripts/rpi5-proof-identity-join-check.sh` now replays retained
  bundles and rejects decisive RP1 hardware classification when selected-tree,
  serial, TFTP, final pre-restore, or restore identity does not join.
- not-an-issue: the old hold-control blocker remains valid as
  capture-staging-blocked evidence.
- deferred: hardware proof-chain readiness is left to the queued known-good
  control task.

## Replayed Result

`hold-control-candidate-run-identity-join-check.json` reports:

```text
classification: capture-staging-blocked
decisive_rp1_hardware_classification_allowed: false
rejection_reasons:
- tftp-expected-fetch-byte-mismatch
- final-pre-restore-selected-tree-mismatch
- final-pre-restore-expected-fetch-byte-mismatch
```

This demonstrates that the retained evidence cannot be summarized as
`mapped/read-value`, `bus-fault/trap`, or any other decisive RP1 hardware
classification.
