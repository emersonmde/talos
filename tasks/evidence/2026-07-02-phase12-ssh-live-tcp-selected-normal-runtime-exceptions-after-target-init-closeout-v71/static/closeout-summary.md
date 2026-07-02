# V71 Exceptions Closeout Summary

Task id:
phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71-20260702.

## Reviewed Evidence

- v70 source/static reconciliation selected the exceptions-ready marker loop as
  the first post-target-init discriminator.
- v71 Pi 5 preflight accepted
  selected-normal-runtime-exceptions-marker-retained.
- v71 classification JSON records no identity-join rejection reasons.
- v71 evidence map points at one accepted candidate window,
  run-20260702T095454Z, and quarantines the incomplete helper rerun.

## Closeout Decision

The selected normal-runtime exceptions-ready frontier is proved. The accepted
v71 window preserves all required joins:

- Selected archive identity:
  target/tmp/selected-normal-runtime-exceptions-ready-v70.tar.gz,
  SHA-256 18007965ceb10991766e01ab2cf4d6f468530eca97d1a8c3a016a39b0402396b.
- Selected TFTP service: da591740/kernel_2712.img served four times at 152,880
  bytes.
- Serial marker retention: TALOS: exceptions ready
  capture-nonce=runtime-marker-route-static retained 881 times.
- Final pre-restore identity: selected tree
  b4c9bf0c09d122def872228a4e3d2a0f5836bfa0c7e4e4cdaa3b42ddf3e8ee9c.
- Restore proof: baseline snapshot
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z restored tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Next Boundary

The next explicit queued task is
phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-after-exceptions-reconciliation-v72-20260702.
It remains no-hardware until promoted separately. This closeout selects it
because the accepted frontier now reaches exceptions ready, and current source
already exposes the next ordered marker-loop boundary at
rpi5_ssh_service_smoltcp_kernel_main_marker_loop.

No route-start, runtime-ready, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility/service readiness, ssh-ready=true, fake command
expansion, broad shell work, or phase transition is selected or proved by this
closeout.
