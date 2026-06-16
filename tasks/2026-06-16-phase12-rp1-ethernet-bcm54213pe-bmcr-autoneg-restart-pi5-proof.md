# Phase 12.1 RP1 Ethernet BCM54213PE BMCR Autoneg Restart Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof-20260616

Status: accepted

Classification: bcm54213pe-bmcr-autoneg-restart-post-status-sampled

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, serial freshness guard v1 replay, JSON validation, and diff
checks.

## Goal

Run one serialized Pi 5 candidate/control proof for the accepted BCM54213PE
BMCR autoneg restart core and decide only whether the bounded write/status
sample path is visible under the selected-tree/TFTP/serial freshness contract.

## Scope Performed

- Acquired the hardwareTestLock before archive publication and released it only
  after restore proof.
- Added task-specific candidate/control boot-tree, archive, and archive-review
  wrappers for the BMCR autoneg restart proof.
- Built run-unique control and candidate archives from the accepted proof-core
  scenarios.
- Published and ran the no-MDIO/no-Ethernet control first, restored baseline,
  then published and ran the candidate.
- Retained archive hashes, kernel hashes, selected-tree identity, TFTP deltas,
  serial cursor/nonce freshness, final pre-restore identity, restore proof, and
  serial freshness guard output for both runs.
- Performed no candidate rerun because the candidate selected-tree, TFTP,
  serial freshness, final identity, and restore evidence were decisive.

## Findings

- fixed: control archive retained SHA-256
  4e4a95a71ccd03362849fd2cf749538b05fcdec38faca5c994517fa0e3baf127, kernel
  SHA-256 c08dba25d9a6de3126589bb9220a90410cc09655ef9c2d1f85e32ca3d25fde3e,
  a 50,192-byte kernel_2712.img, selected tree
  967662d73a6b078450170d4bca3d31446e29afbe4a5aabcf46c3c1c0ea6b809b, two
  matching da591740/kernel_2712.img TFTP serves, 20 fresh serial nonce
  occurrences, and restore to baseline.
- fixed: control proved the no-MDIO/no-Ethernet shape with classification
  no-mdio-no-ethernet-bcm54213pe-bmcr-autoneg-restart-control.
- fixed: candidate archive retained SHA-256
  cb83d8e42b5a55c704a8d8ef177c8371428b0f840dba69a37353c75fcb303250, kernel
  SHA-256 0fdda0d6f95229340d69314628cd9940f156070953f2981c06502864cf1ae9de,
  a 53,112-byte kernel_2712.img, selected tree
  28d2f01e69c584a494bdfc6c5dd3ab82cc9a2ce175abe5a5ac62f6c8709bd15f, two
  matching da591740/kernel_2712.img TFTP serves, 17 fresh serial nonce
  occurrences, and restore to baseline.
- fixed: candidate reached the bounded BMCR write/status path and reported one
  PHY1 BMCR write frame 0x50821200 for value 0x1200.
- fixed: candidate post-status sampled BMCR 0x1000, BMSR first/second 0x7949,
  ANAR 0x1e1, ANLPAR 0x0, MII_CTRL1000 0x200, MII_STAT1000 0x0, and passive
  MACB_NSR 0x6 with MACB_NSR_LINK false.
- fixed: serial freshness guard v1 accepted both bundles with
  serial-freshness-guard-v1-ready and no rejection reasons.
- rejected: the candidate does not prove link readiness; post-BMSR link status
  and autoneg complete remained false, passive MACB_NSR_LINK remained false,
  and link-ready acceptance is explicitly rejected.
- rejected: GPIO32 reset ownership, Broadcom selector/config writes, interrupt
  ownership, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain unaccepted.
- not-an-issue: no candidate rerun or inconclusive-run triage was needed
  because the first candidate run satisfied selected-tree, TFTP, serial
  freshness, final identity, and restore invariants.
- removed: generated boot archives remain target/tmp artifacts; retained task
  evidence records hashes, byte counts, and lab captures.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/capture-summary.json.
- Static archive review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/archive-review/.
- Control run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/control-run/.
- Candidate run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/candidate-run/.

## Validation

- static archive/image review: scripts/rpi5-archive-review.sh plus the
  task-specific control/candidate review wrappers passed.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and GET / evidence
  retained selected-tree identity, final identity, and restore proof.
- same-power-cycle TFTP evidence: control retained two matching 50,192-byte
  da591740/kernel_2712.img serves; candidate retained two matching 53,112-byte
  serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained the BMCR autoneg restart post-status marker and bounded
  raw/decoded values.
- serial freshness guard v1 replay: both bundles passed.
- sh -n on touched shell scripts: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not required because docs/src files were not touched.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware lock acquisition/release and restore evidence are recorded:
  satisfied.
- Paired control proves the no-MDIO/no-Ethernet output shape: satisfied.
- Candidate selected-tree, same-power-cycle TFTP, serial freshness, final
  identity, and restore evidence agree: satisfied.
- Candidate proves the selected BMCR/autoneg restart/status boundary:
  satisfied only for the bounded post-status-sampled classification.
- Inconclusive-run triage was not needed and no candidate rerun was performed.
- Rejected claims remain explicit: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-closeout-20260616 on the
next worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not start packet I/O, networking, SSH, Phase 12.2, or a
phase transition from this proof.
