# Talos Obsolete Bloat Full Inventory

Task ID: talos-obsolete-bloat-full-inventory-20260527
Status: accepted

## Goal

Inventory all diagnostic, smoke, proof, gate, cfg, script, task, and
documentation surfaces that may now be obsolete before the multi-core
preemption core starts.

This is a comprehensive cleanup inventory. It is not limited to the largest
old Pi 5 diagnostics or the historical QEMU discriminator path.

## Inventory Method

- Static inspection: git status --short before edits showed a clean Talos
  worktree at 820f531.
- Static inventory: rg --files scripts, rg TALOS_BOOT_SCENARIO,
  rg talos_boot_scenario, and rg diagnostic/smoke/proof/gate/PASS/classification
  over build.rs, src/, scripts/, docs/, and tasks/ excluding raw evidence
  payloads.
- Static review: compared results against docs/src/project/diagnostic-surface-policy.md,
  the accepted talos-diagnostic-surface-retirement-audit-20260525, and
  Matthew's 2026-05-27 direction to remove obsolete bloat comprehensively.

## Surface Counts

- Scripts: 74 total.
- QEMU script surfaces: 20, including scripts/qemu-runner.sh.
- Pi 5/Raspberry Pi script surfaces: 53.
- Shared non-scenario helpers: 10: scripts/objcopy-tool.sh,
  scripts/qemu-runner.sh, scripts/rpi5-archive-review.sh,
  scripts/rpi5-boot-img.sh, scripts/rpi5-boot-ramdisk-tree.sh,
  scripts/rpi5-boot-tree.sh, scripts/rpi5-format-guard-check.sh,
  scripts/rpi5-image.sh, scripts/rpi5-tftp-cursor.sh, and
  scripts/rpi5-wait-tftp-delta.sh.
- Boot-scenario registry entries in build.rs: 49.
- Source files with diagnostic/proof scenario routing: build.rs, src/main.rs,
  src/target/qemu_virt.rs, src/target/rpi5.rs, src/boot/rpi5.rs,
  src/diagnostics/rpi5.rs, src/arch/aarch64/boot.S,
  src/arch/aarch64/vectors.S, src/arch/aarch64/exceptions.rs,
  src/runtime_console.rs, src/pl011.rs, and src/smp.rs.

## Classification Summary

- Keep active: 39 surfaces.
- Promote to real test or feature: 4 surfaces.
- Remove now: 19 surfaces.
- Defer with owner and expiry: 9 surfaces.

The counts are surface groups, not raw rg hits. One surface may include a
script, a boot-scenario value, and cfg-gated source routing.

## Keep Active

These are retained because they are current regression gates, current hardware
proof reproduction paths, or shared proof infrastructure.

| Surface | Owner | Purpose | Gate | Expiry or review trigger |
| --- | --- | --- | --- | --- |
| Shared helper scripts: objcopy-tool, qemu-runner, rpi5 image/tree helpers, TFTP cursor/delta, archive review, format guard | repo-health worker | Build, run, and review current QEMU/Pi 5 proofs | Existing QEMU/Pi 5 proof scripts and archive review | Review when a unified test runner replaces them |
| QEMU broad/timer/scheduler/console gates: qemu-smoke, context switch, scheduler yield, timer IRQ, timer preemption, TTY RX, diagnostic command channel | phase owners for Phases 4 and 5 | Preserve accepted local timer, scheduler, console, TTY, and diagnostic-command coverage | Named scripts | Review at Phase 5 production-console closeout or when promoted to ordinary tests |
| QEMU Phase 6.2 and 6.3 gates: secondary-core workload, SMP lock, per-core ownership, cross-core IPI, remote wake request, remote wake to local runnable, production secondary dispatch, secondary scheduler service loop, shared scheduler metadata, shared runqueue migration, load balancing | Phase 6 worker | Keep active scheduler/SMP invariants reproducible before multi-core preemption | Named scripts | Review at Phase 6.3 closeout after multi-core preemption proof |
| Pi 5 Phase 6 hardware proof pairs: PSCI secondary alive, secondary workload, SMP lock, cross-core IPI, remote wake request, remote wake to local runnable, production secondary dispatch, secondary scheduler service loop, shared scheduler metadata, shared runqueue migration, load balancing | Phase 6 worker | Preserve latest physical hardware claims for each accepted boundary | Named image and boot-tree scripts plus evidence summaries | Review when a later checkpoint supersedes the boundary or replaces physical proof shape |
| Current scheduler proof source routing in src/target/qemu_virt.rs, src/target/rpi5.rs, src/main.rs, src/boot/rpi5.rs, src/smp.rs, and assembly SMP entry defines | Phase 6 worker | Scenario-specific QEMU/Pi 5 proof dispatch for active SMP and scheduler boundaries | Scenario scripts above | Review at multi-core preemption closeout |

## Promote To Real Test Or Feature

These are useful behaviors but should stop growing as one-off proof-only
surfaces.

| Surface | Owner | Purpose | Current gate | Expiry or review trigger |
| --- | --- | --- | --- | --- |
| Pi 5 timer IRQ diagnostic image and rpi5_timer_irq cfg/source routing | Phase 4/5 owner | Reproduce accepted physical timer interrupt evidence | scripts/rpi5-timer-irq-diagnostic-image.sh | Promote to always-on timer diagnostics or retire after Phase 6.3 closeout |
| Pi 5 timer preemption diagnostic image and rpi5_timer_preemption cfg/source routing | Phase 4/5 owner | Reproduce accepted physical timer-preemption evidence | scripts/rpi5-timer-preemption-diagnostic-image.sh | Promote to ordinary scheduler/timer regression proof or retire after multi-core preemption supersedes it |
| Pi 5 UART10 polling RX diagnostic image and rpi5_uart10_polling_rx cfg/source routing | Phase 5 owner | Reproduce accepted UART polling input evidence | scripts/rpi5-uart10-rx-diagnostic-image.sh | Promote into console/TTY regression coverage before UART interrupt work |
| Pi 5 diagnostic command channel image and rpi5_diagnostic_command_channel cfg/source routing | Phase 5 owner | Reproduce accepted local diagnostic-command evidence | scripts/rpi5-diagnostic-command-channel-image.sh | Promote into local diagnostics regression coverage before Phase 7 or shell work |

## Remove Now

These surfaces are obsolete proof-only or bring-up-only paths. They should be
removed by the follow-up sweep after this inventory is accepted.

| Surface group | Files and cfg/scenario names |
| --- | --- |
| Historical QEMU secondary-core discriminator | scripts/qemu-secondary-core-discriminator.sh, qemu_secondary_core_discriminator, TALOS_QEMU_SMP_BOOT_SCENARIO use only for that discriminator path, run_secondary_core_discriminator, and matching dispatch/docs references that are not accepted evidence summaries |
| Old Pi 5 exception report probes | scripts/rpi5-exception-report-diagnostic-image.sh, scripts/rpi5-normal-exception-report-diagnostic-image.sh, scripts/rpi5-undefined-instruction-report-diagnostic-image.sh, scripts/rpi5-data-abort-report-diagnostic-image.sh, scripts/rpi5-current-sp0-sync-diagnostic-image.sh, plus rpi5_exception_report, rpi5_normal_exception_report, rpi5_undefined_instruction_report, rpi5_data_abort_report, and rpi5_current_sp0_sync source routing |
| Old Pi 5 exception-return and translation-fault probes | scripts/rpi5-exception-return-diagnostic-image.sh, scripts/rpi5-translation-fault-diagnostic-image.sh, scripts/rpi5-translation-fault-diagnostic-tree.sh, plus rpi5_exception_return, rpi5_translation_fault, assembly exception-return defines, and matching src/boot/rpi5.rs / src/diagnostics/rpi5.rs paths |
| Old Pi 5 panic probes | scripts/rpi5-panic-report-diagnostic-image.sh, scripts/rpi5-full-panic-info-diagnostic-image.sh, scripts/rpi5-nested-panic-diagnostic-image.sh, scripts/rpi5-nested-panic-diagnostic-tree.sh, plus rpi5_panic_report, rpi5_full_panic_info, and rpi5_nested_panic source routing |
| Old Pi 5 allocator/container probes | scripts/rpi5-alloc-oom-diagnostic-image.sh, scripts/rpi5-realloc-growth-diagnostic-image.sh, scripts/rpi5-vec-growth-diagnostic-image.sh, scripts/rpi5-string-growth-diagnostic-image.sh, scripts/rpi5-alloc-format-diagnostic-image.sh, scripts/rpi5-page-frame-reuse-diagnostic-image.sh, scripts/rpi5-heap-expansion-policy-diagnostic-image.sh, plus rpi5_alloc_oom, rpi5_realloc_growth, rpi5_vec_growth, rpi5_string_growth, rpi5_alloc_format, rpi5_page_frame_reuse, and rpi5_heap_expansion_policy source routing in src/diagnostics/rpi5.rs and src/boot/rpi5.rs |
| Remove-only references in active docs/tasks | Checklist, policy, roadmap, and decision-log references that name the remove-now scripts as retained or queued cleanup surfaces, while preserving accepted evidence summaries and artifact digests |

## Defer With Owner And Expiry

These are not remove-now because they either preserve accepted evidence or are
queued but not yet implemented.

| Surface | Owner | Reason to defer | Expiry or review trigger |
| --- | --- | --- | --- |
| tasks/evidence raw accepted evidence payloads | repo-health worker | Deletion requires external artifact storage or a manifest-only retention decision | Revisit when talos-evidence-archive-large-raw-lab-artifacts-20260525 is unblocked |
| Older accepted task records under tasks/ | repo-health worker | They are the durable summary layer for hardware claims | Keep; update only stale references during the removal sweep |
| Decision log accepted-evidence entries | repo-health worker | They preserve why/evidence and should not be deleted as bloat | Keep; update only current-frontier wording if a surface is retired |
| Roadmap/checkpoint references to old diagnostics | supervisor/worker | Some references are historical evidence, not active gates | Sweep active-gate wording during removal while preserving history |
| phase6-multicore-preemption queued proof surfaces | Phase 6 worker | They are planned but not implemented; no script/cfg exists yet | Create only when core proof task is ready |
| Generic Pi 5 boot image/tree helpers | repo-health worker | Required by current hardware proof workflow | Review only after a replacement hardware runner exists |
| QEMU/Pi 5 remote-wake-to-local-runnable implied scenarios | Phase 6 worker | They are current active gates that reuse remote-wakeup dispatch via implied cfg values | Review at Phase 6.3 closeout |
| Timer/UART/diagnostic-command Pi 5 proof scripts | Phase 4/5 owner | Promote first; deleting immediately would weaken accepted Phase 4/5 physical claims | Review before Phase 7 or UART interrupt ownership work |
| Large raw lab captures | repo-health worker | Covered by the blocked archive task, not this removal sweep | Revisit when external archive storage or explicit no-delete plan exists |

## Follow-Up Removal Groups

The removal sweep should stay bounded by these groups:

1. Delete the QEMU secondary-core discriminator script, scenario, dispatch,
   and source routing while preserving the accepted task/evidence summaries.
2. Delete old Pi 5 exception/panic/translation-fault diagnostic scripts,
   boot-scenario registry entries, assembly defines, boot routing, and
   diagnostics code.
3. Delete old Pi 5 allocator/container diagnostic scripts, boot-scenario
   registry entries, boot routing, and diagnostics code.
4. Update policy, roadmap, decision-log, and task text that currently treats
   those remove-now surfaces as retained or queued cleanup candidates.
5. Run scenario-build checks for retained active scenarios that share edited
   source files, plus git diff --check and mdbook build.

## Validation

- static inspection: git status --short before edits passed; worktree was
  clean at 820f531.
- static inventory: rg --files scripts | wc -l found 74 scripts.
- static inventory: rg --files scripts | rg '/qemu-.*\.sh$' | wc -l found
  20 QEMU scripts.
- static inventory: rg --files scripts | rg '/rpi5-|/pi5-' | wc -l found
  53 Pi 5/Raspberry Pi scripts.
- static inventory: rg -o 'value: "[^"]+"' build.rs found 49 registered boot
  scenarios.
- static inventory: rg talos_boot_scenario/TALOS_BOOT_SCENARIO/diagnostic/smoke/proof/gate/PASS/classification
  reviewed code, scripts, docs, and task records outside raw evidence.
- documentation: docs/src/project/diagnostic-surface-policy.md updated with
  the comprehensive 2026-05-27 classification.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Result

Accepted as the comprehensive obsolete-bloat inventory. No code or script was
removed in this task. The follow-up removal sweep should remove all remove-now
surfaces above before phase6-multicore-preemption-core-20260527 starts.
