# Phase 12 SSH Live TCP Selected-Image Entry Invariant Reconciliation V15

Task id: phase12-ssh-live-tcp-selected-image-entry-invariant-reconciliation-v15-20260630

Status: accepted after commit.

Classification: selected-image-entry-source-repair-ready.

Evidence level: no-hardware source/helper review, non-published Pi 5
boot-tree/archive materialization, archive/token inspection, task-owned JSON
evidence, docs build, and diff checks. No hardware action, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Goal

Find and repair, or precisely classify, the smallest selected-image entry
invariant that explains why the v14 selected minimal current-tree image was
served by TFTP but emitted no Talos entry markers.

## Selected Invariant

For a selected Pi 5 kernel_2712.img after firmware NETWORK output, the
selected image must expose an entry-progress ladder that can fail closed:

1. TALOS: asm_start from _start immediately after the arm64 Image header branch.
2. TALOS: asm_pre_rust_entry after BSS clear and stack setup, before rust_entry.
3. TALOS: rust_entry, boot-info-parsed, target-init, exceptions-ready, and
   kernel_main from the normal Rust entry route.
4. The nonce-bearing TALOS: minimal-entry-control-ready marker from the
   minimal entry-control scenario.

The contradicting v14 evidence is precise: the selected run-unique minimal
entry-control image was served twice as da591740/kernel_2712.img at 52,848
bytes with final pre-restore identity on tree 59b29c6..., but retained serial
contained firmware NETWORK output only. It did not contain rust_entry,
boot-info-parsed, target-init, exceptions-ready, kernel_main, or the
nonce-bearing minimal-entry-control-ready marker.

## Repair

The v14 minimal control contract was too late and too coarse for the missing
fact it needed to resolve. Its required marker lived in boot::rpi5::kernel_main,
so absence of the marker could not distinguish firmware-selected-image
non-entry from pre-Rust entry failure. The production-timer control reached a
downstream PASS marker under the selected-path contract, but it did not make
early phase-line absence decisive for the minimal path.

This task made the minimal entry-control route fail closed at the earliest
selected-image boundary:

- build.rs now defines TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO for only
  rpi5_minimal_entry_control.
- scripts/rpi5-minimal-entry-control-archive-review.sh now requires both
  TALOS: asm_start and TALOS: asm_pre_rust_entry in addition to the existing
  nonce, minimal-entry-control, selected-fetch, and non-claim tokens.

This is not an acceptance shim. The old v14 run remains blocked-control-entry.
The repair makes the next selected-image preflight classify whether the
selected image reaches _start, reaches Rust, reaches kernel_main, or reaches
the minimal marker.

## Static Materialization

Non-published static materialization used:

- boot source: target/tmp/rpi5-observed-gpio-status-known-good-tree.
- static nonce: phase12-entry-v15-static.
- helper: scripts/rpi5-minimal-entry-control-boot-tree.sh.
- archive review helper: scripts/rpi5-minimal-entry-control-archive-review.sh.
- selected path: da591740/kernel_2712.img.
- kernel byte count: 52,832.
- kernel SHA-256: 16d1fe575ea47b5863b26019786dc8bc25de40ff9d84df99277c7aeb41cb5643.
- non-published archive SHA-256:
  599195f80594583e2bf55cfc742fbba5c260080ca9948f50fcc8e80536a19756.
- Image header: text_offset=0, header_image_size=52,832, flags=12.

The generated boot tree and archive bytes were removed after metadata and
token evidence were retained.

## Findings

- fixed: rpi5_minimal_entry_control now includes the existing assembly-level
  early-entry provenance markers.
- fixed: minimal entry-control archive review now rejects archives that omit
  TALOS: asm_start or TALOS: asm_pre_rust_entry.
- fixed: retained non-published static evidence proving the repaired selected
  image contains the assembly-entry, minimal-entry, nonce, selected-fetch, and
  non-claim tokens.
- deferred: the serialized Pi 5 preflight must rebuild a run-unique repaired
  archive and capture hardware evidence before any packet-I/O or OpenSSH path
  can be reconsidered.
- not-an-issue: TALOS_CAPTURE_NONCE is retained as a separate string fragment;
  runtime UART writes concatenate capture-nonce= and the nonce, so archive
  review checks both tokens rather than requiring one contiguous image string.
- removed: blind v14 rerun, packet-I/O, OpenSSH/generated-root retry, remote
  receipt, compatibility/service readiness claim, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-invariant-reconciliation-v15/evidence-map.json.
- Static materialization metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-invariant-reconciliation-v15/materialized/.
- Validation output:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-invariant-reconciliation-v15/validation/.

## Next Task

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15-20260630.

The successor must rebuild its own run-unique repaired minimal entry-control
archive, record source/archive/kernel identity, require TALOS: asm_start,
TALOS: asm_pre_rust_entry, rust_entry, boot-info-parsed, target-init,
exceptions-ready, kernel_main, and the nonce-bearing
minimal-entry-control-ready marker contract, restore to
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, and fail closed if selected
identity, TFTP byte agreement, final pre-restore identity, restore proof, or
any required marker boundary is missing.

Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility
claims, service success, ssh-ready=true, broad shell work, and phase transition
remain blocked.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned state promotion.
- sh -n scripts/rpi5-minimal-entry-control-boot-tree.sh: pass.
- sh -n scripts/rpi5-minimal-entry-control-archive-review.sh: pass.
- Non-published minimal entry-control boot-tree/archive materialization: pass.
- scripts/rpi5-minimal-entry-control-archive-review.sh on repaired archive:
  pass; required assembly and minimal-entry tokens were retained.
- cargo fmt --all -- --check: recorded in validation before commit.
- jq empty on task-owned JSON evidence and supervisor state: recorded in
  validation before commit.
- git diff --check: recorded in validation before commit.
- /home/node/.cargo/bin/mdbook build: recorded in validation before commit.
- git diff --cached --check: recorded before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
