# Phase 12 RP1 Ethernet PHY1 Status Diagnostic Closeout

Task id: phase12-rp1-ethernet-phy1-status-diagnostic-closeout-20260614

Status: accepted

Classification: rp1-ethernet-phy1-status-diagnostic-frontier-closed

Evidence level: static/task evidence inspection, capture-chain-v4 replay
review, boot-staging identity gate review, lab-controller API evidence review,
serial hardware boot/output evidence review, stable same-cursor TFTP delta
evidence review, and restore evidence review.

## Goal

Close out the accepted PHY1 status diagnostic without broadening beyond the
visible corrected-target Clause 22 status frontier.

## Scope Performed

- Inspected the accepted PHY1 status diagnostic task record, classification
  JSON, evidence map, candidate/control capture-chain-v4 JSON, boot-staging
  identity gate outputs, serial windows, TFTP deltas, final identity evidence,
  and restore proof.
- Reconciled the accepted raw and decoded PHY1 status evidence against the
  prior v4 register-vector frontier.
- Recorded the closed diagnostic frontier, deferred risks, and the next
  bounded task direction.
- Updated Phase 12 project and roadmap docs with the closed frontier wording.
- Selected only the already queued link-readiness source-contract task as the
  next mechanically dependency-gated follow-up.

## Findings

- fixed: the diagnostic task record, classification JSON, and evidence map
  agree on the accepted candidate/control capture chain, selected-tree
  identity, same-power-cycle TFTP byte agreement, final pre-restore identity,
  serial freshness, and restore proof.
- fixed: the accepted candidate is limited to
  mdio-phy1-status-diagnostic-visible for the corrected-target PHY1 Clause 22
  status vector: BMCR 0x1000, BMSR 0x7949, PHYSID1 0x600d, PHYSID2 0x84a2,
  ANAR 0x01e1, and ANLPAR 0x0000.
- fixed: decoded status is retained only as register-state visibility: BMCR
  reset=false, loopback=false, speed=10M, autoneg-enable=true; BMSR
  link-status=false, autoneg-complete=false, autoneg-ability=true; PHY ID OUI
  0x180361, model 0x0a, revision 0x02; ANAR advertises 10/100 half/full; and
  ANLPAR is empty.
- fixed: the paired control used the same reporting surface while constructing
  no MDIO target, no MAN frame, and no runtime MDIO transaction.
- fixed: lab restore evidence remains baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: link readiness, link usability, PHY configuration writes,
  autonegotiation restart, link forcing, PHY reset/GPIO32 ownership, broad
  MDIO/PHY ownership, Ethernet driver behavior, interrupts, DMA/descriptors,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  future explicitly planned tasks.
- removed: no stale evidence, source code, task records, or helper scripts were
  removed.
- not-an-issue: the accepted diagnostic used only the corrected-target MDIO
  read boundary; it performed no NCR write, PHY configuration write,
  GPIO32/PHY reset action, autonegotiation restart, link forcing, packet I/O,
  DMA, interrupt, socket, SSH, or Phase 12.2 work.

## Reconciliation

The diagnostic proof task is
phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof-20260614 at commit
c9eafe7df7f269c8c46120322c1981caa144031d.

Candidate capture-chain-v4 and boot-staging identity are both ready. The
candidate selected tree was
39eeabec22164e31bb0290f05b4985fc6392d38a8703ca6725693621739b84b6, TFTP
served two da591740/kernel_2712.img fetches at the expected 54,008 bytes, and
final pre-restore identity stayed on the selected tree. Serial freshness was
true with the run-unique nonce present after power and absent before power.
The accepted diagnostic vector is BMCR 0x1000, BMSR 0x7949, PHYSID1 0x600d,
PHYSID2 0x84a2, ANAR 0x01e1, and ANLPAR 0x0000, decoded only as the visible
PHY1 register state listed in the proof.

Control capture-chain-v4 and boot-staging identity are both ready. The control
selected tree was
3afdd601766c459afd88c33eb92b716bc797c0c51fbba8744efe8a985799d16d, TFTP
served two da591740/kernel_2712.img fetches at the expected 49,736 bytes, and
final pre-restore identity stayed on the selected tree. The control proves only
the no-MDIO/no-Ethernet reporting path for this diagnostic shape.

The accepted proof task markdown, classification JSON, evidence map,
candidate/control v4 outputs, candidate/control boot-staging identity gates,
serial windows, TFTP deltas, final pre-restore identity, and restore evidence
agree on the narrow diagnostic classification.

## Frontier

Closed frontier:
rp1-ethernet-phy1-status-diagnostic-frontier-closed.

Accepted: visible corrected-target PHY1 Clause 22 status diagnostic values and
decoded register-state fields under capture-chain-v4, boot-staging identity,
same-power-cycle TFTP byte agreement, final pre-restore identity, serial
freshness, and restore evidence.

Not accepted: link readiness, link usability, PHY configuration writes,
autonegotiation restart, link forcing, PHY reset/GPIO32 ownership, broad
MDIO/PHY ownership, Ethernet driver behavior, interrupts, DMA/descriptors,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Direction

The next bounded task is
phase12-rp1-ethernet-phy1-link-readiness-source-contract-20260614. It is a
source/task evidence contract only: it must name one exact next discriminator
that follows from the accepted PHY1 status frontier and must preserve explicit
allowed/forbidden operations and evidence gates before any implementation or
hardware proof.

This closeout does not authorize broad MDIO/PHY ownership, PHY configuration,
reset/GPIO32 action, packet I/O, networking, SSH, Phase 12.2, or a phase
transition.

## Evidence

- Diagnostic proof task:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof.md.
- Diagnostic proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/classification.json.
- Diagnostic proof evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/evidence-map.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/control-run/v4-check.json.
- Candidate boot-staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/candidate-run/staging-identity-gate-output.json.
- Control boot-staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/control-run/staging-identity-gate-output.json.
- Candidate serial hardware output:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/candidate-run/serial-observe-window.json.
- Control serial hardware output:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/control-run/serial-observe-window.json.
- Candidate same-power-cycle TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/candidate-run/tftp-delta-stable-pre-restore.json.
- Control same-power-cycle TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/control-run/tftp-delta-stable-pre-restore.json.
- Final restored lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/final-lab-status.json.
- Closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: diagnostic proof task, classification JSON,
  evidence map, candidate/control v4 JSON, boot-staging identity outputs,
  serial windows, TFTP deltas, final identity, restore evidence, Phase 12 docs,
  roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Raw and decoded PHY1 status evidence reconciled without broadening beyond the
  diagnostic frontier: satisfied.
- Next task direction stated: satisfied; the selected next task is
  phase12-rp1-ethernet-phy1-link-readiness-source-contract-20260614.
- Inconclusive/blocker policy: not applicable; the diagnostic proof was
  accepted, and same-shaped retry is not selected from this closeout.
- Closeout committed before any follow-up starts: satisfied by the closeout
  commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-phy1-link-readiness-source-contract-20260614 on the next
worker wake if dependencies remain satisfied. Do not start implementation,
hardware proof, PHY configuration, reset/GPIO32 action, packet I/O,
networking, SSH, Phase 12.2, or a phase transition directly from this
closeout.
