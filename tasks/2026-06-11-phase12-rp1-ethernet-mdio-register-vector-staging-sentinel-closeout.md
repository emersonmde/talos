# Phase 12 RP1 Ethernet MDIO Register Vector Staging Sentinel Closeout

Task id:
phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout-20260611

Status: accepted

Classification: selected-tree-identity-durability-accepted

Evidence level: static inspection, capture-chain-v4 replay review, staging
identity gate review, lab-controller API evidence review, serial hardware
boot/output evidence review, stable TFTP delta evidence review, and restore
evidence review.

## Goal

Close out the staging sentinel proof and decide whether a guarded
register-vector v3 proof is mechanically unblocked.

## Scope Performed

- Inspected the accepted staging sentinel proof task record.
- Inspected candidate/control archive reviews, capture-chain-v4 JSON, staging
  identity gate output, capture summary, classification JSON, evidence map,
  final lab restore evidence, Phase 12 docs, roadmap, and git history.
- Reconciled the candidate/control sentinel evidence without expanding beyond
  selected-tree identity durability.
- Selected the queued register-vector v3 retry because the sentinel proof
  accepted selected-tree identity durability for both candidate and control.

## Findings

- fixed: the accepted sentinel proof established selected-tree identity
  durability for the no-MDIO/no-Ethernet candidate-shaped archive and paired
  control through publication, power, TFTP, final pre-restore identity, and
  restore proof.
- fixed: candidate capture-chain-v4 and staging identity gate both returned
  ready with selected tree
  a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0, two
  matching 47,832-byte TFTP fetches, final pre-restore identity on the
  selected tree, baseline restore proof, and true serial freshness.
- fixed: control capture-chain-v4 and staging identity gate both returned
  ready with selected tree
  9d9b3cdb7b1f230d9cd2bf0c04b7c32dd98b53dd8ec7de77e99860c5b231908d, two
  matching 47,824-byte TFTP fetches, final pre-restore identity on the
  selected tree, baseline restore proof, and true serial freshness.
- fixed: the final lab state after the sentinel proof is restored to baseline
  tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  kernel_2712.img at 104,136 bytes and hardwareTestLock released/restored.
- deferred: the guarded register-vector v3 retry remains a separate serialized
  hardware proof and must still pass staging identity gate and
  evidence-consistency guard output before any register-vector result can be
  accepted.
- deferred: broad MDIO/PHY ownership, PHY absence, PHY reset/GPIO32 ownership,
  Ethernet behavior, interrupts, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future work.
- removed: no stale source, docs, or evidence was removed.
- not-an-issue: the sentinel candidate/control did not perform MDIO, NCR, MAN,
  GPIO32/PHY reset, Ethernet, DMA, interrupt, packet I/O, networking, sockets,
  SSH, Phase 12.2, or phase-transition work, so its accepted frontier is
  limited to staging identity durability.

## Reconciliation

The accepted sentinel proof task is
phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof-20260611
at commit 73650da806da726fbccb1b1b4b5989c3a4a0a7e0.

Candidate capture-chain-v4 and staging identity gate output both classify the
candidate as ready. The selected tree was
a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0, the
expected fetch was da591740/kernel_2712.img at 47,832 bytes, both observed
TFTP fetches matched that byte count, final pre-restore identity stayed on the
selected tree, restore returned to the baseline tree, and serial freshness was
true with the run-unique nonce absent before power and present after power.

Control capture-chain-v4 and staging identity gate output both classify the
control as ready. The selected tree was
9d9b3cdb7b1f230d9cd2bf0c04b7c32dd98b53dd8ec7de77e99860c5b231908d, the
expected fetch was da591740/kernel_2712.img at 47,824 bytes, both observed
TFTP fetches matched that byte count, final pre-restore identity stayed on the
selected tree, restore returned to the baseline tree, and serial freshness was
true with the run-unique nonce absent before power and present after power.

The proof task markdown, classification JSON, capture summary, evidence map,
candidate/control v4 JSON, candidate/control staging identity gate output, and
final lab status agree. Selected-tree identity durability is accepted for this
no-MDIO/no-Ethernet staging sentinel only.

## Boundary

Accepted: selected-tree identity durability for the no-MDIO/no-Ethernet
candidate-shaped staging sentinel and paired control.

Not accepted: RP1 MDIO register-vector MAN.DATA values, NCR writes, MAN
writes, GPIO32/PHY reset, broad MDIO/PHY ownership, PHY absence, Ethernet
behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## Next Selection

The queued
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity-20260611
task is mechanically unblocked by this closeout only because the sentinel proof
accepted selected-tree identity durability for both candidate and control.

That v3 proof remains bounded by its own acceptance criteria: it must acquire
hardwareTestLock, preserve candidate/control identity, run the staging identity
gate, run the evidence-consistency guard, capture serial/TFTP/final
identity/restore evidence, and reject broad MDIO/PHY ownership, PHY absence,
Ethernet behavior, networking, SSH, Phase 12.2, and phase transition claims.

## Evidence

- Sentinel proof task:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof.md.
- Sentinel proof commit:
  73650da806da726fbccb1b1b4b5989c3a4a0a7e0.
- Candidate v4 JSON:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/candidate-run/v4-check.json.
- Control v4 JSON:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/control-run/v4-check.json.
- Candidate staging identity gate:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/candidate-run/staging-identity-gate-output.json.
- Control staging identity gate:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/control-run/staging-identity-gate-output.json.
- Sentinel classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/classification.json.
- Sentinel capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/capture-summary.json.
- Sentinel evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/evidence-map.json.
- Final lab status:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/final-lab-status.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout/evidence-map.json.

## Validation

- static inspection: sentinel proof task record, candidate/control gate output,
  candidate/control v4 JSON, capture/classification/evidence-map JSON, Phase
  12 docs, roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Closeout reconciles candidate/control sentinel evidence without expanding
  beyond staging identity durability: satisfied.
- Because selected-tree identity durability is accepted, nextAction explicitly
  selects
  phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity-20260611:
  satisfied.
- Staging is not blocked or inconclusive, so planningNeeded does not need to
  be set by this task: satisfied.
- Accepted checkpoint committed before follow-up starts: satisfied by the
  closeout commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity-20260611
on the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, and supervisorIntervention remains inactive. Do not start
broad MDIO/PHY ownership, Ethernet behavior, networking, SSH, Phase 12.2, or a
phase transition from this closeout.
