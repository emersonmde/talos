# Phase 12.4 Socket Readiness/Poll Core

Task: phase12-network-socket-readiness-poll-core-20260621

Status: accepted

Classification: phase12-network-socket-readiness-poll-core-accepted

## Scope

Implement only the accepted private, nonblocking readiness/poll core selected
by phase12-network-socket-readiness-poll-abi-contract-20260621:

- TALOS_POLL_SYSCALL = 13
- socket-table-aware process descriptor dispatch only
- fixed 16-byte user poll entries with fd, events, and overwritten revents
- bounded entry count from 1 through 8
- private READ, WRITE, HANGUP, and ERROR readiness bits
- process-local readiness over accepted local socket table state

This task does not add shell /bin/sockdiag readiness output, retained smoke
evidence, blocking sleep, scheduler wait queues, timeout handling, UDP/TCP
payload transport, live driver adapters, live packet I/O, hardware
reachability, hardwareTestLock acquisition, Pi 5 hardware work, lab mutation,
boot publication, SSH, smoltcp adoption, public stable socket ABI acceptance,
broad socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: src/syscall.rs now defines private TALOS_POLL_SYSCALL = 13 and
  SyscallNumber::TalosPoll. Scalar/default dispatch remains ENOTSUP outside
  the socket-table-aware process descriptor dispatch path.
- fixed: src/syscall.rs implements the accepted user-memory entry shape:
  x0 points to a readable/writable entry array, x1 is 1 through 8 entries, x2
  is flags=0, and x3=x4=x5 are reserved zero registers. Unsupported event
  bits, unsupported flags, nonzero reserved registers, zero entries, and more
  than eight entries fail the whole call with EINVAL; caller-buffer copy
  failures return EFAULT.
- fixed: src/network.rs now owns NetworkSocketReadiness and computes readiness
  from existing descriptor-backed socket state without mutating accept queues
  or payload queues.
- fixed: Listener sockets report READ when the pending accept queue is
  nonempty and READ was requested.
- fixed: Connected and Accepted sockets report READ when inbound bytes are
  queued, WRITE when the unique local peer exists and has at least one byte of
  inbound FIFO capacity, and HANGUP when the accepted reverse-endpoint peer is
  absent after close/drop.
- fixed: Peer close/drop preserves terminal nonblocking behavior: queued bytes
  after close report READ | HANGUP when READ was requested, and an empty queue
  after close still reports READ | HANGUP so the accepted recv path can expose
  EPIPE.
- fixed: Per-entry bad descriptors, non-socket descriptors, wrong-owner or
  missing-backing relationships, and oversized fd values report
  TALOS_POLL_ERROR in revents without failing the whole mixed poll call.
- fixed: Unit tests cover listener accept-readiness, connected/accepted
  read/write readiness, peer FIFO backpressure, peer close/hangup readiness,
  invalid descriptors, non-socket descriptors, no-current-owner behavior,
  malformed user buffers, unsupported event bits, unsupported flags, count
  bounds, and scalar dispatch fail-closed behavior.
- not-an-issue: Existing NetworkSocketPendingQueue::len, socket recv_queue
  state, reverse-endpoint peer lookup, and close/drop cleanup are enough for
  this private nonblocking readiness slice; no scheduler wait queue or wakeup
  registration is required.
- deferred: Shell-visible /bin/sockdiag readiness output, retained smoke
  evidence, blocking waits, scheduler wait queues, timeout handling, UDP/TCP
  payload transport, cross-process/global poll sets, live packet I/O, SSH,
  public socket ABI acceptance, broad socket expansion, and phase transition
  remain deferred.
- removed: No dead code or broad refactor outside the accepted readiness/poll
  core was justified.

## Evidence

- source anchors:
  - src/syscall.rs: private selector constants, SyscallNumber::TalosPoll,
    scalar-dispatch ENOTSUP, socket-table-aware poll dispatch, entry copy-in/
    copy-out ordering, event-mask validation, per-entry ERROR, and unit tests.
  - src/network.rs: NetworkSocketReadiness and
    NetworkSocketDescriptorTable::readiness over Listening, Connected, and
    Accepted states.
- source/unit tests:
  - talos_poll_reports_listener_local_payload_and_peer_hangup_readiness
  - talos_poll_reports_write_backpressure_and_deterministic_entry_errors
  - talos_poll_rejects_malformed_calls_and_scalar_dispatch_fails_closed
- documentation:
  - docs/src/project/phase12-networking-ssh.md
  - docs/src/roadmap.md

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- focused source/unit tests: cargo -Zjson-target-spec test talos_poll --quiet
  passed with QEMU 9.2.0 on PATH.
- full source/unit tests: cargo -Zjson-target-spec test --quiet passed.
- diff validation: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff validation: git diff --cached --check passed before commit.

No shell /bin/sockdiag readiness output, retained smoke transcript, Pi 5
hardware run, hardwareTestLock acquisition, boot archive publication, lab
mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, UDP/TCP payload transport, SSH, smoltcp, broad socket expansion,
public stable socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is source/unit host/QEMU-substitute evidence for private
descriptor-backed nonblocking readiness over local AF_INET stream sockets only.

Selected next task:
phase12-network-shell-sockdiag-readiness-poll-core-20260621.

Commit: recorded in durable supervisor state after commit creation.
