# Phase 12 RP1 Ethernet MDIO Register Vector Evidence Consistency Guard Core

Task id: phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core-20260611

Status: accepted

Classification: mdio-register-vector-evidence-consistency-guard-accepted

Evidence level: local/static evidence-consistency guard over retained task
records and task-owned JSON evidence.

## Goal

Make the post-recovery register-vector proof contradiction mechanically
rejectable before any same-shaped Pi 5 hardware retry.

## Scope Performed

- Inspected the committed contradictory register-vector retry evidence and the
  closeout blocker.
- Added scripts/rpi5-evidence-consistency-guard.sh, a local/static guard that
  treats candidate/control capture-chain-v4 JSON as authoritative for capture
  readiness.
- Checked the contradictory register-vector retry as a negative control.
- Checked the accepted GEM MID decode discriminator V2 Pi 5 proof as a
  retained positive capture-chain-v4 control.
- Recorded guard outputs, classification JSON, evidence map, and validation
  results.

No hardware was run, no boot archive was published, and no runtime Ethernet,
MDIO, PHY, MMIO, DMA, descriptor-ring, interrupt, networking, sockets, SSH, or
userspace code was changed.

## Findings

- fixed: the acceptance gap is now explicit. The retry task markdown and
  aggregate accepted claims said the candidate/control capture-chain-v4 proof
  was ready, while the authoritative candidate v4 JSON classified the
  candidate as capture-staging-blocked.
- fixed: the guard rejects that contradictory shape with
  aggregate-claims-candidate-ready-overclaim and
  task-markdown-candidate-ready-overclaim.
- fixed: the guard accepts a retained positive capture-chain-v4 proof where
  candidate/control v4 JSON, classification JSON, capture summary, evidence
  map, and task markdown agree.
- fixed: future Pi 5 proof validation now has a concrete guard command:
  run scripts/rpi5-evidence-consistency-guard.sh --evidence-dir DIR
  --task-record TASK.md and require a zero exit before accepting any hardware
  proof whose task claims candidate/control capture-chain-v4 readiness.
- deferred: broader MDIO/PHY ownership, PHY reset/GPIO32 action, Ethernet
  behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain future explicit work.
- removed: no stale source, docs, or evidence was removed.
- not-an-issue: the paired control from the contradictory retry is internally
  capture-chain-v4-ready, but it cannot make a blocked candidate acceptable.

## Guard Rule

The task-owned candidate/control v4 JSON is authoritative for the capture
identity/freshness gate. Task markdown, aggregate classification JSON, capture
summary JSON, and evidence map accepted claims must not claim
capture-chain-v4-ready, matching selected-tree identity, matching TFTP bytes,
run-unique serial freshness, or final pre-restore identity when the relevant
v4 JSON blocks those claims.

Future Pi 5 proof tasks must include the candidate/control v4 JSON, aggregate
classification, capture summary, evidence map, task record, and guard output
as acceptance evidence. If the guard exits nonzero, the proof is blocked until
the disagreement is fixed or the supervisor defines different acceptance
criteria.

## Negative Control

Input:

- Task record:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery.md.
- Evidence directory:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-retry-after-staging-recovery.

Guard result:

~~~text
exit=1
classification=evidence-consistency-blocked
consistent=false
rejection_reasons=[
  aggregate-claims-candidate-ready-overclaim,
  task-markdown-candidate-ready-overclaim
]
candidate_v4_classification=capture-staging-blocked
candidate_v4_allowed=false
control_v4_classification=capture-chain-v4-ready
control_v4_allowed=true
~~~

This satisfies the negative-control requirement: the contradictory retry is
mechanically rejected and the register-vector serial values remain unaccepted
as hardware proof.

## Positive Control

Input:

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof.md.
- Evidence directory:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof.

Guard result:

~~~text
exit=0
classification=evidence-consistency-ready
consistent=true
rejection_reasons=[]
candidate_v4_classification=capture-chain-v4-ready
candidate_v4_allowed=true
control_v4_classification=capture-chain-v4-ready
control_v4_allowed=true
~~~

This satisfies the positive-control requirement: the guard is not hardcoded to
the failing register-vector task and accepts a retained valid capture-chain-v4
proof shape.

## Evidence

- Implementation:
  scripts/rpi5-evidence-consistency-guard.sh.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core/evidence-map.json.
- Negative-control guard output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core/negative-contradictory-register-vector-guard-output.json.
- Positive-control guard output:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-core/positive-retained-capture-chain-v4-guard-output.json.

## Validation

- local/static guard negative control: contradictory register-vector retry
  exited 1 and classified evidence-consistency-blocked.
- local/static guard positive control: retained accepted capture-chain-v4 proof
  exited 0 and classified evidence-consistency-ready.
- shell syntax: sh -n scripts/rpi5-evidence-consistency-guard.sh.
- JSON validation: jq empty on task-owned classification, evidence map, and
  guard outputs.
- capture-chain helper validation: not required; no capture-chain helper was
  touched.
- Rust validation: not required; no Rust source or tests were touched.
- docs validation: not required; no docs/src files were touched.
- diff check: git diff --check.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Durable local/static guard exists and rejects the contradictory
  register-vector retry evidence shape: satisfied.
- Guard accepts one retained positive capture-chain-v4 proof shape: satisfied.
- Validation requirements for future Pi 5 proof tasks explicitly require
  candidate/control v4 JSON, aggregate classification, capture summary,
  evidence map, task markdown, and guard output to agree before acceptance:
  satisfied.
- Accepted work is committed before any hardware retry or closeout starts:
  satisfied by this task commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-evidence-consistency-guard-closeout-20260611
on the next worker wake if supervisorIntervention remains inactive. Do not run
hardware, publish boot archives, acquire hardwareTestLock, retry the
register-vector proof, broaden MDIO/PHY ownership, start Ethernet behavior,
networking, SSH, Phase 12.2, or a phase transition from this guard core.
