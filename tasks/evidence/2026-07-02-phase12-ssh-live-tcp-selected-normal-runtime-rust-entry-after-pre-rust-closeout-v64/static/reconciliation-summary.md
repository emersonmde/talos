# V64 Rust Entry Closeout Reconciliation Summary

The v64 closeout is a no-hardware reconciliation task. It accepts the v63
static discriminator contract and the v64 Pi 5 hardware preflight as the source
of truth for the current selected normal-runtime frontier.

## Accepted Inputs

- v63 produced the selected normal-runtime rust_entry marker-loop archive from
  commit ca15cbd2c36619813ff70517c1e99c6c7d018bbd:
  target/tmp/selected-normal-runtime-rust-entry-v63.tar.gz.
- v63 archive SHA-256:
  7211853ae0fe6008b10b340725799503ff3ff9be46518428d2e5d3fdbf4e641f.
- v63 selected da591740/kernel_2712.img: 152,816 bytes, SHA-256
  347679f5797d2c99d61a56d5b250ee0245a0f19e9ac5f927491c4b9a019709c6.
- v64 selected tree:
  d0a5132b630258a98de56fa7e9c0eb9d1cdb41358b68e91321384461a835b6b2.
- v64 TFTP evidence served da591740/kernel_2712.img twice at 152,816 bytes.
- v64 serial evidence retained TALOS: rust_entry 208 times.
- v64 restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z with post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Reconciled Boundary

The selected normal-runtime branch has now crossed the pre-rust boundary and
entered rust_entry on Pi 5. That proves TALOS: rust_entry for this selected
candidate, but it does not prove the later normal-runtime continuation:
BootInfo parsing, target init, exceptions, kernel_main, route-start,
runtime-blocked, runtime-ready, packet-I/O, OpenSSH compatibility, service
readiness, ssh-ready=true, fake command expansion, broad shell work, and phase
transition remain outside this closeout.

The next bounded task is the already queued no-hardware source/static
reconciliation:
phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65-20260702.
It must decide the smallest objective post-rust-entry boundary before any
hardware preflight or later feature work is selected.
