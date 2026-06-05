# Phase 11 RP1 Diagnostic Entry Pi 5 Proof

Task: `phase11-rp1-diagnostic-entry-pi5-proof-20260605`
Status: completed with blocker

## Goal

Run the serialized Pi 5 proof for the revised RP1 UART0 FR diagnostic candidate and retain decisive entry/handoff evidence. This task does not change source code or claim RP1 mapping acceptance unless the serial output reaches the diagnostic classification path.

## Work Performed

- Acquired `hardwareTestLock` and snapshotted the restored accepted boot tree as `phase11-rp1-entry-proof-pre-20260605T1452Z`.
- Published only the accepted revised candidate archive `target/talos-rpi5-rp1-uart0-fr-read-preentry-handoff-source-core.tar.gz`.
- Captured candidate identity, lab status, fresh serial cursor, fresh TFTP cursor, serial output, and pre-restore TFTP delta.
- Applied the mandatory inconclusive-run triage: candidate identity, fresh serial cursor, TFTP delta, known-good control, then candidate rerun.
- Restored the pre-run accepted boot tree and released `hardwareTestLock`.

## Findings

- fixed: evidence capture used the pre-restore TFTP delta before restoring the boot tree, so served file sizes remain tied to the candidate tree.
- not-an-issue: the known-good control restored tree `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, fetched the 104,136-byte control kernel, reached `TALOS: kernel_main`, and retained accepted PASS output.
- deferred: the revised candidate tree `0b25c8e08b7cdbac0447ee80a962ed7ee0fa9d219eafc3f060cfcd902c035511` fetched the selected 87,480-byte `da591740/kernel_2712.img` twice during the rerun, but serial did not reach `TALOS: kernel_main`, `rpi5-rp1-uart0-fr-read: start`, `rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or `PASS`.
- not-an-issue: post-run restore returned the lab boot tree to `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

## Evidence

- Candidate identity and summary: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/candidate-identity.txt` and `proof-summary.txt`.
- Pre-run lab state and snapshot: `lab-status-before.json`, `pre-run-snapshot.json`, and `boot-files-before.json`.
- First candidate run: `candidate-publish.json`, `candidate-serial-cursor.json`, `candidate-tftp-cursor.json`, `candidate-power-cycle.json`, `candidate-serial-observe.json`, `candidate-serial-observe-followup.json`, and `candidate-tftp-delta-followup-pre-restore.json`.
- Known-good control: `known-good-restore-before-control.json`, `known-good-status-after-restore.json`, `known-good-serial-observe-followup.json`, and `known-good-tftp-delta-followup.json`.
- Candidate rerun: `candidate-rerun-publish.json`, `candidate-rerun-status-after-publish.json`, `candidate-rerun-serial-cursor.json`, `candidate-rerun-tftp-cursor.json`, `candidate-rerun-power-cycle.json`, `candidate-rerun-serial-observe-final-pre-restore.json`, and `candidate-rerun-tftp-delta-followup-pre-restore.json`.
- Restore evidence: `final-restore.json`, `lab-status-after-restore.json`, and `serial-peek-after-restore.json`.

## Validation

- lab-controller API: `GET /status` before publish reported restored tree `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`; `GET /status` after final restore reported the same tree and `effective_kernel=kernel_2712.img`.
- serial hardware boot/output: known-good control reached `TALOS: kernel_main` and PASS output after restore.
- TFTP hardware evidence: candidate rerun served `da591740/kernel_2712.img` with size 87,480 twice before restore.
- serial hardware boot/output: candidate rerun retained firmware/RP1 boot output through `Boot mode: NETWORK (02) order f1`, but no Talos pre-MMIO or diagnostic output.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed with the existing large search-index warning.

## Result

Completed with blocker: `blocked-pre-entry-or-handoff-after-candidate-fetch`.

The revised candidate was fetched from TFTP, but the Pi did not emit Talos entry, pre-MMIO, diagnostic classification, or PASS output. Because the known-good control still reaches accepted Talos output on the restored tree, this proof does not accept RP1 mapped/unmapped behavior and should feed a supervisor-planned source/handoff investigation rather than RP1 constant churn or broader Phase 11 work.
