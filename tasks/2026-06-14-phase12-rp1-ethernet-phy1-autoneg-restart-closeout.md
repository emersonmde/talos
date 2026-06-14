# Phase 12 RP1 Ethernet PHY1 Autoneg Restart Closeout

Task id: phase12-rp1-ethernet-phy1-autoneg-restart-closeout-20260614

Status: accepted

Classification: rp1-ethernet-phy1-autoneg-restart-capture-staging-blocker-frontier-closed

Evidence level: static/task evidence inspection, static archive/image review,
lab-controller API evidence review, serial hardware boot/output
evidence review, stable same-cursor TFTP delta evidence review,
capture-chain-v4 replay review, boot-staging identity replay review,
known-good baseline triage review, and restore proof review.

## Goal

Close out the guarded PHY1 BMCR autonegotiation-restart proof as a precise
capture-staging blocker without broadening it into PHY/autoneg runtime,
link-readiness, Ethernet, packet I/O, networking, SSH, or Phase 12.2 evidence.

## Scope Performed

- Inspected the accepted source contract, guard core, Pi 5 proof task record,
  classification JSON, capture summary, evidence map, candidate static archive
  review, control static archive review, candidate lab identity evidence,
  serial window, stable same-cursor TFTP delta, capture-chain-v4 output,
  boot-staging identity output, known-good baseline triage evidence, and final
  restore evidence.
- Reconciled the candidate blocker against the accepted PHY1 link-not-ready
  frontier, passive MACB_NSR_LINK=false frontier, corrected-target MDIO/MAN
  boundary, retained GPIO32 blockers, and rejected claims.
- Recorded the closed autoneg-restart proof frontier as a capture-staging
  blocker. No runtime PHY1 BMCR write evidence was accepted.
- Updated Phase 12 project and roadmap docs with the closed blocker frontier.
- Set supervisor planning as the next action because no explicit queued
  mechanically objective follow-up task exists after this closeout.

## Findings

- fixed: the source contract, guard core, proof task record, classification
  JSON, capture summary, and evidence map agree that no runtime PHY1
  autonegotiation-restart evidence was accepted.
- fixed: candidate static archive review passed for selected tree
  6bf7d36a3f07426f450fd8a4def73b9cc8bbbc5b730ba50503fd0ee8f41609e1 and
  expected da591740/kernel_2712.img size 52360 bytes.
- blocked: candidate same-power-cycle TFTP evidence served four baseline-sized
  104136-byte da591740/kernel_2712.img fetches instead of the selected
  52360-byte candidate kernel.
- blocked: candidate capture-chain-v4 rejected the hardware evidence as
  capture-staging-blocked because the run-unique marker was absent, TFTP bytes
  did not match the selected candidate, and final pre-restore identity was the
  baseline tree.
- blocked: candidate boot-staging identity replay rejected the evidence for
  TFTP byte mismatch, selected-tree mismatch, final baseline identity, and
  final expected-fetch byte mismatch.
- blocked: known-good baseline triage power succeeded but produced no fresh
  TFTP events, so a candidate rerun was not attempted.
- fixed: lab restore evidence returns to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: paired control hardware publication/run, runtime BMCR write
  evidence, link-still-not-ready/physical-precondition classification, and
  any capture-layer recovery remain future supervisor-planned work.
- removed: no stale evidence, source code, task records, or helper scripts
  were removed.
- not-an-issue: the proof retained the non-goal boundary: no GPIO32/PHY reset,
  MACB write, NCR write, link forcing, packet I/O, DMA/descriptors,
  interrupts, networking, sockets, SSH, Phase 12.2, or phase transition.

## Reconciliation

The autoneg restart proof task is
phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof-20260614 at commit
84fc42a1747186a3eb14e4d4f69ecbc3464c5ed9.

The accepted source contract defined one future corrected-target PHY1 BMCR
autonegotiation-restart attempt: pre-read PHY1 BMCR/BMSR/ANAR/ANLPAR, reject
preconditions before any write if needed, perform exactly one guarded BMCR
write setting BMCR_ANENABLE and BMCR_ANRESTART while preserving other pre-read
BMCR bits, then bounded-read BMCR, double-sample BMSR, read ANAR/ANLPAR, and
passively compare MACB_NSR_LINK. The guard core implemented that candidate
report surface and the paired no-MDIO/no-MACB control surface locally.

The Pi 5 proof did not reach the runtime discriminator. Candidate publication
was visible through /boot/files as selected tree
6bf7d36a3f07426f450fd8a4def73b9cc8bbbc5b730ba50503fd0ee8f41609e1 with
expected da591740/kernel_2712.img size 52360 bytes, but the hardware capture
path then observed four 104136-byte baseline TFTP fetches. Final pre-restore
identity was baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and the
candidate run-unique serial marker was absent.

The first failing invariant is: same-power-cycle TFTP and final pre-restore
identity did not match the selected candidate tree. That blocks all runtime
interpretation of the candidate output for BMCR write success/failure,
autonegotiation restart, link readiness, or physical/operator precondition
state.

Known-good baseline triage restored the baseline and power-cycled once.
Final identity remained baseline with da591740/kernel_2712.img size 104136
bytes, but stable same-cursor TFTP delta contained zero expected fetch events.
That triage is not a hardware/lab incident; it is evidence that the current
capture layer was not producing a fresh TFTP observation suitable for a
same-shaped candidate rerun.

## Frontier

Closed frontier:
rp1-ethernet-phy1-autoneg-restart-capture-staging-blocker-frontier-closed.

Accepted: the guarded PHY1 autoneg-restart proof reached a precise
capture-staging blocker under retained static archive review, lab-controller
selected-tree evidence, rejected same-power-cycle TFTP byte agreement,
rejected final pre-restore identity, capture-chain-v4 replay, boot-staging
identity replay, known-good baseline triage, and restore evidence.

Not accepted: runtime PHY1 BMCR write evidence, autonegotiation restart
success or failure, link-ready, link-still-not-ready,
physical/operator-precondition state, PHY configuration ownership, GPIO32/PHY
reset ownership, MACB writes, link forcing, Ethernet readiness, DMA/descriptors,
interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Next Direction

No explicit queued mechanically objective follow-up task exists after this
closeout. Supervisor planning is required before any capture-layer recovery,
same-shaped hardware retry, paired-control hardware run, PHY configuration
attempt, GPIO32/PHY reset action, packet I/O, networking, SSH, Phase 12.2, or
phase transition.

Any follow-up must first reconcile why /boot/files selected-tree identity
diverged from same-power-cycle TFTP and final pre-restore identity for this
candidate, and why known-good baseline triage produced no fresh TFTP events.
The autoneg restart source/guard work remains available, but it is not
hardware-accepted progress until the capture path can prove selected-tree
identity for a fresh run.

## Evidence

- Source contract task:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-source-contract.md.
- Guard core task:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-guard-core.md.
- Pi 5 proof task:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/classification.json.
- Pi 5 proof capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/capture-summary.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/evidence-map.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/v4-check.json.
- Candidate boot-staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/boot-staging-identity.json.
- Candidate TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/tftp-delta-stable-pre-restore.json.
- Known-good baseline triage classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/triage-known-good-baseline/classification.json.
- Final lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/final-lab-status.json.
- Closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: source contract, guard core, proof task,
  classification JSON, capture summary, evidence map, candidate/control static
  archive reviews, candidate v4 JSON, candidate boot-staging identity, serial
  window, TFTP delta, known-good baseline triage, final identity, restore
  evidence, Phase 12 docs, roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Closeout reconciles capture-blocked result without accepting runtime BMCR
  write evidence: satisfied.
- Rejected claims for GPIO32/PHY reset ownership, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition preserved: satisfied.
- Closeout selects at most one objective follow-up or planning-needed reason:
  satisfied; no explicit queued follow-up exists, so supervisor planning is
  required.
- Accepted closeout committed before follow-up starts: satisfied by the
  closeout commit.

## Next Action

Set planningNeeded=true for supervisor planning of the next bounded Phase 12.1
task. Do not infer autoneg runtime evidence, link recovery, PHY configuration,
GPIO32/PHY reset ownership, Ethernet behavior, packet I/O, networking, SSH,
Phase 12.2, or a phase transition from this closeout.
