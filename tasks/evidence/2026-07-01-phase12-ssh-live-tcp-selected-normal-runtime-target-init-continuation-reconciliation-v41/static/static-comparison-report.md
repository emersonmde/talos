# V41 Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-target-init-continuation-reconciliation-v41-20260701

Classification: selected-normal-runtime-target-init-discriminator-ready.

## Accepted Frontier

- v34 proves the selected normal-runtime archive class reaches TALOS: asm_start
  on Pi 5.
- v36 proves selected TALOS: asm_pre_rust_entry after assembly setup and
  before Rust.
- v38 proves selected TALOS: rust_entry on Pi 5.
- v39 defines the selected normal-runtime BootInfo discriminator.
- v40 proves selected-byte Pi 5 evidence reaches TALOS: boot info parsed.

## Target-Init Boundary

The new rpi5_ssh_service_smoltcp_target_init_marker_loop scenario keeps the
selected runtime-ready route linked through the existing implied scenario, then
uses the normal rust_entry path until target::init(&boot_info) returns. It emits
the required loop marker only after the existing target-init phase line and
before exceptions initialization, kernel_main, route-start, runtime-ready,
packet-I/O, service success, OpenSSH compatibility, ssh-ready=true, or phase
transition.

Required successor marker: TALOS: target init.

Marker family for the future hardware capture: TALOS: asm_start, TALOS:
asm_pre_rust_entry, TALOS: rust_entry, TALOS: boot info parsed, TALOS: target
init.

## Artifact Contract

- Archive path: target/tmp/selected-normal-runtime-target-init-v41.tar.gz.
- Archive SHA-256:
  b3d56e302e816c68c7fbdbeb007ef70861e690587a80579f2ef2eeccc054ae47.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  9bbea12314f09731a458cb6b7dbdf4071bd8eca4419f61af1d44251af98c0326.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12,
  magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Findings

- fixed: added rpi5_ssh_service_smoltcp_target_init_marker_loop as a separate
  selected normal-runtime discriminator after BootInfo parsing and target
  initialization.
- fixed: registered the scenario in build.rs with the same selected runtime
  service shape implied by rpi5_ssh_service_smoltcp_runtime_ready.
- fixed: added boot-tree and archive-review helpers for the target-init marker
  loop contract.
- fixed: added the missing top-level dead-code allowance entries for the
  BootInfo and target-init marker-loop scenarios.
- not-an-issue: contiguous downstream runtime-route strings remain in the
  image because preserving the selected normal-runtime service shape is part of
  the contract; the target-init marker line itself withholds later milestone
  claims.
- deferred: serialized Pi 5 proof of TALOS: target init requires a supervisor
  queued v42 hardware task because no explicit successor task currently exists
  in taskQueue.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as immediate
  successors.
