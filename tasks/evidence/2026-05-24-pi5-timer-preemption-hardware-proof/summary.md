# Pi 5 Timer-Preemption Hardware Proof Evidence

Task: phase4-pi5-timer-preemption-hardware-proof-20260524

- Archive: `target/talos-rpi5-timer-preemption-boot.tar.gz`.
- Archive sha256:
  `950763917580e17aadfacd0f4e1ba3bba9e2b6960e800285e85db83cfaaa5f07`.
- Kernel image sha256:
  `417fd5f589b851c1fc1b2b1c77d7640fedc2abad32d0573effe8bc9606e550cb`.
- Kernel image size: 103,152 bytes.
- TFTP proof: `tftp-delta.json` shows the Pi fetched `kernel_2712.img` at
  103,152 bytes from the candidate archive.
- Serial proof: `serial-observe-after-tftp.json` shows task1=3, task2=3,
  ticks=6, requests=6, handled=6, timer-preemptions=6, dispatch-switches=6,
  voluntary-yields=0, vector=5, iar=0x0000001a, intid=26, unexpected=0, and
  `rpi5-timer-preemption-smoke: PASS`.
- Safe state: pre-run snapshot
  `pre-phase4-pi5-timer-preemption-20260524T090536Z` was restored after
  capture; `post-restore-status.json` reports the restored tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

The first serial observe captured firmware output before Talos entry. The
follow-up observe after fresh TFTP evidence captured the full Talos boot and
accepted timer-preemption diagnostic output before snapshot restore.
