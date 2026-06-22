# Phase 12.6 SSH NEWKEYS packet crypto core

Task id: phase12-ssh-newkeys-packet-crypto-core-20260622

Status: accepted.

Classification: phase12-ssh-newkeys-packet-crypto-core-accepted.

## Goal

Implement the private NEWKEYS activation and fixed-fixture encrypted-packet
diagnostic selected by the accepted NEWKEYS packet crypto contract, without
accepting authentication, session, shell, live reachability, public
compatibility, hardware, broad expansion, or a phase transition.

## Findings and Disposition

- fixed: added independent private send and receive NEWKEYS activation state
  to the accepted runtime KEX packet-state handles. encrypted-packet-state-active
  is true only after both directions are active.
- fixed: added fixed-label NEWKEYS and encrypted-packet diagnostic reports for
  sshservicediag-newkeys-not-ready, sshservicediag-newkeys-send-active,
  sshservicediag-newkeys-receive-active,
  sshservicediag-encrypted-packet-state-active,
  sshservicediag-encrypted-packet-sequence-advanced,
  sshservicediag-encrypted-packet-sequence-overflow,
  sshservicediag-encrypted-packet-crypto-failed, and
  sshservicediag-encrypted-packet-diagnostic-ready.
- fixed: successful diagnostics use only caller-owned fixed public fixture
  packet buffers, zeroize packet and tag material before returning durable
  labels, and advance exactly one private direction sequence number.
- fixed: missing NEWKEYS direction, post-NEWKEYS plaintext I/O attempt,
  malformed packet shape, crypto failure, and sequence overflow fail closed
  without sequence advancement.
- not-an-issue: ssh-ready remains false because authentication/session/shell,
  live reachability, and public compatibility remain outside this task.
- deferred: retained smoke evidence for these labels is intentionally left to
  phase12-shell-ssh-newkeys-packet-crypto-smoke-20260622.

## Source Evidence

- src/ssh_runtime_crypto.rs adds private NEWKEYS activation, encrypted packet
  diagnostic labels, per-direction sequence advancement, zeroization of packet
  and tag material, and overflow/malformed fail-closed handling.
- src/ssh_service_readiness.rs maps the new fixed runtime labels into the
  sshservicediag label vocabulary without changing default ssh-ready behavior.
- Unit evidence:
  - ssh_runtime_crypto::tests::newkeys_activation_is_independent_and_diagnostic_advances_one_sequence
  - ssh_runtime_crypto::tests::encrypted_packet_diagnostic_fails_closed_for_overflow_and_malformed_packet

## Validation

- cargo fmt --all: pass.
- cargo -Zjson-target-spec test --quiet: pass with QEMU path configured; 758
  no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

An initial cargo test attempt without the configured QEMU path failed with
qemu-system-aarch64 not found; rerunning with
/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin on PATH
passed.

## Redaction Review

Durable evidence retains only task ids, file paths, fixed labels, public
algorithm names, key/IV lengths, public test names, validation command names,
sequence transition labels, booleans, counters, and classifications. It does
not retain private keys, IV bytes, tags, plaintext, ciphertext, exchange
hashes, shared secrets, signatures, peer raw input, operator identity,
key-derived identifiers, stable session identifiers, live peer addresses, or
hardware data.

## Non-Goals Preserved

No user authentication, authorized-key parsing, session/channel success, shell
attachment, TCP/hardware reachability, OpenSSH/POSIX/Linux compatibility
claim, broad expansion, phase transition, live socket connection, hardware/lab
action, boot publication, rekey support, standalone HMAC emission for the AEAD
packet path, broad SSH packet parser, transport loop, or ssh-ready=true claim
is accepted.

## Result

Accepted. selected_next_task=phase12-shell-ssh-newkeys-packet-crypto-smoke-20260622.
