# Phase 12 RP1 Ethernet Post-Physical Link Status V2 Pi 5 Proof

Task id: phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof-20260615

Status: accepted

Classification: post-physical-link-status-phy-not-ready

Evidence level: static archive/image review, lab-controller API evidence,
serial hardware boot/output, stable same-cursor TFTP delta, capture-chain-v4
replay, boot-staging identity replay, evidence-consistency guard, and restore
proof.

## Goal

Run the serialized Pi 5 proof for the accepted v2 post-physical link-status
contract after MAN read-command accounting was accepted.

## Scope Performed

- Acquired hardwareTestLock before archive publication and power-cycle work.
- Built run-unique candidate/control boot archives for the v2 post-physical
  link-status proof.
- Ran static archive/image review for the candidate and paired control.
- Published, power-cycled, captured, restored, and recorded candidate/control
  Pi 5 evidence.
- Reconciled runtime output against capture-chain-v4, boot-staging identity,
  same-power-cycle TFTP byte agreement, final pre-restore identity, restore
  proof, and the accepted v2 MAN read-command accounting boundary.

## Findings

- fixed: candidate/control selected-tree identity, expected TFTP fetch bytes,
  final pre-restore identity, fresh serial markers, and restore proof were
  decisive.
- fixed: candidate runtime output reports the accepted v2 contract id, the
  confirmed physical-link precondition, five selected PHY1 MAN read-command
  transactions, bounded MAN read-command accounting, passive MACB_NSR read
  accounting, and explicit forbidden-action booleans.
- fixed: candidate MAN activity is limited to the accepted Clause 22 PHY1 read
  commands for BMCR, BMSR first, BMSR second, ANAR, and ANLPAR.
- fixed: candidate reports BMCR write, PHY configuration write, GPIO32/PHY reset
  action, link forcing, DMA/descriptors, packet I/O, networking, SSH, Phase
  12.2, and phase transition claims as false.
- fixed: paired control uses the same report surface while constructing no
  MDIO/MAN/MACB target and performing no volatile Ethernet access.
- deferred: post-physical link remains not ready; closeout must select a
  source-grounded follow-up focused on PHY power/reset/strap/autoneg status
  recovery or pause.
- removed: generated boot archives were removed after upload; retained evidence
  keeps archive hashes, kernel hashes, and byte counts.
- not-an-issue: the bounded MAN read-command writes are accepted by the v2
  contract and are accounted separately from forbidden configuration writes.
- not-an-issue: a first local helper attempt used a too-large serial max_bytes
  value and received an API 400 before accepted evidence was captured; the lab
  was restored before the retained run and the accepted proof uses the 65536
  byte serial API limit.

## Candidate

Candidate classification: post-physical-link-status-phy-not-ready.

Static review retained archive SHA-256
acf2a2f76228bbc4ce416eab5b2593c2bdeffccd168dc7a82b678cf5e4a6b872,
kernel SHA-256 b41992697ed69d9d44cfa688cecc8a2c50adc8612d7a753103434ad77cca0bce,
kernel_2712.img size 54488 bytes, and nonce
postphys-v2-20260615T1610Z-candidate.

Hardware capture retained selected tree
2d5ded595b98df7fec9048292f03d6694b8ee6417450ccae6e8a7838f372e026, two
matching 54488-byte da591740/kernel_2712.img TFTP fetches, final pre-restore
identity on the selected tree, and 14 fresh candidate marker/nonce occurrences.
capture-chain-v4 and boot-staging identity both classified the candidate as
ready.

Runtime fields:

~~~text
post-physical-link-status-contract-id=phase12-rp1-ethernet-post-physical-link-status-contract-v2
physical-link-precondition=confirmed
ncr-before=0x10 ncr-mpe-precondition-met=true ncr-after=0x10
selected-phy1-man-read-commands=BMCR:0x00:0x60820000,BMSR-first:0x01:0x60860000,BMSR-second:0x01:0x60860000,ANAR:0x04:0x60920000,ANLPAR:0x05:0x60960000
bmcr-raw=0x1000 bmsr-first-raw=0x7949 bmsr-second-raw=0x7949 anar-raw=0x1e1 anlpar-raw=0x0 macb-nsr-raw=0x6
bmsr-second-link-status=false bmsr-second-autoneg-complete=false anlpar-nonzero=false macb-nsr-link=false
mdio-read-count=5 man-read-command-write-count=5 phy-configuration-write-count=0 bmcr-write-count=0 macb-read-count=1 macb-configuration-write-count=0
bmcr-write-performed=false phy-config-write-performed=false phy-reset-or-gpio32-action=false link-forcing=false packet-io-performed=false
classification=post-physical-link-status-phy-not-ready
~~~

## Control

Control classification: no-mdio-no-macb-post-physical-link-status-control.

Static review retained archive SHA-256
bf7657ff9e025ab61589e264f2c95e39f93955865a27f5ac1f885565f90ff1f1,
kernel SHA-256 c257592ab7a25f3c58852c076f4cbe6f37df7cfff1e160c00acaf495b841f7b8,
kernel_2712.img size 51528 bytes, and nonce
postphys-v2-20260615T1610Z-control.

Hardware capture retained selected tree
50a38c766f12f320900c09a1a7def1702e547c8bf05cff4df5a529e6b4386b1b, two
matching 51528-byte da591740/kernel_2712.img TFTP fetches, final pre-restore
identity on the selected tree, and 15 fresh control marker/nonce occurrences.
capture-chain-v4 and boot-staging identity both classified the control as
ready.

Runtime fields:

~~~text
post-physical-link-status-contract-id=phase12-rp1-ethernet-post-physical-link-status-contract-v2
report-kind=no-mdio-no-macb-control
target=none controller=none compatible=none
ncr-observed-target=not-constructed nsr-observed-target=not-constructed man-observed-target=not-constructed macb-nsr-target=not-constructed
selected-reads=withheld selected-phy1-man-read-commands=withheld
mdio-read-count=0 man-read-command-write-count=0 phy-configuration-write-count=0 bmcr-write-count=0 macb-read-count=0 macb-configuration-write-count=0
bmcr-write-performed=false phy-config-write-performed=false phy-reset-or-gpio32-action=false link-forcing=false packet-io-performed=false
classification=no-mdio-no-macb-post-physical-link-status-control
~~~

## Rejected Claims And Retained Risks

Rejected claims:

- PHY reset ownership;
- GPIO32 ownership or action;
- PHY configuration writes;
- BMCR writes;
- MAC configuration writes;
- autonegotiation restart;
- link forcing;
- DMA/descriptors;
- packet I/O;
- interrupts;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The accepted result is only a PHY/MAC status sample at the selected instant.
- A not-ready result still requires source-grounded follow-up before PHY reset,
  strap, power, or configuration work.
- Packet I/O and network stack work remain blocked until later explicit tasks
  accept their prerequisites.

## Evidence

- Classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/capture-summary.json.
- Candidate static review:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/archive-review/candidate-static-review.txt.
- Control static review:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/control-run/v4-check.json.
- Candidate boot-staging identity:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/candidate-run/boot-staging-identity.json.
- Control boot-staging identity:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/control-run/boot-staging-identity.json.
- Final lab status:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/final-lab-status.json.

## Validation

- static archive/image review: candidate and control review scripts passed and
  recorded archive/kernel hashes and byte counts.
- lab-controller API: candidate/control publication, boot file identity, TFTP
  logs, power-cycle records, restore snapshots, and final lab status were
  recorded.
- serial hardware boot/output: candidate and control serial windows contained
  fresh run-unique nonces and the expected report surfaces.
- stable same-cursor TFTP delta: candidate saw two matching 54488-byte
  da591740/kernel_2712.img fetches; control saw two matching 51528-byte
  fetches.
- capture-chain-v4 replay: candidate and control classified as
  capture-chain-v4-ready.
- boot-staging identity replay: candidate and control classified as
  boot-staging-identity-ready.
- restore proof: both runs restored to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- evidence-consistency guard: candidate/control task evidence is
  evidence-consistency-ready.
- JSON validation: jq empty on task-owned JSON evidence.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Candidate/control selected-tree identity, expected TFTP fetch bytes, final
  pre-restore identity, fresh serial markers, and restore proof: satisfied.
- Candidate output reports the accepted v2 contract id, confirmed physical-link
  precondition, five selected PHY1 read transactions, bounded MAN accounting,
  passive MACB_NSR read accounting, and explicit forbidden-action booleans:
  satisfied.
- Candidate MAN activity is limited to accepted Clause 22 read-command frames:
  satisfied.
- Paired control reports the same surface while constructing no MDIO/MAN/MACB
  target and performing no volatile Ethernet access: satisfied.
- Classification remains bounded and does not accept packet I/O, networking,
  SSH, Phase 12.2, or a phase transition: satisfied.
- Accepted result committed before closeout starts: satisfied once this task is
  committed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-post-physical-link-status-v2-closeout-20260615 on the next
worker wake if dependencies remain satisfied and hardwareTestLock is
unlocked/restored. The closeout should reconcile the not-ready v2 result without
asking Matthew to reconfirm the already accepted physical link precondition.
