# Phase 6 Pi 5 SMP Lock Cache Coherence Proof

Task: `phase6-pi5-smp-lock-cache-coherence-proof-20260524`

Status: accepted through
`phase6-pi5-smp-lock-cache-coherence-final-proof-20260525`.

## Scope

This task adds a physical Pi 5 validation surface for the accepted
`SpinLock<T>` primitive. The diagnostic is intentionally bounded: it starts
secondary cores through the accepted PSCI/trampoline path, has logical cores
1, 2, and 3 contend on the accepted spin lock for a fixed counter invariant,
and reports per-core participation, final counter, error count,
classification, and PASS/FAIL.

It does not add scheduler migration, shared run queues, IPIs, cross-core
wakeups, UART interrupts, userspace, descriptors, filesystem, networking,
SSH, shell behavior, RP1/PCIe, or DMA ownership.

## Implementation

- Added `TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF`.
- Added `scripts/rpi5-smp-lock-cache-coherence-image.sh`.
- Added `scripts/rpi5-smp-lock-cache-coherence-boot-tree.sh`.
- Reused the accepted Pi 5 PSCI secondary entry and per-core stack layout.
- Added a Pi 5 diagnostic-only shared `SpinLock<SmpLockContentionState>`
  with the same target invariant as the accepted QEMU contention smoke:
  `counter=192 expected=192 participants=3 errors=0`.

## Current Evidence

Candidate archive:

- Archive: `target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`.
- Archive SHA256:
  `4bf352206804e4093b53c5ef791eaabe83d7850a443e80bda23f98b1cb089616`.
- Kernel SHA256:
  `c5a891f3a337e6e0f22f50a5638b188fb68b2bae1f4216352faf1ff909a9e9b1`.
- Kernel size: 92,376 bytes.
- Archive review passed with `file_count=19`, `header_image_size=92376`,
  `text_offset=0`, and `flags=12`.

Local validation:

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 102 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed with
  `counter=192 expected=192 participants=3 errors=0` and classification
  `qemu-smp-lock-contention-complete`.
- image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz` passed.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` is unavailable in the container.

Hardware iteration:

- Hardware lock was acquired for
  `tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/`.
- The archive was published and restored repeatedly under named pre-run
  snapshots.
- TFTP evidence from the latest run
  `rerun3-tftp-delta-before-restore.json` shows repeated fetches of
  `da591740/kernel_2712.img` at 92,376 bytes.
- Fresh serial evidence from `rerun3-serial-observe.json` did not reach BL31
  or Talos `asm_start`; the candidate was restored to the 82,045-byte pre-run
  boot tree in `rerun3-post-restore-status.json`.

Current classification:
`pi5-smp-lock-cache-coherence-candidate-fetched-no-talos-entry`.

Entry-discriminator iteration:

- Static inspection compared accepted secondary-workload and lock-coherence
  image layouts. Both keep entry point `0x200000` and `.bss` in the same
  `0x217000..0x21c000` region; the lock candidate is 1,088 bytes larger.
- Added an entry-only discriminator mode that compiles the SMP lock cfg and
  secondary-entry surface but returns before resetting secondary state or
  touching the spin-lock state.
- Latest entry-only archive
  `target/talos-rpi5-smp-lock-cache-coherence-entry-discriminator-boot.tar.gz`
  has SHA256
  `853bc6665746b630bce4962c6014739b49abc0acddd2ecd1ac5b63e6379794ca`;
  kernel SHA256
  `81e3ae73c29c6f1d7a2afa1e05975dc8aeaf169c092d7838a0839431f031561c`;
  kernel size 87,792 bytes.
- `entrydisc5-tftp-delta-before-restore.json` shows the Pi fetched
  `da591740/kernel_2712.img` twice at 87,792 bytes.
- `entrydisc5-serial-observe-after-restore.json` captured only two bytes
  after the pre-run drain cursor, so the dispatch/discriminator serial marker
  was not observed.
- Restore evidence in `entrydisc5-post-restore-status.json` shows the
  82,045-byte pre-run boot tree restored.

Current discriminator classification:
`pi5-smp-lock-entry-discriminator-candidate-fetched-no-dispatch-serial`.

Final proof follow-up:

- The supervisor-planned final proof task
  `phase6-pi5-smp-lock-cache-coherence-final-proof-20260525` accepted the
  physical Pi 5 lock/cache-coherence proof after the secondary cacheable-MMU
  handoff and report-invariant correction.
- Evidence:
  `tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/`.
- Archive SHA256:
  `73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`.
- Kernel SHA256:
  `e28596b5f259775c4c239c3e18b57e3d61d24ff453aa3c762c879e38075f7278`.
- Serialized Pi 5 serial output reports boot CPU and logical cores 1, 2, and 3
  in the accepted cacheable-MMU regime, per-core `ok=true` reports, final
  `counter=192 expected=192 participants=3 errors=0`,
  `mixed-cache-mmu=false`,
  `classification=pi5-smp-lock-cache-coherence-complete`, and `PASS`.
- The pre-run boot snapshot was restored after the hardware run.

Early-entry discriminator rerun:

- Moved the entry-only marker earlier in the candidate path:
  assembly _start, Rust rust_entry, and immediate kernel_main.
- Rebuilt and reviewed the entry-only archive
  target/talos-rpi5-smp-lock-cache-coherence-entry-discriminator-boot.tar.gz.
- Archive SHA256:
  ef2bc1f8548d5bc77a4171ac3339caccfde4cfc3a71da7488d101e35c6dd9374.
- Kernel SHA256:
  d95b2a0a0d2525dee33b3b5e7346f14d7bde4dee06c0d61977d2400e1aeeee53.
- Kernel size: 87,888 bytes.
- Local validation passed: cargo fmt --all -- --check,
  cargo -Zjson-target-spec test with 102 no_std tests,
  scripts/qemu-smoke.sh, scripts/qemu-smp-lock-contention-smoke.sh,
  scripts/rpi5-archive-review.sh on the entry-only archive,
  string inspection, and git diff --check; mdbook is unavailable.
- Hardware rerun entrydisc7 published the candidate and restored the
  pre-run snapshot. TFTP evidence captured before restore shows
  da591740/kernel_2712.img served twice at 87,888 bytes.
- Cursor-based serial observe after the delayed pickup captured only two bytes;
  the non-cursor serial peek mixed older normal Talos output with the current
  firmware/TFTP window, so no reliable candidate serial marker was captured.

Current discriminator classification:
`pi5-smp-lock-entry-discriminator-candidate-fetched-serial-cursor-inconclusive`.

## Supervisor Intervention Reframe

The failure must be separated into four layers:

- Staging/fetch: archive review and TFTP evidence show the lab served the
  intended candidate sizes: 92,376 bytes for the full lock proof, 87,792 bytes
  for the first entry-only image, and 87,888 bytes for the early-entry image.
  Restore evidence returned the boot tree to the pre-run 82,045-byte image.
- Serial capture: cursor-based observes after delayed pickup often returned
  only two bytes. Non-cursor peeks include old normal Talos output mixed with
  the current firmware window, so they are useful for suspicion only, not for
  acceptance.
- Boot entry: no cursor-valid candidate transcript has reached BL31, the
  early assembly marker, Rust entry, kernel-main dispatch, or the
  entry-discriminator PASS marker for the lock-proof candidate.
- SMP lock behavior: no Pi 5 evidence has reached the secondary CPU_ON,
  per-core contention, final counter, or error-count surface. Lock behavior is
  therefore not yet tested on hardware.

For a fetched candidate archive, the invariant is: if firmware executes the
served `kernel_2712.img`, the first reliable candidate-origin marker must be
the entry-discriminator assembly `_start` marker immediately after the generic
`TALOS: asm_start` marker. The Rust-entry and kernel-main markers are later
checks. A TFTP `kernel_2712.img` fetch proves staging and transfer, not
execution.

Evidence review:

- `rerun3`: TFTP repeatedly served the 92,376-byte full candidate. The
  cursor observe captured only a NUL byte. The non-cursor peek contained BL31
  and normal Talos output, but it also had no candidate run marker and was not
  cursor-valid.
- `entrydisc5`: TFTP served the 87,792-byte entry-only candidate twice. The
  post-restore observe captured only NUL/`e`. The peek showed ordinary Talos
  output without the discriminator marker, but it was not cursor-valid.
- `entrydisc6`: the useful serial files show firmware bytes after restore and
  stale ordinary Talos output in peeks, while the TFTP delta does not establish
  a candidate fetch. It is not decisive for candidate execution.
- `entrydisc7`: the initial TFTP delta observed an 82,045-byte restore image,
  while the recovery TFTP delta observed the 87,888-byte early-entry candidate.
  Cursor serial again captured only NUL/`e`. Non-cursor recovery peeks mixed
  ordinary Talos output with a current firmware window, so they contradict the
  candidate-fetch evidence but are stale/non-cursor, not acceptance evidence.

Unproven assumptions:

- A pre-run drain cursor plus a delayed observe reliably brackets the current
  boot; current evidence suggests the window can be missed.
- A candidate TFTP fetch necessarily reaches ARM execution; it may only prove
  firmware transfer before retry, reset, or fallback.
- The SMP lock image is boot-compatible just because its header and layout are
  close to the accepted workload image.
- Marker ordering is intact under all cache/MMU and UART-preservation states;
  only the static image inspection proves the strings are present.

Two qualitatively different next approaches are available:

- A/B known-good control: publish the accepted secondary-core workload archive
  and use the same cursor-drain plus immediate short-read loop planned for the
  lock proof. If the accepted workload also fails to produce cursor-valid
  Talos markers, classify the problem as serial-capture/windowing. If the
  accepted workload is visible and the lock entry image is not, classify it as
  candidate-not-executed or early candidate fault.
- Static/image bisection: keep hardware idle and compare the accepted workload
  image against progressively smaller lock-proof images by build flags,
  symbols, entry path, and archive headers. This can identify an image/layout
  incompatibility before another hardware run, but it cannot by itself prove
  serial capture health.

The smallest decisive discriminator is the A/B known-good control with an
immediate serial read loop from a fresh pre-run cursor. It changes the
observation method and the archive under test, not just marker text. It can
classify serial-capture failure versus candidate-specific non-execution before
any more lock-contention work.

Temporary entry-discriminator flags, scripts, and markers are quarantined as
evidence-only scaffolding. They must not be part of the accepted production
lock-proof commit unless the final classification explicitly depends on them.
Before acceptance, remove the entry-discriminator build flag, early marker
emissions, and `rpi5-smp-lock-cache-coherence-entry-discriminator-*` scripts,
or move the rationale into this task record if the task is closed as a
decisive blocker instead of a passing proof.

## Next Action

## A/B Serial Discriminator

The A/B known-good control used an immediate cursor-based serial read loop
instead of the delayed observe used by the earlier inconclusive runs.

- Control archive:
  `target/talos-rpi5-secondary-core-workload-boot.tar.gz`, SHA256
  `73e7419eef2ddc0e5ba6a4ac3756d5c0b1d0c2f5b6888b7759b9b921f6621fa7`,
  kernel size 91,288 bytes.
- Evidence:
  `abcontrol-secondary-workload-serial-combined.txt`,
  `abcontrol-secondary-workload-serial-loop-result.json`,
  `abcontrol-secondary-workload-tftp-delta-before-restore.json`, and
  `abcontrol-secondary-workload-post-restore-status.json`.
- Result: cursor-valid serial captured `TALOS: asm_start`, the accepted
  secondary workload start line, reports for logical cores 1, 2, and 3 with
  `progress=64 target=64 ok=true`, classification
  `pi5-secondary-core-controlled-workload-complete`, and
  `rpi5-secondary-core-workload: PASS`.

Classification: `pi5-serial-capture-health-proven-by-known-good-control`.

The early-entry lock discriminator then used the same read loop.

- Archive:
  `target/talos-rpi5-smp-lock-cache-coherence-entry-discriminator-boot.tar.gz`,
  SHA256
  `ef2bc1f8548d5bc77a4171ac3339caccfde4cfc3a71da7488d101e35c6dd9374`,
  kernel size 87,888 bytes.
- Evidence:
  `ablock-entrydisc-serial-combined.txt`,
  `ablock-entrydisc-serial-loop-result.json`,
  `ablock-entrydisc-tftp-delta-before-restore.json`, and
  `ablock-entrydisc-post-restore-status.json`.
- Result: cursor-valid serial captured `TALOS: asm_start`, early
  `rpi5-smp-lock-cache-coherence` discriminator fragments, normal Talos
  boot output, and `kernel_main`. The full discriminator strings were
  fragmented by early UART interleaving, but candidate execution was proven.

Classification:
`pi5-smp-lock-entry-discriminator-executed-serial-fragmented`.

## Full Lock Proof Reruns

After the A/B discriminator proved serial capture health and candidate
execution, the full lock proof was rerun with the same immediate read-loop
method.

- Archive: `target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`,
  SHA256
  `4bf352206804e4093b53c5ef791eaabe83d7850a443e80bda23f98b1cb089616`,
  kernel size 92,376 bytes.

- Evidence:
  `lockproof-clean-serial-combined.txt`,
  `lockproof-clean-serial-loop-result.json`,
  `lockproof-clean-tftp-delta-before-restore.json`,
  `lockproof-clean-serial-peek-before-restore.json`, and
  `lockproof-clean-post-restore-status.json`.
- Result: cursor-valid serial captured `TALOS: asm_start`, normal boot,
  the full lock proof start line, CPU_ON for logical core 1, a visible
  secondary entry/Rust-entry/state-publish interleave for logical core 1, and
  `affinity-after logical=1 ... state=on raw=0`. The transcript then reached
  the logical-2 CPU_ON/secondary-entry region but did not reach per-core
  workload reports, final invariant, PASS, or FAIL before the observation
  loop hung and the pre-run boot snapshot was restored.

Classification:
`pi5-smp-lock-cache-coherence-secondary-lock-workload-stall`.

The A/B result rules out the earlier serial-windowing and fetched-but-not-run
classifications for the current archive shape. The remaining problem is inside
the full lock workload after secondary-core entry begins. The leading working
hypothesis is that the proof mixes boot-CPU cache-enabled shared
`SpinLock<T>` state with secondary cores that have not joined the same
cache/MMU regime; the accepted controlled workload only used explicit
per-core state clean/invalidate handoff and did not contend on shared atomic
state.

## Next Action

## Lock/Cache-Regime Discriminator

The follow-up discriminator made the full lock proof bounded and observable
before another hardware run:

- Secondary cores publish diagnostic phase, progress, lock attempts, release
  count, timeout count, and `SCTLR_EL2` through explicit PoC
  clean/invalidate handoff separate from the generic `SpinLock<T>`.
- The secondary lock workload now uses bounded `try_lock()` attempts, so the
  diagnostic can distinguish pre-lock stall, lock-held stall, non-visible
  generic lock progress, and lock-acquire timeout instead of spinning
  forever.
- The boot CPU prints wait observations before the final invariant report,
  including whether each secondary is in the same cacheable MMU regime.

Local validation for the discriminator:

- `cargo fmt --all -- --check`: passed.
- `cargo -Zjson-target-spec test`: passed with 102 no_std tests.
- `scripts/qemu-smoke.sh`: passed.
- `scripts/qemu-smp-lock-contention-smoke.sh`: passed.
- `scripts/rpi5-archive-review.sh
  target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`: passed for the
  95,608-byte discriminator archive.

Hardware rerun `lockproof-diag2`:

- Archive SHA256:
  `aece5aa77113905e5b48404d126d693340010f97ab4c16df998ce1460f43170a`.
- Kernel SHA256:
  `25ec4753a89987af5d3637cf79cd2ec5fc3aab95e80995cd6ba18e23089484f9`.
- Kernel size: 95,608 bytes.
- TFTP before restore served `da591740/kernel_2712.img` twice at
  95,608 bytes.
- Cursor-valid serial reached the lock proof start line, CPU_ON for logical
  cores 1, 2, and 3, secondary Rust/state-publish interleaving, and wait
  observations for all three secondaries.
- The first wait observations show the boot CPU at
  `boot-sctlr-el2=0x0000000030c51835` with `boot-cacheable-mmu=true`, while
  logical cores 1, 2, and 3 report
  `diag-sctlr-el2=0x0000000030c50830` and `diag-cacheable-mmu=false`, with
  `diag-phase=before-lock-attempt`, `diag-progress=0`, `diag-attempts=0`,
  and `diag-releases=0`.
- The pre-run 82,045-byte boot snapshot was restored.

Classification:
`pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime`.

This classifies the current Pi 5 lock proof as invalid before claiming
physical cache/coherence behavior for the generic primitive. The task should
not be accepted as a passing hardware proof until the supervisor either plans
a bounded secondary-cache/MMU handoff task or closes this task as a documented
hardware/cache-regime blocker. Keep the entry-discriminator scaffolding
quarantined as evidence-only until that planning decision is made.

## Post-Handoff Status

The secondary cacheable-MMU handoff prerequisite was accepted in
`tasks/2026-05-25-phase6-secondary-cacheable-mmu-handoff-pi5-proof.md`, but
this lock proof remains unaccepted.

Post-handoff Pi 5 evidence reached the shared lock invariant:
`counter=192 expected=192 participants=3 diag-participants=3 errors=0`,
`lock-available=true`, `generic-state-visible=true`, and
`mixed-cache-mmu=false`. It still failed the final report invariant because
logical cores 1 and 2 reported complete lock progress while their final
`PerCoreState` identity fields were zeroed:
`context=0 mpidr=0x0000000000000000 affinity=0x0 sp=0x0000000000000000`.

Current classification:
`pi5-smp-lock-cache-coherence-invariant-failed`.

The bounded inventory task
`tasks/2026-05-25-phase6-smp-lock-evidence-hygiene-and-report-inventory.md`
reconciles the evidence and recommends targeting `PerCoreState` identity
publication/reset/cache-maintenance after secondary cacheable-MMU handoff,
without broadening into scheduler or generic lock contract work.
