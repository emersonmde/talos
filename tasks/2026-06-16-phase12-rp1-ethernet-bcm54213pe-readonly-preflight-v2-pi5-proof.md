# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight v2 Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof-20260616

Status: accepted

Classification: bcm54213pe-readonly-preflight-v2-post-read-values-visible

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, serial freshness guard v1 replay, JSON validation, docs build, and
diff checks.

## Goal

Run one serialized Pi 5 candidate/control proof for the accepted BCM54213PE
read-only preflight v2 core, using cursor-nonce serial freshness plus pre-MDIO
and post-read markers to decide whether the candidate reaches MDIO reads or
fails earlier.

## Scope Performed

- Acquired the hardwareTestLock before archive publication and released it only
  after restore proof.
- Built run-unique control and candidate archives from the accepted v2 proof
  core scenarios.
- Published and ran the no-MDIO/no-Ethernet control first, then restored the
  baseline before publishing and running the candidate.
- Retained archive hashes, kernel hashes, selected-tree identity, TFTP deltas,
  serial cursor freshness, final pre-restore identity, restore proof, and guard
  replay output for both runs.
- Performed no candidate rerun because candidate selected-tree, TFTP, serial
  freshness, final identity, and restore evidence were decisive.

## Findings

- fixed: the control archive retained SHA-256
  eea0b2d5c11ed28d6128053fb71ebe8f08504774d5c41f8e7711d3b0d5f1de21, kernel
  SHA-256 01110ff2eff264ef0df22b4a76d002808911f4a7e0a9efe9b5e9d3dc9e5139a2,
  a 50,856-byte kernel_2712.img, selected tree
  035d4affb2ed54ffe8d02a7f6cd2879ba404775ec49379062dd6f694f9e40abb, two
  matching da591740/kernel_2712.img TFTP serves, 17 fresh serial marker/nonce
  occurrences, and restore to baseline.
- fixed: the candidate archive retained SHA-256
  cca28da60fad02fd917cf531d08756ed50a2d8a07c50e5302196b42c65a60c73, kernel
  SHA-256 58ecb41a1290b6cf938b77531f1e6ef65a88e46d7f1ed3748b2fa11f6e008474,
  a 52,056-byte kernel_2712.img, selected tree
  012e2aeae1fb00699b3ae9ead98433f68b8e093d96f4105332dfe6146b3b6ab3, two
  matching da591740/kernel_2712.img TFTP serves, 17 fresh serial marker/nonce
  occurrences, and restore to baseline.
- fixed: the candidate reached the post-read marker and reported fresh read-only
  PHY1 values: MII_CTRL1000 0x09 raw 0x0200 valid, MII_STAT1000 0x0a raw 0x0000
  valid, completed-register-count 2.
- fixed: MII_CTRL1000 decoded advertise-1000-full=true and
  advertise-1000-half=false; MII_STAT1000 decoded local/remote receiver OK
  false and link-partner 1000 full/half false.
- fixed: serial freshness guard v1 accepted both bundles with
  serial-freshness-guard-v1-ready and no rejection reasons.
- rejected: the candidate does not prove link readiness, GPIO32/PHY reset
  ownership, BMCR/autoneg behavior, Broadcom shadow/MMD/aux access, interrupt
  ownership, broad PHY/MAC configuration, packet I/O, networking, sockets, SSH,
  Phase 12.2, or a phase transition.
- removed: generated boot archives remain target/tmp artifacts; retained task
  evidence records hashes, kernel byte counts, and lab captures.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/capture-summary.json.
- Control run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/control-run/.
- Candidate run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/candidate-run/.
- Static archive review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/archive-review/.

## Validation

- static archive/image review: scripts/rpi5-archive-review.sh passed for
  control and candidate archives.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and GET /status evidence
  retained selected-tree identity, final identity, and restore proof.
- same-power-cycle TFTP evidence: control retained two matching 50,856-byte
  da591740/kernel_2712.img serves; candidate retained two matching 52,056-byte
  serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained the post-read marker and raw/decoded MII_CTRL1000 and
  MII_STAT1000 values.
- serial freshness guard v1 replay: both bundles passed.
- JSON validation: jq empty on task-owned JSON evidence passed.
- diff whitespace check: git diff --check passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance Check

- Hardware lock acquisition/release and restore evidence are recorded:
  satisfied.
- Control run proves the no-MDIO/no-Ethernet serial-freshness/control shape:
  satisfied.
- Candidate selected-tree, same-power-cycle TFTP, serial freshness guard, final
  identity, and restore evidence agree: satisfied.
- Candidate distinguishes pre-MDIO entry from post-read values: satisfied by the
  post-read marker and fresh raw/decoded register values.
- Inconclusive candidate rerun triage was not needed; no candidate rerun was
  performed.
- Rejected claims remain explicit: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-closeout-20260616 on the
next worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not start GPIO32/reset recovery, BMCR/autoneg writes,
Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration,
packet I/O, networking, SSH, Phase 12.2, or a phase transition from this proof.
