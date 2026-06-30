# Phase 12 SSH Live TCP Minimal-Control Scenario-Specific Reconciliation

Task id: phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation-20260630

Status: accepted after commit.

Classification: diagnostic-control-quarantined-candidate-contract-ready.

Evidence level: static accepted-evidence comparison, non-published Pi 5
candidate boot-tree/archive materialization, archive/helper validation,
task-owned JSON evidence, docs build, and diff checks. No hardware, lab
publication, boot snapshot mutation, Pi 5 power action, live TCP candidate
retry, packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility
claim, service success claim, ssh-ready=true, broad shell work, or phase
transition was performed.

## Goal

Reconcile the failed minimal entry-control diagnostic after a current-tree
production-timer control proved that current-tree selected bytes still reach
downstream Talos execution under the selected-path capture contract.

## Scope Performed

- Promoted this queued no-hardware reconciliation only after the serialized
  current-tree production-timer discriminator accepted
  current-tree-entry-path-passes-control and selected this exact task.
- Compared the minimal-control contract/blocker, selected known-good baseline,
  entry-artifact delta reconciliation, and current-tree production-timer Pi 5
  control.
- Materialized a non-published
  rpi5_ssh_service_smoltcp_runtime_ready candidate boot tree/archive for static
  review only, then removed generated boot bytes after retaining manifests,
  hashes, header fields, token review, and archive-review output.
- Stopped before hardware, lab publication, candidate publication, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, or phase transition.

## Terminal Classification

diagnostic-control-quarantined-candidate-contract-ready.

The earlier minimal entry-control diagnostic is now quarantined as a
scenario-specific control rather than a broad current-tree entry blocker. Its
acceptance contract required early TALOS: kernel_main and nonce-bearing
TALOS: minimal-entry-control-ready markers. Both accepted downstream controls
prove that those early markers are metadata-only absent under the current
selected-path capture policy:

- selected known-good a0452458... emitted
  rpi5-production-timer-preemption: PASS with two 104,136-byte selected
  da591740/kernel_2712.img serves while TALOS: kernel_main was absent;
- current-tree production-timer emitted rpi5-production-timer-preemption: PASS
  twice with selected current-tree da591740/kernel_2712.img serves while
  TALOS: kernel_main was absent.

The current-tree production-timer control is the decisive new fact: current-tree
selected bytes can reach Rust-side downstream execution on Pi 5 under the same
selected-path capture contract. Therefore the minimal-control no-marker result
does not justify another minimal-control v2 proof before defining the direct
live TCP candidate preflight contract.

Static candidate review proves the next contract is objectively checkable
without hardware: the non-published
rpi5_ssh_service_smoltcp_runtime_ready archive keeps root and
da591740/kernel_2712.img identical at 152,152 bytes, SHA-256
f6d12eb3377cb4dc545057b137e151e318cc8a76a4b91ac007b98631c964850f, valid
Image header fields, and the route-start/runtime-ready marker tokens for
capture nonce scenario-reconciliation-runtime-v10.

selected_next_task:
phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10-20260630.

planningNeeded: false.

## Findings

- not-an-issue: the selected-path capture contract is not generically broken;
  accepted known-good and current-tree production-timer controls both emitted
  downstream PASS markers.
- fixed: the minimal entry-control diagnostic is quarantined as
  scenario-specific because its required early markers are metadata-only absent
  in both accepted downstream PASS controls.
- fixed: non-published candidate materialization and archive review prove the
  direct live TCP candidate preflight contract is objectively checkable in the
  selected next no-hardware task.
- deferred: candidate publication, Pi 5 power, packet-I/O, OpenSSH/generated-root
  retry, remote receipt, compatibility, service success, ssh-ready=true, broad
  shell work, and phase transition remain blocked.
- removed: generated non-published candidate boot tree and archive bytes were
  removed after metadata retention.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/evidence-map.json.
- Accepted evidence summaries:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/static/current-tree-production-timer-pass-summary.json,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/static/selected-known-good-pass-summary.json,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/static/entry-artifact-delta-summary.json,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/static/minimal-entry-control-contract-summary.json.
- Candidate archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/validation/candidate-archive-review.stdout.txt.
- Candidate metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/materialized/candidate/boot-tree-manifest.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/materialized/candidate/archive-sha256.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/static/candidate-kernel-sha256.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/static/candidate-image-header-words.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation/static/candidate-token-review.txt.

## Redaction Review

This no-hardware task retained no raw serial text, raw TFTP peer/log-line
fields, packet payloads, key material, session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data. Evidence is limited to task ids, hashes, byte counts, static strings,
validation command results, and fixed classification strings.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Static comparison of accepted current-tree production-timer PASS evidence
  against failing minimal-control/runtime-marker evidence: pass.
- Non-published Pi 5 candidate boot-tree/archive materialization and static
  review: pass; generated boot bytes removed after metadata retention.
- rpi5 archive/helper validation: pass; selected da591740/kernel_2712.img
  visibility, header fields, byte/hash metadata, route-start/runtime-ready
  marker tokens, and fail-closed non-claims are retained as static evidence.
- cargo fmt --all -- --check: not run; Rust source was not touched.
- cargo -Zjson-target-spec test --quiet: not run; Rust source or target
  routing was not touched.
- sh -n: not run; shell helpers/classifiers were not touched.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
