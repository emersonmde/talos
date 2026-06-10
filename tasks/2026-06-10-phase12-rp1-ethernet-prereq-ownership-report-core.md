# Phase 12 RP1 Ethernet Prerequisite Ownership Report Core

Task id: phase12-rp1-ethernet-prereq-ownership-report-core-20260610
Status: accepted
Owner: worker
Evidence level: local/static implementation, static inspection, fmt, QEMU-backed no_std tests, JSON validation, docs validation, diff checks

## Goal

Implement the deterministic local/static candidate and paired control report
surface selected by the accepted RP1 Ethernet prerequisite ownership source
contract.

## Scope

- Consumed the accepted source contract
  phase12-rp1-ethernet-prereq-ownership-source-contract-20260610.
- Added source-backed prerequisite ownership report data in
  src/rp1_ethernet.rs.
- Preserved observed-window MACB_MID identity only as context: target
  0x1c001000fc, raw 0x70109, idnum 0x7, rev 0x109.
- Preserved rp1_eth prerequisite facts: compatible strings, RP1 bus window,
  RP1_INT_ETH 6, clock names and ids, RGMII-ID phy1, RP1 GPIO32 active-low
  PHY reset for 5 ms, Cadence/RP1 config, and no-ownership policy
  classifications.
- Added paired control evidence that uses the same report path while
  withholding candidate-only Ethernet prerequisite facts.
- Added deterministic validators for shape bypasses and forbidden ownership or
  runtime readiness claims.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
RP1 MMIO writes, clock/reset ownership, GPIO32 or PHY reset ownership, MDIO
transactions, PHY ownership, DMA, descriptor rings, interrupt delivery or
completion, packet I/O, networking, sockets, SSH, Phase 12.2 work, or phase
transition.

## Implementation

- Added the contract/source-task constants and prerequisite source facts:
  clock ids, clock names, interrupt number, PHY reset route, PHY/MDIO policy,
  and DMA/descriptor dependency policy.
- Added Rp1EthernetPrereqOwnershipSourceContractEvidence,
  Rp1EthernetPrereqOwnershipReportInput, Rp1EthernetPrereqOwnershipReport, and
  Rp1EthernetPrereqOwnershipReportEvidence.
- Added build_rp1_ethernet_prereq_ownership_report and
  rp1_ethernet_prereq_ownership_report_evidence.
- Added rejected_rp1_ethernet_prereq_ownership_report_evidence for stable
  rejection classification output.
- Added focused tests for candidate report construction, paired control report
  construction, shape/source-contract rejection, and every forbidden runtime or
  ownership claim.

## Findings

- fixed: implemented the accepted local/static candidate report field set from
  the source contract.
- fixed: implemented the paired no-ownership/no-Ethernet prerequisite control
  that withholds candidate-only facts.
- fixed: validators reject source-contract bypasses, control attempts carrying
  Ethernet prerequisite facts, and all forbidden ownership/readiness claims.
- fixed: tests cover candidate construction, control construction, source shape
  bypasses, and deterministic overclaim rejection.
- deferred: serialized Pi 5 report visibility proof, any actual
  clock/reset/GPIO32/PHY/MDIO/DMA/interrupt ownership, Ethernet driver
  readiness, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  local/static implementation only.

No findings were removed.

## Evidence

- static inspection: accepted source contract and touched src/rp1_ethernet.rs.
- fmt: cargo fmt --all -- --check passed after formatting.
- focused tests: cargo -Zjson-target-spec test --quiet rp1_ethernet passed
  under the documented QEMU tool path; 472 no_std tests passed, including the
  new rp1_ethernet prerequisite ownership candidate/control/rejection tests.
- JSON validation: jq empty on task-owned evidence-map/classification JSON
  passed.
- diff check: git diff --check passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed with the existing
  large search-index warning.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Implementation exposes deterministic candidate and paired control report
  evidence derived only from the accepted prerequisite ownership source
  contract: satisfied.
- Candidate report includes accepted MACB_MID identity context and every
  selected prerequisite/preflight field required by the contract: satisfied.
- Control report preserves the report path while withholding candidate-only
  MMIO/prerequisite targets and carrying
  no-ownership-no-ethernet-rp1-ethernet-prereq-control: satisfied.
- Validators reject forbidden runtime/hardware ownership and readiness claims:
  satisfied.
- Focused tests cover accepted candidate, accepted control, and deterministic
  rejection cases: satisfied.
- Accepted implementation/evidence committed before closeout starts:
  satisfied by commit recorded after acceptance.

## Next Action

Mechanically promote
phase12-rp1-ethernet-prereq-ownership-report-closeout-20260610 on the next
worker wake. The closeout must reconcile this local/static report-core evidence
against the source contract without expanding acceptance to hardware/runtime
ownership, Ethernet readiness, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.
