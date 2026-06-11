# Phase 12 RP1 Ethernet MDIO Register Vector Proof V2 Closeout

Task id: phase12-rp1-ethernet-mdio-register-vector-proof-v2-closeout-20260611

Status: accepted

Classification: mdio-register-vector-guarded-v2-candidate-identity-mismatch-frontier-closed

Evidence level: static inspection, capture-chain-v4 replay review,
evidence-consistency guard/check review, lab-controller API evidence review,
serial hardware boot/output evidence review, stable TFTP delta evidence
review, and restore evidence review.

## Goal

Close out the guarded register-vector proof/blocker without expanding beyond
the selected corrected-target Clause 22 register-vector boundary.

## Scope Performed

- Inspected the guarded proof task record, candidate/control v4 JSON,
  evidence-consistency guard output, classification JSON, capture summary,
  evidence map, Phase 12 docs, roadmap, and commit history.
- Reconciled candidate/control v4 JSON with aggregate task-owned evidence and
  task markdown.
- Recorded same-shaped retry policy for the guarded register-vector proof
  shape.
- Updated Phase 12 project and roadmap docs with the closed frontier wording.

## Findings

- fixed: the guarded proof aggregate evidence and markdown agree with
  candidate/control v4 JSON and the evidence-consistency guard.
- fixed: the candidate is classified as a precise capture/staging blocker,
  not accepted register-vector hardware proof.
- fixed: the paired no-MDIO/no-Ethernet control passed capture-chain-v4 and
  proves only the reporting path for this proof shape.
- fixed: the lab restore evidence remains baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: a future retry requires a qualitatively different staging or
  power-cycle identity discriminator with explicit acceptance criteria.
- deferred: broad MDIO/PHY ownership, PHY absence, PHY reset/GPIO32 ownership,
  Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  work.
- removed: no stale evidence, source code, or task records were removed.
- not-an-issue: closing a precise hardware blocker is acceptable because the
  blocker evidence is committed, the evidence-consistency guard agrees with
  v4 JSON, and hardwareTestLock is unlocked/restored.

## Reconciliation

The guarded proof task is
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard-20260611
at commit a28a10b215af387cfd356733f5ee1f547565bb24.

Candidate v4 JSON is authoritative and classified the candidate as
capture-staging-blocked. The first failing invariant is
tftp-expected-fetch-byte-mismatch. The candidate selected tree was
dd4ad2732e2dea9b9d86017fed7e52e107f968fc5c5c1f925ef9e40b41a3bdbc, but both
TFTP fetches were 104136 bytes instead of the expected 52352 bytes and final
pre-restore identity was the restored baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Candidate
serial freshness was observed with 37 marker occurrences, but no
register-vector MAN.DATA values are accepted as hardware proof.

Control v4 JSON classified the paired control as capture-chain-v4-ready. It
had matching 50112-byte TFTP fetches, final pre-restore identity on selected
tree db5f356f5ca4eed9ca21c232ea40a7ff6dfb2f7520337c08695ee0223129238d, and
39 serial marker occurrences. This proves only the no-MDIO/no-Ethernet
reporting path for the proof shape.

The proof task markdown, classification JSON, capture summary, evidence map,
and evidence-consistency guard output all agree with those v4 results. The
guard output classifies the aggregate evidence as evidence-consistency-ready.

## Same-Shaped Retry Policy

Same-shaped guarded register-vector hardware retries are closed for this
candidate/control pair. Repeating the same proof shape after a candidate
TFTP/final-identity mismatch would not make the blocked candidate decisive or
prove register-vector visibility. Any future retry needs a qualitatively
different staging or power-cycle identity discriminator with explicit
acceptance criteria.

## Evidence

- Candidate v4 JSON:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/candidate-run/v4-check.json.
- Control v4 JSON:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/control-run/v4-check.json.
- Guard output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/evidence-consistency-guard-output.json.
- Guarded proof classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/classification.json.
- Guarded proof capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-proof-v2-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-proof-v2-closeout/evidence-map.json.

## Validation

- static inspection: guarded proof task record, candidate/control v4 JSON,
  guard output, capture/classification/evidence-map JSON, Phase 12 docs,
  roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Guarded proof/blocker reconciled without expanding selected register-vector
  boundary: satisfied.
- Aggregate evidence and task markdown agree with candidate/control v4 JSON
  and guard output: satisfied.
- Same-shaped retry policy is explicit: satisfied; same-shaped retries are
  closed until a qualitatively different staging/power-cycle identity
  discriminator exists.
- NextAction selects a bounded follow-up only if mechanically objective:
  satisfied; no existing follow-up is mechanically objective, so supervisor
  planning is required.
- Accepted checkpoint committed before follow-up starts: satisfied by the
  closeout commit.

## Next Action

Set planningNeeded=true for supervisor planning of the next bounded Phase 12.1
discriminator or a qualitatively different staging/power-cycle identity gate.
Do not infer broad MDIO/PHY ownership, Ethernet behavior, networking, SSH,
Phase 12.2, or a phase transition from this blocker.
