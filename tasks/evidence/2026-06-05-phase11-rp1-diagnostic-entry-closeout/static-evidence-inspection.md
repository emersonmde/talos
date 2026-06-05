# Static Evidence Inspection

Task: `phase11-rp1-diagnostic-entry-closeout-20260605`

Evidence level: static inspection of accepted task records and retained lab
artifacts. No hardware lock, archive publication, TFTP operation, serial
capture, power cycle, or source change was performed by this closeout.

## Source/Handoff Evidence

- `tasks/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core.md`
  is accepted at commit `2c3530064f51b92f28900a63cd7911fc29de3477`.
- The revised candidate archive is
  `target/talos-rpi5-rp1-uart0-fr-read-preentry-handoff-source-core.tar.gz`
  with SHA-256
  `2640ab9ceabee343ee1426b7137e1597687517f56d3b61f58a7ac0e7ab4b6608`.
- The root/prefixed kernel SHA-256 is
  `4500b99a4405f91176d39dc8178fcd396611e97577eb98c357927df05de6f792`,
  size `87480`, with `text_offset=0`, `header_image_size=87480`,
  `flags=12`, `magic=ARMd`, and entry/`_start` at `0x200000`.
- Source comparison disposition is accepted:
  - fixed: print `rpi5-rp1-uart0-fr-read: start` and
    `rpi5-rp1-uart0-fr-read: pre-mmio-read` before the single RP1 UART0
    FR read.
  - removed: raw assembly entry-provenance marker routing remains
    quarantined.
  - not-an-issue: standard Pi 5 boot-tree helper shape and config cleanup are
    retained.
  - deferred: only serialized Pi 5 evidence can decide whether hardware
    reaches pre-MMIO, traps/hangs on the RP1 read, returns a raw value, or
    remains blocked before visible Talos output.

## Hardware Proof Evidence

- `tasks/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof.md` is completed
  with blocker at commit `d1aff42567e95708aa6fa346a7fe39d7d0eb0632`.
- `proof-summary.txt` records
  `classification=blocked-pre-entry-or-handoff-after-candidate-fetch`.
- Published candidate tree:
  `0b25c8e08b7cdbac0447ee80a962ed7ee0fa9d219eafc3f060cfcd902c035511`.
- Pre-run/restored accepted tree:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- Candidate rerun TFTP evidence records two serves of selected
  `da591740/kernel_2712.img`, size `87480`, before restore.
- Candidate rerun serial evidence records no `TALOS: kernel_main`,
  `rpi5-rp1-uart0-fr-read: start`,
  `rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or PASS.
- Known-good control evidence records restored accepted tree output reaching
  `TALOS: kernel_main` and PASS with the 104,136-byte control kernel.
- Final restore evidence records the lab boot tree returned to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` and
  `hardwareTestLock` released/restored.

## Closeout Classification

Accepted:

- source-backed RP1/PCIe address contract exists from the prior Milestone 11.1
  contract task.
- local diagnostic candidate includes a pre-MMIO discriminator and preserves
  the one-read RP1 UART0 FR contract.
- hardware publication and TFTP fetch of the revised candidate are proven.
- known-good control confirms the lab restore and serial observation path.

Blocked:

- candidate execution did not reach Talos Rust entry or the pre-MMIO marker
  after TFTP fetch.
- RP1 mapped/read-value remains unaccepted.

Deferred:

- source-level pre-entry/handoff investigation.
- any revised diagnostic shape.
- Milestone 11.2, GPIO ownership, interrupts, DMA/cache policy, networking,
  SSH, storage, generated-root blocker work, or broader PCIe work.
