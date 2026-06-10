# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Write/Restore Source Contract

Task id: phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-20260610
Status: accepted
Owner: worker
Classification: rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-accepted
Evidence level: static inspection of accepted GPIO32 read-only preflight
proof closeout, accepted GPIO32 source contract, accepted Phase 11 GPIO
frontiers, retained Raspberry Pi Linux GPIO/RIO/pads/MACB source excerpts,
Phase 12 docs, and task-owned JSON. No runtime implementation, Pi 5 hardware
run, boot archive publication, hardwareTestLock acquisition, RP1 MMIO write,
GPIO32/RIO/pad write, PHY reset assertion/deassertion, or MDIO transaction was
performed.

## Goal

Define the bounded source-backed GPIO32 / ETH_RST_N write/restore ownership
contract after accepted read-only GPIO32 preflight visibility, without
asserting reset or accepting GPIO/MDIO/Ethernet driver readiness by
implication.

## Scope

- Consumed accepted GPIO32 read-only preflight proof closeout commit
  822af798ccab7f8d67e686e0798a6a25c6267441.
- Consumed accepted GPIO32 PHY-reset source contract commit
  278b754ff3a8f589429cd91aaadc3085db6e7b90.
- Reconciled retained Linux pinctrl/GPIO/RIO/pad source facts against the
  accepted observed RP1 aperture and accepted Phase 11 GPIO ownership
  constraints.
- Derived exact observed-aperture addresses and fields for GPIO32 STATUS/CTRL,
  RIO OUT/OE/IN, and pad state needed before any future write/restore
  operation.
- Defined no-write preconditions and explicit blocked classifications for
  sentinel reads, unsafe function/route state, unexpected interrupt/event
  state, missing restore baseline, and capture-chain failures.
- Defined the future candidate/control shape, write set, restore rule, and
  rejected claim boundary.
- Selected the local/static GPIO32 write/restore guard core as the next
  bounded follow-up.
- Recorded findings with disposition.

## Non-Goals

No implementation beyond source/docs/evidence contract, no Pi 5 hardware run,
no boot archive publication, no hardwareTestLock acquisition, no runtime
GPIO/RIO/pad/MMIO write, no PHY reset assertion/deassertion, no MDIO
transaction, no interrupt enablement or completion, no DMA/descriptors, no
packet I/O, no networking, no sockets, no SSH, no Phase 12.2, and no phase
transition. No broad GPIO framework, no generic pinctrl ownership API, no MDIO
bus ownership, and no Ethernet driver initialization.

## Accepted Input Frontier

The accepted input frontier is deliberately narrow:

- read-only GPIO32 PHY-reset preflight visibility/control output, accepted in
  commit 822af798ccab7f8d67e686e0798a6a25c6267441;
- the GPIO32 PHY-reset source contract, accepted in commit
  278b754ff3a8f589429cd91aaadc3085db6e7b90;
- observed-window RP1 aperture visibility at 0x1c00000000 and accepted
  rp1_eth identity context;
- accepted Phase 11 GPIO source/status and blocker evidence, with GPIO
  ownership, function changes, RIO/pad/INTE/CTRL writes, and event generation
  still unaccepted;
- retained Raspberry Pi Linux source excerpts for rp1_gpio, pinctrl-rp1,
  RP1 MFD block offsets, Pi 5 phy-reset-gpios, and MACB MDIO reset behavior.

This contract does not reinterpret read-only visibility or source facts as
GPIO ownership, MDIO/PHY ownership, Ethernet driver readiness, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Selected Target

Contract id:
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-v1

GPIO32 belongs to RP1 GPIO bank1 because pinctrl-rp1.c defines bank1 as
minimum GPIO 28 with six pins. GPIO32 therefore has bank-local bit 4. The
accepted observed-aperture rule uses CPU physical base 0x1c00000000 plus the
source RP1 offset.

| Field | Value |
| --- | --- |
| GPIO controller | rp1_gpio |
| Signal | ETH_RST_N |
| GPIO line | 32 |
| Bank | bank1 |
| Bank-local bit | 4 |
| Source RP1 IO_BANK1 base | 0xc0_400d_4000 |
| Observed CPU IO_BANK1 base | 0x1c000d4000 |
| GPIO32 STATUS | source 0xc0_400d_4020, observed 0x1c000d4020 |
| GPIO32 CTRL | source 0xc0_400d_4024, observed 0x1c000d4024 |
| RIO1 OUT | source 0xc0_400e_4000, observed 0x1c000e4000 |
| RIO1 OE | source 0xc0_400e_4004, observed 0x1c000e4004 |
| RIO1 IN | source 0xc0_400e_4008, observed 0x1c000e4008 |
| GPIO32 pad | source 0xc0_400f_4014, observed 0x1c000f4014 |
| Width/access | 32-bit little-endian volatile load/store only in a future task |
| Active-low assertion | drive the raw output bit low for GPIO32 / bank1 bit 4 |
| Active-low deassertion | drive the raw output bit high for GPIO32 / bank1 bit 4 |
| Source reset duration | 5 ms |

The GPIO32 STATUS/CTRL address comes from RP1_IO_BANK0_BASE 0x0d0000,
bank1 gpio_offset 0x4000, and GPIO32 bank-local index 4 multiplied by the
8-byte STATUS/CTRL pair. The RIO addresses come from RP1_SYS_RIO0_BASE
0x0e0000, bank1 rio_offset 0x4000, and RIO OUT/OE/IN offsets 0x00, 0x04,
and 0x08. The pad address comes from RP1_PADS_BANK0_BASE 0x0f0000, bank1
pads_offset 0x4004, and bank-local index 4 multiplied by 4.

## Relevant Fields

The future guard and proof must report, but not necessarily modify, these
fields:

- GPIO32 STATUS raw value and event bits: raw falling/rising/low/high bits
  20-23 and filtered falling/rising/low/high bits 24-27.
- GPIO32 CTRL raw value and routing fields: FUNCSEL bits 4:0, OUTOVER bits
  13:12, OEOVER bits 15:14, INOVER bits 17:16, IRQ enable bits 20-27,
  IRQRESET bit 28, and IRQOVER bits 31:30.
- RIO1 OUT/OE/IN raw values and bank-local bit 4.
- GPIO32 pad raw value: SLEWFAST bit 0, SCHMITT bit 1, PULL bits 3:2, DRIVE
  bits 5:4, IN_ENABLE bit 6, and OUT_DISABLE bit 7.

## No-Write Preconditions

A future write-backed candidate must classify as blocked/no-write before any
GPIO32/RIO/pad store if any of these conditions hold:

- any selected GPIO32 STATUS/CTRL, RIO OUT/OE/IN, or required pad read returns
  a sentinel value, all-ones, all-zero where the source expects a meaningful
  visible register, a synchronous fault, or an inconclusive capture result;
- GPIO32 CTRL FUNCSEL is not the accepted GPIO function value 5, unless a
  later supervisor-planned task explicitly accepts a function-change
  write/restore scope;
- GPIO32 CTRL override fields make the raw RIO OUT/OE path unsafe or
  ambiguous for ETH_RST_N ownership;
- GPIO32 CTRL IRQ enable bits 20-27, IRQRESET bit 28 side effects, or IO_BANK1
  interrupt/event state indicate this would become an interrupt/event task;
- pad OUT_DISABLE/IN_ENABLE or other pad fields make output drive unsafe and
  the task lacks an explicit pad write/restore scope;
- a complete restore baseline is missing for every register the candidate
  would write;
- candidate/control identity, selected-tree/TFTP, serial freshness, final
  identity, or restore evidence is inconclusive in a future hardware proof.

Blocked/no-write classifications must not be reported as GPIO ownership,
PHY-reset ownership, MDIO readiness, or Ethernet readiness.

## Future Candidate Sequence

A future candidate may be implemented only by a separately queued task with
explicit local/static guard and hardware proof gates. The selected operation is
this bounded sequence:

1. Capture baseline raw values for GPIO32 STATUS, GPIO32 CTRL, RIO1 OUT, RIO1
   OE, RIO1 IN, and GPIO32 pad if pad state is part of the accepted write
   scope.
2. Check all no-write preconditions before any store.
3. Assert active-low ETH_RST_N only through GPIO32 bank1 bit 4: make GPIO32
   output-enabled through the accepted RIO/OE path, drive the raw output bit
   low, and report assertion readback.
4. Wait the source-backed 5 ms reset duration.
5. Deassert ETH_RST_N by driving GPIO32 bank1 bit 4 raw output high and report
   deassertion readback.
6. Restore every touched register or register bit to the captured baseline,
   including RIO OUT/OE and any explicitly accepted GPIO32 CTRL or pad fields.
7. Read back restore state for every touched field and classify success only
   if restore equals baseline.

The first accepted write-backed proof may write only GPIO32/ETH_RST_N
ownership fields needed by this sequence. It must not write MDIO, Ethernet
MAC/GEM, clock, reset-controller, interrupt-controller, DMA, descriptor,
packet, socket, SSH, or Phase 12.2 paths.

## Paired Control Requirements

The paired control must preserve the same report/capture path while
constructing no GPIO32/RIO/pad/MMIO writable target, performing no volatile
store, and withholding candidate-only GPIO32/ETH_RST_N write/restore facts.
It may report control-only simulated fields, but it must classify as
no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control.

## Future Classification Set

The future hardware proof may classify only:

- rp1-ethernet-gpio32-phy-reset-write-restored
- rp1-ethernet-gpio32-phy-reset-write-assertion-mismatch-restored
- rp1-ethernet-gpio32-phy-reset-write-deassertion-mismatch-restored
- rp1-ethernet-gpio32-phy-reset-write-restore-failed
- rp1-ethernet-gpio32-phy-reset-blocked-sentinel-read
- rp1-ethernet-gpio32-phy-reset-blocked-unsafe-function
- rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state
- rp1-ethernet-gpio32-phy-reset-blocked-missing-restore-baseline
- rp1-ethernet-gpio32-phy-reset-inconclusive-capture
- no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control
- staging/build-blocker

Every accepted classification remains bounded to GPIO32 / ETH_RST_N
write/restore and paired control output.

## Findings

- fixed: derived GPIO32 as RP1 bank1, bank-local bit 4.
- fixed: derived observed GPIO32 STATUS/CTRL addresses 0x1c000d4020 and
  0x1c000d4024 from retained pinctrl source and the accepted 0x1c RP1
  aperture.
- fixed: derived observed RIO1 OUT/OE/IN addresses 0x1c000e4000,
  0x1c000e4004, and 0x1c000e4008, with bit 4 selected for GPIO32.
- fixed: derived the GPIO32 pad address 0x1c000f4014 and retained pad state
  as relevant precondition/restore evidence.
- fixed: mapped active-low ETH_RST_N assertion to raw output low and
  deassertion to raw output high while preserving the source-backed 5 ms
  duration.
- fixed: defined no-write blockers for sentinel/all-ones/zero reads, unsafe
  GPIO function/route state, unexpected event/interrupt state, missing restore
  baseline, and capture-chain inconclusive evidence.
- fixed: selected the local/static GPIO32 write/restore guard core as the next
  mechanically objective follow-up.
- deferred: runtime guard implementation, serialized Pi 5 proof, MDIO/PHY
  ownership, interrupt completion, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/docs/evidence only.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims:

- broad GPIO framework or generic pinctrl ownership;
- function-change ownership for GPIO32;
- pad write ownership unless a later task explicitly selects it;
- interrupt/event ownership or GPIO IRQ delivery;
- MDIO transactions or PHY ownership;
- Ethernet driver readiness or broad Ethernet MMIO readiness;
- clock/reset, reset-controller, DMA, descriptor, packet, network, socket, or
  SSH readiness;
- Phase 12.2 work or phase transition.

Retained risks:

- The first hardware run may classify no-write if GPIO32 is not already in a
  safe GPIO route or if pad/override state is unsafe.
- A successful GPIO32 write/restore would prove only the selected reset-line
  store/readback/restore boundary, not MDIO/PHY ownership or Ethernet packet
  readiness.
- Pad state may need a later explicit write/restore scope if readback shows
  output drive is disabled or unsafe.
- Hardware proof still needs candidate/control identity, TFTP evidence, serial
  freshness, final identity, restore evidence, and task-owned JSON.

## Evidence

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract/evidence-map.json.
- Accepted GPIO32 read-only proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-proof-closeout.md.
- Accepted GPIO32 source contract:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract.md.
- Accepted Phase 11 GPIO frontier:
  tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md.
- Phase 11 GPIO source notes:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/source-reference-notes.md.
- Retained Linux pinctrl source:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c.
- Retained Linux RP1 MFD source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-mfd.h.
- Retained Pi 5 board source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts.
- Retained Linux MACB source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.

## Validation

- static inspection: accepted GPIO32 read-only proof closeout, accepted GPIO32
  source contract, accepted Phase 11 GPIO docs/tasks, retained Raspberry Pi
  Linux pinctrl/GPIO/RIO/pad/MACB source excerpts, and touched docs/evidence
  reviewed.
- JSON validation: jq empty on classification/evidence-map JSON passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-core-20260610 on
the next worker wake if dependencies remain satisfied. Keep that follow-up
local/static only: no hardware run, no hardwareTestLock, no runtime GPIO/RIO/
pad/MMIO write, no PHY reset assertion/deassertion, no MDIO, no packet I/O,
no networking, no sockets, no SSH, no Phase 12.2, and no phase transition.
