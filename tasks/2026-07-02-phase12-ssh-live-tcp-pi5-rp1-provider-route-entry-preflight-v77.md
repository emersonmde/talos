# Phase 12 Live TCP Pi 5 RP1 Provider Route-Entry Preflight V77

Task: phase12-ssh-live-tcp-pi5-rp1-provider-route-entry-preflight-v77-20260702
Status: accepted
Terminal classification: provider-route-entry-blocked-before-kernel-main

## Summary

This task ran one serialized Pi 5 discriminator for the accepted v76 selected
archive. The selected archive staged, served, and retained final pre-restore
identity, but the selected serial window retained no early Talos marker,
kernel_main, provider route-start, runtime-blocked, or runtime-ready marker.
The result is a blocked feature frontier before kernel_main, not packet I/O,
OpenSSH compatibility, service success, ssh_ready=true, or a phase transition.

## Candidate Identity

- archive: target/talos-rpi5-rp1-provider-route-entry-v76-boot.tar.gz
- archive SHA-256:
  e2779312ef50ddb55573524c4079608c2d0ee0626cdb1e57e8e30b1c43269332
- archive bytes: 301590
- selected da591740/kernel_2712.img bytes: 155096
- selected da591740/kernel_2712.img SHA-256:
  6fc026100f0ea9e5157997eec12e1b3cc12000fdae243067f4349c7f4abffc20
- capture nonce: route-entry-v76
- selected tree hash:
  051125b2e9111036f7b5310634078a1e4e673bacbd53f4d05803f0882a460a70
- restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z
- restored tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

## Hardware Evidence

Accepted run:
tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-rp1-provider-route-entry-preflight-v77/candidate-selected-20260702T171320Z

- hardwareTestLock was acquired before selected archive publication and
  released only after restore proof.
- The v76 archive staged selected tree
  051125b2e9111036f7b5310634078a1e4e673bacbd53f4d05803f0882a460a70 with
  effective kernel_2712.img.
- Stable same-cursor TFTP before restore retained 13 events, including two
  selected da591740/kernel_2712.img serves at 155,096 bytes.
- Final pre-restore identity still matched the selected tree and selected
  kernel byte count.
- Serial used the saturated-cursor direct-read fallback, retained 4,475 bytes,
  and retained two firmware NETWORK markers.
- Serial retained zero occurrences of the route-entry marker family:
  provider route-start, runtime-blocked, and runtime-ready were all absent.
- Serial also retained no kernel_main marker, so the first missing feature fact
  is before kernel_main.
- The lab restored to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z with tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: The hardware proof was serialized under hardwareTestLock and captured
  selected TFTP service, selected tree identity, final pre-restore identity,
  restore identity, and marker-family counts.
- not-an-issue: The v76 archive and selected kernel identity matched the
  recorded candidate metadata before publication and in final pre-restore
  identity.
- not-an-issue: The saturated-cursor direct-read fallback was decisive because
  the identity join accepted the capture and the nonce-bearing marker family
  was absent.
- deferred: Provider route-start, runtime-blocked, runtime-ready, packet-I/O
  proof, OpenSSH/generated-root retry, compatibility, service success,
  ssh_ready=true, fake command expansion, broad shell work, and phase
  transition remain blocked or deferred.

## Evidence

- image/archive inspection: scripts/rpi5-archive-review.sh passed for the v76
  selected archive before publication.
- lab-controller API: selected publication identity matched tree
  051125b2e9111036f7b5310634078a1e4e673bacbd53f4d05803f0882a460a70 and
  selected da591740/kernel_2712.img at 155,096 bytes.
- serial hardware boot/output: direct-read serial retained 4,475 bytes, two
  firmware NETWORK markers, no kernel_main marker, and zero route-start,
  runtime-blocked, or runtime-ready marker-family occurrences.
- lab-controller API: TFTP stable same-cursor delta retained 13 events and two
  selected kernel serves at 155,096 bytes.
- lab-controller API: final pre-restore identity matched the selected tree; the
  lab restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Disposition

first_missing_fact: selected Pi 5 serial retained no Talos kernel_main or
provider route-entry marker after selected kernel TFTP service and final
pre-restore selected identity.

selected_next_task:
phase12-ssh-live-tcp-rp1-provider-route-entry-closeout-v77-20260702

planningNeeded: false

Redaction review: retained evidence is limited to archive/kernel digests,
file-event metadata, marker counts, concise serial classifications, lab tree
identities, and restore proof. No packet payloads, SSH key/session material,
private user data, unnecessary raw hardware log bytes, or stable external
identifiers are retained.
