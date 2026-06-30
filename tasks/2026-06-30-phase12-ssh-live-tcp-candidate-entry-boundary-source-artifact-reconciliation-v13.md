# Phase 12 SSH Live TCP Candidate Entry Boundary Source/Artifact Reconciliation V13

Task id: phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13-20260630

Status: accepted after commit.

Classification: candidate-entry-control-contract-required.

Evidence level: accepted v12 task/evidence inspection, static
source/artifact/symbol/marker inspection, fresh non-published archive metadata
review, task-owned JSON evidence, docs build, and diff checks. No hardware
action, lab publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Review the v12 selected-fetch/no-runtime-marker boundary and either accept a
bounded source/helper/archive repair or define the next precise control
question before any rerun.

## Scope Performed

- Promoted this ready no-hardware task after the v12 closeout accepted
  candidate-kernel-entry-boundary-needs-supervisor-planning at commit
  15942cd6fcbd3a53bb6d589f8804bb576a63a472.
- Inspected the accepted v12 selected-fetch evidence, repaired TFTP contract,
  source commit 1a199ad3f5f8416af2d5088214c5a6d3bf433433, boot scenario
  routing, config/kernel path selection, Image/linker/startup entry symbols,
  marker emission path, and selected-control/current-tree context.
- Confirmed no source, script, linker, target, Cargo, or build-script files
  changed between the v12 candidate source commit and this task's static
  review head.
- Materialized a fresh non-published
  rpi5_ssh_service_smoltcp_runtime_ready archive for metadata/static review
  only, retained hashes/header fields/symbol and marker-token evidence, then
  removed the generated archive and boot tree before acceptance.

## Terminal Classification

candidate-entry-control-contract-required.

Static review found no bounded source/archive defect that explains selected
TFTP fetches with no visible kernel/runtime entry marker. The selected
da591740/kernel_2712.img mirror matches the root kernel_2712.img, the archive
review passed with valid Image header fields, and entry symbols include
_start, rust_entry, the top-level kernel_main, boot::rpi5::kernel_main, and
run_ssh_service_smoltcp_runtime_ready_route. The live TCP route-start marker is
emitted only after rust_entry, boot-info parsing, target init, exception init,
boot::rpi5::kernel_main, target service construction, and boot identity
reporting.

The first missing fact is therefore under-specified by v12 evidence: v12 proved
selected fetch and absence of TALOS: kernel_main plus route-start/runtime-ready
markers, but it did not contract a decisive check for the earlier rust_entry,
boot-info-parsed, target-init, and exceptions-ready phase lines. The next
mechanical step is a no-hardware control contract that defines one serialized
hardware/control discriminator for this entry boundary.

selected_next_task:
phase12-ssh-live-tcp-candidate-entry-control-contract-v13-20260630.

planningNeeded: false.

## Findings

- not-an-issue: v12 selected fetch identity is a real serial-prefixed mirror of
  the root kernel_2712.img, not a different selected kernel.
- not-an-issue: no source, script, linker, target, Cargo, or build-script files
  changed after the v12 candidate source commit before this review.
- not-an-issue: the live TCP runtime route is correctly later than kernel entry
  and therefore cannot explain a missing kernel_main marker by itself.
- deferred: v12 did not classify the earlier rust_entry, boot-info-parsed,
  target-init, and exceptions-ready phase lines, so the next hardware task needs
  an explicit control contract for those facts before rerun.
- fixed: the first local archive materialization used an over-64-character
  nonce and hit the build.rs nonce guard before image creation; the static
  review reran with the build-valid nonce v13src-105617.
- removed: packet-I/O/OpenSSH/generated-root retry as a permissible successor
  from the v13 source/artifact reconciliation; those remain blocked until a
  future task accepts candidate-capture-ready under explicit criteria.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13/candidate-entry-boundary-source-artifact-reconciliation-v13-20260630T105617Z/.
- Static evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13/candidate-entry-boundary-source-artifact-reconciliation-v13-20260630T105617Z/static-evidence-map.json.
- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13/candidate-entry-boundary-source-artifact-reconciliation-v13-20260630T105617Z/validation/rpi5-ssh-runtime-archive-review.stdout.txt.
- Entry symbols:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13/candidate-entry-boundary-source-artifact-reconciliation-v13-20260630T105617Z/static-review/entry-symbols.txt.
- Marker strings:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13/candidate-entry-boundary-source-artifact-reconciliation-v13-20260630T105617Z/static-review/kernel-marker-strings.txt.
- V12 source-diff check:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13/candidate-entry-boundary-source-artifact-reconciliation-v13-20260630T105617Z/static-review/source-diff-from-v12-candidate.txt.

## Redaction Review

This task retained no raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH/session/key material, boot artifact bytes, private user data,
stable secret-derived identifiers, or unnecessary hardware data. Generated
archive and boot-tree bytes were removed before acceptance.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before promotion.
- Static source/artifact/symbol/marker inspection: pass.
- Fresh non-published archive review: pass after rerun with a build-valid short
  nonce; the initial overlong nonce failed before image creation.
- Generated boot bytes removed before acceptance: pass.
- jq empty on task-owned JSON evidence/state files: pass.
- cargo fmt/tests: not run; no Rust source changed.
- shell helper checks: not run; no shell helper changed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-candidate-entry-control-contract-v13-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
