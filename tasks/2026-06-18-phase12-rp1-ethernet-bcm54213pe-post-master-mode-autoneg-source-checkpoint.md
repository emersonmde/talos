# Phase 12.1 RP1 Ethernet BCM54213PE Post-Master-Mode Autoneg Source Checkpoint

Task id: phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-source-checkpoint-20260618

Status: accepted

Classification:
bcm54213pe-post-master-mode-autoneg-no-distinct-source-backed-discriminator-pause

Evidence level: static/source/task evidence review, rg/source inspection notes,
task-owned JSON evidence, docs build, and diff checks. No runtime
implementation, Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, GPIO32/PHY reset action, interrupt
masking/acknowledgement, APD/EEE/lifecycle write, MACB configuration, packet
I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Reconcile the accepted BCM54213PE MII_CTRL1000 master-mode write/readback plus
one BMCR autoneg restart proof, which still ended timeout/link-not-ready, with
the retained BCM54213PE, GPIO32, interrupt, APD/EEE/lifecycle, and MAC/phylink
source evidence. Select exactly one next source-grounded discriminator only if
it is feature-led and qualitatively distinct from accepted timeout/status/
autoneg polling; otherwise pause Phase 12.1 for supervisor planning.

## Scope Performed

- Reviewed the accepted master-mode autoneg closeout and Pi 5 proof evidence.
- Reconciled the current terminal hardware facts: PHY1 MII_CTRL1000 pre-read
  0x0200, write/readback 0x1a00, exactly one BMCR autoneg restart write,
  terminal BMSR link false, BMSR autoneg-complete false, ANLPAR 0x0000,
  MII_STAT1000 0x0000, and MACB_NSR_LINK false.
- Inspected retained Raspberry Pi Linux BCM54213PE/config-init source evidence
  and Talos source contracts for GPIO32/ETH_RST_N, Broadcom shadow/AUX RGMII
  delay, MII_CTRL1000 master-mode, APD/powerdown, EEE, interrupt ISR/IMR/ECR,
  suspend/resume lifecycle, and MAC/phylink boundaries.
- Classified each candidate family with disposition.
- Recorded selected_discriminator=null, selected_next_task=null, and
  planningNeeded=true because no remaining candidate is mechanically ready
  inside this checkpoint without new supervisor scope.

## Findings

- fixed: accepted master-mode plus autoneg evidence is preserved as the current
  Phase 12.1 frontier. MII_CTRL1000 master-mode write/readback and one BMCR
  autoneg restart are hardware-visible, but terminal link-ready and
  autoneg-complete remain rejected.
- blocked: GPIO32 / ETH_RST_N reset ownership remains controlled by the
  accepted persistent-or-firmware-owned GPIO32 event-state blocker. This task
  does not weaken the no-reset/no-output boundary or select a GPIO32 write/
  restore retry.
- rejected: same-shaped BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/
  MACB_NSR polling, another bare BMCR restart, another master-mode autoneg
  retry, convergence wait tuning, and marker/capture-only retry would repeat
  accepted timeout/link-not-ready evidence.
- rejected: Broadcom RGMII delay/TX-order work is already closed for this
  frontier. The accepted prior proof exercised RX delay, TX selected-read/
  readback with GTXCLK_EN already set, exactly one BMCR restart, and bounded
  convergence, still ending link-not-ready.
- deferred: APD/powerdown and EEE are source-backed but are PHY lifecycle and
  MMD/shadow configuration surfaces. A future task would need explicit source
  ownership, side-effect, write/restore, and terminal classification criteria
  before implementation; this checkpoint does not select them.
- deferred: interrupt ISR/IMR/ECR work is not selected. ISR reads may
  acknowledge pending interrupts, while IMR/ECR writes are interrupt ownership
  rather than direct link readiness.
- deferred: suspend/resume, BMCR powerdown, soft reset, LED/WOL/PTP, and other
  Broadcom lifecycle paths are broader than a thin link-not-ready
  discriminator and need new supervisor-planned scope.
- deferred: MAC/phylink work crosses into MAC configuration and driver
  readiness. It remains blocked by unaccepted link readiness and lower-level
  ownership.
- rejected: the link-ready packet-readiness checkpoint remains dependency-gated
  because BMSR link, BMSR autoneg-complete, and MACB_NSR_LINK are all still
  false.
- removed: no source, helper, docs, task, or evidence files were removed.
- not-an-issue: no hardware lock or inconclusive-run triage was needed because
  this checkpoint is source/task/evidence-only.

## Candidate Family Dispositions

| Candidate family | Disposition | Reason |
| --- | --- | --- |
| MII_CTRL1000 master-mode plus BMCR autoneg | fixed | Accepted input frontier, but terminal link-ready/autoneg-complete stayed false. |
| GPIO32 / ETH_RST_N reset ownership | blocked | Persistent-or-firmware-owned GPIO32 event-state still controls reset risk. |
| Same-shaped status/autoneg/convergence retry | rejected | Repeats accepted timeout/link-not-ready evidence. |
| Broadcom RGMII delay / TX-order | rejected | Already exercised and closed as timeout/link-not-ready. |
| APD/powerdown | deferred | Source-backed lifecycle/shadow write surface needing separate ownership and restore contract. |
| EEE / Clause 45 MMD | deferred | Source-backed MMD configuration surface; not a direct accepted link discriminator. |
| Interrupt ISR/IMR/ECR | deferred | ISR read may acknowledge; IMR/ECR writes require interrupt ownership. |
| Suspend/resume / BMCR powerdown / soft reset | deferred | Broad lifecycle path, not a bounded discriminator in this task. |
| MAC/phylink | deferred | Crosses into MAC configuration and driver readiness. |
| Link-ready packet-readiness checkpoint | rejected | Dependency gate remains unsatisfied. |
| Packet I/O, networking, SSH, Phase 12.2, phase transition | rejected | Link readiness and lower-level ownership are unaccepted. |

## Decision

Selected discriminator: null.

Selected next task: null.

Planning needed: true.

No mechanically ready, source-backed, qualitatively distinct feature
discriminator remains in this task after accepted MII_CTRL1000 master-mode
write/readback plus one BMCR autoneg restart still ended link-not-ready.
Supervisor planning is required before the queued selected-discriminator core,
any hardware proof, GPIO32/PHY reset, interrupt/APD/EEE/lifecycle work,
MAC/phylink work, packet I/O, networking, SSH, Phase 12.2, or phase-transition
work.

## Evidence

- Master-mode autoneg closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout.md.
- Master-mode autoneg closeout classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout/classification.json.
- Master-mode autoneg Pi 5 proof classification and hardware summary:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/classification.json and
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/hardware-run-summary.json.
- BCM54213PE config-init source contract:
  tasks/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract.md.
- BCM54213PE config-init source excerpts:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/.
- Prior post-TX-order source checkpoint:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-txorder-link-not-ready-source-checkpoint.md.
- Task classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-source-checkpoint/classification.json.
- Source inspection notes:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-source-checkpoint/source-inspection.json.
- Task evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-source-checkpoint/evidence-map.json.

## Acceptance Check

- Reviewed candidate families and recorded findings with dispositions:
  satisfied.
- Classification records selected_discriminator=null, selected_next_task=null,
  planningNeeded=true, and a concise pause rationale: satisfied.
- GPIO32 persistent-event-state, no packet/networking/SSH, no Phase
  12.2/phase-transition, and link-ready packet-readiness gates remain explicit:
  satisfied.
- No hardware, lab mutation, runtime Ethernet behavior, boot publication,
  packet I/O, networking, SSH, or phase transition was performed: satisfied.

## Validation

- static/source/task evidence review: pass.
- rg/source inspection notes: pass, recorded in task-owned JSON.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass because docs/src files were touched.
- git diff --cached --check: pass before commit.

## Next Action

Stop for supervisor planning. Do not promote
phase12-rp1-ethernet-bcm54213pe-post-master-mode-selected-discriminator-core-20260618
unless a future accepted task supplies selected_discriminator non-null and
selects that exact core. Do not promote hardware, GPIO32/PHY reset,
interrupt/APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, SSH, Phase
12.2, or phase-transition work from this checkpoint.
