# Phase 12 RP1 Ethernet GEM MID Decode Discriminator Core

Task: phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610

Status: accepted

Classification: rp1-ethernet-gem-mid-decode-discriminator-local-static-accepted

Evidence level: local/static Rust report construction, focused unit tests, and
static inspection. No Pi 5 hardware run was performed.

## Goal

Implement only the local/static candidate-control report shape selected by the
accepted GEM MID blocker reconciliation closeout.

## Scope

- Consumed the accepted
  phase12-rp1-ethernet-gem-mid-blocker-reconciliation-closeout-20260610
  nextAction.
- Added a decode-discriminator report surface in src/rp1_ethernet.rs.
- Preserved the accepted GEM MID source contract for rp1_eth / MACB_MID
  at translated CPU physical 0x1f001000fc.
- Added the selected observed RP1 SYSINFO_CHIP_ID positive-control target at
  0x1c00000000, expected value 0x20001927, as part of the same-run
  candidate shape.
- Added a paired no-MMIO/no-Ethernet control using the same report contract
  while withholding both observed RP1 and Ethernet MMIO targets.
- Rejected unsupported runtime and hardware claims.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
RP1 MMIO writes, DMA programming, descriptor rings, transfer completion,
interrupt completion, clock/reset writes, PHY reset, packet I/O, Ethernet
driver implementation, networking, sockets, SSH, Phase 12.2 work, or phase
transition.

## Implementation

The accepted local/static candidate is represented by
Rp1EthernetGemMidDecodeDiscriminatorReportEvidence with:

- same_run_required=true;
- changed_from_gem_mid_only_proof=true;
- observed positive-control register SYSINFO_CHIP_ID;
- observed positive-control CPU physical target 0x1c00000000;
- observed positive-control expected value 0x20001927;
- Ethernet target MACB_MID at RP1 bus 0xc0401000fc and CPU physical
  0x1f001000fc;
- expected candidate classifications:
  observed-rp1-positive-control-gem-mid-0x1f-window-sentinel,
  observed-rp1-positive-control-and-gem-mid-visible,
  observed-rp1-positive-control-sentinel, and staging/build-blocker.

The paired control carries classification
no-mmio-no-ethernet-rp1-ethernet-gem-mid-decode-discriminator-control and
leaves both observed positive-control and Ethernet target fields unset.

## Findings

- fixed: implemented the selected local/static same-run observed
  SYSINFO_CHIP_ID positive-control plus translated MACB_MID report shape.
- fixed: added a paired no-MMIO/no-Ethernet control using the same
  discriminator report contract.
- fixed: validators reject missing candidate targets, control target leakage,
  malformed source-contract evidence, malformed observed positive-control
  evidence, and unsupported Ethernet/MMIO/DMA/descriptor/interrupt/clock/PHY/
  packet/networking/socket/SSH/Phase 12.2/phase-transition claims.
- deferred: Pi 5 proof, live GEM identity, broad Ethernet MMIO readiness,
  bridge/window enablement, Ethernet clock/reset ownership, PHY reset/MDIO,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition work.
- not-an-issue: no hardwareTestLock was acquired because this task is
  local/static only.

No findings were removed.

## Rejected Claims And Retained Risks

This task does not accept live GEM visibility, broad Ethernet MMIO readiness,
Ethernet driver readiness, RP1 MMIO/DMA programming, DMA ownership, descriptor
rings, transfer completion, interrupt completion, clock/reset ownership, PHY
reset ownership, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

Retained risks:

- the translated 0x1f RP1 window sentinel source remains unproven;
- PCIe/RP1 bridge or window enablement remains unaccepted;
- Ethernet clock/reset dependency remains unaccepted;
- no live GEM identity, broad Ethernet MMIO readiness, PHY reset ownership, or
  packet I/O has been accepted.

## Evidence

- Implementation and focused unit tests: src/rp1_ethernet.rs.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-core/evidence-map.json.
- Accepted selector closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-blocker-reconciliation-closeout.md.
- Phase 12 project doc: docs/src/project/phase12-networking-ssh.md.

## Validation

- static inspection: reviewed accepted closeout, existing GEM MID contract
  surface, implementation diff, and Phase 12 project doc.
- format check: cargo fmt --all -- --check passed.
- focused unit tests: cargo -Zjson-target-spec test --quiet rp1_ethernet
  passed with the Talos QEMU path configured.
- task-owned JSON: jq empty on evidence-map/classification JSON passed.
- diff check: git diff --check passed.
- documentation build: /home/node/.cargo/bin/mdbook build passed because
  docs/src was touched.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation matches exactly the discriminator shape selected by the
  accepted closeout: satisfied.
- Candidate and paired control reports are locally constructible and reject
  unsupported claims: satisfied.
- Report shape includes a changed discriminator from the accepted GEM MID-only
  proof: satisfied by the same-run observed positive-control plus GEM MID
  target fields.
- Accepted local/static implementation is committed before closeout or hardware
  proof planning proceeds: satisfied by the commit recorded in supervisor
  state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gem-mid-decode-discriminator-closeout-20260610 on the
next worker wake. It must reconcile only this local/static implementation and
decide whether the existing blocked serialized Pi 5 discriminator proof is
ready to unlock later. It must not run hardware, acquire hardwareTestLock,
implement Ethernet behavior, program RP1 MMIO/DMA, create descriptor rings,
claim interrupts/clock/reset/PHY ownership, perform packet I/O, add
networking/sockets/SSH, start Phase 12.2, or claim a phase transition.
