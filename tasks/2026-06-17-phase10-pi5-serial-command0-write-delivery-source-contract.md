# Phase 10 Pi 5 Serial Command 0 Write Delivery Source Contract

Task id: phase10-pi5-serial-command0-write-delivery-source-contract-20260617

Status: accepted

Classification:
serial-command0-write-delivery-contract-guard-core-selected

Evidence level: static/source/task evidence inspection, accepted command 0
prelude Pi 5 contrast evidence, accepted serial capture/readiness Pi 5 blocker
evidence, command-loop source inspection, task-owned JSON evidence, docs build,
and diff checks. No implementation work, hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, storage, networking,
SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Define the smallest source/static discriminator for why command 0 write
delivery is not retained after an accepted serial readiness boundary.

## First Failing Invariant

After same-boot firmware-initramfs valid-artifact readiness, ready command=0,
a visible talos> prompt, and an empty fresh pre-write boundary are retained, an
accepted 9-byte rootinfo serial write must either:

- be retained as command 0 line/dispatch/readiness evidence in target output;
  or
- be classified at the exact lab write, capture, or target command-loop timing
  boundary that prevents that evidence from being retained.

The accepted command 0 prelude proof shows that rootinfo can reach the command
loop under the same generated-root proof scenario. Its selected run retained an
empty command 0 pre-write read, /serial/write accepted 9 bytes, and the first
post-write direct-read window retained:

- line command=0 hex=72 6f 6f 74 69 6e 66 6f;
- dispatch command=0 status=handled responses=1;
- ready command=1.

The accepted serial capture/readiness proof reached the readiness boundary that
the earlier source-response proof had missed: same-boot firmware-initramfs
valid-artifact readiness, ready command=0, a visible prompt, and an empty fresh
pre-write read. The lab serial write endpoint again accepted 9 bytes for
rootinfo, but retained target output did not show rootinfo, command 0 line
evidence, dispatch command=0, generated-root source text, or ready command=1.
It later retained empty input-error timeouts for command=1 and command=2.

This is a command 0 write-delivery/capture blocker before command 0
source-response retention can be evaluated. It is not generated-root
command-input success, and it is not proof that the command-loop rootinfo
handler cannot produce a response.

## Source Findings

- src/local_command_loop.rs writes the prompt first, then waits in
  tty::run_polling_rx_diagnostic_with_limit. If no completed line reaches the
  command loop before the wait limit, dispatch_completed_line reports
  talos: input-error timeout with an empty line and then the next ready marker.
- A handled command is recorded only after dispatch_local_command receives the
  line and the command response is written. Therefore dispatch command=0
  status=handled responses=1 and ready command=1 are delivery evidence; their
  absence after a write-accepted response keeps delivery unresolved.
- The lab /serial/write response proves only that the lab service accepted the
  write request bytes. It does not prove the bytes were ordered into the target
  UART receive stream before the command-loop wait window expired.
- The accepted prelude proof is a positive contrast for target-side ability to
  receive rootinfo and advance to ready command=1. The accepted readiness proof
  is a negative contrast for the later write/capture timing shape after a
  stronger readiness gate.

## Compared Explanations

- lab write endpoint delivery or ordering: /serial/write may acknowledge before
  the payload is observable by the target input path, or the write may arrive
  after the command 0 wait window. This fits the retained readiness proof:
  command=0 timed out without line evidence, then the proof observed later empty
  command indexes.
- target command-loop timeout/capture consumption: the target may already have
  entered or completed an empty command 0 receive window before the write became
  visible, while the direct-read collector captured only the later timeout
  records. This also fits the observed command=1 and command=2 empty timeouts.
- source-response generation failure: rejected for this task. The retained
  output does not show command 0 reaching dispatch, so source-response
  generation remains non-evaluable.
- same-shaped hardware retry: rejected until local/static guards can classify
  write-accepted-only, prompt-only, dispatch-only, stale, unordered, and
  source-response-only evidence without accepting a weaker feature.

## Selected Follow-Up Surface

The selected dependency-gated follow-up is
phase10-pi5-serial-command0-write-delivery-guard-core-20260617.

That task may edit only surfaces directly needed to make command 0 write
delivery locally/static-checkable before hardware:

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh and
  directly paired task-owned fixtures or validators;
- task-owned evidence under
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core/;
- the guard-core task record;
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md,
  docs/src/project/lab-controller.md, and docs/src/roadmap.md only if the
  evidence contract changes.

The guard-core task must preserve command0-source-response-retention-guard-v2
as the later source-response transaction gate. A write-delivery pass must not
by itself accept generated-root command-input success.

## Findings

- fixed: restated the first failing invariant as command 0 write delivery after
  an accepted readiness boundary, tied to retained prelude and readiness proof
  evidence.
- fixed: selected a local/static guard-core discriminator before any new Pi 5
  hardware retry.
- fixed: separated lab write acceptance from target-visible delivery evidence.
- deferred: lab-service, kernel command-loop, target proof-source, and hardware
  retry changes remain gated behind guard-core evidence proving they are
  necessary.
- rejected: command0 source-response retention success, generated-root
  command-input success, write-accepted-only evidence, prompt-only evidence,
  dispatch-only evidence when delivery evidence is required, unordered output,
  stale serial output, storage, networking, SSH, Phase 11/12 expansion, and
  phase transition.
- not-an-issue: no hardware lock or Pi 5 rerun was required because this is a
  source/static contract over committed proof evidence.

## Evidence

- Command 0 prelude Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof.md.
- Prelude classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/classification.json.
- Prelude command 0 direct-read summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/candidate-command0-atomic-20260617T061825Z/serial/command0-direct-read-summary.json.
- Prelude command 0 write:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/candidate-command0-atomic-20260617T061825Z/serial/command0-write.json.
- Serial capture/readiness Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof.md.
- Serial capture/readiness classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/classification.json.
- Serial capture/readiness evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/evidence-map.json.
- Candidate rerun command 0 write:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/triage-control-and-rerun-20260617T092509Z/candidate-serial-capture-readiness-rerun-20260617T092509Z/serial/command0-write.json.
- Candidate rerun command 0 direct-read summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/triage-control-and-rerun-20260617T092509Z/candidate-serial-capture-readiness-rerun-20260617T092509Z/serial/command0-direct-read-summary.json.
- Command-loop source:
  src/local_command_loop.rs.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-source-contract/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-source-contract/evidence-map.json.

## Acceptance Check

- First failing invariant is stated in source/contract terms after accepted
  readiness and write acceptance: satisfied.
- Accepted prelude proof and accepted readiness proof are contrasted without
  treating either as generated-root command-input success: satisfied.
- At least two qualitatively different explanations are evaluated and the
  smallest discriminator is selected: satisfied.
- selected_next_task is
  phase10-pi5-serial-command0-write-delivery-guard-core-20260617: satisfied.
- Rejected claims include command0 source-response retention success,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition: satisfied.

## Validation

- static/source/task/evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-write-delivery-guard-core-20260617 on the
next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not run hardware, start storage work, networking, SSH, Phase 11/12
expansion, or a phase transition from this source-contract task.
