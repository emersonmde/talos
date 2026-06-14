# Phase 12 RP1 Ethernet PHY1 Status Diagnostic Pi 5 Proof

Task id:
phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof-20260614

Status: accepted

Classification: mdio-phy1-status-diagnostic-visible

Evidence level: static archive review, unit tests, image/archive inspection,
lab-controller API, serial hardware boot/output, stable same-cursor TFTP delta,
capture-chain-v4 replay, boot-staging identity gate, and restore evidence.

## Goal

Implement and prove the smallest user-visible PHY1 status diagnostic on top of
the accepted corrected-target MDIO register-vector frontier, decoding live
Clause 22 fields without claiming Ethernet driver or networking behavior.

## Scope Performed

- Added PHY1 status decode structures and helpers for BMCR, BMSR, PHY ID, ANAR,
  and ANLPAR.
- Added candidate and paired no-MDIO/no-Ethernet control boot scenarios and
  archive/review scripts.
- Built fresh run-unique candidate and control archives from the accepted Pi 5
  boot source.
- Statically reviewed both archives before hardware publication.
- Acquired hardwareTestLock before lab publication, power-cycle, serial/TFTP
  capture, or restore.
- Captured one confused candidate overlap run and retained it as non-acceptance
  evidence only.
- Reran clean candidate and control captures, retaining serial, TFTP, final
  identity, capture-chain-v4, boot-staging identity, and restore evidence.
- Restored the lab to the pre-run baseline snapshot before releasing the
  hardware lock.

## Findings

- fixed: candidate archive review passed with archive SHA-256
  4d607211a1c0587a847dbd6fd5185672054f2b473a51e9bfb46faf251524da02,
  kernel SHA-256
  790696cccd6b87b5d86139a0e1fd380cea0c627150fa4b112022122dfb490ab0,
  and kernel_2712.img at 54,008 bytes.
- fixed: control archive review passed with archive SHA-256
  c6b067da21498e25a3e2e32ed46ca3603bbffcf71926dcbb6edfcf88066f0d4b,
  kernel SHA-256
  ed9336a6d644b44a853672ee1b6a7f17e70ec78106331e44cc1575cf7bfe058e,
  and kernel_2712.img at 49,736 bytes.
- fixed: candidate capture-chain-v4 and boot-staging identity both passed. The
  candidate selected tree was
  39eeabec22164e31bb0290f05b4985fc6392d38a8703ca6725693621739b84b6, TFTP
  served two da591740/kernel_2712.img fetches at the expected 54,008 bytes, and
  final pre-restore identity stayed on the selected tree.
- fixed: control capture-chain-v4 and boot-staging identity both passed. The
  control selected tree was
  3afdd601766c459afd88c33eb92b716bc797c0c51fbba8744efe8a985799d16d, TFTP
  served two da591740/kernel_2712.img fetches at the expected 49,736 bytes, and
  final pre-restore identity stayed on the selected tree.
- fixed: candidate serial output captured and decoded a visible PHY1 status
  vector: BMCR 0x1000, BMSR 0x7949, PHYSID1 0x600d, PHYSID2 0x84a2, ANAR
  0x01e1, and ANLPAR 0x0000.
- fixed: candidate decoded BMCR reset=false, loopback=false, speed=10M,
  autoneg-enable=true; BMSR link-status=false, autoneg-complete=false,
  autoneg-ability=true; PHY ID OUI 0x180361, model 0x0a, revision 0x02.
- fixed: paired control used the same reporting surface while constructing no
  MDIO target, no MAN frame, and no runtime MDIO transaction.
- fixed: final restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: PHY reset/GPIO32 ownership, broad MDIO/PHY ownership, link
  usability, Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicit tasks.
- removed: no source code, docs, or evidence was removed.
- not-an-issue: the candidate performed MAN reads against the accepted
  corrected-target MDIO read boundary; it performed no NCR write, no PHY
  configuration write, no PHY reset/GPIO32 action, no autonegotiation restart,
  and no link forcing.

## Candidate Result

~~~text
classification=mdio-phy1-status-diagnostic-visible
capture-chain-v4=capture-chain-v4-ready
selected_tree=39eeabec22164e31bb0290f05b4985fc6392d38a8703ca6725693621739b84b6
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=54008
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=39eeabec22164e31bb0290f05b4985fc6392d38a8703ca6725693621739b84b6
raw_vector=BMCR:0x1000,BMSR:0x7949,PHYSID1:0x600d,PHYSID2:0x84a2,ANAR:0x01e1,ANLPAR:0x0000
bmcr=reset:false,loopback:false,speed:10M,autoneg:true
bmsr=link:false,autoneg-complete:false,autoneg-ability:true
phy_id=oui:0x180361,model:0x0a,revision:0x02
anar=selector:1,10hd:true,10fd:true,100tx-hd:true,100tx-fd:true
anlpar=selector:0,acknowledge:false
serial_freshness_ok=true
pre_power_nonce_occurrences=0
~~~

## Control Result

~~~text
classification=no-mdio-no-ethernet-rp1-ethernet-phy1-status-diagnostic-control
capture-chain-v4=capture-chain-v4-ready
selected_tree=3afdd601766c459afd88c33eb92b716bc797c0c51fbba8744efe8a985799d16d
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=49736
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=3afdd601766c459afd88c33eb92b716bc797c0c51fbba8744efe8a985799d16d
claims_runtime_mdio_transaction=false
constructed_mdio_target=false
constructed_man_frame=false
serial_freshness_ok=true
pre_power_nonce_occurrences=0
~~~

## Boundary

Accepted: a visible PHY1 status diagnostic over the accepted corrected-target
MDIO read boundary, with raw values and decoded BMCR, BMSR, PHY ID, ANAR, and
ANLPAR fields under capture-chain-v4, boot-staging identity, same-power-cycle
TFTP byte agreement, final identity, serial freshness, and restore evidence.

Not accepted: link usability beyond decoded register state, PHY configuration
writes, PHY reset/GPIO32 action, autonegotiation restart, link forcing, broad
MDIO/PHY ownership, Ethernet driver behavior, interrupts, DMA/descriptors,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/evidence-map.json.
- Candidate archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/archive-review/candidate-static-review.txt.
- Control archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/control-run/v4-check.json.
- Final restored lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/final-lab-status.json.

## Validation

- static archive review: candidate and control passed.
- image/archive inspection: archive SHA, kernel SHA, kernel size, and forbidden
  string checks retained for candidate/control.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- lab-controller API: snapshot creation, publication, power cycle, serial,
  TFTP, final identity, restore, and post-restore identity retained.
- serial hardware boot/output: candidate/control nonce-bearing markers were
  present after power and absent before power.
- stable TFTP delta: candidate/control each observed two matching expected
  da591740/kernel_2712.img fetches.
- capture-chain-v4 replay: candidate/control classified capture-chain-v4-ready.
- boot-staging identity gate: candidate/control classified boot-staging-identity-ready.
- JSON validation: jq empty passed over task-owned JSON evidence.
- diff check: git diff --check passed.
- documentation build: mdbook build passed after docs updates.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Candidate reports live corrected-target PHY1 raw vector and decoded BMCR,
  BMSR, PHY ID, ANAR, and ANLPAR fields: satisfied.
- Candidate uses only the accepted corrected-target MDIO read boundary and
  performs no NCR write, PHY configuration write, reset/GPIO32 action,
  autonegotiation restart, link forcing, packet I/O, DMA, interrupt, socket, or
  SSH work: satisfied.
- Paired control constructs no MDIO target or MAN frame and proves the
  no-MDIO/no-Ethernet reporting path: satisfied.
- Candidate/control evidence passes capture-chain-v4, boot-staging identity,
  same-power-cycle TFTP byte agreement, serial freshness, final pre-restore
  identity, and restore proof: satisfied.
- Classification is limited to a visible PHY1 status diagnostic and paired
  control: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-phy1-status-diagnostic-closeout-20260614 on the next
worker wake if dependencies remain satisfied. Do not start link-readiness,
PHY configuration, reset/GPIO32, packet I/O, networking, SSH, Phase 12.2, or a
phase transition directly from this proof.
