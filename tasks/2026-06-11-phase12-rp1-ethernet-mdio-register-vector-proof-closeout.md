# Phase 12 RP1 Ethernet MDIO Register Vector Proof Closeout

Task id: phase12-rp1-ethernet-mdio-register-vector-proof-closeout-20260611

Status: accepted

Classification: mdio-register-vector-retry-evidence-inconsistent-frontier-blocked

Evidence level: static inspection of the post-recovery proof task record,
task-owned JSON evidence, Phase 12 docs, roadmap, and git history.

## Goal

Close out the post-recovery MDIO register-vector proof retry without expanding
beyond the selected register-vector boundary.

## Scope Performed

- Inspected the accepted post-recovery proof retry task record and committed
  evidence at commit 09cdc499bc578935c05b72b18082f5ca6a2c0fb9.
- Inspected task-owned classification, capture-summary, evidence-map, and
  candidate/control capture-chain-v4 JSON.
- Reconciled the proof record against the JSON evidence without accepting broad
  MDIO/PHY ownership, Ethernet behavior, packet I/O, networking, SSH, Phase
  12.2, or a phase transition.
- Recorded a precise closeout classification and evidence map.

## Findings

- fixed: the closeout identified a material contradiction in the accepted retry
  record. The task markdown says the candidate capture-chain-v4 proof was ready
  with matching selected-tree identity, TFTP bytes, run-unique serial markers,
  and final identity.
- fixed: the committed candidate JSON evidence says the opposite:
  classification capture-staging-blocked,
  decisive_rp1_hardware_classification_allowed=false, zero marker occurrences,
  zero expected-fetch byte matches, final pre-restore tree restored to the
  baseline instead of the selected tree, and final expected-fetch bytes
  mismatched.
- fixed: the paired control JSON evidence is internally ready, but it cannot
  rescue the candidate evidence join.
- deferred: the candidate serial report still contains register-vector-looking
  lines, but the capture-chain-v4 evidence does not prove they came from the
  selected post-power candidate transaction. Treating them as accepted hardware
  data would violate the proof boundary.
- deferred: broad MDIO/PHY ownership, PHY absence claims, GPIO32/PHY reset,
  Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicit work.
- removed: no source, docs, or stale evidence was removed.
- not-an-issue: Phase 12 docs and roadmap did not yet claim the inconsistent
  post-recovery register-vector proof as an accepted frontier, so no docs/src
  wording update was required in this closeout.

## Evidence Reconciliation

The previous task record asserted:

~~~text
classification=capture-chain-v4-ready
decisive_rp1_hardware_classification_allowed=true
expected_fetch_byte_match_count=2
final_pre_restore_tree=b901be8a925e644ffc3f932d258ed7413522f95a8a36a3df7c8ae5182ee745fc
serial_marker_occurrences=15
serial_freshness_ok=true
~~~

The committed candidate v4 JSON records:

~~~text
classification=capture-staging-blocked
decisive_rp1_hardware_classification_allowed=false
rejection_reasons=[
  run-unique-serial-freshness-not-proven,
  run-unique-capture-nonce-present-before-power,
  run-unique-capture-nonce-not-present-after-power,
  required-marker-not-present-after-power,
  tftp-expected-fetch-byte-mismatch,
  final-pre-restore-selected-tree-mismatch,
  final-pre-restore-expected-fetch-byte-mismatch
]
expected_fetch_byte_match_count=0
final_pre_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_marker_occurrences=0
serial_freshness_ok=false
~~~

The candidate capture-invariant summary preserves the same blocker: preflight
identity matched the selected archive, but the observed post-power/TFTP/final
identity did not. The TFTP delta served the same path with baseline-size
104136-byte images instead of the selected 52344-byte image, and final
pre-restore identity was the baseline tree. Therefore the register-vector
serial report is not accepted as a decisive current-run hardware proof.

The paired control v4 JSON remains ready:

~~~text
classification=capture-chain-v4-ready
decisive_rp1_hardware_classification_allowed=true
expected_fetch_byte_match_count=2
serial_marker_occurrences=20
serial_freshness_ok=true
~~~

This closeout accepts only the reconciliation result: the post-recovery proof
retry is blocked by inconsistent candidate evidence and the prior acceptance
record overclaims what the task-owned JSON supports.

## Same-Shaped Retry Policy

Same-shaped register-vector candidate/control retries are not closed as
successful hardware evidence. They are blocked until supervisor planning
defines a bounded recovery or retry task with explicit acceptance criteria for
the publication/restore/capture mismatch, or a qualitatively different
discriminator. Do not repeat the same candidate/control shape solely from this
closeout.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-proof-closeout/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-proof-closeout/evidence-map.json.
- Inspected proof retry task record:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery.md.
- Inspected proof retry classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/classification.json.
- Inspected proof retry capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/capture-summary.json.
- Inspected proof retry candidate v4 check:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/candidate-run/v4-check.json.
- Inspected proof retry control v4 check:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery/control-run/v4-check.json.
- Inspected docs:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Inspected git history: git log --oneline -6 and git show for commit
  09cdc499bc578935c05b72b18082f5ca6a2c0fb9.

## Validation

- static inspection: proof retry task record, classification/capture/evidence
  JSON, candidate/control v4 JSON, Phase 12 docs, roadmap, and git history.
- JSON validation: jq empty on task-owned classification and evidence-map JSON.
- diff check: git diff --check.
- docs validation: not required; no docs/src files touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles proof/blocker evidence without expanding beyond the
  selected register-vector boundary: satisfied.
- Same-shaped retry policy is explicit: satisfied; blocked pending supervisor
  planning for a bounded recovery/retry or a qualitatively different
  discriminator.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with concrete blocker reason: satisfied by
  setting planningNeeded for supervisor planning.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  this task commit.

## Next Action

Set planningNeeded=true for supervisor planning. The blocker is the mismatch
between the accepted retry task record and the committed candidate JSON
evidence: candidate capture-chain-v4 is blocked by stale/pre-power serial
nonce evidence, TFTP byte mismatch, and final selected-tree mismatch. Do not
promote Phase 12.2, Ethernet behavior, networking, SSH, or any phase
transition from this closeout.
