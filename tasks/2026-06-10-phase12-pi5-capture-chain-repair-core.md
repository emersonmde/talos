# Phase 12 Pi 5 Capture Chain Repair Core

Task id: phase12-pi5-capture-chain-repair-core-20260610

Status: accepted

Classification: pi5-capture-chain-repair-core-local-static-accepted

## Goal

Implement the accepted local/static Pi 5 capture-chain repair without running
hardware or changing Ethernet diagnostic behavior.

## Scope

- Consumed accepted repair contract
  phase12-pi5-capture-chain-repair-contract-20260610 and commit
  ba22f223a20d46ed7da44cfdde4f50c97a5e5184.
- Updated only capture/checker helper scripts:
  scripts/rpi5-capture-invariant-proof-bundle.sh,
  scripts/rpi5-observe-serial-window.sh,
  scripts/rpi5-proof-identity-join-v4-check.sh, and
  scripts/rpi5-capture-chain-v4-retained-fixtures.sh.
- Did not change Ethernet diagnostic source, RP1 MMIO behavior, boot archive
  publication, hardwareTestLock, or Pi 5 hardware state.

## Findings And Disposition

- fixed: capture bundle dry-run and live helper metadata now name
  pi5-capture-chain-v4 and retain GET / endpoint evidence separately from
  /boot/files selected-tree identity.
- fixed: GET / fallback is explicit; if GET / is unusable, /boot/files remains
  the authoritative selected-tree identity source for preflight, final, and
  restore checks.
- fixed: direct serial windows now retain marker occurrence count, nonce token,
  nonce occurrence count, capture mode, response byte count, cursor fields, and
  a marker excerpt.
- fixed: added scripts/rpi5-proof-identity-join-v4-check.sh to replay
  selected-tree identity, expected TFTP bytes, final pre-restore identity,
  run-unique marker freshness, direct serial marker retention, and
  candidate/control marker expectations.
- fixed: added retained fixture replay covering accepted endpoint fallback and
  deterministic rejection for missing selected identity, missing expected TFTP,
  missing final identity, missing marker, stale nonce, and missing control
  marker.
- not-an-issue: existing Ethernet diagnostic output already supports
  TALOS_CAPTURE_NONCE; no Ethernet report semantics changed.
- deferred: no Pi 5 rerun is authorized by this core alone. The closeout must
  select the guarded v2 proof before hardware can run.

No findings were removed.

## Evidence

- Retained fixture replay:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-core/retained-fixture-replay.json.
- Capture bundle dry-run:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-core/capture-bundle-dry-run.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-core/classification.json.

## Validation

- static inspection: accepted repair contract, retained proof artifacts, and
  touched scripts reviewed.
- bash -n on touched shell scripts: passed.
- task-owned replay/fixture command: passed; retained-fixture-replay.json shows
  seven fixture cases and passed=true.
- capture-bundle dry-run: passed and records pi5-capture-chain-v4 endpoint
  fallback metadata.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No Rust source or docs/src files were touched, so cargo and mdbook gates were
not required.

## Next Action

phase12-pi5-capture-chain-repair-closeout-20260610 is mechanically objective:
review the accepted v4 helper/checker evidence and select the guarded Pi 5
decode-discriminator v2 proof only if the closeout accepts the capture-chain
repair. Do not run hardware, change Ethernet behavior, or start Phase 12.2 from
this core task.
