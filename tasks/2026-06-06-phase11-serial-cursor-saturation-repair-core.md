# Phase 11 Serial Cursor Saturation Repair Core

Task id: phase11-serial-cursor-saturation-repair-core-20260606

Status: accepted

## Goal

Repair or decisively classify the Pi 5 serial cursor/capture saturation that produced zero bytes from cursor 4194304 after candidate, known-good control, and candidate rerun fetch evidence.

## Scope

- Inspected the committed RP1 UART0 FR-read Pi 5 proof, the prior successful UART10 marker-loop proof, capture-invariant helpers, and lab-controller serial cursor contract.
- Repaired the reusable serial-window helper so a saturated saved cursor uses direct /serial/read capture instead of cursor-based /serial/observe replay.
- Updated the capture-invariant bundle summary contract so future evidence records carry the actual serial-window contract and saturated-cursor mode.
- Documented the lab-controller proof rule for saturated serial cursors.
- Per task scope, did not publish a boot archive, acquire hardwareTestLock, power-cycle the Pi 5, rerun RP1 UART0 FR-read, change RP1 constants, or accept RP1 MMIO behavior.

## Findings And Disposition

- fixed: the FR-read proof's first candidate run reached a fresh serial cursor of 4194304 and retained stable TFTP evidence with two 45,832-byte da591740/kernel_2712.img fetches, but cursor-based serial observation from 4194304 returned zero bytes.
- fixed: the known-good control also started at cursor 4194304, returned zero serial bytes, and still retained stable TFTP evidence with two 104,136-byte known-good kernel fetches. That rules out a simple candidate-only fetch failure and points at the serial capture path.
- fixed: the candidate rerun again started at cursor 4194304 and returned zero serial bytes. Repeating the same cursor-based FR-read run would not discriminate RP1 behavior.
- fixed: the successful UART10 marker-loop proof started below the cap at cursor 4133556, advanced to 4194304, and retained 60,748 fresh serial bytes with 2,961 TALOS: reu10-loop markers. The later FR-read proof started after that saturation boundary.
- fixed: scripts/rpi5-observe-serial-window.sh now auto-selects direct /serial/read when the saved cursor is at TALOS_SERIAL_CURSOR_SATURATION_LIMIT and annotates observe_contract=deadline-loop-direct-read-after-saturated-cursor.
- fixed: empty serial text now reports marker occurrence counts as zero rather than -1.
- fixed: scripts/rpi5-capture-invariant-proof-bundle.sh dry-run and summary metadata now describe the serial-window helper as auto observe-or-read and record the actual capture mode/start-saturation state from the helper output.
- fixed: docs/src/project/lab-controller.md now states that an empty observe window from a saturated cursor is not proof that the current boot emitted no serial output.
- not-an-issue: TFTP stability and restore evidence from the FR-read proof remain useful candidate/control boot evidence; this task changes only the serial capture contract.
- deferred: a serialized known-good Pi 5 proof still must confirm whether direct-read fallback can observe fresh Talos bytes after the prior saturation boundary before any same-shaped RP1 FR-read rerun is valid.

## Evidence

- Static inspection: tasks/evidence/2026-06-06-phase11-serial-cursor-saturation-repair-core/static-evidence-inspection.md.
- Evidence map: tasks/evidence/2026-06-06-phase11-serial-cursor-saturation-repair-core/evidence-map.json.
- Script syntax and capture-bundle dry run: tasks/evidence/2026-06-06-phase11-serial-cursor-saturation-repair-core/script-validation.log.
- Saturated-cursor direct-read fallback validation with fake curl: tasks/evidence/2026-06-06-phase11-serial-cursor-saturation-repair-core/read-fallback-validation.log and read-fallback-output.json.

## Validation

- static inspection of FR-read blocker evidence and prior UART10 marker-loop evidence: passed.
- bash -n scripts/rpi5-observe-serial-window.sh scripts/rpi5-capture-invariant-proof-bundle.sh: passed.
- no-hardware capture-bundle dry run with explicit dummy arguments: passed.
- no-hardware saturated-cursor direct-read fallback with fake curl: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

Accepted with classification ready-for-known-good-serial-cursor-completeness-pi5.

The next serialized hardware task may prove only serial cursor/capture completeness through the repaired direct-read fallback if hardwareTestLock remains unlocked/restored. RP1 mapped/read-value, trap/unmapped, firmware-state, pre-MMIO reachability, GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain unaccepted.
