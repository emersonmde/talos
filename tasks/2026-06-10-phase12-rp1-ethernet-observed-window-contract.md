# Phase 12 RP1 Ethernet Observed Window Contract

Task: phase12-rp1-ethernet-observed-window-contract-20260610

Status: accepted

Classification: rp1-ethernet-observed-window-contract-accepted

Evidence level: static inspection of accepted v2 proof and closeout task
records, Phase 12 networking docs, Phase 11 RP1/PCIe map contract, retained
RP1 Ethernet source evidence, and task-owned JSON. No Pi 5 hardware run or
runtime implementation was performed.

## Goal

Define the next materially different read-only RP1 Ethernet discriminator by
reconciling the accepted observed RP1 SYSINFO window with the retained
translated 0x1f GEM MID sentinel.

## Scope

- Consumed the accepted v2 closeout at commit
  32f8196fb58f9bf1a9b5cbf162fe648a97d6ecae.
- Preserved the accepted positive-control result: SYSINFO_CHIP_ID at
  0x1c00000000 returned 0x20001927.
- Preserved the accepted retained sentinel: translated-window MACB_MID at
  0x1f001000fc returned 0xdeaddead.
- Defined the observed-window candidate target by adding the retained
  rp1_eth/MACB_MID source offset 0x001000fc to the observed RP1 base
  0x1c00000000, yielding 0x1c001000fc.
- Retained 0x1f001000fc only as a comparator/sentinel target for the later
  discriminator.
- Defined the paired no-MMIO/no-Ethernet control report boundary and required
  fields for a later local/static implementation.

## Non-Goals

No runtime code changes except docs and evidence records, no Pi 5 hardware
run, no boot archive publication, no hardwareTestLock acquisition, no RP1
MMIO execution or writes, no DMA, descriptor rings, interrupts,
clock/reset/PHY/MDIO, packet I/O, networking, sockets, SSH, Phase 12.2 work,
or phase transition. This task does not re-open the same-shaped 0x1f GEM MID
decode-discriminator proof as progress.

## Reconciled Inputs

- tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof.md.
- tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout.md.
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout/classification.json.
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout/evidence-map.json.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-source-contract/source/linux-rpi-6.12-macb.h.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/project/phase11-rp1-pcie-map-contract.md.
- docs/src/roadmap.md.

## Observed-Window Contract

The next local/static discriminator contract is:

| Field | Value |
| --- | --- |
| contract id | phase12-rp1-ethernet-observed-window-contract-v1 |
| controller | rp1_eth |
| compatible | raspberrypi,rp1-gem / cdns,macb |
| register | MACB_MID |
| source offset | 0x001000fc |
| source register offset | 0x00fc from rp1_eth |
| observed RP1 base | 0x1c00000000 |
| observed-window candidate target | 0x1c001000fc |
| translated-window comparator target | 0x1f001000fc |
| positive control | SYSINFO_CHIP_ID at 0x1c00000000 |
| positive-control expected value | 0x20001927 |
| width | 32 bits |
| endianness | little-endian |
| access contract | volatile read-only load |

The source offset is retained from the accepted rp1_eth source contract:
rp1.dtsi defines rp1_eth at RP1 bus 0xc0_40100000 and macb.h defines MACB_MID
as offset 0x00fc. The observed-window candidate intentionally does not use the
previous source-translated CPU physical base 0x1f00100000 as the candidate
base. Instead it tests whether the same rp1_eth register offset is visible
under the observed 0x1c RP1 aperture that already returned the RP1 chip ID
0x20001927.

The translated-window target 0x1f001000fc remains useful only as a comparator
and sentinel. A later candidate report may include both targets to demonstrate
that 0x1c001000fc is a materially different read from the accepted
0x1f001000fc sentinel proof, but the report must not classify the 0x1f result
as new progress by itself.

## Required Future Candidate Fields

A later local/static implementation selected by this contract must emit a
candidate report that preserves:

- contract id phase12-rp1-ethernet-observed-window-contract-v1;
- source contract id phase12-rp1-ethernet-gem-mid-source-contract-20260609;
- controller rp1_eth and compatible raspberrypi,rp1-gem / cdns,macb;
- register MACB_MID and source register offset 0x00fc;
- observed RP1 base 0x1c00000000;
- observed-window candidate target 0x1c001000fc;
- translated-window comparator target 0x1f001000fc;
- positive-control register SYSINFO_CHIP_ID at 0x1c00000000 with expected
  value 0x20001927;
- width 32, little-endian, volatile read-only access;
- explicit rejected runtime/hardware claims and retained risks.

The future report may only classify report construction and, in a later
separate serialized proof, read-only candidate/control output. It must not
accept live GEM visibility or broader Ethernet readiness in the local/static
core.

## Paired Control Boundary

The paired control must preserve the same report shape and serial/reporting
path while constructing no observed RP1 target, no translated comparator
target, and no Ethernet MMIO target. The control classification must be
explicit, for example
no-mmio-no-ethernet-rp1-ethernet-observed-window-control.

The control may include the contract id, source contract id, positive-control
identity, rejected-claim list, and retained risks as inert report metadata.
It must reject any visible GEM MID value as control evidence.

## Material Difference From Same-Shaped 0x1f Retries

This contract is materially different from the closed v2 hardware proof
because it changes the candidate target from translated-window MACB_MID at
0x1f001000fc to observed-window MACB_MID at 0x1c001000fc while retaining
0x1f001000fc only as a comparator. The accepted v2 evidence already showed
that 0x1f001000fc returns 0xdeaddead even when same-run SYSINFO at
0x1c00000000 returns 0x20001927. Repeating that same 0x1f candidate/control
proof is closed; testing the same rp1_eth source offset under the observed
0x1c aperture is a different bounded address-window discriminator.

The contract remains read-only and non-destructive. It permits only 32-bit
volatile loads in a future selected proof, carries a no-MMIO/no-Ethernet
paired control, and forbids RP1 writes, DMA, descriptor rings, interrupts,
clock/reset/PHY ownership, packet I/O, networking, sockets, SSH, Phase 12.2,
and phase transition claims.

## Findings

- fixed: defined observed-window MACB_MID candidate target 0x1c001000fc from
  observed RP1 base 0x1c00000000 plus rp1_eth/MACB_MID offset 0x001000fc.
- fixed: preserved SYSINFO_CHIP_ID at 0x1c00000000 returning 0x20001927 as
  the positive control for the later discriminator.
- fixed: retained translated-window MACB_MID at 0x1f001000fc only as the
  comparator/sentinel target already accepted as 0xdeaddead.
- fixed: specified the paired no-MMIO/no-Ethernet control boundary for later
  local/static report construction.
- fixed: explained why this is not a same-shaped 0x1f retry and why it
  remains read-only/non-destructive.
- deferred: local/static report implementation, Pi 5 hardware proof, live GEM
  visibility, broad Ethernet MMIO readiness, Ethernet driver readiness,
  PCIe/RP1 bridge/window ownership, clock/reset/PHY/MDIO ownership, DMA,
  descriptor rings, interrupts, packet I/O, networking, sockets, SSH, Phase
  12.2, and phase transition remain future work.
- not-an-issue: no hardwareTestLock was acquired because this is a
  source/evidence contract only.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims:

- live GEM visibility;
- broad Ethernet MMIO readiness;
- Ethernet driver readiness;
- RP1 MMIO writes or DMA programming;
- descriptor rings or DMA ownership;
- transfer completion or interrupt completion;
- clock/reset ownership;
- PHY reset or MDIO ownership;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- 0x1c001000fc may still return a sentinel or fault in the future hardware
  proof.
- PCIe/RP1 bridge and address-window ownership remain unaccepted.
- Ethernet clock/reset and PHY/MDIO ownership remain unaccepted.
- A later observed-window proof still requires hardwareTestLock, candidate and
  control identity, TFTP evidence, serial freshness, final identity, restore
  proof, and post-run review before any hardware claim is accepted.

## Evidence

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-contract.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-contract/evidence-map.json.
- Accepted v2 proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof.md.
- Accepted v2 closeout task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout.md.
- Retained rp1_eth source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- Retained MACB_MID source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-source-contract/source/linux-rpi-6.12-macb.h.
- Project docs:
  docs/src/project/phase12-networking-ssh.md and
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: reviewed accepted v2 proof and closeout task records,
  accepted closeout classification/evidence JSON, Phase 12 networking docs,
  Phase 11 RP1/PCIe map contract, retained rp1_eth source, and retained
  MACB_MID source.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract states observed-window candidate 0x1c001000fc, translated-window
  comparator 0x1f001000fc, SYSINFO positive-control 0x1c00000000/0x20001927,
  source rp1_eth/MACB_MID offset 0x001000fc, and paired no-MMIO/no-Ethernet
  control boundary: satisfied.
- Contract explains why this is materially different from same-shaped 0x1f GEM
  MID retries and why it remains read-only/non-destructive: satisfied.
- Contract rejects live GEM visibility, broad Ethernet MMIO readiness, driver
  readiness, RP1 MMIO writes, DMA, descriptor rings, interrupts,
  clock/reset/PHY ownership, packet I/O, networking, sockets, SSH, Phase
  12.2, and phase transition claims: satisfied.
- Accepted contract is committed before the local/static discriminator core
  starts: satisfied by the commit recorded in supervisor state after this
  task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-observed-window-discriminator-core-20260610 on the next
worker wake. That task may implement only local/static candidate and paired
control report construction for this contract. It must not run hardware,
publish a boot archive, acquire hardwareTestLock, program RP1 MMIO/DMA,
create descriptor rings, claim interrupts/clock/reset/PHY ownership, perform
packet I/O, add networking/sockets/SSH, start Phase 12.2, or infer a phase
transition.
