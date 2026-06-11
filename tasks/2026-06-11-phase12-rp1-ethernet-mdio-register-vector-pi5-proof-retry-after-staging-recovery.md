# Phase 12 RP1 Ethernet MDIO Register Vector Pi 5 Proof Retry After Staging Recovery

Task id: phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery-20260611

Status: accepted

Classification: mdio-phy1-register-vector-visible

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable TFTP delta,
capture-chain-v4 replay, and restore evidence.

## Goal

Retry the serialized Pi 5 corrected-target MDIO register-vector candidate and
paired no-MDIO/no-Ethernet control after the staging recovery gate proved that
selected-tree identity survives publication through power cycle, TFTP, final
identity, and restore.

## Scope Performed

- Consumed the accepted staging recovery gate and accepted register-vector guard
  closeout.
- Acquired hardwareTestLock before archive publication, staging, power cycle,
  serial/TFTP capture, or runtime MDIO evidence.
- Built candidate and control archives with fresh run-unique capture nonces.
- Ran static archive reviews for both archives.
- Published the candidate archive, ran a serialized Pi 5 capture-chain-v4
  proof, and restored the pre-run baseline snapshot.
- Published the paired no-MDIO/no-Ethernet control archive, ran the same
  capture-chain-v4 proof path, and restored the pre-run baseline snapshot.
- Recorded candidate/control classification JSON, evidence map, capture
  summaries, serial/TFTP/final identity evidence, and final lab restore state.

## Findings

- fixed: the post-recovery candidate selected tree survived publication through
  power cycle to TFTP and final identity.
- fixed: candidate capture-chain-v4 accepted the proof identity and freshness
  gate with no rejection reasons.
- fixed: candidate observed corrected-target PHY1 register vector values:
  BMCR 0x1000, BMSR 0x7949, PHYSID1 0x600d, PHYSID2 0x84a2, ANAR 0x1e1, and
  ANLPAR 0x0.
- fixed: candidate observed NCR.MPE already set with ncr-before 0x10 and
  performed no NCR write.
- fixed: candidate performed selected MAN writes only after the NCR.MPE
  precondition was met; touched-fields is MAN.
- fixed: paired control capture-chain-v4 accepted the proof identity and
  freshness gate with no rejection reasons.
- fixed: paired control constructed no MDIO target, no MAN frame, no runtime
  MDIO transaction, no volatile target access, and touched no fields.
- fixed: final lab state was restored to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  release of the hardware lock.
- deferred: broad MDIO/PHY ownership, PHY absence claims from all-ones values,
  GPIO32/PHY reset action, Ethernet driver behavior, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future explicit work.
- removed: no source, docs, or stale evidence was removed.
- not-an-issue: pre-power serial drain saturation on the candidate is handled
  by capture-chain-v4 run-unique nonce freshness; the paired control had an
  empty pre-power serial read.

## Candidate Result

The candidate v4 replay accepted the run:

~~~text
classification=capture-chain-v4-ready
decisive_rp1_hardware_classification_allowed=true
rejection_reasons=[]
selected_tree=b901be8a925e644ffc3f932d258ed7413522f95a8a36a3df7c8ae5182ee745fc
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=52344
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=b901be8a925e644ffc3f932d258ed7413522f95a8a36a3df7c8ae5182ee745fc
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_marker_occurrences=15
serial_freshness_ok=true
~~~

Accepted candidate serial report:

~~~text
classification=mdio-phy1-register-vector-visible
observed-window-macb-mid-context-raw=0x70109
corrected-window-comparator-cpu-physical-target=0x1f001000fc
ncr-observed-target=0x1c00100000
nsr-observed-target=0x1c00100008
man-observed-target=0x1c00100034
ncr-before=0x10
ncr-mpe-precondition-met=true
ncr-after=0x10
ncr-mpe-write-performed=false
bmcr=0x1000
bmsr=0x7949
physid1=0x600d
physid2=0x84a2
anar=0x1e1
anlpar=0x0
completed-register-count=6
nsr-before-vector=0x6,0x6,0x6,0x6,0x6,0x6
nsr-after-vector=0x6,0x6,0x6,0x6,0x6,0x6
man-after-vector=0x60821000,0x60867949,0x608a600d,0x608e84a2,0x609201e1,0x60960000
man-writes-performed=true
man-restore-write-performed=false
touched-fields=MAN
claims-ncr-write-executed=false
claims-mdio-phy-ownership=false
claims-phy-absence-from-all-ones=false
claims-ethernet-ready=false
claims-networking=false
claims-ssh=false
claims-phase-transition=false
~~~

## Control Result

The paired control v4 replay accepted the run:

~~~text
classification=capture-chain-v4-ready
decisive_rp1_hardware_classification_allowed=true
rejection_reasons=[]
selected_tree=6b44308801a677b59beb1f5c3a951f6b2ff2e7e531c2671cbeb51ee2b49e92f6
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=50104
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=6b44308801a677b59beb1f5c3a951f6b2ff2e7e531c2671cbeb51ee2b49e92f6
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_marker_occurrences=20
serial_freshness_ok=true
~~~

Accepted control serial report:

~~~text
classification=no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control
target=none
controller=none
phy-address=none
ncr-observed-target=not-constructed
nsr-observed-target=not-constructed
man-observed-target=not-constructed
completed-register-count=0
man-writes-performed=false
touched-fields=none
claims-runtime-mdio-transaction=false
claims-ncr-write-executed=false
claims-mdio-phy-ownership=false
claims-ethernet-ready=false
~~~

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/evidence-map.json.
- Candidate v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/candidate-run/v4-check.json.
- Candidate serial output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/candidate-run/serial-observe-window.json.
- Candidate serial tail:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/candidate-run/serial-peek-after-candidate-restore.json.
- Candidate stable TFTP delta:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/candidate-run/tftp-delta-stable-pre-restore.json.
- Control v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/control-run/v4-check.json.
- Control serial output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/control-run/serial-observe-window.json.
- Control stable TFTP delta:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/control-run/tftp-delta-stable-pre-restore.json.
- Restore/final lab evidence:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/final-lab-status.json.

## Validation

- static archive review: candidate and control archive reviews passed.
- image/archive inspection: candidate/control archive SHA and kernel image
  sizes retained.
- lab-controller API: snapshot, publication, power-cycle, final identity,
  TFTP delta, and restore evidence retained for candidate and control.
- serial hardware boot/output: candidate and control run-unique markers
  observed.
- capture-chain-v4 replay: candidate and control classified
  capture-chain-v4-ready with no rejection reasons.
- JSON validation: jq empty on task-owned classification/evidence-map/
  capture-summary JSON.
- diff check: git diff --check.
- docs validation: not required; no docs/src files touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- HardwareTestLock acquisition/release and boot restore state are recorded:
  satisfied.
- Candidate/control selected-tree identity, expected TFTP fetches, fresh serial
  markers, final pre-restore identity, restore proof, capture summary,
  classification JSON, and evidence map: satisfied.
- Candidate records accepted selected register-vector MAN.DATA values after
  satisfied MPE gate and no NCR write: satisfied.
- Control proves no MDIO target/MAN frame/volatile access through the same
  reporting path: satisfied.
- Accepted proof committed before proof closeout starts: satisfied by this
  task commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-proof-closeout-20260611 on the next
worker wake if dependencies remain satisfied. Do not infer broad MDIO/PHY
ownership, Ethernet behavior, networking, SSH, Phase 12.2, or a phase
transition from this proof.
