# Phase 10 Pi 5 Local Cat CWD Single-Runner Proof Rerun

Task: phase10-pi5-local-cat-cwd-single-runner-proof-rerun-20260603
Status: blocked-control-inconclusive

## Goal

Produce a clean serialized Pi 5 proof for the accepted cwd-aware
`cat banner.txt` feature after the prior prompt-gated attempt was invalidated
by multiple worker-owned runner instances.

## Outcome

Classification: `blocked-control-inconclusive`.

The single-runner preflight passed and hardware access was serialized through
`hardwareTestLock`. The accepted prompt-responsive `ls` cwd control archive
was published and fetched over TFTP, including the expected 110624-byte
`da591740/kernel_2712.img`, but fresh serial evidence from the pre-power
cursor advanced only by two bytes and retained no Talos prompt, command
transcript, `pi5-local-ls-cwd-complete`, or
`rpi5-local-ls-cwd-proof: PASS`.

Because the control was inconclusive, the cat-cwd candidate was not run. No
Talos runtime code, archive contents, feature semantics, marker vocabulary, or
acceptance criteria were changed.

## Evidence

Evidence directory:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/`.

Preflight:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/preflight/preflight-summary.txt`.

Control evidence:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/local1-ls-cwd-control/`.

Control identity:

- archive: `target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz`
- archive sha256:
  `1f986f73b793b269e5b7aa0cf34cfc4cbf3b58358b0d9b409181e762b986919e`
- kernel sha256:
  `da6bb65ad8529912e1feca037d6f1e3cfbc46c5ea052ee32a1ab669b000bfd3e`
- kernel size: 110624 bytes

Settled same-cursor TFTP evidence:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/local1-ls-cwd-control/tftp-delta-settled-before-restore.json`.

Serial evidence:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/local1-ls-cwd-control/manual-observe-after-power.json`.

Result summaries:

- `tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/proof-result.txt`
- `tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/classification-summary.json`

Restore evidence:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-single-runner-proof-rerun/cleanup/status-post-restore.json`.

The original snapshot
`pre-cat-cwd-single-runner-20260603T035710Z` was restored. Post-restore lab
status returned to tree hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

## Validation

- Single-runner preflight: passed; no orphaned prior serial/proof process was
  recorded and the fresh evidence directory was used.
- Image/archive inspection: `scripts/rpi5-archive-review.sh` passed for both
  the accepted `ls` cwd control archive and the unchanged cat-cwd candidate
  archive before hardware acquisition.
- Lab-controller API: retained health/status, snapshot, publish, fixed-port
  power-cycle, TFTP, serial observe, restore, and post-restore artifacts.
- TFTP/lab-controller API: settled same-cursor delta retained 13 fresh events,
  including the expected 110624-byte `da591740/kernel_2712.img` control
  fetches before restore.
- Serial hardware boot/output: control was inconclusive; fresh observe from the
  pre-power cursor retained only two bytes and no Talos prompt/PASS transcript.
- Restore proof: post-restore status confirms the saved pre-run tree was
  restored, and `hardwareTestLock` was released/restored.
- Static inspection: `git diff --check` passed after evidence/task updates.

## Next Action

Supervisor planning is required before any further cat-cwd hardware rerun. Do
not change cat-cwd runtime semantics or acceptance criteria based on this
control result.
