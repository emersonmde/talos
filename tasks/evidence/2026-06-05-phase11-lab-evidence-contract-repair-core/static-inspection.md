# Static Inspection: Phase 11 Lab Evidence Contract Repair Core

Task id: phase11-lab-evidence-contract-repair-core-20260605

## Inputs Reviewed

- `tasks/2026-06-05-phase11-staging-capture-repair-closeout.md`
- `tasks/2026-06-05-phase11-staging-capture-log-stability-core.md`
- `tasks/2026-06-05-phase11-staging-capture-known-good-pi5-proof.md`
- `docs/src/project/lab-controller.md`
- `docs/src/project/phase11-rp1-pcie-map-contract.md`
- `docs/src/roadmap.md`
- `scripts/rpi5-wait-tftp-delta.sh`
- `scripts/rpi5-tftp-cursor.sh`

## Findings

- fixed: `GET /status` is now documented as the authoritative deployed lab API
  boot identity endpoint. The previously observed `GET /` 404 is retained as
  endpoint-semantics evidence only.
- fixed: hardware proof records now require `GET /status`, `GET /boot/files`,
  `GET /boot/snapshots`, fresh serial and TFTP cursors, stable pre-restore TFTP
  evidence, and final pre-restore status/boot-files/TFTP samples when
  inconclusive.
- fixed: classification rules now distinguish staging/publication mismatch,
  TFTP capture/logging blindness, serial-only firmware reboot, and valid
  known-good Talos readiness without accepting a narrower RP1 target.
- not-an-issue: `scripts/rpi5-wait-tftp-delta.sh` already records stability
  metadata over unchanged `cursor_end`, `log_size`, `truncated`, and parsed
  events. No helper edit was needed for this no-hardware contract repair.
- deferred: `phase11-known-good-boot-state-api-probe-20260605` must verify the
  deployed read-only API shape with retained `health`, `status`, `boot-files`,
  `boot-snapshots`, and `tftp-tail` evidence.

## Boundaries

No hardware lock was acquired, no power cycle occurred, no boot archive was
published, and no runtime/kernel/RP1 source was changed. This evidence does not
accept candidate fetch, Rust entry, entry-control reachability, RP1
mapped/read-value, RP1 unmapped/trap, known-good boot health, or Milestone 11.2
progress.
