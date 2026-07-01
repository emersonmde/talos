# V45 Kernel_Main Static Comparison Report

v45 compares the accepted selected normal-runtime exceptions-ready frontier
against the next kernel_main boundary.

- v34 proves selected normal-runtime assembly entry at TALOS: asm_start.
- v36 proves selected normal-runtime asm_pre_rust_entry after CPACR setup, BSS
  clear, and stack setup.
- v38 proves selected normal-runtime rust_entry after Rust begins.
- v40 proves selected normal-runtime BootInfo parsing at TALOS: boot info
  parsed.
- v42 proves selected normal-runtime target init after target::init(&boot_info).
- v43 defines the selected normal-runtime exceptions-ready marker loop after
  arch::aarch64::exceptions::init().
- v44 proves the selected v43 archive reaches TALOS: exceptions ready on Pi 5,
  serves da591740/kernel_2712.img twice at 152,880 bytes, and restores to the
  named baseline. It does not accept TALOS: kernel_main.
- v45 adds rpi5_ssh_service_smoltcp_kernel_main_marker_loop, selected by
  scripts/rpi5-ssh-service-smoltcp-kernel-main-marker-loop-boot-tree.sh. The
  marker loop is invoked immediately after boot::rpi5::kernel_main starts and
  before report_boot_identity(), run_ssh_service_smoltcp_runtime_ready_route(),
  or any route-start/runtime-ready path.

The v45 archive contract keeps the selected normal-runtime route linked but
makes TALOS: kernel_main capture-nonce=runtime-marker-route-static the next
required hardware marker. The marker line explicitly withholds route-start,
runtime-ready, packet-I/O, service success, ssh-ready, and phase-transition
claims.

Terminal classification:
selected-normal-runtime-kernel-main-discriminator-ready.

First unresolved continuation facts: kernel_main hardware proof, route-start,
runtime-ready, packet-I/O, remote receipt, compatibility/service readiness,
OpenSSH, ssh-ready=true, fake command expansion, broad shell work, and phase
transition.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46-20260701.

planningNeeded: true because the v46 hardware task is not yet explicit in
taskQueue.

Redaction review: this report retains no raw serial text, raw TFTP peer/log
lines, packet payloads, SSH/session/key material, boot artifact bytes, private
data, or stable secret-derived identifiers.
