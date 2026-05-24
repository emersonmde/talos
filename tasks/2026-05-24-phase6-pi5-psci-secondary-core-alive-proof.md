# Phase 6 Pi 5 PSCI Secondary-Core Alive Proof

Task: `phase6-pi5-psci-secondary-core-alive-proof-20260524`

## Goal

Prove on physical Raspberry Pi 5 hardware that PSCI starts secondary cores and
each secondary reports a bounded alive record through the accepted Phase 6.1
per-core state and stack boundary.

## Implementation Shape

- Focused diagnostic cfg: `TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF`.
- Image helper: `scripts/rpi5-psci-secondary-core-alive-image.sh`.
- Boot-tree helper: `scripts/rpi5-psci-secondary-core-alive-boot-tree.sh`.
- PSCI conduit: SMC `CPU_ON` function ID `0xc4000003`.
- Expected Pi 5 target affinities: `0x100`, `0x200`, and `0x300`.
- Secondary entry: a narrow AArch64 trampoline selects the accepted per-core
  4 KiB stack slot by logical CPU context, calls `talos_rpi5_secondary_entry`,
  records MPIDR identity, stack pointer, lifecycle state, and parks in `wfe`.

The diagnostic does not add controlled kernel-thread workloads, SMP scheduler
migration, load balancing, SMP locks, cross-core wakeups, UART interrupts, EL0,
syscalls, descriptors, filesystem, networking, SSH, or shell behavior.

## Local Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 96 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-secondary-core-discriminator.sh` passed after
  the shared secondary-entry assembly refactor.
- Image/archive inspection: `scripts/rpi5-image.sh` built the normal Pi 5
  image.
- Image/archive inspection:
  `scripts/rpi5-psci-secondary-core-alive-image.sh` built the focused
  diagnostic image.
- Image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-psci-secondary-core-alive-boot.tar.gz` passed with archive
  sha256
  `5479aebe32d835935e9f2ce391039da8e0cc11ed44ea545da672a513e2286c92`,
  kernel_size=90016, header_image_size=90016, text_offset=0, and flags=12.
- Kernel image sha256:
  `74d27f56da55a1924c1e6436a5bbf9169b151feec9ca31a097223419e91f3654`.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

## Hardware Attempt 1

Evidence directory:
`tasks/evidence/2026-05-24-pi5-psci-secondary-core-alive-proof/`.

- Hardware lock: acquired before publish/power-cycle and released after TFTP,
  serial, and restore evidence.
- Candidate archive:
  `target/talos-rpi5-psci-secondary-core-alive-boot.tar.gz`.
- TFTP/archive proof: `tftp-delta.json` and `second-tftp-delta-late.json`
  show 10.42.1.4 was served `da591740/kernel_2712.img` at 90,016 bytes from
  the candidate boot tree.
- Serial hardware boot/output: `serial-observe.json` and
  `second-serial-observe.json` captured only a NUL/newline after the
  candidate fetches. No Talos `rust_entry`, PSCI `cpu-on`, or secondary-core
  alive lines were captured.
- Safe-state note: pre-run boot snapshot
  `pre-phase6-pi5-psci-alive-20260524T175400Z` was restored after capture;
  `post-restore-status.json` records the restored boot tree state.

Classification: hardware evidence remains inconclusive with a
`serial-capture-or-pre-entry-output-ambiguity` failure. The TFTP evidence ties
the run to the candidate image, but the serial transcript does not prove PSCI
failure, core identity mismatch, stack/state registration failure, or success.

## Next Action

Continue this task with a bounded pre-entry/early-serial discriminator before
another PSCI alive acceptance attempt. The next iteration should determine
whether the no-output result is a serial capture/staging issue or a candidate
pre-`rust_entry` boot failure, while preserving the same non-goals.

## Hardware Attempt 2

Added two focused assembly markers to the Pi 5 PSCI proof image:
`TALOS: asm_start` immediately after preserving the firmware `x0` DTB
pointer, and `TALOS: asm_pre_rust_entry` immediately before branching to
`rust_entry`.

The first discriminator image reached `asm_start` only. Static review found
the discriminator helper itself clobbered `x30` through nested `bl` calls
before a stack was available, so this was classified as
`discriminator-helper-return-bug`, not as a Talos boot failure.

After preserving the helper return address in `x22`, the rerun archive
`target/talos-rpi5-psci-secondary-core-alive-discriminator-boot.tar.gz`
had archive sha256
`92addd41df49466bc7e588546086d7067b1dc73197c1726834ff474aceb5a906`,
kernel sha256
`705f50c71212430d298002b54ebc9823e911a27db7885d3750855e0bf27da62b`,
and kernel size 90,016 bytes.

Evidence files:

- TFTP/archive proof: `discriminator2-tftp-delta.json` shows
  `da591740/kernel_2712.img` served twice at 90,016 bytes.
- Serial hardware boot/output: `discriminator2-serial-observe-second.json`
  shows `asm_start`, `asm_pre_rust_entry`, `rust_entry`, normal Talos boot
  reports, PSCI `CPU_ON` result 0 for target affinities `0x100`, `0x200`,
  and `0x300`, then all three secondary records remaining `parked`.
- Restore proof: `discriminator2-restore-pre-snapshot.json` and
  `discriminator2-post-restore-status.json` record restore of the pre-run
  boot tree.

Classification:
`pi5-psci-started-but-state-or-stack-incomplete`. This rules out the prior
serial capture/staging ambiguity for the current candidate image, but it does
not satisfy the alive-proof acceptance criteria because no secondary core
reported owned stack/state.

## Current Next Action

Continue with a bounded secondary-entry discriminator. The next iteration
should distinguish "PSCI accepted but secondary never branches to the entry
point" from "secondary reaches the trampoline but does not publish state",
without adding scheduler migration, SMP locks, userspace, filesystem,
networking, SSH, or shell behavior.

## Hardware Attempt 3

Added two fixed UART10 markers to the Pi 5 secondary trampoline:
TALOS: secondary_entry at the first instruction after saving the PSCI context,
and TALOS: secondary_pre_rust_entry after selecting the accepted per-core stack
slot and before branching to talos_rpi5_secondary_entry.

The entry-discriminator archive
target/talos-rpi5-psci-secondary-core-alive-entry-discriminator-boot.tar.gz
had archive sha256
1ef6ec1daf33cc99feae786dc2daa765dbff9aa9308edd71b3240f117769df6f,
kernel sha256
5e099ff4e75986cc7043fc196d41565fb9ada25321a8f8386be9c45c7d0931e7,
and kernel size 90,016 bytes.

Local validation for the discriminator rerun:

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- Unit tests: cargo -Zjson-target-spec test passed 96 no_std tests.
- QEMU/substitute: scripts/qemu-secondary-core-discriminator.sh passed.
- QEMU/substitute: scripts/qemu-smoke.sh passed.
- Image/archive inspection:
  scripts/rpi5-psci-secondary-core-alive-image.sh built the focused image.
- Image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-psci-secondary-core-alive-entry-discriminator-boot.tar.gz
  passed with file_count=19, kernel_size=90016, header_image_size=90016,
  text_offset=0, and flags=12.
- fmt/lint/typecheck: scripts/rpi5-format-guard-check.sh and git diff --check
  passed.
- static inspection: mdbook was unavailable in the container.

Hardware evidence files:

- First entry-discriminator run:
  entrydisc-tftp-delta.json, entrydisc-serial-observe.json, and
  entrydisc-restore-pre-snapshot.json. This run restored cleanly but is
  classified as staging-or-serial-window-ambiguity: serial only captured an
  early firmware burst and the initial TFTP cursor was stale/truncated.
- Corrected entry-discriminator run:
  entrydisc2-tftp-delta.json,
  entrydisc2-post-restore-serial-peek-20k.json,
  entrydisc2-restore-pre-snapshot.json, and
  entrydisc2-post-restore-status.json.

Corrected-run result:

- TFTP/archive proof: entrydisc2-tftp-delta.json shows 10.42.1.4 was served
  da591740/kernel_2712.img at 90,016 bytes from the candidate boot tree.
- Serial hardware boot/output: entrydisc2-post-restore-serial-peek-20k.json
  shows asm_start, asm_pre_rust_entry, rust_entry, normal Talos boot reports,
  PSCI CPU_ON result 0 for target affinities 0x100, 0x200, and 0x300, and then
  all three secondary records remaining parked.
- The same serial capture contains no TALOS: secondary_entry or
  TALOS: secondary_pre_rust_entry marker.
- Restore proof: entrydisc2-restore-pre-snapshot.json and
  entrydisc2-post-restore-status.json show the pre-run boot snapshot was
  restored to the prior 82,045-byte kernel tree.

Classification:
pi5-psci-accepted-secondary-entry-not-observed. This is decisive against the
"secondary reaches the trampoline but does not publish state" branch for the
current candidate, but it does not satisfy the alive-proof acceptance criteria.

## Current Next Action

Continue this same task with a bounded PSCI-entry-address or PSCI-state
discriminator. The next iteration should explain why Pi 5 BL31 returns CPU_ON
success for the accepted target affinities without any observable branch to the
provided secondary entry point. Do not add scheduler migration, SMP locks,
userspace, filesystem, networking, SSH, or shell behavior.

## Hardware Attempt 4

Added a bounded PSCI state discriminator to the same focused diagnostic image:

- `PSCI_VERSION` via SMC function `0x84000000`.
- `PSCI_FEATURES` for `CPU_ON` (`0xc4000003`) and `AFFINITY_INFO`
  (`0x84000004`).
- `AFFINITY_INFO` before each `CPU_ON`, immediately after each `CPU_ON`, and
  again while reporting the final per-core state.

The state-discriminator archive
`target/talos-rpi5-psci-secondary-core-alive-state-discriminator-boot.tar.gz`
had archive sha256
`e2e16f292d5f8ad9eff8b139af47f0491d7f9af44397488941ec2dbf8a449bca`,
kernel sha256
`cd90dde7543838ad8f95203b92a0d90914ff62695d7c881d717a1cf8d478d954`,
and kernel size 91,000 bytes.

Local validation for the state discriminator:

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 96 no_std tests.
- QEMU/substitute: `scripts/qemu-secondary-core-discriminator.sh` passed.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- Image/archive inspection:
  `scripts/rpi5-psci-secondary-core-alive-image.sh` built the focused image.
- Image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-psci-secondary-core-alive-state-discriminator-boot.tar.gz`
  passed with file_count=19, kernel_size=91000, header_image_size=91000,
  text_offset=0, and flags=12.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

Hardware evidence files:

- Initial state-discriminator staging attempts:
  `statedisc-*` and `statedisc2-*`. These are classified as
  staging-or-serial-window-ambiguity because the restored 82,045-byte boot tree,
  not the 91,000-byte candidate, was fetched by the Pi before durable candidate
  serial evidence was captured.
- Corrected state-discriminator run:
  `statedisc3-publish.json`, `statedisc3-post-publish-status.json`,
  `statedisc3-power-cycle.json`, `statedisc3-tftp-delta.json`,
  `statedisc3-serial-peek-100k-before-restore.json`,
  `statedisc3-post-restore-serial-peek-500k.json`,
  `statedisc3-restore-pre-snapshot.json`, and
  `statedisc3-post-restore-status.json`.

Corrected-run result:

- TFTP/archive proof: `statedisc3-tftp-delta.json` shows 10.42.1.4 was served
  `da591740/kernel_2712.img` twice at 91,000 bytes from the candidate boot
  tree.
- Serial hardware boot/output:
  `statedisc3-post-restore-serial-peek-500k.json` shows the corrected run early
  firmware output after the candidate fetch, but no BL31 notice, no
  `TALOS: asm_start`, no `TALOS: rust_entry`, and no PSCI state-discriminator
  lines from that boot.
- Restore proof: `statedisc3-restore-pre-snapshot.json` and
  `statedisc3-post-restore-status.json` show the pre-run boot snapshot was
  restored to the prior 82,045-byte kernel tree.

Classification:
`pi5-state-discriminator-candidate-fetched-no-bl31-or-asm-entry`. This
candidate does not satisfy the alive-proof acceptance criteria and does not yet
explain the prior CPU_ON-success/no-secondary-entry result.

## Current Next Action

Continue this same task with a bounded local image/disassembly review and a
smaller PSCI state discriminator if needed. The next iteration should determine
why adding PSCI state queries produced a fetched 91,000-byte image with no
observable BL31/asm entry before attempting another hardware acceptance run.

## Hardware Attempt 5

Local image/disassembly review found the PSCI state-discriminator image was
structurally valid: the ARM64 image header matched the file size, the
`TALOS: asm_start` and PSCI diagnostic strings were present, and the primary
and secondary entry symbols still resolved to the expected low-memory
addresses. The SMC wrapper was tightened to declare the SMCCC caller-clobbered
argument/result registers, and the state discriminator was reduced to a smaller
post-`CPU_ON` `AFFINITY_INFO` probe.

The minimal-state archive
`target/talos-rpi5-psci-secondary-core-alive-minstate-boot.tar.gz` had archive
sha256
`542f3b87302b82a91776f72d0e04408c24cf9680205537acd19447a00e0475dd`,
kernel sha256
`50295ba874792d6e732c2af6b70fdffd708e86847e4b30c0fb873442dd71807f`,
and kernel size 90,416 bytes.

Local validation for the minimal-state discriminator:

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 96 no_std tests.
- QEMU/substitute: `scripts/qemu-secondary-core-discriminator.sh` passed.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- Image/archive inspection:
  `scripts/rpi5-psci-secondary-core-alive-image.sh` built the focused image.
- Image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-psci-secondary-core-alive-minstate-boot.tar.gz` passed
  with file_count=19, kernel_size=90416, header_image_size=90416,
  text_offset=0, and flags=12.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

Hardware evidence files:

- `minstate-publish.json`, `minstate-post-publish-status.json`, and
  `minstate-power-cycle.json`.
- `minstate-tftp-delta.json`.
- `minstate-serial-observe.json`,
  `minstate-serial-observe-second.json`, and
  `minstate-serial-peek-500k-before-restore.json`.
- `minstate-restore-pre-snapshot.json` and
  `minstate-post-restore-status.json`.

Result:

- TFTP/archive proof: `minstate-tftp-delta.json` shows the Pi repeatedly
  fetched `da591740/kernel_2712.img` at 90,416 bytes from the candidate boot
  tree.
- Serial hardware boot/output: serial observed after the run showed firmware
  network/link progress and NUL/newline bytes, but did not show current-run
  BL31 handoff, `TALOS: asm_start`, `TALOS: rust_entry`, or the minimal
  `AFFINITY_INFO` lines after the candidate fetch.
- Restore proof: `minstate-restore-pre-snapshot.json` and
  `minstate-post-restore-status.json` show the pre-run boot snapshot was
  restored.

Classification:
`pi5-minstate-discriminator-candidate-fetched-no-current-entry`. This does not
satisfy the alive-proof acceptance criteria and keeps the original
CPU_ON-success/no-secondary-entry result unexplained.

## Current Next Action

Continue this same task with a bounded comparison between the last entry
discriminator that reached Talos and the state/minstate images that were fetched
without current-candidate entry. The next iteration should isolate whether the
state probe changes the boot image/layout enough to affect firmware handoff, or
whether the lab is observing repeated network-boot retries without a completed
BL31-to-kernel handoff.

## Hardware Attempts 6-8

The comparison rerun of the last known Talos-entry archive
target/talos-rpi5-psci-secondary-core-alive-entry-discriminator-boot.tar.gz
again fetched the 90,016-byte candidate and reached the primary Talos proof.
Serial now showed secondary trampoline markers interleaved with primary CPU_ON
output, but the final reports still read parked with zero MPIDR, affinity,
context, and stack pointer. This reclassified the gap as state
publication/visibility, not PSCI entry-address failure.

A Rust-entry marker discriminator then proved each secondary reached
talos_rpi5_secondary_entry and emitted TALOS: secondary_state_published, while
the primary still reported parked zero state. That localized the remaining
failure to cache visibility of the accepted per-core state records.

The accepted cache-coherent archive
target/talos-rpi5-psci-secondary-core-alive-cachecoherent-boot.tar.gz had
archive sha256
58803e6c4fd21a7c40d2f36245e8e7c366e80ea50dbcdba2afd2952d952c4d22,
kernel sha256
2f1622d7694f84446153240d1136b9f095df0cd09d69e9f0ae88de2ae1ef9996, and
kernel size 90,784 bytes. It cleans secondary per-core state updates to the
point of coherency and invalidates the primary view before proof snapshots.

Local validation for the accepted proof:

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- Unit tests: cargo -Zjson-target-spec test passed 96 no_std tests.
- QEMU/substitute: scripts/qemu-secondary-core-discriminator.sh passed.
- QEMU/substitute: scripts/qemu-smoke.sh passed.
- Image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-psci-secondary-core-alive-cachecoherent-boot.tar.gz
  passed with file_count=19, kernel_size=90784, header_image_size=90784,
  text_offset=0, and flags=12.
- fmt/lint/typecheck: scripts/rpi5-format-guard-check.sh and
  git diff --check passed.
- static inspection: mdbook was unavailable in the container.

Accepted hardware evidence files:

- Archive/staging: cachecoh2-publish.json,
  cachecoh2-post-publish-status.json, and cachecoh2-pre-restore-status.json.
- TFTP proof: cachecoh2-tftp-delta-before-restore.json shows
  da591740/kernel_2712.img served twice at 90,784 bytes before restore.
- Serial proof: cachecoh2-serial-peek-before-restore.json contains secondary
  Rust-entry/state-published markers, distinct MPIDRs 0x81000100,
  0x81000200, and 0x81000300, mapped affinities 0x100, 0x200, and 0x300,
  distinct stack slots, handoff-ready lifecycle for logical cores 1-3,
  classification pi5-psci-smc-secondary-cores-alive, and PASS.
- Restore proof: cachecoh2-restore-pre-snapshot.json and
  cachecoh2-post-restore-status.json restored the pre-run 82,045-byte boot
  tree.

Classification: pi5-psci-smc-secondary-cores-alive.

## Acceptance

Accepted. Pi 5 hardware now proves PSCI CPU_ON starts the three secondary
Cortex-A76 cores. Each reports stable logical identity, MPIDR/affinity,
exclusive stack ownership, per-core state registration, and controlled
handoff-ready parking. This task does not introduce a controlled kernel-thread
workload, SMP-safe locks, scheduler migration, cross-core wakeups, UART
interrupts, EL0, syscalls, descriptors, filesystem, networking, SSH, or shell
behavior.
