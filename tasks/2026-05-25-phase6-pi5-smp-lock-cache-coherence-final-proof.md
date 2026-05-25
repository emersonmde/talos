# Phase 6 Pi 5 SMP Lock Cache Coherence Final Proof

Task: `phase6-pi5-smp-lock-cache-coherence-final-proof-20260525`

Status: accepted.

## Scope

This task runs the serialized physical Pi 5 proof for the accepted
`SpinLock<T>` primitive after the secondary cacheable-MMU handoff and
report-invariant correction. It does not add scheduler migration, shared run
queues, cross-core wakeups, IPIs, userspace, descriptors, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA
policy.

## Evidence

Evidence directory:
`tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/`.

- Archive: `target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`.
- Archive SHA256:
  `73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`.
- Kernel SHA256:
  `e28596b5f259775c4c239c3e18b57e3d61d24ff453aa3c762c879e38075f7278`.
- Kernel size: 96,824 bytes.
- Archive review passed with `file_count=19`, `header_image_size=96824`,
  `text_offset=0`, `flags=12`, and `loader_diagnostic=false`.
- Lab-controller records captured health, pre-run snapshot,
  archive publish, post-publish status, power cycle, TFTP delta, serial
  observe, restore, and post-restore status.
- The post-publish boot tree showed both `kernel_2712.img` and
  `da591740/kernel_2712.img` at 96,824 bytes before power cycle.
- TFTP logs after the publish/power cycle show the Pi 5 at `10.42.1.4` /
  `88:a2:9e:ae:c8:7f` fetching the serial-prefixed boot files. The delayed
  TFTP parser's byte field is not used as the candidate identity source after
  restore; candidate identity is tied through the archive digest,
  post-publish status, and candidate-only serial transcript.

## Hardware Result

The cursor-valid serial observe started at byte cursor `2057899` and ended at
`2066014`. The transcript reports:

- boot CPU:
  `boot-sctlr-el2=0x0000000030c51835 boot-cacheable-mmu=true`.
- handoff plan:
  `mair-el2=0x00000000000004ff`,
  `tcr-el2=0x0000000000053510`,
  `ttbr0-el2=0x000000002f000000`,
  `sctlr-el2=0x0000000030c51835`,
  `cacheable-mmu=true`.
- logical cores 1, 2, and 3 each reached `workload-complete` with
  `progress=64`, `diag-progress=64`, `diag-sctlr-el2=0x0000000030c51835`,
  `diag-cacheable-mmu=true`, and `ok=true`.
- final invariant:
  `counter=192 expected=192 participants=3 diag-participants=3 errors=0`,
  `lock-available=true generic-state-visible=true mixed-cache-mmu=false`,
  `classification=pi5-smp-lock-cache-coherence-complete`.
- final result: `rpi5-smp-lock-cache-coherence: PASS`.

Pre-run snapshot
`pre-phase6-pi5-smp-lock-final-proof-20260525T033151Z` was restored.
`post-restore-status.json` shows the prior 82,045-byte boot tree restored.

## Validation

- static inspection: `git status --short` was inspected before the run and
  showed a clean Talos worktree.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 103 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed with
  `counter=192 expected=192 participants=3 errors=0`,
  `classification=qemu-smp-lock-contention-complete`, and per-core
  `ok=true` reports for logical cores 1, 2, and 3.
- image/archive inspection:
  `scripts/rpi5-smp-lock-cache-coherence-image.sh` and
  `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`
  passed.
- serial hardware boot/output: serialized Pi 5 hardware run under
  `hardwareTestLock` passed with the invariant above.
- restore proof: pre-run snapshot was restored and post-restore status was
  captured.
- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in the container.

## Acceptance

Accepted as the physical Pi 5 cache/coherence proof for the first Milestone
6.2 SMP-safe primitive. The proof accepts generic lock contention across the
boot CPU and secondary cores only after the accepted secondary cacheable-MMU
handoff. It does not accept scheduler migration, shared run queues,
cross-core wakeups, IPIs, userspace, descriptors, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-coherent
driver policy.
