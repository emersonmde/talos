# Phase 12 RP1 Ethernet Capture-Staging Recovery Closeout

Task id: phase12-rp1-ethernet-capture-staging-recovery-closeout-20260614

Status: accepted

Classification: capture-staging-recovery-closeout-accepted

Evidence level: static/task evidence inspection, task-owned JSON evidence,
retained lab-controller API evidence review, retained serial hardware
boot/output evidence review, retained stable same-cursor TFTP delta evidence
review, retained capture-chain-v4 replay review, retained staging identity gate
review, retained restore proof review, and docs validation.

## Goal

Close the capture-staging recovery loop after the minimal sentinel proof by
recording whether selected-tree/TFTP/final-identity freshness is recovered, and
by preserving the boundary before any future runtime Ethernet retry.

## Scope Performed

- Inspected the accepted capture-staging recurrence checkpoint.
- Inspected the accepted minimal sentinel proof, classification JSON, evidence
  map, candidate/control capture-chain-v4 outputs, staging identity gates,
  stable TFTP deltas, serial windows, final lab status, and restore evidence.
- Reconciled the recovered capture-staging path against the previously blocked
  autoneg-restart proof boundary.
- Updated the Phase 12 docs and roadmap with the closed recovery frontier.
- Did not run hardware, publish a boot archive, mutate the lab, or start any
  autonegotiation, MDIO, MACB, GPIO32, packet, networking, SSH, Phase 12.2, or
  phase-transition work.

## Findings

- fixed: current capture-staging selected-tree/TFTP/final-identity freshness is
  recovered for the accepted minimal no-MDIO/no-Ethernet candidate/control
  pair.
- fixed: the accepted control selected tree
  9e3442962d40beff3b31668065df4f5d8ac37ee770fa3c4bba725d049bc78db3 had two
  matching 47832-byte same-power-cycle TFTP fetches, fresh serial marker
  evidence, final pre-restore selected-tree identity, and restore evidence.
- fixed: the accepted candidate rerun selected tree
  520785f412ba93da8c25577e5f5e4635ffba02b2969fbf3e02a346e97e061799 had two
  matching 47848-byte same-power-cycle TFTP fetches, fresh serial marker
  evidence, final pre-restore selected-tree identity, and restore evidence.
- fixed: the confounded original candidate run remains quarantined from
  acceptance; it does not weaken the accepted control plus candidate-rerun
  evidence because the task retained the confound and reran only after control
  triage.
- deferred: no future autoneg-restart retry or runtime Ethernet proof is
  promoted by this closeout because no explicit queued follow-up task exists.
- removed: no source, helper, task, evidence, or documentation files were
  removed.
- not-an-issue: the accepted minimal sentinel proof does not need to prove
  runtime Ethernet behavior; its boundary is capture-staging freshness only.

## Result

~~~text
classification=capture-staging-recovery-closeout-accepted
capture_staging_status=recovered-for-minimal-sentinel
minimal_sentinel_classification=capture-staging-minimal-sentinel-proof-accepted
selected_next_task=null
planning_needed=true
planning_reason=no-explicit-queued-follow-up-after-capture-staging-recovery-closeout
~~~

## Boundary

Accepted: capture-staging selected-tree/TFTP/final-identity freshness is
recovered for one minimal no-MDIO/no-Ethernet candidate/control sentinel pair.

Not accepted: autonegotiation restart, runtime BMCR writes, MDIO register
vectors, MACB_NSR reads, GPIO32/PHY reset ownership, Ethernet link readiness,
packet I/O, DMA/descriptors, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Evidence

- Closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-recovery-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-recovery-closeout/evidence-map.json.
- Recurrence checkpoint:
  tasks/2026-06-14-phase12-rp1-ethernet-capture-staging-recurrence-checkpoint.md.
- Minimal sentinel proof:
  tasks/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof.md.
- Minimal sentinel classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/classification.json.
- Minimal sentinel evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/evidence-map.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run/v4-check.json.
- Candidate staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run/staging-identity-gate-output.json.
- Candidate stable TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run/tftp-delta-stable-pre-restore.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/control-run/v4-check.json.
- Control staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/control-run/staging-identity-gate-output.json.
- Control stable TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/control-run/tftp-delta-stable-pre-restore.json.
- Final lab status before release:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/final-lab-status-before-release.json.

## Validation

- static/task evidence inspection: recurrence checkpoint, minimal sentinel
  proof, task-owned classification/evidence-map JSON, candidate/control
  capture-chain-v4 outputs, staging identity gates, TFTP deltas, serial
  windows, final lab status, restore evidence, Phase 12 docs, and roadmap were
  inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Records whether capture-staging selected-tree/TFTP/final-identity freshness
  is recovered, still blocked, or partially proven: satisfied, recovered for
  the minimal sentinel boundary.
- Preserves rejected claims for autoneg runtime evidence, link readiness,
  GPIO32/PHY reset ownership, packet I/O, networking, SSH, Phase 12.2, and
  phase transition: satisfied.
- Selects at most one objective follow-up task id for supervisor planning, or
  sets planningNeeded with a precise blocker reason if no safe objective
  follow-up exists: satisfied, planningNeeded is set because no explicit queued
  follow-up exists after this closeout.
- Accepted closeout committed before any follow-up task starts: satisfied by
  the commit for this task.

## Next Action

Supervisor planning is required before any fresh autoneg-restart retry,
capture-layer recovery, paired-control hardware run, PHY configuration,
GPIO32/PHY reset action, packet I/O, networking, SSH, Phase 12.2, or phase
transition.
