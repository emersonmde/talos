# Phase 12.1 RP1 Ethernet BCM54213PE Link-Not-Ready Discriminator Selection

Task id: phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection-20260618

Status: accepted

Classification:
bcm54213pe-link-not-ready-master-mode-gate-source-contract-selected

Evidence level: static/source/task evidence review, task-owned JSON evidence,
docs build, and diff checks. No hardware run, lab mutation, boot archive
publication, runtime Ethernet behavior, GPIO32/PHY reset action,
packet I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Reconcile the post-generated-root BCM54213PE timeout/link-not-ready frontier
against retained source and task evidence, then select exactly one next
source-backed discriminator only if it is distinct from same-shaped timeout,
status, restart, polling, or capture retries.

## Scope Performed

- Reviewed the accepted post-generated-root pause closeout and the retained
  BCM54213PE timeout/link-not-ready frontier.
- Rechecked the accepted BCM54213PE config-init source contract, read-only
  preflight, BMCR/autoneg restart, convergence, RGMII delay, corrected
  TX-order proof, post-TX-order checkpoint, GPIO32 reset blocker, PHY
  power/strap checkpoint, and generated-root resumption closeout.
- Classified the remaining candidate families: GPIO32/ETH_RST_N reset,
  MII_CTRL1000 master-mode writes, interrupt/APD/EEE/lifecycle/MAC-phylink
  work, same-shaped retry classes, and packet/networking/SSH work.
- Selected one local/static source-contract follow-up for the MII_CTRL1000
  master-mode gate. The next task is not a hardware proof; it must decide the
  exact source contract before any Pi 5 action can be promoted.

## Findings

- fixed: generated-root command-input success closes the non-Ethernet detour,
  but it does not change the BCM54213PE terminal facts. Link-ready and
  autoneg-complete remain unaccepted after the accepted TX-order delay path,
  exactly one BMCR restart, and bounded convergence polling.
- fixed: same-shaped BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/
  MACB_NSR polling, another bare BMCR restart, convergence wait tuning, and
  marker/capture-only retry are rejected because they repeat accepted
  timeout/link-not-ready evidence.
- blocked: GPIO32 / ETH_RST_N reset ownership remains behind the accepted
  persistent-or-firmware-owned GPIO32 event-state blocker. This task performs
  no reset action and does not relax the no-write boundary.
- fixed: MII_CTRL1000 master-mode is source-backed and distinct enough for a
  local/static source-contract task. Linux gates the BCM54210E/BCM54213PE write
  on PHY_BRCM_EN_MASTER_MODE, reads PHY1 MII_CTRL1000, ORs
  CTL1000_AS_MASTER and CTL1000_ENABLE_MASTER, and writes MII_CTRL1000 back.
  The selected next task must prove whether Talos can select that gate and
  model the write/restore and post-status contract safely.
- deferred: Broadcom interrupt ISR/IMR/ECR, APD/EEE, LED/WOL, suspend/resume,
  and MAC/phylink work remain broader lifecycle or interrupt ownership work.
  They are not selected by this task.
- rejected: packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain blocked until link-ready/autoneg-complete and lower-level
  ownership are accepted by explicit evidence.
- removed: no source, helper, task, docs, or evidence file was removed.
- not-an-issue: no hardware lock, boot publication, or inconclusive-run triage
  was needed for this static/source selection task.

## Selected Discriminator

Selected discriminator:
bcm54213pe-phy1-mii-ctrl1000-master-mode-gate-source-contract.

Selected next task:
phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core-20260618.

Source owners:

- Raspberry Pi Linux rpi-6.12.y Broadcom PHY driver:
  bcm54xx_config_init -> bcm54213pe_config_init -> bcm54210e_config_init.
- BCM54210E/BCM54213PE MII_CTRL1000 master-mode gate:
  PHY_BRCM_EN_MASTER_MODE, CTL1000_AS_MASTER, and CTL1000_ENABLE_MASTER.
- Existing accepted Talos source evidence for BCM54213PE exact ID, RGMII-ID
  delay handling, read-only MII_CTRL1000/MII_STAT1000 visibility, BMCR restart,
  and TX-order convergence ending link-not-ready.

Allowed operations for the next task:

- local/static source-contract work only;
- encode the PHY1 MII_CTRL1000 master-mode gate as a candidate contract;
- name the candidate/control report surfaces, source preconditions, write
  intent, restore or rollback expectations, and terminal classifications;
- reject unselected GPIO32 reset, interrupt, APD/EEE/lifecycle, MAC/phylink,
  packet, networking, SSH, Phase 12.2, and phase-transition claims.

Forbidden operations from this selection:

- no Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
  lab mutation, TFTP/serial capture, or power-cycle;
- no runtime PHY register write, GPIO32/RIO/pad write, ETH_RST_N assertion or
  deassertion, interrupt acknowledgement/configuration, APD/EEE/lifecycle
  write, MACB/phylink configuration, packet I/O, networking, sockets, SSH,
  Phase 12.2, or phase transition.

Expected terminal classifications for the next source/core task:

- bcm54213pe-mii-ctrl1000-master-mode-source-contract-core-local-static;
- bcm54213pe-mii-ctrl1000-master-mode-source-contract-paused;
- bcm54213pe-mii-ctrl1000-master-mode-source-contract-blocked;
- no-mdio-no-ethernet-bcm54213pe-mii-ctrl1000-master-mode-control.

Why this is distinct:

MII_CTRL1000 master-mode is not another passive status sample, not another
BMCR restart, not a convergence wait tweak, and not a capture retry. It is the
only retained BCM54213PE config_init write family that is narrow enough to
contract locally before hardware while remaining directly adjacent to gigabit
master/slave negotiation state. The next task still has to prove the source
contract and may pause rather than select hardware.

## Candidate Disposition Map

- GPIO32/ETH_RST_N reset ownership: blocked. The accepted event-clear proof
  retained persistent-or-firmware-owned event bits, so reset action remains
  unsafe without a new source-backed ownership discriminator.
- MII_CTRL1000 master-mode gate: fixed/selected. It is source-backed by the
  BCM54210E/BCM54213PE config_init path and selected only for local/static
  source-contract work.
- Interrupt ISR/IMR/ECR work: deferred. ISR reads may acknowledge interrupts
  and IMR/ECR writes are interrupt ownership.
- APD/EEE/LED/WOL/suspend-resume lifecycle work: deferred. These are broader
  lifecycle or power-management paths requiring separate source contracts.
- MAC/phylink work: deferred. It crosses into MAC configuration and link
  management, outside the current PHY source-contract step.
- Same-shaped status/restart/poll/capture retry: rejected. It repeats accepted
  timeout/link-not-ready evidence.
- Packet I/O, networking, sockets, SSH, Phase 12.2, phase transition:
  rejected until lower-level link and ownership evidence exists.

## Evidence

- Post-generated-root closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-post-generated-root-link-not-ready-pause-closeout.md.
- BCM54213PE link-not-ready pause closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-not-ready-frontier-pause-closeout.md.
- BCM54213PE config-init source contract:
  tasks/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract.md.
- Retained BCM54213PE config excerpt and checksum:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-broadcom-bcm54213pe-config-contract-excerpt.txt
  with sha256 c745a529512ef69185ba7f2712079e66b5d9efbdb963eb194ec91291fc297aa6.
- Retained Broadcom register excerpt and checksum:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-brcmphy-register-contract-excerpt.txt
  with sha256 b4a5d4d3272d53f0898179307e66ae50947fe94b25b72193868e7118a8d7adf6.
- Retained MII/read-status context:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/.
- Task classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection/evidence-map.json.

## Acceptance Check

- Candidate families reviewed with dispositions: satisfied.
- Exactly one selected_discriminator and selected_next_task are supplied:
  satisfied with bcm54213pe-phy1-mii-ctrl1000-master-mode-gate-source-contract
  and phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core-20260618.
- Selected discriminator names source owners, allowed operations, forbidden
  operations, terminal classifications, and why it is not same-shaped retry:
  satisfied.
- GPIO32 persistent-event-state, no-write reset safety, no packet/networking/
  SSH, and no Phase 12.2/phase-transition boundaries remain explicit:
  satisfied.
- No hardware, lab mutation, runtime Ethernet behavior, boot publication,
  packet I/O, networking, SSH, or phase transition performed: satisfied.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core-20260618
on the next worker wake if dependencies remain satisfied. Do not promote
hardware, GPIO32/PHY reset, interrupt/APD/EEE/lifecycle, packet I/O,
networking, SSH, Phase 12.2, or phase-transition work from this selection.
