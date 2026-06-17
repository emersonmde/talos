# Phase 10 Pi 5 Serial Command 0 Prelude Source Contract

Task id: phase10-pi5-serial-command0-prelude-source-contract-20260617

Status: accepted

Classification:
serial-command0-prelude-capture-boundary-guard-core-selected

Evidence level: static/source/task evidence inspection, lab serial endpoint
contract inspection, command-loop source inspection, accepted direct-read Pi 5
proof evidence, task-owned JSON evidence, docs build, and diff checks. No
hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, runtime code change, persistence, storage work, networking, SSH,
Phase 11/12 expansion, or phase transition was performed by this task.

## Goal

Define the next feature-led command-input contract after Milestone 10.3 closed
generated-root transport while leaving shell-visible generated-root command
input paused at command 0.

## Source Findings

The direct-read Pi 5 proof did not retain enough post-write evidence to accept
command input. It retained selected-tree identity, same-power-cycle TFTP
evidence, same-boot source=firmware-initramfs reason=valid-artifact, ready
command=0, a visible prompt, fresh command 0 pre-write direct-read evidence, and
a successful /serial/write of rootinfo.

The retained command 0 post-write window did not include rootinfo command text,
the generated-root source response, or dispatch command=0 status=handled
responses=1. It did include the tail of the command 0 edit line and
ready command=1, then command 1 timeout/readiness output. That shape is
important: the kernel command loop appears to have advanced past command 0, but
the retained evidence window cannot prove the handled command or response. The
missing boundary is therefore the proof/capture prelude around command 0, not
accepted generated-root command-input behavior.

The relevant source contract is:

- src/local_command_loop.rs writes a prompt, waits for a canonical line,
  dispatches rootinfo through write_generated_root_selection_line, then reports
  command status and response counts through the target proof wrapper.
- src/target/rpi5.rs wraps the local command loop with command-indexed ready,
  line, dispatch, edit, and final proof records for the
  rpi5_generated_root_boot_transport scenario.
- docs/src/project/lab-controller.md defines /serial/read as a consuming direct
  read and /serial/write as byte write with optional newline; direct-read
  command-input proof is acceptable only when same-boot source, exact command
  ready marker, pre-write freshness, write result, command text, response
  output, dispatch status/count, and post-command readiness are all retained.

## Compared Approaches

- rejected: repeat the same direct-read timing with a longer wait. The accepted
  proof already shows ready command=1 and then later command timeouts; a longer
  post-write read mostly increases the chance of consuming later command
  windows without recovering the missing command 0 response boundary.
- rejected: accept /serial/write success plus ready command=1 as command input.
  That would collapse the user-visible feature into byte acceptance and would
  not prove rootinfo was handled or that generated-root source evidence reached
  the shell.
- selected: add a local/static command 0 prelude guard contract to the proof
  helper/validator. The guard must bind write rootinfo to the next command-ready
  boundary as one command transaction: command 0 pre-write freshness, write
  response, retained rootinfo text, retained generated-root source response,
  dispatch command=0 status=handled responses=1, and ready command=1 before any
  command=1 timeout or later-command evidence is accepted.
- deferred: a hardware-backed proof using the guard. It is only authorized if
  the guard-core task accepts the helper/validator contract and explicitly
  selects the Pi 5 proof follow-up.

## Selected Invariant

The next proof must preserve the user-visible feature: Pi 5 serial shell command
input consuming the accepted firmware-initramfs generated-root artifact. The
minimal accepted command 0 shape is:

- same boot selected generated-root source=firmware-initramfs reason=valid-artifact;
- ready command=0 and talos> prompt;
- immediate pre-write freshness read for command 0;
- successful /serial/write of rootinfo with newline;
- retained rootinfo command text or an equivalent line record for command=0;
- retained talos: generated-root source=firmware-initramfs reason=valid-artifact response;
- retained dispatch command=0 status=handled responses=1;
- retained ready command=1 before accepting any command=1 timeout, later
  command readiness, manifest output, PASS, or final failure as command 0
  success.

If any of those fields is absent, the first failing invariant must be classified
as publication/staging, serial freshness/capture, serial write delivery,
command-loop input, command-0 dispatch, generated-root regression, TFTP
evidence, final identity, or restore failure.

## Allowed Follow-Up Surface

The selected dependency-gated follow-up is
phase10-pi5-serial-command0-prelude-guard-core-20260617.

That task may update only the proof helper/validator and directly related task
records/evidence/docs unless it records a precise blocker before changing
scope. Expected files/surfaces:

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh and any
  directly paired local/static validator logic it invokes or embeds;
- task-owned JSON evidence under
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-guard-core/;
- the guard-core task record;
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md and
  docs/src/roadmap.md if the evidence contract changes.

The follow-up must reject same-shaped wait/marker/cursor retries unless a new
discriminator explains why they answer a different question. It must not modify
the kernel command loop or Pi hardware boot path unless the local/static helper
contract proves those source changes are necessary and records that scope
change for supervisor planning.

## Findings

- fixed: explained the command 0 proof gap as a retained-evidence boundary:
  ready command=1 was retained, but rootinfo text, generated-root response, and
  command=0 dispatch were not.
- fixed: selected a command 0 prelude guard contract that keeps the feature as
  real serial shell command input against the firmware-initramfs generated-root
  artifact.
- deferred: Pi 5 hardware rerun remains gated behind accepted guard-core
  helper/validator evidence.
- rejected: prompt-only, /serial/write-only, ready-command-only, and later
  command output as command 0 success.
- rejected: same-shaped direct-read timing retries without a new command
  boundary discriminator.
- rejected: persistence, writable filesystem, SD/USB/block storage,
  networking, SSH, Phase 11/12 expansion, and phase transition claims.
- not-an-issue: no hardware lock, boot publication, or Pi 5 rerun was required
  because this is a source/static contract over committed evidence.

## Evidence

- Direct-read Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-pi5-proof.md.
- Direct-read proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-pi5-proof/classification.json.
- Direct-read selected run evidence:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-pi5-proof/candidate-direct-read-20260617T043803Z/.
- Direct-read closeout:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-closeout.md.
- Milestone 10.3 closeout:
  tasks/2026-06-17-phase10-pi5-generated-root-milestone-10-3-closeout.md.
- Lab serial endpoint contract:
  docs/src/project/lab-controller.md.
- Command-loop source:
  src/local_command_loop.rs and src/target/rpi5.rs.
- Source-contract classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-source-contract/classification.json.
- Source-contract evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-source-contract/evidence-map.json.

## Acceptance Check

- Explains why the direct-read proof did not retain rootinfo/source/dispatch,
  or records the exact missing evidence: satisfied. It records missing retained
  command text, generated-root source response, and dispatch command=0 despite
  retained ready command=1.
- Preserves the user-visible feature as Pi 5 serial shell command input
  consuming the accepted firmware-initramfs generated-root artifact: satisfied.
- Compares at least two qualitatively different next approaches and rejects
  same-shaped retries: satisfied.
- selected_next_task is phase10-pi5-serial-command0-prelude-guard-core-20260617:
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

Promote phase10-pi5-serial-command0-prelude-guard-core-20260617 on the next
worker wake if dependencies remain satisfied, the repository remains clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
Do not run hardware, retry command input, start persistence/storage work,
networking, SSH, Phase 11/12 expansion, or a phase transition from this source
contract.
