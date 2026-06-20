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
