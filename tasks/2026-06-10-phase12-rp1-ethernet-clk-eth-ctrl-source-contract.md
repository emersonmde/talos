# Phase 12 RP1 Ethernet CLK_ETH_CTRL Source Contract

Task id: phase12-rp1-ethernet-clk-eth-ctrl-source-contract-20260610
Status: accepted
Owner: worker
Classification: rp1-ethernet-clk-eth-ctrl-source-contract-accepted
Evidence level: static inspection of the accepted CLK_ETH_TSU_CTRL
write/restore proof closeout, retained Raspberry Pi Linux clock/rp1_eth
source excerpts, Talos clock/reset context, project docs, and task-owned JSON.
No Pi 5 hardware run, RP1 MMIO write, runtime implementation, boot archive
publication, or hardwareTestLock acquisition was performed.

## Goal

Define the next bounded Ethernet-private clock write/restore contract for
CLK_ETH_CTRL / RP1_CLK_ETH / rp1_eth tx_clk without touching shared pclk/hclk
or runtime Ethernet behavior.

## Scope

- Consumed accepted CLK_ETH_TSU_CTRL write/restore proof closeout commit
  7fced20c614f677a41def1b27bf76f3ed49ba5c8.
- Reconciled retained Linux source facts for rp1_eth tx_clk against the
  accepted observed 0x1c RP1 aperture and existing clock-manager
  write/restore discipline.
- Selected one target: CLK_ETH_CTRL for RP1_CLK_ETH / rp1_eth tx_clk.
- Defined exact source offset, Talos register address, width, writable value
  rule, preserved fields, operation order, paired control requirements, future
  proof classification set, rejected claims, and retained risks.
- Recorded why this task is materially different from the accepted
  CLK_ETH_TSU_CTRL proof and still does not imply broad clock/reset ownership
  or downstream Ethernet readiness.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, no Pi 5 hardware run, no boot archive publication,
no hardwareTestLock acquisition, no RP1 MMIO write, no non-idempotent clock
transition, no RP1_CLK_SYS pclk/hclk write, no divider/source/PLL/
frequency-counter/GPCLK output-enable write, no reset-controller write, no
GPIO32/PHY reset, no MDIO/PHY ownership, no DMA, no descriptors, no
interrupts or completions, no packet I/O, no networking, no sockets, no SSH,
no Phase 12.2, and no phase transition.

## Accepted Input Frontier

The accepted input frontier remains narrow:

- accepted observed RP1 aperture base 0x1c00000000;
- accepted rp1_eth source facts for pclk, hclk, tsu_clk, and tx_clk;
- accepted CLK_ETH_TSU_CTRL idempotent write/restore proof and closeout;
- retained Linux clock-manager source excerpts for RP1_CLK_ETH / clk_eth.

This contract does not reinterpret the accepted TSU write/restore proof as
broad clock/reset ownership and does not accept Ethernet driver readiness,
broad Ethernet MMIO readiness, shared-clock writes, reset-controller
ownership, GPIO32/PHY reset ownership, MDIO/PHY ownership, interrupts, DMA,
descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Selected Target

Contract id: phase12-rp1-ethernet-clk-eth-ctrl-source-contract-v1

    target: rp1-ethernet-clk-eth-ctrl-idempotent-write-restore
    clock name: tx_clk
    clock id: RP1_CLK_ETH (16)
    clock-manager source register: CLK_ETH_CTRL
    source block: RP1 clocks@18000, compatible raspberrypi,rp1-clocks
    observed RP1 base: 0x1c00000000
    source offset from observed RP1 base: 0x018064
    Talos register address: 0x1c00018064
    width: 32-bit little-endian volatile load/store
    allowed write value: the pre-read raw value only

CLK_ETH_CTRL is selected because the retained rp1_eth device-tree source names
tx_clk as RP1_CLK_ETH, Linux assigns RP1_CLK_ETH id 16 and 125 MHz, and
clk-rp1.c maps RP1_CLK_ETH to CLK_ETH_CTRL. It is materially different from
the accepted TSU proof because it selects the direct rp1_eth transmit clock
control register instead of the timestamp-unit control register at
CLK_ETH_TSU_CTRL. The allowed operation is still only an idempotent write of
the pre-read raw value followed by readback and restore; this does not accept
non-idempotent clock transitions, direct transmit-clock ownership, or runtime
Ethernet behavior.

## Source Evidence

Retained source facts supporting the target:

- linux-rpi-6.12-rp1.dtsi lines 999-1009 define rp1_eth at ethernet@100000,
  compatible raspberrypi,rp1-gem / cdns,macb, with clocks RP1_CLK_SYS,
  RP1_CLK_SYS, RP1_CLK_ETH_TSU, and RP1_CLK_ETH named pclk, hclk, tsu_clk,
  and tx_clk.
- linux-rpi-6.12-rp1.dtsi lines 24-59 define the RP1 clock manager at
  clocks@18000, assign RP1_CLK_ETH, and set its assigned rate to 125 MHz.
- linux-rpi-6.12-rp1-clock.h defines RP1_CLK_ETH as clock id 16 and
  RP1_CLK_ETH_TSU as clock id 29.
- clk-rp1.c lines 65-67 define CLK_ETH_CTRL at offset 0x00064,
  CLK_ETH_DIV_INT at 0x00068, and CLK_ETH_SEL at 0x00070.
- clk-rp1.c lines 1920-1939 register clk_eth with ctrl register
  CLK_ETH_CTRL, divider register CLK_ETH_DIV_INT, select register CLK_ETH_SEL,
  max frequency 125 MHz, and no GPCLK output-enable mask in the retained
  descriptor.
- clk-rp1.c lines 1033-1067 show Linux enable/disable helpers modify only
  CLK_CTRL_ENABLE in a clock's own control register unless the descriptor has
  a nonzero GPCLK output-enable mask.
- clk-rp1.c lines 239-243 define CLK_CTRL_ENABLE bit 11,
  CLK_CTRL_AUXSRC bits 9:5, and source bits starting at bit 0.

The accepted observed-aperture rule uses 0x1c00000000 plus source offset for
visible RP1 clock-manager registers. Applying that rule to clock-manager base
offset 0x018000 plus CLK_ETH_CTRL offset 0x00064 yields 0x1c00018064.

## Allowed Future Candidate Operations

A future real candidate may perform only this ordered sequence:

1. Pre-read CLK_ETH_CTRL at 0x1c00018064 and retain pre_raw.
2. Write pre_raw back to CLK_ETH_CTRL at 0x1c00018064.
3. Post-read CLK_ETH_CTRL and retain post_raw.
4. Restore-write pre_raw back to CLK_ETH_CTRL.
5. Restore-read CLK_ETH_CTRL and retain restore_raw.

The first write is intentionally idempotent. A partial run after that write
does not intentionally change hardware state because the written value is the
pre-read value. The restore operation writes the same value again.

## Preserved Fields

The future proof must preserve all fields unless a later source contract
explicitly changes scope:

- the full 32-bit raw CLK_ETH_CTRL value;
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
- Do not write CLK_ETH_DIV_INT, CLK_ETH_SEL, CLK_ETH_TSU_CTRL,
  CLK_ETH_TSU_DIV_INT, CLK_ETH_TSU_SEL, PLL registers, frequency-counter
  registers, or GPCLK_OE_CTRL.
- Do not treat this as a same-shaped CLK_ETH_TSU_CTRL retry; that proof is
  already closed.
- Do not use reset-controller writes because the retained Pi 5 rp1_eth node
  supplies no accepted reset target or restore semantics.
- Do not assert or deassert GPIO32 PHY reset and do not perform MDIO
  transactions.
- Do not infer packet I/O, interrupts, DMA, descriptor readiness, networking,
  sockets, SSH, Phase 12.2, or phase transition from this clock write/restore
  contract.

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

- rp1-ethernet-clk-eth-ctrl-idempotent-write-restored
- rp1-ethernet-clk-eth-ctrl-idempotent-write-mismatch-restored
- rp1-ethernet-clk-eth-ctrl-idempotent-write-restore-failed
- rp1-ethernet-clk-eth-ctrl-blocked-missing-clock-manager
- rp1-ethernet-clk-eth-ctrl-inconclusive-capture
- no-clock-write-no-ethernet-rp1-ethernet-clk-eth-ctrl-control
- staging/build-blocker

Every accepted hardware classification must remain bounded to the selected
idempotent write/restore proof and paired control output.

## Findings

- fixed: selected exactly one Ethernet-private target, CLK_ETH_CTRL for
  RP1_CLK_ETH / tx_clk.
- fixed: derived the exact Talos register address 0x1c00018064 from the
  accepted observed RP1 aperture plus retained clock-manager/source offsets.
- fixed: defined the only allowed write as the pre-read raw value, with
  post-read, restore-write, and restore-read evidence.
- fixed: preserved all control-register fields, including enable, auxsource,
  source, and reserved bits.
- fixed: rejected writes to shared RP1_CLK_SYS, the already-closed TSU target,
  divider, select, PLL, frequency-counter, GPCLK output-enable,
  reset-controller, GPIO32/PHY reset, MDIO, DMA, descriptor, interrupt,
  packet, networking, SSH, Phase 12.2, and phase-transition surfaces.
- fixed: defined paired control and future proof classifications without
  accepting direct transmit-clock ownership or runtime Ethernet behavior by
  implication.
- deferred: local/static write/restore report core, serialized Pi 5
  write/restore proof, non-idempotent field transitions, GPIO32/PHY reset,
  MDIO/PHY, interrupts, DMA, descriptors, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition remain future tasks.
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
- same-shaped CLK_ETH_TSU_CTRL retry claims;
- reset-controller ownership;
- GPIO32 ownership or PHY reset assertion/deassertion;
- MDIO transactions or PHY ownership;
- interrupt delivery, handler ownership, or completion;
- DMA, descriptor rings, channel ownership, or transfer completion;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- CLK_ETH_CTRL idempotent write/restore would prove only one
  Ethernet-private tx clock-manager store/readback boundary, not broad
  Ethernet clock ownership.
- The selected observed-aperture address depends on the accepted 0x1c RP1
  aperture frontier; a future hardware proof must still record identity, TFTP,
  serial freshness, final identity, and restore evidence.
- Non-idempotent clock transitions remain unaccepted.
- PHY reset remains GPIO32/MDIO-owned and must not be hidden under
  clock/reset work.

## Evidence

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-source-contract.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-source-contract/evidence-map.json.
- Accepted TSU write/restore proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout.md.
- Accepted TSU write/restore hardware proof:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof.md.
- Retained Linux Ethernet source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- Retained Linux clock ids:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-clock.h.
- Retained Linux clock-manager source:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c.
- Talos clock/reset source context:
  src/rp1_ethernet.rs.

## Validation

- static inspection: accepted TSU write/restore proof closeout, retained Linux
  clock/rp1_eth source excerpts, Talos clock/reset source context, project
  docs, and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- documentation build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Contract selects exactly one target: satisfied by CLK_ETH_CTRL for
  RP1_CLK_ETH / rp1_eth tx_clk.
- Contract defines exact source evidence, observed-aperture address derivation,
  volatile access width, pre-read/write/post-read/restore-write/restore-read
  order, preserved fields, equality/restore reporting, paired control
  requirements, and allowed future classifications: satisfied.
- Contract rejects shared RP1_CLK_SYS pclk/hclk writes, TSU same-shaped
  retries, non-idempotent transitions, reset-controller, GPIO32/PHY reset,
  MDIO/PHY, DMA, descriptors, interrupts, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition: satisfied.
- Accepted source contract is committed before implementation/core work
  starts: satisfied by commit recorded in supervisor state after acceptance.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core-20260610 on the next
worker wake if it remains queued and dependencies are satisfied. The core may
only implement local/static candidate/control report evidence derived from
this source contract. Do not acquire hardwareTestLock, publish boot archives,
run hardware, perform runtime writes, touch RP1_CLK_SYS, repeat the TSU
proof, select reset-controller/GPIO32/PHY/MDIO/DMA/descriptor/interrupt/
packet/network/socket/SSH surfaces, Phase 12.2, or a phase transition.
