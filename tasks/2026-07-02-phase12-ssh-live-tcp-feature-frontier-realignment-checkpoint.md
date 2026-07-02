# Phase 12 SSH Live TCP Feature Frontier Realignment Checkpoint

Task id: phase12-ssh-live-tcp-feature-frontier-realignment-checkpoint-20260702

Status: accepted after no-hardware feature-frontier checkpoint.

Classification: feature-frontier-blocked-for-supervisor-planning.

Evidence level: git status inspection, supervisor-state JSON validation,
roadmap/static architecture review, early POSIX/VFS frontier review, Phase 12
task-record review, recent git log review, docs build, and diff checks. No
hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
serial capture, TFTP capture, kernel_main proof, route-start proof,
runtime-ready proof, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.

## Goal

Reconcile the completed architecture review campaign and accepted local
POSIX/VFS foundations against the current Phase 12 live TCP/SSH frontier before
allowing any stale marker-chain successor to continue.

## Scope Performed

- Promoted this ready checkpoint after the supervisor paused stale v53, v60,
  and v72 continuations and cleared planningNeeded.
- Reviewed roadmap current status, early POSIX/VFS shape, Phase 12 networking
  status, the v71 exceptions-ready closeout, the taskQueue dispositions for
  v53/v60/v72, and the recent git log.
- Preserved the accepted v71 fact that the selected Pi 5 candidate reached
  TALOS: exceptions ready with selected TFTP identity and restore proof.
- Reconciled that fact against Matthew's feature-led process correction: a
  marker-only kernel_main continuation is not enough by itself to count as the
  next live TCP/SSH feature task.

## Checkpoint Result

The architecture quality campaign is complete, and the local POSIX/VFS
foundation chain has already advanced through descriptor-backed VFS I/O,
program loading, userspace launch/status, descriptor inheritance, argv, PATH
lookup, pipelines, process-status VFS views, redirection, and append
redirection at static/unit/QEMU-substitute evidence level. Those local
foundations remain accepted support for later SSH and shell work; this
checkpoint does not reopen that campaign.

The current live TCP/SSH frontier remains Phase 12. A prior local/static
networking chain accepted descriptor-facing socket and smoltcp boundaries
without live packet I/O, remote receipt, OpenSSH compatibility, service
success, or ssh-ready=true. The later Pi 5 selected-normal-runtime chain then
proved the selected candidate can reach exceptions-ready on the device. v71
closed with selected tree
b4c9bf0c09d122def872228a4e3d2a0f5836bfa0c7e4e4cdaa3b42ddf3e8ee9c, four
152,880-byte selected da591740/kernel_2712.img TFTP serves, 881 retained TALOS:
exceptions ready occurrences, final pre-restore selected identity, and restored
baseline tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first missing feature-planning fact is the next explicitly selected
feature-led Phase 12 objective. It may choose a runtime-discriminator only if
that discriminator is justified as the smallest proof needed by the live
TCP/SSH feature path, with mechanically checkable dependencies. The existing
v72 task is a marker-chain kernel_main reconciliation selected before this
feature-frontier checkpoint. This checkpoint therefore does not accept v72 as
the next task. It also does not select v60 runtime-ready or v53 packet-I/O
because their dependencies remain stale and their predecessor facts no longer
match the accepted v71 frontier.

## Terminal Classification

feature-frontier-blocked-for-supervisor-planning.

selected_next_task: null.

planningNeeded: true.

The required next supervisor decision is a single bounded feature-led Phase 12
task. It must either:

- explicitly reselect v72 as a feature-required kernel_main discriminator with
  refreshed dependencies from v71 and a clear removal/promotion rule; or
- replace it with a smaller implementation-sized or evidence-contract task that
  directly advances live TCP/SSH acceptance without stale marker-chain
  auto-promotion.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake/kernel-backed command
expansion, broad shell work, and phase transition remain blocked until such a
task is explicitly planned.

## Stale Task Dispositions

- deferred: v72
  phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-after-exceptions-reconciliation-v72-20260702
  remains paused. It may be reselected only by an accepted feature-led
  supervisor task or checkpoint that ties kernel_main proof to the next live
  TCP/SSH feature boundary.
- deferred: v60
  phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-after-no-route-start-repair-reconciliation-v60-20260702
  remains blocked. Its dependency chain points to older no-route-start/route
  readiness facts and is not refreshed from the accepted v71 exceptions-ready
  frontier.
- deferred: v53
  phase12-ssh-live-tcp-selected-normal-runtime-packet-io-continuation-reconciliation-v53-20260701
  remains blocked. Packet-I/O is a later live-networking claim and cannot be
  promoted from this checkpoint without a refreshed feature-led evidence
  contract.

## Findings

- fixed: stopped automatic promotion of the stale v72 marker-chain successor
  after Matthew's feature-led correction and the supervisor refresh.
- fixed: recorded that the architecture review/refactor campaign and local
  POSIX/VFS foundation chain are complete and should not be rerun by this
  worker.
- fixed: preserved v71 as the accepted exceptions-ready Pi 5 frontier while
  separating that proof from later kernel_main, route-start, runtime-ready, and
  packet-I/O claims.
- deferred: exact next Phase 12 feature-led task selection is left to the
  supervisor because the worker may not create a new task or broaden the queue.
- not-an-issue: no hardware triage is required; this checkpoint used only
  retained docs/state/task evidence and did not make a hardware-dependent
  claim.
- removed: stale marker-chain auto-promotion as the default next action.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-feature-frontier-realignment-checkpoint/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-feature-frontier-realignment-checkpoint/evidence-map.json.
- Static checkpoint summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-feature-frontier-realignment-checkpoint/static/checkpoint-summary.md.
- Roadmap:
  docs/src/roadmap.md.
- Early POSIX/VFS frontier:
  docs/src/project/early-posix-shape.md.
- Phase 12 frontier:
  docs/src/project/phase12-networking-ssh.md.
- v71 closeout:
  tasks/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71.md.

## Redaction Review

Task-owned checkpoint evidence retains task ids, source/path labels, hashes,
byte counts, marker counts, classifications, and validation outcomes. It does
not retain private user data, credentials, packet payloads, SSH/session/key
material, public-key blobs, signatures, fingerprints, operator identities, or
external account data.

## Validation

- git status --short --branch before edits/action: pass.
- jq empty on supervisor state and task-owned JSON evidence: pass.
- Roadmap, early POSIX/VFS frontier, Phase 12 task records, and recent git log
  review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
