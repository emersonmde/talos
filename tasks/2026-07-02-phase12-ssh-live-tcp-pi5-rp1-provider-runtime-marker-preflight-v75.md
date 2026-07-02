# Phase 12 Live TCP Pi 5 RP1 Provider Runtime Marker Preflight V75

Task: phase12-ssh-live-tcp-pi5-rp1-provider-runtime-marker-preflight-v75-20260702
Status: accepted
Terminal classification: pi5-rp1-provider-runtime-marker-blocked

## Summary

This task ran the serialized Pi 5 proof for the v74 provider-bound runtime
marker candidate. The selected v74 archive staged and served successfully, and
the lab was restored to the named baseline, but the selected serial window did
not retain the expected provider-bound runtime-ready marker. This is a blocked
feature result, not a live packet I/O or SSH readiness claim.

## Candidate Identity

- archive: target/talos-rpi5-rp1-provider-runtime-marker-v74-boot.tar.gz
- archive SHA-256:
  15ac4dd758408676440babf4808adebbdd18bc0d9bbfc0e4a33a9f846b173665
- selected da591740/kernel_2712.img bytes: 154520
- selected da591740/kernel_2712.img SHA-256:
  8402ca01172608252eec7f6a933fb024b0cf782e57208e7102f5d8225e9d59f2
- selected tree hash:
  997bd5ddeaade62681e3d44481eb99b52bf253023a609c25d47fe1a1f11520dd
- restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z
- restored tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

Expected serial marker:

~~~text
TALOS: ssh-service-smoltcp-runtime-ready runtime-binding=accepted-deterministic-device-interface-delivery descriptor-facing-connection-delivered=true deterministic-device-interface-bound=true hardware-frame-provider-bound=true hardware-frame-provider-classification=rp1-ethernet-hardware-frame-provider-source-bound-local-only driver-packet-rx-frames=6 driver-packet-tx-frames=6 live-packet-io-accepted=false live-reachability-accepted=false remote-receipt-accepted=false compatibility-accepted=false ssh-ready=false claims-service-success=false claims-phase-transition=false
~~~

## Hardware Evidence

Accepted run:
tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-rp1-provider-runtime-marker-preflight-v75/candidate-rerun-clean-20260702T155657Z

- hardwareTestLock was acquired before publication and released only after
  restore proof.
- The clean run staged selected tree
  997bd5ddeaade62681e3d44481eb99b52bf253023a609c25d47fe1a1f11520dd.
- Stable same-cursor TFTP before restore retained 13 events, including two
  selected da591740/kernel_2712.img serves at 154520 bytes.
- Serial used the saturated-cursor direct-read fallback after an empty
  pre-power read and retained 4479 bytes with two firmware NETWORK markers.
- The expected provider-bound runtime-ready marker was absent:
  required_marker_present=false and provider-runtime-ready occurrences=0.
- Final pre-restore identity still matched the selected tree and selected
  kernel bytes.
- Restore returned the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z with tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The helper's identity join allowed the clean hardware classification. The
candidate-window v5 replay remains fail-closed because this route did not carry
a run-unique nonce and the required marker was not present after power; the task
uses the helper summary and selected-window identity evidence for the blocked
classification rather than treating that replay as retained-marker acceptance.

## Findings

- fixed: The first two proof-helper launches overlapped because long-running
  command polling was mishandled. Their evidence was discarded from the
  committed record and excluded from acceptance.
- fixed: A clean serialized rerun was captured after the lab was restored, with
  no concurrent proof helper processes.
- not-an-issue: The v74 archive and selected kernel identity matched the
  recorded candidate metadata before publication and in final pre-restore
  identity.
- not-an-issue: The clean run proves selected TFTP service but not Talos runtime
  marker progress; firmware NETWORK output is reboot/firmware evidence only.
- deferred: v72, v60, v53, packet-I/O proof, OpenSSH/generated-root retry,
  compatibility, service success, ssh_ready=true, fake command expansion, broad
  shell work, and phase transition remain blocked or deferred.

## Disposition

first_missing_fact: selected Pi 5 serial window did not retain the
provider-bound runtime-ready marker after selected kernel TFTP service and
final pre-restore selected identity.

selected_next_task:
phase12-ssh-live-tcp-rp1-provider-runtime-marker-closeout-v75-20260702

planningNeeded: false

Redaction review: retained evidence is limited to lab identity summaries, TFTP
file-event metadata, serial marker counts/excerpts, archive digests, and
restore proof. No packet payloads, SSH key/session material, private user data,
unnecessary hardware log bytes, or stable external identifiers are retained.
