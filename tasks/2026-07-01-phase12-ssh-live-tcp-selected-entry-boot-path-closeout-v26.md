# Phase 12 SSH Live TCP Selected-Entry Boot-Path Closeout V26

Task id: phase12-ssh-live-tcp-selected-entry-boot-path-closeout-v26-20260701

Status: accepted after commit.

Classification: selected-entry-boot-path-frontier-reconciled-supervisor-planning.

Evidence level: accepted v19/v20/v22/v24/v25/v26 task and evidence inspection,
task-owned JSON evidence, docs build, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the v26 Pi 5 selected-entry boot-path result against the preceding
selected-byte/no-marker chain and stop at the exact proved boundary without
inventing the next feature task.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26-20260701
  accepted selected-entry-boot-path-marker-retained and selected this exact
  task.
- Compared v26 against the accepted v19 selected-image handoff proof and the
  v20, v22, and v24 selected-byte/no-marker hardware chain.
- Preserved the decisive v26 facts: selected da591740/kernel_2712.img was
  served twice at 45,400 bytes, final pre-restore identity remained on selected
  tree a8da6043..., the fresh post-power serial window retained 13,796
  TALOS: reu10-loop occurrences, and restore returned the lab to tree
  a0452458....
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, or phase transition.

## Terminal Classification

selected-entry-boot-path-frontier-reconciled-supervisor-planning.

The v26 run proves selected Image execution reaches rust_entry and the UART10
early output path before BootInfo parsing, target init, allocator, scheduler,
RP1 MMIO, networking, service code, or shell behavior. That resolves the
selected-entry boot-path blocker that remained after v24's compact _start
selected-kernel-entry marker was absent, because v25/v26 used a qualitatively
different repeated rust_entry UART10 discriminator and retained the marker
decisively.

selected_next_task: null.

planningNeeded: true.

planningReason: v26 proves selected Image execution reaches rust_entry and the
UART10 early output path, but no queued successor has refreshed dependencies
for the next smallest feature step. The supervisor must plan before any
packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, or phase transition.

## Findings

- fixed: reconciled v19/v20/v22/v24 selected-byte and no-marker evidence with
  v26's retained rust_entry UART10 marker loop.
- fixed: accepted the first newly proved boundary as selected Image execution
  reaching rust_entry and the UART10 early output path.
- not-an-issue: the v24 compact _start marker absence no longer blocks the
  selected-entry boot-path boundary because v25/v26 used a qualitatively
  different repeated rust_entry UART10 discriminator.
- deferred: BootInfo parsing, target init, allocator, scheduler, RP1 MMIO,
  networking, service code, and shell behavior remain unproved and require a
  refreshed successor task.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-closeout-v26/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-closeout-v26/classification.json.
- Accepted v19 selected-image handoff proof:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19.md.
- Accepted v20 minimal-entry polled-console preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20.md.
- Accepted v22 minimal-entry console-boundary preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22.md.
- Accepted v24 selected-kernel-entry preflight and closeout:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24.md and
  tasks/2026-06-30-phase12-ssh-live-tcp-selected-kernel-entry-discriminator-closeout-v24.md.
- Accepted v25 replacement-discriminator reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-reconciliation-v25.md.
- Accepted v26 Pi 5 preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26.md.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data. It
references task-owned hardware evidence retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
