# Phase 12.6 SSH KEXINIT negotiation contract

Task id: phase12-ssh-kexinit-negotiation-contract-20260622

Status: accepted.

Classification: phase12-ssh-kexinit-negotiation-contract-accepted.

## Goal

Define the first bounded Talos-owned SSH KEXINIT and algorithm-negotiation
surface after the accepted local listener/transport slice, without accepting
actual key exchange, encryption, authentication, session, shell, hardware
reachability, public compatibility, broad expansion, or a phase transition.

## Scope

- Review the accepted listener/transport, owned banner, CSPRNG, host-key
  metadata, authorized-key metadata, persistence/exposure, and OpenSSH
  compatibility-target evidence.
- Select one bounded first KEXINIT negotiation surface for a local modeled SSH
  connection: packet framing limits, algorithm-list policy, CSPRNG cookie
  handling and redaction, client-list matching behavior, failure labels, and
  the close-before-ECDH boundary.
- Record findings with disposition and select the next implementation task only
  if objective source/unit-test scope is clear.

## Non-goals

- No code behavior change in this contract task.
- No actual ECDH/key exchange, shared-secret computation, encryption/MAC
  enablement, NEWKEYS, host-key signing, authentication success,
  session/channel execution, PTY allocation, shell attachment, hardware/lab
  action, boot publication, hardware reachability, OpenSSH/POSIX/Linux
  compatibility claim, broad socket expansion, broad SSH service expansion,
  stale link-ready discriminator work, broad expansion, or phase transition.
- No generated random bytes, KEX cookie bytes, private host-key bytes,
  authorized-key bytes, shared secrets, fingerprints, digests, peer addresses,
  peer identification text, operator identity, key-derived identifiers, stable
  transport/session identifiers, or client-provided algorithm-list text
  retained in docs or evidence.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-listener-transport-closeout.md
- tasks/2026-06-22-phase12-ssh-listener-transport-core.md
- tasks/2026-06-22-phase12-shell-ssh-listener-transport-smoke.md
- tasks/2026-06-22-phase12-ssh-listener-transport-contract.md
- tasks/2026-06-21-phase12-operator-seeded-csprng-closeout.md
- tasks/2026-06-21-phase12-ssh-host-key-readiness-closeout.md
- tasks/2026-06-21-phase12-ssh-authorized-key-readiness-closeout.md
- tasks/2026-06-22-phase12-ssh-persistence-exposure-readiness-closeout.md
- tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md
- tasks/2026-06-22-phase12-ssh-implementation-dependency-feasibility-contract.md
- tasks/2026-06-22-phase12-ssh-russh-host-build-probe.md
- src/ssh_service_readiness.rs
- src/ssh_key_readiness.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Contract

Talos adds only a local modeled KEXINIT negotiation step after the accepted
local listener/transport exchange. The model remains host/QEMU-substitute
source/unit evidence over the accepted private descriptor-backed socket
surface. It must not claim hardware reachability, public socket ABI
compatibility, OpenSSH compatibility, or ssh-ready.

The ordered exchange for the next implementation is:

1. Reuse the accepted disabled, prerequisite-missing, and shape-modeled
   sshservicediag lifecycle gates.
2. In the shape-modeled state, perform the accepted private local
   bind/listen/connect/accept exchange.
3. Send the accepted local identification literal SSH-2.0-Talos_0.1 followed
   by CRLF.
4. Consume exactly one remote identification line using the accepted 255-byte
   owned-banner rules.
5. If the remote identification is valid, model exactly one cleartext SSH
   binary-packet KEXINIT parse and algorithm negotiation.
6. Generate a server KEXINIT cookie from the accepted operator-seeded CSPRNG
   boundary, record only that a cookie was generated and redacted, and retain
   no bytes, digest, fingerprint, or stable identifier derived from it.
7. Close before ECDH, shared-secret computation, host-key signing, NEWKEYS,
   encryption/MAC enablement, authentication, session/channel behavior, or
   shell attachment.

The first modeled packet surface is SSH_MSG_KEXINIT only. It may parse the SSH
binary packet wrapper solely enough to classify one cleartext KEXINIT packet:

- packet_length is big-endian u32, local modeled cap <= 1024 bytes;
- padding_length must leave a non-empty payload and at least four bytes of
  padding;
- payload cap <= 768 bytes;
- message number must be 20;
- cookie length is exactly 16 bytes and redacted;
- each name-list length is capped at 256 bytes;
- each name-list may contain at most 16 names;
- total retained diagnostics may not include any client-provided name text;
- malformed packet, unexpected message number, oversized packet/list, empty
  required list, unsupported algorithm, and prerequisite-disabled paths fail
  closed with fixed labels only.

The server policy for this modeled slice is deliberately narrow and reversible:

- kex_algorithms: curve25519-sha256
- server_host_key_algorithms: ssh-ed25519
- encryption_algorithms_client_to_server: chacha20-poly1305@openssh.com
- encryption_algorithms_server_to_client: chacha20-poly1305@openssh.com
- mac_algorithms_client_to_server: hmac-sha2-256
- mac_algorithms_server_to_client: hmac-sha2-256
- compression_algorithms_client_to_server: none
- compression_algorithms_server_to_client: none
- languages_client_to_server: empty
- languages_server_to_client: empty

This policy is only an algorithm-negotiation model. It does not accept the
runtime crypto backend, X25519 implementation, Ed25519 parsing or signing,
ChaCha20-Poly1305, HMAC, packet encryption, MAC verification, or NEWKEYS.
Even when all names match, diagnostics must retain a fail-closed
crypto-backend-unaccepted label and ssh-ready remains false.

Algorithm selection uses the normal SSH first-mutual rule with the server
policy above as the accepted Talos policy. A positive modeled negotiation
requires a mutual value in every required list and none compression in both
directions. first_kex_packet_follows=true may be classified but must not cause
Talos to retain or process a guessed follow-up packet in this slice.

## Diagnostic Contract

The next implementation may add fixed labels and counters to sshservicediag,
but only redacted facts are allowed:

- kexinit-modeled;
- kexinit-cookie-generated-redacted;
- kexinit-client-packet-valid;
- kexinit-algorithm-negotiated;
- kexinit-algorithm-unsupported;
- kexinit-packet-malformed;
- kexinit-packet-over-limit;
- kexinit-list-over-limit;
- kexinit-first-packet-follows-ignored;
- selected fixed algorithm labels for the accepted server-policy names, if a
  positive negotiation occurs;
- packet/list classification booleans and small counters, without retaining
  client list text or packet bytes.

The existing not-ready labels remain authoritative: dependency-unaccepted,
crypto-backend-unaccepted, authentication-unimplemented,
session-unimplemented, and sshservicediag-not-ready. ssh-ready remains false.

## Findings

- fixed: selected a single KEXINIT-only negotiation surface after the accepted
  local listener/transport exchange.
- fixed: capped packet, payload, name-list, and name-count behavior so the next
  core is a small source/unit-test implementation rather than a broad parser.
- fixed: tied the server KEXINIT cookie to the accepted operator-seeded CSPRNG
  boundary while requiring byte-level redaction.
- fixed: named a narrow reversible algorithm policy for the modeled negotiation
  without accepting runtime crypto or OpenSSH compatibility.
- fixed: required fixed-label diagnostics only; client-provided packet bytes,
  name-list text, peer text, addresses, key bytes, random bytes, digests,
  fingerprints, and stable identifiers remain excluded from evidence.
- deferred: actual X25519/ECDH, Ed25519 host-key parsing/signing, packet
  encryption/MAC, NEWKEYS, authentication, sessions/channels, PTY allocation,
  shell attachment, live transport, hardware reachability, and compatibility
  proof.
- not-an-issue: the server algorithm names are retained because they are fixed
  public Talos policy labels, not peer-provided text or secret material.
- not-an-issue: docs/src/decisions/README.md was not updated; this is a
  reversible local modeled KEXINIT policy, not an accepted runtime crypto or
  dependency decision.

## Validation

- static task/docs/evidence review: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: not run; no SSH
  service source touched.
- cargo -Zjson-target-spec test --quiet: not run; no Rust source or Cargo
  metadata touched.
- jq empty on task-owned JSON evidence: not applicable; no task-owned JSON
  evidence created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- The contract names one exact KEXINIT/algorithm-negotiation slice with
  packet-size/list limits, CSPRNG cookie handling and redaction rules, selected
  algorithm policy, diagnostic labels, and close-before-ECDH boundary.
- ssh-ready remains false and crypto-backend-unaccepted remains authoritative
  until actual key exchange/encryption is separately accepted.
- Findings are recorded with disposition.
- selected_next_task=phase12-ssh-kexinit-negotiation-core-20260622.
- No actual key exchange, encryption/MAC, NEWKEYS, host-key signing,
  authentication/session/shell behavior, hardware reachability,
  OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition is
  accepted.

selected_next_task=phase12-ssh-kexinit-negotiation-core-20260622.
