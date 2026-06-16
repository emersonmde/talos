# Phase 12.1 RP1 Ethernet BCM54213PE Link-Not-Ready Frontier Pause Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-link-not-ready-frontier-pause-closeout-20260616

Status: accepted

Classification:
bcm54213pe-link-not-ready-frontier-paused-return-to-generated-root-transport

Evidence level: static/task evidence inspection, task-owned JSON evidence,
docs build, and diff checks. No runtime code change, Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, power-cycle,
TFTP/serial capture, GPIO32/PHY reset action, MDIO/Broadcom write, interrupt
ownership, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Close the current BCM54213PE link-not-ready research frontier as an explicit
paused Phase 12.1 boundary. Preserve the accepted Ethernet evidence and
rejected claims before returning to the next known non-Ethernet feature
blocker.

## Scope Performed

- Reconciled accepted Phase 12.1 Ethernet evidence from physical-link
  correction through BMCR/autoneg, convergence timeout, read-only preflight,
  RGMII delay, corrected TX-order proof, and the post-TX-order source
  checkpoint.
- Recorded findings with dispositions.
- Updated the Phase 12 docs and roadmap to state that no mechanically ready,
  source-backed, qualitatively distinct link-not-ready discriminator is
  selected now.
- Selected
  phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract-20260616
  as the next non-Ethernet task, with selected_reason
  return-to-known-generated-root-boot-transport-blocker.
- Kept the existing link-ready packet-readiness and generic link-not-ready
  discriminator tasks dependency-gated.

## Findings

- fixed: the accepted TX-order closeout remains the terminal Ethernet frontier.
  The corrected candidate retained selected-tree identity, same-power-cycle
  TFTP byte agreement, serial freshness, final identity, restore proof, RX
  delay read/write/readback, TX selected read/readback with GTXCLK_EN already
  set, skipped the redundant TX write under the accepted policy, executed
  exactly one BMCR restart, and completed eight convergence samples ending
  link-ready-terminal=false.
- fixed: the post-TX-order source checkpoint found no mechanically ready,
  source-backed, qualitatively distinct follow-up discriminator inside its
  scope.
- blocked: link-ready packet-readiness remains dependency-gated until a future
  accepted proof establishes link-ready or autoneg-complete.
- blocked: the generic link-not-ready discriminator core remains
  dependency-gated because selected_discriminator is null.
- deferred: GPIO32 / ETH_RST_N reset ownership remains feature-relevant but is
  still controlled by the accepted persistent-or-firmware-owned event-state
  blocker.
- deferred: MII_CTRL1000 master-mode writes remain source-backed only behind
  the unselected PHY_BRCM_EN_MASTER_MODE gate.
- deferred: interrupt, APD, EEE, LED, WOL, expansion, suspend/resume,
  MAC/phylink, packet, networking, sockets, SSH, Phase 12.2, and
  phase-transition work need future supervisor-planned scope if they are ever
  selected.
- rejected: another BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/MACB_NSR
  poll, another bare BMCR restart, another convergence wait-count tweak, or a
  marker/capture-only retry would repeat accepted timeout/link-not-ready
  evidence and is not progress.
- removed: no source, helper, task, docs, or evidence files were removed.
- not-an-issue: no hardware lock or Pi 5 inconclusive-run triage was needed for
  this static closeout.

## Reconciliation

The accepted Phase 12.1 Ethernet path proved a real sequence of prerequisites
rather than a shim: physical link precondition, selected-tree/TFTP/serial
freshness reliability, MDIO/PHY identity, BMCR/autoneg restart, bounded
convergence, BCM54213PE source-backed RGMII delay handling, and corrected
RX-to-TX delay order. The final hardware proof still ended with BMSR
link-status=false, BMSR autoneg-complete=false, passive MACB_NSR link=false,
and link-ready-terminal=false.

The post-TX-order checkpoint inspected the remaining plausible BCM54213PE
source surfaces. None is mechanically ready without new planning: GPIO32 reset
ownership remains behind the persistent event-state blocker, master-mode writes
remain behind an unselected dev_flags gate, and interrupt or PHY lifecycle
paths are broader ownership work. Repeating the same status vector would shrink
acceptance quality rather than advance the feature.

The correct closeout is therefore a pause at the accepted
timeout/link-not-ready frontier. Phase 12.1 Ethernet evidence remains retained
context, but it does not authorize packet-readiness, packet I/O, networking,
SSH, Phase 12.2, or a phase transition.

## Frontier

Closed input frontier:
bcm54213pe-post-txorder-link-not-ready-no-distinct-source-backed-discriminator-pause.

Accepted closeout frontier:
bcm54213pe-link-not-ready-frontier-paused-return-to-generated-root-transport.

Selected next task:
phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract-20260616.

Selected reason:
return-to-known-generated-root-boot-transport-blocker.

Retained generated-root blocker: local/QEMU generated-root transport is
accepted, but Pi 5 firmware-loaded generated-root artifact consumption remains
deferred until Talos reserves or copies the firmware initramfs range before
early memory setup can overwrite it, then proves the result on Pi 5.

## Evidence

- Post-TX-order source checkpoint:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-txorder-link-not-ready-source-checkpoint.md.
- Post-TX-order checkpoint classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-txorder-link-not-ready-source-checkpoint/classification.json.
- TX-order closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout.md.
- TX-order Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/classification.json.
- GPIO32 reset/event-state context:
  docs/src/project/phase12-networking-ssh.md.
- Generated-root boot transport closeout:
  tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-closeout.md.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-not-ready-frontier-pause-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-not-ready-frontier-pause-closeout/evidence-map.json.

## Acceptance Check

- Closeout classification explicitly pauses Phase 12.1 at the accepted
  timeout/link-not-ready frontier rather than shrinking acceptance to a shim or
  repeating same-shaped status evidence: satisfied.
- Accepted Ethernet evidence and rejected claims are reconciled with findings
  dispositions: satisfied.
- Link-ready packet-readiness remains blocked until a future accepted proof
  establishes link-ready/autoneg-complete: satisfied.
- Next selected non-Ethernet task is
  phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract-20260616:
  satisfied.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once this task is committed.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

After this closeout is accepted and committed, promote
phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract-20260616
on a future worker wake if dependencies remain satisfied. Do not promote
generic Ethernet discriminator, link-ready packet-readiness, hardware,
GPIO32/reset, interrupt, packet I/O, networking, SSH, Phase 12.2, or
phase-transition work from this closeout.
