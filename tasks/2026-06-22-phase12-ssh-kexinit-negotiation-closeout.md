# Phase 12.6 SSH KEXINIT negotiation closeout

Task id: phase12-ssh-kexinit-negotiation-closeout-20260622
Status: accepted
Owner: worker
Classification: phase12-ssh-kexinit-negotiation-closeout-accepted.

## Goal

Close out the bounded local SSH KEXINIT/algorithm-negotiation slice before any
actual key exchange, encryption, authentication, session/channel, shell,
hardware reachability, compatibility, broad expansion, or phase transition.

## Scope

- Reconciled the accepted KEXINIT/algorithm-negotiation contract, source/unit
  implementation, retained host/QEMU-substitute smoke evidence, redaction
  posture, docs, and deferred work.
- Confirmed the accepted frontier remains a local modeled sshservicediag
  surface over the accepted listener/transport path.
- Selected the next bounded task only because the queued runtime crypto
  contract has explicit dependencies, acceptance criteria, validation gates,
  docs, evidence requirements, and remains in the same Phase 12.6 slice.

## Findings

- fixed: The accepted KEXINIT slice is reconciled through the contract, core,
  smoke transcript, docs, redaction reviews, and commits.
- fixed: The accepted behavior is still local modeled evidence only: one
  cleartext SSH_MSG_KEXINIT packet model after the accepted local
  listener/transport and remote-identification path.
- fixed: The retained frontier enforces the accepted caps: <= 1024 byte packet,
  <= 768 byte payload, <= 256 byte name-list, <= 16 names per list, message
  number 20, 16-byte cookie shape with byte redaction, and fail-closed
  malformed, oversized, list-limit, unsupported-algorithm, disabled, and
  prerequisite-missing classifications.
- fixed: The modeled server KEXINIT cookie remains generated through the
  accepted operator-seeded CSPRNG boundary and immediately redacted/zeroized;
  retained evidence contains only fixed labels, counters, booleans, test names,
  and validation output.
- deferred: actual X25519/ECDH, shared-secret computation, Ed25519 host-key
  parsing/signing, packet encryption/MAC, NEWKEYS, authentication, session and
  channel behavior, PTY allocation, shell attachment, live transport,
  hardware reachability, and OpenSSH/POSIX/Linux compatibility.
- not-an-issue: Fixed Talos server policy names are retained as public local
  policy labels; client-provided packet bytes and algorithm-list text are not
  retained.
- not-an-issue: No Pi 5 hardware, lab-controller API action, boot publication,
  or hardwareTestLock acquisition is required for this host/QEMU-substitute
  closeout.

## Evidence

- Contract task:
  tasks/2026-06-22-phase12-ssh-kexinit-negotiation-contract.md
- Core task:
  tasks/2026-06-22-phase12-ssh-kexinit-negotiation-core.md
- Smoke task:
  tasks/2026-06-22-phase12-shell-ssh-kexinit-negotiation-smoke.md
- Retained smoke transcript:
  tasks/evidence/2026-06-22-ssh-kexinit-negotiation-smoke/qemu-shell-ssh-kexinit-negotiation-smoke.log
- Commits:
  - contract: 08680ec1
  - core: 0259c86c
  - smoke: ba06e744

## Redaction Review

Closeout retained no generated random bytes, KEX cookie bytes, client packet
bytes, client algorithm-list text, private host-key bytes, authorized-key
bytes, shared secrets, signatures, fingerprints, digests, peer addresses, peer
identification text, operator identity, key-derived identifiers, or stable
transport/session identifiers. The task record and docs retain only fixed
Talos policy labels, task ids, file paths, validation command names, and
bounded findings.

## Validation

- static task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: not applicable; no task-owned JSON
  evidence created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Validation levels: static inspection, docs build, and diff checks. No Rust
source, Cargo metadata, task-owned JSON evidence, Pi 5 hardware run,
lab-controller API action, boot publication, generated-root publication, live
packet I/O, hardware reachability, runtime SSH crypto, authentication/session
work, shell attachment, OpenSSH/POSIX/Linux compatibility, broad expansion, or
phase transition was performed.

## Acceptance

Accepted. The bounded local KEXINIT/algorithm-negotiation slice is reconciled
through accepted contract, implementation, smoke evidence, redaction review,
docs, and validation. ssh-ready remains false. No actual key exchange,
encryption/MAC, host-key signing, authentication/session success, shell
attachment, hardware reachability, public compatibility, broad expansion, or
phase transition is accepted.

selected_next_task=phase12-ssh-runtime-kex-crypto-contract-20260622.
