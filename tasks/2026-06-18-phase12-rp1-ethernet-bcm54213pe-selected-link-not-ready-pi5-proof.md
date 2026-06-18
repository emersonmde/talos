# Phase 12.1 RP1 Ethernet BCM54213PE Selected Link-Not-Ready Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof-20260618

Status: accepted

Classification: mii-ctrl1000-master-mode-write-readback-visible

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, serial freshness guard v1, JSON evidence validation, docs build,
and diff checks.

## Goal

Run one serialized Pi 5 proof for the accepted BCM54213PE MII_CTRL1000
master-mode discriminator, retaining selected-tree identity, fresh serial/TFTP
evidence, final identity, and baseline restore proof.

## Scope Performed

- Acquired hardwareTestLock before lab archive publication and retained it
  through final restore evidence.
- Built and statically reviewed the paired no-MDIO/no-Ethernet control archive
  and the MII_CTRL1000 master-mode candidate archive.
- Ran the control first, then the candidate. The initial candidate run was
  retained as inconclusive because capture/staging identity failed before the
  candidate marker could be accepted.
- Performed the task-required inconclusive-run triage before the candidate
  rerun: candidate identity, fresh serial cursor/drain, TFTP delta, known-good
  restore control, then candidate rerun.
- Rebuilt and reviewed a run-unique candidate archive with nonce
  master-mode-candidate-20260618T1410Z and reran the candidate.
- Restored the baseline snapshot
  bcm54213pe-master-mode-pre-20260618T1348Z after the hardware run and retained
  final restored boot-tree evidence.

## Findings

- fixed: the control retained the no-MDIO/no-Ethernet shape with classification
  no-mdio-no-ethernet-bcm54213pe-mii-ctrl1000-master-mode-control, matching
  selected-tree/TFTP bytes, serial nonce freshness, final pre-restore identity,
  and restore proof.
- fixed: the first candidate run was correctly rejected as
  capture-staging-blocked because serial freshness, TFTP expected-byte match,
  and final selected-tree identity were not proven.
- fixed: the decisive candidate rerun retained matching selected-tree identity
  515684b45744c6c89847652c1b34d643a850094d4da3101207fa3b4462d00784,
  two matching 50936-byte da591740/kernel_2712.img TFTP serves, fresh serial
  nonce evidence, final pre-restore identity, and restore proof.
- fixed: the decisive candidate reported NCR 0x10 before and after the proof,
  PHY1 MII_CTRL1000 pre-read 0x0200, write value 0x1a00, readback 0x1a00,
  ctrl1000-pre-read-completed=true, ctrl1000-write-completed=true, and
  ctrl1000-readback-completed=true.
- rejected: link readiness, autoneg-complete, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition remain unaccepted.
- rejected: GPIO32/ETH_RST_N reset ownership, interrupts, APD/EEE/lifecycle,
  MAC/phylink configuration, and same-shaped status/restart/poll retries remain
  outside this proof.
- deferred: closeout belongs to
  phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout-20260618.

## Evidence

- Classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/evidence-map.json.
- Hardware summary:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/hardware-run-summary.json.
- Control archive review:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/control-archive-review.txt.
- Decisive candidate archive review:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/candidate-rerun-archive-review.txt.
- Control run:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/control-run/.
- Initial candidate run:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/candidate-run/.
- Decisive candidate rerun:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/candidate-rerun/.
- Final restore proof:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/final-post-rerun-restore-boot-files.json.

## Validation

- static archive/image review: control and decisive candidate review wrappers
  passed.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and final GET /boot/files
  evidence retained selected-tree identity, final identity, and restore proof.
- same-power-cycle TFTP evidence: control and decisive candidate retained
  matching da591740/kernel_2712.img byte serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  decisive candidate retained the MII_CTRL1000 master-mode marker and runtime
  facts.
- serial freshness guard v1: control and decisive candidate passed.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- HardwareTestLock was serialized, restored, and released with task-owned
  evidence: satisfied.
- Candidate/control selected-tree identity, same-power-cycle TFTP byte serves,
  serial freshness, final pre-restore identity, and restore proof are retained:
  satisfied.
- Terminal classification is one of the accepted source-core classifications:
  mii-ctrl1000-master-mode-write-readback-visible.
- The result does not claim packet I/O, networking, SSH, Phase 12.2, or phase
  transition: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout-20260618 on a
future worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and projects/talos
is clean. Do not start packet I/O, networking, SSH, Phase 12.2, or a phase
transition from this proof.
