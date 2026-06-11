# Phase 12 RP1 Ethernet MDIO Register Vector Evidence Consistency Guard Closeout

Task id: phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-closeout-20260611

Status: accepted

Classification: mdio-register-vector-evidence-consistency-guard-closeout-accepted

Evidence level: static inspection of accepted guard task record, guard
implementation, task-owned JSON evidence, retained negative/positive guard
outputs, and validation logs.

## Goal

Close out the evidence-consistency guard and decide whether a guarded
register-vector Pi 5 retry is mechanically objective.

## Scope Performed

- Inspected the accepted guard core task record and committed guard
  implementation.
- Inspected the retained negative contradictory register-vector guard output.
- Inspected the retained positive capture-chain-v4 proof guard output.
- Reconciled the guard invariant against future Pi 5 proof acceptance
  requirements.
- Selected only the already queued guarded Pi 5 retry because the guard makes
  the previous aggregate/v4 disagreement mechanically rejectable.

No hardware was run, no boot archive was published, hardwareTestLock was not
acquired, and no runtime Ethernet, MDIO, PHY, MMIO, DMA, descriptor-ring,
interrupt, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition work was performed.

## Findings

- fixed: the guard invariant is now closed out as a Phase 12.1 proof boundary.
  Candidate/control capture-chain-v4 JSON is authoritative for capture
  readiness, and aggregate classification, capture-summary, evidence-map, and
  task markdown must not overclaim readiness when v4 JSON blocks it.
- fixed: the contradictory register-vector retry remains blocked. The retained
  negative output classifies evidence-consistency-blocked with
  aggregate-claims-candidate-ready-overclaim and
  task-markdown-candidate-ready-overclaim because candidate v4 JSON is
  capture-staging-blocked.
- fixed: the positive retained GEM MID decode discriminator V2 proof passes the
  guard as evidence-consistency-ready, so the guard is not hardcoded to the
  failing register-vector evidence shape.
- fixed: the next bounded task is mechanically objective only as the guarded
  Pi 5 retry, phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard-20260611,
  with guard output required to agree with candidate/control v4 JSON before
  any hardware proof or blocker can be accepted.
- deferred: hardware retry, MDIO register-vector proof, PHY ownership,
  Ethernet behavior, interrupts, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain outside this closeout.
- removed: no stale source, docs, or evidence was removed.
- not-an-issue: docs/src frontier wording did not require an update because
  this closeout only accepts the validation guard boundary, not new hardware
  register-vector evidence.

## Guard Invariant

The invariant now enforced for Pi 5 proof acceptance is:

~~~text
candidate/control capture-chain-v4 JSON is authoritative.
If candidate or control v4 JSON is blocked, task markdown, aggregate
classification JSON, capture-summary JSON, and evidence-map accepted claims
must not claim capture-chain-v4 readiness, selected-tree identity, matching
TFTP bytes, run-unique serial freshness, final pre-restore identity, or
decisive_rp1_hardware_classification_allowed=true for that side.
~~~

The guard must run over the task-owned evidence directory and task markdown
before accepting future Pi 5 proof evidence. A nonzero guard result is a
proof blocker until the disagreement is corrected or supervisor planning
changes the acceptance criteria.

## Evidence Reconciliation

The retained negative control is the prior post-recovery register-vector retry:

~~~text
classification=evidence-consistency-blocked
consistent=false
candidate_v4_classification=capture-staging-blocked
candidate_v4_allowed=false
control_v4_classification=capture-chain-v4-ready
control_v4_allowed=true
rejection_reasons=[
  aggregate-claims-candidate-ready-overclaim,
  task-markdown-candidate-ready-overclaim
]
~~~

That result makes the previous overclaim mechanically rejectable. Same-shaped
unguarded register-vector retries remain closed, and the prior serial register
values remain unaccepted as hardware proof.

The retained positive control is the accepted GEM MID decode discriminator V2
Pi 5 proof:

~~~text
classification=evidence-consistency-ready
consistent=true
candidate_v4_classification=capture-chain-v4-ready
candidate_v4_allowed=true
control_v4_classification=capture-chain-v4-ready
control_v4_allowed=true
rejection_reasons=[]
~~~

That result proves the guard accepts a valid retained capture-chain-v4 evidence
shape and can be used as an acceptance gate for the guarded retry.

## Same-Shaped Retry Policy

Same-shaped unguarded register-vector retries remain closed. The only selected
follow-up is the already queued guarded Pi 5 retry, which must retain
candidate/control v4 JSON, aggregate classification, capture summary, evidence
map, guard output, task markdown, serial/TFTP/final identity evidence, and
restore evidence that agree before accepting either a hardware proof or a
precise blocker.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-closeout/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-closeout/evidence-map.json.
- Inspected guard core task record:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core.md.
- Inspected guard implementation:
  scripts/rpi5-evidence-consistency-guard.sh.
- Inspected negative guard output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core/negative-contradictory-register-vector-guard-output.json.
- Inspected positive guard output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core/positive-retained-capture-chain-v4-guard-output.json.

## Validation

- static inspection: guard core task record, guard implementation,
  classification JSON, evidence-map JSON, negative/positive guard outputs, and
  validation logs.
- JSON validation: jq empty on task-owned classification and evidence-map JSON.
- diff check: git diff --check.
- docs validation: not required; no docs/src files touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Closeout reconciles the guard invariant, negative contradictory evidence,
  positive retained evidence, and validation results: satisfied.
- NextAction selects the guarded Pi 5 retry only if the guard makes the
  previous overclaim mechanically rejectable: satisfied.
- Same-shaped unguarded register-vector retries remain closed: satisfied.
- Accepted closeout is committed before any hardware retry starts: satisfied by
  this task commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard-20260611
on the next worker wake if hardwareTestLock remains unlocked/restored and
supervisorIntervention remains inactive. Do not run hardware from this
closeout, do not repeat an unguarded register-vector proof, do not broaden
MDIO/PHY ownership, and do not start Ethernet behavior, networking, SSH, Phase
12.2, or a phase transition.
