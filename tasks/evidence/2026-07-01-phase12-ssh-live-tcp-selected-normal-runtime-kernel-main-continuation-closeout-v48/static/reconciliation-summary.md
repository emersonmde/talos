# V48 Kernel Main Closeout Reconciliation Summary

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
- v45/v47: the authoritative kernel_main discriminator contract emits
  TALOS: kernel_main capture-nonce=runtime-marker-route-static only after
  entering boot::rpi5::kernel_main, using archive SHA-256
  72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19 and
  selected kernel SHA-256
  2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5.
- v48: selected tree 9d2f354810e8f445705dd083c8876f47bd25fa5f1aec52762c5af98662fdf60a
  stayed staged through final pre-restore identity, da591740/kernel_2712.img
  was served twice at 152,896 bytes, TALOS: kernel_main
  capture-nonce=runtime-marker-route-static was retained 1,794 times, and the
  lab was restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The closeout accepts selected-normal-runtime-kernel-main-frontier-proved. It
does not prove route-start, runtime-ready, packet-I/O, remote receipt,
compatibility/service readiness, OpenSSH, ssh-ready=true, fake command
expansion, broad shell work, or phase transition.

The next bounded task remains a static/no-hardware route-start continuation
reconciliation:
phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49-20260701.
