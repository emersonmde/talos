# Pi 5 Diagnostic Command-Channel Proof Evidence

Evidence level: serialized hardware boot/output plus lab serial injection and
TFTP/archive proof.

- Accepted archive:
  `target/talos-rpi5-diagnostic-command-channel-prefixed-boot.tar.gz`.
- Archive SHA256:
  `babf8d0161fa37891319461e136f53d616d453966f63059ba479eb44afc10f66`.
- Kernel SHA256:
  `83aa4425449e79989e15a91df35902de047b1db2d9e303027f766caf91a8305b`.
- Kernel size: 96,304 bytes.
- TFTP proof: `tftp-delta-settle.json` shows `da591740/kernel_2712.img`
  served to 10.42.1.4 at 96,304 bytes.
- Serial injection: `serial-write-settle-help.json`,
  `serial-write-settle-list.json`, `serial-write-settle-bogus.json`, and
  `serial-write-settle-status.json` record the four injected commands.
- Serial result: `serial-observe-settle-full.json` records help/list,
  `bogus` unknown-command handling, status responses, and
  `rpi5-diagnostic-command-channel-proof: PASS`.
- Restore: `restore-pre-snapshot-settle-trap.json` restored
  `pre-phase5-pi5-diag-cmd-20260524T155800Z`; post-restore status reports
  tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

Classification: accepted local diagnostic command-channel proof over the Pi 5
firmware-preserved UART10 path. No descriptor, syscall, userspace shell,
filesystem-backed command, networking, SSH, SMP, UART interrupt, scheduler
blocking I/O, RP1 UART0, termios, signal, or PTY behavior is claimed.
