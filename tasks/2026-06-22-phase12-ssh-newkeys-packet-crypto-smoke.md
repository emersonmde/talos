# Phase 12.6 SSH NEWKEYS packet crypto smoke

Task id: phase12-shell-ssh-newkeys-packet-crypto-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-newkeys-packet-crypto-smoke-accepted.

## Goal

Retain focused local smoke evidence for the accepted NEWKEYS activation and
encrypted-packet diagnostic core without exposing packet or key material, and
without accepting authentication, session, shell, live reachability, public
compatibility, hardware, broad expansion, or a phase transition.

## Findings and Disposition

- fixed: added a task-owned smoke unit test that records fixed-label evidence
  for missing KEX readiness, missing both NEWKEYS directions, missing receive
  NEWKEYS after send activation, both-directions-active reporting, one
  successful packet diagnostic, and sequence advancement.
- fixed: retained deterministic fail-closed evidence for malformed packet
  shape/crypto-failed and u32 sequence overflow without sequence advancement.
- not-an-issue: crypto-failed is represented by the accepted malformed packet
  shape path because the core API intentionally avoids exposing key or cipher
  mutation hooks that would manufacture secret-dependent failures.
- not-an-issue: ssh-ready remains false because authentication/session/shell,
  live reachability, and public compatibility remain outside this task.
- deferred: closeout reconciliation is intentionally left to
  phase12-ssh-newkeys-packet-crypto-closeout-20260622.

## Smoke Evidence

- src/ssh_runtime_crypto.rs:
  - ssh_runtime_crypto::tests::newkeys_packet_crypto_smoke_retains_fixed_label_evidence
- Fixed retained labels:
  - sshservicediag-kex-csprng-not-ready
  - sshservicediag-newkeys-not-ready
  - sshservicediag-newkeys-send-active
  - sshservicediag-newkeys-receive-active
  - sshservicediag-encrypted-packet-state-active
  - sshservicediag-encrypted-packet-sequence-advanced
  - sshservicediag-encrypted-packet-crypto-failed
  - sshservicediag-encrypted-packet-sequence-overflow
  - sshservicediag-encrypted-packet-diagnostic-ready

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass with configured QEMU path; 759
  no_std tests passed, including
  ssh_runtime_crypto::tests::newkeys_packet_crypto_smoke_retains_fixed_label_evidence.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

## Redaction Review

Durable evidence retains only task ids, file paths, public test names, fixed
labels, public algorithm names, public key/IV lengths, sequence transition
labels, booleans, counters, validation command names, and classifications. It
does not retain private keys, IV bytes, tags, plaintext, ciphertext, exchange
hashes, shared secrets, signatures, peer raw input, operator identity,
key-derived identifiers, stable session identifiers, live peer addresses, or
hardware data.

## Non-Goals Preserved

No user authentication, authorized-key parsing, session/channel success, shell
attachment, TCP/hardware reachability, OpenSSH/POSIX/Linux compatibility
claim, broad expansion, phase transition, live socket connection, hardware/lab
action, boot publication, new feature behavior beyond smoke evidence, live
peer, socket, network driver, OpenSSH client, authentication, session/channel,
or shell behavior is accepted.

## Result

Accepted. selected_next_task=phase12-ssh-newkeys-packet-crypto-closeout-20260622.
