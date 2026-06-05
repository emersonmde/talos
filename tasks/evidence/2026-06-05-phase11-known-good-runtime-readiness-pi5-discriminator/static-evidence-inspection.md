# Static Evidence Inspection

Task id: phase11-known-good-runtime-readiness-pi5-discriminator-20260605

## Inspection

- lock: hardwareTestLock was acquired for the task and released after
  final-restore.json reported ok=true.
- boot identity: lab-status-before.json, lab-status-pre-restore.json, and
  lab-status-after-restore.json all report tree_hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
  effective_kernel=kernel_2712.img.
- serial: known-good-runtime-readiness-observe.json starts at cursor 4095332,
  ends at 4096040, and contains 708 bytes of Raspberry Pi firmware/RP1 boot
  output with no TALOS: kernel_main, talos>, or
  rpi5-production-timer-preemption: PASS.
- TFTP: tftp-tail-before.json retained fresh cursor 4095602. Stable replay
  after restore from that cursor returned 13 events on both checks, including
  two served da591740/kernel_2712.img fetches of 104,136 bytes.
- caveat: known-good-tftp-delta-pre-restore.json and
  known-good-tftp-delta-stable-pre-restore.json were collected with a blank
  cursor because the wrapper initially looked for top-level cursor_end instead
  of .tftp.cursor_end. The task records this as blocker evidence rather than
  accepting runtime readiness.
- restore: final-restore.json and lab-status-after-restore.json show the
  pre-run known-good tree restored.

## Disposition

- fixed: one serialized hardware run and all identity/serial/restore evidence
  were retained.
- fixed: durable TFTP cursor replay proves known-good kernel fetch visibility
  after the fresh cursor.
- deferred: valid known-good Talos runtime readiness remains unaccepted because
  serial markers did not appear.
- deferred: pre-restore delta collection has an evidence-capture caveat that
  must be handled by closeout or supervisor planning before candidate reuse.
- removed: no same-shaped rerun, source change, candidate publication, or
  alternate capture path was added.
- not-an-issue: helper exit code 1 matches absent readiness markers.
