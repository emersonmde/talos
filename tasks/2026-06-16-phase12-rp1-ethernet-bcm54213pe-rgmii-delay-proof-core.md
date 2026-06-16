# Phase 12 RP1 Ethernet BCM54213PE RGMII Delay Proof Core

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core-20260616

Status: accepted

Classification: bcm54213pe-rgmii-delay-proof-core-local-static

Commit: recorded in supervisor state after commit

## Scope

Implemented the local/static proof core selected by the accepted BCM54213PE
RGMII delay source contract. The candidate/control pair is bounded to the
source-backed PHY1 rgmii-id RX/TX delay write/readback surface, followed only by
the already accepted BMCR restart and bounded convergence poll if both delay
readbacks match.

This task did not run Pi 5 hardware, acquire hardwareTestLock, publish a boot
archive, mutate the lab, clear GPIO32, reset the PHY, enable interrupts, perform
packet I/O, networking, SSH, Phase 12.2, or a phase transition.

## Findings

- fixed: Added boot-scenario routing for
  rpi5_rp1_ethernet_bcm54213pe_rgmii_delay_candidate and
  rpi5_rp1_ethernet_bcm54213pe_rgmii_delay_no_mdio_control.
- fixed: Added local/static contract evidence and validators for PHY1, AUX_CTL
  0x18 selector 0x7007, RGMII_SKEW_EN 0x0100, SHD 0x1c selector 0x0c00,
  GTXCLK_EN 0x0200, exactly one accepted BMCR restart frame 0x50821200, and the
  eight-sample convergence poll.
- fixed: Candidate runtime path performs RX delay read-modify-write/readback,
  then TX delay read-modify-write/readback, stops before BMCR restart on
  readback mismatch, and records terminal classification strings for the later
  hardware proof.
- fixed: Control runtime path emits the same contract/rejection shape while
  withholding MDIO, MAN, MACB, GPIO32, PHY, interrupt, packet, networking, and
  SSH target construction.
- not-an-issue: The scenario image builds emit the existing unreachable-tail
  warning caused by cfg-selected early proof functions returning ! before normal
  kernel_main fallthrough.
- deferred: Serialized Pi 5 hardware proof remains a separate queued task with
  selected-tree, TFTP, serial freshness, final identity, restore, and hardware
  lock evidence requirements.
- deferred: MII_CTRL1000 master-mode writes, APD/EEE/LED/WOL/suspend-resume,
  GPIO32/PHY reset, interrupt ownership, broad PHY/MAC configuration, packet
  I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain outside
  this proof core.

## Evidence

- static/source/task evidence inspection: accepted source contract commit
  817712f6837a7e3ca659cea1833875c22e04f588 selected this exact proof-core
  boundary.
- fmt/lint/typecheck: cargo fmt --all -- --check passed after implementation.
- unit tests/QEMU substitute: cargo -Zjson-target-spec test --quiet passed with
  527 no_std tests using the documented QEMU path.
- focused validator tests: rp1_ethernet_bcm54213pe_rgmii_delay_core_* tests
  pass and reject delay contract drift, BMCR write-count drift, control target
  construction, MII_CTRL1000 master-mode writes, uncontracted selector/config
  access, link-ready acceptance, networking, and phase transition claims.
- image/archive inspection: candidate compile-only image
  kernel_2712-bcm54213pe-rgmii-delay-candidate.img is 53,720 bytes with sha256
  e5592f1671b42ffd14057668ae22ca48d70e25a52ce0200b377b93e71d294a0c.
- image/archive inspection: no-MDIO control compile-only image
  kernel_2712-bcm54213pe-rgmii-delay-control.img is 49,984 bytes with sha256
  240348cbd3f023a7915aab3486c0dc36a8b857098d2a6c093f21847ae62377e3.
- image/archive inspection: candidate/control string checks passed for required
  contract markers and forbidden opposite-path strings.
- docs build: mdbook build passed with the pre-existing large search-index
  warning.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof-20260616 on the next
worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and the repo is
clean. Do not promote packet I/O, networking, SSH, Phase 12.2, or phase
transition work from this local/static proof core.
