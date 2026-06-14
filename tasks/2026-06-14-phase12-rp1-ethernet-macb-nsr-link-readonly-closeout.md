# Phase 12 RP1 Ethernet MACB NSR_LINK Read-Only Closeout

Task id: phase12-rp1-ethernet-macb-nsr-link-readonly-closeout-20260614

Status: accepted

Classification: rp1-ethernet-macb-nsr-link-readonly-frontier-closed

Evidence level: static/task evidence inspection, capture-chain-v4 replay
review, boot-staging identity gate review, evidence-consistency guard review,
lab-controller API evidence review, serial hardware boot/output evidence
review, stable same-cursor TFTP delta evidence review, and restore evidence
review.

## Goal

Close out the accepted passive MAC-side MACB_NSR_LINK read-only proof without
broadening beyond the selected comparator discriminator.

## Scope Performed

- Inspected the accepted MACB_NSR_LINK proof task record, classification JSON,
  capture summary, evidence map, candidate/control capture-chain-v4 JSON,
  boot-staging identity outputs, evidence-consistency guard, serial windows,
  TFTP deltas, final identity evidence, and restore proof.
- Reconciled the MACB_NSR_LINK clear result against the accepted source
  contract, PHY1 link-not-ready frontier, corrected-target MDIO frontier, and
  GPIO32 no-write/event-clear blockers.
- Recorded the closed MACB_NSR_LINK discriminator frontier and retained
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
  macb-nsr-link-readonly-link-clear for the selected read-only MACB_NSR bit-0
  observation at 0x1c00100008.
- fixed: the candidate reported MACB_MID context 0x70109, MACB_NSR raw 0x6,
  NSR_LINK=false, macb_read_performed=true, macb_write_performed=false,
  mdio_target_constructed=false, and man_frame_constructed=false.
- fixed: the paired control used the same reporting surface while constructing
  no MACB_NSR target, performing no Ethernet volatile load/store, and
  withholding candidate-only target/raw/decode/result fields.
- fixed: candidate/control proof identity remained decisive under
  capture-chain-v4 and boot-staging identity: selected-tree identity,
  same-power-cycle TFTP-served byte agreement, final pre-restore identity,
  serial freshness, evidence-consistency-ready, and restore proof all matched.
- fixed: lab restore evidence remains baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: link recovery, PHY configuration writes, PHY reset/GPIO32 action,
  autonegotiation restart, link forcing, MACB write ownership, broad MDIO/PHY
  ownership, Ethernet driver behavior, interrupts, DMA/descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  future explicitly planned tasks.
- removed: no stale evidence, source code, task records, or helper scripts
  were removed.
- not-an-issue: MACB_NSR_LINK is only a MAC-side comparator at the selected
  instant; the accepted PHY1 BMSR link-not-ready frontier and GPIO32 blockers
  remain in force.

## Reconciliation

The MACB_NSR_LINK proof task is
phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof-20260614 at commit
53eb9f95e8c6b6dab81b89bfc189ef786283fc1e.

Candidate capture-chain-v4 and boot-staging identity are both ready. The
candidate selected tree was
937e30a34797c672f393e6cd7f4c4b12c6b1a0ea1e5b97c8c6afbbc8788a3522, TFTP
served two da591740/kernel_2712.img fetches at the expected 49,728 bytes, and
final pre-restore identity stayed on the selected tree. Serial freshness was
true with the run-unique nonce present after power and absent before power.
The accepted read-only MAC-side comparator observation is MACB_NSR at
0x1c00100008 with raw value 0x6 and NSR_LINK bit 0 decoded as false.

Control capture-chain-v4 and boot-staging identity are both ready. The control
selected tree was
ff82cf02034aa877cf5907a8456be504e966109a6a9ac51992e23a9b79457c70, TFTP
served two da591740/kernel_2712.img fetches at the expected 49,480 bytes, and
final pre-restore identity stayed on the selected tree. The control proves only
the no-MMIO/no-Ethernet reporting path for this proof shape.

The proof task markdown, classification JSON, capture summary, evidence map,
candidate/control v4 outputs, candidate/control boot-staging identity gates,
serial windows, TFTP deltas, final pre-restore identity, restore evidence, and
evidence-consistency guard agree on the narrow MACB_NSR_LINK clear
classification.

## Frontier

Closed frontier:
rp1-ethernet-macb-nsr-link-readonly-frontier-closed.

Accepted: read-only MACB_NSR bit-0 observation at 0x1c00100008 under
capture-chain-v4, boot-staging identity, same-power-cycle TFTP byte agreement,
final pre-restore identity, serial freshness, evidence-consistency-ready, and
restore evidence. The observed MAC-side comparator result is NSR_LINK clear.

Not accepted: link recovery, Ethernet readiness, MACB writes, MDIO/PHY access,
PHY configuration writes, BMCR writes, autonegotiation restart, link forcing,
PHY reset/GPIO32 action, DMA/descriptors, interrupts, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Next Direction

No explicit queued mechanically objective follow-up task exists after this
closeout. Supervisor planning is required to select the next bounded Phase
12.1 task and its acceptance gates. Any follow-up must depend on this closed
frontier and must stay feature-led; this closeout does not authorize MACB
writes, PHY configuration, PHY reset/GPIO32 action, broad MDIO/PHY ownership,
Ethernet behavior, packet I/O, networking, SSH, Phase 12.2, or a phase
transition.

## Evidence

- MACB_NSR_LINK proof task:
  tasks/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof.md.
- MACB_NSR_LINK proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/classification.json.
- MACB_NSR_LINK proof capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/capture-summary.json.
- MACB_NSR_LINK proof evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/evidence-map.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/control-run/v4-check.json.
- Candidate boot-staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/candidate-run/boot-staging-identity.json.
- Control boot-staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/control-run/boot-staging-identity.json.
- Evidence-consistency guard:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/evidence-consistency-guard.json.
- Final restored lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/final-lab-status.json.
- Closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-closeout/evidence-map.json.

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
- MACB_NSR_LINK clear result reconciled with the paired control
  classification: satisfied.
- Result stated relative to the accepted PHY1 link-not-ready frontier and
  GPIO32 blockers: satisfied.
- Rejected claims for MACB writes, PHY configuration/reset, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition preserved:
  satisfied.
- Closeout committed before any follow-up starts: satisfied by the closeout
  commit.
- No objective next task exists: satisfied; planningNeeded is set for
  supervisor planning of the next bounded Phase 12.1 task.

## Next Action

Set planningNeeded=true for supervisor planning of the next bounded Phase 12.1
task. Do not infer MACB write ownership, PHY configuration,
PHY reset/GPIO32 action, broad MDIO/PHY ownership, Ethernet behavior, packet
I/O, networking, SSH, Phase 12.2, or a phase transition from this closeout.
