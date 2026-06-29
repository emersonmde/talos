# Phase 12 SSH Entropy Source Contract

Task id: phase12-ssh-entropy-source-contract-20260629

Status: accepted

Classification: ssh-entropy-source-contract-accepted

Evidence level: static source/docs/evidence review, task-owned JSON evidence,
docs build, and diff checks. No Rust source was changed. No live SSH, TCP,
packet I/O, host-key generation, cryptographic sufficiency acceptance, Pi 5
hardware proof, boot publication, generated-root retry, or phase transition was
performed.

## Goal

Classify the available entropy inputs for the SSH substrate and select a
bounded local diagnostic implementation path without claiming live SSH or
cryptographic readiness.

## Scope Performed

- Reviewed the existing entropy, CSPRNG, SSH key-readiness, diagnostic command,
  SSH runtime crypto, and SSH service-readiness sources.
- Classified candidate entropy inputs for eventual SSH host keys and session
  crypto.
- Selected the existing operator-provided seed material boundary as the only
  source-backed seed input for the next local diagnostic core.
- Recorded diagnostic output shape, source owner files, non-retention rules, and
  local/static validation gates for the next task.

## Findings

- fixed: The selected source contract is now explicit. Operator-provided seed
  material at /etc/talos/operator-seed.bin is the only selected seed input for
  the next local diagnostic core.
- fixed: The contract keeps src/entropy.rs, src/csprng.rs,
  src/ssh_key_readiness.rs, and src/diagnostic_command.rs as the selected owner
  files for the local entropy diagnostic boundary.
- not-an-issue: src/csprng.rs already has an operator-seeded ChaCha20 CSPRNG
  readiness boundary that zeroizes caller output on not-ready paths and does
  not print, digest, fingerprint, retain, or expose seed material or generated
  bytes through diagnostics.
- not-an-issue: src/entropy.rs already classifies local timer,
  scheduler-event, console-timing, deterministic-control, operator-seed, and
  hardware-RNG observations without sampling hardware or asserting SSH
  readiness.
- deferred: local timer, scheduler-event, and console-timing inputs remain
  untrusted diagnostic context only; they are not selected as cryptographic seed
  material.
- deferred: hardware RNG remains entropydiag-hardware-rng-unaccepted until a
  future source/hardware task owns and proves it.
- blocked: live SSH, host-key generation, packet I/O, TCP, Pi 5 proof, and
  cryptographic sufficiency for deployed SSH remain outside this source-contract
  task.
- removed: no source, helper, task, docs, or evidence file was removed.

## Candidate Source Dispositions

- selected: /etc/talos/operator-seed.bin metadata and bytes, owned through
  src/entropy.rs and src/csprng.rs. The local contract accepts only bounded
  metadata/readiness and CSPRNG not-ready/ready labels; durable evidence must
  not retain seed bytes, derived output, fingerprints, digests, signatures, or
  stable secret identifiers.
- deferred: local timer observations via EntropyDiagnosticSnapshot::with_timer.
  These can label entropydiag-untrusted-timer-only but cannot clear
  operator-seed-required or accept cryptographic strength.
- deferred: scheduler-event and console-timing observations. These can
  contribute to entropydiag-untrusted-local-mix but are not selected seed
  material.
- rejected: deterministic-control observations as production entropy. They are
  test/control labels only.
- blocked: hardware RNG observations. The current accepted label is
  entropydiag-hardware-rng-unaccepted; future selection requires separate
  source ownership and hardware evidence.

## Selected Contract

Invariant: the local entropy diagnostic must fail closed unless selected
operator seed metadata is present, and it must never make SSH-ready=true from
entropy alone.

Source owners:

- src/entropy.rs: entropy input labels, operator-seed metadata classifier, and
  diagnostic report labels.
- src/csprng.rs: operator-seeded CSPRNG readiness boundary and zeroized
  not-ready output handling.
- src/ssh_key_readiness.rs: SSH key-readiness dependency on entropy and seed
  material state.
- src/diagnostic_command.rs: public diagnostic output shape for entropy and
  sshkeydiag.

Diagnostic output shape:

- entropy: diag: entropy-label ..., diag: hardware-rng ..., optional
  diag: operator-seed ..., diag: cryptographic-strength ..., and
  diag: ssh-ready false.
- sshkeydiag: fixed sshkeydiag-* readiness labels, including entropy and
  seed-material blockers, plus diag: ssh-ready false.
- CSPRNG readiness labels for local/unit diagnostics: csprng-missing-seed,
  csprng-invalid-seed, csprng-insufficient-seed, csprng-conditioning-failed,
  and csprng-ready.

Non-retention rules:

- Do not write real operator seed bytes, generated random bytes, key bytes,
  fingerprints, digests, signatures, exchange hashes, session identifiers, or
  stable secret-derived identifiers to task records, JSON evidence, logs, docs,
  serial transcripts, or lab artifacts.
- Public deterministic test fixtures may remain in tests only when explicitly
  labeled non-secret and must not be used to claim production entropy.
- Durable evidence may retain only fixed labels, booleans, byte counts, bounded
  path names, and test/filter names.

Local/static validation gates selected for the next task:

- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test --quiet dispatcher_reports_entropy_diagnostic_fail_closed_without_crypto_claim.
- cargo -Zjson-target-spec test --quiet dispatcher_reports_ssh_key_readiness_fail_closed_without_secret_material.
- cargo -Zjson-target-spec test --quiet all_missing_default_reports_every_fail_closed_label.
- Targeted CSPRNG/entropy unit filters if the next task touches src/entropy.rs
  or src/csprng.rs.
- git diff --check, jq empty for task-owned JSON, and mdbook build if docs
  change.

## Decision

Selected next task:
phase12-ssh-entropy-diagnostic-local-core-20260629.

Planning needed: false.

Reason: the selected local diagnostic core is mechanically objective from the
contract above. It can either confirm the existing implementation and remove or
quarantine stale overclaiming helper behavior, or make a bounded local source
patch if the selected invariant is not fully represented.

## Rejected Claims

This task does not accept live SSH, TCP, packet I/O, host-key generation,
authorized-key provisioning, deployed cryptographic sufficiency, hardware RNG,
Pi 5 proof, boot publication, generated-root retry, OpenSSH retry, persistence,
or phase transition.

## Evidence

- Current task source contract:
  tasks/evidence/2026-06-29-phase12-ssh-entropy-source-contract/classification.json.
- Source/evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-entropy-source-contract/evidence-map.json.
- Entropy source owner: src/entropy.rs.
- CSPRNG source owner: src/csprng.rs.
- SSH key-readiness owner: src/ssh_key_readiness.rs.
- Diagnostic command owner: src/diagnostic_command.rs.
- Prior fail-closed smoke script: scripts/qemu-shell-sshkeydiag-smoke.sh.
- Phase 12 project note: docs/src/project/phase12-networking-ssh.md.
- Roadmap: docs/src/roadmap.md.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Candidate entropy inputs are classified as selected, deferred, rejected, or
  blocked with explicit reasons: satisfied.
- Selected contract states invariant, source owner files, diagnostic output
  shape, non-retention rules, and local/static validation gates: satisfied.
- Selected next task is phase12-ssh-entropy-diagnostic-local-core-20260629:
  satisfied.
- No live SSH, TCP, packet I/O, host key generation, cryptographic sufficiency,
  Pi 5 proof, or phase transition is accepted: satisfied.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static source/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass; search index size warning retained.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase12-ssh-entropy-diagnostic-local-core-20260629 on the next worker
wake if dependencies remain satisfied. Keep the next task local/static/unit
only; do not run hardware, generate or retain keys, accept live SSH readiness,
or use any real secret material in durable evidence.
