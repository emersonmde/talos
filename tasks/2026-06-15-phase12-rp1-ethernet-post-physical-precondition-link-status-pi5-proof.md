# Phase 12 RP1 Ethernet Post-Physical-Precondition Link Status Pi 5 Proof

Task id: phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof-20260614

Status: blocked

Classification: post-physical-link-status-source-precondition-blocker

Evidence level: static archive/image review, lab-controller API evidence,
serial hardware boot/output, stable same-cursor TFTP delta, capture-chain-v4
replay, boot-staging identity replay, restore proof, and source-contract gate
review.

## Goal

Run the serialized Pi 5 read-only link-status proof after the operator
confirmed the physical Ethernet link precondition and the source contract was
accepted.

## Scope Performed

- Acquired the hardwareTestLock before boot publication and power-cycle work.
- Built run-unique candidate/control boot archives for the post-physical link
  status proof.
- Ran static archive/image review for the candidate and paired control.
- Published, power-cycled, captured, restored, and recorded candidate/control
  Pi 5 evidence.
- Reconciled the runtime output against capture-chain-v4, boot-staging
  identity, TFTP byte agreement, final pre-restore identity, restore proof,
  and the accepted source contract.

## Findings

- fixed: candidate/control selected-tree identity, same-power-cycle TFTP byte
  agreement, final pre-restore identity, fresh serial nonce evidence, and
  restore evidence were captured for both runs.
- fixed: candidate static review retained archive SHA-256
  7d635de13d5dceb1b73f6af03701d9bb733e22c4c0f51c49736b7eee29b7296e,
  kernel SHA-256
  5e6e9a2d08daedd2b5a6c5bf59d36cc4c35866b1524f76245ef4867760bda6e2,
  kernel_2712.img size 53752 bytes, and nonce
  postphys-20260615T1329Z-candidate.
- fixed: control static review retained archive SHA-256
  7f2053b4233b5178226fecf965b496a9ce303658b39872705735ab05e516b13e,
  kernel SHA-256
  4e25ac12ba7f0f276b3c650e4cda3dcd5b6558bfdc7ae8063f07a072a68266a1,
  kernel_2712.img size 50960 bytes, and nonce
  postphys-20260615T1329Z-control.
- fixed: runtime candidate output reached the intended status sample and
  emitted BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000,
  MACB_NSR 0x00000006, BMSR link-status false, BMSR autoneg-complete false,
  ANLPAR nonzero false, and MACB_NSR_LINK false.
- fixed: paired control constructed no MDIO/MAN/MACB target, performed no
  volatile Ethernet access, and emitted
  no-mdio-no-macb-post-physical-link-status-control.
- deferred: the runtime candidate output by itself would classify as
  post-physical-link-status-phy-not-ready, but this task does not accept that
  runtime classification because the source-contract gate below failed.
- deferred: the candidate implementation uses MACB MAN transactions to issue
  PHY1 MDIO reads while the accepted source contract says the future candidate
  must not write MAN and claims macb_write_count=0. That contradiction requires
  closeout/supervisor reconciliation before the runtime not-ready result can
  drive follow-up planning.
- removed: no source, helper, task, evidence, or documentation files were
  removed.
- not-an-issue: restore evidence returned the lab boot root to the pre-run
  baseline tree before the hardwareTestLock was released.

## Reconciliation

The accepted source contract selected corrected-target PHY1 BMCR, BMSR,
ANAR, ANLPAR reads plus a passive MACB_NSR_LINK read after the physical-link
precondition. It also stated that the candidate must not write BMCR, MAN,
NCR, MACB_NSR, GPIO32, RIO, pads, or any other PHY/MAC/GPIO register, and the
report surface retained macb_write_count=0.

The implemented candidate necessarily issued MACB MAN transactions to perform
the PHY1 MDIO reads:

~~~text
write_rp1_reg_u32_ordered(man, read_frame)
~~~

That makes the source-contract gate fail. The hardware run remains useful
evidence because capture identity, TFTP byte agreement, serial freshness, and
restore all passed, but the accepted classification for this task is limited
to post-physical-link-status-source-precondition-blocker. It does not accept
link readiness, Ethernet readiness, PHY reset ownership, PHY configuration,
packet I/O, networking, SSH, Phase 12.2, or a phase transition. It also does
not accept the runtime phy-not-ready result as a planning frontier until the
closeout reconciles whether MAN read-command writes are allowed, counted, or
require a revised source contract.

## Evidence

- Classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/capture-summary.json.
- Candidate static review:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/archive-review/candidate-static-review.txt.
- Control static review:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/control-run/v4-check.json.
- Candidate boot-staging identity:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/candidate-run/boot-staging-identity.json.
- Control boot-staging identity:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/control-run/boot-staging-identity.json.
- Candidate same-cursor TFTP delta:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/candidate-run/tftp-delta-stable-pre-restore.json.
- Control same-cursor TFTP delta:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/control-run/tftp-delta-stable-pre-restore.json.
- Final lab status:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/final-lab-status.json.

## Validation

- static archive/image review: candidate and control review scripts passed and
  recorded archive/kernel hashes and byte counts.
- lab-controller API: candidate/control publication, boot file identity,
  TFTP logs, power-cycle records, restore snapshots, and final lab status were
  recorded.
- serial hardware boot/output: candidate and control serial windows contained
  fresh run-unique nonces.
- stable same-cursor TFTP delta: candidate saw two matching 53752-byte
  da591740/kernel_2712.img fetches; control saw two matching 50960-byte
  fetches.
- capture-chain-v4 replay: candidate and control classified as
  capture-chain-v4-ready.
- boot-staging identity replay: candidate and control classified as
  boot-staging-identity-ready.
- restore proof: both runs restored to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- source-contract gate review: blocked by MAN read-command writes conflicting
  with the accepted no-MAN-write source contract boundary.
- JSON validation: jq empty on task-owned JSON evidence.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Candidate/control identity, TFTP byte agreement, serial marker, final
  pre-restore identity, and restore evidence: satisfied.
- Classification bounded to the source contract: satisfied by blocker
  classification only; runtime phy-not-ready is recorded but not accepted.
- Hardware lock released only after restore evidence recorded: satisfied.
- Task accepted as runtime link-status proof: not satisfied.
- Task blocked with committed classification/evidence for closeout: satisfied.

## Next Action

Promote
phase12-rp1-ethernet-post-physical-precondition-link-status-closeout-20260614
on the next worker wake if dependencies remain satisfied. The closeout must
reconcile the MAN read-command/source-contract contradiction before selecting
any follow-up.
