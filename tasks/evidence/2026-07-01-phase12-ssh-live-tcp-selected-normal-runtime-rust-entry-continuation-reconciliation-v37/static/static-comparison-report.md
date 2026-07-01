# V37 Static Comparison Report

Task: phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37-20260701

## Boundary

v34 proved selected normal-runtime assembly entry through TALOS: asm_start on
Pi 5. v35/v36 moved the proof past CPACR setup, BSS clear, and stack setup to
TALOS: asm_pre_rust_entry. v37 creates the next bounded discriminator: a
selected normal-runtime artifact that enters rust_entry, emits TALOS:
rust_entry from Rust code, then loops before BootInfo parsing, target init,
exceptions, kernel_main, packet-I/O, service readiness, or any phase
transition.

## Source Review Findings

- fixed: added rpi5_ssh_service_smoltcp_rust_entry_marker_loop as a separate
  boot scenario that implies rpi5_ssh_service_smoltcp_runtime_ready while
  preserving the asm_start and asm_pre_rust_entry provenance markers.
- fixed: added a Rust-side marker loop reached only after rust_entry begins.
  The loop emits TALOS: rust_entry with the capture nonce and explicit false
  claims for BootInfo parsing, target init, exceptions, kernel_main,
  packet-I/O, service success, ssh-ready, and phase transition.
- fixed: kept the loop returnable in the type system so the selected
  normal-runtime route and runtime marker strings remain linked into the
  artifact; the loop still spins forever in practice because its exit guard is
  a false atomic.
- fixed: added task-specific boot-tree and archive-review helpers for the
  rust_entry marker-loop contract.
- not-an-issue: local objdump cannot disassemble the AArch64 ELF in this
  environment; readelf symbol evidence and source/static artifact checks record
  _start, rust_entry, the rust_entry marker loop, and the runtime-ready route.
- deferred: v38 must run serialized Pi 5 hardware evidence for this exact
  archive contract before rust_entry or later normal-runtime progress can be
  accepted.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Static Artifact Review

- Archive: target/tmp/selected-normal-runtime-rust-entry-v37.tar.gz.
- Archive SHA-256:
  b8014b4b935bd81c3fdb077046cc5b10071b57d71af628678c9def68f8b43053.
- Kernel SHA-256:
  c1c1c864ca89babb516c11ce0f52357c69b79c2c6034e42150494a043658f9bc.
- Kernel size: 152816 bytes.
- Image header: text_offset=0, header_image_size=152816, flags=12, magic=ARMd.
- Root kernel_2712.img and da591740/kernel_2712.img are byte-identical.
- Marker strings retained: TALOS: asm_start, TALOS: asm_pre_rust_entry,
  TALOS: rust_entry, capture nonce token, selected-normal-runtime-rust-entry,
  runtime route-start, runtime ready/blocked fail-closed labels,
  ssh-ready=false, claims-service-success=false, and
  claims-phase-transition=false.
- Symbol review retains _start at 0x200000, rust_entry at 0x2093e4, the
  rust_entry marker loop at 0x20547c, and the runtime-ready route at 0x205154.
- Capture helper dry-run contract uses required marker TALOS: rust_entry,
  marker family TALOS: asm_start -> TALOS: asm_pre_rust_entry -> TALOS:
  rust_entry, expected fetch da591740/kernel_2712.img, expected fetch bytes
  152816, and restore target phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Result

The non-published selected normal-runtime rust_entry archive is ready for the
serialized v38 Pi 5 preflight. No hardware action, lab publication, boot
snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility/service readiness claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.
