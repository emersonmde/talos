# Phase 12 RP1 Ethernet Observed Window Discriminator Closeout

Task: phase12-rp1-ethernet-observed-window-discriminator-closeout-20260610

Status: accepted

Classification: rp1-ethernet-observed-window-discriminator-closeout-accepted

Evidence level: static inspection of accepted observed-window contract/core
task records, task-owned JSON, focused tests, Phase 12 docs, roadmap, and git
history. No Pi 5 hardware run was performed.

## Goal

Close out the accepted local/static observed-window GEM MID discriminator and
decide whether the serialized Pi 5 proof is ready, blocked, or should be
replaced by another discriminator.

## Scope

- Consumed the accepted observed-window contract from commit be5371ef and the
  accepted local/static discriminator core from commit 160c1a90.
- Reconciled candidate/control report fields, rejected claims, retained risks,
  tests, docs, and the next hardware-proof boundary.
- Corrected the focused observed-window unit test to assert the observed-window
  retained-risk set already emitted by the implementation.
- Selected the serialized Pi 5 observed-window discriminator proof as the next
  mechanically objective boundary.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation changes beyond the focused test/doc/evidence
correction, no Pi 5 hardware run, no boot archive publication, no
hardwareTestLock acquisition, no Ethernet driver implementation, no RP1 MMIO
writes, no DMA, descriptor rings, interrupts, clock/reset/PHY/MDIO ownership,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition.

## Reconciled Inputs

- tasks/2026-06-10-phase12-rp1-ethernet-observed-window-contract.md
- tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-core.md
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-contract/classification.json
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-contract/evidence-map.json
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-core/classification.json
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-core/evidence-map.json
- src/rp1_ethernet.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- git history through 160c1a90 Accept RP1 Ethernet observed-window
  discriminator core

## Closeout Decision

The accepted local/static core is internally consistent after the focused test
correction: the candidate report preserves the observed-window contract id,
source contract id, SYSINFO positive-control target/value, observed-window
MACB_MID target, translated-window comparator, rp1_eth/MACB_MID identity,
rejected claims, observed-window retained risks, and hardware-proof boundary
classification. The paired control uses the same report path while constructing
no SYSINFO, observed-window, translated-comparator, or Ethernet MMIO target and
carries the explicit no-MMIO/no-Ethernet classification.

The serialized Pi 5 proof is ready as the next bounded task. It is materially
different from the closed 0x1f GEM MID decode-discriminator retries because it
tests observed-window MACB_MID at 0x1c001000fc while retaining translated
0x1f001000fc only as a comparator/sentinel. The proof remains read-only and
non-destructive: it may perform volatile loads for SYSINFO positive control,
observed-window MACB_MID, and translated-window comparator output, plus a
paired control that constructs no MMIO targets.

The proof must preserve the explicit hardware evidence contract:

- hardwareTestLock owner/task id, acquisition, release, and restored state;
- candidate identity from lab API boot/file fields and selected tree;
- fresh serial cursor before candidate and control runs;
- TFTP delta for candidate and control runs;
- candidate/control joined identity through selected-tree identity, expected
  TFTP fetch bytes, run-unique serial marker freshness, final pre-restore
  identity, restore proof, and task-owned JSON;
- final pre-restore identity before restoring the baseline boot state;
- restore proof after the candidate/control runs;
- classification and evidence-map JSON owned by the proof task.

The proof may classify exactly one of:

- observed-window MACB_MID visible read;
- observed-window sentinel/fault with SYSINFO positive-control retained;
- precise staging or capture blocker.

It must continue to reject Ethernet driver readiness, broad Ethernet MMIO
readiness, RP1 MMIO writes, DMA, descriptor rings, interrupts,
clock/reset/PHY/MDIO ownership, packet I/O, networking, sockets, SSH, Phase
12.2, and phase transition claims. A single successful read-only discriminator
result is not broad Ethernet readiness.

## Findings

- fixed: reconciled the accepted observed-window contract and local/static core
  against source, task JSON, docs, focused tests, and git history.
- fixed: corrected the observed-window focused unit test so it asserts
  RP1_ETHERNET_OBSERVED_WINDOW_RETAINED_RISKS, matching the accepted report
  implementation and task JSON.
- fixed: preserved the candidate report boundary:
  SYSINFO_CHIP_ID at 0x1c00000000 / 0x20001927, observed-window MACB_MID at
  0x1c001000fc, and translated-window comparator MACB_MID at 0x1f001000fc.
- fixed: preserved the paired no-MMIO/no-Ethernet control boundary with no
  SYSINFO, observed-window, translated-comparator, or Ethernet MMIO target.
- fixed: selected the serialized Pi 5 proof and recorded the required hardware
  proof evidence gates before it may be accepted.
- deferred: live GEM visibility, broad Ethernet MMIO readiness, Ethernet
  driver readiness, PCIe/RP1 bridge/window ownership, clock/reset/PHY/MDIO
  ownership, DMA, descriptor rings, interrupts, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future work.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims:

- live GEM visibility;
- broad Ethernet MMIO readiness;
- Ethernet driver readiness;
- RP1 MMIO writes;
- DMA and descriptor rings;
- interrupt, transfer, or device completion;
- clock/reset/PHY/MDIO ownership;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- observed-window MACB_MID may still return a sentinel or fault in the
  serialized proof.
- PCIe/RP1 bridge or address-window ownership remains unaccepted.
- Ethernet clock/reset and PHY/MDIO ownership remain unaccepted.
- Future work still requires post-proof review before any Ethernet driver,
  packet I/O, networking, sockets, SSH, or Phase 12.2 claim.

## Evidence

- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-closeout/evidence-map.json.
- Accepted observed-window contract:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-contract.md.
- Accepted observed-window discriminator core:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-core.md.
- Implementation/tests:
  src/rp1_ethernet.rs.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: reviewed accepted contract/core task records,
  classification/evidence JSON, focused tests, Phase 12 docs, roadmap, and git
  history.
- fmt check: cargo fmt --all -- --check.
- focused tests: cargo -Zjson-target-spec test --quiet rp1_ethernet.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles the accepted local/static observed-window
  discriminator without expanding acceptance to hardware visibility or Ethernet
  readiness: satisfied.
- Checkpoint states whether the serialized Pi 5 proof is ready, blocked, or
  should be replaced by a different discriminator: satisfied; ready.
- If proof is ready, checkpoint carries explicit hardware proof requirements:
  satisfied.
- Accepted closeout is committed before the Pi 5 proof starts: satisfied by the
  commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-observed-window-discriminator-pi5-proof-20260610 on the
next worker wake if hardwareTestLock remains unlocked. The proof must acquire
hardwareTestLock before staging or power cycling, capture candidate/control
identity, fresh serial cursor, TFTP delta, joined identity, final pre-restore
identity, restore proof, and task-owned JSON evidence, then release the lock.
Do not accept broad Ethernet readiness, driver readiness, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition from this
read-only discriminator.
