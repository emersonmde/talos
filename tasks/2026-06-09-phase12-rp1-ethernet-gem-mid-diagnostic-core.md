# Phase 12 RP1 Ethernet GEM MID Diagnostic Core

Task: phase12-rp1-ethernet-gem-mid-diagnostic-core-20260609

Status: accepted

Evidence level: static inspection of the accepted GEM MID source contract,
local/static Rust implementation, focused QEMU-backed no_std test run, JSON
checks, documentation build, and git diff checks.

## Goal

Implement the local/static candidate and paired no-Ethernet/no-MMIO control
report construction for the accepted RP1 Ethernet GEM MID source contract.

## Scope

- Consume the accepted source contract
  phase12-rp1-ethernet-gem-mid-source-contract-20260609.
- Add a bounded report module for candidate and control construction.
- Preserve exact source-contract identity, rp1_eth target, MACB_MID register
  address, width, source evidence, rejected claims, retained risks, and
  hardware-proof boundary classification.
- Make the paired control use the same report construction path while
  withholding Ethernet MMIO target fields and carrying an explicit
  no-Ethernet/no-MMIO classification.
- Reject overclaiming inputs for Ethernet readiness, broad MMIO readiness, RP1
  MMIO/DMA programming, descriptor rings, DMA ownership, transfer completion,
  interrupt completion, clock/reset or PHY ownership, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition.
- Record findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
live Ethernet MMIO read, RP1 MMIO write, RP1 DMA programming, descriptor ring,
DMA ownership, transfer completion, interrupt completion, clock/reset
ownership, PHY reset ownership, packet I/O, network stack, sockets, SSH, Phase
12.2 work, or phase transition.

## Implementation

- src/rp1_ethernet.rs defines
  RP1_ETHERNET_GEM_MID_DIAGNOSTIC_REPORT_CONTRACT_ID as
  phase12-rp1-ethernet-gem-mid-diagnostic-report-contract-v1.
- Candidate report construction requires accepted source-contract evidence and
  emits the exact MACB_MID target:
  - controller: rp1_eth
  - compatible: raspberrypi,rp1-gem / cdns,macb
  - RP1 bus base: 0xc0_40100000
  - CPU physical base: 0x1f00100000
  - register: MACB_MID
  - offset: 0x00fc
  - RP1 bus target: 0xc0_401000fc
  - CPU physical target: 0x1f001000fc
  - width: 32 bits
  - endianness/access: little-endian read-only volatile load
- Control report construction carries the same report contract id and source
  contract id, but all Ethernet MMIO target fields are None and classification
  is no-ethernet-no-mmio-rp1-ethernet-gem-mid-control.
- Rejected-input evidence returns contract-rejected-input plus the deterministic
  rejection name.

## Findings

- fixed: added local/static GEM MID candidate report construction for the
  accepted source contract only.
- fixed: added paired no-Ethernet/no-MMIO control construction that withholds
  controller/register/address fields while preserving report identity.
- fixed: added deterministic validators for source-contract bypass, target
  mismatch, field mismatch, missing source evidence, and runtime/hardware
  overclaims.
- fixed: added focused tests for candidate construction, control construction,
  source-contract bypass rejection, and overclaim rejection.
- deferred: Pi 5 proof, boot archive publication, live MMIO visibility,
  bridge/outbound completion, clock/reset ownership, PHY reset ownership,
  descriptor rings, DMA, interrupts, packets, networking, sockets, SSH, and
  Phase 12.2.
- not-an-issue: this task is local/static and therefore does not acquire
  hardwareTestLock or publish boot artifacts.

No findings were removed.

## Validation

- static inspection: reviewed accepted source contract, source inventory
  excerpts, path ADR, src/rp1_ethernet.rs, src/main.rs, docs, and task
  evidence.
- fmt: cargo fmt --all -- --check passed after formatting.
- focused tests: cargo -Zjson-target-spec test --quiet rp1_ethernet passed
  with the QEMU runner on 460 no_std tests, including the four
  rp1_ethernet::tests cases.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Findings with disposition: satisfied.
- Candidate/control report construction for the accepted GEM MID source
  contract only: satisfied by src/rp1_ethernet.rs.
- Candidate includes contract identity, register target, address, width, source
  evidence, rejected claims, retained risks, and hardware-proof boundary:
  satisfied.
- Control preserves report path while withholding Ethernet MMIO target
  construction and carrying explicit no-Ethernet/no-MMIO classification:
  satisfied.
- Validators reject overclaiming inputs for Ethernet readiness, broad MMIO, RP1
  MMIO/DMA programming, descriptor rings, DMA ownership, transfer/interrupt
  completion, clock/reset or PHY ownership, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition: satisfied.
- Focused tests cover candidate construction, control construction, and
  deterministic rejection cases: satisfied.
- Accepted implementation/evidence is committed before closeout starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote phase12-rp1-ethernet-gem-mid-diagnostic-closeout-20260609
on the next worker wake. Do not run hardware, publish boot archives, acquire
hardwareTestLock, claim live Ethernet MMIO readiness, program RP1 MMIO/DMA,
construct descriptor rings, claim transfer or interrupt completion, perform
packet I/O, build networking, sockets, SSH, Phase 12.2, or phase transition
from this local/static implementation alone.
