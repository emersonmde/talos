# Phase 12 SSH Live TCP Lab Capture Contract Reconciliation

Task id: phase12-ssh-live-tcp-lab-capture-contract-reconciliation-20260629

Status: accepted after commit.

Classification: lab-capture-contract-reconciled-known-good-discriminator-ready.

Evidence level: static task/evidence/source/docs review, focused helper dry-run,
task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware/lab
action, hardwareTestLock acquisition, boot publication, power-cycle, candidate
run/rerun, packet-I/O discriminator, OpenSSH/generated-root retry, remote
receipt, compatibility, ssh-ready=true, runtime russh adoption, fake command
expansion, broad shell work, or phase transition was performed.

## Goal

Reconcile the blocked known-good control from the accepted Pi 5 candidate
preflight v2 before any further live TCP hardware work.

## Scope Performed

- Reviewed the accepted Pi 5 proof contract, preflight v2 task record,
  preflight v2 classification/evidence summaries, lab-controller docs, helper
  source, Phase 12 docs, roadmap, and supervisor state.
- Replayed the known-good readiness classifier over the retained v2 artifacts to
  confirm the recorded blocker was reproducible.
- Fixed the static helper contract defect where
  scripts/rpi5-capture-invariant-proof-bundle.sh wrote *-status.json artifacts
  from /boot/files instead of /status.
- Updated lab-controller docs so the helper evidence contract explicitly says
  status artifacts come from /status and byte/root visibility comes from
  /boot/files.
- Stopped at the static/source reconciliation boundary and selected the
  known-good-only capture discriminator as the next hardware task.

## Contradiction Reconciliation

- fixed: endpoint identity source. The capture-invariant helper now captures
  pre-status.json, final-pre-restore-status.json, and post-restore-status.json
  from GET /status; /boot/files remains the selected file/byte visibility
  source. This removes the status/files endpoint drift for future proof bundles.
- blocked-for-hardware-discriminator: missing production success marker. The v2
  retained primary readiness artifact used saturated direct-read fallback with
  zero response bytes, so it contained neither TALOS: kernel_main nor the
  required production-timer success marker. Static review cannot repair that
  missing serial fact.
- blocked-for-hardware-discriminator: unstable/missing boot identity join. The
  v2 known-good control pre-status tree was the restored baseline
  6ead8933..., while final status/TFTP served the later candidate-sized 87432
  kernel under tree 18e467bf.... A known-good-only discriminator must prove a
  single stable control tree before any candidate publication.
- deferred-until-known-good-ready: candidate expected-fetch byte mismatch. The
  retained candidate TFTP delta showed baseline-sized 82045 kernel serves while
  candidate identity expected 87432; this remains a candidate preflight v3
  question after known-good capture is proven.
- deferred-until-known-good-ready: final pre-restore tree mismatch. The retained
  candidate final pre-restore identity had returned to the restored baseline
  tree, so candidate capture cannot be accepted without a later candidate-only
  v3 run under the reconciled endpoint contract.

## Findings

- fixed: capture-invariant helper status artifacts now use GET /status.
- fixed: lab-controller docs now distinguish /status identity samples from
  /boot/files file/byte samples in the helper contract.
- blocked: the retained v2 known-good serial success marker is absent and must
  be reproved by the next known-good-only hardware discriminator.
- blocked: the retained v2 known-good identity tuple drifted from baseline to
  candidate-sized boot files and must be reproved by the next known-good-only
  hardware discriminator.
- deferred: candidate fetch-byte and final-pre-restore mismatches are deferred
  to candidate preflight v3 after known-good capture is accepted.
- not-an-issue: no Rust source gate is required because no Rust source changed.
- removed: no source, docs, task, or evidence artifact was removed.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-lab-capture-contract-reconciliation/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-lab-capture-contract-reconciliation/evidence-map.json.
- Reviewed proof contract:
  tasks/2026-06-29-phase12-ssh-live-tcp-pi5-proof-contract.md.
- Reviewed blocked preflight:
  tasks/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2.md.
- Reviewed retained v2 evidence:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/.
- Source changed:
  scripts/rpi5-capture-invariant-proof-bundle.sh.
- Docs changed:
  docs/src/project/lab-controller.md.

## Redaction Review

Durable evidence records task ids, source paths, public classifier names,
tree-hash prefixes, kernel byte counts, validation commands/results, and
metadata-only labels. It does not add peer identifiers, addresses, packet
payload contents, key material, session material, boot artifact bytes, private
user data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task promotion.
- Static review of accepted proof contract, blocked preflight v2 task/evidence,
  lab-controller docs, helper source, Phase 12 docs, roadmap, and supervisor
  state: pass.
- Replayed known-good readiness classifier over retained v2 artifacts: pass;
  reproduced expected exit 1 with known-good-readiness-v3-blocked.
- scripts/rpi5-capture-invariant-proof-bundle.sh --dry-run: pass.
- sh -n scripts/rpi5-capture-invariant-proof-bundle.sh: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: phase12-ssh-live-tcp-known-good-capture-discriminator-20260629.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
