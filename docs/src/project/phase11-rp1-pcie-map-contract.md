# Phase 11 RP1/PCIe Map Contract

This contract defines the first Talos RP1/PCIe mapping slice. It is a source-backed contract for one narrow read-only diagnostic; it does not claim a driver, GPIO ownership, interrupt routing, DMA/cache policy, Ethernet, networking, SSH, or generated-root progress.

## Inputs

- Raspberry Pi Linux `rpi-6.12.y` device-tree sources retained under `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/`: `bcm2712.dtsi`, `bcm2712-rpi-5-b.dts`, `bcm2712-rpi.dtsi`, and `rp1.dtsi`.
- Existing Talos reference notes for Pi 5, RP1 UART0, and RP1 pin-control bring-up evidence.
- Lab-controller status from `GET /status` showing the current boot tree is restored and the boot config includes `enable_rp1_uart=1`.
- Talos' existing PL011 model, where the flag register is at offset `0x18`.

## Address Contract

Linux `bcm2712.dtsi` defines the Pi 5 `pcie2` non-prefetchable PCIe window:

~~~text
PCIe 00_00000000 -> CPU physical 0x1f_0000_0000
size             -> 0xffff_fffc bytes in the source range
~~~

Linux `bcm2712-rpi-5-b.dts` supplies the RP1 child range under `pcie2`:

~~~text
RP1 bus 0xc0_40000000 -> PCIe 00_00000000
size                  -> 0x0041_0000 bytes
~~~

For this first slice Talos treats the initial RP1 peripheral CPU mapping as:

~~~text
cpu_phys = 0x1f_0000_0000 + (rp1_bus - 0xc0_4000_0000)
~~~

Relevant translated addresses:

| RP1 object | RP1 bus address | CPU physical address | Width | Use |
| --- | ---: | ---: | ---: | --- |
| RP1 UART0 PL011 base | `0xc0_4003_0000` | `0x1f_0003_0000` | 32-bit MMIO | Parent block only |
| RP1 UART0 PL011 flag register | `0xc0_4003_0018` | `0x1f_0003_0018` | 32-bit read | First diagnostic target |
| RP1 GPIO bank0 control base | `0xc0_400d_0000` | `0x1f_000d_0000` | 32-bit MMIO | Documented only |
| RP1 IO_BANK0 GPIO14 STATUS | `0xc0_400d_0070` | `0x1f_000d_0070` | 32-bit read | First Milestone 11.2 status diagnostic |
| RP1 pads bank0 base | `0xc0_400f_0000` | `0x1f_000f_0000` | 32-bit MMIO | Documented only |

## Firmware-State Assumptions

The first diagnostic depends on firmware/boot configuration leaving RP1 and UART0 mapped and clocked enough for a non-destructive flag-register read. The current lab boot config reports `enable_rp1_uart=1`, and historical Talos first-light work already used RP1 UART0 through the `pcie2` address path.

The next task must not configure GPIO14/GPIO15, change UART baud/programming, touch RP1 clocks or resets, enable interrupts, or allocate DMA. If the read faults or returns an implausible value, classify that as evidence about this firmware-state assumption rather than expanding the diagnostic in place.

## Diagnostic Contract

The next diagnostic-core task may implement only this first read:

~~~text
name: rp1-uart0-fr-read
address: 0x1f_0003_0018
width: 32-bit volatile little-endian load
source target: RP1 UART0 PL011 flag register
expected success class: mapped/read-value
~~~

The diagnostic should report:

- the source contract id `phase11-rp1-pcie-map-contract-v1`;
- the target name, address, width, and raw value;
- a classification: `mapped/read-value`, `bus-fault/trap`, `firmware-state-dependency`, or `staging/build-blocker`;
- enough serial text to tie the output to the candidate artifact in the later Pi 5 proof task.

The accepted local core may add build/static/archive-review evidence only. The serialized Pi 5 proof remains a separate task that must acquire `hardwareTestLock`, capture candidate identity, serial cursor, TFTP delta, and restore evidence.

## Milestone 11.2 GPIO14 STATUS Boundary

`phase11-rp1-gpio-status-repaired-proof-pi5-20260607` accepts the first
read-only GPIO/status diagnostic boundary. The decisive candidate rerun staged
tree `cb7827b07a3822370fc610dfd18a8ab580cea31a47c4559e41a242975976f83a`,
retained two 46,336-byte `da591740/kernel_2712.img` TFTP fetches, passed the
v2 identity join with final selected-tree identity, and restored the lab to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`. Serial
retained 390 `TALOS: gpio14-status-result` records for contract
`phase11-rp1-irq-clock-gpio-contract-v1`, target `rp1-gpio14-status-read`,
address `0x1f000d0070`, width 32, raw `0xdeaddead`, and
`classification=diagnostic-result-visible`.

This accepts only the one read-only GPIO14 STATUS load and its observable
result shape. It does not accept GPIO ownership, pin-control or pad writes,
clock/reset programming, interrupt enablement or delivery, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

phase11-rp1-irq-clock-gpio-repaired-proof-closeout-20260607 reconciles the
source contract, local diagnostic core, repaired capture identity replay,
repaired no-MMIO control, and real GPIO14 STATUS repaired proof as
gpio14-status-read-frontier-closed. The frontier remains limited to the
read-only GPIO14 STATUS diagnostic boundary above. Same-shaped GPIO14 STATUS
hardware reruns are blocked unless a future supervisor task supplies a
different discriminator or new acceptance criteria. The next Milestone 11.2
step requires supervisor planning for an interrupt-routing source contract;
GPIO ownership, pin-control or pad writes, clock/reset programming, interrupt
enablement or delivery, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, and phase transition remain
unaccepted.

## Milestone 11.2 Interrupt-Routing Boundary

phase11-rp1-interrupt-routing-source-contract-20260607 accepts
phase11-rp1-interrupt-routing-source-contract-v1 as the next narrow
source-backed interrupt-routing contract. The selected diagnostic is the
read-only/no-enable rp1-io-bank0-msix-cfg-read, a single 32-bit volatile load
from RP1 MSIX_CFG(0) at CPU physical 0x1f00108008. Source inspection predicts
RP1 hwirq 0 through PCI MSI-X vector 0 and MIP0 MSI vector 0 to GIC SPI 128 /
INTID 160, but this source route is not itself proof of delivered interrupts.

phase11-rp1-interrupt-routing-diagnostic-core-20260607 accepts a local/static
real candidate with exactly one contracted volatile load and a paired
no-MMIO/no-enable control candidate that constructs no forbidden RP1
interrupt, GPIO, pads, RIO, clock/reset, MSI-X, PCIe config, MIP, or GIC MMIO
path. phase11-rp1-interrupt-routing-no-mmio-control-pi5-20260607 accepts that
paired control output shape as visible on Pi 5 before the real diagnostic
proof.

phase11-rp1-interrupt-routing-diagnostic-pi5-20260607 accepts the real proof
as routing-msix-cfg-visible. The decisive rerun passed the v2 identity join for
tree 63800845c9837b3d57153051583b269070b028412bcd57ea9c55a5f9e56a2304,
retained two 46,648-byte da591740/kernel_2712.img TFTP fetches, final
selected-tree identity, restore proof, and 970
TALOS: rp1-interrupt-routing-result records carrying contract
phase11-rp1-interrupt-routing-source-contract-v1, target
rp1-io-bank0-msix-cfg-read, address 0x1f00108008, raw 0xdeaddead, and
classification=routing-msix-cfg-visible.

phase11-rp1-interrupt-routing-diagnostic-closeout-20260607 reconciles that
chain as interrupt-routing-msix-cfg-read-frontier-closed. The accepted
frontier is limited to the source-backed IO_BANK0 interrupt identity, selected
read-only/no-enable MSIX_CFG(0) diagnostic boundary, paired control proof, and
real Pi 5 visibility proof. It does not accept interrupt delivery, ISR/handler
ownership, GPIO ownership, pin-control behavior, pad writes, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or phase transition. Same-shaped MSIX_CFG(0)
hardware reruns are not progress unless a future supervisor task supplies a
different discriminator or new acceptance criteria.

phase11-rp1-gic-visible-route-source-contract-20260607 accepts the next narrow
GIC-visible source contract only. The selected diagnostic target is
`rp1-io-bank0-gic-route-status-read`: a read-only/no-ack GICv2 status snapshot
for the source-predicted RP1 IO_BANK0 route to GIC SPI 128 / INTID 160. The
only allowed reads are `GICD_ISENABLER5` at `0x10_7fff_9114`,
`GICD_ISPENDR5` at `0x10_7fff_9214`, `GICD_ISACTIVER5` at
`0x10_7fff_9314`, and `GICC_HPPIR` at `0x10_7fff_a018`; INTID 160 is bank 5
bit 0. The contract forbids GIC writes, `GICC_IAR`, `GICC_EOIR`, interrupt
unmasking, ISR installation, RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset
MMIO, and any treatment of a status value as proof of interrupt delivery. The
paired control must preserve the output shape while constructing no GIC, RP1,
MSI-X, PCIe, MIP, GPIO, pads, RIO, or clock/reset MMIO address.

phase11-rp1-gic-visible-route-diagnostic-core-20260607 implements that source
contract as the local/static real candidate and paired no-MMIO/no-GIC/no-RP1
control. phase11-rp1-gic-visible-route-no-mmio-control-pi5-20260607 accepts
the paired control output shape as visible on Pi 5.
phase11-rp1-gic-visible-route-diagnostic-pi5-20260607 accepts the real
read-only/no-ack GIC-visible route status proof as gic-route-status-visible.
After an inconclusive first run, a known-good control passed and the decisive
candidate rerun passed the v2 identity join for tree
`8ef75b3125c21d7025cff539f5004d7f6911af057c5523ce1610be46deecbbe4`,
retained two 47,816-byte `da591740/kernel_2712.img` fetches, final
selected-tree identity, restore proof, and 209
`TALOS: rp1-gic-route-status-result` records. The visible result reported
GICD_ISENABLER5, GICD_ISPENDR5, and GICD_ISACTIVER5 raw values of `0x0` for
INTID 160 bank 5 bit 0, and GICC_HPPIR `0x3ff` / INTID 1023 as spurious.
This accepts only the selected read-only status snapshot. Interrupt pending
generation, delivery, IAR/EOIR acknowledgement, ISR/handler ownership, GPIO
ownership, pin-control behavior, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, and
phase transition remain unaccepted.

phase11-rp1-gic-visible-route-closeout-20260607 reconciles that chain as
gic-visible-route-status-frontier-closed. The accepted frontier is limited to
the source-backed RP1 IO_BANK0 route identity, selected read-only/no-ack GICv2
status snapshot for INTID 160, paired no-MMIO/no-GIC/no-RP1 control proof, and
real Pi 5 visibility proof. Same-shaped GIC-visible route status hardware
reruns are not progress unless a future supervisor task supplies a different
discriminator or new acceptance criteria. Interrupt pending generation,
delivery, IAR/EOIR acknowledgement, ISR/handler ownership, GPIO ownership,
pin-control behavior, pad writes, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, and
phase transition remain unaccepted. Supervisor planning is required for the
next Milestone 11.2 feature slice.

phase11-rp1-gpio-bank-source-status-contract-20260607 accepts the next narrow
source contract only. The selected diagnostic target is
`rp1-io-bank0-source-status-read`: a read-only/non-destructive GPIO bank
source-status snapshot for RP1 IO_BANK0 before any GPIO event generation,
interrupt enablement, or delivery work. The primary allowed read is IO_BANK0
`INTS` at CPU physical `0x1f000d0124`; the companion allowed read is
IO_BANK0 `INTE` at CPU physical `0x1f000d011c`. Both are 32-bit volatile
loads. Bank0 covers GPIO0 through GPIO27, and GPIO14 is bit mask
`0x00004000`. Retained Linux source reads `INTS` in the chained GPIO IRQ
handler and acknowledges events separately through GPIO `CTRL` IRQRESET
writes, so this source contract treats the snapshot as read-only and forbids
the acknowledgement path. The paired control must preserve the output shape
while constructing no RP1 GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, or GIC
MMIO address.

phase11-rp1-gpio-bank-source-status-core-20260607 implements that contract as
the local/static real candidate and paired no-MMIO/no-RP1/no-GIC control.
phase11-rp1-gpio-bank-source-status-control-pi5-20260607 accepts the paired
control output shape as visible on Pi 5.
phase11-rp1-gpio-bank-source-status-pi5-20260607 accepts the real read-only
GPIO bank source-status proof as gpio-bank-source-status-visible. After an
inconclusive first candidate run, a known-good control retained matching TFTP
and PASS evidence, and the decisive candidate rerun passed the v2 identity
join for tree
`84ee89db45d5298e49f44c74e6a18b9c07ce2c146879f677aceace6ad252ea0f`,
retained two 46,904-byte `da591740/kernel_2712.img` fetches, final
selected-tree identity, restore proof, and 269
`TALOS: rp1-gpio-bank-source-status-result` records. The visible result
reported IO_BANK0 INTE at `0x1f000d011c` and INTS at `0x1f000d0124`, raw
values `0xdeaddead`/`0xdeaddead`, GPIO14 mask `0x4000`,
gpio14-enabled=true, and gpio14-source-status=true. This accepts only the
selected read-only source-status snapshot visibility and report decoding.
GPIO event generation, interrupt pending generation beyond that snapshot,
interrupt enablement or delivery, IAR/EOIR acknowledgement, ISR/handler
ownership, GPIO ownership, pin-control behavior, pad writes, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gpio-bank-source-status-closeout-20260607 closes the GPIO bank
source-status chain as gpio-bank-source-status-frontier-closed. The accepted
frontier is limited to the source-backed RP1 IO_BANK0 INTE/INTS register
identity, selected read-only source-status snapshot, paired control proof, and
real Pi 5 visibility proof. Same-shaped GPIO bank source-status hardware
reruns are blocked unless a future supervisor task supplies a different
discriminator or new acceptance criteria. No explicit worker-owned task
remains; supervisor planning is required for the next Milestone 11.2 feature
slice. GPIO event generation, interrupt pending generation beyond the
read-only snapshot, interrupt enablement or delivery, IAR/EOIR
acknowledgement, handler ownership, GPIO ownership, pin-control behavior,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, and phase transition remain
unaccepted.

## Milestone 11.3 DMA/Cache Substrate Contract

phase11-rp1-dma-cache-source-inventory-20260609 accepts the source/static
inventory needed before any RP1 DMA-capable driver, Ethernet, storage,
networking, or SSH work. It records retained source facts for RP1 dma-ranges,
the Synopsys AXI DMA controller shape, selected iommu5 attachments, and Talos'
current DMA/cache ownership gaps. The accepted source inventory does not accept
working DMA behavior, DMA engine programming, IOMMU policy, cache-coherent
driver policy, DMA-safe buffers, high-memory allocation, hardware validation,
networking, SSH, or Milestone 11.3 completion by implication.

phase11-rp1-dma-cache-contract-20260609 accepts
phase11-rp1-dma-cache-substrate-contract-v1 as the local/static contract for
the next implementation boundary. The contract defines these required
ownership boundaries before any driver consumes DMA:

- a single owner for each DMA buffer descriptor;
- memory limited to already accepted kernel-owned physical spans;
- explicit CPU physical, CPU visible, RP1 bus, length, alignment, and
  dma-ranges path fields;
- cache state transitions named by direction before ownership crosses the
  CPU/device boundary;
- drivers consuming prepared descriptors rather than declaring arbitrary memory
  DMA-safe.

The accepted minimal API surface is limited to pure local/static data and
validation: DMA direction, cacheability, address path, buffer descriptor,
alignment and range validators, translation overflow checks, forbidden-claim
checks, and evidence formatter fields. Existing SMP cache-line helpers remain
source evidence for instruction shape and ordering only; they are not accepted
as a driver DMA API.

The required evidence output for the next local/static core must identify
contract phase11-rp1-dma-cache-substrate-contract-v1, source inventory
phase11-rp1-dma-cache-source-inventory-20260609, CPU/RP1 addresses, length,
alignment, address path, direction, cacheability, IOMMU classification, and
validator results. Classification is local/static only, using
local-static-dma-cache-contract-visible, contract-rejected-input, or
staging/build-blocker.

This contract explicitly keeps networking, SSH, real DMA device work, Ethernet,
storage, DMA descriptor rings, cache-maintenance execution for driver buffers,
IOMMU programming or bypass policy, hardware validation, and Milestone 11.3
acceptance by implication blocked until later explicit tasks implement and
validate the substrate. Because the worker cannot create the follow-up task,
the accepted next action is supervisor planning for a bounded local/static
DMA/cache substrate core.

phase11-rp1-dma-cache-substrate-core-20260609 implements that bounded
local/static core in src/dma_cache.rs. The accepted frontier is limited to the
descriptor vocabulary, RP1 RAM/peripheral address-translation helpers,
validator surface, and evidence structs for
phase11-rp1-dma-cache-substrate-contract-v1. The validators reject zero-length,
unaligned, overflow, owned-span escape, high-memory, reserved-memory,
coherent/non-cacheable, and unsupported IOMMU claims against the accepted
bootstrap-bump-owned low-tail span. Focused tests prove one valid RP1
RAM-window descriptor and rejected invalid inputs. This accepts no working DMA,
DMA engine programming, descriptor rings, cache-maintenance execution for
driver buffers, cache-coherent/non-cacheable/IOMMU-backed policy,
DMA-safe allocation beyond descriptor validation, Ethernet, storage,
networking, SSH, hardware validation, or Milestone 11.3 completion by
implication.

phase11-rp1-dma-cache-substrate-closeout-20260609 closes the accepted
local/static descriptor substrate frontier. It reconciles the source inventory,
contract, implementation, tests, evidence, and docs while preserving the limits
above: no working DMA, descriptor rings, executed cache maintenance,
cache-coherent/non-cacheable/IOMMU-backed policy, DMA-safe allocation beyond
descriptor validation, Ethernet, storage, networking, SSH, hardware validation,
or Milestone 11.3 completion by implication. The accepted next source-contract
boundary is driver-adjacent cache synchronization planning, not runtime DMA
behavior.

phase11-rp1-dma-cache-driver-adjacent-source-contract-20260609 accepts
phase11-rp1-dma-cache-sync-plan-contract-v1 as the next local/static
driver-adjacent contract. A future implementation may derive a
DmaCacheSyncPlan only from an accepted DmaBufferDescriptor, selecting clean
before device ownership, invalidate after device ownership, or
clean+invalidate for shared bidirectional synchronization boundaries. The plan
must carry descriptor identity, operation, sync boundary, source-backed
64-byte cache-line coverage, line-aligned CPU range, CPU/RP1 addresses,
direction, cacheability, owner transition, IOMMU classification, and rejected
runtime claims. Existing SMP cache helpers remain source evidence for
instruction shape and ordering only; this contract does not accept executed
cache maintenance for driver buffers, barriers in a live driver path, RP1 MMIO,
DMA channel programming, descriptor rings, Ethernet, storage, networking, SSH,
hardware validation, or Milestone 11.3 completion. No Pi 5 hardware proof is
mechanically required by this source contract; future hardware work, if
planned, must use candidate identity, fresh serial cursor, TFTP delta,
known-good control, then candidate rerun after any inconclusive run.

phase11-rp1-dma-cache-sync-plan-core-20260609 implements that bounded
local/static sync-plan core in src/dma_cache.rs. The accepted frontier is
limited to DmaCacheSyncBoundary, DmaCacheSyncOperation, DmaCacheSyncPlan,
evidence formatting, accepted descriptor-evidence validation, deterministic
rejection cases, and 64-byte cache-line range planning. Focused tests prove
valid ToDevice/before, FromDevice/after, and Bidirectional/shared plans plus
rejected overflow, unsupported cacheability/IOMMU, unsupported
direction/boundary, zero-length, non-accepted classification, and evidence
mismatch inputs. This accepts no executed cache maintenance for driver buffers,
live barrier ordering, working DMA, DMA channel programming, RP1 MMIO writes,
descriptor rings, Ethernet, storage, networking, SSH, hardware validation, or
Milestone 11.3 completion by implication.

phase11-rp1-dma-cache-sync-plan-closeout-20260609 closes that frontier as
rp1-dma-cache-sync-plan-local-static-frontier-closed. The accepted checkpoint
is limited to the local/static sync-plan contract, evidence-backed derivation
from accepted DmaBufferDescriptor inputs, direction/boundary operation
selection, source-backed 64-byte cache-line coverage, deterministic rejection
cases, and focused unit-test evidence. Executed cache maintenance, live
barrier ordering, working DMA, descriptor rings, DMA channel programming,
coherent/non-cacheable/IOMMU-backed policy, DMA-safe allocation beyond
descriptor validation, Ethernet, storage, networking, SSH, hardware
validation, Milestone 12 work, and Milestone 11.3 completion remain
unaccepted. The next mechanically objective boundary is the queued
driver-adjacent diagnostic/source-contract task; it must remain a
source-contract task unless a later accepted task explicitly authorizes
runtime or hardware work.

phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract-20260609 accepts
phase11-rp1-dma-cache-maintenance-sequence-contract-v1 as the next
local/static driver-adjacent boundary. The accepted frontier is limited to a
future instruction/barrier sequence derived only from accepted
DmaCacheSyncPlanEvidence: static vocabulary for clean, invalidate, and
clean+invalidate cache-line operations, a source-backed dsb sy barrier shape,
64-byte line coverage, descriptor and sync-plan identity, rejected runtime
claims, and local/static classification. Existing SMP cache helpers remain
instruction/barrier-shape evidence only. Executed cache maintenance for driver
buffers, live barrier ordering, working DMA, RP1 MMIO, DMA channel
programming, descriptor rings, Ethernet, storage, networking, SSH, hardware
validation, Milestone 12 work, and Milestone 11.3 completion remain
unaccepted.

phase11-rp1-dma-cache-maintenance-sequence-core-20260609 implements that
bounded local/static sequence layer in src/dma_cache.rs. The accepted frontier
is limited to static CleanByVirtualAddressToPoC,
InvalidateByVirtualAddressFromPoC, and CleanInvalidateByVirtualAddressToPoC
instruction vocabulary, the source-backed DataSynchronizationBarrierSy shape,
line-count derivation from accepted DmaCacheSyncPlanEvidence, preservation of
descriptor/sync-plan identity, and focused rejection of non-accepted sync-plan
classification, descriptor/sync-plan mismatches, zero covered length,
cache-line mismatch, range overflow, and unsupported runtime claims. The code
emits local/static evidence only; it does not execute dc/dsb instructions,
claim live barrier ordering, program RP1 MMIO or DMA channels, create
descriptor rings, or add Ethernet, storage, networking, SSH, hardware
validation, Milestone 12 work, or Milestone 11.3 completion. The next queued
boundary is the maintenance-sequence closeout checkpoint.

phase11-rp1-dma-cache-maintenance-sequence-closeout-20260609 closes that
local/static sequence frontier as
rp1-dma-cache-maintenance-sequence-local-static-frontier-closed. The accepted
checkpoint is limited to static clean, invalidate, clean+invalidate, and dsb sy
vocabulary derived from accepted sync-plan evidence, with descriptor/sync-plan
identity, line coverage, rejected runtime claims, and focused unit-test
evidence preserved. Same-shaped local/static sequence retries are closed unless
a later supervisor task supplies materially different runtime/execution scope,
source evidence, or acceptance criteria. Executed cache maintenance, live
barrier ordering, working DMA behavior, RP1 MMIO, DMA channel programming,
descriptor rings, coherent/non-cacheable/IOMMU-backed driver policy, DMA-safe
allocation beyond descriptor validation, Ethernet, storage, networking, SSH,
hardware validation, Milestone 12 work, and Milestone 11.3 completion remain
unaccepted. The next guarded boundary is a runtime/execution-contract task; it
must not execute cache maintenance or run hardware unless its own accepted
scope explicitly authorizes that work.

phase11-rp1-dma-cache-runtime-execution-contract-20260609 accepts
phase11-rp1-dma-cache-maintenance-executor-contract-v1 as the next bounded
runtime/execution boundary. The contract requires any future executor to
consume only accepted DmaCacheMaintenanceSequenceEvidence, preserve descriptor,
sync-plan, and maintenance-sequence identity, validate 64-byte line coverage
and rejected-runtime-claim identity, dispatch only dc cvac, dc ivac, dc civac,
and a final dsb sy, and emit runtime-execution evidence. This is not Ethernet,
storage, networking, SSH, RP1 DMA channel programming, descriptor-ring work,
interrupt completion, hardware validation, or Milestone 11.3 completion.

phase11-rp1-dma-cache-maintenance-executor-core-20260609 implements that
bounded executor core in src/dma_cache.rs. The accepted frontier is limited to
an architecture-gated dispatch boundary that validates accepted maintenance
sequence evidence before line-by-line cache maintenance and final ordering
barrier dispatch. The executor rejects wrong contract ids/classification,
wrong cacheability or IOMMU identity, missing prerequisite rejected-runtime
claims, zero or mismatched line coverage, range overflow, and unsupported
operation, instruction, or barrier vocabulary. It returns evidence for the
executor contract, prerequisite contract ids, operation, instruction, barrier,
line coverage, CPU/RP1 addresses, direction, cacheability, owner transition,
IOMMU classification, prerequisite rejected-runtime claims, executor rejected
claims, and runtime-execution classification. It does not accept driver DMA
completion, RP1 MMIO writes, DMA channel programming, descriptor rings,
interrupt completion, Ethernet, storage, networking, SSH, hardware validation,
Milestone 12 work, or Milestone 11.3 completion by implication. The next queued
boundary is the maintenance-executor closeout checkpoint.

phase11-rp1-gpio-event-latch-source-contract-20260607 is accepted as
source-contract-blocked. Retained RP1/Linux source identifies the GPIO14 event
configuration path: GPIO14 CTRL CLR can clear raw event enables, GPIO14 CTRL
SET can write IRQRESET and raw event-enable bits, and IO_BANK0 INTE set/clear
aliases can change the GPIO14 source-enable bit. The exact retained addresses
are GPIO14 STATUS at 0x1f000d0070, GPIO14 CTRL at 0x1f000d0074, GPIO14 CTRL
SET at 0x1f000d2074, GPIO14 CTRL CLR at 0x1f000d3074, IO_BANK0 INTE SET at
0x1f000d211c, IO_BANK0 INTE CLR at 0x1f000d311c, and IO_BANK0 INTS at
0x1f000d0124. No event-latch or pending-generation diagnostic is accepted
because retained evidence does not prove GPIO14 ownership, deterministic event
generation without pinmux/RIO/pad changes, parent-route masking, or exact
restore semantics for a failed partial run. GPIO14 CTRL writes, IO_BANK0 INTE
writes, IRQRESET acknowledgement, interrupt unmasking or delivery, handler
ownership, GPIO ownership, pin-control/pad behavior, clock/reset programming,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gpio-ownership-restore-source-contract-20260607 accepts the next
narrow source contract only. The selected diagnostic target is
`rp1-gpio14-ownership-route-preflight-read`: a read-only GPIO14 ownership and
parent-route preflight before any event-generation retry. The allowed reads are
GPIO14 STATUS/CTRL at `0x1f000d0070`/`0x1f000d0074`, IO_BANK0 INTE/INTS at
`0x1f000d011c`/`0x1f000d0124`, RIO0 OUT/OE/IN at
`0x1f000e0000`/`0x1f000e0004`/`0x1f000e0008`, GPIO14 pad control at
`0x1f000f003c`, and the already accepted read-only INTID 160 GIC status
registers. GPIO14 remains the only candidate pin because prior frontiers
already use GPIO14 STATUS, IO_BANK0 bit 14, and the IO_BANK0 route. The
contract allows no writes; cleanup is no-op hardware-state cleanup because the
diagnostic is read-only. Its paired control must preserve the same output shape
while constructing no RP1 GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, or GIC
MMIO address. This accepts only the read-only preflight contract, not GPIO
ownership, event generation, pending generation, interrupt enablement or
delivery, GIC acknowledgement, handler ownership, GPIO CTRL/INTE/RIO/pad
writes, parent-route masking writes, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.3, or phase transition.

phase11-rp1-gpio-ownership-restore-pi5-20260607 accepts the real Pi 5
preflight proof as gpio14-ownership-preflight-blocked-non-gpio-function.
After a first candidate run was rejected by non-empty pre-power serial drain
evidence, a known-good control passed the v2 identity join and the real
candidate rerun selected tree
91372af6aeecc90b47b57d6d3f1caf46ee5b20f47ec392977fdae2674ac0112f.
Stable pre-restore TFTP evidence retained two served 50056-byte
da591740/kernel_2712.img candidate fetches, final pre-restore identity still
matched the selected tree, and fresh serial retained 93
TALOS: rp1-gpio14-ownership-route-preflight-result markers. The diagnostic
reported GPIO14 fsel 13 / unknown function, RIO GPIO14 out/oe/in true, pad
input disabled and output disabled, INTID160 not enabled, pending, or active,
HPPIR INTID 1023, and classification
gpio14-ownership-preflight-blocked-non-gpio-function. This accepts only the
read-only preflight visibility and blocker; GPIO ownership, event generation,
interrupt pending generation beyond the read-only snapshot, interrupt
enablement or delivery, GIC acknowledgement, handler ownership, GPIO
CTRL/INTE/RIO/pad writes, parent-route masking writes, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, and phase transition remain unaccepted. The lab
was restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

phase11-rp1-gpio-ownership-restore-closeout-20260607 closes the GPIO14
ownership/route preflight chain as
gpio14-ownership-preflight-blocked-frontier-closed. The accepted frontier is
limited to the source-backed GPIO14 ownership/route preflight register
identity, the local real/control candidate split, the no-MMIO/no-RP1/no-GIC
control output proof, and the real Pi 5 visibility proof that blocks later
GPIO14 event-generation work because GPIO14 reported fsel 13 / unknown
function. Same-shaped GPIO ownership/route preflight, event-latch, or
event-generation hardware reruns are blocked unless a future supervisor task
supplies a different discriminator or explicit ownership, masking,
deterministic event-source, partial-write recovery, and restore acceptance
criteria. GPIO ownership, GPIO event generation, interrupt pending generation
beyond the read-only snapshot, interrupt enablement or delivery, GIC
acknowledgement, handler ownership, GPIO CTRL/INTE/RIO/pad writes,
parent-route masking writes, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, and
phase transition remain unaccepted.

phase11-rp1-gpio-owned-event-discriminator-source-contract-20260607 accepts a
new source-only GPIO16 event/source-status discriminator contract after the
GPIO14 blocker. The selected target is
`rp1-gpio16-owned-level-high-event-discriminator`. Retained Pi 5 source names
RP1 GPIO16 as a generic `GPIO16` line, retained fixed board consumers do not
reference GPIO16, the debug UART is `uart10`, and prior Talos RP1 UART0 usage
is confined to GPIO14/GPIO15. The contract derives GPIO16 STATUS/CTRL at
`0x1f000d0080`/`0x1f000d0084`, CTRL SET/CLR at
`0x1f000d2084`/`0x1f000d3084`, GPIO16 pad control at `0x1f000f0044`,
IO_BANK0 INTE/INTS at `0x1f000d011c`/`0x1f000d0124`, RIO0 OUT/OE/IN at
`0x1f000e0000`/`0x1f000e0004`/`0x1f000e0008`, and the accepted INTID 160
GIC-visible status registers. The only accepted writes are the bounded GPIO16
pad/CTRL/RIO/event-enable/IRQRESET/IO_BANK0-INTE bit-16 writes and exact
restore writes named by the task, gated by a read-only parent-route preflight
that must show INTID 160 disabled, not pending, not active, and not visible in
HPPIR. The paired control must preserve the output shape while constructing no
RP1 GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, or GIC MMIO path. This accepts
only the source contract, not hardware behavior, interrupt delivery, GIC
acknowledgement, ISR/handler ownership, broad GPIO ownership, GPIO14
event-generation retry, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
a phase transition.

phase11-rp1-gpio-owned-event-discriminator-control-pi5-20260607 accepts the
paired no-MMIO/no-RP1/no-GIC control output shape as visible on Pi 5. The
accepted rerun selected boot tree
a2cd628f8fed4b70b726c6659f2788762922334289f1d90eef60e61e01963e46, fetched
the 49,480-byte da591740/kernel_2712.img twice in stable pre-restore TFTP
evidence, passed the v2 identity join with no rejection reasons, retained 40
TALOS: rp1-gpio16-owned-event-discriminator-control markers, and restored to
tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
This accepts only the simulated/control output path; real GPIO16 event/source
status behavior remains gated on the separate real Pi 5 diagnostic task.

phase11-rp1-gpio-owned-event-discriminator-pi5-20260607 accepts the real Pi 5
GPIO16 event discriminator run as
gpio16-owned-event-preflight-blocked-pin-function. After an initial
serial-drain-rejected candidate run and a known-good control, the accepted
candidate rerun selected boot tree
348b127402b41ca3115ed09aa2e55cc2dce837dc04a7e4770f0143bd17e4c61c, fetched
the 52,056-byte da591740/kernel_2712.img twice in stable pre-restore TFTP
evidence, passed the v2 identity join with no rejection reasons, retained 38
TALOS: rp1-gpio16-owned-event-discriminator-result markers, and restored to
tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
result markers report GPIO16 fsel 13 / unknown function, so the diagnostic
skipped the accepted action writes and restore writes. This accepts only the
GPIO16 pin-function preflight blocker; GPIO16 event generation, interrupt
pending generation, interrupt delivery, GIC acknowledgement, handler
ownership, broad GPIO ownership, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, and
phase transition remain unaccepted.

phase11-rp1-gpio-owned-event-discriminator-closeout-20260607 closes that
chain as gpio16-owned-event-preflight-blocked-frontier-closed. The accepted
frontier is limited to the source-backed GPIO16 discriminator contract, the
local real/control candidate split, the paired no-MMIO/no-RP1/no-GIC control
output proof, and the real Pi 5 blocker proof. Same-shaped GPIO16
event-discriminator hardware reruns are blocked unless a future supervisor
task supplies a different discriminator or source-backed ownership/function
acceptance criteria that avoid the observed GPIO16 fsel 13 / unknown-function
blocker. GPIO16 event generation, interrupt pending generation, interrupt
enablement or delivery, GIC acknowledgement, handler ownership, broad GPIO
ownership, Talos-owned GPIO state, GPIO16 action writes on hardware,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, and phase transition remain
unaccepted.

phase11-rp1-clock-reset-status-source-contract-20260607 accepts the next
read-only source contract only. The selected target is
`rp1-clock-manager-status-read`: a non-destructive RP1 clock manager status
snapshot after GPIO14 and GPIO16 both blocked on fsel 13 / unknown function.
The allowed 32-bit volatile loads are `PLL_SYS_CS` at `0x1f00020000`,
`CLK_SYS_CTRL` at `0x1f00018014`, `CLK_SYS_DIV_INT` at `0x1f00018018`,
`CLK_SYS_SEL` at `0x1f00018020`, `CLK_SLOW_SYS_CTRL` at `0x1f00018024`,
`CLK_UART_CTRL` at `0x1f00018054`, `CLK_UART_DIV_INT` at `0x1f00018058`,
and `CLK_UART_SEL` at `0x1f00018060`. The diagnostic may decode only
`PLL_CS_LOCK`, `CLK_CTRL_ENABLE`, source, and divider fields, and may report
the retained GPIO14/GPIO16 fsel 13 blocker context. The paired control must
preserve the output shape while constructing no RP1 clock/reset, GPIO/RIO/pads,
MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile load/store to
those paths. Linux's RP1 reset path is retained only as forbidden source
context; reset writes, clock writes, GPIO ownership retries, event generation,
interrupt delivery, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-clock-reset-status-core-20260607 accepts the local/static
diagnostic pair for that contract only. The real candidate performs the eight
contracted read-only clock manager loads and repeats TALOS:
rp1-clock-manager-status-result with decoded PLL_SYS/CLK_SYS/CLK_UART fields.
The paired control repeats TALOS: rp1-clock-manager-status-control with the
same output shape while constructing no RP1 clock/reset, GPIO/RIO/pads,
MSI-X/PCIe/MIP, or GIC MMIO address. No Pi 5 hardware behavior is accepted by
the local/static core.

phase11-rp1-clock-reset-status-control-pi5-20260607 accepts only the serialized
Pi 5 no-MMIO/no-RP1/no-GIC control output/capture path. The accepted control
run selected tree
eeb71c0bfc3cbd259a18c5f53403555628a5cf8f3273d764cab80656087dbb66, retained
two served 47,120-byte da591740/kernel_2712.img TFTP fetches, passed the v2
identity join with no rejection reasons, retained 49 TALOS:
rp1-clock-manager-status-control markers, and restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
proves only the simulated/control output and capture path before the real
diagnostic.

phase11-rp1-clock-reset-status-pi5-20260607 accepts the selected real read-only
RP1 clock manager status boundary on Pi 5. After a first candidate run and
known-good/control run were rejected by serial-drain/capture freshness evidence,
the accepted diagnostic rerun selected tree
3e64059ed440eaf48f096d8e2e4113609dbfe9f78444955003547515439c3704, retained
two served 47,280-byte da591740/kernel_2712.img TFTP fetches, passed the v2
identity join with no rejection reasons, retained 320 TALOS:
rp1-clock-manager-status-result records, and restored the lab to the original
pre-run tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
visible result reported pll-sys-lock=true, clk-sys-enabled=true,
clk-uart-enabled=true, and classification
rp1-clock-manager-status-visible.

phase11-rp1-clock-reset-status-closeout-20260607 closes that chain as
rp1-clock-manager-status-frontier-closed. The accepted frontier is limited to
the source-backed clock manager status contract, the local real/control
candidate split, the paired no-MMIO/no-RP1/no-GIC control output proof, and
the real Pi 5 read-only visibility proof. RP1 clock/reset writes, reset
ownership, GPIO ownership, event generation, interrupt delivery, GIC
acknowledgement, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, and phase
transition remain unaccepted. Future clock/reset writes, GPIO ownership
retries, or interrupt-delivery work require supervisor planning with a new
source contract and acceptance criteria.

phase11-rp1-clock-reset-write-restore-source-contract-20260607 accepts the
next narrow source contract only. The selected target is
`rp1-clk-adc-ctrl-idempotent-write-restore`: a bounded RP1 clock-manager
write/readback/restore proof against `CLK_ADC_CTRL` at CPU physical
`0x1f00018144` (source offset `0x00144`). The only allowed real-candidate
operations are: pre-read `CLK_ADC_CTRL` and retain `pre_raw`, write `pre_raw`
back to `CLK_ADC_CTRL`, post-read and retain `post_raw`, restore-write
`pre_raw`, and restore-read `restore_raw`. The expected unchanged fields are
the full raw value, `CLK_CTRL_ENABLE` bit 11, `CLK_CTRL_AUXSRC` bits 9:5, and
source bits. `clk-rp1.c` models `clk_adc` as a normal clock with no GPCLK
output-enable mask, and retained `rp1.dtsi` marks the ADC consumer disabled;
the contract therefore avoids boot UART, critical system clocks, PCIe/RP1
access, GPIO14/GPIO16 state, interrupt routing, serial capture, and
reset-controller paths. This accepts only an idempotent clock-manager write
path and restore discipline. Non-idempotent clock changes, reset-controller
writes, GPIO ownership retries, event generation, interrupt delivery, GIC
acknowledgement, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-clock-reset-write-restore-core-20260607 accepts the local/static
diagnostic pair for that contract only. The real candidate performs the
accepted `CLK_ADC_CTRL` pre-read, idempotent write, post-read, restore-write,
and restore-read sequence and repeats TALOS:
rp1-clock-adc-ctrl-write-restore-result with the accepted report fields. The
paired control repeats TALOS:
rp1-clock-adc-ctrl-write-restore-control with the same output shape while
constructing no RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO
address. No Pi 5 hardware behavior is accepted by the local/static core.

phase11-rp1-clock-reset-write-restore-control-pi5-20260607 accepts only the
serialized Pi 5 no-MMIO/no-RP1/no-GIC control output/capture path. After a
first candidate run was rejected by serial-drain/capture freshness evidence,
the accepted control rerun selected tree
94775dea793b4493ad2cdbdfd3bd6e8882362d10d440a0fadb1ed9296ab27f8e, retained
two served 46,888-byte da591740/kernel_2712.img TFTP fetches, passed the v2
identity join with no rejection reasons, retained 108 TALOS:
rp1-clock-adc-ctrl-write-restore-control records, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
proves only the simulated/control output and capture path before the real
diagnostic.

phase11-rp1-clock-reset-write-restore-pi5-20260607 accepts the selected real
`CLK_ADC_CTRL` idempotent write/readback/restore boundary on Pi 5. After a
first candidate run and a known-good control were rejected by
serial-drain/capture freshness evidence, the accepted diagnostic rerun selected
tree 3ea80fee925c554e0e65141bbd18174ab661b3e5ac6a73b82d7c130ca7adb709,
retained two served 47,232-byte da591740/kernel_2712.img TFTP fetches, passed
the v2 identity join with no rejection reasons, retained 102 TALOS:
rp1-clock-adc-ctrl-write-restore-result records, and restored the lab to the
original pre-run tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The visible
result reported pre-raw=0xdeaddead, post-raw=0xdeaddead,
restore-raw=0xdeaddead, post-eq-pre=true, restore-eq-pre=true,
retained-gpio14-blocker=fsel13, retained-gpio16-blocker=fsel13, and
classification rp1-clock-adc-ctrl-idempotent-write-restored. This accepts only
the selected idempotent write/readback/restore proof and restore discipline;
broad clock/reset ownership, non-idempotent clock programming,
reset-controller writes, GPIO ownership, event generation, interrupt delivery,
GIC acknowledgement, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-clock-reset-write-restore-closeout-20260607 closes that chain as
rp1-clock-adc-ctrl-write-restore-frontier-closed. The accepted frontier is
limited to the source-backed CLK_ADC_CTRL idempotent write/readback/restore
contract, the local real/control candidate split, the paired no-MMIO/no-RP1/
no-GIC control output proof, and the real Pi 5 proof that the selected
write-back and restore-read returned the pre-read raw value for this run. This
is the first accepted reversible RP1 clock-manager write/restore boundary, but
it does not accept broad clock/reset ownership, non-idempotent clock
programming, reset-controller writes, GPIO ownership, event generation,
interrupt delivery, GIC acknowledgement, handler ownership, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.3, or phase transition. A future GPIO ownership retry, interrupt-delivery
slice, or broader clock/reset step requires supervisor planning with a new
source contract and acceptance criteria.

phase11-rp1-clock-adc-enable-toggle-source-contract-20260607 accepts the next
bounded non-idempotent clock-manager source contract:
phase11-rp1-clock-adc-enable-toggle-source-contract-v1, target
rp1-clk-adc-ctrl-enable-bit-toggle-restore. The selected register remains
CLK_ADC_CTRL at CPU physical 0x1f00018144, and the only selected transition
mask is CLK_CTRL_ENABLE bit 11 (0x00000800). The allowed operation sequence is
pre-read and report pre_raw, compute transition_raw = pre_raw ^ 0x00000800,
write transition_raw, post-read, restore-write pre_raw, and restore-read.
Accepted invariants require the post-read to differ from pre-read only by bit
11, restore-read to equal pre-read, the decoded enable bit to flip and return,
and auxsrc/source fields to remain unchanged. The source evidence is limited
to clk_adc: Linux rp1_clock_on/rp1_clock_off modify bit 11 on a normal clock's
own control register, clk_adc has no GPCLK output-enable mask, and the retained
ADC device-tree consumer is disabled. This source contract accepts only the
reversible enable-bit transition boundary and paired no-MMIO/no-RP1/no-GIC
control requirement; it does not accept hardware behavior, broad clock/reset
ownership, reset-controller writes, GPIO ownership, event generation,
interrupt delivery, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, or a phase transition.

phase11-rp1-clock-adc-enable-toggle-pi5-20260607 completed the paired control
and real Pi 5 proof chain for that contract. The accepted real rerun published
candidate tree 7024bb54a9446c681d4a8b9c80372fe52a4d4f93b7939f299a8eb2d7199a697a,
retained two 47,512-byte da591740/kernel_2712.img TFTP fetches, retained 78
identity-joined result markers, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
terminal classification is rp1-clock-adc-ctrl-enable-toggle-mismatch-restored:
Talos attempted the contracted 0x800 enable-bit transition and restored the
observed pre-read raw value, but the post-read still matched pre-read rather
than the requested transition value. This is accepted as a precise
mismatch-restored blocker, not as successful non-idempotent clock ownership.
Successful enable-bit transition ownership, broad clock/reset ownership, GPIO
ownership, event generation, interrupt delivery, handler ownership, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.3, and a
phase transition remain unaccepted.

phase11-rp1-clock-adc-enable-toggle-closeout-20260607 closes that chain as
rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed. The
accepted frontier is limited to the source-backed CLK_ADC_CTRL enable-bit
transition/readback/restore contract, the local real/control candidate split,
the paired no-MMIO/no-RP1/no-GIC control proof, and the real Pi 5 proof that
the selected transition attempt ran under identity-joined serial/TFTP/final
tree evidence and restored the observed pre-read raw value. It is a precise
blocker: the post-read still matched pre-read instead of the requested
0x00000800 transition. Same-shaped CLK_ADC_CTRL enable-bit transition hardware
reruns are blocked unless a future supervisor task supplies a different
discriminator or new acceptance criteria. Successful non-idempotent clock
ownership, broad clock/reset ownership, GPIO ownership, event generation,
interrupt delivery, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, and phase transition remain
unaccepted.

phase11-rp1-clock-write-effect-discriminator-source-contract-20260607 accepts
the next source contract as
phase11-rp1-clock-write-effect-discriminator-source-contract-v1. It selects one
read-only discriminator, rp1-clk-adc-window-coherence-read, to explain the
retained CLK_ADC_CTRL enable-toggle mismatch before any further RP1 clock
writes. The allowed 32-bit reads are CLK_SYS_CTRL at 0x1f00018014,
CLK_UART_CTRL at 0x1f00018054, two ordered reads of CLK_ADC_CTRL at
0x1f00018144, CLK_ADC_DIV_INT at 0x1f00018148, and CLK_ADC_SEL at
0x1f00018150. The report must retain the prior mismatch context
(pre_raw=0xdeaddead, transition_raw=0xdeadd6ad, post_raw=0xdeaddead,
restore_raw=0xdeaddead) and expose ADC CTRL stability, ADC window repeated
sentinel state, ADC selector shape, and clk_sys/clk_uart guard fields. No
writes or restore operations are selected. The paired no-MMIO/no-RP1/no-GIC
control must pass before any real Pi 5 proof. This accepts only the source
contract for a read-only ADC clock-window coherence discriminator; successful
non-idempotent clock ownership, broad clock/reset ownership,
divider/source/PLL/frequency-counter/reset-controller writes, GPIO ownership,
event generation, interrupt delivery, handler ownership, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-clock-write-effect-discriminator-core-20260607 accepts the
local/static implementation of that read-only ADC clock-window discriminator.
The real candidate emits TALOS: rp1-clock-adc-window-coherence-result with the
accepted ordered reads, guard fields, ADC window raw/decoded fields, retained
enable-toggle mismatch context, and terminal classifications. The paired
control emits TALOS: rp1-clock-adc-window-coherence-control with the same
output shape while constructing no RP1 clock/reset, GPIO/RIO/pads,
MSI-X/PCIe/MIP, or GIC MMIO address. This accepts no Pi 5 hardware behavior
and does not accept any RP1 clock/reset write, broad clock/reset ownership,
GPIO ownership, event generation, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, or phase
transition.

phase11-rp1-clock-write-effect-discriminator-control-pi5-20260607 accepts the
paired no-MMIO/no-RP1/no-GIC Pi 5 control proof. The accepted rerun staged tree
326db32f8082eb83f24df752d81611b77a2a270ff539a7af27adb91b0ef89412 with a
47,360-byte da591740/kernel_2712.img, retained two matching TFTP fetches,
52 occurrences of TALOS: rp1-clock-adc-window-coherence-control, final
pre-restore selected-tree identity, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
accepts only the control output/capture path; real RP1 clock/reset behavior,
clock/reset reads or writes, GPIO ownership, event generation, interrupt
delivery, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-clock-write-effect-discriminator-pi5-20260607 accepts the selected
real read-only ADC clock-window coherence result on Pi 5 as
rp1-clock-adc-window-readback-sentinel. After an inconclusive first capture, a
known-good production-timer control passed the v2 identity join, and the
unchanged real candidate rerun staged tree
f93e47c1d5b68dd243c795d3323cc04249c0b62cda22c3ccb003593c56232902 with a
48,056-byte da591740/kernel_2712.img, retained two matching TFTP fetches,
52 occurrences of TALOS: rp1-clock-adc-window-coherence-result, final
pre-restore selected-tree identity, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
accepted output reported CLK_SYS_CTRL, CLK_UART_CTRL, two ordered CLK_ADC_CTRL
reads, CLK_ADC_DIV_INT, and CLK_ADC_SEL all returning 0xdeaddead, with
adc-ctrl-stable=true, adc-window-all-equal=true,
adc-window-all-deaddead=true, and retained enable-toggle restore equality. This
accepts only the selected read-only sentinel/result boundary; successful
non-idempotent clock ownership, broad RP1 clock/reset ownership, any new
clock/reset write, GPIO ownership, event generation, interrupt delivery,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.3, and phase transition remain unaccepted.

phase11-rp1-clock-write-effect-discriminator-closeout-20260607 accepts the
chain closeout as rp1-clock-adc-window-readback-sentinel-frontier-closed. The
closeout reconciles the source contract, local/static core, no-MMIO/no-RP1/
no-GIC control proof, real Pi 5 proof, restore evidence, and evidence maps.
The accepted frontier is limited to the read-only ADC clock-window coherence
sentinel/result: the selected clock-manager window returned repeated
0xdeaddead values across CLK_SYS_CTRL, CLK_UART_CTRL, two ordered
CLK_ADC_CTRL reads, CLK_ADC_DIV_INT, and CLK_ADC_SEL. This boundary may inform
future supervisor planning for a GPIO ownership retry, interrupt-delivery
slice, or broader clock/reset step, but it does not itself authorize any new
write. Successful non-idempotent clock ownership, broad RP1 clock/reset
ownership, divider/source/PLL/frequency-counter/reset-controller writes,
GPIO ownership, event generation, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-clock-sentinel-address-discriminator-source-contract-20260608
accepts the next source contract as
phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1. It
selects one read-only discriminator, rp1-sysinfo-vs-clock-sentinel-read, to
distinguish a live RP1 SYSINFO identity/address-decode path from the retained
clock-window sentinel before any further clock writes, GPIO ownership retry,
or interrupt-delivery work. The allowed 32-bit reads are SYSINFO_CHIP_ID at
0x1f00000000, SYSINFO_PLATFORM at 0x1f00000004, and CLK_ADC_CTRL at
0x1f00018144. The expected chip identity is 0x20001927 from retained Pi 5
firmware logs and Linux RP1 MFD source context; CLK_ADC_CTRL remains only the
retained sentinel comparator. The report must expose the raw values,
chip-id/sentinel/equality booleans, retained ADC-window sentinel context, and
one terminal classification from the accepted vocabulary. No writes or restore
operations are selected. The paired no-MMIO/no-RP1/no-GIC control must pass
before any real Pi 5 proof. This accepts only the source contract for a
read-only SYSINFO-vs-clock-sentinel discriminator; broad RP1 clock/reset
ownership, clock/reset writes, GPIO ownership, event generation, interrupt
delivery, handler ownership, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-clock-sentinel-address-discriminator-pi5-20260608 accepts the real
Pi 5 proof as rp1-sysinfo-and-clock-window-sentinel. The accepted rerun
published only the committed read-only SYSINFO-vs-clock-sentinel candidate as
tree 22c13cf75878b9f1776d9ae00b760457df45a508b915c3032f4ac792693a74a4;
TFTP retained two 47,776-byte da591740/kernel_2712.img fetches; serial
retained 62 result markers; and the pi5-capture-transaction-v2 identity join
passed with no rejection reasons before restore to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The Talos
reads reported SYSINFO_CHIP_ID=0xdeaddead, SYSINFO_PLATFORM=0xdeaddead, and
CLK_ADC_CTRL=0xdeaddead, so the accepted frontier is a broader
SYSINFO/address-decode sentinel boundary, not live RP1 SYSINFO identity or
clock/reset ownership. Clock/reset writes, GPIO ownership, event generation,
interrupt delivery, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-clock-sentinel-address-discriminator-closeout-20260608 closes the
SYSINFO-vs-clock-window discriminator chain as
rp1-sysinfo-and-clock-window-sentinel-frontier-closed. The closeout reconciles
the source contract, local/static core, no-MMIO/no-RP1/no-GIC control proof,
real Pi 5 proof, restore evidence, and evidence maps. The accepted frontier
is limited to the read-only SYSINFO/address-decode sentinel boundary:
SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and CLK_ADC_CTRL all returned 0xdeaddead
under identity-joined Pi 5 evidence. This boundary may inform future
supervisor planning for a different address/decode discriminator, GPIO
ownership retry, interrupt-delivery slice, or broader clock/reset step, but it
does not itself authorize any new write or feature transition. Live RP1
SYSINFO identity, broad RP1 clock/reset ownership, clock/reset writes,
GPIO ownership, event generation, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted.

## Pi 5 Proof Status

`phase11-rp1-register-read-pi5-proof-20260605` completed with a hardware
blocker, not a mapping acceptance. The lab published candidate tree
`a96f0d8dc17a4872cb52e94c37c85d5adc5312255d26f988dbd8b71e1b6118c9`; TFTP
served the selected 87,392-byte `da591740/kernel_2712.img` before restore in
two candidate runs. Neither run reached `rpi5-rp1-uart0-fr-read`,
`mapped/read-value`, or `PASS` serial output from a fresh cursor. A
known-good control on the restored accepted tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` retained
`TALOS: kernel_main` and accepted command-loop output, so the proof boundary
is `blocked-pre-entry-or-handoff-after-candidate-fetch`.

Do not treat this as evidence that the RP1 UART0 flag register is mapped or
unmapped. The next source-level investigation or revised diagnostic shape must
be supervisor-planned before changing RP1 constants or broadening into GPIO,
interrupt, DMA/cache, networking, SSH, or storage work.

`phase11-rp1-diagnostic-entry-pi5-proof-20260605` reran the revised
pre-MMIO-marker candidate after the source-level handoff task. The candidate
archive SHA-256 was
`2640ab9ceabee343ee1426b7137e1597687517f56d3b61f58a7ac0e7ab4b6608`, and the
published boot tree was
`0b25c8e08b7cdbac0447ee80a962ed7ee0fa9d219eafc3f060cfcd902c035511`. After
mandatory inconclusive-run triage, the known-good control restored
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, fetched
the 104,136-byte control kernel, reached `TALOS: kernel_main`, and retained
PASS output. The candidate rerun fetched the selected 87,480-byte
`da591740/kernel_2712.img` twice before restore, but serial output still did
not reach `TALOS: kernel_main`, `rpi5-rp1-uart0-fr-read: start`,
`rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or `PASS`.
The classification remains `blocked-pre-entry-or-handoff-after-candidate-fetch`;
the proof does not accept RP1 mapped or unmapped behavior.

`phase11-rp1-diagnostic-entry-closeout-20260605` reconciles the revised
source/handoff and Pi 5 proof evidence. It accepts only the source contract,
local diagnostic candidate, candidate publication/TFTP fetch, known-good
control, and restore evidence. It does not accept RP1 mapped/read-value,
unmapped, trap, firmware-state, GPIO, interrupt, DMA/cache, networking, SSH,
storage, or Milestone 11.2 behavior. The next source-level handoff change or
diagnostic revision requires supervisor planning.

phase11-rp1-diagnostic-entry-control-source-core-20260605 adds a local
entry-control candidate for the next serialized proof. The candidate emits
rpi5-rp1-entry-control: rust-entry-control,
rpi5-rp1-entry-control: no-rp1-mmio,
rpi5-rp1-entry-control: classification=entry-control-reached, and
rpi5-rp1-entry-control: PASS immediately after the normal Pi 5 rust_entry
early-phase line, then stops before BootInfo parsing, target::init, RP1
GPIO/pin flushes, boot reports, memory planning, and the RP1 UART0 FR read
path. This is only a source/local discriminator and does not accept handoff
reachability or any RP1 mapped/unmapped behavior until the queued Pi 5 proof
publishes the candidate and retains hardware serial/TFTP evidence.

phase11-rp1-diagnostic-entry-control-pi5-proof-20260605 completed with a
staging-or-capture blocker. The task published only the accepted entry-control
candidate archive
`target/talos-rpi5-rp1-entry-control-source-core.tar.gz`, staging tree
`ab88a3d8549837459c8cebf8cb22580b52b39665421b7eb6d6773ebce8c6f9c2` and a
51,808-byte `da591740/kernel_2712.img`. The first candidate run, known-good
control, and candidate rerun all reached visible Raspberry Pi firmware serial
output through `Boot mode: NETWORK`, but fresh TFTP deltas were empty in all
three runs. The proof therefore does not show candidate fetch, Rust entry,
entry-control PASS, RP1 mapped/read-value, or RP1 unmapped/trap behavior. The
boot tree was restored to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` before
completion.

phase11-rp1-diagnostic-entry-control-closeout-20260605 reconciles that
source/local discriminator and blocker proof. It accepts only the source
contract, the local pre-BootInfo entry-control candidate, recorded
publication/staging state, and restore state. It does not accept candidate
fetch, Rust entry or entry-control reachability, RP1 mapped/read-value,
unmapped, trap, firmware-state, GPIO, interrupt, DMA/cache, networking, SSH,
storage, generated-root, broader PCIe, or Milestone 11.2 behavior. The next
bounded Phase 11 slice requires supervisor planning.

phase11-rp1-entry-control-candidate-rerun-20260605 later reran the accepted
entry-control candidate under the repaired known-good readiness and stable TFTP
rules. The candidate archive
target/talos-rpi5-rp1-entry-control-source-core.tar.gz staged a 51,808-byte
da591740/kernel_2712.img, and stable pre-restore TFTP evidence observed two
candidate kernel fetches. Fresh serial still did not reach TALOS: kernel_main,
the entry-control markers, or PASS. The result classification is
candidate-fetch-observed-without-entry-control; it accepts candidate fetch only,
not Rust entry, entry-control reachability, RP1 mapped/unmapped behavior, or
firmware-state behavior.

phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator-20260606 published
the accepted delayed-marker RP1 UART0 FR candidate after local/static evidence
proved one contracted 32-bit volatile load from 0x1f_0003_0018 after bounded
pre-load UART10 markers. Hardware evidence is still limited to
candidate-fetch-without-final-preload-marker: the staged tree
e9cd5c4a9571cab464ee76c046a7c4a2f42ba9cf75bb91f55de931dba16a3e2a fetched the
46,152-byte da591740/kernel_2712.img twice in the first candidate run and twice
again in the candidate rerun, but repaired saturated-cursor serial capture
showed firmware NETWORK output without the delayed preload loop, final pre-load
marker, post-load output, mapped/read-value classification, or trap/panic
output. The known-good control also retained firmware serial output without
TALOS: kernel_main in its bounded window. This does not accept RP1
mapped/read-value, unmapped/trap, firmware-state, GPIO, interrupt, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, or Milestone 11.2
behavior. The next step must be a non-repetitive closeout-planned discriminator,
not another same-shaped FR-read rerun.

phase11-rp1-uart0-fr-read-delayed-marker-closeout-20260606 reconciles that
source/static candidate and Pi 5 blocker evidence as
candidate-fetch-without-final-preload-marker. The accepted boundary is limited
to the delayed-marker source/static candidate shape, candidate
publication/fetch evidence, and restore hygiene. It does not accept visible
final pre-load marker output, post-load RP1 UART0 FR value output, RP1
mapped/read-value behavior, RP1 unmapped/trap behavior, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, or phase transition. A further same-shaped
delayed-marker FR-read rerun is not a valid next step; supervisor planning is
required for a non-repetitive post-handoff or serial-marker visibility
discriminator before returning to the RP1 UART0 flag-register read.

phase11-rp1-entry-control-handoff-discriminator-core-20260606 replaces the next
source discriminator with a non-published, no-RP1-MMIO reset-side-effect
candidate. The rpi5_rp1_handoff_reset scenario calls PSCI SYSTEM_RESET
immediately from rust_entry, before BootInfo::from_aarch64_x0, target::init,
boot reports, memory planning, allocator setup, or the RP1 UART0 FR read path.
Static inspection of target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz
proves the 45,248-byte image keeps text_offset=0, header_image_size=45248,
flags=12, ARMd magic, and _start -> rust_entry -> smc #0 side-effect
provenance. The next Pi 5 discriminator may accept only candidate fetch and
pre-BootInfo handoff reachability if a repeated TFTP boot/fetch sequence proves
the reset side effect after one candidate power cycle; RP1 mapped/unmapped
behavior remains blocked.

phase11-rp1-entry-control-handoff-pi5-discriminator-20260606 then published
only that accepted archive. The published tree was
760e7e3c59c3d6d6da4f465c9f67fc53a445bfa18850c6a76f2a3972af680d2d with a
45,248-byte da591740/kernel_2712.img. From one fresh power cycle, same-cursor
stable pre-restore TFTP evidence retained four candidate kernel fetches across
two boot sequences at 05:51:46/05:51:47 and 05:52:04/05:52:05 UTC. That
accepts pre-BootInfo rust_entry handoff reachability by the PSCI reset side
effect. It does not accept TALOS: kernel_main serial visibility, RP1
mapped/read-value, unmapped/trap, firmware-state, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, or Milestone 11.2
behavior. The lab restored the pre-run tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
hardware-lock release.

phase11-rp1-entry-control-handoff-closeout-20260606 reconciles that source and
hardware evidence as pre-bootinfo-handoff-reachability-accepted. The accepted
boundary is limited to candidate fetch and the rust_entry-to-PSCI-reset side
effect before BootInfo parsing, target initialization, boot reports, memory
planning, allocator setup, or the RP1 UART0 FR read path. Candidate serial
visibility, entry-control UART marker visibility, RP1 mapped/read-value,
RP1 unmapped/trap, firmware-state behavior, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, and Milestone 11.2
remain unaccepted. The next bounded step requires supervisor planning for a
focused post-handoff observability or entry-control repair before returning to
the serial-reported RP1 UART0 flag-register diagnostic.

phase11-rp1-post-handoff-marker-reset-core-20260606 adds the no-hardware
rpi5_rp1_post_handoff_marker_reset candidate for that focused observability
repair. The candidate enters rust_entry, emits the normal TALOS: rust_entry line
and a unique rpi5-rp1-post-handoff-marker-reset marker/classification through
the current UART10 early-serial path, flushes with wait_uart10_empty_early_phase,
then calls PSCI SYSTEM_RESET. Static archive/disassembly evidence for
target/talos-rpi5-post-handoff-marker-reset-core.tar.gz shows a 51,736-byte
kernel_2712.img, text_offset=0, header_image_size=51736, flags=12, ARMd magic,
_start -> rust_entry -> marker writes -> smc #0, and no RP1 UART0 FR-read
symbol/string. This is source/static evidence only; post-handoff marker
visibility, reset side effect, staging/capture behavior, and restore remain for
the queued serialized Pi 5 discriminator. RP1 mapped/read-value,
unmapped/trap, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, and Milestone 11.2 remain
unaccepted.

phase11-rp1-post-handoff-marker-reset-pi5-discriminator-20260606 published
only that accepted marker/reset archive, but completed as
staging-capture-blocker. The candidate publication staged tree
37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2 with a
51,736-byte `kernel_2712.img`, and fresh serial reached Raspberry Pi
firmware/RP1 output without any `TALOS: rust_entry` or
`rpi5-rp1-post-handoff-marker-reset` marker text. Stable same-cursor TFTP
samples for the candidate, candidate rerun, and restored known-good control
did not retain candidate-tied fetch evidence in their bounded windows. A late
first-run TFTP replay is retained as capture-timing evidence only, not
candidate identity proof, because status had already returned to the restored
tree when it was queried. The closeout classification is
staging-capture-blocked: visible post-handoff serial observability, reset
side-effect evidence, marker-path hang/fault evidence, RP1 UART0 FR-read
readiness, RP1 mapped/unmapped behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, and Milestone 11.2 remain
unaccepted. The queued RP1 UART0 FR-read refresh is therefore not
mechanically unblocked.

phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5-20260606 reran the
same accepted marker/reset archive under the repaired capture-invariant rule.
Preflight and final pre-restore identity matched selected tree
37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2,
effective kernel kernel_2712.img, and the 51,736-byte
da591740/kernel_2712.img. Stable same-cursor TFTP evidence from fresh cursor
4111814 retained 65 events and 10 served candidate kernel fetches before
restore. Fresh serial from cursor 4113931 retained 19,625 bytes over 90
seconds and showed repeated firmware NETWORK boot/fetch cycles, but did not
show `TALOS: kernel_main` or `rpi5-rp1-post-handoff-marker-reset`.
The capture-recheck closeout classification is
reset-side-effect-accepted-marker-visibility-blocked: candidate fetch and the
PSCI reset-loop side effect are accepted for the selected no-RP1-MMIO
marker/reset candidate only. Visible post-handoff serial observability,
RP1 UART0 FR-read readiness, RP1 mapped/read-value, RP1 unmapped/trap,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain
unaccepted. The queued RP1 UART0 FR-read refresh is still not mechanically
unblocked because reset-side-effect-only evidence is not visible marker
observability.

phase11-rp1-rust-entry-uart10-marker-loop-core-20260606 adds the next
no-hardware discriminator candidate without changing RP1 source. The
rpi5_rust_entry_uart10_marker_loop scenario branches directly from rust_entry
to a repeated UART10 marker loop before BootInfo parsing, target::init, boot
reports, memory planning, allocator setup, scheduler work, PSCI SYSTEM_RESET,
or RP1 UART0 MMIO. Static archive/disassembly evidence for
target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz shows a
45,328-byte kernel_2712.img, arm64 Image fields text_offset=0,
header_image_size=45328, flags=12, and _start -> rust_entry ->
run_rust_entry_uart10_marker_loop. The candidate marker is TALOS: reu10-loop;
string review confirms the RP1 UART0 FR-read report strings are absent. This
accepts only the source/static marker-loop candidate and does not accept
visible marker serial output, RP1 UART0 FR-read readiness, RP1
mapped/unmapped behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, or phase transition. The
queued RP1 UART0 FR-read refresh remains blocked until the marker-loop
hardware discriminator and closeout accept visible Rust-entry UART10 marker
observability.

phase11-staging-capture-log-stability-core-20260605 repairs the Pi 5
proof-rule boundary exposed by that blocker. Replay from the retained cursor
`4088847` later returned 13 TFTP events, including a restored known-good
104,136-byte `da591740/kernel_2712.img` fetch, so future hardware proofs must
not classify a single empty `/tftp/logs?cursor=<fresh>` response as no-fetch
evidence. A proof must capture a fresh TFTP cursor immediately before power
cycle, observe serial, then re-query the same TFTP cursor until `cursor_end`,
`log_size`, `truncated`, and parsed events are stable for the accepted
sample count or until a bounded timeout is recorded. Fetch-byte classification
must happen before restore; any zero-event result is meaningful only after this
stable-log rule. This changes evidence semantics only and does not accept
entry-control reachability or RP1 mapped/unmapped behavior.

phase11-staging-capture-known-good-pi5-proof-20260605 then applied that rule to
the restored accepted boot tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` with
`effective_kernel=kernel_2712.img`. Two serialized known-good power cycles
captured fresh serial and TFTP cursors, stable pre-restore TFTP deltas, and
restore evidence, but both stable deltas had zero events and serial did not
reach `TALOS: kernel_main`, command-loop readiness, or PASS. The deployed lab
API also returned `404` for `GET /`, so this proof used the documented
`GET /status` boot identity endpoint. Classification remains
`staging-capture-still-blocked`; the repaired rule is not yet accepted for RP1
candidate reuse.

phase11-staging-capture-repair-closeout-20260605 accepts the stable TFTP
evidence semantics only. It does not accept the lab/staging path for RP1
candidate reruns because the known-good control still produced stable
zero-event TFTP deltas and no Talos serial readiness. The next bounded task
must be supervisor-planned around lab-controller/capture or staging-publication
discrimination before any RP1 diagnostic/source changes, candidate rerun,
Milestone 11.2, networking, SSH, GPIO, interrupts, DMA/cache, storage,
generated-root, or broader PCIe work.

phase11-lab-evidence-contract-repair-core-20260605 repairs the proof contract
for the next discriminator without touching hardware, boot publication, or RP1
runtime code. The deployed lab API's authoritative boot identity endpoint is
`GET /status`; `GET /` returning `404 unknown endpoint: GET /` is recorded as
endpoint-semantics evidence only. The next proof checklist must retain
`GET /status`, `GET /boot/files`, `GET /boot/snapshots`, fresh serial and TFTP
cursors, stable pre-restore TFTP evidence, and final pre-restore
status/boot-file samples when inconclusive. Classification now separates
`staging-publication-mismatch`, `tftp-capture-logging-blindness`,
`serial-only-firmware-reboot`, and `valid-known-good-talos-readiness` without
accepting candidate fetch, Rust entry, RP1 mapped/read-value, RP1 unmapped/trap,
or Milestone 11.2 behavior.

phase11-known-good-capture-staging-pi5-discriminator-20260605 ran one
serialized power cycle on the restored accepted boot tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` with
`effective_kernel=kernel_2712.img`. Final stable pre-restore TFTP evidence
from fresh cursor `4094251` contained 13 events, including two served
104,136-byte `da591740/kernel_2712.img` fetches, so the discriminator observed
known-good capture/staging before restore. Serial from the fresh cursor reached
Raspberry Pi firmware/RP1 boot output but did not reach `TALOS: kernel_main`,
command-loop readiness, or PASS. The task is classified
`known-good-fetch-observed-without-talos-readiness`; it accepts no RP1
candidate fetch, Rust entry, RP1 mapped/read-value, RP1 unmapped/trap, GPIO,
interrupt, DMA/cache, networking, SSH, storage, broader PCIe, or Milestone 11.2
behavior.

phase11-staging-capture-discriminator-closeout-20260605 accepts the repaired
proof semantics and known-good capture/staging health only. It reconciles the
initial zero-event TFTP sample as capture-latency evidence superseded by the
final stable pre-restore replay. The remaining blocker is
boot-runtime-readiness-after-known-good-fetch: the restored known-good tree
fetched kernel_2712.img, but serial did not reach Talos runtime readiness.
RP1 candidate/source work, candidate reruns, mapped/unmapped claims, GPIO,
interrupts, DMA/cache, networking, SSH, storage, generated-root, broader PCIe,
Milestone 11.2, and phase transition remain blocked pending supervisor
planning.

phase11-known-good-runtime-readiness-contract-core-20260605 repairs that
blocker contract without hardware. The next known-good runtime proof must keep
the restored tree identity
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
`effective_kernel=kernel_2712.img`, and the stable pre-restore
104,136-byte `da591740/kernel_2712.img` fetch separate from runtime
readiness. Runtime readiness requires a bounded serial observation from the
fresh cursor with a 75-second timeout, 1000 ms settle, 65536-byte cap, and
the markers `TALOS: kernel_main` plus
`rpi5-production-timer-preemption: PASS` for the current restored control.
`scripts/rpi5-observe-runtime-readiness.sh` records that classification. This
does not accept RP1 candidate fetch, Rust entry, entry-control reachability,
mapped/read-value, unmapped/trap, GPIO, interrupt, DMA/cache, networking, SSH,
storage, generated-root, broader PCIe, Milestone 11.2, or phase transition.

phase11-known-good-runtime-readiness-pi5-discriminator-20260605 applied that
contract to one serialized known-good power cycle. Stable replay from retained
fresh TFTP cursor `4095602` returned 13 events on both checks, including two
served 104,136-byte `da591740/kernel_2712.img` fetches, but the bounded serial
window retained only 708 bytes of Raspberry Pi firmware/RP1 output and did not
reach `TALOS: kernel_main`, `talos>`, or
`rpi5-production-timer-preemption: PASS`. The closeout classification is
`known-good-fetch-accepted-runtime-readiness-blocked`; RP1 entry-control
candidate rerun remains blocked until supervisor-planned work accepts
valid known-good Talos runtime readiness or repairs the blocker.

phase11-known-good-runtime-lineage-and-cursor-repair-20260605 then fixed the
blank-cursor caveat for the reusable TFTP helpers and mapped the restored
known-good tree to the same 104,136-byte da591740/kernel_2712.img image.
phase11-known-good-runtime-direct-cursor-pi5-recheck-20260605 repeated the
known-good hardware proof with fresh serial cursor 4096040 and fresh
authoritative TFTP cursor 4096953. Stable pre-restore replay retained 13
events, including two served 104,136-byte da591740/kernel_2712.img fetches,
and pre-run, pre-restore, and post-restore status all retained the restored
tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
with effective_kernel=kernel_2712.img. The direct-cursor closeout
classification is known-good-direct-cursor-fetch-runtime-readiness-blocked:
fetch visibility and restore hygiene are accepted, but the serial readiness
window still did not reach TALOS: kernel_main, talos>, or
rpi5-production-timer-preemption: PASS. RP1 entry-control candidate rerun,
candidate fetch, Rust entry, entry-control reachability, mapped/read-value,
unmapped/trap, and firmware-state behavior remain blocked until a
supervisor-planned boot/runtime readiness repair or discriminator accepts valid
known-good Talos runtime readiness.

phase11-known-good-runtime-serial-window-contract-20260606 repairs the
serial-observation side of that blocker without running hardware. Static
inspection showed the previous helper used one settled `/serial/observe` call,
which could stop after the first 708-byte firmware/RP1 burst and before the
later network/TFTP/kernel window. `scripts/rpi5-observe-runtime-readiness.sh`
now loops until the requested deadline from the fresh serial cursor, advances
the observe cursor between calls, accumulates serial text, and records
`observe_contract=deadline-loop-accumulated-from-fresh-cursor`. The next
serialized known-good proof is ready only for this serial-window discriminator:
it must retain selected boot identity, stable pre-restore TFTP fetch evidence,
the deadline-looped serial JSON, pre-restore state, restore, and lock-release
evidence before it can accept or reject valid known-good Talos readiness. RP1
candidate rerun/source work remains blocked until that proof and closeout
accept valid known-good Talos readiness.

phase11-known-good-runtime-serial-window-pi5-discriminator-20260606 then
retained a deadline-looped fresh serial window and stable pre-restore TFTP
evidence for the restored known-good tree. TFTP replay showed two served
104,136-byte `da591740/kernel_2712.img` fetches. The fresh serial window
omitted `TALOS: kernel_main` but reached
`rpi5-production-timer-preemption: PASS`. The marker-boundary review
`phase11-known-good-runtime-marker-boundary-review-core-20260606` accepts that
as `valid-known-good-talos-readiness-by-downstream-marker` for the current
restored production-timer control: source order proves the PASS line is emitted
only after the Pi 5 path has entered `kernel_main` and completed the
production-timer proof predicates. This does not accept RP1 candidate fetch,
Rust entry, entry-control reachability, mapped/read-value, unmapped/trap,
firmware-state behavior, GPIO, interrupts, DMA/cache, networking, SSH, storage,
generated-root, broader PCIe, Milestone 11.2, or phase transition. The queued
marker-boundary closeout must reconcile this classification before any RP1
candidate rerun is promoted.

phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator-20260606 accepts
visible post-handoff Rust-entry UART10 marker observability for the selected
marker-loop candidate only. The published candidate tree
`1d7cdd3d265fb983ec77d9281098d6a920e0bc957a1f0a15f279fe35c618ee6c`
served two 45,328-byte `da591740/kernel_2712.img` fetches in stable
pre-restore TFTP evidence. The deadline-looped fresh serial window retained
60,748 bytes over 32 seconds and reached `TALOS: reu10-loop` 2,961 times.
The run restored the boot tree to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
This evidence accepts UART10 marker visibility at Rust entry but does not
accept RP1 UART0 FR-read readiness, RP1 mapped/read-value behavior,
unmapped/trap behavior, firmware-state behavior, GPIO, interrupts, DMA/cache,
networking, SSH, storage, generated-root, broader PCIe, Milestone 11.2, or a
phase transition. The marker-loop closeout must reconcile this boundary before
any RP1 UART0 flag-register refresh is promoted.

phase11-rp1-rust-entry-uart10-marker-loop-closeout-20260606 reconciles the
source/static marker-loop candidate and Pi 5 hardware run as
post-handoff-rust-entry-uart10-marker-visible. The accepted boundary is limited
to visible UART10 marker output after rust_entry for the selected
target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz candidate. It does
not accept RP1 mapped/read-value behavior, unmapped/trap behavior,
firmware-state behavior, GPIO, interrupts, DMA/cache, networking, SSH,
storage, generated-root, broader PCIe, Milestone 11.2, or phase transition.
The existing phase11-rp1-uart0-fr-read-refresh-core-20260606 task is now
mechanically unblocked to refresh the narrow local RP1 UART0 flag-register read
candidate; the serialized hardware proof remains separately gated behind that
refresh.

phase11-rp1-uart0-fr-read-closeout-20260606 reconciles the refreshed local
RP1 UART0 flag-register read candidate and the serialized Pi 5 proof as
serial-capture-saturated-after-candidate-fetch. The refreshed candidate staged
tree `25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71`
with a 45,832-byte `da591740/kernel_2712.img`, and the first proof run
retained stable same-cursor pre-restore TFTP evidence with two served
candidate kernel fetches. The fresh serial cursor was already `4194304`, and
candidate, known-good control, and candidate rerun observations from that
cursor returned zero bytes. The known-good control did retain two 104,136-byte
control kernel fetches; the candidate rerun retained stable zero-event TFTP
evidence. The accepted boundary is limited to source/static candidate refresh,
first-run candidate publication/fetch, restore hygiene, and serial-capture
blocker evidence. It does not accept `mapped/read-value`, unmapped/trap,
firmware-state, pre-MMIO reachability, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition. Another same-shaped RP1 UART0 FR-read hardware rerun remains
blocked until serial cursor/capture completeness is repaired or decisively
classified.

phase11-known-good-serial-cursor-completeness-closeout-20260606 reconciles the
repair-core task and the serialized known-good Pi 5 proof as
serial-cursor-capture-completeness-accepted. The restored accepted tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` served
two 104,136-byte `da591740/kernel_2712.img` fetches in stable pre-restore
TFTP evidence. Starting from saturated serial cursor `4194304`, the repaired
direct-read path retained 6,347 fresh bytes, including firmware NETWORK output
and `rpi5-production-timer-preemption: PASS`. A future explicitly queued RP1
UART0 FR-read rerun may use this repaired proof path without repeating the
cursor-saturation failure class, but the rerun must still independently prove
its candidate identity, serial/TFTP evidence, restore proof, and exact RP1
classification. This closeout does not accept RP1 mapped/read-value behavior,
unmapped/trap behavior, firmware-state behavior, pre-MMIO reachability, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or a phase transition.

phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun-20260606 then reran the
same refreshed RP1 UART0 FR-read candidate with the repaired serial capture
path. The candidate archive SHA-256 was
`da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60`, the
published tree was
`25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71`, and
stable pre-restore TFTP evidence from cursor `4129377` served the 45,832-byte
`da591740/kernel_2712.img` twice. Starting from saturated serial cursor
`4194304`, the repaired direct-read path retained 4,470 bytes of fresh
firmware NETWORK output, but the serial window did not show
`TALOS: kernel_main`, `rpi5-rp1-uart0-fr-read: start`,
`rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or `PASS`.
The classification is `candidate-fetch-reset-loop-without-visible-fr-marker`.
This accepts candidate publication/fetch evidence and restore hygiene only; it
does not accept RP1 mapped/read-value behavior, unmapped/trap behavior,
firmware-state behavior beyond the candidate fetch/reset-loop evidence, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or a phase transition.

phase11-rp1-uart0-fr-read-repaired-cursor-closeout-20260606 reconciles that
rerun as blocker evidence, not an RP1 mapping proof. It removes the prior
serial-cursor-saturation explanation because the repaired direct-read path did
retain fresh candidate serial bytes. The accepted boundary remains limited to
candidate archive identity, publication/fetch evidence, repaired serial
capture of firmware NETWORK output, and restore hygiene. RP1 UART0 FR-read
start/pre-MMIO marker visibility, mapped/read-value behavior, unmapped/trap
behavior, firmware-state behavior beyond candidate fetch/reset-loop evidence,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, and phase transition remain unaccepted. The next
discriminator must be supervisor-planned and non-repetitive; another
same-shaped FR-read hardware rerun is not mechanically unblocked without a
new source/static or marker-path discriminator.

phase11-rp1-uart0-fr-shaped-no-mmio-marker-core-20260606 adds that
non-repetitive source/static discriminator. The
`rpi5_rp1_uart0_fr_shaped_no_mmio_marker` scenario follows the FR-read-shaped
selection from `rust_entry`, emits the same
`rpi5-rp1-uart0-fr-read: start` and `pre-mmio-read` UART10 lines, reports
`classification=no-mmio-marker-before-rp1-read`, flushes UART10, and then
repeats `TALOS: fr-no-mmio-loop`. Static disassembly shows the marker path
does not call `read_rp1_reg_u32`, does not construct `0x1f_0003_0018`, and
does not execute RP1 UART0 FR MMIO before the marker loop. The non-published
candidate archive is
`target/talos-rpi5-rp1-uart0-fr-shaped-no-mmio-marker-core.tar.gz` with
archive SHA-256
`05a6801471ffd5cb3ae61f450734728f7980d8a2c4db20b3a6280d83b470a484`, boot-tree
identity
`05f68072e4f1653c10eadfefbe099c92cefdde024b7f7d985b7c785c48011e45`, and a
45,600-byte `kernel_2712.img`. This accepts only the source/static candidate;
visible marker output, RP1 mapped/read-value behavior, unmapped/trap behavior,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain
unaccepted pending the queued serialized Pi 5 discriminator.

phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator-20260606 then
published only that accepted no-MMIO marker archive. The accepted clean run
staged tree `2bd7db27d7bdf27a356c81408fefce059148f61e332fb3a207d280913b6ec27d`
with a 45,600-byte `da591740/kernel_2712.img`. Stable same-cursor
pre-restore TFTP evidence from cursor `4134781` retained 13 events and two
served candidate kernel fetches. Starting from saturated serial cursor
`4194304`, the repaired direct-read path retained 70,004 bytes with firmware
NETWORK output and 2,730 occurrences of `TALOS: fr-no-mmio-loop`. This
accepts only that the FR-read-shaped path reaches UART10 pre-MMIO marker output
when the volatile RP1 UART0 FR load is absent. It does not accept RP1 UART0
FR-read mapped/read-value behavior, unmapped/trap behavior, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, or a phase transition. The lab restored the
pre-run tree `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
before hardware-lock release.

phase11-rp1-uart0-fr-shaped-no-mmio-marker-closeout-20260606 reconciles that
source/static and Pi 5 evidence as fr-shaped-no-mmio-marker-visible. The
accepted boundary remains limited to the selected FR-read-shaped path reaching
visible UART10 pre-MMIO marker output when the volatile RP1 UART0 FR load is
absent. This closeout does not accept RP1 UART0 FR volatile-read execution,
mapped/read-value behavior, unmapped/trap behavior, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, or a phase transition. Any actual RP1 UART0 FR-read
hardware proof needs a new supervisor-planned task with explicit acceptance
gates.

phase11-rp1-uart0-fr-read-delayed-marker-core-20260606 adds the next
source/static candidate without running hardware. The
rpi5_rp1_uart0_fr_read_delayed_marker scenario keeps the accepted FR-shaped
UART10 start and pre-MMIO report path, emits 32 bounded
TALOS: fr-delayed-preload-loop markers, emits the final
rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker, and then
executes exactly one 32-bit volatile load from the contracted RP1 UART0 FR
address 0x1f_0003_0018. Static disassembly shows post-load contract,
raw-value, mapped/read-value, and PASS output are control-dependent on that
load returning. The non-published archive is
target/talos-rpi5-rp1-uart0-fr-read-delayed-marker-core.tar.gz with archive
SHA-256 90452242f872eb085c9fe7963c02ad67556694326daebd7d199caf4ed5f597f4,
boot-tree identity
bc72d011494343727ebce2a37e4f2d3b14079065f5990100f7c7769f4313fbc6, and a
46,152-byte kernel_2712.img. This accepts only the source/static delayed
marker candidate; RP1 mapped/read-value behavior, trap/no-return behavior,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain
unaccepted pending the queued serialized Pi 5 discriminator.

phase11-rp1-final-preload-marker-hold-core-20260606 adds the next source/static
candidate without running hardware. The rpi5_rp1_final_preload_marker_hold
scenario keeps the delayed-marker FR-read reporting path through the final
pre-load marker, then loops forever on TALOS: fr-final-preload-hold-loop
instead of executing the RP1 UART0 FR volatile load. Static symbol and
disassembly evidence shows _start -> rust_entry ->
run_rp1_final_preload_marker_hold, confirms the required start/pre-MMIO,
before-RP1-read, repeated pre-load, final pre-load, and hold marker strings,
and confirms no read_rp1_reg_u32 symbol/call, no construction/use of
0x1f_0003_0018, and no selected-path RP1 UART0 FR load. The non-published
archive is target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz with
archive SHA-256
07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287, boot-tree
identity ed111afd660d233f95e78a2703c6fd17f12419771e34141ea2dbe3f15ffed3e8, and
a 45,816-byte kernel_2712.img. This accepts only the source/static hold
candidate; visible Pi 5 final marker output, visible hold marker output, RP1
mapped/read-value behavior, trap/no-return behavior, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, and phase transition remain unaccepted pending the queued
serialized Pi 5 marker-visibility discriminator.

phase11-rp1-final-preload-marker-hold-pi5-discriminator-20260606 publishes
that accepted no-RP1-MMIO hold candidate under the hardware lock. Lab status
reported tree
101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47, effective
kernel kernel_2712.img, and the expected 45,816-byte
da591740/kernel_2712.img. Stable pre-restore TFTP evidence retained 13 events
with two served candidate kernel fetches. Direct serial read from the saturated
4194304 cursor retained 57,040 bytes with 1,628 occurrences of
TALOS: fr-final-preload-hold-loop. The result is accepted only as
final-preload-hold-marker-visible for the selected no-RP1-MMIO candidate. The
direct-read window did not retain the earlier final-preload marker, and RP1
mapped/read-value behavior, trap/no-return behavior, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, and phase transition remain unaccepted. The lab was
restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
hardware-lock release.

phase11-rp1-final-preload-marker-hold-closeout-20260606 reconciles the
source/static and Pi 5 discriminator evidence as
final-preload-hold-marker-visible. The accepted boundary is limited to the
source/static no-RP1-MMIO hold candidate shape, candidate publication/fetch
evidence, visible unique hold-marker output from the selected candidate, and
restore hygiene. The direct-read window did not retain the earlier final
pre-load marker, and the candidate intentionally avoided the RP1 UART0 FR
volatile load. Visible final pre-load marker output, RP1 mapped/read-value
behavior, RP1 unmapped/trap behavior, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remain unaccepted. Any return to the
actual RP1 UART0 flag-register read must be supervisor-planned with explicit
source/static and serialized Pi 5 acceptance gates.

phase11-rp1-uart0-fr-read-hold-control-core-20260606 adds that
source/static follow-up candidate. It branches directly from Pi 5 rust_entry
before BootInfo parsing, target initialization, boot reports, memory planning,
allocator setup, scheduler work, or command-loop work. It emits a unique
UART10 pre-read control marker through the accepted hold-marker-visible path,
then performs exactly one contracted 32-bit volatile load from RP1 UART0 FR at
0x1f00030018. If the read returns, it reports contract id
phase11-rp1-pcie-map-contract-v1, target rp1-uart0-fr-read, address, width,
raw value, mapped/read-value classification, and a unique post-read terminal
hold marker. This is accepted only as a local/static/archive candidate; Pi 5
marker visibility for this candidate, RP1 mapped/read-value hardware behavior,
unmapped/trap behavior, firmware-state behavior, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, and
phase transition remain unaccepted until the queued serialized discriminator.

phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator-20260606 publishes
that accepted hold-control FR-read candidate under the hardware lock and
classifies the attempt as capture-staging-blocked. Publication reported lab
tree ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0,
effective kernel kernel_2712.img, and the expected 46,320-byte
da591740/kernel_2712.img. The main direct-read serial window retained 222,783
bytes with 5,582 TALOS: fr-hold-control-post-read-loop occurrences, but stable
same-cursor TFTP evidence for that run recorded restored-tree 104,136-byte
kernel fetches and zero selected 46,320-byte candidate fetches. Required
triage was attempted: the known-good control produced stable zero-event TFTP,
and a candidate rerun again staged the 46,320-byte kernel but produced
zero-event same-cursor TFTP and an empty serial observe file. The lab was
restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
hardware-lock release. This accepts only the capture-staging-blocked
classification; mapped/read-value behavior, bus-fault/trap behavior,
pre-read-control-visible-without-read-result, candidate-fetch-without-control-
marker, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase
transition remain unaccepted.

phase11-pi5-proof-identity-join-repair-core-20260606 formalizes the proof
bundle join rule behind that blocker. Future decisive RP1 hardware
classifications must pass `pi5-proof-identity-join-v1`: the same run label must
tie selected tree hash, effective kernel, expected fetch path and byte count,
serial cursor/window identity, stable TFTP cursor/delta identity, final
pre-restore identity, and restore identity. Replaying the retained
hold-control candidate-run evidence rejects decisive RP1 classification because
the TFTP delta served 104,136-byte restored-tree kernel fetches, final
pre-restore identity had already changed to the restored tree, and neither
matches the selected 46,320-byte hold-control candidate. The accepted boundary
therefore remains capture-staging-blocked; RP1 mapped/read-value and
unmapped/trap behavior remain unaccepted.

phase11-pi5-proof-identity-join-known-good-control-20260606 and
phase11-pi5-proof-identity-join-repair-closeout-20260606 accept the repaired
proof chain as ready for a later candidate rerun. The known-good control used
selected tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, effective
kernel_2712.img, expected da591740/kernel_2712.img byte count 104,136, stable
same-cursor TFTP evidence with two expected 104,136-byte fetches, a fresh
direct-read serial window containing rpi5-production-timer-preemption: PASS,
final pre-restore identity, restore evidence, and post-restore identity. The
identity-join checker reported proof-chain-ready-for-candidate-rerun with no
rejection reasons. This accepts only proof-chain readiness; the next RP1 UART0
FR-read hold-control candidate proof must pass the same
pi5-proof-identity-join-v1 gate before it can accept mapped/read-value,
bus-fault/trap, or other decisive RP1 behavior.

phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5-20260606 then ran
that repaired-proof candidate task and kept the hardware boundary
capture-staging-blocked. The accepted candidate archive was published as the
46,320-byte `da591740/kernel_2712.img` in lab tree
`ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0`, and the
main run retained 973,431 bytes of direct-read serial containing 24,796
`TALOS: fr-hold-control-post-read-loop` occurrences. The repaired
`pi5-proof-identity-join-v1` gate rejected that serial as non-decisive because
stable same-cursor TFTP and final pre-restore identity matched restored
known-good 104,136-byte fetches instead of the selected 46,320-byte candidate
fetch. The required known-good control passed the repaired gate, and one
candidate rerun was stopped after recovery evidence again lacked candidate-byte
TFTP identity. This accepts only capture-staging-blocked; RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, and phase transition remain unaccepted.

phase11-pi5-capture-transaction-forensics-core-20260606 repairs the proof
transaction contract after that mismatch. The retained f274ff7 evidence is
explained as serial-freshness-contract-fixed: v1 rejected the restored-tree
TFTP/final identity mismatch, but it did not require proof that saturated
direct-read serial began after an empty pre-power drain. The new
`pi5-capture-transaction-v2` bundle records
`serial-drain-before-power.json` and rejects saturated direct-read output
unless the pre-power `/serial/read` drain reaches an empty device-buffer
read; `/serial/peek` is only retained-tail/cursor evidence, not drain
proof. Replaying f274ff7 under v2
remains capture-staging-blocked due to missing v2 drain evidence plus
restored-tree TFTP/final identity. This accepts only proof-contract readiness
for a no-RP1-MMIO sentinel; RP1 UART0 FR mapped/read-value behavior,
bus-fault/trap behavior, firmware-state behavior, broader PCIe, Milestone
11.2, and phase transition remain unaccepted.

phase11-pi5-capture-transaction-no-mmio-sentinel-pi5-20260606 validates the
v2 contract on the accepted no-MMIO final-preload-marker hold archive. The
clean rerun staged tree
`101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47`,
proved an empty pre-power `/serial/read` drain, retained two stable
45,816-byte `da591740/kernel_2712.img` fetches, retained final
pre-restore selected-tree identity, and captured 7,489 occurrences of
`TALOS: fr-final-preload-hold-loop` before restoring to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
This accepts the capture transaction as no-mmio-sentinel-identity-joined for
the proof chain only; RP1 UART0 FR mapped/read-value, trap/unmapped, and
firmware-state behavior remain unaccepted until a separate RP1 proof task.

phase11-pi5-capture-transaction-v2-closeout-20260606 reconciles the
forensics repair and no-MMIO sentinel evidence as
proof-chain-ready-for-rp1-fr-read-v2. The accepted boundary is limited to the
v2 proof contract and the sentinel identity join: selected tree, effective
kernel, empty pre-power /serial/read drain, fresh marker serial, stable
pre-restore TFTP, final pre-restore identity, and restore proof all tied to one
run label. The next mechanically unblocked task is the queued RP1 UART0
FR-read hold-control v2 proof, but that proof must independently pass the same
v2 identity join for the selected RP1 candidate before any mapped/read-value,
trap/unmapped, firmware-state, broader PCIe, Milestone 11.2, or phase
transition claim is accepted.

phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5-20260606 then published
the accepted hold-control RP1 UART0 FR-read candidate under the hardware lock
and classified the proof as candidate-fetch-without-control-marker. The first
candidate run had selected-tree TFTP/final identity, but v2 rejected decisive
classification because the pre-power /serial/read drain was not empty before a
saturated direct-read serial window. After rebooting to the restored
known-good tree, the known-good control passed v2 identity join with two
104,136-byte control kernel fetches and PASS serial output. The single
candidate rerun then passed v2 identity join for tree
ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0 with two
46,320-byte da591740/kernel_2712.img fetches, final selected-tree identity,
and restore proof. Its direct-read serial window retained 27,177 occurrences
of TALOS: fr-hold-control-post-read-loop, but did not retain the contracted
rpi5-rp1-uart0-fr-read read-value/classification line, pre-read control
marker, post-read terminal marker, or trap/panic text.

phase11-rp1-uart0-fr-read-hold-control-v2-closeout-20260606 reconciles that
boundary as candidate-fetch-without-control-marker. It accepts selected
candidate publication/fetch, v2 identity join, visible post-read-loop-tail
output without contracted control/read-result markers, the known-good v2
proof-chain control, and restore hygiene. It does not accept RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior,
pre-read-control-visible-without-read-result, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition. The v2 proof contract is no longer the
blocker, but another same-shaped RP1 FR-read hardware rerun is not progress by
itself; the next step requires supervisor planning for a qualitatively
different discriminator that explains why the selected candidate produces the
post-read-loop tail without retaining the contracted control/read-result/trap
markers.

phase11-rp1-uart0-fr-tail-stable-result-core-20260606 adds that next
source/static discriminator. The RP1 candidate branches directly from
rust_entry, emits rpi5-rp1-uart0-fr-tail-stable-result: before-rp1-load,
executes exactly one 32-bit volatile load from RP1_UART0_FR
(0x1f_0003_0018), and, only if the load returns, repeatedly emits
TALOS: fr-tail-stable-result with contract id, target, address, width, raw
value, and classification=mapped/read-value. A matching no-RP1-MMIO control
candidate branches directly from rust_entry, emits
rpi5-rp1-uart0-fr-tail-stable-control: no-rp1-mmio, constructs no RP1 FR
address, executes no RP1 volatile load, and repeatedly emits
TALOS: fr-tail-stable-control with the same compact result-output shape and
classification=simulated/control.

Static assembly evidence accepts only the local candidate shape: one RP1 load
in the RP1 candidate and zero RP1 loads in the control candidate. RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, and phase transition remain unaccepted. The next
hardware control must first prove the no-MMIO tail-stable output shape is
capturable on Pi 5 before any RP1 mapped/read-value proof can be attempted.

phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5-20260606 accepts that
hardware control as tail-stable-control-visible. The accepted candidate rerun
published only the no-RP1-MMIO tail-stable control archive and selected tree
`b4b780193281538a643aec3c17898ae59204c335f32452b90cf08b0cb8e10161` with a
45,728-byte `da591740/kernel_2712.img`. The v2 identity join passed with an
empty pre-power `/serial/read` drain, two stable 45,728-byte TFTP fetches,
final pre-restore selected-tree identity, and restore to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`. The
saturated direct-read window retained 1,771 occurrences of
`TALOS: fr-tail-stable-control`.

This accepts only the no-MMIO simulated/control tail-stable output shape and
proof-chain readiness for the queued RP1 tail-stable result proof. RP1 UART0
FR mapped/read-value behavior, bus-fault/trap behavior, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, and phase transition remain unaccepted
until a separate RP1 proof passes its own hardware lock and classification
gates.

phase11-rp1-uart0-fr-tail-stable-control-closeout-20260606 reconciles the
paired source/static discriminator and no-MMIO Pi 5 control as
tail-stable-control-visible. The accepted boundary is limited to the
one-load/zero-load candidate design, the v2 identity-joined no-MMIO control
rerun, two 45,728-byte candidate TFTP fetches, final selected-tree identity,
restore hygiene, and repeated simulated/control marker retention. This
mechanically unblocks the queued RP1 tail-stable result proof under hardware
lock and supervisor-intervention rules, but still does not accept RP1 UART0 FR
mapped/read-value, bus-fault/trap, firmware-state, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, or a phase transition.

phase11-rp1-uart0-fr-tail-stable-result-pi5-20260606 accepts the queued RP1
tail-stable result proof as mapped-read-value-tail-stable. After an
inconclusive first capture and a passing known-good control, the decisive
candidate rerun selected tree
`0e187f9f73118c237337b25d85e57c51dbf18a18bf87ab0d3850c63291b153eb`, fetched
the 45,800-byte `da591740/kernel_2712.img` twice, passed the v2 identity join
with an empty pre-power serial drain, preserved final selected-tree identity,
and restored to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`. The
serial window retained 1,498 occurrences of
`TALOS: fr-tail-stable-result contract=phase11-rp1-pcie-map-contract-v1 target=rp1-uart0-fr-read address=0x1f00030018 width=32 raw=0xdeaddead classification=mapped/read-value`.
This accepts only the first read-only RP1 UART0 FR mapped/read-value diagnostic
boundary. GPIO/pin-control ownership, RP1 clocks/resets, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain unaccepted.

phase11-rp1-uart0-fr-tail-stable-result-closeout-20260606 reconciles the
source/static core, no-MMIO control, and RP1 tail-stable Pi 5 proof as
mapped-read-value-tail-stable. The accepted frontier is the read-only RP1
UART0 FR single-load diagnostic at 0x1f00030018, tied by v2 candidate
identity, stable TFTP, final pre-restore identity, restore proof, and repeated
tail-stable result markers carrying raw 0xdeaddead. No queued task remains
after the closeout; supervisor planning is required before any next Phase 11
slice. GPIO/pin-control ownership, clocks/resets, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, and
phase transition remain unaccepted.

phase11-rp1-mapping-frontier-checkpoint-20260607 accepts that as the
Milestone 11.1 frontier and explicitly authorizes only the next source-contract
slice, phase11-rp1-irq-clock-gpio-source-contract-20260607. The accepted claims
remain limited to the read-only RP1 UART0 FR single-load diagnostic at
0x1f00030018 with v2 identity, stable TFTP, final pre-restore identity,
restore proof, and repeated mapped/read-value markers. GPIO/pin-control
ownership, clocks/resets, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, and Milestone 11.2 implementation
remain unaccepted until later tasks supply their own source contract and proof.

phase11-rp1-interrupt-routing-source-contract-20260607 accepts the next
Milestone 11.2 source contract only. The selected diagnostic target is
`rp1-io-bank0-msix-cfg-read`: a read-only/no-enable 32-bit volatile load from
RP1 `RP1_PCIE_APBS` `MSIX_CFG(0)` at CPU physical `0x1f00108008`. Source
inspection ties this to `RP1_INT_IO_BANK0 = 0`, the `rp1_gpio` bank0 parent
interrupt, Linux's RP1 irqdomain/MSI-X vector path, and the BCM2712
`pcie2`/`mip0` route that predicts GIC SPI 128 / INTID 160 for MSI vector 0.
That GIC route remains a source-backed assumption, not accepted hardware
delivery. The required paired control must preserve the output shape while
performing no RP1 GPIO/RIO/PADS/clock/reset/MSI-X, PCIe config/MSI/MIP, or GIC
MMIO. Interrupt enablement, ISR installation, delivery, GPIO ownership,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-interrupt-routing-diagnostic-core-20260607 implements that source
contract as a local/static diagnostic pair. The real candidate performs exactly
one 32-bit volatile load from `0x1f00108008` and reports the source-predicted
IO_BANK0 hwirq, MSI-X vector, and GIC route only if the load returns. The
paired no-MMIO/no-enable control preserves the same terminal output shape with
`address=not-constructed` and `classification=simulated/control`, constructs
no forbidden RP1/MSI-X/PCIe/MIP/GIC address, and performs zero forbidden MMIO
loads or stores. No hardware behavior is accepted by this local/static core.

phase11-rp1-interrupt-routing-no-mmio-control-pi5-20260607 accepts only the
serialized Pi 5 no-MMIO/no-enable control output/capture path. The accepted
rerun selected tree
`c4d59ab46368e4f79f59b10543d54cf6b2198e86f57b7a2e0bfdf8c2313dc1ae`,
retained two served 46,520-byte `da591740/kernel_2712.img` TFTP fetches,
passed the v2 identity join with no rejection reasons, and retained 990
occurrences of `TALOS: rp1-interrupt-routing-control`. This proves only that
the simulated/control no-MMIO output shape is visible through the repaired
capture path before the real diagnostic. Real RP1 MSIX_CFG read behavior,
interrupt routing, interrupt delivery, GPIO ownership, pin-control, clocks,
resets, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-interrupt-routing-diagnostic-pi5-20260607 accepts the selected
real read-only/no-enable interrupt-routing diagnostic boundary on Pi 5. After a
first candidate run and known-good control were rejected by non-empty
pre-power serial drain evidence, the accepted diagnostic rerun selected tree
`63800845c9837b3d57153051583b269070b028412bcd57ea9c55a5f9e56a2304`,
retained two served 46,648-byte `da591740/kernel_2712.img` TFTP fetches,
passed the v2 identity join with no rejection reasons, and retained 970
occurrences of `TALOS: rp1-interrupt-routing-result`. The visible result
reported contract `phase11-rp1-interrupt-routing-source-contract-v1`, target
`rp1-io-bank0-msix-cfg-read`, hwirq 0, predicted MSI-X vector 0, predicted
GIC SPI 128 / INTID 160, address `0x1f00108008`, width 32, raw
`0xdeaddead`, enable=true, test=false, iack=true, iack-en=true, and
classification=routing-msix-cfg-visible. This accepts only that MSIX_CFG(0)
read/result boundary. Interrupt delivery, handler ownership, GPIO ownership,
pin-control behavior, pad writes, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted.

## Milestone 11.2 PCIe2 Host-Link Status Boundary

phase11-rp1-pcie-endpoint-config-discriminator-source-contract-20260608
accepts
phase11-rp1-pcie-endpoint-config-discriminator-source-contract-v1. The
selected target is `pcie2-host-link-status-read`: a single read-only 32-bit
volatile load from BCM2712 pcie2 `PCIE_MISC_PCIE_STATUS` at CPU physical
`0x1000124068`. Source inspection ties this address to Linux `bcm2712.dtsi`
pcie2 register base `0x10_0012_0000` plus the Broadcom STB PCIe driver status
offset `0x4068`.

The accepted status fields are `pcie_port` bit `0x80`, `dl_active` bit
`0x20`, `phylinkup` bit `0x10`, `link_in_l23` bit `0x40`, and
`status_is_deaddead`. A non-sentinel status with `dl_active=true` and
`phylinkup=true` separates visible PCIe2 host/link state from the retained RP1
SYSINFO/clock-window sentinel and classifies as
`pcie2-host-link-up-rp1-window-sentinel`. A non-sentinel status without both
link-up bits classifies as `pcie2-host-status-visible-link-down`. A
`0xdeaddead` status classifies as `pcie2-host-status-sentinel`.

The contract rejects direct endpoint config-space probing in this source-only
slice because the Broadcom STB PCIe driver gates endpoint config access on
link-up and uses an `EXT_CFG_INDEX` write before reading `EXT_CFG_DATA`; the
same driver notes config access without link-up can cause a CPU abort. The
paired control must preserve output shape while constructing no BCM2712 PCIe,
RP1 peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, or GIC MMIO address. This
source contract does not accept runtime behavior, hardware behavior, live
endpoint config access, broad RP1 mapping, endpoint ownership, PCIe writes,
clock/reset ownership, GPIO ownership, event generation, interrupt delivery,
DMA/cache, networking, SSH, Milestone 11.3, or phase transition.

phase11-rp1-pcie-endpoint-config-discriminator-pi5-20260608 accepts the real
Pi 5 proof as pcie2-host-link-up-rp1-window-sentinel. The accepted rerun
published only target/talos-rpi5-rp1-pcie2-host-link-status-read-core.tar.gz
as selected tree
6d1fa1cd754adf38a023909651bcdc40b6ed08a06b559e79859f545886a59393, retained
two served 46,880-byte da591740/kernel_2712.img TFTP fetches, passed the
pi5-capture-transaction-v2 identity join with no rejection reasons, retained
120 occurrences of TALOS: rp1-pcie2-host-link-status-result, and restored the
lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The visible status report was raw=0x3e0b0, pcie-port=true, dl-active=true,
phylinkup=true, link-in-l23=false, status-is-deaddead=false, and
retained-rp1-window-sentinel=true. This accepts only that the BCM2712 PCIe2
host-link/status register is visible and link-up while the retained RP1
SYSINFO/clock-window path remains sentinel-shaped. It does not accept endpoint
config-space access, broad RP1 mapping, endpoint ownership, PCIe writes,
interrupt delivery, DMA/cache, networking, SSH, Milestone 11.3, or phase
transition.

phase11-rp1-pcie-endpoint-config-discriminator-closeout-20260608 reconciles
that chain as pcie2-host-link-up-rp1-window-sentinel-frontier-closed. The
accepted frontier is limited to the source-backed PCIe2 host-link status
identity, selected read-only PCIE_MISC_PCIE_STATUS snapshot, paired
no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 visibility/link-up proof.
The retained RP1 SYSINFO/clock-window sentinel remains comparator context, not
a broad mapping claim. Same-shaped PCIe2 host-link status hardware reruns are
not progress unless a future supervisor task supplies a different
discriminator or new acceptance criteria. Endpoint config-space access, broad
RP1 mapping, endpoint ownership, PCIe writes, bridge setup, PERST/link-control
changes, MSI/MIP/GIC operations, interrupt delivery, ISR/handler ownership,
clock/reset ownership, GPIO ownership, event generation, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, and phase transition remain
unaccepted. Supervisor planning is required for the next Milestone 11.2
feature slice.

phase11-rp1-endpoint-config-identity-source-contract-20260608 accepts
phase11-rp1-endpoint-config-identity-source-contract-v1. The selected target
is `rp1-endpoint-config-vendor-device-read`: after confirming the accepted
PCIe2 host-link precondition through `PCIE_MISC_PCIE_STATUS`, write exactly
`0x00100000` to BCM2712 pcie2 `EXT_CFG_INDEX` at CPU physical
`0x1000129000`, then read exactly one 32-bit dword from `EXT_CFG_DATA + 0` at
CPU physical `0x1000128000`. The selected BDF/offset is domain 2, bus 1,
device 0, function 0, offset `0x0`; the expected RP1 identity is vendor
`0x1de4`, device `0x0001`.

The source contract treats the `EXT_CFG_INDEX` write as a bounded controller
target selector for the following read-only config-data access, not as
endpoint configuration mutation, BAR programming, bridge setup, or
restore-owned state. The accepted classifications are
`rp1-endpoint-config-id-visible`, `rp1-endpoint-config-id-unexpected`,
`rp1-endpoint-config-id-all-ones`, `rp1-endpoint-config-id-zero`,
`rp1-endpoint-config-id-sentinel`, `rp1-endpoint-config-link-down-skip`,
`rp1-endpoint-config-id-inconclusive-capture`,
`no-mmio-rp1-endpoint-config-id-control-visible`, and
`staging/build-blocker`. The paired control must preserve output shape while
constructing no BCM2712 PCIe, RP1 peripheral/SYSINFO/clock/GPIO/MSI-X, MIP,
GIC, or DMA MMIO address. Endpoint config writes, BAR discovery beyond offset
0, bridge setup, PERST/link-control changes, interrupt delivery, DMA/cache,
networking, SSH, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-endpoint-config-identity-pi5-20260608 accepts the real Pi 5
result as `rp1-endpoint-config-id-all-ones`. The accepted run passed
`pi5-capture-transaction-v2` with selected tree
`7e66c8cef268d7a94843c0d8e230f89c25161053f0b326a8375c0b6f4ca97d42`, two
served 48,456-byte candidate kernel fetches, and restore to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
Serial output retained 135 result markers. The report reached the link-up
precondition with `PCIE_MISC_PCIE_STATUS=0x3e0b0`, wrote `0x00100000` to
`EXT_CFG_INDEX`, and read `EXT_CFG_DATA + 0` as `0xffffffff`
(`vendor-id=0xffff`, `device-id=0xffff`). This accepts the all-ones
endpoint config identity frontier only; expected RP1 vendor/device visibility,
endpoint ownership, broad RP1 mapping, endpoint configuration mutation, bridge
setup, interrupt delivery, DMA/cache, networking, SSH, Milestone 11.3, and a
phase transition remain unaccepted.

phase11-rp1-endpoint-config-identity-closeout-20260608 reconciles that chain
as `rp1-endpoint-config-id-all-ones-frontier-closed`. The accepted frontier is
limited to the source-backed bounded endpoint config identity attempt, paired
no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 all-ones config-data result
under identity-joined evidence. Same-shaped endpoint config identity hardware
reruns are not progress unless a future supervisor task supplies a different
discriminator or new acceptance criteria. Expected RP1 vendor/device
visibility, endpoint ownership, broad RP1 mapping, endpoint configuration
mutation, BAR programming or discovery, bridge setup, PERST/link-control,
interrupt delivery, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, and phase transition remain unaccepted. Supervisor planning is
required for the next Milestone 11.2 frontier.

phase11-rp1-bridge-config-preflight-source-contract-20260608 accepts
phase11-rp1-bridge-config-preflight-source-contract-v1. The selected target is
pcie2-bridge-misc-ctrl-preflight-read: read the already accepted PCIe2
host-link status register at 0x1000124068, then read exactly one 32-bit dword
from BCM2712 pcie2 PCIE_MISC_MISC_CTRL at CPU physical 0x1000124008. Retained
Broadcom STB PCIe setup source uses that register to set SCB_ACCESS_EN,
CFG_READ_UR_MODE, max burst size, RCB_MPS_MODE, and RCB_64B_MODE before
inbound-window setup, root-complex class setup, and outbound-window
programming.

The accepted classifications are pcie2-bridge-preflight-ready,
pcie2-bridge-preflight-incomplete, pcie2-bridge-preflight-sentinel,
pcie2-bridge-preflight-link-down-skip,
pcie2-bridge-preflight-inconclusive-capture,
no-mmio-pcie2-bridge-preflight-control-visible, and staging/build-blocker.
The paired control must preserve output shape while constructing no BCM2712
PCIe, RP1 peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC, DMA, or other MMIO
address. This source contract does not retry EXT_CFG_INDEX/EXT_CFG_DATA, and
it does not accept runtime or hardware behavior, expected RP1 vendor/device
visibility, endpoint ownership, broad RP1 mapping, endpoint configuration
mutation, BAR discovery or programming, bridge setup, interrupt delivery,
DMA/cache, networking, SSH, Milestone 11.3, or phase transition.

phase11-rp1-bridge-config-preflight-pi5-20260608 accepts the real Pi 5 result
as pcie2-bridge-preflight-ready. After initial capture-staging-blocked
candidate and known-good evidence caused by a non-empty saturated serial
drain, a bounded drain plus known-good control passed the v2 identity join,
and the unchanged candidate rerun passed with selected tree
e66d21ac433225c19dfa63c09a577c8ab6828ebfdf5a437b57efc5fe0e7f260a, two
served 48,000-byte da591740/kernel_2712.img fetches, and restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Serial
output retained 123 result markers. The report reached the link-up
precondition with PCIE_MISC_PCIE_STATUS=0x3e0b0, then read
PCIE_MISC_MISC_CTRL=0xa8003000 with scb-access-en=true,
cfg-read-ur-mode=true, rcb-mps-mode=false, rcb-64b-mode=false,
max-burst-size=0x0, and misc-ctrl-is-sentinel=false. This accepts only the
bridge/config preflight readiness boundary; endpoint ownership, expected RP1
vendor/device visibility, broad RP1 mapping, endpoint configuration mutation,
BAR discovery or programming, bridge setup, PERST/link-control, interrupt
delivery, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
and phase transition remain unaccepted.

phase11-rp1-bridge-config-preflight-closeout-20260608 accepts
pcie2-bridge-preflight-ready-frontier-closed. The accepted frontier is limited
to the source-backed read-only bridge/config preflight discriminator, the
paired no-MMIO/no-PCIe/no-RP1/no-GIC control proof, and the real Pi 5
pcie2-bridge-preflight-ready result. It does not accept expected RP1
vendor/device visibility, endpoint ownership, broad RP1 mapping, endpoint
configuration mutation, BAR discovery or programming, bridge setup,
PERST/link-control, interrupt delivery, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition. The worker creates no
follow-up task from this closeout; supervisor planning is required for the
next Milestone 11.2 frontier.

phase11-rp1-bridge-setup-source-contract-20260608 accepts
phase11-rp1-bridge-setup-source-contract-v1. The selected target is
pcie2-bridge-setup-state-read: a read-only BCM2712 PCIe2 setup-state snapshot
after the accepted bridge/config preflight ready result. The allowed reads are
PCIE_MISC_PCIE_STATUS at 0x1000124068, PCIE_MISC_MISC_CTRL at 0x1000124008,
PCIE_RC_CFG_PRIV1_ID_VAL3 at 0x100012043c, and outbound window 0 registers
PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO/HI at 0x100012400c/0x1000124010,
PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT at 0x1000124070,
PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI at 0x1000124080, and
PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI at 0x1000124084.

Retained Broadcom STB PCIe setup source writes root-complex class code
0x060400 through PCIE_RC_CFG_PRIV1_ID_VAL3 and programs outbound window 0
through brcm_pcie_set_outbound_win() after the MISC_CTRL setup path. Retained
BCM2712/RP1 device-tree sources tie pcie2 to controller base 0x10_0012_0000
and the non-prefetchable PCIe 0x00_0000_0000 to CPU 0x1f_0000_0000 window
that carries RP1 bus 0xc0_4000_0000. The source-expected visible setup shape
is class code 0x060400, pcie outbound base 0, CPU base-low field 0,
limit-low field 0xfff00000, base-high 0x1f, and limit-high 0x1f.

The accepted classifications are pcie2-bridge-setup-state-visible,
pcie2-bridge-setup-state-incomplete, pcie2-bridge-setup-state-sentinel,
pcie2-bridge-setup-state-link-down-skip,
pcie2-bridge-setup-state-inconclusive-capture,
no-mmio-pcie2-bridge-setup-state-control-visible, and staging/build-blocker.
The paired control must preserve output shape while constructing no BCM2712
PCIe, RP1 peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC, DMA, or other MMIO
address. This source contract accepts only a read-only setup-state contract;
it does not accept runtime or hardware behavior, endpoint config retry,
expected RP1 vendor/device visibility, endpoint ownership, broad RP1 mapping,
BAR discovery or programming, bridge setup writes, PERST/link-control,
interrupt delivery, DMA/cache, networking, SSH, Milestone 11.3, or phase
transition.

phase11-rp1-bridge-setup-pi5-20260608 accepts the real Pi 5
bridge/setup-state proof as pcie2-bridge-setup-state-incomplete. The accepted
rerun used target/talos-rpi5-rp1-bridge-setup-state-read-core.tar.gz, selected
boot tree 9fbdcb57cd60519737902b9e3b85799e2479abffd8911a9ca887015a7f0f625a,
retained two 50,736-byte da591740/kernel_2712.img TFTP fetches, passed
capture-transaction-v2-ready identity join, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
identity-joined serial output retained 90 bridge/setup-state result markers.
It reported PCIE_MISC_PCIE_STATUS=0x3e0b0 with pcie_port, dl_active, and
phylinkup true; PCIE_MISC_MISC_CTRL=0xa8003000 with SCB_ACCESS_EN and
CFG_READ_UR_MODE true; and PCIE_RC_CFG_PRIV1_ID_VAL3=0x30060400 with class
code 0x060400.

The same output proves that outbound window 0 is visible but not in the
source-expected PCIe 0 -> CPU 0x1f_0000_0000 shape. The accepted values are
win0_lo=0x80000000, win0_hi=0x0, win0_base_limit=0x3ff00000,
win0_base_hi=0x1c, and win0_limit_hi=0x1c. The decoded result has
pcie_base_is_zero=false, cpu_base_low_matches=true,
cpu_limit_low_matches=false, cpu_base_high_matches=false,
cpu_limit_high_matches=false, and outbound_window0_matches=false. This accepts
only the incomplete setup-state classification. It does not accept
pcie2-bridge-setup-state-visible, expected RP1 vendor/device visibility,
endpoint ownership, broad RP1 mapping, BAR discovery or programming, bridge
setup writes, PERST/link-control, interrupt delivery, DMA/cache, networking,
SSH, Milestone 11.3, or phase transition.

phase11-rp1-bridge-setup-closeout-20260608 accepts
pcie2-bridge-setup-state-incomplete-frontier-closed. The accepted frontier is
limited to the source-backed read-only bridge/setup-state discriminator, the
paired no-MMIO/no-PCIe/no-RP1/no-GIC control proof, and the real Pi 5
pcie2-bridge-setup-state-incomplete result. It accepts visible link/preflight
state, root-complex class code 0x060400, and visible outbound window 0
registers while keeping the source-expected visible setup-state claim
unaccepted because the outbound window values do not match PCIe 0 -> CPU
0x1f_0000_0000.

This closeout does not accept expected RP1 vendor/device visibility, endpoint
ownership, broad RP1 mapping, BAR discovery or programming, bridge setup
writes, PERST/link-control, interrupt delivery, GPIO/clock ownership,
DMA/cache, networking, SSH, Milestone 11.3, or phase transition. Same-shaped
endpoint config identity and same-shaped bridge/setup-state hardware reruns
remain closed unless a future supervisor task supplies a different
discriminator or new acceptance criteria. Supervisor planning is required for
the next Milestone 11.2 frontier.

phase11-rp1-observed-aperture-source-contract-20260608 accepts
phase11-rp1-observed-aperture-source-contract-v1. The selected target is
rp1-uart0-fr-observed-aperture-read: one read-only 32-bit volatile load from
the RP1 UART0 PL011 flag register at observed CPU physical 0x1c_0003_0018.
The source target remains the RP1 UART0 PL011 flag register: retained
rp1.dtsi declares UART0 at RP1 bus 0xc0_4003_0000, and the selected register
is the PL011 FR offset 0x18. The observed CPU address comes from retained
first-light and decision-log evidence for firmware-preserved RP1 UART0 at
0x1c_0003_0000 plus the accepted bridge/setup-state mismatch that observed
window 0 CPU high fields of 0x1c instead of the source-expected 0x1f.

The accepted classifications are
observed-aperture-rp1-uart0-fr-visible,
observed-aperture-rp1-uart0-fr-sentinel,
observed-aperture-rp1-uart0-fr-all-ones,
observed-aperture-rp1-uart0-fr-zero,
observed-aperture-rp1-uart0-fr-no-return-or-trap,
observed-aperture-rp1-uart0-fr-inconclusive-capture,
no-mmio-observed-aperture-control-visible, and staging/build-blocker. The
paired control must preserve the output shape while constructing no BCM2712
PCIe, RP1 peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC, DMA, or other MMIO
address, including neither 0x1c_0003_0018 nor 0x1f_0003_0018.

This contract accepts only the source/evidence-backed observed-aperture
discriminator. It does not accept live RP1 ownership, endpoint ownership,
broad RP1 mapping, UART ownership, interrupt delivery, GPIO/clock ownership,
DMA/cache, networking, SSH, Milestone 11.3, or phase transition.
Same-shaped endpoint config identity, same-shaped bridge/setup-state, and
same-shaped 0x1f RP1 hardware reruns remain closed unless a future supervisor
task supplies a different discriminator or new acceptance criteria.

phase11-rp1-observed-aperture-pi5-20260608 accepts the real Pi 5
observed-aperture proof as observed-aperture-rp1-uart0-fr-visible. After an
initial capture-staging-blocked candidate run and known-good-control triage,
the accepted unchanged rerun published only
target/talos-rpi5-rp1-observed-aperture-read-core.tar.gz, selected boot tree
def82f95b6ee4440de8014a275cbdef3b1baa4d578d9773e30ff7f15cd2d8a87, retained
two 47,664-byte da591740/kernel_2712.img TFTP fetches, passed the
pi5-capture-transaction-v2 identity join with no rejection reasons, retained
69 TALOS: rp1-observed-aperture-result records, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
visible result read observed CPU physical address 0x1c00030018 and reported
raw=0x187, raw-is-deaddead=false, raw-is-all-ones=false, raw-is-zero=false,
raw-is-pl011-fr-shaped=true, and
classification=observed-aperture-rp1-uart0-fr-visible.

This accepts only the selected one-read observed aperture and its report
shape. It does not accept endpoint ownership, broad RP1 mapping, UART
ownership, interrupt delivery, GPIO/clock ownership, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

phase11-rp1-observed-aperture-closeout-20260608 accepts
observed-aperture-rp1-uart0-fr-visible-frontier-closed. The accepted frontier
is limited to the source/evidence-backed observed-aperture discriminator, the
paired no-MMIO/no-PCIe/no-RP1/no-GIC control proof, and the real Pi 5
observed-aperture-rp1-uart0-fr-visible result. The only accepted hardware
operation is the selected one-read observed aperture at CPU physical
0x1c00030018; it returned raw=0x187, raw-is-pl011-fr-shaped=true, and not
sentinel/all-ones/zero under identity-joined evidence.

This closeout does not accept endpoint ownership, broad RP1 mapping, UART
ownership, interrupt delivery, GPIO/clock ownership, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.
Same-shaped endpoint config identity, same-shaped bridge/setup-state,
same-shaped 0x1f RP1 peripheral, and same-shaped 0x1c observed-aperture
hardware reruns remain closed unless a future supervisor task supplies a
different discriminator or new acceptance criteria. Supervisor planning is
required for the next Milestone 11.2 frontier.

phase11-rp1-observed-gpio-status-source-contract-20260608 accepts
phase11-rp1-observed-gpio-status-source-contract-v1. The selected target is
rp1-gpio14-status-ctrl-observed-aperture-read: two read-only 32-bit volatile
loads from observed CPU physical addresses 0x1c_000d_0070 and 0x1c_000d_0074.
Retained RP1 Linux source identifies those registers as IO_BANK0 GPIO14 STATUS
and CTRL: IO_BANK0 base 0xc0_400d_0000 plus GPIO14 offset 14 * 8 and register
offsets 0x0/0x4. The source-expected 0x1f comparators remain blocked for
same-shaped reruns; the selected addresses use only the accepted observed 0x1c
aperture from the UART0 FR proof without accepting broad RP1 mapping.

The accepted classifications are observed-aperture-gpio14-status-ctrl-visible,
observed-aperture-gpio14-status-ctrl-sentinel,
observed-aperture-gpio14-status-ctrl-all-ones,
observed-aperture-gpio14-status-ctrl-zero,
observed-aperture-gpio14-status-ctrl-no-return-or-trap,
observed-aperture-gpio14-status-ctrl-inconclusive-capture,
no-mmio-observed-gpio-status-control-visible, and staging/build-blocker. The
paired control must preserve the output shape while constructing no BCM2712
PCIe, RP1 peripheral/SYSINFO/GPIO/RIO/pads/clock/reset/MSI-X, MIP, GIC, DMA,
or other MMIO address. IO_BANK0 INTE/INTS are not selected for this contract
because they are not acceptance-critical to the per-pin observed-aperture
STATUS/CTRL discriminator.

This contract accepts only the source/evidence-backed read-only observed
GPIO14 STATUS/CTRL contract. It does not accept GPIO ownership, event
generation, interrupt pending generation, interrupt delivery, endpoint
ownership, broad RP1 mapping, pad/RIO/clock/reset ownership, DMA/cache,
networking, SSH, Milestone 11.3, or phase transition.

phase11-rp1-observed-gpio-status-control-pi5-retry-20260608 accepts the paired
no-MMIO/no-RP1/no-GIC control proof as no-mmio-observed-gpio-status-control-visible.
After an initial repaired-freshness candidate retry was rejected for a
TFTP/final-identity mismatch, the known-good production-timer control passed
pi5-capture-transaction-v2 with no rejection reasons. The unchanged candidate
rerun then selected tree
133f2a9b4a4c5c21b206d8f9eb8eba4ffe41f787a69b33bced0ea164a8cf83ab, retained an
empty pre-power serial drain, observed two 48,952-byte
da591740/kernel_2712.img TFTP fetches, retained 41 task-owned control markers,
kept final pre-restore identity on the selected tree, and restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
accepts only the paired control proof; the real observed GPIO14 STATUS/CTRL
read, GPIO ownership, event generation, interrupt pending generation,
interrupt delivery, endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset
ownership, DMA/cache, networking, SSH, Milestone 11.3, and phase transition
remain unaccepted.

phase11-rp1-observed-gpio-status-pi5-20260608 completed as
capture-staging-blocked. The real candidate selected tree
52b5f11000b24f6f6d00ab1b9aaa4d62a4d4114486a0302ad593b713a08c2559, observed
two 49,656-byte da591740/kernel_2712.img TFTP fetches, retained 42 task-owned
result markers, and emitted marker-visible values gpio14-status-raw=0xabe3300,
gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and
classification=observed-aperture-gpio14-status-ctrl-visible. The v2 identity
join rejected that run because the repaired pre-power serial drain exhausted
96 attempts, read 1,095,168 bytes, and did not reach empty-read-before-power.
The required known-good production-timer control failed the same repaired
freshness discriminator despite matching TFTP/final identity evidence, so the
unchanged real candidate was not rerun. The marker-visible GPIO14 STATUS/CTRL
values remain retained evidence, not accepted visibility. Same-shaped real
GPIO14 STATUS/CTRL reruns are blocked pending supervisor planning around the
freshness blocker or a different accepted discriminator.

phase11-rp1-observed-gpio-status-closeout-20260608 accepts that chain as
observed-gpio-status-capture-blocked-frontier-closed. The accepted frontier is
limited to the source/evidence-backed GPIO14 STATUS/CTRL observed-aperture
contract, the local/static real/control core, the serial-drain freshness repair
procedure, the paired no-MMIO/no-RP1/no-GIC control proof, and the committed
real Pi 5 capture-staging blocker. It does not accept observed 0x1c GPIO14
STATUS/CTRL visibility, GPIO ownership, event generation, interrupt
pending/delivery, GIC acknowledgement, endpoint ownership, broad RP1 mapping,
pad/RIO/clock/reset ownership, DMA/cache, storage, generated-root, networking,
SSH, Milestone 11.3, or phase transition. Same-shaped endpoint config
identity, bridge/setup-state, 0x1f RP1 peripheral, 0x1c UART0 FR, and real
0x1c GPIO14 STATUS/CTRL hardware reruns remain closed unless a future
supervisor task supplies a different discriminator or new acceptance criteria.
Supervisor planning is required for the next Milestone 11.2 frontier.

phase11-pi5-capture-freshness-v3-core-20260608 accepts a first-class
`pi5-capture-transaction-v3` replay contract for the saturated serial blocker.
V3 keeps the selected-tree, expected-fetch, TFTP, final-pre-restore, and
restore identity checks from v2, but permits a non-empty bounded pre-power drain
only when the saturated direct-read serial window contains the required marker
after power and that marker is absent from every retained pre-power drain
response. A synthetic stale-marker replay is rejected as
`capture-staging-blocked`. This changes the same-shaped rerun policy only by
requiring the queued v3 known-good/control/real tasks to pass that marker
differential before any marker-visible GPIO14 STATUS/CTRL output can be
considered; it does not retroactively accept GPIO14 STATUS/CTRL visibility,
GPIO ownership, interrupts, endpoint ownership, broad RP1 mapping, DMA/cache,
networking, SSH, Milestone 11.3, or a phase transition.

phase11-pi5-run-unique-capture-marker-core-20260608 supersedes the failed
constant-marker V3 retry policy for the next observed GPIO14 STATUS/CTRL proof
attempt. The diagnostic runtime may now embed a task-owned
`TALOS_CAPTURE_NONCE` into the observed GPIO status result/control marker, and
the accepted replay procedure must use
`pi5-capture-transaction-run-unique-v1`: V3 plus a run-unique
`capture-nonce=` token that is absent before power and present after power.
Constant-marker V2/V3 retries remain capture-staging-blocked after the clean V3
control showed the control marker present 616 times before power. The
run-unique change is only a serial freshness primitive; it does not change the
GPIO/RP1 source contract or accept GPIO14 STATUS/CTRL visibility, GPIO
ownership, interrupts, endpoint ownership, broad RP1 mapping, DMA/cache,
networking, SSH, Milestone 11.3, or a phase transition.

phase11-rp1-observed-gpio-status-run-unique-control-pi5-20260608 accepts only
the paired no-MMIO/no-RP1/no-GIC run-unique control proof as
no-mmio-observed-gpio-status-run-unique-control-visible. The accepted control
used capture nonce ru20260608T195401Z-f84941d7, selected tree
2e0fbbdc8da0ec3066ddc4b74949887c8bcf80c70ac6c4a68edffb5dca6f5173, retained
empty-read-before-power, observed the nonce-bearing control marker after power,
retained two 49,072-byte da591740/kernel_2712.img TFTP fetches, kept final
pre-restore identity on the selected tree, and restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
accepts only the control output/capture path; it does not accept real
GPIO14 STATUS/CTRL visibility or any RP1/GPIO hardware behavior.

phase11-rp1-observed-gpio-status-run-unique-pi5-20260608 completed as
capture-staging-blocked. The primary real run used capture nonce
ru20260608T2012Z-f84941d7 and retained 41 nonce-bearing result markers with
gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and
classification=observed-aperture-gpio14-status-ctrl-visible. The run-unique
checker rejected the proof because TFTP and final pre-restore identity matched
the restored baseline tree rather than the selected candidate tree: observed
TFTP fetches were 104,136 bytes while the candidate expected 49,776-byte
fetches. A clean same-shaped retry with capture nonce
ru20260608T2025Z-f84941d7 also remained capture-staging-blocked after a
1,095,168-byte non-empty pre-power drain, missing required marker after power,
and baseline-sized TFTP fetches. The marker-visible GPIO14 STATUS/CTRL values
remain retained evidence only; they are not accepted visibility.

phase11-rp1-observed-gpio-status-run-unique-closeout-20260608 accepts this
chain as observed-gpio-status-run-unique-capture-blocked-frontier-closed. The
accepted frontier is limited to the observed GPIO14 STATUS/CTRL source
contract, local/static real/control core, serial-drain repair procedure,
run-unique capture marker contract, run-unique no-MMIO control proof, and
committed real Pi 5 capture-staging blocker. It does not accept
GPIO14 STATUS/CTRL visibility, GPIO ownership, event generation, interrupt
pending/delivery, GIC acknowledgement, endpoint ownership, broad RP1 mapping,
pad/RIO/clock/reset ownership, DMA/cache, storage, generated-root, networking,
SSH, Milestone 11.3, or phase transition. Same-shaped endpoint config
identity, bridge/setup-state, 0x1f RP1 peripheral, 0x1c UART0 FR, and real
0x1c GPIO14 STATUS/CTRL hardware reruns remain closed unless a future
supervisor task supplies a different discriminator, capture/staging repair, or
new acceptance criteria. Supervisor planning is required for the next
Milestone 11.2 frontier.

phase11-pi5-boot-staging-identity-repair-core-20260608 accepts the boot-staging
identity discriminator that gates the next GPIO14 STATUS/CTRL retry. The
checker ignores serial/RP1 output and requires selected-tree identity, expected
TFTP fetch bytes, final pre-restore selected-tree identity, and restore proof.
It explains the prior blocked run by retaining marker-visible serial text while
rejecting baseline TFTP/final identity, and it rejects the clean retry for the
same baseline identity class. This procedure repair does not change the
GPIO/RP1 source contract or accept any new hardware behavior by itself.

phase11-pi5-boot-staging-identity-known-good-control-pi5-20260608 accepts the
paired no-MMIO/no-RP1/no-GIC known-good control under that repaired procedure.
The control selected tree
35a30932a7f8e76d8cfa657b7419ec1d5e7e8ce450c5ae898c32e957636734f1, retained
two 49,072-byte candidate TFTP fetches, kept final pre-restore identity on the
selected tree, passed both the run-unique and boot-staging identity checkers,
and restored the lab to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
accepts only the control output/capture path.

phase11-rp1-observed-gpio-status-after-staging-repair-pi5-20260608 accepts the
real Pi 5 read-only observed GPIO14 STATUS/CTRL visibility proof under the
repaired boot-staging identity procedure. The real candidate selected tree
5a499384497595de18d05f250fe146352d964953c9ff759642cc8d20384e0ea6, retained
two 49,784-byte candidate TFTP fetches, kept final pre-restore identity on the
selected tree, retained 38 task-owned result markers, reported
gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, passed both
the run-unique and boot-staging identity checkers, and restored the lab to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

phase11-rp1-observed-gpio-status-after-staging-repair-closeout-20260608 closes
that chain as observed-gpio14-status-ctrl-visible-frontier-closed. The accepted
frontier is limited to the observed GPIO14 STATUS/CTRL source contract,
local/static real/control core, serial-drain repair, run-unique capture marker
contract, boot-staging identity discriminator, paired no-MMIO/no-RP1/no-GIC
control proof, and real read-only observed GPIO14 STATUS/CTRL visibility proof.
GPIO ownership, event generation, interrupt pending/delivery, GIC
acknowledgement, endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset
ownership, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
and phase transition remain unaccepted. Same-shaped GPIO14 STATUS/CTRL hardware
reruns are closed unless future supervisor planning supplies a different
discriminator or new acceptance criteria; supervisor planning is required for
the next Milestone 11.2 feature slice.

phase11-rp1-observed-gpio-ownership-route-source-contract-20260608 accepts the
next read-only observed-aperture preflight contract:
phase11-rp1-observed-gpio-ownership-route-source-contract-v1. The selected
target is rp1-gpio14-ownership-route-observed-aperture-preflight-read. It uses
the accepted observed GPIO14 STATUS/CTRL visibility as a prerequisite, then
adds only ownership-adjacent read-only status fields before any write or event
generation attempt.

The allowed RP1 observed-aperture reads are GPIO14 STATUS/CTRL at
0x1c000d0070/0x1c000d0074, IO_BANK0 INTE/INTS at
0x1c000d011c/0x1c000d0124, RIO0 OUT/OE/IN at
0x1c000e0000/0x1c000e0004/0x1c000e0008, and GPIO14 pad control at
0x1c000f003c. The parent-route inputs remain the accepted read-only INTID 160
GIC status registers: GICD_ISENABLER5 at 0x107fff9114, GICD_ISPENDR5 at
0x107fff9214, GICD_ISACTIVER5 at 0x107fff9314, and GICC_HPPIR at
0x107fffa018. The source-expected 0x1f ownership/route preflight remains
retained context only; this frontier is explicitly the observed 0x1c aperture.

The accepted classifications are observed-gpio14-ownership-route-preflight-visible,
observed-gpio14-ownership-preflight-blocked-non-gpio-function,
observed-gpio14-ownership-preflight-blocked-route-or-source-state,
observed-gpio14-ownership-preflight-sentinel,
observed-gpio14-ownership-preflight-all-ones,
observed-gpio14-ownership-preflight-zero,
observed-gpio14-ownership-preflight-no-return-or-trap,
observed-gpio14-ownership-preflight-inconclusive-capture,
no-mmio-observed-gpio14-ownership-route-control-visible, and
staging/build-blocker. This accepts only a source contract and paired
no-MMIO/no-RP1/no-GIC control requirement. GPIO ownership, event generation,
interrupt pending/delivery, GIC acknowledgement, handler ownership,
GPIO/RIO/pad/INTE writes, parent-route masking writes, DMA/cache, networking,
SSH, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-observed-gpio-ownership-route-core-20260608 implements that
observed-aperture contract as a local/static real candidate plus paired
no-MMIO/no-RP1/no-GIC control. The real candidate branches from the early Rust
entry path, emits the task-owned start/pre-read markers, performs only the
selected 32-bit volatile loads from 0x1c000d0070, 0x1c000d0074,
0x1c000d011c, 0x1c000d0124, 0x1c000e0000, 0x1c000e0004, 0x1c000e0008,
0x1c000f003c, and the accepted read-only GIC status inputs for INTID 160, then
repeats the task-owned terminal result marker. The paired control emits the
same field shape with not-constructed address fields and
no-mmio-observed-gpio14-ownership-route-control-visible while constructing no
RP1 GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, or GIC MMIO address. This is
local/static core evidence only; Pi 5 control and real proofs remain queued
separately.

phase11-rp1-observed-gpio-ownership-route-control-pi5-20260608 accepts the
paired no-MMIO/no-RP1/no-GIC control output path on Pi 5. The proof retained
selected-tree identity, V3/run-unique/boot-staging checker success, two
48,528-byte candidate TFTP fetches, final selected-tree identity, and restore
to the baseline boot tree. It accepts only the control capture and output
shape; it does not accept GPIO/RP1/GIC/PCIe hardware behavior.

phase11-rp1-observed-gpio-ownership-route-pi5-20260608 accepts the real
read-only observed-aperture GPIO14 ownership/route preflight visibility as
observed-gpio14-ownership-preflight-blocked-non-gpio-function. The decisive
capture retained selected tree
e6ded87c576967c770223930463864fc081443467d6e00fbe108f29fa9e33fd2, two
50,496-byte da591740/kernel_2712.img TFTP fetches, final selected-tree
identity, V3 and boot-staging checker success, marker-visible output, and
restore to the baseline tree. The result reported GPIO14 CTRL raw 0x84,
FUNCSEL=4 / uart0, IO_BANK0 INTE/INTS clear, RIO IN GPIO14 true, pad raw
0x56, GIC INTID160 not enabled/pending/active, and HPPIR spurious 1023.

phase11-rp1-observed-gpio-ownership-route-closeout-20260608 closes this
frontier as
observed-gpio14-ownership-route-preflight-non-gpio-blocker-frontier-closed.
The accepted claim is limited to the source contract, local/static
implementation, no-MMIO control proof, and real read-only preflight
classification that GPIO14 is currently muxed to UART0. GPIO ownership, event
generation readiness, interrupt pending generation, interrupt delivery,
IAR/EOIR acknowledgement, handler ownership, broad RP1 mapping,
GPIO/RIO/pad/INTE/CTRL writes, parent-route masking writes, clock/reset
programming, DMA/cache, networking, SSH, Milestone 11.3, and phase transition
remain unaccepted. Same-shaped preflight reruns are not progress without
future supervisor planning around materially different acceptance criteria.

phase11-rp1-observed-gpio16-ownership-event-source-contract-20260609 accepts
the next source-only GPIO16 observed-aperture ownership/event preflight
contract,
phase11-rp1-observed-gpio16-ownership-event-source-contract-v1. The selected
target is rp1-gpio16-ownership-event-observed-aperture-preflight-read. This is
read-only and qualitatively different from the prior source-expected 0x1f
GPIO16 event discriminator: it uses the accepted observed 0x1c RP1 aperture,
performs no GPIO/RIO/pad/INTE/CTRL writes, generates no event, and accepts no
restore or delivery claim. GPIO16 is selected because GPIO14 is currently muxed
to UART0 in the accepted observed-aperture preflight, while retained source
names GPIO16 as a generic GPIO16 line, retained fixed board consumers do not
reference it, the debug UART is uart10, and prior Talos RP1 UART0 usage is
confined to GPIO14/GPIO15.

Allowed read-only loads are GPIO16 STATUS/CTRL at
0x1c000d0080/0x1c000d0084, IO_BANK0 INTE/INTS at
0x1c000d011c/0x1c000d0124, RIO0 OUT/OE/IN at
0x1c000e0000/0x1c000e0004/0x1c000e0008, GPIO16 pad control at 0x1c000f0044,
and the accepted read-only INTID 160 GIC route status registers at
0x107fff9114, 0x107fff9214, 0x107fff9314, and 0x107fffa018. The report must
decode GPIO16 function, bank source-enable/source-status, RIO state, pad state,
and parent route status. Accepted classifications are
observed-gpio16-ownership-event-preflight-visible,
observed-gpio16-ownership-preflight-blocked-non-gpio-function,
observed-gpio16-ownership-preflight-blocked-route-or-source-state,
observed-gpio16-ownership-preflight-sentinel,
observed-gpio16-ownership-preflight-all-ones,
observed-gpio16-ownership-preflight-zero,
observed-gpio16-ownership-preflight-no-return-or-trap,
observed-gpio16-ownership-preflight-inconclusive-capture,
no-mmio-observed-gpio16-ownership-event-control-visible, and
staging/build-blocker. This accepts only the source contract and paired
control requirement; GPIO ownership, event generation, interrupt pending
generation, interrupt delivery, GIC acknowledgement, handler ownership,
GPIO/RIO/pad/INTE/CTRL writes, GPIO14 ownership changes, DMA/cache,
networking, SSH, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-observed-gpio16-ownership-event-core-20260609 accepts the
local/static implementation of that read-only observed-aperture GPIO16
preflight contract. The retained GPIO16 discriminator scenarios now emit the
accepted observed-aperture contract id and target, perform no GPIO/RIO/pad/
INTE/CTRL writes, no IRQRESET, and no action/restore sequence. The real
candidate reads only GPIO16 STATUS/CTRL at 0x1c000d0080/0x1c000d0084,
IO_BANK0 INTE/INTS at 0x1c000d011c/0x1c000d0124, RIO0 OUT/OE/IN at
0x1c000e0000/0x1c000e0004/0x1c000e0008, GPIO16 pad control at 0x1c000f0044,
and the accepted read-only INTID 160 GIC route status registers. The paired
control emits the same report shape with not-constructed address fields and
constructs no RP1 or GIC MMIO address. This is local/static evidence only; Pi 5
control and real proofs remain queued before any hardware behavior or GPIO
ownership/event claim can be accepted.

phase11-pi5-run-unique-serial-visibility-discriminator-core-20260609 accepts
the local/static repair that makes the run-unique nonce token the current-run
serial visibility discriminator when saturated pre-power serial contains stale
older markers. The repaired checker still requires selected-tree identity,
expected TFTP bytes, final identity, and restore gates before accepting a Pi 5
capture.

phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry-20260609 accepts
the paired no-MMIO GPIO16 control output proof as visible on Pi 5 after that
repair. The accepted run staged tree
cdb35bef8b7fbd5b68df9c76a58fbb410e20522d46aed6b77319002b0be6bd19,
retained two 48,744-byte da591740/kernel_2712.img fetches, proved the
task-owned nonce absent before power and present after power, passed the
boot-staging identity checker, and restored to the baseline tree. This accepts
only the control output/capture path; no GPIO/RP1/GIC/PCIe hardware behavior
is accepted from the control.

phase11-rp1-observed-gpio16-ownership-event-pi5-20260609 accepts the real
read-only GPIO16 ownership/event preflight as
observed-gpio16-ownership-preflight-blocked-non-gpio-function. The accepted
run staged tree
908eadd18fab1ba826d2dba92125649383a4857ed39ea18af125feb721a637c3, retained
two 50,640-byte da591740/kernel_2712.img fetches, passed V3 and boot-staging
identity checks, retained marker-visible output, and restored to the baseline
tree. The result reported GPIO16 FUNCSEL=31 / unknown, IO_BANK0 INTE/INTS
clear for GPIO16, RIO GPIO16 OUT/OE/IN false, pad input disabled, pad output
disabled, INTID160 not enabled/pending/active, and HPPIR spurious 1023. This
accepts only selected read-only GPIO16 preflight visibility/classification.

phase11-rp1-observed-gpio16-ownership-event-closeout-20260609 closes the chain
as
observed-gpio16-ownership-event-preflight-non-gpio-blocker-frontier-closed.
The accepted frontier is limited to the source-backed read-only GPIO16
ownership/event preflight contract, local/static real/control implementation,
repaired no-MMIO control proof, and real Pi 5 non-GPIO-function blocker
classification. GPIO ownership, event generation readiness, interrupt pending
generation, interrupt delivery, IAR/EOIR acknowledgement, handler ownership,
broad RP1 mapping, GPIO/RIO/pad/INTE/CTRL writes, parent-route masking writes,
clock/reset programming, DMA/cache, networking, SSH, Milestone 11.3, and phase
transition remain unaccepted. Same-shaped GPIO16 ownership/event preflight
reruns are not progress unless a future supervisor task supplies materially
different acceptance criteria or a new discriminator. Supervisor planning is
required for the next Milestone 11.2 feature slice.

phase11-rp1-clock-reset-dependency-source-contract-20260609 accepts the next
source-only clock/reset dependency preflight contract,
phase11-rp1-clock-reset-dependency-source-contract-v1. The selected target is
rp1-observed-clock-reset-dependency-preflight-read. It uses source-backed
SYSINFO and clock-manager offsets through the observed 0x1c RP1 aperture
because the accepted observed GPIO14/GPIO16 preflights used that aperture,
while the retained source-expected 0x1f SYSINFO-vs-clock discriminator closed
on a broader 0xdeaddead sentinel boundary.

Allowed read-only loads are SYSINFO_CHIP_ID and SYSINFO_PLATFORM at
0x1c00000000/0x1c00000004, PLL_SYS_CS at 0x1c00020000, CLK_SYS_CTRL,
CLK_SYS_DIV_INT, and CLK_SYS_SEL at
0x1c00018014/0x1c00018018/0x1c00018020, CLK_SLOW_SYS_CTRL at 0x1c00018024,
and CLK_UART_CTRL, CLK_UART_DIV_INT, and CLK_UART_SEL at
0x1c00018054/0x1c00018058/0x1c00018060. The report must include raw values,
expected_chip_id=0x20001927, decoded chip-id/sentinel booleans, PLL_SYS lock,
CLK_SYS/CLK_SLOW_SYS/CLK_UART enable bits, selected clock sentinel booleans,
retained GPIO14/GPIO16 blocker context, retained 0x1f SYSINFO/clock sentinel
context, reset_status_source=none-selected-read-only, and a terminal
classification. No reset-controller read is selected because retained Linux
source exposes reset_control_reset as a reset operation, not a safe read-only
reset-status register.

Accepted classifications are observed-clock-reset-dependency-visible,
observed-clock-reset-dependency-blocked-sysinfo-sentinel,
observed-clock-reset-dependency-blocked-clock-manager-sentinel,
observed-clock-reset-dependency-blocked-system-clock-disabled,
observed-clock-reset-dependency-blocked-uart-clock-disabled,
observed-clock-reset-dependency-no-return-or-trap,
observed-clock-reset-dependency-inconclusive-capture,
no-mmio-clock-reset-dependency-control-visible, and staging/build-blocker.
This accepts only the source contract and paired no-MMIO/no-RP1/no-GIC/no-PCIe
control requirement. It does not accept live RP1 identity, runtime behavior,
hardware behavior, clock/reset ownership, clock/reset writes, GPIO function
changes, event generation, interrupt delivery, DMA/cache, networking, SSH,
Milestone 11.3, or phase transition.

phase11-rp1-clock-reset-dependency-core-20260609 implements that contract as a
local/static real candidate plus paired no-MMIO control. The real candidate
performs only the accepted 32-bit volatile loads from 0x1c00000000,
0x1c00000004, 0x1c00020000, 0x1c00018014, 0x1c00018018, 0x1c00018020,
0x1c00018024, 0x1c00018054, 0x1c00018058, and 0x1c00018060, then emits the
contract fields and terminal classification. The paired control emits the same
report shape with address=not-constructed and constructs no RP1, GPIO,
clock/reset, PCIe/MIP, GIC, DMA, or other forbidden MMIO address. This is
local/static evidence only; Pi 5 control and real proofs remain separate
acceptance gates.

phase11-rp1-clock-reset-dependency-control-pi5-20260609 accepts the paired
no-MMIO/no-RP1/no-GIC/no-PCIe control output path on Pi 5 as
no-mmio-clock-reset-dependency-control-visible. The proof retained selected
tree 3f48e70435914a0ca3deb160c517a32205643c3fbd9547d407387895ae417aba, two
48,640-byte da591740/kernel_2712.img fetches, nonce-visible serial output
after power, boot-staging checker success, final selected-tree identity, and
restore to the baseline tree. This accepts only the control output/capture
path; no RP1, clock/reset, GPIO, GIC, PCIe, DMA, or other hardware behavior is
accepted from the control.

phase11-rp1-clock-reset-dependency-pi5-20260609 accepts the real read-only
clock/reset dependency preflight as
observed-clock-reset-dependency-blocked-system-clock-disabled. The accepted run
staged tree ef7b62b81d097a52bda724d2173c982fa512e2b6541541514abebd6d8db1422f,
retained two 49,496-byte da591740/kernel_2712.img fetches, passed V3 and
boot-staging identity checks, retained marker-visible output, and restored to
the baseline tree. The result reported chip-id 0x20001927, platform 0x2,
PLL_SYS_CS 0x80000001, CLK_SYS_CTRL 0x2, CLK_SLOW_SYS_CTRL 0x0,
CLK_UART_CTRL 0x10000840, chip-id-matches-expected=true,
pll-sys-locked=true, clk-sys-enabled=false, clk-slow-sys-enabled=false,
clk-uart-enabled=true, and no selected clock returned the 0xdead_dead
sentinel. This accepts only the selected read-only dependency snapshot
visibility/classification.

phase11-rp1-clock-reset-dependency-closeout-20260609 closes the chain as
clock-reset-dependency-preflight-system-clock-blocker-frontier-closed. The
accepted frontier is limited to the source-backed read-only SYSINFO and
clock-manager dependency contract, local/static real/control implementation,
no-MMIO control proof, and real Pi 5 system-clock-disabled blocker
classification. Clock/reset ownership, reset-controller ownership, clock/reset
writes, GPIO ownership, GPIO function changes, event generation readiness,
interrupt pending generation, interrupt delivery, IAR/EOIR acknowledgement,
handler ownership, broad RP1 mapping, GPIO/RIO/pad/INTE/CTRL writes, endpoint
config retry, bridge setup writes, DMA/cache, networking, SSH, Milestone 11.3,
and phase transition remain unaccepted. Same-shaped clock/reset dependency
preflight reruns are not progress unless a future supervisor task supplies
materially different acceptance criteria or a new discriminator. Supervisor
planning is required for the next Milestone 11.2 feature slice.

phase11-rp1-irq-clock-gpio-milestone-closeout-20260609 accepts the Milestone
11.2 checkpoint as
rp1-irq-clock-gpio-milestone-112-blocker-checkpoint-accepted. The checkpoint
reconciles the accepted interrupt-route documentation, GIC-visible INTID 160
read-only status snapshot, GPIO bank source-status snapshot, GPIO14 UART0
function blocker, GPIO16 FUNCSEL=31 / unknown blocker, and observed
SYSINFO/clock-manager system-clock-disabled blocker. The roadmap acceptance
condition is satisfied by captured blocker evidence with serial hardware
output, not by a working write-backed GPIO/status-LED diagnostic or interrupt
delivery. GPIO ownership, GPIO/RIO/pad/INTE/CTRL writes, event generation,
interrupt pending generation, interrupt delivery, IAR/EOIR acknowledgement,
handler ownership, clock/reset ownership, clock/reset writes, DMA/cache
behavior, networking, SSH, Milestone 11.3 behavior, and phase transition
remain unaccepted. Same-shaped GPIO14, GPIO16, GIC route-status, GPIO bank
source-status, and SYSINFO/clock-manager dependency reruns are not progress
without new supervisor-planned discriminators or acceptance criteria. The next
accepted direction is the queued DMA/cache source inventory only; it does not
implement DMA/cache or authorize DMA-capable driver work.

## Diagnostic Core Implementation

The local diagnostic core is compiled only when
`TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read`,
`TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read_delayed_marker`,
`TALOS_BOOT_SCENARIO=rpi5_rp1_final_preload_marker_hold`,
`TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read_hold_control`,
`TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_tail_stable_result`,
`TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_tail_stable_no_mmio_control`,
`TALOS_BOOT_SCENARIO=rpi5_rp1_interrupt_routing_msix_cfg_read`, or
`TALOS_BOOT_SCENARIO=rpi5_rp1_interrupt_routing_no_mmio_control` is selected.
The original path branches directly from `rust_entry`, reports
`rpi5-rp1-uart0-fr-read: start` and
`rpi5-rp1-uart0-fr-read: pre-mmio-read` through the UART10 early-serial
helper, flushes UART10, then reads exactly `RP1_UART0_FR`
(`0x1f_0003_0018`) with one 32-bit volatile load. The delayed-marker path adds
bounded repeated and final pre-load UART10 markers before the same contracted
load. The final-preload-marker hold path adds the same bounded repeated and
final pre-load UART10 markers, then repeats a unique hold marker without
calling the RP1 read helper or constructing the contracted address. The
hold-control FR-read path uses the accepted visible hold-marker boundary as a
pre-read control, then executes the same contracted load once and enters a
unique post-read terminal hold loop after reporting the returned raw value. A
returned read from the true FR-read paths reports the contract id, target name,
address, width, raw value, and `mapped/read-value` success classification.
The tail-stable RP1 result path keeps the same single-load contract but moves
the returned read-value/classification text into a repeated terminal marker so
a saturated serial window can retain the result if the load returns. The
tail-stable no-MMIO control path shares the repeated result-output shape with
an explicit simulated/control classification and no RP1 address construction.
The interrupt-routing result path performs the accepted single
read-only/no-enable MSIX_CFG(0) load and then repeats the returned routing
fields. The
interrupt-routing no-MMIO/no-enable control path shares that repeated output
shape with address=not-constructed, simulated/control classification, and no
forbidden RP1/MSI-X/PCIe/MIP/GIC address construction or MMIO.
The pre-load and hold-control markers are discriminators for the next
serialized proof: if hardware reaches a pre-read marker but not the read-value
line, the result is an at-or-after-load no-return/trap boundary, not a mapping
acceptance.

The diagnostic does not add raw assembly early-entry UART markers. Prior Phase 10 evidence quarantined that path from prompt-capable Pi 5 controls after it made accepted controls fail, so this slice keeps the marker inside the existing Rust/serial path. It does not add GPIO, pin-control, clock, reset, interrupt, DMA/cache, Ethernet, networking, SSH, storage, generated-root, or shell behavior.

Artifact helpers:

- `scripts/rpi5-rp1-uart0-fr-read-image.sh` builds the candidate image.
- `scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh` stages the candidate image into a Pi 5 boot tree for the later serialized hardware proof.
- `scripts/rpi5-rp1-uart0-fr-read-delayed-marker-image.sh`,
  `scripts/rpi5-rp1-uart0-fr-read-delayed-marker-boot-tree.sh`,
  `scripts/rpi5-rp1-uart0-fr-read-delayed-marker-archive.sh`, and
  `scripts/rpi5-rp1-uart0-fr-read-delayed-marker-review.sh` build and inspect
  the delayed-marker FR-read candidate archive for the queued serialized Pi 5
  discriminator.
- `scripts/rpi5-rp1-final-preload-marker-hold-image.sh`,
  `scripts/rpi5-rp1-final-preload-marker-hold-boot-tree.sh`,
  `scripts/rpi5-rp1-final-preload-marker-hold-archive.sh`, and
  `scripts/rpi5-rp1-final-preload-marker-hold-review.sh` build and inspect the
  no-RP1-MMIO final-preload-marker hold candidate archive for the queued
  serialized Pi 5 marker-visibility discriminator.
- `scripts/rpi5-rp1-uart0-fr-read-hold-control-image.sh`,
  `scripts/rpi5-rp1-uart0-fr-read-hold-control-boot-tree.sh`,
  `scripts/rpi5-rp1-uart0-fr-read-hold-control-archive.sh`, and
  `scripts/rpi5-rp1-uart0-fr-read-hold-control-review.sh` build and inspect
  the hold-control RP1 UART0 FR-read candidate archive for the queued
  serialized Pi 5 discriminator.
- `scripts/rpi5-rp1-uart0-fr-shaped-no-mmio-marker-image.sh`,
  `scripts/rpi5-rp1-uart0-fr-shaped-no-mmio-marker-boot-tree.sh`,
  `scripts/rpi5-rp1-uart0-fr-shaped-no-mmio-marker-archive.sh`, and
  `scripts/rpi5-rp1-uart0-fr-shaped-no-mmio-marker-review.sh` build and
  inspect the no-MMIO marker discriminator archive for the queued serialized
  Pi 5 proof.
- scripts/rpi5-rp1-uart0-fr-tail-stable-result-image.sh,
  scripts/rpi5-rp1-uart0-fr-tail-stable-result-boot-tree.sh,
  scripts/rpi5-rp1-uart0-fr-tail-stable-result-archive.sh, and
  scripts/rpi5-rp1-uart0-fr-tail-stable-result-review.sh build and inspect
  the tail-stable RP1 single-load result candidate archive.
- scripts/rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-image.sh,
  scripts/rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-archive.sh, and
  scripts/rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-review.sh build and
  inspect the no-RP1-MMIO tail-stable control archive required before the RP1
  hardware proof.
- scripts/rpi5-rp1-interrupt-routing-msix-cfg-read-image.sh,
  scripts/rpi5-rp1-interrupt-routing-msix-cfg-read-boot-tree.sh,
  scripts/rpi5-rp1-interrupt-routing-msix-cfg-read-archive.sh, and
  scripts/rpi5-rp1-interrupt-routing-msix-cfg-read-review.sh build and inspect
  the real read-only/no-enable interrupt-routing diagnostic archive.
- scripts/rpi5-rp1-interrupt-routing-no-mmio-control-image.sh,
  scripts/rpi5-rp1-interrupt-routing-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-interrupt-routing-no-mmio-control-archive.sh, and
  scripts/rpi5-rp1-interrupt-routing-no-mmio-control-review.sh build and
  inspect the no-MMIO/no-enable interrupt-routing control archive required
  before the real diagnostic hardware proof.

## DMA/Cache Source Inventory

The accepted DMA/cache source inventory is
phase11-rp1-dma-cache-source-inventory-20260609. It is source/static evidence
only and does not implement DMA, cache maintenance, Ethernet, networking, SSH,
storage, or hardware proof.

The retained Raspberry Pi Linux sources establish these contract inputs:

- bcm2712.dtsi gives pcie2 a 4 MiB 32-bit non-prefetchable DMA window at PCIe
  00_00000000 to CPU physical 0x1f_0000_0000, a 64 GiB RAM-facing
  prefetchable DMA window at PCIe 10_00000000 to CPU physical 0x0, and a 4 KiB
  MIP0 window at PCIe ff_ffff_f000.
- bcm2712-rpi-5-b.dts maps RP1 0xc0_40000000..0xc0_4040ffff to PCIe
  00_00000000 and records RP1 inbound dma-ranges for RAM-facing
  0x10_00000000/0x0_00000000 paths plus the RP1 peripheral-facing
  0xc0_40000000 path.
- rp1.dtsi exposes rp1_dma as Synopsys AXI DMA snps,axi-dma-1.01a at RP1 bus
  0xc0_40188000, with 8 channels, one master, 64 targets, 128-bit data width,
  per-channel block size 0x40000, and AXI burst limits.
- bcm2712-rpi-5-b.dts attaches iommu5 to selected display/camera RP1 bus
  masters, but not to rp1_dma or rp1_eth in the retained source evidence.
- Linux coherent_pool=1M bootargs are context only; they are not Talos evidence
  for coherent allocation, cache maintenance, or DMA-safe buffers.

The next accepted contract must therefore name DMA-safe buffer ownership,
address translation, cache clean/invalidate direction, alignment, lifetime, and
driver evidence fields before any DMA-capable RP1 driver is evaluated.

## DMA/Cache Substrate Contract and Core

phase11-rp1-dma-cache-contract-20260609 accepts the local/static
phase11-rp1-dma-cache-substrate-contract-v1. The accepted boundary names
buffer, memory, address, cache, and future driver ownership fields; RP1
dma-ranges-derived RAM/peripheral translation paths; direction-specific
cache-maintenance semantics; and explicit IOMMU classification. It does not
accept executed cache maintenance, coherent or non-cacheable DMA policy, IOMMU
policy, DMA programming, Ethernet, storage, networking, SSH, or hardware
validation.

phase11-rp1-dma-cache-substrate-core-20260609 implements that contract as a
local/static core in src/dma_cache.rs. The accepted implementation exposes
descriptor/cache/address/IOMMU vocabulary, pure validators, RP1 RAM-window and
peripheral-window translation helpers, and evidence fields for the retained
contract/source ids, CPU/RP1 addresses, length, alignment, direction,
cacheability, owner, IOMMU classification, validation results, and
classification. The tests cover one valid low-tail bootstrap-bump-owned RP1
RAM-window descriptor and rejected alignment, ownership-span, high-memory,
reserved-memory, translation, cacheability, and IOMMU inputs.

phase11-rp1-dma-cache-substrate-closeout-20260609 closes this as a
local/static frontier only. Working DMA behavior, descriptor rings, DMA channel
programming, executed cache maintenance for driver buffers,
cache-coherent/non-cacheable/IOMMU-backed driver policy, DMA-safe allocation or
pinning beyond descriptor validation, RP1 Ethernet readiness, storage
readiness, networking, SSH, hardware validation, Milestone 12 work, and
Milestone 11.3 completion by implication remain unaccepted. CPU-visible address
alias/equality policy remains evidence-only before any future driver consumes
non-identity or high-memory buffers.

phase11-rp1-dma-cache-driver-adjacent-source-contract-20260609 accepts the
next contract boundary as local/static cache synchronization planning derived
from accepted descriptor evidence. It names the operation and ownership
boundary vocabulary, source-backed 64-byte cache-line range coverage, evidence
fields, and validation strategy for a future cache-sync-plan core. The contract
does not accept executed cache maintenance, live barriers, DMA programming,
driver behavior, hardware proof, networking, SSH, or Milestone 11.3 completion;
supervisor planning is required before implementation continues.

phase11-rp1-dma-cache-sync-plan-core-20260609 implements that bounded
local/static sync-plan core in src/dma_cache.rs, and
phase11-rp1-dma-cache-sync-plan-closeout-20260609 closes the accepted
sync-plan frontier. phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract-20260609
then accepts the next source-contract boundary as local/static cache-maintenance
instruction sequencing derived only from accepted sync-plan evidence. The next
implementation frontier may define static instruction/barrier vocabulary and
evidence for clean, invalidate, and clean+invalidate line coverage, but it must
not execute cache maintenance, claim live barrier ordering, program DMA/RP1
MMIO, create descriptor rings, start Ethernet/storage/networking/SSH work, or
accept Milestone 11.3 completion by implication.

phase11-rp1-dma-cache-maintenance-sequence-core-20260609 implements that
local/static maintenance-sequence core in src/dma_cache.rs, and
phase11-rp1-dma-cache-maintenance-sequence-closeout-20260609 closes the
accepted local/static sequence frontier. The accepted frontier is limited to
static dc cvac, dc ivac, dc civac, and dsb sy vocabulary derived only from
accepted DmaCacheSyncPlanEvidence while preserving descriptor/sync-plan
identity, line coverage, rejected runtime claims, and local/static
classification.

phase11-rp1-dma-cache-runtime-execution-contract-20260609 accepts the next
contract boundary as phase11-rp1-dma-cache-maintenance-executor-contract-v1: a
future architecture-gated runtime executor contract that may consume only
accepted DmaCacheMaintenanceSequenceEvidence. The contract requires future
execution work to validate the accepted descriptor, sync-plan, and
maintenance-sequence evidence chain before dispatching dc cvac, dc ivac,
dc civac, and a final dsb sy. This is contract-only and does not accept
executed cache maintenance, live barrier ordering, working DMA, RP1 MMIO
writes, DMA channel programming, descriptor rings, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, or Milestone 11.3
completion by implication. Supervisor planning is required before any
maintenance-executor core implementation continues.

## Deferred Work

- PCIe enumeration and BAR discovery for general external devices.
- Talos-owned RP1 reset, clock, GPIO, pinctrl, UART initialization, and interrupt routing.
- DMA addressability, cache maintenance, IOMMU policy, Ethernet, networking, SSH, storage drivers, and writable persistent filesystems.
- Any fix for the Phase 10 Pi 5 generated-root firmware-initramfs overlap blocker.
