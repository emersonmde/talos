# russh 0.61.2 host-build API notes

Probe manifest:

- russh = "=0.61.2"
- default-features = false
- features = ["ring"]
- isolated probe workspace; no Talos root Cargo dependency graph change

Build classification input:

- cargo metadata: pass; retained in cargo-metadata.json.
- cargo tree -e features: pass; retained in cargo-tree-features.txt.
- cargo check: pass from outside the Talos repo directory so the host-only
  probe does not inherit Talos build-std/JSON-target Cargo config; retained in
  cargo-check.txt.

Feature/runtime observations:

- The allowed probe feature root is russh feature "ring"; the tree does not
  include the rejected russh default optional crates aws-lc-rs, flate2, rsa, or
  pkcs1.
- The checked graph still pulls std-oriented and host-runtime assumptions:
  tokio net/io-util/rt/rt-multi-thread/sync/time, russh-util default runtime,
  rand thread_rng, getrandom default/sys_rng/wasm_js, pkcs8 std/encryption, and
  ring default/dev_urandom_fallback.
- The graph contains 174 packages in metadata. That package count is host
  feasibility evidence only, not an accepted Talos runtime dependency set.

Server construction notes:

- russh::server::Config owns server_id, auth method set, authentication
  rejection timing, Vec<PrivateKey>, rekey limits, window and packet sizes,
  channel and event buffer sizes, preferred algorithms, authentication attempt
  limits, inactivity and keepalive timers, and nodelay.
- Config::default sets keys to an empty Vec, methods to MethodSet::all(),
  maximum_packet_size to 32768, channel_buffer_size to 100, event_buffer_size to
  10, max_auth_attempts to 10, inactivity_timeout to 600s, keepalive disabled,
  and nodelay false.
- Config Debug redacts private keys as "***", which is useful as a reference
  behavior, but Talos diagnostics must continue to avoid retaining key bytes or
  stable identifiers at all.

Transport/runtime notes:

- The high-level Server trait is tied to tokio::net::TcpListener through
  run_on_socket and run_on_address.
- run_stream is the only narrower seam: it accepts an Arc<Config>, a Handler,
  and a generic stream implementing tokio::io::AsyncRead + AsyncWrite + Unpin +
  Send + 'static. That is a useful conceptual protocol/transport split, but it
  is still a Tokio/std async I/O boundary, not a Talos runtime boundary.
- run_stream writes the SSH identification string, reads the peer
  identification, allocates tokio mpsc channels using event_buffer_size, starts
  rekey, and spawns the session through russh_util::runtime.

Authentication/session notes:

- Handler defaults fail closed for auth_none, auth_password, auth_publickey,
  auth_openssh_certificate, and keyboard-interactive by returning Auth::reject()
  or None where applicable.
- auth_publickey_offered defaults to Auth::Accept so the actual signed
  public-key check can happen later; Talos must not inherit that as an
  authorization decision without an explicit authentication contract.
- Session/channel hooks include channel_open_session, direct/forwarded TCP/IP,
  direct streamlocal, data, window adjustment, pty_request, env_request,
  shell_request, exec_request, subsystem_request, X11, agent forwarding, signal,
  and lifecycle callbacks. Talos has not accepted any remote session, PTY, shell,
  forwarding, or process-launch behavior.

Key/randomness notes:

- The example echoserver generates an Ed25519 host key with rand::rng() and uses
  tokio::net::TcpListener. That remains rejected for Talos runtime because it
  relies on ambient host randomness and host sockets.
- The host-check graph includes ring and getrandom/rand paths. This proves the
  allowed host-only feature set compiles, but it does not accept runtime SSH
  crypto, ambient randomness, generated keys, key parsing, signatures,
  fingerprints, digests, or stable operator/key/session identifiers.

Discriminator result:

- Classification: russh-host-build-probe-reference-only.
- Reason: the allowed feature set builds, and run_stream is a useful reference
  seam, but the checked graph and server APIs remain std/Tokio/host-RNG oriented
  enough that Talos should not plan a runtime adapter scaffold yet. The
  mechanically matching branch is a Talos-owned transport/banner contract.
