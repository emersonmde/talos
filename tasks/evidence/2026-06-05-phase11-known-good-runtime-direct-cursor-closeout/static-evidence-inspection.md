# Static Evidence Inspection

Task id: phase11-known-good-runtime-direct-cursor-closeout-20260605

Evidence level: static inspection.

## Inspection

- The accepted lineage/cursor repair maps the restored tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  effective_kernel=kernel_2712.img, and the 104,136-byte
  da591740/kernel_2712.img known-good image.
- The repair task fixed the blank TFTP cursor caveat by making
  scripts/rpi5-tftp-cursor.sh fail on missing numeric cursor output and
  scripts/rpi5-wait-tftp-delta.sh reject blank or non-numeric cursors.
- The direct-cursor Pi 5 recheck retained fresh serial cursor 4096040 and
  fresh authoritative TFTP cursor 4096953 before power cycling the restored
  known-good tree.
- Direct stable pre-restore TFTP replay from cursor 4096953 retained 13 events
  and two served da591740/kernel_2712.img fetches of 104,136 bytes.
- Pre-run, pre-restore, and post-restore lab status all retained the same
  restored known-good tree hash and effective_kernel=kernel_2712.img.
- The bounded 75-second, 1000 ms settle, 65536-byte serial readiness window
  did not contain TALOS: kernel_main, talos>, or
  rpi5-production-timer-preemption: PASS.
- The pre-run snapshot restore completed and hardwareTestLock was released with
  restored=true.

## Disposition

- fixed: direct-cursor TFTP proof quality is accepted for known-good fetch
  visibility after the cursor repair; the earlier blank-cursor caveat is not
  carried forward.
- fixed: evidence map reconciles lineage, hardware, restore, validation, and
  commit records.
- deferred: valid known-good Talos runtime readiness remains unaccepted because
  serial readiness markers were absent after confirmed kernel fetch.
- deferred: RP1 entry-control candidate rerun, candidate fetch, Rust entry,
  entry-control reachability, mapped/read-value, unmapped/trap, and firmware
  state behavior remain blocked.
- removed: no hardware rerun, source change, boot publication, RP1 constant,
  MMIO read, GPIO work, interrupt work, DMA/cache work, storage work,
  generated-root work, networking, SSH, broader PCIe work, Milestone 11.2 work,
  or phase transition was introduced by this closeout.
- not-an-issue: the completed direct-cursor task may remain completed with
  blocker evidence rather than accepted runtime readiness; closeout is the
  acceptance boundary for the evidence classification.

## Result

The closeout accepts known-good-direct-cursor-fetch-runtime-readiness-blocked.
The direct-cursor path proves known-good fetch visibility and restore hygiene,
but it does not prove valid known-good Talos runtime readiness. The smallest
next path is supervisor-planned boot/runtime readiness repair or discriminator
for why the restored known-good tree fetches kernel_2712.img but does not reach
the accepted serial readiness markers.
