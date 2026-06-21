# Phase 12.4 Shell Sockdiag Userspace ABI Core

Task: phase12-network-shell-sockdiag-userspace-abi-core-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-userspace-abi-core-accepted

## Scope

Route the shell-visible /bin/sockdiag TCP diagnostic through the documented
private Talos userspace socket ABI helper surface from
src/userspace_socket_abi.rs, while preserving the accepted host-only smoltcp
TCP diagnostic observations and existing local socket controls.

This task does not retain a smoke transcript, publish a boot image, acquire
hardwareTestLock, mutate the lab, perform live packet I/O, add SSH, add
UDP/raw sockets, accept libc/std or POSIX/Linux compatibility, broaden sockets,
or transition phase.

## Findings And Dispositions

- fixed: Updated src/local_command_loop.rs so the shell-visible /bin/sockdiag
  main socket path dispatches socket, bind, listen, connect, accept, send,
  recv, poll, poll_wait, and close through userspace_socket_abi::SocketAbiCall
  wrapper constructors before reaching the existing descriptor/socket-table
  dispatch.
- fixed: Reused userspace_socket_abi::PollEntry encode/decode for the
  diagnostic poll-entry user-memory layout instead of local hand-encoding the
  16-byte fd/events/revents record.
- fixed: Renamed the focused source/unit test to
  local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi
  and asserted the shell output records userspace-socket-abi-v1 in the
  diagnostic source string.
- fixed: Corrected the accepted ABI helper test's UserMapping length type so
  the full no_std test gate compiles cleanly, and exercised the helper accessors
  that became active in test builds.
- not-an-issue: Deterministic negative controls still use explicit malformed
  calls where needed to prove invalid domain/type/protocol, invalid flags, and
  unsupported event behavior. They remain controls, not the accepted diagnostic
  success path.
- removed: No retained smoke evidence, live driver adapter, live packet I/O,
  Pi 5 hardware behavior, generated-root publication, public ABI claim, SSH,
  UDP/raw socket work, broad socket expansion, or phase-transition claim was
  introduced.
- deferred: Retained host/QEMU-substitute smoke evidence for this ABI-routed
  shell diagnostic remains the selected follow-up task. Live driver adapters,
  live packet I/O, hardware reachability, SSH strategy, UDP/raw sockets, and
  libc/std socket wrappers remain future explicit tasks.

## Implementation

- src/local_command_loop.rs:
  - imports userspace_socket_abi.
  - adds a local socket_abi_dispatch! bridge that converts SocketAbiCall into
    the accepted selector and SyscallArguments before invoking the existing
    socket-table-aware dispatch.
  - routes the shell /bin/sockdiag success path through the ABI helper
    constructors for AF_INET stream socket open, bind, listen, connect, accept,
    send, recv, poll, poll_wait, and close.
  - records source=vfs-userspace-sockdiag+userspace-socket-abi-v1+... in the
    shell-visible diagnostic output.
- src/userspace_socket_abi.rs:
  - fixes the focused helper test mapping length type and covers SocketAbiCall
    and PollEntry accessors used by the shell diagnostic path.

## Evidence

- Focused source/unit test
  local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi:
  passed under cargo -Zjson-target-spec test --quiet. Covers /bin/sockdiag
  VFS/userspace execution, ABI-routed socket diagnostics, Established smoltcp
  handshake state, deterministic frame/step counters, descriptor attachment,
  payload-transfer observation, waitpid, laststatus, malformed/missing
  executable controls, unchanged local socket diagnostics, unchanged
  /bin/pingdiag controls, and bounded syscall vocabulary output.
- Focused source/unit tests
  userspace_socket_abi_constants_match_private_kernel_contract and
  userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge: passed under
  cargo -Zjson-target-spec test --quiet.
- Full gate cargo -Zjson-target-spec test --quiet: passed with 695 no_std
  tests.
- Formatting gate cargo fmt --all -- --check: passed.
- Diff hygiene git diff --check: passed.
- Docs validation /home/node/.cargo/bin/mdbook build: passed.
- Cached diff hygiene git diff --cached --check: passed before commit.

## Accepted Boundary

The accepted evidence level is source/unit plus host/QEMU-substitute cargo
test. The accepted boundary is shell-visible /bin/sockdiag reaching the
accepted private host-only smoltcp TCP diagnostic through the documented
private userspace_socket_abi helper surface and existing descriptor-backed
socket dispatch.

This task does not accept retained smoke evidence, live driver adapters, live
packet I/O, Pi 5 hardware reachability, SSH, UDP/raw sockets, libc/std socket
wrappers, POSIX/Linux compatibility, public stable socket ABI acceptance, broad
socket expansion, or phase transition.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-userspace-abi-smoke-20260621.

Commit: recorded in durable supervisor state after commit creation.
