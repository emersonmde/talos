# Phase 12 RP1 Ethernet MDIO Register Vector Pi 5 Proof V2 After Evidence Guard

Task id: phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard-20260611

Status: blocked

Classification: mdio-register-vector-guarded-v2-candidate-identity-mismatch-blocker

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable TFTP delta,
capture-chain-v4 replay, evidence-consistency guard/check, and restore
evidence.

## Goal

Rerun the selected MDIO register-vector candidate/control proof only after the
evidence-consistency guard is accepted, preserving a mechanically checkable
agreement chain from raw candidate/control v4 JSON through aggregate evidence
and task record.

## Scope Performed

- Performed pre-publication triage of the prior inconsistent retry.
- Built candidate and paired no-MDIO/no-Ethernet control archives with fresh
  run-unique capture nonces.
- Ran static archive reviews for both archives before lab publication.
- Acquired hardwareTestLock before lab publication, power action, serial
  capture, TFTP observation, or restore.
- Created a pre-run boot snapshot.
- Published the candidate archive and ran capture-chain-v4 evidence collection.
- Published the paired control archive and ran capture-chain-v4 evidence
  collection.
- Restored the pre-run snapshot and recorded the final restored lab state.
- Wrote candidate/control v4 JSON, aggregate classification JSON, capture
  summary, evidence map, and evidence-consistency guard output.

## Findings

- fixed: candidate archive review passed with nonce
  guarded-v2-candidate-20260611T1730Z, archive SHA-256
  528608dfed1328a37dd47b6640382ef95a62346a84dd38802c89fceb3bae1840,
  kernel SHA-256
  53c5d28981ab8095df859ffd62674607ff7331aeb81dcc9b36d91535a112ddd8,
  and kernel_2712 size 52352 bytes.
- fixed: control archive review passed with nonce
  guarded-v2-control-20260611T1730Z, archive SHA-256
  a6ccdf39e8f4dfeed52319973b2f48002105588664b1270c1d5514674ccc4856,
  kernel SHA-256
  9316f1dd083d3d141462bfb92d8a6302a257e16b3e13d51771f984b8bfb934a4,
  and kernel_2712 size 50112 bytes.
- fixed: candidate serial marker/nonce freshness was observed with 37 marker
  occurrences and serial_freshness_ok=true.
- fixed: control capture-chain-v4 passed with selected-tree identity, two
  matching 50112-byte TFTP fetches, final pre-restore identity, 39 marker
  occurrences, and serial_freshness_ok=true.
- fixed: the evidence-consistency guard made the blocked aggregate evidence
  agree with candidate/control v4 JSON and task markdown.
- fixed: the lab restored to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  with kernel_2712.img at 104136 bytes before hardware lock release.
- deferred: candidate serial register-vector-looking lines are not accepted,
  because candidate v4 rejected decisive hardware classification.
- deferred: broad MDIO/PHY ownership, PHY absence claims, GPIO32/PHY reset,
  Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicit work.
- removed: no source, docs, or stale evidence was removed.
- not-an-issue: a blocked hardware proof is valid task evidence when the
  blocker is precise and the lab is restored.

## Candidate Result

Candidate capture-chain-v4 is blocked:

~~~text
classification=capture-staging-blocked
decisive_rp1_hardware_classification_allowed=false
selected_tree=dd4ad2732e2dea9b9d86017fed7e52e107f968fc5c5c1f925ef9e40b41a3bdbc
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=52352
expected_fetch_count=2
expected_fetch_byte_match_count=0
expected_fetch_bytes_seen=104136,104136
final_pre_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_marker_occurrences=37
serial_freshness_ok=true
rejection_reasons=[
  tftp-expected-fetch-byte-mismatch,
  final-pre-restore-selected-tree-mismatch,
  final-pre-restore-expected-fetch-byte-mismatch
]
~~~

The candidate serial marker was fresh, but TFTP and final pre-restore identity
joined the restored baseline tree instead of the selected candidate tree.
Therefore no candidate register-vector MAN.DATA values are accepted as
hardware proof.

## Control Result

The paired control capture-chain-v4 is ready:

~~~text
classification=capture-chain-v4-ready
decisive_rp1_hardware_classification_allowed=true
selected_tree=db5f356f5ca4eed9ca21c232ea40a7ff6dfb2f7520337c08695ee0223129238d
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=50112
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=db5f356f5ca4eed9ca21c232ea40a7ff6dfb2f7520337c08695ee0223129238d
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_marker_occurrences=39
serial_freshness_ok=true
~~~

The control proves the no-MDIO/no-Ethernet reporting path, but it cannot make
the blocked candidate decisive.

## Evidence Consistency

The task record, classification JSON, capture summary, evidence map, and
candidate/control v4 JSON agree that this is a blocked proof. The guard output
is retained in
tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/evidence-consistency-guard-output.json.

## Evidence

- Pre-publication triage:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/pre-publication-triage.json.
- Candidate archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/archive-review/candidate-static-review.txt.
- Control archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/archive-review/control-static-review.txt.
- Candidate v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/candidate-run/v4-check.json.
- Control v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/control-run/v4-check.json.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/evidence-map.json.
- Final lab state:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v2-after-evidence-guard/final-lab-status.json.

## Validation

- static archive review: candidate and control archive reviews passed.
- image/archive inspection: candidate/control archive SHA and kernel image
  sizes retained.
- lab-controller API: pre-publication identity, snapshot, candidate/control
  publication, power cycles, serial/TFTP captures, restores, and final
  restored identity retained.
- capture-chain-v4 replay: candidate classified capture-staging-blocked;
  control classified capture-chain-v4-ready.
- evidence-consistency guard/check: task record, classification, capture
  summary, evidence map, and candidate/control v4 JSON agree.
- JSON validation: jq empty on task-owned classification/evidence-map/
  capture-summary JSON.
- diff check: git diff --check.
- docs validation: not required; no docs/src files touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- HardwareTestLock acquisition/release and boot restore state are recorded:
  satisfied.
- Candidate and control evidence each pass or fail capture-chain-v4 with
  explicit JSON retained: satisfied.
- Aggregate classification, capture-summary, evidence-map, guard output, and
  task markdown agree with v4 results: satisfied.
- Candidate blocker names the first failing invariant and does not accept
  serial register values as hardware proof: satisfied.
- Control proves no MDIO target/MAN frame/volatile access through the same
  reporting path: satisfied as a capture-chain-v4-ready no-MDIO/no-Ethernet
  control.
- Accepted proof or blocker committed before proof closeout starts: satisfied
  by this task commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-proof-v2-closeout-20260611 on the
next worker wake if dependencies remain satisfied. Do not repeat this hardware
shape without a bounded staging/power-cycle identity discriminator. Do not
infer broad MDIO/PHY ownership, Ethernet behavior, networking, SSH, Phase
12.2, or a phase transition from this blocker.
