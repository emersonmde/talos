# Talos Console, TTY, Command, and Stdio Review

Task: talos-review-console-tty-command-stdio-20260603
Status: accepted

## Scope

Reviewed the PL011 runtime-console backend, runtime console read/write
contracts, TTY canonical-lite line discipline, diagnostic command channel, local
command loop, descriptor-backed stdio bridge, and command-loop proof banners.

## Findings

- Fixed: TTY control events beyond CONTROL_EVENT_CAPACITY were silently dropped.
  A line with enough allowed Ctrl-U bytes could hide a later unsupported control
  byte and still dispatch a command as clean input. The line discipline now
  records control-history truncation, exposes it through the polling result, and
  the local command loop treats it as an input error.
- Removed: DescriptorBackedLocalCommandSink was an unused output-only bridge
  left behind after the accepted descriptor-backed input/output command loop
  moved to DescriptorBackedLocalCommandIo. It had no remaining source or target
  call sites and had shown up as stale dead code in earlier Pi 5 task notes, so
  it was removed.
- Fixed: The local command-loop status and QEMU/RPi5 proof banners still
  described the builtins only as kernel-backed. They are retained as regression
  and control surfaces during the architecture-quality campaign, not accepted
  POSIX/userspace shell progress. The status and proof banners now print
  kernel-backed-regression-control.
- Not an issue: The diagnostic command parser remains intentionally separate
  from the local command loop. It is a bounded Phase 5 diagnostic channel with a
  smaller command vocabulary, not a shell parser.
- Not an issue: PL011 still implements polling byte I/O rather than UART
  interrupt ownership. UART interrupts, DMA/cache-driver policy, and RP1/PCIe
  plumbing remain outside this review and current accepted frontier.
- Deferred: The remaining local help/status/pwd/ls/cat/cd/echo builtins are
  still kernel-backed fixtures. This review demoted their boundary labeling but
  did not remove them because accepted QEMU/Pi 5 evidence uses them as
  regression/control surfaces until descriptor-backed VFS/open/read and
  userspace launch replace the shell-visible path.

## Changes

- src/tty.rs tracks and exposes control-history truncation, with no_std
  regression coverage for overflowing the bounded control-event buffer.
- src/local_command_loop.rs rejects completed lines whose TTY control history
  was truncated, adds regression coverage for the hidden-control case, removes
  the unused DescriptorBackedLocalCommandSink bridge, and labels retained
  builtins as kernel-backed-regression-control.
- src/target/qemu_virt.rs and src/target/rpi5.rs route command-loop proof
  banners through the same builtin-boundary constant.
- Retained QEMU/substitute evidence was refreshed at
  tasks/evidence/2026-05-31-qemu-local-serial-command-loop-core/qemu-local-serial-command-loop-smoke.log.

No new feature surface, POSIX syscall, userspace shell execution, hardware
claim, networking, RP1/PCIe, UART interrupt ownership, or DMA/cache policy was
added.

## Validation

- Static inspection: reviewed src/pl011.rs, src/runtime_console.rs, src/tty.rs,
  src/diagnostic_command.rs, src/local_command_loop.rs, local command target
  harnesses, and cfg/source references with rg/sed.
- Dead-code inspection: rg showed no remaining DescriptorBackedLocalCommandSink
  references after removal.
- fmt: cargo fmt --all passed; cargo fmt --all -- --check passed.
- default target check: cargo -Zjson-target-spec check --quiet passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed, including the new
  TTY control-history truncation and local command rejection regressions.
- QEMU local command target check: TALOS_BOOT_SCENARIO=qemu_local_serial_command_loop
  cargo -Zjson-target-spec check --quiet passed.
- RPi5 local command target check: TALOS_BOOT_SCENARIO=rpi5_local_serial_command_loop
  cargo -Zjson-target-spec check --quiet passed.
- QEMU/substitute smoke: ./scripts/qemu-local-serial-command-loop-smoke.sh
  passed and refreshed the retained transcript with
  builtins=kernel-backed-regression-control and qemu-local-serial-command-loop:
  PASS.
- docs validation: /home/node/.cargo/bin/mdbook build passed after adding this
  task record.
- diff hygiene: git diff --check and git diff --cached --check passed before
  commit.
- hardwareTestLock remained unlocked/restored and unused; no hardware run was
  performed.

## Remaining Risks

- The command loop still contains kernel-backed prompt-local fixtures. They are
  deliberately retained only as regression/control surfaces until the Phase 8
  VFS/open/read, loader-from-VFS, and userspace-launch chain is accepted.
- ConsoleBackend still models sink failure as all-or-error because current
  runtime backends do not expose partial write counts. A future byte-oriented
  userspace stdout path should revisit partial-write accounting when it connects
  real regular files, pipes, or devices.

Accepted commit: recorded in durable state for
talos-review-console-tty-command-stdio-20260603.
