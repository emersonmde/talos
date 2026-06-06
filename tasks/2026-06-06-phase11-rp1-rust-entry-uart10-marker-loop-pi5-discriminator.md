# Phase 11 RP1 Rust-Entry UART10 Marker Loop Pi 5 Discriminator

Task id: phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator-20260606

Status: accepted

## Goal

Run the accepted Rust-entry UART10 marker-loop candidate on the Pi 5 to decide
whether visible post-handoff Rust-side UART10 serial output is accepted.

## Scope

- Acquired hardwareTestLock and published only the archive accepted by
  phase11-rp1-rust-entry-uart10-marker-loop-core-20260606.
- Retained candidate identity, selected tree hash, effective kernel, expected
  `da591740/kernel_2712.img` bytes, fresh serial cursor, fresh TFTP cursor,
  stable same-cursor TFTP evidence, pre-restore state, restore proof,
  post-restore state, and lock-release evidence.
- Classified only the UART10 marker-loop discriminator boundary.
- Did not change source, run RP1 UART0 FR reads, GPIO/pinmux/clock/reset work,
  interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe, Milestone 11.2, or a phase transition.

## Candidate

- Archive: `target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz`
- Archive SHA-256:
  `ab6de452670427cee2d411cbcd2a92602331e9d03a9d68dae20b75d649d1565b`
- Candidate tree:
  `1d7cdd3d265fb983ec77d9281098d6a920e0bc957a1f0a15f279fe35c618ee6c`
- Effective kernel: `kernel_2712.img`
- Expected fetch: `da591740/kernel_2712.img`
- Expected fetch bytes: `45328`
- Marker: `TALOS: reu10-loop`
- Restore snapshot:
  `phase11-rust-entry-uart10-marker-loop-pre-20260606T1032Z`

## Findings And Disposition

- fixed: selected candidate identity matched before power cycle:
  tree `1d7cdd3d265fb983ec77d9281098d6a920e0bc957a1f0a15f279fe35c618ee6c`,
  effective kernel `kernel_2712.img`, and 45,328-byte
  `da591740/kernel_2712.img`.
- fixed: fresh serial cursor `4133556` was captured before power cycle.
- fixed: fresh TFTP cursor was captured before power cycle, and stable
  same-cursor pre-restore replay retained 13 events, including two served
  45,328-byte `da591740/kernel_2712.img` fetches.
- fixed: deadline-looped fresh serial evidence retained 60,748 bytes over 32
  seconds and observed `TALOS: reu10-loop` 2,961 times.
- fixed: restore returned the boot tree to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- fixed: hardwareTestLock was released/restored in supervisor state after
  evidence capture and commit.
- not-an-issue: known-good control and candidate rerun were not required
  because the first candidate run was definitive marker-visible with stable
  candidate fetch evidence, not capture/staging inconclusive.
- not-an-issue: deployed lab `GET /` returned HTTP 404; the documented
  `GET /status` endpoint retained candidate identity.
- deferred: RP1 UART0 FR-read readiness remains blocked until the marker-loop
  closeout reconciles this visible UART10 marker evidence.
- not-an-issue: no RP1 mapped/read-value, unmapped/trap, firmware-state, GPIO,
  interrupt, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or phase transition behavior is accepted here.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/classification.json`.
- Candidate identity: `candidate-post-publish-status.json`,
  `candidate-post-publish-boot-files.json`,
  `candidate-post-publish-tree-hash.txt`, and
  `candidate-kernel-bytes.txt`.
- Candidate run bundle: `candidate-run/pre-status.json`,
  `candidate-run/pre-boot-files.json`, `candidate-run/pre-snapshots.json`,
  `candidate-run/preflight-identity.json`,
  `candidate-run/serial-peek-before-power.json`,
  `candidate-run/serial-cursor-before-power.txt`,
  `candidate-run/tftp-cursor-before-power.json`,
  `candidate-run/tftp-cursor-before-power.txt`,
  `candidate-run/power-cycle.json`,
  `candidate-run/serial-observe-window.json`,
  `candidate-run/tftp-delta-stable-pre-restore.json`,
  `candidate-run/final-pre-restore-status.json`,
  `candidate-run/final-pre-restore-boot-files.json`,
  `candidate-run/restore-snapshot.json`,
  `candidate-run/post-restore-status.json`,
  `candidate-run/post-restore-boot-files.json`, and
  `candidate-run/capture-invariant-summary.json`.
- Validation summary:
  `tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/validation-summary.txt`.

## Validation

- lab-controller API candidate identity: passed via `GET /status` and
  `GET /boot/files`; deployed `GET /` returned HTTP 404 and is retained as
  endpoint-semantics evidence.
- fresh serial cursor: passed.
- fresh TFTP cursor and stable same-cursor TFTP sampling: passed.
- serial hardware boot/output: passed; `TALOS: reu10-loop` appeared 2,961
  times in the fresh deadline-looped serial window.
- TFTP hardware evidence: passed; two served 45,328-byte candidate kernel
  fetches were retained before restore.
- restore proof and hardware lock release: passed.
- known-good control/candidate rerun: not run because first candidate result
  was definitive, not capture/staging inconclusive.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as `post-handoff-rust-entry-uart10-marker-visible`.

This accepts only visible post-handoff Rust-entry UART10 marker observability
for the selected marker-loop candidate. It does not accept RP1 UART0 FR-read
readiness, RP1 mapped/read-value behavior, RP1 unmapped/trap behavior,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, or phase transition.
