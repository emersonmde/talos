# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight Report Core

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core-20260616

Status: accepted

Classification:
bcm54213pe-readonly-preflight-report-core-candidate-local-static

Evidence level: Rust local/static report-core implementation, focused report
surface unit tests, JSON evidence validation, Rust fmt, full no_std Rust test
suite through QEMU substitute, and diff checks. No Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, power-cycle,
TFTP/serial capture, GPIO/RIO/pad/MMIO write, GPIO32 event clear, PHY reset
assertion/deassertion, BMCR write, Broadcom shadow/MMD/aux access, interrupt
surface access, PHY/MAC configuration, packet I/O, networking, SSH, Phase
12.2, or phase transition was performed.

## Goal

Implement the local/static report-core boundary selected by the accepted
BCM54213PE read-only preflight source contract for exactly two PHY1 Clause 22
targets: MII_CTRL1000 0x09 and MII_STAT1000 0x0a.

## Scope Performed

- Added a BCM54213PE read-only preflight report contract in
  \`src/rp1_ethernet.rs\`.
- Encoded candidate report metadata for PHY1 MII_CTRL1000 and MII_STAT1000,
  including the future Clause 22 MAN read-frame constants 0x60a60000 and
  0x60aa0000 as local/static metadata only.
- Encoded paired no-MDIO/no-Ethernet control evidence that carries no MDIO,
  MAN, MACB, GPIO, or RP1 Ethernet target facts.
- Added validator errors for forbidden hardware access, volatile access,
  GPIO32 action, BMCR/PHY writes, Broadcom shadow/MMD/AUX access, interrupt
  surfaces, PHY/MAC configuration, link-readiness, packet/networking/socket/SSH
  claims, Phase 12.2, and phase transition.
- Added decoders for selected MII_CTRL1000 and MII_STAT1000 fields sourced
  from retained Linux MII definitions.
- Added focused tests for candidate formatting, paired control formatting,
  rejected shape/overclaim paths, and selected register decode behavior.

## Findings

- fixed: candidate/control report surfaces now encode exactly the accepted
  selected target list and no broader PHY/MAC behavior.
- fixed: validators reject forbidden hardware, volatile access, write/restore,
  packet/networking/SSH/Phase 12.2, and phase-transition claims.
- selected: the next explicit queue item is the BCM54213PE read-only preflight
  closeout task; this report core does not authorize hardware proof by itself.
- rejected: same-shaped BMCR/BMSR/ANAR/ANLPAR/MACB_NSR retry, BMCR autoneg
  restart, GPIO32 reset recovery, Broadcom selector access, interrupt surfaces,
  broad PHY/MAC configuration, link readiness, packet I/O, networking, SSH,
  Phase 12.2, and phase transition remain rejected.
- deferred: a later hardware-proof contract remains deferred to closeout or
  supervisor planning and would need full candidate/control identity,
  TFTP/serial, hardware lock, restore, and inconclusive-run triage
  preconditions.
- blocked: GPIO32/ETH_RST_N reset recovery remains blocked by the accepted
  persistent or firmware-owned GPIO32 event-state frontier.
- removed: no source, helper, task, or evidence files were removed.
- not-an-issue: no docs/src update was needed because the accepted roadmap
  frontier did not change beyond selecting the already queued closeout.

## Evidence

- Implementation: \`src/rp1_ethernet.rs\`.
- Classification JSON:
  \`tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core/classification.json\`.
- Evidence map:
  \`tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core/evidence-map.json\`.
- Accepted source contract input:
  \`tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract.md\`.

## Validation

- \`cargo fmt --all -- --check\`: pass after formatting.
- \`cargo -Zjson-target-spec test --quiet\`: pass with QEMU path configured;
  512 no_std tests passed, including the four new
  \`rp1_ethernet_bcm54213pe_readonly_preflight_*\` and
  \`rp1_ethernet_phy1_gigabit_preflight_decodes_selected_registers\` tests.
- \`jq empty\` on task-owned JSON evidence: pass.
- \`git diff --check\`: pass.
- \`mdbook build\`: not run; no \`docs/src\` files were touched.
- \`git diff --cached --check\`: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-closeout-20260616 on the
next worker wake if dependencies remain satisfied. Do not start hardware,
GPIO32 event clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase
12.2, or phase transition from this task.
