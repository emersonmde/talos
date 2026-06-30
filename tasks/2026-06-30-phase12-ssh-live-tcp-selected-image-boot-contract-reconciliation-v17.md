# Phase 12 SSH Live TCP Selected-Image Boot Contract Reconciliation V17

Task id: phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17-20260630

Status: accepted after commit.

Classification: selected-image-boot-contract-discriminator-ready.

Evidence level: static/source review, source/build/helper implementation,
non-published Pi 5 boot-tree/archive materialization, image/archive/header/
symbol/disassembly inspection, task-owned JSON evidence, docs build, and diff
checks. No hardware action, hardwareTestLock acquisition, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, minimal-entry route
repair, broad shell work, or phase transition was performed.

## Goal

Reconcile the first missing v16 fact in source/static terms: selected
da591740/kernel_2712.img bytes are served, but no _start-level handoff
sentinel marker appears after firmware NETWORK.

## First-Principles Problem Statement

For the normal Pi 5 firmware contract, Talos stages an arm64 Image as
kernel_2712.img, mirrors it under da591740/kernel_2712.img, removes forced
kernel_address, and lets firmware load the selected Image at the firmware
chosen address. The Image header at _start advertises text_offset=0,
image_size=__kernel_image_end - _start, flags=12, and magic=ARMd. After the
header branch, firmware/BL31 should enter _start with x0 carrying the DTB
pointer; Talos then runs its entry code.

The invariant that should hold is: if the selected da591740/kernel_2712.img
bytes are served, the final pre-restore tree still names that selected image,
and restore proof succeeds, then a discriminator whose first post-header
instruction repeatedly writes a compact UART10 marker from _start should either
produce that marker or create a decisive firmware/image-handoff blocker. The
discriminator must not depend on BSS clear, stack setup, rust_entry,
kernel_main, networking, packet I/O, OpenSSH, or shell behavior.

## Contradicting Evidence

- Accepted v13/current-tree production-timer controls prove that selected-path
  Pi 5 archives with the same header shape, selected mirror, and
  effective_kernel=kernel_2712.img can reach the downstream
  rpi5-production-timer-preemption: PASS marker.
- Accepted v15 minimal-entry evidence proved selected-byte service for the
  repaired minimal-entry image, but the fresh serial window retained firmware
  NETWORK only and no assembly/Rust entry markers.
- Accepted v16 proved selected-byte service for a one-shot _start handoff
  sentinel image at 87,432 bytes, but the fresh serial window retained no
  TALOS: selected-image-handoff-sentinel-v16 or later Talos marker.

## Unproven Assumptions

- Whether the v16 one-shot helper marker is visible enough to distinguish
  _start absence from marker-helper/capture shape failure.
- Whether the accepted production-timer selected-path PASS can be used to infer
  this smaller selected image's _start handoff without a current-image
  discriminator.
- Whether a one-shot _start marker can survive serial timing as reliably as the
  accepted repeated UART10 marker-loop pattern.

## Reconciliation

Static review did not find a header, linker, config, kernel naming, selected
mirror, or root/selected equality defect:

- linker-rpi5.ld still places _start and __kernel_start at 0x200000 with
  KERNEL_IMAGE_TEXT_OFFSET=0.
- src/arch/aarch64/boot.S still branches over the Image header before Talos
  entry logic.
- scripts/rpi5-boot-tree.sh removes kernel_address and stages
  kernel_2712.img/kernel8.img under the normal selected-path contract.
- The materialized v17 discriminator archive keeps root and
  da591740/kernel_2712.img byte-identical.
- The materialized Image has text_offset=0, header_image_size=87,432,
  flags=12, magic=ARMd, and required marker TALOS: boot-contract-v18.

The concrete issue found is that v16 collapsed two facts into one hardware
result: firmware-to-_start handoff and a one-shot assembly helper marker. The
next discriminator therefore changes topology instead of retrying v16. It adds
rpi5_selected_image_boot_contract_discriminator, whose _start path repeatedly
writes a compact TALOS: boot-contract-v18 marker through UART10 using inline
per-byte writes and FR flushes, with no BL helper, no BSS/stack/Rust path, and
no later feature route.

## Terminal Classification

selected-image-boot-contract-discriminator-ready.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18-20260630.

planningNeeded: false.

The v18 hardware task must publish only the predecessor-named non-published
archive shape from this task, require selected da591740/kernel_2712.img at
87,432 bytes with SHA-256 fb501f7374888158c60f090b3cc0805f8fda97d98fd18e966c307310b5c00753,
expect repeated TALOS: boot-contract-v18, restore to
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, and fail closed without
selecting packet-I/O or OpenSSH.

## Findings

- fixed: added rpi5_selected_image_boot_contract_discriminator and
  TALOS_RPI5_BOOT_CONTRACT_DISCRIMINATOR_SCENARIO.
- fixed: added selected-image boot-contract boot-tree and archive-review
  helpers that require root/prefixed kernel equality, the v18 marker, and the
  absence of v16/minimal-entry/live-TCP markers.
- fixed: materialized a non-published v18 discriminator archive and retained
  header, symbol, disassembly, marker-token, selected-root equality, and
  fail-closed review evidence.
- not-an-issue: Image header/text_offset/image_size/flags, linker
  load/entry symbols, boot-tree config/kernel naming, and selected/root kernel
  equality are consistent with the normal Pi 5 firmware contract.
- not-an-issue: v13/v15/v16 are not contradictory; v13 proves a downstream
  selected-path control can pass, while v15/v16 prove selected-byte service
  without a retained marker for smaller entry images.
- deferred: serialized Pi 5 hardware must decide whether the repeated compact
  _start marker appears.
- removed: v16 same-shape rerun, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, service readiness, minimal-entry repair, broad shell work,
  and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17/classification.json.
- Static/source contract:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17/static/discriminator-contract.txt.
- Header/symbol/disassembly review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17/static/image-header.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17/static/symbols.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17/static/start-disassembly.txt.
- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17/validation/boot-contract-discriminator-archive-review.stdout.txt.

## Redaction Review

Task-owned evidence records task ids, source commit labels, path labels,
hashes, byte counts, marker labels, source/symbol/disassembly snippets,
validation command results, and selected successor metadata. It retains no raw
hardware serial text, raw TFTP peer/log-line fields, packet payloads,
SSH/session/key material, boot artifact bytes, private user data, stable
secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned promotion.
- sh -n on touched shell helpers: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json with
  TALOS_BOOT_SCENARIO=rpi5_selected_image_boot_contract_discriminator: pass.
- Non-published archive materialization and fail-closed archive review: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
