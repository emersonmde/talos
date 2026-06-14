# Phase 12 RP1 Ethernet MDIO Register Vector Pi 5 Proof V4 After Root Recovery

Task id:
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery-20260614

Status: accepted

Classification: mdio-phy1-register-vector-visible

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable same-cursor TFTP delta,
capture-chain-v4 replay, staging identity gate, evidence-consistency guard,
and restore evidence.

## Goal

Run the guarded MDIO register-vector Pi 5 proof after the served-root/root
recovery closeout, and classify only the selected six-register Clause 22 PHY1
vector or a precise capture/precondition blocker.

## Scope Performed

- Reviewed the accepted register-vector source contract, guard/core reports,
  quarantine boundary, minimal sentinel served-root proof, and root-recovery
  closeout before hardware publication.
- Built fresh run-unique candidate and paired no-MDIO/no-Ethernet control
  archives from the accepted Pi 5 boot source.
- Statically reviewed both archives before hardware publication.
- Acquired hardwareTestLock before lab publication, power-cycle, serial/TFTP
  capture, or restore.
- Published and captured exactly one candidate and one control run, retaining
  serial, TFTP, final identity, capture-chain-v4, staging identity, and restore
  evidence for each.
- Restored the lab to the pre-run baseline snapshot before releasing the
  hardware lock.

## Findings

- fixed: candidate archive review passed with archive SHA-256
  7c3f1179eb8b551e85f1c5177d771de6c1ecf55b8cde3f883a4e54afa47491e7,
  kernel SHA-256
  a3b2339f0ecf45b816066d80257d9d6c346713e57e358b94dc243a1516aed641,
  and kernel_2712.img at 52,352 bytes.
- fixed: control archive review passed with archive SHA-256
  b6bf4c52d5809722c21e7741cd705d0d4afc08f395603834a2ec25f83aef228e,
  kernel SHA-256
  73058002ab3aaaaf255eee051df5233a451a791ee955fd5fa410c75ad59bda1d,
  and kernel_2712.img at 50,112 bytes.
- fixed: candidate capture-chain-v4 and staging identity both passed. The
  candidate selected tree was
  043744bcf578d7966c63600c3db0302e35e96ec631f6d535725c8e63002fd43d, TFTP
  served two da591740/kernel_2712.img fetches at the expected 52,352 bytes, and
  final pre-restore identity stayed on the selected tree.
- fixed: control capture-chain-v4 and staging identity both passed. The control
  selected tree was
  0a8aab5b6103bf42c28b7d202ef1022b94c443a08879641c89ef481c59e516a8, TFTP
  served two da591740/kernel_2712.img fetches at the expected 50,112 bytes, and
  final pre-restore identity stayed on the selected tree.
- fixed: candidate serial output captured a visible six-register PHY1 vector
  after corrected NCR.MPE was already set: BMCR 0x1000, BMSR 0x7949, PHYSID1
  0x600d, PHYSID2 0x84a2, ANAR 0x01e1, and ANLPAR 0x0000.
- fixed: paired control used the same reporting path while constructing no
  MDIO target, no MAN frame, and no runtime MDIO transaction.
- fixed: final restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: PHY reset/GPIO32 ownership, broad MDIO/PHY ownership, link state,
  Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicit tasks.
- removed: no source code, docs, or evidence was removed.
- not-an-issue: no NCR write was performed; corrected NCR.MPE was already set
  before MAN transactions.

## Candidate Result

~~~text
classification=mdio-phy1-register-vector-visible
capture-chain-v4=capture-chain-v4-ready
staging-identity-gate=boot-staging-identity-ready
selected_tree=043744bcf578d7966c63600c3db0302e35e96ec631f6d535725c8e63002fd43d
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=52352
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=043744bcf578d7966c63600c3db0302e35e96ec631f6d535725c8e63002fd43d
ncr_before=0x10
ncr_mpe_precondition_met=true
register_vector=BMCR:0x1000,BMSR:0x7949,PHYSID1:0x600d,PHYSID2:0x84a2,ANAR:0x01e1,ANLPAR:0x0000
man_after_vector=0x60821000,0x60867949,0x608a600d,0x608e84a2,0x609201e1,0x60960000
man_writes_performed=true
ncr_mpe_write_performed=false
serial_freshness_ok=true
nonce_token_occurrences=19
pre_power_nonce_occurrences=0
~~~

## Control Result

~~~text
classification=no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control
capture-chain-v4=capture-chain-v4-ready
staging-identity-gate=boot-staging-identity-ready
selected_tree=0a8aab5b6103bf42c28b7d202ef1022b94c443a08879641c89ef481c59e516a8
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=50112
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=0a8aab5b6103bf42c28b7d202ef1022b94c443a08879641c89ef481c59e516a8
claims_runtime_mdio_transaction=false
serial_freshness_ok=true
nonce_token_occurrences=20
pre_power_nonce_occurrences=0
~~~

## Boundary

Accepted: the selected corrected-target PHY1 Clause 22 six-register
register-vector MAN.DATA boundary under capture-chain-v4, staging identity,
same-power-cycle TFTP byte agreement, final identity, and restore evidence.

Not accepted: PHY absence, PHY reset/GPIO32 ownership, broad MDIO/PHY
ownership, link state, Ethernet behavior, interrupts, DMA/descriptors, packet
I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/classification.json.
- Capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/evidence-map.json.
- Candidate archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/archive-review/candidate-static-review.txt.
- Control archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/archive-review/control-static-review.txt.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/candidate-run/v4-check.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/control-run/v4-check.json.
- Evidence-consistency guard:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/evidence-consistency-guard-output.json.
- Final restored lab status:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/final-lab-status.json.

## Validation

- static archive review: candidate and control passed.
- image/archive inspection: archive SHA, kernel SHA, kernel size, and forbidden
  string checks retained for candidate/control.
- lab-controller API: snapshot creation, publication, power cycle, serial,
  TFTP, final identity, restore, and post-restore identity retained.
- serial hardware boot/output: candidate/control nonce-bearing markers were
  present after power and absent before power.
- stable TFTP delta: candidate/control each observed two matching expected
  da591740/kernel_2712.img fetches.
- capture-chain-v4 replay: candidate/control classified capture-chain-v4-ready.
- staging identity gate: candidate/control classified boot-staging-identity-ready.
- evidence-consistency guard: passed.
- JSON validation: jq empty passed over task-owned JSON evidence.
- diff check: git diff --check passed.
- documentation build: mdbook build passed after docs updates.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- HardwareTestLock acquisition/release and post-run restore are recorded:
  satisfied.
- Candidate and control archive reviews retain run-unique nonces, archive
  hashes, kernel hashes, and byte counts: satisfied.
- Candidate and control retain serial evidence, same-cursor TFTP deltas, final
  pre-restore identity, capture-chain-v4 output, staging identity gate output,
  and evidence-consistency output: satisfied.
- Classification is an allowed source-contract outcome:
  mdio-phy1-register-vector-visible for the candidate and
  no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control for the
  control.
- Broad MDIO/PHY ownership, PHY absence, reset ownership, Ethernet behavior,
  networking, SSH, Phase 12.2, and phase transition claims remain rejected:
  satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-v4-closeout-20260614 on the next
worker wake if dependencies remain satisfied. Do not start networking, packet
I/O, DMA/descriptors, interrupts, SSH, Phase 12.2, or a phase transition from
this proof.
