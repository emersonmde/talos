# Phase 12 SSH Live TCP Selected-Image Boot Contract Discriminator Closeout V18

Task id: phase12-ssh-live-tcp-selected-image-boot-contract-discriminator-closeout-v18-20260630

Status: accepted after commit.

Classification: selected-image-boot-contract-handoff-blocker-supervisor-planning.

Evidence level: accepted v17 static/source reconciliation inspection,
accepted serialized Pi 5 boot-contract discriminator task/evidence inspection,
task-owned JSON evidence, docs build, and diff checks. No code implementation,
hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim,
service success claim, ssh-ready=true, fake/kernel-backed command expansion,
minimal-entry route repair, broad shell work, or phase transition was
performed.

## Goal

Reconcile the accepted v18 boot/Image handoff discriminator result without
shrinking acceptance toward a shim or authorizing blind reruns, packet-I/O, or
OpenSSH/generated-root retry.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18-20260630
  accepted blocked-selected-image-handoff-after-boot-contract-discriminator
  and selected this exact task.
- Inspected the accepted v17 boot-contract reconciliation task, accepted v18
  Pi 5 discriminator task, classification JSON, and task-owned evidence map.
- Preserved the decisive hardware facts: the boot-contract discriminator
  selected image was published, da591740/kernel_2712.img was served twice with
  matching 87,432-byte counts, final pre-restore identity stayed on tree
  61811ab93..., and restore to the predecessor-named baseline succeeded.
- Preserved the handoff blocker: the fresh serial window retained firmware
  NETWORK output but no TALOS: boot-contract-v18, TALOS: kernel_main, or later
  Talos marker.
- Stopped at supervisor planning because no existing queued successor is
  mechanically unblocked by the selected-byte/no-_start-marker result.

## Terminal Classification

selected-image-boot-contract-handoff-blocker-supervisor-planning.

The first missing fact remains the firmware-to-selected-image handoff into the
selected kernel bytes, below Image header selection, selected TFTP service,
capture identity, restore identity, CPACR setup, BSS clear, stack setup,
rust_entry, kernel_main, networking, packet I/O, OpenSSH, and shell behavior.
The v18 discriminator changed the v16 marker topology by writing repeated
TALOS: boot-contract-v18 bytes directly from _start through inline UART10
writes and FR flushes, with no BL helper, BSS/stack/Rust path, networking,
packet I/O, OpenSSH, or shell work.

The Pi 5 discriminator proved that firmware requested and received the
selected da591740/kernel_2712.img bytes for that discriminator image before
restore. The run therefore does not classify as selected-image identity
failure, TFTP capture failure, restore failure, or inconclusive evidence.

The fresh serial window still retained only firmware NETWORK output. It did
not retain TALOS: boot-contract-v18, TALOS: kernel_main, or any later Talos
marker. That means the current missing boundary is not live TCP route, packet
I/O, OpenSSH, remote receipt, compatibility, SSH service readiness,
minimal-entry route repair, or broad shell behavior.

No queued successor is mechanically unblocked:

- phase12-ssh-live-tcp-minimal-entry-route-repair-v17-20260630 requires the
  earlier v16 closeout to accept a handoff-entry-reached terminal
  classification and select that exact task. The accepted v18 discriminator
  instead proved selected-byte service with no repeated _start marker.
- Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility,
  service success, ssh-ready=true, fake/kernel-backed command expansion,
  minimal-entry route repair, broad shell work, and phase transition remain
  blocked because the selected-image boot-contract marker remains absent.

selected_next_task: null.

planningNeeded: true.

planningReason: v18 proved selected-byte service for the repeated compact
boot-contract _start discriminator image, final pre-restore identity, and
restore, but still observed no TALOS: boot-contract-v18 or later Talos marker
after firmware NETWORK. Supervisor must plan the next bounded
firmware/image-handoff investigation or discriminator before any rerun,
packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness claim, minimal-entry route repair, broad shell
work, or phase transition.

## Findings

- fixed: reconciled selected identity, same-cursor TFTP byte agreement, final
  pre-restore identity, and restore proof as decisive for the v18
  boot-contract discriminator.
- fixed: preserved the v17/v18 distinction from v16: the repeated compact
  _start marker uses inline UART10 writes and FR flushes, avoiding the v16
  one-shot BL helper and later initialization paths.
- deferred: the firmware/image-handoff reason for missing _start marker output
  requires supervisor planning as a new bounded task or discriminator.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, minimal-entry route repair, broad shell work, and phase
  transition as immediate successors.
- not-an-issue: hardwareTestLock remained unlocked because this closeout is
  no-hardware and relies on accepted predecessor evidence.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-discriminator-closeout-v18/evidence-map.json.
- Accepted v17 selected-image boot-contract reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17.md.
- Accepted v18 Pi 5 boot-contract discriminator:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18.md.
- Accepted v18 Pi 5 discriminator classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18/classification.json.

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
