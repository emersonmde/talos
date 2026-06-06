# Phase 11 RP1 Rust-Entry UART10 Marker Loop Core

Task id: phase11-rp1-rust-entry-uart10-marker-loop-core-20260606

Status: accepted

## Goal

Add the smallest reset-independent Rust-entry UART10 marker-loop candidate so
the next Pi 5 discriminator can test visible post-handoff serial output before
any return to RP1 UART0 flag-register reads.

## Scope

- Added boot scenario rpi5_rust_entry_uart10_marker_loop.
- Routed the selected Pi 5 scenario directly from rust_entry into an infinite
  UART10 marker loop before BootInfo parsing, target initialization, boot
  reports, memory planning, allocator setup, scheduler work, PSCI SYSTEM_RESET,
  or RP1 UART0 MMIO.
- Added task-owned image, boot-tree, archive, and archive-review helper
  scripts.
- Produced a non-published candidate archive at
  target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz.
- Did not acquire hardwareTestLock, publish a boot archive, power cycle the Pi
  5, or run hardware.

## Candidate

- Archive: target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz
- Archive SHA-256:
  ab6de452670427cee2d411cbcd2a92602331e9d03a9d68dae20b75d649d1565b
- Kernel image:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rust-entry-uart10-marker-loop.img
- Kernel SHA-256:
  6335cc2f229c38258d88000fe968248ca2e47d61e47f874bf246862e0d2b248a
- Kernel size: 45,328 bytes
- Arm64 Image fields: text_offset=0, header_image_size=45328, flags=12
- Marker: TALOS: reu10-loop

## Static Path

Static disassembly proves _start preserves x0, clears BSS, sets the stack, and
branches to rust_entry. For this scenario rust_entry contains only a direct
branch to run_rust_entry_uart10_marker_loop. The marker-loop function writes
the marker string through write_early_static, waits for UART10 empty, and
branches back to repeat the marker.

The selected path reaches no BootInfo::from_aarch64_x0, target::init, boot
reporting, allocator, scheduler, PSCI SYSTEM_RESET, or RP1 UART0 FR-read code
before the repeated marker loop.

## Findings And Disposition

- fixed: added a reset-independent marker visibility discriminator using the
  existing direct UART10 early-phase write path.
- fixed: kept the marker compact and unique enough for the Pi 5 proof script:
  TALOS: reu10-loop.
- fixed: produced a non-published candidate archive and retained archive
  SHA-256, kernel SHA-256, kernel size, and arm64 Image header fields.
- fixed: static disassembly shows _start -> rust_entry ->
  run_rust_entry_uart10_marker_loop, with the loop before BootInfo parsing,
  target::init, boot reports, memory planning, allocator setup, scheduler work,
  PSCI SYSTEM_RESET, or RP1 UART0 MMIO.
- fixed: archive review and string scan show the marker is present and the
  forbidden RP1 UART0 FR-read strings are absent.
- deferred: visible UART10 marker observability remains a hardware question for
  the queued Pi 5 discriminator.
- deferred: RP1 UART0 FR-read readiness remains blocked until the marker-loop
  hardware evidence and closeout accept visible Rust-entry UART10 marker
  observability.
- not-an-issue: no hardware run, boot publication, RP1 MMIO, GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
  11.2, or phase transition is accepted by this core task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/static-inspection.md.
- Archive review:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/archive-review.txt.
- Disassembly:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/disassembly-start.txt,
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/disassembly-rust-entry.txt,
  and
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/disassembly-marker-loop.txt.
- Symbol scan:
  tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/symbols.txt.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- image/archive inspection:
  scripts/rpi5-rust-entry-uart10-marker-loop-review.sh passed.
- static image/header/symbol/disassembly inspection: completed and retained.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as ready-for-rust-entry-uart10-marker-loop-pi5-discriminator.

This task accepts only the source/static candidate and its non-published
archive. It does not accept visible marker serial output, RP1 mapped/read-value
behavior, RP1 unmapped/trap behavior, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition.
