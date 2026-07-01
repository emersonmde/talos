# Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43-20260701

The accepted selected normal-runtime frontier now reaches TALOS: target init on
Pi 5. v34 proves TALOS: asm_start, v36 proves TALOS: asm_pre_rust_entry, v38
proves TALOS: rust_entry, v40 proves TALOS: boot info parsed, and v42 proves
TALOS: target init with selected-byte TFTP service and restore proof.

This task adds the next source/static discriminator after
arch::aarch64::exceptions::init() returns and before kernel_main(&boot_info).
The new rpi5_ssh_service_smoltcp_exceptions_ready_marker_loop scenario keeps
the selected rpi5_ssh_service_smoltcp_runtime_ready route linked into the image
through build.rs implied values, but diverts to a marker loop immediately after
the existing EarlyPhaseLine::ExceptionsReady write.

The required future marker is:

    TALOS: exceptions ready capture-nonce=runtime-marker-route-static

The marker line claims only boot-info parsed, target init, and exceptions ready.
It explicitly withholds kernel_main, route-start, runtime-ready, packet-I/O,
service success, ssh-ready, broad shell work, and phase-transition claims.

Non-published archive review:

- archive: target/tmp/selected-normal-runtime-exceptions-ready-v43.tar.gz
- archive SHA-256:
  44733062c516f899b43b3a31241ac19c1bec4dd51d768260e56b066c0db15fed
- selected fetch path: da591740/kernel_2712.img
- selected kernel SHA-256:
  34f17bf595aef2c658f9b95b81703a0c62c0051b6793ab8c7d527882dc164682
- selected kernel bytes: 152,880
- Image header: text_offset=0, header_image_size=152880, flags=12.
- Root and selected da591740/kernel_2712.img are byte-identical.

No hardware action, lab publication, boot snapshot mutation, Pi 5 power action,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility/service
readiness claim, fake command expansion, broad shell work, or phase transition
was performed.
