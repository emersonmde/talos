# Phase 10 Local Cat Cwd Core Task

Task: phase10-local-cat-cwd-core-20260602
Status: accepted

## Goal

Add bounded cwd-aware file reading to the descriptor-backed serial command loop: cd /etc followed by cat banner.txt prints the accepted immutable initramfs banner.

## Scope

Implemented relative cat banner.txt dispatch against the existing prompt-local command-context cwd. The command succeeds only when cwd is /etc; from / it returns talos: not-found. The existing exact cat /etc/banner.txt path remains accepted.

Changed files: build.rs; scripts/qemu-local-cat-cwd-smoke.sh; scripts/qemu-local-serial-command-loop-smoke.sh; scripts/rpi5-local-cat-cwd-boot-tree.sh; scripts/rpi5-local-cat-cwd-image.sh; src/local_command_loop.rs; src/main.rs; src/target/qemu_virt.rs; src/target/rpi5.rs; docs/src/roadmap.md; this task record; retained evidence under tasks/evidence/2026-06-02-qemu-local-cat-cwd-core/.

## Accepted Frontier

Accepted: through fd0/runtime-console0 canonical-lite input and descriptor-backed stdout, cd /etc updates command-context cwd, cat banner.txt prints Talos initramfs fixture, the loop returns to a ready prompt, cd / returns to root cwd, and cat banner.txt from root prints talos: not-found.

This is still kernel-backed, prompt-local shell UX. It is not arbitrary relative path traversal, . or .., POSIX cwd/syscall behavior, descriptor-backed filesystem syscalls, writable filesystems, userspace shell execution, process-local cwd inheritance, globbing, quoting, pipes, redirection, networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache policy.

## Evidence

- QEMU/substitute cat-cwd transcript: tasks/evidence/2026-06-02-qemu-local-cat-cwd-core/qemu-local-cat-cwd-smoke.log.
- Transcript shows cd /etc, cat banner.txt, visible Talos initramfs fixture, cd /, cwd-sensitive cat banner.txt negative output talos: not-found, next-prompt readiness, final classification qemu-local-cat-cwd-complete, and exact PASS line qemu-local-cat-cwd: PASS.
- RPi5 candidate archive: target/talos-rpi5-local-cat-cwd-candidate-archive-core.tar.gz.
- Archive sha256: a1159da288089df9cbbf17edc2289d7900108be7864b675bf8291d6352e62c83.
- Kernel sha256: 86a23a565c25ae094241c07f4c0ff58583f3d47231cb66ac2f5b52291e79492f.
- Kernel size: 110992 bytes.
- Static proof-string inspection found rpi5-local-cat-cwd-proof, pi5-local-cat-cwd-complete, cat-cwd-observed, cat-cwd-negative-observed, and TALOS: command loop proof entered; quarantined raw entry markers TALOS: asm_start and TALOS: asm_pre_rust_entry were absent.
- hardwareTestLock remained unlocked/restored and unused; no archive was published and no Pi 5 power-cycle was performed.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute feature gate: scripts/qemu-local-cat-cwd-smoke.sh passed.
- QEMU/substitute regressions passed: scripts/qemu-local-cat-banner-smoke.sh --quiet; scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet; scripts/qemu-local-ls-cwd-smoke.sh --quiet; scripts/qemu-local-line-editing-smoke.sh --quiet; scripts/qemu-local-line-cancel-smoke.sh --quiet; scripts/qemu-local-line-kill-smoke.sh --quiet; scripts/qemu-local-serial-command-loop-smoke.sh --quiet.
- RPi5 archive/image inspection: scripts/rpi5-archive-review.sh target/talos-rpi5-local-cat-cwd-candidate-archive-core.tar.gz passed.
- Static diff hygiene: git diff --check passed.
- Documentation: mdbook build passed.
- Pre-commit static inspection: git diff --cached --check passed.

Acceptance commit: recorded in durable supervisor state after commit creation.

## Next Action

The next mechanically unblocked task is phase10-pi5-local-cat-cwd-proof-20260602 if hardwareTestLock remains unlocked/restored and supervisorIntervention remains inactive. It must publish only target/talos-rpi5-local-cat-cwd-candidate-archive-core.tar.gz and may accept the feature only with retained Pi 5 serial transcript, settled TFTP evidence before restore, restore proof, pi5-local-cat-cwd-complete, and rpi5-local-cat-cwd-proof: PASS.
