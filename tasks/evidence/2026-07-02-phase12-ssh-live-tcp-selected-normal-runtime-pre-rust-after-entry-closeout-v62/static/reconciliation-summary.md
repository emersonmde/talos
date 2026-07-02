# V62 Pre-Rust Closeout Reconciliation Summary

## Inputs

- v61 selected normal-runtime pre-rust discriminator contract:
  selected-normal-runtime-pre-rust-discriminator-ready.
- v62 serialized Pi 5 preflight:
  selected-normal-runtime-pre-rust-marker-retained.

## Decisive Facts

- Selected tree 2f4d07fc983ec52c2a23dbc358f7730bd608ed27ff95fea3a5ebc7784b1c6823 was staged after publication, retained after power, and retained before restore.
- Same-window TFTP served da591740/kernel_2712.img twice at 152,144 bytes.
- The selected kernel SHA-256 was 90c72361bc67be8933436ddc5e6807dc127a8cb329a3fcab49404c10f8086d59.
- The serial marker-family summary retained TALOS: asm_pre_rust_entry 535 times in the fresh post-power window.
- The same serial window retained zero occurrences of TALOS: rust_entry, route-start, runtime-blocked, and runtime-ready markers.
- The lab restored to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z with tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Reconciled Frontier

The selected normal-runtime frontier advances from TALOS: asm_start to TALOS: asm_pre_rust_entry. The first missing fact is after TALOS: asm_pre_rust_entry and before TALOS: rust_entry.

## Successor

The next bounded task is the already queued no-hardware rust_entry-after-pre-rust reconciliation:

phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63-20260702.

Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility/service readiness, ssh-ready=true, fake command expansion, broad shell work, and phase transition remain blocked until a deeper runtime frontier is proved.
