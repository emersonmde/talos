# Phase 12 SSH Live TCP Selected-Entry Boot-Path Reconciliation V25

Task id: phase12-ssh-live-tcp-selected-entry-boot-path-reconciliation-v25-20260701

Status: accepted after commit.

Classification: selected-entry-boot-path-rust-entry-discriminator-ready.

Evidence level: accepted v19/v20/v22/v23/v24 task and evidence inspection,
source/artifact/header/linker/output-path review, non-published selected
archive materialization, archive review, task-owned JSON evidence, docs build,
and diff checks. No hardware action, lab publication, boot snapshot mutation,
Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.

## Goal

Reconcile v24 selected-byte service versus absent earliest marker, prove or
replace the selected-entry discriminator contract, and select exactly one
bounded next task only if the contract is complete enough for serialized Pi 5
preflight.

## Scope Performed

- Promoted the ready no-hardware v25 task after v24 closeout accepted
  selected-kernel-entry-frontier-blocked-supervisor-planning.
- Compared v24 against accepted v19, v20, v22, and v23/v24 evidence.
- Reviewed Image/header/linker/_start provenance, selected artifact
  materialization, scenario-selection routing, and earliest output path
  assumptions.
- Rejected packet-I/O/OpenSSH/generated-root retry and phase expansion as
  successors.
- Materialized a non-published selected archive using the already accepted
  rpi5_rust_entry_uart10_marker_loop scenario and retained only metadata and
  review output.

## Reconciliation

v24 proved selected-byte service for the v23 _start discriminator image:
selected da591740/kernel_2712.img was served twice at 87,432 bytes, final
pre-restore identity stayed on the selected tree, serial capture was fresh
enough to retain firmware output, and restore returned the lab to the accepted
control tree. It did not prove selected Image execution, because the fresh
post-power serial window retained zero
TALOS: selected-kernel-entry-discriminator-v23 occurrences.

Static review found no header, linker, selected-mirroring, or scenario-routing
defect that explains the absence. The selected path still uses
KERNEL_IMAGE_TEXT_OFFSET=0, _start at the Image base, effective kernel
kernel_2712.img, and root/da591740 kernel equality. The weak assumption was the
compact _start marker as an observable output proxy: it was not retained in
v24, and a similar compact _start boot-contract shape was already weaker than
the later current-tree PASS and Rust-entry UART10 marker-loop evidence.

The replacement discriminator is therefore the existing
rpi5_rust_entry_uart10_marker_loop path. It still proves selected boot-path
entry before BootInfo parsing, target init, allocator, scheduler, RP1 MMIO,
networking, service code, or shell behavior, but it uses a repeated UART10
marker loop that Phase 11 hardware accepted with stable selected TFTP service
and 2,961 retained TALOS: reu10-loop occurrences. That prior run does not count
as current Phase 12 acceptance; it only justifies the replacement contract for
v26.

## Replacement Contract

- Source commit: 3d5f507a2f3fa6e122d0069ea53d0c15a4f4713c.
- Archive materialization:
  scripts/rpi5-rust-entry-uart10-marker-loop-archive.sh
  target/timer-irq-boot-source
  target/tmp/selected-entry-boot-path-v25-rust-entry-uart10.tar.gz.
- Selected fetch path: da591740/kernel_2712.img.
- Expected selected kernel: 45,400 bytes, SHA-256
  b597bc0d28aeda702492b9846ce9110ec5a99db6343c617a319ba265a0c59fa7.
- Non-published archive SHA-256:
  c24545db454f37bb830a2b3fbe06e9cf80fac6296b440f4305db9e7a4cc85ca2.
- Image header: text_offset=0, header_image_size=45,400, flags=12.
- Expected marker: TALOS: reu10-loop.
- Marker/output source path: _start -> rust_entry ->
  run_rust_entry_uart10_marker_loop through the accepted UART10 early-phase
  writer.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Fail-closed classifications: selected-entry-boot-path-marker-retained,
  blocked-selected-entry-boot-path-marker-missing, or
  selected-entry-boot-path-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, broad shell work,
fake/kernel-backed command expansion, and phase transition remain blocked.

## Findings

- fixed: reconciled v24 as selected-byte service without selected-entry marker
  proof and selected a bounded replacement discriminator.
- fixed: replacement artifact uses the hardware-proven repeated Rust-entry
  UART10 marker loop while preserving selected da591740/kernel_2712.img
  service.
- not-an-issue: Image header/linker/_start provenance, selected mirroring, and
  scenario routing are internally consistent in the materialized replacement
  archive.
- deferred: v26 must run serialized Pi 5 hardware evidence for this exact
  archive contract; Phase 11 hardware only justifies the contract choice.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-reconciliation-v25/evidence-map.json.
- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-reconciliation-v25/classification.json.
- Source/artifact review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-reconciliation-v25/static/source-artifact-review.txt.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-reconciliation-v25/validation/archive-review.stdout.txt.

## Redaction Review

Task-owned evidence retains task ids, source/commit/path labels, hashes, byte
counts, marker names, classifications, and validation outcomes. It does not
retain raw serial text, raw TFTP peer/log-line fields, packet payloads,
SSH/session/key material, boot artifact bytes, private user data, stable
secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Archive materialization and review: pass; generated archive bytes removed
  after metadata retention.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
