# Phase 11 RP1 Rust-Entry UART10 Marker Loop Closeout

Task id: phase11-rp1-rust-entry-uart10-marker-loop-closeout-20260606

Status: accepted

## Goal

Close out the Rust-entry UART10 marker-loop discriminator and decide whether
the existing RP1 UART0 flag-register refresh is mechanically unblocked.

## Scope

- Reconciled the marker-loop source/static candidate and serialized Pi 5
  hardware evidence.
- Recorded findings with disposition.
- Updated the Phase 11 RP1/PCIe map contract and roadmap with the accepted
  boundary.
- Confirmed that only the queued RP1 UART0 FR-read refresh core is unblocked
  next.
- Did not run hardware, publish a boot archive, acquire the hardware lock,
  change kernel/RP1 source, run an RP1 UART0 FR read, or promote any broader
  Phase 11 work.

## Final Classification

Classification: post-handoff-rust-entry-uart10-marker-visible.

The accepted source/static core produced the non-published candidate archive
target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz with SHA-256
ab6de452670427cee2d411cbcd2a92602331e9d03a9d68dae20b75d649d1565b. Static
inspection proves the selected path is _start -> rust_entry ->
run_rust_entry_uart10_marker_loop before BootInfo parsing, target::init, boot
reports, allocator/memory planning, scheduler work, PSCI SYSTEM_RESET, or RP1
UART0 MMIO. The loop emits the compact marker TALOS: reu10-loop through the
existing UART10 early-phase path.

The serialized Pi 5 discriminator published only that archive. Preflight
identity matched selected tree
1d7cdd3d265fb983ec77d9281098d6a920e0bc957a1f0a15f279fe35c618ee6c,
effective kernel kernel_2712.img, and a 45,328-byte
da591740/kernel_2712.img. Stable same-cursor TFTP evidence retained 13 events,
including two served 45,328-byte da591740/kernel_2712.img fetches before
restore. The deadline-looped fresh serial window retained 60,748 bytes over 32
seconds and observed TALOS: reu10-loop 2,961 times. Restore returned the boot
tree to a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

This accepts visible post-handoff Rust-entry UART10 marker observability for
the selected marker-loop candidate only. It is enough to unblock the existing
RP1 UART0 FR-read refresh core, because the blocking prerequisite was visible
Rust-side marker output after rust_entry. It does not accept RP1 mapped/read
behavior, RP1 unmapped/trap behavior, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: source/static evidence proved the selected marker-loop path reaches
  rust_entry and repeats TALOS: reu10-loop before BootInfo parsing,
  target::init, scheduler work, PSCI reset, or RP1 UART0 MMIO.
- fixed: candidate identity is tied to the accepted archive SHA-256
  ab6de452670427cee2d411cbcd2a92602331e9d03a9d68dae20b75d649d1565b, kernel
  SHA-256 6335cc2f229c38258d88000fe968248ca2e47d61e47f874bf246862e0d2b248a,
  and 45,328-byte arm64 Image header.
- fixed: Pi 5 hardware evidence tied the selected tree to stable pre-restore
  TFTP fetches of da591740/kernel_2712.img.
- fixed: fresh deadline-looped serial evidence observed the expected UART10
  marker 2,961 times, accepting visible post-handoff Rust-entry UART10 marker
  observability.
- fixed: restore hygiene and hardware lock release were retained; durable
  state reports the hardware lock unlocked/restored.
- fixed: the existing phase11-rp1-uart0-fr-read-refresh-core-20260606 task is
  now mechanically unblocked for the next worker wake because its closeout
  dependency is satisfied by visible marker evidence.
- deferred: RP1 UART0 FR-read behavior remains untested until the already
  queued refresh core and later serialized proof execute.
- not-an-issue: known-good control and candidate rerun were not required for
  the marker-loop discriminator because the first run had stable candidate
  fetch evidence plus visible marker output.
- not-an-issue: no RP1 mapped/unmapped, GPIO, interrupts, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
  transition behavior is accepted by this closeout.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-closeout/evidence-map.json.
- Source/static core task:
  tasks/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core.md.
- Core evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/evidence-map.json.
- Core static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/static-inspection.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator.md.
- Pi 5 evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/evidence-map.json.
- Pi 5 classification:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/classification.json.
- Relevant commits: bf25fe41eb6ce9e8e40dc807e6a10bce69f01cb4 and
  71aa67e6557527ba30ba0fd404650e6871e19fee.

## Validation

- static evidence inspection: completed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as post-handoff-rust-entry-uart10-marker-visible.

This closeout accepts only visible post-handoff Rust-entry UART10 marker
observability for the selected marker-loop candidate. The existing
phase11-rp1-uart0-fr-read-refresh-core-20260606 task is mechanically
unblocked next. RP1 mapped/read-value behavior, RP1 unmapped/trap behavior,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain
unaccepted.
