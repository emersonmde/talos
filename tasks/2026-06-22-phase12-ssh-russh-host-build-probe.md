# Phase 12.6 russh host build probe

Task id: phase12-ssh-russh-host-build-probe-20260622

Status: accepted.

Classification: russh-host-build-probe-reference-only.

## Goal

Run a task-owned host-only russh 0.61.2 build and API discriminator before any
Talos runtime dependency adoption or listener work.

## Scope

- Create an isolated host-only probe under
  tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/.
- Use only russh 0.61.2 with default-features=false and features=["ring"] for
  the probe.
- Retain cargo metadata, cargo tree feature output, cargo check output, exact
  feature leakage, and API notes.
- Classify exactly one result and select the mechanically matching branch task.

## Non-goals

- No Talos runtime dependency adoption, root/kernel Cargo dependency graph
  change, SSH listener, socket bind/accept, packet processing, runtime crypto
  acceptance, authentication, session/channel execution, PTY allocation, shell
  attachment, hardware/lab action, boot publication, reachability claim, public
  ABI/POSIX/Linux compatibility, broad expansion, or phase transition.
- No russh default features, aws-lc-rs/flate2/rsa/default leakage acceptance,
  host tokio::net listener integration, ambient host randomness, generated keys,
  private key bytes, authorized-key bytes, fingerprints, digests, signatures,
  operator identity, key-derived identifiers, stable transport/session
  identifiers, or comparable secret identifiers in retained evidence.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-service-readiness-closeout.md
- tasks/2026-06-22-phase12-ssh-implementation-dependency-feasibility-contract.md
- tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- russh 0.61.2 local registry source:
  - src/server/mod.rs
  - examples/echoserver.rs
  - Cargo.toml

## Probe

The probe is an isolated Cargo workspace at
tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/. Its manifest uses
only:

    russh = { version = "=0.61.2", default-features = false, features = ["ring"] }

The probe library compiles a minimal Handler implementation and references the
server Config and fail-closed Auth surfaces. It does not generate keys, open a
socket, parse keys, retain key material, or wire russh into the Talos root Cargo
graph.

The successful host check had to run from outside the Talos repo directory so
the host-only probe did not inherit Talos build-std/JSON-target Cargo config.
That is an environment constraint on the probe, not a runtime acceptance claim.

## API Notes

Retained API notes are in
tasks/evidence/2026-06-22-ssh-russh-host-build-probe/api-notes.md.

Summary:

- The allowed host-only feature set builds and checks with russh 0.61.2,
  default-features=false, features=["ring"].
- The tree does not include the rejected russh default optional crates
  aws-lc-rs, flate2, rsa, or pkcs1.
- The checked graph still pulls std/Tokio/host-runtime assumptions: tokio
  net/io-util/rt/rt-multi-thread/sync/time, russh-util default runtime, rand
  thread_rng, getrandom default/sys_rng/wasm_js, pkcs8 std/encryption, and ring
  default/dev_urandom_fallback.
- russh::server::Config owns host-key storage, method set, auth timing, packet
  and channel caps, algorithm preferences, auth attempt limits, inactivity and
  keepalive timers, and socket nodelay policy.
- run_on_socket/run_on_address are tied to tokio::net::TcpListener.
- run_stream is a useful reference seam because it takes an Arc<Config>, a
  Handler, and a generic tokio::io AsyncRead + AsyncWrite stream, but it still
  allocates tokio channels, uses russh_util::runtime, and relies on a
  std/Tokio async I/O boundary.
- Handler defaults mostly fail closed, but auth_publickey_offered defaults to
  Auth::Accept as a probe path before the signed public-key check; Talos must
  not inherit that as an authorization policy.
- The example server uses rand::rng() for host-key generation and
  tokio::net::TcpListener, both rejected for Talos runtime.

## Findings

- fixed: created an isolated task-owned probe without modifying the Talos root
  Cargo graph.
- fixed: proved the allowed host-only russh 0.61.2 feature set builds/checks on
  the host when run outside Talos build-std/JSON-target Cargo config.
- fixed: retained cargo metadata, feature tree, check output, feature summary,
  and API notes.
- fixed: classified the result as russh-host-build-probe-reference-only because
  the graph and APIs remain std/Tokio/host-RNG oriented despite the successful
  host build.
- deferred: any runtime russh dependency adoption, fork/port, runtime crypto
  backend, host-key loading/parsing, authorized-key parsing, authentication,
  listener/transport, session/channel execution, PTY allocation, shell
  attachment, reachability proof, public ABI/POSIX/Linux compatibility, and
  broad feature expansion.
- not-an-issue: the retained evidence contains public package names, versions,
  paths, compile logs, feature names, API summaries, fixed labels, and task ids;
  it contains no key bytes, generated random bytes, fingerprints, digests,
  signatures, operator identity, key-derived identifiers, stable
  transport/session identifiers, or comparable secret identifiers.

## Evidence

- Probe manifest and source:
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/Cargo.toml
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/Cargo.lock
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/src/lib.rs
- cargo metadata:
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/cargo-metadata.json
- cargo tree -e features:
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/cargo-tree-features.txt
- cargo check:
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/cargo-check.txt
- Feature summary:
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/feature-summary.txt
- API notes:
  - tasks/evidence/2026-06-22-ssh-russh-host-build-probe/api-notes.md

## Validation

- cargo metadata --manifest-path
  tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/Cargo.toml:
  pass.
- cargo tree -e features --target x86_64-unknown-linux-gnu --manifest-path
  tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/Cargo.toml:
  pass.
- cargo check --target x86_64-unknown-linux-gnu --manifest-path
  tasks/evidence/2026-06-22-ssh-russh-host-build-probe/probe/Cargo.toml:
  pass.
- jq empty on cargo-metadata.json: pass.
- cargo fmt --all -- --check: not run; no Rust source outside task evidence was
  touched.
- cargo -Zjson-target-spec test --quiet: not run; Talos source and root Cargo
  metadata were not touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- Retained evidence proves the allowed host-only russh 0.61.2 feature set
  builds/checks and records the exact transitive feature set.
- API notes identify concrete runtime integration blockers and the one useful
  reference seam, run_stream, without accepting it as a Talos runtime adapter.
- Findings are recorded with disposition.
- Classification is russh-host-build-probe-reference-only.
- selected_next_task=phase12-ssh-owned-transport-banner-contract-20260622.
- No listener/transport/authentication/session/reachability claim is accepted.
- No secret/key/random bytes or stable secret/operator/transport/session
  identifiers are retained.

selected_next_task=phase12-ssh-owned-transport-banner-contract-20260622.
