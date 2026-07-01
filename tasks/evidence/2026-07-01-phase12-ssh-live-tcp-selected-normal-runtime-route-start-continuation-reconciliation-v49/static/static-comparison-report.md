# V49 Route-Start Static Comparison

Task id: phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49-20260701

Classification: selected-normal-runtime-route-start-discriminator-ready.

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
  boot::rpi5::kernel_main on Pi 5 with TALOS: kernel_main
  capture-nonce=runtime-marker-route-static retained in fresh selected-byte
  hardware evidence.

## Route-Start Boundary

The v49 source change adds a selected normal-runtime route-start marker-loop
scenario. The scenario is selected by
rpi5_ssh_service_smoltcp_route_start_marker_loop, implies the normal runtime
route build, and runs from src/boot/rpi5.rs after the accepted kernel_main
frontier reaches the runtime route handoff point.

The first required successor marker is:

TALOS: ssh-service-smoltcp-runtime-route-start
capture-nonce=runtime-marker-route-static

The marker line records selected-normal-runtime-route-start=true and
claims-runtime-ready=false, claims-packet-io=false,
claims-service-success=false, claims-ssh-ready=false, and
claims-phase-transition=false. The marker-loop function intentionally matches
the existing non-diverging marker-loop pattern: it emits the marker in a loop,
but the downstream runtime-ready route remains linked so archive review can
confirm the selected normal-runtime service shape is still present.

## Non-Published Archive Contract

- Archive path: target/tmp/selected-normal-runtime-route-start-v49.tar.gz.
- Archive SHA-256:
  16a8c14c33430f09682b6cb5a725c75f1e392f2372671ed3fea4a8b39ac609a4.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  370cbe055e4836d9605a318704e9337e112f9cbdf57743addc7ec9b13ef28467.
- Selected kernel size: 152,640 bytes.
- Image header: text_offset=0, header_image_size=152640, flags=12,
  magic=ARMd.
- Root kernel_2712.img and da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Dispositions

- fixed: added the route-start marker-loop boot scenario and selected archive
  review helper.
- fixed: placed the route-start marker after the accepted kernel_main frontier
  and before runtime-ready, packet-I/O, service success, ssh-ready, or phase
  transition claims.
- fixed: kept the normal runtime route linked in the archive by using the
  established marker-loop shape rather than a statically divergent path.
- not-an-issue: runtime-ready strings remain embedded in the image because the
  selected normal-runtime route is intentionally preserved; the required
  route-start marker explicitly withholds runtime-ready and later claims.
- deferred: a future serialized Pi 5 preflight must prove the exact selected
  archive reaches the route-start marker before accepting route-start on
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
