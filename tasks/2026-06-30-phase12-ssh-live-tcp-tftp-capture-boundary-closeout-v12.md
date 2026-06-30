# Phase 12 SSH Live TCP TFTP Capture Boundary Closeout V12

Task id: phase12-ssh-live-tcp-tftp-capture-boundary-closeout-v12-20260630

Status: accepted after commit.

Classification: candidate-kernel-entry-boundary-needs-supervisor-planning.

Evidence level: accepted task/evidence inspection, task-owned JSON evidence,
docs build, and diff checks. No code implementation, hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, broad shell work, or phase transition was
performed.

## Goal

Reconcile the blocked v12 capture-boundary result and choose the next smallest
objective step without allowing a blind rerun or packet-I/O drift.

## Scope Performed

- Promoted this queued closeout only after the accepted v12 Pi 5 preflight
  selected this exact task.
- Inspected the accepted v12 TFTP capture-boundary reconciliation and the v12
  Pi 5 runtime-marker preflight task/evidence maps.
- Preserved the repaired TFTP helper contract as accepted evidence: the v12
  hardware run captured selected candidate TFTP fetches in the same
  pre-restore window.
- Rejected packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, and phase
  transition because candidate-capture-ready was not accepted.
- Stopped at supervisor planning because no queued successor is mechanically
  unblocked by the first missing fact.

## Terminal Classification

candidate-kernel-entry-boundary-needs-supervisor-planning.

The first missing fact is no longer TFTP capture. The accepted v12 hardware
run retained a repaired same-cursor pre-restore TFTP delta with 13 parsed
events, including two selected da591740/kernel_2712.img serves at 152,176
bytes. Final pre-restore identity remained on candidate tree
400bf7c5f4ae49ca484322499c7d2ec06cd7f8f57961241d705f01b823035ca9, and the
lab restored to the selected-control tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

candidate-capture-ready remains rejected because the retained serial window
had firmware NETWORK output but no TALOS: kernel_main, no nonce-bearing
TALOS: ssh-service-smoltcp-runtime-route-start, and no nonce-bearing
TALOS: ssh-service-smoltcp-runtime-ready marker. Packet-I/O and
OpenSSH/generated-root retry remain blocked.

No queued successor is mechanically unblocked by this closeout:

- phase12-ssh-live-tcp-pi5-packet-io-discriminator-20260629 requires a
  predecessor that accepted candidate-capture-ready or
  candidate-entry-and-runtime-ready; v12 accepted blocked-candidate-kernel-not-
  starting instead.
- phase12-ssh-live-tcp-candidate-runtime-marker-control-contract-20260630
  requires the earlier source reconciliation to accept
  runtime-marker-control-contract-required and select that exact contract; the
  accepted source reconciliation instead selected the v11 candidate preflight
  with candidate-runtime-marker-route-repair-ready.
- Other queued live TCP/RP1 tasks have explicit predecessor classifications or
  selected_next_task dependencies that are not satisfied by the v12 closeout.

planningNeeded: true.

planningReason: v12 proved selected TFTP fetch identity under the repaired
capture contract but did not reach Talos kernel/runtime markers; supervisor
must plan the next bounded source/helper/control discriminator before any
rerun, packet-I/O, OpenSSH/generated-root retry, or phase transition.

selected_next_task: null.

## Findings

- fixed: the v12 closeout preserves the TFTP capture helper repair as accepted;
  selected candidate fetch identity is no longer the first missing fact.
- blocked: candidate-capture-ready remains rejected because no kernel_main,
  route-start, or runtime-ready marker appeared after selected fetch proof.
- deferred: the next repair/discriminator is not worker-selectable from the
  existing queue; supervisor planning is required.
- removed: packet-I/O/OpenSSH/generated-root retry as a permissible successor
  from any blocked-candidate-kernel-not-starting v12 result.
- not-an-issue: the selected-control restore proof is sufficient for the
  closeout; no additional hardware action is needed.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-closeout-v12/evidence-map.json.
- Accepted v12 preflight task:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12.md.
- Accepted v12 preflight evidence:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/evidence-map.json,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/candidate-run/tftp-delta-stable-pre-restore.json,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/candidate-run/route-start-runtime-ready-marker-check.json.
- Accepted v12 reconciliation task:
  tasks/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12.md.
- Accepted v12 reconciliation evidence:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-tftp-capture-boundary-reconciliation-v12/tftp-capture-boundary-reconciliation-v12-20260630T092407Z/evidence-map.json.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, key material, session material, boot artifact bytes, private
user data, stable secret-derived identifiers, or unnecessary hardware data.
It references task-owned hardware evidence retained by the accepted predecessor.

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
