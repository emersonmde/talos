# Pi 5 SMP Lock Cache Coherence Final Proof Evidence

Task: `phase6-pi5-smp-lock-cache-coherence-final-proof-20260525`

Classification: `pi5-smp-lock-cache-coherence-complete`

## Artifact

- Archive: `target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`
- Archive SHA256:
  `73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`
- Kernel SHA256:
  `e28596b5f259775c4c239c3e18b57e3d61d24ff453aa3c762c879e38075f7278`
- Kernel size: 96,824 bytes
- Archive review: passed with `file_count=19`, `kernel_size=96824`,
  `text_offset=0`, `flags=12`, and `loader_diagnostic=false`

## Hardware Run

The serialized hardware run captured:

- `health.json`
- `pre-status.json`
- `pre-snapshot.json`
- `publish.json`
- `post-publish-status.json`
- `post-publish-files.json`
- `power-cycle.json`
- `tftp-delta-before-restore.json`
- `serial-observe.json` and `serial-observe.txt`
- `serial-key-lines.txt`
- `pre-restore-status.json`
- `restore-pre-snapshot.json`
- `post-restore-status.json`

The archive publish succeeded and `post-publish-status.json` shows both
`kernel_2712.img` and `da591740/kernel_2712.img` at 96,824 bytes before
power cycle. TFTP logs show the Pi 5 at `10.42.1.4` /
`88:a2:9e:ae:c8:7f` fetching the serial-prefixed boot tree after publish.
The TFTP event byte field is not used as the post-restore candidate-size
source; candidate identity is tied through the archive digest, post-publish
status, and candidate-only serial output.

The serial observe used cursor `2057899` and advanced to `2066014`. Key
serial facts:

- Boot CPU: `boot-cacheable-mmu=true` with
  `boot-sctlr-el2=0x0000000030c51835`.
- Handoff plan: `cacheable-mmu=true`,
  `mair-el2=0x00000000000004ff`,
  `tcr-el2=0x0000000000053510`,
  `ttbr0-el2=0x000000002f000000`, and
  `sctlr-el2=0x0000000030c51835`.
- Logical cores 1, 2, and 3 each report `workload-complete`,
  `lock-count=64`, `progress=64`,
  `diag-sctlr-el2=0x0000000030c51835`,
  `diag-cacheable-mmu=true`, and `ok=true`.
- Final invariant:
  `counter=192 expected=192 participants=3 diag-participants=3 errors=0`,
  `mixed-cache-mmu=false`,
  `classification=pi5-smp-lock-cache-coherence-complete`.
- Final result: `rpi5-smp-lock-cache-coherence: PASS`.

Pre-run snapshot
`pre-phase6-pi5-smp-lock-final-proof-20260525T033151Z` was restored.
`post-restore-status.json` shows the prior 82,045-byte boot tree restored.

## Validation Summary

- `cargo fmt --all -- --check`: passed
- `cargo -Zjson-target-spec test`: passed with 103 no_std tests
- `scripts/qemu-smoke.sh`: passed
- `scripts/qemu-smp-lock-contention-smoke.sh`: passed with
  `counter=192 expected=192 participants=3 errors=0`
- `scripts/rpi5-smp-lock-cache-coherence-image.sh`: passed
- `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`:
  passed
- Serialized Pi 5 hardware run under `hardwareTestLock`: passed
- `git diff --check`: passed
- `mdbook build`: unavailable because `mdbook` is not installed in the
  container
