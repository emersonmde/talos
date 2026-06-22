# Phase 12.5 SSH implementation strategy ADR

Task id: phase12-ssh-implementation-strategy-adr-20260622

Status: accepted.

Classification: phase12-ssh-implementation-strategy-adr-accepted.

## Goal

Choose the first concrete SSH implementation strategy after accepted entropy,
CSPRNG, host-key metadata, authorized-key metadata, and persistence/exposure
metadata readiness, without implementing service behavior or adopting a new
dependency in this slice.

## Scope

- Review accepted Phase 12.5 readiness evidence, current Phase 10 process/TTY/VFS
  shell boundaries, accepted smoltcp notes, and the roadmap requirement to
  evaluate porting before writing an SSH server.
- Compare OpenSSH, a smaller Rust SSH server/library candidate, and a minimal
  Talos-owned service path against Talos no_std/alloc/libc/std, crypto,
  process, TTY/PTTY, VFS, smoltcp/network, and failure-mode constraints.
- Record a reversible strategy decision with findings disposition.
- Select exactly one objective dependency/feasibility task.

## Non-goals

- No Cargo dependency changes, source implementation, SSH listener, handshake,
  authentication, channel/session handling, PTY plumbing, service lifecycle
  implementation, live transport, packet I/O, hardware/lab action, boot
  publication, hardware reachability, or public ABI/POSIX/Linux compatibility
  claim.
- No generated host keys, authorized-key parsing, private/public key validation,
  fingerprints, signatures, digests, generated random byte streams, private
  CSPRNG state, operator identity, key-derived identifiers, or comparable stable
  identifiers in docs, diagnostics, logs, task records, or retained evidence.
- No stale link-ready discriminator promotion, broad networking expansion, or
  phase transition.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-persistence-exposure-readiness-closeout.md
- tasks/2026-06-22-phase12-shell-ssh-persistence-exposure-diag-smoke.md
- tasks/2026-06-22-phase12-ssh-persistence-exposure-vfs-core.md
- tasks/2026-06-21-phase12-ssh-authorized-key-readiness-closeout.md
- tasks/2026-06-21-phase12-ssh-host-key-readiness-closeout.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- docs/src/architecture/tty-stdio.md
- docs/src/architecture/lower-el-userspace.md
- crates.io metadata and local registry source for russh 0.61.2 and
  thrussh 0.41.0.
- OpenSSH portable INSTALL public source metadata.

## Strategy Decision

OpenSSH remains the compatibility target for user-observable behavior, wire
protocol expectations, client interoperability, and later regression testing.
It is not the first Talos port target. The first implementation strategy is to
evaluate a smaller Rust SSH server/library boundary, with russh 0.61.2 as the
first candidate for dependency feasibility. The follow-up contract must decide
whether Talos can carve a safe subset from that candidate without importing
host OS networking, ambient randomness, std/Tokio runtime assumptions, broad
algorithm surfaces, or secret-retaining diagnostics.

If the russh feasibility contract cannot define that boundary, the next
supervisor-planned option should be a minimal Talos-owned SSH service skeleton
that implements only the service lifecycle and diagnostics first, still using
OpenSSH as the compatibility oracle and still deferring live SSH readiness. A
direct OpenSSH portable port is deferred until Talos has a materially stronger
libc/POSIX, process/session, PTY, filesystem, privilege/user, and crypto
runtime substrate.

The selected next task is
phase12-ssh-implementation-dependency-feasibility-contract-20260622.

## Candidate Comparison

### OpenSSH Portable

- Compatibility: best target and future oracle for client-visible behavior.
- Talos fit today: poor first port. Public INSTALL metadata requires a C build
  environment and privilege separation support, and lists zlib plus libcrypto
  from LibreSSL/OpenSSL/AWS-LC/BoringSSL as important dependencies. OpenSSH also
  assumes a mature POSIX-style environment for files, users, sessions, PTY,
  sockets, process management, randomness, and privilege boundaries.
- Decision: deferred as implementation target; retained as compatibility target.

### russh 0.61.2

- Compatibility: implements client and server SSH with common OpenSSH-facing
  algorithms and server examples.
- Talos fit today: plausible but unproven. crates.io and local registry metadata
  show default features pull flate2, aws-lc-rs, and rsa; README metadata says at
  least one crypto backend feature is required. The crate depends on tokio
  io-util/sync/time, futures, bytes, rand with thread_rng, getrandom with
  wasm_js, pkcs8 with std, ssh-key, multiple crypto algorithm crates, and
  cryptovec-style sensitive buffers. Those are serious Talos boundary risks but
  also better-scoped than porting a whole C OpenSSH userspace stack.
- Decision: selected for the next feasibility contract only. No dependency
  adoption occurs in this task.

### thrussh 0.41.0

- Compatibility: older Rust client/server library and the ancestor of russh.
- Talos fit today: weaker than russh as first candidate. Metadata shows tokio
  features including net, rt-multi-thread, time, sync, macros, process, plus
  rand, ssh-libsodium, futures, flate2 default, and optional openssl. It appears
  more tied to host runtime behavior and older dependency surfaces.
- Decision: deferred; use only as fallback reference if russh feasibility fails
  for a reason thrussh demonstrably avoids.

### Minimal Talos-Owned Service

- Compatibility: highest control over no_std, smoltcp, VFS, process, TTY/PTTY,
  and failure states, but lowest protocol completeness.
- Talos fit today: practical as a fail-closed service-shape diagnostic and
  lifecycle model, but too risky as the first protocol implementation because it
  would hand-roll SSH before proving an existing-library boundary is impossible.
- Decision: deferred behind the russh feasibility contract; may become the next
  strategy only if existing-server/library feasibility blocks.

## Findings

- accepted: OpenSSH is the compatibility target, not the first implementation
  port.
- accepted: russh 0.61.2 is the first feasibility candidate because it is a
  maintained Rust client/server SSH library with server examples and a narrower
  integration surface than OpenSSH portable.
- deferred: direct OpenSSH porting until Talos has stronger libc/POSIX,
  process/session, PTY, filesystem, privilege/user, and crypto substrate.
- deferred: thrussh 0.41.0 because its metadata indicates older and broader
  host-runtime dependencies than russh.
- deferred: minimal Talos-owned protocol implementation until the existing
  library feasibility check either blocks or defines an unusable boundary.
- not-an-issue: selecting a dependency feasibility contract does not relax
  ssh-ready. SSH service behavior, live transport, reachability, and
  public ABI/POSIX/Linux compatibility remain unaccepted.
- not-an-issue: no secret/key/random bytes or comparable identifiers are
  retained; this task records only public metadata, labels, paths, versions, and
  boundary decisions.

## Validation

- static source/task/docs/evidence review: pass.
- public dependency/source metadata review: pass; reviewed crates.io cargo info
  for russh 0.61.2 and thrussh 0.41.0, local registry Cargo.toml metadata for
  both crates, russh README/Cargo.toml public metadata, and OpenSSH portable
  INSTALL public metadata.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- ADR/decision-index entry records the SSH implementation strategy and why
  non-selected options are rejected or deferred.
- Decision covers OpenSSH compatibility target, russh and thrussh Rust
  candidates, minimal Talos-owned service path, and Talos no_std/alloc/libc/std,
  crypto, process, TTY/PTTY, VFS, smoltcp/network, and failure-mode constraints.
- Findings are recorded with disposition.
- selected_next_task=phase12-ssh-implementation-dependency-feasibility-contract-20260622.
- No SSH service behavior, live transport, hardware reachability, public
  ABI/POSIX/Linux compatibility, or secret/key/random/operator identifier
  retention is accepted.
