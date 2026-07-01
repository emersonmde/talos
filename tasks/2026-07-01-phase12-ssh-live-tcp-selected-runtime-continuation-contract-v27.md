# Phase 12 SSH Live TCP Selected Runtime Continuation Contract V27

Task id: phase12-ssh-live-tcp-selected-runtime-continuation-contract-v27-20260701

Status: accepted after commit.

Classification: selected-runtime-continuation-contract-ready.

Evidence level: static source/artifact inspection, non-published archive
materialization/review, targeted unit test, JSON evidence validation, docs
build, and diff checks. No hardware action, lab publication, boot snapshot
mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility claim, service success claim, ssh-ready=true, fake
command expansion, broad shell work, or phase transition was performed.

## Goal

Produce the thinnest selected normal-runtime continuation contract after v26
proved the selected Image reaches rust_entry and UART10 early output.

## Scope Performed

- Promoted the ready no-hardware v27 contract after the supervisor refreshed
  dependencies from the accepted v26 selected-entry closeout.
- Reused the existing rpi5_ssh_service_smoltcp_runtime_ready scenario rather
  than adding another synthetic entry discriminator.
- Materialized a non-published selected archive for hardware preflight review:
  target/tmp/selected-runtime-continuation-v27.tar.gz.
- Reviewed source routing and artifact metadata for the normal path from
  rust_entry through BootInfo parsing, target init, exceptions init,
  boot::rpi5::kernel_main, and the smoltcp runtime marker route.

## Terminal Classification

selected-runtime-continuation-contract-ready.

The selected normal-runtime continuation contract is ready for serialized Pi 5
preflight. The selected archive is built from source commit
fa2ae39ff6a96fdcc175b2b29deb1f02d6777828 using:

scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh target/timer-irq-boot-source target/tmp/selected-runtime-continuation-v27-boot-tree
tar -czf target/tmp/selected-runtime-continuation-v27.tar.gz -C target/tmp/selected-runtime-continuation-v27-boot-tree .

Selected archive SHA-256:
d8b5c23fd26d2dd236a3cd6206d7367832358b874f96b0b6ecf3f7fea1b439dc.

Selected fetch path: da591740/kernel_2712.img.

Selected kernel SHA-256:
665d993ab7c36065fa4810ae09613ed9d92aba30cdd5881e06e23b50b4d25a72.

Selected kernel size: 152,144 bytes.

Image header: text_offset=0, header_image_size=152144, flags=12.

Expected marker family for v28 hardware classification:

- TALOS: rust_entry.
- TALOS: boot info parsed.
- TALOS: target init.
- TALOS: exceptions ready.
- TALOS: kernel_main.
- TALOS: ssh-service-smoltcp-runtime-route-start with
  capture-nonce=runtime-marker-route-static and
  source=network-device-smoltcp-runtime.
- TALOS: ssh-service-smoltcp-runtime-ready with
  runtime-binding=accepted-deterministic-device-interface-delivery,
  descriptor-facing-connection-delivered=true,
  deterministic-device-interface-bound=true,
  hardware-frame-provider-bound=false, matched nonzero driver packet RX/TX
  frame counts, live-packet-io-accepted=false,
  live-reachability-accepted=false, remote-receipt-accepted=false,
  compatibility-accepted=false, ssh-ready=false,
  claims-service-success=false, and claims-phase-transition=false.

If the runtime-ready line is not retained, v28 must preserve the first missing
fact and continue to block packet-I/O/OpenSSH work.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28-20260701.

planningNeeded: false.

first_missing_fact: null.

## Findings

- fixed: selected the existing normal runtime route as the next thinnest
  feature path after the v26 rust_entry boundary.
- not-an-issue: rpi5_ssh_service_smoltcp_runtime_ready is not in the early
  diagnostic exclusion list, so it follows the normal rust_entry -> BootInfo
  -> target init -> exceptions -> kernel_main path before its route marker.
- not-an-issue: non-published archive review passed with root/selected kernel
  equality, selected da591740/kernel_2712.img mirror, valid Image header
  fields, and the expected selected kernel byte count/hash.
- fixed: recorded the exact fail-closed runtime contract: the route can prove
  deterministic descriptor/device runtime binding while still rejecting live
  packet I/O, remote receipt, compatibility, service success, ssh-ready=true,
  and phase transition claims.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Task-owned evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-contract-v27/evidence-map.json.
- Task-owned classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-contract-v27/classification.json.
- Source/artifact review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-contract-v27/static/source-artifact-review.txt.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-contract-v27/validation/archive-review.stdout.txt.

## Redaction Review

This task retained metadata only. It retained no raw serial text, raw TFTP
peer/log-line fields, packet payloads, SSH/session/key material, boot artifact
bytes, private user data, stable secret-derived identifiers, or unnecessary
hardware data. The archive itself remains under target/tmp and is not committed.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before v27 materialization.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- cargo -Zjson-target-spec test --quiet
  live_tcp_runtime_marker_route_report_reaches_fail_closed_runtime_path: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
