# Phase 12 SSH Live TCP Kernel Entry Boundary Closeout V13

Task id: phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v13-20260630

Status: accepted after commit.

Classification: candidate-entry-boundary-needs-supervisor-planning.

Evidence level: accepted task/evidence inspection, task-owned JSON evidence,
docs build, and diff checks. No code implementation, hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the accepted v13 control-entry result without treating it as a live
TCP candidate success or allowing a blind rerun, packet-I/O drift, or
OpenSSH/generated-root retry.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13-20260630
  accepted control-entry-passes and selected this exact task.
- Inspected the accepted v12 closeout, v13 source/artifact reconciliation, v13
  control contract, and v13 Pi 5 control discriminator records and task-owned
  JSON evidence.
- Preserved the v13 control result: current-tree production-timer selected-path
  bytes can be served by TFTP and reach a downstream Talos PASS marker on the
  Pi 5 under the accepted capture/restore contract.
- Preserved the candidate boundary: the live TCP candidate still has selected
  fetch proof but no kernel/runtime entry markers and no accepted
  candidate-capture-ready result.
- Stopped at supervisor planning because no existing queued successor is
  mechanically unblocked by control-entry-passes.

## Terminal Classification

candidate-entry-boundary-needs-supervisor-planning.

The first missing fact is candidate-specific, not generic selected-path entry.
The v12 hardware run proved selected candidate da591740/kernel_2712.img fetch
identity under the repaired TFTP capture contract, but its serial window had no
TALOS: kernel_main, no nonce-bearing route-start marker, and no nonce-bearing
runtime-ready marker. The v13 static source/artifact reconciliation found no
bounded source, linker, target, route, header, or archive defect that explains
the missing markers.

The v13 control discriminator then proved the current-tree production-timer
selected-path control can enter far enough to emit
rpi5-production-timer-preemption: PASS after selected TFTP fetch. That result
is useful because it rejects a blanket lab selected-path entry failure. It does
not accept the live TCP candidate, because the control uses a different image
and the predecessor contract made absent rust_entry, boot-info-parsed,
target-init, exceptions-ready, and kernel_main phase lines metadata-only only
when the downstream control PASS marker is present.

No queued successor is mechanically unblocked:

- phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v13-20260630
  requires the v13 source/artifact reconciliation to accept
  candidate-entry-source-repair-ready and select that task; it instead accepted
  candidate-entry-control-contract-required.
- phase12-ssh-live-tcp-pi5-packet-io-discriminator-v13-20260630 requires a
  predecessor that accepted candidate-capture-ready and selected packet-I/O;
  no v13 predecessor did so.
- OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, and phase transition remain blocked because
  candidate-capture-ready remains rejected.

selected_next_task: null.

planningNeeded: true.

planningReason: v13 proved the selected-path lab/control can enter Talos and
reach a downstream current-tree marker, while the live TCP candidate remains
selected-fetch-with-no-kernel-entry-marker and no bounded source/archive defect
has been found. Supervisor must plan a new bounded candidate-specific entry
discriminator or source repair before any rerun, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility/service readiness
claim, broad shell work, or phase transition.

## Findings

- fixed: current-tree production-timer control proves selected-path
  entry/capture can reach a downstream Talos PASS marker in the lab.
- deferred: the control PASS does not explain the live TCP candidate's missing
  kernel/runtime entry markers.
- blocked: no queued successor is mechanically unblocked by
  control-entry-passes because candidate-capture-ready was not accepted.
- removed: packet-I/O/OpenSSH/generated-root retry as immediate successors from
  this closeout.
- not-an-issue: hardwareTestLock can remain unlocked because this closeout is
  no-hardware and relies on accepted predecessor evidence.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v13/evidence-map.json.
- Accepted v12 closeout:
  tasks/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-closeout-v12.md.
- Accepted v13 source/artifact reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13.md.
- Accepted v13 control contract:
  tasks/2026-06-30-phase12-ssh-live-tcp-candidate-entry-control-contract-v13.md.
- Accepted v13 Pi 5 control discriminator:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13.md.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data. It
references task-owned hardware evidence retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on referenced JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
