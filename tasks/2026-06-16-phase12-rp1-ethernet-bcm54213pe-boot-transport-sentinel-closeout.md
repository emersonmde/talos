# Phase 12 RP1 Ethernet BCM54213PE Boot-Transport Sentinel Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-closeout-20260616

Status: accepted

Classification:
bcm54213pe-boot-transport-sentinel-frontier-closed-kernel-serial-boundary

Evidence level: static/task evidence inspection, accepted sentinel-core review,
accepted serialized Pi 5 proof review, JSON evidence validation, docs build,
and diff checks. No new Pi 5 hardware run, boot archive publication, lab
mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
volatile Ethernet access, GPIO32 event clear/reset recovery, BMCR write,
Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration,
packet I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the BCM54213PE boot-transport sentinel discriminator, reconcile the
accepted local/static sentinel core and serialized Pi 5 proof evidence, and set
the next explicit Phase 12.1 boundary.

## Scope Performed

- Inspected the accepted sentinel core task, classification JSON, evidence map,
  and local/static candidate/control boot-scenario boundary.
- Inspected the accepted serialized Pi 5 sentinel proof task, classification
  JSON, evidence map, capture summary, candidate/control run summaries, TFTP
  deltas, serial windows, and restore evidence.
- Reconciled accepted, deferred, rejected, removed, and not-an-issue findings
  against the preceding BCM54213PE read-only preflight hardware-proof closeout.
- Updated Phase 12 project docs and roadmap with the closed boot-transport
  sentinel frontier and the remaining fetched-kernel execution or sentinel
  serial-emission boundary.
- Set supervisor planning as the next action because no explicit queued
  follow-up exists after this closeout.

## Findings

- fixed: the sentinel core defines exactly two no-Ethernet/no-MDIO boot
  scenarios:
  `rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate` and
  `rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control`.
- fixed: the sentinel core constructs no Ethernet, MDIO, MAN, MACB, GPIO32,
  PHY, packet, networking, SSH, or Phase 12.2 target facts and performs no
  volatile Ethernet access intent.
- fixed: the serialized Pi 5 proof retained selected-tree identity for both
  candidate and control through final pre-restore status, with effective kernel
  `kernel_2712.img` and expected TFTP path `da591740/kernel_2712.img`.
- fixed: candidate and control each produced two fresh 86,744-byte TFTP serves
  after power-cycle, so the prior BCM54213PE candidate no-fresh-TFTP blocker is
  not a generic selected-tree publication or TFTP fetch failure for a
  no-MDIO/no-Ethernet sentinel.
- deferred: neither sentinel image emitted its run nonce marker in the bounded
  serial window; fresh firmware NETWORK serial was captured, so the remaining
  boundary is fetched-kernel execution or sentinel serial emission.
- rejected: BCM54213PE register values, Ethernet readiness, link readiness,
  GPIO32/PHY reset ownership, BMCR writes, Broadcom shadow/MMD/aux access,
  interrupt ownership, broad PHY/MAC configuration, packet I/O, networking,
  SSH, Phase 12.2, and phase transition remain rejected.
- removed: generated boot archives were already removed after upload by the
  hardware-proof task; retained hashes and lab evidence are the durable
  artifacts.
- not-an-issue: the candidate and control sentinel archives shared the same
  86,744-byte kernel size and selected tree while differing by run nonce and
  sentinel report payload; closeout relies on retained task evidence rather
  than a new archive publication.

## Reconciliation

The preceding read-only preflight hardware proof closed same-shaped
MII_CTRL1000/MII_STAT1000 hardware retries because the candidate rerun staged a
selected tree but produced no fresh TFTP events or serial output after
power-cycle. The boot-transport sentinel was deliberately different: it removed
BCM54213PE register-read code, MDIO/MAN target construction, MACB target facts,
GPIO32/PHY facts, and packet/networking intent while changing only boot
identity/report payload.

That discriminator narrows the blocker. Candidate and control sentinel runs
show selected-tree publication, lab identity, TFTP serving, and restore
mechanics can work after power-cycle for a no-MDIO/no-Ethernet sentinel. The
absence of sentinel nonce markers means this does not prove kernel-level
sentinel report execution, BCM54213PE register access, link readiness, or
packet/networking behavior. The current boundary is therefore not
selected-tree/TFTP transport; it is fetched-kernel execution or sentinel
serial-emission visibility for this sentinel shape.

## Frontier

Closed frontier:
bcm54213pe-boot-transport-sentinel-frontier-closed-kernel-serial-boundary.

Accepted: local/static no-Ethernet/no-MDIO sentinel core shape, selected-tree
publication through lab status, two fresh candidate/control 86,744-byte TFTP
serves after power-cycle, final pre-restore selected-tree identity, firmware
NETWORK serial presence, restore proof, and the conclusion that selected-tree
TFTP transport is not the generic blocker for the sentinel pair.

Deferred: fetched-kernel execution or sentinel serial-emission visibility,
candidate raw/decoded MII_CTRL1000 and MII_STAT1000 values, and any distinct
follow-up discriminator selected by supervisor planning.

Not accepted: BCM54213PE register values, Ethernet driver readiness, link
readiness, GPIO32/PHY reset ownership, BMCR/autoneg retry, Broadcom
shadow/MMD/aux access, interrupt ownership, broad PHY/MAC configuration, packet
I/O, networking, SSH, Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. The next
decision must select one explicit Phase 12.1 boundary, such as a fetched-kernel
execution or serial-emission discriminator, a separate source/static contract,
or an explicit pause.

No dependency-gated queued task remains mechanically unblocked after this
closeout. The closeout does not authorize a register-read retry, write/restore
task, networking task, SSH work, Phase 12.2 work, or phase transition.

## Evidence

- Sentinel core task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-core.md.
- Sentinel core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-core/classification.json.
- Sentinel core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-core/evidence-map.json.
- Serialized Pi 5 proof task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof.md.
- Serialized Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/classification.json.
- Serialized Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/evidence-map.json.
- Serialized Pi 5 capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/capture-summary.json.
- Boot-transport sentinel closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-closeout/classification.json.
- Boot-transport sentinel closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: sentinel core task/classification/evidence
  map, Pi 5 proof task/classification/evidence map, capture summary, docs,
  roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout reconciles sentinel core and hardware evidence, including blocked or
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
