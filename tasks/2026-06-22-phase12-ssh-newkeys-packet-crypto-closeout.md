# Phase 12.6 SSH NEWKEYS packet crypto closeout

Task id: phase12-ssh-newkeys-packet-crypto-closeout-20260622

Status: accepted.

Classification: phase12-ssh-newkeys-packet-crypto-closeout-accepted.

## Goal

Close out the local NEWKEYS/encrypted-packet frontier and reconcile the
accepted contract, core implementation, retained smoke evidence, validation,
and redaction policy before any next SSH layer.

## Findings and Disposition

- fixed: reconciled the accepted contract, core, and smoke evidence into one
  explicit frontier: private local NEWKEYS activation plus a fixed-fixture
  encrypted-packet diagnostic only.
- fixed: confirmed the accepted implementation keeps independent send and
  receive NEWKEYS activation, requires both directions for
  encrypted-packet-state-active, and advances exactly one private direction
  sequence number on the successful diagnostic path.
- fixed: confirmed retained evidence is fixed-label/counter/length/command
  evidence only and excludes keys, IV bytes, tags, plaintext, ciphertext,
  exchange hashes, shared secrets, signatures, peer raw input, operator
  identity, key-derived identifiers, stable session identifiers, live peer
  addresses, and hardware data.
- not-an-issue: malformed packet shape is the retained crypto-failed
  discriminator because the accepted private core intentionally exposes no key
  or cipher mutation hook for manufacturing secret-dependent failures.
- not-an-issue: ssh-ready remains false because authentication/session/shell,
  live reachability, and public compatibility remain outside this frontier.
- deferred: encrypted transport dispatch over post-NEWKEYS packets,
  unsupported-message dispatch policy, pre-authentication state handling,
  authentication/session/channel behavior, shell attachment, live reachability,
  OpenSSH/POSIX/Linux compatibility, hardware proof, broad expansion, and phase
  transition.

## Reconciled Evidence

- phase12-ssh-newkeys-packet-crypto-contract-20260622: accepted NEWKEYS as a
  private bidirectional transport-state transition and limited the first packet
  diagnostic to fixed public fixture payloads and retained fixed labels.
- phase12-ssh-newkeys-packet-crypto-core-20260622: implemented private send and
  receive NEWKEYS activation, encrypted-packet-state-active only after both
  directions, one-direction sequence advancement, zeroization of temporary
  packet/tag material, and fail-closed malformed/overflow/not-ready labels.
- phase12-shell-ssh-newkeys-packet-crypto-smoke-20260622: retained local unit
  smoke evidence for missing KEX readiness, missing NEWKEYS directions,
  independent activation labels, active-state reporting, one successful
  diagnostic, sequence advancement, malformed packet shape/crypto-failed, and
  sequence overflow.
- src/ssh_runtime_crypto.rs static review: the task-owned unit coverage names
  the accepted fixed labels and does not retain packet, key, IV, tag,
  transcript, peer, operator, session, live reachability, or hardware material.
- src/ssh_service_readiness.rs static review: the fixed runtime labels map into
  sshservicediag without making ssh-ready true.

## Accepted Frontier

The accepted frontier is private local NEWKEYS and encrypted-packet diagnostic
readiness only. Talos can model local outbound NEWKEYS, inbound peer NEWKEYS,
both-directions-active encrypted packet state, one fixed-fixture packet
diagnostic, and private sequence advancement. It does not accept user
authentication, authorized-key parsing, session/channel success, shell
attachment, TCP/hardware reachability, OpenSSH/POSIX/Linux compatibility,
hardware/lab action, boot publication, broad expansion, or phase transition.

The next objective dependency is a bounded encrypted transport dispatch
contract that defines how post-NEWKEYS encrypted packets are dispatched or
failed closed before authentication/session/shell behavior. The selected next
task is phase12-ssh-encrypted-transport-dispatch-contract-20260622.

## Validation

- static task/source/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

No Rust source or Cargo metadata was touched, so cargo fmt and cargo test were
not required by this task's conditional gates.

## Redaction Review

Pass. Durable closeout evidence retains only task ids, file paths, fixed
labels, public algorithm names, public lengths, public test names, validation
commands, and classifications. It retains no private keys, IV bytes, tags,
plaintext, ciphertext, exchange hashes, shared secrets, signatures, peer raw
input, operator identity, key-derived identifiers, stable session identifiers,
live peer addresses, or hardware data.

## Non-Goals Preserved

No authentication/session/shell behavior, authorized-key parsing, live socket
connection, hardware/lab action, boot publication, live reachability claim,
OpenSSH/POSIX/Linux compatibility claim, ssh-ready=true claim, broad
expansion, or phase transition is accepted.

## Result

Accepted. selected_next_task=phase12-ssh-encrypted-transport-dispatch-contract-20260622.
