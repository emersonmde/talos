# Phase 12 Pi 5 Runtime Readiness Helper Saturation Core

Task id: phase12-pi5-runtime-readiness-helper-saturation-core-20260610
Status: accepted
Owner: worker
Classification: pi5-runtime-readiness-helper-saturation-core-accepted
Evidence level: source/docs inspection, shell syntax check, focused stub
validation, JSON contract inspection. No Pi 5 hardware run was performed.

## Goal

Repair the known-good runtime-readiness helper so a saturated serial cursor
does not let the next known-good control repeat the same serial-silent capture
failure as an unqualified cursor-observe result.

## Findings

- fixed: accepted recovery evidence showed restored known-good identity and
  TFTP fetch recovery, but serial observation from cursor 4194304 captured zero
  bytes.
- fixed: the runtime-readiness helper now shares the accepted saturation
  contract shape from the serial-window helper: auto mode keeps /serial/observe
  for unsaturated cursors and switches to direct /serial/read when the saved
  cursor is at the retention cap.
- fixed: helper output now records capture_mode, saturation_limit,
  start_cursor_saturated, response_bytes, and the actual observe_contract.
- fixed: saturated direct-read output with zero response bytes is classified as
  saturated-cursor-capture-blocked, not
  known-good-fetch-observed-without-talos-readiness.
- fixed: valid runtime readiness still requires both TALOS: kernel_main and
  the accepted success marker, defaulting to
  rpi5-production-timer-preemption: PASS.
- fixed: lab-controller documentation now names the saturated direct-read
  readiness contract and its blocked classification.
- deferred: whether restored known-good runtime output is currently readable
  remains a follow-up serialized Pi 5 proof under the repaired helper contract.
- not-an-issue: non-saturated fresh cursor behavior remains the
  deadline-loop accumulated observe path; only metadata was added.

No findings were removed.

## Accepted Boundary

This task accepts only the source/docs helper repair and local stub validation.
It does not accept a Pi 5 boot, GPIO32 write/restore, PHY reset
assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior, DMA,
descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or
a phase transition.

## Evidence

- Static inspection:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/static-inspection.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/evidence-map.json.
- Syntax check:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/sh-n-rpi5-observe-runtime-readiness.log.
- Non-saturated observe validation:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/non-saturated-observe-output.json and
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/non-saturated-observe-validation.log.
- Saturated direct-read validation:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/saturated-direct-read-output.json and
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/saturated-direct-read-validation.log.
- JSON and diff/docs gates:
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/jq-empty.log,
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/git-diff-check.log,
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/mdbook-build.log, and
  tasks/evidence/2026-06-10-phase12-pi5-runtime-readiness-helper-saturation-core/git-diff-cached-check.log.

## Validation

- static inspection: accepted recovery closeout, prior saturation repair, helper
  scripts, and lab-controller documentation reviewed.
- shell syntax: sh -n scripts/rpi5-observe-runtime-readiness.sh passed.
- focused stub validation: non-saturated observe mode preserved
  deadline-loop-accumulated-from-fresh-cursor and accepted readiness only with
  required markers.
- focused stub validation: saturated auto mode selected direct /serial/read,
  recorded deadline-loop-direct-read-after-saturated-cursor, and classified
  zero bytes as saturated-cursor-capture-blocked.
- JSON validation: jq empty on task-owned output, classification, and evidence
  JSON passed.
- diff check: git diff --check passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Mechanically promote
phase12-pi5-runtime-readiness-helper-saturation-closeout-20260610 on the next
worker wake. Do not run hardware or retry GPIO32 write/restore from this helper
repair task.
