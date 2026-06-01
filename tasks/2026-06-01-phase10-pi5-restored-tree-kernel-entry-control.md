# Phase 10 Pi 5 Restored Tree Kernel Entry Control

Task: phase10-pi5-restored-tree-kernel-entry-control-20260601

Status: accepted

## Goal

Determine whether the currently restored accepted Pi 5 boot tree reaches Talos
kernel entry and prompt-level local serial interactivity before resuming the
paused help-command proof.

## Scope

This task ran one serialized restored-tree control under hardwareTestLock. It
used the current restored boot tree only, did not publish or rerun the
help-command candidate archive, and did not change Talos runtime, help-command
code, proof harness code, boot scripts, lab-controller code, roadmap, or ADRs.

## Result

Classification: restored-tree-firmware-only.

The fixed Weathertop port 8 power cycle succeeded and retained fresh Raspberry
Pi firmware/RP1 serial bytes from cursor 3828040. TFTP advanced from cursor
3974753 with 13 events and two kernel_2712.img serves. The retained fresh
serial delta did not contain TALOS:, talos:, talos>, prior command-loop
markers, or PASS/classification markers, so no prompt command was sent.

The restored boot tree was preserved:

- configured kernel: kernel_2712.img
- effective kernel: kernel_2712.img
- tree hash: a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

The next blocker is that the restored accepted tree reboots through firmware
and TFTP, but no Talos kernel-entry or prompt bytes are retained from the fresh
serial cursor. The paused help proof remains quarantined; this task did not
rerun the help candidate or change help/runtime behavior.

## Evidence

Evidence directory:

- tasks/evidence/2026-06-01-pi5-restored-tree-kernel-entry-control/local1-restored-tree-control/

Key retained files:

- pre-status.json: pre-run boot identity, guard, snapshots, and tree state.
- pre-serial-peek.json and serial-cursor-before.txt: fresh serial cursor
  3828040.
- pre-tftp-tail.json and tftp-cursor-before.txt: fresh TFTP cursor 3974753.
- power-cycle.json: fixed-port power cycle succeeded on Weathertop port 8 with
  PoE UP.
- serial-observe-after-power-1.json, serial-observe-after-power-2.json, and
  serial-observe-after-power-3.json: fresh serial deltas with firmware/RP1
  output only.
- serial-delta-after-power.txt: decoded serial delta for review.
- tftp-delta-after-power-3.json: 13 fresh TFTP events, including two kernel
  serves.
- post-status.json: post-run boot identity and preserved restored tree hash.
- control-result.txt: classification summary and next blocker.

The lab API root endpoint returned HTTP 404 in this deployment; the task
retained pre-root-http-status.txt and post-root-http-status.txt and used the
documented /status boot identity fields instead.

## Hardware Lock

- owner task: phase10-pi5-restored-tree-kernel-entry-control-20260601
- hardware lock acquired: recorded in durable supervisor state for this task.
- hardware action: one restored-tree power cycle plus serial/TFTP observation.
- help candidate publication/rerun: not performed.
- restore state: current restored boot tree preserved; no archive publication
  occurred, so no rollback restore was needed.
- hardware lock release: recorded in durable supervisor state after acceptance.

## Validation

- static evidence review: control-result.txt, decoded serial delta, TFTP delta,
  power-cycle JSON, and pre/post status JSON were inspected.
- serialized Pi 5 restored-tree control: one fixed-port power cycle and
  serial/TFTP observation produced the retained restored-tree-firmware-only
  classification.
- post-run boot identity proof: post-status.json reports the preserved restored
  tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- static inspection: git diff --check passed.
- documentation: mdbook build was not required; no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

Acceptance commit: recorded in durable supervisor state after commit creation.
