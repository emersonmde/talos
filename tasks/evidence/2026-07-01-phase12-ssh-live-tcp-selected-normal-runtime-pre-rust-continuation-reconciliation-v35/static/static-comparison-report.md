# V35 Static Comparison Report

Task: phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35-20260701

## Boundary

v34 proved the selected 152144-byte normal-runtime entry-loop archive can
reach TALOS: asm_start on Pi 5. v35 keeps the same selected normal-runtime
feature route and moves the loop to TALOS: asm_pre_rust_entry after the
assembly entry setup, CPACR enable, BSS clear, and stack setup, before the
branch to rust_entry.

## Source Review Findings

- fixed: added rpi5_ssh_service_smoltcp_pre_rust_marker_loop as a separate
  boot scenario that implies rpi5_ssh_service_smoltcp_runtime_ready while
  defining only TALOS_RPI5_PRE_RUST_ENTRY_LOOP_SCENARIO.
- fixed: boot.S now emits TALOS: asm_start once for the pre-rust loop
  scenario, then loops on TALOS: asm_pre_rust_entry before rust_entry.
- fixed: added a task-specific boot-tree helper that mirrors root Pi 5 files
  under da591740 for selected TFTP service.
- fixed: repaired rpi5-capture-invariant-proof-bundle.sh so future summaries
  classify retained required-marker occurrences or fresh marker-family evidence
  as post-handoff-marker-visible.
- not-an-issue: rust_entry, BootInfo parsing, target init, exceptions,
  kernel_main, packet-I/O, OpenSSH/generated-root retry, compatibility/service
  readiness, and ssh-ready=true remain beyond this discriminator.

## Static Artifact Review

- Archive: target/tmp/selected-normal-runtime-pre-rust-v35.tar.gz.
- Archive SHA-256:
  2e2f538a7453c6fbce6b05c0c053b282d5e24c8f2d798e4893a2607fc7e7a0b2.
- Kernel SHA-256:
  afd9f9550e2abbdcba80520eb7c3527f1f3a3c3b383a432e8fe98c2381f8c7c1.
- Kernel size: 152144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12, magic=ARMd.
- Root kernel_2712.img and da591740/kernel_2712.img are byte-identical.
- Marker strings retained: TALOS: asm_start, TALOS: asm_pre_rust_entry,
  runtime route-start, runtime blocked/ready fail-closed labels, capture nonce,
  ssh-ready=false, claims-service-success=false, and claims-phase-transition=false.
- Disassembly shows the TALOS: asm_pre_rust_entry loop at _start+0x90 through
  _start+0xac, before the later rust_entry branch at _start+0xb4.

## Result

The non-published selected normal-runtime pre-rust archive is ready for the
serialized v36 Pi 5 preflight. No hardware action, lab publication, boot
snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility/service readiness claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.
