# Phase 12.2 RP1 Ethernet Link-Ready Discriminator Source Contract

Task id: phase12-rp1-ethernet-link-ready-discriminator-source-contract-20260621

Status: accepted

Classification:
phase12-rp1-ethernet-link-ready-discriminator-source-contract-blocked-no-defensible-discriminator

Evidence level: static source/task/docs/evidence review, task-owned JSON
evidence, docs build, and diff checks. No runtime implementation, hardware
run, hardwareTestLock acquisition, lab mutation, boot publication, power-cycle,
live packet I/O, ping reachability, hardware reachability, SSH, UDP/raw
sockets, libc/std wrappers, public ABI/POSIX/Linux compatibility, broad socket
expansion, or phase transition was performed.

## Goal

Define the next qualitatively distinct RP1 Ethernet link-ready discriminator
needed before live packet I/O or ping reachability work, or block with the
missing prerequisite if retained source evidence does not support one.

## Scope Performed

- Reviewed the accepted post-master-mode autoneg pause closeout and its
  source checkpoint, including the terminal link-not-ready facts: MII_CTRL1000
  master-mode write/readback and one BMCR autoneg restart were hardware
  visible, while BMSR link, BMSR autoneg-complete, ANLPAR, MII_STAT1000, and
  MACB_NSR_LINK remained not ready.
- Reviewed the retained BCM54213PE config-init, Broadcom PHY register,
  APD/EEE, interrupt, lifecycle, and MAC/phylink source evidence that the
  accepted checkpoints already classified.
- Reviewed the accepted driver packet adapter closeout and confirmed it
  advances the host/QEMU-substitute packet substrate only; it does not add a
  new PHY, MDIO, GPIO32, interrupt, MACB, or live hardware link fact.
- Reconciled candidate families against the task's requirement to avoid
  same-shaped BMCR restart, status/autoneg polling, marker-only capture, and
  wait-constant retries.

## Findings

- fixed: the source contract now records that no defensible link-ready
  discriminator is selected from the accepted source/evidence set.
- fixed: the accepted input anchors are explicit: post-master-mode autoneg
  pause closeout, post-master-mode source checkpoint, retained Linux
  BCM54213PE source excerpts, current rp1_ethernet guardrails, and driver
  packet adapter closeout.
- blocked: GPIO32 / ETH_RST_N reset ownership remains blocked by the accepted
  persistent-or-firmware-owned GPIO32 event-state evidence.
- rejected: same-shaped BMCR restart, status/autoneg/convergence polling,
  wait-constant tuning, and marker-only capture retries remain non-progress
  because they repeat accepted timeout/link-not-ready evidence.
- rejected: Broadcom RGMII delay/TX-order and MII_CTRL1000 master-mode plus
  BMCR restart are already exercised and closed as link-not-ready.
- deferred: APD/auto-power-down, EEE/MMD, WOL/IDDQ, suspend/resume,
  BMCR powerdown recovery, soft reset, and broader config_init replay are
  lifecycle write/restore surfaces; they require explicit ownership, side
  effects, restore behavior, and terminal classification before implementation.
- rejected: MII_BCM54XX_ISR and WOL status reads are not selected because
  retained Linux source classifies them as side-effecting acknowledgement
  surfaces; bcm_phy_handle_interrupt is interrupt state-machine ownership.
- deferred: IMR/ECR interrupt mask/control paths require interrupt ownership
  and paired restore rules before they can be considered.
- deferred: MAC/phylink remains a broader future boundary because link-ready,
  packet-readiness, and MAC ownership are still unaccepted.
- not-an-issue: the DriverPacketAdapter closeout is valuable future plumbing
  but does not change the physical link blocker because its accepted evidence
  is source/unit plus host/QEMU-substitute only.
- removed: no source, helper, docs, task, or evidence files were removed.

## Decision

Selected discriminator: null.

Selected next task: null.

Planning needed: true.

Concrete missing prerequisite:

- either new source evidence that isolates a narrow, side-effect-bounded
  BCM54213PE/RP1 link-ready discriminator not already covered by the accepted
  RGMII delay, MII_CTRL1000 master-mode, BMCR restart, status/autoneg polling,
  GPIO32 reset blocker, APD/EEE/lifecycle, interrupt, or MAC/phylink
  classifications;
- or explicit supervisor/human strategy selection authorizing a broader
  ownership slice, such as APD/EEE/lifecycle write/restore, interrupt
  mask/control, GPIO32 reset recovery, or MAC/phylink configuration, with
  source contract, restore rules, terminal classifications, and hardware gates.

The queued core/proof/closeout chain is therefore not mechanically unblocked
by this task. Promoting it without the missing prerequisite would reopen an
old proof chain or broaden hardware ownership without accepted source scope.

## Rejected Claims

This task rejects live packet I/O, ping reachability, hardware reachability,
SSH, UDP/raw sockets, libc/std wrappers, POSIX/Linux compatibility, public
stable ABI acceptance, broad socket expansion, and phase transition. It also
rejects any claim that the host/QEMU-substitute DriverPacketAdapter evidence
proves RP1/GEM RX/TX coupling or Pi 5 link readiness.

## Evidence

- Post-master-mode autoneg pause closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-pause-closeout.md.
- Post-master-mode source checkpoint:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-source-checkpoint.md.
- Low-power/lifecycle source checkpoint:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint.md.
- Retained Linux source evidence:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/.
- Driver packet adapter closeout:
  tasks/2026-06-21-phase12-network-driver-packet-adapter-closeout.md.
- Task classification:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract/evidence-map.json.

## Validation

- static source/task/docs/evidence review: pass.
- git diff --check: pass.
- mdbook build: pass because docs/src files were touched.
- git diff --cached --check: pass before commit.

## Next Action

Stop for supervisor planning. Do not promote
phase12-rp1-ethernet-link-ready-discriminator-core-20260621,
phase12-rp1-ethernet-link-ready-discriminator-pi5-proof-20260621, live packet
I/O, ping reachability, hardware reachability, SSH, UDP/raw sockets, libc/std
socket wrappers, public ABI/POSIX/Linux compatibility, broad socket expansion,
or phase transition from this task.
