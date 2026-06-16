# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight Hardware Proof Core

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-core-20260616

Status: accepted

Classification:
bcm54213pe-readonly-preflight-hw-proof-core-local-static

Evidence level: Rust local/static proof-core implementation, focused unit
tests, candidate/control compile-only scenario builds, JSON evidence
validation, Rust fmt, full no_std Rust test suite through QEMU substitute, and
diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, TFTP/serial capture, GPIO/RIO/pad
MMIO write, GPIO32 event clear, PHY reset assertion/deassertion, BMCR write,
Broadcom shadow/MMD/aux access, interrupt surface ownership, PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition
was performed.

## Goal

Implement the non-hardware proof core needed by the later serialized Pi 5
hardware proof for the accepted BCM54213PE read-only preflight frontier.

## Scope Performed

- Added candidate/control boot scenarios for the BCM54213PE read-only
  preflight proof core.
- Wired scenario registration and early dispatch for:
  `rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_candidate` and
  `rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_no_mdio_control`.
- Candidate proof core is limited to PHY1 MII_CTRL1000 0x09 and MII_STAT1000
  0x0a, with MAN read-frame metadata 0x60a60000 and 0x60aa0000.
- Candidate output records target register, MAN frame, raw value, decoded
  fields, completion count, forbidden-claim booleans, and retained-risk
  classification for the later hardware task.
- Paired control output constructs no MDIO target, MAN frame, MACB target,
  GPIO target, RP1 Ethernet target facts, or volatile Ethernet access intent.
- Added local/static proof-core evidence in `src/rp1_ethernet.rs` and a
  focused test that locks the candidate/control shape and fail-closed boundary.

## Findings

- fixed: candidate/control boot scenarios now exist for the later
  dependency-gated Pi 5 proof.
- fixed: candidate scope is exactly PHY1 MII_CTRL1000 and MII_STAT1000; no
  BMCR/BMSR/ANAR/ANLPAR/MACB_NSR retry or broader register vector was added.
- fixed: paired control withholds all candidate-only MDIO/MAN/MACB/GPIO/RP1
  Ethernet target facts.
- fixed: proof-core metadata and runtime strings fail closed for PHY writes,
  selector-write surfaces, GPIO32/reset action, BMCR/autoneg restart,
  Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration,
  link-readiness, packet/networking/SSH/Phase 12.2, and phase transition
  claims.
- deferred: actual hardware proof, boot publication, hardwareTestLock
  acquisition, TFTP/serial evidence, restore proof, and inconclusive-run
  triage remain deferred to
  phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof-20260616.
- not-an-issue: no docs/src update was needed because the accepted frontier
  and selected next boundary did not change.
- removed: no obsolete source, helper, task, or evidence files were removed.

## Evidence

- Implementation: `build.rs`, `src/main.rs`, `src/rp1_ethernet.rs`, and
  `src/target/rpi5.rs`.
- Classification JSON:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-core/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-core/evidence-map.json`.
- Accepted closeout input:
  `tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-closeout.md`.

## Validation

- `cargo fmt --all -- --check`: pass after formatting.
- `cargo -Zjson-target-spec test --quiet rp1_ethernet`: pass.
- Candidate compile-only build:
  `TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_candidate cargo -Zjson-target-spec build --quiet`: pass.
- Control compile-only build:
  `TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_readonly_preflight_no_mdio_control cargo -Zjson-target-spec build --quiet`: pass.
- `cargo -Zjson-target-spec test --quiet`: pass.
- `jq empty` on task-owned JSON evidence: pass.
- `git diff --check`: pass.
- `mdbook build`: not run; no `docs/src` files were touched.
- `git diff --cached --check`: pass before commit.

## Acceptance Check

- Candidate proof core limited to PHY1 MII_CTRL1000 and MII_STAT1000:
  satisfied.
- Candidate records target register, MAN frame, raw value, decoded fields, and
  forbidden-claim state for later hardware proof: satisfied.
- Control constructs no MDIO/MAN/MACB/GPIO/RP1 Ethernet target facts or
  volatile Ethernet access intent: satisfied.
- Validators and runtime metadata fail closed for forbidden write,
  selector-write, reset, ownership, link-readiness, packet/networking/SSH, and
  phase-transition claims: satisfied.
- Hardware proof remains deferred to the queued Pi 5 task: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof-20260616 on the
next worker wake if dependencies remain satisfied. That task must acquire the
hardware lock and retain candidate/control identity, fresh serial cursor, TFTP
delta, known-good control, candidate evidence or precise blocker, restore
proof, and all rejected-claim evidence. Do not start packet I/O, networking,
SSH, Phase 12.2, or a phase transition from this proof core.
