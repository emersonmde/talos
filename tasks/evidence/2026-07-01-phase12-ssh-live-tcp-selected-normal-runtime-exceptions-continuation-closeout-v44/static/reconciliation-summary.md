# V44 Exceptions-Ready Closeout Reconciliation Summary

This closeout reconciles the selected normal-runtime continuation chain through
the accepted v44 Pi 5 exceptions-ready preflight.

- v34: selected normal-runtime assembly-entry proof reaches TALOS: asm_start
  on Pi 5.
- v36: selected normal-runtime pre-rust proof reaches TALOS:
  asm_pre_rust_entry after CPACR setup, BSS clear, and stack setup.
- v38: selected normal-runtime rust_entry proof reaches TALOS: rust_entry
  after Rust begins.
- v40: selected normal-runtime BootInfo proof reaches TALOS: boot info parsed.
- v42: selected normal-runtime target-init proof reaches TALOS: target init
  after target::init(&boot_info).
- v43: selected normal-runtime exceptions-ready contract emits TALOS:
  exceptions ready only after arch::aarch64::exceptions::init() returns.
- v44: selected normal-runtime exceptions-ready Pi 5 proof serves
  da591740/kernel_2712.img twice at 152,880 bytes, retains selected tree
  2c0d4152ebae130632caa5a9e8fa776704ec0336d2c54609ab00a5981328fcde through
  final pre-restore identity, observes TALOS: exceptions ready 2,145 times,
  does not accept TALOS: kernel_main, and restores to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Terminal classification:
selected-normal-runtime-exceptions-frontier-proved.

First unresolved continuation facts: kernel_main, route-start, runtime-ready,
packet-I/O, remote receipt, compatibility/service readiness, OpenSSH,
ssh-ready=true, fake command expansion, broad shell work, and phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45-20260701.

planningNeeded: false.

Redaction review: this summary retains no raw serial text, raw TFTP peer/log
lines, packet payloads, SSH/session/key material, boot artifact bytes, private
data, or stable secret-derived identifiers.
