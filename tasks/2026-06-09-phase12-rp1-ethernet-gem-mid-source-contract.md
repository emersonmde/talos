# Phase 12 RP1 Ethernet GEM MID Source Contract

Task: phase12-rp1-ethernet-gem-mid-source-contract-20260609

Status: accepted

Evidence level: static inspection of accepted Phase 12.1 source inventory,
accepted path ADR, retained Raspberry Pi Linux source excerpts, project docs,
JSON checks, documentation build, and git diff checks.

## Goal

Define the smallest source-only RP1 Ethernet/GEM read contract before any
diagnostic implementation or hardware run.

## Scope

- Consume the accepted RP1 Ethernet source inventory and path ADR.
- Name exactly one read-only Cadence MACB/GEM register target.
- Retain source evidence for the RP1 Ethernet base and MACB MID register
  offset plus IDNUM/REV fields.
- Define the paired no-Ethernet/no-MMIO control shape for the future
  diagnostic implementation.
- Preserve non-goals against Ethernet implementation, live broad MMIO, RP1
  MMIO writes, DMA, descriptor rings, interrupts, clock/reset or PHY ownership,
  packet I/O, networking, sockets, SSH, Phase 12.2, and hardware validation.
- Record findings with disposition.

## Non-Goals

No diagnostic implementation, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 MMIO write, RP1 DMA programming, descriptor
ring, DMA ownership, transfer completion, interrupt completion, clock/reset
write, PHY reset, packet I/O, network stack, sockets, SSH, Phase 12.2 work, or
hardware validation claim.

## Retained Inputs

- tasks/2026-06-09-phase12-rp1-ethernet-source-inventory.md
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/classification.json
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c
- tasks/2026-06-09-phase12-rp1-ethernet-path-adr.md
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-path-adr/classification.json
- tasks/2026-06-09-phase11-rp1-hardware-substrate-closeout.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-source-contract/source/linux-rpi-6.12-macb.h

## Source Contract

The accepted future diagnostic target is exactly one read-only register:

- controller: rp1_eth, compatible raspberrypi,rp1-gem / cdns,macb
- source RP1 bus base: 0xc0_40100000
- source CPU physical base: 0x1f00100000
- register: MACB_MID
- offset: 0x00fc
- source RP1 bus target: 0xc0_401000fc
- source CPU physical target: 0x1f001000fc
- width: 32 bits
- endianness: little-endian
- access contract: volatile load only

The retained rp1.dtsi source excerpt defines rp1_eth: ethernet@100000 with
reg = <0xc0 0x40100000 0x0 0x4000> and compatible strings
raspberrypi,rp1-gem, cdns,macb. The retained MACB header excerpt defines
MACB_MID as offset 0x00fc, with MACB_IDNUM_OFFSET = 16,
MACB_IDNUM_SIZE = 12, MACB_REV_OFFSET = 0, and MACB_REV_SIZE = 16. The
retained Linux MACB driver excerpt uses MACB_MID as the GEM identity register
in hw_is_gem().

This is a source contract, not hardware proof. It does not accept that
0x1f001000fc is readable on the Pi 5, that the bridge/outbound windows are
complete for rp1_eth, or that broad Ethernet MMIO is ready.

## Paired Control Shape

The future diagnostic core must preserve the same report construction and
eventual serial/reporting path while withholding the Ethernet MMIO target. The
control classification must be explicit, such as no-ethernet-no-mmio-control,
and must reject any visible GEM MID value as control evidence.

The control may include the contract id and rejected-claim list so the future
candidate/control outputs are comparable, but it must not construct the
0x1f001000fc target or imply Ethernet MMIO readiness.

## Rejected Claims

This contract does not accept:

- Ethernet driver readiness.
- Live broad RP1 Ethernet MMIO readiness.
- RP1 MMIO writes or RP1 DMA programming.
- Descriptor rings, DMA ownership, transfer completion, or interrupt
  completion.
- Clock/reset ownership or PHY reset ownership.
- Packet I/O, networking, sockets, SSH, or Phase 12.2 work.
- Pi 5 hardware validation or boot archive publication by this task.
- Treating the source-translated address as hardware-proven.

## Findings

- fixed: selected MACB_MID as the exact future read-only GEM identity target.
- fixed: retained the missing MACB header excerpt that defines MACB_MID,
  IDNUM, and REV.
- fixed: tied the target address to the accepted rp1_eth source base and
  source CPU physical translation.
- fixed: defined the paired no-Ethernet/no-MMIO control requirements for a
  future local/static diagnostic core.
- deferred: diagnostic implementation, Pi 5 hardware proof, broad Ethernet
  MMIO readiness, bridge/outbound completion, clock/reset ownership, PHY reset
  ownership, descriptor rings, DMA, interrupts, packets, networking, sockets,
  SSH, and Phase 12.2.
- not-an-issue: this task is source-only and therefore does not acquire
  hardwareTestLock or publish a boot archive.

No findings were removed.

## Validation

- static inspection: reviewed accepted source inventory, accepted path ADR,
  retained RP1 DTS, retained MACB driver excerpt, retained MACB header excerpt,
  Phase 11 closeout, project note, and roadmap.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Source contract names exactly one read-only target: satisfied by MACB_MID
  offset 0x00fc at RP1 bus 0xc0_401000fc / CPU physical 0x1f001000fc, width
  32, little-endian volatile load.
- Contract cites retained source evidence for rp1_eth base and MACB MID
  definitions: satisfied.
- Contract defines the paired no-Ethernet/no-MMIO control shape and report
  requirements: satisfied.
- Contract rejects Ethernet driver readiness, live broad MMIO readiness, RP1
  MMIO/DMA programming, descriptor rings, DMA ownership, transfer completion,
  interrupt completion, clock/reset ownership, PHY reset ownership, packet
  I/O, networking, sockets, SSH, and Phase 12.2: satisfied.
- Accepted source contract is committed before diagnostic implementation,
  hardware publication, or hardware proof starts: satisfied by the commit
  recorded in supervisor state after this task.

## Next Action

Mechanically promote phase12-rp1-ethernet-gem-mid-diagnostic-core-20260609 on
the next worker wake. That task may implement local/static candidate and
paired-control report construction for this source contract only. It must not
run hardware, publish a boot archive, acquire hardwareTestLock, claim live
Ethernet MMIO readiness, program RP1 MMIO/DMA, construct descriptor rings,
claim interrupt or transfer completion, perform packet I/O, build networking,
open sockets, add SSH, start Phase 12.2, or create a phase transition.
