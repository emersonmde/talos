# V56 Runtime-Ready Marker-Family Static Comparison

Task id: phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56-20260701

## Source Boundary

- src/target/rpi5.rs emits TALOS: ssh-service-smoltcp-runtime-route-start before calling live_tcp_runtime_marker_route_report().
- The ready marker is emitted only when marker_route_ready() is true.
- The blocked marker is emitted when live_tcp_runtime_marker_route_report() returns a non-ready report or an error.
- All three route markers use TALOS_CAPTURE_NONCE when present.
- src/network.rs keeps marker_route_ready gated on accepted deterministic descriptor/device-interface delivery, equal nonzero driver packet rx/tx counts, and false packet-I/O, reachability, remote receipt, compatibility, and ssh-ready claims.

## V55 Evidence Reconciliation

Accepted v55 proved selected post-power identity, selected same-window TFTP byte service for da591740/kernel_2712.img at 152,144 bytes, and selected final pre-restore identity. The retained serial marker family only counted the runtime-ready marker and the nonce-bearing runtime-ready marker, both at zero occurrences. That leaves the first missing fact as runtime-ready marker retention, but it does not distinguish route-start-only from runtime-blocked.

## Contract Repair

The v56 contract keeps the v51 non-published archive authoritative:

- archive: target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz
- archive SHA-256: 44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9
- selected fetch path: da591740/kernel_2712.img
- selected kernel SHA-256: b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982
- selected kernel size: 152,144 bytes
- root and selected da591740/kernel_2712.img: byte-identical
- capture nonce: runtime-ready-static
- restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z

The archive review now reports the nonce-bearing route-start, runtime-blocked, and runtime-ready markers as one marker family. The v57 capture dry-run requires the same nonce-bearing family plus asm_start, asm_pre_rust_entry, and kernel_main so the hardware preflight can fail closed at the deepest retained marker.

## V57 Classifications

- selected-normal-runtime-runtime-ready-marker-retained: selected identity/TFTP/final identity are proved and the nonce-bearing runtime-ready marker is retained.
- selected-normal-runtime-runtime-blocked-marker-retained: selected identity/TFTP/final identity are proved and the nonce-bearing runtime-blocked marker is retained without runtime-ready.
- selected-normal-runtime-route-start-only-marker-retained: selected identity/TFTP/final identity are proved and route-start is retained without runtime-blocked or runtime-ready.
- selected-normal-runtime-no-route-start-marker-retained: selected identity/TFTP/final identity are proved but no Talos route-start marker is retained.
- inconclusive-selected-normal-runtime-runtime-ready-marker-family-evidence: capture, staging, serial freshness, TFTP, or restore evidence fails the accepted identity/cursor rules.

## Findings

- fixed: scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh now requires and reports the runtime-blocked marker in addition to route-start and runtime-ready.
- fixed: scripts/rpi5-capture-invariant-proof-bundle.sh dry-run output now records required_success_marker and explicit fail-closed marker-family classifications.
- not-an-issue: no Rust source change is needed; the runtime route already emits route-start, runtime-blocked, and runtime-ready markers with the same capture nonce.
- not-an-issue: the v51 archive remains authoritative because the source/image contract did not change; only the review and capture contract were tightened.
- deferred: v57 must serialize the actual Pi 5 hardware preflight under hardwareTestLock before runtime-ready or packet-I/O can be accepted.

## Validation Evidence

- Archive review: tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56/validation/archive-review.stdout.txt.
- Capture dry-run: tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56/validation/capture-bundle-dry-run.json.
