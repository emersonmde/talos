# Pi 5 Secondary Cacheable MMU Handoff Proof Evidence

Task: phase6-secondary-cacheable-mmu-handoff-pi5-proof-20260524

Classification: `pi5-secondary-cacheable-mmu-handoff-proved-lock-invariant-failed-after-handoff`.

The serialized Pi 5 run published archive `target/talos-rpi5-secondary-cacheable-mmu-handoff-proof-boot.tar.gz` (`21f4e80cef35b40d13792fdac4f7a0fa6cce463af0d3eb3c825d9d6c87653d90`) with kernel image SHA256 `acc334beb5bc82555d6d4c3309d3e24b0b669593768cb9d01e479bc40e350e40`, size 96,792 bytes.

Hardware evidence:

- `publish.json` and `post-publish-status.json`: candidate boot tree staged with 96,792-byte `kernel_2712.img`.
- `power-cycle.json`: fixed-port PoE cycle succeeded.
- `tftp-delta-final-before-restore.json`: Pi 5 fetched `da591740/kernel_2712.img` after the publish/power-cycle step. The event timing is used as TFTP proof; the delayed `bytes` field is not used as the candidate-size source because the lab API computes it from the current boot tree.
- `serial-peek-before-restore.txt`: candidate-only transcript showed boot CPU `boot-cacheable-mmu=true`, published the handoff plan, and showed logical cores 1, 2, and 3 with `diag-sctlr-el2=0x0000000030c51835` and `diag-cacheable-mmu=true`.
- `restore-pre-snapshot.json` and `post-restore-status.json`: restored `pre-phase6-secondary-cacheable-mmu-handoff-proof-20260525T013130Z`.

Result:

- Handoff gate: passed for logical cores 1, 2, and 3.
- Full lock proof: not accepted. The same run ended with `classification=pi5-smp-lock-cache-coherence-invariant-failed`, with final counter 192/192 and `mixed-cache-mmu=false`, but logical cores 1 and 2 had zeroed final identity fields in the per-core report.

Local gates:

- `cargo fmt --all -- --check`: passed.
- `cargo -Zjson-target-spec test`: passed, 102 no_std tests.
- `scripts/qemu-smoke.sh`: passed.
- `scripts/qemu-smp-lock-contention-smoke.sh`: passed.
- `scripts/rpi5-archive-review.sh target/talos-rpi5-secondary-cacheable-mmu-handoff-proof-boot.tar.gz`: passed.
- `git diff --check`: passed.
- `mdbook build`: not run; `mdbook` unavailable.
