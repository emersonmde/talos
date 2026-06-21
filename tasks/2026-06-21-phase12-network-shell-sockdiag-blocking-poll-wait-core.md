# Phase 12.4 Shell Sockdiag Blocking Poll Wait Core

Task: phase12-network-shell-sockdiag-blocking-poll-wait-core-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-blocking-poll-wait-core-accepted

## Scope

Expose the accepted private process-local bounded blocking socket poll wait
through the existing VFS/userspace `/bin/sockdiag` diagnostic only.

The diagnostic remains shell-visible through initramfs/VFS executable
lookup/open/read, startup ABI construction, socket-table-aware descriptor
dispatch, process lifecycle, `waitpid`, and `laststatus`. It exercises
`TALOS_POLL_WAIT_SYSCALL = 14` over the accepted local AF_INET stream socket
readiness core for immediate-ready, wait-then-connect listener wake,
wait-then-send payload wake, finite timeout, peer close/hangup wake, and
deterministic malformed/error controls.

This task does not add retained smoke evidence, fake kernel command output,
UDP/TCP payload transport, smoltcp integration, cross-process/global poll
sets, live driver adapters, live packet I/O, Pi 5 hardware work, SSH, public
stable socket ABI acceptance, broad socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: `src/local_command_loop.rs` now includes the blocking poll-wait
  boundary in `LOCAL_COMMAND_BUILTIN_BOUNDARY`.
- fixed: `/bin/sockdiag` records shell-visible `TALOS_POLL_WAIT` output for
  immediate listener readiness, pending-listener wake after local connect,
  payload-read wake after local send, bounded timeout with zero revents, and
  peer close/hangup wake.
- fixed: The diagnostic calls the accepted wait-aware socket dispatch with a
  bounded `SocketPollWaitTable`, synthetic local task records, scheduler
  `TaskState::Blocked` transitions, and `SingleCoreScheduler::make_runnable`
  resume checks. This is source/unit evidence over the real wait core, not a
  transcript-only retry loop.
- fixed: Deterministic controls preserve scalar fail-closed `ENOTSUP`,
  invalid timeout `EINVAL`, unsupported events `EINVAL`, invalid/non-socket
  ERROR readiness, and existing open/close, bind/listen, connect/accept,
  send/recv, nonblocking poll, `waitpid`, and `laststatus` regressions.
- fixed: The initramfs descriptor dispatch match now includes
  `SyscallNumber::TalosPollWait` in its fail-closed scalar/default path,
  preserving `ENOTSUP` outside the explicit socket-table wait-aware context.
- not-an-issue: The single focused sockdiag regression remains the right
  source/unit surface because it already proves VFS executable lookup,
  startup ABI, descriptor-backed socket syscalls, process lifecycle, waitpid,
  laststatus, and unchanged `/bin/pingdiag` controls.
- deferred: Retained smoke evidence, cross-process/global poll sets, UDP/TCP
  payload transport, smoltcp integration, live packet I/O, hardware
  reachability, SSH, public socket ABI acceptance, broad socket expansion,
  and phase transition remain deferred.
- removed: No dead code or broad refactor outside the accepted shell
  diagnostic path was justified.

## Evidence

- source anchors:
  - `src/local_command_loop.rs`: `LocalCommandSockdiagRecord`,
    `exec_shell_sockdiag_diagnostic`, `local_sockdiag_poll_wait_task`,
    `write_exec_sockdiag_line`, `write_exec_sockdiag_controls_line`, and
    `local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls`.
  - `src/syscall.rs`: accepted `TALOS_POLL_WAIT_SYSCALL`,
    `SocketPollWaitTable`, wait-aware socket dispatch, and
    `resume_ready_or_expired` core, plus scalar/default fail-closed
    descriptor dispatch.
  - `src/scheduler.rs`: `TaskState::Blocked` and
    `SingleCoreScheduler::make_runnable` resume surface.
- focused source/unit host/QEMU-substitute:
  `cargo -Zjson-target-spec test local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls --quiet`
  passed.
- full source/unit host/QEMU-substitute:
  `cargo -Zjson-target-spec test --quiet` passed.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- focused `/bin/sockdiag` blocking poll wait source/unit test:
  `cargo -Zjson-target-spec test local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls --quiet` passed.
- full source/unit test: `cargo -Zjson-target-spec test --quiet` passed.
- diff validation: `git diff --check` passed.
- docs build: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff validation: `git diff --cached --check` passed before commit.

No retained smoke transcript, Pi 5 hardware run, hardwareTestLock acquisition,
boot archive publication, generated-root publication, lab mutation, power
cycle, live driver adapter, live packet I/O, hardware reachability, UDP/TCP
payload transport, SSH, smoltcp, public stable socket ABI acceptance, broad
socket expansion, or phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is source/unit host/QEMU-substitute evidence for
shell-visible VFS/userspace `/bin/sockdiag` execution exercising the accepted
private process-local bounded blocking poll wait over local sockets only.

Selected next task:
phase12-network-shell-sockdiag-blocking-poll-wait-smoke-20260621.

Commit: recorded in durable supervisor state after commit creation.
