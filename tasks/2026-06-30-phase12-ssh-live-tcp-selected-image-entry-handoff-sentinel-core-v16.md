# Phase 12 SSH Live TCP Selected-Image Entry Handoff Sentinel Core V16

Task id: phase12-ssh-live-tcp-selected-image-entry-handoff-sentinel-core-v16-20260630

Status: accepted after commit.

Classification: selected-image-handoff-sentinel-ready.

Evidence level: static/source review, source/build/helper implementation,
non-published Pi 5 boot-tree/archive materialization, image/archive/header/
symbol/disassembly inspection, fail-closed archive-review validation, task-owned
JSON evidence, docs build, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, hardwareTestLock
acquisition, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed
command expansion, broad shell work, or phase transition was performed.

## Goal

Create the smallest source/image-level sentinel that proves whether Pi 5
firmware branches into the selected kernel bytes before BSS clear, stack setup,
Rust entry, networking, packet I/O, OpenSSH, or shell behavior.

## Scope Performed

- Promoted this ready no-hardware task after the accepted v15 closeout.
- Compared the accepted v13 production-timer selected-path PASS evidence with
  the accepted v15 repaired minimal-entry selected-byte/no-entry evidence.
- Inspected the source owners for this boundary:
  src/arch/aarch64/boot.S, linker-rpi5.ld, build.rs, scripts/rpi5-boot-tree.sh,
  scripts/rpi5-minimal-entry-control-boot-tree.sh,
  scripts/rpi5-minimal-entry-control-archive-review.sh,
  scripts/rpi5-archive-review.sh, and the retained v13/v15 task/evidence maps.
- Added rpi5_selected_image_handoff_sentinel, an assembly-first scenario that
  emits TALOS: selected-image-handoff-sentinel-v16 from _start and parks before
  CPACR setup, BSS clear, stack setup, rust_entry, kernel_main, networking,
  packet I/O, OpenSSH, or shell work.
- Added selected-image handoff sentinel boot-tree and archive-review helpers.
- Materialized a non-published selected archive and retained root/prefixed
  da591740 kernel equality, arm64 Image header fields, _start/symbol/
  disassembly evidence, marker-token evidence, archive review output, and
  fail-closed archive-review output, then removed generated boot bytes.

## Terminal Classification

selected-image-handoff-sentinel-ready.

The existing v15 minimal-entry image already included TALOS: asm_start before
BSS clear, but its marker path continued into the normal boot ladder. The
dedicated v16 sentinel narrows the first missing fact: after the firmware
requests and receives selected da591740/kernel_2712.img bytes, does execution
reach _start far enough to write a single marker and park before every later
Talos boot phase?

The non-published selected archive keeps root and da591740/kernel_2712.img
identical. The selected kernel is 87,432 bytes, SHA-256
7a841135cb2e5d6bf9be11d900e8c9fbabbac32a43f539780a90adc50e374888, with arm64
Image header text_offset=0, header_image_size=87432, flags=12, and magic=ARMd.
The archive-review helper accepts the marker-bearing archive and fails closed
against a marker-missing production-timer archive. Disassembly shows _start
branches over the Image header, writes TALOS:
selected-image-handoff-sentinel-v16 through talos_rpi5_early_uart_write, then
parks in WFE before CPACR setup, BSS clear, stack setup, or rust_entry.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-entry-handoff-sentinel-preflight-v16-20260630.

planningNeeded: false.

## Findings

- fixed: added the dedicated rpi5_selected_image_handoff_sentinel scenario and
  TALOS_RPI5_HANDOFF_SENTINEL_SCENARIO assembly path.
- fixed: added selected-image handoff sentinel boot-tree and archive-review
  helpers with root/prefixed kernel equality, marker-token, and fail-closed
  checks.
- not-an-issue: v13 production-timer PASS and v15 selected-byte/no-entry are
  not contradictory; v13 proves selected-path entry works for a different
  control image, while v15 leaves the selected minimal-entry image handoff
  unproved below Rust.
- deferred: Pi 5 marker presence must be established by the serialized v16
  hardware preflight; packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, broad shell work, and phase transition stay
  blocked.
- removed: generated non-published boot tree and archive bytes were removed
  after metadata retention.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-handoff-sentinel-core-v16/evidence-map.json.
- Static/source/image review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-handoff-sentinel-core-v16/static/.
- Materialized archive metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-handoff-sentinel-core-v16/materialized/.
- Validation output:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-handoff-sentinel-core-v16/validation/.

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
  TALOS_BOOT_SCENARIO=rpi5_selected_image_handoff_sentinel: pass.
- Non-published archive materialization and selected-image handoff sentinel
  review: pass; root/prefixed kernel equality, Image header fields, marker
  token, and fail-closed missing-marker behavior retained.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-entry-handoff-sentinel-preflight-v16-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
