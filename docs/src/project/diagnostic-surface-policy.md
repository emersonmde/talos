# Diagnostic Surface Policy

Talos keeps diagnostics as named validation gates only while they still protect
an accepted boundary. One-off proof flags, boot roles, scripts, and serial
markers should either become a retained gate with an owner, be promoted into
ordinary product behavior or tests, or be retired by an explicit cleanup task.

## Current Inventory

The 2026-05-25 audit inspected Rust cfg paths, boot roles, scripts, project
docs, and accepted task records. The current surface is:

- 17 QEMU scripts under scripts/qemu-*.sh.
- 7 shared Pi 5 lab infrastructure scripts: boot image/tree helpers, TFTP
  cursor/delta helpers, and archive review.
- 16 Phase 6 Pi 5 proof image or boot-tree scripts for secondary bring-up,
  SMP locks, cross-core IPI, remote wake, production secondary dispatch, and
  shared scheduler metadata.
- 24 older Pi 5 diagnostic image/tree scripts for allocator, exception,
  panic, translation-fault, timer, UART, and diagnostic-command proof paths.
- Rust cfg-gated QEMU diagnostics in src/target/qemu_virt.rs and dispatch
  routing in src/main.rs.
- Rust cfg-gated Pi 5 proof and diagnostic paths in src/target/rpi5.rs,
  src/boot/rpi5.rs, and src/diagnostics/rpi5.rs.

## Retained Gates

Retain these as named validation gates until a later checkpoint replaces them:

- scripts/qemu-smoke.sh for broad QEMU boot coverage.
- scripts/qemu-context-switch-smoke.sh and scripts/qemu-scheduler-yield-smoke.sh
  for the cooperative scheduler contract.
- scripts/qemu-timer-irq-smoke.sh and scripts/qemu-timer-preemption-smoke.sh
  for timer IRQ and preemption regressions.
- scripts/qemu-tty-rx-diagnostic.sh and
  scripts/qemu-diagnostic-command-channel-smoke.sh for Phase 5 local console,
  TTY, and diagnostic command behavior.
- scripts/qemu-smp-lock-contention-smoke.sh,
  scripts/qemu-per-core-scheduler-ownership-smoke.sh,
  scripts/qemu-cross-core-ipi-delivery-smoke.sh,
  scripts/qemu-remote-wakeup-request-smoke.sh,
  scripts/qemu-remote-wake-to-local-runnable-smoke.sh,
  scripts/qemu-production-secondary-dispatch-smoke.sh, and
  scripts/qemu-shared-scheduler-metadata-smoke.sh for accepted Phase 6.2 and
  Phase 6.3 SMP/scheduler invariants.
- scripts/rpi5-archive-review.sh, scripts/rpi5-tftp-cursor.sh,
  scripts/rpi5-wait-tftp-delta.sh, and the generic Pi 5 image/boot-tree
  helpers for hardware proof reproducibility.
- Phase 6 Pi 5 proof scripts for secondary-core bring-up, SMP lock
  cache/coherence, cross-core IPI, remote wake, production secondary dispatch,
  and shared scheduler metadata while those hardware claims are the latest
  accepted evidence for their boundary.

These gates must not be deleted by opportunistic cleanup. If a future task
replaces one, that task must name the replacement gate and preserve the accepted
task summary, classification, and artifact digests.

## Promote Or Quarantine

Some diagnostics are still useful, but should not keep growing as proof-only
surface:

- QEMU secondary-core discriminator paths are retained only as historical
  bring-up discriminators. Queue removal after the Phase 6 secondary-core and
  scheduler migration checkpoints no longer rely on them.
- Pi 5 timer, UART, and diagnostic-command proof scripts are retained as
  Phase 4/5 hardware reproducibility gates until a later console/timer
  checkpoint promotes equivalent always-on diagnostics or names replacements.
- Allocator, string, vec, realloc, exception, panic, and translation-fault
  Pi 5 diagnostic image scripts should be grouped into a bounded legacy
  runtime/exception diagnostic retirement task. Delete them only after their
  accepted task records have summary coverage and no active roadmap gate names
  the script directly.

## Retirement Rule

A diagnostic surface may be retired only when all of these are true:

- The accepted evidence is summarized outside the raw script or cfg path.
- No active validation gate, checkpoint, or queued task names the surface as a
  required command.
- The replacement is either ordinary product behavior, a smaller host/QEMU
  test, a retained hardware gate, or an explicit decision to stop testing that
  behavior in the current phase.
- The cleanup task runs the smallest relevant gates and records the removed
  script names, cfg names, and validation result.

If any condition is uncertain, keep the surface and queue a follow-up with an
owner and validation gates. The default is to retain hardware proof
reproducibility and reduce bloat through summaries, not to delete evidence or
hardware tooling during unrelated feature work.
