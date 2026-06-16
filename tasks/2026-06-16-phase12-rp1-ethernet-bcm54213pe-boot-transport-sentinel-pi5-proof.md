# Phase 12.1 BCM54213PE Boot-Transport Sentinel Pi 5 Proof

Task:
phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof-20260616.

## Goal

Run the accepted no-Ethernet/no-MDIO BCM54213PE boot-transport sentinel on the
Pi 5 to discriminate whether selected boot identity alone can produce fresh
TFTP fetches and serial output after power-cycle, independent of the earlier
BCM54213PE register-read proof.

## Scope

- Use the accepted local/static sentinel scenarios:
  `rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_candidate` and
  `rpi5_rp1_ethernet_bcm54213pe_boot_transport_sentinel_control`.
- Retain static archive review, lab identity, TFTP delta, serial window,
  final pre-restore identity, restore proof, and hardware lock evidence.
- Reject BCM54213PE register values, link readiness, GPIO32/PHY reset
  ownership, BMCR writes, Broadcom shadow/MMD/aux access, interrupt ownership,
  broad PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, and
  phase transition.

## Findings

- fixed: The accepted sentinel core introduced a compile break by using
  `boot_scenarios` without adding the field to
  `Rp1EthernetBcm54213peBootTransportSentinelCoreEvidence`, and by adding the
  same field to the read-only preflight hardware-proof evidence without a
  constant/initializer. This task fixed the local/static evidence shape before
  running hardware.
- fixed: Candidate and control archives were rebuilt with run-unique
  `TALOS_CAPTURE_NONCE` values and passed static archive review before
  publication.
- fixed: HardwareTestLock was acquired before archive publication and released
  only after candidate/control restore proof returned the lab to baseline tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- fixed: Candidate and control both produced fresh matching TFTP fetches of
  `da591740/kernel_2712.img` at 86,744 bytes, with selected-tree identity held
  through final pre-restore status.
- deferred: Neither sentinel image emitted its run nonce marker in the bounded
  serial window. Serial did capture fresh Raspberry Pi firmware NETWORK output,
  so the result narrows the earlier blocker to fetched-kernel execution or
  sentinel serial emission rather than TFTP publication/fetch.
- not-an-issue: `/status` and `/boot/files` were both retained in the evidence
  bundle; selected-tree identity is taken from the lab boot fields, with
  `effective_kernel=kernel_2712.img`.

## Evidence

- static archive review:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/static-archive-review.json`
- classification:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/classification.json`
- evidence map:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/evidence-map.json`
- candidate retained run summary:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/candidate/run-summary.json`
- control retained run summary:
  `tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-boot-transport-sentinel-pi5-proof/control/run-summary.json`

## Classification

`boot-transport-selected-tree-fresh-tftp-no-kernel-sentinel-serial`.

This accepts only the boot-transport discriminator result: selected
candidate/control boot identities can publish, remain selected, and fetch from
TFTP after power-cycle without Ethernet/MDIO target construction. It does not
accept BCM54213PE register values, link readiness, PHY/MAC behavior, packet
I/O, networking, SSH, Phase 12.2, or phase transition.

## Validation

- static archive/image review: candidate and control passed
  `scripts/rpi5-archive-review.sh`; both kernels were 86,744 bytes.
- lab-controller API: candidate and control `/status`/`/boot/files` retained
  selected-tree identity, final pre-restore identity, and post-restore baseline
  identity.
- serial hardware boot/output: candidate/control serial windows captured fresh
  firmware NETWORK output but no sentinel nonce marker.
- TFTP evidence: candidate/control stable same-cursor deltas each retained two
  86,744-byte `da591740/kernel_2712.img` serves.
- restore proof: candidate/control snapshots restored baseline tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests / QEMU substitute: `cargo -Zjson-target-spec test --quiet`
  passed, 515 tests.
- JSON evidence validation: `jq empty` on task-owned JSON evidence passed.
- diff whitespace check: `git diff --check` passed.
- docs build: `/home/node/.cargo/bin/mdbook build` passed.
