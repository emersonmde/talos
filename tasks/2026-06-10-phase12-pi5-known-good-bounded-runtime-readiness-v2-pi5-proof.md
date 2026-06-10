# Phase 12 Pi 5 Known-Good Bounded Runtime Readiness V2 Proof

Task id: phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof-20260610

## Goal

Run the bounded known-good Pi 5 runtime-readiness proof under the repaired
primary-artifact retention contract.

## Scope

- Promoted the queued v2 proof after the retention closeout accepted the
  retained-primary helper contract.
- Acquired \`hardwareTestLock\` before the Pi 5 power cycle.
- Captured pre-power \`GET /status\`, \`GET /boot/files\`,
  \`GET /boot/snapshots\`, fresh serial cursor, and fresh TFTP cursor.
- Power-cycled the Pi 5 once and retained the primary runtime-readiness helper
  JSON under a run-label-qualified immutable path.
- Captured stable TFTP delta, final \`GET /status\`, final \`GET /boot/files\`,
  and hardware-lock release evidence.
- Did not run GPIO32 write/restore, assert/deassert PHY reset, change runtime
  code, publish a boot archive, or claim Ethernet/networking behavior.

## Evidence Summary

- Run label:
  \`known-good-runtime-readiness-v2-20260610T2332Z\`.
- Pre-power and final boot identity both remained on tree
  \`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10\`
  with \`kernel_2712.img\`.
- \`/power/cycle\` returned \`ok=true\`.
- The retained primary helper artifact used
  \`deadline-loop-direct-read-after-saturated-cursor\` because the saved serial
  cursor was at the retention cap.
- The stable TFTP delta had 13 events and included two served
  \`da591740/kernel_2712.img\` fetches at 104136 bytes.
- The retained serial output included the
  \`rpi5-production-timer-preemption: PASS\` marker, but did not retain
  \`TALOS: kernel_main\`.
- The helper therefore classified the retained primary artifact as
  \`known-good-fetch-observed-without-talos-readiness\` and exited 1.

## Findings

- fixed: hardware lock acquisition and release were recorded in durable state
  and task evidence.
- fixed: the proof used the repaired retained-primary wrapper, producing
  immutable primary JSON, summary, and status artifacts under the run label.
- fixed: stable TFTP evidence was captured before any restore/change and showed
  the expected known-good kernel fetches.
- fixed: final status/files matched the pre-power known-good boot identity, so
  no boot tree restore was required.
- blocked: \`valid-known-good-talos-readiness\` remains unaccepted because the
  retained helper contract did not classify the primary artifact as ready.
- deferred: GPIO32 write/restore v2 remains blocked until a closeout accepts
  \`valid-known-good-talos-readiness\` under the repaired helper contract.
- not-an-issue: no docs/src update was required; this proof only generated task
  evidence and a task record.

## Classification

\`known-good-fetch-pass-marker-observed-helper-readiness-unaccepted\`

The proof accepts stable known-good TFTP fetch evidence, retained primary helper
evidence, power-cycle evidence, and stable final boot identity. It does not
accept \`valid-known-good-talos-readiness\` because the retained primary helper
artifact did not set \`talos_runtime_readiness.valid_known_good_talos_readiness\`.
The observed PASS marker is recorded as evidence, but this task is not
authorized to reinterpret helper output as valid readiness.

Rejected claims:

- GPIO32 write/restore v2 authorization
- PHY reset behavior
- Ethernet driver behavior
- packet I/O
- networking, sockets, SSH
- Phase 12.2 or phase transition

## Validation

- serialized Pi 5 hardware proof through lab-controller API: completed
- repaired runtime-readiness helper output with immutable primary raw artifact:
  completed
- stable TFTP delta evidence: completed
- \`jq empty\` on task-owned JSON: passed
- \`git diff --check\`: passed
- \`/home/node/.cargo/bin/mdbook build\`: not run; no docs/src files were touched
- \`git diff --cached --check\`: passed

## Evidence

- \`tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/classification.json\`
- \`tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/evidence-map.json\`
- \`tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/capture-summary.json\`
- \`tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/known-good-runtime-readiness-v2-20260610T2332Z-runtime-readiness-primary.json\`
- \`tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/tftp-delta-stable.json\`
- \`tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/hardware-lock-released.json\`

Next action: mechanically promote
phase12-pi5-known-good-bounded-runtime-readiness-v2-closeout-20260610 on the
next worker wake. Do not promote GPIO32 v2 from this proof.
