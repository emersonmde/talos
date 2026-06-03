# Phase 10 Pi 5 Local Cat CWD Prompt-Gated Proof Rerun

Task: phase10-pi5-local-cat-cwd-prompt-gated-proof-rerun-20260603
Status: blocked-proof-runner-concurrency-polluted-control

## Goal

Resolve the blocked Pi 5 `cat banner.txt` cwd proof by first proving an
accepted prompt-responsive `ls` cwd control image, then running the unchanged
cat-cwd candidate only after a real Talos prompt.

## Outcome

Classification:
`blocked-proof-runner-concurrency-polluted-control`.

The run is not accepted. Multiple worker-owned proof runner instances were
active at the same time after the initial local runner invocation was polled
incorrectly. They wrote to the same evidence directory and serial control proof
stream. That made the control transcript single-writer provenance invalid, so
the worker stopped the runners, terminated orphaned `serial/observe` calls,
restored the original pre-publication boot snapshot, and did not run the
cat-cwd candidate.

No Talos runtime code, archive contents, feature semantics, marker vocabulary,
or acceptance criteria were changed.

## Evidence

Evidence directory:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-prompt-gated-proof-rerun/`.

Control evidence directory:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-prompt-gated-proof-rerun/local1-ls-cwd-control/`.

Retained control identity:

- archive:
  `target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz`
- archive sha256:
  `1f986f73b793b269e5b7aa0cf34cfc4cbf3b58358b0d9b409181e762b986919e`
- kernel size: 110624 bytes

The retained serial control evidence is useful only as a polluted procedure
artifact. It must not be used as acceptance evidence because multiple runner
instances wrote commands and observe results concurrently.

Cleanup evidence directory:
`tasks/evidence/2026-06-03-pi5-local-cat-cwd-prompt-gated-proof-rerun/cleanup/`.

The original pre-publication snapshot
`pre-cat-cwd-prompt-gated-20260603T022654Z` was restored after stopping the
runner processes. Post-restore lab status shows:

- tree hash:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
- effective kernel: `kernel_2712.img`
- root and serial-prefixed `kernel_2712.img` size: 104136 bytes

Result summaries:

- `tasks/evidence/2026-06-03-pi5-local-cat-cwd-prompt-gated-proof-rerun/proof-result.txt`
- `tasks/evidence/2026-06-03-pi5-local-cat-cwd-prompt-gated-proof-rerun/classification-summary.json`

## Validation

- Image/archive inspection: `scripts/rpi5-archive-review.sh` passed for the
  selected accepted `ls` cwd control archive before publication.
- Lab-controller API: retained control snapshot, publish/status, power-cycle,
  serial observe artifacts, process-stop cleanup, and original snapshot restore
  evidence.
- Serial hardware boot/output: retained output is classified as polluted
  control evidence, not accepted proof evidence.
- Restore proof: `cleanup/status-post-original-restore.json` confirms the lab
  returned to the original pre-publication tree hash and 104136-byte kernel.
- Static inspection: `git diff --check` run after evidence/task updates.

## Next Action

Supervisor planning is required before any rerun. A valid rerun should use a
single runner instance or a simpler foreground shell procedure with one active
serial writer, a fresh evidence directory, and prompt-gated writes. Preserve
this record as a procedure failure; do not change cat-cwd runtime semantics or
acceptance criteria based on it.
