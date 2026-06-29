# Phase 12 SSH Live TCP Selected Fetch Path Reconciliation

Task id: phase12-ssh-live-tcp-selected-fetch-path-reconciliation-20260629

Status: accepted after commit.

Classification: fetch-path-ready.

Evidence level: static inspection, shell syntax check, boot-tree/archive
materialization, archive/static marker review, task-owned JSON evidence, docs
build, and diff checks.

## Goal

Repair or reconcile the selected Pi 5 boot fetch path after v7 stopped at
blocked-candidate-identity, without Pi 5 power-cycle, packet-I/O, OpenSSH
retry, compatibility, service success, ssh-ready=true, broad shell work, or a
phase transition.

## Scope Performed

- Promoted the ready selected-fetch-path reconciliation after v7 accepted
  blocked-candidate-identity.
- Preserved the accepted candidate capture contract's expected fetch path:
  da591740/kernel_2712.img.
- Repaired the runtime-marker-route boot-tree helper so it mirrors the root Pi
  5 boot files under the da591740/ serial-prefixed directory, matching the
  established Pi 5 hardware proof helper pattern.
- Materialized a fresh runtime-marker-route boot tree and archive from the v7
  source lineage and reviewed it locally.
- Stopped before lab publication, Pi 5 power-cycle, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, or phase transition.

## Terminal Classification

fetch-path-ready.

The repaired runtime-marker-route archive contains both root kernel files and
the selected serial-prefixed mirror. Static review proved
da591740/kernel_2712.img is present, has 152,168 bytes, and has SHA-256
24e6cd252fcabc0b34b37009e43269bc97630c4cb82f8577082f4d31a496bc63, matching
the root kernel_2712.img. The existing expected_fetch contract remains
unchanged; root-only kernel visibility is still insufficient for
candidate-capture-ready.

selected_next_task: phase12-ssh-live-tcp-pi5-candidate-preflight-v8-20260629.

planningNeeded: false.

## Findings

- fixed: the runtime-marker-route boot-tree helper omitted the
  serial-prefixed da591740/ mirror required by the accepted candidate capture
  contract.
- fixed: archive/static marker review now proves the runtime marker route and
  selected da591740/kernel_2712.img fetch path in one locally materialized
  archive.
- not-an-issue: no-power lab publication was unnecessary because the repair was
  to archive shape, and static archive review retained the selected fetch-path
  byte/hash evidence required before publication.
- not-an-issue: the accepted capture contract remains
  da591740/kernel_2712.img; no dependent helper or doc was relaxed to accept
  root-only visibility.
- deferred: v8 remains responsible for serialized lab publication, candidate
  preflight, helper-owned restore, and any candidate-capture-ready decision.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-selected-fetch-path-reconciliation/evidence-map.json.
- Boot-tree manifest:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-selected-fetch-path-reconciliation/validation/runtime-marker-boot-tree-manifest.txt.
- Selected fetch-path static review:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-selected-fetch-path-reconciliation/validation/selected-fetch-path-static-review.txt.
- Archive/static marker review:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-selected-fetch-path-reconciliation/validation/runtime-marker-archive-review.stdout.txt.

## Redaction Review

No hardware run, lab publication, serial capture, TFTP delta, packet payload,
remote peer identifier, key material, session material, or boot artifact bytes
were retained. Durable evidence keeps task ids, source/archive path labels,
archive digest, kernel byte count/hash, helper output, validation outputs, and
claim-boundary metadata only.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned edits.
- Static inspection of v7 blocker, runtime-marker-route archive helper, and
  expected_fetch contract: pass.
- sh -n for the touched runtime-marker-route boot-tree helper: pass.
- Runtime-marker-route boot-tree/archive static review: pass; archive manifest
  includes da591740/kernel_2712.img and the runtime marker route tokens.
- Conditional no-power lab publication gate: not run; not required because
  static archive shape repair proved selected fetch-path visibility and no lab
  mutation was performed.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v8-20260629.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
