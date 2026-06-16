# Phase 12.1 RP1 Ethernet BCM54213PE RGMII Delay TX-Order Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof-20260616

Status: accepted

Classification: rgmii-delay-tx-order-timeout-link-not-ready

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, capture-chain-v4 replay, serial freshness guard v1 replay, JSON
validation, docs build, and diff checks.

## Goal

Run the serialized Pi 5 hardware proof for the accepted RGMII delay TX-order
contract and decide only the corrected RX-to-TX delay stage, BMCR restart, and
bounded convergence frontier.

## Scope Performed

- Acquired hardwareTestLock before lab archive publication and released it only
  after baseline restore proof.
- Built and statically reviewed run-unique control and candidate archives using
  the accepted TX-order task, contract, source-correction, stage-accounting, and
  rejected-claim markers.
- Ran the no-MDIO/no-Ethernet control first, then the corrected candidate.
- Retained selected-tree identity, same-power-cycle TFTP byte evidence, serial
  nonce freshness, final pre-restore identity, restore proof, and final restored
  lab status.

## Findings

- fixed: the control retained the no-MDIO/no-Ethernet shape with classification
  no-mdio-no-ethernet-bcm54213pe-rgmii-delay-tx-order-control, capture-chain-v4
  ready evidence, serial freshness guard v1 ready evidence, matching TFTP byte
  serves, and restore proof.
- fixed: the candidate retained selected-tree/TFTP/serial/final-identity/restore
  evidence and capture-chain-v4 plus serial freshness guard v1 both classified
  ready.
- fixed: the candidate reached RX selected read/write/readback and reported
  rx-readback-rgmii-skew-en=true.
- fixed: the candidate reached TX selector/write/readback accounting. It
  observed tx-pre-raw 0x0e00, tx-readback-raw 0x0e00,
  tx-readback-gtxclk-en=true, tx-selected-read-completed=true, and
  tx-readback-completed=true.
- fixed: the candidate skipped a redundant TX write because GTXCLK_EN was
  already enabled, recording tx-delay-write-skipped-already-enabled=true and
  tx-delay-write-completed=false.
- fixed: after RX/TX delay criteria were satisfied, the candidate executed
  exactly one BMCR restart write and then completed eight bounded convergence
  samples.
- fixed: convergence timed out with link not ready: poll-bmsr-link-status=false,
  poll-bmsr-autoneg-complete=false, passive-macb-nsr-link=false, and
  link-ready-terminal=false.
- rejected: link readiness, packet transport, Ethernet driver readiness,
  networking, sockets, SSH, Phase 12.2, and phase transition remain unaccepted.
- rejected: MII_CTRL1000 master-mode writes, GPIO32 reset ownership, broad
  PHY/MAC configuration, interrupt ownership, and packet/DMA work remain
  unaccepted.
- not-an-issue: TX delay write not performed is accepted for this run because
  the selected TX read already had GTXCLK_EN set and the proof core explicitly
  selected the skip policy.
- deferred: closeout belongs to
  phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout-20260616.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/capture-summary.json.
- Control run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/control-run/.
- Candidate run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/candidate-run/.

## Validation

- static archive/image review: task-specific control/candidate review wrappers
  passed.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and final GET /boot/files
  evidence retained selected-tree identity, final identity, and restore proof.
- same-power-cycle TFTP evidence: control and candidate retained matching
  da591740/kernel_2712.img byte serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained the TX-order marker and runtime facts.
- capture-chain-v4 replay: control and candidate passed.
- serial freshness guard v1 replay: control and candidate passed.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware evidence is serialized under hardwareTestLock and includes post-run
  baseline restore proof: satisfied.
- Candidate identity, fresh serial cursor, TFTP delta, candidate/control
  capture, final identity, and restore evidence are retained: satisfied.
- Terminal classification is one of the proof-core allowed classifications:
  rgmii-delay-tx-order-timeout-link-not-ready.
- Broader packet/networking/SSH/Phase 12.2 claims remain rejected: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout-20260616 on a
future worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and projects/talos
is clean. Do not start packet I/O, networking, SSH, Phase 12.2, or a phase
transition from this proof.
