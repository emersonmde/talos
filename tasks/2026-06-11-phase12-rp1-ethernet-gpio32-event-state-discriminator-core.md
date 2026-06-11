# Phase 12 RP1 Ethernet GPIO32 Event-State Discriminator Core

Task id: phase12-rp1-ethernet-gpio32-event-state-discriminator-core-20260611
Status: accepted
Classification: rp1-ethernet-gpio32-event-state-discriminator-local-static-accepted
Evidence level: local/static implementation, focused unit tests, fmt, JSON validation, and diff hygiene.

## Goal

Implement the local/static candidate and paired control report surface for the
accepted read-only GPIO32 event-state discriminator, without hardware, event
clearing, GPIO/RIO/pad/MMIO writes, or GPIO32 write/restore retry.

## Scope Performed

- Consumed the accepted event-state source contract and v2 blocked/no-write
  lineage.
- Added Rp1EthernetGpio32EventStateDiscriminatorReport data shapes in
  src/rp1_ethernet.rs.
- Candidate evidence preserves GPIO32 / ETH_RST_N target identity, STATUS/CTRL,
  RIO1 OUT/OE/IN, pad target fields, source-backed STATUS event bit names,
  v2 proof/closeout/commit/classification, exact accepted v2 raw values, and
  rejected claims.
- Paired no-GPIO/no-Ethernet control uses the same report builder/evidence
  path while carrying no GPIO32/RIO/pad/MMIO target facts and no raw values.
- Validators reject event clearing, GPIO/RIO/pad/MMIO writes, GPIO32 ownership,
  PHY reset assertion/deassertion, GPIO32 write/restore retry, MDIO/PHY
  ownership, Ethernet readiness, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition.
- Focused tests cover accepted candidate construction, accepted control
  construction, accepted v2 blocked-event-state lineage, source-unresolved
  classification, shape rejection, source contract mismatch, lineage mismatch,
  classification mismatch, and deterministic overclaim rejection.

## Findings

- fixed: src/rp1_ethernet.rs had no local/static report surface for the
  accepted GPIO32 event-state discriminator after the v2 no-write blocker.
- fixed: the candidate report now preserves the exact accepted v2 facts:
  STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT 0x10, RIO1 OE 0x10,
  RIO1 IN 0x12, event bits 0x0ab00000, and writes-performed=false.
- fixed: the source-backed STATUS event bit names are limited to bits 20-27
  from retained RP1 pinctrl evidence.
- fixed: source-unresolved event-state classification remains explicit rather
  than inferring stale, clearable, firmware-owned, harmless, or safe-to-ignore
  semantics.
- fixed: the paired control withholds GPIO32/RIO/pad/MMIO target facts and
  raw values while preserving the same report-builder/evidence path.
- not-an-issue: no docs/src update was required because the accepted Phase 12.1
  frontier wording and API names did not change.
- deferred: Pi 5 archive publication and serial read-only proof remain queued
  after the static closeout; this task intentionally performed no hardware
  action.

## Validation

- static inspection: accepted source contract, v2 proof/closeout lineage, and
  touched src/rp1_ethernet.rs.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet rp1_ethernet passed
  with 492 no_std tests, including the new GPIO32 event-state discriminator
  tests.
- JSON: jq empty passed for task-owned classification and evidence-map JSON.
- diff hygiene: git diff --check passed.
- staged diff hygiene: git diff --cached --check passed before commit.

## Acceptance

Accepted. The local/static report core is implemented and tested. It does not
accept hardware evidence, event clearing, GPIO32 ownership, PHY reset
assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior, DMA,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition.

## Next Action

Promote phase12-rp1-ethernet-gpio32-event-state-discriminator-closeout-20260611
on the next worker wake if dependencies remain satisfied. Do not publish
archives, run hardware, clear events, write GPIO/RIO/pad/MMIO, or retry GPIO32
write/restore from the closeout.
