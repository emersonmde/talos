# Phase 12 Pi 5 Capture Chain Repair Contract

Task id: phase12-pi5-capture-chain-repair-contract-20260610

Status: accepted

Classification: pi5-capture-chain-repair-contract-accepted

## Goal

Define the smallest local/static repair contract for the Pi 5 capture chain
that blocked the accepted GEM MID decode-discriminator proof.

## Scope

- Consumed accepted
  phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof-20260610
  evidence and commit a6cf3abcc72fc3d94214bd46bf21026be57f9e26.
- Inspected retained candidate/control capture summaries, direct serial
  windows, TFTP deltas, final identity files, restore evidence, the
  root-endpoint probe, and the capture/checker scripts.
- Defined one local/static repair contract for the capture helper and replay
  checkers.
- Did not modify code, acquire hardwareTestLock, publish a boot archive,
  power-cycle Pi 5, or change Ethernet diagnostic behavior.

## Findings And Disposition

- fixed: identified the failure class as a capture-chain identity/replay
  contract gap, not an Ethernet behavior result.
- fixed: GET / returned 404 in
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/root-endpoint-probe.txt;
  the repair contract must treat /boot/files as the selected-tree identity
  source and retain GET / 404 only as endpoint-semantics evidence.
- fixed: candidate-full-run retained the useful direct serial discriminator
  line, but its expected 49,152-byte candidate fetch and selected tree were
  not retained through final identity; TFTP/final identity showed the restored
  104,136-byte tree instead.
- fixed: candidate-debug-run had no expected TFTP fetch, non-empty pre-power
  drain, and final selected-tree mismatch, so it cannot repair the accepted
  proof from retained artifacts alone.
- fixed: control-direct-run retained a stable TFTP delta for the control boot
  but no serial control marker, so it cannot prove the paired no-MMIO/no-
  Ethernet marker path.
- fixed: the repair contract requires endpoint fallback, selected-tree
  identity source retention, TFTP/final identity replay, run-unique marker
  freshness, direct serial marker retention, and candidate/control summary
  fields in one mechanically checkable helper/checker shape.
- deferred: no Pi 5 rerun is authorized by this contract alone; the repaired
  helper implementation and closeout must be accepted first.
- not-an-issue: no Ethernet driver behavior, packet I/O, DMA, interrupt,
  networking, sockets, SSH, Phase 12.2, or phase transition is accepted here.

No findings were removed.

## Retained Inputs

- Accepted proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof.md.
- Accepted proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/classification.json.
- Accepted proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/evidence-map.json.
- Root endpoint probe:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/root-endpoint-probe.txt.
- Candidate positive serial window:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/candidate-full-run/direct-serial-window-after-observe-stop.json.
- Candidate full capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/candidate-full-run/capture-invariant-summary.json.
- Candidate debug capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/candidate-debug-run/capture-invariant-summary.json.
- Control direct serial window:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/control-direct-run/direct-serial-window-after-observe-stop.json.
- Control direct TFTP/final identity:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/control-direct-run/tftp-delta-stable-pre-restore.json
  and
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/control-direct-run/final-pre-restore-boot-files.json.
- Restored lab identity:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/final-lab-root-before-lock-release.json.
- Relevant helpers:
  scripts/rpi5-capture-invariant-proof-bundle.sh,
  scripts/rpi5-observe-serial-window.sh,
  scripts/rpi5-wait-tftp-delta.sh,
  scripts/rpi5-proof-identity-join-check.sh,
  scripts/rpi5-proof-identity-join-v3-check.sh, and
  scripts/rpi5-proof-identity-join-run-unique-check.sh.

## Existing Artifact Reclassification

The existing evidence cannot be reclassified into a decisive hardware proof
from retained artifacts alone.

Candidate-full-run retained the selected serial line and an empty pre-power
drain, but the identity join rejected it for
tftp-expected-fetch-byte-mismatch, final-pre-restore-tree-mismatch,
final-pre-restore-selected-tree-mismatch, and
final-pre-restore-expected-fetch-byte-mismatch. The observed TFTP events for
the expected path were 104,136-byte restored-tree fetches rather than the
49,152-byte candidate fetch recorded at preflight, and the final identity had
returned to tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Candidate-debug-run and control-run/control-direct-run also remain blockers:
candidate-debug-run lacks the expected TFTP fetch and has final selected-tree
mismatch, while control-direct-run has no retained control serial marker even
though it retained a stable TFTP delta. These retained artifacts are useful
for designing the repair, but they do not satisfy the selected candidate plus
paired control proof contract.

## Repair Contract

The next implementation task should add exactly one local/static capture-chain
repair shape: pi5-capture-chain-v4.

Required behavior:

- Endpoint fallback: the helper must probe GET / only as optional endpoint
  evidence. If it returns 404, the helper must use GET /boot/files as the
  authoritative boot identity source and record the endpoint fallback in the
  run summary.
- Selected-tree identity source: preflight, final-pre-restore, and restore
  identity must all be derived from the same selected-tree fields reported by
  /boot/files: tree_hash, effective_kernel, expected fetch path, and expected
  fetch byte count.
- TFTP/final identity replay: the replay checker must reject any run where the
  stable TFTP delta's expected fetch bytes or the final-pre-restore selected
  tree differ from preflight. It must also report the observed fetch byte list
  and final selected-tree status in the summary.
- Run-unique marker freshness: saturated direct-read serial may be accepted
  only through the existing run-unique nonce rule: the nonce token must be
  absent from all pre-power drain responses and present after power.
- Direct serial marker retention: when serial capture uses direct /serial/read
  because the cursor is saturated, the helper must retain the accumulated
  response text, marker occurrence count, nonce occurrence count, capture mode,
  cursor fields, response byte count, and a marker excerpt for the requested
  marker.
- Candidate/control summary fields: each summary must name run label, report
  kind, expected tree/fetch identity, selected-tree status, TFTP fetch counts
  and byte lists, final-pre-restore identity, restore identity, serial capture
  mode, marker/nonce freshness, rejection reasons, accepted/rejected claim
  boundary, and whether decisive RP1 hardware classification is allowed.

The repair is local/static helper and checker behavior only. It must not
change Ethernet diagnostic output, construct new MMIO targets, acquire
hardwareTestLock, publish a boot archive, or rerun Pi 5 hardware.

## Rejected Claims And Retained Risks

Rejected: live GEM visibility, broad Ethernet MMIO readiness, Ethernet driver
readiness, RP1 MMIO/DMA programming, descriptor rings, DMA ownership, transfer
completion, interrupt completion, clock/reset ownership, PHY reset ownership,
packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition.

Retained risks: the translated 0x1f RP1 window sentinel remains unproven; PCIe
or RP1 bridge/window enablement remains unaccepted; Ethernet clock/reset and
PHY/MDIO ownership remain unaccepted; the next Pi 5 hardware proof remains
blocked until phase12-pi5-capture-chain-repair-core-20260610 and its closeout
are accepted.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-contract/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-contract/classification.json.

## Validation

- static inspection: accepted proof task record, classification JSON, evidence
  map, retained capture summaries/windows, root-endpoint probe, and relevant
  scripts reviewed.
- jq empty on task-owned evidence-map/classification JSON: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No docs/src files or shell scripts were changed, so mdbook and bash -n gates
were not required for this contract-only task.

## Next Action

phase12-pi5-capture-chain-repair-core-20260610 is mechanically objective:
implement the pi5-capture-chain-v4 local/static helper/checker repair above,
then run fixture/replay validation before any guarded Pi 5 rerun. Do not run
hardware, change Ethernet behavior, or start Phase 12.2 from this contract.
