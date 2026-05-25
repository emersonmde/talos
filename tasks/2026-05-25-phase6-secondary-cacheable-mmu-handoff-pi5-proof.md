# Phase 6 Secondary Cacheable MMU Handoff Pi 5 Proof

Task: phase6-secondary-cacheable-mmu-handoff-pi5-proof-20260524

## Summary

This task proves the secondary cacheable EL2 stage-1 handoff on serialized Pi 5 hardware before the original SMP lock proof is resumed. It does not accept the full lock/coherence proof; the hardware run exposed a later lock diagnostic invariant failure after the handoff gate passed.

## Hardware Evidence

Evidence directory: `tasks/evidence/2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof/`.

- Artifact inspection: archive `target/talos-rpi5-secondary-cacheable-mmu-handoff-proof-boot.tar.gz` SHA256 `21f4e80cef35b40d13792fdac4f7a0fa6cce463af0d3eb3c825d9d6c87653d90`; kernel image SHA256 `acc334beb5bc82555d6d4c3309d3e24b0b669593768cb9d01e479bc40e350e40`, size 96,792 bytes.
- Archive review: `scripts/rpi5-archive-review.sh target/talos-rpi5-secondary-cacheable-mmu-handoff-proof-boot.tar.gz` passed with `kernel_size=96792`, `text_offset=0`, `flags=12`, and `loader_diagnostic=false`.
- Lab-controller API: `health.json`, `pre-status.json`, `post-publish-status.json`, `publish.json`, `power-cycle.json`, `restore-pre-snapshot.json`, and `post-restore-status.json` captured the serialized publish, power cycle, and restore flow.
- TFTP evidence: `tftp-delta-final-before-restore.json` records the Pi 5 at `10.42.1.4` / `88:a2:9e:ae:c8:7f` fetching `da591740/kernel_2712.img` between publish and restore. The delayed TFTP byte field is not used as the candidate-size source because the lab API computes that field from the current boot tree at query time; candidate identity is instead tied through `publish.json`, `post-publish-status.json`, the archive digest, and the candidate-only serial output.
- Serial hardware output: `serial-peek-before-restore.txt` contains the current candidate transcript. It reports the boot CPU cacheable-MMU state, publishes the secondary handoff plan, and shows logical cores 1, 2, and 3 completing the lock diagnostic with `diag-sctlr-el2=0x0000000030c51835` and `diag-cacheable-mmu=true`.

Key current-run serial facts:

- boot CPU: `boot-sctlr-el2=0x0000000030c51835 boot-cacheable-mmu=true`.
- handoff plan: `mair-el2=0x00000000000004ff`, `tcr-el2=0x0000000000053510`, `ttbr0-el2=0x000000002f000000`, `sctlr-el2=0x0000000030c51835`, `cacheable-mmu=true`.
- logical cores 1, 2, and 3: `diag-sctlr-el2=0x0000000030c51835` and `diag-cacheable-mmu=true`.
- final lock proof line: `classification=pi5-smp-lock-cache-coherence-invariant-failed`.

Classification: `pi5-secondary-cacheable-mmu-handoff-proved-lock-invariant-failed-after-handoff`.

## Validation

- static inspection: `git status --short` was inspected before hardware work; the tree already contained unrelated untracked evidence from the prior lock proof and those files were preserved.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 102 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed with classification `qemu-smp-lock-contention-complete`.
- image/archive inspection: the focused Pi 5 archive was built and `scripts/rpi5-archive-review.sh` passed.
- serial hardware boot/output: the Pi 5 hardware run proved the secondary cacheable-MMU handoff invariant for logical cores 1, 2, and 3 before any lock-proof acceptance claim.
- restore evidence: pre-run snapshot `pre-phase6-secondary-cacheable-mmu-handoff-proof-20260525T013130Z` was restored; `post-restore-status.json` shows the previous 82,045-byte boot tree.
- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container, so mdBook build was not run.

## Acceptance

Accepted for the secondary cacheable-MMU handoff gate only. The original Pi 5 SMP lock cache/coherence proof remains not accepted because the same hardware run ended with `pi5-smp-lock-cache-coherence-invariant-failed`: the shared counter and cache/MMU state were visible, but the final per-core state report for logical cores 1 and 2 still carried zeroed identity fields. That follow-up must be supervisor-planned as a resumed lock-proof/debug task rather than silently folded into this handoff proof.
