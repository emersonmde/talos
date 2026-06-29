# Phase 12 SSH Live TCP Known-Good Kernel-Entry Blocker Reconciliation

Task id: phase12-ssh-live-tcp-known-good-kernel-entry-blocker-reconciliation-20260629

Status: accepted after commit.

Classification: known-good-control-reselection-ready.

Evidence level: static task/evidence/source/docs review, task-owned JSON
evidence, shell syntax checks for reviewed helpers, docs build, and diff
checks. No lab/hardware action, hardwareTestLock acquisition, boot archive
publication, candidate run, packet-I/O discriminator, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, broad shell work, or phase transition was performed.

## Goal

Reconcile the accepted blocked-known-good-kernel-not-starting result and decide
the smallest objective recovery path before any candidate preflight, packet-I/O
discriminator, OpenSSH/generated-root retry, or phase transition.

## Scope Performed

- Reviewed the accepted known-good capture discriminator, known-good readiness
  contract reconciliation, and serial-window completeness discriminator records.
- Reviewed task-owned classification/evidence maps for the retained 2026-06-29
  known-good runs.
- Reviewed the v3 known-good readiness classifier and runtime-readiness
  retention helpers.
- Reviewed the production-timer boot-tree/image scripts and source ordering for
  the PASS marker.
- Corrected Phase 12/lab documentation so the production-timer known-good
  readiness contract is tied to the accepted 104,136-byte a045... control
  lineage, not to any restored baseline that happens to be stable.

## Terminal Classification

known-good-control-reselection-ready.

The first missing fact behind blocked-known-good-kernel-not-starting is an
invalid/stale control snapshot for the production-marker contract. The recent
known-good-only hardware runs proved stable identity and TFTP service for the
restored 6ead8933... tree with two 82,045-byte da591740/kernel_2712.img
fetches. That is a real boot identity, but it is not the accepted
production-timer known-good control contract used by the v3 readiness gate.

The marker-positive production-timer control lineage is the a0452458... tree
with 104,136-byte da591740/kernel_2712.img fetches. Prior retained evidence
observed rpi5-production-timer-preemption: PASS under that lineage, and source
inspection confirms the PASS line is emitted only from
run_production_timer_preemption_proof after kernel entry and successful
production-timer proof predicates.

No helper/classifier source repair is required before selecting a replacement
control contract. The next bounded task is the no-power/read-only known-good
control reselection contract.

## Findings

- fixed: the first missing fact is now classified as invalid/stale control
  snapshot for the production-marker contract, not serial capture-window
  incompleteness or a runtime networking defect.
- fixed: Phase 12 and lab-controller docs now distinguish the accepted
  production-timer control lineage from the stable 6ead8933... baseline used by
  the blocked known-good runs.
- fixed: selected_next_task is the queued
  phase12-ssh-live-tcp-known-good-control-reselection-contract-20260629.
- not-an-issue: the v3 classifier correctly requires
  rpi5-production-timer-preemption: PASS and treats TALOS: kernel_main as
  metadata-only for this boundary.
- not-an-issue: stable 6ead8933... identity and 82,045-byte TFTP fetches remain
  valid hardware evidence, but they do not satisfy the production-timer
  known-good control contract.
- deferred: candidate preflight v3/v4, packet I/O, OpenSSH/generated-root
  retry, remote receipt, compatibility, service success, ssh-ready=true, broad
  shell work, and phase transition remain deferred until a reselected control
  proof explicitly unblocks them.
- removed: no source, helper, task, or evidence artifact was removed.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-kernel-entry-blocker-reconciliation/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-kernel-entry-blocker-reconciliation/evidence-map.json.
- Reviewed blocked run:
  tasks/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator.md.
- Reviewed accepted production-timer marker-positive proof:
  tasks/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof.md.
- Reviewed source:
  scripts/rpi5-production-timer-preemption-boot-tree.sh,
  scripts/rpi5-production-timer-preemption-image.sh,
  scripts/rpi5-known-good-readiness-v3-classify.sh, and
  src/target/rpi5.rs.

## Redaction Review

Task-owned JSON evidence records task ids, source paths, tree-hash prefixes,
kernel byte counts, classifier labels, validation commands/results, and
selected successor ids. It adds no peer identifiers, packet payloads, key
material, session material, boot artifact bytes, private user data,
secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task promotion.
- jq empty on directly referenced task-owned JSON evidence: pass.
- sh -n scripts/rpi5-known-good-readiness-v3-classify.sh: pass.
- sh -n scripts/rpi5-runtime-readiness-retention-guard.sh: pass.
- sh -n scripts/rpi5-retain-runtime-readiness-primary.sh: pass.
- sh -n scripts/rpi5-production-timer-preemption-boot-tree.sh: pass.
- sh -n scripts/rpi5-production-timer-preemption-image.sh: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-known-good-control-reselection-contract-20260629.

planningNeeded: false.

Candidate preflight v3/v4 remains blocked. The next worker task must select or
reject a valid known-good/control baseline using no-power/read-only evidence
before any hardware control proof, candidate preflight, packet-I/O
discriminator, OpenSSH/generated-root retry, remote receipt, compatibility
claim, service success claim, ssh-ready=true, broad shell work, or phase
transition.

Commit: recorded in talos-supervisor-state.json after final commit.
