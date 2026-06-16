# Phase 12.1 RP1 Ethernet Serial Freshness Closeout

Task:
phase12-rp1-ethernet-serial-freshness-closeout-20260616.

Status: accepted

Classification:
serial-freshness-frontier-closed-cursor-nonce-accepted.

Evidence level: static/task evidence inspection, accepted serial freshness
contract review, accepted guard-core review, accepted serialized Pi 5 proof
review, JSON evidence validation, docs build, and diff checks. No new Pi 5
hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, power-cycle, TFTP/serial capture, Ethernet/MMIO/MDIO/register
retry, GPIO32/PHY reset, BMCR/autoneg write, interrupt ownership, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the serial freshness discriminator, reconcile the accepted contract,
guard-core, and Pi 5 proof evidence, and set the next explicit boundary before
any future BootInfo/report rerun or BCM54213PE register discriminator.

## Scope Performed

- Inspected the accepted serial freshness contract task, classification JSON,
  and evidence map.
- Inspected the accepted guard-core task, classification JSON, evidence map,
  synthetic validator outputs, and guard script boundary.
- Inspected the accepted serialized Pi 5 proof task, classification JSON,
  evidence map, retained guard replay, capture summary, TFTP delta, final
  pre-restore identity, and restore proof.
- Reconciled fixed, rejected, removed, and not-an-issue findings against the
  preceding BootInfo/report-path serial-drain blocker.
- Updated Phase 12 project docs and roadmap with the closed serial freshness
  frontier and the need for supervisor planning before any follow-up.
- Set supervisor planning as the next action because no explicit queued
  follow-up remains after this closeout.

## Findings

- fixed: The source/static contract replaced the old hard empty-drain gate for
  marker-only transport proofs with cursor-nonce-post-power-freshness-v1. A
  non-empty bounded pre-power drain may be accepted only when the run-unique
  marker/nonce is absent before power, present after the saved cursor or
  saturated-cursor direct-read fallback, and joined with selected-tree, TFTP,
  final-identity, and restore evidence.
- fixed: The guard-core task added
  scripts/rpi5-serial-freshness-guard-v1-check.sh, retained
  pre-power-serial-peek.json, and proved the validator accepts a well-formed
  cursor-fresh fixture while rejecting stale backlog, cursor mismatch, missing
  marker, selected-tree/TFTP mismatch, and restore failure fixtures.
- fixed: The serialized Pi 5 proof retained selected tree
  f73c75438663373b3d6df4e0ce451a45f163c4a582d8ba84bd79d161cf9cc68f,
  a 47,352-byte kernel_2712.img, two matching da591740/kernel_2712.img TFTP
  serves, final pre-restore selected-tree identity, and restore proof back to
  baseline a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: The Pi 5 proof retained pre-power serial cursor 4194304 and 65,536
  retained bytes with zero occurrences of the run nonce
  serial-freshness-20260616T085546Z; the post-power saturated direct-read
  fallback retained 70,594 bytes with 45 marker and nonce occurrences, so the
  guard accepted serial-freshness-guard-v1-ready.
- fixed: The serial-drain-not-empty-before-power blocker is closed for this
  marker-only transport class when cursor-nonce-post-power-freshness-v1 and the
  selected-tree/TFTP/final-identity/restore join all pass.
- deferred: Any BootInfo/report-path rerun, BCM54213PE register discriminator,
  or explicit pause requires supervisor planning with a separate task and
  explicit acceptance gates.
- rejected: The closeout does not authorize direct promotion to a BCM54213PE
  register retry, packet I/O, networking, SSH, Phase 12.2, or a phase
  transition.
- rejected: BCM54213PE register values, Ethernet readiness, link readiness,
  GPIO32/PHY reset ownership, BMCR/autoneg, Broadcom shadow/MMD/aux access,
  interrupt ownership, and broad PHY/MAC configuration remain rejected.
- removed: No task-owned source, helper, docs, or evidence files were removed.
- not-an-issue: The accepted Pi 5 proof's bounded pre-power drain remained
  non-empty, but the run-unique nonce absence before power and presence in the
  post-power saturated direct-read window makes that retained backlog
  non-decisive for this marker-only proof.

## Reconciliation

The BootInfo/report-path closeout identified serial-drain-not-empty-before-power
as the first failing invariant for the dual-stage marker proof. It did not
reject selected-tree publication, same-power-cycle TFTP serving, marker
emission, final pre-restore identity, or restore proof.

The serial freshness contract made the missing discriminator explicit. The
guard core then made that contract replayable against retained evidence, and
the serialized Pi 5 proof exercised the thinnest no-Ethernet/no-MDIO marker
path with a run-unique nonce. That proof showed the nonce was absent from
pre-power retained serial bytes and present 45 times after the saved saturated
cursor boundary, while the same power cycle retained matching TFTP, selected
tree, final identity, and restore evidence.

The closed frontier is intentionally narrow. Future proofs may rely on
cursor-based serial freshness for marker-only transport evidence only when they
retain the v1 fields and pass the guard. Empty pre-power drain remains strong
positive evidence, but it is no longer a hard requirement for this class. The
closeout does not reinterpret marker freshness as Ethernet behavior, link
readiness, or BCM54213PE register visibility.

## Frontier

Closed frontier:
serial-freshness-frontier-closed-cursor-nonce-accepted.

Accepted: cursor-nonce-post-power-freshness-v1 as the replacement for a hard
empty-drain gate in marker-only transport proofs; task-owned guard replay for
retained evidence; selected-tree identity, same-power-cycle TFTP byte serves,
post-power marker/nonce visibility after the saved cursor boundary, final
pre-restore identity, and restore proof for the no-MDIO/no-Ethernet Pi 5
freshness proof.

Deferred: supervisor selection of any BootInfo/report-path rerun,
BCM54213PE register discriminator, explicit pause, or other Phase 12.1
boundary that uses this freshness frontier as an input.

Not accepted: BCM54213PE register values, Ethernet driver readiness, link
readiness, GPIO32/PHY reset ownership, BMCR/autoneg or Broadcom selector work,
interrupt ownership, broad PHY/MAC configuration, packet I/O, networking, SSH,
Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. No
dependency-gated queued task remains mechanically unblocked after this closeout.

The closeout does not authorize a hardware run, boot archive publication,
register-read retry, GPIO32/PHY reset work, BMCR/autoneg writes, Broadcom
shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, or a phase transition.

## Evidence

- serial freshness contract task:
  tasks/2026-06-16-phase12-rp1-ethernet-serial-freshness-contract.md.
- serial freshness contract classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-contract/classification.json.
- serial freshness contract evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-contract/evidence-map.json.
- guard-core task:
  tasks/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core.md.
- guard-core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core/classification.json.
- guard-core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core/evidence-map.json.
- guard-core validator summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core/validator-results.json.
- serialized Pi 5 proof task:
  tasks/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof.md.
- serialized Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/classification.json.
- serialized Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/evidence-map.json.
- serialized Pi 5 proof guard replay:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/candidate/serial-freshness-guard.json.
- serialized Pi 5 proof capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/candidate/capture-invariant-summary.json.
- closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-closeout/classification.json.
- closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: contract task/classification/evidence map,
  guard-core task/classification/evidence map/validator summary, Pi 5 proof
  task/classification/evidence map/guard replay/capture summary, docs,
  roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout reconciles contract, guard-core, and hardware evidence with findings
  dispositions: satisfied.
- Frontier states future proofs may rely on cursor-based freshness when the v1
  guard fields pass, while empty drain remains strong positive evidence but not
  a hard gate for marker-only transport proofs: satisfied.
- Rejected claims remain explicit: satisfied.
- Next boundary is explicit: satisfied by planningNeeded=true for supervisor
  selection of one distinct Phase 12.1 follow-up or pause.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once state is updated after this commit.

## Next Action

Set planningNeeded=true for supervisor planning. Do not start hardware,
register-read retry, GPIO32/PHY reset work, BMCR/autoneg writes, Broadcom
shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, or phase transition from this closeout.
