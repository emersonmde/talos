# Phase 11 Lab Evidence Contract Repair Core

Task id: phase11-lab-evidence-contract-repair-core-20260605

Status: accepted

## Goal

Repair the no-hardware lab proof contract around boot identity, endpoint
semantics, stable TFTP evidence, and blocker classification before the next
Phase 11 hardware discriminator.

## Scope

- Reviewed the accepted staging/capture closeout, stable TFTP helper contract,
  known-good blocker evidence, lab-controller docs, and Phase 11 proof status.
- Normalized boot identity to the deployed `GET /status` endpoint and recorded
  `GET /` 404s as endpoint-semantics evidence only.
- Required future hardware proof bundles to retain status, boot files,
  snapshots, fresh serial/TFTP cursors, stable pre-restore TFTP evidence, and
  final pre-restore inconclusive samples.
- Added classification rules for staging/publication mismatch, TFTP
  capture/logging blindness, serial-only firmware reboot, and valid known-good
  Talos readiness.

## Non-Goals Honored

No runtime/kernel/RP1 source changes, boot archive publication, hardware power
cycle, hardwareTestLock acquisition, candidate rerun, GPIO ownership,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2 work, or phase transition was performed.

## Findings And Disposition

- fixed: future proofs now have one authoritative boot identity endpoint:
  `GET /status`.
- fixed: the deployed `GET /` discrepancy is documented as endpoint-semantics
  evidence rather than boot identity evidence.
- fixed: the next hardware proof checklist now includes boot files, snapshots,
  fresh serial/TFTP cursors, stable pre-restore TFTP evidence, and pre-restore
  inconclusive samples.
- fixed: blocker classifications distinguish staging/publication mismatch,
  TFTP capture/logging blindness, serial-only firmware reboot, and valid
  known-good Talos readiness without shrinking the RP1 proof target.
- not-an-issue: `scripts/rpi5-wait-tftp-delta.sh` already implements the
  accepted stable TFTP semantics, so no helper change was required.
- removed: no alternate capture path, workaround wait stack, boot publish, or
  hardware action was added.
- deferred: read-only deployed API sampling is left to the queued
  `phase11-known-good-boot-state-api-probe-20260605` task.

## Evidence

- Static inspection notes:
  `tasks/evidence/2026-06-05-phase11-lab-evidence-contract-repair-core/static-inspection.md`.
- Helper syntax evidence:
  `tasks/evidence/2026-06-05-phase11-lab-evidence-contract-repair-core/sh-n-rpi5-wait-tftp-delta.log`.
- Diff hygiene:
  `tasks/evidence/2026-06-05-phase11-lab-evidence-contract-repair-core/git-diff-check.log`.
- Docs validation:
  `tasks/evidence/2026-06-05-phase11-lab-evidence-contract-repair-core/mdbook-build.log`.
- Updated lab proof contract:
  `docs/src/project/lab-controller.md`.
- Updated Phase 11 proof status:
  `docs/src/project/phase11-rp1-pcie-map-contract.md`.
- Updated roadmap status:
  `docs/src/roadmap.md`.

## Validation

- static inspection of lab proof contract/docs/scripts: passed.
- proof helper syntax: `sh -n scripts/rpi5-wait-tftp-delta.sh` passed; no
  helper code changed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with the existing large
  search-index warning.
- git diff --cached --check before commit: passed.

## Result

Accepted no-hardware contract repair. The next queued task may perform a
read-only lab API probe against the repaired endpoint checklist. Hardware
proofs, RP1 candidate reruns, RP1 diagnostic/source changes, Milestone 11.2,
networking, SSH, GPIO, interrupts, DMA/cache, storage, generated-root, and
broader PCIe remain blocked until explicit queued dependencies accept them.
