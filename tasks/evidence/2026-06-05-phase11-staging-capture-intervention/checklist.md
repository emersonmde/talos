# Phase 11 Staging/Capture Intervention Checklist

Task context: `phase11-rp1-diagnostic-entry-control-closeout-20260605`

Evidence level: static inspection plus non-mutating lab-controller API reads.
No source change, archive publication, power cycle, serial write, boot restore,
hardware lock acquisition, or RP1 diagnostic change was performed.

## 1. First-Principles Problem Statement

Firmware `Boot mode: NETWORK` serial output means the Pi rebooted far enough for
the EEPROM/bootloader to choose the network boot path. It does not, by itself,
prove that the TFTP server served the currently selected boot tree, that
`kernel_2712.img` was fetched after the proof cursor, or that Talos reached
entry.

The latest proof recorded a stronger inconsistency: candidate, known-good
control, and candidate rerun all produced fresh serial bytes through NETWORK,
but the task-owned TFTP deltas were empty. From first principles, at least one
of these must be true:

- capture/tooling missed late or buffered TFTP log writes;
- the proof sampled the TFTP cursor/delta at the wrong point in the boot
  sequence;
- boot tree publication/restore changed the served tree before the firmware
  fetch completed;
- the Pi reached network mode but did not complete a TFTP fetch during the
  task-owned observation window;
- the evidence path was reading a stale or different TFTP log view.

The worker must not treat this as RP1, handoff, or source behavior until the
capture/staging invariant is restored.

## 2. Valid Pi 5 Hardware Proof Invariant

A valid proof needs all of these facts tied to one power-cycle attempt:

- selected boot tree identity: lab status immediately before power cycle
  records the intended `tree_hash`, `effective_kernel=kernel_2712.img`,
  and kernel file size/digest expected by the task;
- fresh serial cursor: a drained serial cursor is captured immediately before
  power cycle and the observed bytes after that cursor include the relevant
  firmware/Talos markers;
- fresh TFTP cursor: a TFTP cursor is captured immediately before power cycle,
  with cursor semantics recorded as a byte offset into the active log;
- fresh TFTP delta: after the boot observation window, querying from that
  cursor returns the files served for that same attempt;
- expected kernel fetch: the delta includes
  `da591740/kernel_2712.img` served to `10.42.1.4`, with bytes matching
  the selected boot tree for that attempt;
- restore state: restore happens only after the attempt has a decisive fetch
  or no-fetch classification, and final lab status proves the accepted tree was
  restored.

If any item is absent or temporally ambiguous, the proof is not allowed to
claim candidate fetch, Talos handoff, or RP1 behavior.

## 3. Contradicting Evidence From Latest Proof

- Candidate publish selected tree
  `ab88a3d8549837459c8cebf8cb22580b52b39665421b7eb6d6773ebce8c6f9c2`
  with 51,808-byte `kernel_2712.img`.
- First candidate serial follow-up
  `candidate-serial-observe-followup.json` recorded 1,733 fresh bytes from
  cursor `4087415` through `4089148` and reached
  `Boot mode: NETWORK`.
- First candidate TFTP delta
  `candidate-tftp-delta-followup-pre-restore.json` queried from cursor
  `4088847` and recorded zero fresh events.
- Known-good restore selected tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  with 104,136-byte `kernel_2712.img`.
- Known-good serial follow-up
  `known-good-serial-observe-followup.json` recorded 1,733 fresh bytes from
  cursor `4089148` through `4090881` and reached NETWORK.
- Known-good TFTP delta `known-good-tftp-delta-followup.json` queried
  from cursor `4088847` and recorded zero fresh events.
- Candidate rerun selected the candidate tree again with 51,808-byte
  `kernel_2712.img`.
- Candidate rerun serial final observe
  `candidate-rerun-serial-observe-final-pre-restore.json` recorded 1,733
  fresh bytes from cursor `4090881` through `4092614` and reached
  NETWORK.
- Candidate rerun TFTP delta
  `candidate-rerun-tftp-delta-followup-pre-restore.json` queried from
  cursor `4088847` and recorded zero fresh events.
- A non-mutating requery performed during this intervention from the same
  cursor `4088847` now returns TFTP events at Jun 5 16:33:36-16:33:38,
  including `da591740/kernel_2712.img` served with 104,136 bytes. That
  late-visible result contradicts the proof-time empty deltas and points first
  at capture timing/log freshness and restore timing, not RP1 source behavior.

## 4. Unproven Assumptions

- `/tftp/logs` cursor offsets are durable byte offsets across log buffering,
  truncation, and delayed writes.
- A proof-time zero-event delta means no TFTP fetch happened, rather than the
  log not yet being flushed into the lab-controller view.
- Serial cursor freshness and TFTP cursor freshness are comparable enough to
  correlate the same physical boot attempt.
- Publishing a tree and observing `GET /status` proves that tree remains the
  served tree until the firmware fetch completes.
- Restore after an inconclusive observe cannot race a still-progressing
  network fetch.
- The known-good control restore identity is sufficient unless the control
  fetch bytes are captured after the control cursor and before any later
  publish/restore.
- Repeated waits alone can repair the proof without changing the evidence
  invariant.

## 5. Qualitatively Different Approaches

Evidence-capture/tooling validation:

- Treat the TFTP log path as suspect before any source or RP1 iteration.
- Replay saved cursors, record log size/cursor monotonicity, and require a
  post-observe requery until the TFTP log is stable or a bounded timeout
  expires.
- Correlate serial observe windows with TFTP line timestamps and kernel byte
  sizes.
- This approach changes proof harness rules, not kernel behavior.

Boot-staging/publication validation:

- Treat the served boot tree as suspect before any source or RP1 iteration.
- Freeze restore until the TFTP decision is captured, record kernel sizes in
  status immediately before power cycle and immediately after the TFTP delta,
  and classify any fetch whose bytes do not match the selected attempt as
  staging/restore timing evidence.
- Use deliberately distinct candidate/control kernel sizes only as proof
  discriminators, then remove them from accepted feature infrastructure.

## 6. Smallest Decisive Discriminator

The smallest decisive discriminator is the evidence-capture/tooling validation
path, starting with non-mutating replay of the latest proof cursor
`4088847`. That has already found late-visible TFTP events absent from the
proof-time deltas:

- evidence:
  `tasks/evidence/2026-06-05-phase11-staging-capture-intervention/tftp-after-proof-cursor-4088847.json`;
- restored-tree status read:
  `tasks/evidence/2026-06-05-phase11-staging-capture-intervention/lab-status-after-intervention-read.json`;
- result: 13 parsed TFTP events, including
  `da591740/kernel_2712.img` served with 104,136 bytes;
- interpretation: the previous zero-event deltas cannot be treated as final
  no-fetch evidence without a log-stability rule. Because the late kernel size
  matches the restored known-good tree, not the 51,808-byte candidate tree, the
  next planned work should separate delayed capture from restore/publication
  timing before any RP1 diagnostic or source-level handoff change.

No hardware run is required to make that planning decision. A later hardware
proof, if supervisor-planned, should first repair the invariant: capture cursor,
power cycle, observe serial, wait/requery TFTP until cursor/log size is stable,
classify fetch bytes against the still-selected tree, then restore.

## 7. Workaround Removal Or Quarantine Plan

- Any new waits, cursor replays, alternate capture files, or proof scripts are
  quarantined under task-owned evidence until a closeout accepts the capture
  invariant.
- They must not become permanent feature infrastructure unless a task records
  the lab-controller semantics they prove and the exact condition they guard.
- Once the invariant is accepted, remove ad hoc one-off probes or promote only
  the smallest reusable proof-harness rule: "do not classify a Pi 5 boot until
  the TFTP cursor delta has been re-queried after log-size stabilization and
  checked against the selected boot tree."
- The RP1 diagnostic code and Phase 11 source path remain unchanged until a
  valid proof distinguishes candidate fetch, handoff failure, or RP1 behavior.

## Result

Supervisor intervention checklist satisfied. Planning is still required for
the next bounded Phase 11 task; this worker run does not promote or create a
task and does not resume implementation.
