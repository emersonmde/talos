# Phase 12 SSH Live TCP Kernel Entry Boundary Closeout V14

Task id: phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v14-20260630

Status: accepted after commit.

Classification: minimal-entry-control-no-kernel-entry-supervisor-planning.

Evidence level: accepted v14 task/evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No code implementation, hardware
action, lab publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the accepted v14 minimal entry-control result without shrinking the
acceptance boundary toward a shim or authorizing blind reruns, packet-I/O, or
OpenSSH/generated-root retry.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14-20260630
  accepted blocked-control-entry and selected this exact task.
- Inspected the accepted v14 candidate-vs-control reconciliation, bisect
  contract, and Pi 5 minimal entry-control discriminator records and
  task-owned JSON evidence.
- Preserved the decisive hardware facts: the run-unique minimal entry-control
  selected-path archive was published, da591740/kernel_2712.img was served
  twice with matching bytes, final pre-restore identity stayed on the selected
  tree, and restore succeeded.
- Preserved the entry blocker: even the minimal entry-control image produced
  firmware NETWORK output only, with no retained rust_entry, boot-info-parsed,
  target-init, exceptions-ready, kernel_main, or nonce-bearing
  minimal-entry-control-ready marker.
- Stopped at supervisor planning because no existing queued successor is
  mechanically unblocked by blocked-control-entry.

## Terminal Classification

minimal-entry-control-no-kernel-entry-supervisor-planning.

The first missing fact is now below the live TCP runtime route and below the
candidate-specific network code. The v14 discriminator removed the larger live
TCP candidate surface by publishing a current-tree minimal entry-control
selected-path archive with a single required marker:
TALOS: minimal-entry-control-ready. The lab-controller API and repaired TFTP
capture proved selected identity and selected-byte service for that image
before restore, so this closeout does not classify the result as identity,
TFTP-capture, restore, or inconclusive.

The fresh serial window still retained only firmware NETWORK output. It did
not retain TALOS: rust_entry, TALOS: boot info parsed, TALOS: target init,
TALOS: exceptions ready, TALOS: kernel_main, or the nonce-bearing minimal
entry-control marker. That means the missing boundary is not yet packet I/O,
OpenSSH, remote receipt, compatibility, or SSH service readiness. It is the
selected-image entry path for a minimal current-tree control image on the Pi 5.

No queued successor is mechanically unblocked:

- phase12-ssh-live-tcp-pi5-candidate-entry-preflight-v14-20260630 requires a
  predecessor that accepted candidate-entry-source-repair-ready and selected
  that exact repaired candidate preflight; no v14 predecessor did so.
- phase12-ssh-live-tcp-pi5-packet-io-discriminator-v13-20260630 requires a
  predecessor that accepted candidate-capture-ready and selected packet-I/O;
  no v14 predecessor did so.
- OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, fake/kernel-backed command expansion, broad shell work, and
  phase transition remain blocked because candidate-capture-ready remains
  rejected and the minimal control cannot yet emit its entry marker.

selected_next_task: null.

planningNeeded: true.

planningReason: v14 proved selected-byte service for a minimal current-tree
entry-control image but still observed no Talos entry or kernel markers after
firmware NETWORK output. Supervisor must plan a new bounded source/entry
investigation or discriminator for the Pi 5 selected-image entry boundary
before any rerun, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness claim, broad shell work, or phase transition.

## Findings

- fixed: selected identity, same-cursor TFTP byte agreement, final
  pre-restore identity, and restore proof were reconciled as decisive for the
  v14 minimal entry-control run.
- blocked: the minimal entry-control image still produced no retained Talos
  entry, kernel_main, or nonce-bearing minimal-entry-control marker.
- deferred: the source/entry reason for missing markers must be planned as a
  new bounded task by the supervisor.
- removed: packet-I/O/OpenSSH/generated-root retry as immediate successors
  from this closeout.
- not-an-issue: hardwareTestLock can remain unlocked because this closeout is
  no-hardware and relies on accepted predecessor evidence.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v14/evidence-map.json.
- Accepted v14 candidate-vs-control reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-candidate-vs-control-entry-delta-reconciliation-v14.md.
- Accepted v14 bisect contract:
  tasks/2026-06-30-phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14.md.
- Accepted v14 Pi 5 minimal entry-control discriminator:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14.md.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data. It
references task-owned hardware evidence retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on referenced JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
