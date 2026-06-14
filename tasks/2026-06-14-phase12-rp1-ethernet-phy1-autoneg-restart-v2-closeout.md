# Phase 12 RP1 Ethernet PHY1 Autoneg Restart V2 Closeout

Task id: phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout-20260614

Status: accepted

Classification: rp1-ethernet-phy1-autoneg-restart-v2-link-not-ready-frontier-closed

Evidence level: static/task evidence inspection, static archive/image review
evidence review, lab-controller API evidence review, serial hardware
boot/output evidence review, stable same-cursor TFTP delta evidence review,
capture-chain-v4 replay review, boot-staging identity replay review, and
restore proof review.

## Goal

Close out the guarded PHY1 BMCR autonegotiation-restart v2 proof after
capture-staging recovery by recording exactly what the accepted candidate and
paired control proved, preserving rejected Ethernet/networking claims, and
selecting at most one bounded follow-up.

## Scope Performed

- Inspected the accepted v2 proof task record, classification JSON, capture
  summary, evidence map, candidate/control static archive reviews,
  candidate/control capture-chain-v4 outputs, boot-staging identity outputs,
  serial windows, same-power-cycle TFTP deltas, final pre-restore identities,
  and restore evidence.
- Reconciled the accepted BMCR write intent and post-read status against the
  earlier PHY1 status diagnostic, BMSR double-sample link-readiness proof,
  MACB_NSR_LINK read-only proof, and rejected GPIO32/PHY reset claims.
- Recorded the closed v2 frontier as a link-not-ready autoneg-restart
  discriminator, not as link readiness or Ethernet readiness.
- Updated Phase 12 project and roadmap docs with the closed frontier and the
  next queued checkpoint.
- Did not run hardware, publish a boot archive, mutate the lab, change runtime
  source, or start any packet I/O, networking, SSH, Phase 12.2, or phase
  transition work.

## Findings

- fixed: candidate/control capture-chain-v4 and boot-staging identity passed
  with selected-tree TFTP byte agreement, final pre-restore selected-tree
  identity, fresh serial markers, and restore evidence.
- fixed: candidate static review retained selected tree
  c7e847e3ff587fc240ed4b493f42f393f7380c45f5c6b5573fe7c7e45db8f851,
  archive SHA-256
  3adb6dff4b37c9b946ca0ba581e83c58c8ed52ca3853a78f33ff8a512bdf079b,
  kernel SHA-256
  385274a5c97231187d73071462dab686c82c0625fc4c58a914a4cf0926106550,
  and kernel_2712.img size 52344 bytes.
- fixed: control static review retained selected tree
  031da5edc1bb199f260358087e443def1e53fbb4fa1f33d212384d898aab5b56,
  archive SHA-256
  d46466e48e22d9e711e6582eecec6986493a415987277486acc079795b645c92,
  kernel SHA-256
  c72920ed5796c4d54fa2ad470a0ef3198c3cd900b4771738a7fe28c5dc555fcd,
  and kernel_2712.img size 49856 bytes.
- fixed: candidate reached the guarded discriminator with NCR.MPE
  precondition true, pre-BMCR 0x1000, pre-BMSR 0x7949, ANAR 0x01e1, ANLPAR
  0x0000, BMCR isolate clear, exactly one BMCR write intent value 0x1200, and
  touched fields BMCR_ANENABLE and BMCR_ANRESTART.
- fixed: candidate post-read status stayed link-not-ready: post-BMCR 0x1000,
  post-BMSR 0x7949/0x7949, post-ANAR 0x01e1, post-ANLPAR 0x0000,
  BMSR link-status false, BMSR autoneg-complete false, ANLPAR nonzero false,
  passive MACB_NSR raw 0x00000006, and passive MACB_NSR_LINK false.
- fixed: paired control constructed no MDIO/MAN/MACB target, performed no
  volatile Ethernet access, withheld candidate-only reads, and classified as
  no-mdio-no-macb-phy1-autoneg-restart-control.
- deferred: the control helper duplicate overlap remains recorded, but it does
  not add or weaken any accepted runtime claim because the retained control
  capture-chain-v4 and boot-staging identity evidence are decisive and restore
  is proven.
- deferred: post-autoneg source/status interpretation is deliberately limited
  to the queued checkpoint task; this closeout does not create a new proof
  shape or broaden the selected follow-up.
- removed: no source, helper, task, evidence, or documentation files were
  removed.
- not-an-issue: post-BMCR readback 0x1000 without ANRESTART retained is an
  expected discriminator outcome and does not itself prove link readiness.

## Reconciliation

The accepted v2 proof task is
phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof-20260614
at commit 802f77112168f5d6ea181c222019c21357deb1c0.

Unlike the first autoneg-restart proof, the v2 run passed the capture identity
boundary. The candidate selected tree
c7e847e3ff587fc240ed4b493f42f393f7380c45f5c6b5573fe7c7e45db8f851 had two
matching 52344-byte da591740/kernel_2712.img TFTP fetches and final
pre-restore selected-tree identity. The paired control selected tree
031da5edc1bb199f260358087e443def1e53fbb4fa1f33d212384d898aab5b56 had two
matching 49856-byte fetches and final pre-restore selected-tree identity.
Both restored to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The candidate performed one guarded corrected-target PHY1 BMCR write intent
with value 0x1200 after passing the BMCR isolate and NCR.MPE preconditions.
The sampled post-read state did not show link becoming usable: BMCR read back
0x1000, BMSR was 0x7949 on both post samples, autoneg-complete was false,
ANLPAR was 0x0000, and passive MACB_NSR_LINK was false. This closes the v2
discriminator as a write-observed/link-not-ready result.

The evidence does not prove whether link-not-ready is caused by physical
carrier state, partner autonegotiation absence, reset/strap state, PHY power,
operator cabling, or another MAC/PHY integration precondition. It also does
not authorize GPIO32/PHY reset action, PHY configuration beyond the bounded
BMCR autoneg-restart write intent, MACB writes, packet I/O, networking, SSH,
Phase 12.2, or a phase transition.

## Frontier

Closed frontier:
rp1-ethernet-phy1-autoneg-restart-v2-link-not-ready-frontier-closed.

Accepted: capture-fresh candidate/control identity for the v2 run, exactly one
guarded corrected-target PHY1 BMCR autoneg-restart write intent value 0x1200,
post-BMCR 0x1000, post-BMSR 0x7949/0x7949 with link-status and
autoneg-complete false, ANAR 0x01e1, ANLPAR 0x0000, passive
MACB_NSR_LINK=false, and a paired no-MDIO/no-MACB control path.

Not accepted: link readiness, Ethernet readiness, PHY reset/GPIO32 ownership,
operator or cabling diagnosis, broad PHY configuration ownership, MACB writes,
NCR writes, link forcing, packet I/O, DMA/descriptors, interrupt completion,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Direction

Selected next task:
phase12-rp1-ethernet-post-autoneg-status-source-checkpoint-20260614.

The checkpoint is mechanically objective only after this closeout because the
v2 evidence now has capture-fresh BMCR write intent plus link-not-ready PHY/MAC
status. It must stay a source/evidence checkpoint that selects one exact future
read-only status proof or records why no safe post-autoneg status follow-up is
available. It must not directly start PHY configuration, GPIO32/PHY reset
action, MACB writes, packet I/O, networking, SSH, Phase 12.2, or a phase
transition.

## Evidence

- V2 proof task:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof.md.
- V2 proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/classification.json.
- V2 proof capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/capture-summary.json.
- V2 proof evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/evidence-map.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/candidate-run/v4-check.json.
- Candidate boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/candidate-run/boot-staging-identity.json.
- Candidate serial window:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/candidate-run/serial-observe-window.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/control-run/v4-check.json.
- Control boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/control-run/boot-staging-identity.json.
- Final lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/final-lab-status.json.
- Closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: accepted v2 proof task, classification JSON,
  capture summary, evidence map, static archive reviews, candidate/control
  capture-chain-v4 outputs, boot-staging identity outputs, serial windows,
  TFTP deltas, final pre-restore identities, restore evidence, Phase 12 docs,
  roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Closeout records capture identity plus runtime BMCR/autoneg discriminator
  evidence: satisfied.
- Rejected claims for link readiness, GPIO32/PHY reset ownership, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition preserved:
  satisfied.
- Closeout selects at most one objective follow-up: satisfied; selected
  phase12-rp1-ethernet-post-autoneg-status-source-checkpoint-20260614.
- Accepted closeout committed before follow-up starts: satisfied by the
  closeout commit.

## Next Action

On the next worker wake, mechanically promote
phase12-rp1-ethernet-post-autoneg-status-source-checkpoint-20260614 if
dependencies remain satisfied. Do not infer link readiness, PHY reset
ownership, Ethernet behavior, packet I/O, networking, SSH, Phase 12.2, or a
phase transition from this closeout.
