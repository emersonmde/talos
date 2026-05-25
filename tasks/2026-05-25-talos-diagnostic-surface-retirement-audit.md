# Talos Diagnostic Surface Retirement Audit

Task ID: talos-diagnostic-surface-retirement-audit-20260525
Status: accepted

## Goal

Audit one-off diagnostic flags, scripts, cfg paths, task roles, and proof-only
code so each is retained as a named gate, promoted, quarantined, or queued for
bounded retirement.

## Inventory

- Static inventory found 17 QEMU scripts, 7 shared Pi 5 lab infrastructure
  scripts, 16 Phase 6 Pi 5 proof image/boot-tree scripts, and 24 older Pi 5
  diagnostic image/tree scripts.
- Rust cfg-gated QEMU diagnostics live mainly in src/target/qemu_virt.rs and
  dispatch from src/main.rs.
- Rust cfg-gated Pi 5 diagnostics and proof roles live mainly in
  src/target/rpi5.rs, src/boot/rpi5.rs, and src/diagnostics/rpi5.rs.
- Active Phase 6.3 gates are the shared scheduler metadata, production
  secondary dispatch, remote wake, IPI, per-core ownership, and broad QEMU
  smoke scripts plus the Pi 5 hardware proof helpers named in the shared
  scheduler metadata closeout.

## Classification

Keep validation gates:

- qemu-smoke, context-switch, scheduler-yield, timer IRQ/preemption,
  TTY/diagnostic command, SMP lock, per-core ownership, IPI, remote wake,
  production secondary dispatch, and shared scheduler metadata QEMU scripts.
- Generic Pi 5 boot/archive/TFTP helpers.
- Phase 6 Pi 5 proof scripts while their hardware claims are the newest
  accepted evidence for the corresponding boundary.

Promote or retain until replacement:

- Pi 5 timer, UART, and diagnostic-command proof scripts remain Phase 4/5
  reproducibility gates until a later checkpoint names always-on diagnostics or
  a smaller replacement.

Quarantine for follow-up:

- QEMU secondary-core discriminator paths are historical bring-up
  discriminators and should be removed only after a bounded follow-up confirms
  no active checkpoint still names them.
- Legacy Pi 5 allocator, string, vec, realloc, exception, panic, and
  translation-fault diagnostic image/tree scripts should be retired as a
  grouped legacy-runtime/exception cleanup task once accepted summaries cover
  the evidence.

Removed in this pass:

- None. The audit found cleanup candidates, but removing them safely requires
  checking accepted evidence summaries and active gate references in a bounded
  follow-up. This preserves hardware proof reproducibility.

## Documentation

Added docs/src/project/diagnostic-surface-policy.md and linked it from the
mdBook summary. The policy names retained gates, promotion/quarantine classes,
and the retirement rule for future cleanup.

## Validation

- static inspection: git status --short before edits passed; Talos repo was
  clean before the audit edits.
- static inventory: rg diagnostic/cfg/script inventory completed over src,
  scripts, docs, and tasks.
- static inventory: script count summary found 17 QEMU scripts, 7 shared Pi 5
  lab infrastructure scripts, 16 Phase 6 Pi 5 proof scripts, and 24 older Pi 5
  diagnostic image/tree scripts.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Follow-Ups

- talos-retire-qemu-secondary-core-discriminator-20260525: remove historical
  QEMU secondary-core discriminator cfg/script paths after checking no active
  checkpoint still names the discriminator as a retained gate.
- talos-retire-legacy-rpi5-runtime-exception-diagnostics-20260525: group the
  old allocator/string/vec/realloc/exception/panic/translation-fault Pi 5
  diagnostic image scripts and cfg paths behind a bounded retirement plan.

Accepted as a diagnostic surface audit. No active Phase 6.3 validation gate was
removed or weakened.
