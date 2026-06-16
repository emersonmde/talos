# Phase 12 RP1 Ethernet BCM54213PE Boot Transport Sentinel Core

Task id: phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-core-20260616

Status: accepted

Classification:
bcm54213pe-boot-transport-sentinel-core-local-static

Evidence level: Rust local/static sentinel-core implementation, focused unit
tests, candidate/control compile-only boot scenario builds, full no_std Rust
test suite through QEMU substitute, JSON evidence validation, docs build, and
diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, TFTP/serial capture, GPIO32 event
clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access, interrupt
ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, or
phase transition was performed.

## Goal

Define the local/static sentinel core for a later serialized Pi 5 discriminator
that asks whether selected boot identity changes alone can produce fresh TFTP
fetches and serial output, independent of BCM54213PE register-read code.

## Scope Performed

- Added two boot scenarios:
  `rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate` and
  `rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control`.
- Added RPi5 scenario dispatch and serial report functions that differ only by
  boot scenario identity, report kind, optional `TALOS_CAPTURE_NONCE`, payload
  marker, and classification.
- Added `src/rp1_ethernet.rs` sentinel-core evidence and validator surfaces
  proving both scenarios construct no Ethernet, MDIO, MAN, MACB, GPIO32, PHY,
  packet, networking, SSH, or Phase 12.2 target facts.
- Preserved the discriminator question:
  selected-tree power/TFTP/serial transport versus BCM54213PE candidate
  report behavior.

## Findings

- fixed: candidate/control boot scenario names are registered in the build
  scenario table and RPi5 dispatch.
- fixed: runtime scenario output withholds all Ethernet/MDIO/MAN/MACB/GPIO32/PHY
  target facts and records the transport-only boundary.
- fixed: local/static validators reject volatile Ethernet access intent,
  BCM54213PE register claims, link readiness, packet I/O, networking, SSH,
  Phase 12.2, and phase transition.
- deferred: Pi 5 publication, power-cycle, TFTP/serial capture, restore proof,
  and hardwareTestLock handling remain deferred to
  phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof-20260616.
- not-an-issue: this task intentionally does not interpret sentinel output as
  Ethernet readiness or MII_CTRL1000/MII_STAT1000 evidence.
- removed: no obsolete source, helper, task, or evidence files were removed.

## Evidence

- Implementation: `build.rs`, `src/main.rs`, `src/rp1_ethernet.rs`, and
  `src/target/rpi5.rs`.
- Classification JSON:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-core/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-core/evidence-map.json`.
- Accepted input:
  `tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-closeout.md`.

## Validation

- `cargo fmt --all -- --check`: pass.
- `cargo -Zjson-target-spec test --quiet rp1_ethernet_bcm54213pe_boot_transport_sentinel`:
  pass.
- `TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate cargo -Zjson-target-spec build --quiet`:
  pass.
- `TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control cargo -Zjson-target-spec build --quiet`:
  pass.
- `cargo -Zjson-target-spec test --quiet`: pass with QEMU path configured.
- `jq empty` on task-owned JSON evidence: pass.
- `git diff --check`: pass.
- `mdbook build`: pass.
- `git diff --cached --check`: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof-20260616 on
the next worker wake if dependencies remain satisfied and hardwareTestLock
remains unlocked/restored. Do not start BCM54213PE register-read retry, GPIO32
event clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase
12.2, or phase transition from this task.
