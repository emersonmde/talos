# Phase 12 SSH Live TCP Selected-Fetch No-Entry Source Reconciliation

Task id: phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation-20260630

Status: accepted after commit.

Classification: candidate-entry-control-contract-required.

Evidence level: static inspection, accepted v9 evidence review,
non-published Pi 5 boot-tree/archive materialization, source/linker/startup
inspection, task-owned JSON evidence, docs build, and diff checks. No hardware,
lab publication, boot snapshot mutation, Pi 5 power action, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility, service success,
ssh-ready=true, broad shell work, or phase transition was performed.

## Goal

Reconcile the clean v9 selected-fetch/no-kernel_main boundary without repeating
the candidate hardware run or expanding to packet-I/O/OpenSSH.

## Scope Performed

- Reviewed the accepted v9 task record and task-owned evidence map.
- Rebuilt a non-published runtime-marker-route boot tree and archive from the
  current committed source, using the same selected da591740/kernel_2712.img
  fetch path and a fresh static review nonce.
- Inspected Image header fields, selected/root kernel byte identity, entry
  symbols, startup disassembly, the Pi 5 Rust entry route, runtime marker route
  tokens, and capture/identity assumptions.
- Preserved the existing quarantine on raw assembly early-entry provenance
  markers.
- Stopped before hardware, lab publication, packet-I/O, OpenSSH/generated-root
  retry, remote receipt, compatibility, service success, ssh-ready=true, broad
  shell work, or phase transition.

## Terminal Classification

candidate-entry-control-contract-required.

No bounded source/archive defect was found that explains v9's missing
TALOS: kernel_main marker:

- the selected da591740/kernel_2712.img path still materializes and matches the
  root kernel_2712.img;
- archive review reports text_offset=0, header_image_size=152152, flags=12,
  and a 152,152-byte kernel image;
- symbol inspection keeps _start and __kernel_start at 0x200000, rust_entry at
  0x20924c, boot::rpi5::kernel_main at 0x20bbb8, and __kernel_image_end at
  0x225258;
- startup disassembly branches over the Image header, clears BSS, sets the
  stack, and branches to rust_entry;
- the runtime-marker-route scenario reaches the normal Pi 5 Rust entry path and
  embeds the route-start, runtime-ready, capture-nonce, runtime-binding,
  descriptor-facing delivery, deterministic device-interface, ssh-ready=false,
  and fail-closed claim tokens.

Because v9 already proved selected TFTP fetch and final pre-restore identity
but retained no Talos Rust-side entry markers, the next smallest materially
different task is a minimal entry-control contract. It should keep the selected
fetch and earliest-entry mechanism while stripping the live TCP runtime route.

selected_next_task:
phase12-ssh-live-tcp-minimal-entry-control-contract-20260630.

planningNeeded: false.

## Findings

- not-an-issue: selected-path archive materialization still mirrors root Pi 5
  boot files under da591740/.
- not-an-issue: Image header, linker placement, startup branch path, BSS clear,
  stack setup, rust_entry, and kernel_main symbols match the accepted Pi 5
  Image contract.
- not-an-issue: the runtime-marker-route source path is wired through the
  normal Pi 5 Rust entry and embeds the expected runtime tokens.
- not-an-issue: v9 capture/identity evidence cleanly proved selected TFTP fetch
  and final pre-restore candidate identity before restore.
- deferred: raw TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO assembly markers
  remain quarantined because prior accepted evidence found that route invasive
  for accepted Pi 5 controls.
- deferred: hardware discrimination now needs the minimal entry-control
  contract; packet-I/O and OpenSSH/generated-root retry remain blocked.
- removed: no generated boot tree, archive, raw serial, raw TFTP log line, or
  packet payload artifact is retained outside ignored target/tmp outputs.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/evidence-map.json.
- Source/artifact inspection:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/static/source-artifact-inspection.md.
- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/validation/archive-review.stdout.txt.
- Runtime route archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/validation/runtime-ready-archive-review.stdout.txt.
- Kernel bytes/hash/header:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/static/kernel-bytes.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/static/kernel-sha256.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/static/image-header-words.txt.
- Symbols and startup disassembly:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/static/symbols.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation/static/entry-disassembly.txt.

## Redaction Review

This no-hardware task retained no raw serial text, raw TFTP peer/log-line
fields, packet payloads, key material, session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data. Evidence is limited to task ids, hashes, byte counts, symbol addresses,
static token checks, validation command results, and fixed classification
strings.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes.
- Static review of v9 task record and task-owned evidence map: pass.
- Non-published boot-tree/archive materialization and byte-level selected-image
  inspection: pass.
- Source/static review of linker/entry/startup/kernel_main path and earliest
  serial marker route: pass.
- cargo fmt --all -- --check: not run; Rust source was not touched.
- cargo -Zjson-target-spec test --quiet: not run; Rust runtime/source files
  were not touched.
- Focused script/unit validation: not run; helper/script files were not
  touched.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-minimal-entry-control-contract-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
