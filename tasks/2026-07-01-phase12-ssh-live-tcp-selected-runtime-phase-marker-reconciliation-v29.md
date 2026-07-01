# Phase 12 SSH Live TCP Selected Runtime Phase Marker Reconciliation V29

Task id: phase12-ssh-live-tcp-selected-runtime-phase-marker-reconciliation-v29-20260701

Status: accepted after final validation and commit.

Classification: selected-runtime-phase-marker-discriminator-ready.

Evidence level: static source inspection, helper implementation, non-published
archive materialization/review, shell syntax validation, dry-run contract
validation, JSON validation, docs build, and diff checks. No hardware action,
hardwareTestLock acquisition, lab publication, boot snapshot mutation, Pi 5
power cycle, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, fake command
expansion, broad shell work, or phase transition was performed.

## Goal

Reconcile the v28 marker-missing result and produce the thinnest corrected
early-phase boundary discriminator for the selected normal-runtime path.

## Scope Performed

- Promoted this ready task after the accepted v28 closeout and verified the
  hardware lock was unlocked/restored with no active supervisor intervention.
- Reviewed v26 rust_entry/UART10 proof, the v27
  rpi5_ssh_service_smoltcp_runtime_ready contract, v28 hardware summaries, and
  the current rust_entry -> BootInfoParsed -> TargetInit -> ExceptionsReady ->
  kernel_main path.
- Fixed the capture helper contract so the next run can retain ordered
  marker-family counts and deepest_present_marker instead of only the final
  runtime marker plus kernel_main.
- Materialized a non-published v30 candidate archive and recorded metadata
  without publishing it to the lab.

## Terminal Classification

selected-runtime-phase-marker-discriminator-ready.

The normal-runtime source path already emits the needed marker family. The
missing implementation was the retained evidence contract: v28's helper
summaries counted only TALOS: kernel_main and the final runtime-ready marker,
so they could not decide whether selected execution stopped before BootInfo,
target init, exceptions, kernel_main, route-start, or runtime-ready. The v30
run is justified because the helper now records the full ordered marker family
and the deepest retained marker before interpreting final marker absence.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30-20260701.

planningNeeded: false.

First missing fact entering v30: selected rpi5_ssh_service_smoltcp_runtime_ready
Image is served by TFTP, but retained v28 evidence does not identify the
deepest reached early normal-runtime marker before kernel_main/runtime marker
absence.

## V30 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-runtime-phase-marker-v30-boot-tree
    tar -czf target/tmp/selected-runtime-phase-marker-v30.tar.gz \
      -C target/tmp/selected-runtime-phase-marker-v30-boot-tree .

- Source commit for boot-image code: 7001a4e0227a48cac4a02547fd1eec62f0b3bf8c.
- Archive path: target/tmp/selected-runtime-phase-marker-v30.tar.gz.
- Archive SHA-256:
  6dd1a37c9979bfee4ef83d6b2bb12863a07ed4f7451434d3e011d8e68eb589d1.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  665d993ab7c36065fa4810ae09613ed9d92aba30cdd5881e06e23b50b4d25a72.
- Selected kernel size: 152,144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family, in order:

- TALOS: rust_entry.
- TALOS: boot info parsed.
- TALOS: target init.
- TALOS: exceptions ready.
- TALOS: kernel_main.
- TALOS: ssh-service-smoltcp-runtime-route-start
  capture-nonce=runtime-marker-route-static
  source=network-device-smoltcp-runtime.
- TALOS: ssh-service-smoltcp-runtime-ready
  capture-nonce=runtime-marker-route-static.

The v30 preflight must set TALOS_SERIAL_MARKER_FAMILY to this ordered family
and use TALOS: ssh-service-smoltcp-runtime-ready
capture-nonce=runtime-marker-route-static as the primary marker. If the final
runtime-ready marker is absent, the deepest retained family marker is the
feature boundary. Packet-I/O/OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, and phase transition remain blocked.

## Findings

- fixed: early-phase marker evidence retention now records ordered
  marker_family counts and deepest_present_marker in
  scripts/rpi5-observe-serial-window.sh.
- fixed: scripts/rpi5-capture-invariant-proof-bundle.sh now carries the
  marker family into dry-run contracts and serial freshness summaries.
- not-an-issue: early-phase marker routing already exists for rust_entry,
  BootInfoParsed, TargetInit, ExceptionsReady, kernel_main, route-start, and
  runtime-ready.
- not-an-issue: UART10 early-phase writes flush after each byte and route
  completion; no source flush defect was found.
- not-an-issue: panic/alloc-error visibility already emits retained TALOS
  panic/alloc text and flushes UART10 before halt/spin.
- not-an-issue: rpi5_ssh_service_smoltcp_runtime_ready is included in Pi 5
  scenario cfg and is not excluded from the normal runtime path.
- not-an-issue: selected archive identity remains the v27/v28 kernel contract
  with unchanged 152,144-byte selected Image SHA-256.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-reconciliation-v29/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-reconciliation-v29/evidence-map.json.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-reconciliation-v29/validation/archive-review.stdout.txt.
- Archive/kernel metadata:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-reconciliation-v29/static/.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before v29 promotion.
- sh -n on touched shell helpers: pass.
- Capture bundle dry-run with TALOS_SERIAL_MARKER_FAMILY: pass.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30-20260701.

planningNeeded: false.

Implementation commit: ec950e11e6dadc1f4db879db3365ab04b5048125.
