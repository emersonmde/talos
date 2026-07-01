# Phase 12 SSH Live TCP Selected Normal Runtime Pre-Entry Reconciliation V31

Task id: phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-reconciliation-v31-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-pre-entry-discriminator-ready.

Evidence level: state inspection, static source inspection, source/helper
implementation, non-published archive materialization/review, shell syntax
validation, dry-run contract validation, cargo fmt/build, JSON validation, docs
build, and diff checks. No hardware action, hardwareTestLock acquisition, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake command expansion, broad shell work, or
phase transition was performed.

## Goal

Reconcile the v30 selected normal-runtime marker-missing result and produce the
thinnest pre-rust-entry discriminator for the same feature image.

## Scope Performed

- Promoted this ready no-hardware task after the accepted v30 closeout and
  verified the hardware lock was unlocked/restored with no active supervisor
  intervention.
- Reviewed v26 rust_entry/UART10 proof, v27/v29 selected normal-runtime
  contracts, v28/v30 marker-missing hardware evidence, and the selected-control
  production-timer baseline.
- Fixed the selected normal-runtime scenario to include assembly early-entry
  provenance markers before CPACR setup, BSS clearing, stack setup, rust_entry,
  BootInfo parsing, target init, exceptions, kernel_main, networking, or
  service code.
- Fixed the v30 capture helper summary bug by passing marker_family into the
  final capture-invariant-summary jq program.
- Materialized a non-published v31 candidate archive and recorded metadata
  without publishing it to the lab.

## Terminal Classification

selected-normal-runtime-pre-entry-discriminator-ready.

v30 proved selected TFTP service for the 152,144-byte normal-runtime image, but
the retained serial window had no ordered normal-runtime marker, including
TALOS: rust_entry. The next smallest feature discriminator is not packet I/O or
an OpenSSH retry; it is the same selected normal-runtime image with assembly
pre-entry provenance enabled so the next Pi 5 preflight can distinguish "Image
did not enter selected code" from "entered assembly but failed before Rust" and
from "reached Rust but failed later."

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32-20260701.

planningNeeded: false.

First missing fact entering v32: selected
rpi5_ssh_service_smoltcp_runtime_ready Image is served by TFTP, but the Pi 5
has not yet shown whether that selected Image reaches TALOS: asm_start,
TALOS: asm_pre_rust_entry, or TALOS: rust_entry.

## V32 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-pre-entry-v31-boot-tree
    tar -czf target/tmp/selected-normal-runtime-pre-entry-v31.tar.gz \
      -C target/tmp/selected-normal-runtime-pre-entry-v31-boot-tree .

- Source base before task changes:
  caa394c4f3e2ceb2cbd3b3e554cada3743c92d96.
- Archive path: target/tmp/selected-normal-runtime-pre-entry-v31.tar.gz.
- Archive SHA-256:
  a1227502fa059774e04c3ebc0b2c819305defa88abd4fbc07254e3166fcad451.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  c169c9553096f3bae24802762f14c03588fc6d6e811b732c8ac6515c47ca8f95.
- Selected kernel size: 152,144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family, in order:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
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

The v32 preflight must set TALOS_SERIAL_MARKER_FAMILY to this ordered family
and use TALOS: ssh-service-smoltcp-runtime-ready
capture-nonce=runtime-marker-route-static as the primary marker. The earliest
predecessor-named marker is TALOS: asm_start. If the final runtime-ready marker
is absent, the deepest retained family marker is the feature boundary.
Packet-I/O/OpenSSH/generated-root retry, remote receipt, compatibility/service
readiness, ssh-ready=true, fake command expansion, broad shell work, and phase
transition remain blocked.

## Findings

- fixed: build.rs now applies TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO to
  rpi5_ssh_service_smoltcp_runtime_ready, adding TALOS: asm_start and
  TALOS: asm_pre_rust_entry to the selected normal-runtime image.
- fixed: scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh now
  fail-closes if the selected archive lacks the assembly pre-entry markers.
- fixed: scripts/rpi5-capture-invariant-proof-bundle.sh now passes
  marker_family into the final summary jq program, resolving the v30 helper
  summary failure.
- not-an-issue: the normal Rust path still emits rust_entry, BootInfoParsed,
  TargetInit, ExceptionsReady, kernel_main, route-start, and runtime-ready
  source writes for rpi5_ssh_service_smoltcp_runtime_ready.
- not-an-issue: the selected image keeps the same 152,144-byte Image size and
  header shape while changing the kernel SHA because the assembly provenance
  markers are now linked in.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-reconciliation-v31/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-reconciliation-v31/evidence-map.json.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-reconciliation-v31/validation/archive-review.stdout.txt.
- Runtime archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-reconciliation-v31/validation/runtime-archive-review.stdout.txt.
- Archive/kernel metadata:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-reconciliation-v31/static/.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before v31 promotion.
- sh -n on touched shell helpers: pass.
- Capture bundle dry-run with TALOS_SERIAL_MARKER_FAMILY including asm_start,
  asm_pre_rust_entry, and rust_entry: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization, scripts/rpi5-archive-review.sh, and
  scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32-20260701.

planningNeeded: false.

Implementation commit: recorded in supervisor state after commit creation.
