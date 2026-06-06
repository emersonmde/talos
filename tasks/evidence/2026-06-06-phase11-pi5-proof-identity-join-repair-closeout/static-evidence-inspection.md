# Static Evidence Inspection: Proof Identity Join Repair Closeout

Task id: phase11-pi5-proof-identity-join-repair-closeout-20260606

## Sources Inspected

- `tasks/2026-06-06-phase11-pi5-proof-identity-join-repair-core.md`
- `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/evidence-map.json`
- `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/hold-control-candidate-run-identity-join-check.json`
- `tasks/2026-06-06-phase11-pi5-proof-identity-join-known-good-control.md`
- `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/evidence-map.json`
- `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/classification.json`
- `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/known-good-run-identity-join-check.json`
- `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/known-good-run/capture-invariant-summary.json`

## Inspection Result

Classification: proof-chain-ready-for-candidate-rerun.

The repair core accepted `proof-harness-identity-join-repaired` and introduced
the `pi5-proof-identity-join-v1` gate. That gate requires a shared run label,
selected tree hash, effective kernel, expected fetch path and byte count,
serial cursor/window identity, stable TFTP cursor/delta identity, final
pre-restore identity, and restore identity before a proof can support decisive
RP1 hardware behavior.

The retained old hold-control candidate run still fails the repaired gate:
the checker reports `capture-staging-blocked` with
`tftp-expected-fetch-byte-mismatch`,
`final-pre-restore-selected-tree-mismatch`, and
`final-pre-restore-expected-fetch-byte-mismatch`. That prevents the earlier
post-read serial bytes from accepting RP1 UART0 FR mapped/read-value or trap
behavior.

The known-good control passes the repaired gate. Its selected tree is
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, effective
kernel is `kernel_2712.img`, expected fetch is `da591740/kernel_2712.img`, and
expected fetch byte count is 104,136. Its stable TFTP evidence retained 13
events with two expected byte-matched fetches. Its fresh direct-read serial
window retained 7,070 bytes and the
`rpi5-production-timer-preemption: PASS` marker. Its final pre-restore and
post-restore identities matched the selected known-good tree. The checker
returned `decisive_rp1_hardware_classification_allowed=true`,
`classification=proof-chain-ready-for-candidate-rerun`, and no rejection
reasons.

## Findings And Disposition

- fixed: the accepted proof-chain boundary now distinguishes the old
  `capture-staging-blocked` hold-control run from a known-good run that passes
  `pi5-proof-identity-join-v1`.
- fixed: the next RP1 candidate proof must retain selected-tree, serial, TFTP,
  final pre-restore, and restore identity under one run label before accepting
  decisive hardware behavior.
- removed: known-good proof-chain readiness is not used as proof of RP1
  candidate fetch, mapped/read-value, or trap/unmapped behavior.
- not-an-issue: the known-good control's missing `TALOS: kernel_main` text does
  not block this proof-chain closeout because the retained downstream PASS
  marker is accepted for this production-timer control by the lab-controller
  proof contract.
- deferred: RP1 UART0 FR-read candidate behavior remains for the separately
  queued serialized Pi 5 candidate proof.

## Accepted And Unaccepted Claims

Accepted:

- Repaired proof-chain contract readiness for a later candidate rerun.
- The next candidate run must pass `pi5-proof-identity-join-v1` before any
  decisive RP1 classification.

Unaccepted:

- RP1 UART0 FR mapped/read-value.
- RP1 bus-fault/trap or unmapped/trap behavior.
- Firmware-state behavior or candidate behavior.
- GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH,
  broader PCIe, Milestone 11.2, or phase transition.
