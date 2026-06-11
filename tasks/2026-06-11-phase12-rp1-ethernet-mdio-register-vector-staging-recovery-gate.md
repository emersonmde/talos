# Phase 12 RP1 Ethernet MDIO Register Vector Staging Recovery Gate

Task id: phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate-20260611

Status: accepted

Classification: boot-staging-identity-ready

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable pre-restore TFTP delta,
and restore evidence. This gate accepts only boot-staging identity durability,
not MDIO register-vector hardware data.

## Goal

Run a bounded lab staging/power-cycle discriminator after the blocked
register-vector proof reported a publication-to-power-cycle identity mismatch.

## Scope Performed

- Acquired hardwareTestLock before archive publication, power cycling, or
  restore actions.
- Built and statically reviewed a fresh register-vector candidate archive with
  run-unique capture nonce `staging-recovery-20260611T1533Z`.
- Created a pre-run restore snapshot and retained baseline boot identity.
- Published the candidate archive and captured selected-tree identity,
  expected `da591740/kernel_2712.img` fetch bytes, fresh serial/TFTP cursors,
  power-cycle evidence, stable pre-restore TFTP delta, final pre-restore
  identity, and restore evidence.
- Ran the staging identity checker against the retained bundle and baseline.
- Restored the baseline boot tree before release of hardwareTestLock.

## Findings

- fixed: the recovery gate reproduced the selected register-vector candidate
  publication with a fresh archive and nonce.
- fixed: selected-tree identity survived publication through power cycle to
  final pre-restore identity.
- fixed: stable pre-restore TFTP evidence showed two matching
  `da591740/kernel_2712.img` fetches with the selected 52,344-byte kernel.
- fixed: final pre-restore boot files still reported the selected tree
  `3b004386594d5e81bb93c5b0a8a3f2c1822dfa26b46ce2c51e544b7b5715d845`.
- fixed: post-run restore returned the lab to baseline tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- not-an-issue: capture-chain-v4 still rejected decisive RP1 hardware
  classification because the pre-power serial drain was non-empty; that is
  outside this gate's accepted staging identity claim.
- deferred: MDIO register-vector candidate/control proof retry, broad MDIO/PHY
  ownership, GPIO32/PHY reset, Ethernet behavior, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future explicit work.
- removed: no source, docs, or stale evidence was removed.

## Staging Result

The staging identity checker accepted the recovery gate:

~~~text
classification=boot-staging-identity-ready
selected_tree=3b004386594d5e81bb93c5b0a8a3f2c1822dfa26b46ce2c51e544b7b5715d845
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=52344
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=3b004386594d5e81bb93c5b0a8a3f2c1822dfa26b46ce2c51e544b7b5715d845
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
~~~

The prior blocked proof's publication-to-power mismatch was not reproduced by
this recovery gate. The current accepted locus is stale capture-chain
bookkeeping or superseded lab publication state, not current lab-controller
publish/restore behavior.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate/evidence-map.json.
- Staging identity check:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate/candidate-staging-run/staging-identity-check.json.
- Capture-chain summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate/candidate-staging-run/capture-invariant-summary.json.
- Publication, serial, TFTP, and restore evidence:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate/candidate-staging-run/.
- Static archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-recovery-gate/archive-review/candidate-static-review.txt.

## Validation

- static archive review: candidate archive review passed with the run-unique
  capture nonce.
- image/archive inspection: archive SHA and kernel strings retained.
- lab-controller API: pre-run snapshot, publication, power-cycle, final
  pre-restore identity, and restore evidence retained.
- serial hardware boot/output: run-unique serial marker observed, but RP1
  hardware classification intentionally remains outside this gate.
- stable pre-restore TFTP delta: two matching selected kernel fetches observed
  before restore.
- staging identity check: `rpi5-boot-staging-identity-check.sh` accepted.
- JSON validation: `jq empty` on task-owned classification/evidence-map/
  capture-summary JSON.
- diff check: `git diff --check`.
- docs validation: not required; no `docs/src` files touched.
- staged diff check: `git diff --cached --check` before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- HardwareTestLock acquisition/release, current boot identity, serial cursor,
  TFTP cursor/delta, restore state, and snapshot evidence are recorded:
  satisfied.
- Mismatch locus identified: satisfied; current gate does not reproduce the
  publication-to-power mismatch and points to stale capture-chain bookkeeping or
  superseded lab publication state.
- Passing recovery gate proves selected-tree identity and expected
  `kernel_2712.img` bytes survive publication through power cycle to
  TFTP/final identity: satisfied.
- Tooling fix: not required.
- Final lab state restored before hardwareTestLock release: satisfied.

## Next Action

Mechanically promote
`phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery-20260611`
on the next worker wake if hardwareTestLock remains unlocked/restored and
supervisorIntervention is inactive. Keep scope to the selected candidate/control
proof; do not infer broad MDIO/PHY ownership, Ethernet behavior, networking,
SSH, Phase 12.2, or a phase transition from this recovery gate.
