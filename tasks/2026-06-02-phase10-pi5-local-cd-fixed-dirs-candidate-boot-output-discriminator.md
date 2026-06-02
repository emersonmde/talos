# Phase 10 Pi 5 Local Cd Fixed Dirs Candidate Boot Output Discriminator

Task: phase10-pi5-local-cd-fixed-dirs-candidate-boot-output-discriminator-20260602
Status: accepted-blocked-candidate-entered-no-prompt

## Goal

Resolve the cd fixed-directories Pi 5 proof blocker far enough to avoid rerunning the same candidate blindly. The invariant is that a freshly fetched cd candidate should produce Talos boot/proof prompt output before the typed pwd/cd sequence, and the original feature proof still requires the full pwd/cd transcript plus PASS.

## Static Review

The focused comparison found no candidate-only boot-tree, image-script, build-routing, proof-main selection, command transcript, or early serial-output routing defect.

- scripts/rpi5-local-cd-fixed-dirs-image.sh and scripts/rpi5-local-cd-fixed-dirs-boot-tree.sh match the accepted local cat-banner/literal-echo script shape.
- build.rs registers rpi5_local_cd_fixed_dirs and implies rpi5_local_serial_command_loop, matching the accepted prompt-visible local command-loop proof family.
- src/boot/rpi5.rs dispatches run_local_serial_command_loop_proof() through the implied rpi5_local_serial_command_loop cfg.
- src/target/rpi5.rs includes the cd proof label, classification, command count, typed sequence, expected cwd outputs, and PASS vocabulary.
- The cd candidate uses the same firmware-preserved UART10/LocalCommandProofConsole output path as prompt-visible controls.

Detailed evidence: tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-candidate-boot-output-discriminator/static-review/static-comparison.txt.

## Hardware Evidence Boundary

No new hardware run was performed in this discriminator. The committed local8 unchanged candidate run is the relevant hardware input: it already fetched the unchanged candidate kernel twice from settled same-cursor TFTP evidence and retained fresh Talos entry through TALOS: dtb memory scan start, followed by NUL/newline and reboot output. It did not retain TALOS: dtb memory scan done, a talos> prompt, the cd/pwd command transcript, final classification, or PASS.

A repeat unchanged-candidate run would not be qualitatively different without a concrete static fix or a new instrumentation plan, and this task found no static routing defect to justify that rerun.

## Classification

Final classification: cd-candidate-entered-no-prompt.

The candidate is not proven, and the original Pi 5 cd fixed-directories proof remains blocked. The blocker is now narrowed to the physical path between DTB memory scan start and the local command-loop proof prompt, not TFTP staging, archive identity, or proof-main routing.

## Validation

- Static inspection: focused comparison against accepted literal-echo and cat-banner Pi 5 proof harnesses.
- Image/archive inspection: rebuilt current cd/literal/cat scenario images and inspected scenario strings/hashes; cd image still contains rpi5-local-cd-fixed-dirs-proof and pi5-local-cd-fixed-dirs-complete.
- Hardware evidence review: reused committed local8 lab-controller/TFTP/serial hardware evidence from tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-proof/local8-unchanged-candidate-status-endpoint-retry/.
- No runtime, proof-harness, boot-tree, image-routing, marker, wait, command-loop, or acceptance code changed.
- git diff --check: passed.
- mdbook build: not run because mdBook docs were not touched.
- git diff --cached --check: passed before commit.

## Next Action

Supervisor planning is needed for the next bounded task. The next task should either instrument or explain the stop between DTB memory scan start and prompt entry, or choose a concrete implementation fix if one is identified. Do not promote the closeout checkpoint or mark the original cd proof accepted until retained Pi 5 evidence satisfies the full cd feature acceptance criteria.
