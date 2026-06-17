# Phase 10 Pi 5 Generated-Root Firmware Initramfs Reservation Core

Task id: phase10-pi5-generated-root-firmware-initramfs-reservation-core-20260616

Status: accepted

Classification:
pi5-generated-root-firmware-initramfs-reservation-core-local-static

Evidence level: static/source inspection, unit tests, compile-only Pi 5 image
build, task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, networking, SSH, persistence,
SD/USB/block-driver work, Phase 11/12 work, or phase transition was performed.

## Goal

Implement the accepted
pi5-generated-root-firmware-initramfs-reserve-by-memory-plan-exclusion-v1
boundary so Pi 5 generated-root transport can preserve firmware-loaded
initramfs bytes before early memory setup can reuse them.

## Implementation

The implementation keeps the source contract's reservation design:

- src/boot/rpi5.rs now retains the FDT /chosen firmware initrd bounds during
  the Pi 5 DTB phase and threads them into boot-memory planning.
- src/memory_map/layout.rs accepts an optional FdtInitrdRange and excludes its
  page-rounded range from the conservative low-tail usable-memory candidate
  before page-frame seeding, bootstrap reservation, translation-table layout,
  allocator initialization, and cache transition.
- src/memory_map/page_frames.rs remains a consumer of the selected candidate;
  its tests pass None for the new optional exclusion and do not gain
  generated-root knowledge.

The exclusion is page-rounded with EARLY_USABLE_ALIGNMENT. This preserves the
accepted invariant that no page intersecting present non-empty
linux,initrd-start..linux,initrd-end bounds is offered as early usable memory.

The firmware-initramfs installer remains after memory setup and the
generated-root parser/fallback behavior is unchanged: missing, invalid,
oversize, malformed, or unsupported artifact bytes still fall back to the
compiled generated-root source.

## Findings

- fixed: threaded FDT /chosen initrd bounds from the Pi 5 DTB phase into the
  low-tail memory planner.
- fixed: excluded the page-rounded firmware initrd range before bootstrap
  reservation and translation-table placement can consume it.
- fixed: added focused planner tests for the accepted Pi 5 blocker shape and
  the no-valid-low-tail failure path.
- not-an-issue: src/initramfs.rs did not need changes because artifact parsing
  and compiled fallback behavior were already the accepted behavior.
- not-an-issue: src/memory_map/page_frames.rs and src/memory_map/translation.rs
  did not need generated-root-specific logic; they continue to consume only the
  selected early usable-memory candidate.
- deferred: serialized Pi 5 proof remains the next dependency-gated task.
- rejected: boot archive publication, lab mutation, hardware proof,
  persistence, SD/USB/block drivers, networking, SSH, Phase 11/12 work, and
  phase transition claims from this local/static implementation.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet conservative_candidate_: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- compile-only generated-root Pi 5 image build:
  ./scripts/rpi5-generated-root-boot-transport-image.sh: pass.
  Image:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-generated-root-boot-transport.img.
  SHA-256:
  c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd.
  Size: 208984 bytes.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Implementation matches the accepted source contract exactly: satisfied with
  reserve-by-memory-plan-exclusion-v1.
- Local tests/builds prove the selected preservation invariant: satisfied by
  focused memory planner tests, full cargo test, and compile-only Pi 5
  generated-root image build.
- Existing compiled-fallback and QEMU/substitute controls remain intact:
  satisfied; initramfs parser/fallback code and QEMU fixed-address transport
  were not changed.
- Hardware follow-up selected:
  phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof-20260616.
- Rejected networking/SSH/persistence/SD/USB/block/phase-transition claims
  remain explicit: satisfied.

## Next Action

After this core implementation is accepted and committed, promote
phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof-20260616
only if dependencies remain satisfied and hardwareTestLock is available and
restored. That proof must serialize hardware access, publish only the
task-owned candidate/control needed for the proof, capture selected-tree,
TFTP, serial freshness, FDT initrd bounds, runtime source classification, final
identity, and restore evidence, and avoid networking, SSH, persistence,
SD/USB/block storage, Phase 11/12 work, and phase transition claims.
