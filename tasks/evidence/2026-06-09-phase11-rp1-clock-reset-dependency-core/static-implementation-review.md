# Clock/Reset Dependency Core Static Review

Task: phase11-rp1-clock-reset-dependency-core-20260609

Evidence level: static implementation and archive inspection.

## Reviewed Files

- build.rs
- src/main.rs
- src/target/rpi5.rs
- scripts/rpi5-rp1-clock-reset-dependency-read-image.sh
- scripts/rpi5-rp1-clock-reset-dependency-read-boot-tree.sh
- scripts/rpi5-rp1-clock-reset-dependency-read-archive.sh
- scripts/rpi5-rp1-clock-reset-dependency-read-review.sh
- scripts/rpi5-rp1-clock-reset-dependency-no-mmio-control-image.sh
- scripts/rpi5-rp1-clock-reset-dependency-no-mmio-control-boot-tree.sh
- scripts/rpi5-rp1-clock-reset-dependency-no-mmio-control-archive.sh
- scripts/rpi5-rp1-clock-reset-dependency-no-mmio-control-review.sh

## Findings

- fixed: registered two bounded boot scenarios for the accepted source
  contract: rpi5_rp1_clock_reset_dependency_read and
  rpi5_rp1_clock_reset_dependency_no_mmio_control.
- fixed: added observed-aperture SYSINFO and clock-manager constants for only
  the accepted read set: 0x1c00000000, 0x1c00000004, 0x1c00020000,
  0x1c00018014, 0x1c00018018, 0x1c00018020, 0x1c00018024,
  0x1c00018054, 0x1c00018058, and 0x1c00018060.
- fixed: the real candidate performs only 32-bit volatile loads from the
  accepted observed-aperture read addresses and emits the accepted contract id,
  target, raw fields, decoded booleans, retained context strings,
  reset_status_source=none-selected-read-only, and classification vocabulary.
- fixed: the paired no-MMIO control preserves output shape with
  address=not-constructed fields, simulated zero raw values, and
  classification=no-mmio-clock-reset-dependency-control-visible while
  constructing no RP1, GPIO, clock/reset, PCIe/MIP, GIC, DMA, or other MMIO
  address.
- fixed: archive review scripts distinguish real and control candidates, check
  the task-owned markers and report fields, and reject retained write-backed
  clock/GPIO diagnostic markers.
- deferred: Pi 5 no-MMIO control proof, real Pi 5 preflight, clock/reset
  writes, reset ownership, GPIO function changes, event generation, interrupt
  delivery, DMA/cache, networking, SSH, Milestone 11.3, and phase transition
  remain future tasks.
- not-an-issue: no docs/src update was required because the implementation
  follows the accepted report fields without changing the source contract.

No findings were removed without replacement.

## Static Claim Boundary

Accepted here:

- local/static real candidate implements the accepted read-only contract.
- local/static control candidate preserves shape without forbidden MMIO
  construction.
- archive/review helpers can produce and inspect both candidates.

Not accepted here:

- Pi 5 hardware behavior.
- live RP1 identity or clock/reset ownership.
- clock/reset writes, GPIO ownership, event generation, interrupt delivery,
  DMA/cache, networking, SSH, Milestone 11.3, or phase transition.
