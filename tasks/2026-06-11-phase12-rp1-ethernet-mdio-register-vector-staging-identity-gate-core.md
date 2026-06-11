# Phase 12 RP1 Ethernet MDIO Register Vector Staging Identity Gate Core

Task id: phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core-20260611

Status: accepted

Classification: staging-identity-gate-core-accepted

Evidence level: static inspection, task-owned fixture replay, jq JSON
validation, and diff checks.

## Goal

Build a local/static staging identity gate so future register-vector hardware
evidence cannot be accepted unless selected-tree identity survives the
publish-to-power-to-TFTP/final-status path.

## Scope Performed

- Added scripts/rpi5-staging-identity-gate-v1-check.sh to replay retained
  pi5-capture-chain-v4 JSON without hardware.
- Used the accepted guarded v2 candidate mismatch as the negative fixture.
- Used the accepted guarded v2 paired no-MDIO/no-Ethernet control as the
  positive fixture.
- Recorded candidate/control gate JSON, exit codes, classification JSON, and
  evidence map.

## Findings

- fixed: the gate rejects the guarded v2 candidate fixture with the same first
  capture-chain-v4 rejection reason, tftp-expected-fetch-byte-mismatch.
- fixed: the gate maps that first rejection to the bounded identity invariant
  expected-fetch-byte-mismatch.
- fixed: the gate accepts the guarded v2 paired control as
  selected-tree-identity-ready without accepting MDIO, Ethernet, or runtime
  hardware behavior.
- fixed: gate JSON names selected tree, expected fetch bytes/count, observed
  TFTP bytes/count, final pre-restore tree, restore tree, serial freshness
  result, and first failing invariant.
- deferred: the serialized staging sentinel proof remains the next hardware
  discriminator; register-vector v3 remains blocked until closeout plus that
  sentinel accepts selected-tree identity durability.
- deferred: register-vector MAN.DATA values, broad MDIO/PHY ownership, PHY
  absence, PHY reset/GPIO32 ownership, Ethernet behavior, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future work.
- removed: no source code, docs, or stale evidence were removed.
- not-an-issue: the candidate serial marker is fresh, but the gate still
  rejects the candidate because TFTP bytes and final pre-restore identity do
  not match the selected tree.

## Gate Behavior

The gate consumes retained v4 JSON and optional evidence-consistency guard
JSON. It does not inspect or accept register-vector values. It emits
pi5-staging-identity-gate-v1 JSON and exits 0 only when v4 identity evidence,
serial freshness, and the supplied consistency guard are ready.

Negative fixture:
tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/candidate-run/v4-check.json.

- gate classification: selected-tree-identity-blocked.
- exit code: 1.
- first failing rejection reason: tftp-expected-fetch-byte-mismatch.
- first failing invariant: expected-fetch-byte-mismatch.
- selected tree: dd4ad2732e2dea9b9d86017fed7e52e107f968fc5c5c1f925ef9e40b41a3bdbc.
- expected fetch bytes/count: 52352 bytes, 2 observed expected-fetch events.
- observed TFTP bytes/count: [104136, 104136], 0 byte-matching events.
- final pre-restore tree:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- restore tree:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- serial freshness: true; 37 marker occurrences and 0 pre-power nonce
  occurrences.

Positive fixture:
tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/control-run/v4-check.json.

- gate classification: selected-tree-identity-ready.
- exit code: 0.
- selected tree: db5f356f5ca4eed9ca21c232ea40a7ff6dfb2f7520337c08695ee0223129238d.
- expected fetch bytes/count: 50112 bytes, 2 observed expected-fetch events.
- observed TFTP bytes/count: [50112, 50112], 2 byte-matching events.
- final pre-restore tree:
  db5f356f5ca4eed9ca21c232ea40a7ff6dfb2f7520337c08695ee0223129238d.
- restore tree:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- serial freshness: true; 39 marker occurrences and 0 pre-power nonce
  occurrences.

## Evidence

- Gate script: scripts/rpi5-staging-identity-gate-v1-check.sh.
- Candidate gate output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/candidate-gate-output.json.
- Candidate gate exit:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/candidate-gate.exit.
- Control gate output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/control-gate-output.json.
- Control gate exit:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/control-gate.exit.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/evidence-map.json.

## Validation

- task-owned negative fixture replay:
  scripts/rpi5-staging-identity-gate-v1-check.sh --v4-check
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/candidate-run/v4-check.json
  --evidence-consistency-guard
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/evidence-consistency-guard-output.json
  --label candidate.
- task-owned positive fixture replay:
  scripts/rpi5-staging-identity-gate-v1-check.sh --v4-check
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/control-run/v4-check.json
  --evidence-consistency-guard
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/evidence-consistency-guard-output.json
  --label control.
- JSON validation: jq empty on task-owned gate output, classification, and
  evidence-map JSON.
- diff check: git diff --check.
- docs validation: not required; docs/src files were not touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Candidate fixture rejected for the same first failing v4 invariant:
  satisfied; tftp-expected-fetch-byte-mismatch.
- Control fixture accepted as selected-tree identity ready without broadening:
  satisfied.
- Gate JSON names required fields: satisfied.
- Future hardware criteria not weakened: satisfied; the register-vector retry
  remains blocked behind this closeout plus a serialized staging sentinel
  proof.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-closeout-20260611
on the next worker wake if dependencies remain satisfied. Do not run hardware
or promote register-vector v3 until the closeout and serialized staging
sentinel proof accept selected-tree identity durability.
