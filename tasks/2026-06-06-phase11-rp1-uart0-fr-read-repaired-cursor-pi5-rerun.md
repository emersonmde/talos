# Phase 11 RP1 UART0 FR-Read Repaired-Cursor Pi 5 Rerun

Task id: phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun-20260606

Status: completed

## Goal

Rerun the narrow RP1 UART0 flag-register diagnostic on the Pi 5 using the
repaired saturated-cursor serial capture path.

## Scope

- Verify the accepted refresh-core candidate archive is present and unchanged.
- Publish only
  `target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz`.
- Acquire the hardware lock before publication/power-cycle and restore the
  pre-run boot tree before release.
- Retain candidate identity, fresh serial and TFTP cursors, stable pre-restore
  TFTP evidence, repaired serial-window evidence, and classification.

## Non-Goals

No source change, RP1 constant change, GPIO, pin mux, clocks/resets,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition.

## Findings And Disposition

- fixed: archive preflight confirmed the candidate archive SHA-256
  `da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60`
  and 45,832-byte `kernel_2712.img` still match the accepted refresh-core
  evidence.
- fixed: candidate publication staged tree
  `25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71`
  with effective kernel `kernel_2712.img` and a 45,832-byte
  `da591740/kernel_2712.img`.
- fixed: fresh TFTP cursor `4129377` produced stable pre-restore evidence
  with 13 events, including two served 45,832-byte
  `da591740/kernel_2712.img` fetches.
- fixed: fresh serial cursor `4194304` used the repaired direct-read capture
  path and retained 4,470 bytes of firmware NETWORK output from the candidate
  power cycle.
- removed: the candidate serial window did not contain `TALOS: kernel_main`,
  `rpi5-rp1-uart0-fr-read: start`,
  `rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or
  `PASS`.
- not-an-issue: a known-good control was not required because the first
  candidate run was not inconclusive for staging, capture, or evidence. The
  selected boot tree matched, TFTP was stable and non-empty, direct serial read
  captured fresh firmware output after the saturated cursor, and restore
  succeeded.
- deferred: RP1 UART0 FR mapped/read-value, unmapped/trap, or pre-MMIO
  reachability remains for a later discriminator; this task records blocker
  evidence only.

## Classification

`candidate-fetch-reset-loop-without-visible-fr-marker`.

The candidate was fetched and firmware reboot/reset-loop evidence was visible,
but no Talos or RP1 FR-read marker appeared in the repaired fresh serial
window. This does not accept RP1 mapped/read-value behavior, unmapped/trap
behavior, firmware-state behavior beyond the candidate fetch/reset-loop
evidence, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, or a phase transition.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/evidence-map.json`
- Classification:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/classification.json`
- Candidate run bundle:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/candidate-run/`
- Validation summary:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/validation-summary.txt`

## Validation

- archive preflight: `scripts/rpi5-archive-review.sh` passed on
  `target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz`.
- serialized Pi 5 hardware run: lab-controller publication, power cycle,
  repaired serial direct-read window, stable same-cursor TFTP delta, and
  restore evidence retained.
- diff checks: `git diff --check` passed before staging.
- docs: `/home/node/.cargo/bin/mdbook build` passed after updating the
  Phase 11 contract.
- staged diff checks: `git diff --cached --check` passed before commit.

## Result

Completed with committed blocker evidence. The next queued closeout may
reconcile the classification without inferring broader Phase 11 progress.
