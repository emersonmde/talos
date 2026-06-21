# Phase 12.1 RP1 Ethernet BCM54213PE Lifecycle Ownership Source Core

Task id: phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core-20260621

Status: accepted

Classification:
bcm54213pe-lifecycle-ownership-powerdown-exit-source-core-local-static

Evidence level: static source/task/docs/evidence review, focused Rust unit
tests, task-owned JSON evidence, docs build, and diff checks. No
hardwareTestLock acquisition, boot archive publication, lab mutation,
power-cycle, Pi 5 run, GPIO32/PHY reset action, interrupt ownership,
MAC/phylink configuration, live packet I/O, ping/hardware reachability, SSH,
UDP/raw sockets, libc/std wrappers, public ABI/POSIX/Linux compatibility,
broad socket expansion, or phase transition was performed.

## Goal

Select and implement the smallest source-backed local/static ownership core for
a broader BCM54213PE low-power/lifecycle recovery sequence after the accepted
link-ready source contract exhausted narrow discriminators.

## Scope Performed

- Reviewed the accepted link-ready discriminator blocker, the low-power/
  lifecycle source checkpoint, retained BCM54213PE/Broadcom Linux source
  excerpts, and current rp1_ethernet guardrails.
- Selected exactly one lifecycle ownership sequence:
  bcm54213pe-phy1-bmcr-powerdown-exit-gate.
- Added local/static rp1_ethernet evidence and validators for the selected
  sequence.
- Recorded exact register ownership, read-before-write snapshot rules, write
  mask/value semantics, ordering, forbidden operations, restore rules, paired
  control shape, and terminal classifications.
- Preserved the accepted hardware-visible frontier: MII_CTRL1000 master-mode
  write/readback and one BMCR autoneg restart are accepted; link-ready,
  autoneg-complete, packet-readiness, and live RX/TX remain unaccepted.

## Findings

- fixed: a concrete broader lifecycle ownership sequence is now represented by
  local/static source evidence instead of a planning note only.
- fixed: the selected sequence is source-backed by Linux bcm54xx_resume /
  genphy_resume semantics: clear BMCR_PDOWN before further configuration and
  wait 40us after powerdown exit.
- fixed: the candidate is fail-closed: it pre-reads BMCR, writes only if
  BMCR_PDOWN is set, clears only bit 11, preserves all other BMCR bits, and
  post-samples BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000 plus passive
  MACB_NSR_LINK context.
- fixed: the paired control constructs no MDIO, MAN, MACB, GPIO32/PHY,
  interrupt, packet, networking, or SSH targets.
- rejected: APD shadow writes, EEE/MMD writes, IDDQ/TOP_MISC writes, soft
  reset without accepted IDDQ prerequisite, interrupt ISR/IMR/ECR access,
  broad config_init replay, GPIO32 reset, MAC/phylink ownership, live packet
  I/O, reachability, SSH, broad socket expansion, and phase transition are not
  selected.
- rejected: local/static evidence does not accept link-ready,
  autoneg-complete, packet-readiness, or live RX/TX.
- deferred: serialized Pi 5 proof belongs to
  phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621.
- removed: no source, helper, docs, task, or evidence files were removed.
- not-an-issue: accepted BMCR 0x1200 already has PDOWN clear, so the later
  proof may classify no-change-link-not-ready without issuing a write.

## Accepted Contract

Selected discriminator:
bcm54213pe-phy1-bmcr-powerdown-exit-gate.

Candidate surface for a later hardware proof:

- retain selected-tree/TFTP/serial/final-identity/restore evidence;
- pre-read PHY1 BMCR 0x00 and retain the full value;
- if and only if BMCR_PDOWN bit 11 is set, write BMCR with only bit 11 cleared
  and all other pre-read bits preserved;
- expected clear mask is 0xf7ff; accepted context pre-BMCR is 0x1200, so the
  context write frame would be 0x50821200 and the expected clear value remains
  0x1200;
- after a PDOWN-clear write, wait at least 40us before post-sampling;
- post-read BMCR, double-sampled BMSR, ANAR, ANLPAR, MII_CTRL1000,
  MII_STAT1000, and passive MACB_NSR_LINK context.

Paired control surface:

- no MDIO target construction;
- no MAN frame construction;
- no MACB target construction;
- no GPIO32/ETH_RST_N/PHY target construction;
- no interrupt, APD, EEE, IDDQ, soft-reset, config_init, packet, networking,
  SSH, Phase 12.2, or phase-transition claim.

Future hardware terminal classifications:

- bcm54213pe-lifecycle-powerdown-exit-link-ready;
- bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready;
- bcm54213pe-lifecycle-powerdown-exit-pdown-clear-sampled-link-not-ready;
- bcm54213pe-lifecycle-powerdown-exit-precondition-blocker;
- bcm54213pe-lifecycle-powerdown-exit-readback-mismatch;
- bcm54213pe-lifecycle-powerdown-exit-capture-blocker;
- no-mdio-no-ethernet-bcm54213pe-lifecycle-ownership-control.

Restore behavior:

- the later Pi 5 proof must acquire hardwareTestLock before publication or
  power action;
- the later Pi 5 proof must retain pre-run snapshot, selected-tree identity,
  TFTP byte evidence, serial freshness, final pre-restore identity, and
  post-restore proof;
- the task must not reassert BMCR_PDOWN as cleanup;
- any need for APD, EEE, IDDQ, soft reset, interrupt, GPIO32 reset, or broad
  config_init replay blocks instead of widening this sequence.

## Rejected Claims And Retained Risks

Rejected claims:

- link-ready, autoneg-complete, or packet-readiness from local/static evidence;
- same-shaped BMCR restart/status/autoneg polling retry;
- APD/EEE/IDDQ/soft-reset/config_init replay as selected hardware scope;
- interrupt ownership;
- GPIO32/ETH_RST_N reset ownership;
- MAC/phylink ownership;
- live packet I/O;
- ping/hardware reachability;
- SSH;
- UDP/raw sockets;
- libc/std wrappers;
- POSIX/Linux compatibility;
- broad socket expansion;
- phase transition.

Retained risks:

- This source core selects a narrow BMCR_PDOWN exit gate; it does not prove
  the PHY is powered, linked, or packet-ready.
- Accepted hardware evidence already showed BMCR 0x1200 with PDOWN clear, so a
  later proof may produce a no-change-link-not-ready result.
- APD, EEE, IDDQ, soft reset, interrupt handling, config_init replay, GPIO32
  reset, MAC/phylink, packet I/O, networking, SSH, and Phase 12.2 remain
  unaccepted.

## Evidence

- Task classification:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core/evidence-map.json.
- Validator output:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core/validator-output.txt.
- Accepted link-ready source blocker:
  tasks/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract.md.
- Accepted low-power/lifecycle source checkpoint:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint.md.
- Retained Linux source excerpts:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/.

## Acceptance Check

- Task record includes findings with dispositions: satisfied.
- Exactly one lifecycle ownership sequence is selected with explicit source
  anchors, invariant, register ownership, read-before-write snapshots, write
  mask/value, ordering, side effects, restore behavior, paired control shape,
  and terminal classifications: satisfied.
- Local/static validators fail closed on unselected candidate families and
  forbidden operations: satisfied.
- selected_next_task records
  phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621:
  satisfied.
- Accepted claims explicitly exclude link-ready, autoneg-complete,
  packet-readiness, live packet I/O, ping/hardware reachability, SSH,
  UDP/raw sockets, libc/std wrappers, POSIX/Linux compatibility, broad socket
  expansion, and phase transition: satisfied.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet
  rp1_ethernet_bcm54213pe_lifecycle_ownership_source_core: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass because docs/src files were touched.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention is inactive, and projects/talos is
clean. Do not start the older generic link-ready core/proof chain, live packet
I/O, reachability, SSH, UDP/raw sockets, libc/std wrappers, public ABI/POSIX/
Linux compatibility, broad socket expansion, or phase-transition work from
this source core.
