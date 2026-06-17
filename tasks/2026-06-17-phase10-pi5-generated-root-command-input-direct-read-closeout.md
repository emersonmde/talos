# Phase 10 Pi 5 Generated-Root Command-Input Direct-Read Closeout

Task id: phase10-pi5-generated-root-command-input-direct-read-closeout-20260617

Status: accepted

Classification:
pi5-generated-root-command-input-command0-paused-milestone-closeout-selected

Evidence level: static/task evidence inspection, accepted direct-read source
contract, local/static helper evidence, blocked serialized Pi 5 direct-read
hardware proof evidence, task-owned JSON evidence, docs build, and diff checks.
No hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, storage work, networking, SSH, Phase 11/12 expansion, or phase
transition was performed by this closeout.

## Goal

Reconcile the accepted direct-read source contract, direct-read harness core,
and serialized Pi 5 direct-read proof. Record the exact command-input boundary
without converting the blocked command 0 evidence into an accepted capability.

## Outcome

Generated-root command input remains blocked and is explicitly paused at the
command 0 prelude invariant.

The direct-read source contract selected a command-indexed replacement for the
prior saturated /serial/observe cursor. The local/static harness core made that
contract mechanically checkable and rejected prompt-only, /serial/write-only,
stale pre-write, and missing-dispatch shapes. The serialized Pi 5 proof then
retained selected-tree identity, stable same-power-cycle TFTP evidence for
da591740/kernel_2712.img and da591740/initramfs_2712, same-boot
source=firmware-initramfs reason=valid-artifact, ready command=0, a visible
talos> prompt, fresh command 0 pre-write direct-read evidence, successful
/serial/write of rootinfo, final pre-restore identity, and restore proof.

The first failing invariant is command 0: after rootinfo was written, the
command 0 direct-read window did not retain rootinfo, source=firmware-initramfs
reason=valid-artifact, or dispatch command=0 status=handled responses=1. The
candidate therefore cannot prove command-indexed command input, and command 1
manifest handling is not accepted.

This closeout pauses the remaining generated-root command-input blocker instead
of selecting another hardware retry or harness adjustment. The accepted
Milestone 10.3 boundary remains Pi 5 firmware-initramfs generated-root
consumption, not shell-visible generated-root command input. The selected
dependency-gated follow-up is the Milestone 10.3 closeout checkpoint so the
program can record the paused boundary before any future supervisor planning.

## Findings

- fixed: reconciled the direct-read source contract, helper/core, and Pi 5
  proof evidence into one retained command-input frontier.
- fixed: preserved the accepted Pi 5 firmware-initramfs generated-root
  consumption boundary as separate from command-input success.
- blocked: command 0 direct-read evidence did not retain rootinfo, source
  evidence, or handled dispatch after /serial/write accepted rootinfo.
- deferred: any future command-input work must be explicitly replanned around
  the command 0 prelude invariant; no same-shaped retry is selected here.
- rejected: treating prompt visibility, /serial/write byte acceptance,
  direct-read readiness, or command 1 evidence after a failed command 0
  prelude as generated-root command-input proof.
- rejected: persistence, writable filesystem, SD/USB/block storage,
  networking, SSH, Phase 11/12 expansion, and phase transition claims.
- not-an-issue: no hardware lock, boot publication, or Pi 5 rerun was required
  because the proof task already retained and committed the hardware evidence
  and restore proof.

## Evidence

- Direct-read source contract:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-source-contract.md.
- Source contract classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-source-contract/classification.json.
- Direct-read harness core:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core.md.
- Harness core classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-harness-core/classification.json.
- Direct-read Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-pi5-proof/classification.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-closeout/evidence-map.json.

## Accepted Boundary

Accepted:

- Pi 5 firmware-initramfs generated-root artifact consumption from the prior
  reservation proof;
- local/static direct-read command-input contract and validator helper;
- retained blocked hardware evidence for the direct-read command-input attempt;
- an explicit pause at the command 0 prelude blocker.

Not accepted:

- shell-visible generated-root command input on Pi 5;
- prompt visibility as command-input proof;
- /serial/write byte acceptance as command-input proof;
- direct-read readiness without command-indexed handled dispatch;
- command 1 manifest proof after a failed command 0 prelude;
- writable persistence;
- SD/USB/block storage;
- broader filesystem mutation;
- networking, sockets, or SSH;
- Phase 11/12 feature expansion from this evidence;
- phase transition.

## Acceptance Check

- Closeout classification matches retained source/core/proof evidence:
  satisfied with
  pi5-generated-root-command-input-command0-paused-milestone-closeout-selected.
- Accepted generated-root command-input capability, blocked invariant, or
  explicit pause is unambiguous: satisfied by the explicit command 0 prelude
  pause; no command-input capability is accepted.
- Rejected claims include persistence, writable storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.
- Task record and task-owned JSON record findings with disposition: satisfied.
- selected_next_task is the Milestone 10.3 closeout checkpoint: satisfied,
  because command input is explicitly paused with rejected claims preserved.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-generated-root-milestone-10-3-closeout-20260617 on the next worker
wake if dependencies remain satisfied, the repository remains clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
Do not start another generated-root command-input hardware retry, persistence,
storage work, networking, SSH, Phase 11/12 expansion, or a phase transition
from this closeout.
