# Phase 12.1 RP1 Ethernet BCM54213PE Link Recovery Source Checkpoint

Task id: phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint-20260616

Status: accepted

Classification:
bcm54213pe-bmcr-autoneg-restart-contract-selected

Evidence level: static/source/evidence inspection, task-owned JSON evidence,
docs build, and diff checks. No Pi 5 hardware run, boot archive publication,
lab mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
GPIO32 event clear/reset recovery, BMCR/autoneg write, Broadcom shadow/MMD/aux
access, interrupt ownership, PHY/MAC configuration, packet I/O, networking,
SSH, Phase 12.2, or phase transition was performed.

## Goal

Select the next feature-led link-recovery boundary after accepted BCM54213PE
read-only MII_CTRL1000/MII_STAT1000 visibility, using retained source and
evidence before authorizing any write or hardware retry.

## Scope Performed

- Reconciled the accepted BCM54213PE read-only v2 frontier:
  PHY1 MII_CTRL1000 0x09 raw 0x0200 valid and PHY1 MII_STAT1000 0x0a raw
  0x0000 valid under selected-tree, same-power-cycle TFTP, cursor-nonce serial
  freshness, final identity, restore, and paired no-MDIO/no-Ethernet control
  evidence.
- Reconciled retained link-not-ready evidence: BMCR 0x1000,
  BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006,
  BMSR link false, BMSR autoneg-complete false, ANLPAR nonzero false, and
  MACB_NSR_LINK false.
- Reconciled GPIO32 blockers: write/restore v2 stopped before GPIO/RIO/pad
  writes with event bits 0x0ab00000, and event clear preserved
  CTRL/RIO/pad invariants while leaving persistent or firmware-owned event
  bits 0x08800000.
- Inspected retained Linux source facts for BMCR autoneg restart,
  BCM54213PE/Broadcom config_init selector/MMD/AUX paths, interrupt paths, and
  suspend/resume lifecycle paths.
- Selected exactly one next boundary: the dependency-gated
  phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616 local
  proof-core task.

## Non-Goals

No runtime code change, no Pi 5 hardware run, no boot archive publication, no
hardwareTestLock acquisition, no lab mutation, no GPIO32 event clear/reset
recovery, no BMCR/autoneg write, no Broadcom shadow/MMD/AUX write, no interrupt
ownership, no PHY/MAC configuration, no packet I/O, no networking, no sockets,
no SSH, no Phase 12.2 work, and no phase transition.

## Findings

- fixed: accepted read-only MII_CTRL1000/MII_STAT1000 values are preserved as
  context only; MII_CTRL1000 advertises 1000baseT full-duplex capability while
  MII_STAT1000 reports local/remote receiver OK false and no 1000baseT link
  partner capability.
- fixed: retained BMCR/BMSR/ANAR/ANLPAR/MACB_NSR evidence still classifies the
  current state as link-not-ready and autoneg incomplete, so packet I/O and
  network stack work remain blocked.
- fixed: retained Linux source facts keep BMCR autoneg restart as the thinnest
  real feature path: Linux genphy_restart_aneg() sets BMCR_ANENABLE and
  BMCR_ANRESTART, and genphy_update_link() treats restart as a recovery attempt
  whose result still needs later status proof.
- fixed: selected the BMCR/autoneg restart local proof-core as the only
  mechanically unblocked follow-up, with a narrow write/read surface and paired
  no-MDIO/no-Ethernet control.
- deferred: the serialized Pi 5 proof, hardware lock, archive publication,
  runtime BMCR write evidence, restore proof, and closeout remain separate
  queued/dependency-gated tasks.
- deferred: GPIO32 reset recovery, Broadcom shadow/MMD/AUX writes, interrupt
  mask/control, RGMII delay, APD, EEE, suspend/resume, broad PHY/MAC
  configuration, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition require future supervisor-planned work.
- rejected: exact BCM54213PE identity and read-only gigabit registers do not
  prove GPIO32 event bits harmless, do not authorize Broadcom selector/MMD/AUX
  access, and do not accept link readiness or Ethernet driver readiness.
- not-an-issue: no hardwareTestLock was acquired because this checkpoint is
  source/docs/evidence only.
- removed: no stale source, helper, task, or evidence file was removed.

## Selected Boundary

Selected classification:
bcm54213pe-bmcr-autoneg-restart-contract-selected.

Selected follow-up:
phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616.

The follow-up may only construct a local/static candidate and paired control
for this exact surface:

~~~text
target: corrected-target Clause 22 PHY1 BMCR 0x00
pre-read context: BMCR, BMSR, ANAR, ANLPAR, MII_CTRL1000, MII_STAT1000,
  passive MACB_NSR_LINK
preconditions: corrected NCR.MPE set, BMCR_ISOLATE clear, no GPIO32/reset
  ownership required
allowed write intent: exactly one PHY1 BMCR write of
  pre_bmcr | BMCR_ANENABLE | BMCR_ANRESTART
post-read context: BMCR readback, double-sampled BMSR, ANAR, ANLPAR,
  MII_CTRL1000, MII_STAT1000, passive MACB_NSR_LINK
paired control: same report surface with no MDIO/MAN/MACB/GPIO32/PHY/RP1
  Ethernet target construction and no volatile Ethernet access intent
~~~

That core task must fail closed for target drift, extra writes, selector
access, GPIO32/reset action, interrupt ownership, broad PHY/MAC configuration,
link-ready acceptance, packet/networking/SSH/Phase 12.2 claims, and phase
transition claims.

## Reconciliation

The accepted v2 read-only preflight proved that the selected Pi 5 can freshly
read BCM54213PE gigabit control/status registers after the serial-freshness
repair. It did not change the link-not-ready frontier: BMSR and MACB_NSR still
show link clear, ANLPAR and MII_STAT1000 still show no partner capability
visibility, and MII_CTRL1000 only confirms local 1000baseT full-duplex
advertisement.

The feature-led next step is therefore not another read-only sample and not
networking. The smallest useful feature path is to model one guarded BMCR
autoneg restart proof core that can later answer whether a single standard
restart attempt changes the PHY status, while keeping Broadcom-specific
configuration, GPIO32 reset, interrupts, and packet I/O out of scope.

The prior PHY1 autoneg restart work remains useful source context but did not
produce accepted runtime BMCR write evidence because its hardware proof closed
on capture/staging blockers. This checkpoint does not revive that hardware run
or authorize a same-shaped retry. It selects only a new BCM54213PE-scoped local
core with the accepted v2 freshness/read-only evidence folded into the contract.

## Rejected Claims And Retained Risks

Rejected claims:

- GPIO32 reset ownership;
- GPIO32 event-clear retry;
- ETH_RST_N assertion or deassertion;
- Broadcom shadow/MMD/AUX writes or selector access;
- interrupt ownership or interrupt completion;
- broad PHY configuration;
- MACB configuration;
- link forcing;
- link readiness or Ethernet driver readiness;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- A BMCR autoneg restart may still classify link-not-ready if the remaining
  blocker is physical, strap, GPIO32 reset, Broadcom-specific configuration, or
  partner-side state.
- Broadcom config_init paths include selector/MMD/AUX writes, interrupt
  register writes, APD/EEE writes, RGMII delay writes, and suspend/resume
  lifecycle writes; none are selected here.
- GPIO32 persistent event state remains an independent blocker for any
  reset-based recovery.
- Packet I/O and network-stack work remain blocked until link and lower-level
  ownership prerequisites are separately accepted.

## Evidence

- Accepted v2 closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-closeout.md.
- Accepted v2 closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-closeout/classification.json.
- Accepted v2 Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/classification.json.
- Retained BCM54213PE config-init source contract:
  tasks/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract.md.
- Retained PHY1 autoneg restart source contract:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-source-contract.md.
- Retained autoneg restart closeout blocker:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-closeout.md.
- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint/evidence-map.json.

## Validation

- static/source/evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- cargo fmt/tests: not run because runtime Rust and scripts were not changed.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616 on the next
worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and projects/talos
remains clean. Do not start the Pi 5 proof, hardware action, GPIO32/reset
recovery, Broadcom shadow/MMD/AUX access, interrupt ownership, packet I/O,
networking, SSH, Phase 12.2, or a phase transition before that core is accepted
and committed.
