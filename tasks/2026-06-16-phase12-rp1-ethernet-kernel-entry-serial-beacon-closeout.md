# Phase 12.1 RP1 Ethernet Kernel Entry Serial Beacon Closeout

Task:
phase12-rp1-ethernet-kernel-entry-serial-beacon-closeout-20260616.

Status: accepted

Classification:
kernel-entry-serial-beacon-frontier-closed-beacon-observed.

Evidence level: static/task evidence inspection, accepted beacon-core review,
accepted serialized Pi 5 proof review, JSON evidence validation, docs build,
and diff checks. No new Pi 5 hardware run, boot archive publication, lab
mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
volatile Ethernet access, register retry, GPIO32 event clear/reset recovery,
BMCR write, Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition
was performed.

## Goal

Close out the earliest-kernel-entry serial beacon discriminator, reconcile the
accepted local/static beacon core and serialized Pi 5 proof evidence, and set
the next explicit Phase 12.1 boundary.

## Scope Performed

- Inspected the accepted beacon core task, classification JSON, evidence map,
  and static artifact review.
- Inspected the accepted serialized Pi 5 proof task, classification JSON,
  evidence map, run summary, capture summary, TFTP evidence, serial evidence,
  and restore evidence.
- Reconciled accepted, deferred, rejected, removed, and not-an-issue findings
  against the preceding boot-transport sentinel closeout.
- Updated Phase 12 project docs and roadmap with the closed kernel-entry
  serial beacon frontier.
- Set supervisor planning as the next action because no explicit queued
  follow-up exists after this closeout.

## Findings

- fixed: the beacon core selected
  `rpi5_rp1_ethernet_kernel_entry_serial_beacon`, which emits
  `TALOS: rp1-ethernet-kernel-entry-serial-beacon` before BootInfo parsing
  and before any Ethernet or MDIO behavior.
- fixed: static artifact review retained the beacon marker and
  `core-static-20260616` nonce in the generated 47,336-byte local image and
  rejected BCM54213PE register values, link readiness, packet I/O, networking,
  SSH, Phase 12.2, and phase-transition claims.
- fixed: the serialized Pi 5 proof retained selected-tree identity
  `68d4c9ae71014c85199391abf7bb54d1bfbe62de17482a3354cb4f7cfea43376`,
  effective kernel `kernel_2712.img`, and two fresh
  `da591740/kernel_2712.img` TFTP serves at 47,360 bytes.
- fixed: the candidate serial window retained the run-unique earliest-entry
  marker with nonce `kernel-entry-beacon-cand-20260616T053728Z` 89 times,
  proving earliest Rust-entry serial visibility for the no-Ethernet/no-MDIO
  discriminator after a fresh Pi 5 TFTP fetch.
- fixed: restore evidence returned the lab boot tree to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- deferred: BCM54213PE register-read code, sentinel/report execution after
  BootInfo parsing, PHY/MAC status, and any later boot/report path that
  constructs Ethernet or MDIO facts remain deferred to a separately planned
  task.
- rejected: BCM54213PE register values, Ethernet readiness, link readiness,
  GPIO32 reset ownership, BMCR writes, Broadcom shadow/MMD/aux access,
  interrupt ownership, broad PHY/MAC configuration, packet I/O, networking,
  SSH, Phase 12.2, and phase transition remain rejected.
- removed: no task-owned source, helper, docs, or evidence files were removed.
- not-an-issue: same-task known-good control was not required by the accepted
  proof because the candidate itself retained selected-tree identity, fresh
  TFTP, firmware serial, beacon serial, final pre-restore identity, and restore
  proof, while the preceding accepted boot-transport sentinel candidate/control
  already proved the publication and capture path.

## Reconciliation

The preceding boot-transport sentinel closeout narrowed the blocker away from
generic selected-tree publication or TFTP transport, but it left fetched-kernel
execution or sentinel serial-emission visibility unaccepted because neither
sentinel image emitted its nonce marker.

The kernel-entry serial beacon proof is a deliberately thinner discriminator:
it removes Ethernet, MDIO, MAN, MACB, GPIO32, PHY, packet, networking, and SSH
facts, then emits a run-unique marker at the earliest Rust-entry branch before
BootInfo parsing. The accepted Pi 5 result proves that a freshly fetched
no-Ethernet/no-MDIO Talos kernel can reach that earliest serial beacon. The
remaining Phase 12.1 blocker is therefore not generic selected-tree transport,
TFTP fetch, firmware serial capture, or earliest Rust-entry serial visibility
for this no-Ethernet/no-MDIO shape.

This closeout does not reinterpret the beacon as proof of the earlier
BCM54213PE read-only candidate, the boot-transport sentinel report path, or any
Ethernet behavior. A future task must be supervisor-planned and must name the
next distinct boundary, such as BootInfo/report-path visibility, candidate
shape isolation, or a different source/static contract.

## Frontier

Closed frontier:
kernel-entry-serial-beacon-frontier-closed-beacon-observed.

Accepted: local/static earliest-entry beacon core, selected-tree publication,
fresh TFTP serving of the selected 47,360-byte kernel, earliest Rust-entry
serial marker visibility after power-cycle, final pre-restore identity, restore
proof, and the conclusion that generic fetched-kernel earliest serial
visibility is not the remaining blocker for this no-Ethernet/no-MDIO beacon.

Deferred: BCM54213PE register values, boot/report path behavior after BootInfo
parsing, sentinel report emission, PHY/MAC status, GPIO32/PHY reset ownership,
BMCR/autoneg or Broadcom selector work, and any distinct follow-up
discriminator selected by supervisor planning.

Not accepted: Ethernet driver readiness, link readiness, packet I/O,
networking, SSH, Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. The next
decision must select one explicit Phase 12.1 boundary or an explicit pause.

No dependency-gated queued task remains mechanically unblocked after this
closeout. The closeout does not authorize a register-read retry, GPIO32 event
clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access, interrupt
ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, or a
phase transition.

## Evidence

- Beacon core task:
  tasks/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-core.md.
- Beacon core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-core/classification.json.
- Beacon core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-core/evidence-map.json.
- Beacon core static artifact review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-core/static-artifact-review.json.
- Serialized Pi 5 proof task:
  tasks/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof.md.
- Serialized Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/classification.json.
- Serialized Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/evidence-map.json.
- Serialized Pi 5 run summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/candidate/run-summary.json.
- Serialized Pi 5 capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/candidate/capture-invariant-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: beacon core task/classification/evidence
  map/static artifact review, Pi 5 proof task/classification/evidence map, run
  summary, capture summary, docs, roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout reconciles beacon core and hardware evidence, including blocked or
  inconclusive paths: satisfied.
- Rejected claims remain explicit: satisfied.
- Next boundary is explicit: satisfied by planningNeeded=true for supervisor
  selection of a distinct Phase 12.1 follow-up or pause.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once state is updated after this commit.

## Next Action

Set planningNeeded=true for supervisor planning. Do not start hardware,
register-read retry, GPIO32 event clear/reset recovery, BMCR write, Broadcom
shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, or phase transition from this closeout.
