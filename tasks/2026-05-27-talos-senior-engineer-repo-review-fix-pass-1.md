# Talos Senior Engineer Repo Review/Fix Pass 1

Task ID: talos-senior-engineer-repo-review-fix-pass-1-20260527
Status: accepted

## Goal

Review the full repository as a senior kernel engineer and fix concrete,
bounded correctness or maintainability issues before later review passes and
Phase 6.3 multi-core preemption implementation continue.

## Review Inventory

- Static inspection: git status was clean at baseline commit
  e53ec784491c02fa34fb370df8d100b3da7ae28d.
- Static review: searched code, scripts, docs, tasks, and build routing for
  TODO/FIXME/HACK/XXX, unsafe, cfg, diagnostic, dead-code, and stale removed
  diagnostic patterns.
- Static review: inspected panic handling, SMP synchronization, scheduler
  migration/load-balancing policy, retained boot-scenario routing, Pi 5 helper
  scripts, and architecture docs affected by the 2026-05-27 cleanup.

## Findings

| ID | Severity | Reference | Finding | Disposition |
| --- | --- | --- | --- | --- |
| F1 | High | src/main.rs PanicInProgress | The Pi 5 panic recursion guard used a shared UnsafeCell<bool> with volatile loads/stores and an unsafe Sync impl. That is a data race if two cores panic concurrently, and Phase 6 already has physical secondary-core execution. | Fixed by replacing it with a word-sized AtomicUsize compare_exchange guard and removing the unsafe Sync impl. |
| F2 | Medium | docs/src/architecture/memory.md; docs/src/architecture/exceptions.md; docs/src/architecture/early-serial.md; docs/src/architecture/lower-el-userspace.md | Current architecture text still described retired Pi 5 allocator, panic, exception, and translation-fault proof-only diagnostics as active wrappers/cfg surfaces after the obsolete-bloat removal sweep. | Fixed by rewriting those passages as historical evidence and naming the active retained proof surface correctly. |

## Rejected Or Deferred Items

- The shell-script static pass matched mktemp templates such as XXXXXX as
  TODO/XXX-like text. This is a false positive; no task was created.
- No concrete issue found in this pass required deferral. Larger future review
  passes remain queued separately by supervisor state.

## Fix Summary

- src/main.rs now uses a word-sized atomic panic-in-progress guard for Pi 5
  panic re-entry detection.
- Architecture docs now distinguish current retained diagnostics from retired
  historical proof-only evidence.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 147 no_std tests.
- QEMU/substitute: scripts/qemu-smoke.sh passed with qemu smoke PASS.
- image/build inspection: scripts/rpi5-image.sh built
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Result

Accepted. The repo is clean after commit and no hardware behavior is claimed by
this review/fix pass.
