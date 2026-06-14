# Phase 12 RP1 Ethernet Capture-Staging Recurrence Checkpoint

Task id: phase12-rp1-ethernet-capture-staging-recurrence-checkpoint-20260614

Status: accepted

Classification: capture-staging-recurrence-checkpoint-accepted

Evidence level: static/task evidence inspection, task-owned JSON evidence,
helper contract inspection, lab-controller documentation inspection, retained
lab-controller API evidence review, retained serial hardware boot/output
evidence review, retained stable same-cursor TFTP delta evidence review,
retained capture-chain-v4 replay review, retained boot-staging identity replay
review, retained known-good baseline triage review, and retained restore proof
review. No hardware action, lab mutation, boot archive publication, runtime
implementation, retry, or phase transition was performed.

## Goal

Reconcile the PHY1 autoneg-restart capture-staging blocker against the accepted
root-recovery minimal sentinel and v4 register-vector identity evidence, then
select one objective recovery discriminator before any same-shaped hardware
retry.

## Scope Performed

- Inspected the accepted autoneg-restart source contract, guard core, Pi 5
  proof, closeout, classification JSON, capture summary, evidence map,
  candidate static archive review, TFTP delta, boot-staging identity output,
  capture-chain-v4 output, known-good baseline triage, and restore evidence.
- Compared the blocker with the accepted minimal sentinel served-root proof and
  accepted v4 MDIO register-vector proof.
- Inspected the retained capture-chain-v4, boot-staging identity, and proof
  bundle helper contracts only far enough to determine whether static repair is
  indicated before another discriminator.
- Inspected lab-controller documentation for served-root, stable TFTP delta,
  final pre-restore identity, and restore evidence boundaries.
- Selected exactly one preplanned next task id.

## Findings

- fixed: the autoneg-restart proof identified the first failing invariant as
  same-power-cycle TFTP and final pre-restore identity not matching the
  selected candidate tree.
- fixed: the autoneg candidate selected tree
  6bf7d36a3f07426f450fd8a4def73b9cc8bbbc5b730ba50503fd0ee8f41609e1 and
  expected da591740/kernel_2712.img size 52360 bytes were visible through the
  lab API before power.
- blocked: the autoneg candidate same-power-cycle TFTP delta served four
  baseline-sized 104136-byte kernel fetches, final pre-restore identity was the
  baseline tree, and the run-unique serial marker was absent.
- blocked: known-good baseline triage after restore produced a stable
  zero-event TFTP delta, so it did not prove that fresh TFTP visibility was
  recovered for a same-shaped candidate rerun.
- fixed: the accepted root-recovery minimal sentinel at commit
  12605ec7263abfe7ffb46a766b4cbf5cd8c8e3e8 proved the selected-tree/TFTP/final
  identity chain for one no-MDIO minimal sentinel: selected tree
  5dd6afef125a27bbb4e76423fbd189fe1dc020bc9cf2186e42bba7eae5581441,
  two matching 47832-byte TFTP fetches, visible run-unique serial marker, final
  selected-tree identity, and restore to baseline.
- fixed: the accepted v4 register-vector proof at commit
  a4d203906e1b9ac5686342cff5c9ffb1bc1d909b proved the same identity chain for
  candidate/control runs after root recovery, including candidate selected tree
  043744bcf578d7966c63600c3db0302e35e96ec631f6d535725c8e63002fd43d with two
  matching 52352-byte TFTP fetches and final selected-tree identity.
- not-an-issue: static helper inspection found the retained capture-chain-v4
  and boot-staging identity gates preserve the right rejection boundary:
  API-visible /boot/files identity alone is insufficient without same-power-cycle
  TFTP byte agreement and final pre-restore identity.
- not-an-issue: the accepted minimal sentinel and v4 register-vector results do
  not by themselves unblock an autoneg retry because the later autoneg run
  regressed at the live capture-staging boundary and the known-good triage did
  not show fresh TFTP events.
- deferred: any helper/code repair is deferred until a fresh minimal sentinel
  reproduces a concrete helper or publication invariant failure; no static
  repair target was found in this checkpoint.
- removed: no source, helper, task, evidence, or documentation files were
  removed.

## Reconciliation

The autoneg-restart proof reached a precise capture-staging blocker before any
runtime PHY1 BMCR write evidence could be accepted. The pre-power selected-tree
identity was coherent, but the same power-cycle did not boot that selected tree:
TFTP served baseline-sized da591740/kernel_2712.img bytes, final pre-restore
identity was the baseline tree, and the run-unique serial marker was absent.
Boot-staging identity and capture-chain-v4 correctly rejected this as
capture-staging-blocked.

The accepted comparator proofs show that the capture/publication path can be
decisive, but only for earlier runs. The root-recovery minimal sentinel proved
one no-MDIO selected-tree/TFTP/final-identity path, and the v4 register-vector
proof later proved candidate/control selected-tree identity with matching TFTP
bytes and visible serial markers. Those accepted runs remain valid historical
evidence; they do not prove that the current live capture path recovered after
the autoneg recurrence. The zero-event known-good TFTP triage makes another
same-shaped autoneg retry non-decisive without a smaller capture-only
discriminator.

Static helper inspection did not find a bounded repair to make before the next
discriminator. The retained helper contracts already encode the needed
boundary: selected /boot/files identity is not enough, the stable TFTP delta
must include the expected fetch with matching bytes before restore, final
pre-restore identity must still be the selected tree, restore identity must be
recorded, and serial markers must be fresh for runtime claims.

## First Failing Invariant

First failing invariant:
same-power-cycle TFTP-served bytes and final pre-restore identity did not match
the selected autoneg candidate tree.

Secondary unresolved invariant:
known-good baseline triage produced no fresh TFTP events, so current TFTP
visibility for a same-shaped retry remained unproven.

## Selected Next Task

Selected next objective task id:
phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof-20260614.

Reason: a fresh minimal sentinel is the smallest preplanned discriminator that
tests the current live selected-tree/TFTP/final-identity path without MDIO,
MACB, PHY, autonegotiation, GPIO32, packet I/O, networking, SSH, or Phase 12.2
behavior. It can either recover confidence in capture-staging freshness or
record the first failing live invariant for a later repair-core task.

Not selected:
phase12-rp1-ethernet-capture-staging-repair-core-20260614. No static helper,
publication, or documentation defect was identified that can be repaired
without fresh live discriminator evidence.

## Rejected Claims Preserved

This checkpoint does not accept runtime autonegotiation evidence, runtime PHY1
BMCR write success/failure, link readiness, physical link partner readiness,
GPIO32/PHY reset ownership, Ethernet readiness, DMA/descriptors, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Autoneg restart proof:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof.md.
- Autoneg proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/classification.json.
- Autoneg proof capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/capture-summary.json.
- Autoneg candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/v4-check.json.
- Autoneg candidate boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/boot-staging-identity.json.
- Autoneg candidate TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/tftp-delta-stable-pre-restore.json.
- Autoneg known-good baseline triage:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/triage-known-good-baseline/classification.json.
- Autoneg closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-closeout.md.
- Root-recovery minimal sentinel:
  tasks/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof.md.
- Root-recovery minimal sentinel capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/capture-summary.json.
- v4 register-vector proof:
  tasks/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery.md.
- v4 register-vector classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/classification.json.
- Capture-chain-v4 helper:
  scripts/rpi5-capture-chain-v4-retained-fixtures.sh.
- Boot-staging identity helper:
  scripts/rpi5-boot-staging-identity-check.sh.
- Proof bundle helper:
  scripts/rpi5-capture-invariant-proof-bundle.sh.
- Lab-controller evidence contract:
  docs/src/project/lab-controller.md.
- Checkpoint classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-recurrence-checkpoint/classification.json.
- Checkpoint evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-recurrence-checkpoint/evidence-map.json.

## Validation

- static/task evidence inspection: source contract, guard core, proof task,
  closeout, classification JSON, capture summary, evidence map, candidate v4
  JSON, boot-staging identity JSON, TFTP delta, known-good baseline triage,
  root-recovery minimal sentinel evidence, v4 register-vector evidence,
  helper scripts, lab-controller docs, Phase 12 docs, roadmap, and git history
  inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- helper replay gate: not run because no capture/publication helper code was
  changed.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- First failing invariant identified: satisfied.
- Blocked autoneg proof compared against accepted root-recovery minimal
  sentinel and v4 register-vector evidence: satisfied.
- Findings listed with disposition: satisfied.
- Task-owned JSON records exactly one selected_next_task: satisfied,
  phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof-20260614.
- Rejected claims preserved: satisfied.
- Accepted checkpoint committed before follow-up starts: satisfied by the
  checkpoint commit.

## Next Action

Promote phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof-20260614
on a later worker wake if dependencies remain satisfied and hardware lock is
available/restored. Do not run autoneg, MDIO, MACB, GPIO32, packet I/O,
networking, SSH, Phase 12.2, or a phase transition from this checkpoint.
