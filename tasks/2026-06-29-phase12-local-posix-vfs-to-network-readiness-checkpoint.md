# Phase 12 Local POSIX/VFS To Network Readiness Checkpoint

Task:
phase12-local-posix-vfs-to-network-readiness-checkpoint-20260629

Status: accepted and committed; durable supervisor state records the final
commit SHA.

## Summary

Checkpointed the accepted local POSIX/VFS/userspace frontier after the
explicit-fd separated redirection-token grammar checkpoint. This task adds no
runtime behavior.

The accepted local frontier remains descriptor-backed and limited to:

- VFS-backed open/read and executable loading from initramfs files.
- Absolute and fixed-/bin bare-name executable dispatch through the accepted
  VFS/userspace path.
- Literal argv, deterministic empty envp, standard descriptor inheritance,
  lifecycle/status, laststatus, waitpid, process-table observations, and
  accepted two-stage pipe/status observations.
- Descriptor-backed local redirection and pipeline redirection only for the
  exact accepted direct and fixed-/bin bare-name witnesses.
- Safe volatile-vfs /tmp leaf output files, including truncate and append
  behavior, with descriptor-backed readback.
- The exact no-space, separated operator/path, and explicit fd/operator/path
  token forms accepted by the prerequisite tasks.

Further shell/redirection grammar expansion is not the next feature by itself.
It would be process drag unless it is required to prove a real
VFS/syscall/userspace capability or a live-network acceptance need. Existing
command surfaces remain regression and control surfaces unless backed by those
layers.

The retained Phase 12 Ethernet frontier is still paused at the accepted
BCM54213PE timeout/link-not-ready boundary. The earlier post-generated-root
source checkpoint found no selected discriminator, but supervisor planning has
now provided one bounded, source/evidence-only reselection task whose purpose
is to review the retained pause state and select at most one concrete
discriminator before any implementation or hardware proof.

Selected next task:
phase12-rp1-ethernet-link-not-ready-discriminator-reselection-20260629.

No Pi 5 hardware/lab action, boot archive publication, generated-root retry,
live networking/SSH acceptance, fake command expansion, persistence claim, or
phase transition was performed.

## Findings

- fixed: Added this checkpoint record so the handoff from accepted local
  POSIX/VFS/redirection work back to bounded Phase 12.1 planning is durable
  independently of transient supervisor state.
- not-an-issue: The accepted local command-visible surface remains backed by
  descriptor/VFS/userspace layers rather than fake kernel command expansion.
- not-an-issue: Additional local shell/redirection grammar expansion is not
  required before reviewing the paused Ethernet frontier because the accepted
  local frontier already covers descriptor-backed file I/O, exec, argv/envp,
  descriptor inheritance, wait/status, pipes, and bounded volatile /tmp
  redirection controls.
- deferred: Persistent writable filesystem behavior, nested/traversal paths,
  paths outside volatile /tmp, explicit fd input redirection, fd
  duplication/close syntax, PATH/current-directory lookup, command lookup
  beyond bounded /bin, arbitrary shell grammar, generated-root retry, Pi 5
  hardware proof, boot publication, live networking/SSH acceptance, and phase
  transition remain outside this checkpoint.
- deferred: The Phase 12.1 link-not-ready discriminator itself is not selected
  here; the next task may select at most one discriminator after a bounded
  source/evidence review.
- removed: no source, helper, task, docs, or evidence file was removed.

## Evidence

- local explicit-fd separated redirection-token checkpoint:
  tasks/2026-06-28-phase12-local-explicit-fd-separated-redirection-token-frontier-checkpoint.md.
- local explicit-fd checkpoint classification:
  tasks/evidence/2026-06-28-phase12-local-explicit-fd-separated-redirection-token-frontier-checkpoint/classification.json.
- local explicit-fd checkpoint evidence map:
  tasks/evidence/2026-06-28-phase12-local-explicit-fd-separated-redirection-token-frontier-checkpoint/evidence-map.json.
- early POSIX shape:
  docs/src/project/early-posix-shape.md.
- roadmap:
  docs/src/roadmap.md.
- generated-root command-input success closeout:
  tasks/2026-06-18-phase10-pi5-generated-root-command-input-success-closeout.md.
- post-generated-root Phase 12 resumption checkpoint:
  tasks/2026-06-18-phase10-to-phase12-post-generated-root-command-input-resumption-checkpoint.md.
- Phase 12 link-not-ready pause closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-not-ready-frontier-pause-closeout.md.
- post-generated-root link-not-ready source checkpoint:
  tasks/2026-06-18-phase12-rp1-ethernet-post-generated-root-link-not-ready-resumption-source-checkpoint.md.

## Accepted Boundary

The accepted local POSIX/VFS/userspace frontier is reconciled without
broadening beyond descriptor-backed VFS/open/read/userspace execution, bounded
/bin lookup, accepted pipe and process observations, volatile /tmp
redirection, and exact accepted grammar surfaces.

Phase 12 live network work remains paused at the link-not-ready frontier.
The next task is a source/evidence-only reselection checkpoint, not a hardware
retry, packet I/O task, OpenSSH task, or phase transition.

selected_next_task:
phase12-rp1-ethernet-link-not-ready-discriminator-reselection-20260629.

planningNeeded=false because the queued reselection task has explicit scope,
non-goals, acceptance criteria, validation gates, docs requirements, evidence
requirements, and objective dependencies after this checkpoint.

## Validation

- git status --short --branch before edits: passed; main ahead of origin with
  no uncommitted Talos changes.
- static inspection of task records, retained evidence paths, docs, and
  supervisor queue: passed.
- git diff --check: passed.
- jq empty on task-owned JSON evidence: not applicable; this checkpoint added
  no task-owned JSON.
- /home/node/.cargo/bin/mdbook build: passed; search index size warning
  retained.
- git diff --cached --check: pending before commit.

Commit: recorded in talos-supervisor-state.json acceptance evidence.
