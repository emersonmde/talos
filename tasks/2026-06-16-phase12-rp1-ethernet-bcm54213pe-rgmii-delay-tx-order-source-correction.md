# Phase 12.1 RP1 Ethernet BCM54213PE RGMII Delay TX-Order Source Correction

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction-20260616

Status: accepted

Classification:
bcm54213pe-rgmii-delay-tx-order-source-correction-proof-core-selected

Evidence level: static/source/task evidence inspection, accepted RGMII delay
proof review, accepted TX selected-read proof review, JSON evidence validation,
docs build, and diff checks. No Pi 5 hardware run, boot archive publication,
lab mutation, hardwareTestLock acquisition, power-cycle, GPIO32/PHY reset,
packet I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Reconcile the accepted isolated TX selected-register read visibility with the
earlier full RGMII delay proof and select the next bounded source correction
without treating a source-control-flow artifact as hardware evidence.

## Scope Performed

- Inspected the accepted post-TX selected-read checkpoint, TX selected-read
  proof/closeout, full RGMII delay proof/closeout, and retained Linux
  BCM54213PE clock-delay source.
- Inspected the current Talos RGMII delay candidate implementation in
  src/target/rpi5.rs and the local/static contract constants in
  src/rp1_ethernet.rs.
- Compared the accepted serial facts with the candidate source control flow.
- Selected a bounded proof-core correction task and rejected same-shaped
  hardware retry, immediate TX write/readback proof, BMCR restart proof, packet
  I/O, networking, SSH, Phase 12.2, and phase transition.

## Findings

- fixed: the next boundary is corrected from a presumed RX-to-TX hardware
  interlock to a source-control-flow correction. The accepted RGMII delay
  candidate initializes classification to rgmii-delay-capture-blocker and never
  changes it after the RX selected read/write/readback sequence succeeds, so the
  later classification gate returns the default blocker before attempting the
  TX selected-register read branch.
- fixed: accepted serial evidence remains decisive for what it directly
  observed: RX delay write/readback reached hardware with rx-pre-raw 0x71e7,
  rx-write-value 0xf1e7, rx-readback-raw 0x71e7, and RGMII_SKEW_EN true.
- fixed: accepted serial evidence does not directly prove that TX selected-read
  failed after RX. The reported TX fields were zero because the current source
  path stopped at the default blocker before attempting the TX branch.
- fixed: the isolated TX selected-read discriminator remains valid and proves
  TX selector write/read visibility by itself: TX selector value 0x0c00,
  selected TX read raw 0x0e00, selected-read-completed=true, and GTXCLK_EN
  0x0200 already set after applying SHD data mask 0x03ff.
- fixed: retained Linux source still orders RX delay handling before TX delay
  handling in bcm54xx_config_clock_delay(), so the corrected Talos proof core
  should preserve RX-then-TX order while ensuring RX success advances to the TX
  selector/read path.
- deferred: the implementation fix, focused validators, compile-only
  candidate/control images, and image/string inspection belong to the queued
  proof-core task. This source-correction task only specifies that boundary.
- rejected: immediate hardware retry, TX delay write/readback hardware
  acceptance, BMCR restart, convergence polling, link readiness, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition are not accepted.
- removed: no source, helper, task, or evidence files were removed.
- not-an-issue: selected-tree/TFTP, serial freshness, capture-chain, and restore
  evidence from the accepted RGMII delay and TX selected-read proofs remain
  valid for their direct observations.

## Selected Proof-Core Boundary

Selected next task:
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core-20260616.

The follow-up proof core must implement only this source correction:

- Preserve the Linux-backed order: RX AUX_CTL selected read, RX RMW write, RX
  selected readback, then TX SHD selector write, selected TX read, optional TX
  RMW write/readback if the TX read succeeds, and only then the already
  accepted BMCR restart/convergence path.
- Replace the current default-blocker sentinel with explicit stage accounting
  so RX success cannot return rgmii-delay-capture-blocker before TX is
  attempted.
- Record separate booleans/counters for rx-selected-read-completed,
  rx-delay-write-completed, rx-readback-completed, tx-selector-write-completed,
  tx-selected-read-completed, tx-delay-write-completed, tx-readback-completed,
  bmcr-write-performed, and convergence-poll-samples.
- Treat an RX selector/read/write/readback failure as an RX-stage blocker; treat
  a TX selector write or selected-read failure as a TX-stage blocker; treat a
  TX readback mismatch as a readback mismatch; do not collapse those stages into
  one default capture blocker.
- If TX selected read returns raw 0x0e00 or any value with GTXCLK_EN already
  set after SHD data mask 0x03ff, the candidate may skip a redundant TX write
  or perform an idempotent write only if the proof core records the exact chosen
  policy and validator coverage. Either path still requires a later Pi 5 proof
  before acceptance.
- Keep the paired control as no-MDIO/no-Ethernet/no-MACB/no-GPIO32 target
  construction with the same report/rejection shape.

Allowed Clause 22 frames for the follow-up proof core:

- RX selector write: MII_BCM54XX_AUX_CTL 0x18 value 0x7007 through MAN write
  frame prefix 0x50e20000.
- RX selected read: MAN read frame 0x60e20000.
- RX delay RMW write: MAN write frame prefix 0x50e20000 with WREN 0x8000,
  RGMII_SKEW_EN 0x0100, misc shadow 0x0007, preserving pre-read bits.
- TX selector write: MII_BCM54XX_SHD 0x1c value 0x0c00 through MAN write frame
  prefix 0x50f20000.
- TX selected read: MAN read frame 0x60f20000.
- TX delay RMW write, if selected by the proof core: MAN write frame prefix
  0x50f20000 with SHD_WRITE 0x8000, CLK_CTL selector 0x0c00, GTXCLK_EN 0x0200,
  and SHD data mask 0x03ff.
- BMCR restart frame 0x50821200 only after corrected RX and TX delay readback
  criteria are satisfied.

Required readback masks:

- RX readback requires RGMII_SKEW_EN 0x0100.
- TX readback requires GTXCLK_EN 0x0200 after SHD data mask 0x03ff.

Allowed terminal classifications for the later Pi 5 proof must be explicit and
stage-specific, including:

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

- hardware success from this static source correction;
- TX delay write/readback success;
- BMCR restart after corrected delay configuration;
- convergence polling after corrected delay configuration;
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

- Correcting source control flow may still expose a real TX selected-read,
  TX write/readback, BMCR, convergence, or link-readiness blocker on hardware.
- The isolated raw TX value already has GTXCLK_EN set, so the next proof core
  must avoid making a redundant TX write the only progress path.
- The accepted RGMII delay proof's classification text and JSON preserve the
  historical blocker label, but this task supersedes the interpretation that TX
  selected-read failure after RX was directly evidenced.

## Evidence

- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction/evidence-map.json.
- Accepted post-TX source checkpoint:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-tx-selected-read-source-checkpoint.md.
- Accepted TX selected-read proof:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof.md.
- Accepted full RGMII delay proof:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof.md.
- Current source inspection:
  src/target/rpi5.rs run_rp1_ethernet_bcm54213pe_rgmii_delay_candidate.
- Retained Linux source citations:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-broadcom-bcm54213pe-config-contract-excerpt.txt
  and
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/linux-rpi-6.12-bcm-phy-lib-selector-read-contract-excerpt.txt.

## Validation

- static/source/task evidence inspection.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core-20260616 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention is inactive, and projects/talos is
clean. Do not run hardware, publish boot archives, or start packet I/O,
networking, SSH, Phase 12.2, or phase-transition work from this source
correction.
