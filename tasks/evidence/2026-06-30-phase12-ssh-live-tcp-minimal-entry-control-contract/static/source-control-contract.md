# Minimal Entry-Control Source Contract

Task id: phase12-ssh-live-tcp-minimal-entry-control-contract-20260630

Scope: no hardware, no lab publication, no boot snapshot mutation, no Pi 5
power action, no live TCP, no packet I/O, and no OpenSSH/generated-root retry.

## Invariant

The later hardware discriminator must prove the selected Pi 5 firmware fetch
path can reach the Rust-side kernel_main marker independently of the live TCP
runtime route.

Expected marker order:

1. selected TFTP fetch of da591740/kernel_2712.img with reviewed bytes/hash.
2. existing TALOS: kernel_main line from boot::rpi5::kernel_main.
3. nonce-bearing TALOS: minimal-entry-control-ready line emitted immediately
   after the kernel_main marker and before BootInfo/reporting/runtime work.

The control uses the same normal Pi 5 Image/startup/rust_entry/kernel_main path
as the v9 candidate, but strips the live TCP runtime route. It does not use the
quarantined raw assembly early-entry provenance scenario.

## Source Findings

- fixed: added rpi5_minimal_entry_control as an allowed boot scenario in
  build.rs and src/main.rs.
- fixed: added run_minimal_entry_control_marker in src/target/rpi5.rs. The
  marker emits the contract id, selected fetch path, capture nonce, and
  fail-closed non-claims: live-tcp-route=false, packet-io=false,
  openssh=false, ssh-ready=false, claims-service-success=false, and
  claims-phase-transition=false.
- fixed: wired the scenario in src/boot/rpi5.rs immediately after the existing
  kernel_main early-phase line, before BootInfo/reporting/runtime routes.
- fixed: added non-published materialization and static archive-review helpers:
  scripts/rpi5-minimal-entry-control-boot-tree.sh and
  scripts/rpi5-minimal-entry-control-archive-review.sh.
- not-an-issue: scripts/rpi5-archive-review.sh still proves the selected
  da591740/kernel_2712.img mirror matches root kernel_2712.img, text_offset=0,
  header_image_size=file size, flags=12, and magic ARMd.
- not-an-issue: symbol inspection retains _start and __kernel_start at
  0x200000, run_minimal_entry_control_marker, rust_entry,
  boot::rpi5::kernel_main, and __kernel_image_end.
- deferred: the later hardware discriminator must prove whether the marker
  appears after selected fetch on Pi 5 hardware; this task only defines and
  statically reviews the control archive.
- deferred: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, and phase
  transition remain blocked.

## Static Materialization

- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/validation/archive-review.stdout.txt.
- Minimal entry-control archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/validation/minimal-entry-control-archive-review.stdout.txt.
- Kernel byte count: 52848.
- Kernel SHA-256:
  313f1842b8905e88642e4f8278a9b17eec4f30ab856e352a77c523160e1bbf21.
- Archive SHA-256:
  f455dc3ab55ba0f2bd7afb7a57d5866b5d2ae1c040ac92bce3360708696a195d.

## Redaction Review

This no-hardware task retained no raw serial text, raw TFTP peer/log-line
fields, packet payloads, key material, session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data. Evidence is limited to task ids, hashes, byte counts, static strings,
symbol names/addresses, validation command results, and fixed classification
strings.
