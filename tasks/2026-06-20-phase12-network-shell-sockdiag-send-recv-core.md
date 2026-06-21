# Phase 12.4 Shell Sockdiag Send/Recv Core

Task: phase12-network-shell-sockdiag-send-recv-core-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-send-recv-core-accepted

## Scope

Extend only the existing VFS/userspace-backed `/bin/sockdiag` diagnostic
surface needed to exercise the accepted private local send/recv payload
transfer after socket open, bind, listen, connect, and accept.

The diagnostic remains shell-visible through initramfs/VFS executable lookup,
open/read, startup ABI construction, and socket-table-aware descriptor
dispatch. It uses the accepted private socket selectors only:

- `TALOS_SOCKET_SYSCALL = 6`
- `TALOS_BIND_SYSCALL = 7`
- `TALOS_LISTEN_SYSCALL = 8`
- `TALOS_CONNECT_SYSCALL = 9`
- `TALOS_ACCEPT_SYSCALL = 10`
- `TALOS_SEND_SYSCALL = 11`
- `TALOS_RECV_SYSCALL = 12`
- `TALOS_CLOSE_SYSCALL = 2`

This task does not add retained smoke evidence, poll/blocking network I/O,
readiness, wait queues, UDP/TCP payload transport, live driver adapters, live
packet I/O, hardware reachability, Pi 5 hardware work, hardwareTestLock
acquisition, lab mutation, boot publication, SSH, cross-process/global port
semantics, broad socket expansion, public stable socket ABI acceptance, or a
phase transition.

## Findings And Dispositions

- fixed: `src/local_command_loop.rs` now extends the existing shell-visible
  `/bin/sockdiag` execution path with descriptor-backed local send/recv
  payload transfer. The output records client-to-server and server-to-client
  byte counts plus literal diagnostic payloads after accepted
  open/bind/listen/connect/accept setup.
- fixed: The sockdiag diagnostic now uses an explicit user-data mapping for
  its local caller buffers so `TALOS_SEND` and `TALOS_RECV` exercise the
  same copy-in/copy-out path as the accepted syscall core.
- fixed: Deterministic shell controls now cover empty recv `EAGAIN`,
  invalid send/recv flags `EINVAL`, payload queue backpressure `ENOSPC`,
  send after peer close `EPIPE`, malformed arguments, missing executable
  identity, unsupported socket parameters, connect/accept controls,
  non-socket descriptors, invalid/closed descriptors, and the bounded syscall
  vocabulary including `TALOS_SEND`/`TALOS_RECV`.
- fixed: `src/network.rs` now snapshots the payload queue length before
  `copy_within` in `NetworkSocketPayloadQueue::consume`; this removes a
  borrow-checker failure on the accepted send/recv receive path.
- fixed: `src/syscall.rs` now corrects the stale
  `talos_send_recv_moves_local_payload_bytes_bidirectionally` fixture length
  for the 14-byte `client->server` payload, keeping the accepted source/unit
  regression runnable.
- not-an-issue: The existing single shell-visible sockdiag regression remains
  the right focused source/unit surface for this core because it already
  proves VFS executable lookup, startup ABI, waitpid, laststatus, and accepted
  socket diagnostic controls.
- deferred: Retained smoke transcript/scripts, poll/blocking behavior,
  readiness/wait queues, UDP/TCP payload transport, live driver adapters,
  live packet I/O, hardware reachability, SSH, public socket ABI acceptance,
  cross-process/global port semantics, broad socket expansion, and phase
  transition remain deferred.

## Evidence

- source anchors:
  - `src/local_command_loop.rs`: `LOCAL_COMMAND_LOOP_BOUNDARY`,
    `LocalCommandSockdiagRecord`, `LocalCommandSockdiagControlRecord`,
    `DescriptorBackedLocalCommandIo::exec_shell_sockdiag_diagnostic`,
    `write_exec_sockdiag_line`, `write_exec_sockdiag_controls_line`, and
    `local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls`.
  - `src/network.rs`: `NetworkSocketPayloadQueue::consume` borrow-checker
    fix on the receive commit path.
  - `src/syscall.rs`: corrected source/unit send/recv fixture payload length.
- source/unit host/QEMU-substitute:
  `cargo -Zjson-target-spec test local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls --quiet`
  passed with the QEMU runner executing the no_std test suite and the focused
  sockdiag regression.
- source/unit evidence from the focused sockdiag regression covers:
  VFS `/bin/sockdiag` lookup/open/read, startup ABI, descriptor-backed socket
  open/bind/listen/connect/accept/send/recv/close, bidirectional local payload
  bytes `client->server` and `server->client`, empty recv, invalid
  send/recv flags, payload queue backpressure, peer close behavior, waitpid,
  laststatus, malformed argv, and missing executable identity.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- focused source/unit host/QEMU-substitute:
  `cargo -Zjson-target-spec test local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls --quiet`
  passed.
- full source/unit host/QEMU-substitute:
  `cargo -Zjson-target-spec test --quiet` passed.
- diff validation: `git diff --check` passed.
- docs build: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff validation: `git diff --cached --check` passed before commit.

No retained smoke transcript, Pi 5 hardware run, hardwareTestLock acquisition,
boot archive publication, generated-root publication, lab mutation, power
cycle, live driver adapter, live packet I/O, hardware reachability, UDP/TCP
payload transport, SSH, smoltcp, broad socket expansion, public stable socket
ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is source/unit host/QEMU-substitute evidence for
shell-visible VFS/userspace `/bin/sockdiag` execution exercising the accepted
private descriptor-backed local send/recv payload path only.

Selected next task:
phase12-network-shell-sockdiag-send-recv-smoke-20260620.

Commit: recorded in durable supervisor state after commit creation.
