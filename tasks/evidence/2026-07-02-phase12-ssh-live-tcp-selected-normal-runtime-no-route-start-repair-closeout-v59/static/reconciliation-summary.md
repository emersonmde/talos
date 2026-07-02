# V59 Closeout Reconciliation Summary

The accepted v58 contract selected the entry-loop discriminator archive at
target/tmp/selected-normal-runtime-entry-loop-v58.tar.gz and required
TALOS: asm_start as the first success marker.

The accepted v59 Pi 5 preflight proved:

- selected post-power tree c8a7e7d3de13900ab5d87b17040f82b85e6e2a557a9de1e6f882812c448f6a0f;
- selected da591740/kernel_2712.img served twice by TFTP at 152,144 bytes;
- final pre-restore identity still selected;
- restore to baseline tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10;
- TALOS: asm_start retained 547 times in the fresh serial window.

The same serial window retained zero occurrences of TALOS: asm_pre_rust_entry,
TALOS: kernel_main, route-start, runtime-blocked, and runtime-ready markers.
Therefore this closeout accepts selected-normal-runtime-entry-frontier-proved
and records the first missing fact after TALOS: asm_start and before
TALOS: asm_pre_rust_entry.

No successor is selected. Packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility/service readiness, ssh-ready=true, fake command
expansion, broad shell work, hardware action, and phase transition remain
blocked pending supervisor planning.
