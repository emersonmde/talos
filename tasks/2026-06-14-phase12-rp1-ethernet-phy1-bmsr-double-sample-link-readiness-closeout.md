# Phase 12 RP1 Ethernet PHY1 BMSR Double-Sample Link Readiness Closeout

Task id: phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout-20260614

Status: accepted

Classification: rp1-ethernet-phy1-bmsr-double-sample-link-readiness-frontier-closed

Evidence level: static/task evidence inspection, capture-chain-v4 replay
review, boot-staging identity gate review, evidence-consistency guard review,
lab-controller API evidence review, serial hardware boot/output evidence
review, stable same-cursor TFTP delta evidence review, and restore evidence
review.

## Goal

Close out the accepted read-only corrected-target PHY1 BMCR plus
double-sampled BMSR link-readiness proof without broadening beyond the selected
register-state discriminator.

## Scope Performed

- Inspected the accepted BMSR double-sample proof task record, classification
  JSON, capture summary, evidence map, candidate/control capture-chain-v4 JSON,
  boot-staging identity outputs, evidence-consistency guard, serial windows,
  TFTP deltas, final identity evidence, and restore proof.
- Reconciled the accepted link-not-ready result against the source contract and
  the prior PHY1 status diagnostic frontier.
- Recorded the closed link-readiness discriminator frontier and retained
  deferred risks.
- Updated Phase 12 project and roadmap docs with the closed frontier wording.
- Set supervisor planning as the next action because no explicit queued
  mechanically objective follow-up task exists after this closeout.

## Findings

- fixed: the proof task record, classification JSON, capture summary,
  evidence map, candidate/control capture-chain-v4 JSON, boot-staging identity
  outputs, and evidence-consistency guard agree on decisive candidate/control
  evidence.
- fixed: the candidate is accepted only as
  mdio-phy1-bmsr-double-sample-link-not-ready for the selected read-only
  corrected-target PHY1 BMCR plus double-sampled BMSR discriminator.
- fixed: the candidate reported BMCR 0x1000, first BMSR 0x7949, and second
  BMSR 0x7949. BMCR reset, loopback, and autoneg-restart were false;
  second-sample BMSR_LSTATUS and BMSR_ANEGCOMPLETE were false.
- fixed: the paired control used the same reporting surface while constructing
  no MDIO target, no MAN frame, and no volatile Ethernet MDIO load/store.
- fixed: candidate/control proof identity remained decisive under
  capture-chain-v4 and boot-staging identity: selected-tree identity,
  same-power-cycle TFTP-served byte agreement, final pre-restore identity,
  serial freshness, evidence-consistency-ready, and restore proof all matched.
- fixed: lab restore evidence remains baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: PHY configuration writes, PHY reset/GPIO32 action,
  autonegotiation restart, link forcing, MACB NSR_LINK proof, broad MDIO/PHY
  ownership, Ethernet driver behavior, interrupts, DMA/descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  future explicitly planned tasks.
- removed: no stale evidence, source code, task records, or helper scripts were
  removed.
- not-an-issue: the accepted candidate performed only the selected
  corrected-target Clause 22 MAN read sequence, BMCR 0x60820000, BMSR first
  0x60860000, and BMSR second 0x60860000, with bounded NSR.IDLE polling and no
  NCR write.

## Reconciliation

The BMSR double-sample proof task is
phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof-20260614
at commit 1ae90c65565b3dc19dadf7eccf2cedf448caa4d1.

Candidate capture-chain-v4 and boot-staging identity are both ready. The
candidate selected tree was
83efecf2cbf7135492907335fb4a00a54c7374768b3d4a87c774721f49c2d94a, TFTP
served two da591740/kernel_2712.img fetches at the expected 52,536 bytes, and
final pre-restore identity stayed on the selected tree. Serial freshness was
true with the run-unique nonce present after power and absent before power.
The accepted read-only link-readiness discriminator is BMCR 0x1000, first BMSR
0x7949, and second BMSR 0x7949. BMCR reset, loopback, and autoneg-restart are
false. Second-sample BMSR_LSTATUS and BMSR_ANEGCOMPLETE are false, so the
accepted classification is link-not-ready.

Control capture-chain-v4 and boot-staging identity are both ready. The control
selected tree was
52c81ff4b8249df118da553e00c528eeaa83b25ab5a44ffb5468675fec31f749, TFTP
served two da591740/kernel_2712.img fetches at the expected 50,112 bytes, and
final pre-restore identity stayed on the selected tree. The control proves only
the no-MDIO/no-Ethernet reporting path for this proof shape.

The proof task markdown, classification JSON, capture summary, evidence map,
candidate/control v4 outputs, candidate/control boot-staging identity gates,
serial windows, TFTP deltas, final pre-restore identity, restore evidence, and
evidence-consistency guard agree on the narrow link-not-ready classification.

## Frontier

Closed frontier:
rp1-ethernet-phy1-bmsr-double-sample-link-readiness-frontier-closed.

Accepted: read-only corrected-target PHY1 BMCR plus double-sampled BMSR
link-readiness discriminator under capture-chain-v4, boot-staging identity,
same-power-cycle TFTP byte agreement, final pre-restore identity, serial
freshness, evidence-consistency-ready, and restore evidence. The observed
register-state result is link-not-ready.

Not accepted: PHY configuration writes, PHY reset/GPIO32 action,
autonegotiation restart, link forcing, MACB NSR_LINK proof, broad MDIO/PHY
ownership, Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Direction

No explicit queued mechanically objective follow-up task exists after this
closeout. Supervisor planning is required to select the next bounded Phase
12.1 task and its acceptance gates. Any follow-up must depend on this closed
frontier and must stay feature-led; this closeout does not authorize PHY
configuration, PHY reset/GPIO32 action, broad MDIO/PHY ownership, Ethernet
behavior, packet I/O, networking, SSH, Phase 12.2, or a phase transition.

## Evidence

- BMSR double-sample proof task:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof.md.
- BMSR double-sample proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/classification.json.
- BMSR double-sample proof capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/capture-summary.json.
- BMSR double-sample proof evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/evidence-map.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/control-run/v4-check.json.
- Candidate boot-staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/candidate-run/boot-staging-identity.json.
- Control boot-staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/control-run/boot-staging-identity.json.
- Evidence-consistency guard:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/evidence-consistency-guard.json.
- Final restored lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/final-lab-status.json.
- Closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: proof task, classification JSON, capture
  summary, evidence map, candidate/control v4 JSON, boot-staging identity
  outputs, evidence-consistency guard, serial windows, TFTP deltas, final
  identity, restore evidence, Phase 12 docs, roadmap, and git history
  inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- BMCR/BMSR samples reconciled without broadening beyond the selected
  read-only link-readiness discriminator: satisfied.
- Accepted proof stated exactly what was proven and what remains unaccepted:
  satisfied.
- Inconclusive/blocker policy: not applicable; the proof was accepted as
  link-not-ready, and same-shaped retry is not selected from this closeout.
- Closeout committed before any follow-up starts: satisfied by the closeout
  commit.
- No objective next task exists: satisfied; planningNeeded is set for
  supervisor planning of the next bounded Phase 12.1 task.

## Next Action

Set planningNeeded=true for supervisor planning of the next bounded Phase 12.1
task. Do not infer PHY configuration, PHY reset/GPIO32 action, broad MDIO/PHY
ownership, Ethernet behavior, packet I/O, networking, SSH, Phase 12.2, or a
phase transition from this closeout.
