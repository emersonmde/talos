# Phase 12.1 RP1 Ethernet BootInfo Report Serial Visibility Closeout

Task:
phase12-rp1-ethernet-bootinfo-report-serial-visibility-closeout-20260616.

Status: accepted

Classification:
bootinfo-report-serial-visibility-frontier-closed-serial-drain-blocked.

Evidence level: static/task evidence inspection, accepted local/static core
review, accepted serialized Pi 5 proof review, JSON evidence validation, docs
build, and diff checks. No new Pi 5 hardware run, boot archive publication, lab
mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
volatile Ethernet access, register retry, GPIO32 event clear/reset recovery,
BMCR write, Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Close out the dual-stage BootInfo/report-path serial visibility discriminator,
reconcile the accepted local/static core and serialized Pi 5 proof evidence,
and set the next explicit boundary.

## Scope Performed

- Inspected the accepted BootInfo/report-path serial visibility core task,
  classification JSON, evidence map, and static artifact review.
- Inspected the accepted serialized Pi 5 proof task, classification JSON,
  evidence map, final control/candidate capture summaries, TFTP evidence,
  serial marker evidence, final pre-restore identity, and restore evidence.
- Reconciled fixed, deferred, blocked, rejected, removed, and not-an-issue
  findings against the preceding boot-transport sentinel and kernel-entry
  serial beacon closeouts.
- Updated Phase 12 project docs and roadmap with the closed
  BootInfo/report-path frontier and the retained serial-drain/backlog blocker.
- Set supervisor planning as the next action because no explicit queued
  follow-up remains after this closeout.

## Findings

- fixed: the core created paired no-Ethernet/no-MDIO scenarios that emit
  bootinfo-report-visibility-earliest-entry-marker before target services and
  BootInfo reporting; only the candidate emits
  bootinfo-report-visibility-post-bootinfo-report-path-marker after
  report_boot_identity reports BootInfo and service metadata.
- fixed: static artifact review retained candidate/control archive and kernel
  hashes, marker strings, nonces, and forbidden-target-fact absence for
  BCM54213PE register values, MDIO/MAN/MACB/GPIO32/PHY facts, volatile
  Ethernet access, packet I/O, networking, SSH, Phase 12.2, and phase
  transition claims.
- fixed: the hardware proof repaired the first capture attempt by retaining the
  earliest-entry marker in repeated output and requiring that retained marker
  in the review scripts.
- fixed: final control evidence retained selected tree
  b886e168d26f69a943a98d77de87a40a7079938fa041aee8494e32cb98ea9178, two
  matching 55,120-byte da591740/kernel_2712.img TFTP serves, 71 earliest
  marker occurrences, zero post-BootInfo marker occurrences, final pre-restore
  identity, and restore proof.
- fixed: final candidate evidence retained selected tree
  38173e8bd614d6034e09e4944e0d5e92ad80dcebafb78b260897be7f74cc8c19, two
  matching 71,168-byte da591740/kernel_2712.img TFTP serves, 69 earliest
  marker occurrences, 68 post-BootInfo marker occurrences, final pre-restore
  identity, and restore proof.
- blocked: both final runs failed decisive classification because the
  capture-chain identity guard rejected serial-drain-not-empty-before-power.
  The bounded 128-attempt pre-power /serial/read drain exhausted without
  observing an empty response.
- deferred: a distinct discriminator for serial drain/backlog freshness, if
  any, must be supervisor-planned before another hardware proof.
- rejected: no same-shaped BootInfo/report-path serial visibility retry is
  authorized without a new discriminator for the serial-drain/backlog
  invariant.
- rejected: BCM54213PE register values, Ethernet readiness, link readiness,
  GPIO32 reset ownership, BMCR writes, Broadcom shadow/MMD/aux access,
  interrupt ownership, broad PHY/MAC configuration, packet I/O, networking,
  SSH, Phase 12.2, and phase transition remain rejected.
- removed: no task-owned source, helper, docs, or evidence files were removed.
- not-an-issue: retained marker output is useful hardware evidence, but the
  current capture contract correctly withholds a decisive both-markers-observed
  classification while pre-power serial freshness is unproved.

## Reconciliation

The preceding boot-transport sentinel closeout proved that selected-tree
publication and fresh TFTP serving were not generic blockers for the
no-Ethernet/no-MDIO sentinel pair. The kernel-entry serial beacon closeout then
proved earliest Rust-entry serial visibility for a freshly fetched
no-Ethernet/no-MDIO kernel.

The BootInfo/report-path proof moved the boundary later in the boot path and
made the hardware evidence mechanically distinguish earliest-only control
output from candidate output after report_boot_identity. Final control and
candidate runs did retain the expected selected trees, same-power-cycle TFTP
fetch sizes, final identity, restore proof, and separate marker counts. That
narrows the remaining blocker away from selected-tree publication, TFTP
serving, earliest-entry marker placement, and static BootInfo/report marker
shape.

The closed frontier is intentionally precise: the current evidence is blocked
by serial-drain-not-empty-before-power. The serial backlog/drain contract, not
Ethernet behavior or BCM54213PE register visibility, is the first failing
invariant for this discriminator. A future task must either select a distinct
serial freshness discriminator or pause; this closeout does not authorize a
register-read retry or broader networking work.

## Frontier

Closed frontier:
bootinfo-report-serial-visibility-frontier-closed-serial-drain-blocked.

Accepted: local/static dual-stage marker shape, selected candidate/control
archives, selected-tree identity for final control and candidate runs, matching
TFTP byte serves for the selected kernels, separate earliest and post-BootInfo
marker counts in retained serial output, final pre-restore identity, restore
proof, and the precise first failing invariant
serial-drain-not-empty-before-power.

Deferred: any distinct serial drain/backlog freshness discriminator, any later
BCM54213PE register-read retry selected by supervisor planning, and any
source/static contract needed to make such a retry materially different.

Not accepted: BCM54213PE register values, Ethernet driver readiness, link
readiness, GPIO32/PHY reset ownership, BMCR/autoneg or Broadcom selector work,
interrupt ownership, broad PHY/MAC configuration, packet I/O, networking, SSH,
Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. The next
decision must select one explicit Phase 12.1 boundary or an explicit pause.

No dependency-gated queued task remains mechanically unblocked after this
closeout. The closeout does not authorize a hardware retry, register-read
retry, GPIO32 event clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux
access, interrupt ownership, PHY/MAC configuration, packet I/O, networking,
SSH, Phase 12.2, or a phase transition.

## Evidence

- BootInfo/report-path core task:
  tasks/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-core.md.
- BootInfo/report-path core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-core/classification.json.
- BootInfo/report-path core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-core/evidence-map.json.
- BootInfo/report-path core static artifact review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-core/static-artifact-review.json.
- Serialized Pi 5 proof task:
  tasks/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof.md.
- Serialized Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/classification.json.
- Serialized Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/evidence-map.json.
- Final control capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/control-final/capture-invariant-summary.json.
- Final candidate capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/candidate-final/capture-invariant-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: core task/classification/evidence map/static
  artifact review, Pi 5 proof task/classification/evidence map, final
  control/candidate capture summaries, docs, roadmap, and git history
  inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout reconciles core and hardware evidence, including inconclusive paths:
  satisfied.
- Rejected claims remain explicit: satisfied.
- Next boundary is explicit: satisfied by planningNeeded=true for supervisor
  selection of one distinct Phase 12.1 follow-up or pause.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once state is updated after this commit.

## Next Action

Set planningNeeded=true for supervisor planning. Do not start hardware,
register-read retry, GPIO32 event clear/reset recovery, BMCR write, Broadcom
shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, or phase transition from this closeout.
