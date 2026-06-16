# Phase 12 RP1 Ethernet Kernel Entry Serial Beacon Core

Task id: phase12-rp1-ethernet-kernel-entry-serial-beacon-core-20260616

Status: accepted

Classification:
kernel-entry-serial-beacon-core-local-static

Evidence level: Rust local/static beacon-core implementation, focused unit
tests, selected Pi 5 compile-only build, static image string/symbol/hash
review, full no_std Rust test suite through QEMU substitute, JSON evidence
validation, docs build, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle,
TFTP/serial capture, volatile Ethernet access, BCM54213PE register read retry,
GPIO32 event clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase
12.2, or phase transition was performed.

## Goal

Create the smallest local/static discriminator for whether a freshly fetched
Pi 5 kernel reaches an earliest Rust-entry serial beacon before any
Ethernet/MDIO behavior.

## Scope Performed

- Added boot scenario rpi5_rp1_ethernet_kernel_entry_serial_beacon.
- Added image helper scripts/rpi5-rp1-ethernet-kernel-entry-serial-beacon-image.sh.
- Added RPi5 dispatch that branches before BootInfo parsing and emits
  TALOS: rp1-ethernet-kernel-entry-serial-beacon with an optional
  TALOS_CAPTURE_NONCE.
- Added local/static evidence and validators in src/rp1_ethernet.rs that
  preserve the first-principles discriminator and reject Ethernet/MDIO target
  facts, register values, link readiness, packet I/O, networking, SSH, Phase
  12.2, and phase-transition claims.

## Findings

- fixed: selected scenario and image helper are registered for the later Pi 5
  proof.
- fixed: the scenario dispatch emits before BootInfo parsing, making the
  hardware question kernel-entry serial visibility rather than a later
  Ethernet/MDIO report path.
- fixed: static marker review found the selected nonce, beacon marker, payload,
  explicit withheld BCM54213PE register values, no MDIO/MAN target construction,
  no packet/network/SSH claims, and the accepted classification in the generated
  image.
- fixed: local/static validators reject volatile Ethernet access,
  BCM54213PE register claims, link readiness, packet I/O, networking, SSH,
  Phase 12.2, and phase transition.
- deferred: hardwareTestLock acquisition, candidate publication, fresh serial
  cursor, TFTP delta, known-good control, candidate rerun, and restore proof
  remain deferred to
  phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof-20260616.
- not-an-issue: this task intentionally does not interpret static marker
  presence as Pi 5 execution evidence.
- removed: no obsolete source, helper, task, or evidence files were removed.

## Static Artifact Review

- Boot scenario: rpi5_rp1_ethernet_kernel_entry_serial_beacon.
- Archive helper: scripts/rpi5-rp1-ethernet-kernel-entry-serial-beacon-image.sh.
- Generated image: target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-ethernet-kernel-entry-serial-beacon.img.
- Image SHA-256: b86b9a01f371c4b2c578503e194a6234245f947854a20dad4a9eaba164b3a546.
- Image bytes: 47336.
- ELF SHA-256: 8fb2a1c56c4da993e357179270d09a3d38781c6b210ab8d619aac5a46ad5c3e0.
- readelf -s --wide found run_rp1_ethernet_kernel_entry_serial_beacon.
- /usr/bin/objdump could not disassemble the AArch64 ELF in this container
  (can't disassemble for architecture UNKNOWN), so disassembly was not used as
  evidence in this task.

## Hardware Proof Plan

The queued Pi 5 proof must record candidate identity, selected archive/kernel
hash and size, fresh serial cursor, fresh TFTP cursor/delta, known-good control
when required by triage, candidate power-cycle capture, candidate rerun before
code changes if inconclusive, restore proof, and hardware lock release.

Allowed classifications for that hardware proof are:
earliest-kernel-entry-beacon-observed,
fresh-tftp-no-beacon-kernel-entry-or-serial-boundary,
known-good-control-failed, staging-capture-inconclusive, or restore-failed.

## Evidence

- Implementation: build.rs, src/main.rs, src/rp1_ethernet.rs,
  src/target/rpi5.rs, and
  scripts/rpi5-rp1-ethernet-kernel-entry-serial-beacon-image.sh.
- Classification JSON:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-core/evidence-map.json.
- Artifact review JSON:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-core/static-artifact-review.json.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet rp1_ethernet_kernel_entry_serial_beacon:
  pass.
- TALOS_CAPTURE_NONCE=core-static-20260616 TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_kernel_entry_serial_beacon cargo -Zjson-target-spec build --quiet:
  pass.
- TALOS_CAPTURE_NONCE=core-static-20260616 ./scripts/rpi5-rp1-ethernet-kernel-entry-serial-beacon-image.sh:
  pass.
- strings/readelf static artifact review: pass with objdump limitation noted
  above.
- cargo -Zjson-target-spec test --quiet: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof-20260616 on the next
worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not start BCM54213PE register-read retry, GPIO32 event
clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access, interrupt
ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, or
phase transition from this task.
