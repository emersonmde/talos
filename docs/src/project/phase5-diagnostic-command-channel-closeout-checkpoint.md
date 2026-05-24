# Phase 5 Diagnostic Command Channel Closeout Checkpoint

Status: accepted as the Milestone 5.3 local diagnostic command-channel closeout.

This checkpoint reconciles the accepted Milestone 5.3 evidence before Talos
starts any descriptor table, syscall, userspace shell, filesystem, networking,
SSH, SMP, UART interrupt, scheduler blocking I/O, RP1 UART0, or later phase
work.

## Accepted Work

- Source inventory: commit `e038fd5`; task record
  `tasks/2026-05-24-phase5-local-diagnostic-command-channel-source-inventory.md`.
- Command-channel contract: commit `2fed739`; architecture document
  `docs/src/architecture/diagnostic-command-channel.md`; task record
  `tasks/2026-05-24-phase5-diagnostic-command-channel-contract.md`.
- QEMU command-channel smoke: commit `6dc9165`; task record
  `tasks/2026-05-24-phase5-qemu-diagnostic-command-channel-smoke.md`;
  transcript captured at `target/qemu-diagnostic-command-channel-smoke.log`
  for the accepted run.
- Pi 5 command-channel proof: commit `7c8598c`; task record
  `tasks/2026-05-24-phase5-pi5-diagnostic-command-channel-proof.md`;
  evidence directory
  `tasks/evidence/2026-05-24-pi5-diagnostic-command-channel-proof/`.

## Accepted Boundary

The local diagnostic command channel is kernel-owned and diagnostic-only. It
consumes complete TTY lines assembled by the accepted canonical-lite line
discipline, dispatches through `src/diagnostic_command.rs`, and writes bounded
`diag:` response lines through runtime-console0.

The accepted command set is deliberately small:

- `help`: retained discovery command with deterministic bounded response text.
- `list`: retained command-list command with deterministic bounded response
  text.
- `status`: retained status command reporting command-channel version,
  runtime-console0 identity, TTY mode and capacity, and command count.
- `bogus`: retained negative regression input proving deterministic
  `unknown-command` classification.

Parser and dispatcher error labels remain internal diagnostic labels. They are
not POSIX errno, shell exit status, descriptor readiness, syscall ABI,
filesystem behavior, networking behavior, or SSH behavior.

## Evidence Reconciliation

QEMU evidence proves the command channel over the QEMU virt PL011 polling TTY
path. The accepted smoke injected `help`, `list`, `bogus`, and `status`
after ready prompts, observed canonical-lite line assembly, and captured the
expected deterministic responses and `PASS`.

Pi 5 evidence proves the same command sequence over the firmware-preserved
UART10 polling TTY path. The accepted hardware run used archive
`target/talos-rpi5-diagnostic-command-channel-prefixed-boot.tar.gz` with
SHA256 `babf8d0161fa37891319461e136f53d616d453966f63059ba479eb44afc10f66`.
The kernel image was 96,304 bytes with SHA256
`83aa4425449e79989e15a91df35902de047b1db2d9e303027f766caf91a8305b`.
TFTP served `da591740/kernel_2712.img` to `10.42.1.4` at the same size, and
the serial transcript
`tasks/evidence/2026-05-24-pi5-diagnostic-command-channel-proof/serial-observe-settle-full.json`
recorded all four commands, deterministic responses, and
`rpi5-diagnostic-command-channel-proof: PASS`.

The hardware run held the hardware test lock for publish, power cycle, serial
injection/capture, TFTP inspection, and restore. The pre-run snapshot
`pre-phase5-pi5-diag-cmd-20260524T155800Z` was restored after capture; the
post-restore tree hash was
`6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

## Retained and Deferred Surfaces

Retained as regression gates:

- `scripts/qemu-diagnostic-command-channel-smoke.sh` for QEMU/substitute
  command-channel behavior.
- `scripts/rpi5-diagnostic-command-channel-image.sh` and the associated Pi 5
  serialized hardware proof path for future hardware regressions when a task
  explicitly requires it.
- Focused parser/dispatcher no_std tests in `src/diagnostic_command.rs`.

Retained as kernel-owned diagnostics:

- `help`, `list`, `status`, parser-error responses, and
  `unknown-command` responses.

Retired or kept out of the first command-channel interface:

- stale Pi 5 probe/proof surfaces removed during the maintainability
  remediation sequence;
- destructive fault triggers, panic commands, allocator stress commands, and
  translation-fault commands unless a later task accepts explicit safety
  criteria.

Deferred:

- descriptor tables, syscall ABI, user/kernel copy, POSIX `read`/`write`,
  readiness polling, errno mapping, and scheduler blocking I/O;
- userspace shell grammar, process execution, pipes, redirection, globbing,
  environment variables, path lookup, and scripts;
- filesystem-backed commands, networking, SSH, SMP, UART interrupts, RP1 UART0,
  DMA/cache ownership, termios, POSIX signals, sessions, and PTYs.

## Remaining Risks

The accepted command channel is still polling and diagnostic-only. It does not
prove long interactive sessions, interrupt-driven UART receive, descriptor
lifetime, syscall return conventions, user address-space copying, shell command
routing, filesystem traversal, network sessions, SSH login, SMP synchronization,
or scheduler wakeups.

## Next Recommendation

Milestone 5.3 is closed for the current local diagnostic command-channel
boundary. The supervisor should plan the next bounded slice explicitly before
any worker continues. Given the current roadmap order, the next recommended
planning target is a Phase 6.1 secondary-core bring-up source inventory and
contract task that does not start SMP implementation until accepted Phase 5
evidence and deferrals are carried forward.

## Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- fmt/lint/typecheck: `git diff --check` passed after checkpoint edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this checkpoint changes only
  Markdown documentation and durable task state.
