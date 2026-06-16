# Phase 12.1 RP1 Ethernet BCM54213PE RGMII Delay TX-Order Proof Core

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core-20260616

Status: accepted

Classification:
bcm54213pe-rgmii-delay-tx-order-proof-core-local-static

Evidence level: static/source/task evidence inspection, focused and full Rust
unit tests, compile-only Pi 5 candidate/control image builds, static image string
inspection, JSON evidence validation, docs build, and diff checks. No Pi 5
hardware run, hardwareTestLock acquisition, boot archive publication, lab
mutation, power-cycle, GPIO32/PHY reset, packet I/O, networking, SSH, Phase 12.2,
or phase transition was performed.

## Goal

Implement the local/static proof core selected by the TX-order source-correction
task so a later Pi 5 proof can test the real corrected feature path instead of
the prior source-control-flow sentinel.

## Scope Performed

- Updated the RGMII delay candidate path in src/target/rpi5.rs to use explicit
  RX-to-TX stage accounting.
- Preserved the Linux-backed order: RX selected read/write/readback, then TX
  selector/selected read, optional TX write/readback, then BMCR restart and
  bounded convergence polling only after RX/TX criteria are satisfied.
- Added an explicit TX policy: if the selected TX read already has GTXCLK_EN set
  after SHD data mask 0x03ff, the candidate skips a redundant TX write and
  records tx-delay-write-skipped-already-enabled=true.
- Updated rp1_ethernet.rs local/static contract constants and validators to the
  TX-order task, contract, source-correction commit, allowed classifications,
  operation order, and rejected claims.
- Updated candidate/control image review scripts and retained direct image
  string-inspection evidence for the new markers.
- Updated Phase 12 roadmap docs to move the frontier from source correction to
  the TX-order proof core and gate the Pi 5 proof as the next boundary.

## Findings

- fixed: the candidate no longer returns the historical default
  rgmii-delay-capture-blocker after successful RX selected read/write/readback.
  RX success now advances to the TX selector/read path.
- fixed: RX failures, TX selector/read failures, readback mismatches, BMCR
  precondition/capture failures, and convergence outcomes have separate terminal
  classifications for the later hardware proof.
- fixed: the candidate records rx-selected-read-completed,
  rx-delay-write-completed, rx-readback-completed, tx-selector-write-completed,
  tx-selected-read-completed, tx-delay-write-completed,
  tx-delay-write-skipped-already-enabled, tx-readback-completed,
  bmcr-write-performed, and convergence poll sample fields.
- fixed: local/static validators now reject order/surface drift through the
  explicit operation-order, selected-surface, readback-mask, rejected-claims, and
  allowed-classification contracts.
- fixed: candidate/control compile-only images retain TX-order task/contract
  markers and required stage-accounting strings.
- deferred: Pi 5 hardware evidence belongs to
  phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof-20260616.
- removed: no source, helper, task, evidence, or doc files were removed.
- not-an-issue: the existing RGMII delay boot scenario names are retained for
  continuity; the emitted task/contract markers disambiguate this corrected
  TX-order proof core from earlier accepted RGMII delay evidence.

## Accepted Contract

Candidate operation order:

- check accepted MACB NCR.MPE precondition;
- RX selector write to MII_BCM54XX_AUX_CTL 0x18 value 0x7007;
- RX selected read through MAN frame 0x60e20000;
- RX RMW write through prefix 0x50e20000 with WREN 0x8000,
  RGMII_SKEW_EN 0x0100, misc shadow 0x0007, preserving pre-read bits;
- RX selected readback and require RGMII_SKEW_EN 0x0100;
- TX selector write to MII_BCM54XX_SHD 0x1c value 0x0c00 through prefix
  0x50f20000;
- TX selected read through MAN frame 0x60f20000;
- if the selected TX data already has GTXCLK_EN 0x0200 after mask 0x03ff, skip
  redundant TX write and record that policy;
- otherwise TX RMW write through prefix 0x50f20000 with SHD_WRITE 0x8000,
  CLK_CTL selector 0x0c00, GTXCLK_EN 0x0200, and SHD data mask 0x03ff, followed
  by selected readback requiring GTXCLK_EN;
- BMCR restart frame 0x50821200 only after RX and TX delay criteria are
  satisfied;
- bounded convergence poll only after the accepted delay criteria and BMCR
  restart boundary.

Allowed terminal classifications for the follow-up hardware proof:

- rgmii-delay-tx-order-link-ready-frontier;
- rgmii-delay-tx-order-timeout-link-not-ready;
- rgmii-delay-tx-order-rx-stage-blocker;
- rgmii-delay-tx-order-tx-selected-read-visible;
- rgmii-delay-tx-order-tx-stage-blocker;
- rgmii-delay-tx-order-readback-mismatch;
- rgmii-delay-tx-order-precondition-blocker;
- rgmii-delay-tx-order-capture-blocker;
- no-mdio-no-ethernet-bcm54213pe-rgmii-delay-tx-order-control.

## Rejected Claims And Retained Risks

Rejected claims:

- hardware success from this local/static proof core;
- Pi 5 TX delay write/readback success;
- Pi 5 BMCR restart or convergence success;
- link readiness;
- GPIO32/PHY reset ownership;
- interrupt ownership;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The later Pi 5 proof may still expose an RX stage, TX stage, BMCR, convergence,
  capture, or hardware-precondition blocker.
- The TX already-enabled skip path depends on hardware returning GTXCLK_EN in the
  selected TX read; a later proof must classify the observed result directly.
- Local/static image evidence proves only code shape and retained markers, not
  hardware behavior.

## Evidence

- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core/evidence-map.json.
- Candidate image review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core/image-review/candidate-review.txt.
- Control image review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core/image-review/control-review.txt.
- Accepted source correction:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction.md.

## Validation

- static/source/task evidence inspection.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test --quiet rp1_ethernet_bcm54213pe_rgmii_delay.
- cargo -Zjson-target-spec test --quiet.
- compile-only candidate/control Pi 5 image builds.
- static image/string/marker inspection.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof-20260616 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention is inactive, and projects/talos is
clean. Do not start packet I/O, networking, SSH, Phase 12.2, or phase-transition
work from this local/static proof core.
