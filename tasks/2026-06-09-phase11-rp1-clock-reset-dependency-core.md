# Phase 11 RP1 Clock/Reset Dependency Core

Task id: phase11-rp1-clock-reset-dependency-core-20260609

Status: accepted

Classification: accepted-local-static-clock-reset-dependency-core

## Goal

Implement the accepted read-only RP1 clock/reset dependency preflight as
local/static real and paired no-MMIO control candidates.

## Scope

- Added the rpi5_rp1_clock_reset_dependency_read real candidate for the
  accepted observed-aperture SYSINFO and clock-manager dependency contract.
- Added the rpi5_rp1_clock_reset_dependency_no_mmio_control paired control
  candidate with matching report shape and not-constructed address fields.
- Added image, boot-tree, archive, and review helpers for both candidates.
- Recorded static/archive evidence and findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication to the lab, hardwareTestLock
acquisition, GPIO/RIO/pad/INTE/CTRL writes, clock/reset writes, IRQRESET,
interrupt unmasking, IAR/EOIR acknowledgement, ISR/handler install, event
generation, interrupt delivery, endpoint config retry, bridge setup write,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

## Findings

- fixed: registered the real and control boot scenarios in build.rs and
  src/main.rs.
- fixed: added observed-aperture SYSINFO and clock-manager constants for the
  exact accepted read set.
- fixed: real candidate performs only the accepted 32-bit volatile loads from
  0x1c00000000, 0x1c00000004, 0x1c00020000, 0x1c00018014,
  0x1c00018018, 0x1c00018020, 0x1c00018024, 0x1c00018054,
  0x1c00018058, and 0x1c00018060.
- fixed: real candidate emits stable task-owned marker, contract id, target,
  raw register fields, decoded booleans, retained GPIO14/GPIO16 and 0x1f
  sentinel context, reset_status_source=none-selected-read-only, and terminal
  classification.
- fixed: paired control emits the same report shape with address=not-constructed
  and classification=no-mmio-clock-reset-dependency-control-visible while
  constructing no forbidden MMIO address.
- fixed: archive review scripts verify real/control identity and reject retained
  write-backed clock/GPIO diagnostic markers.
- deferred: Pi 5 no-MMIO control proof, real Pi 5 preflight, clock/reset
  writes, reset ownership, GPIO function changes, event generation, interrupt
  delivery, DMA/cache, networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: no docs/src update was required because implementation did not
  change accepted report fields or the accepted source contract.

No findings were removed without replacement.

## Candidate Artifacts

Real candidate:

- Archive:
  target/task-evidence/2026-06-09-clock-reset-core/clock-reset-real.tar.gz
- Archive SHA-256:
  a06485f7ed24daaff5b15c794275191c90e2e2c0f677ce4f26b0447446537478
- Kernel SHA-256:
  424bb780ec1c9775cba990bdbe1525c200e0d7e4a27744ece490fc86127cd9bd
- Kernel size: 49,496 bytes
- Marker: TALOS: rp1-clock-reset-dependency-result

Control candidate:

- Archive:
  target/task-evidence/2026-06-09-clock-reset-core/clock-reset-control.tar.gz
- Archive SHA-256:
  b4dbe94bf5872f230f9128f024b71a9e357f6e603a5bc2627c0ad23ede99197a
- Kernel SHA-256:
  aeaae258e6d3de5084a3a6553893a920afb9c079b7311cf5aa6669179a179fe4
- Kernel size: 48,520 bytes
- Marker: TALOS: rp1-clock-reset-dependency-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-core/classification.json.
- Static implementation review:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-core/static-implementation-review.md.
- Real archive review:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-core/control-archive-review.txt.
- Source boot tree:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-core/source-dir.txt.

## Validation

- fmt/lint: cargo fmt --all -- --check passed after formatting.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- shell syntax: bash -n passed for task-owned image, boot-tree, archive, and
  review scripts.
- static/archive inspection:
  scripts/rpi5-rp1-clock-reset-dependency-read-review.sh passed for the real
  archive.
- static/archive inspection:
  scripts/rpi5-rp1-clock-reset-dependency-no-mmio-control-review.sh passed for
  the control archive.
- jq evidence-map/classification checks: passed.
- git diff --check: passed.
- git diff --cached --check: passed before commit.

## Accepted Claims

- The real local/static candidate implements the accepted read-only
  observed-aperture SYSINFO and clock-manager dependency preflight report and
  read set.
- The control local/static candidate preserves output shape without
  constructing forbidden RP1, GPIO, clock/reset, PCIe/MIP, GIC, DMA, or other
  MMIO addresses.

## Rejected Claims And Retained Risks

This task does not accept Pi 5 hardware behavior, live RP1 identity,
clock/reset ownership, clock/reset writes, GPIO ownership, GPIO function
changes, event generation, interrupt delivery, handler ownership, DMA/cache,
networking, SSH, Milestone 11.3, or phase transition.

## Next Action

The next queued worker task is
phase11-rp1-clock-reset-dependency-control-pi5-20260609, mechanically blocked
until this local/static core is committed and hardwareTestLock remains unlocked
and restored.
