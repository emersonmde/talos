# Phase 12 RP1 Ethernet Clock/Reset Guard Core

Task id: phase12-rp1-ethernet-clock-reset-guard-core-20260610
Status: accepted
Owner: worker
Evidence level: local/static implementation, static inspection, fmt,
QEMU-backed no_std tests, JSON validation, diff checks

## Goal

Implement the deterministic local/static candidate and paired control
clock/reset guard surface selected by the accepted RP1 Ethernet clock/reset
ownership contract.

## Scope

- Consumed the accepted ownership contract
  phase12-rp1-ethernet-clock-reset-ownership-contract-20260610.
- Added local/static clock/reset guard report data in src/rp1_ethernet.rs.
- Preserved observed-window MACB_MID identity only as context: target
  0x1c001000fc, raw 0x70109, idnum 0x7, rev 0x109.
- Preserved exact clock/reset source facts: pclk/hclk share RP1_CLK_SYS id
  12; tsu_clk maps to RP1_CLK_ETH_TSU id 29; tx_clk maps to RP1_CLK_ETH id
  16; retained Pi 5 rp1_eth source supplies no accepted reset-controller
  target; PHY reset remains GPIO32/MDIO-owned.
- Added paired no-clock-reset/no-Ethernet control evidence that uses the same
  report path while withholding candidate-only Ethernet clock/reset facts.
- Added deterministic validators for shape bypasses and forbidden runtime,
  hardware, ownership, and phase claims.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
RP1 MMIO writes, clock/reset writes or ownership, RP1_CLK_SYS transition,
reset-controller ownership, GPIO32 or PHY reset ownership, MDIO transactions,
PHY ownership, DMA, descriptor rings, interrupt delivery or completion,
packet I/O, networking, sockets, SSH, Phase 12.2 work, or phase transition.

## Implementation

- Added the guard contract/source-task constants and local/static
  classification strings.
- Added shared-clock and Ethernet-private clock metadata plus read-only
  baseline requirements, future write-backed invariants, rejected claims, and
  retained risks from the accepted contract.
- Added Rp1EthernetClockResetGuardContractEvidence,
  Rp1EthernetClockResetGuardReportInput,
  Rp1EthernetClockResetGuardReport, and
  Rp1EthernetClockResetGuardReportEvidence.
- Added build_rp1_ethernet_clock_reset_guard_report and
  rp1_ethernet_clock_reset_guard_report_evidence.
- Added rejected_rp1_ethernet_clock_reset_guard_report_evidence for stable
  rejection classification output.
- Added focused tests for candidate construction, paired control construction,
  shape/source-contract rejection, and every forbidden runtime or ownership
  claim.

## Findings

- fixed: implemented the accepted local/static candidate report field set from
  the clock/reset ownership contract.
- fixed: implemented the paired no-clock-reset/no-Ethernet control that
  withholds candidate-only clock/reset facts.
- fixed: validators reject guard-contract bypasses, control attempts carrying
  clock/reset facts, and all forbidden runtime, hardware, ownership, and phase
  claims.
- fixed: tests cover candidate construction, control construction, contract
  shape bypasses, and deterministic overclaim rejection.
- deferred: serialized Pi 5 read-only baseline proof, any actual clock/reset
  register target selection, write-backed clock/reset ownership, GPIO32/PHY
  reset ownership, MDIO/PHY, interrupts, DMA, descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  local/static implementation only.

No findings were removed.

## Evidence

- static inspection: accepted clock/reset ownership contract and touched
  src/rp1_ethernet.rs.
- fmt: cargo fmt --all -- --check passed after formatting.
- focused tests: cargo -Zjson-target-spec test --quiet ethernet_clock_reset
  passed under the documented QEMU tool path; 476 no_std tests passed,
  including the new clock/reset guard candidate/control/rejection tests.
- JSON validation: jq empty on task-owned evidence-map/classification JSON
  passed.
- diff check: git diff --check passed.
- docs validation: not run; no docs/src files were touched.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Implementation exposes deterministic candidate and paired control report
  evidence derived only from the accepted clock/reset ownership contract:
  satisfied.
- Candidate report includes observed-window identity context, exact source
  clock facts, reset-controller target absence, read-only baseline
  requirements, write-backed invariants, rejected claims, and retained risks:
  satisfied.
- Control report preserves the report path while withholding candidate-only
  Ethernet clock/reset facts and carrying
  no-clock-reset-no-ethernet-rp1-ethernet-clock-reset-guard-control:
  satisfied.
- Validators reject forbidden runtime/hardware ownership and downstream
  Ethernet behavior claims: satisfied.
- Focused tests cover accepted candidate, accepted control, and deterministic
  rejection cases: satisfied.
- Accepted implementation/evidence committed before closeout starts:
  satisfied by commit recorded in supervisor state after acceptance.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clock-reset-guard-closeout-20260610 on the next worker
wake if the queued task remains mechanically unblocked. The closeout must
reconcile this local/static guard-core evidence against the accepted contract
without expanding acceptance to hardware/runtime ownership, clock/reset
writes, Ethernet readiness, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.
