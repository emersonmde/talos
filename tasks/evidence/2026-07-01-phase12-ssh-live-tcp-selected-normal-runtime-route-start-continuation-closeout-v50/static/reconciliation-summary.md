# V50 Route Start Closeout Reconciliation Summary

The accepted selected normal-runtime frontier now proves the following ordered
Pi 5 boundaries with selected-byte TFTP service and restore proof at each
hardware preflight boundary:

- v34: TALOS: asm_start retained for the selected normal-runtime archive class.
- v36: TALOS: asm_pre_rust_entry retained after CPACR setup, BSS clear, and
  stack setup.
- v38: TALOS: rust_entry retained after Rust begins.
- v40: TALOS: boot info parsed retained after BootInfo parsing.
- v42: TALOS: target init retained after target::init(&boot_info).
- v44: TALOS: exceptions ready retained after arch::aarch64::exceptions::init().
- v48: TALOS: kernel_main capture-nonce=runtime-marker-route-static retained
  after entering boot::rpi5::kernel_main.
- v49: the authoritative route-start discriminator contract emits TALOS:
  ssh-service-smoltcp-runtime-route-start
  capture-nonce=runtime-marker-route-static only after the accepted kernel_main
  frontier and before runtime-ready, packet-I/O, service success, ssh-ready,
  fake command expansion, broad shell work, or phase-transition claims.
- v50: selected tree e1c8ce434afb82517063c9535f53d127ae220b76e2756d65b110fc808193ac63
  stayed staged through final pre-restore identity, da591740/kernel_2712.img
  was served twice at 152,640 bytes, TALOS:
  ssh-service-smoltcp-runtime-route-start
  capture-nonce=runtime-marker-route-static was retained 2,326 times, and the
  lab was restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The closeout accepts selected-normal-runtime-route-start-frontier-proved. It
does not prove runtime-ready, packet-I/O, remote receipt,
compatibility/service readiness, OpenSSH, ssh-ready=true, fake command
expansion, broad shell work, or phase transition.

The next bounded task remains a static/no-hardware runtime-ready continuation
reconciliation:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51-20260701.
