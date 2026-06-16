# Phase 12.1 RP1 Ethernet BCM54213PE TX Selected Read Discriminator Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof-20260616

Status: accepted

Classification: tx-selected-register-read-visible

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, capture-chain-v4 replay, and serial freshness guard v1 replay.

## Goal

Run the serialized Pi 5 proof for the accepted TX selected-register read
discriminator and classify exactly whether the TX selector write plus selected
TX register read reaches hardware. This task does not accept TX delay
write/readback, BMCR restart, packet I/O, networking, SSH, Phase 12.2, or a
phase transition.

## Scope Performed

- Acquired hardwareTestLock before boot archive publication and released it
  only after restore proof.
- Built and reviewed run-unique no-MDIO/no-Ethernet control and candidate
  archives from the accepted local/static discriminator core.
- Ran the control first, restored baseline, then ran the candidate using the
  same snapshot restore boundary.
- Retained selected-tree identity, same-power-cycle TFTP byte evidence, serial
  nonce freshness, final pre-restore identity, restore proof, and task-owned
  classification/evidence JSON.

## Findings

- fixed: control archive retained SHA-256
  b1e3b206cbbd9c830c230fcdf4d1d17795d3487c245da9691fa02510be1f5e7b, kernel
  SHA-256 6e536fe70ccd0ef8bbca5400924b82e48851326ad4fd5b32075bcee797969727,
  a 49,624-byte kernel_2712.img, selected tree
  7b3eaf35548fb7bd406ab50ff57c874db2ac9b9f921f3d16ddb413364d9a34fc, two
  matching da591740/kernel_2712.img TFTP serves, 45 fresh serial marker/nonce
  occurrences, and restore to baseline.
- fixed: control proved the no-MDIO/no-Ethernet report shape with
  classification
  no-mdio-no-ethernet-bcm54213pe-tx-selected-read-discriminator-control and no
  target facts constructed.
- fixed: candidate archive retained SHA-256
  4d8e836b497c7c4e992a06febd4495bb4f189b6d31d735b817694cb2af7ece20, kernel
  SHA-256 b35bb5099318b2575ac53c61120f0f91877f76d5ca7196d13af834b067a8239a,
  a 50,384-byte kernel_2712.img, selected tree
  e6da5228a04e32f7475e87238ec99a0ca6c9d234c6e3d50caefd55ec770ba4a8, two
  matching da591740/kernel_2712.img TFTP serves, 42 fresh serial marker/nonce
  occurrences, and restore to baseline.
- fixed: candidate reached the exact TX selected-register read boundary:
  NCR before/after 0x10, TX selector write value 0x0c00, selector write count
  0x1, selected TX read raw 0x0e00, selected-read-completed=true, and
  classification tx-selected-register-read-visible.
- fixed: candidate retained rx-delay-write-count=0x0, tx-delay-write-count=0x0,
  bmcr-write-count=0x0, claims-rx-delay-write=false,
  claims-tx-delay-write=false, and claims-bmcr-restart=false.
- fixed: capture-chain-v4 and serial freshness guard v1 accepted both retained
  bundles with no rejection reasons.
- rejected: this proof does not accept RX delay write/readback from this
  discriminator, TX delay write/readback, BMCR restart, convergence polling,
  link readiness, packet transport, Ethernet driver readiness, networking,
  sockets, SSH, Phase 12.2, or a phase transition.
- removed: generated boot archives remain untracked target/evidence artifacts;
  retained task evidence records hashes, byte counts, and lab captures.
- not-an-issue: identity/TFTP/serial/restore evidence was decisive, so the
  accepted TX selected-register read visibility is not a staging or
  capture-chain ambiguity.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/capture-summary.json.
- Static archive review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/archive-review/.
- Control run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/control-run/.
- Candidate run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/candidate-run/.

## Validation

- static archive/image review: passed for control and candidate.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and final GET /boot/files
  evidence retained selected-tree identity, final identity, and restore proof.
- same-power-cycle TFTP evidence: control retained two matching 49,624-byte
  da591740/kernel_2712.img serves; candidate retained two matching
  50,384-byte serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained tx-selected-register-read-visible with raw value 0x0e00.
- capture-chain-v4 replay: both bundles passed.
- serial freshness guard v1 replay: both bundles passed.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass with the pre-existing large search-index warning.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware evidence is serialized under hardwareTestLock and includes post-run
  baseline restore proof: satisfied.
- Candidate/control identity, TFTP delta, final identity, and serial freshness
  are decisive: satisfied.
- Terminal classification identifies the exact TX selected-register read stage
  reached: tx-selected-register-read-visible, with selector write complete and
  selected read raw 0x0e00.
- Rejected scope claims remain explicit in task record, evidence, and state:
  satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-closeout-20260616
on the next worker wake if dependencies remain satisfied and hardwareTestLock
remains unlocked/restored. Do not start TX delay write/readback, BMCR restart,
packet I/O, networking, SSH, Phase 12.2, or a phase transition from this proof.
