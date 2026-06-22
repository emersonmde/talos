# Phase 12.5 SSH implementation dependency feasibility contract

Task id: phase12-ssh-implementation-dependency-feasibility-contract-20260622

Status: accepted.

Classification: phase12-ssh-implementation-dependency-feasibility-contract-accepted.

## Goal

Define the selected russh 0.61.2 dependency/source boundary after the accepted
SSH implementation strategy ADR, without adopting a dependency or implementing
SSH service behavior.

## Scope

- Use phase12-ssh-implementation-strategy-adr-20260622 as the authority for
  the selected candidate path.
- Inspect only russh 0.61.2 dependency metadata, source shape, feature surface,
  examples, and docs needed to define a Talos-safe boundary for the next
  service-shape task.
- Record exact allowed features, rejected features/backends, no_std/alloc/libc
  constraints, crypto requirements, heap-pressure expectations, and failure
  modes.
- Select the next fail-closed service-shape contract if the boundary is
  feasible.

## Non-goals

- No Cargo.toml/Cargo.lock edits, dependency adoption, broad crate import,
  source implementation, SSH listener, handshake, authentication,
  channel/session handling, PTY plumbing, service lifecycle implementation,
  live transport, packet I/O, hardware/lab action, boot publication,
  reachability claim, or public ABI/POSIX/Linux compatibility claim.
- No secret/key/random/operator identifier retention.
- No host OS network/RNG backend, default feature leakage, stale link-ready
  discriminator promotion, broad expansion, or phase transition.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md
- docs/src/decisions/README.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- russh 0.61.2 crates.io/local registry metadata at
  ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/russh-0.61.2/Cargo.toml
- russh 0.61.2 README metadata and examples/echoserver.rs from the local
  registry source.
- russh 0.61.2 src/lib.rs, src/lib_inner.rs, src/server/mod.rs, and
  src/sshbuffer.rs from the local registry source.

## Contract

Talos keeps russh 0.61.2 as the selected existing-library candidate, but the
accepted boundary is source/dependency feasibility only. Talos does not adopt
russh as a Cargo dependency in this slice.

The allowed near-term use of russh is:

- source/reference inspection for service-shape, algorithm, packet-buffer,
  channel, authentication, and failure-mode modeling;
- later host-only build experiments only if separately tasked, using
  default-features=false with exactly features=["ring"] as the first probe
  set. That feature set is allowed only to satisfy russh's compile-time backend
  requirement in a host-only feasibility probe, not as accepted Talos runtime
  crypto;
- no compression, RSA, DES, serde, async-trait compatibility layer,
  legacy-ed25519-pkcs8-parser, SFTP, examples, dev-dependencies, benches, or
  host socket/service helpers in any future Talos runtime slice unless a later
  task separately accepts them.

The rejected runtime boundary is:

- no direct Talos runtime dependency on russh 0.61.2 yet;
- no default feature set, because it enables flate2, aws-lc-rs, and rsa;
- no tokio::net listener or host TcpStream/TcpListener integration;
- no tokio runtime/process/dev-test assumptions as service readiness;
- no rand thread_rng, getrandom host/wasm_js backend, generated host keys, or
  ambient OS randomness;
- no pkcs8 std/encryption parsing path as a readiness prerequisite;
- no aws-lc-rs backend in the first probe because the accepted strategy already
  rejected default feature leakage and the default backend would import a broad
  external crypto/build surface before Talos has a runtime dependency contract;
- no CryptoVec/mlock/munlock security claim until Talos has an explicit memory
  protection and allocation-failure policy for sensitive buffers;
- no OpenSSH forwarding, SFTP, agent forwarding, Pageant, PPK, certificate,
  password, keyboard-interactive, direct-tcpip, or streamlocal feature
  acceptance in the first service-shape slice.

This boundary is feasible for the next service-shape contract because russh's
server module exposes the right conceptual seams: a server Config with methods,
keys, limits, window and packet sizes, channel and event buffers, inactivity and
keepalive timers, a Handler trait for authentication/channel events, and a
run_stream path that can conceptually separate SSH protocol state from the
listener. Talos may model those inputs and fail-closed lifecycle states without
linking russh, opening a listener, or accepting transport.

## Integration Constraints

- no_std/alloc/libc/std: Talos may use alloc-backed vectors and explicit
  metadata in its own model, but russh itself is std/tokio-oriented today.
  Any future adoption requires a supervisor-planned port/adaptation task that
  proves std, tokio time/sync/io, and error-path assumptions are contained.
- crypto: exactly one backend is required by russh, but no backend is accepted
  for Talos runtime here. The only allowed separately tasked host-probe backend
  is ring; aws-lc-rs is rejected for the first probe. Future runtime backend
  evaluation must bind to the accepted Talos CSPRNG/entropy path and must
  reject ambient host RNG fallback.
- process and TTY/PTTY: russh session/channel callbacks may inform the service
  shape, but Talos has not accepted remote PTY allocation, shell attachment, or
  process launch over SSH.
- VFS: accepted read-only metadata for operator seed, host key, authorized
  keys, and exposure marker may feed only readiness modeling. No key parsing,
  key loading, or authorized-key matching is accepted.
- smoltcp/network: live packet I/O and TCP accept remain unaccepted. The first
  service-shape task must keep transport disabled and report that fact.
- heap pressure: packet buffers, channel windows, event buffers, key material,
  and algorithm negotiation require explicit caps. The next contract must name
  diagnostic caps rather than inheriting russh defaults as Talos runtime
  policy.
- failure modes: the model must fail closed for missing dependency adoption,
  disabled exposure, missing transport, missing service implementation, missing
  authentication/session/PTTY support, crypto backend unavailable, entropy not
  cryptographically strong, malformed material metadata, excessive buffer caps,
  and any attempt to expose secret/key/random bytes or stable identifiers.

## Findings

- fixed: restricted russh to a source/reference and future explicitly tasked
  build-probe boundary; no Cargo dependency adoption occurs here.
- fixed: rejected russh default features for Talos runtime because they enable
  compression, aws-lc-rs, and rsa without a Talos-owned dependency contract.
- fixed: rejected host tokio::net listener, host TcpStream/TcpListener,
  tokio runtime/process/dev-test assumptions, rand thread_rng, getrandom host
  or wasm_js backends, generated keys, and ambient OS randomness.
- fixed: named default-features=false with features=["ring"] as the only
  allowed future host-only build-probe feature set; this is not runtime crypto
  acceptance.
- deferred: any direct russh runtime dependency, fork, port, or build probe
  until a later task proves std/tokio/crypto/RNG assumptions are contained.
- deferred: compression, RSA, DES, serde, async-trait, legacy key parsers,
  forwarding, SFTP, agent/Pageant/PPK/cert extras, password auth,
  keyboard-interactive auth, remote PTY, shell attachment, and live transport.
- deferred: CryptoVec/mlock/munlock memory-protection claims until Talos has an
  explicit sensitive-buffer and allocation-failure policy.
- not-an-issue: selecting phase12-ssh-service-shape-contract-20260622 as the
  next task does not make ssh-ready true. SSH service behavior, live transport,
  reachability, authentication/session success, and ABI/POSIX/Linux
  compatibility remain unaccepted.
- not-an-issue: retained evidence contains only public dependency metadata,
  source file paths, versions, labels, and boundary decisions; it contains no
  secret/key/random bytes or stable secret/operator identifiers.

## Validation

- static source/task/docs/evidence review: pass.
- public dependency/source metadata review for selected candidate: pass;
  reviewed cargo info russh@0.61.2 --offline plus russh 0.61.2 Cargo.toml,
  README, echoserver example, lib.rs, lib_inner.rs, server/mod.rs, and
  sshbuffer.rs from local registry source.
- cargo metadata --offline: not run; no Cargo files touched.
- cargo -Zjson-target-spec test --quiet: not run; no Cargo or source files
  touched.
- cargo fmt --all -- --check: not run; no Rust source touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- Contract records selected candidate dependency/source boundary, allowed
  features, rejected features/backends, and Talos integration constraints.
- Findings are recorded with disposition and
  selected_next_task=phase12-ssh-service-shape-contract-20260622.
- Fail-closed behavior is preserved: no SSH-ready claim, listener,
  authentication, transport, reachability, or ABI/POSIX/Linux compatibility is
  accepted.
- No secret/key/random bytes or stable secret/operator identifiers are retained.

selected_next_task=phase12-ssh-service-shape-contract-20260622.
