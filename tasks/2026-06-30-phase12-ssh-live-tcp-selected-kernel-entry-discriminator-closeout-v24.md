# Phase 12 SSH Live TCP Selected-Kernel-Entry Discriminator Closeout V24

Task id: phase12-ssh-live-tcp-selected-kernel-entry-discriminator-closeout-v24-20260630

Status: accepted after commit.

Classification: selected-kernel-entry-frontier-blocked-supervisor-planning.

Evidence level: accepted no-hardware v19/v20/v21/v22/v23/v24 task and
evidence inspection, task-owned JSON evidence, docs build, and diff checks. No
hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim,
service success claim, ssh-ready=true, fake/kernel-backed command expansion,
broad shell work, or phase transition was performed.

## Goal

Reconcile the v24 Pi 5 selected-kernel-entry discriminator result and select
the next bounded step without performing hardware action or broad feature
expansion.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24-20260630
  accepted blocked-selected-kernel-entry-marker-missing and selected this exact
  task.
- Inspected accepted v19, v20, v21, v22, v23, and v24 task records plus
  task-owned evidence maps.
- Preserved the decisive v24 hardware facts: selected
  da591740/kernel_2712.img at 87,432 bytes with SHA-256 8051d7a6..., empty
  pre-power serial drain, two stable selected TFTP serves, final pre-restore
  selected identity on tree dbe73980..., and restore to the
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z a0452458... control tree.
- Reconciled the first missing fact as selected-kernel entry itself: the
  discriminator emitted TALOS: selected-kernel-entry-discriminator-v23 at
  _start before CPACR, BSS, stack, Rust entry, BootInfo, minimal-entry,
  networking, or service code, but the fresh post-power serial window retained
  firmware output and zero occurrences of that marker.

## Terminal Classification

selected-kernel-entry-frontier-blocked-supervisor-planning.

The v24 run proves selected-byte service, serial freshness, final selected
identity, and restore for the v23 _start selected-kernel-entry discriminator
image. It does not prove entry into the selected kernel image: the earliest
available _start marker, TALOS: selected-kernel-entry-discriminator-v23, was
absent from the fresh post-power serial window even though firmware NETWORK
output was present.

selected_next_task: null.

planningNeeded: true.

planningReason: v24 proved selected-byte service and restore for the
_start-level selected-kernel-entry discriminator image, but the fresh
post-power serial window still had zero
TALOS: selected-kernel-entry-discriminator-v23 occurrences. No queued successor
is mechanically unblocked until the supervisor defines the next bounded
selected-kernel-entry repair, lab discriminator, or dependency refresh.
Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, broad shell work,
fake/kernel-backed command expansion, and phase transition remain blocked.

## Findings

- fixed: reconciled the v24 hardware preflight as decisive selected-byte,
  serial-freshness, final-identity, and restore evidence for the v23 _start
  selected-kernel-entry discriminator image.
- deferred: the first missing fact is selected-kernel image entry, because the
  run retained firmware output but zero earliest _start discriminator marker
  occurrences.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.
- not-an-issue: no known-good control or candidate rerun was required in v24
  because identity, TFTP selected-byte service, serial freshness, final
  identity, and restore facts were decisive.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-closeout-v24/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-closeout-v24/classification.json.
- Accepted v19 selected-image handoff discriminator:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19.md.
- Accepted v20 minimal-entry polled-console preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20.md.
- Accepted v20 closeout:
  tasks/2026-06-30-phase12-ssh-live-tcp-minimal-entry-polled-console-closeout-v20.md.
- Accepted v21 console-boundary reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-minimal-entry-console-boundary-reconciliation-v21.md.
- Accepted v22 console-boundary preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22.md.
- Accepted v22 closeout:
  tasks/2026-06-30-phase12-ssh-live-tcp-minimal-entry-console-boundary-closeout-v22.md.
- Accepted v23 selected-kernel-entry reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-reconciliation-v23.md.
- Accepted v24 Pi 5 selected-kernel-entry preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24.md.

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
