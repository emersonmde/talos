# Phase 10 Pi 5 Generated-Root Command-Input Direct-Read Source Contract

Task id: phase10-pi5-generated-root-command-input-direct-read-source-contract-20260617

Status: accepted

Classification:
direct-read-after-saturated-cursor-source-contract-selected

Evidence level: static/task/doc/source evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No runtime code change, hardware run,
boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, persistence, storage-driver work, networking, SSH, Phase 11/12
expansion, or phase transition was performed.

## Goal

Select a bounded command-input evidence contract that can prove the Pi 5
generated-root manifest command after the prior /serial/observe proof was
blocked by the retained serial cursor saturating at 4194304 bytes.

## Source Contract

The selected follow-up contract is
direct-read-after-saturated-cursor-command-input-v1.

The prior capture-harness proof could not evaluate command input because
/serial/observe from the saved cursor returned zero bytes at the serial
retention boundary. The replacement signal is not direct-read output by itself;
it is a command-indexed direct /serial/read sequence whose windows are bounded
by source facts:

1. selected-tree identity, expected TFTP fetches, final identity, and restore
   proof are retained for the same Pi 5 boot;
2. direct-read readiness for command 0 retains
   source=firmware-initramfs reason=valid-artifact, the generated-root proof
   ready command=0 marker, and a visible talos> prompt;
3. an immediate pre-write freshness read after that prompt records no pending
   command response before rootinfo is written;
4. rootinfo is written with append_newline=true and the following direct-read
   command 0 window retains rootinfo, dispatch command=0 status=handled,
   responses=1, source=firmware-initramfs reason=valid-artifact, and ready
   command=1;
5. an immediate pre-write freshness read after ready command=1 records no
   pending command 1 response before cat /generated/manifest.txt is written;
6. cat /generated/manifest.txt is written with append_newline=true and the
   following direct-read command 1 window retains the command text,
   Talos generated-root external artifact A, dispatch command=1 status=handled,
   responses=1, and ready command=2, ready-for-next prompt=true, or final PASS.

This keeps the user-visible feature unchanged: Pi 5 command input must read
/generated/manifest.txt from the firmware-initramfs generated-root artifact. It
does not accept a fake/kernel-backed command expansion, prompt visibility alone,
/serial/write byte acceptance alone, or direct-read output that is not tied to
the command index and same boot.

## Findings

- fixed: selected a source-backed replacement for the missing saturated
  /serial/observe signal. Direct /serial/read may replace that missing signal
  only when bounded by per-command pre-write freshness and retained
  command-indexed response fragments.
- fixed: preserved the original generated-root feature boundary: command 1 must
  read /generated/manifest.txt and retain Talos generated-root external
  artifact A after same-boot source=firmware-initramfs reason=valid-artifact.
- fixed: carried forward the TFTP/final-identity/restore requirements so a
  command-input proof cannot pass on stale or restored baseline output.
- not-an-issue: lab service mutation is not required for this contract because
  the existing /serial/read endpoint consumes newly available device bytes and
  reports the cursor after appending them to the retained log.
- deferred: helper implementation and Pi 5 publication/power-cycle are owned by
  dependency-gated follow-up tasks.
- rejected: direct-read-only output as a substitute for command-indexed proof,
  prompt visibility alone, /serial/write byte acceptance alone, persistence,
  writable filesystem, SD/USB/block storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Evidence

- Accepted capture-harness closeout:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-closeout.md.
- Capture-harness closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-closeout/classification.json.
- Saturated /serial/observe proof:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof.md.
- Lab-controller serial endpoint contract:
  docs/src/project/lab-controller.md.
- Generated-root transport contract update:
  docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-source-contract/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-source-contract/evidence-map.json.

## Validation

- static/task/doc/source evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- The selected contract explains why prior /serial/observe cursor saturation
  blocked acceptance and what evidence replaces that missing signal: satisfied.
- The contract preserves Pi 5 firmware-initramfs generated-root command input
  for cat /generated/manifest.txt: satisfied.
- Direct-read-after-saturated-cursor acceptance requirements are explicit:
  satisfied with same-boot source, command-indexed rootinfo and manifest
  windows, retained command text, artifact output, handled dispatch,
  post-command readiness, stable TFTP, final identity, and restore proof.
- Lab-service mutation blocker is avoided: satisfied by selecting a bounded
  contract over the existing /serial/read endpoint.
- selected_next_task is the direct-read harness core: satisfied.

## Next Action

Promote
phase10-pi5-generated-root-command-input-direct-read-harness-core-20260617 on
the next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not acquire hardwareTestLock or run Pi 5 hardware from this
source-contract task.
