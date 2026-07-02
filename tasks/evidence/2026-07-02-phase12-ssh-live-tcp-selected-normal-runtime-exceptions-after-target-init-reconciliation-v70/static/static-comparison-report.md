# Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70-20260702

## Inputs

- Accepted v69 target-init frontier proof:
  phase12-ssh-live-tcp-selected-normal-runtime-target-init-after-bootinfo-closeout-v69-20260702.
- Current source commit: 9a4c6578899f1f08937d31091925ff233608ee85.
- Current source paths: src/main.rs rust_entry normal-runtime path and
  src/target/rpi5.rs exceptions-ready marker loop.
- Current helpers:
  scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-boot-tree.sh,
  scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-archive-review.sh,
  scripts/rpi5-archive-review.sh, and
  scripts/rpi5-capture-invariant-proof-bundle.sh.

## Source Order

The accepted v69 frontier ends at TALOS: target init after BootInfo parsing and
target::init(&boot_info) return. In current source, the next selected
normal-runtime discriminator path is:

1. src/main.rs has parsed BootInfo and returned from target::init(&boot_info).
2. src/main.rs emits the TargetInit early-phase line and the accepted v69
   selected target-init loop can run only after that point.
3. arch::aarch64::exceptions::init() is called.
4. src/main.rs emits the ExceptionsReady early-phase line.
5. For talos_boot_scenario
   rpi5_ssh_service_smoltcp_exceptions_ready_marker_loop,
   run_ssh_service_smoltcp_exceptions_ready_marker_loop() emits
   TALOS: exceptions ready.

The selected exceptions-ready loop records claims-bootinfo-parsed=true and
claims-target-init=true, then records claims-kernel-main=false,
claims-route-start=false, claims-runtime-ready=false, claims-packet-io=false,
claims-service-success=false, claims-ssh-ready=false, and
claims-phase-transition=false.

## Decision

The next smallest feature-led discriminator after target init is TALOS:
exceptions ready. Current source and helpers can specify a non-published archive
contract for that marker without broadening to kernel_main, route-start,
runtime-ready, packet-I/O, OpenSSH, service readiness, fake command expansion,
broad shell work, or a phase transition.

## Archive Contract

- Archive path: target/tmp/selected-normal-runtime-exceptions-ready-v70.tar.gz.
- Archive SHA-256:
  18007965ceb10991766e01ab2cf4d6f468530eca97d1a8c3a016a39b0402396b.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  7a62150e4232fc8215a7c7ec8e502697bdabb3a9e6bcd62f640c75aba722e455.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12, magic=ARMd.
- Root and da591740/kernel_2712.img are byte-identical.
- Required marker: TALOS: exceptions ready
  capture-nonce=runtime-marker-route-static.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Negative Claims

This task does not prove kernel_main, route-start, runtime-ready, packet-I/O,
OpenSSH/generated-root behavior, remote receipt, compatibility/service
readiness, ssh-ready=true, fake command expansion, broad shell work, hardware
behavior, or any phase transition.
