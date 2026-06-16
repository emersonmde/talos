# Phase 12.1 RP1 Ethernet BCM54213PE TX Selected Read Discriminator Core

Task id: phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-core-20260616

Status: accepted

Classification: bcm54213pe-tx-selected-read-discriminator-core-local-static

Evidence level: static/source/task evidence inspection, fmt/lint/typecheck,
unit tests, Pi 5 target compile-only image builds, image/string inspection

## Goal

Implement the local/static core for the supervisor-planned TX selected-register
read discriminator after the accepted RGMII delay closeout classified the first
failing runtime layer as the TX delay selected-register read.

## Scope

- Added candidate/control boot scenarios:
  rpi5_rp1_ethernet_bcm54213pe_tx_selected_read_discriminator_candidate and
  rpi5_rp1_ethernet_bcm54213pe_tx_selected_read_discriminator_no_mdio_control.
- Added a candidate report surface that checks NCR.MPE, writes only the PHY1
  TX shadow selector value 0x0c00 to MII_BCM54XX_SHD 0x1c, reads the selected
  TX shadow register through MAN read frame 0x60f20000, then stops.
- Added a paired no-MDIO/no-Ethernet control that constructs no MDIO, MAN,
  MACB, GPIO32, PHY, interrupt, packet, networking, or SSH targets.
- Added validator evidence that rejects TX delay write/readback, RX delay
  write acceptance from this discriminator, BMCR restart, convergence polling,
  packet I/O, networking, SSH, Phase 12.2, and phase transition claims.

## Findings

- fixed: src/rp1_ethernet.rs now records the discriminator contract, selected
  boot scenarios, stage boundaries, allowed terminal classifications, rejected
  claims, retained risks, and source evidence tied to accepted closeout commit
  fef9a9818f05836eb9f28e77eb6ec34b09b55c21.
- fixed: src/target/rpi5.rs now exposes a candidate runtime report that is
  visibly distinct from the accepted broad RGMII delay proof: it performs only
  TX selector write plus selected TX read, and records tx-delay-write-count=0x0
  and bmcr-write-count=0x0.
- fixed: src/target/rpi5.rs now exposes a paired control report with
  target=none, not-constructed NCR/NSR/MAN targets, withheld TX selector/read
  fields, and the no-mdio/no-ethernet control classification.
- fixed: build.rs and src/main.rs route both new boot scenarios.
- fixed: focused validator tests reject target drift and overclaims, including
  TX delay write, BMCR restart, control target facts, networking, and phase
  transition.
- not-an-issue: no shell script was added or changed, so sh -n had no touched
  shell-script input for this task.
- deferred: serialized Pi 5 hardware proof is left to
  phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof-20260616.

## Evidence

- Focused Rust/QEMU gate:
  cargo -Zjson-target-spec test --quiet
  rp1_ethernet_bcm54213pe_tx_selected_read_discriminator passed with 529 no_std
  tests; the focused filter included:
  rp1_ethernet_bcm54213pe_tx_selected_read_discriminator_core_shapes_candidate_and_control
  and
  rp1_ethernet_bcm54213pe_tx_selected_read_discriminator_core_rejects_drift.
- Pi 5 target compile-only candidate build:
  TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_tx_selected_read_discriminator_candidate
  cargo -Zjson-target-spec build --quiet --target
  targets/aarch64-talos-rpi5-bcm2712.json passed.
- Pi 5 target compile-only control build:
  TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_tx_selected_read_discriminator_no_mdio_control
  cargo -Zjson-target-spec build --quiet --target
  targets/aarch64-talos-rpi5-bcm2712.json passed.
- Candidate binary inspection:
  target/aarch64-talos-rpi5-bcm2712/debug/talos,
  sha256 17a779a6043c08ed45bed823ad68744db43e4f44099a62e0a43fad85d28df5d2,
  3458312 bytes. Retained strings include
  tx-selected-read-discriminator-candidate,
  selected-discriminator=bcm54213pe-phy1-tx-selected-register-read,
  tx-selector-write-value=0x0c00, tx-read-frame=0x60f20000,
  tx-delay-write-count=0x0, bmcr-write-count=0x0, and
  tx-selected-register-read-visible.
- Control binary inspection:
  target/aarch64-talos-rpi5-bcm2712/debug/talos,
  sha256 115ce869d55bcd524cfecebf6388d60dbb6b95c1bc4e671207a65f91e20b96ba,
  3449032 bytes. Retained strings include
  tx-selected-read-discriminator-control, target=none,
  ncr-observed-target=not-constructed, nsr-observed-target=not-constructed,
  man-observed-target=not-constructed, tx-selector-write-value=withheld, and
  no-mdio-no-ethernet-bcm54213pe-tx-selected-read-discriminator-control.
- Task-owned JSON evidence:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-core/classification.json
  and
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-core/evidence-map.json.

## Rejected Claims

This task does not accept hardware success, TX delay write/readback, BMCR
restart, convergence polling, RX delay write/readback acceptance from this
discriminator, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof-20260616
on the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and projects/talos is
clean. Do not start packet I/O, networking, SSH, Phase 12.2, or phase
transition work from this core.
