# Static Evidence Inspection

Task id: phase11-serial-cursor-saturation-repair-core-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof.md.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/candidate-run/capture-invariant-summary.json.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/known-good-control-run/run-summary.json.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/candidate-rerun/run-summary.json.
- tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/candidate-run/capture-invariant-summary.json.
- docs/src/project/lab-controller.md serial endpoint contract.
- scripts/rpi5-observe-serial-window.sh and scripts/rpi5-capture-invariant-proof-bundle.sh.

## Findings

- The successful UART10 marker-loop proof started at serial cursor 4133556 and advanced to cursor 4194304 while retaining 60,748 fresh serial bytes and 2,961 required markers. This proves the prior helper can capture serial before the retained log hits the 4 MiB cap.
- The FR-read first candidate started at cursor 4194304, retained two 45,832-byte candidate kernel fetches, and got zero serial bytes from cursor-based observation. Its pre-power peek tail still contained stale TALOS: reu10-loop output, matching a saturated retained-log boundary after the prior marker loop.
- The known-good control started at cursor 4194304, retained two 104,136-byte known-good kernel fetches, and got zero serial bytes. The same failure with a known-good tree makes a candidate-only RP1 explanation insufficient.
- The candidate rerun again started at cursor 4194304 and got zero serial bytes, while retaining a stable zero-event TFTP delta. The run does not establish RP1 mapped/read-value, trap/unmapped, firmware-state, or pre-MMIO reachability.
- Lab-controller docs state the serial log is capped to the most recent bytes with a default 4 MiB retention. Cursor 4194304 is exactly that default cap, so cursor-based replay from that point can remain pinned while newer bytes replace older retained bytes.

## Repair Classification

Repository-side repair is possible for the next proof path: when the saved cursor is saturated, avoid cursor-based replay and use deadline-looped direct /serial/read so newly consumed device bytes are returned directly. The repaired helper annotates the proof as deadline-loop-direct-read-after-saturated-cursor, with capture_mode=read and start_cursor_saturated=true.

This accepts only the capture-path repair/classification. A serialized known-good Pi 5 proof is still required to show fresh Talos bytes can be observed after the prior saturation boundary.
