# Phase 10 Pi 5 Local Ls Cwd Proof

Task: phase10-pi5-local-ls-cwd-proof-20260602
Status: accepted
Commit: recorded in talos-supervisor-state.json after the accepted proof commit is created.

## Goal

Prove the accepted bare `ls` current-directory feature on the serialized
Raspberry Pi 5 serial command loop.

## Candidate

The selected archive was the accepted RPi5 `ls` cwd candidate from
`phase10-rpi5-local-ls-cwd-candidate-archive-core-20260602`:

- archive: `target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz`
- archive sha256:
  `1f986f73b793b269e5b7aa0cf34cfc4cbf3b58358b0d9b409181e762b986919e`
- kernel sha256:
  `da6bb65ad8529912e1feca037d6f1e3cfbc46c5ea052ee32a1ab669b000bfd3e`
- kernel size: 110624 bytes

No code, archive contents, feature semantics, or acceptance criteria were
changed during this proof task.

## Run

Accepted evidence is retained under:

`tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/`.

The worker held `hardwareTestLock` for the serialized proof, captured fresh
serial cursor `4025624` and TFTP cursor `4070674`, published only the
selected candidate archive, power-cycled the fixed Pi 5 port, collected settled
same-cursor TFTP evidence before restore, then restored named snapshot
`pre-ls-cwd-20260602T2348Z` before releasing the lock.

Pre-run and post-restore boot tree hashes matched:

`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

The post-publish boot tree selected `kernel_2712.img` and exposed the selected
110624-byte candidate kernel at both `kernel_2712.img` and
`da591740/kernel_2712.img`. The settled TFTP delta retained 13 events,
including fresh `da591740/config.txt`, `da591740/kernel_2712.img`,
`da591740/bcm2712-rpi-5-b.dtb`, overlay, and `cmdline.txt` requests from
`10.42.1.4`. The candidate kernel was served twice at 110624 bytes.

## Feature Proof

The retained serial transcript proves the full accepted feature sequence:

- `pwd` prints `/`.
- Bare `ls` at `/` prints `bin`, `dir`, `empty`, and `etc`.
- `cd /etc` followed by bare `ls` prints `banner.txt`.
- `cd /bin` followed by bare `ls` prints `init`.
- `cd /` followed by bare `ls` returns to the root entries.
- `bogus` remains an unknown-command regression.
- The loop returns to a ready `talos>` prompt and prints
  `pi5-local-ls-cwd-complete` plus exact
  `rpi5-local-ls-cwd-proof: PASS`.

The startup proof line also retained the descriptor-backed input/output
identity:

`input=fd0/runtime-console0/tty-canonical-lite ... descriptor-backed-input=true descriptor-backed-output=true`.

## Classification

Final classification: accepted, `ls-cwd-feature-pass-retained`.

This accepts bounded kernel-backed command-context cwd behavior for bare `ls`
over the Pi 5 serial command loop. It does not accept broad path traversal,
relative paths, `.` or `..`, POSIX `chdir`, descriptor-backed filesystem
syscalls, process-local cwd inheritance, userspace shell execution, networking,
SSH, RP1/PCIe, UART interrupt ownership, DMA, or cache-driver policy.

## Evidence

- result summary:
  `tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/proof-result.txt`
- serial transcript:
  `tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/serial-transcript.txt`
- settled TFTP delta:
  `tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/tftp-delta-settled-before-restore.json`
- publish and restore status:
  `tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/post-publish-status.json`
  and
  `tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/post-restore-status.json`
- classification summary:
  `tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/classification-summary.json`

## Validation

- Serialized hardwareTestLock: acquired before publication, released/restored
  after the named snapshot restore.
- Image/archive inspection: `scripts/rpi5-archive-review.sh` passed for the
  selected accepted candidate archive.
- Lab-controller API: health, pre/post status, snapshot, publish, boot files,
  fixed-port power-cycle, serial write/observe, TFTP logs, restore, and
  post-restore status artifacts are retained.
- TFTP/lab-controller API: settled same-cursor delta before restore retained
  fresh candidate boot-file requests, including two 110624-byte
  `da591740/kernel_2712.img` serves.
- Serial hardware boot/output: retained transcript shows the full `pwd`,
  bare `ls`, `cd`, bare `ls`, `bogus`, ready-prompt, final
  classification, and PASS feature sequence.
- Restore proof: post-restore status returned the lab to the pre-run tree hash,
  and hardwareTestLock was released/restored after the run.
- Static inspection: `git diff --check` passed after evidence/doc updates.
- Documentation validation: `mdbook build` passed after roadmap update.
- Staged static inspection: `git diff --cached --check` passed before commit.

## Next Action

The local `ls` cwd closeout checkpoint is mechanically unblocked on the next
worker wake if supervisorIntervention remains inactive and hardwareTestLock
remains unlocked/restored.
