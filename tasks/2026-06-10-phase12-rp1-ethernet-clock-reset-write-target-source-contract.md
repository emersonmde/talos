# Phase 12 RP1 Ethernet Clock/Reset Write-Target Source Contract

Task id: phase12-rp1-ethernet-clock-reset-write-target-source-contract-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clock-reset-write-target-source-contract-accepted
Evidence level: static inspection of accepted clock/reset contract, guard core,
read-only baseline proof/closeout, retained Raspberry Pi Linux source excerpts,
Talos clock/reset source context, project docs, and task-owned JSON. No Pi 5
hardware run, RP1 MMIO write, clock/reset write, runtime implementation, or
hardwareTestLock acquisition was performed.

## Goal

Select exactly one Ethernet-private RP1 clock-manager control register and
define a bounded pre-read, idempotent write, readback, restore-write, and
restore-read contract for a future hardware proof.

## Scope

- Consumed accepted read-only baseline closeout commit
  ea16def2aa9d692b8ace54818b38e7a1acf0956a.
- Reconciled retained Linux source facts for rp1_eth clocks against the
  accepted observed 0x1c RP1 aperture and Phase 11 clock-manager write/restore
  discipline.
- Selected one target: CLK_ETH_TSU_CTRL for RP1_CLK_ETH_TSU / rp1_eth tsu_clk.
- Defined exact source offset, Talos register address, width, writable value
  rule, preserved fields, operation order, paired control requirements, future
  proof classification set, rejected claims, and retained risks.
- Selected the local/static write/restore core as the next mechanically
  objective follow-up.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, no Pi 5 hardware run, no boot archive publication,
no hardwareTestLock acquisition, no RP1 MMIO write, no clock enable/disable,
no divider/source/PLL/frequency-counter/GPCLK output-enable write, no
RP1_CLK_SYS pclk/hclk transition, no reset-controller write, no GPIO32/PHY
reset, no MDIO/PHY ownership, no DMA, no descriptors, no interrupts or
completions, no packet I/O, no networking, no sockets, no SSH, no Phase 12.2,
and no phase transition.

## Accepted Input Frontier

The accepted input frontier remains narrow:

- observed-window MACB_MID identity context only at 0x1c001000fc, raw
  0x00070109, idnum 0x7, rev 0x109;
- accepted prerequisite and clock/reset guard report visibility/control
  output only;
- accepted read-only baseline report visibility/control output only;
- Phase 11 accepted clock-manager write discipline for one prior ADC control
  register: pre-read, write the pre-read raw value back, post-read,
  restore-write the same pre-read raw value, and restore-read.

This contract does not reinterpret report visibility as runtime ownership and
does not accept Ethernet driver readiness, broad Ethernet MMIO readiness, RP1
MMIO writes, GPIO32/PHY reset ownership, MDIO/PHY ownership, interrupts, DMA,
descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Selected Target

Contract id:
phase12-rp1-ethernet-clock-reset-write-target-source-contract-v1

    target: rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore
    clock name: tsu_clk
    clock id: RP1_CLK_ETH_TSU (29)
    clock-manager source register: CLK_ETH_TSU_CTRL
    source block: RP1 clocks@18000, compatible raspberrypi,rp1-clocks
    observed RP1 base: 0x1c00000000
    source offset from observed RP1 base: 0x018134
    Talos register address: 0x1c00018134
    width: 32-bit little-endian volatile load/store
    allowed write value: the pre-read raw value only

CLK_ETH_TSU_CTRL is selected over CLK_ETH_CTRL because it is one of the two
rp1_eth Ethernet-private clocks, it has an exact retained Linux clock-manager
register, and the bounded idempotent write/restore sequence can exercise the
future Ethernet-private write path without touching shared RP1_CLK_SYS pclk/hclk
or the direct RGMII transmit clock. Selecting this target does not imply that
Talos owns the timestamp unit clock or Ethernet runtime behavior.

## Source Evidence

Retained source facts supporting the target:

- linux-rpi-6.12-rp1.dtsi lines 999-1009 define rp1_eth at ethernet@100000,
  compatible raspberrypi,rp1-gem / cdns,macb, with clocks RP1_CLK_SYS,
  RP1_CLK_SYS, RP1_CLK_ETH_TSU, and RP1_CLK_ETH named pclk, hclk, tsu_clk,
  and tx_clk.
- linux-rpi-6.12-rp1.dtsi lines 24-59 define the RP1 clock manager at
  clocks@18000, assign RP1_CLK_ETH_TSU, and set its assigned rate to 50 MHz.
- linux-rpi-6.12-rp1-clock.h defines RP1_CLK_ETH_TSU as clock id 29 and
  RP1_CLK_ETH as clock id 16.
- clk-rp1.c lines 119-121 define CLK_ETH_TSU_CTRL at offset 0x00134,
  CLK_ETH_TSU_DIV_INT at 0x00138, and CLK_ETH_TSU_SEL at 0x00140.
- clk-rp1.c lines 2079-2097 register clk_eth_tsu with ctrl register
  CLK_ETH_TSU_CTRL, divider register CLK_ETH_TSU_DIV_INT, select register
  CLK_ETH_TSU_SEL, max frequency 50 MHz, and no GPCLK output-enable mask in
  the retained descriptor.
- clk-rp1.c lines 1033-1067 show Linux enable/disable helpers modify only
  CLK_CTRL_ENABLE in a clock's own control register unless the descriptor has
  a nonzero GPCLK output-enable mask.
- clk-rp1.c lines 239-243 define CLK_CTRL_ENABLE bit 11, CLK_CTRL_AUXSRC
  bits 9:5, and source bits starting at bit 0.

The accepted observed-aperture dependency source contract selected
0x1c00000000 + source offset as the Talos read address rule for visible RP1
clock-manager registers. Applying that rule to clock-manager base offset
0x018000 plus CLK_ETH_TSU_CTRL offset 0x00134 yields 0x1c00018134.

## Allowed Future Candidate Operations

A future real candidate may perform only this ordered sequence:

1. Pre-read CLK_ETH_TSU_CTRL at 0x1c00018134 and retain pre_raw.
2. Write pre_raw back to CLK_ETH_TSU_CTRL at 0x1c00018134.
3. Post-read CLK_ETH_TSU_CTRL and retain post_raw.
4. Restore-write pre_raw back to CLK_ETH_TSU_CTRL.
5. Restore-read CLK_ETH_TSU_CTRL and retain restore_raw.

The first write is intentionally idempotent. A partial run after that write
does not intentionally change hardware state because the written value is the
pre-read value. The restore operation is the same value written again.

## Preserved Fields

The future proof must preserve all fields unless a later source contract
explicitly changes scope:

- the full 32-bit raw CLK_ETH_TSU_CTRL value;
- CLK_CTRL_ENABLE bit 11;
- CLK_CTRL_AUXSRC bits 9:5;
- clock source bits starting at bit 0;
- any reserved or currently undocumented bits in the register.

The future proof must report equality booleans for post_raw == pre_raw and
restore_raw == pre_raw. Any mismatch must be classified as restored or
restore-failed, not as successful ownership.

## Safety Invariants

- Do not disable, gate, or transition RP1_CLK_SYS through rp1_eth pclk or
  hclk.
- Do not write CLK_ETH_DIV_INT, CLK_ETH_SEL, CLK_ETH_TSU_DIV_INT,
  CLK_ETH_TSU_SEL, PLL registers, frequency-counter registers, or GPCLK_OE_CTRL.
- Do not write CLK_ETH_CTRL in this contract; it remains a separate
  Ethernet-private target requiring separate acceptance criteria.
- Do not use reset-controller writes because the retained Pi 5 rp1_eth node
  supplies no accepted reset target or restore semantics.
- Do not assert or deassert GPIO32 PHY reset and do not perform MDIO
  transactions.
- Do not infer packet I/O, interrupts, DMA, descriptor readiness, networking,
  sockets, SSH, Phase 12.2, or phase transition from this clock write/restore
  proof.

## Paired Control Requirements

The future paired control must preserve the same report path and
classification vocabulary while constructing no writable RP1 clock-manager
target and performing no volatile load/store to RP1 clock, Ethernet, reset,
GPIO, MDIO, DMA, descriptor, interrupt, PCIe/MIP, GIC, or packet paths.

The control may emit simulated raw values and a control-only classification,
but it must withhold the candidate-only selected register address and
write/restore facts.

## Future Proof Classification Set

The future hardware proof may classify only:

- rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored
- rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-mismatch-restored
- rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore-failed
- rp1-ethernet-clk-eth-tsu-ctrl-blocked-missing-clock-manager
- rp1-ethernet-clk-eth-tsu-ctrl-inconclusive-capture
- no-clock-write-no-ethernet-rp1-ethernet-write-restore-control
- staging/build-blocker

Every accepted hardware classification must remain bounded to the selected
idempotent write/restore proof and paired control output.

## Findings

- fixed: selected exactly one Ethernet-private target, CLK_ETH_TSU_CTRL for
  RP1_CLK_ETH_TSU / tsu_clk.
- fixed: derived the exact Talos register address 0x1c00018134 from the
  accepted observed RP1 aperture plus retained clock-manager/source offsets.
- fixed: defined the only allowed write as the pre-read raw value, with
  post-read, restore-write, and restore-read evidence.
- fixed: preserved all control-register fields, including enable, auxsource,
  source, and reserved bits.
- fixed: rejected writes to shared RP1_CLK_SYS, CLK_ETH_CTRL, divider, select,
  PLL, frequency-counter, GPCLK output-enable, reset-controller, GPIO32/PHY
  reset, MDIO, DMA, descriptor, interrupt, packet, networking, SSH, Phase 12.2,
  and phase-transition surfaces.
- fixed: defined paired control and future proof classifications without
  accepting runtime clock/reset ownership by implication.
- deferred: local/static write/restore report core, serialized Pi 5
  write/restore proof, CLK_ETH_CTRL, GPIO32/PHY reset, MDIO/PHY, interrupts,
  DMA, descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and
  phase transition remain future tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/static only.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims:

- broad clock/reset ownership;
- runtime Ethernet driver readiness;
- broad Ethernet MMIO readiness;
- non-idempotent clock enable/disable, divider, selector, PLL, or
  frequency-counter ownership;
- RP1_CLK_SYS pclk/hclk writes or transitions;
- reset-controller ownership;
- GPIO32 ownership or PHY reset assertion/deassertion;
- MDIO transactions or PHY ownership;
- interrupt delivery, handler ownership, or completion;
- DMA, descriptor rings, channel ownership, or transfer completion;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- CLK_ETH_TSU_CTRL idempotent write/restore would prove only one
  Ethernet-private clock-manager store/readback boundary, not broad Ethernet
  clock ownership.
- The selected observed-aperture address depends on the accepted 0x1c RP1
  aperture frontier; a future hardware proof must still record identity, TFTP,
  serial freshness, final identity, and restore evidence.
- CLK_ETH_CTRL remains unselected and may need a separate contract before
  direct transmit-clock ownership.
- PHY reset remains GPIO32/MDIO-owned and must not be hidden under
  clock/reset work.

## Evidence

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-target-source-contract.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-target-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-target-source-contract/evidence-map.json.
- Accepted read-only baseline closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-closeout.md.
- Accepted read-only baseline proof:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof.md.
- Accepted clock/reset ownership contract:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-ownership-contract.md.
- Accepted clock/reset guard core:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-guard-core.md.
- Retained Linux Ethernet source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- Retained Linux clock ids:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-clock.h.
- Retained Linux clock-manager source:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c.
- Accepted observed-aperture clock/reset dependency source contract:
  tasks/2026-06-09-phase11-rp1-clock-reset-dependency-source-contract.md.

## Validation

- static inspection: accepted clock/reset contract, guard core, read-only
  baseline proof/closeout, retained Linux source excerpts, Phase 11 observed
  aperture source contract, project docs, and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- documentation build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Contract identifies exactly one selected Ethernet-private clock target with
  source evidence, Talos register address, width, writable fields, preserved
  fields, pre-read/write/restore/readback sequence, and safety invariants:
  satisfied by CLK_ETH_TSU_CTRL at 0x1c00018134.
- Contract rejects RP1_CLK_SYS pclk/hclk writes and keeps reset-controller,
  GPIO32/PHY reset, MDIO/PHY, DMA, descriptors, interrupts, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition out of scope:
  satisfied.
- Contract defines paired control requirements and the future hardware proof
  classification set without accepting runtime ownership by implication:
  satisfied.
- NextAction selects exactly one mechanically objective follow-up because the
  target/restore contract is accepted: satisfied.
- Accepted contract is committed before follow-up starts: satisfied by commit
  recorded in supervisor state after acceptance.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clock-reset-write-restore-core-20260610 on the next
worker wake if it remains queued and dependencies are satisfied. The core may
only implement local/static candidate/control report evidence derived from
this source contract. Do not acquire hardwareTestLock, publish boot archives,
run hardware, perform runtime writes, select CLK_ETH_CTRL, touch RP1_CLK_SYS,
reset-controller, GPIO32/PHY reset, MDIO, DMA, descriptors, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.
