# Phase 12 Pi 5 Known-Good Bounded Runtime Readiness V3 Proof

Task id: phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof-20260611

## Goal

Run the bounded known-good Pi 5 runtime-readiness proof under the accepted v3
readiness contract.

## Scope

- Promoted the queued v3 proof after the v3 contract closeout accepted the
  changed contract and selected this serialized hardware proof.
- Acquired `hardwareTestLock` before the Pi 5 power cycle and released it after
  the final boot identity check.
- Captured pre-power `GET /status`, `GET /boot/files`,
  `GET /boot/snapshots`, fresh serial cursor, and fresh TFTP cursor.
- Power-cycled the Pi 5 once and retained the primary runtime-readiness helper
  JSON under a run-label-qualified immutable path.
- Captured stable TFTP delta, final `GET /status`, final `GET /boot/files`,
  and v3 classifier output over the retained primary artifact joined with
  same-run status/TFTP evidence.
- Did not run GPIO32 write/restore, assert/deassert PHY reset, change runtime
  code, publish a boot archive, or claim Ethernet/networking behavior.

## Evidence Summary

- Run label: `known-good-runtime-readiness-v3-20260611T0100Z`.
- Pre-power and final boot identity both remained on tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  with `kernel_2712.img`.
- `/power/cycle` returned `ok=true`.
- The retained primary helper artifact used
  `deadline-loop-direct-read-after-saturated-cursor` because the saved serial
  cursor was at the retention cap.
- The stable TFTP delta had 13 events and included two served
  `kernel_2712.img` fetches.
- The retained serial output included the
  `rpi5-production-timer-preemption: PASS` marker and did not retain
  `TALOS: kernel_main`.
- The older retained-primary helper exited 1 under the v2 helper contract, but
  the accepted v3 classifier accepted the same retained primary artifact when
  joined to stable boot identity and stable TFTP evidence.

## Findings

- fixed: hardware lock acquisition and release were recorded in durable state
  and task evidence.
- fixed: the proof used the retained-primary wrapper, producing immutable
  primary JSON, summary, and status artifacts under the run label.
- fixed: stable TFTP evidence was captured from a fresh cursor and showed the
  expected known-good kernel fetches.
- fixed: final status/files matched the pre-power known-good boot identity, so
  no boot tree restore was required.
- fixed: the accepted v3 classifier set
  `valid_known_good_talos_readiness_v3=true` from retained serial hardware
  output joined with stable status/TFTP evidence.
- deferred: GPIO32 write/restore v2 remains blocked until the v3 closeout
  explicitly accepts the readiness proof and selects that guarded task.
- not-an-issue: no docs/src update was required; this proof only generated
  task evidence and a task record.

## Classification

`valid-known-good-talos-readiness-v3`

The proof accepts `valid-known-good-talos-readiness-v3` under the accepted v3
contract. The evidence level is serial hardware boot/output plus
lab-controller status/TFTP evidence classified through the accepted local/static
v3 contract. This task does not unlock GPIO32 write/restore v2 by itself; that
decision belongs to the queued v3 closeout.

Rejected claims:

- GPIO32 write/restore v2 authorization by this proof alone
- PHY reset behavior
- Ethernet driver behavior
- packet I/O
- networking, sockets, SSH
- Phase 12.2 or phase transition

## Validation

- serialized Pi 5 hardware proof through lab-controller API: completed
- accepted v3 runtime-readiness helper output with immutable primary raw
  artifact and summary: completed
- stable TFTP delta evidence: completed
- `jq empty` on task-owned JSON: passed
- `git diff --check`: passed
- `/home/node/.cargo/bin/mdbook build`: not run; no docs/src files were touched
- `git diff --cached --check`: passed

## Evidence

- `tasks/evidence/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof/classification.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof/evidence-map.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof/capture-summary.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof/known-good-runtime-readiness-v3-20260611T0100Z-runtime-readiness-primary.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof/readiness-v3-classification.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof/tftp-delta-stable.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof/hardware-lock-released.json`

Next action: mechanically promote
phase12-pi5-known-good-bounded-runtime-readiness-v3-closeout-20260611 on the
next worker wake. GPIO32 v2 remains held until that closeout accepts
`valid-known-good-talos-readiness-v3` and explicitly selects it.
