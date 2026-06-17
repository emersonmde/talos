# Phase 10 Pi 5 Serial Command 0 Prelude Guard Core

Task id: phase10-pi5-serial-command0-prelude-guard-core-20260617

Status: accepted

Classification:
serial-command0-prelude-guard-core-accepted-hardware-proof-selected

Evidence level: static/source/task evidence inspection, shell syntax check,
task-owned local/static validator output, task-owned JSON evidence, docs build,
and diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, persistence, storage work, networking, SSH, Phase
11/12 expansion, or phase transition was performed by this task.

## Goal

Implement the accepted command 0 prelude guard as a local/static helper contract
before any hardware retry.

## Implementation

The proof-review helper now treats command input as an ordered transaction, not
as a bag of serial fragments. For command 0, the optional evidence validator
requires:

- same-boot source=firmware-initramfs reason=valid-artifact, selected-tree,
  stable TFTP, final identity, and restore fields;
- ready command=0 and a visible prompt before the write;
- fresh pre-write read for command 0 that has not already captured rootinfo,
  the equivalent command=0 line record, command 0 dispatch, or the generated
  manifest output;
- /serial/write of rootinfo with newline;
- command 0 direct-read evidence ordered as rootinfo text or
  line command=0 hex=726f6f74696e666f, generated-root source/reason response,
  dispatch command=0 status=handled responses=1, then ready command=1;
- ready command=1 must be retained before any later command=1 timeout evidence
  can be treated as part of the proof.

Command 1 evidence remains checked for the generated-root manifest command, but
now accepts either the literal command text or the target proof line-hex record
as the line evidence. The command 1 response, dispatch, response count, and
ready/PASS boundary are also ordered.

## Local Static Evidence

The task-owned positive fixture
tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/local-static-valid-direct-read-evidence.json
passes the helper and produces
tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/local-static-valid-review.json.

The accepted blocked direct-read evidence remains rejected by the helper. The
negative-control result is recorded in
tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/blocked-direct-read-rejection.json.

## Findings

- fixed: the helper now requires command 0 line/source/dispatch/ready evidence
  in order, with line-hex accepted as the target proof's equivalent command
  record.
- fixed: stale pre-write windows that already contain the command line marker,
  dispatch, or generated-root manifest output remain rejected.
- fixed: the helper output names command0-write-to-next-ready-guard-v1 and
  selects the hardware proof follow-up.
- deferred: Pi 5 hardware validation remains gated behind
  phase10-pi5-serial-command0-prelude-pi5-proof-20260617.
- rejected: prompt-only evidence, /serial/write-only evidence, unordered
  fragment presence, same-shaped timing retries, persistence, storage drivers,
  networking, SSH, Phase 11/12 expansion, and phase transition.
- not-an-issue: no kernel command-loop or Pi boot-path source change was
  required for this local/static guard.

## Evidence

- Accepted source contract:
  tasks/2026-06-17-phase10-pi5-serial-command0-prelude-source-contract.md.
- Updated helper:
  scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh.
- Positive local/static validator fixture and output:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/local-static-valid-direct-read-evidence.json
  and
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/local-static-valid-review.json.
- Negative-control rejection:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/blocked-direct-read-rejection.json.
- Classification and evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/classification.json
  and
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/evidence-map.json.

## Acceptance Check

- Implementation matches the accepted source contract exactly: satisfied.
- Local/static evidence proves the selected command 0 prelude guard: satisfied.
- Generated-root source=firmware-initramfs and command-indexed
  rootinfo/manifest expectations remain mechanically checked: satisfied.
- Task-owned JSON records findings with disposition: satisfied.
- selected_next_task is
  phase10-pi5-serial-command0-prelude-pi5-proof-20260617: satisfied.

## Validation

- sh -n on touched shell script: pass.
- task-owned local/static validator positive fixture: pass.
- task-owned local/static validator negative control: pass; blocked evidence
  remains rejected.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-prelude-pi5-proof-20260617 on the next
worker wake if dependencies remain satisfied, the repository remains clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
