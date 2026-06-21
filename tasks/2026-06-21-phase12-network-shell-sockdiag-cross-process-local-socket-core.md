# Task: phase12-network-shell-sockdiag-cross-process-local-socket-core-20260621

Status: accepted
Classification: phase12-network-shell-sockdiag-cross-process-local-socket-core-accepted

## Goal

Expose the accepted private cross-process local socket rendezvous core through
the existing VFS/userspace `/bin/sockdiag` diagnostic without accepting UDP/TCP,
live packet I/O, SSH, public socket ABI stability, broad socket expansion, or a
phase transition.

## Scope

- Extend only the existing `/bin/sockdiag` diagnostic path.
- Exercise two distinct `ProcessOwnerId` descriptor tables over the accepted
  private cross-process local socket core.
- Report deterministic shell-visible markers for descriptor ownership,
  connect/accept, payload transfer, readiness/wait, cleanup/EOF, and rejected
  claims.

## Findings

- fixed: `src/local_command_loop.rs` widens the local command descriptor-store
  fixture to two owners so `/bin/sockdiag` can model a shell/server process and
  a distinct client process without collapsing same-number descriptors.
- fixed: `/bin/sockdiag` now opens a server-owned listener and a client-owned
  socket through the socket-table-aware syscall dispatch, connects across owner
  boundaries, accepts into the server descriptor table, and verifies the
  accepted fd is not installed in the client descriptor table.
- fixed: The diagnostic proves cross-process bidirectional payload transfer,
  listener and payload `TALOS_POLL_WAIT` wakeups, peer-close hangup readiness,
  deterministic cleanup close, and backing socket release.
- fixed: The rendered sockdiag transcript now includes deterministic
  `cross-process-*` markers for owners, fds, readiness bits, wait revents,
  payload text, ownership boundary, cleanup close, and backing release.
- not-an-issue: Existing process-local sockdiag open/close, bind/listen,
  connect/accept, send/recv, readiness/poll, blocking poll-wait, malformed
  argument, missing executable, and syscall vocabulary controls remain in the
  same focused regression.
- deferred: Retained host/QEMU-substitute smoke transcript is intentionally left
  to `phase12-network-shell-sockdiag-cross-process-local-socket-smoke-20260621`.
- deferred: UDP/TCP payload transport, smoltcp integration, live driver
  adapters, live packet I/O, Pi 5 hardware runs, lab mutation, boot
  publication, hardware reachability, SSH, public stable socket ABI acceptance,
  broad socket expansion, and phase transition remain out of scope.

## Source Anchors

- `src/local_command_loop.rs:36` records the shell sockdiag capability boundary.
- `src/local_command_loop.rs:756` records the new cross-process transcript
  fields.
- `src/local_command_loop.rs:1262` creates the local command harness with two
  process descriptor owners.
- `src/local_command_loop.rs:2989` starts the cross-process sockdiag diagnostic
  over distinct server/client owners.
- `src/local_command_loop.rs:3078` records listener poll-wait blocking and wake
  after the client connect.
- `src/local_command_loop.rs:3190` accepts into the server descriptor table and
  checks the accepted descriptor is absent from the client table.
- `src/local_command_loop.rs:3217` records payload poll-wait blocking and wake
  after cross-process send.
- `src/local_command_loop.rs:3361` closes the client side and records hangup
  readiness plus cleanup release.
- `src/local_command_loop.rs:7190` renders deterministic `cross-process-*`
  shell-visible markers.
- `src/local_command_loop.rs:8191` keeps the focused shell-visible sockdiag
  regression over `exec /bin/sockdiag`, waitpid, laststatus, malformed args, and
  missing executable controls.

## Validation

- source/unit tests: `cargo -Zjson-target-spec test local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls --quiet` passed.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- diff validation: `git diff --check` passed.
- docs build: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff validation: `git diff --cached --check` passed before commit.

## Acceptance

Accepted evidence is source/unit plus docs only. `/bin/sockdiag` now exposes the
private cross-process local socket rendezvous through VFS/userspace execution
and deterministic transcript markers. Retained smoke, UDP/TCP, live packet I/O,
hardware reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, and phase transition remain rejected.

selected_next_task=phase12-network-shell-sockdiag-cross-process-local-socket-smoke-20260621
