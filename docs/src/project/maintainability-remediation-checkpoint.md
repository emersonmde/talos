# Maintainability Remediation Checkpoint

Date: 2026-05-24

Status: accepted as this checkpoint task

This checkpoint closes the senior-review remediation sequence that paused
Phase 4 timer work after the QEMU EL2 timer IRQ smoke. No runtime feature,
normal Pi 5 boot behavior, GIC/timer behavior, allocator policy, MMU/cache
programming, boot archive, hardware lock, or hardware run changed in this
checkpoint.

## Finding Map

| Finding | Disposition | Evidence |
| --- | --- | --- |
| Validation hygiene drift: cargo fmt --check had drifted and broad timer/GIC dead_code allowances could hide new Phase 4 warning noise. | Resolved. Formatting was restored and broad module-level allowances were removed from generic_timer and gicv2; no replacement allow was needed. | 45e9e1a and the validation-hygiene task evidence: fmt, no-std tests, QEMU smoke, Pi 5 image, format guard, clean warning rebuild, and diff check. |
| Historical Pi 5 assembly proofs and stale archive modes still looked runnable. | Resolved. boot.S now contains only the supported normal entry path, stale src/arch/aarch64/rpi5_*.S proofs were deleted, and archive review accepts only current loader modes. | 964be83, tasks/2026-05-24-maintainability-delete-stale-pi5-probes.md, and the source/script inventory below. |
| src/boot/rpi5.rs had one deeply nested Pi 5 boot orchestration path. | Resolved. kernel_main is now an ordered phase list with named helpers and explicit unavailable-report paths. | 6169369 and tasks/2026-05-24-maintainability-flatten-pi5-boot-pipeline.md. |
| src/main.rs still owned cross-module FDT and target tests with large inline fixtures. | Resolved. Tests moved to the owning device_tree and target modules; main.rs keeps only crate-level smoke/test-runner ownership. | aee54d2 and tasks/2026-05-24-maintainability-move-tests-to-owning-modules.md. |
| Feature work could resume without a closeout checkpoint. | Resolved by this document and supervisor-state update. The next ready Phase 4 task depends on this checkpoint before Pi 5 hardware timer work resumes. | This checkpoint commit and durable supervisor state. |

## Final Source Inventory

src/main.rs, src/boot/rpi5.rs, and src/arch/aarch64/boot.S no longer violate
the review findings:

- src/main.rs is 265 lines and owns top-level entry, panic/OOM handling, QEMU
  smoke entry, and the crate-level smoke test.
- src/boot/rpi5.rs is 532 lines and owns the Pi 5 boot phase pipeline through
  named helpers rather than a nested orchestration body.
- src/arch/aarch64/boot.S is 51 lines and owns only the normal arm64 Image
  header, x0 preservation, CPACR enable, BSS clear, stack setup, and rust_entry
  handoff.

Repo-wide source and script inspection after remediation found no stale
standalone src/arch/aarch64/rpi5_*.S proof files. The only remaining
TALOS_RPI5_* assembly conditionals are the retained exception-report and
exception-return paths in vectors.S, both advertised by build.rs and wrapper
scripts. Historical proof names such as asm-uart-proof,
asm-entry-reset-proof, cargo-asm-uart-proof, and transition-diagnostic remain
only in task records, reference notes, and decision history; they are no longer
accepted by active source or scripts.

## Retained Diagnostics

The retained Pi 5 diagnostics are current gates or narrow regression probes:

- normal Pi 5 image, boot-tree/image, format guard, archive review, TFTP cursor,
  and TFTP wait helpers;
- allocator and alloc-crate diagnostics: alloc OOM, realloc growth, Vec growth,
  String growth, alloc-format, page-frame reuse, and heap-expansion policy;
- exception/fault diagnostics: exception report, normal exception report,
  exception return, undefined instruction, data abort, current SP0 sync, and
  translation fault;
- panic diagnostics: panic report, full panic info, and nested panic.

No follow-up maintainability task is required before returning to the queued
Phase 4 Pi 5 timer smoke. Remaining architecture deferrals are the existing
Phase 4 non-goals: UART interrupts, SMP, lower ELs, DMA/RP1 routing, scheduler
policy, and userspace timer access.

## Final Validation

Validation level for this checkpoint is static documentation/source inspection,
fmt/lint/typecheck, no-std unit tests, QEMU substitute, image/archive
inspection, retained diagnostic image inspection, whitespace inspection, and
mdBook availability inspection. No Pi 5 hardware run was required because the
checkpoint changed docs/state only and cites already accepted behavior-preserving
remediation commits.

Final gate output:

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test: passed 51 no-std tests.
- scripts/qemu-smoke.sh: passed with talos: qemu smoke PASS.
- scripts/rpi5-image.sh: produced
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img.
- scripts/rpi5-format-guard-check.sh: passed with
  Pi 5 formatted early-console build PASS.
- Representative retained diagnostic image builds passed for panic report,
  normal exception report, translation fault, alloc OOM, page-frame reuse, and
  heap expansion policy.
- git diff --check: passed.
- mdbook build: not run because mdbook is unavailable in the container.

## Resume Decision

Phase 4 feature work may resume at the queued Pi 5 GIC-400 EL2 timer smoke
after this checkpoint is committed and supervisor state records the checkpoint
commit. The worker must still acquire hardwareTestLock before staging or
running any Pi 5 hardware candidate.
