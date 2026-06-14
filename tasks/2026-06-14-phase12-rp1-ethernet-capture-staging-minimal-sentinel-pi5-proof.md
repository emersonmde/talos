# Phase 12 RP1 Ethernet Capture-Staging Minimal Sentinel Pi 5 Proof

Task id: phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof-20260614

Status: accepted

Classification: capture-staging-minimal-sentinel-proof-accepted

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable same-cursor TFTP delta,
capture-chain-v4 replay, staging identity gate, and restore evidence.

## Goal

Run the fresh minimal candidate/control sentinel selected by the recurrence
checkpoint to prove whether the current live capture-staging path can again
hold selected-tree identity, same-power-cycle TFTP-served byte identity, serial
freshness, final pre-restore identity, and restore evidence without MDIO, MACB,
PHY, GPIO32, packet I/O, networking, SSH, or Phase 12.2 behavior.

## Scope Performed

- Acquired the hardware lock and serialized Pi 5 lab actions.
- Built fresh run-unique no-MDIO/no-Ethernet candidate and control sentinel
  archives from the accepted staging-sentinel scenario.
- Statically reviewed both archives before publication and retained archive
  hashes, kernel hashes, kernel byte counts, and archive file lists.
- Ran a paired control after an initially confounded candidate attempt; the
  confounded attempt is retained but excluded from acceptance because a manual
  restore was issued while the background session was still completing.
- Re-ran the candidate only after the control proved the capture path was
  fresh and decisive.
- Captured candidate/control selected-tree identity, fresh serial evidence,
  stable same-cursor TFTP deltas, final pre-restore identity, and restore
  evidence.

## Findings

- fixed: control selected tree
  9e3442962d40beff3b31668065df4f5d8ac37ee770fa3c4bba725d049bc78db3 with
  expected da591740/kernel_2712.img size 47832 bytes; same-power-cycle TFTP
  served two matching 47832-byte fetches and final pre-restore identity stayed
  on the selected tree.
- fixed: candidate rerun selected tree
  520785f412ba93da8c25577e5f5e4635ffba02b2969fbf3e02a346e97e061799 with
  expected da591740/kernel_2712.img size 47848 bytes; same-power-cycle TFTP
  served two matching 47848-byte fetches and final pre-restore identity stayed
  on the selected tree.
- fixed: candidate and control both had fresh run-unique serial nonces present
  after power and absent before power.
- fixed: final lab restore returned to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  da591740/kernel_2712.img at 104136 bytes.
- deferred: the original candidate attempt is retained under
  candidate-run-aborted-20260614T143813Z but excluded from acceptance because a
  manual restore confounded its TFTP/final-identity evidence.
- removed: no source, helper, task, or documentation files were removed.
- not-an-issue: the reused staging-sentinel image contains the required
  no-MDIO/no-Ethernet/no-MMIO-target-construction strings and rejects runtime
  Ethernet/link claims.

## Result

~~~text
classification=capture-staging-minimal-sentinel-proof-accepted
candidate_gate=selected-tree-identity-ready
candidate_tree=520785f412ba93da8c25577e5f5e4635ffba02b2969fbf3e02a346e97e061799
candidate_expected_fetch_bytes=47848
candidate_tftp_expected_fetch_count=2
candidate_tftp_expected_fetch_byte_match_count=2
control_gate=selected-tree-identity-ready
control_tree=9e3442962d40beff3b31668065df4f5d8ac37ee770fa3c4bba725d049bc78db3
control_expected_fetch_bytes=47832
control_tftp_expected_fetch_count=2
control_tftp_expected_fetch_byte_match_count=2
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
selected_next_task=phase12-rp1-ethernet-capture-staging-recovery-closeout-20260614
~~~

## Boundary

Accepted: current capture-staging selected-tree/TFTP/final-identity freshness
for one minimal no-MDIO/no-Ethernet candidate/control pair.

Not accepted: autonegotiation restart, BMCR writes, MDIO register vectors,
MACB_NSR reads, GPIO32/PHY reset action, Ethernet link readiness, packet I/O,
DMA/descriptors, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/evidence-map.json.
- Candidate rerun summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run/run-summary.json.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run/v4-check.json.
- Candidate staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run/staging-identity-gate-output.json.
- Candidate stable TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run/tftp-delta-stable-pre-restore.json.
- Control summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/control-run/run-summary.json.
- Control capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/control-run/v4-check.json.
- Control staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/control-run/staging-identity-gate-output.json.
- Control stable TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/control-run/tftp-delta-stable-pre-restore.json.
- Confounded original candidate attempt:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/candidate-run-aborted-20260614T143813Z/.

## Validation

- static archive review: candidate and control reviews passed.
- image/archive inspection: archive hashes, kernel hashes, kernel byte counts,
  and archive file lists retained.
- lab-controller API: snapshot creation, publication, selected-tree identity,
  TFTP cursor, final pre-restore identity, restore, and final lab status
  retained.
- serial hardware boot/output: candidate and control run-unique serial markers
  were present after power and absent before power.
- stable same-cursor TFTP delta: candidate and control each observed two
  matching expected fetches before restore.
- capture-chain-v4 replay: candidate and control were capture-chain-v4-ready.
- staging identity gate: candidate and control were selected-tree-identity-ready.
- JSON validation: jq empty on task-owned JSON evidence.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.

## Acceptance Check

- Candidate and control use fresh run-unique markers and task-owned evidence:
  satisfied.
- Candidate selected-tree identity, expected kernel bytes, TFTP-served bytes,
  serial marker freshness, and final pre-restore identity agree in the same
  power-cycle: satisfied by the candidate rerun.
- Known-good/control triage present before candidate rerun after the confounded
  attempt: satisfied by the accepted control run.
- Final lab restore returns to baseline and hardwareTestLock is released:
  satisfied.
- Task-owned classification/evidence JSON selects
  phase12-rp1-ethernet-capture-staging-recovery-closeout-20260614 next:
  satisfied.
- Accepted proof committed before closeout starts: satisfied by the commit for
  this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-capture-staging-recovery-closeout-20260614 on the next
worker wake if dependencies remain satisfied. Do not start autoneg retry,
BMCR/MDIO/MACB/GPIO32 work, packet I/O, networking, SSH, Phase 12.2, or a
phase transition directly from this proof.
