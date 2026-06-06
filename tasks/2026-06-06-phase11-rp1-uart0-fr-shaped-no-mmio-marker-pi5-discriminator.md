# Phase 11 RP1 UART0 FR-Shaped No-MMIO Marker Pi 5 Discriminator

Task id: phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator-20260606

Status: accepted

## Goal

Run the accepted FR-shaped no-MMIO marker candidate on the Pi 5 to decide
whether the selected FR-read-shaped path can produce visible pre-MMIO UART10
serial markers when the RP1 load is absent.

## Scope

- Published only the archive accepted by
  phase11-rp1-uart0-fr-shaped-no-mmio-marker-core-20260606.
- Acquired the hardware lock before candidate publication and retained
  hardware-lock evidence.
- Captured candidate identity, fresh serial/TFTP cursors, stable pre-restore
  TFTP evidence, repaired saturated-cursor serial direct-read evidence,
  pre-restore state, restore state, post-restore state, and lock-release
  evidence.
- Ran an additional clean candidate proof after a restored-tree power cycle and
  serial drain to rule out stale marker bytes in the direct-read path.

## Non-Goals

No source/code changes, RP1 UART0 FR volatile read, RP1 address constant
change, GPIO/pinmux/clock/reset work, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition.

## Classification

fr-shaped-no-mmio-marker-visible.

The accepted clean run staged tree
`2bd7db27d7bdf27a356c81408fefce059148f61e332fb3a207d280913b6ec27d` with a
45,600-byte `da591740/kernel_2712.img`. Stable pre-restore TFTP evidence
from cursor `4134781` retained 13 events, including two served candidate
kernel fetches. Starting from saturated serial cursor `4194304`, the repaired
direct-read window retained 70,004 bytes, firmware NETWORK output, and 2,730
occurrences of `TALOS: fr-no-mmio-loop`.

This accepts only that the FR-read-shaped path reaches UART10 pre-MMIO marker
output when the volatile RP1 UART0 FR load is absent. It does not accept RP1
UART0 FR mapped/read-value behavior, unmapped/trap behavior, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: the accepted candidate archive matched SHA-256
  `05a6801471ffd5cb3ae61f450734728f7980d8a2c4db20b3a6280d83b470a484`
  before hardware publication.
- fixed: the first candidate proof retained stable pre-restore TFTP evidence
  with 13 events, two 45,600-byte candidate kernel fetches, restored the
  pre-run tree, and showed 2,730 marker occurrences.
- fixed: a clean rerun power-cycled the restored tree and drained serial before
  republishing the candidate, then reproduced the same 13-event TFTP shape,
  two candidate kernel fetches, firmware NETWORK output, and 2,730 marker
  occurrences before restore.
- fixed: the lab boot tree was restored to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  before hardware-lock release.
- removed: the no-MMIO marker run did not and must not execute the volatile
  RP1 UART0 FR load, so it cannot classify mapped/read-value or unmapped/trap
  RP1 behavior.
- deferred: the original RP1 UART0 FR-read candidate still needs a separate,
  supervisor-planned next discriminator before another same-shaped FR-read
  hardware rerun.
- not-an-issue: the clean-run direct-read path started at the saturated serial
  cursor and therefore used `deadline-loop-direct-read-after-saturated-cursor`,
  which is the accepted repaired path for this lab state.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator/classification.json`.
- Accepted candidate proof:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator/candidate-clean-run/`.
- Initial candidate proof:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator/candidate-run/`.
- Validation summary:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator/validation-summary.txt`.

## Validation

- lab-controller API / serial hardware boot/output: passed for the accepted
  clean candidate run.
- TFTP stable same-cursor evidence: passed with 13 stable events and two
  45,600-byte candidate kernel fetches before restore.
- repaired saturated-cursor serial direct-read path: passed with 2,730 marker
  occurrences before restore.
- restore proof: passed; post-restore tree hash was
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- git diff --check: passed.
- mdbook build: passed because docs/src changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as fr-shaped-no-mmio-marker-visible. The next queued closeout may
reconcile this hardware discriminator before any further Phase 11 expansion or
same-shaped RP1 UART0 FR-read rerun.
