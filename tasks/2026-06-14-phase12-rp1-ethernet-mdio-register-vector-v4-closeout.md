# Phase 12 RP1 Ethernet MDIO Register Vector V4 Closeout

Task id: phase12-rp1-ethernet-mdio-register-vector-v4-closeout-20260614

Status: accepted

Classification: mdio-phy1-register-vector-visible-frontier-closed

Evidence level: static/task evidence inspection, capture-chain-v4 replay
review, staging identity gate review, evidence-consistency guard review,
lab-controller API evidence review, serial hardware boot/output evidence
review, stable same-cursor TFTP delta evidence review, and restore evidence
review.

## Goal

Close out the guarded v4 register-vector proof after served-root recovery
without broadening beyond the selected corrected-target Clause 22 PHY1
six-register boundary.

## Scope Performed

- Inspected the accepted v4 proof task record, classification JSON, capture
  summary, evidence map, candidate/control capture-chain-v4 JSON, staging
  identity gate outputs, and evidence-consistency guard output.
- Reconciled accepted and rejected claims against the source contract,
  quarantine boundary, and root-recovery evidence.
- Recorded the narrow accepted frontier and deferred risks.
- Updated Phase 12 project and roadmap docs with the closed frontier wording.
- Set supervisor planning as the next action because no explicit queued
  mechanically objective follow-up task exists after this closeout.

## Findings

- fixed: the v4 proof aggregate evidence and markdown agree with
  candidate/control capture-chain-v4 JSON, staging identity gate JSON, and the
  evidence-consistency guard.
- fixed: the candidate is accepted only as
  mdio-phy1-register-vector-visible for the selected Clause 22 PHY1 register
  vector: BMCR 0x1000, BMSR 0x7949, PHYSID1 0x600d, PHYSID2 0x84a2, ANAR
  0x01e1, and ANLPAR 0x0000.
- fixed: the paired control passed the same capture/reporting path while
  constructing no MDIO target, no MAN frame, and no runtime MDIO transaction.
- fixed: candidate/control proof identity remained decisive under
  capture-chain-v4 and boot-staging identity: selected-tree identity,
  same-power-cycle TFTP-served byte agreement, final pre-restore identity, and
  restore proof all matched.
- fixed: lab restore evidence remains baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: PHY reset/GPIO32 ownership, broad MDIO/PHY ownership, link state,
  Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicitly planned tasks.
- removed: no stale evidence, source code, task records, or helper scripts were
  removed.
- not-an-issue: no NCR write was performed during the accepted vector proof;
  corrected NCR.MPE was already set before MAN transactions.

## Reconciliation

The v4 proof task is
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery-20260614
at commit a4d203906e1b9ac5686342cff5c9ffb1bc1d909b.

Candidate capture-chain-v4 and boot-staging identity are both ready. The
candidate selected tree was
043744bcf578d7966c63600c3db0302e35e96ec631f6d535725c8e63002fd43d, TFTP
served two da591740/kernel_2712.img fetches at the expected 52,352 bytes, and
final pre-restore identity stayed on the selected tree. Serial freshness was
true with the run-unique nonce present after power and absent before power.
The accepted MAN.DATA vector is BMCR 0x1000, BMSR 0x7949, PHYSID1 0x600d,
PHYSID2 0x84a2, ANAR 0x01e1, and ANLPAR 0x0000.

Control capture-chain-v4 and boot-staging identity are both ready. The control
selected tree was
0a8aab5b6103bf42c28b7d202ef1022b94c443a08879641c89ef481c59e516a8, TFTP
served two da591740/kernel_2712.img fetches at the expected 50,112 bytes, and
final pre-restore identity stayed on the selected tree. The control proves
only the no-MDIO/no-Ethernet reporting path for this proof shape.

The proof task markdown, classification JSON, capture summary, evidence map,
candidate/control v4 outputs, candidate/control staging identity gates, and
evidence-consistency guard output all agree. The guard output classifies the
aggregate evidence as evidence-consistency-ready.

## Frontier

Closed frontier:
rp1-ethernet-mdio-register-vector-phy1-visible-frontier-closed.

Accepted: the selected corrected-target PHY1 Clause 22 six-register
register-vector MAN.DATA boundary under capture-chain-v4, boot-staging
identity, same-power-cycle TFTP byte agreement, final pre-restore identity,
serial freshness, and restore evidence.

Not accepted: PHY absence, PHY reset/GPIO32 ownership, broad MDIO/PHY
ownership, link state, Ethernet driver behavior, interrupts, DMA/descriptors,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Direction

No explicit queued mechanically objective follow-up task exists after this
closeout. Supervisor planning is required to select the next bounded Phase
12.1 task and its acceptance gates. Any follow-up must depend on this closed
frontier and must stay feature-led; this closeout does not authorize
networking, SSH, Phase 12.2, or a broad MDIO/PHY ownership claim.

## Evidence

- V4 proof task:
  tasks/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery.md.
- V4 proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/classification.json.
- V4 proof capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/capture-summary.json.
- V4 proof evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/evidence-map.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/control-run/v4-check.json.
- Candidate staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/candidate-run/staging-identity-gate-output.json.
- Control staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/control-run/staging-identity-gate-output.json.
- Evidence-consistency guard:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/evidence-consistency-guard-output.json.
- Closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-v4-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-v4-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: proof task, classification JSON, capture
  summary, evidence map, candidate/control v4 JSON, staging identity outputs,
  evidence-consistency guard output, Phase 12 docs, roadmap, and git history
  inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- V4 evidence reconciled without broadening beyond the selected
  register-vector contract: satisfied.
- Next task direction stated: satisfied; no explicit mechanically objective
  follow-up exists, so planningNeeded is set for supervisor planning.
- Inconclusive/blocker policy: not applicable; v4 proof was accepted, and
  same-shaped retry is not selected from this closeout.
- Closeout committed before any follow-up starts: satisfied by the closeout
  commit.

## Next Action

Set planningNeeded=true for supervisor planning of the next bounded Phase 12.1
task. Do not infer broad MDIO/PHY ownership, Ethernet behavior, packet I/O,
networking, SSH, Phase 12.2, or a phase transition from this closeout.
