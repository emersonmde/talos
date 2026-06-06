# Phase 11 RP1 UART0 FR-Read Pi 5 Proof

Task id: phase11-rp1-uart0-fr-read-pi5-proof-20260606

Status: completed

## Goal

Run the refreshed RP1 UART0 flag-register read candidate on the Pi 5 and
classify only the narrow FR-read proof boundary.

## Scope

- Acquired hardwareTestLock, snapshotted the restored pre-run boot tree, and
  published the archive accepted by
  phase11-rp1-uart0-fr-read-refresh-core-20260606.
- Captured candidate identity, boot tree identity, fresh serial and TFTP
  cursors, stable same-cursor TFTP evidence, serial observations, known-good
  control evidence, candidate rerun evidence, and restore evidence.
- Did not change source or accept GPIO, pinmux, clocks/resets, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or a phase transition.

## Candidate

- Archive: target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz
- Archive SHA-256:
  da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60
- Candidate tree:
  25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71
- Effective kernel: kernel_2712.img
- Expected fetch: da591740/kernel_2712.img
- Expected fetch bytes: 45832
- Restore snapshot:
  phase11-rp1-uart0-fr-read-pre-20260606T1119Z

## Findings And Disposition

- fixed: selected candidate identity matched before the candidate power cycle:
  tree 25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71,
  effective kernel kernel_2712.img, and 45,832-byte
  da591740/kernel_2712.img.
- fixed: the first candidate run retained stable same-cursor pre-restore TFTP
  evidence with 13 events, including two served 45,832-byte candidate
  da591740/kernel_2712.img fetches. The authoritative pre-restore fetch bytes
  are retained in candidate-run/capture-invariant-summary.json.
- fixed: the first candidate run captured a fresh drained serial cursor
  4194304, but the bounded serial observation from that cursor returned zero
  bytes and did not contain rpi5-rp1-uart0-fr-read: start, pre-mmio-read,
  classification=mapped/read-value, or PASS.
- fixed: mandatory inconclusive-run triage was completed before any code
  change: candidate identity, fresh serial cursor, stable TFTP delta, known-good
  control, and candidate rerun were retained.
- fixed: the restored known-good control retained stable TFTP evidence with two
  104,136-byte da591740/kernel_2712.img fetches, but the fresh serial cursor was
  again 4194304 and the bounded serial observation returned zero bytes with no
  TALOS: kernel_main.
- fixed: the candidate rerun started from the republished candidate tree but
  again observed serial cursor 4194304 with zero serial bytes; its stable TFTP
  replay had zero events.
- fixed: the lab was restored to pre-run tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- removed: no RP1 mapped/read-value, trap/unmapped, firmware-state, or
  pre-MMIO reachability claim is made from this proof.
- deferred: the smallest next discriminator is a serial capture/cursor
  completeness repair or proof path that can observe fresh bytes after cursor
  4194304; another same-shaped FR-read rerun would not distinguish RP1 behavior
  from capture saturation.
- not-an-issue: candidate TFTP fetch evidence is sufficient to prove candidate
  publication/fetch for the first run, but not sufficient to accept RP1 MMIO
  behavior without serial classification output.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/classification.json.
- First candidate run:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/candidate-run/.
- Known-good control:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/known-good-control-run/.
- Candidate rerun:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/candidate-rerun/.
- Validation summary:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/validation-summary.txt.

## Validation

- lab-controller API candidate identity: passed through GET /status and
  GET /boot/files.
- serial hardware boot/output: completed with blocker evidence; fresh serial
  cursor was captured, but observations from cursor 4194304 returned zero bytes
  for candidate, known-good control, and candidate rerun.
- TFTP hardware evidence: passed for the first candidate run and known-good
  control; candidate rerun retained stable zero-event TFTP evidence.
- restore proof and hardware lock release: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

## Result

Completed with blocker classification
serial-capture-saturated-after-candidate-fetch.

This does not accept RP1 mapped/read-value behavior, RP1 unmapped/trap
behavior, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or a phase
transition.
