# Phase 11 RP1 Diagnostic Entry-Control Pi 5 Proof

Task: phase11-rp1-diagnostic-entry-control-pi5-proof-20260605
Status: completed with blocker

## Goal

Run the serialized Pi 5 proof for the entry-control discriminator and classify
whether the same scenario/archive plumbing reaches Talos before any RP1 MMIO.

## Work Performed

- Acquired hardwareTestLock and snapshotted the restored accepted boot tree as
  phase11-rp1-entry-control-proof-pre-20260605T1632Z.
- Published only the accepted entry-control candidate archive
  target/talos-rpi5-rp1-entry-control-source-core.tar.gz.
- Captured candidate identity, lab status, fresh serial cursor, fresh TFTP
  cursor, serial output, and pre-restore TFTP deltas.
- Applied the mandatory inconclusive-run triage: candidate identity, fresh
  serial cursor, TFTP delta, known-good control, then candidate rerun.
- Restored the pre-run accepted boot tree; hardwareTestLock release is recorded
  in durable supervisor state for this run.

## Findings

- fixed: candidate identity now records the archive SHA-256
  dcbcf06ebdf2304630dc52d0aac689c6ec363f04074a055bc391a0c7829e5f37, kernel
  SHA-256 b3e62b950cf007a0ee8d1d7f420fd8c26c28573c5b6925a7f0d93d0b77a367ea,
  and the 51,808-byte da591740/kernel_2712.img artifact selected for proof.
- not-an-issue: publication staged candidate tree
  ab88a3d8549837459c8cebf8cb22580b52b39665421b7eb6d6773ebce8c6f9c2 with
  effective_kernel=kernel_2712.img and restored the pre-run tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 afterward.
- deferred: all three power-cycle attempts reached visible Raspberry Pi
  firmware serial output through Boot mode: NETWORK, but fresh TFTP deltas were
  empty for the first candidate run, known-good control, and candidate rerun.
- deferred: because the known-good control also produced no fresh TFTP events
  and did not reach TALOS: kernel_main or PASS in this run, the proof is a
  staging-or-capture blocker rather than handoff reachability evidence.
- not-an-issue: no serial output claimed RP1 mapped/read-value, unmapped, trap,
  firmware-state behavior, or entry-control-reached.

## Evidence

- Candidate identity and summary:
  tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/candidate-identity.txt
  and
  tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/proof-summary.txt.
- Pre-run lab state and snapshot: lab-status-before.json,
  boot-files-before.json, boot-snapshots-before.json, serial-peek-before.json,
  tftp-cursor-before.json, pre-run-snapshot-name.txt, and pre-run-snapshot.json.
- First candidate run: candidate-publish.json, lab-status-after-publish.json,
  candidate-serial-cursor.json, candidate-tftp-cursor.json,
  candidate-power-cycle.json, candidate-serial-observe.json,
  candidate-serial-observe-followup.json, lab-status-candidate-after-observe.json,
  and candidate-tftp-delta-followup-pre-restore.json.
- Known-good control: known-good-restore-before-control.json,
  known-good-status-after-restore.json, known-good-serial-cursor.json,
  known-good-tftp-cursor.json, known-good-power-cycle.json,
  known-good-serial-observe.json, known-good-serial-observe-followup.json, and
  known-good-tftp-delta-followup.json.
- Candidate rerun: candidate-rerun-publish.json,
  candidate-rerun-status-after-publish.json, candidate-rerun-serial-cursor.json,
  candidate-rerun-tftp-cursor.json, candidate-rerun-power-cycle.json,
  candidate-rerun-serial-observe.json,
  candidate-rerun-serial-observe-final-pre-restore.json,
  candidate-rerun-status-pre-restore.json, and
  candidate-rerun-tftp-delta-followup-pre-restore.json.
- Restore evidence: final-restore.json, lab-status-after-restore.json, and
  serial-peek-after-restore.json.

## Validation

- lab-controller API: before publish, the restored tree was
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective_kernel=kernel_2712.img; after final restore, the same tree hash and
  effective kernel were reported.
- serial hardware boot/output: first candidate run, known-good control, and
  candidate rerun all retained Raspberry Pi firmware serial output through
  Boot mode: NETWORK.
- TFTP hardware evidence: fresh TFTP deltas after all three power-cycle
  attempts were empty, so no candidate fetch, known-good fetch, or handoff claim
  is accepted from this run.
- diff hygiene: git diff --check passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed with the existing
  large search-index warning only.

## Result

Completed with blocker: staging-or-capture-blocker.

This run does not prove the candidate kernel was fetched and does not prove
entry-control reachability. It also does not accept RP1 mapped/unmapped
behavior. The lab boot tree was restored before completion, and the next task
must remain supervisor-planned rather than changing RP1 constants, GPIO,
interrupts, DMA/cache, networking, SSH, storage, generated-root behavior, or
Milestone 11.2 scope from this inconclusive hardware evidence.
