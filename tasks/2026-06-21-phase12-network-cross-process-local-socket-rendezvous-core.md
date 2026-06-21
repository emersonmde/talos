# Phase 12 Cross-Process Local Socket Rendezvous Core

Task: phase12-network-cross-process-local-socket-rendezvous-core-20260621
Status: accepted
Classification: phase12-network-cross-process-local-socket-rendezvous-core-accepted

## Goal

Implement the accepted private cross-process local socket rendezvous core with
source/unit evidence over distinct process descriptor ownership.

## Scope

- Implement only the accepted private cross-process local socket rendezvous
  core from the prior contract.
- Use bounded kernel-local socket table state to let distinct process
  descriptor stores connect, accept, exchange local payloads, observe
  readiness, block with bounded waits, and clean up close/exit paths.
- Add focused source/unit tests for success, capacity, cleanup, readiness,
  blocking wake/timeout, EOF/error, and descriptor ownership invariants.
- Keep behavior private/internal until a later task explicitly accepts a
  public ABI.

## Findings

- fixed: NetworkSocketDescriptorTable::connect only searched for listeners
  owned by the connecting process. It now resolves active listeners across the
  bounded socket table, records the client owner in the pending peer, and uses
  a private connection id to join cross-process client and accepted sockets.
- fixed: peer lookup previously depended on reversed endpoint tuples within a
  single owner. Connected and accepted socket states now carry a connection id,
  so same-number backing descriptors in different processes do not collapse
  ownership or alias peer discovery.
- fixed: listener readiness could have reported a stale pending connection
  after a client-side close/drop. Readiness now requires a pending peer whose
  client owner/backing descriptor still exists with the expected connection id;
  accept skips stale pending peers and returns EAGAIN when no live peer
  remains.
- fixed: process-exit style cleanup lacked a bounded socket-table owner cleanup
  primitive. close_owner removes all socket backing entries for a
  ProcessOwnerId, causing peers to observe hangup/EPIPE through the same
  readiness and recv boundaries as explicit close.
- fixed: duplicate active listeners for the same endpoint were not rejected
  before global listener discovery. listen now rejects duplicate active
  listener endpoints with EEXIST.
- not-an-issue: process-visible descriptor allocation remains in
  ProcessDescriptorStore and is not shared between owners. Cross-process
  accept creates only a server-owned process descriptor; connect never mutates
  the listener process descriptor table.
- deferred: /bin/sockdiag cross-process diagnostics, retained smoke evidence,
  public socket/libc ABI acceptance, UDP/TCP payload transport, smoltcp
  integration, live packet I/O, hardware reachability, SSH, broad socket
  expansion, and phase transition remain explicit later tasks.
- removed: No runtime shell command expansion, hardware proof path, or packet
  networking path was added.

## Source Anchors

- src/network.rs owns NetworkSocketDescriptorTable,
  NetworkSocketPendingLocalPeer, connection ids, global listener lookup,
  connected peer lookup, readiness, close, and owner cleanup.
- src/syscall.rs owns socket-table-aware private dispatch,
  current_socket_descriptor, process descriptor ownership checks,
  TALOS_POLL_SYSCALL, TALOS_POLL_WAIT_SYSCALL, and SocketPollWaitTable bounded
  wait/resume behavior.
- src/posix.rs owns ProcessDescriptorStore, per-owner DescriptorTables,
  DescriptorObjectKind::Socket, descriptor access, and PosixError.
- src/scheduler.rs owns ProcessOwnerId, TaskId, TaskState::Blocked, and
  runnable transitions.

## Accepted Behavior

The accepted implementation is private source/unit runtime core only:

- A client owned by one ProcessOwnerId can connect to a listener owned by
  another ProcessOwnerId through the shared bounded socket backing table.
- Accept allocates a server-owned accepted socket backing entry and a
  server-owned process descriptor without exposing or mutating the client's
  descriptor table.
- Bidirectional send/recv works across the connection while preserving each
  process's descriptor ownership.
- Nonblocking readiness reports pending accept, payload read, write capacity,
  and peer hangup across the cross-process pair.
- SocketPollWaitTable can block and later wake on cross-process pending
  accept, payload readiness, and peer hangup without waking unrelated tasks.
- Stale pending client entries are skipped after client close/drop; owner-wide
  socket cleanup exposes peer hangup, drains queued bytes, and then returns
  EPIPE.
- Duplicate active listener endpoints fail with EEXIST; exhausted backing,
  descriptor, pending, and payload capacity remains deterministic.

## Evidence

- source/unit tests:
  - cargo -Zjson-target-spec test --quiet cross_process: passed.
  - Added tests:
    - talos_cross_process_local_socket_rendezvous_preserves_descriptor_ownership
    - talos_cross_process_poll_wait_wakes_on_accept_payload_and_peer_close
    - talos_cross_process_close_cleanup_releases_pending_and_connected_capacity
- full no_std suite:
  - cargo -Zjson-target-spec test --quiet: passed.
- fmt/lint:
  - cargo fmt --all -- --check: passed.
- docs:
  - /home/node/.cargo/bin/mdbook build: passed.
- diff hygiene:
  - git diff --check: passed.
  - git diff --cached --check: passed before commit.

## Rejected Claims

This task does not accept shell-visible /bin/sockdiag cross-process output,
retained smoke evidence, UDP/TCP payload transport, smoltcp integration, live
driver adapters, live packet I/O, Pi 5 hardware runs, lab mutation, boot
publication, hardware reachability, SSH, public stable socket ABI acceptance,
broad socket expansion, or phase transition.

## Next Action

The only mechanically unblocked follow-up if this task is accepted is
phase12-network-shell-sockdiag-cross-process-local-socket-core-20260621.
