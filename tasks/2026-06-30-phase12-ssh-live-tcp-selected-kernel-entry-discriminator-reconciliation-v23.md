# Phase 12 SSH Live TCP Selected-Kernel-Entry Discriminator Reconciliation v23

Task id: phase12-ssh-live-tcp-selected-kernel-entry-discriminator-reconciliation-v23-20260630

Status: accepted after commit.

Classification: selected-kernel-entry-discriminator-repair-ready.

Evidence level: static accepted-evidence review, source/assembly/helper repair,
non-published Pi 5 boot-tree/archive materialization, selected root/image
equality review, task-owned JSON evidence, docs build, Rust format/build
checks, shell syntax checks, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power action, packet I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the v22 selected-byte/no-marker result with the smallest bounded
selected-kernel-entry discriminator before any packet-I/O, OpenSSH, generated
root, remote-receipt, compatibility, service-readiness, broad shell, or phase
transition work.

## Scope Performed

- Promoted the ready no-hardware v23 task after v22 closeout accepted
  minimal-entry-console-boundary-frontier-blocked-supervisor-planning.
- Reviewed accepted v19, v20, v21, v22 preflight, and v22 closeout evidence.
- Added rpi5_selected_kernel_entry_discriminator as a Pi 5 boot scenario with a
  dedicated assembly define.
- Added a _start-level UART10 marker,
  TALOS: selected-kernel-entry-discriminator-v23, that emits before CPACR setup,
  BSS clearing, stack setup, Rust entry, BootInfo/reporting, minimal-entry
  markers, networking, or service code, then parks in WFE.
- Added task-owned boot-tree and archive-review helpers that preserve the
  da591740/kernel_2712.img selected fetch path and reject later runtime marker
  tokens.
- Materialized a non-published selected-kernel-entry discriminator boot tree and
  archive, retained metadata/review output, and removed generated boot bytes.

## Terminal Classification

selected-kernel-entry-discriminator-repair-ready.

The accepted v22 first missing fact was selected-kernel entry visibility for the
minimal-entry boundary path: the clean v22 run proved selected-byte service for
the 69,816-byte v21 image but retained zero
capture-nonce=phase12-console-boundary-v21-static occurrences, including zero
direct early boundary-marker and zero post-boot-identity ready-marker
occurrences. Static inspection of the retained v22 serial window also found
firmware NETWORK output but no generic asm_start or asm_pre_rust_entry Talos
entry markers.

The v23 repair changes the discriminator boundary rather than retrying the v22
shape. The selected successor image writes exactly one task-owned marker at
_start and parks before the Rust/BootInfo/minimal-entry path. Non-published
archive review retained:

- source commit: this task's commit;
- scenario: rpi5_selected_kernel_entry_discriminator;
- selected fetch path: da591740/kernel_2712.img;
- expected marker: TALOS: selected-kernel-entry-discriminator-v23;
- marker source/console path: _start via talos_rpi5_early_uart_write on UART10;
- boundary stage: selected-kernel-entry-before-rust;
- selected kernel bytes: 87,432;
- selected kernel SHA-256:
  8051d7a600fe0867cfe093ffc6322ccdb532abaf58f323ece3f4013cca8054c7;
- archive SHA-256:
  8bab1eff73e2d4e8116c7f454bfeccb2494cf1c78e49b58fc44b9b6ca6be2199;
- Image header: text_offset=0, header_image_size=87,432, flags=12;
- root/selected kernel equality: true;
- later runtime markers absent:
  TALOS: minimal-entry-console-boundary-start,
  TALOS: minimal-entry-control-ready,
  TALOS: ssh-service-smoltcp-runtime, and
  rpi5-production-timer-preemption: PASS.

This is qualitatively different from v22 because v22 depended on entering
src/boot/rpi5.rs and then the later BootInfo/console path. v23 can distinguish
selected-kernel _start entry from a failure before the Rust boot path, while
still preserving selected TFTP identity and avoiding packet-I/O or service
claims.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24-20260630.

planningNeeded: false.

## Findings

- fixed: added a bounded _start-level selected-kernel-entry discriminator that
  emits before CPACR, BSS, stack, Rust, BootInfo, minimal-entry, networking, or
  service code.
- fixed: added boot-tree and archive-review helpers for the selected
  da591740/kernel_2712.img path and fail-closed marker review.
- not-an-issue: v22 selected-byte service, final selected identity, serial
  freshness, firmware NETWORK output, and restore proof remain decisive.
- not-an-issue: non-published v23 materialization preserves root/selected
  kernel equality, valid Image header fields, and selected fetch path.
- removed: the stale assumption that a Rust/BootInfo-level minimal-entry marker
  can decide selected-kernel _start entry after v22 showed no retained Talos
  marker at either direct early or post-boot-identity stages.
- deferred: only the serialized v24 Pi 5 preflight can decide whether the new
  _start marker is retained on hardware.
- deferred: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, broad shell work, fake command expansion,
  and phase transition remain blocked.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-reconciliation-v23/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-reconciliation-v23/classification.json.
- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-reconciliation-v23/validation/archive-review.stdout.txt.
- Static materialization metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-reconciliation-v23/static/.

## Redaction Review

Task-owned evidence records task ids, path labels, hashes, byte counts, Image
header fields, marker names, helper names, validation outcomes, and selected
successor metadata. It does not retain generated boot artifact bytes, packet
payloads, SSH keys/session material, private user data, stable secret-derived
identifiers, or raw hardware serial/TFTP logs.

## Validation

- git status --short --branch before edits/action: pass; main was ahead of
  origin with no uncommitted Talos repo changes.
- jq empty on referenced accepted JSON evidence and task-owned JSON evidence:
  pass.
- sh -n on touched shell helpers: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization and archive review: pass; generated
  boot bytes removed after metadata retention.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
