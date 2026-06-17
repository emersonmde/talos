# Phase 10 Pi 5 Serial Capture Readiness Source Contract

Task id: phase10-pi5-serial-capture-readiness-source-contract-20260617

Status: accepted

Classification:
serial-capture-readiness-contract-guard-core-selected

Evidence level: static/task evidence inspection, accepted command0
source-response-retention closeout/proof/core evidence, task-owned JSON
evidence, docs build, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, runtime code change,
persistence, storage work, networking, SSH, Phase 11/12 expansion, or phase
transition was performed.

## Goal

Select the smallest source-backed discriminator for the command0
source-response-retention blocker after the latest Pi 5 proof became
non-evaluable at serial readiness/capture rather than command0 response
generation.

## Source Contract

The selected follow-up contract is
serial-capture-readiness-guard-v1.

The accepted command0 source-response retention closeout preserves three facts:

- command0-source-response-retention-guard-v2 is still the correct local/static
  acceptance surface for a future command0 transaction;
- the Pi 5 candidate archive, selected-tree identity, same-power-cycle TFTP
  evidence, final identity, known-good control, candidate rerun, and restore
  proof were retained;
- command0 source-response retention could not be evaluated because the serial
  direct-read path did not retain a usable generated-root ready command=0 window
  before command-loop timeout.

The current first failing invariant is serial readiness/capture setup. The
first candidate readiness capture retained later stale shell state: input-error
timeout, ready command=3, and fresh_after_prompt=false; its command0 direct-read
summary retained zero bytes. The known-good control and candidate rerun then
retained only early firmware/RP1 bytes under the long-settle direct-read
strategy. That evidence does not prove command0 response generation failed; it
proves the proof was not armed against the right command0 readiness boundary.

Before another hardware run, the proof surface needs a local/static guard that
classifies readiness/capture quality separately from command0 source-response
quality. The guard must accept only an evaluable command0 window that retains
same-boot firmware-initramfs valid-artifact readiness, ready command=0, a
visible prompt, an immediate fresh pre-write boundary, command0 write delivery,
and then the existing command0-source-response-retention-guard-v2 transaction.
It must reject early-firmware-only capture, stale later-command readiness,
dispatch-only metadata, prompt-only/write-only evidence, and tail-only command0
source response.

## Compared Approaches

- selected: implement a local/static readiness/capture guard over the proof
  helper and task-owned fixtures. This is qualitatively different from another
  hardware retry because it first rejects the exact non-evaluable evidence shape
  that escaped the previous hardware proof.
- deferred: change the lab serial service or kernel/target proof source. This
  is only justified if the guard-core task proves the existing direct-read proof
  surface cannot distinguish an evaluable command0 readiness window from stale
  or early-firmware-only capture.
- rejected: another same-shaped Pi 5 run with only longer settle timing,
  cursor-only changes, or command0 source-response expectations. The last proof
  reached candidate identity and TFTP but failed before command0 could be
  evaluated.
- rejected: treating early firmware/RP1 bytes, stale ready command=3 output,
  dispatch-only metadata, prompt-only evidence, or write-only evidence as
  generated-root command-input success.

## Selected Follow-Up Surface

The selected dependency-gated follow-up is
phase10-pi5-serial-capture-readiness-guard-core-20260617.

That task may edit only surfaces directly needed to make serial
readiness/capture locally/static-checkable before hardware:

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh and
  directly paired task-owned fixtures or validators;
- task-owned evidence under
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-guard-core/;
- the guard-core task record;
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md,
  docs/src/project/lab-controller.md, and docs/src/roadmap.md only if the
  evidence contract changes.

The guard-core task must preserve command0-source-response-retention-guard-v2
as the eventual command0 transaction acceptance gate. A readiness/capture pass
must not by itself accept generated-root command-input success.

## Findings

- fixed: restated the first failing invariant as serial readiness/capture setup,
  tied to retained first-candidate readiness, command0 summary, known-good
  control, and candidate rerun evidence.
- fixed: selected a local/static guard-core discriminator before another
  hardware run.
- fixed: preserved command0-source-response-retention-guard-v2 as the later
  transaction acceptance surface.
- deferred: lab-service, kernel command-loop, and target proof-source changes
  remain gated behind guard-core evidence proving they are necessary.
- rejected: same-shaped hardware retry, early-firmware-only capture, stale
  later-command readiness, dispatch-only metadata, prompt-only/write-only
  evidence, and tail-only source-response acceptance.
- rejected: generated-root command-input success, command0 response-generation
  failure, persistence, writable storage, networking, SSH, Phase 11/12
  expansion, and phase transition.
- not-an-issue: no hardware lock or Pi 5 rerun was required because this is a
  source/static contract over committed proof evidence.

## Evidence

- Accepted command0 source-response retention closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-source-response-retention-closeout.md.
- Closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-closeout/classification.json.
- Pi 5 proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/classification.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/evidence-map.json.
- First candidate readiness summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/candidate-command0-retention-20260617T074126Z/serial/readiness-summary.json.
- First candidate command0 direct-read summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/candidate-command0-retention-20260617T074126Z/serial/command0-direct-read-summary.json.
- Known-good control readiness:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/triage-control-and-rerun-20260617T074637Z/known-good-control/serial/readiness-control.json.
- Candidate rerun readiness:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/triage-control-and-rerun-20260617T074637Z/candidate-command0-retention-rerun/serial/readiness-summary.json.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-source-contract/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-source-contract/evidence-map.json.

## Acceptance Check

- The selected contract explains why the last Pi 5 proof was non-evaluable at
  serial readiness/capture rather than command0 response generation: satisfied.
- The invariant, contradicting evidence, unproven assumptions, and smallest
  decisive discriminator are named before another hardware run: satisfied.
- selected_next_task is
  phase10-pi5-serial-capture-readiness-guard-core-20260617: satisfied.
- Rejected claims include generated-root command-input success, command0
  source-response generation failure, persistence, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-capture-readiness-guard-core-20260617 on the next
worker wake if dependencies remain satisfied, the repository remains clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
Do not run hardware, start persistence/storage work, networking, SSH, Phase
11/12 expansion, or a phase transition from this source-contract task.
