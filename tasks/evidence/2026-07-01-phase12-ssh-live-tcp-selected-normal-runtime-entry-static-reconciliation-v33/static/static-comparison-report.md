# V33 Static Comparison Report

## Evidence Compared

- v26 selected-entry marker-loop hardware: 45,400-byte
  da591740/kernel_2712.img retained TALOS: reu10-loop 13,796 times and proves
  the earlier selected-entry artifact reaches rust_entry/UART10.
- v31 static normal-runtime pre-entry contract: 152,144-byte
  da591740/kernel_2712.img with SHA-256
  c169c9553096f3bae24802762f14c03588fc6d6e811b732c8ac6515c47ca8f95, valid
  Image header, root/selected equality, and embedded asm_start,
  asm_pre_rust_entry, route-start, and runtime-ready strings.
- v32 hardware: primary and unchanged rerun served the selected 152,144-byte
  image twice but retained no asm_start, asm_pre_rust_entry, rust_entry, boot
  info parsed, target init, exceptions ready, kernel_main, route-start, or
  runtime-ready marker.
- v32 known-good control: the restored 104,136-byte production-timer control
  retained rpi5-production-timer-preemption: PASS on the same capture path.

## Findings

- fixed: the v33 discriminator isolates selected Image entry by adding
  rpi5_ssh_service_smoltcp_entry_marker_loop, a scenario that implies
  rpi5_ssh_service_smoltcp_runtime_ready for Rust cfg but loops in assembly on
  TALOS: asm_start before CPACR setup, BSS clear, stack setup, rust_entry,
  BootInfo parsing, target init, exceptions, kernel_main, networking, or
  service code.
- fixed: the existing rpi5_ssh_service_smoltcp_runtime_ready scenario remains a
  continuation path with one-shot asm_start/asm_pre_rust_entry provenance.
- fixed: the non-published v33 archive preserves the selected
  da591740/kernel_2712.img path, valid Image header, root/selected equality,
  and embedded normal-runtime marker strings.
- not-an-issue: v26's rust_entry/UART10 result applies to the earlier
  45,400-byte selected-entry marker-loop artifact and should not be treated as
  proof that the 152,144-byte normal-runtime artifact enters Rust.
- deferred: only serialized v34 hardware can decide whether the repeated
  asm_start marker is retained from the selected 152,144-byte image.

## V33 Artifact Metadata

- Archive: target/tmp/selected-normal-runtime-entry-loop-v33.tar.gz.
- Archive SHA-256:
  cf57163942a3cc9989b6346a7c3bc3a30dd295118cbc86afbd5f0844118db0f3.
- Selected kernel SHA-256:
  5aa2b4ab51afa018d4c39fc5843e5df01a76dbc42bce2b40287693b5c77d311d.
- Selected kernel size: 152,144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12, magic=ARMd.
- Root/selected kernel equality: true.
- Expected marker family for v34: TALOS: asm_start.

## Blocked Claims

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, and phase transition remain blocked.
