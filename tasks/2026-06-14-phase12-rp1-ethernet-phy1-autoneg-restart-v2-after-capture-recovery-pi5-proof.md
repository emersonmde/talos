# Phase 12 RP1 Ethernet PHY1 Autoneg Restart V2 After Capture Recovery Pi 5 Proof

Task id: phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof-20260614

Status: accepted

Classification: phy1-autoneg-restart-write-observed-link-not-ready

Evidence level: static archive/image review, lab-controller API, serial
hardware boot/output, stable same-cursor TFTP delta, capture-chain-v4 replay,
boot-staging identity replay, and restore proof.

## Goal

Retry the guarded corrected-target PHY1 BMCR autonegotiation-restart proof only
after the capture-staging recovery closeout, with candidate/control selected
tree identity, TFTP byte agreement, serial freshness, final pre-restore
identity, and restore evidence.

## Scope Performed

- Rebuilt fresh candidate and paired no-MDIO/no-MACB control archives with
  run-unique capture nonces.
- Ran static archive reviews for both archives.
- Acquired the hardwareTestLock and created restore snapshot
  autoneg-v2-pre-20260614T1545Z.
- Ran the candidate archive on the Pi 5 and retained capture-chain-v4,
  boot-staging identity, serial, TFTP, final identity, and restore evidence.
- Ran the paired no-MDIO/no-MACB control archive and retained the same evidence
  classes.
- Restored the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: capture-staging selected-tree/TFTP/final-identity freshness was
  recovered for this guarded autoneg-restart candidate/control pair.
- fixed: candidate static review retained nonce
  candidate-v2-20260614T154228Z, archive SHA-256
  3adb6dff4b37c9b946ca0ba581e83c58c8ed52ca3853a78f33ff8a512bdf079b,
  kernel SHA-256
  385274a5c97231187d73071462dab686c82c0625fc4c58a914a4cf0926106550,
  and kernel_2712.img size 52344 bytes.
- fixed: control static review retained nonce
  control-v2-20260614T154228Z, archive SHA-256
  d46466e48e22d9e711e6582eecec6986493a415987277486acc079795b645c92,
  kernel SHA-256
  c72920ed5796c4d54fa2ad470a0ef3198c3cd900b4771738a7fe28c5dc555fcd,
  and kernel_2712.img size 49856 bytes.
- fixed: candidate capture-chain-v4 and boot-staging identity passed with
  selected tree c7e847e3ff587fc240ed4b493f42f393f7380c45f5c6b5573fe7c7e45db8f851,
  two matching 52344-byte same-power-cycle TFTP fetches, fresh serial nonce,
  final pre-restore selected-tree identity, and restore evidence.
- fixed: candidate runtime evidence reached the guarded discriminator:
  NCR.MPE precondition true, BMCR isolate clear, exactly one BMCR write intent
  value 0x1200, touched fields BMCR_ANENABLE and BMCR_ANRESTART, post-BMCR
  readback 0x1000, post-BMSR samples 0x7949/0x7949, ANAR 0x01e1, ANLPAR
  0x0000, passive MACB_NSR raw 0x6 / NSR_LINK=false.
- fixed: paired control capture-chain-v4 and boot-staging identity passed with
  selected tree 031da5edc1bb199f260358087e443def1e53fbb4fa1f33d212384d898aab5b56,
  two matching 49856-byte same-power-cycle TFTP fetches, fresh serial nonce,
  final pre-restore selected-tree identity, and restore evidence.
- fixed: paired control constructed no MDIO/MAN/MACB target, performed no
  volatile Ethernet access, withheld candidate-only fields, and classified as
  no-mdio-no-macb-phy1-autoneg-restart-control.
- deferred: the control run was accidentally duplicated while the first helper
  was still active; the retained v4 and boot-staging evidence are ready and the
  lab restore is proven, but no additional runtime claim is accepted from the
  duplicate overlap.
- not-an-issue: candidate post-BMCR readback not retaining ANRESTART is a valid
  outcome of the bounded discriminator, not proof of link readiness.
- removed: no source, helper, task, docs, or evidence files were removed.

## Candidate Result

~~~text
classification=phy1-autoneg-restart-physical-or-operator-precondition-blocker
selected_tree=c7e847e3ff587fc240ed4b493f42f393f7380c45f5c6b5573fe7c7e45db8f851
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=52344
tftp_expected_fetch_count=2
tftp_expected_fetch_byte_match_count=2
serial_marker_present=true
ncr-before=0x10
ncr-mpe-precondition-met=true
ncr-after=0x10
pre-bmcr=0x1000
pre-bmsr=0x7949
pre-anar=0x01e1
pre-anlpar=0x0000
bmcr-isolate-precondition-clear=true
bmcr-write-value=0x1200
bmcr-write-count=1
touched-fields=BMCR_ANENABLE,BMCR_ANRESTART
post-bmcr=0x1000
post-bmsr-first=0x7949
post-bmsr-second=0x7949
post-anar=0x01e1
post-anlpar=0x0000
passive-macb-nsr-raw=0x00000006
passive-macb-nsr-link=false
bmcr-write-performed=true
mdio-man-transactions-performed=true
macb-read-performed=true
macb-write-performed=false
phy-reset-or-gpio32-action=false
link-forcing=false
claims-ethernet-ready=false
claims-packet-io=false
claims-networking=false
claims-ssh=false
claims-phase-12-2=false
claims-phase-transition=false
~~~

## Control Result

~~~text
classification=no-mdio-no-macb-phy1-autoneg-restart-control
selected_tree=031da5edc1bb199f260358087e443def1e53fbb4fa1f33d212384d898aab5b56
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=49856
tftp_expected_fetch_count=2
tftp_expected_fetch_byte_match_count=2
serial_marker_present=true
target=none
controller=none
selected-reads=withheld
bmcr-write-count=0
touched-fields=none
bmcr-write-performed=false
mdio-man-transactions-performed=false
macb-read-performed=false
macb-write-performed=false
phy-reset-or-gpio32-action=false
link-forcing=false
claims-ethernet-ready=false
claims-packet-io=false
claims-networking=false
claims-ssh=false
claims-phase-12-2=false
claims-phase-transition=false
~~~

## Boundary

Accepted: candidate/control capture identity for the v2 run, one guarded
corrected-target PHY1 BMCR write attempt with value 0x1200, post-read BMCR
0x1000, unchanged link-not-ready PHY/MAC status samples, and a paired control
with no MDIO/MAN/MACB target construction.

Not accepted: link readiness, Ethernet readiness, PHY reset/GPIO32 ownership,
MACB writes, NCR writes, link forcing, packet I/O, DMA/descriptors, interrupt
completion, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/classification.json.
- Capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/evidence-map.json.
- Candidate archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/archive-review/candidate-static-review.txt.
- Control archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/candidate-run/v4-check.json.
- Candidate boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/candidate-run/boot-staging-identity.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/control-run/v4-check.json.
- Control boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/control-run/boot-staging-identity.json.
- Final lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/final-lab-status.json.

## Validation

- passed: static archive/image review for candidate and control.
- passed: serialized Pi 5 hardware run under hardwareTestLock.
- passed: candidate/control capture-chain-v4 and boot-staging identity gates.
- passed: stable same-cursor TFTP delta and fresh serial cursor evidence for
  candidate and control.
- passed: final pre-restore identity and restore proof for candidate and
  control.
- passed: jq empty on task-owned JSON evidence.
- passed: git diff --check.
- passed: mdbook build because docs/src files were touched.
- passed: git diff --cached --check before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout-20260614 on the next
worker wake if dependencies remain satisfied. The closeout must preserve the
link-not-ready and no-broader-Ethernet boundary before any follow-up status
checkpoint.
