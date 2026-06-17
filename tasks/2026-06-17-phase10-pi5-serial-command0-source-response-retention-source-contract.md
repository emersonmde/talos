# Phase 10 Pi 5 Serial Command 0 Source Response Retention Source Contract

Task id: phase10-pi5-serial-command0-source-response-retention-source-contract-20260617

Status: accepted

Classification:
serial-command0-source-response-retention-contract-core-selected

Evidence level: static/source/task evidence inspection, lab serial endpoint
contract inspection, command-loop source inspection, accepted command 0 prelude
hardware evidence, task-owned JSON evidence, docs build, and diff checks. No
Pi 5 hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, runtime code change, persistence, storage work, networking, SSH,
Phase 11/12 expansion, or phase transition was performed by this task.

## Goal

Define the next bounded source contract after the command 0 prelude closeout
proved command 0 line/dispatch/readiness on Pi 5 but did not retain the full
firmware-initramfs generated-root source response in the same command
transaction.

## First Failing Invariant

The accepted command 0 prelude proof kept the feature under test as real Pi 5
serial shell command input against the firmware-initramfs generated-root
artifact. Its selected run retained:

- firmware-initramfs generated-root readiness and a visible prompt;
- selected-tree identity, same-power-cycle TFTP evidence, final pre-restore
  identity, and post-run baseline restore;
- command 0 line evidence for rootinfo;
- dispatch command=0 status=handled responses=1;
- ready command=1.

The same command 0 direct-read window did not retain the full response line
starting with talos: generated-root source=firmware-initramfs reason=valid-artifact.
It retained only the tail of that response line, beginning at the exec-len/path
portion, before the target proof line/dispatch/readiness records. This is a
source-response retention failure, not a command-loop dispatch failure.

Dispatch status=handled responses=1 is necessary but not sufficient for the
user-visible command-input feature. It proves that the target wrapper observed
one response line from the command loop after rootinfo returned, but it does
not by itself prove the response was retained in the task evidence or visible
to the shell user. The accepted user-visible feature still requires retained
command-indexed source text, not only target proof metadata.

## Source Findings

- src/local_command_loop.rs dispatches rootinfo by calling
  write_generated_root_selection_line and returns Handled only after writing one
  generated-root selection line.
- src/target/rpi5.rs records the command-indexed line, dispatch status,
  response count, edit summary, and next ready marker after the local command
  cycle returns.
- docs/src/project/lab-controller.md allows direct-read command-input evidence
  only when it is bound to the same boot and exact command index, with retained
  command text, response output, dispatch status/count, and post-command
  readiness.
- The retained proof evidence shows the command loop produced one response
  line, but the read window started in the middle of that line. Therefore
  accepting dispatch-only evidence would weaken the feature below shell-visible
  command input.

## Compared Approaches

- selected: repair or tighten only the proof/capture/validation surface for
  command 0 source-response retention. The next core task should inspect and, if
  needed, adjust the task-owned proof helper/harness so command 0 response
  evidence is retained as a complete ordered transaction: immediate pre-write
  freshness, write rootinfo, full generated-root source/reason response,
  dispatch command=0 status=handled responses=1, and ready command=1. Partial
  tail-only source-response evidence must remain rejected.
- deferred: change the command-loop or target proof source to add a stronger
  command 0 response anchor. This is only allowed if the core task proves from
  local/static evidence that the existing source response cannot be retained
  reliably through the proof/capture surface. Such a change must remain bounded
  to command 0 source-response retention and must not change generated-root
  behavior.
- rejected: accept dispatch command=0 responses=1 plus ready command=1 as
  generated-root command-input success. That would prove target metadata, not
  retained user-visible output.
- rejected: same-shaped timing, wait-count, marker-name, and cursor-only
  retries. They do not change the discriminator because the selected proof
  already observed command 0 dispatch and advanced readiness while retaining
  only a partial response line.

## Selected Follow-Up Surface

The selected dependency-gated follow-up is
phase10-pi5-serial-command0-source-response-retention-core-20260617.

That task may edit only surfaces directly needed to make command 0
source-response retention locally/static-checkable before hardware:

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh and
  directly paired task-owned fixtures or validators;
- task-owned evidence under
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-core/;
- the core task record;
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md and
  docs/src/roadmap.md if the evidence contract changes.

The core task must record a precise blocker before changing kernel command-loop
or Pi boot-path source. It must reject stale, unordered, prompt-only,
write-only, dispatch-only, and tail-only source-response evidence unless it
introduces a stronger discriminator and records why that discriminator preserves
the user-visible feature.

## Findings

- fixed: restated the first failing invariant as command 0 source-response
  retention, tied to retained line/dispatch/ready evidence from the accepted
  Pi 5 proof.
- fixed: recorded why dispatch responses=1 plus missing retained source text is
  insufficient for shell-visible generated-root command-input acceptance.
- fixed: compared a proof/capture/validation-only approach with a possible
  command-loop or target proof source change and selected the narrower
  proof/helper core first.
- rejected: same-shaped timing, wait-count, marker-name, cursor-only,
  prompt-only, write-only, dispatch-only, and tail-only retries as acceptance.
- deferred: hardware proof remains gated behind an accepted core task that names
  the exact retained evidence contract.
- rejected: persistence, writable filesystem, SD/USB/block storage, networking,
  SSH, Phase 11/12 expansion, and phase transition.
- not-an-issue: no hardware lock or Pi 5 rerun was required because this is a
  source/static contract over committed proof evidence.

## Evidence

- Command 0 prelude source contract:
  tasks/2026-06-17-phase10-pi5-serial-command0-prelude-source-contract.md.
- Command 0 prelude guard core:
  tasks/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core.md.
- Command 0 prelude Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof.md.
- Command 0 prelude closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-prelude-closeout.md.
- Selected Pi 5 run command 0 direct-read summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/candidate-command0-atomic-20260617T061825Z/serial/command0-direct-read-summary.json.
- Command-loop source:
  src/local_command_loop.rs.
- Pi 5 target proof wrapper:
  src/target/rpi5.rs.
- Lab serial endpoint contract:
  docs/src/project/lab-controller.md.
- Source-response-retention classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-source-contract/classification.json.
- Source-response-retention evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-source-contract/evidence-map.json.

## Acceptance Check

- The first failing invariant is restated from first principles and tied to
  concrete retained evidence: satisfied.
- Explains why dispatch responses=1 plus missing retained source text is not
  sufficient for the user-visible command-input feature: satisfied.
- Compares at least two qualitatively different approaches and rejects
  same-shaped retries: satisfied.
- selected_next_task is
  phase10-pi5-serial-command0-source-response-retention-core-20260617:
  satisfied.
- Persistence, storage drivers, networking, SSH, Phase 11/12 expansion, and
  phase transition remain rejected: satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-source-response-retention-core-20260617 on
the next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not run hardware, start persistence/storage work, networking, SSH,
Phase 11/12 expansion, or a phase transition from this source contract.
