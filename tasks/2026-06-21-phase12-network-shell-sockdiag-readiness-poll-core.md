# Phase 12.4 Shell Sockdiag Readiness/Poll Core

Task: phase12-network-shell-sockdiag-readiness-poll-core-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-readiness-poll-core-accepted

## Scope

Expose the accepted private, nonblocking socket readiness/poll core through
the existing VFS/userspace /bin/sockdiag diagnostic only.

This task extends the existing shell-visible sockdiag source/unit path to
exercise listener accept-readiness, connected/accepted read-readiness,
write-readiness, FIFO backpressure, peer hangup, invalid descriptors,
non-socket descriptors, unsupported poll events, and unchanged accepted socket
diagnostics.

This task does not add retained smoke evidence, a new shell command,
fake/kernel-only command paths, blocking sleep, scheduler wait queues, timeout
handling, UDP/TCP payload transport, live driver adapters, live packet I/O,
hardware reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, or a phase transition.

## Findings And Dispositions

- fixed: src/local_command_loop.rs now includes the accepted readiness/poll
  boundary in LOCAL_COMMAND_BUILTIN_BOUNDARY.
- fixed: /bin/sockdiag records poll revents for empty listener, pending
  listener, empty recv queue, queued payload, writable peer FIFO, full peer
  FIFO backpressure, peer close/hangup, invalid descriptors, and non-socket
  descriptors.
- fixed: /bin/sockdiag exercises TALOS_POLL_SYSCALL = 13 through the same VFS
  executable lookup/open/read, startup ABI, descriptor store, socket table,
  UserMapping copy path, and accepted socket syscalls used by prior sockdiag
  diagnostics.
- fixed: the sockdiag controls now include poll-specific unsupported-event,
  invalid-descriptor, and non-socket-descriptor vocabulary, while preserving
  existing open/close, bind/listen, connect/accept, send/recv, waitpid, and
  laststatus regressions.
- not-an-issue: The diagnostic remains source/unit host/QEMU-substitute only;
  retained smoke transcript evidence is explicitly deferred to the selected
  smoke task.
- deferred: retained smoke evidence, blocking waits, scheduler wait queues,
  timeout handling, UDP/TCP payload transport, live packet I/O, hardware
  reachability, SSH, public socket ABI acceptance, broad socket expansion, and
  phase transition remain deferred.
- removed: No dead code or broad refactor outside the accepted shell
  readiness/poll diagnostic path was justified.

## Evidence

- source anchors:
  - src/local_command_loop.rs: /bin/sockdiag readiness/poll transcript fields,
    poll entry helpers, control vocabulary, and source/unit test.
  - src/syscall.rs: accepted TALOS_POLL_SYSCALL socket-table-aware dispatch.
  - src/network.rs: accepted NetworkSocketReadiness source of readiness bits.
- focused source/unit test:
  - local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls
- documentation:
  - docs/src/project/phase12-networking-ssh.md
  - docs/src/roadmap.md

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- focused /bin/sockdiag readiness/poll source/unit tests: cargo
  -Zjson-target-spec test
  local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls
  --quiet passed with QEMU 9.2.0 on PATH; the no_std runner executed the full
  679-test suite.
- full source/unit tests: cargo -Zjson-target-spec test --quiet passed.
- diff validation: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff validation: git diff --cached --check passed before commit.

No retained smoke transcript, Pi 5 hardware run, hardwareTestLock
acquisition, boot archive publication, lab mutation, power cycle, live driver
adapter, live packet I/O, hardware reachability, UDP/TCP payload transport,
SSH, smoltcp, broad socket expansion, public stable socket ABI acceptance, or
phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is source/unit host/QEMU-substitute evidence for
shell-visible /bin/sockdiag readiness/poll output over private
descriptor-backed nonblocking local AF_INET stream socket readiness only.

Selected next task:
phase12-network-shell-sockdiag-readiness-poll-smoke-20260621.

Commit: recorded in durable supervisor state after commit creation.
