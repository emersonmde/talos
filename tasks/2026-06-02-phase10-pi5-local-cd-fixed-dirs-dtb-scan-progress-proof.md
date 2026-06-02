# Phase 10 Pi 5 Local Cd Fixed Dirs DTB Scan Progress Proof

Task: phase10-pi5-local-cd-fixed-dirs-dtb-scan-progress-proof-20260602
Status: accepted-blocked-dtb-memory-scan-not-entered

## Goal

Use the accepted RPi5 DTB memory-scan progress core to rerun the cd
fixed-directories candidate on Pi 5 far enough to classify the pre-prompt
blocker or prove the full cd feature.

## Candidate

- Source task: phase10-rpi5-dtb-memory-scan-progress-core-20260602,
  accepted at commit db883fd.
- Archive: target/talos-rpi5-local-cd-fixed-dirs-dtb-scan-progress-local1.tar.gz.
- Archive sha256:
  05fc09129458a4a6e93fc5bd67db07e351bff561c1ea49789764b0769746fdc6.
- Kernel sha256:
  773a7088c7f902c857950d7698d465e0bfd13c74f540b1a85b4f185fa384357f.
- Archive review passed with kernel_2712.img size 110008 bytes.
- Kernel strings retained the new progress markers for DTB scan, memory-plan,
  cache transition, and command-loop proof readiness.

## Hardware Runs

Evidence root:
tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-dtb-scan-progress-proof/.

The worker held hardwareTestLock for the serialized proof attempts and restored
the pre-run tree hash after the final candidate run:
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

- local1-dtb-progress-candidate: fetched only da591740/config.txt before the
  restore, retained fresh firmware/RP1 serial output, but restored before a
  settled candidate kernel fetch. Kept as incomplete collection evidence.
- local2-settled-dtb-progress-candidate and local3-settled-dtb-progress-candidate:
  aborted during evidence collection after publish; cleanup/manual restore
  returned the boot tree to the pre-run hash. These are not acceptance evidence.
- local4-waited-dtb-progress-candidate: retained a settled same-cursor TFTP
  delta with 13 events, including da591740/kernel_2712.img served twice at
  110008 bytes, and restored the pre-run hash. Its fresh serial observe was
  collected before the delayed kernel fetch and therefore did not classify the
  post-kernel boundary.
- local5-post-tftp-serial-dtb-progress-candidate: retained the decisive
  evidence. The same-cursor TFTP delta reached a stable 13-event window after
  the candidate kernel fetch; da591740/kernel_2712.img was served twice at
  110008 bytes along with the candidate DTB, overlays, config, and cmdline.
  After that settled TFTP evidence, fresh serial observe from the pre-run
  cursor retained only NUL/space bytes and no TALOS: rust_entry, DTB memory
  scan marker, command-loop marker, prompt, cd transcript, classification, or
  PASS.

The full pwd/cd transcript was not typed because no fresh prompt or command-loop
ready marker appeared in the candidate serial evidence.

## Classification

Final classification: dtb-memory-scan-not-entered.

The candidate archive was fetched by firmware from TFTP, but the fresh
post-kernel serial evidence did not retain Talos entry or the first DTB memory
scan marker. The original Pi 5 cd fixed-directories proof remains blocked. This
task does not accept the cd feature, because it lacks the required pwd/cd
sequence, cwd outputs, ready prompt, pi5-local-cd-fixed-dirs-complete
classification, and rpi5-local-cd-fixed-dirs-proof: PASS.

## Validation

- pre-run static state: Talos git status was clean except task evidence generated
  by this task.
- image/archive inspection: scripts/rpi5-archive-review.sh passed for the
  candidate archive; marker string inspection confirmed the accepted progress
  core was present in the kernel image.
- lab-controller API: health, status, snapshot, publish, boot files, fixed-port
  power-cycle, TFTP logs, serial observe/peek, named snapshot restore, and
  post-restore status artifacts are retained.
- TFTP/lab-controller API: local5 retained a settled same-cursor TFTP delta
  before restore with candidate kernel fetches at 110008 bytes.
- serial hardware boot/output: local5 retained fresh post-TFTP serial evidence
  from the captured cursor; it contained no Talos entry, DTB memory scan,
  prompt, cd transcript, classification, or PASS.
- restore proof: local5 pre/post tree hashes match and hardwareTestLock was
  released/restored after the run.
- static diff hygiene: git diff --check passed.
- documentation: mdBook was not required because mdBook docs were not touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Action

The original cd proof remains blocked with the narrowed hardware classification.
Supervisor planning is required for any follow-up implementation task before
another Pi 5 rerun; do not promote the cd closeout checkpoint.
