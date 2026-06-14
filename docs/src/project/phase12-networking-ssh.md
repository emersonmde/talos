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
