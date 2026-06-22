# Phase 12 Networking and SSH

Phase 12 starts with source-only RP1 Ethernet research. The accepted Phase 11
frontier allows source inventory and design selection, but it does not accept
Ethernet implementation, packet I/O, live DMA, networking, sockets, or SSH.

## RP1 Ethernet Source Inventory

phase12-rp1-ethernet-source-inventory-20260609 accepts the source-backed RP1
Ethernet inventory for Milestone 12.1. Retained and fetched Raspberry Pi Linux
sources identify `rp1_eth` as `raspberrypi,rp1-gem` / `cdns,macb` at RP1 bus
`0xc0_40100000`, with source CPU physical translation `0x1f_0010_0000`,
`RP1_INT_ETH`, clocks `RP1_CLK_SYS`, `RP1_CLK_ETH_TSU`, and `RP1_CLK_ETH`,
`phy-mode = "rgmii-id"`, and Pi 5 PHY reset through RP1 GPIO32.

The inventory also accepts that the Linux Cadence MACB/GEM path depends on DMA
descriptor rings, packet buffers, MDIO/PHY reset, phylink, clocks, interrupts,
and completion handling. Talos keeps those as prerequisites, not accepted
runtime behavior.

## Retained Boundaries

The Phase 12.1 inventory does not accept broad RP1 Ethernet MMIO readiness,
descriptor-ring ownership, RP1 DMA channel ownership, transfer completion,
interrupt completion, GPIO32 PHY reset ownership, clock/reset ownership, packet
I/O, network stack behavior, sockets, SSH, or Phase 12.2 work. The next
bounded step is a path-selection ADR/design note before any implementation.

## Path Selection ADR

phase12-rp1-ethernet-path-adr-20260609 selects the direct RP1 Cadence GEM path
as the Phase 12.1 target, but only after staged hardware-substrate proofs. The
decision rejects immediate driver work because the accepted Linux MACB/GEM
source path depends on unaccepted Ethernet MMIO, DMA descriptors and packet
buffers, ring-base programming, MDIO/PHY reset, clocks, interrupts, and
completion handling.

The ADR defers no_std driver reuse until Talos proves the required hardware and
driver substrate boundaries. It also defers a simpler non-RP1 transport because
that would not retire the explicit RP1 Ethernet research unknowns. The next
program step requires supervisor planning for a bounded read-only RP1
GEM/Ethernet MMIO source-contract diagnostic with paired no-Ethernet/no-MMIO
control evidence. No Ethernet implementation, packet I/O, network stack,
sockets, SSH, live DMA, descriptor rings, or Phase 12.2 work is accepted by
the ADR.

## GEM MID Source Contract

phase12-rp1-ethernet-gem-mid-source-contract-20260609 accepts the source-only
contract for the first bounded GEM visibility diagnostic. The exact future
candidate target is MACB_MID at offset 0x00fc from rp1_eth, producing source
RP1 bus address 0xc0_401000fc and source-translated CPU physical address
0x1f001000fc. The contract is limited to a 32-bit little-endian volatile load
in a future diagnostic, paired with a no-Ethernet/no-MMIO control report that
withholds the Ethernet MMIO target.

Retained Raspberry Pi Linux source evidence defines the rp1_eth base at
0xc0_40100000, MACB_MID at 0x00fc, and the IDNUM and REV fields inside MID.
This remains source evidence only: live broad RP1 Ethernet MMIO readiness, RP1
MMIO writes, DMA, descriptor rings, interrupts, clock/reset ownership, PHY
reset ownership, packet I/O, networking, sockets, SSH, and Phase 12.2 remain
unaccepted.

## GEM MID Diagnostic Core

phase12-rp1-ethernet-gem-mid-diagnostic-core-20260609 accepts the local/static
report construction for the GEM MID source contract. The candidate report
preserves the accepted rp1_eth/MACB_MID target, source evidence, rejected
claims, retained risks, and hardware-proof boundary classification. The paired
control uses the same report contract while withholding Ethernet MMIO target
fields and carrying no-ethernet-no-mmio-rp1-ethernet-gem-mid-control.

The diagnostic core is not a hardware proof. Live GEM visibility, broad
Ethernet MMIO readiness, RP1 MMIO/DMA programming, descriptors, interrupts,
clock/reset ownership, PHY reset ownership, packet I/O, networking, sockets,
SSH, and Phase 12.2 remain unaccepted until later explicit tasks.

## GEM MID Diagnostic Closeout

phase12-rp1-ethernet-gem-mid-diagnostic-closeout-20260609 closes the
local/static GEM MID report frontier before hardware publication. The
checkpoint reconciles the accepted source contract and diagnostic core,
retains the no-Ethernet/no-MMIO paired control boundary, and selects the
queued serialized Pi 5 visibility/control proof as the next bounded step.

The closeout does not accept live GEM visibility, broad Ethernet MMIO
readiness, Ethernet driver readiness, RP1 MMIO/DMA programming, descriptor
rings, interrupt completion, clock/reset ownership, PHY reset ownership,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## GEM MID Pi 5 Proof

phase12-rp1-ethernet-gem-mid-pi5-proof-20260609 accepts the serialized Pi 5
visibility/control proof as a precise blocker. The no-Ethernet/no-MMIO
control proved the serial reporting path without constructing the Ethernet
MMIO target. The candidate reached the bounded `MACB_MID` read at CPU physical
`0x1f001000fc`, but the read returned `raw=0xdeaddead`, classified as
`rp1-ethernet-gem-mid-blocked-address-decode-sentinel`.

This is not live GEM visibility and does not accept broad Ethernet MMIO
readiness. It keeps Ethernet driver readiness, RP1 MMIO/DMA programming,
descriptor rings, transfer completion, interrupt completion, clock/reset
ownership, PHY reset ownership, packet I/O, networking, sockets, SSH, Phase
12.2, and phase transition work rejected. The next step requires supervisor
planning around the GEM MID address-decode or bridge-enable dependency.

## GEM MID Blocker Reconciliation

phase12-rp1-ethernet-gem-mid-blocker-reconciliation-20260610 refines the
`0xdeaddead` result as
`rp1-ethernet-gem-mid-retained-0x1f-window-sentinel`. Retained source evidence
still supports the source translation to `0x1f001000fc`; the stronger clue is
that Phase 11 had already retained `0xdeaddead` through the translated
`0x1f` RP1 aperture while observed `0x1c` RP1 sysinfo/clock/GPIO reads were
visible.

The selected next discriminator is a local/static same-run report shape:
observed RP1 `SYSINFO_CHIP_ID` at `0x1c00000000` as the positive control plus
`MACB_MID` at `0x1f001000fc`, with a paired no-MMIO/no-Ethernet control.
This still rejects live GEM visibility, broad Ethernet MMIO readiness, Ethernet
driver readiness, packet I/O, DMA, interrupts, clock/reset ownership, PHY reset
ownership, networking, sockets, SSH, Phase 12.2, and phase transition work.

## GEM MID Decode Discriminator Core

phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610 accepts only
the local/static report construction for that changed discriminator. The
candidate report requires the same-run observed RP1 `SYSINFO_CHIP_ID`
positive-control target at `0x1c00000000`, expected value `0x20001927`, plus
the source-backed `MACB_MID` target at `0x1f001000fc`. The paired control uses
the same report contract but constructs neither the observed RP1 target nor
the Ethernet MMIO target.

This core does not run hardware or accept live GEM visibility. It only makes a
later hardware proof mechanically different from the accepted GEM MID-only
proof. Ethernet driver readiness, broad Ethernet MMIO readiness, RP1 MMIO/DMA
programming, descriptor rings, interrupts, clock/reset ownership, PHY reset
ownership, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
transition work remain unaccepted.

## PHY1 Link-Not-Ready Recovery Checkpoint

phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint-20260614
accepts a source/evidence checkpoint after the read-only PHY1 BMSR
double-sample proof classified link-not-ready. The checkpoint accounts for
BMCR 0x1000, BMSR first/second 0x7949, ANAR 0x01e1, ANLPAR 0x0000, the
corrected-target MDIO read boundary, and the prior GPIO32 no-write and
persistent-or-firmware-owned event-clear blockers.

The selected next objective task id for supervisor planning is
phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract-20260614. That
task is only a source/docs/evidence contract for a future read-only MACB_NSR
NSR_LINK comparator at observed-window address 0x1c00100008 with a paired
no-MMIO/no-Ethernet control. Same-shaped BMSR retries are not progress from
the accepted link-not-ready proof, and this checkpoint does not authorize
hardware action, MACB writes, PHY configuration writes, BMCR writes,
autonegotiation restart, link forcing, GPIO32/PHY reset action, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract-20260614 accepts
the source contract
phase12-rp1-ethernet-macb-nsr-link-readonly-contract-v1. Retained Raspberry
Pi Linux source defines MACB_NSR at offset 0x0008, NSR_LINK as bit 0, and
macb_get_pcs_fixed_state() as a read-only NSR_LINK to phylink state mapping.
With the accepted observed-window rp1_eth base 0x1c00100000, the future
read-only target is 0x1c00100008. The future candidate may only volatile-read
MACB_NSR and decode bit 0, with a paired no-MMIO/no-Ethernet control. This
contract accounts for the accepted PHY1 link-not-ready vector and GPIO32
blockers, and it selects only
phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof-20260614 for supervisor
planning; it does not authorize hardware action, MACB writes, MDIO
transactions, PHY configuration writes, BMCR writes, GPIO32/PHY reset action,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Post-Physical Link Status Closeout

phase12-rp1-ethernet-post-physical-precondition-link-status-closeout-20260614
accepts the post-physical link-status proof only as a source-contract revision
blocker. The Pi 5 candidate/control run retained decisive selected-tree,
same-power-cycle TFTP byte agreement, serial freshness, final identity,
capture-chain-v4, boot-staging identity, and restore evidence. The candidate
runtime report sampled BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR
0x0000, and MACB_NSR 0x00000006, which would otherwise indicate PHY/MAC link
not ready after the confirmed physical-link precondition.

That runtime result is not an accepted planning frontier because the accepted
source contract forbids MAN writes while corrected-target PHY1 reads were
performed through MACB MAN read-command transactions. A supervisor-planned
source-contract revision, no-MAN alternate discriminator, or explicit pause is
required before any fresh link-status proof. This closeout still rejects PHY
configuration, GPIO32/PHY reset action, packet I/O, networking, SSH, Phase
12.2, and phase transition claims.

phase12-rp1-ethernet-post-physical-link-status-man-read-accounting-core-20260615
accepts the revised v2 report boundary for those PHY1 reads. The future
candidate may issue only the five selected Clause 22 PHY1 MAN read-command
stores for BMCR, BMSR first, BMSR second, ANAR, and ANLPAR, then passively read
MACB_NSR. The report surface now records man-read-command-write-count separately
from PHY configuration writes, BMCR writes, MAC configuration writes, GPIO32/PHY
reset action, DMA, and packet I/O; the paired control still constructs no MDIO,
MAN, or MACB target and performs no volatile Ethernet access. The selected
queued follow-up is
phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof-20260615.

phase12-rp1-ethernet-post-physical-link-status-v2-closeout-20260615 accepts
the v2 Pi 5 proof as a bounded phy-not-ready status frontier. The candidate
kept the accepted v2 MAN read-command accounting boundary and reported BMCR
0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006,
BMSR link false, autoneg complete false, ANLPAR nonzero false, and
MACB_NSR_LINK false; the paired control constructed no MDIO/MAN/MACB targets.
The closeout requires supervisor planning for one source-grounded PHY
power/reset/strap/autoneg status recovery discriminator or an explicit pause.
It does not reopen the accepted physical link precondition and does not accept
PHY reset/GPIO32 ownership, PHY configuration, packet I/O, networking, SSH,
Phase 12.2, or a phase transition.

phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint-20260615
reconciles that phy-not-ready frontier with the accepted GPIO32 write/restore,
event-state, and event-clear evidence. The checkpoint does not select a GPIO32
reset-recovery proof: write/restore v2 stopped before GPIO/RIO/pad writes with
event bits 0x0ab00000, and the only accepted IRQRESET clear attempt preserved
CTRL/RIO/pad invariants but left event bits 0x08800000. Retained RP1 pinctrl
source names the event bits and IRQRESET clear mechanism, but does not prove
the remaining bits are harmless for ETH_RST_N ownership or safe to ignore.
The resulting blocker is persistent-or-firmware-owned GPIO32 event state.
Supervisor planning is required for a distinct source-grounded follow-up or an
explicit pause; GPIO32 reset recovery, packet I/O, networking, SSH, Phase 12.2,
and phase transition remain unaccepted.

## GEM MID Decode Discriminator Closeout

phase12-rp1-ethernet-gem-mid-decode-discriminator-closeout-20260610 closes the
local/static discriminator checkpoint and selects the serialized Pi 5 proof as
the next bounded task. The selected candidate archive must capture the same-run
observed RP1 `SYSINFO_CHIP_ID` positive-control load at `0x1c00000000`
alongside the translated `MACB_MID` target at `0x1f001000fc`. The paired
control must use the same reporting path while constructing neither observed
RP1 nor Ethernet MMIO targets.

The closeout only authorizes a later serialized proof with hardwareTestLock
ownership, candidate/control identity, fresh serial cursor, TFTP delta, serial
transcript, final pre-restore identity, restore proof, and task-owned
classification/evidence JSON. It does not accept live GEM visibility, broad
Ethernet MMIO readiness, Ethernet driver readiness, RP1 MMIO/DMA programming,
descriptor rings, interrupts, clock/reset ownership, PHY reset ownership,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## GEM MID Decode Discriminator Pi 5 Proof

phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof-20260610 accepts
only a precise capture-chain blocker. The selected candidate archive was
published under hardwareTestLock and serial output showed the changed
discriminator line: observed RP1 SYSINFO_CHIP_ID returned 0x20001927, while
translated MACB_MID at 0x1f001000fc returned 0xdeaddead, yielding
observed-rp1-positive-control-gem-mid-0x1f-window-sentinel.

That serial line is not accepted as live GEM visibility or Ethernet readiness.
The retained capture did not join into one decisive transaction with stable
TFTP/final-identity/control-marker evidence, and this lab API exposed boot
identity through /boot/files while GET / returned 404. The next step requires
supervisor planning for capture-chain repair or a different bounded acceptance
slice before any same-shaped Pi 5 retry. Ethernet driver readiness, packet
I/O, DMA, interrupts, networking, sockets, SSH, Phase 12.2, and phase
transition work remain unaccepted.

## GEM MID Decode Discriminator V2 Pi 5 Proof

phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof-20260610
accepts the repaired pi5-capture-chain-v4 hardware proof as
observed-rp1-positive-control-gem-mid-0x1f-window-sentinel. The candidate and
paired no-MMIO/no-Ethernet control both joined selected-tree identity, expected
TFTP fetch bytes, run-unique serial marker freshness, final pre-restore
identity, and restore evidence. The candidate observed SYSINFO_CHIP_ID at
0x1c00000000 as 0x20001927 and translated MACB_MID at 0x1f001000fc as
0xdeaddead.

This closes the capture-chain blocker but preserves the hardware blocker: the
accepted result is still the translated 0x1f GEM MID sentinel, not live GEM
visibility. PCIe/RP1 bridge or address-window enablement, Ethernet
clock/reset, PHY/MDIO ownership, interrupts, DMA, descriptor rings, packet
I/O, networking, sockets, SSH, Phase 12.2, and phase transition work remain
unaccepted.

## GEM MID Decode Discriminator V2 Closeout

phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout-20260610 closes
same-shaped GEM MID decode-discriminator hardware retries. The accepted v2
proof repaired the capture chain and decisively classified the current
candidate/control pair, but the hardware frontier remains the translated
0x1f001000fc GEM MID sentinel: same-run SYSINFO_CHIP_ID is visible at
0x1c00000000 as 0x20001927, while MACB_MID remains 0xdeaddead.

The next useful Phase 12.1 step requires supervisor planning around a
different bounded discriminator or a bridge/address-window dependency slice
with explicit acceptance criteria. This closeout does not accept live GEM
visibility, broad Ethernet MMIO readiness, Ethernet driver readiness, packet
I/O, DMA, descriptor rings, interrupts, networking, sockets, SSH, Phase 12.2,
or a phase transition.

## Observed-Window GEM MID Contract

phase12-rp1-ethernet-observed-window-contract-20260610 defines the next
materially different read-only discriminator. It preserves the accepted v2
proof facts that SYSINFO_CHIP_ID is visible at the observed RP1 base
`0x1c00000000` as `0x20001927`, while translated-window `MACB_MID` at
`0x1f001000fc` remains the retained `0xdeaddead` sentinel.

The new candidate target is observed-window `MACB_MID` at `0x1c001000fc`,
computed as observed RP1 base `0x1c00000000` plus the retained
rp1_eth/MACB_MID source offset `0x001000fc`. The prior translated target
`0x1f001000fc` is retained only as a comparator/sentinel. The paired control
must use the same report path while constructing no observed RP1 target,
translated comparator, or Ethernet MMIO target.

This contract remains source/evidence-only and read-only. It does not accept
live GEM visibility, broad Ethernet MMIO readiness, Ethernet driver readiness,
RP1 MMIO writes, DMA, descriptor rings, interrupts, clock/reset ownership,
PHY/MDIO ownership, packet I/O, networking, sockets, SSH, Phase 12.2, or a
phase transition.

## Observed-Window GEM MID Discriminator Core

phase12-rp1-ethernet-observed-window-discriminator-core-20260610 accepts the
local/static report construction for the observed-window contract. The
candidate report preserves the source contract id, SYSINFO_CHIP_ID positive
control at `0x1c00000000` / `0x20001927`, observed-window `MACB_MID` target
`0x1c001000fc`, translated-window comparator `0x1f001000fc`, rp1_eth/MACB_MID
identity, rejected claims, retained risks, and the hardware-proof boundary
classification.

The paired control uses the same discriminator report path while constructing
no SYSINFO, observed-window, translated-comparator, or Ethernet MMIO target and
carries classification
`no-mmio-no-ethernet-rp1-ethernet-observed-window-control`. This remains a
local/static implementation only; it does not accept live GEM visibility,
broad Ethernet MMIO readiness, Ethernet driver readiness, RP1 MMIO writes,
DMA, descriptor rings, interrupts, clock/reset/PHY/MDIO ownership, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Observed-Window GEM MID Discriminator Closeout

phase12-rp1-ethernet-observed-window-discriminator-closeout-20260610
reconciles the accepted contract and local/static report core without
expanding acceptance to hardware visibility or Ethernet readiness. The
checkpoint preserves candidate reporting for SYSINFO_CHIP_ID at
0x1c00000000 / 0x20001927, observed-window MACB_MID at 0x1c001000fc, and
translated-window comparator 0x1f001000fc, plus the paired no-MMIO/no-Ethernet
control path.

The checkpoint selects the serialized Pi 5 observed-window proof as the next
bounded task. That proof must acquire hardwareTestLock and join
candidate/control evidence through selected-tree identity, expected TFTP fetch
bytes, run-unique serial marker freshness, final pre-restore identity, restore
proof, and task-owned JSON. It may classify only an observed-window visible
read, an observed-window sentinel/fault with SYSINFO positive-control retained,
or a precise staging/capture blocker. It still does not accept broad Ethernet
MMIO readiness, Ethernet driver readiness, RP1 MMIO writes, DMA, descriptor
rings, interrupts, clock/reset/PHY/MDIO ownership, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof-20260611 accepts
the serialized read-only Pi 5 discriminator proof as
rp1-ethernet-gpio32-event-state-readonly-pi5-proof-accepted. The candidate
reported GPIO32 STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10, RIO1 IN
0x12, pad 0x56, source-backed event bits 0x0ab00000, writes-performed=false,
and event-clear-performed=false, classifying as
rp1-ethernet-gpio32-event-state-blocked-event-state. The paired control used
the same capture-chain-v4 path with no GPIO32/RIO/pad/MMIO target facts and
classified as no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control.
Both retained selected-tree identity, run-unique serial output, stable TFTP
delta, final pre-restore identity, and restore evidence with v4 checker
classification capture-chain-v4-ready. This proof remains read-only evidence;
it does not accept event clearing, GPIO32 ownership, PHY reset, GPIO32
write/restore retry or success, MDIO/PHY ownership, Ethernet driver behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.

phase12-rp1-ethernet-gpio32-event-state-proof-closeout-20260611 closes that
read-only proof frontier as
rp1-ethernet-gpio32-event-state-proof-blocked-event-state-frontier-closed.
Same-shaped read-only event-state candidate/control hardware retries are
closed for this pair because repeating the accepted report would not decide
whether the source-backed event bits are stale, clearable, firmware-owned,
harmless, or safe to ignore. Any future GPIO32 follow-up requires supervisor
planning for a qualitatively different event-state/source-clearance or
ownership discriminator before event clearing, GPIO/RIO/pad/MMIO writes,
GPIO32 write/restore retry, PHY reset, MDIO/PHY, Ethernet driver behavior,
packet I/O, networking, SSH, Phase 12.2, or phase transition work.

## Observed-Window GEM MID Pi 5 Proof

phase12-rp1-ethernet-observed-window-discriminator-pi5-proof-20260610 accepts
the serialized Pi 5 proof as observed-window-macb-mid-visible. The candidate
joined capture-chain-v4 selected-tree, TFTP, run-unique serial freshness, final
pre-restore identity, and restore gates, then read SYSINFO_CHIP_ID at
0x1c00000000 as 0x20001927 and observed-window MACB_MID at 0x1c001000fc as raw
0x70109, idnum 0x7, rev 0x109. The paired no-MMIO/no-Ethernet control retained
the same capture-chain gates without constructing MMIO targets.

This is a read-only identity discriminator only. It does not accept Ethernet
driver readiness, broad Ethernet MMIO readiness, RP1 MMIO writes, DMA,
descriptor rings, interrupts, clock/reset/PHY/MDIO ownership, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Observed-Window GEM MID Proof Closeout

phase12-rp1-ethernet-observed-window-discriminator-proof-closeout-20260610
closes the observed-window discriminator proof frontier. The accepted boundary
is the read-only identity result only: SYSINFO_CHIP_ID at 0x1c00000000 returned
0x20001927 and observed-window MACB_MID at 0x1c001000fc returned raw 0x70109,
idnum 0x7, rev 0x109, with a paired no-MMIO/no-Ethernet control.

Same-shaped observed-window candidate/control hardware retries are closed for
this pair. A future Phase 12.1 task must be supervisor-planned with explicit
scope and acceptance criteria, such as a source-backed ownership contract for
the next Ethernet prerequisite. This closeout does not accept Ethernet driver
readiness, broad Ethernet MMIO readiness, RP1 MMIO writes, DMA, descriptor
rings, interrupts, clock/reset/PHY/MDIO ownership, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet Prerequisite Ownership Contract

phase12-rp1-ethernet-prereq-ownership-source-contract-20260610 accepts the
next source-backed prerequisite boundary after observed-window MACB_MID
identity. The selected prerequisite is a local/static ownership report for the
Ethernet clocks, reset/PHY/MDIO route, interrupt dependency, and
DMA/descriptor dependency before any driver or packet path.

The contract preserves observed-window MACB_MID at 0x1c001000fc as identity
context only. It reconciles retained Linux source facts: rp1_eth uses
RP1_INT_ETH 6; clocks pclk/hclk/tsu_clk/tx_clk map to RP1_CLK_SYS,
RP1_CLK_SYS, RP1_CLK_ETH_TSU, and RP1_CLK_ETH; RP1_CLK_SYS is 12,
RP1_CLK_ETH is 16, and RP1_CLK_ETH_TSU is 29; Pi 5 enables phy1 at address 1
with RGMII-ID mode and RP1 GPIO32 active-low PHY reset for 5 ms; the Linux
MACB path enables clocks, may toggle the PHY reset GPIO through MDIO reset,
performs MDIO transactions, programs DMA descriptor ring bases, and uses
interrupt/completion handling.

The selected follow-up is local/static report construction only. It selects no
new hardware read field and no write-backed ownership claim, because the next
material prerequisites are clock/reset, GPIO32 PHY reset, MDIO/PHY,
interrupt-completion, DMA, and descriptor-ring ownership, all still unaccepted
by Phase 11 frontiers. The paired control must use the same report path while
withholding the accepted Ethernet prerequisite facts. This contract does not
accept Ethernet driver readiness, broad Ethernet MMIO readiness, RP1 MMIO
writes, clock/reset ownership, GPIO32 or PHY reset ownership, MDIO
transactions, interrupt delivery/completion, DMA, descriptor rings, packet
I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-prereq-ownership-report-core-20260610 implements that
local/static report surface. The candidate report preserves the contract id,
source task id, observed-window MACB_MID identity context, rp1_eth source
window, RP1_INT_ETH 6, pclk/hclk/tsu_clk/tx_clk clock names and clock ids,
RGMII-ID phy1, RP1 GPIO32 active-low reset duration, PHY/MDIO policy, and
DMA/descriptor dependency policy. The paired control uses the same reporting
path while withholding candidate-only Ethernet prerequisite facts and carrying
no-ownership-no-ethernet-rp1-ethernet-prereq-control. This remains
local/static evidence only and does not accept hardware/runtime ownership,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet Prerequisite Ownership Report Closeout

phase12-rp1-ethernet-prereq-ownership-report-closeout-20260610 closes the
local/static prerequisite ownership report frontier. The accepted boundary is
candidate/control report construction only: the candidate carries source-backed
rp1_eth clock, interrupt, PHY reset, PHY/MDIO, DMA/descriptor dependency
metadata plus observed-window MACB_MID identity context, while the paired
control uses the same report path and withholds candidate-only Ethernet
prerequisite facts.

Same-shaped local/static report retries are closed for this candidate/control
pair unless future scope supplies materially different evidence or acceptance
criteria. The closeout selects the serialized Pi 5 prerequisite proof as the
next bounded task, limited to report visibility/control output. It does not
accept Ethernet driver readiness, broad Ethernet MMIO readiness, RP1 MMIO
writes, clock/reset ownership, GPIO32 or PHY reset ownership, MDIO
transactions, interrupt delivery/completion, DMA, descriptor rings, packet
I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet Prerequisite Ownership Pi 5 Proof

phase12-rp1-ethernet-prereq-ownership-pi5-proof-20260610 accepts the
serialized Pi 5 proof as
rp1-ethernet-prereq-ownership-report-visibility-control-output. The candidate
and control capture-chain-v4 identity joins passed. The candidate serial output
printed the accepted prerequisite report fields, including context-only
observed-window MACB_MID identity, RP1_INT_ETH, pclk/hclk/tsu_clk/tx_clk,
RGMII-ID phy1, GPIO32 PHY reset metadata, PHY/MDIO policy, DMA/descriptor
policy, rejected claims, and classification
rp1-ethernet-prereq-ownership-report-visible. The paired control used the same
report path while withholding candidate-only prerequisite facts and classifying
no-ownership-no-ethernet-rp1-ethernet-prereq-control.

This proof accepts report visibility/control output only. It does not accept
hardware/runtime prerequisite ownership, Ethernet driver readiness, broad
Ethernet MMIO readiness, RP1 MMIO writes, clock/reset writes or ownership,
GPIO32/PHY reset ownership, MDIO/PHY ownership, interrupt delivery/completion,
DMA, descriptor rings, packet I/O, networking, sockets, SSH, Phase 12.2, or a
phase transition.

## RP1 Ethernet Prerequisite Ownership Proof Closeout

phase12-rp1-ethernet-prereq-ownership-proof-closeout-20260610 closes the
prerequisite report visibility frontier. The accepted boundary remains only
that the candidate/control report path is visible on Pi 5 serial under
capture-chain-v4: the candidate prints the source-backed prerequisite metadata
and the paired no-ownership/no-Ethernet control withholds candidate-only
facts.

Same-shaped prerequisite report visibility hardware retries are closed for
this candidate/control pair. A future Phase 12.1 task must be
supervisor-planned with different explicit scope and acceptance criteria before
any runtime prerequisite ownership is attempted. This closeout does not accept
Ethernet driver readiness, broad Ethernet MMIO readiness, RP1 MMIO writes,
clock/reset ownership, GPIO32/PHY reset ownership, MDIO/PHY ownership,
interrupt delivery/completion, DMA, descriptor rings, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet Clock/Reset Ownership Contract

phase12-rp1-ethernet-clock-reset-ownership-contract-20260610 defines the next
clock/reset ownership slice as a contract only. The accepted input frontier is
observed-window MACB_MID identity plus prerequisite report visibility/control
output. It does not reinterpret that report as runtime ownership.

The contract identifies the exact source/API surfaces for the next guard:
rp1_eth clock names pclk, hclk, tsu_clk, and tx_clk; clock ids
RP1_CLK_SYS 12 for pclk/hclk, RP1_CLK_ETH_TSU 29 for tsu_clk, and
RP1_CLK_ETH 16 for tx_clk; Linux macb_clk_init as the source enable path; and
the retained Pi 5 rp1_eth node's lack of an accepted reset-controller target.
PHY reset through RP1 GPIO32 remains GPIO/MDIO ownership, not clock/reset
ownership.

Any future write-backed ownership task must first prove a read-only baseline,
map the selected clock id to an exact Talos register target and restore
sequence, avoid disabling or transitioning shared RP1_CLK_SYS through pclk or
hclk, use pre-read/post-read/restore-read evidence, and keep PHY/MDIO,
interrupts, DMA, descriptors, packets, networking, sockets, SSH, Phase 12.2,
and phase transition out of scope. The selected follow-up is only a
local/static clock-reset guard core with a paired no-clock-reset/no-Ethernet
control.

## RP1 Ethernet Clock/Reset Guard Core

phase12-rp1-ethernet-clock-reset-guard-core-20260610 accepts the local/static
guard report construction selected by the ownership contract. The candidate
preserves observed-window MACB_MID identity only as context, source-backed
pclk/hclk/tsu_clk/tx_clk clock names and RP1 clock IDs, shared-clock policy for
RP1_CLK_SYS, absence of an accepted Pi 5 rp1_eth reset-controller target,
read-only baseline requirements, future write-backed invariants, rejected
claims, and retained risks.

The paired control uses the same report path while withholding candidate-only
Ethernet clock/reset facts and carrying
no-clock-reset-no-ethernet-rp1-ethernet-clock-reset-guard-control. Validators
reject guard-contract bypasses and forbidden runtime, hardware, ownership,
downstream Ethernet, and phase claims. This remains local/static evidence only
and does not accept hardware visibility, clock/reset ownership, RP1 MMIO or
clock/reset writes, GPIO32/PHY reset ownership, MDIO/PHY, DMA, descriptors,
interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## RP1 Ethernet Clock/Reset Guard Closeout

phase12-rp1-ethernet-clock-reset-guard-closeout-20260610 closes the
local/static guard frontier. The accepted boundary is candidate/control guard
report construction only; same-shaped local/static guard retries are closed
for this candidate/control pair unless future scope supplies materially
different evidence or acceptance criteria.

The selected next task is the serialized read-only clock/reset baseline Pi 5
proof. That proof may only classify read-only baseline visibility/current
state, a precise sentinel/fault/blocker with identity retained, or a precise
staging/capture blocker. It must not accept clock/reset ownership, RP1 MMIO or
clock/reset writes, Ethernet driver readiness, GPIO32/PHY reset ownership,
MDIO/PHY, DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## RP1 Ethernet Clock/Reset Read-Only Baseline Proof

phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof-20260610 accepts
the serialized Pi 5 read-only baseline report proof as
rp1-ethernet-clock-reset-readonly-baseline-report-visibility-control-output.
Candidate/control capture-chain-v4 joins passed. The candidate selected tree
047815dc8bfde65c28be5d4a5844eb5bf83c4dc60749d7a9c76c8dce402599c3 fetched
da591740/kernel_2712.img at 50056 bytes and retained 19 run-unique serial
markers with observed-window MACB_MID context 0x1c001000fc/raw 0x70109/idnum
0x7/rev 0x109 plus pclk/hclk/tsu_clk/tx_clk baseline facts. The paired control
selected tree 16745426bc0d0f1cc2b1844f48d6e656a8c900afb6fcca42caee5553afc7f4fd
fetched da591740/kernel_2712.img at 49176 bytes and retained 25 run-unique
serial markers while withholding candidate-only clock/reset facts. The lab boot
tree was restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This proof
accepts report visibility/control output only; it does not accept clock/reset
ownership, writes, reset-controller ownership, PHY/MDIO, DMA, packet I/O,
networking, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet Clock/Reset Read-Only Baseline Closeout

phase12-rp1-ethernet-clock-reset-readonly-baseline-closeout-20260610 closes
the read-only baseline proof frontier. The accepted boundary remains only the
candidate/control report visibility proof: the candidate prints the
observed-window MACB_MID identity context plus selected pclk/hclk/tsu_clk and
tx_clk baseline facts, while the paired control uses the same report/capture
path and withholds candidate-only clock/reset facts.

Same-shaped read-only baseline report visibility hardware retries are closed
for this candidate/control pair. Report visibility alone does not make a
write-backed ownership task mechanically objective; the next Phase 12.1
clock/reset ownership slice requires supervisor planning with explicit
source-backed register/restore, shared-clock safety, reset-controller, or
PHY/MDIO/GPIO32 scope and acceptance criteria. This closeout does not accept
clock/reset ownership, RP1 MMIO or clock/reset writes, Ethernet driver
readiness, GPIO32/PHY reset ownership, MDIO/PHY, DMA, descriptors, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet Clock/Reset Write-Target Source Contract

phase12-rp1-ethernet-clock-reset-write-target-source-contract-20260610 accepts
the next source/static write-backed clock/reset slice. The selected
Ethernet-private target is CLK_ETH_TSU_CTRL for RP1_CLK_ETH_TSU / rp1_eth
tsu_clk. The Talos register address is 0x1c00018134, derived from the accepted
observed RP1 base 0x1c00000000, clock-manager base offset 0x018000, and
retained Linux CLK_ETH_TSU_CTRL offset 0x00134.

The future candidate operation is limited to a 32-bit little-endian volatile
pre-read, writing the pre-read raw value back, post-read, restore-write of the
same pre-read raw value, and restore-read. The full raw value, enable bit 11,
auxsource bits 9:5, source bits, and reserved bits must remain preserved. The
paired control must use the same report path while constructing no writable RP1
clock target and performing no volatile load/store to RP1 clock, Ethernet,
reset, GPIO, MDIO, DMA, descriptor, interrupt, PCIe/MIP, GIC, or packet paths.

This contract explicitly rejects shared RP1_CLK_SYS pclk/hclk writes,
CLK_ETH_CTRL, divider/source/PLL/frequency-counter/GPCLK output-enable writes,
reset-controller ownership, GPIO32/PHY reset ownership, MDIO/PHY, DMA,
descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, and
phase transition. It selects the local/static write/restore report core as the
next bounded follow-up; no hardware action or runtime write is accepted by the
source contract itself.

phase12-rp1-ethernet-clock-reset-write-restore-core-20260610 and its closeout
accept only the local/static candidate/control report surface for the selected
CLK_ETH_TSU_CTRL write/restore target. The candidate preserves the exact
0x1c00018134 target, pre-read-raw-only write rule, operation sequence,
preserved fields, safety invariants, retained risks, and future proof
classification set; the paired control uses the same output path while
withholding writable target construction and candidate-only facts. The next
bounded task is the serialized Pi 5 candidate/control write/restore proof with
hardwareTestLock, selected-tree identity, expected TFTP fetches, fresh serial
markers, final pre-restore identity, restore verification, archive reviews,
classification JSON, evidence map, and capture summary. Broad clock/reset
ownership, shared-clock ownership, reset-controller, GPIO32/PHY, MDIO, DMA,
descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, and
phase transition remain unaccepted.

phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof-20260610 accepts the
serialized Pi 5 candidate/control proof for that exact target only. The
candidate reported CLK_ETH_TSU_CTRL at 0x1c00018134 with pre_raw,
post_raw, and restore_raw all 0x10000800, post_eq_pre=true, restore_eq_pre=true,
and classification rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored.
The paired control used the same capture/report path and classified as
no-clock-write-no-ethernet-rp1-ethernet-write-restore-control. Capture-chain-v4
joined selected boot-tree identity, expected TFTP fetches, fresh serial nonce
markers, final pre-restore identity, and restore proof for both runs. This
accepts one idempotent Ethernet-private clock register write/restore proof; it
does not accept broad clock/reset ownership, shared-clock ownership,
reset-controller, GPIO32/PHY reset, MDIO/PHY, DMA, descriptors, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet Clock/Reset Write-Restore Proof Closeout

phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout-20260610 closes
the write/restore proof frontier as
rp1-ethernet-clk-eth-tsu-ctrl-write-restore-frontier-closed. The accepted
boundary remains exactly one Ethernet-private CLK_ETH_TSU_CTRL idempotent
write/readback/restore proof with a paired no-clock-write control. It does
not reinterpret that result as broad clock/reset ownership or as Ethernet
driver readiness.

Same-shaped CLK_ETH_TSU_CTRL idempotent write/restore hardware retries are
closed for this candidate/control pair. A future Phase 12.1 task must be
supervisor-planned with materially different explicit scope and acceptance
criteria, such as a non-idempotent field transition with restore proof, a
separate CLK_ETH_CTRL or shared-clock safety contract, reset-controller
evidence, GPIO32/PHY reset ownership, MDIO/PHY ownership, interrupt
completion, DMA/descriptor ownership, or packet I/O scope. This closeout does
not accept broad clock/reset ownership, shared-clock ownership, CLK_ETH_CTRL,
reset-controller, GPIO32/PHY reset, MDIO/PHY, DMA, descriptors, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet CLK_ETH_CTRL Source Contract

phase12-rp1-ethernet-clk-eth-ctrl-source-contract-20260610 accepts the next
source/static Ethernet-private clock target contract. The selected target is
CLK_ETH_CTRL for RP1_CLK_ETH / rp1_eth tx_clk. The Talos register address is
0x1c00018064, derived from the accepted observed RP1 base 0x1c00000000,
clock-manager base offset 0x018000, and retained Linux CLK_ETH_CTRL offset
0x00064.

The future candidate operation is limited to a 32-bit little-endian volatile
pre-read, writing the pre-read raw value back, post-read, restore-write of the
same pre-read raw value, and restore-read. The full raw value, enable bit 11,
auxsource bits 9:5, source bits, and reserved bits must remain preserved. This
is materially different from the accepted CLK_ETH_TSU_CTRL proof because it
selects the direct rp1_eth transmit clock control register instead of the
timestamp-unit control register.

The paired control must use the same report path while constructing no
writable RP1 clock target and performing no volatile load/store to RP1 clock,
Ethernet, reset, GPIO, MDIO, DMA, descriptor, interrupt, PCIe/MIP, GIC, or
packet paths. This contract explicitly rejects shared RP1_CLK_SYS pclk/hclk
writes, same-shaped TSU retries, non-idempotent field transitions,
divider/source/PLL/frequency-counter/GPCLK output-enable writes,
reset-controller ownership, GPIO32/PHY reset ownership, MDIO/PHY, DMA,
descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, and
phase transition. It selects only the local/static CLK_ETH_CTRL write/restore
core as the next bounded follow-up; no hardware action or runtime write is
accepted by the source contract itself.

phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core-20260610 accepts that
local/static report surface in src/rp1_ethernet.rs. The candidate report
preserves the CLK_ETH_CTRL contract identity, 0x1c00018064 target address,
pre-read-raw-only operation order, preserved fields, rejected claims, retained
risks, and future proof vocabulary. The paired control uses the same report
path while constructing no writable target and withholding candidate-only
CLK_ETH_CTRL facts. The accepted evidence remains fmt/unit-test/static only:
no hardware run, no boot archive publication, no hardwareTestLock, no runtime
RP1 MMIO write, no shared-clock ownership, no Ethernet driver readiness, no
packet I/O, no networking, no SSH, no Phase 12.2, and no phase transition.

phase12-rp1-ethernet-clk-eth-ctrl-write-restore-closeout-20260610 closes the
local/static CLK_ETH_CTRL report frontier as
rp1-ethernet-clk-eth-ctrl-write-restore-core-frontier-closed. Same-shaped
local/static CLK_ETH_CTRL write/restore report retries are closed for this
candidate/control pair. The closeout selects the serialized Pi 5
candidate/control proof as the next bounded task because the accepted
contract/core and queued proof already define the exact target, paired
control, capture gates, restore requirements, allowed classifications, and
task-owned evidence artifacts.

This closeout does not accept runtime RP1 MMIO writes, hardware readback,
broad clock/reset ownership, shared-clock ownership, reset-controller
ownership, GPIO32/PHY reset, MDIO/PHY, DMA, descriptors, interrupts, packet
I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof-20260610 accepts
the serialized Pi 5 CLK_ETH_CTRL write/restore proof as
rp1-ethernet-clk-eth-ctrl-idempotent-write-restored-with-control. The accepted
candidate rerun joined capture-chain-v4 selected-tree identity
8d71d54345a64913e451969b9303cd7df351baa64950dffd2fca890897cf05b3, two
matching TFTP fetches of da591740/kernel_2712.img at 50040 bytes, run-unique
serial nonce freshness, final pre-restore identity, and restore proof. The
candidate serial reported CLK_ETH_CTRL at 0x1c00018064 with pre_raw
0x10000800, post_raw 0x10000800, restore_raw 0x10000800, post_eq_pre=true,
restore_eq_pre=true, and classification
rp1-ethernet-clk-eth-ctrl-idempotent-write-restored. The paired control joined
selected-tree identity 5c5144ce68c0537b39dcb216b2ae1343c9197ac7deb310f5c7bcc811efe31d40,
two matching TFTP fetches at 49464 bytes, run-unique serial freshness, final
identity, and restore proof while withholding writable CLK_ETH_CTRL target
construction with classification
no-clock-write-no-ethernet-rp1-ethernet-clk-eth-ctrl-control.

The proof retains two non-acceptance candidate attempts: a short-window
inconclusive capture and a staging-blocked attempt whose preflight and serial
marker showed the candidate but whose TFTP/final identity rejoined the
baseline tree. The paired control and unchanged candidate rerun completed the
required inconclusive-run triage. Final restore returned the lab boot tree to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136 bytes.
This proof does not accept broad clock/reset ownership, shared-clock
ownership, reset-controller ownership, GPIO32/PHY reset, MDIO/PHY, DMA,
descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or
a phase transition.

## RP1 Ethernet CLK_ETH_CTRL Write-Restore Proof Closeout

phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout-20260610 closes
the CLK_ETH_CTRL proof frontier as
rp1-ethernet-clk-eth-ctrl-write-restore-frontier-closed. The accepted boundary
remains exactly one Ethernet-private CLK_ETH_CTRL idempotent
write/readback/restore proof with a paired no-clock-write control. It does not
reinterpret that result as broad clock/reset ownership, shared-clock
ownership, or Ethernet driver readiness.

Same-shaped CLK_ETH_CTRL idempotent write/restore hardware retries are closed
for this candidate/control pair. A future Phase 12.1 task must be
supervisor-planned with materially different explicit scope and acceptance
criteria, such as a functional non-idempotent field transition with restore
proof, shared-clock safety, reset-controller evidence, GPIO32/PHY reset
ownership, MDIO/PHY ownership, interrupt completion, DMA/descriptor ownership,
or packet I/O scope. This closeout does not accept broad clock/reset
ownership, shared-clock ownership, reset-controller, GPIO32/PHY reset, MDIO,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.

## RP1 Ethernet GPIO32 PHY-Reset Source Contract

phase12-rp1-ethernet-gpio32-phy-reset-source-contract-20260610 accepts the
next source/static GPIO32 / ETH_RST_N prerequisite boundary. Retained Pi 5
device-tree source identifies `rp1_eth` `phy-reset-gpios` as `rp1_gpio` line
32 with `GPIO_ACTIVE_LOW` polarity and `phy-reset-duration = <5>`. Retained
Linux MACB source obtains the optional `"phy-reset"` GPIO as `GPIOD_OUT_LOW`,
installs `macb_mdio_reset` as the MDIO bus reset hook, asserts logical value
1, sleeps for the configured duration, then deasserts logical value 0. Because
the route is active-low, logical assertion drives ETH_RST_N physically low and
logical deassertion drives it physically high.

The selected follow-up is only a local/static read-only GPIO32 PHY-reset
preflight report core with a paired no-GPIO/no-Ethernet control. Candidate
fields must preserve the source contract id, observed-window MACB_MID identity
context, accepted prerequisite and clock proof closeouts, `rp1_eth`/`phy1`
identity, GPIO32/ETH_RST_N polarity and duration facts, Linux MDIO reset hook
relationship, Phase 11 GPIO constraints, rejected claims, and retained risks.
This contract does not accept GPIO ownership, PHY reset assertion/deassertion,
MDIO/PHY ownership, runtime Ethernet readiness, RP1 MMIO/GPIO/RIO/pad/INTE/
CTRL writes, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

phase12-rp1-ethernet-gpio32-phy-reset-preflight-core-20260610 accepts that
local/static report surface in `src/rp1_ethernet.rs`. The candidate report
preserves the GPIO32 source contract id, accepted input frontier, `rp1_eth`
and `phy1` identities, `rp1_gpio` line 32 / ETH_RST_N route, active-low
logical assertion/deassertion mapping, 5 ms source reset duration, Linux MACB
MDIO reset hook relationship, Phase 11 GPIO constraints, future write/restore
safety invariants, rejected claims, and retained risks. The paired control
uses the same report path while constructing no GPIO32/ETH_RST_N/PHY-reset
target and withholding candidate-only facts.

The accepted evidence remains fmt/unit-test/static only: no hardware run, no
boot archive publication, no hardwareTestLock, no runtime RP1 MMIO/GPIO/RIO/
pad write, no PHY reset assertion/deassertion, no MDIO/PHY ownership, no
Ethernet driver readiness, no packet I/O, no networking, no SSH, no Phase
12.2, and no phase transition.

phase12-rp1-ethernet-gpio32-phy-reset-preflight-closeout-20260610 closes the
local/static preflight frontier as
rp1-ethernet-gpio32-phy-reset-preflight-frontier-closed. Same-shaped
local/static GPIO32 PHY-reset preflight report retries are closed for this
candidate/control pair. The next bounded follow-up is only the serialized
read-only Pi 5 preflight proof, gated by hardwareTestLock and the existing
candidate/control capture requirements. This closeout does not select a
write-backed GPIO32 reset task and does not accept GPIO ownership, PHY reset
assertion/deassertion, MDIO/PHY ownership, packet I/O, networking, SSH, Phase
12.2, or a phase transition.

phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof-20260610
accepts the serialized Pi 5 read-only preflight visibility/control proof as
rp1-ethernet-gpio32-phy-reset-readonly-preflight-visible-with-control. The
candidate joined capture-chain-v4 selected-tree identity
25933d095429b5b91ab2185caa1e5c2ce586346452d838a853dbebacea5c4ba7, two
matching TFTP fetches of da591740/kernel_2712.img at 49528 bytes, run-unique
serial freshness, final pre-restore identity, and restore proof. Its serial
line retained the accepted GPIO32 / ETH_RST_N preflight fields and
classification
rp1-ethernet-gpio32-phy-reset-readonly-preflight-report-visible.

The paired no-GPIO/no-Ethernet control joined selected-tree identity
ddd753ab2040cdadde6a6b665b24a96886db2377be76bac006806ea035907bda, two
matching TFTP fetches at 48688 bytes, run-unique serial freshness, final
identity, and restore proof while withholding candidate-only GPIO32/PHY-reset
facts with classification
no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control. Final restore
returned the lab boot tree to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136 bytes.
This proof does not accept GPIO ownership, PHY reset assertion/deassertion,
MDIO/PHY ownership, GPIO/RIO/pad/MMIO writes, Ethernet driver readiness,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-proof-closeout-20260610
closes the read-only GPIO32 PHY-reset preflight proof frontier as
rp1-ethernet-gpio32-phy-reset-readonly-preflight-frontier-closed.
Same-shaped GPIO32 PHY-reset read-only preflight hardware retries are closed
for this candidate/control pair. A future task must provide materially
different scope and explicit acceptance criteria, such as GPIO32 write/restore
ownership with restore proof, MDIO/PHY ownership, interrupt completion,
DMA/descriptor ownership, or packet I/O. This closeout does not select such a
task and does not accept GPIO ownership, PHY reset assertion/deassertion,
MDIO/PHY ownership, RP1 GPIO/RIO/pad/MMIO writes, Ethernet driver readiness,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.

## RP1 Ethernet GPIO32 PHY-Reset Write/Restore Source Contract

phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-20260610
accepts the source/docs/evidence contract for a future bounded GPIO32 /
ETH_RST_N write/restore ownership proof. GPIO32 is RP1 GPIO bank1,
bank-local bit 4. The observed-aperture targets are GPIO32 STATUS at
0x1c000d4020, GPIO32 CTRL at 0x1c000d4024, RIO1 OUT/OE/IN at
0x1c000e4000/0x1c000e4004/0x1c000e4008, and GPIO32 pad state at
0x1c000f4014. Active-low ETH_RST_N assertion maps to raw GPIO32 output low,
deassertion maps to raw output high, and the retained source duration remains
5 ms.

The future candidate is allowed only after a local/static guard and later
serialized proof are explicitly queued. It must capture all touched register
baselines, no-write on sentinel/all-ones/unsafe/inconclusive reads, unsafe
function or route state, unexpected event/interrupt state, missing restore
baseline, or capture-chain uncertainty, then assert, wait, deassert, restore
every touched field to baseline, and verify restore readback. The paired
control must use the same report path while constructing no GPIO32/RIO/pad
writable target and performing no volatile store.

This contract selects the local/static GPIO32 write/restore guard core as the
next bounded follow-up. It does not implement or run hardware, does not accept
GPIO ownership, function-change ownership, pad-write ownership, PHY reset
assertion/deassertion, MDIO/PHY ownership, interrupt/event ownership, Ethernet
driver readiness, broad Ethernet MMIO readiness, DMA, descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## RP1 Ethernet GPIO32 PHY-Reset Write/Restore Guard Core

phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-core-20260610
accepts the local/static report surface in src/rp1_ethernet.rs. The candidate
report carries the exact GPIO32 / ETH_RST_N target identity, no-write
preconditions, restore-baseline fields, operation order, blocked/no-write
classifications, allowed future proof classifications, rejected claims, and
retained risks from the accepted source contract.

The paired control uses the same report construction path while carrying no
writable GPIO32/RIO/pad/MMIO target facts and classifies as
no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control. A
blocked/no-write report kind is also accepted for explicit precondition
failures such as sentinel reads, unsafe function state, unexpected event
state, missing restore baseline, or capture-chain inconclusive evidence. The
guard validators reject missing restore baseline, non-GPIO32 writes, MDIO/PHY
overclaims, interrupt/DMA/descriptor/packet/network/socket/SSH claims, Phase
12.2, and phase transition.

This guard core does not run hardware, acquire hardwareTestLock, publish a
boot archive, perform a volatile store, assert/deassert PHY reset, or accept
Ethernet driver readiness. The next bounded step is the static guard closeout
before any serialized Pi 5 proof can be authorized.

## RP1 Ethernet GPIO32 PHY-Reset Write/Restore Guard Closeout

phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-closeout-20260610
closes the local/static GPIO32 / ETH_RST_N write/restore report frontier as
rp1-ethernet-gpio32-phy-reset-write-restore-guard-frontier-closed.
Same-shaped local/static report retries are closed for this candidate/control
pair. The accepted source contract and guard core now define the exact target
identity, no-write preconditions, restore baseline fields, operation order,
paired no-GPIO-write/no-Ethernet control, allowed classifications, rejected
claims, and evidence artifacts needed by the next proof.

The selected next boundary is the serialized Pi 5 candidate/control
write/restore proof. That proof must acquire hardwareTestLock before archive
publication, staging, power cycling, or any runtime GPIO/RIO/pad/MMIO write,
and it must join selected-tree identity, expected TFTP fetches, fresh serial
markers, final pre-restore identity, restore proof, classification JSON, and
evidence map in one retained capture-chain transaction. This closeout does not
accept MDIO/PHY ownership, Ethernet driver readiness, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## RP1 Ethernet GPIO32 PHY-Reset Write/Restore Proof Closeout

phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof-20260610 is
closed as a committed lab-power-cycle-no-fetch blocker, not as a GPIO32 /
ETH_RST_N ownership proof. The candidate and paired no-MMIO/no-GPIO control
archives passed static review and reached selected-tree identity, but the
candidate retained no accepted marker with zero sampled TFTP events. The
paired control then produced zero TFTP events and only NUL+newline serial
output after successful /power/cycle.

The restored known-good control tree also produced zero TFTP events and only
NUL+newline serial output after /power/cycle, and the final boot tree was
restored to a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
with effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
bytes before hardwareTestLock release. Therefore
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-proof-closeout-20260610
holds same-shaped GPIO32 write/restore hardware retries until known-good
power-cycle TFTP and serial output recover, or a separate lab-recovery task is
planned.

phase12-pi5-lab-known-good-power-cycle-recovery-20260610 narrows that blocker:
the restored known-good boot tree still matched
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136 bytes,
and the authorized known-good power cycle recovered TFTP fetches with 13
events including two 104136-byte da591740/kernel_2712.img transfers. Serial
observe from fresh cursor 4194304 captured 0 bytes and no expected Talos
output, so the accepted recovery classification is
known-good-power-cycle-tftp-recovered-serial-silent-blocker. Same-shaped
GPIO32 write/restore retries remain held until a bounded serial-silent
known-good boot discriminator or another explicit recovery gate is accepted.

This checkpoint does not accept GPIO32 ownership, PHY reset
assertion/deassertion, MDIO/PHY ownership, RP1 GPIO/RIO/pad/MMIO writes,
Ethernet driver readiness, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-pi5-runtime-readiness-helper-saturation-core-20260610 repairs the
known-good runtime-readiness helper contract at the saturated 4 MiB serial
cursor boundary. Saturated cursors now route to bounded direct serial reads or
an explicit saturated-cursor capture blocker instead of an unqualified empty
cursor-observe-only readiness classification. The helper still accepts valid
known-good Talos readiness only when the configured readiness markers are
present.

phase12-pi5-known-good-bounded-runtime-readiness-pi5-proof-20260610 confirms
that restored-tree identity and TFTP fetches recovered for the known-good
tree, including two 104136-byte da591740/kernel_2712.img fetches, but it
does not accept valid known-good Talos readiness. The primary derived helper
summary saw the downstream production-timer PASS marker under the repaired
direct-read contract, but the raw primary helper JSON was overwritten by a
follow-up read and the retained raw helper artifact classifies as
saturated-cursor-capture-blocked.

phase12-pi5-known-good-bounded-runtime-readiness-closeout-20260610 therefore
closes this checkpoint as
known-good-readiness-evidence-retention-blocker-closeout-accepted. GPIO32
write/restore v2 remains held; the selected next boundary is the queued serial
endpoint readiness follow-up, which must resolve whether the remaining blocker
is serial endpoint/capture readability, restored-tree runtime output absence,
or lab/human intervention. This closeout does not accept GPIO32 ownership, PHY
reset assertion/deassertion, MDIO/PHY ownership, RP1 GPIO/RIO/pad/MMIO writes,
Ethernet driver readiness, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-pi5-runtime-readiness-evidence-retention-core-20260610 repairs the
primary runtime-readiness artifact retention path so follow-up serial reads
cannot overwrite the primary helper JSON. The follow-up v2 known-good readiness
proof retains the primary helper artifact under a run-label-qualified path and
derives the summary from that artifact. The run again proves stable restored
known-good identity and TFTP fetches, and it observes
rpi5-production-timer-preemption: PASS, but the retained helper artifact does
not contain TALOS: kernel_main and therefore still does not accept
valid-known-good-talos-readiness.

phase12-pi5-known-good-bounded-runtime-readiness-v2-closeout-20260610 closes
that v2 proof as
known-good-fetch-pass-marker-observed-helper-readiness-unaccepted-closeout.
GPIO32 write/restore v2 remains blocked because its valid-known-good readiness
dependency is still unsatisfied. Same-shaped GPIO32 or known-good runtime
readiness hardware retries require supervisor planning with a changed
discriminator or changed readiness contract.

phase12-pi5-known-good-bounded-runtime-readiness-v3-closeout-20260611 accepts
valid-known-good-talos-readiness-v3 under the changed readiness contract. The
accepted proof retains stable known-good boot identity, stable TFTP evidence,
and the rpi5-production-timer-preemption: PASS marker in the primary serial
artifact. The absence of TALOS: kernel_main is recorded as v3 metadata rather
than a rejection. This closeout mechanically selected only the already queued
GPIO32 / ETH_RST_N write/restore v2 proof; it did not accept GPIO32 ownership,
PHY reset behavior, MDIO/PHY ownership, Ethernet driver behavior, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof-20260610 is
accepted only as a precise blocked/no-write hardware result. The candidate and
paired no-GPIO/no-MMIO control both retained selected-tree identity, run-unique
serial markers, stable 13-event TFTP deltas, and final restore proof, so the
earlier lab no-fetch blocker is closed for this v2 attempt. The candidate did
not perform the GPIO32 write/restore sequence: it reported
writes-performed=false after observing baseline-status=0xabe3300,
baseline-ctrl=0x85, baseline-out=0x10, baseline-oe=0x10, baseline-in=0x12,
and event-bits=0xab00000.

phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout-20260610
closes that proof frontier as
rp1-ethernet-gpio32-phy-reset-write-restore-v2-blocked-no-write-frontier-closed.
Same-shaped GPIO32 write/restore hardware retries are closed for this
candidate/control pair. A future GPIO32 follow-up requires supervisor planning
for a qualitatively different discriminator, such as event-state/source
clearance or equivalent pre-write ownership conditions. GPIO32 ownership, PHY
reset assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2,
and phase transition remain unaccepted.

## RP1 Ethernet GPIO32 Event-State Source Contract

phase12-rp1-ethernet-gpio32-event-state-source-contract-20260611 accepts only
the source/docs/evidence contract for a read-only GPIO32 event-state
discriminator after the accepted GPIO32 write/restore v2 blocked/no-write
result. The candidate preserves the v2 blocker lineage, exact GPIO32 /
ETH_RST_N target identity, and observed-aperture read set: GPIO32 STATUS at
0x1c000d4020, GPIO32 CTRL at 0x1c000d4024, RIO1 OUT/OE/IN at
0x1c000e4000/0x1c000e4004/0x1c000e4008, and GPIO32 pad at 0x1c000f4014. No
write, event clear, IRQRESET, INTE/CTRL mutation, RIO/pad mutation, PHY reset
assertion/deassertion, or write/restore retry is accepted.

Retained RP1 pinctrl source backs only GPIO STATUS event-state bit names for
bits 20-27: raw falling/rising/low/high and filtered
falling/rising/low/high. The follow-up discriminator must report any
unretained stale/clearable/owned/harmless interpretation as
source-unresolved-event-state rather than inferring it. The paired control
must preserve the same report path while constructing no GPIO32/RIO/pad/MMIO
target facts and classifying as
no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control.

The next bounded step is only the local/static event-state discriminator core.
This contract does not accept GPIO32 ownership, event clearing, PHY reset
assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior,
interrupt delivery/completion, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-gpio32-event-state-discriminator-closeout-20260611 closes
the local/static discriminator frontier as
rp1-ethernet-gpio32-event-state-discriminator-static-frontier-closed. The
accepted core preserves candidate GPIO32 STATUS/CTRL, RIO1 OUT/OE/IN, pad
target identity, v2 blocked/no-write lineage, source-backed STATUS event bits
20-27, source-unresolved semantics for stale/clearable/owned/harmless
interpretations, rejected claims, and the paired no-GPIO/no-Ethernet control
path. The checkpoint selects only the serialized read-only Pi 5 proof, which
must acquire hardwareTestLock before archive publication, staging, or power
action and must not clear events or write GPIO/RIO/pad/MMIO. This closeout
does not accept hardware evidence, GPIO32 ownership, GPIO32 write/restore
retry or success, PHY reset assertion/deassertion, MDIO/PHY ownership,
Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-gpio32-event-clear-source-contract-20260611 accepts the
source-backed contract for a future GPIO32 event-clear discriminator. Retained
RP1 pinctrl source clears latched events by writing RP1_GPIO_CTRL_IRQRESET
(BIT(28), value 0x10000000) to the per-pin GPIO CTRL SET alias. For GPIO32,
the selected future write target is observed address 0x1c000d6024, derived
from GPIO32 CTRL observed address 0x1c000d4024 plus RP1_SET_OFFSET 0x2000.

The contract is limited to a future guarded proof. Before that write, the
candidate must re-read GPIO32 STATUS/CTRL, RIO1 OUT/OE/IN, and pad state,
prove STATUS & 0x0ff00000 == 0x0ab00000, and preserve the accepted
candidate/control evidence lineage from the read-only event-state proof. After
the write, the proof must show STATUS event bits cleared or classify
persistent/source-owned event state while preserving GPIO32 CTRL non-IRQRESET
fields, RIO1 OUT/OE/IN, and pad state. The paired control must use the same
report path while constructing no GPIO32/RIO/pad/MMIO target facts.

This event-clear source contract does not accept hardware evidence, GPIO32
ownership, GPIO32 write/restore retry or success, PHY reset
assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior,
interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition. The next bounded step is only the
local/static event-clear guard core.

phase12-rp1-ethernet-gpio32-event-clear-guard-core-20260611 accepts the
local/static event-clear guard report surface for that source contract. The
candidate report preserves the exact GPIO32 CTRL SET-alias IRQRESET target
0x1c000d6024, write value 0x10000000, STATUS event mask 0x0ff00000,
accepted event bits 0x0ab00000, pre-read and post-read requirements,
forbidden writes, accepted event-state lineage, rejected claims, retained
risks, and source evidence. The paired control uses the same report path while
withholding GPIO32/RIO/pad/MMIO target facts and classifying as
no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control.

phase12-rp1-ethernet-gpio32-event-clear-guard-closeout-20260611 closes the
local/static event-clear guard frontier as
rp1-ethernet-gpio32-event-clear-guard-static-frontier-closed. Same-shaped
local/static guard retries are closed for this candidate/control pair. The
checkpoint selects only the serialized Pi 5 event-clear proof, which must
acquire hardwareTestLock before archive publication, staging, power action, or
the guarded event-clear attempt and must preserve candidate/control identity,
TFTP, serial freshness, final identity, restore proof, and task-owned JSON.

This guard closeout does not accept hardware evidence, event clearing, GPIO32
ownership, GPIO32 write/restore retry or success, PHY reset
assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior,
interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

phase12-rp1-ethernet-gpio32-event-clear-pi5-proof-20260611 accepts the
serialized event-clear proof as an event-clear persistent/firmware-owned
blocker. The candidate passed capture-chain-v4, matched the accepted pre-state
STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10, RIO1 IN 0x12, pad 0x56, and
event bits 0x0ab00000, then wrote only GPIO32 CTRL SET IRQRESET value
0x10000000. Post-readback preserved CTRL/RIO/pad invariants but retained
event bits 0x08800000, so event clearing did not prove ownership or clear the
source-backed event state. The accepted paired control-rerun2 passed the same
capture-chain-v4 path with no GPIO32/RIO/pad/MMIO target construction and no
event clear.

This proof accepts only the persistent/source-owned event-state blocker with
no-reset/no-output invariants preserved. It does not accept GPIO32 ownership,
PHY reset assertion/deassertion, GPIO32 write/restore retry or success,
MDIO/PHY ownership, Ethernet driver behavior, interrupt completion,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

phase12-rp1-ethernet-gpio32-event-clear-proof-closeout-20260611 closes the
event-clear proof frontier as
rp1-ethernet-gpio32-event-clear-persistent-or-firmware-owned-frontier-closed.
Same-shaped event-clear hardware retries are closed for this candidate/control
pair. The accepted proof already showed the guarded IRQRESET write preserved
CTRL/RIO/pad/no-output invariants while event bits persisted, so repeating the
same write would not prove GPIO32 ownership, PHY reset ownership, or whether
firmware owns or reasserts the event state.

Future GPIO32 ownership, GPIO32 write/restore retry, PHY reset, MDIO/PHY,
Ethernet driver, interrupt, DMA/descriptor, packet I/O, networking, socket,
SSH, Phase 12.2, or phase-transition work requires supervisor planning with a
qualitatively different discriminator or explicit ownership contract. This
closeout does not accept those claims.

phase12-rp1-ethernet-clock-reset-prereq-closeout-20260611 reconciles the
accepted clock/reset prerequisite frontier after the GPIO32 blocker evidence.
The accepted boundary is exactly observed-window MACB_MID read-only identity
context, prerequisite report visibility/control output, and two
Ethernet-private idempotent write/readback/restore proofs: CLK_ETH_TSU_CTRL at
0x1c00018134 and CLK_ETH_CTRL at 0x1c00018064, each with paired no-clock-write
controls and pre/post/restore raw value 0x10000800. This checkpoint does not
accept broad clock/reset ownership, shared-clock ownership, reset-controller
ownership, GPIO32/PHY reset ownership, runtime MDIO/PHY ownership, Ethernet
driver readiness, interrupts, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

Same-shaped CLK_ETH_TSU_CTRL and CLK_ETH_CTRL idempotent write/restore retries
remain closed, and same-shaped GPIO32 write/restore and event-clear retries
remain closed. The selected next bounded Phase 12.1 task is only
phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611 as source/docs/
evidence contract work for a distinct non-GPIO32 prerequisite before packet
I/O. It must not perform runtime MDIO transactions, assert/deassert PHY reset,
retry GPIO32 event clear or write/restore, implement Ethernet behavior,
program DMA or descriptors, handle interrupts, perform packet I/O, add
networking, sockets, SSH, Phase 12.2, or create a phase transition.

phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611 accepts the
source/docs/evidence contract for the smallest useful non-GPIO32 MDIO/PHY-ID
discriminator. The selected future candidate is a paired Clause 22 PHY-ID read
for rp1_eth phy1 / ethernet-phy@1 address 1, using MII_PHYSID1 register 0x02
and MII_PHYSID2 register 0x03. The source-backed observed-window targets are
NCR at 0x1c00000000, NSR at 0x1c00000008, and MAN at 0x1c00000034. A future
candidate must require NCR.MPE bit 4 already set or classify
source-contract-violated-blocker without writing NCR; it must poll NSR.IDLE
bit 2 before and after each MAN write; and it must extract 16-bit results from
MAN.DATA bits 15:0. The exact future MAN frames are 0x600a0000 for PHYSID1
and 0x600e0000 for PHYSID2.

The paired control must use the same reporting path while constructing no
MDIO target or MAN frame and performing no Ethernet MDIO volatile load/store.
This contract does not execute MDIO, does not accept NCR.MPE write ownership,
does not require GPIO32/PHY reset success, and does not accept full MDIO/PHY
ownership, Ethernet driver readiness, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-mdio-phy-id-pi5-proof-20260611 accepts the serialized
Pi 5 MDIO/PHY-ID proof as a no-write source-contract blocker. Candidate and
paired no-MDIO control both passed capture-chain-v4 with selected-tree
identity, run-unique serial markers, stable TFTP deltas, final pre-restore
identity, and restore evidence. The candidate observed MACB_MID context
0x70109 at 0x1c001000fc and NCR 0x20001927 at 0x1c00000000, but NCR.MPE bit
4 was clear, so it classified as
mdio-phy1-physid-source-contract-violated-blocker without writing NCR, MAN,
or GPIO32/PHY reset state. The paired control constructed no MDIO target and
classified as no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control. This
proof accepts no visible PHY-ID read, runtime MDIO transaction, NCR.MPE write
ownership, broad MDIO/PHY ownership, PHY reset ownership, Ethernet driver
readiness, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or phase transition.

phase12-rp1-ethernet-mdio-phy-id-proof-closeout-20260611 closes that proof
frontier as rp1-ethernet-mdio-phy-id-ncr-mpe-clear-frontier-closed.
Same-shaped MDIO PHY-ID hardware retries are closed for this candidate/control
pair because repeating the guarded no-write discriminator cannot make NCR.MPE
set, prove visible PHY-ID reads, or grant NCR.MPE write ownership. The accepted
boundary remains only the capture-chain-v4 no-write blocker: NCR.MPE bit 4 was
clear, no NCR or MAN write occurred, claims-runtime-mdio-transaction=false, and
touched-fields=none.

Future NCR.MPE enablement, PHY reset, MDIO/PHY ownership, Ethernet driver,
interrupt, DMA/descriptor, packet I/O, networking, socket, SSH, Phase 12.2, or
phase-transition work requires supervisor planning with a qualitatively
different discriminator or explicit source-backed ownership contract. This
closeout does not accept those claims or select a follow-up task.

phase12-rp1-ethernet-mdio-mpe-enable-source-contract-20260611 accepts the
source/docs/evidence contract for the smallest NCR.MPE enable/readback/restore
prerequisite. The selected target is MACB/GEM NCR at observed-window
0x1c00100000, derived from rp1_eth offset 0x00100000 and MACB_NCR offset
0x0000, with MPE bit 4 and mask 0x00000010. A future candidate may only
pre-read NCR, write pre_raw | 0x00000010, read back MPE set state,
restore-write exact pre_raw, and restore-read exact pre_raw. The paired
control must use the same reporting path while constructing no NCR target and
performing no volatile load/store.

This contract does not run hardware and does not accept runtime MPE write
success, MAN writes, PHY-ID reads, broad MDIO/PHY ownership, PHY reset/GPIO32
ownership, Ethernet driver readiness, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition. Same-shaped
MDIO PHY-ID retries remain closed until this distinct NCR.MPE ownership path is
separately implemented and accepted.

phase12-rp1-ethernet-mdio-mpe-enable-guard-core-20260611 accepts the
local/static NCR.MPE enable guard report surface for that source contract. The
candidate report preserves source contract id
phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1, guard report contract
id phase12-rp1-ethernet-mdio-mpe-enable-guard-report-contract-v1, observed
MACB/GEM NCR target 0x1c00100000, MPE bit 4, mask 0x00000010, write rule
pre_raw | 0x00000010, exact pre_raw restore invariant, allowed future proof
classifications, rejected claims, retained risks, and source evidence. The
paired control uses the same report path while constructing no NCR/MPE target
and no write intent.

phase12-rp1-ethernet-mdio-mpe-enable-guard-closeout-20260611 closes the
local/static NCR.MPE guard frontier as
rp1-ethernet-mdio-mpe-enable-guard-static-frontier-closed. Same-shaped
local/static guard retries are closed for this candidate/control pair. The
checkpoint selects only the serialized Pi 5 NCR.MPE set/readback/restore proof,
which must acquire hardwareTestLock and preserve candidate/control identity,
archive review output, fresh serial cursor/output, TFTP delta, final
pre-restore identity, lab boot restore evidence, capture summary,
classification JSON, and evidence map.

This guard closeout does not accept hardware evidence, runtime NCR.MPE write
success, runtime RP1 MMIO access, MAN writes, PHY-ID reads, broad MDIO/PHY
ownership, PHY reset/GPIO32 ownership, Ethernet driver behavior, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof-20260611 accepts the serialized
Pi 5 NCR.MPE proof as rp1-ethernet-mdio-mpe-enable-already-set-restored.
Candidate-rerun4 and paired control-rerun5 passed capture-chain-v4 with
selected-tree identity, run-unique serial markers, stable TFTP deltas, final
pre-restore identity, and restore evidence. The candidate observed MACB_MID
context 0x70109 at 0x1c001000fc and NCR 0x10 at 0x1c00100000, then performed
only the accepted NCR.MPE set/readback/restore sequence: pre_raw 0x10,
write_value 0x10, post_raw 0x10, restore_raw 0x10, restore_eq_pre=true. The
paired control constructed no NCR/MPE target and performed no volatile
load/store, MAN write, or PHY-ID read.

This proof accepts only the NCR.MPE write/readback/restore boundary. It does
not accept MAN transactions, visible PHY-ID reads, broad MDIO/PHY ownership,
PHY reset/GPIO32 ownership, Ethernet driver behavior, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout-20260611 closes the
NCR.MPE proof frontier as
rp1-ethernet-mdio-mpe-enable-ownership-frontier-closed. Same-shaped NCR.MPE
set/readback/restore hardware retries are closed for this candidate/control
pair: repeating the accepted already-set/restored boundary would not prove MAN
transaction safety, PHY-ID visibility, PHY reset, broad MDIO/PHY ownership, or
packet I/O. Future MDIO/PHY progress requires supervisor planning with a
qualitatively different discriminator and explicit acceptance criteria.

The accepted boundary remains exactly the candidate/control capture-chain-v4
evidence for one NCR.MPE set/readback/restore sequence plus paired no-MDIO
control. MAN transactions, visible PHY-ID reads, runtime MDIO transaction
success, PHY reset/GPIO32 ownership, Ethernet driver behavior, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
transition remain unaccepted.

phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-20260611 accepts
the source/docs/evidence contract for a corrected-target after-MPE MDIO
PHY-ID discriminator. It explicitly preserves the earlier PHY-ID proof as a
closed wrong-target/no-write blocker: the prior NCR/NSR/MAN target set
0x1c00000000/0x1c00000008/0x1c00000034 is not the accepted observed-window
MACB/GEM target set and accepted no runtime MDIO transaction, MAN write, or
PHY-ID read.

The corrected future targets are observed-window NCR at 0x1c00100000, NSR at
0x1c00100008, MAN at 0x1c00100034, and MACB_MID context at 0x1c001000fc. The
selected future candidate is a no-NCR-write Clause 22 PHY-ID discriminator for
phy1 / ethernet-phy@1 address 1, registers MII_PHYSID1 0x02 and MII_PHYSID2
0x03, MAN frames 0x600a0000 and 0x600e0000, NSR.IDLE bit 2 polling before and
after each MAN write, and MAN.DATA bits 15:0 extraction. If corrected NCR.MPE
bit 4 is clear, the future proof must perform no NCR write and no MAN write,
then classify a precise after-MPE precondition blocker.

The paired control must use the same reporting path while constructing no MDIO
target and no MAN frame and performing no volatile load/store. This contract
does not run hardware and does not accept runtime MDIO transaction success,
visible PHY-ID reads, NCR write permission for the first corrected-target
proof, broad MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet driver
readiness, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core-20260611 accepts the
local/static corrected-target after-MPE MDIO PHY-ID guard report surface. The
candidate report preserves source contract id
phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-v1, report contract
id phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-report-contract-v1,
accepted MPE frontier
rp1-ethernet-mdio-mpe-enable-ownership-frontier-closed, MACB_MID context
0x1c001000fc, corrected NCR/NSR/MAN targets 0x1c00100000/0x1c00100008/
0x1c00100034, PHY address 1, PHYSID1/PHYSID2 MAN frames 0x600a0000 and
0x600e0000, no-NCR-write MPE precondition, bounded NSR.IDLE polling,
MAN.DATA extraction, rejected claims, retained risks, and hardware-proof
boundary fields.

The paired control uses the same report path while constructing no MDIO
targets, no MAN frames, no candidate-only result fields, and no volatile
access intent. The guard core does not run hardware and does not accept
runtime MDIO transaction success, visible PHY-ID reads, NCR write permission,
broad MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet driver
readiness, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-closeout-20260611 closes the
local/static after-MPE guard frontier as
rp1-ethernet-mdio-phy-id-after-mpe-guard-static-frontier-closed. Same-shaped
local/static guard retries are closed for this candidate/control pair. The
checkpoint selects only the serialized Pi 5 corrected-target after-MPE PHY-ID
proof, which must acquire hardwareTestLock and preserve candidate/control
selected-tree identity, archive review output, fresh serial cursor/output,
run-unique serial markers, TFTP delta, final pre-restore identity, restore
evidence, capture summary, classification JSON, and evidence map.

The selected proof may not write NCR. It may write MAN only if corrected
NCR.MPE bit 4 is already set; if that precondition fails, it must perform no
NCR or MAN write and classify a precise blocker. This guard closeout does not
accept hardware evidence, runtime RP1 MMIO access, visible PHY-ID reads, broad
MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet driver behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof-20260611 accepts the
serialized Pi 5 corrected-target after-MPE MDIO PHY-ID proof as
mdio-phy1-physid-after-mpe-visible. Candidate and paired control passed
capture-chain-v4 with selected-tree identity, run-unique serial markers,
stable TFTP deltas, final pre-restore identity, and restore evidence. The
candidate observed MACB_MID context 0x70109 at 0x1c001000fc and corrected
NCR 0x10 at 0x1c00100000, performed no NCR write, then wrote only the accepted
PHYSID1/PHYSID2 MAN frames after the MPE gate passed. MAN.DATA returned
physid1 0xffff and physid2 0xffff with both valid flags true. The paired
control constructed no MDIO target or MAN frame and performed no volatile
load/store.

This proof accepts only the selected corrected-target MAN transaction and
MAN.DATA return boundary. It does not accept PHY reset/GPIO32 ownership,
broad MDIO/PHY ownership, link state, Ethernet driver behavior, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition. The visible 0xffff/0xffff result remains evidence for the selected
transaction boundary only, not for a responsive PHY or usable Ethernet link.

phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout-20260611 closes the
corrected-target after-MPE PHY-ID frontier as
rp1-ethernet-mdio-phy-id-after-mpe-visible-frontier-closed. Same-shaped
after-MPE PHY-ID hardware retries are closed for this candidate/control pair:
repetition can reconfirm the selected MAN transaction and 0xffff/0xffff
MAN.DATA return boundary, but cannot prove PHY responsiveness, ETH_RST_N/GPIO32
ownership, link state, Ethernet driver behavior, or broad MDIO/PHY ownership.

phase12-rp1-ethernet-mdio-register-vector-source-contract-20260611 accepts the
source/docs/evidence contract for a qualitatively different corrected-target
MDIO discriminator after the accepted 0xffff/0xffff PHYSID-only return. The
selected future candidate is a no-NCR-write, no-reset Clause 22 register
vector for rp1_eth phy1 / ethernet-phy@1 address 1. It preserves MACB_MID
context 0x1c001000fc, corrected NCR/NSR/MAN targets
0x1c00100000/0x1c00100008/0x1c00100034, and the MPE precondition that
corrected NCR bit 4 must already be set before any MAN write.

The selected vector is BMCR 0x00, BMSR 0x01, PHYSID1 0x02, PHYSID2 0x03,
ANAR 0x04, and ANLPAR 0x05. Source-backed Clause 22 MAN frame construction
includes SOF=1, READ=2, PHYA=1, REGA, and CODE=2, yielding frames
0x60820000, 0x60860000, 0x608a0000, 0x608e0000, 0x60920000, and 0x60960000.
The prior accepted after-MPE proof remains bounded to its recorded selected
MAN.DATA return values and is not broadened into broad PHY1 responsiveness.

The future proof may classify a visible mixed vector, a global all-ones vector,
a PHYSID-only all-ones mixed vector, a timeout, a precondition blocker, a
capture blocker, or the paired no-MDIO/no-Ethernet control. An all-ones vector
would be evidence for the selected register-vector return only; it does not
prove PHY absence, PHY reset ownership, link state, usable Ethernet, broad
MDIO/PHY ownership, Ethernet driver behavior, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition. The next bounded
step is only the local/static register-vector guard core.

phase12-rp1-ethernet-mdio-register-vector-guard-core-20260611 accepts the
local/static corrected-target MDIO register-vector guard report surface. The
candidate report preserves source contract id
phase12-rp1-ethernet-mdio-register-vector-source-contract-v1, report contract
id phase12-rp1-ethernet-mdio-register-vector-guard-report-contract-v1, the
selected six-register vector, exact PHYA=1 MAN frames, corrected
NCR/NSR/MAN targets 0x1c00100000/0x1c00100008/0x1c00100034, the no-NCR-write
MPE precondition, bounded NSR.IDLE polling, MAN.DATA extraction, rejected
claims, retained risks, and hardware-proof boundary fields.

The paired control uses the same report path while constructing no MDIO
targets, no MAN frames, no candidate-only register-vector/result fields, and
no volatile access intent. The guard core does not run hardware and does not
accept runtime MDIO transaction success, register-vector MAN.DATA values,
NCR write permission, PHY absence from all-ones vectors, broad MDIO/PHY
ownership, PHY reset/GPIO32 ownership, Ethernet driver readiness, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

phase12-rp1-ethernet-mdio-register-vector-guard-closeout-20260611 closes the
local/static register-vector guard frontier as
rp1-ethernet-mdio-register-vector-guard-static-frontier-closed. Same-shaped
local/static guard retries are closed for this candidate/control pair: the
accepted guard already fixes the corrected-target six-register report and
paired no-MDIO/no-Ethernet control shape, and repetition would not prove MAN
transaction safety, register-vector visibility, PHY reset ownership, broad
MDIO/PHY ownership, Ethernet driver behavior, or packet I/O.

The checkpoint selects only the serialized Pi 5 register-vector proof as the
next bounded task. That proof must hold hardwareTestLock, preserve
candidate/control identity, fresh serial cursor/output, TFTP delta, final
pre-restore identity, restore proof, capture summary, classification JSON, and
evidence map, and must perform standard inconclusive-run triage before code
changes if staging evidence is not decisive. It may perform no NCR write and
no GPIO32/PHY reset action; MAN writes are allowed only if corrected NCR.MPE
bit 4 is already set. This guard closeout does not accept hardware evidence,
runtime MDIO transaction success, register-vector MAN.DATA values, PHY
absence from all-ones vectors, broad MDIO/PHY ownership, Ethernet driver
behavior, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard-20260611
records a guarded Pi 5 register-vector retry as a precise capture/staging
blocker. The candidate serial marker was fresh, but capture-chain-v4 rejected
decisive hardware classification because the selected candidate tree expected
two 52352-byte TFTP fetches while the lab observed two 104136-byte baseline
fetches and final pre-restore identity was the restored baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The paired
no-MDIO/no-Ethernet control passed capture-chain-v4 and proves only the
reporting path for this proof shape.

phase12-rp1-ethernet-mdio-register-vector-proof-v2-closeout-20260611 closes
that guarded proof frontier as
rp1-ethernet-mdio-register-vector-guarded-v2-candidate-identity-mismatch-frontier-closed.
Same-shaped guarded register-vector hardware retries are closed for this
candidate/control pair unless a future task supplies a qualitatively different
staging or power-cycle identity discriminator with explicit acceptance
criteria. This closeout accepts no register-vector MAN.DATA values, PHY
absence, broad MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet driver
behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or phase transition.

phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-core-20260611
and
phase12-rp1-ethernet-mdio-register-vector-staging-identity-gate-closeout-20260611
accept the local/static staging identity gate used to reject the guarded v2
candidate mismatch and accept the guarded v2 control fixture as selected-tree
identity ready. The gate does not weaken future hardware acceptance: any
register-vector retry still requires selected-tree identity, expected TFTP
bytes, final pre-restore identity, restore proof, and task-owned aggregate
evidence consistency.

phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof-20260611
accepts selected-tree identity durability for a no-MDIO/no-Ethernet staging
sentinel candidate-shaped archive and paired control. The candidate selected
tree a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0 and
control selected tree 9d9b3cdb7b1f230d9cd2bf0c04b7c32dd98b53dd8ec7de77e99860c5b231908d
both survived publication, power, matching TFTP fetches, final pre-restore
identity, and restore proof with true serial freshness.

phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout-20260611
closes that staging sentinel frontier as
selected-tree-identity-durability-accepted and selects only the queued
guarded register-vector v3 proof as the next bounded hardware task. The
sentinel proof accepts no register-vector MAN.DATA values, PHY absence, broad
MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet behavior, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition.

phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery-20260614
accepts the guarded register-vector Pi 5 proof after the served-root/root
recovery closeout. Candidate and paired no-MDIO/no-Ethernet control both
passed capture-chain-v4 and boot-staging identity with same-power-cycle TFTP
served byte agreement and final pre-restore identity on the selected tree.

The candidate accepted result is mdio-phy1-register-vector-visible for the
selected Clause 22 PHY1 vector only: BMCR 0x1000, BMSR 0x7949, PHYSID1
0x600d, PHYSID2 0x84a2, ANAR 0x01e1, and ANLPAR 0x0000. Corrected NCR.MPE was
already set, no NCR write was performed, and the paired control constructed no
MDIO target or MAN frame. This does not accept PHY absence, PHY reset/GPIO32
ownership, broad MDIO/PHY ownership, link state, Ethernet driver behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.

phase12-rp1-ethernet-mdio-register-vector-v4-closeout-20260614 closes the
accepted v4 register-vector proof frontier as
rp1-ethernet-mdio-register-vector-phy1-visible-frontier-closed. The accepted
boundary remains only the selected corrected-target PHY1 Clause 22
six-register MAN.DATA vector under capture-chain-v4, boot-staging identity,
same-power-cycle TFTP byte agreement, final pre-restore identity, serial
freshness, and restore evidence. The selected follow-up is a bounded PHY1
status diagnostic, not broader MDIO/PHY ownership, Ethernet behavior, packet
I/O, networking, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof-20260614 accepts a
visible PHY1 status diagnostic over the accepted corrected-target MDIO read
boundary. Candidate/control capture-chain-v4 and boot-staging identity both
passed with selected-tree identity, matching same-power-cycle TFTP fetch bytes,
final pre-restore identity, serial freshness, and restore evidence. The
candidate decoded BMCR 0x1000, BMSR 0x7949, PHYSID1 0x600d, PHYSID2 0x84a2,
ANAR 0x01e1, and ANLPAR 0x0000 as BMCR reset=false, loopback=false,
speed=10M, autoneg-enable=true; BMSR link-status=false,
autoneg-complete=false, autoneg-ability=true; PHY ID OUI 0x180361, model 0x0a,
revision 0x02; ANAR advertising 10/100 half/full; and empty ANLPAR. The paired
control constructed no MDIO target and no MAN frame. This does not accept link
usability beyond decoded register state, PHY configuration writes, PHY
reset/GPIO32 action, broad MDIO/PHY ownership, Ethernet driver behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.

phase12-rp1-ethernet-phy1-status-diagnostic-closeout-20260614 closes the
accepted PHY1 status diagnostic frontier as
rp1-ethernet-phy1-status-diagnostic-frontier-closed. The accepted boundary is
only the visible corrected-target PHY1 Clause 22 raw vector and decoded
register-state fields under capture-chain-v4, boot-staging identity,
same-power-cycle TFTP byte agreement, final pre-restore identity, serial
freshness, and restore evidence. The selected follow-up is only
phase12-rp1-ethernet-phy1-link-readiness-source-contract-20260614, a source
contract for one exact next discriminator; it does not authorize PHY
configuration, reset/GPIO32 action, broad MDIO/PHY ownership, packet I/O,
networking, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-phy1-link-readiness-source-contract-20260614 accepts the
source contract
phase12-rp1-ethernet-phy1-bmsr-latch-low-double-sample-link-readiness-contract-v1.
The selected future discriminator is a read-only corrected-target PHY1 BMCR
read followed by two BMSR reads, using the second BMSR sample for
BMSR_LSTATUS/BMSR_ANEGCOMPLETE classification because Linux v6.12
genphy_update_link treats BMSR link status as latched low. The future
candidate may construct only the selected BMCR/BMSR MAN read frames, with a
paired no-MDIO/no-Ethernet control. This source contract accepts no runtime
link-readiness proof, PHY configuration write, autonegotiation restart, link
forcing, MACB NSR_LINK discriminator, GPIO32/PHY reset ownership, broad
MDIO/PHY ownership, Ethernet driver behavior, interrupts, DMA/descriptors,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition.

phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof-20260614
accepts the read-only corrected-target PHY1 BMCR plus double-sampled BMSR
link-readiness proof as mdio-phy1-bmsr-double-sample-link-not-ready.
Candidate/control capture-chain run-unique and boot-staging identity checks
both passed with selected-tree identity, matching same-power-cycle TFTP fetch
bytes, final pre-restore identity, serial freshness, and restore evidence. The
candidate read BMCR 0x1000, BMSR first 0x7949, and BMSR second 0x7949; BMCR
reset, loopback, and autoneg-restart were false, while second-sample
BMSR_LSTATUS and BMSR_ANEGCOMPLETE were false. The paired control constructed
no MDIO target and no MAN frame. This accepts only the selected register-state
link-not-ready discriminator; it does not accept PHY configuration writes,
PHY reset/GPIO32 action, autonegotiation restart, link forcing, broad MDIO/PHY
ownership, Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout-20260614
closes the accepted BMSR double-sample discriminator frontier as
rp1-ethernet-phy1-bmsr-double-sample-link-readiness-frontier-closed. The closed
frontier accepts only the read-only corrected-target PHY1 BMCR plus
double-sampled BMSR register-state result under capture-chain-v4,
boot-staging identity, same-power-cycle TFTP byte agreement, final
pre-restore identity, serial freshness, evidence-consistency-ready, and
restore evidence. No explicit queued mechanically objective follow-up exists
after this closeout, so supervisor planning is required for the next bounded
Phase 12.1 task; the closeout does not authorize PHY configuration,
PHY reset/GPIO32 action, broad MDIO/PHY ownership, Ethernet behavior, packet
I/O, networking, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof-20260614 accepts the
passive MAC-side MACB_NSR_LINK read-only proof as
macb-nsr-link-readonly-link-clear. Candidate/control capture-chain-v4 and
boot-staging identity checks both passed with selected-tree identity, matching
same-power-cycle TFTP fetch bytes, final pre-restore identity, serial
freshness, and restore evidence. The candidate read MACB_NSR at
0x1c00100008 as 0x6 and decoded NSR_LINK bit 0 as false; the paired control
constructed no MACB_NSR target and performed no Ethernet volatile load/store.
This accepts only the selected MAC-side comparator value at the selected
instant; it does not accept link recovery, Ethernet readiness, MACB writes,
MDIO/PHY access, PHY configuration writes, BMCR writes, autonegotiation
restart, link forcing, PHY reset/GPIO32 action, DMA/descriptors, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-macb-nsr-link-readonly-closeout-20260614 closes the
accepted MACB_NSR_LINK read-only frontier as
rp1-ethernet-macb-nsr-link-readonly-frontier-closed. The closeout preserves
the accepted MAC-side comparator result, the paired no-MMIO/no-Ethernet
control, the PHY1 link-not-ready frontier, and the GPIO32 blocker context
without accepting link recovery, Ethernet readiness, MACB writes, PHY
configuration/reset, packet I/O, networking, sockets, SSH, Phase 12.2, or a
phase transition. No explicit queued mechanically objective follow-up exists
after this closeout, so supervisor planning is required for the next bounded
Phase 12.1 task.

phase12-rp1-ethernet-phy1-autoneg-restart-source-contract-20260614 accepts
the source-backed recovery contract for one future guarded corrected-target
PHY1 BMCR autonegotiation restart. The contract preserves the accepted PHY1
BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, and MACB_NSR
raw 0x6 / NSR_LINK=false frontier as input evidence only. Linux mii.h defines
BMCR_ANENABLE as 0x1000 and BMCR_ANRESTART as 0x0200; Linux
genphy_restart_aneg() sets BMCR_ANENABLE | BMCR_ANRESTART, and
genphy_update_link() treats BMCR_ANRESTART as autoneg being started, so link
must still be proven by later bounded status reads. The selected follow-up is
phase12-rp1-ethernet-phy1-autoneg-restart-guard-core-20260614. This source
contract does not accept runtime BMCR write evidence, GPIO32/PHY reset
ownership, MACB writes, link forcing, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

phase12-rp1-ethernet-phy1-autoneg-restart-guard-core-20260614 accepts the
local/static guard and report surface for that bounded PHY1 autoneg-restart
candidate/control pair. The candidate report permits only one corrected-target
PHY1 BMCR write value, pre_bmcr | BMCR_ANENABLE | BMCR_ANRESTART, after NCR.MPE
and BMCR_ISOLATE preconditions; it records pre/post BMCR, double BMSR,
ANAR/ANLPAR, passive MACB_NSR_LINK, touched fields, and rejected claims. The
paired control constructs no MDIO/MAN/MACB target and performs no volatile
load/store. This guard core does not accept hardware evidence, runtime BMCR
write success, link recovery, PHY reset ownership, packet I/O, networking,
SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof-20260614 accepts the
serialized Pi 5 proof only as a precise capture-staging blocker. Candidate
publication was visible through /boot/files as selected tree
6bf7d36a3f07426f450fd8a4def73b9cc8bbbc5b730ba50503fd0ee8f41609e1 with
expected da591740/kernel_2712.img size 52360 bytes, but same-power-cycle TFTP
served four 104136-byte baseline fetches and final pre-restore identity was
baseline tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
Capture-chain-v4 and boot-staging identity both rejected the candidate, and
known-good baseline triage produced no fresh TFTP events after power. The lab
was restored to the baseline tree. No runtime PHY1 BMCR write evidence,
autonegotiation restart success/failure, link readiness, Ethernet readiness,
packet I/O, networking, SSH, Phase 12.2, or phase transition is accepted.

phase12-rp1-ethernet-phy1-autoneg-restart-closeout-20260614 closes the guarded
autoneg-restart proof frontier as
rp1-ethernet-phy1-autoneg-restart-capture-staging-blocker-frontier-closed. The
closed frontier preserves the source/guard work as available but not
hardware-accepted runtime progress until the capture path can prove selected
tree identity for a fresh run. No explicit queued mechanically objective
follow-up exists after this closeout, so supervisor planning is required before
any capture-layer recovery, paired-control hardware run, PHY configuration,
GPIO32/PHY reset action, packet I/O, networking, SSH, Phase 12.2, or phase
transition.

phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof-20260614
accepts the guarded autoneg-restart v2 proof as
phy1-autoneg-restart-write-observed-link-not-ready. Candidate/control
capture-chain-v4 and boot-staging identity checks passed with selected-tree
identity, matching same-power-cycle TFTP fetch bytes, final pre-restore
identity, serial freshness, and restore evidence. The candidate reached the
guarded corrected-target PHY1 BMCR discriminator: NCR.MPE was set, BMCR
isolate was clear, exactly one BMCR write intent value 0x1200 was emitted,
post-BMCR readback was 0x1000, post-BMSR samples remained 0x7949/0x7949,
ANAR remained 0x01e1, ANLPAR remained 0x0000, and passive MACB_NSR remained
0x6 / NSR_LINK=false. The paired control constructed no MDIO/MAN/MACB target
and performed no volatile Ethernet access. This accepts only the bounded BMCR
restart attempt and link-not-ready result; it does not accept link readiness,
Ethernet readiness, PHY reset/GPIO32 ownership, MACB writes, NCR writes, link
forcing, packet I/O, DMA/descriptors, interrupts, networking, sockets, SSH,
Phase 12.2, or a phase transition.

phase12-rp1-ethernet-capture-staging-recurrence-checkpoint-20260614 accepts a
non-hardware reconciliation of the autoneg capture-staging recurrence. The
first failing invariant remains same-power-cycle TFTP-served bytes and final
pre-restore identity not matching the selected autoneg candidate tree; the
secondary unresolved invariant is that known-good baseline triage produced no
fresh TFTP events. Accepted minimal-sentinel and v4 register-vector comparator
proofs remain valid historical evidence that the identity path can work, but
they do not unblock an autoneg retry after the later recurrence. No static
capture/publication helper defect was found. The selected follow-up is the
fresh minimal sentinel task
phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof-20260614, with
no MDIO, MACB, PHY, autonegotiation, GPIO32, packet I/O, networking, SSH,
Phase 12.2, or phase transition claims.

phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof-20260614
accepts current capture-staging freshness for a minimal no-MDIO/no-Ethernet
candidate/control pair. The control selected tree
9e3442962d40beff3b31668065df4f5d8ac37ee770fa3c4bba725d049bc78db3 with two
matching 47832-byte TFTP fetches, and the candidate rerun selected tree
520785f412ba93da8c25577e5f5e4635ffba02b2969fbf3e02a346e97e061799 with two
matching 47848-byte TFTP fetches. Both passed capture-chain-v4 and the staging
identity gate with fresh serial markers, final pre-restore selected-tree
identity, and restore to baseline. This does not accept autonegotiation
restart, BMCR writes, MDIO register vectors, MACB_NSR reads, GPIO32/PHY reset
action, link readiness, packet I/O, networking, SSH, Phase 12.2, or a phase
transition; the next bounded task is the capture-staging recovery closeout.

phase12-rp1-ethernet-capture-staging-recovery-closeout-20260614 closes the
capture-staging recovery frontier as
rp1-ethernet-capture-staging-minimal-sentinel-recovered-frontier-closed. The
closed frontier accepts only selected-tree/TFTP/final-identity freshness for
one minimal no-MDIO/no-Ethernet candidate/control sentinel pair; it does not
accept runtime autonegotiation restart, BMCR writes, MDIO register vectors,
MACB_NSR reads, GPIO32/PHY reset ownership, link readiness, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition. No explicit
queued mechanically objective follow-up exists after this closeout, so
supervisor planning is required before any fresh autoneg retry or broader
Ethernet work.

phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof-20260614
accepts the guarded autoneg-restart v2 discriminator as
phy1-autoneg-restart-write-observed-link-not-ready. The candidate selected tree
c7e847e3ff587fc240ed4b493f42f393f7380c45f5c6b5573fe7c7e45db8f851 and paired
control selected tree
031da5edc1bb199f260358087e443def1e53fbb4fa1f33d212384d898aab5b56 passed
capture-chain-v4, boot-staging identity, same-power-cycle TFTP byte agreement,
fresh serial marker, final pre-restore selected-tree identity, and restore
proof. The candidate reached the corrected-target PHY1 discriminator with
NCR.MPE true, BMCR isolate clear, one BMCR write intent value 0x1200, post-BMCR
0x1000, post-BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, and passive
MACB_NSR_LINK=false. The paired control constructed no MDIO/MAN/MACB target and
performed no volatile Ethernet access. This does not accept link readiness,
Ethernet readiness, GPIO32/PHY reset ownership, packet I/O, networking, SSH,
Phase 12.2, or a phase transition; the next bounded task is the v2 closeout.

phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout-20260614 closes the v2
frontier as
rp1-ethernet-phy1-autoneg-restart-v2-link-not-ready-frontier-closed. The
closed frontier accepts capture-fresh candidate/control identity, one guarded
corrected-target PHY1 BMCR autoneg-restart write intent, and link-not-ready
post-read PHY/MAC status only. It preserves rejected link readiness, operator
or cabling diagnosis, GPIO32/PHY reset ownership, broad PHY configuration,
packet I/O, networking, SSH, Phase 12.2, and phase-transition claims. The
selected next task is
phase12-rp1-ethernet-post-autoneg-status-source-checkpoint-20260614, limited to
choosing one future read-only status proof or recording why no such follow-up is
safe.

phase12-rp1-ethernet-post-autoneg-status-source-checkpoint-20260614 accepts
that no mechanically safe same-frontier read-only post-autoneg status proof is
selected. The accepted v2 proof already retained the bounded post-write BMCR,
double-BMSR, ANAR, ANLPAR, and passive MACB_NSR status fields named by the
current contracts, and those values stayed link-not-ready. Another immediate
read-only status proof would be same-shaped unless supervisor planning defines
a distinct source-backed discriminator, timing model, or external precondition.
The checkpoint keeps GPIO32/PHY reset ownership, packet I/O, networking, SSH,
Phase 12.2, and phase transition rejected and requires supervisor planning for
the next bounded Phase 12.1 task.

phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract-20260614
accepts the post-physical-precondition read-only status contract after operator
confirmation that the Pi 5 physical Ethernet link path is present. The selected
future proof may sample only corrected-target PHY1 BMCR, double-sampled BMSR,
ANAR, ANLPAR, and passive MACB_NSR_LINK in one bounded immediate window after
the boot marker and recorded physical-link precondition. The paired control
must use the same report surface while constructing no MDIO/MAN/MACB target and
performing no volatile Ethernet access. This source contract does not accept
runtime link readiness, PHY reset ownership, PHY configuration, packet I/O,
networking, SSH, Phase 12.2, or a phase transition; the selected queued
follow-up is
phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof-20260614.

phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof-20260614
is blocked as post-physical-link-status-source-precondition-blocker. The Pi 5
candidate/control capture identity, same-power-cycle TFTP byte agreement,
fresh serial nonces, final pre-restore identity, and restore evidence passed;
the runtime candidate emitted BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1,
ANLPAR 0x0000, MACB_NSR 0x00000006, and a runtime phy-not-ready result.
However, the source-contract gate failed because the candidate issued MACB MAN
read-command transactions to perform the selected PHY1 reads while the accepted
contract says no MAN writes and the report claims macb_write_count=0. The
runtime phy-not-ready result is recorded but not accepted as a planning
frontier until a revised source-contract boundary is accepted. Packet I/O,
networking, SSH, Phase 12.2, and phase transition remain rejected.

phase12-rp1-ethernet-post-physical-link-status-man-read-accounting-core-20260615
reconciles that boundary by accepting
phase12-rp1-ethernet-post-physical-link-status-contract-v2. The v2 candidate
surface distinguishes the five allowed MAN read-command stores from forbidden
PHY configuration writes, BMCR writes, MAC configuration writes, GPIO32/PHY
reset action, DMA, and packet I/O. The paired control still constructs no
MDIO/MAN/MACB targets and performs no volatile Ethernet access. The selected
queued follow-up is
phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof-20260615.

phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof-20260615 accepts a
serialized Pi 5 v2 status proof with decisive candidate/control capture-chain
and boot-staging identity. The candidate reports the bounded status sample as
post-physical-link-status-phy-not-ready: BMCR 0x1000, BMSR 0x7949/0x7949, ANAR
0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006, BMSR link false, autoneg complete
false, ANLPAR nonzero false, and MACB_NSR_LINK false. The proof accepts only
that bounded status frontier and the five accounted MAN read-command stores;
it does not accept GPIO32/PHY reset action, PHY configuration writes, BMCR
writes, MACB configuration writes, DMA/descriptors, packet I/O, networking,
SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-post-physical-link-status-v2-closeout-20260615 accepts
the v2 proof as the current post-physical phy-not-ready frontier. Follow-up
planning must remain source-grounded around PHY power/reset/strap/autoneg
status recovery or an explicit pause, and must not ask Matthew to reconfirm the
already accepted physical-link precondition. Packet I/O, networking, SSH,
Phase 12.2, and phase transition remain rejected.

phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint-20260615
accepts a static/source checkpoint over that phy-not-ready frontier and the
prior GPIO32 blockers. GPIO32 write/restore v2 still stopped before
GPIO/RIO/pad writes with event bits 0x0ab00000, and the accepted GPIO32 CTRL
SET IRQRESET clear attempt left event bits 0x08800000 while preserving
CTRL/RIO/pad invariants. Retained RP1 source names the event bits and clear
mechanism but does not justify treating the persistent bits as harmless for
ETH_RST_N ownership, so no GPIO32 reset-recovery proof is selected.

phase12-rp1-ethernet-phy-power-strap-source-checkpoint-20260615 accepts the
next static/source checkpoint as
post-physical-phy-power-strap-source-checkpoint-no-distinct-discriminator. The
retained source backs PHY1 at address 1, GPIO32 active-low ETH_RST_N reset with
5 ms duration, `rgmii-id` mode, Broadcom powerdown/EEE quirks, the MACB MDIO
reset hook, and phylink/MACB dependency facts. Those facts reconcile the
accepted phy-not-ready and GPIO32 blocker frontier but do not identify a
distinct safe hardware discriminator; same-shaped status samples, GPIO32
event-clear retry, GPIO32 write/restore retry, and BMCR autoneg-restart retry
remain rejected. Supervisor planning is required for a new source-gathering
task or explicit pause before any further Phase 12.1 hardware action.

phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory-20260615 accepts
the observed corrected-target PHY ID as an exact Broadcom BCM54213PE source
match. PHYSID1 0x600d and PHYSID2 0x84a2 combine to 0x600d84a2; Raspberry Pi
Linux rpi-6.12.y names that value PHY_ID_BCM54213PE, and the Broadcom driver
table matches it as Broadcom BCM54213PE with phy_id_mask 0xffffffff. The
driver inventory records source-backed BCM54213PE behavior for RGMII internal
delay, APD/powerdown, EEE broken-mode handling, interrupt
acknowledgement/configuration, and suspend/resume. This is qualitatively
different from another passive status sample, GPIO32 event clear/write-restore
attempt, or BMCR autoneg-restart retry, but it is still source/static evidence
only. No hardware proof, GPIO32 action, BMCR write, PHY configuration, MACB
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition is
authorized. Supervisor planning is required to queue the selected
source/static follow-up
phase12-rp1-ethernet-bcm54213pe-config-init-source-contract-20260615 or an
explicit alternate/pause.

phase12-rp1-ethernet-bcm54213pe-config-init-source-contract-20260615 accepts
the BCM54213PE source/static contract as
bcm54213pe-config-init-source-contract-readonly-preflight-contract-selected.
The Linux helper path is bcm54xx_config_init -> bcm54213pe_config_init ->
bcm54210e_config_init -> bcm54xx_config_clock_delay, with related generic
read_status/aneg, APD/powerdown, EEE, interrupt, and suspend/resume surfaces.
The contract separates read-only candidates such as MII_STAT1000,
MII_CTRL1000, MII_BCM54XX_ISR/ECR/IMR context, Broadcom shadow/AUX context,
and EEE MMD context from write targets such as IMR/ECR mask changes, RGMII
delay shadow writes, APD/EEE writes, BMCR lifecycle writes, LED/WOL/PTP writes,
and MACB/phylink configuration. No direct hardware proof is selected because
the smallest potentially distinct reads still need a source/static preflight
contract for side effects and selector mechanics. Link readiness, GPIO32/PHY
reset ownership, BMCR retry, broad PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, and phase transition remain rejected.

phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract-20260616
accepts
bcm54213pe-readonly-preflight-source-contract-report-core-selected. The source
contract classifies the BCM54213PE candidate read surfaces and selects only a
local/static report-core follow-up for MII_CTRL1000 0x09 and MII_STAT1000
0x0a on PHY1. MII_BCM54XX_ISR is rejected because retained Linux source treats
ISR reads as interrupt acknowledgement; MII_BCM54XX_ECR and MII_BCM54XX_IMR
are deferred as interrupt context; Broadcom AUX/shadow delay and Clause 45/MMD
EEE reads are blocked from the pure read-only set because retained Linux
source requires selector writes before reading. The accepted phy-not-ready
status and GPIO32 persistent-event-state blocker remain unchanged. No hardware
proof, GPIO32 action, BMCR write, Broadcom shadow/MMD/aux access, PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition is
authorized.

phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core-20260616 and
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-closeout-20260616 close the
read-only preflight frontier as
bcm54213pe-readonly-preflight-frontier-closed-hardware-proof-planning-required.
The report core encodes only local/static candidate metadata for PHY1
MII_CTRL1000 0x09 and MII_STAT1000 0x0a plus the paired
no-MDIO/no-Ethernet control shape, and its validators reject hardware proof,
volatile access, GPIO32 action, BMCR/PHY writes, Broadcom shadow/MMD/aux
access, interrupt surfaces, PHY/MAC configuration, link-readiness, packet I/O,
networking, SSH, Phase 12.2, and phase-transition claims. Any later use of the
closed target set requires supervisor planning for an explicit
candidate/control hardware-proof contract with candidate identity, fresh serial
cursor, same-power-cycle TFTP delta, known-good control, candidate rerun on
inconclusive evidence, hardwareTestLock, restore proof, and post-run forbidden
claim review. The closeout itself authorizes no hardware run, write/restore
surface, networking, SSH, Phase 12.2, or phase transition.

phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-core-20260616,
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof-20260616, and
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-closeout-20260616
close the hardware-proof slice as
bcm54213pe-readonly-preflight-hw-proof-frontier-closed-candidate-fetch-blocker.
The proof core preserved only the selected PHY1 MII_CTRL1000 0x09 and
MII_STAT1000 0x0a candidate target set plus the no-MDIO/no-Ethernet control
shape. The serialized Pi 5 proof accepted the control path with selected-tree
identity, two matching 50536-byte TFTP fetches, fresh serial marker output,
boot-staging-identity-ready, and restore evidence. The candidate rerun staged a
selected 51512-byte candidate tree but produced no fresh TFTP events or serial
output after power-cycle, so candidate raw/decoded register values remain
deferred behind that blocker. Same-shaped hardware retries are closed for this
candidate/control pair until supervisor planning selects a distinct
discriminator or pause. Link readiness, GPIO32/PHY reset ownership, BMCR
writes, Broadcom shadow/MMD/aux access, interrupt ownership, broad PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, and phase transition
remain rejected.

phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-core-20260616 and
phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof-20260616
accept boot-transport-selected-tree-fresh-tftp-no-kernel-sentinel-serial. The
sentinel pair constructs no Ethernet, MDIO, MAN, MACB, GPIO32, PHY, packet,
networking, SSH, or Phase 12.2 target facts. Serialized Pi 5 evidence shows
both selected candidate/control boot trees remained selected through final
pre-restore status and each produced two fresh 86,744-byte TFTP serves of
da591740/kernel_2712.img after power-cycle. Fresh serial captured Raspberry Pi
firmware NETWORK output, but neither sentinel image emitted its run nonce
marker in the bounded window. This narrows the previous BCM54213PE candidate
no-fresh-TFTP/no-serial blocker: selected-tree publication and TFTP fetch work
for a no-MDIO/no-Ethernet sentinel, while fetched-kernel execution or sentinel
serial emission remains unaccepted. BCM54213PE register values, link readiness,
GPIO32/PHY reset ownership, BMCR writes, Broadcom shadow/MMD/aux access,
interrupt ownership, broad PHY/MAC configuration, packet I/O, networking, SSH,
Phase 12.2, and phase transition remain rejected.

phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-closeout-20260616
closes the sentinel frontier as
bcm54213pe-boot-transport-sentinel-frontier-closed-kernel-serial-boundary. The
closed frontier accepts selected-tree publication, final pre-restore identity,
fresh TFTP serving, firmware NETWORK serial presence, and restore proof for the
no-Ethernet/no-MDIO sentinel pair. It does not accept sentinel nonce visibility
or kernel main output, so the remaining boundary is fetched-kernel execution or
sentinel serial-emission visibility rather than generic selected-tree/TFTP
transport. Supervisor planning is required for a distinct Phase 12.1 follow-up
or explicit pause. BCM54213PE register values, link readiness, GPIO32/PHY reset
ownership, BMCR writes, Broadcom shadow/MMD/aux access, interrupt ownership,
broad PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, and phase
transition remain rejected.

phase12-rp1-ethernet-kernel-entry-serial-beacon-core-20260616,
phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof-20260616, and
phase12-rp1-ethernet-kernel-entry-serial-beacon-closeout-20260616 close the
earliest-entry beacon frontier as
kernel-entry-serial-beacon-frontier-closed-beacon-observed. The selected
no-Ethernet/no-MDIO beacon image emits before BootInfo parsing and before any
Ethernet or MDIO behavior. Serialized Pi 5 evidence retained selected-tree
identity, two fresh 47,360-byte TFTP serves of da591740/kernel_2712.img,
89 run-unique beacon serial markers, final pre-restore identity, and restore
proof. This proves generic fetched-kernel earliest Rust-entry serial visibility
for the selected no-Ethernet/no-MDIO shape, but it does not accept BCM54213PE
register values, sentinel/report behavior after BootInfo parsing, link
readiness, GPIO32/PHY reset ownership, BMCR writes, Broadcom shadow/MMD/aux
access, interrupt ownership, broad PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, or a phase transition. Supervisor planning is
required for one distinct Phase 12.1 follow-up or an explicit pause.

phase12-rp1-ethernet-bootinfo-report-serial-visibility-core-20260616 accepts
the local/static dual-stage BootInfo/report-path serial visibility
discriminator as bootinfo-report-serial-visibility-core-local-static. The
candidate and paired earliest-only control both emit a run-unique
bootinfo-report-visibility-earliest-entry-marker before target services and
report_boot_identity consume BootInfo. Only the candidate emits
bootinfo-report-visibility-post-bootinfo-report-path-marker after
report_boot_identity reports BootInfo and service metadata. Candidate/control
static artifact review retained distinct archive and kernel hashes, proved the
selected markers/nonces, and rejected BCM54213PE register values, MDIO/MAN,
MACB, GPIO32/PHY target facts, volatile Ethernet access, BMCR writes, packet
I/O, networking, SSH, Phase 12.2, and phase transition claims. The selected
next boundary is the serialized Pi 5 proof that must classify no selected
TFTP, no earliest marker, earliest marker only, both markers observed, or a
precise capture/restore blocker without accepting Ethernet behavior.

phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof-20260616
accepts staging-capture-inconclusive with the precise first failing invariant
serial-drain-not-empty-before-power. The task repaired the proof instrumentation
after the initial hardware run showed the one-shot earliest marker could scroll
out under saturated direct-read capture; the repeated control/candidate report
now retains the earliest marker token and the review scripts require it. Final
Pi 5 control evidence retained selected tree
b886e168d26f69a943a98d77de87a40a7079938fa041aee8494e32cb98ea9178, two matching
55,120-byte da591740/kernel_2712.img TFTP serves, 71 earliest marker
occurrences, zero post-BootInfo marker occurrences, final pre-restore identity,
and restore proof. Final candidate evidence retained selected tree
38173e8bd614d6034e09e4944e0d5e92ad80dcebafb78b260897be7f74cc8c19, two matching
71,168-byte da591740/kernel_2712.img TFTP serves, 69 earliest marker
occurrences, 68 post-BootInfo marker occurrences, final pre-restore identity,
and restore proof. The capture-chain identity guard still rejects decisive
classification because the 128-attempt pre-power serial drain exhausted without
an empty /serial/read response. No same-shaped retry is authorized without a
new discriminator for the serial-drain/backlog invariant. BCM54213PE register
values, link readiness, Ethernet readiness, GPIO32/PHY reset ownership, BMCR
writes, Broadcom shadow/MMD/aux access, interrupt ownership, broad PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, and phase transition
remain rejected.

phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-proof-core-20260616
accepts bcm54213pe-readonly-preflight-v2-proof-core-local-static. The v2 core
keeps the accepted PHY1 MII_CTRL1000 0x09 and MII_STAT1000 0x0a target set, but
adds the accepted cursor-nonce-post-power-freshness-v1 contract and splits the
candidate serial surface into a pre-MDIO entry marker and a separate post-read
values marker. The paired control emits the same capture-nonce freshness shape
while constructing no MDIO, MAN, MACB, GPIO32/PHY, or RP1 Ethernet target
facts. This local/static core accepts no register values, link readiness,
GPIO32/PHY reset ownership, BMCR/autoneg, Broadcom shadow/MMD/aux access,
interrupt ownership, packet I/O, networking, SSH, Phase 12.2, or phase
transition.

phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof-20260616
accepts bcm54213pe-readonly-preflight-v2-post-read-values-visible. The paired
control retained selected tree
035d4affb2ed54ffe8d02a7f6cd2879ba404775ec49379062dd6f694f9e40abb, two
matching 50,856-byte da591740/kernel_2712.img TFTP serves, 17 fresh serial
marker/nonce occurrences, and restore proof while constructing no Ethernet or
MDIO target facts. The candidate retained selected tree
012e2aeae1fb00699b3ae9ead98433f68b8e093d96f4105332dfe6146b3b6ab3, two
matching 52,056-byte TFTP serves, 17 fresh serial marker/nonce occurrences,
final identity, and restore proof, then reached the post-read marker with PHY1
MII_CTRL1000 0x09 raw 0x0200 valid and MII_STAT1000 0x0a raw 0x0000 valid.
This accepts only the bounded read-only visibility of those two registers under
the v2 freshness contract. It does not accept link readiness, GPIO32/PHY reset
ownership, BMCR/autoneg, Broadcom shadow/MMD/aux access, interrupt ownership,
broad PHY/MAC configuration, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition. The selected next boundary is the queued v2 closeout.

phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-closeout-20260616
closes the frontier as
bcm54213pe-readonly-preflight-v2-frontier-closed-read-values-accepted. The
accepted narrow fact is read-only visibility of PHY1 MII_CTRL1000 0x09 raw
0x0200 valid and PHY1 MII_STAT1000 0x0a raw 0x0000 valid, joined with selected
tree identity, same-power-cycle TFTP byte serves, cursor-nonce serial
freshness guard replay, final identity, restore proof, and the paired
no-MDIO/no-Ethernet control. No follow-up write, configuration, link recovery,
packet I/O, networking, SSH, Phase 12.2, or phase transition is authorized by
this closeout. Supervisor planning must select any later GPIO32/PHY reset,
BMCR/autoneg, Broadcom selector, interrupt, PHY/MAC configuration, packet I/O,
networking, SSH, explicit pause, or other Phase 12.1 boundary.

phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint-20260616
accepts bcm54213pe-bmcr-autoneg-restart-contract-selected. The checkpoint keeps
the accepted MII_CTRL1000 0x0200 and MII_STAT1000 0x0000 values as context
only, reconciles the retained BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1,
ANLPAR 0x0000, MACB_NSR 0x00000006 link-not-ready frontier, and preserves the
GPIO32 persistent-event-state blocker. The only selected follow-up is the
local/static phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616
task. Its allowed surface is one corrected-target PHY1 BMCR restart contract:
pre-read BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000 plus passive
MACB_NSR_LINK context, require corrected NCR.MPE set and BMCR_ISOLATE clear,
model exactly one BMCR write of pre_bmcr | BMCR_ANENABLE | BMCR_ANRESTART, and
post-read BMCR, double BMSR, ANAR, ANLPAR, MII_CTRL1000, MII_STAT1000, and
passive MACB_NSR_LINK. The paired control must construct no
MDIO/MAN/MACB/GPIO32/PHY/RP1 Ethernet target and no volatile Ethernet access.
This checkpoint does not authorize a hardware run, BMCR write, GPIO32/reset
recovery, Broadcom shadow/MMD/AUX access, interrupt ownership, broad PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616,
phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof-20260616, and
phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-closeout-20260616 close
the BCM54213PE BMCR/autoneg restart frontier as
bcm54213pe-bmcr-autoneg-restart-frontier-closed-post-status-link-not-ready. The
core pinned exactly one corrected-target PHY1 BMCR write frame 0x50821200 for
value 0x1200 and a paired no-MDIO/no-Ethernet control. Serialized Pi 5 proof
retained selected-tree identity, same-power-cycle TFTP byte serves,
cursor-nonce serial freshness, final pre-restore identity, and restore proof
for both control and candidate. The candidate executed the bounded restart path
and sampled post-BMCR 0x1000, post-BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR
0x0000, MII_CTRL1000 0x0200, MII_STAT1000 0x0000, and passive MACB_NSR
0x00000006 with BMSR link false, BMSR autoneg-complete false, and
MACB_NSR_LINK false. This accepts the bounded write/status sample only; it does
not accept link readiness, GPIO32/PHY reset ownership, Broadcom selector/config
writes, interrupt ownership, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition. Supervisor planning is required before any follow-up
task or explicit pause.

phase12-rp1-ethernet-bootinfo-report-serial-visibility-closeout-20260616 closes
that frontier as
bootinfo-report-serial-visibility-frontier-closed-serial-drain-blocked. The
closed evidence accepts the local/static dual-stage marker shape, final
candidate/control selected-tree identity, matching same-power-cycle TFTP byte
serves, separate earliest and post-BootInfo marker counts, final identity, and
restore proof, while preserving serial-drain-not-empty-before-power as the first
failing invariant. Same-shaped retries are closed until supervisor planning
selects a distinct serial drain/backlog freshness discriminator, a source/static
contract, or an explicit pause. BCM54213PE register values, link readiness,
Ethernet readiness, GPIO32/PHY reset ownership, BMCR writes, Broadcom
shadow/MMD/aux access, interrupt ownership, broad PHY/MAC configuration, packet
I/O, networking, SSH, Phase 12.2, and phase transition remain rejected.

phase12-rp1-ethernet-serial-freshness-contract-20260616,
phase12-rp1-ethernet-serial-freshness-guard-core-20260616,
phase12-rp1-ethernet-serial-freshness-pi5-proof-20260616, and
phase12-rp1-ethernet-serial-freshness-closeout-20260616 close the serial
freshness frontier as serial-freshness-frontier-closed-cursor-nonce-accepted.
The accepted boundary replaces the hard empty-drain gate for marker-only
transport proofs with cursor-nonce-post-power-freshness-v1: retained evidence
must show the run-unique marker/nonce absent before power, present after the
saved cursor boundary or saturated-cursor direct-read fallback, and joined with
selected-tree identity, same-power-cycle TFTP, final pre-restore identity, and
restore proof. The Pi 5 proof retained selected tree
f73c75438663373b3d6df4e0ce451a45f163c4a582d8ba84bd79d161cf9cc68f, two
matching 47,352-byte da591740/kernel_2712.img serves, zero pre-power nonce
occurrences, 45 post-power marker/nonce occurrences, final identity, and
restore proof. Empty pre-power drain remains strong positive evidence, but it
is no longer a hard gate for this marker-only class. Supervisor planning is
required for any BootInfo/report-path rerun, BCM54213PE register discriminator,
explicit pause, or other Phase 12.1 boundary. BCM54213PE register values, link
readiness, Ethernet readiness, GPIO32/PHY reset ownership, BMCR/autoneg,
Broadcom shadow/MMD/aux access, interrupt ownership, broad PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, and phase transition
remain rejected.

phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core-20260616,
phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof-20260616, and
phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-closeout-20260616 close
the BCM54213PE convergence frontier as
bcm54213pe-autoneg-convergence-frontier-closed-timeout-link-not-ready. The
accepted runtime fact is one corrected-target PHY1 BMCR restart write frame
0x50821200 for value 0x1200 followed by eight bounded convergence poll samples
under selected-tree identity, same-power-cycle TFTP byte serves, cursor-nonce
serial freshness, final identity, restore proof, and paired
no-MDIO/no-Ethernet control evidence. The terminal sample still reported BMCR
0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MII_CTRL1000 0x0200,
MII_STAT1000 0x0000, passive MACB_NSR 0x00000006, BMSR link false,
BMSR autoneg-complete false, MACB_NSR_LINK false, and link-ready-terminal
false. The only mechanically unblocked follow-up is
phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint-20260616;
the link-ready packet-readiness checkpoint remains blocked. This closeout does
not authorize GPIO32/PHY reset ownership, Broadcom selector/config writes,
interrupt ownership, broad PHY/MAC configuration, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint-20260616
selects the next timeout follow-up as
bcm54213pe-post-convergence-timeout-rgmii-delay-source-contract-selected. It
does not repeat the accepted BMCR restart/convergence timeout shape, and it
does not weaken the GPIO32 persistent-event-state blocker. The checkpoint
compares retained GPIO32 reset evidence, BCM54213PE config_init source,
physical/partner-state context, and interrupt/status-only options, then selects
only a supervisor-planned local/static BCM54213PE RGMII delay source contract
for the rgmii-id config_init path. That future source contract should pin the
MII_BCM54XX_AUX_CTL shadow MII_BCM54XX_AUXCTL_SHDWSEL_MISC path with WREN and
RGMII_SKEW_EN, plus BCM54810_SHD_CLK_CTL with GTXCLK_EN, before any hardware
or write proof. MII_CTRL1000 master-mode writes, APD/EEE/LED/WOL/suspend-resume
paths, GPIO32/PHY reset, interrupt acknowledgement/masking, MACB configuration,
packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
rejected or deferred pending explicit supervisor planning.

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract-20260616 accepts
bcm54213pe-rgmii-delay-source-contract-proof-core-selected. The selected
implementation boundary is the local/static
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core-20260616 task only. It
pins PHY1, rgmii-id, MII_BCM54XX_AUX_CTL 0x18 shadow 0x07 with selector
0x7007, RGMII_SKEW_EN 0x0100, SHD 0x1c shadow 0x03, and GTXCLK_EN 0x0200. A
future candidate must read-modify-write/read back RX then TX delay state,
stop before BMCR restart on readback mismatch, and otherwise reuse exactly one
accepted BMCR restart write frame 0x50821200 followed by the bounded convergence
poll vector. Hardware execution, link-ready acceptance, GPIO32/PHY reset,
interrupt ownership, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
transition remain rejected here.

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core-20260616 accepts
bcm54213pe-rgmii-delay-proof-core-local-static. The candidate/control core adds
the selected boot-scenario routing, source-contract validators, compile-only
candidate image e5592f1671b42ffd14057668ae22ca48d70e25a52ce0200b377b93e71d294a0c
(53,720 bytes), and no-MDIO control image
240348cbd3f023a7915aab3486c0dc36a8b857098d2a6c093f21847ae62377e3
(49,984 bytes). Static string inspection confirms the candidate carries the
PHY1 RGMII RX/TX delay write/readback contract and the control withholds target
construction. The only mechanically unblocked follow-up is the serialized Pi 5
RGMII delay proof; packet I/O, networking, sockets, SSH, Phase 12.2, and phase
transition remain rejected.

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof-20260616 accepts
rgmii-delay-capture-blocker. Control selected-tree, same-power-cycle TFTP,
serial freshness, and restore evidence proved the no-MDIO/no-Ethernet shape.
Candidate evidence was equally decisive for identity and freshness: the RX
delay write/readback matched on PHY1 AUX_CTL, but the TX delay selected-register
read did not complete, so the candidate stopped before TX write, BMCR restart,
or convergence polling. This preserves the blocker at the TX delay read layer;
packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
rejected.

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-closeout-20260616 accepts
bcm54213pe-rgmii-delay-frontier-closed-tx-delay-read-capture-blocker. The
closeout accepts only the source/core/proof reconciliation and the precise
runtime blocker: RX delay write/readback reached hardware and reported
RGMII_SKEW_EN true, then the TX delay selected-register read failed before TX
write, BMCR restart, or convergence polling. Link readiness, link-not-ready
after the full delay path, packet I/O, networking, sockets, SSH, Phase 12.2,
and phase transition remain rejected. No explicit queued follow-up is
mechanically unblocked; supervisor planning must select a precise TX delay
selected-register read blocker follow-up or explicit pause before more Phase
12.1 work.

phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof-20260616
accepts tx-selected-register-read-visible. The control and candidate both
passed capture-chain-v4 and serial freshness guard v1 with selected-tree
identity, same-power-cycle TFTP byte agreement, final identity, and restore
proof. The no-MDIO/no-Ethernet control withheld target facts. The candidate
kept NCR 0x10 before/after, completed one TX selector write for value 0x0c00,
read the selected TX shadow register as 0x0e00, and reported
tx-selected-read-completed=true. The proof keeps rx-delay-write-count=0x0,
tx-delay-write-count=0x0, and bmcr-write-count=0x0, so TX delay write/readback,
BMCR restart, convergence polling, link readiness, packet I/O, networking,
sockets, SSH, Phase 12.2, and phase transition remain rejected.

phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-closeout-20260616
accepts tx-selected-register-read-visible-frontier-closed. The closeout
reconciles the local/static discriminator core and serialized Pi 5 proof without
a new hardware run. The closed frontier accepts only selected-tree/TFTP/serial
freshness/final-identity/restore evidence, the no-MDIO/no-Ethernet control, the
TX selector write completion, and selected TX shadow register read visibility at
raw 0x0e00. The next mechanically dependency-satisfied boundary is the queued
post-TX selected-read source checkpoint, which may decide between TX delay
write/readback resume, source-contract correction, or explicit pause. TX delay
write/readback, BMCR restart, convergence polling, link readiness, packet I/O,
networking, sockets, SSH, Phase 12.2, and phase transition remain rejected by
this closeout.

phase12-rp1-ethernet-bcm54213pe-post-tx-selected-read-source-checkpoint-20260616
accepts
bcm54213pe-post-tx-selected-read-source-contract-correction-selected. The
checkpoint reconciles the accepted isolated TX selected-register read success
with the earlier full RGMII delay blocker. The isolated TX discriminator proved
the TX selector write/read path and observed raw 0x0e00, whose SHD data contains
GTXCLK_EN 0x0200. The full RGMII delay proof still reached RX delay
write/readback first and then failed the following TX selected-register read
before TX write, BMCR restart, or convergence polling. The selected next
boundary is supervisor planning for a local/static source-contract correction
that explains the RX-to-TX order/interlock question before any fresh hardware
proof. TX delay write/readback success, BMCR restart after delay configuration,
convergence polling, link readiness, packet I/O, networking, sockets, SSH,
Phase 12.2, and phase transition remain rejected.

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction-20260616
accepts
bcm54213pe-rgmii-delay-tx-order-source-correction-proof-core-selected. Static
source inspection corrects the accepted blocker interpretation before a fresh
hardware attempt: the current RGMII delay candidate keeps its default
rgmii-delay-capture-blocker sentinel after the RX delay read/write/readback
stage succeeds, and that sentinel causes the source path to return before the
TX selected-register read branch. The retained hardware evidence still proves
RX delay write/readback and isolated TX selected-read visibility, but not TX
selected-read failure after RX. The selected next boundary is a local/static
proof core that fixes stage accounting, preserves Linux RX-then-TX order, names
the exact Clause 22 frames and readback masks, and rejects packet/networking/SSH
claims before any Pi 5 proof can be promoted.

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core-20260616
accepts bcm54213pe-rgmii-delay-tx-order-proof-core-local-static. The corrected
candidate/control proof core removes the source-control-flow blocker by recording
separate RX selected-read, RX delay write, RX readback, TX selector write,
TX selected-read, optional TX delay write/readback, BMCR restart, and convergence
poll stages. It preserves the accepted Linux-backed RX-then-TX order and records
an explicit skip policy when the selected TX read already has GTXCLK_EN set. The
next boundary is the serialized Pi 5 proof task; local/static evidence still
rejects hardware success, link readiness, packet I/O, networking, sockets, SSH,
Phase 12.2, and phase transition.

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof-20260616
accepts rgmii-delay-tx-order-timeout-link-not-ready, and
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout-20260616 closes
that frontier as
bcm54213pe-rgmii-delay-tx-order-frontier-closed-timeout-link-not-ready. The
serialized Pi 5 proof retained selected-tree identity, same-power-cycle TFTP
byte agreement, serial nonce freshness, capture-chain-v4 readiness, final
identity, and restore proof for the paired no-MDIO/no-Ethernet control and
corrected candidate. The candidate completed RX delay write/readback, completed
TX selected read/readback with GTXCLK_EN already set, skipped the redundant TX
write under the accepted policy, executed exactly one BMCR restart, and then
timed out after eight convergence samples with BMSR link-status=false, BMSR
autoneg-complete=false, passive MACB_NSR link=false, and
link-ready-terminal=false. The link-ready packet-readiness checkpoint remains
blocked; supervisor planning is required for any timeout/link-not-ready
follow-up or explicit pause. Packet I/O, networking, sockets, SSH, Phase 12.2,
and phase transition remain rejected.

phase12-rp1-ethernet-bcm54213pe-post-txorder-link-not-ready-source-checkpoint-20260616
accepts
bcm54213pe-post-txorder-link-not-ready-no-distinct-source-backed-discriminator-pause.
It reconciles the accepted TX-order timeout/link-not-ready frontier against the
prior BMCR/autoneg, convergence, RGMII delay, GPIO32/reset, physical-link, and
BCM54213PE source evidence. No mechanically ready, source-backed,
qualitatively distinct link-not-ready discriminator is selected: GPIO32 /
ETH_RST_N reset ownership remains held by the persistent-or-firmware-owned
event-state blocker, MII_CTRL1000 master-mode writes remain behind an
unselected PHY_BRCM_EN_MASTER_MODE gate, and interrupt, APD/EEE/lifecycle,
MAC/phylink, packet, networking, sockets, SSH, Phase 12.2, and phase-transition
work all require separate planning.

phase12-rp1-ethernet-bcm54213pe-link-not-ready-frontier-pause-closeout-20260616
accepts
bcm54213pe-link-not-ready-frontier-paused-return-to-generated-root-transport.
This closeout pauses Phase 12.1 at the accepted timeout/link-not-ready frontier
instead of repeating status evidence or shrinking the claim to a shim. The
accepted Ethernet evidence remains useful as retained context, but it does not
unblock link-ready packet-readiness, GPIO32/PHY reset action, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition. The next selected
non-Ethernet task is
phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract-20260616,
which returns the program to the earlier Pi 5 generated-root boot transport
blocker.

phase12-rp1-ethernet-post-generated-root-link-not-ready-resumption-source-checkpoint-20260618
accepts
post-generated-root-link-not-ready-no-distinct-discriminator-planning-needed,
and
phase12-rp1-ethernet-post-generated-root-link-not-ready-pause-closeout-20260618
accepts post-generated-root-link-not-ready-frontier-paused-planning-required.
Pi 5 generated-root command-input success closes the non-Ethernet detour, but
it does not change the retained BCM54213PE Ethernet terminal facts: link-ready
and autoneg-complete remain unaccepted. The generic selected-link-not-ready
discriminator core remains dependency-gated because selected_discriminator and
selected_next_task are null. GPIO32 / ETH_RST_N reset ownership, MII_CTRL1000
master-mode writes, interrupts, APD/EEE/lifecycle, MAC/phylink, packet I/O,
networking, sockets, SSH, Phase 12.2, and phase transition all require future
supervisor-planned scope.

phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection-20260618
accepts bcm54213pe-link-not-ready-master-mode-gate-source-contract-selected.
The selection reopens Phase 12.1 only for a local/static BCM54213PE source
contract, not for hardware. The selected discriminator is
bcm54213pe-phy1-mii-ctrl1000-master-mode-gate-source-contract, with selected
next task
phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core-20260618.
Linux source backs the narrow candidate through bcm54xx_config_init ->
bcm54213pe_config_init -> bcm54210e_config_init, where
PHY_BRCM_EN_MASTER_MODE gates a PHY1 MII_CTRL1000 read/modify/write that sets
CTL1000_AS_MASTER and CTL1000_ENABLE_MASTER. The next task must keep this as
source/static contract work and may pause or block if the gate cannot be
safely selected. GPIO32 / ETH_RST_N reset, interrupts, APD/EEE/lifecycle,
MAC/phylink, packet I/O, networking, sockets, SSH, Phase 12.2, phase
transition, and same-shaped timeout/status/restart/poll/capture retries remain
rejected from this selection.

phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core-20260618
accepts bcm54213pe-mii-ctrl1000-master-mode-source-contract-core-local-static.
The local/static contract now records the selected MII_CTRL1000 master-mode gate
candidate/control surface: PHY1 MII_CTRL1000 0x09 pre-read, MAN read frame
0x60a60000, write prefix 0x50a60000, CTL1000_AS_MASTER 0x0800,
CTL1000_ENABLE_MASTER 0x1000, accepted pre-value 0x0200, expected write value
0x1a00, and expected write frame 0x50a61a00. The paired control withholds MDIO,
MAN, MACB, GPIO32/PHY, interrupt, packet, networking, and SSH target facts. The
next mechanically gated task is
phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof-20260618,
which must provide hardwareTestLock, selected-tree/TFTP/serial/final-identity,
restore, paired control, and post-hardware review evidence before any runtime
claim can be accepted. Link-ready, autoneg-complete, packet I/O, networking,
SSH, Phase 12.2, phase transition, and same-shaped retry claims remain
rejected.

phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof-20260618
accepts mii-ctrl1000-master-mode-write-readback-visible. The decisive candidate
rerun retained selected tree
515684b45744c6c89847652c1b34d643a850094d4da3101207fa3b4462d00784,
same-power-cycle 50936-byte TFTP serves, fresh serial nonce evidence, final
pre-restore identity, and restore proof. It observed PHY1 MII_CTRL1000 pre-read
0x0200, write value 0x1a00, and readback 0x1a00. The paired control retained
the no-MDIO/no-Ethernet classification. This proves only the selected
MII_CTRL1000 master-mode write/readback boundary; link-ready, autoneg-complete,
GPIO32/PHY reset ownership, interrupts, APD/EEE/lifecycle, MAC/phylink, packet
I/O, networking, sockets, SSH, Phase 12.2, phase transition, and same-shaped
status/restart/poll/capture retries remain rejected.

phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout-20260618
accepts
bcm54213pe-master-mode-write-readback-frontier-paused-planning-required. The
closeout reconciles the selected source/core and Pi 5 proof into the current
Phase 12.1 frontier: MII_CTRL1000 master-mode write/readback is hardware
visible, but no next Ethernet task is mechanically unblocked. selected_next_task
is null, planningNeeded=true, and supervisor planning is required before any
future Phase 12.1 hardware action, packet I/O, networking, SSH, Phase 12.2, or
phase transition.

phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core-20260618
accepts bcm54213pe-master-mode-autoneg-source-contract-core-local-static. The
source/static contract is a distinct boundary from same-shaped status polling and
bare BMCR restart retries because the future candidate must first repeat the
accepted PHY1 MII_CTRL1000 master-mode read/modify/write/readback sequence
(pre-read 0x0200, write/readback 0x1a00) and only then issue one BMCR autoneg
enable plus restart frame 0x50821200 followed by bounded BMCR, double-sampled
BMSR, ANAR, ANLPAR, MII_CTRL1000, MII_STAT1000, and passive MACB_NSR_LINK
sampling. The paired control constructs no MDIO, MAN, MACB, GPIO32/PHY,
interrupt, packet, networking, or SSH target facts. The selected next task is
phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof-20260618, which
must retain selected-tree/TFTP/serial/final-identity/restore evidence before any
link-ready or autoneg-complete claim can be accepted. Packet I/O, networking,
sockets, SSH, Phase 12.2, phase transition, GPIO32 reset, interrupts,
APD/EEE/lifecycle, MAC/phylink, marker-only retry, same-shaped status-only retry,
and bare BMCR restart retry remain rejected.

phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof-20260618
accepts bcm54213pe-master-mode-autoneg-timeout-link-not-ready. The paired
control retained the no-MDIO/no-Ethernet shape with selected-tree/TFTP/serial,
final pre-restore identity, and restore evidence. The candidate retained
selected tree 8b9eddafc3f0210f4be8c2c0f649286e0f92a17f65e0611952b618c89af03b7d,
same-power-cycle 54072-byte TFTP serves, fresh serial nonce evidence, final
pre-restore identity, and restore proof. It repeated the accepted MII_CTRL1000
master-mode sequence with pre-read 0x0200, write value 0x1a00, and readback
0x1a00, then issued exactly one BMCR autoneg enable/restart write 0x1200. The
bounded terminal sample reported BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1,
ANLPAR 0x0000, MII_CTRL1000 0x1a00, MII_STAT1000 0x0000, passive MACB_NSR
0x00000006, BMSR link false, BMSR autoneg-complete false, and MACB_NSR_LINK
false. This accepts only the master-mode plus BMCR-autoneg-restart execution and
timeout/link-not-ready classification; link-ready, autoneg-complete, GPIO32/PHY
reset ownership, interrupts, APD/EEE/lifecycle, MAC/phylink, packet I/O,
networking, sockets, SSH, Phase 12.2, phase transition, and more same-shaped
status/autoneg polling remain rejected.

phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout-20260618
accepts
bcm54213pe-master-mode-autoneg-frontier-paused-link-not-ready-planning-required.
The closeout reconciles the accepted source contract and Pi 5 proof into the
current Phase 12.1 frontier: MII_CTRL1000 master-mode write/readback and one
BMCR autoneg restart are hardware visible, but direct terminal evidence still
shows BMSR link false, BMSR autoneg-complete false, and MACB_NSR_LINK false.
The link-ready packet-readiness checkpoint is not mechanically unblocked.
selected_next_task is null, planningNeeded=true, and supervisor planning is
required before any future Phase 12.1 hardware action, packet I/O, networking,
SSH, Phase 12.2, or phase transition.

phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-source-checkpoint-20260618
accepts
bcm54213pe-post-master-mode-autoneg-no-distinct-source-backed-discriminator-pause.
The source checkpoint preserves the same hardware frontier and classifies the
remaining candidate families without selecting a follow-up: GPIO32 /
ETH_RST_N reset ownership remains blocked by persistent-or-firmware-owned
event state; same-shaped status/autoneg/convergence retries and link-ready
packet-readiness are rejected; prior RGMII delay/TX-order work is already
closed as timeout/link-not-ready; APD, EEE, interrupt ISR/IMR/ECR,
suspend/resume lifecycle, and MAC/phylink work are deferred pending new
supervisor-planned source scope. selected_discriminator and selected_next_task
are null, planningNeeded=true, and no hardware, GPIO32/PHY reset,
interrupt/APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, SSH, Phase
12.2, or phase-transition work is mechanically unblocked.

phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-pause-closeout-20260618
accepts
bcm54213pe-post-master-mode-autoneg-frontier-paused-no-distinct-discriminator.
The closeout freezes this Phase 12.1 frontier after the accepted source
checkpoint: MII_CTRL1000 master-mode write/readback and one BMCR autoneg restart
remain the only accepted hardware-visible behavior, while BMSR link, BMSR
autoneg-complete, and MACB_NSR_LINK remain false. selected_discriminator and
selected_next_task remain null, planningNeeded=true, and supervisor/human
strategy planning is required before any selected-discriminator core, hardware
proof, GPIO32/PHY reset action, interrupt/APD/EEE/lifecycle work, MAC/phylink,
packet I/O, networking, SSH, Phase 12.2, or phase-transition work.

phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint-20260618
accepts
bcm54213pe-low-power-lifecycle-no-distinct-source-backed-discriminator-pause.
This source/static checkpoint refines the previously deferred low-power,
EEE, interrupt, WOL/IDDQ, suspend/resume, BMCR powerdown, soft-reset, and
config_init/lifecycle source surfaces. It selects no next discriminator:
APD, EEE, WOL/IDDQ, resume, and soft-reset paths are write/restore lifecycle
ownership; ISR and WOL status reads have side effects; interrupt IMR/ECR paths
need ownership and restore rules; MAC/phylink remains a broader boundary; and
same-shaped status/autoneg polling repeats the accepted timeout/link-not-ready
frontier. selected_discriminator and selected_next_task are null,
planningNeeded=true, and no hardware, GPIO32/PHY reset, APD/EEE/lifecycle,
interrupt, MAC/phylink, packet I/O, networking, SSH, Phase 12.2, or
phase-transition work is mechanically unblocked.

phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-pause-closeout-20260618
accepts
bcm54213pe-low-power-lifecycle-frontier-paused-no-distinct-discriminator.
The closeout freezes the current Phase 12.1 Ethernet frontier after the
low-power/lifecycle checkpoint: MII_CTRL1000 master-mode write/readback and one
BMCR autoneg restart remain the only accepted hardware-visible BCM54213PE
behavior, while BMSR link, BMSR autoneg-complete, ANLPAR, MII_STAT1000, and
MACB_NSR_LINK remain not ready. The accepted checkpoint's APD/EEE/WOL/IDDQ,
suspend/resume, BMCR powerdown, soft-reset, interrupt, config_init, and
MAC/phylink findings are preserved as deferred or rejected rather than reopened
as implementation work. selected_discriminator and selected_next_task remain
null, planningNeeded=true, and the next strategy checkpoint is
phase12-rp1-ethernet-strategy-decision-checkpoint-after-low-power-lifecycle-20260618.
Hardware, GPIO32/PHY reset action, APD/EEE/lifecycle ownership, interrupt
ownership, MAC/phylink, packet I/O, networking, sockets, SSH, Phase 12.2, phase
transition, and same-shaped status/autoneg retry work remain explicitly
rejected until an explicit supervisor/human strategy selection changes the
frontier.

phase12-rp1-ethernet-link-ready-discriminator-source-contract-20260621 accepts
phase12-rp1-ethernet-link-ready-discriminator-source-contract-blocked-no-defensible-discriminator.
The source contract reconciles the accepted post-master-mode/autoneg
link-not-ready frontier, retained BCM54213PE Linux source evidence, current
rp1_ethernet guardrails, and the driver packet adapter closeout. It selects no
new discriminator: the adapter closeout is host/QEMU-substitute packet plumbing
only and adds no PHY/MAC link fact; same-shaped BMCR restart/status/autoneg
polling remains rejected; GPIO32 reset ownership remains blocked; RGMII
delay/TX-order and MII_CTRL1000 master-mode plus BMCR restart are already
closed as link-not-ready; APD/EEE/lifecycle, interrupt, and MAC/phylink work
need broader ownership and restore scope. selected_discriminator and
selected_next_task are null, planningNeeded=true, and the queued generic
core/proof/closeout chain is not mechanically unblocked without new source
evidence or explicit supervisor/human strategy selection.

phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core-20260621
accepts bcm54213pe-lifecycle-ownership-powerdown-exit-source-core-local-static
after supervisor strategy selection authorized a broader low-power/lifecycle
ownership slice. The selected discriminator is
bcm54213pe-phy1-bmcr-powerdown-exit-gate: the later candidate may pre-read
PHY1 BMCR, clear only BMCR_PDOWN bit 11 if that bit is set, preserve all other
BMCR bits, wait at least 40us after any powerdown exit, and then post-sample
BMCR, double-sampled BMSR, ANAR, ANLPAR, MII_CTRL1000, MII_STAT1000, and
passive MACB_NSR_LINK context. The paired control constructs no MDIO, MAN,
MACB, GPIO32/PHY, interrupt, packet, networking, or SSH targets. APD, EEE,
IDDQ/TOP_MISC, soft reset without accepted IDDQ prerequisite, interrupt
ISR/IMR/ECR access, broad config_init replay, GPIO32 reset, MAC/phylink, live
packet I/O, reachability, SSH, broad socket expansion, and phase transition
remain rejected. Local/static evidence does not accept link-ready,
autoneg-complete, packet-readiness, or live RX/TX. selected_next_task is
phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621.

phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621
accepts bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready. The
serialized Pi 5 proof retained selected-tree identity, same-power-cycle TFTP
byte evidence, serial nonce freshness, final pre-restore identity, and restore
to the pre-run boot tree. The no-MDIO/no-Ethernet control classified
no-mdio-no-ethernet-bcm54213pe-lifecycle-ownership-control. The candidate
observed BMCR 0x1000 with BMCR_PDOWN already clear, so the selected
fail-closed gate performed no BMCR clear write; post-sampling retained BMCR
0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MII_CTRL1000 0x0200,
MII_STAT1000 0x0000, and passive MACB_NSR_LINK=false. This accepts only the
no-change link-not-ready terminal. It does not accept link-ready,
autoneg-complete, packet-readiness, live RX/TX, packet I/O, networking, SSH,
Phase 12.2, or a phase transition. The hardware-visible BCM54213PE frontier is
now MII_CTRL1000 master-mode write/readback, one BMCR autoneg restart, and a
BMCR_PDOWN-exit gate observed no-change because PDOWN was already clear.

phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout-20260621
accepts
bcm54213pe-lifecycle-ownership-closeout-no-link-ready-planning-needed. The
closeout reconciles the accepted source/core and serialized Pi 5 proof as a
no-change link-not-ready Phase 12.1 frontier. The selected BMCR_PDOWN exit gate
did not issue a clear write because BMCR_PDOWN was already clear, while BMSR
link, BMSR autoneg-complete, and passive MACB_NSR_LINK all remained false.

No queued link-ready discriminator core/proof/closeout task is mechanically
unblocked by this terminal, because those tasks require accepted selected
link-ready evidence that the lifecycle proof rejected. Link-ready,
autoneg-complete, packet-readiness, live RX/TX, packet I/O, ping/hardware
reachability, Ethernet driver readiness, networking, sockets, SSH, Phase 12.2,
public ABI/POSIX/Linux compatibility, broad socket expansion, and phase
transition remain unaccepted. selected_next_task=null and planningNeeded=true
pending supervisor planning for any further Phase 12.1 hardware strategy or
return to host-only network work.

## Phase 12.2 Host Network Abstraction Core

phase12-network-device-abstraction-ethernet-arp-ip-host-core-20260618 accepts
phase12-network-device-abstraction-ethernet-arp-ip-host-core-local-static. This
is host/testable protocol-boundary progress after the explicit strategy choice
to defer further BCM54213PE link-hardware probing. src/network.rs defines a
no_std boundary where NetworkDevice implementors own raw frame movement and the
protocol layer parses immutable byte slices.

The accepted parser surface covers Ethernet II destination/source/EtherType
splitting, Ethernet/IPv4 ARP request/reply shape parsing, and IPv4
version/IHL/total-length/protocol/source/destination/payload parsing. Unit tests
cover positive and negative Ethernet, ARP, and IPv4 cases, including truncated
frames and malformed ARP/IPv4 headers.

This task does not change the Phase 12.1 hardware frontier. MII_CTRL1000
master-mode write/readback plus one BMCR autoneg restart remain the only
accepted hardware-visible BCM54213PE behavior, while link-ready,
autoneg-complete, packet-readiness, live packet I/O, DMA descriptor ownership,
RP1 Ethernet driver readiness, sockets, SSH, and phase transition remain
unaccepted.

## Phase 12.3 Local Packet Dispatch and ICMP Echo

phase12-network-local-packet-dispatch-icmp-echo-core-20260619 accepts
phase12-network-local-packet-dispatch-icmp-echo-core-accepted. This is local
source/test IP-stack progress following the smoltcp evaluation checkpoint; it
does not add smoltcp, live packet I/O, sockets, SSH, or hardware-driver
readiness.

src/network.rs now includes dispatch_local_packet over immutable input frames
and caller-provided output buffers. The accepted shapes are Ethernet/ARP
requests targeting the configured local IPv4 identity with a local or broadcast
Ethernet destination, and Ethernet/IPv4/ICMP echo requests targeting the
configured local IPv4/MAC identity. Reply generation writes Ethernet/ARP
replies and Ethernet/IPv4/ICMP echo replies into caller-owned buffers,
including IPv4 and ICMP checksum generation.

The accepted error boundary rejects unsupported EtherTypes, non-ICMP IPv4
protocols, IPv4 options, IPv4 fragments, invalid IPv4 checksums, malformed ICMP
echo input, invalid ICMP checksums, nonlocal Ethernet/IP destinations, and
too-small output buffers deterministically.

phase12-network-packet-buffer-device-polling-core-20260619 accepts
phase12-network-packet-buffer-device-polling-core-accepted. src/network.rs now
includes poll_local_network_device, a one-step local polling boundary that
receives into caller-owned storage through NetworkDevice, dispatches with
dispatch_local_packet, and transmits from caller-owned storage only when ARP or
ICMP echo reply generation succeeds. LocalPollStepResult distinguishes no
frame, receive-buffer pressure, receive error, nonlocal no-reply, dispatch
error, transmit error, and successful reply transmission without allocation.

phase12-network-arp-cache-core-20260619 accepts
phase12-network-arp-cache-core-accepted. src/network.rs now includes a
fixed-capacity, allocation-free ArpCache boundary that stores IPv4-to-MAC
neighbors inside caller/kernel-owned fixed storage. The accepted API covers
lookup miss and hit, insertion, existing-entry update, zero-capacity
no-state-change behavior, and deterministic oldest-slot round-robin replacement
when the cache is full.

The accepted ARP learning helper validates Ethernet II ARP frames and
Ethernet/IPv4 ARP packet shape before recording sender protocol/hardware
address facts from ARP requests and replies. Malformed, truncated, unsupported
EtherType, and unsupported ARP operation inputs return PacketError without
changing existing cache state. This cache-only slice does not wire neighbor
state into dispatch_local_packet, outbound resolution, packet queues, driver
adapters, live packet I/O, ping behavior, sockets, SSH, smoltcp adoption,
network reachability, link readiness, or a phase transition.

phase12-network-arp-cache-dispatch-integration-core-20260619 accepts
phase12-network-arp-cache-dispatch-integration-core-accepted. src/network.rs
now includes dispatch_local_packet_with_arp_cache and
poll_local_network_device_with_arp_cache, compatibility-preserving wrappers
that learn valid Ethernet/IPv4 ARP sender facts through a caller-provided
ArpCache before using the existing local dispatch and poll behavior.

The cache-aware path records sender IPv4/MAC facts from valid ARP requests and
ARP replies. ARP requests can still produce the same local ARP reply as the
cache-unaware dispatcher; ARP replies learn the neighbor and produce no
transmit. Malformed or unsupported ARP does not mutate the cache, non-ARP ICMP
echo behavior remains byte-for-byte compatible with dispatch_local_packet,
no-frame leaves cache state unchanged, and transmit errors keep any ARP fact
learned before reply transmission failed.

Driver adapters, packet queues, outbound neighbor resolution, UDP/TCP, DHCP,
DNS, routing, smoltcp integration, sockets, SSH, live packet I/O, RP1 Ethernet
driver readiness, link readiness, ping/network reachability behavior, and phase
transition remain unaccepted.

phase12-network-arp-cache-dispatch-integration-closeout-20260619 accepts
phase12-network-arp-cache-dispatch-integration-closeout-accepted. The closeout
confirms the host-only cache-aware local dispatch/poll frontier and selects the
queued outbound-neighbor-resolution source checkpoint as the next bounded
Phase 12.3 planning task. It does not authorize outbound neighbor-resolution
implementation, packet queues, driver adapters, live packet I/O, sockets, SSH,
network reachability, ping behavior, hardware readiness, or phase transition.

phase12-network-outbound-neighbor-resolution-core-20260619 accepts
phase12-network-outbound-neighbor-resolution-core-accepted. src/network.rs now
includes OutboundNeighborResolution and resolve_outbound_neighbor, a pure
host-only helper that reads immutable ArpCache state to return either the
cached destination MAC for a known IPv4 neighbor or an unresolved result
carrying the destination IPv4.

The accepted boundary is cached-only and allocation-free. Updated cache entries
are reflected by later resolution calls, zero-capacity caches remain
deterministic misses, and cache-aware poll learning remains compatible with
later resolution. ARP request emission, retry timers, packet queues, routing,
subnet/gateway selection, outbound frame construction, driver transmit
scheduling, live packet I/O, RP1 Ethernet driver readiness, smoltcp adoption,
sockets, SSH, ping/network reachability behavior, and phase transition remain
unaccepted.

phase12-network-outbound-neighbor-resolution-closeout-20260619 accepts
phase12-network-outbound-neighbor-resolution-closeout-accepted. The closeout
confirms the host-only cached-resolution frontier and selects the queued
outbound-frame-construction source checkpoint as the next bounded Phase 12.3
planning task. It does not authorize ARP request emission, packet queues,
driver transmit, live packet I/O, sockets, SSH, network reachability, ping
behavior, hardware readiness, or phase transition.

phase12-network-outbound-frame-construction-core-20260619 accepts
phase12-network-outbound-frame-construction-core-accepted. src/network.rs now
includes build_outbound_ethernet_frame and OutboundFrameError, a pure
host-only caller-buffered Ethernet II construction boundary for already
resolved outbound neighbors.

The accepted helper writes destination MAC, source MAC, EtherType, and exact
payload bytes into caller-owned output storage and returns the deterministic
frame length. Unresolved neighbors and too-small output buffers are rejected
before any frame bytes are accepted as progress. The helper composes with the
cached outbound neighbor resolver, remains allocation-free, and does not
mutate ARP cache state, consult a driver, transmit frames, queue packets, emit
ARP requests, construct IPv4/ICMP requests, adopt smoltcp, claim sockets/SSH,
claim ping/network reachability behavior, claim RP1 Ethernet readiness, or
change the Phase 12.1 hardware frontier.

phase12-network-outbound-frame-construction-closeout-20260619 accepts
phase12-network-outbound-frame-construction-closeout-accepted. The closeout
confirms the host-only caller-buffered Ethernet II frame-construction frontier
and selects no next worker task. selected_next_task is null,
planningNeeded=true, and supervisor planning is required before any further
Phase 12.3 work such as outbound IPv4/ICMP request construction, ARP request
emission, neighbor-discovery plumbing, packet queues, driver transmit, live
packet I/O, sockets, SSH, ping/network reachability, hardware readiness, or
phase transition.

phase12-network-outbound-ipv4-icmp-echo-request-core-20260619 accepts
phase12-network-outbound-ipv4-icmp-echo-request-core-accepted. src/network.rs
now includes build_outbound_ipv4_icmp_echo_request, a pure host-only helper
that builds a complete Ethernet II IPv4 ICMP echo request frame for an already
resolved outbound neighbor into caller-owned storage.

The accepted helper writes deterministic Ethernet destination/source MACs,
IPv4 EtherType, IPv4 version/IHL/total length/TTL/protocol/source/destination
and checksum fields, ICMP echo request type/code/identifier/sequence/payload
and checksum fields, and returns the deterministic frame length. Unresolved
neighbors, too-small output buffers, and oversized IPv4 payloads are rejected
before partial frame construction is accepted as success. The helper composes
with the cached outbound neighbor resolver, remains allocation-free, and does
not mutate ARP cache state, access a driver, queue packets, transmit frames,
emit ARP requests, adopt smoltcp, claim sockets/SSH, claim ping/network
reachability behavior, claim RP1 Ethernet readiness, or change the Phase 12.1
hardware frontier.

phase12-network-outbound-ipv4-icmp-echo-request-closeout-20260619 accepts
phase12-network-outbound-ipv4-icmp-echo-request-closeout-accepted. The
closeout preserves the exact host-only caller-buffered Ethernet IPv4 ICMP echo
request construction frontier for already resolved neighbors and selects the
queued phase12-network-arp-request-emission-source-checkpoint-20260619 as the
next same-slice source/static checkpoint. The selected checkpoint is bounded to
unresolved-neighbor/ARP-request planning only; it does not authorize ARP
request implementation, retry timers, packet queues, driver transmit, live
packet I/O, sockets, SSH, smoltcp adoption, ping/network reachability behavior,
Pi 5 hardware work, boot publication, lab mutation, link-readiness work, or a
phase transition.

phase12-network-arp-request-emission-core-20260619 accepts
phase12-network-arp-request-emission-core-accepted. src/network.rs now includes
build_outbound_arp_request, a pure host-only helper that builds a complete
Ethernet II ARP request frame for a local endpoint and target IPv4 into
caller-owned storage.

The accepted helper writes deterministic broadcast destination MAC, local
source MAC, ARP EtherType, Ethernet/IPv4 hardware and protocol fields, ARP
request operation, endpoint sender MAC/IP, zero target MAC, caller-provided
target IPv4, and exact frame length. Too-small output buffers are rejected
before partial frame construction is accepted as success. The helper composes
with unresolved outbound neighbor resolution by using the unresolved target
IPv4, remains allocation-free, and does not mutate ARP cache state, consult a
driver, transmit frames, queue packets, schedule retries, claim live packet
I/O, claim sockets/SSH, claim ping/network reachability behavior, claim RP1
Ethernet readiness, or change the Phase 12.1 hardware frontier.

phase12-network-arp-request-emission-closeout-20260619 accepts
phase12-network-arp-request-emission-closeout-accepted-planning-needed. The
closeout preserves the exact host-only caller-buffered Ethernet/IPv4 ARP
request construction frontier and selects no follow-up implementation task.
Supervisor planning is required before packet queues, retry timers,
neighbor-discovery state, driver transmit, live packet I/O, sockets, SSH,
smoltcp adoption, ping/network reachability behavior, Pi 5 hardware work, boot
publication, lab mutation, link-readiness work, or a phase transition.

phase12-network-outbound-request-selection-core-20260619 accepts
phase12-network-outbound-request-selection-core-accepted. src/network.rs now
includes OutboundRequestKind, OutboundRequestSelection, and
select_outbound_ipv4_icmp_echo_request, a pure host-only helper that selects
one caller-buffered outbound request frame for a requested IPv4 ICMP echo.

The accepted selector reads immutable ArpCache state through the accepted
resolve_outbound_neighbor helper. A resolved destination builds an
Ethernet/IPv4/ICMP echo request using the accepted ICMP request constructor; an
unresolved destination builds an Ethernet/IPv4 ARP request using the accepted
ARP request constructor. The result reports deterministic request kind and
frame length. Buffer pressure and oversized resolved ICMP payloads remain
deterministic errors. The selector does not mutate ARP cache state, call or
wrap NetworkDevice, transmit frames, queue packets, schedule retries, claim
live packet I/O, claim sockets/SSH, claim ping/network reachability behavior,
claim RP1 Ethernet readiness, or change the Phase 12.1 hardware frontier.

phase12-network-outbound-request-selection-closeout-20260619 accepts
phase12-network-outbound-request-selection-closeout-accepted. The closeout
preserves the host-only one-frame request-selection frontier and selects
phase12-network-outbound-one-shot-device-transmit-core-20260619 as the next
mechanically unblocked task. The selected follow-up is limited to a
fake/trait-level one-shot NetworkDevice transmit wrapper over the accepted
selector. Packet queues, retry timers, live driver transmit, live packet I/O,
hardware work, sockets, SSH, smoltcp adoption, ping/network reachability
behavior, RP1 Ethernet readiness, lab mutation, boot publication, and phase
transition remain rejected.

phase12-network-outbound-one-shot-device-transmit-core-20260619 accepts
phase12-network-outbound-one-shot-device-transmit-core-accepted. src/network.rs
now includes OutboundTransmitResult and
transmit_one_outbound_ipv4_icmp_echo_request, a fake/trait-level host helper
that composes the accepted outbound request selector with
NetworkDevice::transmit_frame.

The helper builds one resolved Ethernet/IPv4/ICMP echo request or unresolved
Ethernet/IPv4 ARP request into caller-owned storage and calls
NetworkDevice::transmit_frame exactly once after successful construction. The
result distinguishes ICMP echo request transmit, ARP request transmit,
request-selection/build error with no transmit attempt, and transmit error with
request kind/frame length. Tests use fake NetworkDevice implementations only.
Packet queues, retry timers, receive loops, live driver transmit, live packet
I/O, hardware work, sockets, SSH, smoltcp adoption, ping/network reachability
behavior, RP1 Ethernet readiness, lab mutation, boot publication, and phase
transition remain rejected.

phase12-network-outbound-one-shot-device-transmit-closeout-20260619 accepts
phase12-network-outbound-one-shot-device-transmit-closeout-accepted. The
closeout preserves the fake/trait-level one-shot transmit frontier and selects
phase12-network-phase12-3-host-frontier-closeout-20260619 as the next
mechanically unblocked task.

The selected checkpoint is limited to reconciling accepted host/testable Phase
12.3 receive dispatch, ARP cache learning/resolution, outbound ICMP/ARP
construction, request selection, and one-shot trait-level transmit behavior.
Packet queues, retry timers, routing, live driver transmit, live packet I/O,
hardware work, sockets, SSH, smoltcp adoption, ping/network reachability
behavior, RP1 Ethernet readiness, lab mutation, boot publication, and phase
transition remain rejected.

phase12-network-phase12-3-host-frontier-closeout-20260619 accepts
phase12-network-phase12-3-host-frontier-closeout-accepted-planning-needed. The
checkpoint reconciles the host/testable Phase 12.3 frontier through local
receive dispatch, ARP cache learning/resolution, caller-buffered outbound ICMP
and ARP construction, immutable request selection, and fake/trait-level
one-shot NetworkDevice transmit.

This checkpoint accepts no live networking behavior. Packet queues, retry
timers, neighbor-discovery state beyond immutable cache lookup/request
emission, routing/subnet/gateway policy, live driver adapters, smoltcp
adoption, UDP/TCP, sockets, hardware packet I/O, ping/network reachability
behavior, SSH, Pi 5 hardware work, boot publication, lab mutation, and phase
transition remain rejected. selected_next_task is null and planningNeeded=true
because no later queued Phase 12.3 task has complete scope, non-goals,
dependencies, acceptance criteria, validation gates, docs, and evidence
requirements.

phase12-network-single-pending-icmp-after-arp-resolution-core-20260619 accepts
phase12-network-single-pending-icmp-after-arp-resolution-core-accepted.
src/network.rs now includes an allocation-free SinglePendingIcmpEcho boundary
for exactly one unresolved outbound IPv4 ICMP echo request.

The accepted helper can emit one deterministic Ethernet/IPv4 ARP request for
an unresolved destination and retain the endpoint, destination IPv4,
identifier, sequence number, TTL, and payload bytes in fixed caller-selected
storage after successful fake/trait-level transmit. A matching Ethernet/IPv4
ARP reply, or an existing accepted ARP cache resolution, can advance that
pending request into one deterministic Ethernet/IPv4/ICMP echo request transmit
and clear pending state after successful transmit.

The accepted boundary covers resolved-neighbor, no-pending,
duplicate-pending/backpressure, payload pressure, output-buffer pressure,
malformed ARP, nonmatching ARP, unresolved cache, and transmit-error behavior
with local unit tests. It remains host/testable only. Packet queues, retry
timers, multi-entry buffering, route/subnet/gateway policy, live driver
adapters, smoltcp adoption, UDP/TCP, sockets, hardware packet I/O,
ping/network reachability behavior, SSH, Pi 5 hardware work, boot publication,
lab mutation, and phase transition remain rejected.

phase12-network-single-pending-icmp-after-arp-resolution-closeout-20260619
accepts
phase12-network-single-pending-icmp-after-arp-resolution-closeout-accepted.
The closeout reconciles the host-only single-pending ARP-to-ICMP progression
with source, task, test, docs, and evidence records. It also corrects the core
classification JSON evidence so the recorded jq validation claim is true.

The accepted boundary remains limited to one allocation-free pending outbound
ICMP echo request behind ARP resolution. Route policy is selected as the next
bounded host-only Phase 12.3 task, but this closeout does not implement or
accept routing behavior. Packet queues, retry timers, multi-entry buffering,
live driver adapters, smoltcp adoption, sockets, SSH, ping/network reachability
behavior, hardware packet I/O, Pi 5 hardware work, lab mutation, boot
publication, and phase transition remain rejected.

phase12-network-local-ipv4-egress-route-policy-core-20260619 accepts
phase12-network-local-ipv4-egress-route-policy-core-accepted. src/network.rs
now includes a host-only Ipv4EgressRoutePolicy and route decision boundary for
local IPv4 egress.

The accepted policy chooses the destination IPv4 itself as the ARP next hop for
same-subnet destinations, chooses a configured gateway IPv4 as the ARP next hop
for off-subnet destinations, and reports a deterministic no-route result when
an off-subnet destination has no gateway. A routed outbound ICMP selector can
consume that decision without mutating ARP cache state: same-subnet routes
preserve the existing direct behavior, gateway routes address the Ethernet
frame to the gateway MAC while keeping the IPv4 packet destination as the final
destination, and unresolved gateway routes emit an ARP request for the gateway
IPv4 rather than the final destination.

The accepted boundary is still local source/test behavior over caller-owned
buffers and fake/trait-level packet construction only. Dynamic routing, DHCP,
DNS, packet queues, retry timers, live driver adapters, smoltcp adoption,
UDP/TCP, sockets, hardware packet I/O, ping/network reachability behavior,
SSH, Pi 5 hardware work, boot publication, lab mutation, and phase transition
remain rejected.

phase12-network-local-ipv4-egress-route-policy-closeout-20260619 accepts
phase12-network-local-ipv4-egress-route-policy-closeout-accepted-planning-needed.
The closeout reconciles the accepted route-policy behavior with source, unit
test, task, docs, and evidence records. The Phase 12.3 host-only frontier now
includes deterministic destination-vs-gateway next-hop selection and no-route
handling for local IPv4 egress.

Supervisor planning later selected
phase12-network-routed-single-pending-icmp-after-arp-resolution-core-20260619
as the next bounded host-only Phase 12.3 task. Retry timing, packet queues,
route-table expansion, live driver adapters, smoltcp adoption, UDP/TCP,
sockets, hardware packet I/O, ping/network reachability behavior, SSH, Pi 5
hardware work, boot publication, lab mutation, and phase transition remain
rejected except through the explicit queued dependency chain.

phase12-network-routed-single-pending-icmp-after-arp-resolution-core-20260619
accepts
phase12-network-routed-single-pending-icmp-after-arp-resolution-core-accepted.
src/network.rs now carries both final destination IPv4 and ARP next-hop IPv4
inside PendingIcmpEchoRequest and adds a route-aware single-pending ICMP entry
point.

The accepted host-only boundary applies route_ipv4_egress before pending ICMP
ARP resolution. Same-subnet unresolved destinations emit ARP for the
destination and store a pending request whose next hop is the destination.
Gateway-routed unresolved destinations emit ARP for the configured gateway and
store the final IPv4 destination separately from the gateway next hop. Matching
ARP resolution for that next hop transmits an ICMP echo request to the final
IPv4 destination using the resolved next-hop MAC and clears pending state after
successful fake-device transmit. Off-subnet no-gateway requests report
NoRouteToDestination before output mutation, device transmit, or pending-state
mutation.

The existing direct single-pending API remains available and stores direct
requests with next_hop_ipv4 equal to destination_ipv4. This accepted boundary
is source/test evidence only over caller-owned buffers and fake NetworkDevice
transmit. Packet queues, retry timers, route-table expansion, live driver
adapters, smoltcp adoption, UDP/TCP, sockets, hardware packet I/O,
ping/network reachability behavior, SSH, Pi 5 hardware work, boot publication,
lab mutation, and phase transition remain rejected.

phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout-20260619
accepts
phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout-accepted.
The closeout reconciles the accepted route-aware pending ICMP boundary with
source, unit-test, task, docs, and evidence records. The accepted frontier is
still host/testable: route-aware pending ICMP can ARP for same-subnet
destinations or configured gateways, retain final destination and next-hop
identity separately, and transmit only after matching next-hop ARP resolution
through fake NetworkDevice evidence.

The closeout selects the queued explicit single-pending ARP retry core as the
next bounded Phase 12.3 host-only task. It does not accept retry behavior
itself. Packet queues, autonomous timers, live driver adapters, smoltcp
adoption, UDP/TCP, sockets, hardware packet I/O, ping/network reachability
behavior, SSH, Pi 5 hardware work, boot publication, lab mutation, and phase
transition remain rejected.

phase12-network-single-pending-arp-retry-core-20260619 accepts
phase12-network-single-pending-arp-retry-core-accepted. src/network.rs now
includes explicit caller-driven ARP retry state for the existing single-pending
ICMP request boundary.

The accepted host-only retry path stores a deterministic retry budget in the
pending request. A caller may explicitly ask to retry ARP for the stored
next-hop IPv4; this covers gateway-routed pending requests because the final
IPv4 destination and ARP next hop remain separate. Successful fake-device ARP
retry transmit decrements the stored budget and leaves the pending ICMP request
available for later matching ARP resolution. Budget exhaustion, output-buffer
pressure, and transmit errors leave the pending request stored with documented
state, and no-pending returns a deterministic no-pending result.

This remains source/test evidence only over caller-owned buffers and fake
NetworkDevice transmit. It does not accept packet queues, autonomous timers,
live driver adapters, smoltcp adoption, UDP/TCP, sockets, hardware packet I/O,
ping/network reachability behavior, SSH, Pi 5 hardware work, boot publication,
lab mutation, or phase transition.

phase12-network-single-pending-arp-retry-closeout-20260619 accepts
phase12-network-single-pending-arp-retry-closeout-accepted. The closeout
reconciles the explicit retry source, unit tests, task record, docs, and
evidence with the accepted route-aware pending frontier.

The accepted boundary remains host/testable and caller-driven. One stored
pending ICMP echo request may re-emit exactly one ARP request for its stored
next-hop IPv4 only when the caller invokes the retry helper and retry budget
remains. Successful fake-device ARP retry transmit decrements budget and keeps
the pending request for later matching ARP resolution. Budget exhaustion,
no-pending, output-buffer pressure, and transmit-error behavior are
deterministic and covered by unit tests.

The closeout selects
phase12-network-phase12-3-route-aware-outbound-frontier-closeout-20260619 as
the next same-milestone checkpoint. It does not accept packet queues,
autonomous timers, live driver adapters, smoltcp adoption, UDP/TCP, sockets,
hardware packet I/O, ping/network reachability behavior, SSH, Pi 5 hardware
work, boot publication, lab mutation, or phase transition.

phase12-network-phase12-3-route-aware-outbound-frontier-closeout-20260619
accepts
phase12-network-route-aware-outbound-frontier-closeout-accepted-planning-needed.
The checkpoint reconciles the accepted host/testable route-aware outbound
frontier: deterministic same-subnet versus gateway next-hop selection,
caller-buffered outbound ARP/ICMP request selection, one route-aware pending
ICMP echo request with separate final destination and ARP next-hop IPv4, and
explicit caller-driven ARP retry over that stored next hop.

No live networking behavior is accepted. Packet queues, autonomous retry
timers, multi-entry neighbor-discovery state, live driver adapters, smoltcp
adoption, UDP/TCP, sockets, hardware packet I/O, ping/network reachability
behavior, SSH, Pi 5 hardware work, boot publication, lab mutation, and phase
transition remain rejected. selected_next_task is null and planningNeeded=true
because no later queued task exists after this checkpoint with complete
objective dependencies, acceptance criteria, validation gates, docs, and
evidence requirements.

Supervisor planning later selected
phase12-network-pending-aware-arp-reply-poll-core-20260619 as the next bounded
host-only Phase 12.3 continuation.

phase12-network-pending-aware-arp-reply-poll-core-20260619 accepts
phase12-network-pending-aware-arp-reply-poll-core-accepted. src/network.rs now
includes PendingIcmpEchoPollResult and
poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request, a
one-step host-only NetworkDevice receive boundary for stored pending ICMP echo
requests waiting on ARP.

The accepted helper returns NoPendingRequest before receiving when no pending
request exists, maps no-frame and receive-error boundaries deterministically,
and delegates a received frame to the accepted ARP-reply learn/transmit path.
A matching ARP reply for the stored next-hop IPv4 learns that neighbor, emits
exactly one Ethernet/IPv4/ICMP echo request through NetworkDevice, and clears
pending only after successful ICMP transmit. Gateway-routed pending requests
learn the gateway next-hop while preserving the final IPv4 destination in the
emitted IPv4 packet.

Nonmatching ARP replies, malformed ARP frames, no-frame, no-pending,
receive-buffer pressure, receive errors, output-buffer pressure, and transmit
errors have deterministic outcomes and preserve pending state except on
successful ICMP transmit. Existing local ARP reply and inbound ICMP echo
reply-to-request behavior remains covered. Live packet I/O, driver adapters,
packet queues, autonomous polling/timers, sockets, shell ping, SSH, smoltcp
adoption, reachability, hardware work, lab mutation, boot publication, and
phase transition remain rejected. The selected next task is
phase12-network-pending-aware-arp-reply-poll-closeout-20260619.

phase12-network-pending-aware-arp-reply-poll-closeout-20260619 accepts
phase12-network-pending-aware-arp-reply-poll-closeout-accepted. The closeout
reconciles the accepted pending-aware ARP reply poll source, unit tests, task
record, docs, and commit.

The accepted boundary remains host/testable: one caller-driven NetworkDevice
receive can process a matching ARP reply for the stored route-aware pending
request, learn the next-hop neighbor, transmit exactly one ICMP echo request
through the trait, and clear pending only after successful ICMP transmit.
Gateway-routed pending requests still preserve final destination and next-hop
identity separately. No-frame, no-pending, receive-buffer pressure, receive
errors, nonmatching ARP, malformed ARP, output-buffer pressure, and transmit
errors are deterministic and preserve pending state except on successful ICMP
transmit.

The closeout selects
phase12-network-single-inflight-icmp-echo-reply-observation-core-20260619 as
the next bounded host-only task. It does not accept ICMP echo reply completion
tracking, shell ping, packet queues, autonomous polling/timers, live driver
adapters, sockets, SSH, smoltcp adoption, reachability, hardware work, lab
mutation, boot publication, or phase transition.

phase12-network-single-inflight-icmp-echo-reply-observation-core-20260619
accepts phase12-network-single-inflight-icmp-echo-reply-observation-core-accepted.
src/network.rs now includes SingleInflightIcmpEcho,
InflightIcmpEchoRequest, InflightIcmpEchoPollResult, and host-only helpers to
record one in-flight IPv4 ICMP echo request and observe a matching echo reply.

The accepted observation boundary is deterministic and allocation-free. A
reply completes the single in-flight request only when the inbound Ethernet
IPv4 ICMP echo reply is addressed to the local endpoint, has source IPv4 equal
to the stored destination, destination IPv4 equal to the local endpoint, valid
IPv4 and ICMP checksums, echo-reply type/code, matching identifier and
sequence number, and matching payload bytes. Nonmatching source, destination,
identifier, sequence, or payload preserves the in-flight record. No-inflight,
no-frame, receive-buffer pressure, receive errors, malformed/unsupported
frames, bad checksums, duplicate in-flight records, and payload-capacity
pressure have explicit host/unit outcomes.

This remains source/test evidence only over caller-owned receive storage and a
fake/trait-level NetworkDevice receive boundary. It does not wire reply
observation into outbound transmit helpers, shell ping, packet queues,
autonomous polling/timers, live driver adapters, sockets, SSH, smoltcp
adoption, reachability, hardware work, lab mutation, boot publication, or
phase transition.

phase12-network-single-inflight-icmp-echo-reply-observation-closeout-20260619
accepts phase12-network-single-inflight-icmp-echo-reply-observation-closeout-accepted.
The closeout reconciles the accepted core commit, source, unit tests, task
record, docs, and rejected claims. The Phase 12.3 host-only boundary now
includes deterministic completion of one recorded in-flight ICMP echo request
by one matching received echo reply over caller-owned receive storage and a
fake/trait-level NetworkDevice receive path. It remains explicitly not a live
ping path: outbound transmit integration, shell ping, packet queues,
autonomous polling/timers, live driver adapters, sockets, SSH, smoltcp
adoption, reachability, hardware work, lab mutation, boot publication, and
phase transition are still rejected. The selected next task is
phase12-network-host-ping-transaction-frontier-closeout-20260619.

phase12-network-host-ping-transaction-frontier-closeout-20260619 accepts
phase12-network-host-ping-transaction-frontier-closeout-accepted. The
checkpoint reconciles the accepted host-only ping-like transaction frontier:
route-aware outbound selection, ARP request emission and matching ARP reply
learning, trait-level ICMP echo request transmit after ARP resolution, and
single-inflight ICMP echo reply observation.

The accepted boundary is still source/unit-test/task evidence over
caller-owned buffers and fake/trait-level NetworkDevice behavior. It proves
the pieces needed for one host-only ping-like transaction, but not a single
integrated user-visible ping path. Automatic transmit-to-in-flight wiring,
timeout/retry scheduling, packet queues, live driver adapters, shell ping,
sockets, SSH, smoltcp adoption, reachability, hardware work, lab mutation,
boot publication, and phase transition remain rejected. The selected next task
is null and planningNeeded=true because no later queued task has complete
objective dependencies after this checkpoint.

phase12-network-integrated-single-ping-transaction-core-20260619 accepts
phase12-network-integrated-single-ping-transaction-core-accepted. The accepted
host-only boundary now includes one integrated single-ping transaction
coordinator over caller-owned buffers and fake/trait-level NetworkDevice
behavior. A caller can start one route-aware ICMP echo request; the coordinator
either transmits it immediately for a resolved next hop and records one
in-flight request after successful transmit, or emits one ARP request for an
unresolved next hop and retains one pending route-aware request. A later
caller-driven poll can consume a matching ARP reply, transmit exactly one ICMP
echo request, clear pending, and record in-flight only after successful
transmit. A matching echo reply completes and clears the in-flight transaction.

The accepted evidence is still host/unit-test evidence only. It does not
accept live packet I/O, live driver adapters, packet queues, autonomous retry
or timeout scheduling, shell ping, sockets, SSH, smoltcp adoption,
reachability, hardware work, lab mutation, boot publication, or phase
transition. The selected next task is
phase12-network-integrated-single-ping-transaction-closeout-20260619.

phase12-network-integrated-single-ping-transaction-closeout-20260619 accepts
phase12-network-integrated-single-ping-transaction-closeout-accepted. The
closeout reconciles the integrated single-ping transaction source, unit tests,
task record, docs, and commit evidence.

The accepted boundary remains host/testable: one caller-owned, fake/trait-level
NetworkDevice transaction can start a route-aware ICMP echo request, transmit
immediately for a resolved next hop, or emit one ARP request and retain one
pending route-aware request for an unresolved next hop. A caller-driven poll can
learn a matching ARP reply, transmit exactly one ICMP echo request, clear
pending, and record in-flight only after successful transmit. A matching echo
reply completes and clears the in-flight transaction.

The single pending and single in-flight state ownership is deterministic enough
to select the next caller-driven retry/timeout slice. This closeout still does
not accept live packet I/O, live driver adapters, packet queues, autonomous
retry or timeout scheduling, shell ping, sockets, SSH, smoltcp adoption,
reachability, hardware work, lab mutation, boot publication, or phase
transition. The selected next task is
phase12-network-single-ping-caller-driven-retry-timeout-core-20260619.

phase12-network-single-ping-caller-driven-retry-timeout-core-20260619 accepts
phase12-network-single-ping-caller-driven-retry-timeout-core-accepted.
src/network.rs now exposes caller-driven status, retry, and timeout controls
for the integrated host-only single-ping transaction.

The accepted boundary remains deterministic and allocation-free. A caller can
inspect whether the transaction is idle, pending ARP, or in-flight; start an
unresolved route with an explicit ARP retry budget; retry the stored route-aware
pending ARP request while preserving final destination and next-hop identity;
observe retry exhaustion or retry transmit errors without losing pending state;
and timeout exactly one pending or in-flight transaction. Timed-out and
completed transactions return to idle, and late frames after timeout are not
consumed.

This is host/unit-test evidence only over caller-owned buffers and
fake/trait-level NetworkDevice behavior. It does not accept autonomous timers,
scheduler wakeups, background polling, packet queues, multi-ping behavior,
dynamic routing, shell ping, sockets, UDP/TCP, SSH, smoltcp adoption, live
driver adapters, live packet I/O, reachability, hardware work, lab mutation,
boot publication, or phase transition. The selected next task is
phase12-network-single-ping-caller-driven-retry-timeout-closeout-20260619.

phase12-network-single-ping-caller-driven-retry-timeout-closeout-20260619
accepts
phase12-network-single-ping-caller-driven-retry-timeout-closeout-accepted. The
closeout reconciles the integrated host-only single-ping lifecycle plus
caller-driven status, retry, and timeout source/test/task evidence.

The accepted Phase 12.3 frontier now covers one deterministic fake/trait-level
transaction that can start a route-aware ICMP echo request, retain unresolved
pending ARP state, retry that pending ARP request while budget remains, advance
a matching ARP reply to one ICMP echo transmit, complete on a matching echo
reply, report retry exhaustion or retry transmit errors, expose idle/pending
ARP/in-flight status, and explicitly timeout one pending or in-flight
transaction.

This closeout still does not accept live packet I/O, live driver adapters,
autonomous timers, scheduler wakeups, packet queues, multi-ping behavior,
dynamic routing, shell ping, sockets, UDP/TCP, SSH, smoltcp adoption,
reachability, hardware work, lab mutation, boot publication, or phase
transition. selected_next_task is null and planningNeeded=true because no later
queued task has complete objective dependencies after this checkpoint.

phase12-network-single-ping-transaction-qemu-smoke-core-20260619 accepts
phase12-network-single-ping-transaction-qemu-smoke-core-accepted.
src/network.rs now includes a named QEMU/substitute single-ping transaction
smoke test, and scripts/qemu-single-ping-transaction-smoke.sh retains the
host substitute transcript under
tasks/evidence/2026-06-19-qemu-single-ping-transaction-smoke/.

The accepted evidence frontier now includes a durable transcript for the
host-only single-ping transaction lifecycle: unresolved ARP starts pending,
a matching ARP reply advances the transaction to one ICMP echo transmit and
in-flight record, a matching echo reply completes the transaction, and status
returns to idle. The same smoke also covers caller-driven ARP retry budget
exhaustion followed by an explicit pending timeout. This remains
QEMU/substitute and unit-test evidence over caller-owned buffers and
fake/trait-level NetworkDevice behavior only; it does not accept shell ping,
sockets, UDP/TCP, smoltcp, live driver adapters, live packet I/O, hardware,
reachability, autonomous timers, packet queues, lab mutation, boot
publication, SSH, or phase transition.

phase12-network-single-ping-transaction-qemu-smoke-closeout-20260619 accepts
phase12-network-single-ping-transaction-qemu-smoke-closeout-accepted. The
closeout reconciles the retained QEMU/substitute smoke transcript with the
accepted source/unit evidence for SinglePingTransaction, explicit
caller-driven retry, and explicit timeout.

The accepted evidence level remains host-only QEMU/substitute plus source/unit
tests over caller-owned buffers and fake/trait-level NetworkDevice behavior.
It is sufficient to plan the next user-boundary strategy checkpoint, but it
does not accept shell ping, sockets, UDP/TCP, smoltcp, live driver adapters,
live packet I/O, hardware, reachability, autonomous timers, packet queues, lab
mutation, boot publication, SSH, or phase transition. The selected next task is
phase12-network-host-ping-user-boundary-strategy-checkpoint-20260619.

phase12-network-host-ping-user-boundary-strategy-checkpoint-20260619 accepts
phase12-network-host-ping-user-boundary-strategy-checkpoint-accepted-planning-needed.
The checkpoint separates the accepted host-only single-ping evidence from a
user-visible ping feature boundary. A fake or kernel-backed shell ping command
is not feature progress unless it is backed by accepted userspace, descriptor,
socket, and network-stack layers; existing command surfaces remain only
regression/control surfaces.

The next useful feature direction is a caller-driven single-transaction packet
pump/service boundary over the accepted SinglePingTransaction and NetworkDevice
contracts. That future boundary should own one transaction, consume received
frames, produce transmit attempts through NetworkDevice, expose status, and
accept explicit retry/timeout advancement without autonomous timers. No such
follow-up task is already queued with complete objective dependencies,
acceptance criteria, validation gates, docs, and evidence requirements, so
selected_next_task is null and planningNeeded=true. Shell ping, sockets,
UDP/TCP, smoltcp, live driver adapters, live packet I/O, hardware reachability,
autonomous timers, broad packet queues, lab mutation, boot publication, SSH,
Phase 12.1 link-hardware retry, and phase transition remain rejected.

phase12-network-single-transaction-packet-service-core-20260619 accepts
phase12-network-single-transaction-packet-service-core-accepted.
src/network.rs now exposes SinglePingPacketService, a caller-driven host-only
service/pump that owns exactly one SinglePingTransaction plus a bounded ARP
cache while borrowing NetworkDevice and caller-owned receive/transmit buffers
per operation.

The accepted service API can start one route-aware ping-like transaction,
retain unresolved pending ARP state, process a matching ARP reply into exactly
one ICMP echo transmit and in-flight record, process a matching echo reply into
completion, report idle/pending/in-flight status, retry pending ARP requests,
and explicitly timeout pending or in-flight state. Unit evidence covers
no-frame, malformed/unsupported frame, nonmatching ARP/reply,
receive-buffer pressure, receive errors, transmit errors, retry exhaustion,
duplicate/active start, and late frames after timeout without losing state
except on accepted completion or explicit timeout.

This remains host-only source/unit-test and QEMU/substitute evidence over
caller-owned buffers and fake/trait-level NetworkDevice behavior. It does not
accept shell ping, sockets, UDP/TCP, smoltcp, live driver adapters, live packet
I/O, hardware reachability, autonomous timers, broad packet queues, lab
mutation, boot publication, SSH, Phase 12.1 link-hardware retry, or phase
transition. The selected next task is
phase12-network-single-transaction-packet-service-closeout-20260619.

phase12-network-single-transaction-packet-service-closeout-20260619 accepts
phase12-network-single-transaction-packet-service-closeout-accepted.
The closeout reconciles the accepted SinglePingPacketService core,
source/unit evidence, retained QEMU/substitute smoke evidence, task record,
docs, and commit e673b08c0e8c8c3d8b25a9de4bf70ee22c40d81e.

The accepted frontier remains host-only: one caller-driven packet service/pump
owns exactly one SinglePingTransaction plus a bounded ARP cache while borrowing
NetworkDevice and caller-owned receive/transmit buffers per operation. The
evidence does not accept shell ping, sockets, UDP/TCP, smoltcp, live driver
adapters, live packet I/O, hardware reachability, autonomous timers, broad
packet queues, lab mutation, boot publication, SSH, Phase 12.1 link-hardware
retry, or phase transition. selected_next_task is null and planningNeeded=true
because no later queued Phase 12.3 task exists with complete objective
dependencies, acceptance criteria, validation gates, docs, and evidence
requirements.

phase12-network-phase12-3-host-only-stack-frontier-checkpoint-20260619 accepts
phase12-network-phase12-3-host-only-stack-frontier-checkpoint-accepted.
The checkpoint reconciles the accepted Phase 12.3 host-only stack frontier:
local packet dispatch, ARP cache, outbound frame construction, route-aware
single-ping transaction, caller-driven retry/timeout, retained
QEMU/substitute smoke evidence, and SinglePingPacketService. The accepted
boundary is one caller-driven service/pump over caller-owned receive/transmit
buffers and fake/trait-level NetworkDevice behavior. It can start, pump,
observe status, retry, and timeout one transaction, but it is not a live driver
adapter, socket API, or user-visible command.

Lab-network ping remains unaccepted because Phase 12.1 live link/packet
hardware remains paused, and no live driver adapter or live packet I/O has
been accepted. Shell ping, sockets, UDP/TCP, smoltcp, live driver adapters,
live packet I/O, hardware reachability, autonomous timers, broad packet
queues, lab mutation, boot publication, SSH, Phase 12.1 link-hardware retry,
and phase transition remain rejected. The selected next task is
phase12-network-userspace-ping-operation-contract-core-20260619, a bounded
host-only userspace/descriptor-facing operation contract over the accepted
SinglePingPacketService and NetworkDevice abstractions.

phase12-network-userspace-ping-operation-contract-core-20260619 accepts
phase12-network-userspace-ping-operation-contract-core-accepted.
src/network.rs now exposes UserspacePingOperation, a host-only local
userspace/descriptor-facing operation contract over the accepted
SinglePingPacketService. The operation can start one ping-like transaction,
pump caller-owned receive/transmit buffers through fake/trait-level
NetworkDevice behavior, expose idle/pending/in-flight/completed/timed-out
status, retry pending ARP explicitly, timeout pending or in-flight state, and
map boundary errors into the accepted POSIX error vocabulary.

Unit evidence covers unresolved ARP through echo-reply completion, terminal
status observation, duplicate/active start, caller-driven retry, retry
exhaustion, explicit timeout, and receive/transmit IO error mapping. The
accepted boundary remains host-only and does not bind to an actual descriptor
object, syscall ABI, shell command, public socket API, live driver adapter,
live packet I/O, hardware reachability, UDP/TCP, smoltcp, SSH, autonomous
timers, broad queues, lab mutation, boot publication, Phase 12.1
link-hardware retry, or phase transition. The selected next task is
phase12-network-userspace-ping-operation-contract-closeout-20260619.

phase12-network-userspace-ping-operation-contract-closeout-20260619 accepts
phase12-network-userspace-ping-operation-contract-closeout-accepted. The
closeout reconciles the accepted UserspacePingOperation core, source/unit
evidence, task record, and docs. The exact accepted boundary is a host-only
local operation over SinglePingPacketService, caller-owned buffers, and
fake/trait-level NetworkDevice behavior. It exposes start, pump, status,
retry, timeout, duplicate/active busy, retry exhaustion, and receive/transmit
error mapping outcomes through the accepted POSIX error vocabulary.

The boundary remains unbound to a real descriptor object or syscall ABI
because no accepted network descriptor/syscall contract exists yet. Shell ping,
public sockets, live driver adapters, live packet I/O, hardware reachability,
SSH, smoltcp, UDP/TCP, autonomous timers, broad queues, lab mutation, boot
publication, Phase 12.1 link-hardware retry, and phase transition remain
rejected. The selected next task is
phase12-network-userspace-ping-operation-substitute-smoke-core-20260619, which
must retain host-only substitute evidence before later user-visible or live I/O
work is considered.

phase12-network-userspace-ping-operation-substitute-smoke-core-20260619 accepts
phase12-network-userspace-ping-operation-substitute-smoke-core-accepted.
scripts/qemu-userspace-ping-operation-smoke.sh now retains a host/QEMU
substitute transcript under
tasks/evidence/2026-06-19-userspace-ping-operation-substitute-smoke/. The
smoke runs the userspace_ping_operation host tests and records the accepted
UserspacePingOperation lifecycle: unresolved ARP starts pending, matching ARP
advances to ICMP transmit/in-flight tracking, matching echo reply completes
the operation, terminal status remains observable, caller-driven retry
exhaustion and explicit timeout are covered, and busy/receive/transmit IO
error edges remain mapped at the operation boundary.

The accepted evidence remains host-only over SinglePingPacketService,
fake/trait-level NetworkDevice behavior, and caller-owned receive/transmit
buffers. It does not accept shell ping, public sockets, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, autonomous
timers, broad queues, lab mutation, boot publication, Phase 12.1
link-hardware retry, or phase transition. The selected next task is
phase12-network-userspace-ping-operation-substitute-smoke-closeout-20260619.

phase12-network-userspace-ping-operation-substitute-smoke-closeout-20260619
accepts phase12-network-userspace-ping-operation-substitute-smoke-closeout-accepted.
The closeout reconciles the retained substitute smoke transcript with the
accepted UserspacePingOperation contract, SinglePingPacketService packet
service, fake/trait-level NetworkDevice behavior, and caller-owned buffers.
The exact accepted evidence level remains host-only: one operation can drive
the unresolved-ARP to echo-reply lifecycle and demonstrate status, retry
exhaustion, explicit timeout, duplicate/active busy, and receive/transmit IO
error mapping through retained QEMU/substitute evidence.

The boundary remains unbound to a real descriptor object or syscall ABI.
Shell ping, public sockets, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, autonomous timers, broad queues, lab
mutation, boot publication, Phase 12.1 link-hardware retry, and phase
transition remain rejected. selected_next_task is null and planningNeeded=true
because no later queued Phase 12.3 task has complete objective dependencies,
acceptance criteria, validation gates, docs, and evidence requirements.

phase12-network-ping-operation-descriptor-contract-core-20260620 accepts
phase12-network-ping-operation-descriptor-contract-core-accepted.
src/network.rs now exposes NetworkPingOperationDescriptor and
NetworkPingOperationDescriptorTable, a host-only fd-like identity layer around
the accepted UserspacePingOperation. The table can open one descriptor-shaped
ping operation, drive start, pump, retry, timeout, status, and close through
that descriptor identity, and remove closed descriptors deterministically.

The descriptor contract delegates protocol behavior to UserspacePingOperation,
SinglePingPacketService, and fake/trait-level NetworkDevice behavior rather
than duplicating ARP, IPv4, ICMP, route, retry, timeout, or device logic. Unit
evidence covers unresolved ARP through echo-reply completion, invalid and
closed descriptor lookup as EBADF, zero-capacity open as EMFILE, duplicate
active operation open as EBUSY, retry exhaustion as EAGAIN, explicit timeout
with terminal timed-out status, and receive/transmit IO error mapping.

The accepted boundary remains host-only source/unit-test and QEMU/substitute
evidence over caller-owned buffers and fake/trait-level NetworkDevice
behavior. It does not accept shell ping, public sockets, syscall ABI, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, autonomous timers, broad queues, lab mutation, boot publication,
Phase 12.1 link-hardware retry, or phase transition. The selected next task is
phase12-network-ping-operation-descriptor-contract-closeout-20260620.

phase12-network-ping-operation-descriptor-contract-closeout-20260620 accepts
phase12-network-ping-operation-descriptor-contract-closeout-accepted. The
closeout reconciles the descriptor-shaped operation contract core, source/unit
evidence, task record, docs, and commit
e91f7ee2b8a576eaaa620afd5193dabe1839808c.

The accepted evidence level remains host-only over NetworkPingOperationDescriptor,
NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned buffers. The descriptor layer owns fd-like identity and lifecycle
only: open, start, pump, retry, timeout, status, and close. Shell ping, public
sockets, syscall ABI acceptance, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, and phase transition remain rejected. The
selected next task is
phase12-network-ping-operation-descriptor-substitute-smoke-core-20260620.

phase12-network-ping-operation-descriptor-substitute-smoke-core-20260620
accepts phase12-network-ping-operation-descriptor-substitute-smoke-core-accepted.
scripts/qemu-ping-operation-descriptor-smoke.sh now retains a host/QEMU
substitute transcript under
tasks/evidence/2026-06-20-ping-operation-descriptor-substitute-smoke/. The
smoke runs the network_ping_descriptor target test filter and records the
accepted descriptor-shaped lifecycle: descriptor open, unresolved ARP pending,
matching ARP advancement to ICMP transmit and in-flight tracking, matching
echo reply completion, terminal status observation, and descriptor close.

The retained evidence also covers caller-driven retry exhaustion, explicit
timeout, invalid and closed descriptors, zero-capacity open, duplicate active
open, transmit IO errors, and receive IO errors through
NetworkPingOperationDescriptorTable over UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned buffers. The accepted boundary remains host-only and does not
accept shell ping, public sockets, syscall ABI acceptance, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, or phase
transition. The selected next task is
phase12-network-ping-operation-descriptor-substitute-smoke-closeout-20260620.

phase12-network-ping-operation-descriptor-substitute-smoke-closeout-20260620
accepts phase12-network-ping-operation-descriptor-substitute-smoke-closeout-accepted.
The closeout reconciles the retained descriptor substitute smoke transcript
with the accepted descriptor-shaped operation contract, source/tests, task
evidence, and docs. The exact accepted evidence level remains host-only over
NetworkPingOperationDescriptorTable, NetworkPingOperationDescriptor,
UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, and caller-owned buffers.

The transcript covers descriptor open/start/pump/status/retry/timeout/close,
unresolved ARP pending, matching ARP advancement to ICMP transmit and
in-flight tracking, matching echo-reply completion, terminal status
observation, retry exhaustion, explicit timeout, invalid and closed
descriptors, zero-capacity open, duplicate active open, transmit IO error, and
receive IO error. Shell ping, public sockets, syscall ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, and
phase transition remain rejected. selected_next_task is null and
planningNeeded=true because no later queued Phase 12.3 task has complete
objective dependencies, acceptance criteria, validation gates, docs, and
evidence requirements.

phase12-network-ping-operation-syscall-substitute-contract-20260620 accepts
phase12-network-ping-operation-syscall-substitute-contract-accepted. The
selected host-only binding is a proof-only ping-operation syscall-substitute
adapter in src/syscall.rs, explicitly separate from stable SVC syscall
dispatch. It will borrow the accepted NetworkPingOperationDescriptorTable,
UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, and caller-owned receive/transmit buffers from its
caller rather than creating a public socket table, shell command, live driver
adapter, packet queue, or autonomous timer layer.

The selected operation vocabulary is open, start, pump, status, retry,
timeout, and close. The contract requires descriptor lifetime and terminal
status observation through the accepted descriptor table: invalid and closed
descriptors map to EBADF, zero descriptor capacity to EMFILE, duplicate active
open to EBUSY, retry exhaustion and nonmatching frames to EAGAIN, timeout to a
terminal timed-out status, and receive/transmit IO through the existing POSIX
device-error mappings. Shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and phase
transition remain rejected. The selected next task is
phase12-network-ping-operation-syscall-substitute-core-20260620.

phase12-network-ping-operation-syscall-substitute-core-20260620 accepts
phase12-network-ping-operation-syscall-substitute-core-accepted. src/syscall.rs
now provides PingOperationSyscallSubstitute, a host-only proof adapter that
borrows NetworkPingOperationDescriptorTable plus caller-owned receive and
transmit buffers and drives open/start/pump/status/retry_arp/timeout/close
through the accepted descriptor table. It also exposes scalar-shaped
PingOperationSyscallSubstituteStatus and PingOperationSyscallSubstituteStep
records so tests can observe state, frame length, payload length, retry count,
destination, and timeout destination without accepting a stable syscall ABI.

Unit and QEMU-substitute evidence cover unresolved ARP through echo-reply
completion, terminal completed status, invalid and closed descriptors,
zero-capacity open, duplicate active open, retry exhaustion, explicit timeout
with terminal timed-out status, transmit IO error, receive IO error, and
pump-time transmit IO error through the adapter. Shell ping, public sockets,
stable syscall ABI acceptance, socket syscall ABI, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and
phase transition remain rejected. The selected next task is
phase12-network-ping-operation-syscall-substitute-closeout-20260620.

phase12-network-ping-operation-syscall-substitute-closeout-20260620 accepts
phase12-network-ping-operation-syscall-substitute-closeout-accepted. The
closeout reconciles the adapter implementation, tests, task record, docs, and
rejected claims as host-only source/unit/QEMU-substitute evidence over
PingOperationSyscallSubstitute, NetworkPingOperationDescriptorTable,
UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, and caller-owned receive/transmit/status buffers.

The accepted boundary remains proof-only and explicitly separate from stable
SVC syscall dispatch. Shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and phase
transition remain rejected. The selected next task is
phase12-network-ping-operation-syscall-substitute-smoke-core-20260620.

phase12-network-ping-operation-syscall-substitute-smoke-core-20260620 accepts
phase12-network-ping-operation-syscall-substitute-smoke-core-accepted.
scripts/qemu-ping-operation-syscall-substitute-smoke.sh retains a
task-owned QEMU/substitute transcript for the accepted proof-only adapter. The
smoke exercises PingOperationSyscallSubstitute over
NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned receive/transmit/status buffers.

The retained evidence covers adapter open/start/pump/status/retry_arp,
timeout, and close, unresolved ARP pending, matching ARP advancement to ICMP transmit and
in-flight status, matching echo-reply completion, terminal status observation,
retry exhaustion, explicit timeout, invalid and closed descriptors,
zero-capacity open, duplicate active open, start-time transmit IO error,
receive IO error, and pump-time transmit IO error. Shell ping, public sockets,
stable syscall ABI acceptance, socket syscall ABI, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and
phase transition remain rejected. The selected next task is
phase12-network-ping-operation-syscall-substitute-smoke-closeout-20260620.

phase12-network-ping-operation-syscall-substitute-smoke-closeout-20260620
accepts
phase12-network-ping-operation-syscall-substitute-smoke-closeout-accepted. The
closeout reconciles the retained smoke transcript, full-suite transcript,
adapter source/tests, task record, docs, and rejected claims. The accepted
evidence level remains host-only QEMU/substitute smoke over the proof-only
PingOperationSyscallSubstitute adapter, NetworkPingOperationDescriptorTable,
UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, and caller-owned receive/transmit/status buffers.

The closeout accepts only the retained fake-device adapter lifecycle evidence:
open/start/pump/status/retry_arp/timeout/close, unresolved ARP pending,
ARP-to-ICMP advancement, echo-reply completion, terminal status observation,
retry exhaustion, explicit timeout, invalid and closed descriptors, capacity,
busy, and IO-error mapping. Shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and phase
transition remain rejected. selected_next_task is null and
planningNeeded=true because no later queued Phase 12.3 or Phase 12.4 task has
complete objective dependencies, acceptance criteria, validation gates, and
evidence requirements.

phase12-network-runtime-device-pump-core-20260620 accepts
phase12-network-runtime-device-pump-core-accepted. src/network.rs now provides
NetworkRuntimeDevicePump, a host-only caller-driven runtime/service boundary
over NetworkDevice. The pump owns fixed-capacity local ARP and ping operation
state, receives exactly one frame into caller-owned storage per pump call,
gives local ARP/ICMP response generation first chance, and then offers
non-reply traffic to one selected active NetworkPingOperationDescriptor.

The accepted boundary preserves local ARP reply and local IPv4 ICMP echo reply
behavior while allowing one ping operation descriptor to advance through ARP
resolution, ICMP transmit, echo-reply completion, retry, explicit timeout, and
terminal status observation. Unit/QEMU-substitute evidence covers no-frame,
nonlocal/no-reply, receive-buffer pressure, receive error, local and active
transmit errors, and deterministic ordering when inbound responder and active
operation work are both possible. Shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, autonomous timers, broad packet
queues, lab mutation, boot publication, Phase 12.1 link-hardware retry, Phase
12.4 socket expansion, and phase transition remain rejected. The selected next
task is phase12-network-runtime-device-pump-closeout-20260620.

phase12-network-runtime-device-pump-closeout-20260620 accepts
phase12-network-runtime-device-pump-closeout-accepted. The closeout reconciles
the accepted runtime pump implementation, source/unit/QEMU-substitute
validation, task record, docs, durable state, and rejected claims. The accepted
boundary remains host-only over NetworkDevice/fake-device behavior,
caller-owned buffers, fixed-capacity state, local ARP/ICMP dispatch, and the
accepted ping operation stack.

The closeout accepts no live driver adapter, live packet I/O, public sockets,
stable syscall ABI, shell ping, hardware reachability, SSH, smoltcp, UDP/TCP,
lab mutation, boot publication, Phase 12.1 link-hardware retry, Phase 12.4
socket expansion, or phase transition. The selected next task is
phase12-network-runtime-device-pump-substitute-smoke-core-20260620.

phase12-network-runtime-device-pump-substitute-smoke-core-20260620 accepts
phase12-network-runtime-device-pump-substitute-smoke-core-accepted. The
retained host/QEMU-substitute smoke command is
scripts/qemu-network-runtime-device-pump-smoke.sh, with transcript evidence
under tasks/evidence/2026-06-20-network-runtime-device-pump-substitute-smoke/.
The smoke exercises the accepted NetworkRuntimeDevicePump boundary over fake
NetworkDevice behavior, local ARP/ICMP replies, one active ping descriptor
lifecycle, caller-owned buffers, and fixed-capacity state.

The accepted smoke evidence covers local ARP reply transmit, local ICMP echo
reply transmit, local responder priority over active ping, unresolved ARP to
ICMP transmit advancement, echo-reply completion, terminal completed status,
retry exhaustion, explicit timeout, no-frame, receive-buffer pressure, receive
IO error, local transmit IO error, and active transmit IO error. Shell ping,
public sockets, stable syscall ABI acceptance, socket syscall ABI, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, Phase 12.4 socket
expansion, and phase transition remain rejected. The selected next task is
phase12-network-runtime-device-pump-substitute-smoke-closeout-20260620.

phase12-network-runtime-device-pump-substitute-smoke-closeout-20260620 accepts
phase12-network-runtime-device-pump-substitute-smoke-closeout-accepted. The
closeout reconciles the retained smoke transcript, full-suite transcript,
runtime pump source/tests, task record, docs, durable state, and rejected
claims. The accepted evidence level remains host/QEMU-substitute smoke over
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior,
NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned receive/transmit buffers, and fixed-capacity state.

The closeout accepts only the retained fake-device runtime pump evidence:
local ARP reply transmit, local ICMP echo reply transmit, local responder
priority, unresolved ARP to ICMP transmit advancement, echo-reply completion,
terminal completed status, retry exhaustion, explicit timeout, no-frame,
receive-buffer pressure, receive IO error, local transmit IO error, and active
transmit IO error. Shell ping, public sockets, stable syscall ABI acceptance,
socket syscall ABI, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
12.1 link-hardware retry, Phase 12.4 socket expansion, and phase transition
remain rejected. selected_next_task is null and planningNeeded=true because no
later queued Phase 12.3 or Phase 12.4 task has complete objective
dependencies, acceptance criteria, validation gates, and evidence
requirements.

phase12-network-runtime-ping-syscall-substitute-core-20260620 accepts
phase12-network-runtime-ping-syscall-substitute-core-accepted. src/syscall.rs
now provides RuntimePingOperationSyscallSubstitute, a host-only proof/control
adapter that borrows NetworkRuntimeDevicePump plus caller-owned receive and
transmit buffers. The adapter routes open/start/status/retry_arp/timeout/close
through the runtime pump and exposes one pump step that preserves local
ARP/ICMP responder priority before active ping descriptor dispatch.

The accepted boundary preserves the earlier PingOperationSyscallSubstitute
status/step vocabulary for active ping work and adds explicit local-pump
outcomes for no-frame, local no-reply, local ARP/ICMP reply, and active ping
progress. Unit/QEMU-substitute evidence covers open/start/status, unresolved
ARP to inflight through NetworkRuntimeDevicePump, echo-reply completion through
active-ping dispatch, local ARP and ICMP reply dispatch while a descriptor is
open, close and bad descriptors, zero-capacity and duplicate-open behavior,
retry exhaustion, explicit timeout, receive IO error, local transmit IO error,
and active-ping transmit IO error. Shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and phase
transition remain rejected. The selected next task is
phase12-network-runtime-ping-syscall-substitute-closeout-20260620.

phase12-network-runtime-ping-syscall-substitute-closeout-20260620 accepts
phase12-network-runtime-ping-syscall-substitute-closeout-accepted. The closeout
reconciles the accepted RuntimePingOperationSyscallSubstitute implementation,
source/unit/QEMU-substitute validation, task record, docs, durable state, and
rejected claims. The accepted evidence level remains host-only over
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
fake/trait-level NetworkDevice behavior, caller-owned buffers, and
fixed-capacity state.

The closeout accepts no shell ping command, public sockets, stable syscall ABI,
socket syscall ABI, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
link-hardware retry, Phase 12.4 socket expansion, or phase transition. The
selected next task is
phase12-network-runtime-ping-syscall-substitute-smoke-core-20260620.

phase12-network-runtime-ping-syscall-substitute-smoke-core-20260620 accepts
phase12-network-runtime-ping-syscall-substitute-smoke-core-accepted. The
retained host/QEMU-substitute smoke command is
scripts/qemu-runtime-ping-syscall-substitute-smoke.sh, with transcript evidence
under tasks/evidence/2026-06-20-runtime-ping-syscall-substitute-smoke/. The
smoke exercises the accepted RuntimePingOperationSyscallSubstitute boundary
over NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
fake/trait-level NetworkDevice behavior, caller-owned buffers, and
fixed-capacity state.

The accepted smoke evidence covers open/start/status, unresolved ARP,
runtime-pump ARP advancement to ICMP transmit, inflight status, runtime-pump
echo-reply completion, terminal completed status, close, local ARP and ICMP
reply dispatch while a descriptor is open, retry exhaustion, explicit timeout,
invalid descriptor, closed descriptor, zero-capacity, busy-open, receive IO
error, local transmit IO error, and active-ping transmit IO error. Shell ping,
public sockets, stable syscall ABI acceptance, socket syscall ABI, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, Phase 12.4 socket
expansion, and phase transition remain rejected. The selected next task is
phase12-network-runtime-ping-syscall-substitute-smoke-closeout-20260620.

phase12-network-runtime-ping-syscall-substitute-smoke-closeout-20260620 accepts
phase12-network-runtime-ping-syscall-substitute-smoke-closeout-accepted. The
closeout reconciles the retained smoke transcript, smoke script, accepted
RuntimePingOperationSyscallSubstitute source boundary, task record, docs,
durable state, and rejected claims. The accepted evidence level remains
host/QEMU-substitute only over RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
fake/trait-level NetworkDevice behavior, caller-owned buffers, and
fixed-capacity state.

The closeout accepts only the retained fake-device runtime ping syscall
substitute smoke evidence: open/start/status, unresolved ARP, runtime-pump ARP
advancement to ICMP transmit, inflight status, runtime-pump echo-reply
completion, terminal completed status, close, local ARP and ICMP reply dispatch
while a descriptor is open, retry exhaustion, explicit timeout, invalid
descriptor, closed descriptor, zero-capacity, busy-open, receive IO error,
local transmit IO error, and active-ping transmit IO error. Shell ping, public
sockets, stable syscall ABI acceptance, socket syscall ABI, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, Phase 12.4 socket
expansion, and phase transition remain rejected. selected_next_task is null and
planningNeeded=true because no later queued Phase 12.3 or Phase 12.4 task has
complete objective dependencies, acceptance criteria, validation gates, and
evidence requirements.

phase12-network-host-ping-frontier-checkpoint-20260620 accepts
phase12-network-host-ping-frontier-checkpoint-accepted. The checkpoint
reconciles the accepted Phase 12.3 host-only ping stack through
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local ARP/ICMP
responder behavior, active ping descriptor dispatch, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned buffers, fixed-capacity state, retained smoke transcript,
implementation commits, task records, docs, and durable-state planning.

The accepted evidence level remains host/QEMU-substitute only. It covers the
runtime-pump-backed syscall substitute/control path, local responder priority,
active ping descriptor dispatch, caller-driven retry/timeout/status handling,
descriptor capacity and lifecycle errors, and receive/local-transmit/active
transmit IO errors. Shell ping, public sockets, stable syscall ABI acceptance,
socket syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
link-hardware retry, Phase 12.4 socket expansion, and phase transition remain
rejected. selected_next_task is
phase12-network-descriptor-shaped-ping-control-contract-20260620.

phase12-network-descriptor-shaped-ping-control-contract-20260620 accepts
phase12-network-descriptor-shaped-ping-control-contract-accepted. The selected
Phase 12.4 contract is a narrow crate-internal, host-only descriptor-shaped
control layer over the accepted RuntimePingOperationSyscallSubstitute and
NetworkRuntimeDevicePump. It keeps caller-owned receive/transmit/status
buffers, fixed-capacity state, and one-operation scope while exposing open,
start, pump/read-result, status, retry_arp, timeout, and close.

The contract maps local responder and active ping behavior to the accepted
runtime pump rather than duplicating ARP, IPv4, ICMP, route, retry, timeout, or
device logic. open/close own descriptor lifecycle; start begins one
route-aware ping-like operation; pump/read-result performs exactly one runtime
pump step with local ARP/ICMP responder priority before active ping progress;
status preserves terminal completed/timed-out observation in caller-owned
storage; retry_arp and timeout remain caller-driven. EBADF, EMFILE, EBUSY,
EAGAIN, ENOSPC, and existing internal packet/device error mappings remain the
selected error vocabulary.

The accepted evidence level is host-only contract evidence over
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local ARP/ICMP
responder behavior, active ping descriptor dispatch, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned buffers, fixed-capacity state, source/task/doc review, and durable
state. Shell ping, public sockets, stable syscall ABI acceptance, socket
syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
12.1 link-hardware retry, broad Phase 12.4 socket expansion, and phase
transition remain rejected. The selected next task is
phase12-network-descriptor-shaped-ping-control-core-20260620.

phase12-network-descriptor-shaped-ping-control-core-20260620 accepts
phase12-network-descriptor-shaped-ping-control-core-accepted. src/syscall.rs
now provides DescriptorShapedPingControl, a thin crate-internal host-only
control wrapper over RuntimePingOperationSyscallSubstitute. The wrapper exposes
the accepted descriptor-shaped lifecycle: open, start, status,
pump_or_read_result, retry_arp, timeout, and close while borrowing a
caller-provided NetworkRuntimeDevicePump plus caller-owned receive/transmit
buffers.

The accepted source/unit evidence covers one successful fake-device lifecycle:
open, idle status, start to pending ARP, runtime-pump ARP advancement to
inflight, runtime-pump echo-reply completion, terminal completed status, close,
and closed-descriptor EBADF. It also covers invalid descriptor, closed
descriptor, zero descriptor capacity, duplicate active open, retry exhaustion,
explicit timeout, caller receive-buffer pressure, receive IO error, local
transmit IO error, and active-ping transmit IO error behavior. Shell ping,
public sockets, stable syscall ABI acceptance, socket syscall ABI acceptance,
live driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, broad
Phase 12.4 socket expansion, and phase transition remain rejected. The selected
next task is phase12-network-descriptor-shaped-ping-control-smoke-20260620.

phase12-network-descriptor-shaped-ping-control-smoke-20260620 accepts
phase12-network-descriptor-shaped-ping-control-smoke-accepted. The retained
host/QEMU-substitute smoke command is
scripts/qemu-descriptor-shaped-ping-control-smoke.sh, with transcript evidence
under tasks/evidence/2026-06-20-descriptor-shaped-ping-control-smoke/. The
smoke exercises DescriptorShapedPingControl over
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local
ARP/ICMP responder behavior, active ping descriptor dispatch,
UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, caller-owned buffers, and fixed-capacity state.

The accepted smoke evidence covers open/start/status, unresolved ARP,
runtime-pump ARP advancement to inflight, runtime-pump echo-reply completion,
terminal completed status, close, invalid and closed descriptors, zero
descriptor capacity, duplicate active open, caller receive-buffer pressure,
retry exhaustion, explicit timeout, receive IO error, local transmit IO error,
and active-ping transmit IO error. Shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI acceptance, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket
expansion, and phase transition remain rejected. selected_next_task is null and
planningNeeded=true because no later queued Phase 12.4 task has complete
explicit acceptance gates.

phase12-network-descriptor-shaped-ping-control-closeout-20260620 accepts
phase12-network-descriptor-shaped-ping-control-closeout-accepted. The closeout
reconciles the accepted DescriptorShapedPingControl source/unit evidence,
task-owned smoke script, retained host/QEMU-substitute transcript, task records,
docs, and rejected claims. The accepted boundary remains a crate-internal
host-only control wrapper over RuntimePingOperationSyscallSubstitute and
NetworkRuntimeDevicePump with caller-owned receive/transmit/status storage and
fixed-capacity state.

The accepted evidence level remains host/QEMU-substitute over fake/trait-level
NetworkDevice behavior. It covers open/start/status, unresolved ARP,
runtime-pump ARP advancement to inflight, runtime-pump echo-reply completion,
terminal completed status, close, invalid and closed descriptors, zero
descriptor capacity, duplicate active open, caller receive-buffer pressure,
retry exhaustion, explicit timeout, receive IO error, local transmit IO error,
and active-ping transmit IO error. Shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI acceptance, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket
expansion, and phase transition remain rejected. selected_next_task is null and
planningNeeded=true pending supervisor planning for any next bounded task.

phase12-network-process-local-ping-descriptor-contract-20260620 accepts
phase12-network-process-local-ping-descriptor-contract-accepted. The selected
contract routes one accepted DescriptorShapedPingControl lifecycle through the
existing process-local descriptor ownership model: ProcessDescriptorStore owns
per-ProcessOwnerId DescriptorTable instances, the process-local descriptor is
the caller-visible handle, and the descriptor object reference indexes only a
crate-internal fixed-capacity ping-control description store.

The future core task may open, start, pump/read-result, observe status,
retry_arp, timeout, and close one ping-control operation through that
process-local handle while preserving caller-owned receive/transmit/status
storage, fixed-capacity state, explicit terminal status observation, and the
accepted EBADF/EMFILE/EBUSY/EAGAIN/ENOSPC/error vocabulary. ARP, IPv4, ICMP,
route policy, retry behavior, timeout behavior, local responder behavior, and
fake/trait-level NetworkDevice I/O remain delegated to DescriptorShapedPingControl,
RuntimePingOperationSyscallSubstitute, and NetworkRuntimeDevicePump. Public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, shell
ping, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad socket expansion, and phase transition remain rejected. The
selected next bounded task is
phase12-network-process-local-ping-descriptor-core-20260620.

phase12-network-process-local-ping-descriptor-core-20260620 accepts
phase12-network-process-local-ping-descriptor-core-accepted. src/syscall.rs now
provides ProcessLocalPingDescriptorControl, a crate-internal host-only wrapper
that routes one DescriptorShapedPingControl operation through the existing
ProcessDescriptorStore and per-ProcessOwnerId DescriptorTable model. The
process-local descriptor is the caller-visible handle; the descriptor object is
an OtherKernelObject whose reference indexes the backing ping-control descriptor.

The accepted source/unit evidence covers process-local open after inherited
stdio, idle status, start to pending ARP, runtime-pump ARP advancement to
inflight, echo-reply completion, terminal completed status, close, closed
descriptor EBADF, missing current owner EBADF, full process descriptor table
EMFILE with backing-descriptor unwind, duplicate active operation EBUSY,
wrong-kind stdio descriptor EBADF, retry exhaustion EAGAIN, explicit timeout,
receive IO error, and local transmit IO error. ARP, IPv4, ICMP, route policy,
retry behavior, timeout behavior, local responder behavior, and fake/trait-level
NetworkDevice I/O remain delegated to DescriptorShapedPingControl,
RuntimePingOperationSyscallSubstitute, and NetworkRuntimeDevicePump. Public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, shell
ping, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad socket expansion, and phase transition remain rejected. The
selected next bounded task is
phase12-network-process-local-ping-descriptor-closeout-20260620.

phase12-network-process-local-ping-descriptor-closeout-20260620 accepts
phase12-network-process-local-ping-descriptor-closeout-accepted. The closeout
reconciles the accepted process-local ping descriptor contract, core
implementation, source/unit evidence, task records, docs, durable state, and
rejected claims. The accepted boundary remains crate-internal and host-only:
ProcessLocalPingDescriptorControl maps a process-local DescriptorTable handle to
one backing DescriptorShapedPingControl operation through OtherKernelObject
references.

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over fake/trait-level NetworkDevice behavior, process-local descriptor
ownership, DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned receive/transmit/status storage, and
fixed-capacity state. It covers open/start/pump-or-read-result/status,
retry_arp, timeout, close, inherited-stdio descriptor allocation, invalid and
closed descriptors, missing current owner, process descriptor capacity unwind,
duplicate active operation, wrong-kind stdio descriptors, retry exhaustion,
explicit timeout, receive IO error, and local transmit IO error. Shell ping,
public sockets, stable syscall ABI acceptance, socket syscall ABI acceptance,
live driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, broad
socket expansion, and phase transition remain rejected. The selected next
bounded task is phase12-network-process-local-ping-descriptor-smoke-20260620.

phase12-network-process-local-ping-descriptor-smoke-20260620 accepts
phase12-network-process-local-ping-descriptor-smoke-accepted. The retained
host/QEMU-substitute transcript exercises ProcessLocalPingDescriptorControl
through ProcessDescriptorStore process-local ownership,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, fake/trait-level NetworkDevice behavior,
caller-owned buffers, and fixed-capacity state.

The smoke evidence covers open process descriptor, idle status, start to
unresolved-ARP pending, runtime-pump ARP advancement to inflight, runtime-pump
echo-reply completion, terminal completed status, close process descriptor,
missing owner, full process descriptor table with backing-descriptor unwind,
duplicate active open, wrong-kind stdio descriptor, closed descriptor, retry
exhaustion, explicit timeout, receive IO error, and local transmit IO error.
Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad Phase 12.4 socket expansion, and phase transition remain rejected.
The selected next bounded task is
phase12-network-process-local-ping-descriptor-smoke-closeout-20260620.

phase12-network-process-local-ping-descriptor-smoke-closeout-20260620 accepts
phase12-network-process-local-ping-descriptor-smoke-closeout-accepted. The
closeout reconciles the accepted process-local ping descriptor contract, core
implementation, retained smoke evidence, task records, docs, durable state, and
rejected claims.

The accepted evidence level remains host/QEMU-substitute smoke evidence over
fake/trait-level NetworkDevice behavior, process-local descriptor ownership,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, and fixed-capacity state. It
covers one accepted ping-control lifecycle through the process-local descriptor
path plus deterministic descriptor lifecycle and error controls: missing owner,
full process descriptor table with backing-descriptor unwind, duplicate active
open, wrong-kind stdio descriptor, closed descriptor, retry exhaustion, explicit
timeout, receive IO error, and local transmit IO error. Shell ping, public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP,
lab mutation, boot publication, Phase 12.1 link-hardware retry, broad Phase
12.4 socket expansion, and phase transition remain rejected. selected_next_task
is null and planningNeeded=true pending supervisor planning for any next
bounded task.

phase12-network-process-local-ping-svc-dispatch-contract-20260620 accepts
phase12-network-process-local-ping-svc-dispatch-contract-accepted. The contract
selects an unstable crate-internal, host-only dispatch facade as the next
implementation boundary for driving ProcessLocalPingDescriptorControl through
the existing process descriptor/syscall dispatch shape.

The future core task may route open, start, pump_or_read_result, status,
retry_arp, timeout, and close through explicit dispatch context containing the
current process owner, ProcessDescriptorStore, caller-owned buffers, and
NetworkRuntimeDevicePump. It must preserve the existing stable syscall vocabulary
of TalosNop, TalosWrite, TalosClose, TalosDup, TalosRead, and TalosOpen; no new
stable SyscallNumber variant, TALOS_*_SYSCALL constant, public socket API, or
stable userspace ABI is accepted by this contract. The accepted evidence level
is static host-only source/task/doc review over the dispatch_process_descriptor*
patterns, ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump,
fake/trait-level NetworkDevice behavior, caller-owned buffers, and
fixed-capacity state. Shell ping, public sockets, stable syscall ABI acceptance,
socket syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
link-hardware retry, broad socket expansion, and phase transition remain
rejected. The selected next bounded task is
phase12-network-process-local-ping-svc-dispatch-core-20260620.

phase12-network-process-local-ping-svc-dispatch-core-20260620 accepts
phase12-network-process-local-ping-svc-dispatch-core-accepted. The core
implementation adds an unstable crate-internal host-only dispatch facade for
driving ProcessLocalPingDescriptorControl through explicit process-dispatch
context without changing the stable SyscallNumber vocabulary or adding public
socket ABI.

The accepted facade routes open, start, pump_or_read_result, status, retry_arp,
timeout, and close through ProcessLocalPingDescriptorControl with the current
process owner, ProcessDescriptorStore, caller-owned receive/transmit buffers,
task-owned result/status slots, NetworkRuntimeDevicePump, and fake/trait-level
NetworkDevice context. The source/unit evidence covers one dispatch-shaped
lifecycle from open through unresolved ARP, ARP-to-ICMP advancement, echo-reply
completion, terminal completed status, and close. It also covers invalid and
closed descriptors, missing current owner, process descriptor capacity unwind,
duplicate active operation, retry exhaustion, explicit timeout with terminal
status, caller receive-buffer pressure, receive IO error, local transmit IO
error, and active transmit IO error. Shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-process-local-ping-svc-dispatch-closeout-20260620.

phase12-network-process-local-ping-svc-dispatch-closeout-20260620 accepts
phase12-network-process-local-ping-svc-dispatch-closeout-accepted. The closeout
reconciles the accepted contract, core implementation, source/unit evidence,
task records, docs, durable state, and rejected claims for the crate-internal
host-only dispatch facade.

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over fake/trait-level NetworkDevice behavior, process-local descriptor
ownership, internal dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state. It accepts only that the internal facade can drive one
process-local ping descriptor through open, start, pump/read-result, status,
retry_arp, timeout, and close with deterministic owner, descriptor lifetime,
capacity, busy, retry, timeout, receive-buffer pressure, and device-error
controls. Shell ping, public sockets, stable syscall ABI acceptance, socket
syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
link-hardware retry, broad socket expansion, and phase transition remain
rejected. The selected next bounded task is
phase12-network-process-local-ping-svc-dispatch-smoke-20260620.

phase12-network-process-local-ping-svc-dispatch-smoke-20260620 accepts
phase12-network-process-local-ping-svc-dispatch-smoke-accepted. The retained
host/QEMU-substitute transcript exercises
dispatch_process_local_ping_descriptor_operation through
ProcessLocalPingDescriptorControl, ProcessDescriptorStore,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, fake/trait-level NetworkDevice behavior,
caller-owned buffers, task-owned result/status slots, and fixed-capacity state.

The smoke evidence covers open, start, pump_or_read_result, status, retry_arp,
timeout, and close through the dispatch facade, including unresolved ARP,
ARP-to-ICMP advancement, echo-reply completion, terminal status observation,
invalid and closed descriptors, missing owner, process descriptor table capacity
unwind, duplicate active operation, retry exhaustion, explicit timeout,
receive-buffer pressure, receive IO error, local transmit IO error, and active
transmit IO error controls. Shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket expansion,
and phase transition remain rejected. The selected next bounded task is
phase12-network-process-local-ping-svc-dispatch-smoke-closeout-20260620.


phase12-network-process-local-ping-svc-dispatch-smoke-closeout-20260620 accepts
phase12-network-process-local-ping-svc-dispatch-smoke-closeout-accepted. The
closeout reconciles the accepted dispatch contract, core implementation,
retained smoke transcript, task records, docs, durable state, and rejected
claims for the crate-internal host-only process-local ping SVC dispatch facade.

The accepted evidence level remains host/QEMU-substitute smoke evidence over
fake/trait-level NetworkDevice behavior, process-local descriptor ownership,
internal dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state. It accepts only that the internal facade has retained
smoke coverage for one dispatch-shaped ping-control lifecycle plus deterministic
descriptor, owner, lifetime, capacity, retry, timeout, receive-buffer, and
device-error controls. Shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket expansion,
and phase transition remain rejected. selected_next_task is null and
planningNeeded=true pending supervisor planning for any next bounded Phase 12.4
task.

phase12-network-process-local-ping-svc-user-argument-contract-20260620 accepts
phase12-network-process-local-ping-svc-user-argument-contract-accepted. The
contract selects an unstable, crate-internal, host-only user-argument decoder as
the next smallest useful feature step after the accepted process-local ping SVC
dispatch smoke closeout at commit 3b55c149e86d3dbc0c84e286081d7b0d456cdb04.
The accepted predecessor evidence remains the host/QEMU-substitute dispatch
smoke over fake/trait-level NetworkDevice behavior, process-local descriptor
ownership, caller-owned buffers, task-owned result/status slots, and
fixed-capacity state.

The future core may decode only experimental operation selectors for open,
start, pump_or_read_result, status, retry_arp, timeout, and close. Scalar
arguments are responsible for the selector, process descriptor where required,
route policy, destination IPv4, identifier, sequence number, TTL, payload,
result, and status user addresses and lengths, and ARP retry budget.
User-memory handling must use the existing UserMapping plus
copy_from_user/copy_to_user style with bounded kernel scratch and caller-owned
buffers. Open returns the process descriptor as a scalar; pump_or_read_result
and status copy accepted task-owned result/status records to caller memory;
start, retry_arp, timeout, and close return scalar success after delegated
control. EBADF, EMFILE, EBUSY, EAGAIN, ENOSPC, EFAULT, and EINVAL retain their
accepted descriptor, capacity, busy, retry, storage, user-memory, and
malformed-argument roles, while device errors remain delegated through the
accepted runtime pump stack. The stable SyscallNumber vocabulary,
STABLE_SVC_IMMEDIATE, and TALOS_* syscall constants remain unchanged. Shell
ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad Phase 12.4 socket expansion, and phase transition remain rejected.
The selected next bounded task is
phase12-network-process-local-ping-svc-user-argument-core-20260620.

phase12-network-process-local-ping-svc-user-argument-core-20260620 accepts
phase12-network-process-local-ping-svc-user-argument-core-accepted. The core
implementation adds dispatch_process_local_ping_descriptor_user_arguments, a
crate-internal host-only decoder that maps experimental scalar/user-memory
inputs into the accepted process-local ping dispatch facade.

The accepted decoder routes open, start, pump_or_read_result, status, retry_arp,
timeout, and close through ProcessLocalPingDispatchOperation with explicit
current-owner, ProcessDescriptorStore, UserMapping copy-in/copy-out, bounded
kernel scratch, caller-owned result/status buffers, NetworkRuntimeDevicePump,
and fake/trait-level NetworkDevice context. Source/unit evidence covers a full
user-argument lifecycle from open through idle status copy-out, copied user
payload start, ARP-to-ICMP pump result copy-out, echo-reply completion,
completed status copy-out, and close. Negative evidence covers unchanged stable
SyscallNumber/TALOS_* vocabulary, malformed selector and reserved fields,
missing owner, process descriptor capacity, invalid descriptors, output-buffer
pressure, invalid user memory, scratch pressure, zero TTL, and invalid route
prefix. Shell ping, public sockets, stable syscall ABI acceptance, socket
syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
link-hardware retry, broad Phase 12.4 socket expansion, and phase transition
remain rejected. The selected next bounded task is
phase12-network-process-local-ping-svc-user-argument-closeout-20260620.

phase12-network-process-local-ping-svc-user-argument-closeout-20260620 accepts
phase12-network-process-local-ping-svc-user-argument-closeout-accepted. The
closeout reconciles the accepted contract, core implementation, source/unit
evidence, task records, durable state, and rejected claims for the unstable
crate-internal user-argument decoder.

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over fake/trait-level NetworkDevice behavior, experimental user-argument
decoding, UserMapping copy-in/copy-out, process-local descriptor ownership,
internal dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state. It accepts only that the host-only decoder can drive
one process-local ping dispatch lifecycle through experimental scalar/user
memory arguments, with deterministic malformed selector, reserved-field,
owner/descriptor, capacity, user-memory, buffer-pressure, scratch-pressure, TTL,
route-prefix, and stable syscall-vocabulary controls. Shell ping, public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP,
lab mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, and phase transition remain rejected. The selected next bounded task
is phase12-network-process-local-ping-svc-user-argument-smoke-20260620.

phase12-network-process-local-ping-svc-user-argument-smoke-20260620 accepts
phase12-network-process-local-ping-svc-user-argument-smoke-accepted. The
retained host/QEMU-substitute transcript exercises
dispatch_process_local_ping_descriptor_user_arguments through UserMapping,
ProcessLocalPingDispatchOperation, ProcessLocalPingDescriptorControl,
ProcessDescriptorStore, NetworkRuntimeDevicePump, fake/trait-level
NetworkDevice behavior, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state.

The smoke evidence covers one experimental user-argument lifecycle: open, idle
status copy-out, start from copied user payload, ARP-to-ICMP pump result
copy-out, echo-reply completion, completed status copy-out, and close. It also
covers selectors for open, start, pump_or_read_result, status, retry_arp,
timeout, and close; payload copy-in; result/status copy-out; bounded scratch;
malformed selector and reserved fields; missing owner; process descriptor
capacity; invalid descriptors; output-buffer pressure; invalid user memory;
scratch pressure; zero TTL; invalid route prefix; and unchanged stable
SyscallNumber/TALOS_* vocabulary. Shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket expansion,
and phase transition remain rejected. The selected next bounded task is
phase12-network-process-local-ping-svc-user-argument-smoke-closeout-20260620.

phase12-network-process-local-ping-svc-user-argument-smoke-closeout-20260620
accepts
phase12-network-process-local-ping-svc-user-argument-smoke-closeout-accepted.
The closeout reconciles the accepted contract, core implementation,
source/unit evidence, retained smoke transcript, task records, durable state,
and rejected claims for the unstable crate-internal user-argument decoder.

The accepted evidence level remains host/QEMU-substitute smoke evidence over
fake/trait-level NetworkDevice behavior, experimental user-argument decoding,
UserMapping copy-in/copy-out, process-local descriptor ownership, internal
dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state. It accepts only that the experimental user-argument
decoder has retained smoke coverage for one process-local ping dispatch
lifecycle plus deterministic selector, reserved-field, owner, descriptor,
capacity, user-memory, buffer-pressure, scratch-pressure, TTL, route-prefix,
and stable syscall-vocabulary controls. Shell ping, public sockets, stable
syscall ABI acceptance, socket syscall ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket
expansion, and phase transition remain rejected. selected_next_task is null
and planningNeeded=true pending supervisor planning for any next bounded Phase
12.4 task.

phase12-network-vfs-ping-diagnostic-svc-contract-20260620 accepts
phase12-network-vfs-ping-diagnostic-svc-contract-accepted. The contract selects
a VFS-backed userspace diagnostic fixture as the next smallest useful Phase
12.4 feature step after the accepted user-argument smoke closeout commit
a029de8844513dec66197bd4af17ee10f83679bf. The future core may add only a
task-owned, diagnostic-only VFS/initramfs executable-shaped fixture and
host/QEMU-substitute harness path that drives the accepted
dispatch_process_local_ping_descriptor_user_arguments bridge.

The accepted future operation sequence is open, status, start from
diagnostic-owned payload memory, pump_or_read_result through ARP-to-ICMP
progression, completed status copy-out, and close. User-memory handling must
use UserMapping plus copy_from_user/copy_to_user with bounded kernel scratch
and caller-owned payload/result/status buffers. Results remain scalar
descriptor/success returns plus copied internal task-owned pump/status records;
the public SyscallNumber vocabulary, STABLE_SVC_IMMEDIATE, and TALOS_* syscall
constants remain unchanged. The future evidence must include deterministic
malformed selector or payload, missing owner, invalid or closed descriptor,
capacity, user-memory, buffer-pressure, timeout/retry, and device-error
controls. Shell ping, kernel-backed fake command expansion, public sockets,
stable syscall ABI acceptance, socket syscall ABI acceptance, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, and phase transition remain rejected. The selected next bounded task
is phase12-network-vfs-ping-diagnostic-svc-core-20260620.

phase12-network-vfs-ping-diagnostic-svc-core-20260620 accepts
phase12-network-vfs-ping-diagnostic-svc-core-accepted. src/syscall.rs now
provides VfsPingDiagnosticSvcFixture, a crate-internal host-only diagnostic
fixture that first resolves a task-owned executable-shaped path through a
ReadOnlyInitramfs regular-file lookup and then drives the accepted
dispatch_process_local_ping_descriptor_user_arguments bridge.

The accepted source/unit evidence covers one VFS-backed diagnostic lifecycle:
open, idle status copy-out, start from copied diagnostic payload memory,
pump_or_read_result through ARP-to-ICMP progression, completed status copy-out,
and close. It also covers VFS executable lookup failure, malformed selector and
payload, missing owner, invalid and closed descriptors, process descriptor
capacity, output-buffer pressure, invalid user memory, scratch pressure, retry
exhaustion, explicit timeout, caller receive-buffer pressure, device receive IO
error, and unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
The evidence level remains source/unit host/QEMU-substitute over
VFS/initramfs diagnostic identity, experimental user-argument decoding,
UserMapping copy-in/copy-out, process-local descriptor ownership, internal
dispatch-shaped control, fake/trait-level NetworkDevice behavior,
caller-owned buffers, task-owned result/status slots, and fixed-capacity state.
Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad socket expansion, and
phase transition remain rejected. The selected next bounded task is
phase12-network-vfs-ping-diagnostic-svc-closeout-20260620.

phase12-network-vfs-ping-diagnostic-svc-closeout-20260620 accepts
phase12-network-vfs-ping-diagnostic-svc-closeout-accepted. The closeout
reconciles the accepted contract, core implementation, source/unit transcript,
task records, docs, durable state, and rejected claims before retaining broader
smoke evidence. It confirms that VfsPingDiagnosticSvcFixture remains a
crate-internal host-only diagnostic bridge over a VFS/initramfs
executable-shaped identity,
dispatch_process_local_ping_descriptor_user_arguments, UserMapping
copy-in/copy-out, process-local descriptor ownership, internal dispatch-shaped
control, fake/trait-level NetworkDevice behavior, caller-owned buffers,
task-owned result/status slots, and fixed-capacity state.

The accepted evidence level remains source/unit host-only. It accepts only the
already committed diagnostic lifecycle and deterministic controls for malformed
selector and payload, missing owner, invalid and closed descriptors, process
descriptor capacity, VFS executable lookup failure, output-buffer pressure,
invalid user memory, scratch pressure, retry exhaustion, explicit timeout,
caller receive-buffer pressure, device receive IO error, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell ping,
kernel-backed fake command expansion, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-vfs-ping-diagnostic-svc-smoke-20260620.

phase12-network-vfs-ping-diagnostic-svc-smoke-20260620 accepts
phase12-network-vfs-ping-diagnostic-svc-smoke-accepted. The retained
host/QEMU-substitute transcript preserves the accepted VFS/userspace
diagnostic SVC lifecycle through VfsPingDiagnosticSvcFixture and
dispatch_process_local_ping_descriptor_user_arguments without adding runtime
behavior. The smoke command records VFS executable lookup, open, idle status
copy-out, start from copied diagnostic payload memory, pump_or_read_result
through ARP-to-ICMP result copy-out, echo-reply completion, completed status
copy-out, and close over fake/trait-level NetworkDevice behavior.

The retained smoke transcript also records deterministic controls for missing
executable identity, malformed selector and payload, missing owner, invalid
and closed descriptors, process descriptor capacity, output-buffer pressure,
invalid user memory, scratch pressure, caller receive-buffer pressure, retry
exhaustion, explicit timeout, device receive IO error, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. The accepted evidence
level remains host/QEMU-substitute over a VFS/userspace diagnostic SVC bridge,
VFS/initramfs executable identity, experimental user-argument decoding,
UserMapping copy-in/copy-out, process-local descriptor ownership, internal
dispatch-shaped control, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state. Shell ping, kernel-backed fake command expansion,
public sockets, stable syscall ABI acceptance, socket syscall ABI acceptance,
live driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, broad
socket expansion, and phase transition remain rejected. The selected next
bounded task is phase12-network-vfs-ping-diagnostic-svc-smoke-closeout-20260620.

phase12-network-vfs-ping-diagnostic-svc-smoke-closeout-20260620 accepts
phase12-network-vfs-ping-diagnostic-svc-smoke-closeout-accepted. The accepted
evidence level remains host/QEMU-substitute smoke evidence over a
VFS/userspace diagnostic SVC bridge, VFS/initramfs executable identity,
experimental user-argument decoding, UserMapping copy-in/copy-out,
process-local descriptor ownership, internal dispatch-shaped control,
fake/trait-level NetworkDevice behavior, caller-owned buffers, task-owned
result/status slots, and fixed-capacity state.

The closeout reconciles the accepted contract, core, core closeout, retained
smoke transcript, task records, docs, durable state, and rejected claims. It
accepts that the retained smoke evidence covers the diagnostic lifecycle from
VFS executable lookup through close plus deterministic missing-executable,
malformed argument, owner, descriptor, capacity, user-memory, buffer-pressure,
scratch-pressure, timeout/retry, device-error, and stable syscall-vocabulary
controls. Shell ping, kernel-backed fake command expansion, public sockets,
stable syscall ABI acceptance, socket syscall ABI acceptance, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, and phase transition remain rejected. No later bounded task is
mechanically unblocked; supervisor planning is required before any next Phase
12.4 task or phase transition.

phase12-network-vfs-ping-diagnostic-packet-queue-contract-20260620 accepts
phase12-network-vfs-ping-diagnostic-packet-queue-contract-accepted. The
contract selects the next host-only Phase 12.4 feature boundary after the
accepted VFS ping diagnostic SVC smoke closeout: a crate-internal,
fixed-capacity packet queue/adapter that makes the diagnostic's ARP and ICMP
packet movement observable without widening the public ABI.

The future core may only record outbound ARP request and ICMP echo request
frames, inject ARP/ICMP reply frames through fake/trait-level NetworkDevice
behavior, and preserve the existing VFS executable lookup,
dispatch_process_local_ping_descriptor_user_arguments, UserMapping
copy-in/copy-out, process-local descriptor ownership, caller-owned
payload/result/status buffers, and task-owned diagnostic state. Deterministic
future evidence must cover outbound ARP/ICMP recording, injected reply
progression, queue capacity, buffer pressure, malformed input and injected
frames, wrong owner or descriptor, invalid and closed descriptors,
timeout/retry, device/error controls, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell ping,
kernel-backed fake command expansion, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-vfs-ping-diagnostic-packet-queue-core-20260620.

phase12-network-vfs-ping-diagnostic-packet-queue-core-20260620 accepts
phase12-network-vfs-ping-diagnostic-packet-queue-core-accepted. src/network.rs
now provides a crate-internal PacketQueueNetworkDevice over fixed-capacity
receive and transmit queues of bounded PacketQueueFrame records. The adapter
implements NetworkDevice for host-only diagnostic use: transmitted frames are
copied into an inspectable queue, injected receive frames are copied out through
receive_frame, full queues and oversized frames remain deterministic, and
injected receive/transmit errors map through the existing trait-level
DeviceError boundary.

The accepted source/unit evidence wires VfsPingDiagnosticSvcFixture through
that queue-backed NetworkDevice path. The diagnostic still resolves /bin/pingdiag
through ReadOnlyInitramfs, copies payload bytes from UserMapping-backed memory,
opens a process-local descriptor, starts one unresolved ping-like transaction,
records the outbound Ethernet/IPv4 ARP request, injects a matching ARP reply,
records the outbound Ethernet/IPv4/ICMP echo request, injects a matching ICMP
echo reply, copies completed status/result records back to user memory, and
closes the descriptor. Deterministic controls cover transmit queue capacity,
oversized injected frames, caller output-buffer pressure, malformed injected
frames, explicit retry, timeout, receive/transmit IO errors, invalid descriptor,
and unchanged SyscallNumber/TALOS_* vocabulary. Existing accepted tests continue
to cover owner rejection, closed descriptors, caller receive-buffer pressure,
malformed arguments, user-memory faults, scratch pressure, and the
descriptor-shaped/VFS diagnostic lifecycle. Shell ping, public sockets,
stable/socket ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
link-hardware retry, broad socket expansion, and phase transition remain
rejected. The selected next bounded task is
phase12-network-vfs-ping-diagnostic-packet-queue-closeout-20260620.

phase12-network-vfs-ping-diagnostic-packet-queue-closeout-20260620 accepts
phase12-network-vfs-ping-diagnostic-packet-queue-closeout-accepted. The
closeout reconciles the accepted packet queue core source, source/unit tests,
task record, durable state, architecture notes, roadmap, and rejected claims.
The accepted evidence level remains host/QEMU-substitute source/unit evidence
over crate-internal fixed-capacity packet queue records behind the VFS-backed
userspace ping diagnostic SVC path.

The closeout preserves the accepted boundary: PacketQueueNetworkDevice records
outbound ARP request and IPv4/ICMP echo request frames, accepts injected
ARP/ICMP reply frames, completes status/result copy-out through UserMapping,
and closes the process-local descriptor without exposing a public socket or
stable networking ABI. Deterministic coverage remains with the accepted core
for queue capacity, oversized frames, output-buffer pressure, malformed injected
frames, explicit retry, timeout, device errors, invalid descriptors, unchanged
SyscallNumber/TALOS_* vocabulary, and predecessor descriptor/VFS diagnostic
controls. Shell ping, public sockets, stable/socket ABI acceptance, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, and phase transition remain rejected. The selected next bounded task
is phase12-network-vfs-ping-diagnostic-packet-queue-smoke-20260620.

phase12-network-vfs-ping-diagnostic-packet-queue-smoke-20260620 accepts
phase12-network-vfs-ping-diagnostic-packet-queue-smoke-accepted. The retained
host/QEMU-substitute smoke transcript records the VFS-backed diagnostic
lifecycle through the packet queue boundary: /bin/pingdiag VFS executable
lookup, diagnostic SVC argument decoding, UserMapping payload copy-in,
process-local descriptor open/start/pump/status/close, PacketQueueNetworkDevice
outbound ARP request recording, injected ARP reply progression, outbound
IPv4/ICMP echo request recording, injected ICMP echo reply progression, and
UserMapping status/result copy-out.

The smoke evidence distinguishes queue-backed behavior from immediate
fake-device-only behavior by requiring outbound ARP and ICMP frames to be copied
into inspectable packet queue transmit records and inbound ARP/ICMP replies to
be injected through the receive queue before progress is observed.
Deterministic controls cover missing VFS identity, malformed selector and
payload, wrong or missing owner, invalid and closed descriptors, process
descriptor capacity, packet queue capacity, frame capacity, caller output and
receive buffer pressure, malformed injected frames, invalid user memory,
scratch pressure, timeout/retry, receive/transmit device errors, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell ping, public
sockets, stable/socket ABI acceptance, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, broad socket expansion, and phase transition
remain rejected. The selected next bounded task is
phase12-network-vfs-ping-diagnostic-packet-queue-smoke-closeout-20260620.

phase12-network-vfs-ping-diagnostic-packet-queue-smoke-closeout-20260620
accepts
phase12-network-vfs-ping-diagnostic-packet-queue-smoke-closeout-accepted. The
closeout reconciles the accepted packet queue contract, core, core closeout,
retained smoke transcript, docs, durable state, and rejected claims. The
accepted evidence level remains host/QEMU-substitute smoke evidence over
crate-internal fixed-capacity packet queue records, the VFS/userspace
diagnostic SVC bridge, experimental user-argument decoding, UserMapping
copy-in/copy-out, process-local descriptor ownership, fake/trait-level
NetworkDevice behavior, caller-owned buffers, task-owned state, and fixed
capacity.

The retained smoke frontier proves /bin/pingdiag VFS executable lookup,
diagnostic SVC argument decoding, outbound ARP and IPv4/ICMP echo request
records, injected ARP and ICMP reply progression, status/result copy-out, and
close. Deterministic controls cover missing VFS identity, malformed
selector/payload, owner and descriptor failures, queue/frame capacity, caller
buffer pressure, malformed injected frames, invalid user memory, scratch
pressure, timeout/retry, device errors, and unchanged syscall vocabulary. Shell
ping, public sockets, stable/socket ABI acceptance, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected. No later bounded task is mechanically unblocked;
supervisor planning is required before live driver packet I/O, public sockets,
shell ping, SSH, or phase transition.

phase12-network-driver-packet-pump-contract-20260620 accepts
phase12-network-driver-packet-pump-contract-accepted. The contract selects the
next host-only Phase 12.4 feature boundary after the accepted packet queue
smoke closeout: a crate-internal driver-facing packet pump that drains
diagnostic outbound packet records to trait-level NetworkDevice transmit
behavior and polls trait-level receive behavior back into diagnostic inbound
packet records.

The future core may only bridge the accepted fixed-capacity packet queue records
and fake/trait-level NetworkDevice behavior. It must preserve /bin/pingdiag VFS
lookup, diagnostic SVC argument decoding, UserMapping copy-in/copy-out,
process-local descriptor ownership, caller-owned buffers, task-owned state,
fixed-capacity queues, transmit FIFO ordering, explicit receive polling order,
bounded backpressure, deterministic malformed-frame handling, device-error
propagation, close/drop cleanup, timeout/retry controls, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell ping, public
sockets, stable/socket ABI acceptance, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, broad socket expansion, and phase transition
remain rejected. The selected next bounded task is
phase12-network-driver-packet-pump-core-20260620.

phase12-network-driver-packet-pump-core-20260620 accepts
phase12-network-driver-packet-pump-core-accepted. The core implements the
host-only crate-internal packet pump boundary between accepted diagnostic
packet queue records and trait-level NetworkDevice behavior. It adds
PacketQueueDriverPumpStep plus PacketQueueNetworkDevice::pump_driver, which
drains one outbound record in FIFO order to transmit_frame before polling
receive_frame into the inbound diagnostic queue.

The accepted source/unit evidence proves outbound ARP and IPv4/ICMP records now
cross the packet pump into a trait-level driver queue, and injected ARP/ICMP
replies cross back through the pump before the VFS-backed process-local
descriptor observes progress. Deterministic coverage includes transmit
ordering, transmit retry preservation after device errors, receive polling
order, receive queue backpressure, caller receive-buffer pressure, oversized
frames, malformed injected frames, missing/wrong owner or descriptor, invalid
and closed descriptors, timeout/retry, close/drop behavior, receive/transmit
device errors, process descriptor capacity, user-memory faults, scratch
pressure, and unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
Shell ping, public sockets, stable/socket ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad socket expansion, and
phase transition remain rejected. The selected next bounded task is
phase12-network-driver-packet-pump-closeout-20260620.

phase12-network-driver-packet-pump-closeout-20260620 accepts
phase12-network-driver-packet-pump-closeout-accepted. The closeout reconciles
the accepted packet pump core source, unit evidence, task record, docs, durable
state, and rejected claims before retaining broader smoke evidence.

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over crate-internal packet queue records, trait-level NetworkDevice pump
behavior, the VFS/userspace diagnostic SVC bridge, UserMapping copy-in/copy-out,
process-local descriptor ownership, caller-owned buffers, task-owned state,
transmit FIFO ordering, explicit receive polling, deterministic backpressure,
device-error propagation, timeout/retry controls, close/drop behavior, and
unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell ping,
public sockets, stable/socket ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-driver-packet-pump-smoke-20260620.

phase12-network-driver-packet-pump-smoke-20260620 accepts
phase12-network-driver-packet-pump-smoke-accepted. The retained
host/QEMU-substitute smoke command is
scripts/qemu-driver-packet-pump-smoke.sh, with transcript evidence under
tasks/evidence/2026-06-20-driver-packet-pump-smoke/. The smoke exercises the
accepted VFS /bin/pingdiag diagnostic lifecycle through
PacketQueueNetworkDevice::pump_driver over trait-level NetworkDevice behavior.

The accepted smoke evidence covers /bin/pingdiag VFS lookup, diagnostic SVC
argument decoding, UserMapping copy-in/copy-out, process-local descriptor
ownership, outbound ARP and IPv4/ICMP echo request records crossing pump_driver
to trait-level transmit behavior, injected ARP and ICMP echo replies crossing
back through pump_driver, completed status/result copy-out, close/drop
behavior, and deterministic controls for missing VFS identity, malformed
arguments, missing or wrong owner, invalid and closed descriptors, process
descriptor capacity, queue capacity/backpressure, caller buffer pressure,
malformed received frames, timeout/retry, transmit and receive device errors,
and unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell
ping, public sockets, stable/socket ABI acceptance, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-driver-packet-pump-smoke-closeout-20260620.

phase12-network-driver-packet-pump-smoke-closeout-20260620 accepts
phase12-network-driver-packet-pump-smoke-closeout-accepted. The closeout
reconciles the accepted smoke transcript, task record, source anchors, docs,
durable state, and rejected claims before supervisor planning selects any later
live adapter, socket, shell ping, SSH, or phase-transition work.

The accepted evidence level remains host/QEMU-substitute smoke over
crate-internal packet queue records, trait-level NetworkDevice pump behavior,
the VFS/userspace diagnostic SVC bridge, experimental user-argument decoding,
UserMapping copy-in/copy-out, process-local descriptor ownership, caller-owned
buffers, task-owned state, fixed capacity, transmit FIFO ordering, explicit
receive polling, deterministic backpressure, device-error propagation,
timeout/retry controls, close/drop behavior, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell ping, public
sockets, stable/socket ABI acceptance, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, broad socket expansion, and phase transition
remain rejected. No later bounded task is mechanically unblocked; supervisor
planning is required before live driver adapters, live packet I/O, public
sockets, shell ping, stable/socket ABI acceptance, SSH, broad socket expansion,
Phase 12.1 hardware retry, or a phase transition.

phase12-network-shell-pingdiag-contract-20260620 accepts
phase12-network-shell-pingdiag-contract-accepted. Supervisor planning after the
driver packet pump smoke closeout selected a shell-visible `/bin/pingdiag`
diagnostic contract as the next smallest user-visible Phase 12.4 feature step.
The future core must expose a command-loop or VFS/userspace execution transcript
that opens, starts, pumps, reports status/result, and closes `/bin/pingdiag`
through the accepted VFS executable identity, diagnostic SVC user-argument
bridge, process-local descriptor ownership, UserMapping copy-in/copy-out,
packet queues, and PacketQueueNetworkDevice::pump_driver.

The contract requires deterministic future coverage for successful ARP/ICMP
diagnostic progression plus malformed arguments, missing VFS executable
identity, owner/descriptor failures, invalid and closed descriptors, queue
capacity/backpressure, timeout/retry, transmit and receive device errors,
close/drop behavior, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Kernel-backed fake shell
commands, public sockets, stable/socket ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 hardware retry, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-shell-pingdiag-core-20260620.

phase12-network-shell-pingdiag-core-20260620 accepts
phase12-network-shell-pingdiag-core-accepted. The core adds `/bin/pingdiag` to
the read-only initramfs executable set and wires `exec /bin/pingdiag` through
the existing shell-visible VFS open/read, ELF planning, startup ABI, lifecycle,
`waitpid`, and `laststatus` transcript path. After the VFS execution boundary
is established, the command runs the accepted diagnostic SVC lifecycle over
process-local descriptor ownership, UserMapping copy-in/copy-out, packet
queues, and `PacketQueueNetworkDevice::pump_driver`.

The accepted source/unit host/QEMU-substitute evidence proves a shell-visible
transcript that opens the diagnostic descriptor, starts ARP resolution, pumps
the outbound ARP request to a trait-level driver queue, injects and pumps the
ARP reply, pumps the outbound IPv4/ICMP echo request, injects and pumps the
ICMP echo reply, observes completed status/result copy-out, closes the
descriptor, and preserves waitpid/laststatus lifecycle observation. Focused
shell controls cover malformed `/bin/pingdiag` arguments and missing VFS
executable identity; the accepted diagnostic SVC tests continue to cover wrong
owner/descriptor, invalid and closed descriptors, queue backpressure,
timeout/retry, transmit and receive device errors, close/drop behavior, and
unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Kernel-backed
fake shell commands, public sockets, stable/socket ABI acceptance, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 hardware retry, broad shell expansion,
broad socket expansion, and phase transition remain rejected. The selected
next bounded task is phase12-network-shell-pingdiag-closeout-20260620.

phase12-network-shell-pingdiag-closeout-20260620 accepts
phase12-network-shell-pingdiag-closeout-accepted. The closeout reconciles the
accepted shell-visible `/bin/pingdiag` core source, tests, docs, task
evidence, durable-state frontier, accepted claims, and rejected claims. The
accepted evidence level remains source/unit host/QEMU-substitute over
shell-visible VFS/userspace diagnostic execution: VFS executable identity,
command-loop transcript, diagnostic SVC user-argument decoding, process-local
descriptor ownership, UserMapping copy-in/copy-out, packet queues,
`PacketQueueNetworkDevice::pump_driver`, status/result copy-out,
close/drop behavior, and `waitpid`/`laststatus` lifecycle observation.

The closeout confirms this is not a kernel-backed fake shell command and does
not accept public sockets, stable/socket ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, broad shell expansion, broad socket expansion, Phase 12.1
hardware retry, or a phase transition. The retained smoke task is objective
and bounded because it has explicit dependencies, acceptance criteria,
validation gates, evidence requirements, and rejected-claim boundaries for a
host/QEMU-substitute transcript only. The selected next bounded task is
phase12-network-shell-pingdiag-smoke-20260620.

phase12-network-shell-pingdiag-smoke-20260620 accepts
phase12-network-shell-pingdiag-smoke-accepted. The retained smoke evidence
records a shell-visible `exec /bin/pingdiag` transcript through VFS executable
lookup, VFS open/read execution, startup ABI, diagnostic SVC user-argument
decoding, process-local descriptor ownership, UserMapping copy-in/copy-out,
packet queues, `PacketQueueNetworkDevice::pump_driver`, completed
status/result copy-out, close, `waitpid`, and `laststatus`.

The accepted evidence level is host/QEMU-substitute smoke. The transcript
proves successful ARP and ICMP progression through the accepted packet-pump
layers and retains deterministic controls for malformed arguments, missing VFS
executable identity, owner/descriptor failures, invalid and closed descriptors,
queue capacity/backpressure, caller buffer pressure, malformed received frames,
timeout/retry, transmit and receive device errors, close/drop behavior, and
unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. It does not
accept kernel fake commands, public sockets, stable/socket ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP,
lab mutation, boot publication, Phase 12.1 hardware retry, broad shell
expansion, broad socket expansion, or a phase transition. The selected next
bounded task is phase12-network-shell-pingdiag-smoke-closeout-20260620.

phase12-network-shell-pingdiag-smoke-closeout-20260620 accepts
phase12-network-shell-pingdiag-smoke-closeout-accepted. The closeout reconciles
the retained smoke transcript, task evidence, source anchors, docs, durable
state, accepted claims, and rejected claims before supervisor planning selects
any later public socket, live packet I/O, SSH, hardware retry, or
phase-transition work.

The accepted evidence level remains host/QEMU-substitute smoke over
shell-visible VFS/userspace diagnostic execution, VFS executable lookup, VFS
open/read execution, startup ABI, diagnostic SVC user-argument decoding,
process-local descriptor ownership, UserMapping copy-in/copy-out,
fixed-capacity packet queues, `PacketQueueNetworkDevice::pump_driver`,
caller-owned buffers, task-owned state, completed status/result copy-out,
close/drop behavior, `waitpid`, `laststatus`, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Kernel fake commands,
public sockets, stable/socket ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, broad shell expansion, broad socket expansion, Phase 12.1 hardware
retry, and phase transition remain rejected. At closeout acceptance, no later
bounded task was mechanically unblocked; supervisor planning was required
before public sockets, live driver packet I/O, SSH, Phase 12.1 hardware retry,
broad socket expansion, or a phase transition.

phase12-network-socket-open-close-abi-contract-20260620 accepts
phase12-network-socket-open-close-abi-contract-accepted. Supervisor planning
after the shell-visible pingdiag smoke closeout selected the smallest socket
integration boundary: a private experimental socket-open selector paired with
the existing process descriptor close path.

The accepted contract reserves a future private `TALOS_SOCKET_SYSCALL = 6`
selector on the existing `STABLE_SVC_IMMEDIATE = 0` path with scalar
`socket(domain, type, protocol)` arguments. The only accepted success tuple is
`AF_INET=2`, `SOCK_STREAM=1`, and `protocol=0`; the returned value is the
lowest available process descriptor backed by a fixed-capacity
`DescriptorObjectKind::Socket` entry owned by the current process. Close uses
the existing `TALOS_CLOSE_SYSCALL = 2` path and must drop both the process
descriptor and socket backing entry. Error vocabulary is bounded to existing
Talos POSIX errors for unsupported tuple, reserved argument, descriptor
capacity, backing capacity, wrong-owner, invalid, and closed descriptor cases.

This contract does not accept send, recv, bind, connect, listen, accept,
poll/blocking network I/O, UDP/TCP payload transport, explicit `IPPROTO_TCP`,
datagram/raw sockets, live driver adapters, live packet I/O, hardware
reachability, smoltcp, SSH, lab mutation, boot publication, broad socket
expansion, public stable socket ABI acceptance, or phase transition. The
selected next bounded task is phase12-network-socket-open-close-core-20260620.

phase12-network-socket-open-close-core-20260620 accepts
phase12-network-socket-open-close-core-accepted. The implementation adds the
private experimental `TALOS_SOCKET_SYSCALL = 6` selector to
`SyscallNumber` and a socket-table-aware process descriptor dispatch path.
The accepted runtime surface remains limited to opening `AF_INET=2`,
`SOCK_STREAM=1`, `protocol=0` sockets into
`DescriptorObjectKind::Socket` process descriptors and closing them through
the existing `TALOS_CLOSE_SYSCALL = 2` descriptor lifetime path. Backing
entries record owner/domain/type/protocol in a fixed-capacity socket table;
close validates owner/backing identity, drops the process descriptor, and then
drops the matching socket backing entry.

Focused source/unit evidence covers successful open/close, unsupported
domain/type/protocol, reserved arguments, missing owner, process descriptor
capacity, socket backing capacity, wrong-owner backing rejection, invalid and
closed descriptor behavior, and unchanged non-socket close behavior. This core
does not add `/bin/sockdiag`, generated-root content, send, recv, bind,
connect, listen, accept, poll/blocking network I/O, UDP/TCP payload transport,
live packet I/O, hardware reachability, smoltcp, SSH, lab mutation, boot
publication, broad socket expansion, public stable socket ABI acceptance, or a
phase transition. The selected next bounded task is
phase12-network-shell-sockdiag-open-close-core-20260620.

phase12-network-shell-sockdiag-open-close-core-20260620 accepts
phase12-network-shell-sockdiag-open-close-core-accepted. The implementation
adds `/bin/sockdiag` to the read-only initramfs executable set and wires
`exec /bin/sockdiag` through the existing shell-visible VFS open/read, ELF
planning, startup ABI, process lifecycle, `waitpid`, and `laststatus`
transcript path. After that VFS/userspace boundary is established, the
diagnostic opens an accepted `AF_INET=2`, `SOCK_STREAM=1`, `protocol=0`
socket through `TALOS_SOCKET_SYSCALL = 6`, observes the
`DescriptorObjectKind::Socket`/read-write descriptor, closes it through
`TALOS_CLOSE_SYSCALL = 2`, and verifies backing-state drop.

Focused source/unit host/QEMU-substitute evidence covers the shell-visible
VFS/userspace execution path, successful socket open/close, unsupported
domain/type/protocol controls, invalid closed-descriptor control, unchanged
`/bin/pingdiag` regression coverage, updated `/bin` listings, and unchanged
waitpid/laststatus lifecycle observation. This task does not accept send,
recv, bind, connect, listen, accept, poll/blocking network I/O, UDP/TCP
payload transport, live packet I/O, hardware reachability, smoltcp, SSH, lab
mutation, boot publication, generated-root publication, broad socket
expansion, public stable socket ABI acceptance, or phase transition. The
selected next bounded task is
phase12-network-shell-sockdiag-open-close-smoke-20260620.

phase12-network-shell-sockdiag-open-close-smoke-20260620 accepts
phase12-network-shell-sockdiag-open-close-smoke-accepted. The retained
host/QEMU-substitute smoke evidence records a shell-visible
`exec /bin/sockdiag` transcript through VFS executable lookup/open/read,
startup ABI, `TALOS_SOCKET_SYSCALL = 6` socket open,
`DescriptorObjectKind::Socket` process descriptor ownership, bounded socket
backing state, `TALOS_CLOSE_SYSCALL = 2` close/drop, `waitpid`, and
`laststatus`.

The smoke transcript retains deterministic controls for malformed arguments,
missing executable identity, unsupported domain/type/protocol, invalid and
closed descriptors, wrong-owner backing, descriptor capacity, socket backing
capacity, no-partial-allocation failures, scalar dispatch ENOTSUP outside the
socket-table-aware path, bounded syscall vocabulary, and unchanged
`/bin/pingdiag` behavior. This task does not accept send, recv, bind,
connect, listen, accept, poll/blocking network I/O, UDP/TCP payload
transport, live packet I/O, hardware reachability, smoltcp, SSH, lab
mutation, boot publication, broad socket expansion, public stable socket ABI
acceptance, or phase transition. The selected next bounded task is
phase12-network-shell-sockdiag-open-close-closeout-20260620.

phase12-network-shell-sockdiag-open-close-closeout-20260620 accepts
phase12-network-shell-sockdiag-open-close-closeout-accepted. The closeout
reconciles the ABI contract, socket open/close core, shell /bin/sockdiag core,
and retained smoke evidence as a host/QEMU-substitute frontier only. The
accepted boundary is shell-visible socket open/close through VFS/userspace
execution: executable lookup/open/read for /bin/sockdiag, startup ABI,
TALOS_SOCKET_SYSCALL = 6 for AF_INET/SOCK_STREAM/protocol 0, process
DescriptorObjectKind::Socket ownership, bounded socket backing state,
TALOS_CLOSE_SYSCALL = 2 close/drop, waitpid, laststatus, deterministic
negative controls, and unchanged /bin/pingdiag behavior.

The closeout explicitly rejects send, recv, bind, connect, listen, accept,
poll/blocking network I/O, UDP/TCP payload transport, live driver adapters,
live packet I/O, hardware reachability, lab mutation, boot publication, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, and
phase transition. selected_next_task is null and planningNeeded=true pending
supervisor planning for any next bounded Phase 12.4 socket or network task.

phase12-network-socket-bind-listen-abi-contract-20260620 accepts
phase12-network-socket-bind-listen-abi-contract-accepted. Supervisor planning
after the shell-visible sockdiag open/close closeout selected the next bounded
socket integration boundary: private descriptor-backed bind/listen state for
sockets created by the accepted `TALOS_SOCKET_SYSCALL = 6` AF_INET stream open
path.

The accepted contract reserves future private `TALOS_BIND_SYSCALL = 7` and
`TALOS_LISTEN_SYSCALL = 8` selectors on the existing
`STABLE_SVC_IMMEDIATE = 0` path. Bind uses scalar `x0=fd`, `x1=ipv4_be`,
`x2=port`, and `x3..x5=0`; `ipv4_be` must fit in 32 bits and `port` must be in
`1..=65535`. Listen uses scalar `x0=fd`, `x1=backlog`, and `x2..x5=0`;
`backlog` must be in `1..=4`. The socket backing state is limited to
`OpenUnbound`, `Bound { local_endpoint }`, and
`Listening { local_endpoint, backlog }`.

Bind succeeds only on an open unbound socket owned by the current process and
transitions it to bound state; repeated bind or bind after listen returns
`EINVAL`. Listen succeeds on a bound socket and transitions it to listening
state; repeated listen on an already listening socket updates the recorded
backlog and returns success. Invalid endpoints, reserved arguments,
listen-before-bind, and invalid state return `EINVAL`; invalid, closed,
non-socket, missing-owner, wrong-owner, or missing-backing descriptors return
`EBADF`. Close remains `TALOS_CLOSE_SYSCALL = 2` and drops unbound, bound, or
listening backing state with the process descriptor.

This contract does not accept send, recv, connect, accept, poll/blocking
network I/O, UDP/TCP payload transport, accept queues, global port registry or
address-conflict policy, live driver adapters, live packet I/O, hardware
reachability, smoltcp, SSH, lab mutation, boot publication, broad socket
expansion, public stable socket ABI acceptance, or phase transition. The
selected next bounded task is
phase12-network-socket-bind-listen-core-20260620.

phase12-network-socket-bind-listen-core-20260620 accepts
phase12-network-socket-bind-listen-core-accepted. The implementation adds
private experimental `TALOS_BIND_SYSCALL = 7` and
`TALOS_LISTEN_SYSCALL = 8` selectors only to the socket-table-aware process
descriptor dispatch path. Scalar dispatch without socket-table context still
returns `ENOTSUP`, preserving the accepted private task-chain boundary.

The backing socket record now carries explicit state: `OpenUnbound`,
`Bound { local_endpoint }`, and `Listening { local_endpoint, backlog }`.
Bind validates reserved arguments, a 32-bit big-endian IPv4 scalar, port
`1..=65535`, current-process descriptor ownership, `DescriptorObjectKind::Socket`,
and backing socket owner before transitioning `OpenUnbound` to `Bound`.
Listen validates reserved arguments, backlog `1..=4`, descriptor/backing
ownership, and state before transitioning a bound socket to listening or
updating an already listening socket backlog. Failed bind/listen calls leave
prior socket state unchanged; closing an unbound, bound, or listening socket
continues to drop the process descriptor and matching backing entry.

The accepted evidence level is source/unit host/QEMU-substitute coverage for
successful bind/listen transitions, repeated bind rejection, repeated listen
backlog updates, close/drop cleanup, scalar-dispatch `ENOTSUP`, malformed
arguments, listen-before-bind, non-socket descriptors, wrong-owner backing
rejection, and state preservation on failures. This core does not accept shell
`/bin/sockdiag` bind/listen reporting, send, recv, connect, accept,
poll/blocking network I/O, UDP/TCP payload transport, accept queues, global
port registry or address-conflict policy, live driver adapters, live packet
I/O, hardware reachability, smoltcp, SSH, lab mutation, boot publication,
broad socket expansion, public stable socket ABI acceptance, or phase
transition. The selected next bounded task is
phase12-network-shell-sockdiag-bind-listen-core-20260620.

phase12-network-shell-sockdiag-bind-listen-core-20260620 accepts
phase12-network-shell-sockdiag-bind-listen-core-accepted. The implementation
extends the existing shell-visible /bin/sockdiag VFS/userspace diagnostic to
exercise the accepted socket open -> bind -> listen -> close path through the
socket-table-aware descriptor dispatch. The transcript still proves
initramfs/VFS executable lookup/open/read, startup ABI, lifecycle, waitpid,
laststatus, descriptor kind/access, and close/drop cleanup, and now records the
bound IPv4 endpoint, listen backlog, bind/listen return values, and listening
socket state.

The accepted evidence level is source/unit host/QEMU-substitute coverage for
shell-visible /bin/sockdiag over TALOS_SOCKET_SYSCALL = 6,
TALOS_BIND_SYSCALL = 7, TALOS_LISTEN_SYSCALL = 8, and TALOS_CLOSE_SYSCALL = 2.
The deterministic controls cover malformed arguments, missing executable
identity, unsupported socket domain/type/protocol, bind on a closed descriptor,
listen-before-bind, invalid bind endpoint, invalid backlog, repeated bind,
repeated listen backlog update, double-close EBADF, waitpid, laststatus, and
unchanged accepted open/close diagnostic behavior.

This core does not accept retained smoke evidence, send, recv, connect,
accept, poll/blocking network I/O, UDP/TCP payload transport, accept queues,
global port registry or address-conflict policy, live driver adapters, live
packet I/O, hardware reachability, smoltcp, SSH, lab mutation, boot
publication, generated-root publication, broad socket expansion, public stable
socket ABI acceptance, or phase transition. The selected next bounded task is
phase12-network-shell-sockdiag-bind-listen-smoke-20260620.

phase12-network-shell-sockdiag-bind-listen-smoke-20260620 accepts
phase12-network-shell-sockdiag-bind-listen-smoke-accepted. The retained smoke
evidence records shell-visible /bin/sockdiag bind/listen behavior over the
accepted VFS/userspace execution path: VFS executable lookup/open/read,
startup ABI, TALOS_SOCKET_SYSCALL = 6, TALOS_BIND_SYSCALL = 7,
TALOS_LISTEN_SYSCALL = 8, TALOS_CLOSE_SYSCALL = 2, process descriptor socket
ownership, descriptor-backed listening state, close/drop cleanup, waitpid, and
laststatus.

The accepted evidence level is host/QEMU-substitute smoke only. The smoke
transcript retains deterministic controls for malformed arguments, missing
executable identity, unsupported socket domain/type/protocol, listen-before-bind,
invalid bind endpoint, invalid backlog, repeated bind, repeated listen backlog
update, invalid/closed descriptors, wrong-owner backing, scalar-dispatch
ENOTSUP outside the socket-table-aware path, bounded syscall vocabulary,
unchanged socket open/close behavior, and unchanged /bin/pingdiag behavior.

This smoke does not accept source runtime behavior beyond the retained smoke
script/evidence, send, recv, connect, accept, poll/blocking network I/O,
UDP/TCP payload transport, accept queues, global port registry or
address-conflict policy, live driver adapters, live packet I/O, hardware
reachability, smoltcp, SSH, lab mutation, boot publication, generated-root
publication, broad socket expansion, public stable socket ABI acceptance, or
phase transition. The selected next bounded task is
phase12-network-shell-sockdiag-bind-listen-closeout-20260620.

phase12-network-shell-sockdiag-bind-listen-closeout-20260620 accepts
phase12-network-shell-sockdiag-bind-listen-closeout-accepted. The closeout
reconciles the bind/listen ABI contract, socket bind/listen core, shell
/bin/sockdiag core, and retained smoke evidence as a host/QEMU-substitute
frontier only. The accepted boundary is shell-visible socket bind/listen
through VFS/userspace execution: executable lookup/open/read for /bin/sockdiag,
startup ABI, TALOS_SOCKET_SYSCALL = 6 for AF_INET/SOCK_STREAM/protocol 0,
TALOS_BIND_SYSCALL = 7 for the accepted local endpoint, TALOS_LISTEN_SYSCALL =
8 for bounded listening state, process DescriptorObjectKind::Socket ownership,
TALOS_CLOSE_SYSCALL = 2 close/drop, waitpid, laststatus, deterministic
controls, unchanged socket open/close behavior, unchanged /bin/pingdiag
behavior, and unchanged bounded syscall vocabulary.

The closeout explicitly rejects send, recv, connect, accept, poll/blocking
network I/O, UDP/TCP payload transport, accept queues, global port registry or
address-conflict policy, live driver adapters, live packet I/O, hardware
reachability, lab mutation, boot publication, generated-root publication, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, and
phase transition. selected_next_task is null and planningNeeded=true pending
supervisor planning for any next bounded Phase 12.4 socket or network task.

phase12-network-socket-connect-accept-abi-contract-20260620 accepts
phase12-network-socket-connect-accept-abi-contract-accepted. Supervisor
planning after the shell-visible sockdiag bind/listen closeout selected the
next bounded socket integration boundary: private descriptor-backed local
connect/accept handshake state for AF_INET stream sockets created by the
accepted socket open, bind, and listen path.

The accepted contract reserves future private TALOS_CONNECT_SYSCALL = 9 and
TALOS_ACCEPT_SYSCALL = 10 selectors on the existing STABLE_SVC_IMMEDIATE = 0
path. Connect uses scalar x0=fd, x1=ipv4_be, x2=port, and x3..x5=0. Accept
uses scalar x0=listener_fd and x1..x5=0, returning a new current-process
descriptor for the accepted server-side socket.

Listener lookup is intentionally process-local and deterministic: connect
targets exactly one Listening socket owned by the current process whose local
endpoint matches the requested IPv4/port. Zero or multiple matches return
EINVAL. The listener pending-peer queue is bounded by the accepted listen
backlog; full queues return ENOSPC and accept on an empty queue returns EAGAIN.
Client endpoints are synthetic local state only, 127.0.0.1:(49152 +
client_socket_descriptor.raw()), so this contract does not accept ephemeral
port allocation, routing policy, TCP behavior, or packet transport.

The backing socket states remain descriptor-owned and extend the accepted
OpenUnbound, Bound, and Listening model with Connected and Accepted local peer
states. Connect and accept are all-or-nothing: failed calls leave client,
listener, pending queue, process descriptor, and socket backing state
unchanged. Closing a listener drops queued local peers; closing connected or
accepted descriptors drops only that descriptor backing state.

This contract does not accept send, recv, poll/blocking network I/O, UDP/TCP
payload transport, cross-process sockets, live driver adapters, live packet
I/O, hardware reachability, lab mutation, boot publication, SSH, smoltcp,
broad socket expansion, public stable socket ABI acceptance, or phase
transition. The selected next bounded task is
phase12-network-socket-connect-accept-core-20260620.

phase12-network-socket-connect-accept-core-20260620 accepts
phase12-network-socket-connect-accept-core-accepted. The implementation adds
private TALOS_CONNECT_SYSCALL = 9 and TALOS_ACCEPT_SYSCALL = 10 selectors only
to the socket-table-aware process descriptor dispatch path; scalar dispatch
still returns ENOTSUP outside that context.

The accepted source/unit frontier is descriptor-backed local handshake state:
connect transitions an open-unbound current-process AF_INET stream socket to
Connected and queues one synthetic local peer on exactly one matching
current-process listener; accept dequeues one pending peer and creates a new
current-process descriptor backed by an Accepted server-side socket. The
listener pending queue is bounded by the accepted listen backlog, client
endpoints remain synthetic 127.0.0.1:(49152 + socket_descriptor.raw()), and
failures leave prior client/listener/descriptor/backing state unchanged.

Focused unit coverage records successful handshake state, close/drop cleanup,
scalar-dispatch ENOTSUP, reserved and malformed scalar arguments, listener
absence, empty accept, full listener queue, process descriptor capacity, socket
backing capacity, and non-socket descriptor rejection. This does not accept
shell /bin/sockdiag connect/accept output, retained smoke evidence, send, recv,
poll/blocking network I/O, UDP/TCP payload transport, cross-process sockets,
live driver adapters, live packet I/O, hardware reachability, lab mutation,
boot publication, SSH, smoltcp, broad socket expansion, public stable socket
ABI acceptance, or phase transition. The selected next bounded task is
phase12-network-shell-sockdiag-connect-accept-core-20260620.

phase12-network-shell-sockdiag-connect-accept-core-20260620 accepts
phase12-network-shell-sockdiag-connect-accept-core-accepted. The implementation
extends the existing shell-visible /bin/sockdiag VFS/userspace diagnostic to
exercise the accepted private local connect/accept selectors through the
socket-table-aware descriptor dispatch. The diagnostic now records listener,
client, and accepted descriptors; the local bind endpoint; bind/listen,
connect, accept, and close return values; and the expected Listening,
Connected, and Accepted socket states.

The accepted evidence level is source/unit host/QEMU-substitute coverage for
shell-visible /bin/sockdiag over TALOS_SOCKET_SYSCALL = 6,
TALOS_BIND_SYSCALL = 7, TALOS_LISTEN_SYSCALL = 8,
TALOS_CONNECT_SYSCALL = 9, TALOS_ACCEPT_SYSCALL = 10, and
TALOS_CLOSE_SYSCALL = 2. The deterministic controls cover malformed
arguments, missing executable identity, unsupported socket domain/type/protocol,
listen-before-bind, invalid bind endpoint, invalid backlog, repeated bind,
repeated listen backlog update, accept-before-connect, no matching listener,
full pending queue, non-socket descriptors, invalid/closed descriptors,
waitpid, laststatus, and unchanged accepted open/bind/listen behavior.

This core does not accept retained smoke evidence, send, recv, poll/blocking
network I/O, UDP/TCP payload transport, live driver adapters, live packet I/O,
hardware reachability, smoltcp, SSH, lab mutation, boot publication,
generated-root publication, broad socket expansion, public stable socket ABI
acceptance, or phase transition. The selected next bounded task is
phase12-network-shell-sockdiag-connect-accept-smoke-20260620.

phase12-network-shell-sockdiag-connect-accept-smoke-20260620 accepts
phase12-network-shell-sockdiag-connect-accept-smoke-accepted. The retained
host/QEMU-substitute smoke evidence records shell-visible /bin/sockdiag local
connect/accept behavior over the accepted VFS/userspace execution path:
executable lookup/open/read for /bin/sockdiag, startup ABI, TALOS_SOCKET,
TALOS_BIND, TALOS_LISTEN, TALOS_CONNECT, TALOS_ACCEPT, and TALOS_CLOSE through
socket-table-aware process descriptor dispatch. The transcript records
listener fd 3, client fd 4, accepted fd 6, successful connect/accept returns,
Listening/Connected/Accepted state, close/drop cleanup, waitpid, and
laststatus.

The retained controls cover malformed arguments, missing executable identity,
unsupported socket domain/type/protocol, listen-before-bind, invalid endpoint
and backlog, repeated bind/listen behavior, accept-before-connect, missing
listener, pending queue backpressure, non-socket descriptors, invalid/closed
descriptors, descriptor and backing capacity, unchanged open/close behavior,
unchanged bind/listen behavior, and unchanged /bin/pingdiag behavior. The
evidence remains host/QEMU-substitute smoke only and does not accept send,
recv, payload bytes, poll/blocking network I/O, UDP/TCP payload transport,
live driver adapters, live packet I/O, hardware reachability, SSH, public
stable socket ABI acceptance, broad socket expansion, or phase transition. The
selected next bounded task is
phase12-network-shell-sockdiag-connect-accept-closeout-20260620.

phase12-network-shell-sockdiag-connect-accept-closeout-20260620 accepts
phase12-network-shell-sockdiag-connect-accept-closeout-accepted. The closeout
reconciles the connect/accept ABI contract, descriptor-backed core, shell
/bin/sockdiag core, and retained smoke evidence as a host/QEMU-substitute
frontier only. The accepted boundary is shell-visible local connect/accept
through VFS/userspace execution: executable lookup/open/read for /bin/sockdiag,
startup ABI, TALOS_SOCKET_SYSCALL = 6 for AF_INET/SOCK_STREAM/protocol 0,
TALOS_BIND_SYSCALL = 7 for the accepted local endpoint,
TALOS_LISTEN_SYSCALL = 8 for bounded listening state,
TALOS_CONNECT_SYSCALL = 9 for one current-process local listener,
TALOS_ACCEPT_SYSCALL = 10 for a new accepted current-process descriptor,
process DescriptorObjectKind::Socket ownership, listener/client/accepted
descriptor-backed state, TALOS_CLOSE_SYSCALL = 2 close/drop, waitpid,
laststatus, deterministic controls, unchanged socket open/close behavior,
unchanged bind/listen behavior, unchanged /bin/pingdiag behavior, and
unchanged bounded syscall vocabulary.

The closeout explicitly rejects send, recv, payload bytes, poll/blocking
network I/O, UDP/TCP payload transport, cross-process sockets, global port
registry or address-conflict policy, live driver adapters, live packet I/O,
hardware reachability, lab mutation, boot publication, generated-root
publication, SSH, smoltcp, broad socket expansion, public stable socket ABI
acceptance, and phase transition. At closeout acceptance time,
selected_next_task was null and planningNeeded=true pending supervisor
planning for any next bounded Phase 12.4 socket or network task.

phase12-network-socket-send-recv-abi-contract-20260620 accepts
phase12-network-socket-send-recv-abi-contract-accepted. Supervisor planning
after the shell-visible sockdiag connect/accept closeout selected the next
bounded socket integration boundary: private descriptor-backed local send/recv
payload transfer for AF_INET stream sockets already connected or accepted by
the accepted local socket chain.

The accepted contract reserves future private TALOS_SEND_SYSCALL = 11 and
TALOS_RECV_SYSCALL = 12 selectors on the existing STABLE_SVC_IMMEDIATE = 0
path. Send uses scalar x0=fd, x1=user_buffer_start, x2=len, x3=flags=0, and
x4=x5=0; it copies readable caller bytes into the peer socket's inbound queue.
Recv uses scalar x0=fd, x1=user_buffer_start, x2=len, x3=flags=0, and
x4=x5=0; it copies queued bytes from the caller socket's inbound queue into
writable caller memory.

Each Connected or Accepted socket owns a fixed 64-byte inbound FIFO. Send is
nonblocking and all-or-nothing: len 0 returns 0, oversize or insufficient peer
queue capacity returns ENOSPC, and no partial send is accepted. Recv is
nonblocking: len 0 returns 0, an empty queue returns EAGAIN while the peer
exists, otherwise recv returns and consumes min(len, queued_bytes). Missing,
closed, wrong-owner, duplicate, or non-connected peers return EPIPE for send;
recv may drain already queued bytes before reporting EPIPE once the queue is
empty and no peer exists.

This contract does not accept shell /bin/sockdiag send/recv output, retained
smoke evidence, poll/blocking network I/O, readiness/wait queues, UDP/TCP
payload transport, cross-process sockets, live driver adapters, live packet
I/O, hardware reachability, lab mutation, boot publication, SSH, smoltcp,
broad socket expansion, public stable socket ABI acceptance, or phase
transition. The selected next bounded task is
phase12-network-socket-send-recv-core-20260620.

phase12-network-socket-send-recv-core-20260620 accepts
phase12-network-socket-send-recv-core-accepted. The implementation adds private
TALOS_SEND_SYSCALL = 11 and TALOS_RECV_SYSCALL = 12 selectors only to the
socket-table-aware process descriptor dispatch path; scalar dispatch and
descriptor dispatch paths without socket-table state still return ENOTSUP.

The accepted source/unit frontier is descriptor-backed local payload transfer
between accepted Connected and Accepted AF_INET stream socket states. Each
connected or accepted socket now owns a 64-byte inbound FIFO. Send validates
the current-process socket descriptor, connected state, unique reverse-endpoint
peer, peer queue capacity, and readable caller buffer before appending all
bytes to the peer inbound queue. Recv validates the descriptor and local queue,
copies queued bytes into writable caller memory, and consumes bytes only after
copy-out succeeds.

Focused unit coverage records client-to-server and server-to-client byte
transfer, short reads, scalar-dispatch ENOTSUP, empty receive EAGAIN, malformed
flags, non-socket and non-connected descriptors, oversize and full-queue
ENOSPC, caller-buffer EFAULT with unchanged queue state, close/drop cleanup,
queued-byte drain after peer close, and EPIPE after the disconnected peer queue
is empty. This does not accept shell /bin/sockdiag send/recv output, retained
smoke evidence, poll/blocking network I/O, readiness/wait queues, UDP/TCP
payload transport, cross-process sockets, live driver adapters, live packet
I/O, hardware reachability, lab mutation, boot publication, SSH, smoltcp,
broad socket expansion, public stable socket ABI acceptance, or phase
transition. The selected next bounded task is
phase12-network-shell-sockdiag-send-recv-core-20260620.

phase12-network-shell-sockdiag-send-recv-core-20260620 accepts
phase12-network-shell-sockdiag-send-recv-core-accepted. The shell-visible
`/bin/sockdiag` diagnostic now exercises the accepted private local
send/recv path through VFS executable lookup/open/read, startup ABI,
descriptor-backed socket open/bind/listen/connect/accept,
`TALOS_SEND_SYSCALL = 11`, `TALOS_RECV_SYSCALL = 12`,
`TALOS_CLOSE_SYSCALL = 2`, waitpid, and laststatus.

The accepted source/unit frontier proves local bidirectional payload bytes
(`client->server` and `server->client`) move between the connected client
and accepted server descriptors through the accepted caller-buffer copy path.
Controls cover empty recv `EAGAIN`, invalid send/recv flags `EINVAL`,
payload queue backpressure `ENOSPC`, send after peer close `EPIPE`,
malformed arguments, missing executable identity, unsupported socket
parameters, connect/accept controls, non-socket descriptors, invalid/closed
descriptors, and bounded syscall vocabulary. Retained smoke evidence,
poll/blocking I/O, readiness/wait queues, UDP/TCP payload transport,
cross-process sockets, live driver adapters, live packet I/O, hardware
reachability, lab mutation, boot publication, generated-root publication, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, and
phase transition remain rejected. The selected next bounded task is
phase12-network-shell-sockdiag-send-recv-smoke-20260620.

phase12-network-shell-sockdiag-send-recv-smoke-20260620 accepts
phase12-network-shell-sockdiag-send-recv-smoke-accepted. The retained
host/QEMU-substitute smoke evidence records shell-visible /bin/sockdiag local
send/recv over the accepted private socket path. The transcript covers VFS
executable lookup/open/read for /bin/sockdiag, startup ABI, TALOS_SOCKET,
TALOS_BIND, TALOS_LISTEN, TALOS_CONNECT, TALOS_ACCEPT, TALOS_SEND,
TALOS_RECV, TALOS_CLOSE, waitpid, laststatus, and socket-table-aware process
descriptor dispatch.

The retained smoke proves bidirectional local payload bytes
(client->server and server->client) between connected client and accepted
server descriptors, plus deterministic controls for malformed arguments,
missing executable identity, unsupported socket parameters, bind/listen and
connect/accept controls, empty recv, invalid send/recv flags, payload queue
backpressure, send after peer close, non-socket descriptors, invalid/closed
descriptors, descriptor/backing capacity, scalar dispatch ENOTSUP, unchanged
socket open/close behavior, unchanged bind/listen behavior, unchanged
connect/accept behavior, and unchanged /bin/pingdiag behavior. It does not
accept poll/blocking I/O, readiness/wait queues, UDP/TCP payload transport,
cross-process sockets, live driver adapters, live packet I/O, hardware
reachability, lab mutation, boot publication, generated-root publication, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, or phase
transition. The selected next bounded task is
phase12-network-shell-sockdiag-send-recv-closeout-20260620.

phase12-network-shell-sockdiag-send-recv-closeout-20260620 accepts
phase12-network-shell-sockdiag-send-recv-closeout-accepted. The closeout
reconciles the send/recv ABI contract, descriptor-backed core, shell
/bin/sockdiag core, and retained smoke evidence as a source/unit plus
host/QEMU-substitute frontier only. The accepted boundary is shell-visible
local payload transfer through VFS/userspace execution: executable lookup/open/
read for /bin/sockdiag, startup ABI, TALOS_SOCKET_SYSCALL = 6 for
AF_INET/SOCK_STREAM/protocol 0, TALOS_BIND_SYSCALL = 7,
TALOS_LISTEN_SYSCALL = 8, TALOS_CONNECT_SYSCALL = 9,
TALOS_ACCEPT_SYSCALL = 10, TALOS_SEND_SYSCALL = 11,
TALOS_RECV_SYSCALL = 12, TALOS_CLOSE_SYSCALL = 2, process
DescriptorObjectKind::Socket ownership, descriptor-backed listener/client/
accepted socket state, per-socket 64-byte inbound FIFOs, bidirectional local
payload bytes, close/drop, waitpid, laststatus, deterministic controls,
unchanged socket open/close behavior, unchanged bind/listen behavior,
unchanged connect/accept behavior, unchanged /bin/pingdiag behavior, and
unchanged bounded syscall vocabulary.

The closeout explicitly rejects poll/blocking network I/O, readiness/wait
queues, UDP/TCP payload transport, smoltcp integration, cross-process sockets,
global port registry or address-conflict policy, live driver adapters, live
packet I/O, hardware reachability, lab mutation, boot publication,
generated-root publication, SSH, broad socket expansion, public stable socket
ABI acceptance, and phase transition. selected_next_task is null and
planningNeeded=true pending supervisor planning for any next bounded Phase
12.4 socket or network task.

phase12-network-socket-readiness-poll-abi-contract-20260621 accepts
phase12-network-socket-readiness-poll-abi-contract-accepted. Supervisor
planning after the shell-visible sockdiag send/recv closeout selected the next
bounded socket integration boundary: private nonblocking readiness observation
for accepted process-local socket states.

The accepted contract reserves future private TALOS_POLL_SYSCALL = 13 on the
existing STABLE_SVC_IMMEDIATE = 0 path. The syscall uses x0 as a caller-owned
user poll-entry array pointer, x1 as an entry count from 1 through 8, x2 as
flags=0, and x3=x4=x5 as reserved zero registers. Each entry is a fixed
16-byte native layout: fd, events, and revents. Supported private readiness
bits are READ, WRITE, HANGUP, and ERROR; unsupported flags, reserved
registers, bad event masks, zero entries, or too many entries return EINVAL,
and caller-buffer copy failures return EFAULT.

Readiness remains descriptor-backed and process-local only. Listening sockets
report READ when a pending local peer can be accepted. Connected and Accepted
sockets report READ when their inbound FIFO has bytes, WRITE when a one-byte
send would fit in the unique local peer's inbound FIFO, and HANGUP when the
accepted reverse-endpoint peer is absent or closed. Queued bytes after peer
close report READ | HANGUP, and an empty queue after peer close still reports
READ | HANGUP so a subsequent nonblocking recv can observe EPIPE. Invalid,
closed, wrong-owner, non-socket, or missing-backing descriptors report
per-entry ERROR. This contract does not accept runtime implementation, shell
/bin/sockdiag readiness output, retained smoke evidence, blocking sleep,
scheduler wait queues, wakeup registration, timeout handling, UDP/TCP payload
transport, cross-process/global poll sets, live packet I/O, hardware
reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, or phase transition. The selected next bounded task is
phase12-network-socket-readiness-poll-core-20260621.

phase12-network-socket-readiness-poll-core-20260621 accepts
phase12-network-socket-readiness-poll-core-accepted. The implementation adds
private TALOS_POLL_SYSCALL = 13 only to the socket-table-aware process
descriptor dispatch path while scalar/default dispatch still fails closed with
ENOTSUP. The accepted behavior is a bounded nonblocking user poll-entry array:
x0 points to 1 through 8 fixed 16-byte entries, x1 is the entry count, x2 and
x3 through x5 must be zero, and each entry carries fd, events, and overwritten
revents. Unsupported event bits or malformed scalar arguments fail the whole
call with EINVAL; copy faults fail with EFAULT; per-entry bad descriptors,
non-socket descriptors, wrong-owner or missing-backing relationships, and
oversized fd values report ERROR in revents.

The accepted readiness model remains process-local and descriptor-backed only.
Listening sockets report READ for a nonempty pending accept queue. Connected
and Accepted sockets report READ for queued inbound bytes, WRITE when the
unique local peer has at least one byte of inbound FIFO capacity, and HANGUP
after peer close/drop. Queued bytes after peer close report READ | HANGUP when
READ was requested, and an empty queue after peer close still reports
READ | HANGUP so the accepted recv path can expose EPIPE. Source/unit tests
cover listener readiness, local read/write readiness, peer FIFO backpressure,
peer close/hangup, invalid and non-socket descriptors, malformed user buffers,
unsupported flags/events, count bounds, and scalar dispatch. Shell /bin/sockdiag
readiness output, retained smoke evidence, blocking waits, scheduler wait
queues, timeout handling, UDP/TCP payload transport, cross-process/global poll
sets, live packet I/O, hardware reachability, SSH, public socket ABI
acceptance, broad socket expansion, and phase transition remain rejected. The
selected next bounded task is
phase12-network-shell-sockdiag-readiness-poll-core-20260621.

phase12-network-shell-sockdiag-readiness-poll-core-20260621 accepts
phase12-network-shell-sockdiag-readiness-poll-core-accepted. The shell-visible
/bin/sockdiag diagnostic now exercises the accepted private
TALOS_POLL_SYSCALL = 13 readiness path through VFS executable lookup/open/read,
startup ABI, descriptor-backed socket open/bind/listen/connect/accept,
send/recv, poll, close, waitpid, and laststatus. The transcript records
poll-empty-listener=0, poll-pending-listener=READ, poll-empty-recv=0,
poll-payload-recv=READ, poll-write-ready=WRITE, poll-write-backpressure=0,
poll-peer-hangup=READ | HANGUP, poll-invalid-descriptor=ERROR, and
poll-non-socket-descriptor=ERROR. Controls also exercise unsupported poll
events as EINVAL while preserving existing socket open/close, bind/listen,
connect/accept, send/recv, malformed argument, missing executable, non-socket,
and invalid/closed descriptor regressions.

The accepted evidence level is source/unit host/QEMU-substitute only and the
diagnostic remains private, nonblocking, descriptor-backed, and process-local.
Retained smoke evidence, blocking waits, scheduler wait queues, timeout
handling, UDP/TCP payload transport, cross-process/global poll sets, live
packet I/O, hardware reachability, SSH, public socket ABI acceptance, broad
socket expansion, and phase transition remain rejected. The selected next
bounded task is phase12-network-shell-sockdiag-readiness-poll-smoke-20260621.

phase12-network-shell-sockdiag-readiness-poll-smoke-20260621 accepts
phase12-network-shell-sockdiag-readiness-poll-smoke-accepted. The retained
host/QEMU-substitute smoke evidence records shell-visible /bin/sockdiag
readiness/poll behavior over the accepted private socket path. The transcript
covers VFS executable lookup/open/read for /bin/sockdiag, startup ABI,
TALOS_SOCKET, TALOS_BIND, TALOS_LISTEN, TALOS_CONNECT, TALOS_ACCEPT,
TALOS_SEND, TALOS_RECV, TALOS_POLL, TALOS_CLOSE, waitpid, laststatus, and
socket-table-aware process descriptor dispatch.

The retained smoke proves listener pending accept READ, empty recv queue zero
readiness, queued payload READ, writable peer FIFO WRITE, full peer FIFO zero
write readiness, peer close READ | HANGUP, invalid descriptor ERROR,
non-socket descriptor ERROR, unsupported poll events EINVAL, malformed poll
calls, scalar dispatch ENOTSUP, unchanged accepted socket diagnostics, and
unchanged /bin/pingdiag behavior. It does not accept blocking waits, scheduler
wait queues, timeout handling, UDP/TCP payload transport, cross-process/global
poll sets, live driver adapters, live packet I/O, hardware reachability, lab
mutation, boot publication, generated-root publication, SSH, smoltcp, broad
socket expansion, public stable socket ABI acceptance, or phase transition.
The selected next bounded task is
phase12-network-shell-sockdiag-readiness-poll-closeout-20260621.

phase12-network-shell-sockdiag-readiness-poll-closeout-20260621 accepts
phase12-network-shell-sockdiag-readiness-poll-closeout-accepted. The closeout
freezes the shell-visible readiness/poll frontier at source/unit plus
host/QEMU-substitute evidence over VFS/userspace /bin/sockdiag execution and
private nonblocking local socket readiness. The accepted boundary includes VFS
executable lookup/open/read, startup ABI, TALOS_SOCKET, TALOS_BIND,
TALOS_LISTEN, TALOS_CONNECT, TALOS_ACCEPT, TALOS_SEND, TALOS_RECV, TALOS_POLL,
TALOS_CLOSE, descriptor-backed listener/client/accepted socket state, waitpid,
laststatus, deterministic readiness/error controls, unchanged accepted socket
diagnostics, and unchanged /bin/pingdiag behavior.

The closeout explicitly rejects blocking waits, scheduler wait queues, timeout
handling, UDP/TCP payload transport, smoltcp integration, cross-process/global
poll sets, live driver adapters, live packet I/O, hardware reachability, lab
mutation, boot publication, generated-root publication, SSH, broad socket
expansion, public stable socket ABI acceptance, and phase transition.
selected_next_task is null and planningNeeded=true pending supervisor planning
for any next bounded Phase 12.4 socket or network task.

phase12-network-socket-blocking-poll-wait-contract-20260621 accepts
phase12-network-socket-blocking-poll-wait-contract-accepted. Supervisor
planning after the nonblocking readiness/poll closeout selected the next
bounded socket integration boundary: one private process-local blocking wait
over accepted local socket readiness states.

The accepted contract reserves future private TALOS_POLL_WAIT_SYSCALL = 14 on
the existing STABLE_SVC_IMMEDIATE = 0 path. The syscall reuses the accepted
16-byte TALOS_POLL entry layout, takes x0 as a caller-owned user poll-entry
array, x1 as an entry count from 1 through 8, x2 as a finite relative timeout
from 1 through 1024 scheduler ticks, x3 as flags=0, and x4=x5 as reserved zero
registers. TALOS_POLL_SYSCALL = 13 remains the unchanged nonblocking readiness
query; zero-timeout behavior stays there rather than being folded into the
blocking selector.

The accepted wait model is scheduler-owned rather than a diagnostic retry
loop. If any requested entry is immediately ready, TALOS_POLL_WAIT returns
without sleeping. If no entry is ready, the kernel records one wait object tied
to the current task, process owner, descriptor snapshot, requested readiness
bits, and deadline tick; transitions that task to TaskState::Blocked; and
resumes it only through listener pending-accept enqueue, inbound payload bytes,
peer FIFO write capacity becoming available, peer close/hangup, descriptor
invalidation/error, or timeout expiration. Timeout returns success value 0 with
all revents zero. Invalid, closed, wrong-owner, non-socket, missing-backing, or
locally invalidated descriptors report per-entry ERROR where possible.

The contract does not accept runtime implementation, /bin/sockdiag blocking
wait output, retained smoke evidence, busy-loop acceptance, cross-process or
global poll sets, UDP/TCP payload transport, smoltcp integration, live packet
I/O, hardware reachability, SSH, public stable socket ABI acceptance, broad
socket expansion, signals/restart semantics, arbitrary cancellation, or phase
transition. The selected next bounded task is
phase12-network-socket-blocking-poll-wait-core-20260621.

phase12-network-socket-blocking-poll-wait-core-20260621 accepts
phase12-network-socket-blocking-poll-wait-core-accepted. The runtime core now
implements the private TALOS_POLL_WAIT_SYSCALL = 14 path selected by the
contract while leaving TALOS_POLL_SYSCALL = 13 unchanged as the accepted
nonblocking readiness query. Scalar/default dispatch still fails closed with
ENOTSUP unless the explicit socket-table wait-aware dispatch boundary supplies
the current task, current scheduler tick, user mappings, descriptor store,
socket table, and bounded SocketPollWaitTable.

The accepted behavior reuses the fixed 16-byte poll-entry array and accepts
finite relative timeouts from 1 through 1024 scheduler ticks. Immediate-ready
entries return without sleeping and write the same revents bits as TALOS_POLL.
When no requested entry is ready, the kernel records one wait for the current
task, snapshots process-local socket descriptors and requested readiness bits,
records the deadline tick, transitions the task to TaskState::Blocked, and
resumes it through SingleCoreScheduler::make_runnable when accepted local
socket readiness appears or the timeout expires. Timeout writes zero revents
and returns success value 0.

Source/unit tests cover immediate readiness, local send/recv wake, listener
pending-accept wake, peer close/hangup wake, timeout, malformed arguments,
scalar fail-closed behavior, and nonblocking TALOS_POLL compatibility. This
accepts only private process-local bounded blocking waits over local sockets.
It does not accept /bin/sockdiag blocking wait output, retained smoke evidence,
cross-process/global poll sets, UDP/TCP payload transport, smoltcp
integration, live packet I/O, hardware reachability, SSH, public socket ABI
acceptance, broad socket expansion, or phase transition. The selected next
bounded task is
phase12-network-shell-sockdiag-blocking-poll-wait-core-20260621.

phase12-network-shell-sockdiag-blocking-poll-wait-core-20260621 accepts
phase12-network-shell-sockdiag-blocking-poll-wait-core-accepted. The existing
shell-visible `/bin/sockdiag` VFS/userspace diagnostic now exercises the
accepted private process-local bounded blocking poll wait over local AF_INET
stream sockets. The transcript covers executable lookup/open/read, startup
ABI, descriptor-backed socket setup, wait registration through
`TALOS_POLL_WAIT_SYSCALL = 14`, scheduler-visible `TaskState::Blocked`,
resume through `SingleCoreScheduler::make_runnable`, waitpid, and laststatus.

The accepted shell-visible cases are immediate-ready listener readiness,
pending-listener wake after local connect, payload-read wake after local send,
finite timeout with zero revents, peer close/hangup wake, scalar fail-closed
`ENOTSUP`, invalid timeout `EINVAL`, unsupported events `EINVAL`, and
unchanged accepted open/close, bind/listen, connect/accept, send/recv, and
nonblocking `TALOS_POLL` controls. This remains source/unit
host/QEMU-substitute evidence only. It does not accept retained smoke
evidence, cross-process/global poll sets, UDP/TCP payload transport, smoltcp
integration, live packet I/O, hardware reachability, SSH, public stable socket
ABI acceptance, broad socket expansion, or phase transition. The selected next
bounded task is
phase12-network-shell-sockdiag-blocking-poll-wait-smoke-20260621.

phase12-network-shell-sockdiag-blocking-poll-wait-smoke-20260621 accepts
phase12-network-shell-sockdiag-blocking-poll-wait-smoke-accepted. The retained
host/QEMU-substitute smoke evidence records shell-visible /bin/sockdiag
execution through VFS/userspace lookup/open/read, startup ABI, the accepted
private local socket open/bind/listen/connect/accept/send/recv/poll/wait/close
path, scheduler-visible blocked/resume state, waitpid, and laststatus.

The retained smoke covers immediate-ready wait completion, pending-listener
wake after local connect, payload-read wake after local send, finite timeout
with zero revents, peer close/hangup wake, scalar fail-closed ENOTSUP, invalid
timeout EINVAL, unsupported events EINVAL, unchanged accepted socket
diagnostics, unchanged nonblocking TALOS_POLL, unchanged /bin/pingdiag, and
bounded syscall vocabulary. This remains host/QEMU-substitute smoke evidence
only. It does not accept UDP/TCP payload transport, smoltcp integration,
cross-process/global poll sets, live packet I/O, hardware reachability, SSH,
public stable socket ABI acceptance, broad socket expansion, or phase
transition. The selected next bounded task is
phase12-network-shell-sockdiag-blocking-poll-wait-closeout-20260621.

phase12-network-shell-sockdiag-blocking-poll-wait-closeout-20260621 accepts
phase12-network-shell-sockdiag-blocking-poll-wait-closeout-accepted. The
closeout reconciles the accepted blocking poll-wait contract, runtime core,
shell-visible /bin/sockdiag source/unit diagnostic, retained smoke evidence,
architecture status, roadmap status, and durable state as a source/unit plus
host/QEMU-substitute frontier only.

The accepted boundary is private process-local bounded blocking waits over
descriptor-backed local socket readiness through VFS/userspace /bin/sockdiag
execution: executable lookup/open/read, startup ABI, local socket
open/bind/listen/connect/accept/send/recv/poll/wait/close, scheduler-visible
TaskState::Blocked and SingleCoreScheduler::make_runnable resume, waitpid,
laststatus, deterministic controls, unchanged accepted socket diagnostics,
unchanged nonblocking TALOS_POLL, unchanged /bin/pingdiag, and unchanged
bounded syscall vocabulary. Cross-process/global poll sets, UDP/TCP payload
transport, smoltcp integration, live driver adapters, live packet I/O,
hardware reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, and phase transition remain rejected. selected_next_task is null
and planningNeeded=true pending supervisor planning for any next bounded
Phase 12.4 socket or network task.

phase12-network-cross-process-local-socket-rendezvous-contract-20260621
accepts
phase12-network-cross-process-local-socket-rendezvous-contract-accepted.
Supervisor planning after the blocking poll-wait closeout selected the next
bounded socket integration boundary: a private cross-process local rendezvous
contract for two distinct process descriptor stores.

The accepted contract keeps process-visible file descriptors in their owning
process descriptor tables and uses a bounded kernel-local rendezvous table for
listener records, pending connection records, and connected-pair records.
Connect targets a listener endpoint without creating an fd in the listener
process. Accept consumes a pending connection, allocates a server-owned socket
backing descriptor, installs a descriptor-table entry in the accepting
process, and joins that accepted socket to the client's connection id. Payload
state is connection-local with two bounded queues, one per direction.

The accepted readiness and wait semantics are the prior local socket semantics
lifted across the connection record: listener pending-accept READ, inbound
payload READ, peer queue capacity WRITE, peer close/drop HANGUP, descriptor or
connection invalidation ERROR, and finite scheduler-owned poll-wait timeout.
Close/drop/exit cleanup must wake affected waiters, preserve queued bytes
until drained after peer close, and release connection capacity only after both
endpoints are gone and queues are drained.

This remains source/task/docs evidence only. Runtime implementation,
shell-visible /bin/sockdiag cross-process diagnostics, retained smoke
evidence, UDP/TCP payload transport, smoltcp integration, live driver
adapters, live packet I/O, hardware reachability, SSH, public stable socket
ABI acceptance, broad socket expansion, and phase transition remain rejected.
The selected next bounded task is
phase12-network-cross-process-local-socket-rendezvous-core-20260621.

phase12-network-cross-process-local-socket-rendezvous-core-20260621 accepts
phase12-network-cross-process-local-socket-rendezvous-core-accepted. The
private socket backing table now supports cross-process local rendezvous for
distinct ProcessOwnerId descriptor stores while keeping process-visible file
descriptors in their owning descriptor tables. Listener discovery is global to
the bounded socket table, pending peers record the client owner and backing
descriptor, and connected/accepted sockets share a private connection id so
same-number descriptors in different processes cannot collapse ownership.

The accepted source/unit behavior covers cross-process bind/listen/connect/
accept, bidirectional send/recv, nonblocking readiness, bounded poll-wait
wakeups for listener readiness, payload readiness, and peer hangup, stale
pending-client cleanup, owner-wide socket cleanup for process-exit style
teardown, queued-byte drain after peer close, EPIPE after drain, duplicate
active-listener rejection, and bounded capacity errors. This is still private
source/unit evidence through the socket-table-aware syscall dispatch and
SocketPollWaitTable; no shell-visible /bin/sockdiag cross-process diagnostic
or retained smoke is accepted by this task.

UDP/TCP payload transport, smoltcp integration, live driver adapters, live
packet I/O, Pi 5 hardware runs, lab mutation, boot publication, hardware
reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, and phase transition remain rejected. The selected next bounded
task is phase12-network-shell-sockdiag-cross-process-local-socket-core-20260621.

phase12-network-shell-sockdiag-cross-process-local-socket-core-20260621
accepts
phase12-network-shell-sockdiag-cross-process-local-socket-core-accepted. The
existing shell-visible `/bin/sockdiag` VFS/userspace diagnostic now exercises
the accepted private cross-process local socket rendezvous core through two
distinct `ProcessOwnerId` descriptor tables. The local command harness can
hold a shell/server owner and a client owner, while the diagnostic reports
server-owned listener/accepted descriptors, a client-owned connected
descriptor, cross-process connect/accept, bidirectional payload transfer,
listener and payload poll-wait wakeups, peer-close hangup readiness, and
deterministic cleanup.

This remains source/unit evidence through `/bin/sockdiag` and the
socket-table-aware syscall dispatch only. Retained smoke evidence, UDP/TCP
payload transport, smoltcp integration, live driver adapters, live packet I/O,
Pi 5 hardware runs, lab mutation, boot publication, hardware reachability,
SSH, public stable socket ABI acceptance, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-shell-sockdiag-cross-process-local-socket-smoke-20260621.

phase12-network-shell-sockdiag-cross-process-local-socket-smoke-20260621
accepts
phase12-network-shell-sockdiag-cross-process-local-socket-smoke-accepted. The
retained host/QEMU-substitute smoke evidence records shell-visible
`/bin/sockdiag` cross-process local socket rendezvous through VFS/userspace
execution. It covers executable lookup/open/read, startup ABI, distinct
server/client ProcessOwnerId descriptor owners, server-owned listener and
accepted descriptors, a client-owned connected descriptor, bidirectional
payload transfer, listener and payload bounded wait wakeups, peer-close hangup
readiness, cleanup release, waitpid, laststatus, malformed/missing executable
controls, prior process-local sockdiag diagnostics, and unchanged /bin/pingdiag
behavior.

The first smoke attempt exposed predecessor drift in source comparisons for
Connected/Accepted connection_id fields and a duplicate-listener borrow shape;
the smoke task fixed those compile blockers without accepting a broader runtime
contract. Accepted evidence remains host/QEMU-substitute only. UDP/TCP payload
transport, smoltcp integration, live driver adapters, live packet I/O, Pi 5
hardware runs, lab mutation, boot publication, hardware reachability, SSH,
public stable socket ABI acceptance, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-shell-sockdiag-cross-process-local-socket-closeout-20260621.

phase12-network-shell-sockdiag-cross-process-local-socket-closeout-20260621
accepts
phase12-network-shell-sockdiag-cross-process-local-socket-closeout-accepted.
The cross-process local socket frontier is now closed at source/unit plus
host/QEMU-substitute evidence over shell-visible VFS/userspace /bin/sockdiag
and private local rendezvous only. The accepted boundary covers distinct
server/client ProcessOwnerId descriptor owners, server-owned listener and
accepted descriptors, client-owned connected descriptor, cross-process
connect/accept, bidirectional local payload transfer, listener and payload
bounded wait wakeups, peer-close hangup readiness, cleanup release, waitpid,
laststatus, deterministic controls, and unchanged prior diagnostics.

This closeout rejects public stable socket ABI acceptance, UDP/TCP payload
transport, smoltcp integration, live driver adapters, live packet I/O, Pi 5
hardware behavior, lab mutation, boot publication, generated-root publication,
hardware reachability, SSH, broad socket expansion, and phase transition.
planningNeeded=true; no later bounded Phase 12.4 or network task is
mechanically unblocked without supervisor planning.

phase12-network-smoltcp-adoption-contract-20260621 accepts
phase12-network-smoltcp-adoption-contract-accepted. Supervisor planning after
the cross-process local socket closeout selected smoltcp adoption as the next
bounded network feature path, but only as a contract and dependency-core
boundary before packet movement or TCP behavior.

The accepted contract adopts smoltcp 0.13.1 for a future host-only TCP
frontier using default-features=false, medium-ethernet, proto-ipv4, socket-tcp,
and only the smallest bounded count/buffer features required by compilation.
The contract rejects std, host OS phy backends, DHCP/DNS, IPv6, fragmentation,
UDP, raw/ICMP sockets, async, multicast, auto ICMP reply, defmt, live packet
I/O, hardware reachability, SSH, socket syscall bridging, public stable socket
ABI acceptance, broad socket expansion, and phase transition.

State ownership remains Talos-first. NetworkDevice, PacketQueueNetworkDevice,
process descriptors, NetworkSocketDescriptorTable, SocketPollWaitTable,
/bin/pingdiag, and /bin/sockdiag remain separate accepted surfaces until later
tasks explicitly adapt them to smoltcp. The selected next bounded task is
phase12-network-smoltcp-no-std-dependency-core-20260621, which may add the
dependency and minimal fail-closed source boundary but may not accept UDP/TCP
payload transport.

phase12-network-smoltcp-no-std-dependency-core-20260621 accepts
phase12-network-smoltcp-no-std-dependency-core-accepted. Cargo now pins
smoltcp 0.13.1 with default-features=false and only medium-ethernet,
proto-ipv4, and socket-tcp enabled. src/network.rs owns
SmoltcpDependencyCore as a Talos-first closed/no-device-bound source boundary
using smoltcp EthernetAddress, Ipv4Cidr, and TCP State symbols. The focused
source/unit test proves the dependency compiles and remains Closed/
NoDeviceBound until a later adapter task binds packet queues to smoltcp.

The accepted evidence level is source/unit plus host/QEMU-substitute only:
cargo fmt and the full cargo -Zjson-target-spec test suite passed with the
project QEMU path. This task does not accept TCP connection establishment,
UDP/TCP payload transport, packet movement through smoltcp, socket syscall
bridging, shell diagnostic expansion, live driver adapters, live packet I/O,
Pi 5 hardware behavior, lab mutation, boot publication, hardware reachability,
SSH, public stable socket ABI acceptance, broad socket expansion, or phase
transition. The selected next bounded task is
phase12-network-smoltcp-packet-device-adapter-core-20260621.

phase12-network-smoltcp-packet-device-adapter-core-20260621 accepts
phase12-network-smoltcp-packet-device-adapter-core-accepted. src/network.rs
now owns SmoltcpPacketDeviceAdapter as a host-only smoltcp phy::Device boundary
over PacketQueueNetworkDevice. The adapter keeps frame storage fixed-capacity,
maps receive outcomes to Received, NoFrame, TransmitQueueFull,
ReceiveBufferTooSmall, and ReceiveError, maps transmit outcomes to Ready,
Transmitted, TransmitQueueFull, FrameTooLarge, and TransmitError, and exposes
Ethernet DeviceCapabilities with a single-frame burst.

The accepted evidence level is source/unit plus host/QEMU-substitute only:
cargo fmt and the full cargo -Zjson-target-spec test suite passed with the
project QEMU path. Focused tests cover receive/reply frame movement,
no-frame behavior, transmit queue pressure without consuming receive frames,
device receive errors, transmit errors, and frame bounds. This task does not
accept TCP handshake behavior, UDP/TCP payload transport, socket syscall
bridging, shell diagnostic expansion, live driver adapters, live packet I/O,
Pi 5 hardware behavior, lab mutation, boot publication, hardware reachability,
SSH, public stable socket ABI acceptance, broad socket expansion, or phase
transition. The selected next bounded task is
phase12-network-smoltcp-loopback-tcp-handshake-core-20260621.

phase12-network-smoltcp-loopback-tcp-handshake-core-20260621 accepts
phase12-network-smoltcp-loopback-tcp-handshake-core-accepted. src/network.rs
now has host-only source/unit evidence that two smoltcp Ethernet interfaces,
TCP sockets, and the accepted SmoltcpPacketDeviceAdapter boundary can complete
one deterministic TCP three-way handshake over fixed packet queues.

The accepted handshake test owns both endpoints' MAC/IP configuration, socket
storage, TCP buffers, packet queues, and time progression explicitly. It
records a bounded two-step path with three client-to-server frames and two
server-to-client frames, ending with both sockets in Established. A companion
backpressure test proves that a zero-capacity client transmit queue leaves the
client in SynSent, the server in Listen, moves no frames, and records
TransmitQueueFull instead of claiming connection establishment.

The accepted evidence level is source/unit plus host/QEMU-substitute only:
cargo fmt and the full cargo -Zjson-target-spec test suite passed with the
project QEMU path. This task does not accept Talos socket syscall bridging,
/bin/sockdiag TCP diagnostics, retained smoke evidence, UDP/TCP payload
transport beyond the handshake state transition, live driver adapters, live
packet I/O, Pi 5 hardware behavior, lab mutation, boot publication, hardware
reachability, SSH, public stable socket ABI acceptance, broad socket expansion,
or phase transition. The selected next bounded task is
phase12-network-smoltcp-tcp-frontier-closeout-20260621.

phase12-network-smoltcp-tcp-frontier-closeout-20260621 accepts
phase12-network-smoltcp-tcp-frontier-closeout-accepted. The initial smoltcp/TCP
host-only frontier is now closed at source/unit plus host/QEMU-substitute
evidence only. The accepted boundary includes smoltcp 0.13.1 with no default
features, Talos-owned fixed packet queues, SmoltcpPacketDeviceAdapter, explicit
MAC/IP/time/socket storage ownership, deterministic TCP Established handshake
evidence, and a deterministic transmit-backpressure control.

This closeout rejects Talos socket syscall bridging, /bin/sockdiag TCP
diagnostics, retained smoke evidence, live driver adapters, live packet I/O,
Pi 5 hardware behavior, lab mutation, boot publication, hardware reachability,
SSH, public stable socket ABI acceptance, broad socket expansion, and phase
transition. planningNeeded=true; no later bounded bridge, socket, or network
task is mechanically unblocked without supervisor planning.

phase12-network-smoltcp-socket-bridge-contract-20260621 accepts
phase12-network-smoltcp-socket-bridge-contract-accepted. Supervisor planning
after the smoltcp/TCP frontier closeout selected the first private bridge from
descriptor-backed AF_INET stream sockets to the accepted host-only smoltcp TCP
frontier.

The contract keeps process-visible sockets as normal
DescriptorObjectKind::Socket entries owned by ProcessOwnerId descriptor
tables, preserves the accepted local socket rendezvous and poll-wait surfaces
as regression controls, and assigns bridge state to fixed-capacity
kernel-owned smoltcp Interface, SocketSet, TCP buffer, TCP handle, and packet
queue records. Time progression remains explicit and deterministic, and
SmoltcpPacketDeviceAdapter remains the only accepted smoltcp phy boundary for
this slice.

The selected implementation boundary is host-only/source-unit TCP behavior
through the existing private socket syscall dispatch for socket, bind, listen,
connect, accept, send, recv, poll, poll-wait, and close. It is private
experimental Talos behavior, not Linux syscall-number compatibility and not a
public stable socket ABI. Runtime implementation, shell-visible /bin/sockdiag
TCP diagnostics, retained smoke evidence, live driver adapters, live packet
I/O, hardware reachability, SSH, public stable socket ABI acceptance, broad
socket expansion, and phase transition remain rejected. The selected next
bounded task is phase12-network-smoltcp-socket-bridge-core-20260621.

phase12-network-smoltcp-socket-bridge-core-20260621 accepts
phase12-network-smoltcp-socket-bridge-core-accepted. src/network.rs now keeps
fixed-capacity SmoltcpSocketBridgeRecord entries beside the private
NetworkSocketDescriptorTable and records deterministic smoltcp Established
handshake plus payload-transfer observations for descriptor-backed
AF_INET/SOCK_STREAM connections.

The accepted path remains private and host-only. connect creates bridge
backing only after a bounded smoltcp TCP handshake over
SmoltcpPacketDeviceAdapter, fixed packet queues, fixed TCP buffers, and
explicit MAC/IP/time inputs reaches Established. send records a bounded
smoltcp payload transfer before the existing descriptor-backed recv queue is
made readable. close/drop removes matching bridge records while preserving the
accepted peer hangup/EPIPE readiness behavior. Focused source/unit evidence
exercises socket, bind, listen, connect, accept, send, recv, poll, poll-wait,
and close through the private syscall dispatch and process descriptor
ownership path.

The accepted evidence level is source/unit plus host/QEMU-substitute only:
cargo fmt and cargo -Zjson-target-spec test --quiet passed with the project
QEMU path, and the static test-case count is 693. This task does not accept a
shell-visible /bin/sockdiag TCP diagnostic, retained smoke transcript, live
driver adapter, live packet I/O, Pi 5 hardware behavior, lab mutation, boot
publication, hardware reachability, SSH, public stable socket ABI acceptance,
broad socket expansion, UDP/raw sockets, or phase transition. The selected
next bounded task is
phase12-network-shell-sockdiag-smoltcp-tcp-core-20260621.

phase12-network-shell-sockdiag-smoltcp-tcp-core-20260621 accepts
phase12-network-shell-sockdiag-smoltcp-tcp-core-accepted. The existing
shell-visible /bin/sockdiag VFS/userspace diagnostic now reports the private
host-only smoltcp TCP bridge frontier by executing the accepted socket,
bind, listen, connect, accept, send, recv, poll, poll-wait, and close syscall
path and reading SmoltcpSocketBridgeRecord evidence from the same
descriptor-backed connection.

The diagnostic records the smoltcp connection id, Established client/server
handshake states, deterministic handshake step/frame counters, accepted
descriptor attachment, one bounded payload-transfer observation, and
Established payload states. This remains source/unit plus
host/QEMU-substitute evidence only. It does not accept retained smoke
evidence, live packet I/O, hardware reachability, SSH, public stable socket
ABI acceptance, broad socket expansion, UDP/raw sockets, or phase transition.
The selected next bounded task is
phase12-network-shell-sockdiag-smoltcp-tcp-smoke-20260621.

phase12-network-shell-sockdiag-smoltcp-tcp-smoke-20260621 accepts
phase12-network-shell-sockdiag-smoltcp-tcp-smoke-accepted. The retained
host/QEMU-substitute smoke evidence records shell-visible /bin/sockdiag
reaching the private host-only smoltcp TCP bridge diagnostic through VFS
executable lookup/open/read, startup ABI, the existing private socket syscall
path, and descriptor-backed SmoltcpSocketBridgeRecord evidence.

The retained smoke covers Established smoltcp client/server handshake states,
deterministic handshake step/frame counters, accepted-descriptor attachment,
one bounded payload-transfer observation, Established payload states, waitpid,
laststatus, malformed arguments, missing executable identity, unchanged local
socket diagnostics, unchanged /bin/pingdiag, and unchanged bounded syscall
vocabulary. Evidence remains host/QEMU-substitute only. Live packet I/O, Pi 5
hardware behavior, hardware reachability, lab mutation, boot publication,
generated-root publication, SSH, public stable socket ABI acceptance, broad
socket expansion, UDP/raw sockets, and phase transition remain rejected. The
selected next bounded task is
phase12-network-shell-sockdiag-smoltcp-tcp-closeout-20260621.

phase12-network-shell-sockdiag-smoltcp-tcp-closeout-20260621 accepts
phase12-network-shell-sockdiag-smoltcp-tcp-closeout-accepted. The
shell-visible smoltcp TCP diagnostic frontier is now closed at source/unit
plus retained host/QEMU-substitute smoke evidence over VFS/userspace
/bin/sockdiag execution and the private descriptor-backed host-only smoltcp TCP
bridge only.

The accepted boundary covers VFS executable lookup/open/read, startup ABI,
private socket/bind/listen/connect/accept/send/recv/poll/poll-wait/close
dispatch, process descriptor ownership, SmoltcpSocketBridgeRecord reporting,
Established client/server handshake states, deterministic step/frame counters,
accepted-descriptor attachment, one bounded payload-transfer observation,
waitpid, laststatus, deterministic controls, unchanged local socket
diagnostics, unchanged /bin/pingdiag, and bounded syscall vocabulary. This
closeout rejects live driver adapters, live packet I/O, Pi 5 hardware
behavior, lab mutation, boot publication, generated-root publication, hardware
reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, UDP/raw sockets, and phase transition. planningNeeded=true; no
later bounded socket/network task is mechanically unblocked without supervisor
planning.

phase12-network-socket-userspace-abi-contract-20260621 accepts
phase12-network-socket-userspace-abi-contract-accepted. Supervisor planning
after the shell-visible smoltcp TCP closeout selected a documentation-only
contract for the private Talos userspace socket ABI before any wrapper or
runtime expansion.

The accepted contract records the stable trap shape svc #0, x8 selector,
x0 through x5 scalar arguments, x0 return, and negative x0 errno encoding for
the current socket surface. The private selector vocabulary is TALOS_CLOSE=2,
TALOS_SOCKET=6, TALOS_BIND=7, TALOS_LISTEN=8, TALOS_CONNECT=9,
TALOS_ACCEPT=10, TALOS_SEND=11, TALOS_RECV=12, TALOS_POLL=13, and
TALOS_POLL_WAIT=14. The accepted socket subset remains AF_INET=2,
SOCK_STREAM=1, protocol=0, backlog 1..=4, 64-byte bounded payload queues, and
16-byte little-endian poll entries carrying fd/events/revents with at most
eight entries and a 1024-tick bounded wait. send, recv, poll, and poll_wait
use the accepted user-memory copy helpers.

The evidence level is static source/task/docs/evidence review tied to prior
source/unit and retained host/QEMU-substitute evidence. The contract rejects
runtime implementation, live driver adapters, live packet I/O, Pi 5 hardware
behavior, lab mutation, boot publication, generated-root publication, hardware
reachability, SSH, UDP/raw sockets, libc/std socket wrappers, POSIX/Linux
compatibility, public stable socket ABI acceptance, broad socket expansion,
and phase transition. The selected next bounded task is
phase12-network-socket-userspace-abi-core-20260621.

phase12-network-socket-userspace-abi-core-20260621 accepts
phase12-network-socket-userspace-abi-core-accepted. src/userspace_socket_abi.rs
now compiles the private Talos no_std socket ABI helper surface against the
accepted contract. The helper mirrors the private selector vocabulary,
AF_INET/SOCK_STREAM/protocol constants, bounded poll entry layout, poll/wait
limits, and errno values, and provides const wrapper constructors for socket,
bind, listen, connect, accept, send, recv, poll, poll_wait, and close.

Focused source/unit evidence routes wrapper-built calls through the accepted
socket-table-aware dispatch and reaches the host-only SmoltcpSocketBridgeRecord
path: connect records Established client/server handshake state, accept records
accepted descriptor attachment, send records one bounded smoltcp payload
transfer, and recv returns the payload through the descriptor-backed queue. The
accepted evidence level remains source/unit plus host/QEMU-substitute only.
This task rejects live driver adapters, live packet I/O, Pi 5 hardware
behavior, lab mutation, boot publication, generated-root publication, hardware
reachability, SSH, UDP/raw sockets, libc/std socket wrappers, POSIX/Linux
compatibility, public stable socket ABI acceptance, broad socket expansion,
and phase transition. The selected next bounded task is
phase12-network-shell-sockdiag-userspace-abi-core-20260621.

phase12-network-shell-sockdiag-userspace-abi-core-20260621 accepts
phase12-network-shell-sockdiag-userspace-abi-core-accepted. The shell-visible
/bin/sockdiag VFS/userspace TCP diagnostic now routes its accepted success path
through the documented private src/userspace_socket_abi.rs helper surface
before invoking the existing descriptor-backed socket-table dispatch.

The diagnostic uses userspace_socket_abi wrapper constructors for socket, bind,
listen, connect, accept, send, recv, poll, poll_wait, and close, and uses the
ABI PollEntry codec for the 16-byte little-endian fd/events/revents user-memory
layout. Its shell output records userspace-socket-abi-v1 in the diagnostic
source while preserving the accepted Established smoltcp client/server
handshake states, deterministic step/frame counters, accepted descriptor
attachment, one bounded payload-transfer observation, waitpid, laststatus,
local socket controls, /bin/pingdiag controls, and bounded syscall vocabulary.

The accepted evidence level remains source/unit plus host/QEMU-substitute cargo
test only. This task rejects retained smoke evidence, live driver adapters,
live packet I/O, Pi 5 hardware behavior, lab mutation, boot publication,
generated-root publication, hardware reachability, SSH, UDP/raw sockets,
libc/std socket wrappers, POSIX/Linux compatibility, public stable socket ABI
acceptance, broad socket expansion, and phase transition. The selected next
bounded task is
phase12-network-shell-sockdiag-userspace-abi-smoke-20260621.

phase12-network-shell-sockdiag-userspace-abi-smoke-20260621 accepts
phase12-network-shell-sockdiag-userspace-abi-smoke-accepted. The retained
host/QEMU-substitute smoke evidence records shell-visible /bin/sockdiag over
VFS executable lookup/open/read, startup ABI, the documented private
userspace_socket_abi helper constructors, ABI PollEntry layout, and the
existing descriptor-backed socket dispatch.

The retained transcript proves userspace-socket-abi-v1 shell output reaches
the accepted host-only smoltcp TCP diagnostic with Established client/server
handshake states, deterministic frame/step counters, accepted descriptor
attachment, one bounded payload-transfer observation, waitpid, laststatus,
malformed/missing executable controls, unchanged local socket diagnostics,
unchanged /bin/pingdiag behavior, ABI constant/wrapper coverage, and bounded
syscall vocabulary.

The accepted evidence level remains host/QEMU-substitute only. This task
rejects source behavior changes, live driver adapters, live packet I/O, Pi 5
hardware behavior, lab mutation, boot publication, generated-root publication,
hardware reachability, SSH, UDP/raw sockets, broad socket expansion,
POSIX/Linux compatibility, public stable socket ABI acceptance, and phase
transition. The selected next bounded task is
phase12-network-shell-sockdiag-userspace-abi-closeout-20260621.

phase12-network-shell-sockdiag-userspace-abi-closeout-20260621 accepts
phase12-network-shell-sockdiag-userspace-abi-closeout-accepted. The documented
private userspace socket ABI frontier is now closed at source/unit plus
retained host/QEMU-substitute evidence over shell-visible VFS/userspace
/bin/sockdiag execution through userspace_socket_abi helper constructors and
the existing descriptor-backed host-only smoltcp TCP bridge.

The accepted boundary covers read-only VFS executable lookup/open/read,
startup ABI, svc #0/x8 private selector shape, x0..x5 scalar arguments,
negative errno returns, AF_INET/SOCK_STREAM constants, socket/bind/listen/
connect/accept/send/recv/poll/poll-wait/close wrappers, 16-byte PollEntry
fd/events/revents layout, process descriptor ownership, Established smoltcp
client/server states, deterministic frame/step counters, accepted descriptor
attachment, one bounded payload-transfer observation, waitpid, laststatus,
deterministic controls, unchanged local socket diagnostics, unchanged
/bin/pingdiag behavior, ABI constant/wrapper coverage, and bounded syscall
vocabulary.

This closeout rejects live driver adapters, live packet I/O, Pi 5 hardware
behavior, lab mutation, boot publication, generated-root publication, hardware
reachability, SSH, UDP/raw sockets, libc/std socket wrappers, POSIX/Linux
compatibility, public stable socket ABI acceptance, broad socket expansion, and
phase transition. planningNeeded=true; no later bounded socket/network task is
mechanically unblocked without supervisor planning.

phase12-network-driver-packet-adapter-contract-20260621 accepts
phase12-network-driver-packet-adapter-contract-accepted. Supervisor planning
after the userspace socket ABI closeout selected a contract-only slice for the
next driver packet adapter substrate before any live packet I/O, hardware
reachability, SSH, UDP/raw sockets, public ABI stabilization, or phase
transition.

The contract ties the accepted PacketQueueNetworkDevice driver packet pump,
SmoltcpPacketDeviceAdapter, private descriptor-backed smoltcp TCP bridge,
userspace_socket_abi helper surface, /bin/pingdiag controls, and /bin/sockdiag
diagnostics into one host-only source/unit implementation target. The future
core must keep copied fixed-capacity PacketQueueFrame records as the only
accepted frame ownership boundary, with compile-time RX/TX/frame capacities,
explicit Full and FrameTooLarge behavior, deterministic DeviceError mapping,
caller-supplied smoltcp::time::Instant progression, and no ambient hardware
time dependency.

The selected core may inject one driver RX frame into the accepted smoltcp
packet-device path, observe one smoltcp-produced TX frame, and prove
backpressure/error behavior through source/unit tests. Later shell diagnostics
may report adapter RX/TX queue counts, last RX/TX result names, smoltcp bridge
continuity, waitpid, and laststatus only through the accepted VFS/userspace
private socket ABI path.

The evidence level is static source/task/docs/evidence review only. Runtime
implementation, live driver programming, live packet I/O, Pi 5 hardware
behavior, lab mutation, boot publication, generated-root publication, hardware
reachability, SSH, UDP/raw sockets, libc/std socket wrappers, POSIX/Linux
compatibility, public stable ABI acceptance, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-driver-packet-adapter-core-20260621.

phase12-network-driver-packet-adapter-core-20260621 accepts
phase12-network-driver-packet-adapter-core-accepted. The source/unit core adds
DriverPacketAdapter as the host-only driver packet adapter substrate over the
accepted SmoltcpPacketDeviceAdapter and PacketQueueNetworkDevice boundaries.

DriverPacketAdapter keeps copied fixed-capacity PacketQueueFrame records as
the only frame ownership boundary. Driver RX input enters through
inject_driver_rx, smoltcp consumes at most one RX token through
receive_one_for_smoltcp with a caller-supplied smoltcp::time::Instant, smoltcp
TX output enters through transmit_one_from_smoltcp, and driver-visible TX
records leave through pop_driver_tx. Source/unit coverage proves copied RX/TX
movement, TX backpressure preserving a queued RX frame, Full and FrameTooLarge
capacity behavior, and deterministic DeviceError mapping.

The accepted evidence level remains source/unit plus host/QEMU-substitute only.
This core preserves the accepted local socket, smoltcp TCP bridge, poll/wait,
userspace_socket_abi, /bin/pingdiag, and /bin/sockdiag regression surfaces
through the full cargo test gate. Shell diagnostic observation, retained smoke
evidence, live driver programming, live packet I/O, Pi 5 hardware behavior,
lab mutation, boot publication, generated-root publication, hardware
reachability, SSH, UDP/raw sockets, libc/std socket wrappers, POSIX/Linux
compatibility, public stable ABI acceptance, broad socket expansion, and phase
transition remain rejected. The selected next bounded task is
phase12-network-shell-sockdiag-driver-packet-adapter-core-20260621.

phase12-network-shell-sockdiag-driver-packet-adapter-core-20260621 accepts
phase12-network-shell-sockdiag-driver-packet-adapter-core-accepted. The
shell-visible /bin/sockdiag VFS/userspace diagnostic now reports deterministic
DriverPacketAdapter state through the accepted private userspace_socket_abi and
descriptor-backed socket dispatch path.

The diagnostic records one driver RX frame consumed by the smoltcp packet-device
boundary, one smoltcp-produced TX frame observed by the driver side, the
post-pop TX queue state, and a separate TX-queue-full backpressure step that
preserves one queued RX frame. The same /bin/sockdiag line still reports VFS
executable lookup/open/read, startup ABI, private socket/bind/listen/connect/
accept/send/recv/poll/poll-wait/close behavior, cross-process local rendezvous,
private smoltcp TCP bridge continuity, waitpid, laststatus, malformed argument
and missing executable controls, unchanged local socket diagnostics, unchanged
/bin/pingdiag behavior, and bounded syscall vocabulary.

The accepted evidence level remains source/unit plus host/QEMU-substitute cargo
test only and is explicitly labeled host-qemu-substitute-not-live-packet-io.
Retained smoke evidence, live driver programming, live packet I/O, Pi 5
hardware behavior, lab mutation, boot publication, generated-root publication,
hardware reachability, SSH, UDP/raw sockets, libc/std socket wrappers,
POSIX/Linux compatibility, public stable ABI acceptance, broad socket
expansion, and phase transition remain rejected. The selected next bounded task
is phase12-network-shell-sockdiag-driver-packet-adapter-smoke-20260621.

phase12-network-shell-sockdiag-driver-packet-adapter-smoke-20260621 accepts
phase12-network-shell-sockdiag-driver-packet-adapter-smoke-accepted. Retained
host/QEMU-substitute smoke evidence now archives shell-visible /bin/sockdiag
execution over the deterministic DriverPacketAdapter substrate through the
accepted VFS/userspace/private socket ABI path.

The smoke records VFS executable lookup/open/read, startup ABI,
userspace_socket_abi wrapper dispatch, descriptor-backed socket dispatch,
private smoltcp TCP bridge continuity, deterministic driver RX consumption,
smoltcp TX observation and driver-side pop, TX backpressure preserving queued
RX, capacity/error controls, waitpid, laststatus, malformed/missing executable
controls, unchanged local socket diagnostics, unchanged /bin/pingdiag
behavior, and bounded syscall vocabulary.

The accepted evidence level remains retained host/QEMU-substitute only. This
task rejects source behavior changes, live driver programming, live packet I/O,
Pi 5 hardware behavior, lab mutation, boot publication, generated-root
publication, hardware reachability, SSH, UDP/raw sockets, libc/std socket
wrappers, POSIX/Linux compatibility, public stable ABI acceptance, broad socket
expansion, and phase transition. The selected next bounded task is
phase12-network-driver-packet-adapter-closeout-20260621.

phase12-network-driver-packet-adapter-closeout-20260621 accepts
phase12-network-driver-packet-adapter-closeout-accepted. The closeout freezes
the driver packet adapter slice at source/unit plus retained
host/QEMU-substitute evidence over the accepted VFS/userspace/private socket
ABI path.

The accepted boundary is deterministic copied PacketQueueFrame ownership:
inject_driver_rx supplies adapter RX input, receive_one_for_smoltcp consumes at
most one RX token with caller-supplied smoltcp time, transmit_one_from_smoltcp
records at most one smoltcp-produced TX frame, and pop_driver_tx exposes the
driver-visible TX record. /bin/sockdiag observes that substrate through VFS
executable lookup/open/read, startup ABI, userspace_socket_abi wrappers,
descriptor-backed socket dispatch, private smoltcp TCP bridge continuity,
waitpid, laststatus, deterministic controls, unchanged local socket
diagnostics, unchanged /bin/pingdiag behavior, and bounded syscall vocabulary.

The remaining gaps are real hardware RX/TX coupling, packet scheduling and
backpressure on live hardware, Pi 5 reachability evidence, SSH strategy,
entropy, host keys, service shape, exposure controls, libc/std socket wrappers,
UDP/raw sockets, and any public ABI/POSIX/Linux compatibility claim. Live
driver programming, live packet I/O, hardware reachability, SSH, UDP/raw
sockets, libc/std wrappers, POSIX/Linux compatibility, public stable socket ABI
acceptance, broad socket expansion, and phase transition remain rejected.
selected_next_task=null and planningNeeded=true pending supervisor planning for
any next bounded socket/network task.

phase12-network-frontier-pause-and-ssh-strategy-checkpoint-20260621 accepts
phase12-network-frontier-pause-and-ssh-strategy-checkpoint-accepted. This
checkpoint pauses live Ethernet hardware expansion after the accepted
BCM54213PE lifecycle proof terminal remained no-change link-not-ready and the
link-ready discriminator source contract selected no defensible discriminator.

The accepted host-only frontier remains the descriptor-backed socket substrate,
private userspace_socket_abi helpers, host-only smoltcp TCP bridge diagnostics,
and deterministic DriverPacketAdapter evidence through shell-visible
/bin/sockdiag. That evidence is still source/unit plus retained
host/QEMU-substitute only, not live packet I/O or hardware reachability.

The next bounded frontier moves to SSH-enabling strategy prerequisites:
entropy, key management, service shape, and exposure controls. This checkpoint
selects phase12-entropy-ssh-strategy-contract-20260621 and rejects stale
generic link-ready discriminator promotion, live link-ready, packet I/O,
hardware reachability, SSH service acceptance, public ABI/POSIX/Linux
compatibility, broad socket expansion, and phase transition.

phase12-entropy-ssh-strategy-contract-20260621 accepts
phase12-entropy-ssh-strategy-contract-accepted. The selected SSH-enabling
strategy is prerequisite-first: establish a Talos-owned entropy source contract
and diagnostic frontier before adopting crypto dependencies, generating host
keys, porting an SSH server, or accepting any SSH service behavior.

OpenSSH remains the long-term compatibility target, but it is not the near-term
first implementation target. A future existing-server port is preferred once
entropy, key storage/provisioning, crypto dependencies, process/service
plumbing, and libc/std constraints are clear; a smaller Talos-first Rust SSH
service remains deferred as a practical first service option if OpenSSH's
assumptions are still too large.

The next bounded task is phase12-entropy-source-contract-20260621. That task
must name accepted/rejected entropy input classes, deterministic controls, and
fail-closed labels for a later diagnostic. This strategy contract accepts no
runtime implementation, dependency adoption, crypto implementation, host key
generation, SSH service, live packet I/O, hardware reachability, public
ABI/POSIX/Linux compatibility, broad socket expansion, or phase transition.

phase12-entropy-source-contract-20260621 accepts
phase12-entropy-source-contract-accepted. The accepted entropy frontier is a
diagnostic/classification contract, not a random-byte generator. The next
implementation may classify source-grounded local input candidates: generic
timer counter/tick samples, scheduler or process event observations once
explicitly exposed to the diagnostic, console/serial input timing deltas when
paired with timer samples, and future operator-provisioned seed material.

The contract rejects deterministic boot constants, DTB addresses, kernel
layout, initramfs contents, generated-root manifests, fixed task IDs, fixed
socket diagnostic payloads, lab API metadata, TFTP byte counts, serial
transcripts, lab-provided randomness, external randomness services, and
hardware RNG claims without source-grounded proof. The required diagnostic
labels are entropydiag-fail-closed-no-input,
entropydiag-deterministic-control, entropydiag-untrusted-timer-only,
entropydiag-untrusted-local-mix, entropydiag-operator-seed-required, and
entropydiag-hardware-rng-unaccepted. The selected next bounded task is
phase12-entropydiag-core-20260621. This contract accepts no cryptographic
strength, SSH readiness, hardware randomness, live packet I/O, reachability,
public ABI/POSIX/Linux compatibility, broad expansion, or phase transition.

phase12-entropydiag-core-20260621 accepts
phase12-entropydiag-core-accepted. Talos now has a source/unit entropy
diagnostic classifier over explicit caller-supplied observations. The accepted
snapshot inputs are optional timer samples, scheduler-event samples,
console-timing samples, operator-provisioned seed metadata, deterministic test
control state, and a hardware-RNG-observed flag that remains rejected as a
randomness source.

The internal diagnostic command channel now includes entropy, which reports the
default no-input fail-closed baseline:
entropydiag-fail-closed-no-input, entropydiag-hardware-rng-unaccepted,
entropydiag-operator-seed-required, cryptographic-strength false, and
ssh-ready false. Source/unit tests prove fixed timer-only input,
fixed local-event mixes, fixed deterministic seed controls, hardware RNG
rejection, and local-input-without-seed operator-seed-required behavior.

The accepted evidence level is source/unit plus full host/QEMU-substitute no_std
test coverage. The implementation does not sample ambient hardware or lab state,
generate random bytes, derive keys, persist seeds, expose a public ABI, or claim
SSH readiness. Host key generation, crypto RNG/DRBG selection, seed persistence,
authorized-key storage, service lifecycle, SSH service behavior, hardware
randomness, live packet I/O, reachability, broad expansion, and phase transition
remain rejected. The selected next bounded task is
phase12-entropy-ssh-strategy-closeout-20260621.

phase12-entropy-ssh-strategy-closeout-20260621 accepts
phase12-entropy-ssh-strategy-closeout-accepted-planning-needed. The closeout
reconciles the accepted prerequisite-first SSH strategy, entropy source
contract, and source/unit entropy diagnostic implementation. Talos now has a
documented fail-closed entropy diagnostic frontier, but it still has no
accepted cryptographic entropy, random-byte generation, seed persistence,
host-key generation, crypto dependency integration, SSH service, live transport
reachability, public ABI/POSIX/Linux compatibility, or phase transition.

Supervisor planning selected the next bounded Phase 12.5 task:
phase12-ssh-key-management-readiness-contract-20260621. The stale link-ready
discriminator chain remains blocked by missing selected discriminator and
selected_next_task evidence.

phase12-ssh-key-management-readiness-contract-20260621 accepts
phase12-ssh-key-management-readiness-contract-accepted. The contract defines a
fail-closed SSH key-management readiness classifier boundary over explicit
prerequisite states. It preserves the accepted entropy diagnostic baseline and
names the next diagnostic labels: sshkeydiag-missing-host-key,
sshkeydiag-missing-authorized-key, sshkeydiag-entropy-unready,
sshkeydiag-seed-material-missing, sshkeydiag-seed-material-insufficient,
sshkeydiag-persistence-unavailable, sshkeydiag-exposure-disabled, and
sshkeydiag-not-ready.

The default aggregate result remains not ready. The future implementation may
only prove deterministic negative controls over supplied metadata; it may not
read or persist secret material, generate keys, derive keys, import crypto/SSH
dependencies, inspect ambient hardware/lab state, expose a public ABI, or start
an SSH service. A narrow entropy test assertion was corrected so the no-input
case checks the hardware-RNG rejection through hardware_rng_label() while
preserving entropydiag-fail-closed-no-input as the input label. The selected
next bounded task is phase12-sshkeydiag-core-20260621.

phase12-sshkeydiag-core-20260621 accepts
phase12-sshkeydiag-core-accepted. Talos now has a source/unit SSH
key-readiness classifier over explicit caller-supplied prerequisite metadata:
host-key metadata, authorized-key metadata, entropy diagnostic report, seed
material metadata, persistence metadata, and exposure state.

The internal diagnostic command channel now includes sshkeydiag, which reports
the default fail-closed baseline:
sshkeydiag-not-ready, sshkeydiag-missing-host-key,
sshkeydiag-missing-authorized-key, sshkeydiag-entropy-unready,
sshkeydiag-seed-material-missing, sshkeydiag-persistence-unavailable,
sshkeydiag-exposure-disabled, and ssh-ready false. Focused source/unit tests
prove all-missing defaults, deterministic-control entropy rejection, untrusted
local entropy without seed material, missing versus insufficient seed material,
independent persistence and exposure blockers, and key-metadata negative
controls.

The accepted evidence level is source/unit plus full host/QEMU-substitute
no_std test coverage. The implementation does not read or persist secret
material, generate keys, derive keys, parse authorized key material, import
crypto/SSH dependencies, inspect ambient hardware or lab state, expose a public
ABI, or start/accept an SSH service. Host-key generation or provisioning,
authorized-key storage, seed persistence, crypto RNG/DRBG selection, service
lifecycle, authentication policy, live packet I/O, hardware reachability,
public ABI/POSIX/Linux compatibility, broad expansion, and phase transition
remain rejected. The selected next bounded task is
phase12-shell-sshkeydiag-smoke-20260621.

phase12-shell-sshkeydiag-smoke-20260621 accepts
phase12-shell-sshkeydiag-smoke-accepted. Talos now retains
host/QEMU-substitute smoke evidence for the shell-visible/internal diagnostic
path to sshkeydiag. The retained transcript exercises the accepted
metadata-only fail-closed SSH key-readiness surface and records the default
not-ready labels: sshkeydiag-not-ready, sshkeydiag-missing-host-key,
sshkeydiag-missing-authorized-key, sshkeydiag-entropy-unready,
sshkeydiag-seed-material-missing, sshkeydiag-persistence-unavailable,
sshkeydiag-exposure-disabled, and ssh-ready false.

The smoke also preserves the accepted entropy diagnostic boundary:
entropydiag-fail-closed-no-input, entropydiag-hardware-rng-unaccepted,
entropydiag-operator-seed-required, cryptographic-strength false, and
ssh-ready false. The accepted evidence level remains retained
host/QEMU-substitute only. It does not accept host key generation, secret
persistence, crypto/SSH dependency adoption, SSH service behavior, live packet
I/O, hardware reachability, public ABI/POSIX/Linux compatibility, broad
expansion, or phase transition. The selected next bounded task is
phase12-ssh-key-management-readiness-closeout-20260621.

phase12-ssh-key-management-readiness-closeout-20260621 accepts
phase12-ssh-key-management-readiness-closeout-accepted-planning-needed. The
closeout reconciles the accepted Phase 12.5 prerequisite slice: a fail-closed
SSH key-management readiness contract, the source/unit sshkeydiag classifier
and internal diagnostic command, and retained host/QEMU-substitute smoke
evidence for the default not-ready state.

The accepted frontier remains metadata-only and diagnostic. Talos can report
that SSH key management is not ready because host-key metadata, authorized-key
metadata, accepted entropy/seed material, persistence, and explicit exposure
are absent or disabled. Talos still does not have accepted cryptographic
entropy, random-byte generation, seed persistence, host-key generation or
provisioning, authorized-key storage, crypto/SSH dependency integration, SSH
service behavior, live packet I/O, hardware reachability, public
ABI/POSIX/Linux compatibility, broad expansion, or phase transition. Supervisor
planning is required before any next Phase 12.5 key-management, entropy-source,
crypto, service, or exposure-control task; the stale link-ready discriminator
chain remains blocked by missing selected discriminator and selected_next_task
evidence.

phase12-operator-seed-vfs-contract-20260621 accepts
phase12-operator-seed-vfs-contract-accepted. The accepted operator seed
frontier is a read-only VFS/initramfs diagnostic contract, not a random-byte
generator or writable seed store. The optional operator-provisioned seed file
is /etc/talos/operator-seed.bin, a regular file of opaque raw bytes. The first
implementation may expose only metadata to diagnostics: missing/invalid,
insufficient, or sufficient-length state and byte length. It must not print,
retain, derive, digest, fingerprint, or otherwise expose actual seed bytes or
cross-boot comparable secret identifiers.

The sufficient metadata threshold is 32 bytes, with a first-slice diagnostic
read limit of 4096 bytes. Missing seed material preserves
entropydiag-operator-seed-required and sshkeydiag-seed-material-missing.
Lengths 1 through 31 are insufficient and map to
sshkeydiag-seed-material-insufficient. Lengths 32 through 4096 may clear only
the seed-material prerequisite; cryptographic-strength remains false and
ssh-ready remains false. Invalid objects such as directories, unsupported VFS
objects, unreadable objects, malformed VFS state, and oversized files remain
not ready and may be classified as insufficient until a later contract adds a
dedicated invalid label.

The accepted evidence level is static source/docs/evidence review plus docs
validation. This contract does not accept cryptographic entropy, random-byte
generation, CSPRNG/conditioning, host-key generation or provisioning,
authorized-key storage, writable seed persistence, crypto/SSH dependency
adoption, SSH service behavior, live packet I/O, hardware reachability, public
ABI/POSIX/Linux compatibility, broad expansion, stale link-ready discriminator
promotion, or phase transition. The selected next bounded task is
phase12-operator-seed-vfs-core-20260621.

phase12-operator-seed-vfs-core-20260621 accepts
phase12-operator-seed-vfs-core-accepted. Talos now has source/unit
VFS-backed operator seed material classification for the accepted
/etc/talos/operator-seed.bin contract. The implementation exposes only
metadata: missing, invalid, insufficient, or sufficient state plus byte length.
It does not read, print, retain, digest, fingerprint, or otherwise expose seed
bytes. The default Phase 8 initramfs fixture remains missing and preserves the
existing fail-closed entropydiag and sshkeydiag command outputs.

The entropy classifier can build a snapshot from read-only initramfs metadata:
missing and invalid seed material keep entropydiag-operator-seed-required,
lengths 1 through 31 are present but insufficient, and lengths 32 through 4096
clear only the operator-seed-required indication while cryptographic-strength
and ssh-ready remain false. The SSH key-readiness classifier maps missing seed
metadata to sshkeydiag-seed-material-missing, invalid or insufficient metadata
to sshkeydiag-seed-material-insufficient, and sufficient metadata to clearing
only the seed-material label. Host-key, authorized-key, persistence/exposure,
cryptographic entropy, crypto/SSH service, and reachability prerequisites
remain unaccepted.

The accepted evidence level is source/unit tests plus the full no_std suite:
operator seed focused tests and cargo -Zjson-target-spec test both passed with
717 tests. This implementation does not accept random-byte generation,
CSPRNG/conditioning, cryptographic-strength, host-key generation or
provisioning, authorized-key storage, writable seed persistence, crypto/SSH
dependency adoption, SSH service behavior, live packet I/O, hardware
reachability, public ABI/POSIX/Linux compatibility, broad expansion, stale
link-ready discriminator promotion, or phase transition. The selected next
bounded task is phase12-shell-operator-seed-diag-smoke-20260621.

phase12-shell-operator-seed-diag-smoke-20260621 accepts
phase12-shell-operator-seed-diag-smoke-accepted. Talos now retains
host/QEMU-substitute shell-visible diagnostic evidence for the read-only VFS
operator seed metadata path. A metadata-aware diagnostic dispatch helper can
format entropydiag and sshkeydiag output from an explicit initramfs fixture
while the default diagnostic command path remains fail-closed for the Phase 8
fixture.

The retained transcript covers missing, insufficient, and sufficient operator
seed metadata without printing seed bytes, digests, fingerprints, or derived
material. Missing seed material reports entropydiag-operator-seed-required and
sshkeydiag-seed-material-missing. Insufficient seed material reports
sshkeydiag-seed-material-insufficient. Sufficient metadata clears only the
seed-material label; cryptographic-strength and ssh-ready remain false because
cryptographic entropy, host-key metadata, authorized-key metadata, persistence,
exposure, crypto/SSH service, and reachability prerequisites remain
unaccepted. This smoke does not accept random-byte generation,
CSPRNG/conditioning, host-key generation or provisioning, authorized-key
storage, writable seed persistence, crypto/SSH dependency adoption, SSH service
behavior, live packet I/O, hardware reachability, public ABI/POSIX/Linux
compatibility, broad expansion, stale link-ready discriminator promotion, or
phase transition. The selected next bounded task is
phase12-operator-seed-vfs-closeout-20260621.

phase12-operator-seed-vfs-closeout-20260621 accepts
phase12-operator-seed-vfs-closeout-accepted. The operator seed material slice is
now closed at the read-only diagnostic metadata boundary. Talos can classify
/etc/talos/operator-seed.bin as missing, invalid, insufficient, or
sufficient-length metadata for entropydiag and sshkeydiag without exposing seed
bytes, digests, fingerprints, derived material, or cross-boot comparable secret
identifiers.

This closeout reconciles the accepted contract, source/unit implementation,
retained shell-visible smoke transcript, docs, deferred work, and rejected
claims. The accepted frontier remains diagnostic-only: sufficient operator seed
metadata can clear only the operator-seed-required and seed-material diagnostic
labels while cryptographic-strength and ssh-ready remain false. Talos still
does not accept random-byte generation, CSPRNG/conditioning, host-key
generation or provisioning, authorized-key storage, writable seed persistence,
crypto/SSH dependency adoption, SSH service behavior, live transport, hardware
reachability, public ABI/POSIX/Linux compatibility, broad expansion, stale
link-ready discriminator promotion, or phase transition. No explicit queued
Phase 12.5 prerequisite task is mechanically unblocked after this closeout;
supervisor planning is required before the next crypto, host-key, persistence,
SSH service, live transport, hardware reachability, or phase-transition task.

phase12-operator-seed-secret-material-contract-20260621 accepts
phase12-operator-seed-secret-material-contract-accepted. The operator seed path
remains /etc/talos/operator-seed.bin in the accepted read-only
VFS/generated-root model, but the secret-material boundary is now narrower than
the diagnostic metadata path. Only a future Talos-owned CSPRNG
seed-conditioning/internal RNG component may read seed bytes, and only as
bounded seed input from a regular file of 32 through 4096 bytes.

Diagnostics, shell output, serial logs, task evidence, and public surfaces stay
metadata-only: path, missing/invalid/insufficient/sufficient state, byte
length, length bucket, redaction labels, and fail-closed readiness labels.
They must not expose seed bytes, partial bytes, generated random bytes, CSPRNG
state, actual digests, actual fingerprints, key-derivation output, or any
stable identifier that can compare real operator secrets across boots. Missing,
invalid, insufficient, zero-length, unreadable, malformed, or oversized inputs
fail closed. Source/unit tests may use deterministic public fixtures for length
and redaction behavior, but real operator seed bytes must not be retained in
repository evidence.

This contract still does not accept crypto dependency adoption, random-byte
generation, cryptographic-strength, host-key generation or provisioning,
authorized-key storage, writable persistence, SSH service behavior, live
transport, hardware reachability, public ABI/POSIX/Linux compatibility, broad
expansion, stale link-ready discriminator promotion, or phase transition. The
selected next bounded task is
phase12-csprng-dependency-selection-contract-20260621.

phase12-csprng-dependency-selection-contract-20260621 accepts
phase12-csprng-dependency-selection-contract-accepted. The selected no_std
CSPRNG dependency strategy for the next implementation slice is RustCrypto
chacha20 0.10.0 with default-features=false and features rng,zeroize, plus
direct zeroize 1.8.1 with default-features=false for Talos-owned temporary
seed/output buffers.

The future core must wrap chacha20::ChaCha20Rng privately and use only the
accepted operator seed secret-material path as seed input after bounded
conditioning. The dependency strategy rejects std/default features, host OS RNG
or getrandom/SysRng, network access, ambient lab randomness, serde, general
stream-cipher exposure, and public state/seed serialization. Source inspection
found that dependency state/seed helper methods exist, so Talos must not expose
or call them through diagnostics, shell output, serial logs, task evidence, or
public ABI surfaces.

The future internal CSPRNG API boundary is metadata-only until ready: accepted
inputs are read-only VFS metadata plus at most 4096 operator seed bytes; error
states fail closed for missing, invalid, insufficient, or conditioning-failed
input; bounded random-byte output is only through caller-provided buffers after
ready state. Retained evidence may name public fixtures, lengths, labels, and
validation commands, but not real seed bytes, generated bytes, RNG state,
digests, fingerprints, serialized state, stream identifiers, or comparable
secret identifiers. This contract does not implement random-byte generation,
accept cryptographic-strength, generate/provision host keys, store authorized
keys, add writable persistence, start SSH service behavior, prove live
transport/hardware reachability, broaden scope, promote stale link-ready
discriminator work, or transition phase. The selected next bounded task is
phase12-operator-seeded-csprng-core-20260621.

phase12-operator-seeded-csprng-core-20260621 accepts
phase12-operator-seeded-csprng-core-accepted. Talos now has the first
operator-seeded internal CSPRNG boundary. The implementation adds the selected
no_std dependencies, RustCrypto chacha20 0.10.0 with default-features=false and
features rng,zeroize plus direct zeroize 1.8.1 with default-features=false. The
new csprng module privately wraps chacha20::ChaCha20Rng and exposes only a
Talos-owned readiness/error report plus bounded caller-provided output after
ready state.

The CSPRNG core reads raw bytes only through the accepted
/etc/talos/operator-seed.bin boundary. Missing, invalid, zero-length,
oversized, and insufficient seed inputs fail closed and preserve not-ready
metadata. Sufficient 32 through 4096 byte inputs are bounded, conditioned into
a private 32-byte ChaCha seed, and clear only the accepted CSPRNG/entropy
metadata prerequisite. The diagnostics model can now represent
cryptographic-strength true when an internal CSPRNG readiness report is ready,
but ssh-ready remains false until host-key, authorized-key,
persistence/exposure, service, and reachability prerequisites are accepted
separately.

This implementation does not expose seed bytes, generated bytes, RNG state,
digests, fingerprints, serialized state, stream identifiers, or comparable
secret identifiers in shell output, serial logs, docs, task evidence, or public
ABI surfaces. The retained unit evidence uses public fixture seeds without
recording generated byte streams. This task still does not accept host-key
generation/provisioning, authorized-key storage, writable seed persistence, SSH
service behavior, live transport, hardware reachability, public ABI/POSIX/Linux
compatibility, broad expansion, stale link-ready discriminator promotion, or a
phase transition. The selected next bounded task is
phase12-shell-csprng-readiness-smoke-20260621.

phase12-shell-csprng-readiness-smoke-20260621 accepts
phase12-shell-csprng-readiness-smoke-accepted. Talos now retains
host/QEMU-substitute smoke evidence for the operator-seeded CSPRNG readiness
metadata path. The task-owned transcript exercises missing, insufficient, and
sufficient public fixture operator seed cases through the accepted CSPRNG,
entropy, and SSH key-readiness classifiers without exposing seed bytes,
generated bytes, digests, fingerprints, stream identifiers, RNG state,
serialized state, or comparable secret identifiers.

The retained sufficient public fixture case demonstrates only the accepted
CSPRNG/cryptographic-strength metadata transition: csprng-ready and
cryptographic-strength true are observed, while ssh-ready remains false because
host-key, authorized-key, persistence/exposure, service, and reachability
prerequisites are still unaccepted. Missing and insufficient seed cases remain
fail-closed and preserve not-ready diagnostic labels. This smoke evidence does
not accept host-key generation/provisioning, authorized-key storage, writable
seed persistence, SSH service behavior, live transport, hardware reachability,
public ABI/POSIX/Linux compatibility, broad expansion, stale link-ready
discriminator promotion, or a phase transition. The selected next bounded task
is phase12-operator-seeded-csprng-closeout-20260621.

phase12-operator-seeded-csprng-closeout-20260621 accepts
phase12-operator-seeded-csprng-closeout-accepted. The operator-seeded CSPRNG
readiness slice is now closed. The accepted slice covers the secret-material
contract for /etc/talos/operator-seed.bin, the no_std chacha20/zeroize
dependency/API strategy, the private Talos CSPRNG core, and retained
host/QEMU-substitute smoke evidence for missing, insufficient, and sufficient
public fixture seed cases.

The accepted frontier is internal CSPRNG readiness plus metadata-only
cryptographic-strength reporting. Sufficient read-only operator seed material
can initialize the private CSPRNG and allow cryptographic-strength metadata to
become true, but ssh-ready remains false because host-key, authorized-key,
persistence/exposure, service, and reachability prerequisites are still
unaccepted. Retained evidence includes only labels, public fixture names,
lengths, paths, and validation commands; it excludes real operator secrets,
generated byte streams, digests, fingerprints, RNG state, serialized state,
stream identifiers, and comparable secret identifiers. Host-key
generation/provisioning, authorized-key storage, writable seed persistence, SSH
service behavior, live transport, hardware reachability, public
ABI/POSIX/Linux compatibility, broad expansion, stale link-ready discriminator
promotion, and phase transition remain rejected. selected_next_task is null and
planningNeeded=true because no later queued Phase 12.5 prerequisite task has
complete objective dependencies and gates.

phase12-ssh-host-key-provisioning-policy-contract-20260621 accepts
phase12-ssh-host-key-provisioning-policy-contract-accepted. Talos selects
operator-provisioned read-only VFS host-key material as the first reversible
host-key policy after accepted operator-seeded CSPRNG readiness. The reserved
path is /etc/talos/ssh/ssh_host_ed25519_key. The path is intended for a future
OpenSSH-format Ed25519 private host key, but this contract accepts no key
parsing, public-key derivation, fingerprinting, generation, writable
persistence, authorized-key storage, SSH service behavior, live transport,
hardware reachability, public ABI/POSIX/Linux compatibility, broad expansion,
or phase transition.

The next implementation may expose only metadata for that read-only VFS file:
missing path preserves sshkeydiag-missing-host-key; non-regular, unreadable,
zero-length, or greater-than-4096-byte material reports
sshkeydiag-host-key-invalid; regular readable length 1 through 63 reports
sshkeydiag-host-key-insufficient; and regular readable length 64 through 4096
clears only the host-key metadata prerequisite. Diagnostics, shell output,
serial logs, docs, task evidence, and public surfaces must not retain, print,
digest, fingerprint, derive from, compare, or otherwise expose private-key
bytes or stable secret identifiers. ssh-ready remains false until authorized-key
metadata, persistence/exposure, SSH service behavior, live transport, and
reachability prerequisites are separately accepted. The selected next bounded
task is phase12-ssh-host-key-vfs-metadata-core-20260621.

phase12-ssh-host-key-vfs-metadata-core-20260621 accepts the metadata-only
read-only VFS implementation for that host-key path. Talos now classifies
/etc/talos/ssh/ssh_host_ed25519_key by VFS metadata only: missing path keeps
sshkeydiag-missing-host-key; non-regular, unreadable, zero-length, or
greater-than-4096-byte material reports sshkeydiag-host-key-invalid; regular
readable length 1 through 63 reports sshkeydiag-host-key-insufficient; and
regular readable length 64 through 4096 clears only the host-key metadata
prerequisite. sshkeydiag combines that host-key metadata with accepted operator
seed and entropy metadata, but ssh-ready remains false until authorized-key
metadata, persistence/exposure, SSH service behavior, live transport, and
reachability prerequisites are separately accepted. Retained source/unit
evidence uses public byte-count fixtures only and excludes real private-key
bytes, generated keys, public-key derivation, fingerprints, digests,
signatures, and stable secret identifiers. The selected next bounded task is
phase12-shell-ssh-host-keydiag-smoke-20260621.

phase12-shell-ssh-host-keydiag-smoke-20260621 accepts retained
host/QEMU-substitute smoke evidence for the read-only VFS host-key metadata
sshkeydiag path. The retained transcript covers missing, invalid,
insufficient, and sufficient public-fixture host-key metadata states. Missing
metadata keeps sshkeydiag-missing-host-key; invalid metadata reports
sshkeydiag-host-key-invalid; insufficient metadata reports
sshkeydiag-host-key-insufficient; and sufficient public-fixture metadata clears
only the host-key prerequisite while ssh-ready remains false. The evidence
retains labels and public fixture state names only; it excludes real private-key
bytes, generated keys, derived public keys, digests, fingerprints, signatures,
and comparable stable secret identifiers. The selected next bounded task is
phase12-ssh-host-key-readiness-closeout-20260621.

phase12-ssh-host-key-readiness-closeout-20260621 accepts the host-key metadata
readiness closeout. The accepted slice now covers the operator-provisioned
read-only VFS host-key policy, metadata-only classification for
/etc/talos/ssh/ssh_host_ed25519_key, and retained host/QEMU-substitute
sshkeydiag smoke evidence. The current frontier clears only the host-key
metadata prerequisite when sufficient public-fixture metadata is present;
ssh-ready remains false because authorized-key metadata, persistence/exposure,
SSH service behavior, live transport, and reachability remain unaccepted. No
authorized-key storage, writable persistence, SSH service behavior, live
transport, hardware reachability, public ABI/POSIX/Linux compatibility, broad
expansion, stale link-ready discriminator promotion, or phase transition is
accepted. selected_next_task is null and planningNeeded=true because no later
queued Phase 12.5 prerequisite task has complete objective dependencies and
gates.

phase12-ssh-authorized-key-policy-contract-20260621 accepts the first
authorized-key source policy after host-key metadata readiness. Talos selects
operator-provisioned read-only VFS material at
/etc/talos/ssh/authorized_keys as the smallest reversible next prerequisite.
The path is intended for a future OpenSSH authorized_keys-compatible public-key
list, but this contract accepts no authorized-key parsing, user authentication,
operator identity binding, writable storage, persistence/exposure, SSH service
behavior, live transport, hardware reachability, public
ABI/POSIX/Linux compatibility, broad expansion, stale link-ready discriminator
promotion, or phase transition.

The next implementation may expose only metadata for that read-only VFS file:
missing path preserves sshkeydiag-missing-authorized-key; non-regular,
unreadable, zero-length, or greater-than-4096-byte material is invalid;
regular readable length 1 through 63 is insufficient; and regular readable
length 64 through 4096 is metadata-present and clears only the authorized-key
metadata prerequisite. Diagnostics, shell output, serial logs, docs, task
evidence, and public surfaces must not retain, print, digest, fingerprint,
derive from, compare, or otherwise expose authorized-key bytes, operator
identity, key-derived identifiers, or comparable stable identifiers. ssh-ready
remains false until persistence/exposure, SSH service behavior, live transport,
and reachability prerequisites are separately accepted. The selected next
bounded task is phase12-ssh-authorized-key-vfs-metadata-core-20260621.

phase12-ssh-authorized-key-vfs-metadata-core-20260621 accepts the
metadata-only read-only VFS implementation for /etc/talos/ssh/authorized_keys.
Talos now classifies missing authorized-key material as
sshkeydiag-missing-authorized-key; non-regular, unreadable, zero-length, or
greater-than-4096-byte material reports sshkeydiag-authorized-key-invalid;
regular readable length 1 through 63 reports
sshkeydiag-authorized-key-insufficient; and regular readable length 64 through
4096 clears only the authorized-key metadata prerequisite. sshkeydiag combines
that authorized-key metadata with accepted host-key, operator seed, and entropy
metadata, but ssh-ready remains false until persistence/exposure, SSH service
behavior, live transport, and reachability prerequisites are separately
accepted. This implementation does not accept authorized-key parsing, key
validation, fingerprinting, user/account modeling, authentication, writable
persistence, SSH service behavior, live transport, hardware reachability,
public ABI/POSIX/Linux compatibility, broad expansion, stale link-ready
discriminator promotion, or phase transition. The selected next bounded task is
phase12-shell-ssh-authorized-keydiag-smoke-20260621.

phase12-shell-ssh-authorized-keydiag-smoke-20260621 accepts retained
host/QEMU-substitute smoke evidence for the read-only VFS authorized-key
metadata sshkeydiag path. The retained transcript covers missing, invalid,
insufficient, and sufficient public-fixture authorized-key metadata states.
Missing metadata keeps sshkeydiag-missing-authorized-key; invalid metadata
reports sshkeydiag-authorized-key-invalid; insufficient metadata reports
sshkeydiag-authorized-key-insufficient; and sufficient public-fixture metadata
clears only the authorized-key prerequisite while ssh-ready remains false. The
evidence retains labels and public fixture state names only; it excludes real
authorized public keys, operator identities, fingerprints, digests, signatures,
key-derived identifiers, private keys, generated keys, and comparable stable
identifiers. The selected next bounded task is
phase12-ssh-authorized-key-readiness-closeout-20260621.

phase12-ssh-authorized-key-readiness-closeout-20260621 accepts the
authorized-key metadata readiness closeout. The accepted slice now covers the
operator-provisioned read-only VFS authorized-key policy, metadata-only
classification for /etc/talos/ssh/authorized_keys, and retained
host/QEMU-substitute sshkeydiag smoke evidence. The current frontier clears
only the authorized-key metadata prerequisite when sufficient public-fixture
metadata is present; ssh-ready remains false because persistence/exposure, SSH
service behavior, live transport, and reachability remain unaccepted. No
writable persistence, SSH service behavior, live transport, hardware
reachability, public ABI/POSIX/Linux compatibility, broad expansion, stale
link-ready discriminator promotion, or phase transition is accepted.
selected_next_task is null and planningNeeded=true because no later queued
Phase 12.5 prerequisite task has complete objective dependencies and gates.

phase12-ssh-persistence-exposure-policy-contract-20260622 accepts the first
persistence/exposure metadata policy after authorized-key metadata readiness.
Talos selects read-only generated-root/initramfs metadata as the first
persistence boundary: /etc/talos/operator-seed.bin,
/etc/talos/ssh/ssh_host_ed25519_key, and /etc/talos/ssh/authorized_keys must
all be present as sufficient metadata under their accepted contracts before
sshkeydiag-persistence-unavailable may be cleared. This is not a writable
persistence claim, durable key-store policy, SSH service behavior, live
transport, or reachability proof.

Talos also selects /etc/talos/ssh/exposure-enabled as the explicit operator
exposure opt-in marker. Missing, invalid, non-regular, unreadable, malformed,
or oversized marker metadata keeps sshkeydiag-exposure-disabled; a regular
readable marker of 0 through 4096 bytes is explicitly enabled, with contents
ignored and not retained. Sufficient persistence/exposure metadata may clear
only sshkeydiag-persistence-unavailable and sshkeydiag-exposure-disabled.
sshkeydiag-not-ready remains present and ssh-ready remains false until SSH
service behavior, live transport, and reachability are accepted separately.
The selected next bounded task is
phase12-ssh-persistence-exposure-vfs-core-20260622.

phase12-ssh-persistence-exposure-vfs-core-20260622 implements the
metadata-only read-only VFS persistence/exposure boundary. sshkeydiag now
derives persistence metadata from sufficient generated-root metadata for all
three accepted material paths: /etc/talos/operator-seed.bin,
/etc/talos/ssh/ssh_host_ed25519_key, and
/etc/talos/ssh/authorized_keys. Missing, invalid, or insufficient metadata on
any of those paths keeps sshkeydiag-persistence-unavailable. sshkeydiag also
classifies /etc/talos/ssh/exposure-enabled as the explicit exposure marker:
missing, invalid, non-regular, or oversized marker metadata keeps
sshkeydiag-exposure-disabled, while a regular readable 0 through 4096 byte
marker clears only that exposure label. Even with sufficient
persistence/exposure metadata, sshkeydiag-not-ready remains present and
ssh-ready remains false until SSH service behavior, live transport, and
reachability are accepted separately. This implementation does not parse keys,
retain secret material, accept writable persistence, expose SSH service
behavior, validate hardware reachability, broaden ABI/POSIX/Linux
compatibility, or create a phase transition. The selected next bounded task is
phase12-shell-ssh-persistence-exposure-diag-smoke-20260622.

phase12-shell-ssh-persistence-exposure-diag-smoke-20260622 accepts retained
host/QEMU-substitute smoke evidence for the read-only VFS persistence/exposure
metadata sshkeydiag path. The retained transcript covers default disabled
exposure, missing exposure marker with otherwise sufficient public-fixture
persistence metadata, invalid exposure marker metadata, and sufficient
public-fixture persistence/exposure metadata. Sufficient metadata clears only
sshkeydiag-persistence-unavailable and sshkeydiag-exposure-disabled;
sshkeydiag-not-ready remains present and ssh-ready remains false because SSH
service behavior, live transport, and reachability remain unaccepted. The
evidence retains labels and public fixture state names only; it excludes real
operator seed bytes, host private key bytes, authorized public key bytes,
generated key material, generated random byte streams, private CSPRNG state,
operator identity, key-derived identifiers, digests, fingerprints, signatures,
and comparable stable identifiers. The selected next bounded task is
phase12-ssh-persistence-exposure-readiness-closeout-20260622.
