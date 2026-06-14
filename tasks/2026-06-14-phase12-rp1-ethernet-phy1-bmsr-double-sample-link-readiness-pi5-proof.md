# Phase 12 RP1 Ethernet PHY1 BMSR Double-Sample Link Readiness Pi 5 Proof

Task id: phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof-20260614

Status: accepted

Classification: mdio-phy1-bmsr-double-sample-link-not-ready

Evidence level: static archive/image review, unit tests, lab-controller API,
serial hardware boot/output, stable same-cursor TFTP delta, capture-chain-v4
run-unique replay, boot-staging identity replay, and restore proof.

## Goal

Prove the selected read-only corrected-target PHY1 BMCR plus double-sampled
BMSR link-readiness discriminator, without PHY configuration, reset,
autonegotiation restart, link forcing, packet I/O, networking, SSH, Phase
12.2, or a phase transition.

## Scope Performed

- Added candidate and paired no-MDIO/no-Ethernet control boot scenarios and
  image/archive/review scripts for the BMSR double-sample discriminator.
- Built fresh run-unique candidate and control archives from the accepted Pi 5
  boot source and statically reviewed both archives before hardware
  publication.
- Acquired hardwareTestLock before lab publication, power-cycle, serial/TFTP
  capture, or restore.
- Ran serialized candidate and control Pi 5 captures with run-unique serial
  nonces, stable same-cursor TFTP deltas, final pre-restore identity, restore
  proof, capture-chain run-unique replay, and boot-staging identity replay.
- Restored the lab to the pre-run baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: candidate archive review passed with archive SHA-256
  ae921b0cd82029e367fc5a27a1bbb1ac2220309bdeefcc1426f6e94ebd648676,
  kernel SHA-256 0c29ff0cba5c1b9f35dc52d5cdff33fb65df2b3f08acd8846dfb2bec80ea18be,
  and kernel_2712.img size 52,536 bytes.
- fixed: control archive review passed with archive SHA-256
  32209d5fd3ad2d80923d30a139af9d227c4e39af84924b42c35887b069d64a49,
  kernel SHA-256 224a1a603bc3ba67c16e0b19e878c504958825b864b541001260f01c0e69761a,
  and kernel_2712.img size 50,112 bytes.
- fixed: candidate capture-chain run-unique and boot-staging identity checks
  passed. The selected tree 83efecf2cbf7135492907335fb4a00a54c7374768b3d4a87c774721f49c2d94a
  served da591740/kernel_2712.img twice at 52,536 bytes and remained staged
  through final pre-restore identity.
- fixed: control capture-chain run-unique and boot-staging identity checks
  passed. The selected tree 52c81ff4b8249df118da553e00c528eeaa83b25ab5a44ffb5468675fec31f749
  served da591740/kernel_2712.img twice at 50,112 bytes and remained staged
  through final pre-restore identity.
- fixed: candidate serial output reported corrected-target BMCR 0x1000, first
  BMSR 0x7949, and second BMSR 0x7949. BMCR reset, loopback, and
  autoneg-restart were false; second-sample BMSR link-status and
  autoneg-complete were false. The accepted classification is
  mdio-phy1-bmsr-double-sample-link-not-ready.
- fixed: paired control serial output constructed no MDIO target, no MAN
  frame, and no volatile Ethernet MDIO load/store, and classified as
  no-mdio-no-ethernet-rp1-ethernet-phy1-link-readiness-control.
- deferred: PHY reset/GPIO32 ownership, PHY configuration, autonegotiation
  restart, link forcing, broad MDIO/PHY ownership, Ethernet driver behavior,
  interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase
  12.2, and phase transition remain future explicit tasks.
- removed: no source code, docs, or evidence was removed.
- not-an-issue: the candidate performed only the selected corrected-target
  Clause 22 MAN read sequence: BMCR 0x60820000, BMSR first 0x60860000, and
  BMSR second 0x60860000, with bounded NSR.IDLE polling and no NCR write.

## Candidate Result

~~~text
classification=mdio-phy1-bmsr-double-sample-link-not-ready
capture-chain-v4=capture-transaction-run-unique-ready
boot-staging-identity=boot-staging-identity-ready
selected_tree=83efecf2cbf7135492907335fb4a00a54c7374768b3d4a87c774721f49c2d94a
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=52536
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=83efecf2cbf7135492907335fb4a00a54c7374768b3d4a87c774721f49c2d94a
raw_bmcr=0x1000
raw_bmsr_first=0x7949
raw_bmsr_second=0x7949
bmcr_preconditions_clear=true
second_bmsr_link_status=false
second_bmsr_autoneg_complete=false
second_bmsr_autoneg_ability=true
pre_power_nonce_occurrences=0
post_power_nonce_occurrences=18
~~~

## Control Result

~~~text
classification=no-mdio-no-ethernet-rp1-ethernet-phy1-link-readiness-control
capture-chain-v4=capture-transaction-run-unique-ready
boot-staging-identity=boot-staging-identity-ready
selected_tree=52c81ff4b8249df118da553e00c528eeaa83b25ab5a44ffb5468675fec31f749
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=50112
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=52c81ff4b8249df118da553e00c528eeaa83b25ab5a44ffb5468675fec31f749
constructed_mdio_target=false
constructed_man_frame=false
pre_power_nonce_occurrences=0
post_power_nonce_occurrences=20
~~~

## Boundary

Accepted: a read-only corrected-target PHY1 BMCR plus double-sampled BMSR
link-readiness discriminator under capture-chain-v4/run-unique freshness,
boot-staging identity, stable TFTP byte agreement, final pre-restore identity,
and restore proof. The observed result is link-not-ready because the second
BMSR sample has BMSR_LSTATUS=false and BMSR_ANEGCOMPLETE=false.

Not accepted: PHY configuration writes, PHY reset/GPIO32 action,
autonegotiation restart, link forcing, broad MDIO/PHY ownership, Ethernet
driver behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets,
SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/evidence-map.json.
- Candidate archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/archive-review/candidate-static-review.txt.
- Control archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain run-unique:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain run-unique:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/control-run/v4-check.json.
- Candidate boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/candidate-run/boot-staging-identity.json.
- Control boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/control-run/boot-staging-identity.json.
- Evidence consistency guard:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/evidence-consistency-guard.json.
- Final restored lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/final-lab-status.json.

## Validation

- formatting: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 508 tests.
- static archive/image review: candidate and control passed.
- lab-controller API: snapshot creation, publication, power cycle, serial,
  TFTP, final identity, restore, and post-restore identity retained.
- serial hardware boot/output: candidate/control nonce-bearing markers were
  absent before power and present after power.
- stable TFTP delta: candidate/control each observed two matching expected
  da591740/kernel_2712.img fetches.
- capture-chain-v4 run-unique replay: candidate/control classified
  capture-transaction-run-unique-ready.
- boot-staging identity replay: candidate/control classified
  boot-staging-identity-ready.
- evidence-consistency guard: classified evidence-consistency-ready.
- JSON validation: jq empty passed over task-owned JSON evidence.
- diff check: git diff --check passed.
- documentation build: mdbook build passed after docs updates.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Candidate output names the selected contract and reports BMCR, first BMSR,
  second BMSR, BMCR preconditions, second-sample BMSR_LSTATUS, and
  second-sample BMSR_ANEGCOMPLETE: satisfied.
- Candidate performs only BMCR 0x60820000, BMSR first 0x60860000, and BMSR
  second 0x60860000 with bounded NSR.IDLE polling: satisfied.
- BMCR preconditions are clear and second-sample BMSR link/autoneg-complete
  classify link-readiness as link-not-ready: satisfied.
- Paired control uses the same reporting path but constructs no MDIO target or
  MAN frame and withholds candidate-only fields: satisfied.
- Candidate/control evidence passes capture-chain-v4, boot-staging identity,
  same-power-cycle TFTP byte agreement, serial freshness, final pre-restore
  identity, and restore proof: satisfied.
- Classification is limited to an allowed link-not-ready result and paired
  control: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout-20260614
on the next worker wake if dependencies remain satisfied. Do not start PHY
configuration, reset/GPIO32 action, packet I/O, networking, SSH, Phase 12.2,
or a phase transition directly from this proof.
