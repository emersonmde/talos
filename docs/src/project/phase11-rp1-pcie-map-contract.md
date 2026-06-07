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

## Deferred Work

- PCIe enumeration and BAR discovery for general external devices.
- Talos-owned RP1 reset, clock, GPIO, pinctrl, UART initialization, and interrupt routing.
- DMA addressability, cache maintenance, IOMMU policy, Ethernet, networking, SSH, storage drivers, and writable persistent filesystems.
- Any fix for the Phase 10 Pi 5 generated-root firmware-initramfs overlap blocker.
