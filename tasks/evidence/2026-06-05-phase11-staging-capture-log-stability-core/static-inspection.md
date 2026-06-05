# Static Inspection - Phase 11 Staging/Capture Log Stability Core

Evidence level: static inspection plus lab-controller API replay.

No hardware lock, boot archive publication, power cycle, restore, serial write,
or Talos runtime/RP1 source change was performed.

## Prior Evidence Reviewed

- Intervention checklist: tasks/evidence/2026-06-05-phase11-staging-capture-intervention/checklist.md.
- Retained cursor replay: tasks/evidence/2026-06-05-phase11-staging-capture-intervention/tftp-after-proof-cursor-4088847.json.
- Entry-control proof task record: tasks/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof.md.
- Existing helper: scripts/rpi5-wait-tftp-delta.sh.

## Findings

- fixed: scripts/rpi5-wait-tftp-delta.sh previously returned on the first non-empty TFTP delta and timed out on empty deltas without proving the log view had stabilized. It now re-queries from the same cursor until cursor_end, log_size, truncated, and parsed events are unchanged for the required sample count, or until timeout.
- fixed: the helper output now carries talos_tftp_stability metadata naming stable versus timeout, stable_samples, and required_samples. Stable non-empty deltas exit 0; stable zero-event or timeout outcomes exit 1 and still retain the annotated response for blocker evidence.
- fixed: docs/src/project/lab-controller.md now requires stable TFTP classification before restore, and explains that stable zero-event evidence is meaningful only under the repeated-query rule.
- fixed: docs/src/roadmap.md and docs/src/project/phase11-rp1-pcie-map-contract.md now record the evidence-semantics repair without accepting candidate fetch, entry reachability, or RP1 behavior.
- not-an-issue: no Talos runtime code, boot archive helper, RP1 constants, GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, or Milestone 11.2 behavior needed to change for this evidence repair.
- deferred: serialized Pi 5 proof under the repaired rule remains queued for phase11-staging-capture-known-good-pi5-proof-20260605.

## Rule Demonstration

Dry-run/replay against cursor 4088847 through the lab-controller API produced tasks/evidence/2026-06-05-phase11-staging-capture-log-stability-core/tftp-cursor-4088847-stable-replay.json.

The stable replay returned 13 events after cursor 4088847, including two da591740/kernel_2712.img served events with 104136 bytes. That is exactly the late-visible known-good fetch the proof-time empty deltas missed, so the new helper would not have allowed the earlier single empty sample to stand as final no-fetch evidence.
