# Phase 12.1 RP1 Ethernet BCM54213PE RGMII Delay Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof-20260616

Status: accepted

Classification: rgmii-delay-capture-blocker

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, capture-chain-v4 replay, serial freshness guard v1 replay, focused
Rust tests, JSON validation, docs build, and diff checks.

## Goal

Run the serialized Pi 5 hardware proof for the accepted BCM54213PE RGMII delay
contract and decide only whether the RX/TX delay write/readback boundary reaches
the already accepted BMCR restart/convergence poll frontier.

## Scope Performed

- Acquired hardwareTestLock before lab archive publication and released it only
  after restore proof.
- Added task-specific image, boot-tree, archive, and review wrappers for the
  RGMII delay candidate/control pair.
- Built and reviewed run-unique control and candidate archives.
- Ran the no-MDIO/no-Ethernet control first, restored baseline, then ran the
  candidate.
- Retained selected-tree identity, same-power-cycle TFTP byte evidence, serial
  nonce freshness, final pre-restore identity, restore proof, and final lab
  status.

## Findings

- fixed: control archive retained SHA-256
  8f58e3ff4a19ed9c16d47a1378a733a35f2e2332bc7deaeda2352a6bf051403e, kernel
  SHA-256 72709babec41329add3bcc8898e3907649b4e5759d87417b900a1bafaaa5770c,
  a 50,008-byte kernel_2712.img, selected tree
  8064606a64700931ae0887c2a7d4a0dfb8f899af9f09e7f86c6d8f2ae3b9282c, two
  matching da591740/kernel_2712.img TFTP serves, 42 fresh serial control-marker
  occurrences, and restore to baseline.
- fixed: control proved the no-MDIO/no-Ethernet shape with classification
  no-mdio-no-ethernet-bcm54213pe-rgmii-delay-control.
- fixed: candidate archive retained SHA-256
  d3960ebc0d8054408eb69a3d14b011ca90445d98bbd7ac257488ee604eb0486e, kernel
  SHA-256 a7af33f56baf25845fc251539cedd2fe416b1245069679487ad29b5f6ecffa9f,
  a 53,736-byte kernel_2712.img, selected tree
  9d34d9007a837a0f671c0e627fe85c98531d9a1fa5fe60b88b802a350483be58, two
  matching da591740/kernel_2712.img TFTP serves, 36 fresh serial candidate-marker
  occurrences, and restore to baseline.
- fixed: candidate performed exactly one RX delay write/readback attempt on the
  accepted PHY1 AUX_CTL surface. It observed ncr-before/ncr-after 0x10,
  rx-pre-raw 0x71e7, rx-write-value 0xf1e7, rx-readback-raw 0x71e7, and
  rx-readback-rgmii-skew-en=true.
- fixed: candidate stopped before TX delay write, BMCR restart, and convergence
  polling because the TX selected-register read did not complete. It retained
  tx-pre-raw 0x0, tx-write-value 0x0, tx-readback-raw 0x0,
  tx-readback-gtxclk-en=false, rgmii-delay-write-count 0x1, bmcr-write-count
  0x0, and classification rgmii-delay-capture-blocker.
- fixed: capture-chain-v4 and serial freshness guard v1 accepted both retained
  bundles with no rejection reasons.
- rejected: the candidate does not prove link readiness, packet transport,
  Ethernet driver readiness, networking, sockets, SSH, Phase 12.2, or a phase
  transition.
- rejected: MII_CTRL1000 master-mode writes, GPIO32 reset ownership, Broadcom
  uncontracted selector/config writes, interrupt ownership, and broad PHY/MAC
  configuration remain unaccepted.
- not-an-issue: identity/TFTP/serial/restore evidence was decisive, so the
  capture-blocker classification is a runtime hardware boundary, not a staging
  or capture-chain ambiguity.
- removed: generated boot archives remain untracked target/evidence artifacts;
  retained task evidence records hashes, byte counts, and lab captures.

## Control

The control capture-chain-v4 replay is capture-chain-v4-ready with
decisive_rp1_hardware_classification_allowed=true. The retained serial marker
is the run-unique no-MDIO/no-Ethernet control marker, with no MDIO, MAN, MACB,
GPIO32, PHY, interrupt, packet, networking, SSH, or phase-transition target
construction.

## Candidate

The candidate capture-chain-v4 replay is capture-chain-v4-ready with
decisive_rp1_hardware_classification_allowed=true. The retained runtime
classification is rgmii-delay-capture-blocker. The first failing runtime layer
is the TX delay selected-register read after RX delay write/readback matched.
Because BMCR restart was not reached, link-ready, packet I/O, networking, SSH,
Phase 12.2, and phase transition remain rejected.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/capture-summary.json.
- Static archive review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/archive-review/.
- Control run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/control-run/.
- Candidate run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/candidate-run/.

## Validation

- static archive/image review: scripts/rpi5-archive-review.sh plus the
  task-specific control/candidate review wrappers passed.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and final GET
  /boot/files evidence retained selected-tree identity, final identity, and
  restore proof.
- same-power-cycle TFTP evidence: control retained two matching 50,008-byte
  da591740/kernel_2712.img serves; candidate retained two matching 53,736-byte
  serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained the RGMII delay marker and the TX-read capture-blocker
  runtime facts.
- capture-chain-v4 replay: both bundles passed.
- serial freshness guard v1 replay: both bundles passed.
- cargo fmt --all -- --check: pass.
- focused Rust tests: cargo -Zjson-target-spec test
  rp1_ethernet_bcm54213pe_rgmii_delay -- --nocapture passed with the configured
  QEMU path.
- sh -n on touched shell scripts: pass.
- jq empty on task-owned JSON evidence: pass.
- evidence consistency guard: pass.
- git diff --check: pass.
- mdbook build: pass with the pre-existing large search-index warning.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware evidence is serialized under hardwareTestLock and includes post-run
  baseline restore proof: satisfied.
- Candidate/control identity, TFTP delta, and serial freshness are decisive:
  satisfied.
- Terminal classification is rgmii-delay-capture-blocker, one of the allowed
  terminal classifications: satisfied.
- Rejected packet I/O, networking, SSH, Phase 12.2, and phase-transition claims
  remain explicit: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-closeout-20260616 on the next worker
wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not start packet I/O, networking, SSH, Phase 12.2, or a
phase transition from this proof.
