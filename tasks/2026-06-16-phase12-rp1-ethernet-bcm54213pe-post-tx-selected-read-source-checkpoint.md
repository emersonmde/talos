# Phase 12.1 RP1 Ethernet BCM54213PE Post-TX Selected Read Source Checkpoint

Task id: phase12-rp1-ethernet-bcm54213pe-post-tx-selected-read-source-checkpoint-20260616

Status: accepted

Classification:
bcm54213pe-post-tx-selected-read-source-contract-correction-selected

Evidence level: static/task/source evidence inspection, accepted TX selected
read closeout review, JSON evidence validation, docs build, and diff checks.
No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, GPIO32/PHY reset, TX delay write,
BMCR restart, convergence polling, packet I/O, networking, SSH, Phase 12.2, or
phase transition was performed.

## Goal

Checkpoint the accepted TX selected-register read visibility frontier and
choose the next bounded source direction without broadening it into TX delay
write/readback, BMCR restart, link readiness, packet I/O, networking, SSH, or a
phase transition.

## Scope Performed

- Inspected the accepted RGMII delay source contract, proof core, Pi 5 proof,
  closeout, TX selected-read discriminator core, TX selected-read Pi 5 proof,
  and TX selected-read closeout.
- Inspected retained Raspberry Pi Linux BCM54213PE source excerpts for
  bcm54xx_config_clock_delay(), bcm54xx_auxctl_read(), bcm54xx_auxctl_write(),
  and Broadcom SHD helper mechanics.
- Compared the accepted full RGMII delay failure with the accepted isolated TX
  selected-read success.
- Selected a source-contract correction boundary for supervisor planning rather
  than a same-shaped full RGMII delay retry or immediate TX delay write/readback
  hardware proof.

## Findings

- fixed: accepted evidence now proves the PHY1 TX shadow selector write path and
  selected TX shadow read path can complete when isolated from the preceding RX
  AUX_CTL read/write sequence.
- fixed: the accepted TX selected-read value was raw 0x0e00; after applying the
  accepted SHD data mask 0x03ff, GTXCLK_EN bit 0x0200 is already set in that
  isolated sample.
- fixed: the earlier full RGMII delay proof remains a distinct runtime fact:
  RX delay write/readback reached hardware and reported RGMII_SKEW_EN true, then
  the subsequent TX selected-register read failed before TX write, BMCR restart,
  or convergence polling.
- fixed: retained Linux source still orders RX delay handling before TX delay
  handling in bcm54xx_config_clock_delay(), but the accepted Talos hardware
  evidence shows the next unknown is the interaction/order boundary between the
  RX AUX_CTL sequence and the TX SHD selected read, not generic selected-tree,
  TFTP, serial freshness, or standalone TX selected-read visibility.
- deferred: supervisor planning must create the next explicit local/static
  source contract if the program continues this path. That contract should
  decide whether to model an RX-then-TX selected-read interlock discriminator, a
  TX-delay-already-enabled resume path, or a guarded TX write/readback path with
  ordering/readback criteria.
- rejected: immediate TX delay write/readback hardware proof, BMCR restart,
  convergence polling, link-ready acceptance, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition are not accepted from this checkpoint.
- not-an-issue: the accepted physical Ethernet link precondition, hardware
  capture chain v4, serial freshness guard v1, and selected-tree/TFTP transport
  evidence remain valid inputs; they are not the current blocker.
- removed: no source, helper, task, or evidence files were removed.

## Selected Boundary

The selected next boundary is supervisor planning for a local/static
source-contract correction:

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction-20260616.

That future task should stay in Milestone 12.1 and should be dependency-gated
on the accepted TX selected-register read visibility closeout. It should not
run hardware. It should reconcile these facts before any implementation or
serialized proof:

- Linux source order: RX delay read/modify/write first, then TX delay
  read/modify/write.
- Accepted full RGMII delay hardware fact: RX delay write/readback reached
  hardware, then the TX selected-register read failed.
- Accepted isolated TX selected-read hardware fact: the TX selector write and
  selected read completed with raw 0x0e00, whose SHD data includes GTXCLK_EN
  0x0200.
- Existing rejected claims: TX delay write/readback, BMCR restart after the
  full delay path, convergence polling after delay configuration, link
  readiness, packet I/O, networking, SSH, Phase 12.2, and phase transition.

This checkpoint deliberately does not promote the blocked link-ready
packet-readiness checkpoint and does not authorize a fresh hardware run.

## Rejected Claims And Retained Risks

Rejected claims:

- TX delay write/readback success;
- BMCR restart after delay configuration;
- convergence polling after delay configuration;
- link readiness;
- link-not-ready after the full corrected delay path;
- GPIO32/PHY reset ownership;
- interrupt ownership;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The isolated TX selected-read success may not remain true after RX AUX_CTL
  access, so same-shaped full RGMII delay retries remain closed until the next
  source contract corrects the ordering question.
- The isolated raw TX value already has GTXCLK_EN set, so a future source
  contract must avoid treating a redundant TX write as the only possible
  progress path.
- BMCR restart and convergence polling remain previously accepted primitives,
  but they are still downstream of a corrected RGMII delay boundary.

## Evidence

- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-tx-selected-read-source-checkpoint/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-tx-selected-read-source-checkpoint/evidence-map.json.
- Accepted TX selected-read closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-closeout.md.
- Accepted TX selected-read proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/classification.json.
- Accepted full RGMII delay proof:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof.md.
- Retained source citations:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-broadcom-bcm54213pe-config-contract-excerpt.txt
  and
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/linux-rpi-6.12-bcm-phy-lib-selector-read-contract-excerpt.txt.

## Validation

- static/task/source evidence inspection.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

## Next Action

Supervisor planning is required for
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction-20260616
or an explicit pause. No queued task is mechanically unblocked by this
checkpoint. Do not start TX delay write/readback, BMCR restart, packet I/O,
networking, SSH, Phase 12.2, or phase-transition work from this checkpoint.
