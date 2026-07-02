# Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67-20260702

## Inputs

- v68 selected BootInfo lineage proof:
  phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-rust-entry-lineage-reconciliation-v68-20260702.
- Current source commit: a4297c8912c767e05bd8b649fcad72c1b89a51fa.
- Current source path: src/main.rs rust_entry normal-runtime path and
  src/target/rpi5.rs target-init marker loop.
- Current helpers:
  scripts/rpi5-ssh-service-smoltcp-target-init-marker-loop-boot-tree.sh,
  scripts/rpi5-ssh-service-smoltcp-target-init-marker-loop-archive-review.sh,
  and scripts/rpi5-capture-invariant-proof-bundle.sh.

## Source Order

The accepted BootInfo frontier ends at TALOS: boot info parsed. In current
source, the next selected normal-runtime discriminator path is:

1. src/main.rs rust_entry(dtb_pa) has already entered Rust.
2. BootInfo::from_aarch64_x0(dtb_pa) returns and BootInfoParsed output is
   emitted.
3. target::init(&boot_info) executes.
4. TargetInit early-phase output is emitted.
5. For talos_boot_scenario
   rpi5_ssh_service_smoltcp_target_init_marker_loop,
   run_ssh_service_smoltcp_target_init_marker_loop() emits
   TALOS: target init.

The selected target-init loop records claims-bootinfo-parsed=true and records
claims-exceptions-ready=false, claims-kernel-main=false,
claims-route-start=false, claims-runtime-ready=false,
claims-packet-io=false, claims-service-success=false, claims-ssh-ready=false,
and claims-phase-transition=false.

## Decision

The next smallest feature-led discriminator after BootInfo is TALOS: target
init. Current source and helpers can specify a non-published archive contract
for that marker without broadening to exceptions, kernel_main, route-start,
runtime-ready, packet-I/O, OpenSSH, or service readiness.

## Archive Contract

- Archive path: target/tmp/selected-normal-runtime-target-init-v67.tar.gz.
- Archive SHA-256:
  18270d2ca0bef45c72898beaa55971b48d748f3a87a767556074423821f17352.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  4513bd97689673f904a849b60aee0377d6ddcc813ad0d00a18e422b3cc52ef82.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12, magic=ARMd.
- Root and da591740/kernel_2712.img are byte-identical.
- Required marker: TALOS: target init.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Negative Claims

This task does not prove exceptions ready, kernel_main, route-start,
runtime-ready, packet-I/O, OpenSSH/generated-root behavior, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, hardware behavior, or any phase transition.
