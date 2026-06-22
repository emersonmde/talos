# Phase 12.6 SSH dependency path closeout

Task id: phase12-ssh-dependency-path-closeout-20260622

Status: accepted.

Classification: phase12-ssh-dependency-path-closeout-accepted.

## Goal

Close out the selected SSH dependency/transport path slice before broader
listener, authentication, session, shell, hardware reachability, or
phase-transition work.

## Scope

- Reconcile the accepted russh probe, selected branch contract, selected
  implementation task, docs, validation, deferred work, and redaction posture.
- Record findings with disposition.
- Select exactly one objective next task if evidence makes it mechanically
  unblocked, or record planningNeeded=true with a concrete reason.

## Reconciled Evidence

- phase12-ssh-russh-host-build-probe-20260622 accepted the host-only russh
  0.61.2 build/API discriminator. The allowed feature set
  default-features=false with features=["ring"] builds/checks in an isolated
  host probe and excludes the rejected russh default optional crates, but the
  retained graph and APIs still assume std/Tokio/host-runtime behavior. The
  classification is russh-host-build-probe-reference-only.
- phase12-ssh-owned-transport-banner-contract-20260622 accepted the
  reference-only branch contract for a Talos-owned SSH identification/banner
  model over accepted private socket/readiness surfaces only. It fixed the
  local identification literal, remote line limits, fail-closed pre-KEX close,
  diagnostic labels, redaction rules, and forbidden compatibility claims.
- phase12-ssh-owned-transport-banner-core-20260622 accepted the first
  Talos-owned SSH identification/banner implementation slice. The core adds
  the fixed SSH-2.0-Talos_0.1 CRLF local literal, bounded remote
  identification classification, fixed labels, and the
  transport-closed-before-kex outcome while keeping ssh-ready false.

The selected path is therefore Talos-owned banner modeling with russh retained
as reference-only material. No runtime russh dependency adoption, runtime SSH
crypto, listener/transport reachability, authentication success,
channel/session execution, shell attachment, hardware reachability,
OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition is
accepted by this closeout.

## Findings

- fixed: Reconciled the accepted russh probe, owned banner contract, and owned
  banner core into one current dependency/transport-path frontier.
- fixed: Recorded that russh remains reference-only for the Talos runtime
  because the accepted host probe still carries std/Tokio/host-runtime
  assumptions despite a successful isolated host build/check.
- fixed: Recorded that the current implemented SSH-shaped behavior is limited
  to deterministic identification/banner modeling and fail-closed pre-KEX
  close, with ssh-ready false and transport/crypto/authentication/session/shell
  readiness still unaccepted.
- deferred: Any next SSH step that changes runtime service behavior,
  dependency adoption, crypto backend, listener/transport integration,
  authentication, session/channel execution, PTY allocation, shell attachment,
  hardware reachability, OpenSSH compatibility proof, public ABI/POSIX/Linux
  compatibility, or phase-transition status requires supervisor planning.
- not-an-issue: No task-owned JSON evidence was created, so the jq JSON
  validation gate is not applicable.
- not-an-issue: Retained closeout evidence references public task ids, fixed
  labels, fixed command names, fixed package names/versions, paths, booleans,
  zero counters, and validation commands; it does not retain key bytes,
  authorized-key bytes, random bytes, fingerprints, digests, signatures,
  operator identity, key-derived identifiers, peer identification text, peer
  addresses, stable transport/session identifiers, or comparable identifiers.

## Validation

- static task/docs/evidence review: pass. Reviewed
  tasks/2026-06-22-phase12-ssh-russh-host-build-probe.md,
  tasks/2026-06-22-phase12-ssh-owned-transport-banner-contract.md,
  tasks/2026-06-22-phase12-ssh-owned-transport-banner-core.md,
  tasks/evidence/2026-06-22-ssh-russh-host-build-probe/api-notes.md,
  docs/src/project/phase12-networking-ssh.md, docs/src/roadmap.md, and
  docs/src/decisions/README.md.
- jq empty on task-owned JSON evidence: not applicable; no JSON evidence was
  created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No cargo test was required because no Rust source or Cargo metadata changed.
No hardwareTestLock acquisition, lab mutation, hardware run, boot publication,
generated-root publication, live packet I/O, hardware reachability, runtime
SSH crypto, authentication/session work, OpenSSH/POSIX/Linux compatibility
claim, broad expansion, or phase transition was performed.

## Acceptance

- Closeout reconciles selected dependency/transport branch evidence, docs,
  validation, deferred work, and risks.
- Findings are recorded with disposition.
- No unaccepted listener, authentication, session, shell, reachability,
  compatibility, broad expansion, or phase-transition claim is accepted.
- selected_next_task=null and planningNeeded=true because no queued or ready
  task exists after this closeout, and the next SSH service slice must be
  planned with explicit scope, dependencies, acceptance criteria, validation
  gates, docs, evidence, and non-goals before work continues.

selected_next_task=null.
