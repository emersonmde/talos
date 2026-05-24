# Pi 5 UART10 Polling RX Proof Evidence

Evidence level: serialized hardware boot/output plus lab serial injection and
TFTP/archive proof.

- Archive: `target/talos-rpi5-uart10-rx-boot.tar.gz`.
- Archive SHA256:
  `bab86eacea7868b4fd92423370c4991b11cc6f270c60b7b38ae5960336f54209`.
- Kernel SHA256:
  `2a497150163f6e53ec6b5d4b33c4e44f0f3d29f6f34f4b319a9e93515ba83a6d`.
- Kernel size: 90,344 bytes.
- TFTP proof: `tftp-delta-second.json` shows `kernel_2712.img` served at
  90,344 bytes to 10.42.1.4.
- Serial injection: `serial-injection-request-second.json` records
  `61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d`; the serial write response
  reports 15 bytes.
- Serial result: `serial-observe-second.json` shows the kernel reached the
  diagnostic, reported echo bytes
  `61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a`,
  `control-events=ctrl-c`, and `rpi5-uart10-rx-diagnostic: PASS`.
- Restore: `restore-pre-snapshot-second.json` restored
  `pre-phase5-pi5-uart10-rx-second-20260524T140500Z`; post-restore status
  reports tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

Classification: accepted polling RX on the firmware-preserved Pi 5 UART10 path
for the bounded local TTY diagnostic. No descriptor, syscall, scheduler
blocking, UART interrupt, shell, filesystem, networking, SSH, RP1 UART0,
termios, signal, or PTY behavior is claimed.
