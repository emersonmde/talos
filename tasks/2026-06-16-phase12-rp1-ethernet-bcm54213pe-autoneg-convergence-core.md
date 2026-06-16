# Phase 12.1 RP1 Ethernet BCM54213PE Autoneg Convergence Core

Task id: phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core-20260616

Status: accepted

Classification:
bcm54213pe-autoneg-convergence-proof-core-local-static

Evidence level: local/static Rust contract tests, candidate/control
compile-only builds, task-owned JSON evidence, and diff checks. No Pi 5
hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, power-cycle, TFTP/serial capture, restore, GPIO32 event
clear/reset recovery, Broadcom shadow/MMD/AUX selector access, interrupt
ownership, packet I/O, networking, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Implement the smallest local/static proof core that extends the accepted
BCM54213PE PHY1 BMCR autoneg restart lineage with a bounded convergence poll.

## Scope Performed

- Added a BCM54213PE autoneg convergence contract in src/rp1_ethernet.rs.
- Added focused tests that accept exactly one corrected-target PHY1 BMCR write
  frame 0x50821200 followed by a bounded eight-sample poll schedule.
- Added candidate/control boot scenarios and compile-only image scripts.
- The candidate records BMCR, double-sampled BMSR, ANAR, ANLPAR,
  MII_CTRL1000, MII_STAT1000, and passive MACB_NSR_LINK for the terminal poll
  sample.
- The control preserves the same freshness/report shape while constructing no
  MDIO, MAN, MACB, GPIO32, PHY, or RP1 Ethernet target facts.

## Non-Goals

No hardware run, no lab or boot publication, no GPIO32 event clear/reset
recovery, no Broadcom selector/MMD/AUX access, no interrupt ownership, no MAC
configuration, no DMA/descriptors, no packet I/O, no networking, no sockets, no
SSH, no Phase 12.2 work, and no phase transition.

## Findings

- fixed: the convergence core now preserves the accepted BMCR write lineage and
  adds a bounded wait/poll schedule instead of reinterpreting the immediate
  post-status sample.
- fixed: candidate compile-only code performs exactly one BMCR autoneg restart
  write and then only status polling.
- fixed: focused tests reject extra writes, selector/config access, GPIO32
  reset action, link forcing, networking, and phase transition claims.
- fixed: the no-MDIO/no-Ethernet control constructs no target facts while
  retaining the same report/freshness shape.
- deferred: Pi 5 selected-tree/TFTP/serial/runtime evidence remains in the
  queued hardware proof task.
- deferred: any packet I/O, networking, SSH, or Phase 12.2 progress remains
  outside this Phase 12.1 convergence boundary.
- not-an-issue: no hardware lock was acquired because this is local/static
  implementation and compile-only validation.
- removed: no stale helper or task evidence was removed.

## Accepted Contract

~~~text
contract-id: phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-proof-contract-v1
core-task: phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core-20260616
accepted-restart-commit: 0de9450e8e1c2d5f458d1c778f4f25e80860c0a2
selected-discriminator: bcm54213pe-phy1-bmcr-autoneg-restart-plus-convergence-poll
candidate scenario: rpi5_rp1_ethernet_bcm54213pe_autoneg_convergence_candidate
control scenario: rpi5_rp1_ethernet_bcm54213pe_autoneg_convergence_no_mdio_control
write frame: one PHY1 BMCR frame 0x50821200
poll bound: 8 samples
poll wait: 200000 spin loops before each sample
poll sample: BMCR, double BMSR, ANAR, ANLPAR, MII_CTRL1000, MII_STAT1000,
  passive MACB_NSR_LINK
control: same freshness/report shape with no MDIO, MAN, MACB, GPIO32, PHY, or
  RP1 Ethernet target facts
~~~

Allowed terminal classifications for the future Pi 5 proof are limited to:

- bcm54213pe-autoneg-convergence-link-ready;
- bcm54213pe-autoneg-convergence-still-timeout;
- bcm54213pe-autoneg-convergence-precondition-blocker;
- bcm54213pe-autoneg-convergence-capture-blocker;
- no-mdio-no-ethernet-bcm54213pe-autoneg-convergence-control.

## Evidence

- Accepted restart closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-closeout.md.
- Code contract and tests: src/rp1_ethernet.rs.
- Candidate/control boot dispatch: build.rs, src/main.rs, src/target/rpi5.rs.
- Compile-only scripts:
  scripts/rpi5-rp1-ethernet-bcm54213pe-autoneg-convergence-candidate-image.sh,
  scripts/rpi5-rp1-ethernet-bcm54213pe-autoneg-convergence-control-image.sh.
- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core/evidence-map.json.

## Validation

- cargo fmt --all -- --check: pass.
- focused Rust tests: cargo -Zjson-target-spec test
  rp1_ethernet_bcm54213pe_autoneg_convergence -- --nocapture passed.
- candidate compile-only build:
  TALOS_CAPTURE_NONCE=bcm54213pe-convergence-core-candidate-20260616T1553Z
  ./scripts/rpi5-rp1-ethernet-bcm54213pe-autoneg-convergence-candidate-image.sh
  passed; image sha256
  e005a79bb39f24c804a63a1c072ff127aa0124af7faf4bae82258ff46bd444d4,
  size 52224 bytes.
- control compile-only build:
  TALOS_CAPTURE_NONCE=bcm54213pe-convergence-core-control-20260616T1553Z
  ./scripts/rpi5-rp1-ethernet-bcm54213pe-autoneg-convergence-control-image.sh
  passed; image sha256
  49e34f482f36fe488a1686448f840763c73a430413add20c51b878cdd78af7d1,
  size 49704 bytes.
- cargo -Zjson-target-spec test --quiet: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof-20260616 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and projects/talos
remains clean. Do not promote reset/config/interrupt/packet/networking/SSH,
Phase 12.2, or a phase transition from local/static evidence alone.
