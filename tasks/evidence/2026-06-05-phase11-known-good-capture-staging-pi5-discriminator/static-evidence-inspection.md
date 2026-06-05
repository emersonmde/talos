# Static Evidence Inspection

Task id: phase11-known-good-capture-staging-pi5-discriminator-20260605

Evidence level: static inspection of retained lab-controller API, serial
hardware, TFTP hardware, and restore records.

## Checks

- lab-status-before.json, lab-status-pre-restore.json, and
  lab-status-after-restore.json all report boot tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
  effective_kernel=kernel_2712.img.
- pre-run-snapshot.json and final-restore.json report successful snapshot
  creation and restore for
  pre-run-phase11-known-good-capture-staging-pi5-discriminator-20260605.
- known-good-serial-observe.json starts at fresh serial cursor 4094624 and
  contains Raspberry Pi firmware/RP1 boot output from the power cycle, but no
  TALOS: kernel_main, command-loop readiness, or PASS marker.
- known-good-tftp-delta-stable-pre-restore-rerun.json starts from fresh TFTP
  cursor 4094251, is stable, and contains 13 events, including two served
  da591740/kernel_2712.img fetches of 104,136 bytes.
- The initial stable TFTP query from the same cursor contains zero events,
  which is retained as capture-latency evidence for closeout reconciliation.
- No evidence file records RP1 candidate publication, RP1 runtime/source
  changes, RP1 mapped/read-value, RP1 unmapped/trap, GPIO, interrupts,
  DMA/cache, networking, SSH, broader PCIe, Milestone 11.2, or phase-transition
  behavior.

## Classification

known-good-fetch-observed-without-talos-readiness.

TFTP capture/staging observed the restored known-good boot fetches before
restore. Talos runtime readiness remains unproven because serial did not reach
accepted Talos markers from the fresh cursor.
