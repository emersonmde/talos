# Phase 12.6 SSH service readiness closeout

Task id: phase12-ssh-service-readiness-closeout-20260622

Status: accepted.

Classification: phase12-ssh-service-readiness-closeout-planning-needed.

## Goal

Close out the first fail-closed SSH service readiness diagnostic slice by
reconciling accepted strategy, dependency feasibility, service-shape,
diagnostic implementation, validation evidence, redaction posture, deferred
work, and the next planning boundary.

## Scope

- Reconcile accepted strategy, dependency feasibility, service-shape contract,
  diagnostic implementation, validation, docs, deferred work, and redaction
  posture.
- Record findings with disposition.
- Select exactly one objective next task if accepted evidence makes a
  listener/transport prerequisite mechanically unblocked, or record
  planningNeeded=true with a concrete reason.

## Non-goals

- No new SSH listener, socket bind/accept, handshake, authentication success,
  channel/session execution, PTY allocation, shell attachment, live transport,
  hardware/lab action, boot publication, hardware reachability, public
  ABI/POSIX/Linux compatibility, stale link-ready discriminator promotion,
  broad expansion, or phase transition.
- No secret/key/random bytes or stable secret/operator identifiers retained in
  closeout evidence.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md
- tasks/2026-06-22-phase12-ssh-implementation-dependency-feasibility-contract.md
- tasks/2026-06-22-phase12-ssh-service-shape-contract.md
- tasks/2026-06-22-phase12-ssh-service-readiness-diagnostic-core.md
- tasks/evidence/2026-06-22-ssh-service-readiness-diagnostic-core/qemu-shell-ssh-service-diag-smoke.log
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- src/ssh_service_readiness.rs
- src/diagnostic_command.rs

## Closeout

The accepted slice now covers the first fail-closed SSH service readiness
model and diagnostic:

- OpenSSH remains the compatibility target and future behavior oracle.
- russh 0.61.2 remains source/reference material and an allowed future
  host-only build-probe candidate only when separately tasked.
- Talos has an accepted service-shape contract that models readiness from
  existing CSPRNG, host-key metadata, authorized-key metadata, persistence
  metadata, and exposure opt-in inputs.
- Talos has a source/unit sshservicediag classifier and dispatcher path for
  disabled, prerequisites-missing, and shape-modeled/no-transport states.
- sshservicediag keeps dependency adoption, runtime SSH crypto,
  listener/transport, authentication, session, shell attachment, and
  reachability unaccepted with fixed labels, zero counters, false caps, and
  ssh-ready false.

No next listener, transport, crypto-backend, authentication, session, or shell
prerequisite task is mechanically unblocked from the current task queue. The
next step needs supervisor planning because the repo has no explicit queued
task with complete scope, non-goals, acceptance criteria, validation gates,
docs requirements, and evidence requirements for dependency adoption, runtime
crypto backend selection, listener/transport modeling, authentication/session
shape, or shell attachment.

planningNeeded=true.

planningReason=Accepted SSH service readiness diagnostic closeout has no
queued or ready follow-up task after currentIndex 414. Supervisor planning is
required before dependency adoption, runtime SSH crypto, listener/transport,
authentication/session behavior, shell attachment, hardware reachability,
public ABI/POSIX/Linux compatibility, broad expansion, or phase transition.

selected_next_task=null.

## Findings

- fixed: reconciled the accepted strategy, dependency feasibility,
  service-shape, and diagnostic implementation as one fail-closed service
  readiness slice.
- fixed: confirmed the diagnostic frontier remains labels/booleans/zero
  counters only and keeps ssh-ready false.
- fixed: recorded that no explicit queued listener/transport prerequisite task
  is mechanically unblocked.
- deferred: runtime russh dependency adoption or fork/port, runtime SSH crypto
  backend, host-key loading/parsing, authorized-key parsing, user/account
  policy, listener/TCP accept, packet processing, authentication, channel and
  session execution, PTTY allocation, shell attachment, reachability proof,
  and runtime buffer/window/algorithm limits.
- not-an-issue: the retained diagnostic evidence contains only fixed labels,
  booleans, zero counters, task ids, paths, and validation commands; it does
  not retain secret/key/random bytes or stable secret/operator identifiers.

## Evidence

- Accepted strategy ADR:
  - tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md.
- Accepted dependency/source feasibility contract:
  - tasks/2026-06-22-phase12-ssh-implementation-dependency-feasibility-contract.md.
- Accepted service-shape contract:
  - tasks/2026-06-22-phase12-ssh-service-shape-contract.md.
- Accepted diagnostic core:
  - tasks/2026-06-22-phase12-ssh-service-readiness-diagnostic-core.md.
- Retained host/QEMU-substitute smoke evidence:
  - tasks/evidence/2026-06-22-ssh-service-readiness-diagnostic-core/qemu-shell-ssh-service-diag-smoke.log.
- Static task/docs/evidence review: pass.
- Redaction review: retained closeout evidence names only public paths, task
  ids, labels, booleans, zero counters, and validation commands.

## Validation

- static task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: not run; no JSON evidence was created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- Closeout reconciles accepted strategy, dependency feasibility, service-shape,
  diagnostic implementation, validation, docs, deferred work, and redaction
  posture for the first SSH service readiness diagnostic slice.
- Findings are recorded with disposition.
- planningNeeded=true is recorded with a concrete reason because no explicit
  queued listener/transport prerequisite task is mechanically unblocked.
- No live SSH connection, listener/transport reachability,
  authentication/session success, shell attachment, hardware reachability,
  public ABI/POSIX/Linux compatibility, broad expansion, or phase-transition
  claim is accepted.
- No secret/key/random bytes or stable secret/operator identifiers are
  retained.

selected_next_task=null.
