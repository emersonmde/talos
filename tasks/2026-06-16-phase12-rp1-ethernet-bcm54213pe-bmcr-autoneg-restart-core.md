# Phase 12.1 RP1 Ethernet BCM54213PE BMCR Autoneg Restart Core

Task id: phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616

Status: accepted

Classification:
bcm54213pe-bmcr-autoneg-restart-proof-core-local-static

Evidence level: local/static Rust contract tests, candidate/control
compile-only builds, task-owned JSON evidence, docs build, and diff checks. No
Pi 5 hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, power-cycle, TFTP/serial capture, restore, GPIO32 event
clear/reset recovery, Broadcom shadow/MMD/AUX access, interrupt ownership,
packet I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Implement the smallest local/static proof core for the source-checkpoint
selected BCM54213PE Clause 22 PHY1 BMCR autonegotiation restart boundary, with
a paired no-MDIO/no-Ethernet control and fail-closed drift checks.

## Scope Performed

- Added a BCM54213PE-specific local/static contract in src/rp1_ethernet.rs for
  the exact selected BMCR/autoneg restart surface.
- Added focused tests that accept only candidate construction of PHY1 MDIO/MAN
  facts, passive MACB_NSR_LINK context, exactly one BMCR write frame for
  pre_bmcr | BMCR_ANENABLE | BMCR_ANRESTART, and a control with no
  MDIO/MAN/MACB/GPIO32/PHY/RP1 Ethernet target facts.
- Added candidate/control boot scenarios and compile-only image scripts.
- Rejected target drift, extra PHY writes, Broadcom selector/shadow/MMD/AUX
  access, GPIO32/reset action, interrupt ownership, broad PHY/MAC
  configuration, link-ready acceptance, packet I/O, networking, SSH,
  Phase 12.2, and phase transition claims.
- Selected the serialized Pi 5 proof task
  phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof-20260616 as
  the next dependency-gated task.

## Non-Goals

No hardware run, no lab or boot publication, no runtime acceptance of a BMCR
write, no GPIO32 event clear/reset recovery, no Broadcom selector/MMD/AUX
access, no interrupt ownership, no MAC configuration, no DMA/descriptors, no
packet I/O, no networking, no sockets, no SSH, no Phase 12.2 work, and no
phase transition.

## Findings

- fixed: candidate/control local/static surfaces now encode the checkpoint
  selected BMCR/autoneg restart boundary under the BCM54213PE task lineage.
- fixed: the exact BMCR write frame is pinned to 0x50821200, derived from
  Clause 22 PHY1 BMCR write prefix 0x50820000 and accepted pre-BMCR 0x1000
  plus BMCR_ANRESTART 0x0200.
- fixed: focused tests fail closed on BMCR write drift, control target facts,
  extra PHY writes, selector access, GPIO32 reset action, link-ready
  acceptance, networking, and phase transition.
- fixed: candidate/control compile-only images build under the new boot
  scenarios without lab mutation.
- deferred: runtime BMCR write evidence, selected-tree/TFTP/serial identity,
  restore proof, and hardware lock handling remain in the queued Pi 5 proof.
- deferred: any follow-up after hardware proof remains closeout/supervisor
  territory; this task does not authorize packet I/O or networking.
- rejected: link-ready acceptance is not a local/static outcome even if later
  runtime BMSR/MACB_NSR bits improve.
- rejected: Broadcom-specific selector/shadow/MMD/AUX paths and GPIO32 reset
  recovery remain outside this proof core.
- not-an-issue: no hardware lock was acquired because this is local/static
  implementation and compile-only validation.
- removed: no stale helper or task evidence was removed.

## Accepted Contract

The accepted local/static contract is:

~~~text
contract-id: phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-proof-contract-v1
core-task: phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616
selected-discriminator: bcm54213pe-phy1-bmcr-autoneg-restart
candidate scenario: rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_candidate
control scenario: rpi5_rp1_ethernet_bcm54213pe_bmcr_autoneg_restart_no_mdio_control
preconditions: corrected NCR.MPE set, BMCR_ISOLATE clear
read frames: BMCR, BMSR, ANAR, ANLPAR, MII_CTRL1000, MII_STAT1000
write frame: one PHY1 BMCR frame 0x50821200
post-read context: BMCR, double BMSR, ANAR, ANLPAR, MII_CTRL1000,
  MII_STAT1000, passive MACB_NSR_LINK
control: same freshness/report shape with no MDIO/MAN/MACB/GPIO32/PHY/RP1
  Ethernet target facts
~~~

Allowed hardware classifications for the future proof are limited to:

- bcm54213pe-bmcr-autoneg-restart-post-status-sampled;
- bcm54213pe-bmcr-autoneg-restart-precondition-blocker;
- bcm54213pe-bmcr-autoneg-restart-timeout;
- bcm54213pe-bmcr-autoneg-restart-capture-blocker;
- no-mdio-no-ethernet-bcm54213pe-bmcr-autoneg-restart-control.

## Evidence

- Source checkpoint:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint.md.
- Source checkpoint classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint/classification.json.
- Code contract and tests:
  src/rp1_ethernet.rs.
- Candidate/control boot dispatch:
  build.rs, src/main.rs, src/target/rpi5.rs.
- Compile-only scripts:
  scripts/rpi5-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-candidate-image.sh,
  scripts/rpi5-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-control-image.sh.
- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core/evidence-map.json.

## Validation

- cargo fmt --all -- --check: pass.
- focused Rust tests: cargo -Zjson-target-spec test
  rp1_ethernet_bcm54213pe_bmcr_autoneg_restart -- --nocapture passed,
  including the two new BMCR/autoneg restart tests.
- candidate compile-only build:
  TALOS_CAPTURE_NONCE=bcm54213pe-bmcr-core-candidate-20260616T1349Z
  ./scripts/rpi5-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-candidate-image.sh
  passed; image sha256
  d4a70e105b4a899bc23595de159b1c1b316aa20a38c3d3f8f86226882b6ac06e,
  size 53112 bytes.
- control compile-only build:
  TALOS_CAPTURE_NONCE=bcm54213pe-bmcr-core-control-20260616T1349Z
  ./scripts/rpi5-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-control-image.sh
  passed; image sha256
  56694f459167ae8231e97dcb61a8aaa4434ef0ed5a9b9d96ac38c1d1cf448bb0,
  size 50184 bytes.
- cargo -Zjson-target-spec test --quiet: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof-20260616 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and projects/talos
remains clean. Do not promote closeout, packet I/O, networking, SSH,
Phase 12.2, or a phase transition from local/static evidence alone.
