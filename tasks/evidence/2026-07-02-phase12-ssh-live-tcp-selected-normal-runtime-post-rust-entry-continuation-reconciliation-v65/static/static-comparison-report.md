# V65 Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65-20260702

## Boundary

Accepted v64 closeout proves selected normal-runtime execution reaches TALOS:
rust_entry on Pi 5. The selected v64 candidate retained TALOS: rust_entry 208
times with selected-byte TFTP service and restore proof, while BootInfo parsing,
target init, exceptions, kernel_main, route-start, runtime-blocked, and
runtime-ready markers were absent.

The next ordered fact is BootInfo parsing: rust_entry must call
BootInfo::from_aarch64_x0(dtb_pa), emit the BootInfo early-phase marker, and
then stop before target init or later runtime/service work.

## Implementation Review

- fixed: confirmed build.rs still defines
  rpi5_ssh_service_smoltcp_bootinfo_marker_loop with the selected
  rpi5_ssh_service_smoltcp_runtime_ready implication and early-entry assembly
  provenance.
- fixed: confirmed src/main.rs calls the BootInfo marker loop immediately after
  BootInfo::from_aarch64_x0(dtb_pa) and the existing BootInfo early-phase line,
  before target::init(&boot_info).
- fixed: confirmed src/target/rpi5.rs emits TALOS: boot info parsed with the
  capture nonce and false claims for target init, exceptions, kernel_main,
  route-start, runtime-ready, packet-I/O, service success, ssh-ready, and phase
  transition.
- fixed: materialized a current-source non-published archive and reviewed the
  selected root mirror plus marker tokens.
- not-an-issue: this task needed no source edit; the current BootInfo
  discriminator already exists and remains the smallest useful feature-led
  boundary after rust_entry.
- deferred: the future v66 task must run the exact selected archive on Pi 5
  before BootInfo parsing is accepted as hardware evidence.
- removed: target-init/exceptions/kernel_main proof, route-start/runtime-ready
  proof, packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action in v65, and phase transition as accepted
  outcomes.

## Archive Contract

- Archive: target/tmp/selected-normal-runtime-bootinfo-v65.tar.gz.
- Archive SHA-256:
  68a3e9356753c66b646477880f786fc10a01b021bd8758d19484f409df81ad9d.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  87bbaab6842cbd83c1dff548d81151af6f9ff5309236b7ba65481174560987a8.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Required future marker: TALOS: boot info parsed.
- Marker family: TALOS: asm_start, TALOS: asm_pre_rust_entry, TALOS:
  rust_entry, TALOS: boot info parsed.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Capture Helper

The future v66 dry-run contract uses TALOS_SERIAL_MARKER_FAMILY with TALOS:
asm_start, TALOS: asm_pre_rust_entry, TALOS: rust_entry, and TALOS: boot info
parsed; scripts/rpi5-capture-invariant-proof-bundle.sh --dry-run; evidence
directory tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65/lab/v66-candidate;
restore snapshot phase12-ssh-v10-openssh-clean-pre-20260624T074100Z; label
selected-normal-runtime-bootinfo-v66; expected kernel kernel_2712.img;
expected fetch da591740/kernel_2712.img; expected fetch bytes 152880; and
serial marker TALOS: boot info parsed.

This is a static contract only. It does not publish to the lab, acquire
hardwareTestLock, mutate boot snapshots, power-cycle the Pi 5, or claim
target-init, packet-I/O, OpenSSH, service readiness, or phase transition.
