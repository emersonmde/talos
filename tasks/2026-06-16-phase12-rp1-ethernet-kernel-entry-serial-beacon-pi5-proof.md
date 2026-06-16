# Phase 12.1 RP1 Ethernet Kernel Entry Serial Beacon Pi 5 Proof

Task:
phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof-20260616.

## Goal

Run the serialized Pi 5 proof for the earliest-kernel-entry serial beacon
discriminator selected by the core task, before any Ethernet or MDIO behavior.

## Findings

- fixed: HardwareTestLock was acquired before publication and released only
  after restore proof. The selected candidate tree
  68d4c9ae71014c85199391abf7bb54d1bfbe62de17482a3354cb4f7cfea43376
  was restored to baseline
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: The candidate archive retained the run nonce
  kernel-entry-beacon-cand-20260616T053728Z; static archive review found the
  beacon marker and nonce in the selected 47,360-byte kernel_2712.img.
- fixed: The Pi 5 run retained fresh stable TFTP evidence with two served
  da591740/kernel_2712.img fetches at 47,360 bytes, final pre-restore
  selected-tree identity, and post-restore baseline identity.
- fixed: The serial window retained the run-unique earliest-kernel-entry beacon
  marker 89 times after the fresh power-cycle. This proves kernel-entry serial
  visibility for this no-Ethernet/no-MDIO discriminator.
- not-an-issue: Deployed lab GET / returned 404 during this run. The proof
  retained that endpoint result and used /status plus /boot/files for boot
  identity, matching the current deployed lab-controller contract.
- not-an-issue: A same-task known-good control was not required because the
  candidate itself retained selected-tree identity, fresh TFTP, firmware serial,
  final identity, beacon serial, and restore proof; the preceding accepted
  boot-transport sentinel candidate/control already proved the publication and
  capture path.

## Classification

earliest-kernel-entry-beacon-observed.

The accepted result is limited to earliest kernel-entry serial beacon
visibility after a fresh Pi 5 TFTP fetch. It does not accept BCM54213PE
register values, Ethernet readiness, link readiness, packet I/O, networking,
SSH, Phase 12.2, or a phase transition.

## Evidence

- classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/classification.json
- evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/evidence-map.json
- run summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/candidate/run-summary.json
- capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-kernel-entry-serial-beacon-pi5-proof/candidate/capture-invariant-summary.json

## Validation

- static archive/image review: scripts/rpi5-archive-review.sh passed; strings
  review retained the beacon marker and run nonce.
- lab-controller API: /status, /boot/files, TFTP cursor/delta, serial capture,
  final pre-restore identity, snapshot restore, and post-restore identity
  evidence were retained.
- serial hardware boot/output: bounded direct-read capture retained the beacon
  marker and nonce 89 times.
- TFTP evidence: stable same-cursor delta retained two 47,360-byte
  da591740/kernel_2712.img serves.
- JSON evidence validation: jq empty on task-owned JSON evidence passed.
- diff whitespace check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
