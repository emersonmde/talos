# Phase 10 Pi 5 Serial Command 0 Saturated-Capture Source Contract

Task id: phase10-pi5-serial-command0-saturated-capture-source-contract-20260617

Status: accepted

Classification:
command0-saturated-capture-source-contract-guard-core-selected

Evidence level: static/source/task/lab-doc evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, helper implementation,
command0 write-delivery success, command0 source-response retention acceptance,
generated-root command-input acceptance, storage, networking, SSH, Phase 11/12
expansion, or phase transition was performed.

## Goal

Select the smallest distinct discriminator for command0 write delivery after
the cursor-bound post-write observe proof became non-evaluable at the saturated
serial cursor.

## First Failing Invariant

The user-visible feature remains local serial interactivity: after Talos prints
a visible generated-root prompt and reports ready command=0, a lab serial write
of rootinfo plus newline should reach the runtime console, dispatch command 0,
emit responses=1, and advance to ready command=1.

The accepted selected-kernel/TFTP precondition remains proven for the
208984-byte da591740/kernel_2712.img candidate. The remaining failing invariant
is capture/evidence for command0 write delivery after that selected boot:
/serial/write accepted 9 bytes for rootinfo, but the proof could not retain
fresh command0 output. The post-write observe attempt started from cursor
4194304, the lab controller's retained serial cursor cap, and returned zero
readiness bytes plus zero post-write bytes. A later non-gating peek showed
rootinfo processed only as stale command=3 after command=1 and command=2
timeouts, which is not ordered command0 write delivery.

## Approaches Compared

Endpoint/capture repair would ask the lab service to expose a monotonic serial
cursor beyond the retained-log cap, rotate/clear the retained log safely, or add
a source-backed endpoint that proves post-write bytes without the saturation
boundary. That is the cleanest long-term observability repair, but it is
outside this Talos repo task, would require lab-service/operator action, and
would pause the feature path without producing local implementation progress.

Command-indexed saturated direct-read fallback uses the existing lab-controller
contract already documented for saturated cursors: when a saved cursor is at the
retention cap, repository helpers may switch from cursor-bound /serial/observe
to a bounded direct /serial/read deadline loop and must label the evidence
deadline-loop-direct-read-after-saturated-cursor. This does not accept direct
read output by itself. It is acceptable only when bounded by same-boot
readiness, selected-kernel/TFTP agreement, immediate pre-write freshness,
accepted rootinfo write, ordered command0 response fragments, final identity,
and restore proof.

Explicit pause remains valid if the helper/core task cannot encode a contract
that is materially different from the already-blocked v2 direct-read proof. The
distinguishing requirement for proceeding is that the guard must reject empty
saturated observe windows and stale later-command-only output while using a
deadline-loop direct-read window specifically because the saved cursor is
saturated.

## Selected Contract

The selected next task is
phase10-pi5-serial-command0-saturated-capture-guard-core-20260617.

That task should implement a local/static guard for
command0-saturated-capture-guard-v1. The guard must require:

- selected-kernel/TFTP agreement for the same 208984-byte
  da591740/kernel_2712.img candidate before command behavior is evaluated;
- same-boot generated-root readiness with source=firmware-initramfs,
  reason=valid-artifact, ready command=0, and a visible talos> prompt;
- an immediate pre-write freshness read that does not already contain rootinfo,
  line command=0, dispatch command=0, responses=1, ready command=1, command=2,
  command=3, or generated-root source-response output;
- POST /serial/write accepting rootinfo with append_newline=true and 9 bytes;
- a bounded post-write direct /serial/read deadline loop labeled
  deadline-loop-direct-read-after-saturated-cursor when the saved cursor is at
  4194304;
- ordered post-write command0 evidence: rootinfo or line command=0,
  dispatch command=0 status=handled, responses=1, and ready command=1;
- final pre-restore identity and post-run restore proof.

The guard must reject:

- empty saturated observe windows;
- /serial/write byte acceptance alone;
- stale pre-write output that already contains command0 output;
- stale later-command-only output, including rootinfo processed after command=1
  or command=2 timeouts;
- unordered command0 fragments;
- source-response-only evidence without command0 line/rootinfo and dispatch
  ordering.

## Findings

- fixed: restated the first failing invariant without shrinking the original
  command0 write-delivery feature boundary.
- fixed: compared endpoint/capture repair with a command-indexed saturated
  direct-read fallback and explicit pause.
- fixed: selected the guard-core follow-up because the documented saturated
  cursor fallback provides a distinct local/static contract without lab-service
  mutation.
- deferred: helper implementation and any Pi 5 hardware publication are owned
  by dependency-gated follow-up tasks.
- rejected: command0 write-delivery success, command0 source-response
  retention success, generated-root command-input success, storage, networking,
  SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Accepted post-write observe closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-post-write-observe-closeout.md.
- Post-write observe closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-closeout/classification.json.
- Post-write observe Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof.md.
- Observe proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/classification.json.
- Observe proof evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/candidate-command0-post-write-observe-20260617T150944Z/post-write-observe-evidence.json.
- Accepted write-delivery v2 closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-v2-closeout.md.
- V2 Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition.md.
- Lab-controller serial endpoint contract:
  docs/src/project/lab-controller.md.
- Saturated cursor fallback helper precedent:
  scripts/rpi5-observe-serial-window.sh.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-source-contract/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-source-contract/evidence-map.json.

## Acceptance Check

- The first failing invariant is restated without shrinking the command0
  write-delivery goal: satisfied.
- At least two qualitatively different next approaches are compared: satisfied
  with endpoint/capture repair, command-indexed saturated direct-read fallback,
  and explicit pause.
- selected_next_task is the saturated-capture guard core or null with
  planningNeeded=true: satisfied; the guard core is selected.
- The selected evidence contract rejects empty saturated observe windows,
  write-accepted-only evidence, stale pre-write output, stale later-command-only
  output, and source-response-only evidence: satisfied.
- Rejected claims include command0 write-delivery success, command0
  source-response retention success, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition:
  satisfied.

## Validation

- static/source/task/lab-doc evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-saturated-capture-guard-core-20260617 on
the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention remains inactive, and the repository
has no conflicting uncommitted changes. Do not run hardware from this
source/static task.
