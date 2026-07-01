# V39 Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39-20260701

## Boundary

Accepted v34 hardware evidence proves selected normal-runtime artifacts reach
TALOS: asm_start. Accepted v36 hardware evidence proves the selected pre-rust
continuation reaches TALOS: asm_pre_rust_entry. Accepted v38 hardware evidence
proves the selected 152,816-byte rust_entry marker-loop archive reaches TALOS:
rust_entry on Pi 5.

The next ordered fact is BootInfo parsing: rust_entry must call
BootInfo::from_aarch64_x0(dtb_pa) and then emit a marker before target init,
exceptions, kernel_main, route-start, runtime-ready, packet-I/O, or service
readiness.

## Implementation Review

- fixed: added rpi5_ssh_service_smoltcp_bootinfo_marker_loop as a separate
  selected normal-runtime scenario in build.rs.
- fixed: placed the marker-loop call in src/main.rs immediately after
  BootInfo::from_aarch64_x0(dtb_pa) and the existing early-phase BootInfo
  line, before target::init(&boot_info).
- fixed: made the BootInfo marker loop returnable in the type system, matching
  the v37 rust_entry loop pattern, so downstream selected runtime-route code
  remains linked in the non-published archive while hardware execution stops
  at the BootInfo boundary.
- fixed: added boot-tree and archive-review helpers for the selected BootInfo
  marker-loop artifact.
- not-an-issue: the contiguous TALOS: rust_entry image string is not required
  by the v39 archive helper because the normal path emits rust_entry through
  the byte-wise early-phase line writer; v38 already supplies the decisive
  hardware proof for that predecessor fact, and v40 still includes it in the
  serial marker family.
- deferred: v40 must run the exact selected archive on Pi 5 before BootInfo
  parsing is accepted as hardware evidence.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action in v39, and phase transition as accepted
  outcomes.

## Archive Contract

- Archive: target/tmp/selected-normal-runtime-bootinfo-v39.tar.gz.
- Archive SHA-256:
  23ba0d4dee7cde85e6b6ef914528f209c20cebf0edc022723a4bd1c84ea4cec5.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  62c285790a87b3ab8395aa9dcbd8167318506c940fe5a4f61e07371c0806486b.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Required v40 marker: TALOS: boot info parsed.
- Marker family: TALOS: asm_start, TALOS: asm_pre_rust_entry, TALOS:
  rust_entry, TALOS: boot info parsed.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Capture Helper

The v40 dry-run contract uses TALOS_SERIAL_MARKER_FAMILY with TALOS:
asm_start, TALOS: asm_pre_rust_entry, TALOS: rust_entry, and TALOS: boot info
parsed; scripts/rpi5-capture-invariant-proof-bundle.sh --dry-run; evidence
directory tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39/lab/v40-candidate;
restore snapshot phase12-ssh-v10-openssh-clean-pre-20260624T074100Z; label
selected-normal-runtime-bootinfo-v40; expected kernel kernel_2712.img;
expected fetch da591740/kernel_2712.img; expected fetch bytes 152880; and
serial marker TALOS: boot info parsed.

This is a static contract only. It does not publish to the lab, acquire
hardwareTestLock, mutate boot snapshots, power-cycle the Pi 5, or claim
packet-I/O/OpenSSH/service readiness.
