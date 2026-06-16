# Phase 12.1 RP1 Ethernet BCM54213PE Autoneg Convergence Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof-20260616

Status: accepted

Classification: bcm54213pe-autoneg-convergence-timeout-link-not-ready

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, capture-chain-v4 replay, serial freshness guard v1 replay,
focused Rust tests, JSON validation, and diff checks.

## Goal

Run one serialized Pi 5 control/candidate proof for the accepted BCM54213PE
autoneg convergence core and decide only whether the accepted single PHY1 BMCR
autoneg restart write converges to link-ready/autoneg-complete inside the
bounded poll window.

## Scope Performed

- Acquired the hardwareTestLock before archive publication and released it only
  after restore proof.
- Added task-specific boot-tree, archive, and review wrappers for the
  convergence proof.
- Built and reviewed run-unique control and candidate archives.
- Ran the no-MDIO/no-Ethernet control first, restored baseline, then ran the
  candidate.
- Retained selected-tree identity, same-power-cycle TFTP byte evidence, serial
  nonce freshness, final pre-restore identity, restore proof, and final lab
  status.
- Fixed a terminal-label mismatch found by the first hardware run:
  local/static code emitted `bcm54213pe-autoneg-convergence-still-timeout`,
  while this Pi 5 proof's acceptance gate requires
  `bcm54213pe-autoneg-convergence-timeout-link-not-ready`. The source label
  and validators were corrected, focused tests passed, and the hardware proof
  was rerun.

## Findings

- fixed: control archive retained SHA-256
  6415a5b6967b0af5d8f022af048f8472ca5cad0200c9637ace940341c4f46510, kernel
  SHA-256 6ca31cec3b2ff92a5ca894d10c9f308387fc20798c74456b7202ced152ccc3a7,
  a 49,712-byte kernel_2712.img, selected tree
  c0cba209ffcf845d644f4e7461e3305aaed0fc6d5bb0edf2d798bb57f331e17b, two
  matching da591740/kernel_2712.img TFTP serves, 44 fresh serial nonce
  occurrences, and restore to baseline.
- fixed: control proved the no-MDIO/no-Ethernet shape with classification
  no-mdio-no-ethernet-bcm54213pe-autoneg-convergence-control.
- fixed: candidate archive retained SHA-256
  fb16311fa5c0a1cc8aad645037e68a582d4aed9d857c8fec2921e784dee55fab, kernel
  SHA-256 f47dab86743978fabadfda8c747e9ddb008d60a5b02d36adb22ad99e8a5c6502,
  a 52,248-byte kernel_2712.img, selected tree
  a932c281bd02341694a1440eb1316b6ea6c582c814e1add9f1fef5e2727bafa4, two
  matching da591740/kernel_2712.img TFTP serves, 39 fresh serial nonce
  occurrences, and restore to baseline.
- fixed: candidate performed exactly one corrected-target PHY1 BMCR write
  frame 0x50821200 for value 0x1200, then completed the bounded eight-sample
  poll window.
- fixed: candidate terminal poll values were BMCR 0x1000, BMSR 0x7949/0x7949,
  ANAR 0x01e1, ANLPAR 0x0000, MII_CTRL1000 0x0200, MII_STAT1000 0x0000, and
  passive MACB_NSR 0x00000006.
- fixed: candidate classified timeout/link-not-ready: BMSR link status false,
  BMSR autoneg-complete false, passive MACB_NSR_LINK false, and
  link-ready-terminal false after eight samples.
- fixed: capture-chain-v4 and serial freshness guard v1 accepted both retained
  rerun bundles with no rejection reasons.
- rejected: the candidate does not prove link readiness, packet transport,
  Ethernet driver readiness, networking, sockets, SSH, Phase 12.2, or a phase
  transition.
- rejected: GPIO32 reset ownership, Broadcom selector/config writes, interrupt
  ownership, broad PHY/MAC configuration, and link forcing remain unaccepted.
- not-an-issue: post-fix rerun evidence was decisive, so no inconclusive-run
  triage or candidate-only retry was needed.
- removed: generated boot archives remain untracked target/evidence artifacts;
  retained task evidence records hashes, byte counts, and lab captures.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/capture-summary.json.
- Static archive review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/archive-review/.
- Control rerun:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/control-rerun-run/.
- Candidate rerun:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/candidate-rerun-run/.

## Validation

- static archive/image review: scripts/rpi5-archive-review.sh plus the
  task-specific control/candidate review wrappers passed.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and final GET
  /boot/files evidence retained selected-tree identity, final identity, and
  restore proof.
- same-power-cycle TFTP evidence: control retained two matching 49,712-byte
  da591740/kernel_2712.img serves; candidate retained two matching 52,248-byte
  serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained the convergence-poll marker and timeout/link-not-ready
  raw/decoded values.
- capture-chain-v4 replay: both rerun bundles passed.
- serial freshness guard v1 replay: both rerun bundles passed.
- cargo fmt --all -- --check: pass.
- focused Rust tests: cargo -Zjson-target-spec test
  rp1_ethernet_bcm54213pe_autoneg_convergence -- --nocapture passed with the
  configured QEMU path.
- sh -n on touched shell scripts: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: required because docs/src was touched.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware lock acquisition/release and restore evidence are recorded:
  satisfied.
- Paired control proves the no-MDIO/no-Ethernet shape without target facts:
  satisfied.
- Candidate selected-tree, same-power-cycle TFTP, serial freshness, final
  identity, and restore evidence agree: satisfied.
- Candidate proves exactly one accepted BMCR restart write followed by the
  bounded convergence poll schedule: satisfied.
- Terminal classification is
  bcm54213pe-autoneg-convergence-timeout-link-not-ready: satisfied.
- Rejected reset/config/interrupt/packet/networking/SSH/Phase 12.2 claims
  remain explicit: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-closeout-20260616 on the
next worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not start GPIO32/reset, Broadcom selector/config,
interrupt, packet I/O, networking, SSH, Phase 12.2, or a phase transition from
this proof.
