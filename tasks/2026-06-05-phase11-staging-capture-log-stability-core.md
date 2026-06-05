# Phase 11 Staging/Capture Log Stability Core

Task id: phase11-staging-capture-log-stability-core-20260605

Status: accepted

## Goal

Repair the Pi 5 proof-rule boundary for TFTP log stability and restore timing before any further RP1 diagnostic or source-level handoff work.

## Scope

- Updated the reusable scripts/rpi5-wait-tftp-delta.sh helper so it classifies a TFTP delta only after repeated /tftp/logs queries from the same cursor are stable.
- Defined the stable-log condition as unchanged cursor_end, log_size, truncated, and parsed event set for the required sample count.
- Documented that zero-event TFTP evidence is meaningful only after the stable-log rule and before restore.
- Validated the rule against retained cursor 4088847 replay without hardwareTestLock acquisition, boot archive publication, power cycle, or restore.

## Non-Goals Honored

No Talos runtime behavior, RP1 diagnostic, source-level handoff, MMIO constant, GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe, boot archive publication, hardwareTestLock acquisition, or Milestone 11.2 work was performed.

## Findings And Disposition

- fixed: the previous helper could accept first non-empty events and could not distinguish a stable zero-event delta from a premature empty sample.
- fixed: future proof records now get annotated stability metadata from the helper.
- fixed: lab-controller documentation now requires stable TFTP classification before restore.
- fixed: Phase 11 roadmap and map-contract proof status now record the capture-rule repair without overstating RP1 behavior.
- not-an-issue: the late-visible cursor 4088847 replay points to proof/capture semantics first, not to RP1 source behavior.
- deferred: known-good Pi 5 validation under the repaired rule remains a separate serialized hardware task.

## Evidence

- static inspection: tasks/evidence/2026-06-05-phase11-staging-capture-log-stability-core/static-inspection.md.
- lab-controller API replay: tasks/evidence/2026-06-05-phase11-staging-capture-log-stability-core/tftp-cursor-4088847-stable-replay.json.
- script syntax: sh -n scripts/rpi5-wait-tftp-delta.sh.
- replay summary: cursor_start=4088847, cursor_end=4090198, stable=true, stable_samples=2, required_samples=2, event_count=13, kernel_2712.img bytes=104136.

## Validation

- static inspection of prior proof evidence and intervention checklist: passed.
- replay/dry-run against retained cursor 4088847 through lab-controller API: passed.
- relevant script validation: sh -n scripts/rpi5-wait-tftp-delta.sh passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted evidence-semantics repair only. Future Pi 5 proofs must classify TFTP/log stability before restore; any new waits or alternate capture paths remain quarantined unless a closeout accepts their semantics. This task does not accept candidate fetch, Rust entry, entry-control reachability, RP1 mapped/read-value, RP1 trap/unmapped, GPIO ownership, interrupts, DMA/cache, storage, generated-root work, networking, SSH, broader PCIe, or Milestone 11.2 behavior.
