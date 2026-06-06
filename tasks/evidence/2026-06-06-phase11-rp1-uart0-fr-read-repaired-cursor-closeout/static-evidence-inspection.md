# Static Evidence Inspection

Task id: phase11-rp1-uart0-fr-read-repaired-cursor-closeout-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun.md.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/evidence-map.json.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/classification.json.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/candidate-run/capture-invariant-summary.json.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/candidate-run/tftp-delta-stable-pre-restore.json.
- docs/src/project/phase11-rp1-pcie-map-contract.md.

## Findings

- The completed rerun selected the refreshed RP1 UART0 FR-read archive
  target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz with SHA-256
  da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60.
- The lab published tree
  25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71 with
  effective_kernel=kernel_2712.img.
- The expected candidate fetch was da591740/kernel_2712.img at 45,832 bytes.
  Stable same-cursor TFTP evidence from cursor 4129377 retained 13 events and
  two served candidate kernel fetches before restore.
- The fresh serial cursor was the saturated cursor 4194304, and the repaired
  direct-read capture retained 4,470 bytes of fresh firmware NETWORK output.
- The retained serial evidence did not reach TALOS: kernel_main,
  rpi5-rp1-uart0-fr-read: start,
  rpi5-rp1-uart0-fr-read: pre-mmio-read, mapped/read-value, trap/hang
  boundary text, or PASS.
- The first candidate run was not inconclusive for staging, capture, or
  evidence. Candidate identity, selected tree, stable candidate TFTP fetches,
  repaired serial capture, and restore evidence were retained.
- The restored post-run boot tree was
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- The evidence accepts candidate fetch/reset-loop and restore hygiene only. It
  does not accept RP1 UART0 FR mapped/read-value behavior, unmapped/trap
  behavior, firmware-state behavior beyond the reset-loop evidence,
  pre-MMIO reachability, GPIO, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, or a phase transition.

## Closeout Classification

candidate-fetch-reset-loop-without-visible-fr-marker

This closeout removes serial cursor saturation as the current blocker, because
fresh serial bytes were retained through the repaired path. It does not prove
the FR-read diagnostic reached its Rust serial markers or the RP1 MMIO
boundary. A future task should use a non-repetitive discriminator that separates
the FR-read candidate's pre-MMIO entry/reporting path from the already accepted
Rust-entry UART10 marker-loop observability before attempting another FR-read
hardware proof.
