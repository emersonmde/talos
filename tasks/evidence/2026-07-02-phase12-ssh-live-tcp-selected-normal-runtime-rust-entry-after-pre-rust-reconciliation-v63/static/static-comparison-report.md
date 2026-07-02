# V63 Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63-20260702

## Inputs

- Accepted v62 closeout:
  tasks/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-closeout-v62.md.
- Accepted v37 static rust_entry discriminator:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37.md.
- Accepted v38 Pi 5 rust_entry preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38.md.
- Current source commit used for materialization:
  ca15cbd2c36619813ff70517c1e99c6c7d018bbd.

## Boundary

v62 proves selected post-power identity, selected TFTP byte service, selected
final pre-restore identity, restore proof, and TALOS: asm_pre_rust_entry
retention. It explicitly leaves the first missing fact after TALOS:
asm_pre_rust_entry and before TALOS: rust_entry.

The current source still contains the selected normal-runtime rust_entry marker
loop introduced in v37. That loop enters rust_entry, emits TALOS: rust_entry,
and records negative claims for BootInfo, target init, exceptions, kernel_main,
packet-I/O, service success, ssh-ready, and phase transition. It keeps later
normal-runtime route strings linked into the artifact, but the v64 success
marker is only TALOS: rust_entry.

## Refreshed Artifact

- Archive: target/tmp/selected-normal-runtime-rust-entry-v63.tar.gz.
- Archive SHA-256:
  7211853ae0fe6008b10b340725799503ff3ff9be46518428d2e5d3fdbf4e641f.
- Selected fetch: da591740/kernel_2712.img.
- Selected kernel size: 152,816 bytes.
- Selected kernel SHA-256:
  347679f5797d2c99d61a56d5b250ee0245a0f19e9ac5f927491c4b9a019709c6.
- Image header: text_offset=0, header_image_size=152816, flags=12,
  magic=ARMd.
- Root and da591740/kernel_2712.img: byte-identical.

## Successor Contract

The next serialized Pi 5 task should publish only the refreshed v63 archive,
acquire hardwareTestLock before any lab mutation, and classify with:

- success: selected-normal-runtime-rust-entry-marker-retained when selected
  identity/TFTP/final identity/restore proof join to TALOS: rust_entry;
- missing marker: blocked-selected-normal-runtime-rust-entry-marker-missing
  when selected identity/TFTP/final identity/restore proof are decisive but
  TALOS: rust_entry is absent;
- inconclusive: selected-normal-runtime-rust-entry-inconclusive-after-triage
  only after the standard identity, serial cursor, TFTP delta, known-good
  control when applicable, candidate rerun when applicable, and restore
  sequence cannot make the result decisive.

Objectively specified preflight id:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64-20260702.

The worker did not create that queued task; supervisor planning is required.
