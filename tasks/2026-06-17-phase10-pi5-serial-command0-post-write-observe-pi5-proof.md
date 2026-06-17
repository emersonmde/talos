# Phase 10 Pi 5 Serial Command 0 Post-Write Observe Pi 5 Proof

Task id: phase10-pi5-serial-command0-post-write-observe-pi5-proof-20260617

Status: accepted

Classification:
command0-post-write-observe-inconclusive-serial-cursor-saturated

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, cursor-bound /serial/observe hardware output, same-cursor
stable TFTP log evidence, restore proof, task-owned JSON evidence, docs build,
and diff checks.

## Goal

Run the selected cursor-bound post-write observe proof for command 0: after a
visible generated-root prompt and an accepted rootinfo write, retain command0
output with POST /serial/observe from the saved pre-write cursor before
accepting command0 write delivery.

## Result

The run did not accept command0 write delivery. It is also not a command-loop
failure proof, because the selected observe transaction never retained a fresh
command0 readiness/prompt boundary.

The selected candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published. Its archive SHA-256 was
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c, and
kernel_2712.img was the expected 208984 bytes.

Before power-cycle and before restore, /boot/files exposed selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212,
effective kernel kernel_2712.img, and da591740/kernel_2712.img at 208984
bytes. The same-cursor TFTP delta retained two selected
da591740/kernel_2712.img serves, both at 208984 bytes.

The serial cursor was already saturated at 4194304 before the run. Repeated
POST /serial/observe calls from that cursor retained zero bytes during the
readiness loop, and the post-write observe window from the same cursor also
retained zero bytes after /serial/write accepted 9 bytes for rootinfo. The
helper therefore rejected the evidence. A non-gating post-run peek after
restore showed that the rootinfo write was eventually processed as stale later
command=3 after command=1 and command=2 timeouts, which does not satisfy the
command0 write-delivery contract.

Restore returned the lab to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with the
104136-byte baseline kernel.

## Findings

- fixed: selected-kernel/TFTP agreement remained proven during this observe
  proof; two same-power-cycle selected kernel serves matched 208984 bytes.
- fixed: final pre-restore identity and post-run restore proof were retained.
- inconclusive: cursor-bound /serial/observe could not retain fresh command0
  readiness because the serial log cursor was saturated at 4194304.
- inconclusive: post-write /serial/observe from the saved saturated cursor
  retained zero bytes, so command0 line/dispatch/responses/ready evidence was
  not evaluable.
- deferred: a closeout/supervisor decision is needed before choosing a
  different freshness discriminator or retrying with a different capture
  contract.
- rejected: command0 source-response retention success, generated-root
  command-input success, storage, networking, SSH, Phase 11/12 expansion, and
  phase transition.

## Evidence

- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/evidence-map.json.
- Selected run:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/candidate-command0-post-write-observe-20260617T150944Z/.
- Run classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/candidate-command0-post-write-observe-20260617T150944Z/classification.json.
- Observe evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/candidate-command0-post-write-observe-20260617T150944Z/post-write-observe-evidence.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/candidate-command0-post-write-observe-20260617T150944Z/tftp/tftp-delta-stable-pre-restore.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/candidate-command0-post-write-observe-20260617T150944Z/restore/post-restore-boot-files.json.

## Acceptance Check

- Accepted proof retains ready command=0, visible prompt, fresh pre-write
  boundary/cursor, accepted 9-byte rootinfo write, ordered post-write command0
  line or rootinfo evidence, dispatch command=0 status=handled, responses=1,
  ready command=1, selected-kernel/TFTP byte agreement, final identity, and
  restore proof: not satisfied; fresh readiness and post-write observe evidence
  were both missing because the observe cursor was saturated.
- Blocked proof records the first failing invariant without claiming
  generated-root command-input success: satisfied as an inconclusive serial
  cursor/capture boundary, not as command-loop behavior.
- Inconclusive proof records the exact missing evidence: satisfied; fresh
  serial cursor/readiness and post-write observe evidence were missing, while
  candidate identity, TFTP, final identity, and restore proof were present.
- selected_next_task is
  phase10-pi5-serial-command0-post-write-observe-closeout-20260617: satisfied.
- Rejected claims remain explicit: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: inconclusive at
  cursor-bound serial observe freshness.
- candidate identity via lab API /boot/files before run: pass.
- fresh serial cursor and post-write /serial/observe evidence per accepted
  contract: inconclusive; /serial/observe from cursor 4194304 returned zero
  bytes.
- TFTP delta via GET /tftp/logs before restore, stable under same-cursor
  re-query: pass.
- post-run baseline restore proof: pass.
- task-owned proof validator: expected fail for the inconclusive selected
  hardware evidence.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-post-write-observe-closeout-20260617 on the
next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, and supervisorIntervention is inactive. The closeout should
decide whether supervisor planning is required for a different serial
freshness discriminator rather than a same-shaped saturated-cursor observe
retry. Do not accept command0 source-response retention, generated-root
command-input success, storage, networking, SSH, Phase 11/12 expansion, or a
phase transition from this proof.
