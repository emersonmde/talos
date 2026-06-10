# Phase 12 RP1 Ethernet Observed Window Discriminator Core

Task: phase12-rp1-ethernet-observed-window-discriminator-core-20260610

Status: accepted

Classification: rp1-ethernet-observed-window-discriminator-local-static-accepted

Evidence level: local/static Rust report construction, focused unit tests,
static inspection, task-owned JSON, and docs validation. No Pi 5 hardware run
was performed.

## Goal

Implement the local/static observed-window GEM MID discriminator report surface
without running hardware or accepting Ethernet behavior.

## Scope

- Consumed the accepted observed-window contract from
  phase12-rp1-ethernet-observed-window-contract-20260610.
- Added candidate report construction for SYSINFO_CHIP_ID at 0x1c00000000,
  observed-window MACB_MID at 0x1c001000fc, and translated-window comparator
  MACB_MID at 0x1f001000fc.
- Added paired no-MMIO/no-Ethernet control construction through the same
  report path while constructing no SYSINFO, observed-window, translated
  comparator, or Ethernet MMIO target.
- Added deterministic rejection coverage for live GEM visibility or broad
  Ethernet/MMIO/DMA/descriptor/interrupt/clock/PHY/packet/networking/socket/
  SSH/Phase 12.2/phase-transition claims.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
RP1 MMIO execution or writes, Ethernet driver implementation, DMA, descriptor
rings, interrupts, clock/reset/PHY/MDIO ownership, packet I/O, networking,
sockets, SSH, Phase 12.2 work, or phase transition.

## Implementation

The accepted candidate report is represented by
Rp1EthernetObservedWindowDiscriminatorReportEvidence and preserves:

- observed-window contract id phase12-rp1-ethernet-observed-window-contract-v1;
- discriminator contract id phase12-rp1-ethernet-observed-window-discriminator-contract-v1;
- source contract id phase12-rp1-ethernet-gem-mid-source-contract-20260609;
- SYSINFO_CHIP_ID positive control at 0x1c00000000 with expected value 0x20001927;
- rp1_eth/MACB_MID source identity and register offset 0x00fc;
- source offset from observed RP1 base 0x001000fc;
- observed RP1 base 0x1c00000000;
- observed-window MACB_MID target 0x1c001000fc;
- translated-window comparator target 0x1f001000fc with comparator-sentinel-only role;
- width, endianness, access, retained source evidence, rejected runtime claims,
  retained risks, and hardware-proof boundary classification.

The paired control carries classification
no-mmio-no-ethernet-rp1-ethernet-observed-window-control and leaves all
SYSINFO, observed-window, translated-comparator, and Ethernet MMIO target
fields unset.

## Findings

- fixed: implemented candidate and paired control report construction for the
  observed-window discriminator contract in src/rp1_ethernet.rs.
- fixed: preserved the SYSINFO positive control, observed-window MACB_MID
  target, translated-window comparator target, source rp1_eth/MACB_MID
  identity, rejected claims, retained risks, and hardware-proof boundary.
- fixed: validators reject missing candidate evidence, control target leakage,
  malformed source-contract evidence, malformed observed positive-control
  evidence, and unsupported Ethernet/MMIO/DMA/descriptor/interrupt/clock/PHY/
  packet/networking/socket/SSH/Phase 12.2/phase-transition claims.
- deferred: Pi 5 proof, live GEM identity, broad Ethernet MMIO readiness,
  Ethernet driver readiness, bridge/window ownership, clock/reset/PHY/MDIO,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition work.
- not-an-issue: no hardwareTestLock was acquired because this task is
  local/static only.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims: live GEM visibility, broad Ethernet MMIO readiness, Ethernet
driver readiness, RP1 MMIO writes, DMA, descriptor rings, interrupts,
clock/reset/PHY/MDIO ownership, packet I/O, networking, sockets, SSH, Phase
12.2, and phase transition.

Retained risks: the observed-window target may still return a sentinel or
fault in a future hardware proof; PCI/RP1 bridge or address-window ownership
remains unaccepted; Ethernet clock/reset and PHY/MDIO ownership remain
unaccepted; future hardware proof still needs capture-chain-v4-style
candidate/control evidence.

## Evidence

- Implementation: src/rp1_ethernet.rs.
- Phase 12 docs: docs/src/project/phase12-networking-ssh.md.
- Task classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-core/evidence-map.json.
- Accepted observed-window contract:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-contract.md.

## Validation

- static inspection of accepted contract and touched source modules: passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet rp1_ethernet: passed.
- jq empty on task-owned JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Acceptance Criteria

- Task record lists findings with disposition: satisfied.
- Implementation exposes candidate and paired control report construction:
  satisfied.
- Candidate report preserves required identity, targets, claims, risks, and
  hardware-proof boundary fields: satisfied.
- Control report preserves the same output path while constructing no MMIO
  targets and carrying explicit no-MMIO/no-Ethernet classification: satisfied.
- Focused tests cover candidate construction, control construction, and
  deterministic rejection cases: satisfied.
- Accepted implementation/evidence is committed before closeout starts:
  satisfied by worker state commit record.

## Next Action

Mechanically promote phase12-rp1-ethernet-observed-window-discriminator-closeout-20260610
on the next worker wake if it remains queued and dependencies are satisfied.
