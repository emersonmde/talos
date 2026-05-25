# Pi 5 SMP Lock Cache Coherence Proof Evidence

Task: `phase6-pi5-smp-lock-cache-coherence-proof-20260524`

Status: in progress.

## Candidate

- Archive: `target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`
- Archive SHA256:
  `4bf352206804e4093b53c5ef791eaabe83d7850a443e80bda23f98b1cb089616`
- Kernel SHA256:
  `c5a891f3a337e6e0f22f50a5638b188fb68b2bae1f4216352faf1ff909a9e9b1`
- Kernel size: 92,376 bytes
- Archive review: passed, `file_count=19`, `header_image_size=92376`,
  `text_offset=0`, `flags=12`

## Hardware Runs

Evidence directory:
`tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/`

The candidate was published and restored under the hardware lock. The first
two observe attempts collected firmware/TFTP evidence but restored before the
normal Talos print window. The third run slept through the firmware window
before serial collection.

Latest run:

- Publish/status: `rerun3-publish.json`,
  `rerun3-post-publish-status.json`
- Power: `rerun3-power-cycle.json`
- Serial: `rerun3-serial-observe.json`,
  `rerun3-serial-peek-before-restore.json`
- TFTP: `rerun3-tftp-delta-before-restore.json`
- Restore: `rerun3-restore-pre-snapshot.json`,
  `rerun3-post-restore-status.json`

TFTP repeatedly served `da591740/kernel_2712.img` at 92,376 bytes. Fresh
serial did not reach BL31 or Talos `asm_start` for this candidate. Restore
returned the boot tree to the prior 82,045-byte kernel.

Classification:
`pi5-smp-lock-cache-coherence-candidate-fetched-no-talos-entry`.

## Entry Discriminator

The follow-up discriminator compared the accepted secondary-workload image
layout with the SMP lock image and then staged an entry-only image that enables
`TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF` plus
`TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_ENTRY_DISCRIMINATOR`.

Layout comparison:

- Accepted secondary workload: 91,288-byte image, entry point `0x200000`,
  executable segment size `0xea2c`, rodata segment size `0x7a68`,
  `.bss` at `0x217000..0x21c000`.
- SMP lock proof candidate: 92,376-byte image, entry point `0x200000`,
  executable segment size `0xed38`, rodata segment size `0x7b98`,
  `.bss` at `0x217000..0x21c000`.
- Entry-only discriminator: 87,792-byte image with the dispatch and
  discriminator strings present in the image.

Latest entry-only hardware attempt:

- Archive: `target/talos-rpi5-smp-lock-cache-coherence-entry-discriminator-boot.tar.gz`
- Archive SHA256:
  `853bc6665746b630bce4962c6014739b49abc0acddd2ecd1ac5b63e6379794ca`
- Kernel SHA256:
  `81e3ae73c29c6f1d7a2afa1e05975dc8aeaf169c092d7838a0839431f031561c`
- Kernel size: 87,792 bytes
- Publish/status: `entrydisc5-publish.json`,
  `entrydisc5-post-publish-status.json`
- TFTP: `entrydisc5-tftp-delta-before-restore.json` shows
  `da591740/kernel_2712.img` fetched twice at 87,792 bytes.
- Serial: `entrydisc5-serial-observe-after-restore.json` contains only two
  bytes after the pre-run drain cursor; no Talos dispatch or discriminator
  marker was observed.
- Restore: `entrydisc5-restore-pre-snapshot.json`,
  `entrydisc5-post-restore-status.json` restored the 82,045-byte pre-run
  boot tree.

Current classification:
`pi5-smp-lock-entry-discriminator-candidate-fetched-no-dispatch-serial`.

## Early Entry Discriminator Rerun

The next static/code-path discriminator moved the entry-only markers earlier
without touching the spin-lock state:

- build.rs now passes
  -DTALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_ENTRY_DISCRIMINATOR to the assembly
  build.
- _start emits
  rpi5-smp-lock-cache-coherence: entry-discriminator asm-start immediately
  after the generic TALOS: asm_start marker.
- rust_entry emits
  rpi5-smp-lock-cache-coherence: entry-discriminator rust-entry before
  BootInfo parsing.
- kernel_main emits the existing
  rpi5-smp-lock-cache-coherence: kernel-main-dispatch marker at function
  entry, before DTB and memory reporting.

Local image/archive inspection:

- Archive:
  target/talos-rpi5-smp-lock-cache-coherence-entry-discriminator-boot.tar.gz
- Archive SHA256:
  ef2bc1f8548d5bc77a4171ac3339caccfde4cfc3a71da7488d101e35c6dd9374
- Kernel SHA256:
  d95b2a0a0d2525dee33b3b5e7346f14d7bde4dee06c0d61977d2400e1aeeee53
- Kernel size: 87,888 bytes
- Archive review: passed, file_count=19, header_image_size=87888,
  text_offset=0, flags=12
- String inspection confirmed the assembly, Rust-entry, kernel-main, and
  entry-discriminator PASS markers are present in the candidate image.

Hardware rerun:

- Publish/status: entrydisc7-publish.json,
  entrydisc7-post-publish-status.json
- Power: entrydisc7-power-cycle.json
- TFTP before restore:
  entrydisc7-recovery-tftp-delta-before-restore.json
- Serial: entrydisc7-serial-observe.json,
  entrydisc7-recovery-serial-peek-before-restore.json
- Restore: entrydisc7-restore-pre-snapshot.json,
  entrydisc7-post-restore-status.json

The recovery TFTP query captured before restore shows
da591740/kernel_2712.img served twice at 87,888 bytes. The pre-run boot
snapshot was restored to the prior 82,045-byte boot tree. Cursor-based serial
observe after the delayed pickup captured only two bytes (NUL and e), while
the non-cursor serial peek mixed older normal Talos output with the current
firmware/TFTP window. No reliable candidate serial marker was captured.

Current discriminator classification:
`pi5-smp-lock-entry-discriminator-candidate-fetched-serial-cursor-inconclusive`.

## Supervisor Intervention Reframe

The hardware proof is paused until the next run uses a decisive discriminator.
The current evidence proves candidate staging and TFTP fetches, but it does not
prove candidate execution or SMP lock behavior.

- First reliable invariant: for the 87,888-byte early-entry candidate, a
  cursor-valid transcript should show the assembly `_start` discriminator
  marker immediately after `TALOS: asm_start` if the fetched kernel executes.
  Later Rust-entry, kernel-main, and PASS markers are subordinate evidence.
- Cursor-valid observations: `rerun3-serial-observe.json`,
  `entrydisc5-serial-observe-after-restore.json`,
  `entrydisc6-serial-observe*.json`, and `entrydisc7-serial-observe.json`
  captured only tiny firmware/byte fragments, not candidate Talos markers.
- Stale/non-cursor observations: `rerun3-serial-peek-before-restore.json`,
  `entrydisc5-serial-peek-before-restore.json`,
  `entrydisc6-serial-peek-before-restore.json`, and
  `entrydisc7-recovery-serial-peek-before-restore.json` contain ordinary
  Talos output mixed with current firmware windows. They are not acceptance
  evidence for the candidate.
- Contradiction: TFTP shows the candidate sizes were served, while reliable
  serial does not show candidate entry. This leaves serial cursor/windowing,
  fetch-without-execute, and early candidate fault all open.

Selected discriminator: run an A/B known-good control before more marker-only
candidate changes. Publish the accepted secondary-core workload archive under
the hardware lock and capture it with an immediate cursor-based serial read
loop. If that known-good archive is not cursor-visible, classify the current
blocker as serial-capture/windowing. If it is visible, rerun the early-entry
lock discriminator with the same observe loop to classify candidate
non-execution versus early boot fault.

Quarantine plan: entry-discriminator build flags, scripts, and markers remain
temporary evidence scaffolding. Remove them before accepting a passing
lock-contention proof unless the final task result is a documented blocker that
depends on those artifacts.

## A/B Known-Good Control

The next hardware run used an immediate cursor-based serial read loop from a
fresh pre-run drain cursor.

- Known-good control archive:
  `target/talos-rpi5-secondary-core-workload-boot.tar.gz`
- Archive SHA256:
  `73e7419eef2ddc0e5ba6a4ac3756d5c0b1d0c2f5b6888b7759b9b921f6621fa7`
- Kernel size: 91,288 bytes
- Serial:
  `abcontrol-secondary-workload-serial-combined.txt`,
  `abcontrol-secondary-workload-serial-loop-result.json`
- Restore:
  `abcontrol-secondary-workload-restore-pre-snapshot.json`,
  `abcontrol-secondary-workload-post-restore-status.json`

The control transcript is cursor-valid and includes `TALOS: asm_start`,
the secondary-workload start line, logical cores 1, 2, and 3 reporting
`progress=64 target=64 ok=true`, classification
`pi5-secondary-core-controlled-workload-complete`, and
`rpi5-secondary-core-workload: PASS`.

Control classification:
`pi5-serial-capture-health-proven-by-known-good-control`.

The early-entry lock discriminator then used the same loop.

- Entry-discriminator archive:
  `target/talos-rpi5-smp-lock-cache-coherence-entry-discriminator-boot.tar.gz`
- Archive SHA256:
  `ef2bc1f8548d5bc77a4171ac3339caccfde4cfc3a71da7488d101e35c6dd9374`
- Kernel size: 87,888 bytes
- Serial:
  `ablock-entrydisc-serial-combined.txt`,
  `ablock-entrydisc-serial-loop-result.json`
- TFTP/archive proof:
  `ablock-entrydisc-tftp-delta-before-restore.json`
- Restore:
  `ablock-entrydisc-restore-pre-snapshot.json`,
  `ablock-entrydisc-post-restore-status.json`

The entry-discriminator transcript reached `TALOS: asm_start`, normal Talos
boot, `kernel_main`, and fragmented `rpi5-smp-lock-cache-coherence` marker
output. This proves the candidate executed under the improved observation
method even though early UART marker interleaving prevented exact full-string
matching.

Entry-discriminator classification:
`pi5-smp-lock-entry-discriminator-executed-serial-fragmented`.

## Full Lock Proof Rerun

The full lock proof was rerun after the A/B discriminator ruled out the
earlier serial-windowing and fetched-but-not-run classifications.

- Archive:
  `target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`
- Archive SHA256:
  `4bf352206804e4093b53c5ef791eaabe83d7850a443e80bda23f98b1cb089616`
- Kernel size: 92,376 bytes
- Serial:
  `lockproof-clean-serial-combined.txt`,
  `lockproof-clean-serial-loop-result.json`
- TFTP/archive and restore:
  `lockproof-clean-tftp-delta-before-restore.json`,
  `lockproof-clean-serial-peek-before-restore.json`,
  `lockproof-clean-restore-pre-snapshot.json`,
  `lockproof-clean-post-restore-status.json`

Cursor-valid serial reached `TALOS: asm_start`, normal Talos boot, the full
lock proof start line, logical-1 CPU_ON, secondary-entry/Rust-entry/state
publish interleaving for logical core 1, and
`affinity-after logical=1 ... state=on raw=0`. It reached the logical-2
CPU_ON/secondary-entry region but did not reach per-core lock reports, final
counter, PASS, or FAIL before the observation loop hung. The pre-run
82,045-byte boot snapshot was restored.

Current classification:
`pi5-smp-lock-cache-coherence-secondary-lock-workload-stall`.

The leading hypothesis is no longer serial capture or candidate execution.
The likely fault boundary is the full lock workload's shared atomic state:
the boot CPU is running with caches enabled while secondary cores enter the
proof without the same cache/MMU regime. The previously accepted controlled
workload used explicit per-core clean/invalidate handoff and did not contend
on shared `SpinLock<T>` state.

## Local Validation

- `cargo fmt --all -- --check`: passed
- `cargo -Zjson-target-spec test`: passed, 102 no_std tests
- `scripts/qemu-smoke.sh`: passed
- `scripts/qemu-smp-lock-contention-smoke.sh`: passed
- `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`: passed
- `scripts/rpi5-archive-review.sh target/talos-rpi5-secondary-core-workload-boot.tar.gz`: passed
- `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-entry-discriminator-boot.tar.gz`: passed
- `git diff --check`: passed
- `mdbook build`: not run because `mdbook` is unavailable

## Lock/Cache-Regime Discriminator

The next discriminator added diagnostic state outside the generic lock:
secondary cores publish phase, progress, lock attempts, timeout count, release
count, and `SCTLR_EL2`; the boot CPU emits wait observations before the final
invariant report. This was intended to distinguish pre-lock stall, lock-held
stall, non-visible lock progress, and invalid mixed cache/MMU state.

Local gates after this discriminator:

- `cargo fmt --all -- --check`: passed
- `cargo -Zjson-target-spec test`: passed, 102 no_std tests
- `scripts/qemu-smoke.sh`: passed
- `scripts/qemu-smp-lock-contention-smoke.sh`: passed
- `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`: passed

Hardware run `lockproof-diag2`:

- Archive SHA256:
  `aece5aa77113905e5b48404d126d693340010f97ab4c16df998ce1460f43170a`
- Kernel SHA256:
  `25ec4753a89987af5d3637cf79cd2ec5fc3aab95e80995cd6ba18e23089484f9`
- Kernel size: 95,608 bytes
- Serial:
  `lockproof-diag2-serial-combined.txt`,
  `lockproof-diag2-serial-loop-result.json`
- TFTP/archive and restore:
  `lockproof-diag2-tftp-delta-before-restore.json`,
  `lockproof-diag2-restore-pre-snapshot.json`,
  `lockproof-diag2-post-restore-status.json`

The TFTP delta captured `da591740/kernel_2712.img` served twice at
95,608 bytes before restore. Cursor-valid serial reached the lock proof start
line, CPU_ON for logical cores 1, 2, and 3, secondary Rust/state-publish
interleaving, and wait observations. The wait observations show the boot CPU
running with `boot-sctlr-el2=0x0000000030c51835` and
`boot-cacheable-mmu=true`, while all three secondaries report
`diag-sctlr-el2=0x0000000030c50830`, `diag-cacheable-mmu=false`,
`diag-phase=before-lock-attempt`, `diag-progress=0`, `diag-attempts=0`, and
`diag-releases=0`.

Current classification:
`pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime`.

The pre-run 82,045-byte boot snapshot was restored. This is not a passing
hardware proof for the generic lock; it decisively classifies the current
physical proof setup as invalid until secondary cores enter the same cacheable
MMU regime as the boot CPU or the supervisor closes the task as a documented
cache-regime blocker.

## Post-Handoff Report Invariant Failure

The follow-up secondary cacheable-MMU handoff proof is accepted separately in
`tasks/2026-05-25-phase6-secondary-cacheable-mmu-handoff-pi5-proof.md`.
That run proves the mixed-cache/MMU blocker is removed, but it does not accept
this lock proof.

Evidence:
`tasks/evidence/2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof/serial-key-lines.txt`.

Observed facts from the post-handoff run:

- Boot CPU and all three secondary diagnostics used
  `diag-sctlr-el2=0x0000000030c51835` with `diag-cacheable-mmu=true`.
- The shared lock state completed:
  `counter=192 expected=192 participants=3 diag-participants=3 errors=0`,
  `lock-available=true`, `generic-state-visible=true`, and
  `mixed-cache-mmu=false`.
- Logical cores 1 and 2 reported `lock-count=64`, `progress=64`,
  `diag-progress=64`, `diag-attempts=64`, and `diag-releases=64`, but
  their final identity fields were zero:
  `context=0 mpidr=0x0000000000000000 affinity=0x0 sp=0x0000000000000000`.
- Logical core 3 reported the expected identity and stack ownership fields.

Current classification:
`pi5-smp-lock-cache-coherence-invariant-failed`.

The report-invariant inventory in
`tasks/2026-05-25-phase6-smp-lock-evidence-hygiene-and-report-inventory.md`
narrows the next implementation surface to `PerCoreState` identity
publication/reset/cache-maintenance around secondary entry and cacheable-MMU
handoff. It does not identify a generic `SpinLock<T>` contract bug.
