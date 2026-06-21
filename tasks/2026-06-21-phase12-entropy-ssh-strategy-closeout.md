# Phase 12 Entropy SSH Strategy Closeout

Task id: phase12-entropy-ssh-strategy-closeout-20260621

Status: accepted

Classification:
phase12-entropy-ssh-strategy-closeout-accepted-planning-needed

Evidence level: static task/docs/evidence review, docs build, and diff checks.
No new runtime implementation, host key generation, SSH service, hardware/lab
action, live packet I/O, hardware reachability, public ABI/POSIX/Linux
compatibility claim, broad expansion, or phase transition was performed.

## Goal

Close out the initial entropy/SSH-strategy slice, reconcile accepted evidence,
and decide whether key-management, crypto dependency evaluation, or another
prerequisite task is mechanically unblocked.

## Scope Performed

- Reconciled the accepted SSH strategy contract, entropy source contract, and
  entropy diagnostic implementation evidence.
- Reviewed Phase 12 docs and roadmap frontier text for drift against accepted
  evidence.
- Recorded findings with disposition and selected no follow-up task because no
  explicit queued key-management, crypto dependency, seed persistence, service
  shape, or exposure-control task exists with objective acceptance gates.
- Requested supervisor planning for the next bounded Phase 12.5 prerequisite
  task.

## Findings

- fixed: the Phase 12.5 frontier now closes over the accepted prerequisite-first
  strategy, the entropy source contract, and the source/unit entropy diagnostic
  implementation instead of implying immediate SSH service work.
- fixed: docs and roadmap now state that the entropy diagnostic is a
  classification/fail-closed surface only, not a random-byte generator,
  cryptographic-strength claim, SSH readiness claim, or host-key path.
- deferred: kernel entropy collection, conditioning/DRBG selection,
  operator-seed provisioning and persistence, host-key generation/storage,
  authorized-key storage, authentication policy, time requirements,
  heap-pressure limits, crypto dependency evaluation, process/service
  lifecycle, and SSH service shape remain future work.
- blocked: no next implementation task is mechanically unblocked from the
  existing queue. The only queued tasks after this closeout are the older
  link-ready discriminator core/proof/closeout chain, which still depends on a
  selected discriminator and selected_next_task that the accepted source
  contract did not provide.
- rejected: stale generic link-ready discriminator promotion, live packet I/O,
  hardware reachability, SSH service acceptance, public ABI/POSIX/Linux
  compatibility, broad socket expansion, and phase transition remain outside
  this closeout.
- removed: no code, tests, docs section, hardware helper, or task evidence was
  removed.
- not-an-issue: ending this slice with planningNeeded=true is consistent with
  the worker role because the supervisor owns creation of new Phase 12.5
  key-management, crypto, seed, or service-shape tasks.

## Reconciled Evidence

- Strategy contract:
  tasks/2026-06-21-phase12-entropy-ssh-strategy-contract.md.
- Entropy source contract:
  tasks/2026-06-21-phase12-entropy-source-contract.md.
- Entropy diagnostic implementation:
  tasks/2026-06-21-phase12-entropydiag-core.md.
- Docs:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

The accepted stack is prerequisite-first: Talos has a documented SSH-enabling
strategy, a bounded entropy source/diagnostic contract, and a source/unit
diagnostic classifier that fails closed. It still does not have accepted
cryptographic entropy, random-byte generation, seed persistence, key
management, crypto dependency integration, SSH host keys, an SSH server, live
transport reachability, or public socket/OS compatibility.

## Validation

- static task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: not run; no task-owned JSON evidence was
  created.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Closeout reconciles accepted work, deferred work, docs, validation, and risks:
  satisfied.
- Findings are recorded with disposition: satisfied.
- The task either selects one mechanically unblocked next task with objective
  dependencies or records planningNeeded=true with a concrete reason: satisfied;
  planningNeeded=true because no explicit queued Phase 12.5 follow-up task
  exists and the remaining queued link-ready discriminator chain is still
  objectively blocked.
- No SSH service, hardware reachability, public ABI/POSIX/Linux compatibility,
  broad expansion, or phase-transition claim is accepted: satisfied.

## Next Action

Set planningNeeded=true for supervisor planning. The next bounded task should
be explicitly planned before any key-management, crypto dependency, seed
persistence, host-key generation, SSH service, hardware reachability, live
packet I/O, public ABI/POSIX/Linux compatibility, broad expansion, or phase
transition work.
