# Pi 5 Monotonic Tick Accounting Evidence

Task: phase4-monotonic-tick-accounting-20260524

- Archive: `target/talos-rpi5-monotonic-tick-boot.tar.gz`.
- Archive sha256:
  `8faded95158d043b0270684dc208ad1e4e4652ced6d39b19229642bcefa3c022`.
- Kernel image sha256:
  `809b0f110a34209dae762a87e1769490eadb89895b38c923e54b0c537e123890`.
- Kernel image size: 86,661 bytes.
- TFTP proof: `tftp-delta-third.json` shows the Pi fetched
  `kernel_2712.img` at 86,661 bytes from the candidate archive.
- Serial proof: `serial-observe-third.json` shows `tick-count=4 target=4`,
  `vector=5`, `iar=0x0000001a`, `intid=26`, `unexpected=0`, post-tick
  workload progress, and `rpi5-timer-irq-smoke: PASS`.
- Safe state: pre-run snapshot `pre-phase4-monotonic-tick-20260524T051820Z`
  was restored after capture; `post-final-restore-status.json` reports PoE `UP` and
  the restored tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

The first power-cycle observation captured only firmware output before timeout;
the later serialized runs captured the accepted Talos periodic tick evidence,
with the final run using the archive hash recorded above.
