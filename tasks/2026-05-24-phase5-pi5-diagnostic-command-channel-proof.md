# Phase 5 Pi 5 Diagnostic Command-Channel Proof

Task: `phase5-pi5-diagnostic-command-channel-proof-20260524`

## Goal

Prove the accepted Milestone 5.3 diagnostic command-channel shape on physical
Raspberry Pi 5 hardware over the firmware-preserved UART10 path.

## Implementation Shape

- Focused diagnostic cfg: `TALOS_RPI5_DIAGNOSTIC_COMMAND_CHANNEL_PROOF`.
- Image helper: `scripts/rpi5-diagnostic-command-channel-image.sh`.
- Input path: UART10 PL011 polling input through canonical-lite TTY line
  assembly.
- Dispatch path: `diagnostic_command::dispatch_default_diagnostic_command`.
- Output path: runtime-console0 backed by the firmware-preserved UART10 PL011.
- Hardware-proof-only capture settle:
  `DIAGNOSTIC_COMMAND_CAPTURE_SETTLE_SPINS=10000000` after each completed
  command line and before response output, so lab serial capture records the
  bounded transcript completely.

## Command Transcript

Accepted transcript:
`tasks/evidence/2026-05-24-pi5-diagnostic-command-channel-proof/serial-observe-settle-full.json`.

```text
rpi5-diagnostic-command-channel-proof: start command-count=4 backend=runtime-console0/bcm2712-uart10-pl011 input=tty-canonical-lite
rpi5-diagnostic-command-channel-proof: ready command=0
rpi5-diagnostic-command-channel-proof: line command=0 hex=68 65 6c 70
diag: ok help
diag: commands help list status
rpi5-diagnostic-command-channel-proof: dispatch command=0 status=handled responses=2
rpi5-diagnostic-command-channel-proof: ready command=1
rpi5-diagnostic-command-channel-proof: line command=1 hex=6c 69 73 74
diag: ok list
diag: commands help list status
rpi5-diagnostic-command-channel-proof: dispatch command=1 status=handled responses=2
rpi5-diagnostic-command-channel-proof: ready command=2
rpi5-diagnostic-command-channel-proof: line command=2 hex=62 6f 67 75 73
diag: error unknown-command
rpi5-diagnostic-command-channel-proof: dispatch command=2 status=unknown-command responses=1
rpi5-diagnostic-command-channel-proof: ready command=3
rpi5-diagnostic-command-channel-proof: line command=3 hex=73 74 61 74 75 73
diag: ok status
diag: version phase5.3-contract-v1
diag: runtime-console runtime-console0
diag: tty canonical-lite line-capacity 8
diag: command-count 3
diag: commands help list status
rpi5-diagnostic-command-channel-proof: dispatch command=3 status=handled responses=6
rpi5-diagnostic-command-channel-proof: PASS
```

Retained command classification:

- `help`: discovery command; handled with two bounded response lines.
- `list`: command-list command; handled with two bounded response lines.
- `bogus`: deterministic negative input; `unknown-command` with one
  bounded error response line.
- `status`: status command tied to command-channel version,
  runtime-console0, and TTY state; handled with six bounded response lines.

## Hardware Evidence

Evidence directory:
`tasks/evidence/2026-05-24-pi5-diagnostic-command-channel-proof/`.

- Accepted archive:
  `target/talos-rpi5-diagnostic-command-channel-prefixed-boot.tar.gz`.
- Archive SHA256:
  `babf8d0161fa37891319461e136f53d616d453966f63059ba479eb44afc10f66`.
- Kernel SHA256:
  `83aa4425449e79989e15a91df35902de047b1db2d9e303027f766caf91a8305b`.
- Kernel size: 96,304 bytes.
- TFTP/archive proof: `tftp-delta-settle.json` shows 10.42.1.4 was served
  `da591740/kernel_2712.img` at 96,304 bytes from the accepted candidate
  boot tree.
- Serial hardware boot/output: `serial-observe-settle-full.json` shows Talos
  reached the diagnostic and completed the full command-channel sequence with
  `PASS`.
- Hardware lock: acquired before publish/power-cycle and released after serial,
  TFTP, and restore evidence.
- Safe-state note: pre-run boot snapshot
  `pre-phase5-pi5-diag-cmd-20260524T155800Z` was restored after capture;
  `post-restore-status-settle-trap.json` reports ok=true and tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

Earlier unprefixed/prefixed capture attempts were retained as evidence because
they exposed lab boot/capture ambiguity, not a command-channel semantic change.
The accepted run uses the serial-prefixed mirror that the lab TFTP path can
serve deterministically and the bounded settle window that captures complete
response lines.

## Local Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 90 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-diagnostic-command-channel-smoke.sh` passed.
- Image/archive inspection: `scripts/rpi5-image.sh` built the normal Pi 5
  image.
- Image/archive inspection:
  `scripts/rpi5-diagnostic-command-channel-image.sh` built the focused
  diagnostic image.
- Image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-diagnostic-command-channel-prefixed-boot.tar.gz` passed.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh`,
  `git diff --check`, and `git diff --cached --check` passed.
- static inspection: `mdbook` was unavailable in the container.

Classification: accepted hardware delivery of the local diagnostic
command-channel over Pi 5 UART10. This is not descriptor-table, syscall,
userspace shell, filesystem-backed command, networking, SSH, SMP, UART
interrupt, scheduler-blocking I/O, RP1 UART0, POSIX signal, termios, or PTY
evidence.
