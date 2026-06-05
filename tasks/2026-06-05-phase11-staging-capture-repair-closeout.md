# Phase 11 Staging/Capture Repair Closeout

Task id: phase11-staging-capture-repair-closeout-20260605

Status: accepted

## Goal

Close out the staging/capture repair and decide whether the entry-control candidate proof can be rerun under accepted evidence rules.

## Scope

- Reconciled the accepted TFTP log-stability rule with the known-good Pi 5 blocker evidence.
- Confirmed hardware lock and restore state from durable task records and supervisor state.
- Updated Phase 11 status docs to distinguish accepted evidence semantics from unresolved lab/staging blockers.
- Prepared supervisor-facing next action without publishing a boot archive, acquiring hardware, or changing RP1 source/runtime behavior.

## Non-Goals Honored

No runtime behavior changes, RP1 source changes, boot archive publication, hardware run, hardwareTestLock acquisition, GPIO ownership, interrupts, DMA/cache policy, storage, generated-root work, networking, SSH, broader PCIe, or Milestone 11.2 work was performed.

## Findings And Disposition

- fixed: the closeout now explicitly distinguishes the accepted stable-log evidence rule from the unresolved staging/capture blocker.
- fixed: the evidence map ties the replay repair, known-good hardware blocker, serial/TFTP/status/restore paths, and commits together.
- fixed: roadmap and Phase 11 map-contract docs now state that RP1 candidate reruns remain blocked pending supervisor planning.
- deferred: the known-good proof did not show a stable known-good `kernel_2712.img` fetch or Talos serial readiness under the repaired rule.
- deferred: the deployed lab API `GET /` versus `GET /status` endpoint discrepancy remains a lab-controller semantics issue outside this closeout.
- removed: no workaround waits, alternate capture paths, boot archive publication, or RP1 diagnostic/source changes were added.
- not-an-issue: the stable-log helper remains useful as required proof-record semantics even though the known-good control did not validate the staging path.

## Evidence Map

- Stable-log repair task: `tasks/2026-06-05-phase11-staging-capture-log-stability-core.md`.
- Stable-log replay evidence: `tasks/evidence/2026-06-05-phase11-staging-capture-log-stability-core/tftp-cursor-4088847-stable-replay.json`.
- Stable-log repair commit: `e5f41c7c8af02395fb558edd9b4d3856382d4e39`.
- Known-good proof task: `tasks/2026-06-05-phase11-staging-capture-known-good-pi5-proof.md`.
- Known-good proof summary: `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/proof-summary.json`.
- Attempt 1 stable TFTP evidence: `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/known-good-tftp-delta-stable-pre-restore.json`.
- Attempt 2 stable TFTP evidence: `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/attempt2-known-good-tftp-delta-stable-pre-restore.json`.
- Boot identity evidence: `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/lab-status-before.json` and `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/attempt2-lab-status-before.json`.
- Restore evidence: `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/final-restore-attempt2.json`.
- Known-good proof blocker commit: `2ed74f813888c482042750cd62ea5c42b778145c`.
- Closeout static inspection: `tasks/evidence/2026-06-05-phase11-staging-capture-repair-closeout/static-evidence-inspection.md`.

## Validation

- static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted closeout with blocker classification.

Future Pi 5 proof records must use the repaired stable TFTP rule as evidence semantics, but the known-good control did not validate the staging/capture path. RP1 candidate reruns, diagnostic/source changes, Milestone 11.2, networking, SSH, GPIO, interrupts, DMA/cache, storage, generated-root, and broader PCIe remain blocked until supervisor planning defines a bounded lab-controller/capture or staging-publication discriminator.
