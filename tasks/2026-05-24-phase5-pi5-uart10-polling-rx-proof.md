# Phase 5 Pi 5 UART10 Polling RX Proof

Task: phase5-pi5-uart10-polling-rx-proof-20260524

## Goal

Run a serialized Pi 5 UART10 polling RX proof for the accepted local TTY input
path, or record a decisive bounded timeout/unavailable classification.

## Implementation Shape

- Focused diagnostic cfg: `TALOS_RPI5_UART10_POLLING_RX_DIAGNOSTIC`.
- Image helper: `scripts/rpi5-uart10-rx-diagnostic-image.sh`.
- Input backend: `target::rpi5::firmware_console()` over BCM2712 UART10 at
  `0x10_7d00_1000`.
- Runtime boundary: `runtime_console::poll_default_console_input`.
- TTY parser: `tty::run_polling_rx_diagnostic_with_limit`.

The diagnostic keeps UART selection target-owned, uses the internal
runtime-console input-result contract, and exercises the target-independent
canonical-lite line discipline. It does not add descriptor tables, syscalls,
userspace, shell input, scheduler blocking I/O, UART interrupts, RP1 UART0,
filesystem, networking, SSH, termios, signals, or PTYs.

## Local Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 84 no-std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed with
  `talos: qemu smoke PASS`.
- Image/archive inspection: `scripts/rpi5-image.sh` built the normal Pi 5
  image.
- Image/archive inspection: `scripts/rpi5-uart10-rx-diagnostic-image.sh` built
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-uart10-rx-diagnostic.img`.
- Image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-uart10-rx-boot.tar.gz` passed with archive SHA256
  `bab86eacea7868b4fd92423370c4991b11cc6f270c60b7b38ae5960336f54209`,
  kernel_size=90344, header_image_size=90344, text_offset=0, and flags=12.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

## Hardware Evidence

Evidence directory:
`tasks/evidence/2026-05-24-pi5-uart10-polling-rx-proof/`.

- Accepted archive path: `target/talos-rpi5-uart10-rx-boot.tar.gz`.
- Accepted archive SHA256:
  `bab86eacea7868b4fd92423370c4991b11cc6f270c60b7b38ae5960336f54209`.
- Accepted kernel image SHA256:
  `2a497150163f6e53ec6b5d4b33c4e44f0f3d29f6f34f4b319a9e93515ba83a6d`.
- Accepted kernel image size: 90,344 bytes.
- TFTP/archive proof: `tftp-delta-second.json` shows 10.42.1.4 was served
  `kernel_2712.img` at 90,344 bytes from the candidate boot tree.
- Serial injection: `serial-injection-request-second.json` wrote
  `61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d`; the API response reports
  15 bytes written.
- Serial hardware boot/output: `serial-observe-second.json` shows Talos reached
  `rpi5-uart10-rx-diagnostic: ready`, then reported the expected echo bytes
  `61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a`,
  `control-events=ctrl-c`, and `rpi5-uart10-rx-diagnostic: PASS`.
- Hardware lock: acquired before publish/power-cycle and released after serial
  and TFTP capture.
- Safe-state note: pre-run boot snapshot
  `pre-phase5-pi5-uart10-rx-second-20260524T140500Z` was restored after
  capture; `post-restore-status-second.json` reports ok=true and tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

The first hardware iteration is retained as negative staging evidence:
`serial-observe-first-normal-staging.json` and
`tftp-delta-first-normal-staging.json` show a normal 86,472-byte image was
served after the generated diagnostic image path was overwritten during staging.
The pre-run snapshot was restored before the accepted second run.

Classification: accepted Pi 5 UART10 polling RX proof for the bounded local
TTY diagnostic path. This is not UART interrupt, descriptor, blocking I/O,
userspace, shell, filesystem, networking, SSH, RP1 UART0, termios, signal, or
PTY evidence.
