# Task: phase12-network-shell-sockdiag-connect-accept-core-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-connect-accept-core-accepted

## Scope

Extend only the existing shell-visible `/bin/sockdiag` diagnostic for the
accepted private local socket connect/accept core. The implementation stays on
the VFS/userspace execution path and socket-table-aware descriptor dispatch:
`TALOS_SOCKET_SYSCALL = 6`, `TALOS_BIND_SYSCALL = 7`,
`TALOS_LISTEN_SYSCALL = 8`, `TALOS_CONNECT_SYSCALL = 9`,
`TALOS_ACCEPT_SYSCALL = 10`, and `TALOS_CLOSE_SYSCALL = 2`.

This task does not add retained smoke evidence, send/recv, poll/blocking
network I/O, UDP/TCP payload transport, live packet I/O, hardware reachability,
SSH, public stable socket ABI acceptance, broad socket expansion, or a phase
transition.

## Findings

- fixed: `src/local_command_loop.rs` now reports shell-visible `/bin/sockdiag`
  local connect/accept output with listener fd, client fd, accepted fd,
  connect/accept return values, listener/client/accepted socket states,
  descriptor kind/access, close/drop cleanup, waitpid, and laststatus.
- fixed: The diagnostic has enough bounded local descriptor/socket capacity for
  one listener, one connected client, one queue-backpressure client, and one
  accepted server-side descriptor. All backings are closed before the diagnostic
  returns.
- fixed: Deterministic controls now include accept-before-connect `EAGAIN`,
  no matching listener `EINVAL`, full pending queue `ENOSPC`, non-socket
  descriptor `EBADF`, unsupported domain/type/protocol, invalid endpoint,
  invalid backlog, repeated bind/listen behavior, invalid/closed descriptor,
  malformed arguments, missing executable identity, and bounded syscall
  vocabulary.
- not-an-issue: Retained smoke artifacts and scripts are intentionally deferred
  to the queued smoke task; this core accepts source/unit host/QEMU-substitute
  evidence only.
- deferred: send, recv, poll/blocking network I/O, UDP/TCP payload transport,
  live driver adapters, live packet I/O, hardware reachability, SSH, public
  socket ABI acceptance, and broad socket expansion.

## Evidence

- source/unit host/QEMU-substitute:
  `local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls`
  covers VFS executable lookup/open/read for `/bin/sockdiag`, startup ABI,
  descriptor-backed socket open/bind/listen/connect/accept/close, accepted
  client/server socket state, deterministic controls, waitpid, and laststatus.
- source/unit host/QEMU-substitute: `cargo -Zjson-target-spec test --quiet
  local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls`
  completed with the full 673-test no_std suite passing.

## Validation

- `cargo fmt --all -- --check`: passed after formatting.
- `cargo -Zjson-target-spec test --quiet`: passed.
- `git diff --check`: passed.
- `/home/node/.cargo/bin/mdbook build`: passed.
- `git diff --cached --check`: passed before commit.

## Closeout

Accepted as source/unit host/QEMU-substitute evidence for shell-visible
`/bin/sockdiag` local connect/accept diagnostics only. The next bounded queued
task is `phase12-network-shell-sockdiag-connect-accept-smoke-20260620`.

Commit: recorded in durable supervisor state after acceptance.
