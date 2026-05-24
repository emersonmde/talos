# Phase 6 Pi 5 PSCI Secondary-Core Alive Proof Evidence

Candidate archive:
`target/talos-rpi5-psci-secondary-core-alive-boot.tar.gz`

- Archive SHA256:
  `5479aebe32d835935e9f2ce391039da8e0cc11ed44ea545da672a513e2286c92`
- Kernel image SHA256:
  `74d27f56da55a1924c1e6436a5bbf9169b151feec9ca31a097223419e91f3654`
- Kernel size: 90,016 bytes
- TFTP proof: `tftp-delta.json`, `second-tftp-delta-late.json`
- Serial proof: `serial-observe.json`, `second-serial-observe.json`
- Restore proof: `restore-pre-snapshot.json`, `post-restore-status.json`

Result: inconclusive hardware attempt. The Pi fetched the candidate
`da591740/kernel_2712.img`, but serial capture after the candidate runs
contained only a NUL/newline and no Talos PSCI alive transcript.

Classification: `serial-capture-or-pre-entry-output-ambiguity`.

## Discriminator Rerun

Candidate archive:
`target/talos-rpi5-psci-secondary-core-alive-discriminator-boot.tar.gz`

- Archive SHA256:
  `92addd41df49466bc7e588546086d7067b1dc73197c1726834ff474aceb5a906`
- Kernel image SHA256:
  `705f50c71212430d298002b54ebc9823e911a27db7885d3750855e0bf27da62b`
- Kernel size: 90,016 bytes
- TFTP proof: `discriminator2-tftp-delta.json`
- Serial proof: `discriminator2-serial-observe-second.json`
- Restore proof: `discriminator2-restore-pre-snapshot.json`,
  `discriminator2-post-restore-status.json`

Result: decisive hardware failure, not accepted proof. The Pi fetched the
candidate image and serial showed `asm_start`, `asm_pre_rust_entry`,
`rust_entry`, normal Talos boot output, and PSCI `CPU_ON` success for
logical cores 1, 2, and 3. The secondary-core reports remained `parked` with
zero MPIDR, affinity, context, and stack pointer.

Classification: `pi5-psci-started-but-state-or-stack-incomplete`.

## Secondary-Entry Discriminator Rerun

Candidate archive:
target/talos-rpi5-psci-secondary-core-alive-entry-discriminator-boot.tar.gz

- Archive SHA256:
  1ef6ec1daf33cc99feae786dc2daa765dbff9aa9308edd71b3240f117769df6f
- Kernel image SHA256:
  5e099ff4e75986cc7043fc196d41565fb9ada25321a8f8386be9c45c7d0931e7
- Kernel size: 90,016 bytes
- TFTP proof: entrydisc2-tftp-delta.json
- Serial proof: entrydisc2-post-restore-serial-peek-20k.json
- Restore proof: entrydisc2-restore-pre-snapshot.json,
  entrydisc2-post-restore-status.json

Result: decisive hardware failure, not accepted proof. The Pi fetched the
candidate image, primary boot reached rust_entry, and PSCI CPU_ON returned 0
for logical cores 1, 2, and 3. The secondary trampoline markers TALOS:
secondary_entry and TALOS: secondary_pre_rust_entry did not appear, and the
secondary-core reports remained parked with zero MPIDR, affinity, context, and
stack pointer.

Classification: pi5-psci-accepted-secondary-entry-not-observed.

## PSCI State Discriminator Rerun

Candidate archive:
`target/talos-rpi5-psci-secondary-core-alive-state-discriminator-boot.tar.gz`

- Archive SHA256:
  `e2e16f292d5f8ad9eff8b139af47f0491d7f9af44397488941ec2dbf8a449bca`
- Kernel image SHA256:
  `cd90dde7543838ad8f95203b92a0d90914ff62695d7c881d717a1cf8d478d954`
- Kernel size: 91,000 bytes
- TFTP proof: `statedisc3-tftp-delta.json`
- Serial proof: `statedisc3-post-restore-serial-peek-500k.json`
- Restore proof: `statedisc3-restore-pre-snapshot.json`,
  `statedisc3-post-restore-status.json`

Result: decisive hardware failure for this discriminator image, not accepted
proof. The Pi fetched `da591740/kernel_2712.img` twice at 91,000 bytes from
the candidate boot tree, but the serial log for that power cycle did not show
BL31, `TALOS: asm_start`, `TALOS: rust_entry`, or PSCI state-discriminator
lines before the pre-run snapshot was restored.

Classification:
`pi5-state-discriminator-candidate-fetched-no-bl31-or-asm-entry`.

## Minimal PSCI State Discriminator Rerun

Candidate archive:
`target/talos-rpi5-psci-secondary-core-alive-minstate-boot.tar.gz`

- Archive SHA256:
  `542f3b87302b82a91776f72d0e04408c24cf9680205537acd19447a00e0475dd`
- Kernel image SHA256:
  `50295ba874792d6e732c2af6b70fdffd708e86847e4b30c0fb873442dd71807f`
- Kernel size: 90,416 bytes
- TFTP proof: `minstate-tftp-delta.json`
- Serial proof: `minstate-serial-observe.json`,
  `minstate-serial-observe-second.json`,
  `minstate-serial-peek-500k-before-restore.json`
- Restore proof: `minstate-restore-pre-snapshot.json`,
  `minstate-post-restore-status.json`

Result: decisive non-acceptance for this discriminator image. Local
image/disassembly review showed the image header, entry symbols, and diagnostic
strings were present, and the smaller archive was fetched repeatedly as
`da591740/kernel_2712.img` at 90,416 bytes. Serial observed after the run did
not contain current-candidate BL31-to-Talos entry or the minimal
`AFFINITY_INFO` diagnostic lines before the pre-run snapshot was restored.

Classification:
`pi5-minstate-discriminator-candidate-fetched-no-current-entry`.

## Cache-Coherent Pi 5 PSCI Alive Proof

Accepted candidate archive:
`target/talos-rpi5-psci-secondary-core-alive-cachecoherent-boot.tar.gz`

- Archive SHA256:
  `58803e6c4fd21a7c40d2f36245e8e7c366e80ea50dbcdba2afd2952d952c4d22`
- Kernel image SHA256:
  `2f1622d7694f84446153240d1136b9f095df0cd09d69e9f0ae88de2ae1ef9996`
- Kernel size: 90,784 bytes
- TFTP proof: `cachecoh2-tftp-delta-before-restore.json`
- Serial proof: `cachecoh2-serial-peek-before-restore.json`
- Restore proof: `cachecoh2-restore-pre-snapshot.json`,
  `cachecoh2-post-restore-status.json`

Result: accepted Pi 5 PSCI secondary-core alive proof. The Pi fetched
`da591740/kernel_2712.img` twice at 90,784 bytes before restore. Serial
showed secondaries reached the trampoline and Rust entry, published per-core
state, and the primary observed logical cores 1-3 as `handoff-ready` with
MPIDR affinities `0x100`, `0x200`, and `0x300`, distinct stack pointers
inside their owned 4 KiB slots, `ok=true`, classification
`pi5-psci-smc-secondary-cores-alive`, and `PASS`.

Classification: `pi5-psci-smc-secondary-cores-alive`.
