# Static Evidence Inspection

Task id: phase11-staging-capture-discriminator-closeout-20260605

Evidence level: static inspection of accepted task records, lab-controller API
payloads, serialized Pi 5 hardware evidence, and docs state.

## Checks

- The contract repair accepted GET /status as the deployed authoritative
  boot identity endpoint and required stable pre-restore TFTP cursor replay
  before no-fetch classification.
- The read-only API probe classified the restored known-good tree as
  ready-for-serialized-discriminator. GET /status and GET /boot/files agreed
  on tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
  effective_kernel=kernel_2712.img.
- The serialized known-good discriminator retained lock, snapshot, status,
  boot-file, serial cursor, TFTP cursor, power-cycle, stable TFTP, restore,
  and post-restore evidence.
- The final stable pre-restore TFTP replay from cursor 4094251 produced 13
  events, including two served 104,136-byte da591740/kernel_2712.img fetches.
- Serial from the fresh cursor reached Raspberry Pi firmware/RP1 boot output
  but not TALOS: kernel_main, command-loop readiness, or PASS.
- The initial zero-event stable TFTP sample from the same cursor is retained as
  capture-latency evidence, but it is superseded by the final pre-restore
  stable replay for fetch classification.
- No accepted evidence records RP1 candidate fetch, Rust entry,
  entry-control reachability, RP1 mapped/read-value, RP1 unmapped/trap, GPIO,
  interrupt, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or phase-transition behavior.

## Classification

known-good-capture-staging-accepted-runtime-readiness-blocked.

The lab capture/staging path is accepted for known-good fetch visibility under
the repaired stable-log rule. Talos runtime readiness remains blocked after
observed known-good TFTP fetch, so RP1 candidate proof reuse requires
supervisor-planned follow-up.
