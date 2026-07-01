# V51 Runtime-Ready Static Comparison

Task id: phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51-20260701

Classification: selected-normal-runtime-runtime-ready-discriminator-ready.

## Reconciled Frontier

- v34 proved the selected normal-runtime archive can reach the assembly entry
  marker on Pi 5.
- v36 proved the selected normal-runtime archive can reach the pre-rust marker
  on Pi 5.
- v38 proved the selected normal-runtime archive can enter rust_entry on Pi 5.
- v40 proved the selected normal-runtime archive can reach BootInfo parsing on
  Pi 5.
- v42 proved the selected normal-runtime archive can reach target init on Pi 5.
- v44 proved the selected normal-runtime archive can reach exceptions ready
  after target init and arch::aarch64::exceptions::init().
- v48 proved the selected normal-runtime archive reaches
  boot::rpi5::kernel_main on Pi 5.
- v50 proved the selected normal-runtime archive reaches TALOS:
  ssh-service-smoltcp-runtime-route-start
  capture-nonce=runtime-marker-route-static on Pi 5 with selected-byte TFTP
  evidence and restore proof.

## Runtime-Ready Boundary

The existing rpi5_ssh_service_smoltcp_runtime_ready route in src/target/rpi5.rs
first emits TALOS: ssh-service-smoltcp-runtime-route-start, then calls
crate::network::live_tcp_runtime_marker_route_report(). It emits TALOS:
ssh-service-smoltcp-runtime-ready only when marker_route_ready() is true.

The network report requires accepted deterministic descriptor/device-interface
delivery, descriptor-facing connection delivery, deterministic device-interface
binding, no hardware-frame provider binding, equal nonzero driver packet
rx/tx counts, and false values for live packet-I/O, live reachability, remote
receipt, compatibility, and ssh-ready.

The first required successor marker is:

TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static

The route then prints claims-service-success=false and
claims-phase-transition=false. That makes runtime-ready a bounded continuation
after the accepted route-start frontier and before packet-I/O,
OpenSSH/service-readiness, ssh-ready, fake command expansion, broad shell work,
or phase-transition claims.

## Non-Published Archive Contract

- Archive path: target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Archive SHA-256:
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- Selected kernel size: 152,144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12,
  magic=ARMd.
- Root kernel_2712.img and da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Dispositions

- fixed: selected the existing runtime-ready route as the next bounded
  discriminator after accepted route-start proof.
- fixed: materialized and reviewed a non-published v51 archive with a distinct
  runtime-ready capture nonce and matching selected root/da591740 kernel bytes.
- not-an-issue: no source change was needed because the route already gates
  runtime-ready on the deterministic runtime marker report and withholds later
  milestone claims.
- deferred: a future serialized Pi 5 preflight must prove the exact selected
  archive reaches the runtime-ready marker before accepting runtime-ready on
  hardware.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Redaction

Task-owned evidence retains task ids, paths, hashes, byte counts, marker names,
helper arguments, classifications, and validation outcomes. It does not retain
raw serial text, raw TFTP peer/log-line fields, packet payloads,
SSH/session/key material, boot artifact bytes, private data, or stable
secret-derived identifiers.
