# Phase 12 SSH Live TCP Minimal-Entry Polled-Console Closeout V20

Task id: phase12-ssh-live-tcp-minimal-entry-polled-console-closeout-v20-20260630

Status: accepted after commit.

Classification: minimal-entry-polled-console-frontier-blocked-supervisor-planning.

Evidence level: accepted no-hardware v17 route repair inspection, accepted v19
selected-image handoff proof inspection, accepted serialized Pi 5 v20
minimal-entry polled-console preflight inspection, task-owned JSON evidence,
docs build, and diff checks. No code implementation, hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the repaired minimal-entry Pi 5 preflight and select the next bounded
step without performing hardware action or broad feature expansion.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20-20260630
  accepted blocked-minimal-entry-polled-console-marker-missing and selected this
  exact task.
- Inspected the accepted v17 route repair, v19 selected-image handoff proof,
  v20 hardware preflight task record, classification JSON, and evidence map.
- Preserved the decisive v20 hardware facts: repaired v17 selected image
  source=kernel-main-entry-control-polled-console, selected
  da591740/kernel_2712.img at 52,728 bytes with SHA-256 ccc95535..., empty
  pre-power serial drain, two stable selected TFTP serves, final pre-restore
  selected identity on tree 3eee516f..., and restore to the
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z a0452458... control tree.
- Reconciled the first missing fact as the still-absent
  TALOS: minimal-entry-control-ready marker from the fresh post-power serial
  window. Selected-byte service, serial freshness, final identity, and restore
  are no longer the blocker for this exact preflight shape.

## Terminal Classification

minimal-entry-polled-console-frontier-blocked-supervisor-planning.

The repaired minimal-entry polled-console marker was not retained by the Pi 5
run even though the selected image was served and the lab identity/restore path
was decisive. That blocks the v21 runtime-marker post-minimal-entry preflight:
the prerequisite minimal-entry marker-retained fact is absent.

selected_next_task: null.

planningNeeded: true.

planningReason: v20 proved selected-byte service and restore for the repaired
minimal-entry selected image, but the fresh post-power serial window still had
zero TALOS: minimal-entry-control-ready occurrences. No queued successor is
mechanically unblocked until the supervisor defines the next bounded
minimal-entry route/handoff discriminator or repair. Packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility/service readiness,
ssh-ready=true, broad shell work, fake/kernel-backed command expansion, and
phase transition remain blocked.

## Findings

- fixed: reconciled the v20 hardware preflight as decisive selected-byte,
  serial-freshness, final-identity, and restore evidence for the repaired v17
  minimal-entry selected image.
- deferred: the first missing fact remains the absent
  TALOS: minimal-entry-control-ready marker after selected-byte service.
- removed: the queued v21 runtime-marker post-minimal-entry preflight as a
  mechanically unblocked successor, because its marker-retained prerequisite is
  not satisfied.
- not-an-issue: no known-good control or candidate rerun was required in v20
  because the preflight was not inconclusive for identity, TFTP capture, serial
  freshness, or restore.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-polled-console-closeout-v20/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-polled-console-closeout-v20/classification.json.
- Accepted v17 route repair:
  tasks/2026-06-30-phase12-ssh-live-tcp-minimal-entry-route-repair-v17.md.
- Accepted v19 selected-image handoff discriminator:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19.md.
- Accepted v20 Pi 5 preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20.md.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data. It
references task-owned hardware evidence retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
