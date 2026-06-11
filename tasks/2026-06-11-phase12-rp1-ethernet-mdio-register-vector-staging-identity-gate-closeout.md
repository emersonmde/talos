# Phase 12 RP1 Ethernet MDIO Register Vector Staging Identity Gate Closeout

Task id:
phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-closeout-20260611

Status: accepted

Classification: staging-identity-gate-closeout-accepted

Evidence level: static inspection, task-owned fixture replay review, jq JSON
validation, and diff checks.

## Goal

Close out the local/static staging identity gate without expanding beyond
staging identity durability, and select the serialized staging sentinel proof
only if the gate has both negative and positive fixture evidence.

## Scope Performed

- Inspected the accepted staging identity gate core task record, candidate and
  control gate outputs, classification JSON, evidence map, gate script, and
  commit history.
- Reconciled the gate evidence against the accepted guarded v2 candidate
  mismatch and paired no-MDIO/no-Ethernet control fixtures.
- Recorded that same-shaped register-vector hardware retries remain closed
  until a serialized staging sentinel proof accepts selected-tree identity
  durability.
- Selected only the queued staging sentinel Pi 5 proof as the next bounded
  task, preserving the no-MDIO/no-Ethernet boundary.

## Findings

- fixed: the gate core is accepted and committed at
  da183c68dee4deea773ec7c0285fea351610fb5f.
- fixed: the negative fixture evidence rejects the guarded v2 candidate on the
  same first capture-chain-v4 failure,
  tftp-expected-fetch-byte-mismatch, mapped to
  expected-fetch-byte-mismatch.
- fixed: the positive fixture evidence accepts the paired control as
  selected-tree-identity-ready with matching selected tree, expected TFTP
  bytes, final pre-restore identity, restore proof, and serial freshness.
- fixed: closeout selects
  phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof-20260611
  because both required fixture replays exist and are JSON-valid.
- deferred: actual publish-to-power-to-TFTP/final selected-tree durability for
  a fresh sentinel remains a serialized hardware proof.
- deferred: register-vector MAN.DATA values, broad MDIO/PHY ownership, PHY
  absence, PHY reset/GPIO32 ownership, Ethernet behavior, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future explicit work.
- removed: no stale evidence, source code, docs, or task records were removed.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  local/static checkpoint over already committed gate evidence.

## Reconciliation

The staging identity gate core accepts only local/static replay of retained
capture-chain-v4 evidence. It does not inspect or accept register-vector
MAN.DATA values.

The candidate fixture is
tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/candidate-run/v4-check.json.
Gate output classified it as selected-tree-identity-blocked. The selected tree
was dd4ad2732e2dea9b9d86017fed7e52e107f968fc5c5c1f925ef9e40b41a3bdbc with
expected 52352-byte fetches, but observed TFTP fetches were 104136 bytes and
final pre-restore identity was the restored baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The first
failing invariant is expected-fetch-byte-mismatch. Candidate serial freshness
was true, so freshness alone is not enough to make the blocked candidate
decisive.

The control fixture is
tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/control-run/v4-check.json.
Gate output classified it as selected-tree-identity-ready. The selected tree
was db5f356f5ca4eed9ca21c232ea40a7ff6dfb2f7520337c08695ee0223129238d with
matching 50112-byte TFTP fetches, final pre-restore identity on the selected
tree, restore proof to the baseline tree, and true serial freshness. This
accepts only the reporting path and staging identity gate behavior for the
paired no-MDIO/no-Ethernet control.

## Same-Shaped Retry Policy

Same-shaped register-vector hardware retries remain closed. The accepted gate
proves that retained evidence can reject the prior identity mismatch and
accept the paired reporting-path control, but it does not prove that a newly
published candidate tree survives the power/TFTP/final-status path.

The next bounded task is the serialized staging sentinel proof. It must acquire
hardwareTestLock, publish candidate/control sentinel archives, run the staging
identity gate over candidate/control evidence, capture fresh serial output,
TFTP deltas, final pre-restore identity, restore proof, classification JSON,
capture summary, and evidence map, and commit either accepted selected-tree
identity durability or a precise first-failing invariant.

## Evidence

- Accepted gate core task:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core.md.
- Gate script:
  scripts/rpi5-staging-identity-gate-v1-check.sh.
- Candidate gate output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/candidate-gate-output.json.
- Control gate output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/control-gate-output.json.
- Gate core classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/classification.json.
- Gate core evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-closeout/evidence-map.json.

## Validation

- static inspection: gate task record, candidate/control gate outputs,
  classification/evidence-map JSON, gate script, and git history reviewed.
- JSON validation: jq empty on task-owned closeout classification and
  evidence-map JSON.
- diff check: git diff --check.
- docs validation: not required; docs/src files were not touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Closeout reconciles gate evidence without expanding beyond staging identity
  durability: satisfied.
- Closeout selects the staging sentinel proof only because both negative and
  positive fixture evidence exist: satisfied.
- Same-shaped register-vector retries remain closed unless the sentinel later
  accepts selected-tree identity durability: satisfied.
- Accepted checkpoint committed before hardware follow-up starts: satisfied by
  the closeout commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof-20260611
on the next worker wake if hardwareTestLock remains unlocked/restored and
supervisorIntervention remains inactive. Do not promote register-vector v3,
broad MDIO/PHY ownership, Ethernet behavior, networking, SSH, Phase 12.2, or a
phase transition unless the sentinel proof first accepts selected-tree identity
durability.
