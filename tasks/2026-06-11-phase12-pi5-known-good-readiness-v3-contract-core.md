# Phase 12 Pi 5 Known-Good Readiness V3 Contract Core

Task id: phase12-pi5-known-good-readiness-v3-contract-core-20260611

## Goal

Implement the local/source v3 known-good runtime-readiness contract so retained
primary artifacts can be classified without relying on mutable/latest paths and
without making `TALOS: kernel_main` a mandatory known-good readiness marker.

## Scope

- Consumed the accepted v2 readiness proof and closeout evidence, including the
  retained primary helper artifact, stable TFTP delta, pre/final status, and
  blocker classification.
- Added `scripts/rpi5-known-good-readiness-v3-classify.sh` to classify a
  retained run-label-qualified primary artifact joined to stable boot identity,
  stable TFTP evidence, served effective-kernel fetches, and the accepted
  production success marker.
- Added `scripts/rpi5-known-good-readiness-v3-fixtures.sh` to exercise the v3
  contract against retained v2 evidence and deterministic rejection fixtures.
- Did not run Pi 5 hardware, publish a boot archive, acquire
  `hardwareTestLock`, change GPIO32/PHY reset behavior, or claim Ethernet,
  networking, sockets, SSH, Phase 12.2, or any phase transition.

## Contract

The accepted v3 classifier sets
`talos_runtime_readiness_v3.valid_known_good_talos_readiness_v3=true` only when
all of the following are true:

- the primary artifact path is retained and run-label-qualified, ending in
  `-runtime-readiness-primary.json`, and does not contain `latest`;
- the primary serial artifact contains the accepted production success marker
  `rpi5-production-timer-preemption: PASS`;
- pre/final lab-controller status both report stable boot tree identity and the
  same effective kernel;
- stable TFTP evidence is present and includes at least one served fetch for
  the final effective kernel;
- retained primary and TFTP cursor windows are present.

`TALOS: kernel_main` is metadata only in this v3 contract. Its absence is
recorded as a retained risk, not as a mandatory readiness failure.

## Findings

- fixed: added a v3 classifier that accepts retained primary artifacts only
  through joined stable status/TFTP evidence plus the production success marker.
- fixed: recorded `TALOS: kernel_main` absence as metadata/retained risk rather
  than a mandatory readiness marker.
- fixed: added local fixture coverage for retained v2 primary acceptance,
  missing success marker rejection, missing identity/TFTP join rejection, and
  primary-artifact overwrite prevention through the existing retention guard.
- not-an-issue: no Pi 5 hardware run or hardware lock acquisition was required
  for this source/script/evidence-workflow task.
- not-an-issue: no docs/src update was required because the contract and helper
  names are recorded in task-owned evidence.

## Classification

`known-good-readiness-v3-contract-core-local-static-accepted`

The retained v2 primary artifact classifies as
`valid-known-good-talos-readiness-v3` under the v3 contract when joined with the
same proof stable TFTP delta and stable pre/final boot identity. This task
accepts only the local/static v3 contract and fixture validation; GPIO32 v2 is
not unlocked until the queued v3 closeout explicitly accepts the contract and
selects the hardware proof path.

Rejected claims:

- GPIO32 write/restore authorization by this core task
- PHY reset assertion/deassertion
- MDIO/PHY ownership
- Ethernet driver behavior
- DMA/descriptors
- interrupts
- packet I/O
- networking, sockets, SSH
- Phase 12.2 or phase transition

## Validation

- static inspection of accepted v2 proof/closeout records, retained primary
  JSON/summary, helper scripts, and lab-controller docs: completed
- `bash -n` on touched shell scripts: passed
- focused local fixture/guard validation: passed
- `jq empty` on task-owned evidence JSON: passed
- `git diff --check`: passed
- `/home/node/.cargo/bin/mdbook build`: not run; no docs/src files were touched
- `git diff --cached --check`: passed

## Evidence

- `tasks/evidence/2026-06-11-phase12-pi5-known-good-readiness-v3-contract-core/classification.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-readiness-v3-contract-core/evidence-map.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-readiness-v3-contract-core/retained-v2-v3-classification.json`
- `tasks/evidence/2026-06-11-phase12-pi5-known-good-readiness-v3-contract-core/fixture-validation.json`
- `scripts/rpi5-known-good-readiness-v3-classify.sh`
- `scripts/rpi5-known-good-readiness-v3-fixtures.sh`

Next action: mechanically promote
phase12-pi5-known-good-readiness-v3-contract-closeout-20260611 on the next
worker wake. Do not run hardware from the core task.
