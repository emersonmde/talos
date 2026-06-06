# Phase 11 Pi 5 Proof Identity Join Repair Core

Task id: phase11-pi5-proof-identity-join-repair-core-20260606

Status: accepted

## Goal

Repair the Pi 5 proof bundle/evidence join so candidate identity, serial
cursor/window, TFTP delta, and observed serial bytes are tied to one run before
any further RP1 UART0 FR-read hardware attempt.

## Scope

- Inspected the accepted hold-control RP1 UART0 FR-read blocker evidence and
  the capture-invariant proof-bundle helpers.
- Identified the evidence-join gap that allowed visible serial bytes and TFTP
  events to be too weakly tied to the selected candidate.
- Updated the capture-invariant summary contract and added a no-hardware replay
  checker for the `pi5-proof-identity-join-v1` proof contract.
- Replayed the repaired checker against the retained hold-control candidate-run
  evidence.
- Updated the lab-controller proof contract and Phase 11 map-contract proof
  status language.
- Did not run hardware, publish a boot archive, acquire hardwareTestLock, power
  cycle the Pi 5, or change RP1 diagnostic source.

## Final Classification

Classification: proof-harness-identity-join-repaired.

The repaired contract requires one shared run label to tie the selected tree
hash, effective kernel, expected fetch path and byte count, serial cursor and
window identity, stable TFTP cursor/delta identity, final pre-restore identity,
and restore identity. If any of those fields are missing or mismatched, the
bundle is classified as `capture-staging-blocked` and cannot support a decisive
RP1 hardware classification.

Replaying the retained hold-control candidate-run evidence keeps the old
blocker classified as `capture-staging-blocked`. The run label, selected tree,
effective kernel, serial window, TFTP cursor, and restore identity are present,
but the stable TFTP delta contains two 104,136-byte restored-tree
`da591740/kernel_2712.img` fetches instead of the selected 46,320-byte
hold-control candidate, and final pre-restore identity has changed to the
restored tree. The visible post-read loop serial bytes therefore remain
untied to selected-candidate fetch evidence.

Accepted claims are limited to proof-harness/evidence-chain repair. This task
does not accept RP1 mapped/read-value, bus-fault/trap, pre-read-control
visibility, candidate-fetch-without-control-marker, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: the capture-invariant dry-run contract now names
  `pi5-proof-identity-join-v1` and its required selected-tree, kernel, fetch,
  serial, TFTP, final pre-restore, and restore fields.
- fixed: `capture-invariant-summary.json` now records `proof_run_identity`,
  `identity_join_contract`, expected-fetch byte-match counts, and rejection
  reasons before suggesting a capture classification.
- fixed: `scripts/rpi5-proof-identity-join-check.sh` replays retained proof
  bundles without hardware and exits nonzero when decisive RP1 classification
  is blocked by missing or mismatched identity-join fields.
- fixed: replay against retained hold-control evidence rejects decisive RP1
  classification because TFTP fetch bytes and final pre-restore tree identity
  do not match the selected 46,320-byte candidate.
- fixed: docs/src/project/lab-controller.md documents the checker as the gate
  before accepting decisive RP1 hardware behavior from capture-invariant
  bundles.
- fixed: docs/src/project/phase11-rp1-pcie-map-contract.md records that the
  hold-control blocker remains capture-staging-blocked under the repaired
  identity-join contract.
- not-an-issue: the retained hold-control serial bytes remain useful blocker
  evidence, but they are not candidate-tied hardware behavior.
- deferred: the queued known-good control task must prove the repaired proof
  chain on hardware before any FR-read candidate rerun is mechanically valid.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/evidence-map.json.
- Capture-invariant dry-run:
  tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/capture-invariant-proof-bundle-dry-run.json.
- Retained hold-control replay:
  tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/hold-control-candidate-run-identity-join-check.json.
- Replay exit code:
  tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/hold-control-candidate-run-identity-join-check.exit.

## Validation

- static inspection of latest hold-control blocker evidence and
  capture-invariant helper records: passed.
- bash -n scripts/rpi5-capture-invariant-proof-bundle.sh
  scripts/rpi5-proof-identity-join-check.sh: passed.
- no-hardware capture-invariant dry-run: passed.
- no-hardware replay of repaired identity-join checker against retained
  hold-control candidate-run evidence: passed with expected nonzero blocker
  exit and classification `capture-staging-blocked`.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as proof-harness-identity-join-repaired.

The next queued hardware control may prove only the repaired proof-chain
identity join on a known-good run if hardwareTestLock remains unlocked and
restored. It must not accept RP1 UART0 FR mapped/read-value or trap/unmapped
behavior.
