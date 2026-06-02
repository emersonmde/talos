# Phase 10 Pi 5 Local Cd Fixed Dirs Proof Task

Task: phase10-pi5-local-cd-fixed-dirs-proof-20260602
Status: blocked-candidate-no-talos-serial

## Scope

Carry the accepted bounded cd fixed-directories command-loop behavior to serialized Raspberry Pi 5 serial hardware evidence. The worker added narrow Pi 5 proof-harness wiring and image/boot-tree scripts in commit 91021dc before hardware staging, without changing local command-loop cd semantics.

## Current Evidence

- Harness/static build: commit 91021dc adds rpi5_local_cd_fixed_dirs, scripts/rpi5-local-cd-fixed-dirs-image.sh, and scripts/rpi5-local-cd-fixed-dirs-boot-tree.sh.
- Archive/image inspection: local4 archive review passed; staged candidate kernel_2712.img size was 109192 bytes and image strings include rpi5-local-cd-fixed-dirs-proof and pi5-local-cd-fixed-dirs-complete.
- Candidate local4: tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-proof/local4-unchanged-candidate-rerun/proof-result.txt records fresh TFTP candidate kernel fetches for 109192 bytes but no Talos serial output beyond NUL/newline across cursor-advanced observes.
- Control local5: tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-proof/local5-clean-known-good-literal-echo-control/control-result.txt records fresh TFTP control kernel fetches for 100352 bytes but no retained literal-echo prompt/PASS before restore.
- Control local6: tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-proof/local6-known-good-literal-echo-control/control-result.txt records a clean repeated known-good literal-echo control with manual short cursor polls. It served fresh config/kernel/DTB/cmdline requests from settled same-cursor TFTP evidence, including kernel_2712.img size 100352 twice, but retained only NUL/newline serial bytes followed by empty serial polls before restore.
- Control phase10-pi5-serial-output-control-discriminator-20260602: accepted in commit 5b8e94f24a5f952ae98ab971642387df5bb34718 as prompt-capture healthy. The accepted literal-echo control retained prompt, ready-for-next prompt, classification, and PASS through non-draining serial peek, unblocking one unchanged cd candidate proof rerun.
- Candidate local8: tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-proof/local8-unchanged-candidate-status-endpoint-retry/proof-result.txt records the unchanged cd archive hash 8897a58f519d45de1cc6e8be91b2d1cc469c722536902edc1355e8a685f70f68 and kernel hash 46ad6246b42f9a2459cf1ebe4fd0c0e1128e7ba5645da35cdd43d5a2dd1ac969. The lab staged kernel_2712.img size 109192, served fresh settled TFTP candidate requests twice from cursor 4054462, and restored the pre-run tree hash, but the fresh serial observe from cursor 4015894 retained only NUL/newline and no rpi5-local-cd-fixed-dirs prompt, command transcript, classification, or PASS.
- Candidate local7: tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-proof/local7-unchanged-candidate-after-serial-output-control/ is an aborted pre-hardware setup attempt. It stopped before snapshot, publish, power-cycle, or serial write because this lab-controller deployment returns 404 for GET /; local8 used GET /status and performed the actual hardware run.
- Earlier local1 is retained as an invalid candidate proof because restore timing contaminated the TFTP delta.

## Validation This Wake

- fmt/lint: cargo fmt --all -- --check passed before harness commit.
- unit tests: cargo -Zjson-target-spec test --quiet passed before harness commit, with existing dead_code warnings.
- QEMU/substitute regression: scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet passed before harness commit.
- image/archive review: scripts/rpi5-archive-review.sh target/talos-rpi5-local-cd-fixed-dirs-local1.tar.gz passed.
- lab-controller API: health, status, snapshot, publish, boot files, fixed-port power-cycle, TFTP logs, serial observe, serial peek, named snapshot restore, and post-restore status artifacts retained.
- hardware proof gate: local8 retained fresh candidate TFTP requests and restore proof, but did not retain the required cd command transcript, ready-for-next prompt, pi5-local-cd-fixed-dirs-complete classification, or rpi5-local-cd-fixed-dirs-proof: PASS.

## Current Classification

Blocked for supervisor planning. The accepted feature is still not proven on Pi 5 hardware. After the separate serial-output control discriminator accepted prompt capture as healthy, local8 performed one unchanged cd candidate proof rerun. The candidate archive was fetched from TFTP with the expected 109192-byte kernel, but the fresh serial observe retained no cd proof output beyond NUL/newline. Non-draining peek also contained stale prompt/PASS tail text from earlier accepted runs, so it is not used as candidate acceptance evidence for local8.

HardwareTestLock was released/restored at end of wake.

## Follow-up Discriminator

phase10-pi5-local-cd-fixed-dirs-candidate-boot-output-discriminator-20260602 is accepted-blocked-candidate-entered-no-prompt. Its focused static comparison found no candidate-only boot-tree, image-script, build-routing, proof-main selection, command transcript, or early serial-output routing defect. It reclassified the committed local8 evidence as a fresh candidate that entered Talos through `TALOS: dtb memory scan start` but did not retain `TALOS: dtb memory scan done`, prompt, command transcript, final classification, or PASS. The original cd proof remains blocked pending supervisor-planned instrumentation or a concrete fix for the stop between DTB memory scan start and prompt entry.
