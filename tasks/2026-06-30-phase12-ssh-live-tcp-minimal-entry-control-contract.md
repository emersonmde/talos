# Phase 12 SSH Live TCP Minimal Entry-Control Contract

Task id: phase12-ssh-live-tcp-minimal-entry-control-contract-20260630

Status: accepted after commit.

Classification: minimal-entry-control-proof-ready.

Evidence level: source/static inspection, non-published Pi 5 boot-tree/archive
materialization, archive/script validation, task-owned JSON evidence, Rust
fmt/test, docs build, and diff checks. No hardware, lab publication, boot
snapshot mutation, Pi 5 power action, live TCP, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility, service success,
ssh-ready=true, broad shell work, or phase transition was performed.

## Goal

Define and statically prove the smallest control archive that can determine
whether the selected Pi 5 boot path reaches Rust-side kernel_main independently
of the live TCP runtime route.

## Minimal Entry-Control Invariant

The later hardware discriminator must observe this order in one serialized
helper-owned Pi 5 window before accepting the control:

1. selected TFTP fetch of da591740/kernel_2712.img with reviewed bytes/hash.
2. existing TALOS: kernel_main early-phase marker from boot::rpi5::kernel_main.
3. nonce-bearing TALOS: minimal-entry-control-ready marker from the new
   rpi5_minimal_entry_control scenario.

This control uses the normal Pi 5 Image/startup/rust_entry/kernel_main path and
then stops immediately after the kernel_main marker. It strips the live TCP
runtime route, packet I/O, OpenSSH/generated-root retry, service-success
claims, and phase-transition claims. It does not use the quarantined raw
assembly early-entry provenance scenario.

## Terminal Classification

minimal-entry-control-proof-ready.

selected_next_task:
phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator-20260630.

planningNeeded: false.

## Findings

- fixed: added rpi5_minimal_entry_control to the boot-scenario allowlist and
  source cfg surface.
- fixed: added run_minimal_entry_control_marker, emitting the contract id,
  selected fetch path, nonce-bearing minimal marker, and fail-closed non-claims.
- fixed: wired the control immediately after the existing kernel_main
  early-phase line in boot::rpi5::kernel_main.
- fixed: added non-published materialization and static review helpers for the
  minimal entry-control archive.
- not-an-issue: archive review proves root and da591740/kernel_2712.img match,
  with kernel_size=52848, header_image_size=52848, text_offset=0, flags=12, and
  selected mirror files present.
- not-an-issue: symbol inspection retains _start/__kernel_start, rust_entry,
  boot::rpi5::kernel_main, run_minimal_entry_control_marker, and
  __kernel_image_end in the expected Image.
- deferred: Pi 5 hardware still must prove selected fetch, kernel_main, and the
  nonce-bearing minimal control marker in order.
- deferred: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, and phase
  transition remain blocked.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/evidence-map.json.
- Source/static contract:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/static/source-control-contract.md.
- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/validation/archive-review.stdout.txt.
- Minimal entry-control archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/validation/minimal-entry-control-archive-review.stdout.txt.
- Kernel bytes/hash/header:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/static/kernel-bytes.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/static/kernel-sha256.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/static/image-header-words.txt.
- Marker token and symbol review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/static/minimal-entry-control-token-review.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-control-contract/static/symbols.txt.

## Fail-Closed Classifications For The Later Hardware Task

- minimal-entry-control-passes: selected fetch identity, final pre-restore
  identity, restore proof, TALOS: kernel_main, and nonce-bearing
  TALOS: minimal-entry-control-ready all appear in order.
- blocked-selected-path-entry-control: selected fetch succeeds but kernel_main
  or the minimal control marker is absent.
- blocked-control-identity: selected fetch bytes/hash, final pre-restore
  identity, or selected mirror identity disagree.
- blocked-control-tftp-capture: the TFTP/log window cannot prove selected
  da591740/kernel_2712.img bytes before restore.
- blocked-restore: restore proof is missing or disagrees.
- inconclusive-with-required-discriminator: the first missing fact is named and
  no same-shaped retry is selected without supervisor planning.

## Redaction Review

This no-hardware task retained no raw serial text, raw TFTP peer/log-line
fields, packet payloads, key material, session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data. Evidence is limited to task ids, hashes, byte counts, static strings,
symbol names/addresses, validation command results, and fixed classification
strings.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes.
- Non-published control archive materialization/static inspection: pass.
- Focused helper/script validation: pass; rpi5-archive-review.sh and
  rpi5-minimal-entry-control-archive-review.sh accepted the archive.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pending before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
