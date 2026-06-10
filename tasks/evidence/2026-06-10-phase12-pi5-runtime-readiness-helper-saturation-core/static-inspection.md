# Static Inspection

Task id: phase12-pi5-runtime-readiness-helper-saturation-core-20260610

Reviewed:

- tasks/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery-closeout.md
- tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/capture-summary.json
- tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/serial-observe-after-power.json
- tasks/2026-06-06-phase11-serial-cursor-saturation-repair-core.md
- scripts/rpi5-observe-runtime-readiness.sh
- scripts/rpi5-observe-serial-window.sh
- docs/src/project/lab-controller.md

Findings:

- The accepted recovery closeout held GPIO32 retries because the restored
  known-good tree recovered TFTP but not expected Talos serial output.
- The retained recovery summary records serial_cursor_before_power=4194304,
  matching the default lab-controller retained-log cap.
- The retained serial observe result starts and ends at cursor 4194304 with
  zero bytes.
- The prior Phase 11 saturation repair established the repository-side
  contract for saturated cursors: direct /serial/read with explicit metadata
  instead of a cursor-based /serial/observe replay.
- Before this task, scripts/rpi5-observe-runtime-readiness.sh always used
  cursor-based /serial/observe, so the same saturated cursor could repeat the
  serial-silent failure without recording a distinct capture contract.

Disposition:

- fixed: applied the saturated-cursor read fallback to the runtime-readiness
  helper while preserving the unsaturated observe loop.
- fixed: added helper metadata for capture mode, saturation state, response
  bytes, and selected observe/read contract.
- fixed: classified saturated direct-read zero bytes as
  saturated-cursor-capture-blocked.
- deferred: proving valid known-good runtime readiness remains a serialized
  Pi 5 proof after closeout.
