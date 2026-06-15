# Phase 12 RP1 Ethernet Post-Physical Link Status MAN Read Accounting Core

Task id: phase12-rp1-ethernet-post-physical-link-status-man-read-accounting-core-20260615

Status: accepted

Classification: post-physical-link-status-man-read-accounting-contract-accepted

Evidence level: static/source/task evidence inspection, local report-surface
implementation, static review-script validation, unit tests, and task-owned JSON
evidence. No Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, lab mutation, PHY configuration write, BMCR write, GPIO32/PHY reset
action, packet I/O, networking, SSH, Phase 12.2 work, or phase transition was
performed.

## Goal

Accept or reject the v2 boundary for post-physical link-status sampling where
corrected-target Clause 22 PHY1 reads are initiated by bounded MACB MAN
read-command stores, while preserving all forbidden configuration and packet I/O
boundaries.

## Scope Performed

- Reconciled the accepted v1 source contract, Pi 5 proof/blocker, and closeout.
- Accepted the v2 contract boundary for five exact corrected-target PHY1 Clause
  22 read commands: BMCR, BMSR first sample, BMSR second sample, ANAR, and
  ANLPAR.
- Updated the runtime report surface to stop implying a zero-MAN-write contract:
  it now records man-read-command-write-count separately from PHY configuration
  writes, BMCR writes, MAC configuration writes, and packet I/O.
- Updated candidate/control archive review scripts so static image validation
  requires the v2 contract id and the new accounting fields.
- Preserved the paired control as no MDIO/MAN/MACB target construction and no
  volatile Ethernet access.

## Non-Goals

No hardware action, boot publication, lab mutation, hardwareTestLock acquisition,
PHY configuration write, BMCR write, GPIO32/PHY reset action, link forcing,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2 work, or phase
transition.

## Source-Grounded V2 Boundary

Accepted contract id:
phase12-rp1-ethernet-post-physical-link-status-contract-v2.

The candidate may issue only these MACB MAN read-command stores:

~~~text
BMCR:        Clause 22 PHY1 register 0x00, MAN frame 0x60820000
BMSR first:  Clause 22 PHY1 register 0x01, MAN frame 0x60860000
BMSR second: Clause 22 PHY1 register 0x01, MAN frame 0x60860000
ANAR:        Clause 22 PHY1 register 0x04, MAN frame 0x60920000
ANLPAR:      Clause 22 PHY1 register 0x05, MAN frame 0x60960000
~~~

Each selected PHY1 read remains gated by corrected NCR.MPE, NSR.IDLE before and
after the MAN read-command store, and DATA[15:0] extraction from MAN. The
candidate may also perform the passive MACB_NSR load at 0x1c00100008 and decode
bit 0 as NSR_LINK.

This boundary does not accept PHY configuration writes, BMCR writes, MAC
configuration writes, GPIO32/PHY reset action, link forcing, DMA/descriptors,
packet I/O, networking, SSH, Phase 12.2, or a phase transition.

## Report Surface

The candidate report now includes:

- post-physical-link-status-contract-id=phase12-rp1-ethernet-post-physical-link-status-contract-v2.
- task-id=phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof-20260615.
- source-contract-task-id=phase12-rp1-ethernet-post-physical-link-status-man-read-accounting-core-20260615.
- selected-phy1-man-read-commands with the five exact registers and MAN frames.
- man-read-command-write-count=5.
- phy-configuration-write-count=0, bmcr-write-count=0, and
  macb-configuration-write-count=0.
- bounded-runtime-hardware-claims=clause22-phy1-man-read-command-writes,passive-macb-nsr-read.
- claims-runtime-man-read-command-writes=true.

The paired control keeps the same surface but reports withheld selected MAN
commands, man-read-command-write-count=0, no constructed MDIO/MAN/MACB targets,
and no volatile Ethernet access.

## Findings

- fixed: v2 report fields distinguish bounded MAN read-command stores from PHY
  configuration writes, BMCR writes, MAC configuration writes, DMA, and packet
  I/O.
- fixed: candidate report no longer claims the old zero-MAN-write contract while
  performing Clause 22 read-command stores.
- fixed: static review scripts require the v2 contract id, v2 proof task id,
  five exact MAN read-command frames, and separated write accounting.
- fixed: paired control remains no-MDIO/no-MAN/no-MACB target construction with
  zero MAN read-command writes.
- deferred: Pi 5 runtime proof remains the queued follow-up and was not run in
  this task.
- removed: the old v1 zero-write-looking static requirements were removed from
  the candidate/control review scripts and are now forbidden strings.
- not-an-issue: MAN read-command stores are still MACB register writes, but
  under this v2 contract they are bounded read-transaction initiators, not PHY
  or MAC configuration writes.

## Rejected Claims And Retained Risks

Rejected claims:

- PHY reset ownership;
- GPIO32 ownership or action;
- PHY configuration writes;
- BMCR writes;
- MAC configuration writes;
- autonegotiation restart;
- link forcing;
- DMA/descriptors;
- packet I/O;
- interrupts;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- A future proof can classify only sampled PHY/MAC status at the selected
  instant.
- A not-ready v2 result still requires a source-grounded follow-up before PHY
  reset, strap, power, or configuration work.
- Packet I/O and network stack work remain blocked until later explicit tasks
  accept their prerequisites.

## Evidence

- Prior v1 source contract:
  tasks/2026-06-14-phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract.md.
- Prior Pi 5 proof/blocker:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof.md.
- Prior closeout:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-closeout.md.
- Runtime report implementation:
  src/target/rpi5.rs.
- Static report validators:
  scripts/rpi5-rp1-ethernet-post-physical-link-status-candidate-review.sh and
  scripts/rpi5-rp1-ethernet-post-physical-link-status-control-review.sh.
- Classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-man-read-accounting-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-man-read-accounting-core/evidence-map.json.

## Validation

- static/source/task evidence inspection: inspected v1 source contract, Pi 5
  proof/blocker, closeout, runtime report implementation, and static review
  scripts.
- shell syntax validation: sh -n on both post-physical link-status review
  scripts.
- Rust formatting: cargo fmt --all -- --check.
- Rust unit tests: cargo -Zjson-target-spec test --quiet.
- static report/validator check: built candidate/control archives and ran both
  post-physical link-status archive review scripts against the new v2 surface.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings use disposition labels: satisfied.
- V2 boundary names exact allowed Clause 22 PHY1 read transactions for BMCR,
  BMSR first, BMSR second, ANAR, and ANLPAR: satisfied.
- Report and validators distinguish bounded MAN read-command transaction writes
  from forbidden configuration writes, GPIO32 action, DMA, and packet I/O:
  satisfied.
- Candidate report no longer claims a zero-MAN-write contract while source
  inspection shows MAN read-command stores: satisfied.
- Paired control constructs no MDIO/MAN/MACB targets and performs no volatile
  Ethernet access: satisfied.
- NextAction selects the queued v2 Pi 5 proof because the contract/accounting
  boundary is accepted: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof-20260615 on the next
worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not accept the retained v1 runtime phy-not-ready result as
a frontier from this task.
