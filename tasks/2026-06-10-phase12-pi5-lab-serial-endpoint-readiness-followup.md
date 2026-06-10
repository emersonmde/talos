# Phase 12 Pi 5 Lab Serial Endpoint Readiness Follow-up

Task id: phase12-pi5-lab-serial-endpoint-readiness-followup-20260610

## Goal

If repaired known-good readiness still lacks serial markers despite restored
identity and TFTP fetches, isolate whether the remaining blocker is the lab
serial endpoint/capture path before any GPIO32 retry.

## Scope

- Consumed the accepted bounded runtime-readiness closeout because it explicitly
  selected this task for a repaired-helper serial/readiness blocker.
- Analyzed the retained readiness helper output, serial/read contracts, TFTP
  delta, status/files, and lab-controller health without changing Talos runtime
  code.
- Acquired the hardware lock for the no-power serial endpoint discriminator,
  then released it after evidence capture.
- Ran the smallest decisive no-power discriminator selected by the closeout:
  bounded `/serial/observe` from the saturated cursor versus direct
  `/serial/read` under the same restored known-good identity.
- Did not power cycle, publish a boot archive, write serial input, restore a
  snapshot, stage code, or run GPIO32 write/restore.

## Evidence Summary

- Run label:
  `serial-endpoint-readiness-followup-20260610T222700Z`.
- Lab health returned `ok=true`.
- Pre- and post-endpoint identity both matched restored known-good tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- Pre- and post-endpoint effective kernel were `kernel_2712.img`, and
  `da591740/kernel_2712.img` remained present with 104,136 bytes.
- `/serial/observe` from cursor `4194304` returned `ok=true`,
  `cursor_end=4194304`, and zero bytes.
- Direct `/serial/read` returned `ok=true`, cursor `4194304`, and zero
  bytes.
- The source proof's primary derived helper summary remains the only retained
  evidence that direct read captured post-power output: it read 7,046 bytes and
  saw `rpi5-production-timer-preemption: PASS`, but the raw primary helper
  artifact was overwritten and the retained raw helper JSON has zero bytes.

## Findings

- fixed: confirmed the accepted closeout dependency selected this follow-up and
  did not select GPIO32 write/restore v2.
- fixed: captured a no-power endpoint discriminator under stable restored
  known-good identity.
- fixed: confirmed the lab serial endpoints were reachable: both
  `/serial/observe` and direct `/serial/read` returned `ok=true`.
- not-an-issue: the no-power discriminator did not need a power cycle, serial
  write, boot archive, restore, or Talos runtime code change to answer the
  endpoint readability question.
- deferred: valid known-good Talos readiness remains unaccepted because the
  retained current device buffer is empty and the raw primary helper artifact
  from the prior proof was overwritten.
- deferred: GPIO32 write/restore v2 remains held until a later supervisor task
  accepts valid known-good Talos readiness under an explicit changed evidence
  contract.

## Classification

`serial-endpoints-readable-current-device-buffer-empty`

The no-power discriminator points away from an unavailable lab serial endpoint:
both endpoint calls were reachable and returned valid JSON under stable
restored known-good identity. It does not recover valid known-good Talos
readiness, because the current serial buffer is empty and the prior raw primary
runtime-readiness helper artifact is still overwritten.

Rejected claims:

- valid known-good Talos readiness
- GPIO32 write/restore v2 authorization
- PHY reset behavior
- Ethernet driver behavior
- packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition

## Validation

- static inspection of accepted bounded readiness closeout and retained
  helper/TFTP/status evidence: completed
- smallest lab-controller serial endpoint/control check:
  no-power `/serial/observe` versus direct `/serial/read` completed under
  hardware lock
- `jq empty` on task-owned JSON: passed
- `git diff --check`: passed
- `/home/node/.cargo/bin/mdbook build`: not required; no `docs/src` files
  touched
- `git diff --cached --check`: passed

## Evidence

- `tasks/evidence/2026-06-10-phase12-pi5-lab-serial-endpoint-readiness-followup/classification.json`
- `tasks/evidence/2026-06-10-phase12-pi5-lab-serial-endpoint-readiness-followup/evidence-map.json`
- `tasks/evidence/2026-06-10-phase12-pi5-lab-serial-endpoint-readiness-followup/endpoint-discriminator-summary.json`
- `tasks/evidence/2026-06-10-phase12-pi5-lab-serial-endpoint-readiness-followup/serial-observe-from-saturated-cursor.json`
- `tasks/evidence/2026-06-10-phase12-pi5-lab-serial-endpoint-readiness-followup/serial-direct-read-after-observe.json`
- Source closeout classification:
  `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-closeout/classification.json`
- Source proof primary summary:
  `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof/runtime-readiness-primary-summary.json`

Next action: supervisor planning required. Do not promote GPIO32 v2 because
valid-known-good-talos-readiness remains unaccepted. Do not repeat the same
no-power saturated-cursor endpoint discriminator without changed acceptance
criteria or a changed serial/readiness evidence contract.
