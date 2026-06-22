# Phase 12.6 SSH NEWKEYS packet crypto contract

Task id: phase12-ssh-newkeys-packet-crypto-contract-20260622

Status: accepted.

Classification: phase12-ssh-newkeys-packet-crypto-contract-accepted.

## Goal

Define the first bounded NEWKEYS activation and encrypted-packet diagnostic
boundary after the accepted local runtime KEX closeout.

## Scope

- Reviewed the accepted runtime KEX closeout, runtime KEX core, retained smoke
  evidence, and current private packet-state shape.
- Defined the first NEWKEYS activation boundary as a private transport-state
  transition that consumes the accepted runtime KEX result only after local
  send and receive NEWKEYS conditions are both explicitly satisfied.
- Defined the first encrypted-packet diagnostic as a task-owned local packet
  crypto check over fixed public fixture payloads, sequence numbers, and fixed
  labels without retaining keys, IVs, tags, plaintext, ciphertext, exchange
  hashes, shared secrets, signatures, peer raw input, or stable session ids.
- Selected exactly one bounded implementation follow-up:
  phase12-ssh-newkeys-packet-crypto-core-20260622.

## Non-goals

No source behavior change, Cargo dependency adoption beyond already accepted
runtime crypto crates, live transport I/O, hardware/lab action, boot
publication, authentication/session success, authorized-key parsing, shell
attachment, public syscall/API surface, live reachability claim,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, or phase transition
is accepted.

This contract does not retain private keys, random bytes, shared secrets,
exchange hashes, derived keys, IVs, tags, packet plaintext, packet ciphertext,
signatures, public-key blobs, peer raw input, operator identity, key-derived
identifiers, or stable transport/session identifiers.

## Accepted Contract

The first NEWKEYS and encrypted-packet slice remains a private runtime SSH
transport boundary inside the existing accepted local model:

- Prerequisites: the implementation may consume only an accepted
  SshRuntimeKexReady value from the real runtime KEX core, the accepted
  KEXINIT negotiation result for curve25519-sha256/ssh-ed25519/
  chacha20-poly1305@openssh.com/hmac-sha2-256/none, and task-owned public
  fixture packet payloads for retained tests. Missing KEX readiness,
  unsupported algorithms, missing outbound NEWKEYS, missing inbound NEWKEYS,
  packet crypto failure, sequence overflow, malformed packet shape, and
  post-NEWKEYS plaintext I/O attempts must fail closed with fixed labels.
- NEWKEYS activation: Talos must model SSH_MSG_NEWKEYS as a private transport
  state transition. Send encryption may become active only after the local
  outbound NEWKEYS condition is satisfied. Receive decryption may become active
  only after the peer inbound NEWKEYS condition is satisfied. The combined
  diagnostic may report encrypted-packet-state-active only when both
  directions are active; this still does not make ssh-ready true.
- Packet crypto: the first implementation may use the existing private
  chacha20-poly1305@openssh.com packet-state handles created by runtime KEX
  and must keep sequence numbers private u32 state. Initial sequence numbers
  are zero in both directions; each successful encrypted packet operation
  increments exactly one direction. Sequence overflow must fail closed before
  encryption/decryption.
- Diagnostic surface: retained diagnostics may expose only fixed labels,
  booleans, counters, algorithm names, key/IV lengths, and sequence-number
  transition labels. They must not retain packet bytes, tags, key material,
  IV material, exchange hashes, shared secrets, signatures, or peer raw input.
- HMAC policy: hmac-sha2-256 remains the negotiated MAC name for algorithm
  policy and future non-AEAD packet work, but the accepted
  chacha20-poly1305@openssh.com encrypted packet path must not emit or retain
  a standalone HMAC.

The first implementation boundary should stay private, tentatively extending
ssh_runtime_crypto with an owned post-KEX packet-crypto state and exposing only
fixed-label status to sshservicediag. It may add tests or a task-owned smoke
script, but it must not connect to live sockets or authenticate users.

Required fixed labels for the first implementation are:

- sshservicediag-newkeys-not-ready
- sshservicediag-newkeys-send-active
- sshservicediag-newkeys-receive-active
- sshservicediag-encrypted-packet-state-active
- sshservicediag-encrypted-packet-sequence-advanced
- sshservicediag-encrypted-packet-sequence-overflow
- sshservicediag-encrypted-packet-crypto-failed
- sshservicediag-encrypted-packet-diagnostic-ready

The implementation must keep existing runtime KEX labels intact and keep
ssh-ready false until later authentication/session/shell and reachability tasks
explicitly accept readiness.

## Findings

- fixed: defined NEWKEYS as a private bidirectional activation state rather
  than implicit readiness from the accepted runtime KEX packet-state handles.
- fixed: separated send-side and receive-side activation so a future task can
  fail closed on asymmetric NEWKEYS state before encrypted packet I/O.
- fixed: defined sequence-number ownership, zero initial state, one-direction
  advancement per successful packet operation, and fail-closed overflow.
- fixed: limited retained evidence to fixed labels, booleans, counters,
  public algorithm names, lengths, validation command names, and
  classifications.
- not-an-issue: hmac-sha2-256 remains a negotiated policy name, but the first
  chacha20-poly1305@openssh.com packet path must not emit a standalone HMAC.
- deferred: encrypted packet implementation, retained packet smoke,
  authentication, authorized-key parsing, sessions/channels, PTY/shell
  attachment, live reachability, OpenSSH/POSIX/Linux compatibility, hardware
  proof, broad expansion, and phase transition.

## Evidence

- phase12-ssh-runtime-kex-closeout-20260622: accepted local modeled runtime
  KEX frontier with private encrypted-packet state readiness and
  ssh-ready=false.
- src/ssh_runtime_crypto.rs static review: runtime KEX currently constructs
  private chacha20-poly1305@openssh.com client-to-server and server-to-client
  packet states with key length 32, IV length 8, and sequence number zero, but
  does not activate NEWKEYS or perform encrypted packet I/O.
- src/ssh_service_readiness.rs static review: sshservicediag currently reports
  fixed runtime KEX labels and keeps authentication/session/shell and
  ssh-ready false.

## Redaction Review

Pass. This task retained only task ids, file paths, public crate and algorithm
names, fixed diagnostic labels, public lengths, sequence-number policy,
validation command names, and classifications. It retained no private keys,
random bytes, shared secrets, exchange hashes, derived keys, signatures,
public-key blobs, IV material, packet plaintext/ciphertext, tags, peer raw
input, operator identity, key-derived identifiers, stable session identifiers,
live peer addresses, or hardware data.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Validation levels: static task/source/docs review, docs build, and diff
checks. No Rust source was touched, so cargo fmt and cargo test were not
required by this task's gates. No Pi 5 hardware run, lab-controller API
action, hardwareTestLock acquisition, boot publication, NEWKEYS activation,
encrypted packet I/O, authentication/session work, shell attachment, live
reachability claim, compatibility claim, broad expansion, or phase transition
was performed.

## Acceptance

Accepted. Talos now has a bounded contract for the next private local SSH
runtime slice: bidirectional NEWKEYS activation plus one task-owned encrypted
packet diagnostic over the accepted chacha20-poly1305@openssh.com packet-state
handles. ssh-ready remains false.

selected_next_task=phase12-ssh-newkeys-packet-crypto-core-20260622.
