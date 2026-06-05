# Phase 11 Staging/Capture Known-Good Pi 5 Proof

Task id: phase11-staging-capture-known-good-pi5-proof-20260605

Status: completed with blocker

## Goal

Validate the repaired staging/capture proof rule on a serialized Pi 5
known-good control before reusing it for an RP1 candidate.

## Scope

- Acquired hardwareTestLock for the known-good control only.
- Used the restored accepted boot tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  with `effective_kernel=kernel_2712.img`.
- Captured fresh serial and TFTP cursors before each power cycle.
- Queried TFTP through the accepted stable-log rule before restore.
- Restored the pre-run boot tree snapshot and released the hardware lock.

## Non-Goals Honored

No RP1 candidate publication, RP1 MMIO read, source-level handoff change, GPIO
ownership, interrupts, DMA/cache policy, storage, generated-root work,
networking, SSH, broader PCIe, Milestone 11.2 work, or workaround stack was
performed.

## Findings And Disposition

- fixed: hardware lock, pre-run snapshot, fresh serial cursor, fresh TFTP
  cursor, stable pre-restore TFTP query, serial output, and restore evidence
  were retained for both attempts.
- deferred: the deployed lab API returned `404 unknown endpoint: GET /`; the
  proof used `GET /status` for boot identity because that is the documented
  endpoint in this repo.
- deferred: attempt 1 reached only early Raspberry Pi firmware serial output,
  and the stable TFTP delta from cursor `4091549` had zero events.
- deferred: attempt 2 reached Raspberry Pi firmware/RP1 boot output through
  `RP1 FW: load 0`, and the stable TFTP delta from cursor `4092900` had zero
  events.
- not-an-issue: both attempts restored the boot tree to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- removed: no candidate archive was published and no source/runtime changes
  were made during the proof.

## Evidence

- Summary: `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/proof-summary.json`.
- Pre-run identity: `lab-status-before.json`, `boot-snapshots-before.json`,
  and `pre-run-snapshot.json`.
- Attempt 1: `known-good-serial-cursor.txt`, `tftp-cursor-before.txt`,
  `known-good-power-cycle.json`, `known-good-serial-observe.json`,
  `known-good-serial-observe-followup.json`, and
  `known-good-tftp-delta-stable-pre-restore.json`.
- Attempt 2: `attempt2-known-good-serial-cursor.txt`,
  `attempt2-tftp-cursor-before.txt`, `attempt2-known-good-power-cycle.json`,
  `attempt2-known-good-serial-observe.json`,
  `attempt2-known-good-serial-observe-followup.json`, and
  `attempt2-known-good-tftp-delta-stable-pre-restore.json`.
- Restore: `final-restore.json`, `final-restore-attempt2.json`,
  `lab-status-after-restore.json`, and
  `attempt2-lab-status-after-restore.json`.

## Validation

- lab-controller API: `GET /status` before and after restore showed boot tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` and
  `effective_kernel=kernel_2712.img`.
- lab-controller API: `GET /` returned `404 unknown endpoint: GET /`; this
  is retained as endpoint-semantic evidence, not as boot identity evidence.
- serial hardware boot/output: two power cycles produced Raspberry Pi firmware
  serial output, but did not reach `TALOS: kernel_main`, command-loop
  readiness, or PASS.
- TFTP hardware evidence: both attempts returned stable zero-event deltas under
  `scripts/rpi5-wait-tftp-delta.sh <cursor> 90/120 3`.
- restore evidence: the pre-run snapshot was restored after each attempt.

## Result

Completed with blocker: `staging-capture-still-blocked`.

The repaired stable-log rule did not validate a known-good control fetch in two
serialized attempts. This proof therefore does not unblock reuse of the rule
for RP1 candidate reruns and does not accept candidate fetch, Rust entry,
entry-control reachability, RP1 mapped/read-value, RP1 unmapped/trap, GPIO,
interrupts, DMA/cache, storage, generated-root work, networking, SSH, broader
PCIe, or Milestone 11.2 behavior.
