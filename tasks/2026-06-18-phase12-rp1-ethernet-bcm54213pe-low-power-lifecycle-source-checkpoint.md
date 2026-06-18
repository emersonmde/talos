# Phase 12.1 RP1 Ethernet BCM54213PE Low-Power Lifecycle Source Checkpoint

Task id: phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint-20260618

Status: accepted

Classification:
bcm54213pe-low-power-lifecycle-no-distinct-source-backed-discriminator-pause

Evidence level: static/source/task evidence inspection, rg/source inspection
notes, task-owned JSON evidence, docs build, and diff checks. No runtime
implementation, Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, GPIO32/PHY reset action, APD/EEE/lifecycle write,
interrupt mask/unmask/acknowledgement action, MAC/phylink work, packet I/O,
networking, sockets, SSH, Phase 12.2, or phase transition was performed.

## Goal

Decide whether a BCM54213PE low-power, EEE, interrupt-context, or lifecycle
surface gives a source-grounded, qualitatively distinct link-not-ready
discriminator after the accepted master-mode/autoneg frontier, or pause Phase
12.1 again with explicit reasons.

## Scope Performed

- Reconciled the accepted post-master-mode/autoneg pause closeout with retained
  Raspberry Pi Linux BCM54213PE, Broadcom PHY library, brcmphy register, APD,
  EEE, interrupt, WOL/IDDQ, suspend/resume, BMCR powerdown, and config_init
  source excerpts.
- Classified each reviewed candidate family by side effect: pure read,
  read-with-side-effect, write/restore, blocked, rejected, or deferred.
- Preserved the accepted input frontier: MII_CTRL1000 master-mode
  write/readback and one BMCR autoneg restart were hardware visible, but BMSR
  link, BMSR autoneg-complete, ANLPAR, MII_STAT1000, and MACB_NSR_LINK remained
  not ready.
- Recorded selected_discriminator=null, selected_next_task=null, and
  planningNeeded=true.
- Updated visible Phase 12 documentation and roadmap entries because this
  source checkpoint is now the latest paused Phase 12.1 frontier.

## Findings

- fixed: the accepted post-master-mode/autoneg frontier is preserved as the
  input state for this source checkpoint.
- deferred: APD/auto-power-down is source-backed but writes SCR3/APD shadow
  state and wake behavior, so it needs lifecycle ownership and restore scope.
- deferred: EEE/Clause 45 MMD control is source-backed but writes feature and
  advertisement state, so it needs ownership and restore scope.
- rejected: MII_BCM54XX_ISR reads are read-with-side-effect because Linux
  documents that they clear pending interrupts.
- deferred: IMR/ECR interrupt mask/control writes require interrupt ownership,
  paired restore, and terminal classification rules.
- rejected: bcm_phy_handle_interrupt is handler/state-machine ownership, not a
  bounded link-not-ready discriminator.
- deferred: WOL/IDDQ, wake IRQ, suspend/resume, BMCR powerdown exit, soft
  reset, and rerun-config_init are lifecycle sequences requiring explicit
  ownership and restore rules.
- rejected: BMCR_PDOWN as a standalone candidate intentionally powers down the
  PHY and does not advance a feature-led link-ready path.
- not-an-issue: the narrow config_init-derived MII_CTRL1000 master-mode ordering
  has already been selected and accepted; broader config_init replay would only
  broaden closed work.
- deferred: MAC/phylink remains future supervisor-planned work because this
  checkpoint accepts no link-ready or MAC ownership evidence.
- rejected: same-shaped BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/
  MACB_NSR polling repeats accepted timeout/link-not-ready evidence.
- removed: no source, helper, task, docs, or evidence files were removed.

## Decision

Selected discriminator: null.

Selected next task: null.

Planning needed: true.

The remaining source-backed BCM54213PE surfaces are not a mechanically safe
next feature discriminator from this task. APD, EEE, WOL/IDDQ, suspend/resume,
BMCR powerdown recovery, and config_init replay are lifecycle write/restore
surfaces. Interrupt status/handler paths either clear pending state or cross
into interrupt ownership. MAC/phylink work crosses a broader ownership boundary,
and pure status/autoneg reads repeat accepted timeout/link-not-ready evidence.

## Evidence

- Task classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint/classification.json.
- Source inspection:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint/source-inspection.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint/evidence-map.json.
- Accepted post-master-mode/autoneg pause closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-pause-closeout.md.
- Retained BCM54213PE source excerpts:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/.
- Retained Broadcom APD/EEE/interrupt source excerpts:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/source/.

## Acceptance Check

- Task record summarizes the accepted input frontier and records
  candidate-family findings with dispositions: satisfied.
- Every reviewed APD/powerdown, EEE/MMD, interrupt-context, WOL/IDDQ,
  suspend/resume, BMCR powerdown/soft reset, and MAC/phylink-adjacent candidate
  is classified by side effect: satisfied.
- No next discriminator is selected; classification JSON records
  selected_discriminator=null, selected_next_task=null, planningNeeded=true, and
  a pause rationale: satisfied.
- GPIO32/PHY reset action, packet I/O, networking, sockets, SSH, Phase 12.2,
  phase transition, and link-ready packet-readiness remain explicitly rejected:
  satisfied.

## Validation

- static/source/task evidence inspection: pass.
- rg/source inspection notes recorded in task-owned JSON: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass because docs/src files were touched.
- git diff --cached --check: pass before commit.

## Next Action

Stop for supervisor planning. Do not promote
phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-selected-core-20260618 or
any hardware, GPIO32/PHY reset, APD/EEE/lifecycle, interrupt, MAC/phylink,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase-transition task from
this checkpoint.
