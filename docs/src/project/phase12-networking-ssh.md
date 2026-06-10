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
