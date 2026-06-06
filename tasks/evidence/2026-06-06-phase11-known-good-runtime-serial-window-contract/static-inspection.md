# Static Inspection

Task id: phase11-known-good-runtime-serial-window-contract-20260606

## Inputs

- Prior accepted known-good runtime evidence:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/known-good-serial-observe-followup.json`.
- Latest direct-cursor missing-readiness evidence:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck/known-good-runtime-readiness-observe.json`.
- TFTP/fetch classification:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck/classification.json`.
- No-actionable source/artifact classification:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/evidence-map.json`.
- Lab-controller serial documentation and `scripts/rpi5-observe-runtime-readiness.sh`.

## Findings

- The prior accepted control reached `TALOS: kernel_main` and later accepted PASS output from a serial window beginning at cursor 4076948 and ending at 4085680.
- The latest direct-cursor run fetched the restored 104,136-byte known-good kernel twice under stable pre-restore TFTP evidence, but its serial observe from cursor 4096040 ended at 4096748 after only 708 bytes of firmware/RP1 output.
- The old helper used one `/serial/observe` request. With `settle_ms=1000`, that can return after an early quiet firmware gap; it is not equivalent to a 75-second readiness window.
- The repair is a tooling/contract change only: repeat observe calls until the requested deadline, advance the cursor between calls, and accumulate all text from the original fresh cursor before classifying markers.

## Disposition

- fixed: serial observer contract defect.
- fixed: docs now require `deadline-loop-accumulated-from-fresh-cursor` for known-good runtime readiness proofs.
- deferred: hardware truth remains for the next serialized Pi 5 discriminator.
- not-an-issue: no RP1 mapped/unmapped behavior is accepted by this repair.
