# Phase 12 RP1 Ethernet PHY Power/Strap Source Checkpoint

Task id: phase12-rp1-ethernet-phy-power-strap-source-checkpoint-20260615

Status: accepted

Classification:
post-physical-phy-power-strap-source-checkpoint-no-distinct-discriminator

Evidence level: static/source/task evidence inspection, JSON evidence
validation, diff checks, and docs build. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle,
GPIO/RIO/pad/MMIO write, event clear, PHY reset assertion/deassertion, BMCR
write, PHY configuration write, MACB configuration write, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Reconcile the accepted post-physical phy-not-ready frontier and GPIO32
persistent-event-state blocker against source-backed PHY power, reset, strap,
and MACB/phylink facts before selecting any follow-up.

## Scope Performed

- Inspected the accepted post-physical v2 status proof/closeout and task-owned
  classification/evidence map.
- Inspected the accepted GPIO32 reset-recovery source checkpoint and prior
  GPIO32 event-state, event-clear, and write/restore blockers.
- Inspected accepted PHY1 status, BMSR double-sample link-readiness,
  MACB_NSR_LINK, autoneg-restart v2, and clock/reset prerequisite closeouts.
- Re-read retained Raspberry Pi Linux RP1 device-tree and MACB source excerpts
  for PHY node/address, reset GPIO polarity/timing, `rgmii-id` mode, MDIO bus
  reset hook, and phylink/MACB interaction.
- Determined whether a qualitatively distinct future hardware proof can be
  selected without repeating same-shaped BMCR/BMSR/ANAR/ANLPAR/MACB_NSR
  sampling, GPIO32 event-clear retry, GPIO32 write/restore retry, or BMCR
  write/restart retry.
- Updated Phase 12 docs and roadmap with the resulting no-distinct-
  discriminator blocker.

## Findings

- fixed: the accepted post-physical v2 frontier remains phy-not-ready with
  BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR
  0x00000006, BMSR link-status=false, autoneg-complete=false,
  ANLPAR nonzero=false, and MACB_NSR_LINK=false.
- fixed: prior PHY1 status evidence observed PHYSID1 0x600d and PHYSID2
  0x84a2 at `ethernet-phy@1`, but retained source does not name a more precise
  PHY model beyond the devicetree child node, Broadcom-specific quirks, and
  observed Clause 22 ID fields.
- fixed: retained Raspberry Pi devicetree source backs `phy-handle = <&phy1>`,
  `phy-reset-gpios = <&rp1_gpio 32 GPIO_ACTIVE_LOW>`,
  `phy-reset-duration = <5>`, PHY address 1, `brcm,powerdown-enable`,
  `eee-broken-1000t`, and `eee-broken-100tx`.
- fixed: retained RP1 source backs the MAC-side mode as `phy-mode =
  "rgmii-id"` and the controller as `raspberrypi,rp1-gem`, `cdns,macb` with
  RP1 Ethernet clocks and interrupt metadata already handled by prior
  prerequisites.
- fixed: retained MACB source shows `macb_mdio_reset()` asserts the optional
  reset GPIO, sleeps `phy_reset_ms`, then deasserts it through the MDIO bus
  reset hook; this source fact strengthens the GPIO32/ETH_RST_N relevance but
  does not bypass the accepted GPIO32 persistent-event-state no-write blocker.
- fixed: retained MACB/phylink source shows PHY connection and MAC
  configuration flow depend on phylink, MDIO/PHY attachment, and possible
  MACB NCFGR/NCR writes; this is broader than the accepted Talos read-only
  status frontier and cannot be selected as a thin recovery proof here.
- blocked: no source-backed power, strap, firmware/event-state, or reset-
  controller fact in the retained evidence justifies a new hardware proof that
  is distinct from the already rejected same-shaped status samples, GPIO32
  event clear, GPIO32 write/restore, or BMCR autoneg-restart paths.
- deferred: a future task may revisit the frontier only with new source-backed
  evidence for firmware/event-state ownership, PHY-specific power/strap
  registers, or reset-controller ownership, and with an explicit candidate/
  control contract.
- not-an-issue: the accepted physical Ethernet link precondition remains
  accepted and is not re-asked.
- removed: no source, helper, task, or evidence files were removed.

## Reconciliation

The accepted status frontier is not a cabling question and not a general
Ethernet readiness proof. It is a bounded post-physical sample over the
selected corrected-target PHY1 registers and passive MACB_NSR bit after the
physical-link precondition was accepted. The result stayed not ready: no BMSR
link bit, no autoneg completion, no ANLPAR partner advertisement, and no
MACB_NSR link bit.

The source-backed PHY path still routes through PHY1 at address 1, RGMII-ID
mode, and GPIO32 / ETH_RST_N active-low reset. Linux's MACB source uses the
optional `phy_reset_gpio` as an MDIO bus reset hook: logical assertion drives
the active-low reset line low, waits the configured duration, then deasserts
the line. That makes reset relevant, but it also confirms that a reset
recovery proof is a GPIO32 ownership proof, not a passive status sample.

The accepted GPIO32 checkpoint remains the limiting evidence. GPIO32
write/restore v2 stopped before writes with event bits 0x0ab00000, and the
only accepted IRQRESET clear attempt left event bits 0x08800000 while
preserving CTRL/RIO/pad invariants. Retained RP1 pinctrl source names the bits
and clear mechanism but does not prove they are stale, firmware-owned in a
benign way, or safe to ignore before driving ETH_RST_N.

The clock/reset prerequisite frontier does not unlock this. It accepted only
idempotent restore-style proofs for Ethernet-private clock control registers
and report visibility for prerequisite metadata. It did not accept broad
reset-controller ownership, GPIO32 ownership, PHY reset, runtime PHY
configuration, packet I/O, or MACB configuration writes.

## Decision

No future hardware proof is selected by this checkpoint.

Reason:

- another BMCR/BMSR/ANAR/ANLPAR/MACB_NSR sample would repeat the already
  accepted post-physical status shape;
- another BMCR autoneg-restart write would repeat the accepted v2 link-not-
  ready recovery shape;
- another GPIO32 event-clear or write/restore attempt would repeat the
  persistent-event-state blocker without a new source-backed ownership fact;
- retained source identifies PHY1, RGMII-ID, GPIO32 active-low reset, MACB
  reset hook, and phylink/MACB dependencies, but not a narrow power/strap/
  firmware ownership discriminator that Talos can prove without broadening
  scope.

This checkpoint sets planningNeeded=true for supervisor selection of either a
new source-gathering task with explicit evidence requirements or an explicit
pause. Any future hardware task must name an exact task id, source-backed
preconditions, allowed operations, forbidden operations, paired control shape,
hardware lock and restore rules, report fields, and rejected claims.

## Rejected Claims And Retained Risks

Rejected claims:

- GPIO32 ownership;
- ETH_RST_N reset assertion or deassertion;
- GPIO32 event-clear retry;
- GPIO32 write/restore retry or success;
- BMCR write or autoneg-restart retry;
- PHY configuration writes;
- MACB configuration writes;
- link forcing;
- reset-controller ownership;
- firmware/event-state ownership;
- PHY-specific power/strap register ownership;
- broad MDIO/PHY ownership;
- DMA/descriptors;
- packet I/O;
- interrupts;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The PHY may still require a reset, power, strap, firmware, or PHY-specific
  configuration step that the retained evidence does not yet make safe.
- GPIO32 event bits may still be stale, level-reasserted, firmware-owned, or
  tied to another ownership path; current evidence does not decide it.
- The observed PHY ID fields identify a real corrected-target PHY response but
  do not by themselves prove link readiness or safe recovery writes.
- Packet I/O and network-stack work remain blocked until link and lower-level
  ownership prerequisites are separately accepted.

## Evidence

- V2 post-physical closeout task:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout.md.
- V2 post-physical closeout classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout/classification.json.
- GPIO32 reset-recovery source checkpoint:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint.md.
- GPIO32 reset-recovery checkpoint classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint/classification.json.
- PHY1 status diagnostic closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-closeout.md.
- PHY1 BMSR double-sample closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout.md.
- MACB_NSR_LINK closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-closeout.md.
- PHY1 autoneg-restart v2 closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout.md.
- Clock/reset prerequisite closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-clock-reset-prereq-closeout.md.
- Retained Raspberry Pi Linux source excerpts:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts,
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi, and
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- Checkpoint classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-phy-power-strap-source-checkpoint/classification.json.
- Checkpoint evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-phy-power-strap-source-checkpoint/evidence-map.json.

## Validation

- static/source/task evidence inspection: accepted post-physical v2 closeout,
  GPIO32 reset-recovery checkpoint, prior PHY1 status/link/MAC/autoneg
  closeouts, clock/reset prerequisite closeout, retained Raspberry Pi Linux
  device-tree and MACB source excerpts, Phase 12 docs, roadmap, and git
  history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON and
  referenced input classification/evidence JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Accepted phy-not-ready evidence, GPIO32 persistent-event-state blocker,
  clock/reset prerequisite boundary, prior PHY1 status/link/MAC/autoneg
  evidence, and source-backed PHY facts reconciled: satisfied.
- No future hardware proof selected because no distinct safe discriminator is
  justified by the retained source/evidence boundary: satisfied.
- Precise blocker and planningNeeded reason recorded: satisfied.
- GPIO32 reset recovery, event-clear retry, BMCR write, PHY configuration,
  packet I/O, networking, SSH, Phase 12.2, and phase transition remain
  explicitly rejected: satisfied.
- Accepted checkpoint committed before any follow-up starts: satisfied once
  this task is committed.

## Next Action

Set planningNeeded=true. Supervisor must select a new source-gathering task
with explicit evidence requirements or an explicit pause. Do not start
hardware, GPIO32 reset recovery, event-clear retry, BMCR write, PHY
configuration, packet I/O, networking, SSH, Phase 12.2, or a phase transition
without a new explicit queued task.
