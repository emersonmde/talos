# Phase 10 Pi 5 Serial Command0 Lab Write Boundary Discriminator Core

Task id: phase10-pi5-serial-command0-lab-write-boundary-discriminator-core-20260617

Status: accepted

Classification:
command0-lab-write-boundary-discriminator-core-selected

Evidence level: static/source/task/lab-doc evidence inspection, task-owned
JSON evidence, docs build, and diff checks. No Pi 5 hardware run, boot archive
publication, power-cycle, lab mutation, hardwareTestLock acquisition, source
response retention proof, generated-root command-input success claim, storage,
networking, SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Select the smallest concrete discriminator for the remaining lab
/serial/write-to-UART10 delivery/capture boundary after a visible command0
prompt.

## Problem Statement

The feature under test is still local serial interactivity on the generated-root
Pi 5 boot: after Talos reports ready command=0 and prints a visible talos>
prompt, a lab write of rootinfo plus newline should enter the UART10-backed
runtime console, dispatch command0, emit one response, and advance to ready
command=1.

The local/source boundary is intact. The accepted input-delivery core found no
command-loop, rootinfo dispatch, canonical-lite input, or UART10 polling defect,
and a QEMU/substitute serial ingress smoke showed prompt-delayed serial writes
reaching command dispatch. The accepted Pi 5 write-delivery proof after
selected-kernel recovery also proved the selected 208984-byte kernel/TFTP
precondition, retained command0 readiness and a visible prompt, and got an
accepted POST /serial/write response for rootinfo. The contradicting evidence
is that the bounded post-write observe retained zero bytes from saturated
cursor 4194304 and no rootinfo, line command=0, dispatch command=0
status=handled, responses=1, or ready command=1.

The unproven assumption is therefore outside the kernel source: whether the lab
write reached the Pi UART10 RX path at the command0 boundary, and whether the
selected capture endpoint was already too late or pinned by serial retention to
observe the command0 response.

## Approaches Compared

Same-shaped cursor-bound observe retry is rejected. It would repeat POST
/serial/write followed by POST /serial/observe from a saturated cursor, the
exact evidence shape that already retained zero bytes. The lab-controller docs
explicitly say an empty observe window from a saturated cursor must not be
accepted as proof that the current boot emitted no serial output.

Post-write direct-read retry is also insufficient by itself. Earlier
direct-read work showed that direct reads after a write can retain only the tail
of command output or later timeout readiness, so starting capture only after the
write still leaves a race between command output and evidence retention.

The selected discriminator is a prearmed live-read capture around the normal
POST /serial/write. After command0 readiness and a fresh pre-write boundary are
recorded, the Pi 5 proof starts a bounded POST /serial/read in the background,
waits only long enough for the request to be active, then sends rootinfo through
POST /serial/write with append_newline=true. The prearmed read is live before
the write, so it can discriminate the lab write/capture boundary without using a
same-shaped post-write observe retry and without changing kernel code or lab
service behavior.

## Selected Discriminator Contract

The selected next task is
phase10-pi5-serial-command0-lab-write-boundary-pi5-proof-20260617.

The proof must use the existing generated-root command-input candidate identity:

- archive target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz;
- selected tree 06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212;
- fetch path da591740/kernel_2712.img;
- expected selected kernel size 208984 bytes.

The exact command payload is rootinfo with append_newline=true, expected bytes=9.

The serial strategy is:

1. Record selected-kernel/TFTP precondition before command behavior is
   evaluated.
2. Retain same-boot generated-root readiness with source=firmware-initramfs,
   reason=valid-artifact, ready command=0, and a visible talos> prompt.
3. Record a pre-write freshness read or peek immediately after the command0
   prompt. It must not already contain rootinfo, line command=0, dispatch
   command=0, responses=1, ready command=1, later command readiness, or
   generated-root source-response output.
4. Start a bounded POST /serial/read live capture before the write with
   timeout_seconds=12, settle_ms=500, and max_bytes=16384. Label it
   prearmed-direct-read-during-write.
5. After a short activation delay, send POST /serial/write with text=rootinfo
   and append_newline=true. Record action, ok, and bytes.
6. The prearmed read is the primary post-write evidence. It must retain ordered
   command0 output: rootinfo or line command=0, dispatch command=0
   status=handled, responses=1, and ready command=1.
7. If the saved cursor is not saturated, a secondary POST /serial/observe from
   that cursor may be retained as corroboration. If the cursor is saturated,
   empty observe output is diagnostic only and must not reject the prearmed
   read.
8. Record stable selected-kernel TFTP delta, final pre-restore identity, and
   post-run baseline restore proof before releasing hardwareTestLock.

Terminal classifications are:

- command0-lab-write-boundary-input-delivery-accepted: selected-kernel/TFTP
  precondition passes, /serial/write accepts 9 bytes, and prearmed read retains
  ordered command0 output.
- command0-lab-write-boundary-write-accepted-capture-empty: /serial/write
  accepts 9 bytes but the prearmed live read retains no command0/rootinfo
  output.
- command0-lab-write-boundary-write-failed: POST /serial/write fails or returns
  an unexpected byte count.
- command0-lab-write-boundary-precondition-blocked: selected-kernel/TFTP,
  command0 readiness, pre-write freshness, final identity, or restore proof
  fails before the write-delivery invariant can be evaluated.
- command0-lab-write-boundary-inconclusive-triage-required: capture or identity
  evidence is internally inconsistent; before code changes, run the standard
  Pi 5 inconclusive sequence of candidate identity, fresh serial cursor, TFTP
  delta, known-good control, then candidate rerun.

## Findings

- fixed: restated the first failing invariant as lab write-to-UART10 delivery
  plus capture around command0, not as source-response retention or broader
  generated-root command input.
- fixed: rejected the same-shaped saturated cursor observe retry because it
  adds no new evidence beyond the accepted zero-byte observe blocker.
- fixed: rejected post-write-only direct read because earlier evidence shows it
  can miss the command0 output race and retain stale later-command output.
- fixed: selected a prearmed live-read discriminator that still uses the normal
  POST /serial/write payload but starts capture before the write.
- deferred: serialized Pi 5 hardware publication, hardwareTestLock acquisition,
  and proof classification are owned by the selected follow-up task.
- not-an-issue: no local command-loop, TTY, UART, or lab-controller code change
  is required to express this discriminator.
- rejected: source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Accepted input-delivery closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-input-delivery-closeout.md.
- Accepted input-delivery core:
  tasks/2026-06-17-phase10-pi5-serial-command0-input-delivery-core.md.
- Accepted write-delivery after selected-kernel recovery:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery.md.
- Prior UART10 polling RX proof:
  tasks/2026-05-24-phase5-pi5-uart10-polling-rx-proof.md.
- Lab-controller serial endpoint contract:
  docs/src/project/lab-controller.md.
- This classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core/classification.json.
- Selected discriminator contract:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core/discriminator-contract.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core/evidence-map.json.

## Acceptance Check

- Records why the blocker is outside the local source boundary and what lab
  serial invariant must be discriminated next: satisfied.
- Compares at least two different strategies and rejects same-shaped retry
  unless it adds new evidence: satisfied.
- Selected Pi 5 proof has exact command payload, cursor/read/observe strategy,
  terminal classifications, and inconclusive-run triage before code changes:
  satisfied.
- If no safe discriminator exists, selected_next_task=null with planningNeeded:
  not applicable; a safe discriminator is selected.
- Rejected claims remain explicit: satisfied.

## Validation

- static/source/task/evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-lab-write-boundary-pi5-proof-20260617 on
the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention remains inactive, and the repository
has no conflicting uncommitted changes. The proof must run the prearmed
live-read discriminator and must not accept source-response retention,
generated-root command-input success, storage, networking, SSH, Phase 11/12
expansion, or a phase transition.
