# Phase 12 RP1 Ethernet MDIO Register Vector Pi 5 Proof V3 After Staging Identity

Task id: phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity-20260611

Status: blocked

Classification: mdio-register-vector-v3-capture-staging-blocker

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable TFTP delta,
capture-chain-v4 replay, staging identity gate, and restore evidence.

## Goal

Retry the guarded MDIO register-vector Pi 5 proof only after the staging
sentinel accepted selected-tree identity durability, and classify the result
without broadening beyond selected Clause 22 MAN.DATA evidence.

## Scope Performed

- Promoted the queued v3 proof after the staging sentinel closeout accepted
  selected-tree identity durability.
- Built candidate and paired no-MDIO/no-Ethernet control archives with
  run-unique nonces.
- Ran static archive reviews before publication.
- Acquired hardwareTestLock before publication and power actions.
- Captured candidate and control hardware runs, including reruns after the
  initial candidate/control captures exposed stale or mismatched staging
  evidence.
- Restored the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: candidate rerun2 archive review passed with archive SHA-256
  f98b1944b1829f34fe465a62aeb1eb706d9e69387ec649a6d58d5606a5fdfcf8,
  kernel SHA-256 4f330a963eb1d9a02b0b9e64ec1b7b1e77b2c9ab0c02e222c5b413565d254906,
  and kernel_2712 size 52,352 bytes.
- fixed: control rerun archive review passed with archive SHA-256
  7fccea74636074b9667a9d4795e6ef4add3c09766a6820bbc46f2d884efac555,
  kernel SHA-256 8db26d10e5c6a508e749020952994d2b0b12232d404f9216f2dacaa6ed474bd2,
  and kernel_2712 size 50,120 bytes.
- fixed: candidate rerun2 serial output was fresh and marker-visible, with 16
  required marker occurrences and 0 pre-power nonce occurrences.
- fixed: control rerun serial output was fresh and marker-visible, with 19
  required marker occurrences and 0 pre-power nonce occurrences.
- fixed: candidate rerun2 and control rerun both restored to the baseline tree
  after capture.
- deferred: selected-tree staging identity failed for both rerun evidence sets,
  so no register-vector values are accepted from this task.
- removed: no source or docs code was removed.
- not-an-issue: rejected hardware proof evidence is valid progress because it
  names the first failing invariant and keeps the lab restored.

## Candidate Result

The authoritative candidate rerun2 capture-chain-v4 result is blocked:

~~~text
classification=capture-staging-blocked
staging-identity-gate=selected-tree-identity-blocked
first-failing-invariant=expected-fetch-byte-mismatch
selected_tree=e81550ef7ba1252f10763a055d89c1f72b9cbc0b85bb60e512d0b7890bf0c724
expected_fetch_bytes=52352
observed_tftp_fetch_bytes=104136,104136
expected_fetch_byte_match_count=0
final_pre_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_freshness_ok=true
required_marker_occurrences=16
pre_power_nonce_occurrences=0
~~~

No candidate register-vector MAN.DATA values are accepted because selected-tree
staging identity failed before the evidence-consistency boundary.

## Control Result

The authoritative control rerun capture-chain-v4 result is blocked:

~~~text
classification=capture-staging-blocked
staging-identity-gate=selected-tree-identity-blocked
first-failing-invariant=expected-fetch-byte-mismatch
selected_tree=aed051ee00bc30a808a4ad8b84b983d4c06924971b3305a3fd7c0cae905eb93d
expected_fetch_bytes=50120
observed_tftp_fetch_bytes=104136,104136
expected_fetch_byte_match_count=0
final_pre_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_freshness_ok=true
required_marker_occurrences=19
pre_power_nonce_occurrences=0
~~~

The control proves the serial reporting path was fresh, but it also confirms the
selected-tree publication-to-power identity blocker for this proof shape.

## Boundary

Accepted: precise blocked evidence for the guarded register-vector v3 proof.

Not accepted: register-vector MAN.DATA values, PHY absence, broad MDIO/PHY
ownership, NCR write permission or execution, GPIO32/PHY reset ownership,
Ethernet behavior, interrupts, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Candidate rerun2 archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/archive-review/candidate-rerun2-static-review.txt.
- Control rerun archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/archive-review/control-rerun-static-review.txt.
- Candidate rerun2 v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/candidate-rerun2/v4-check.json.
- Control rerun v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/control-rerun/v4-check.json.
- Candidate rerun2 staging identity gate:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/candidate-rerun2/staging-identity-gate-output.json.
- Control rerun staging identity gate:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/control-rerun/staging-identity-gate-output.json.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/evidence-map.json.

## Validation

- static archive review: candidate/control rerun archive reviews passed.
- image/archive inspection: candidate/control archive SHA, kernel SHA, and
  kernel image sizes retained.
- lab-controller API: publication, power cycles, TFTP deltas, final identity,
  restores, and final baseline status retained.
- serial hardware boot/output: candidate/control rerun serial markers were
  present after power and absent before power.
- stable TFTP delta: candidate/control reruns observed stable deltas, but each
  fetched baseline 104,136-byte kernel_2712.img instead of selected candidate
  or control bytes.
- capture-chain-v4 replay: candidate/control reruns classified
  capture-staging-blocked.
- staging identity gate: candidate/control reruns classified
  selected-tree-identity-blocked.
- evidence-consistency guard/check: task record, classification, capture
  summary, evidence map, and candidate/control v4 JSON agree.
- JSON validation: jq empty on task-owned classification, capture-summary,
  evidence-map, candidate/control gate outputs, and candidate/control v4 checks.
- diff check: git diff --check.
- docs validation: not required; docs/src files were not touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- HardwareTestLock acquisition/release and boot restore state recorded:
  satisfied.
- Candidate and control pass staging identity gate or candidate is classified
  as a precise blocker: satisfied as precise blocked evidence; neither side is
  accepted as register-vector proof.
- Aggregate classification, capture summary, evidence map, guard output,
  candidate/control identity evidence, and task markdown agree: satisfied.
- Accepted register-vector result stays limited and does not claim broad
  ownership: satisfied by accepting no register-vector result.
- Accepted proof or precise blocker committed before proof closeout starts:
  satisfied by this task commit.

## Next Action

Supervisor planning required for a register-vector v3 proof closeout. Do not
accept register-vector values or repeat this same hardware proof shape without
resolving the selected-tree publication-to-power identity blocker.

## Supervisor Intervention Discriminator

After the supervisor paused same-shaped register-vector retries, this task ran
a no-MDIO selected-tree publication-to-power discriminator.

Classification: tftp-root-diverges-from-lab-api-selected-tree.

Findings:

- fixed: the required first-principles problem statement, selected-tree
  invariant, contradiction map, unproven assumptions, two approaches, smallest
  discriminator, and quarantine plan are recorded in
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/intervention-discriminator/required-before-resume.md.
- fixed: pre-power lab API status and boot files selected the no-MDIO sentinel
  candidate tree
  a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0 with
  47,832-byte `kernel_2712.img` and
  `da591740/kernel_2712.img`.
- fixed: the single power-cycle TFTP delta later showed dnsmasq served
  104,136-byte baseline `da591740/kernel_2712.img` twice during that same
  selected-tree run.
- fixed: final pre-restore lab API status still reported the selected no-MDIO
  tree, so the mismatch is between the lab API-visible boot-root
  publication/reporting path and the actual dnsmasq-served TFTP root/cache.
- fixed: the lab was restored to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 after the
  discriminator.
- deferred: supervisor planning is required before any helper quarantine,
  lab-controller repair, register-vector retry, broad MDIO/PHY work, or proof
  closeout.

Additional evidence:

- Intervention classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/intervention-discriminator/classification.json.
- Intervention evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/intervention-discriminator/evidence-map.json.
- Late TFTP delta:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/intervention-discriminator/tftp-delta-after-power-late.json.

Next action: supervisor planning required. Quarantine the prior staging
sentinel selected-tree durability claim as insufficient for register-vector
retries until the lab API/dnsmasq TFTP root mismatch is explained and fixed.
