# Phase 12 RP1 Ethernet PHY1 Autoneg Restart Guard Core

Task id: phase12-rp1-ethernet-phy1-autoneg-restart-guard-core-20260614

Status: accepted

Classification: phy1-autoneg-restart-guard-core-accepted

Evidence level: static source inspection, fmt/lint/typecheck/unit test gates,
and task-owned static validator. No Pi 5 hardware run, hardwareTestLock
acquisition, lab mutation, boot publication, GPIO32/RIO/pad write, PHY reset,
MACB write, NCR write, link forcing, packet I/O, DMA/descriptors, interrupts,
networking, sockets, SSH, Phase 12.2, or phase transition was performed.

## Goal

Implement the local/static guard and report surface for the future PHY1 BMCR
autonegotiation-restart discriminator without hardware publication.

## Scope Performed

- Added candidate and paired-control boot scenarios:
  rpi5_rp1_ethernet_phy1_autoneg_restart_candidate and
  rpi5_rp1_ethernet_phy1_autoneg_restart_no_mdio_control.
- Wired build scenario registration and main dispatch only; no archive was
  published and no hardware was run.
- Candidate report path preserves the accepted PHY1/MACB frontier:
  BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000,
  MACB_NSR raw 0x6 / NSR_LINK=false, and retained GPIO32 blockers.
- Candidate preconditions corrected NCR.MPE and BMCR_ISOLATE before permitting
  exactly one BMCR write.
- Candidate allowed write value is exactly pre_bmcr | BMCR_ANENABLE |
  BMCR_ANRESTART; touched fields are limited to BMCR_ANENABLE and
  BMCR_ANRESTART.
- Candidate reports pre BMCR/BMSR/ANAR/ANLPAR, post BMCR, double post BMSR,
  post ANAR/ANLPAR, passive MACB_NSR_LINK, write count, touched fields, and
  rejected-claim booleans.
- Paired control uses the same report surface while constructing no
  MDIO/MAN/MACB target, performing no volatile load/store, and withholding
  candidate-only target/raw/decode/result-valid fields.
- Added a task-owned static validator and JSON static review evidence.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, lab mutation, boot archive
publication, GPIO32/RIO/pad write, PHY reset, MACB write, NCR write, link
forcing, packet I/O, DMA/descriptors, interrupts, networking, sockets, SSH,
Phase 12.2, or phase transition.

## Findings

- fixed: added candidate/control scenario registration and dispatch for the
  future guarded PHY1 autoneg-restart proof.
- fixed: candidate report emits the accepted PHY1 and MACB link-clear frontier,
  exact allowed write value, pre/post readback fields, passive MACB_NSR_LINK,
  touched fields, and rejected-claim booleans.
- fixed: candidate is guarded by corrected NCR.MPE and BMCR_ISOLATE
  preconditions and permits only one BMCR write with ANENABLE/ANRESTART set.
- fixed: control constructs no MDIO/MAN/MACB target, performs no volatile
  load/store, withholds candidate-only fields, and reports the control
  classification.
- deferred: Pi 5 proof, boot publication, lab mutation, runtime serial/TFTP
  evidence, and restore evidence remain the next queued task.
- not-an-issue: no docs/src update was needed because this guard core does not
  change the documented Phase 12 frontier.
- removed: no obsolete implementation or evidence was removed.

## Evidence

- Accepted source contract commit:
  4d1fd92bfa1e0286fc9869a91a6448ae937fa061.
- Static review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-guard-core/static-review.json.
- Validator:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-guard-core/validate-autoneg-restart-guard.sh.
- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-guard-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-guard-core/evidence-map.json.

## Validation

- passed: cargo fmt --all -- --check.
- passed: cargo -Zjson-target-spec test --quiet.
- passed: candidate scenario compile-only build with
  talos_boot_scenario=rpi5_rp1_ethernet_phy1_autoneg_restart_candidate.
- passed: control scenario compile-only build with
  talos_boot_scenario=rpi5_rp1_ethernet_phy1_autoneg_restart_no_mdio_control.
- passed: sh -n on task-owned shell script.
- passed: task-owned static validator generated static-review.json.
- passed: jq empty on task-owned JSON evidence.
- passed: git diff --check.
- not run: mdbook build because docs/src files were not touched.
- passed: git diff --cached --check before commit.

## Next Action

Selected next task:
phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof-20260614.
