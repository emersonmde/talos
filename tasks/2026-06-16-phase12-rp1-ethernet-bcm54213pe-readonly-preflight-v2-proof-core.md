# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight V2 Proof Core

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-proof-core-20260616

Status: accepted

Classification:
bcm54213pe-readonly-preflight-v2-proof-core-local-static

Evidence level: Rust local/static proof-core implementation, focused unit
tests, candidate/control compile-only scenario builds, JSON evidence
validation, Rust fmt, full no_std Rust test suite through QEMU substitute, docs
build, and diff checks. No Pi 5 hardware run, boot archive publication, lab
mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
restore, GPIO32 event clear/reset recovery, BMCR/autoneg write, Broadcom
shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Implement the smallest distinct BCM54213PE read-only preflight v2 proof core:
keep the accepted PHY1 MII_CTRL1000/MII_STAT1000 target set, add the accepted
cursor-nonce serial freshness v1 evidence shape, and split candidate serial
output into a pre-MDIO entry marker and a separate post-read values marker.

## Scope Performed

- Added v2 candidate/control boot scenarios:
  rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_candidate and
  rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_no_mdio_control.
- Added local/static v2 proof-core evidence and validation in src/rp1_ethernet.rs.
- Wired build registration and early dispatch in build.rs and src/main.rs.
- Added RPi 5 target code for a candidate that emits a run-unique pre-MDIO
  marker before bounded Clause 22 MAN reads and emits a post-read marker only
  after both selected raw values complete.
- Added a paired no-MDIO/no-Ethernet control that emits the same capture nonce
  freshness shape without constructing MDIO, MAN, MACB, GPIO32/PHY, or RP1
  Ethernet target facts.
- Added focused tests for v2 proof-core marker shape and fail-closed forbidden
  claim rejection.

## Findings

- fixed: Candidate/control v2 proof-core surfaces now encode exactly PHY1
  MII_CTRL1000 0x09 and MII_STAT1000 0x0a as the accepted target set.
- fixed: Candidate serial output has separate pre-MDIO and post-read markers so
  a later Pi 5 proof can distinguish pre-MDIO entry visibility from completed
  raw/decoded register values.
- fixed: Control serial output retains the cursor-nonce freshness shape but
  withholds all MDIO/MAN/MACB/GPIO32/PHY/RP1 Ethernet target facts.
- fixed: Static validators reject target drift, selector-write surfaces,
  GPIO32 reset action, BMCR/autoneg restart, Broadcom shadow/MMD/aux access,
  interrupt ownership, broad PHY/MAC configuration, link-readiness,
  packet/networking/SSH/Phase 12.2, and phase-transition claims.
- selected: the queued serialized Pi 5 v2 proof is the next mechanically
  unblocked task after this core is committed, if hardware lock and repository
  cleanliness remain satisfied.
- rejected: no BCM54213PE register values, link readiness, GPIO32/PHY reset
  ownership, BMCR/autoneg behavior, packet I/O, networking, SSH, Phase 12.2,
  or phase transition are accepted by this local/static core.
- removed: no source, helper, docs, task, or evidence files were removed.
- not-an-issue: no boot archive publication is needed for this local/static
  core; compile-only scenario builds cover the newly registered surfaces.

## Evidence

- Implementation: build.rs, src/main.rs, src/rp1_ethernet.rs, and
  src/target/rpi5.rs.
- Classification JSON:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-proof-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-proof-core/evidence-map.json.
- Accepted serial freshness closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-serial-freshness-closeout.md.
- Prior BCM54213PE hardware-proof closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-closeout.md.

## Acceptance Check

- Findings list includes dispositions: satisfied.
- Candidate/control v2 proof-core surfaces encode exactly the accepted target
  set and serial-freshness v1 fields: satisfied.
- Candidate has pre-MDIO and post-read marker/nonce boundaries: satisfied.
- Control constructs no MDIO/MAN/MACB/GPIO/RP1 Ethernet target facts and
  performs no volatile Ethernet access intent: satisfied.
- Validators fail closed for forbidden writes, selector surfaces,
  GPIO32/reset, BMCR/autoneg, Broadcom shadow/MMD/aux, interrupt ownership,
  broad PHY/MAC, link, packet/networking/SSH/Phase 12.2, and phase transition:
  satisfied.
- Queued v2 Pi 5 proof is selected only after this code/evidence/docs work is
  committed: satisfied once state records this commit.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet rp1_ethernet_bcm54213pe_readonly_preflight_v2:
  pass.
- Candidate compile-only build:
  TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_candidate cargo -Zjson-target-spec build --quiet:
  pass.
- Control compile-only build:
  TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_v2_no_mdio_control cargo -Zjson-target-spec build --quiet:
  pass.
- cargo -Zjson-target-spec test --quiet: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof-20260616 on
the next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisor intervention remains inactive, and the worktree
is clean. Do not start GPIO32 event clear/reset recovery, BMCR/autoneg writes,
Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration,
packet I/O, networking, SSH, Phase 12.2, or phase transition from this core.
