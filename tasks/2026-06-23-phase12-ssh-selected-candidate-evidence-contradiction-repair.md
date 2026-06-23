# Phase 12.6 SSH selected-candidate evidence contradiction repair

Task id: phase12-ssh-selected-candidate-evidence-contradiction-repair-20260623
Status: accepted
Owner: worker
Classification: selected-candidate-acceptance-quarantined

## Goal

Repair the selected-candidate lab-capture acceptance contradiction before any
live OpenSSH retry.

## Reviewed Inputs

- memory/talos-supervisor-state.json currentTask and taskQueue entries for
  phase12-ssh-lab-capture-selected-candidate-discriminator-20260623,
  phase12-ssh-live-openssh-client-discriminator-retry-v2-20260623,
  phase12-ssh-live-openssh-client-discriminator-closeout-v2-20260623, and the
  v3 rerun path.
- tasks/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator.md.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/selected-candidate-discriminator.summary.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/tftp-delta.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/final-pre-restore-status.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/final-pre-restore-boot-files.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/final-status.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/final-boot-files.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/archive-review.txt.
- tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/archive-sizes.txt.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- Commit 8ad40fea767b56063f547845ebb4ba64f93af081.

## Findings And Disposition

- fixed: superseded the selected-candidate task record so it no longer claims
  selected-candidate-fetch-observed=true or selects retry-v2 from contradictory
  retained JSON.
- fixed: corrected docs/src/project/phase12-networking-ssh.md and
  docs/src/roadmap.md to state the actual retained selected-candidate evidence:
  capture-chain-inconclusive, selected_candidate_fetch_observed=false,
  selected_next_task=null, two 104,136-byte da591740/kernel_2712.img TFTP
  fetches, and final pre-restore identity on the baseline/control tree.
- fixed: kept retry-v2 and closeout-v2 blocked/superseded in durable state so
  they cannot be mechanically promoted from the invalid prerequisite.
- fixed: selected
  phase12-ssh-selected-candidate-lab-capture-rerun-v2-20260623 as the next
  bounded task after this repair is accepted and committed.
- removed: the invalid selected-candidate-fetch-observed=true acceptance claim
  from the task/docs/state frontier.
- deferred: live OpenSSH retry-v3 remains queued behind a future rerun-v2 that
  must independently accept selected-candidate-fetch-observed=true.
- not-an-issue: retained raw/sanitized evidence was not edited to match the old
  prose; the JSON evidence remains authoritative.

## Before And After State

Before repair:

- The selected-candidate task prose/docs claimed selected-candidate-fetch-observed=true.
- The selected-candidate retained summary JSON recorded
  classification=capture-chain-inconclusive,
  selected_candidate_fetch_observed=false, selected_next_task=null,
  final_pre_restore_tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  and two 104,136-byte TFTP kernel fetches.
- retry-v2 and closeout-v2 existed in the queue and had to remain blocked
  because their selected-candidate prerequisite was invalid.

After repair:

- The selected-candidate task is superseded/quarantined as
  capture-chain-inconclusive.
- Docs and state record that no live OpenSSH precondition was accepted by the
  selected-candidate task.
- retry-v2 and closeout-v2 remain blocked/superseded and cannot be promoted from
  the invalid prerequisite.
- selected_next_task is
  phase12-ssh-selected-candidate-lab-capture-rerun-v2-20260623 for a future
  worker wake after this repair is committed.

## Evidence

- static task/docs/evidence/state review: pass.
- selected-candidate retained summary JSON: capture-chain-inconclusive,
  selected_candidate_fetch_observed=false, selected_next_task=null,
  final pre-restore restored baseline tree, and two 104,136-byte baseline TFTP
  kernel fetches.
- task record correction:
  tasks/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator.md.
- docs correction: docs/src/project/phase12-networking-ssh.md and
  docs/src/roadmap.md.
- state correction: memory/talos-supervisor-state.json keeps retry-v2 and
  closeout-v2 blocked/superseded and records this repair as accepted.

## Validation

- jq empty on memory/talos-supervisor-state.json and task-owned JSON evidence:
  pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static inspection, JSON syntax check, docs build, diff checks,
and git commit.

## Acceptance

Accepted as selected-candidate-acceptance-quarantined.

selected_next_task=phase12-ssh-selected-candidate-lab-capture-rerun-v2-20260623.

planningNeeded=false.

No lab/hardware action, boot publication, power cycle, serial/TFTP capture,
OpenSSH execution, network action, runtime source change, compatibility claim,
remote-receipt=true, phase transition, or ssh-ready=true is accepted.
