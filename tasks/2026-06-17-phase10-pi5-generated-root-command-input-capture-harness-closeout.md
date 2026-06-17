# Phase 10 Pi 5 Generated-Root Command-Input Capture Harness Closeout

Task id: phase10-pi5-generated-root-command-input-capture-harness-closeout-20260617

Status: accepted

Classification:
pi5-generated-root-command-input-capture-harness-blocked-serial-observe-saturated

Evidence level: static/task evidence inspection, task-owned JSON evidence,
diff checks, and docs build. No hardware run, boot archive publication, lab
mutation, hardwareTestLock acquisition, power-cycle, storage work, networking,
SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Reconcile the accepted capture-harness core and its serialized Pi 5 proof
result without converting saturated serial retention into generated-root
command-input acceptance.

## Result

Generated-root command input remains unaccepted on Pi 5. The capture-harness
core fixed the local proof contract and selected a two-step, command-indexed
hardware proof. That hardware proof published the selected archive and retained
same-power-cycle TFTP evidence for the expected kernel and initramfs, while a
direct-read diagnostic confirmed firmware-initramfs generated-root prompt
readiness. The accepted command-input evidence contract still failed at the
first invariant: the saved /serial/observe cursor was already at the 4194304
byte saturation boundary and returned no bytes.

Direct /serial/read output remains diagnostic only. It cannot replace retained
command-indexed /serial/observe evidence for the command text, manifest output,
dispatch status, and next ready marker.

## Findings

- fixed: the local/static harness core made the command-input proof contract
  source-backed and command-indexed: command 0 rootinfo prelude, then command 1
  cat /generated/manifest.txt acceptance.
- fixed: the Pi 5 proof retained candidate archive identity, expected TFTP
  fetches, serial observe saturation, direct-read diagnostic prompt readiness,
  baseline direct-read control, final pre-restore identity, and restore proof.
- blocked: command-input acceptance remains blocked by serial observe/cursor
  saturation. The required command text and generated-root manifest output were
  not retained through /serial/observe.
- deferred: resolving serial retention/cursor saturation or replanning the
  accepted command-input evidence contract requires supervisor planning before
  another hardware attempt.
- rejected: generated-root command-input success, /serial/write or direct-read
  output alone as command-input proof, persistence, writable filesystem,
  SD/USB/block storage, networking, SSH, Phase 11/12 expansion, and phase
  transition.

## Evidence

- Capture-harness core:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-core.md.
- Capture-harness core classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-core/classification.json.
- Capture-harness Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof.md.
- Capture-harness Pi 5 proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof/classification.json.
- Capture-harness Pi 5 proof evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof/evidence-map.json.
- This closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-closeout/classification.json.
- This closeout evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Closeout classification matches retained core/proof evidence: satisfied with
  pi5-generated-root-command-input-capture-harness-blocked-serial-observe-saturated.
- Accepted generated-root command-input capability, blocked invariant, or
  explicit pause is unambiguous: satisfied; command input remains blocked on
  serial observe/cursor saturation.
- Rejected claims include persistence, writable storage, networking, SSH, Phase
  11/12 expansion, and phase transition: satisfied.
- Task record and task-owned JSON record findings with disposition: satisfied.
- Next task selection is dependency-gated and does not require supervisor
  judgment if objective: no next worker task selected; supervisor planning is
  required because the remaining blocker is an evidence-contract/lab retention
  question, not a mechanically unblocked feature task.

## Next Action

Supervisor planning is required before another generated-root command-input
hardware attempt, serial-retention/capture-harness change, evidence-contract
change, persistence, storage, networking, SSH, Phase 11/12 expansion, or phase
transition. Do not promote the Milestone 10.3 closeout from this task unless the
supervisor explicitly decides that the serial observe/cursor saturation blocker
is an acceptable milestone pause boundary.
