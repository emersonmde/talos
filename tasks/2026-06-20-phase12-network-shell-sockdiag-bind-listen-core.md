# Phase 12.4 Shell Sockdiag Bind/Listen Core

Task: phase12-network-shell-sockdiag-bind-listen-core-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-bind-listen-core-accepted

## Scope

Expose only the accepted descriptor-backed socket bind/listen state through the
existing shell-visible /bin/sockdiag VFS/userspace diagnostic path. The
diagnostic still resolves /bin/sockdiag through initramfs/VFS executable
lookup/open/read, preserves the existing startup ABI, lifecycle, waitpid, and
laststatus surfaces, and now exercises socket open -> bind -> listen -> close
through TALOS_SOCKET_SYSCALL = 6, TALOS_BIND_SYSCALL = 7,
TALOS_LISTEN_SYSCALL = 8, and TALOS_CLOSE_SYSCALL = 2.

This task does not add send, recv, connect, accept, poll/blocking network I/O,
UDP/TCP payload transport, live packet I/O, live driver adapters, hardware
reachability, Pi 5 hardware work, hardwareTestLock acquisition, lab mutation,
boot publication, generated-root publication, SSH, smoltcp adoption, broad
socket expansion, public stable socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: src/local_command_loop.rs now extends the existing /bin/sockdiag
  diagnostic to call the socket-table-aware bind/listen syscall dispatch after
  opening the accepted AF_INET stream socket.
- fixed: The shell transcript records bind endpoint, bind return, listening
  backlog, listen return, listening state, descriptor kind/access, close return,
  and backing close/drop state in one VFS/userspace diagnostic line.
- fixed: The diagnostic verifies deterministic negative controls for unsupported
  socket domain/type/protocol, bind on a closed descriptor, listen-before-bind,
  invalid bind endpoint, invalid backlog, repeated bind, repeated listen, and
  double-close EBADF.
- fixed: The existing shell-visible sockdiag regression test now asserts the
  bind/listen transcript while retaining VFS executable lookup, malformed
  argument, missing executable, waitpid, laststatus, and open/close descriptor
  behavior.
- not-an-issue: The accepted socket core already owns endpoint/backlog
  validation and close/drop cleanup; the shell diagnostic only observes and
  reports that accepted state.
- deferred: Retained smoke evidence, send/recv, connect/accept, poll/blocking
  behavior, UDP/TCP payload transport, accept queues, global port registry,
  address-conflict policy, smoltcp, live packet I/O, SSH, hardware work,
  generated-root publication, public stable socket ABI acceptance, broad socket
  expansion, and phase transition remain deferred.
- removed: No dead-code removal was justified inside this bounded shell
  diagnostic slice.

## Evidence

- Source/unit host/QEMU-substitute evidence:
  - src/local_command_loop.rs updates
    DescriptorBackedLocalCommandIo::exec_shell_sockdiag_diagnostic to use the
    accepted socket-table-aware process descriptor dispatch for open, bind,
    listen, and close.
  - src/local_command_loop.rs updates LocalCommandSockdiagRecord,
    LocalCommandSockdiagControlRecord, and transcript rendering to retain the
    bind/listen state.
  - src/local_command_loop.rs test
    local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls
    proves shell-visible VFS/userspace execution, socket open/bind/listen/close,
    deterministic controls, waitpid, and laststatus.
- Accepted predecessor:
  - phase12-network-socket-bind-listen-core-20260620 accepted and committed at
    857a779d451d42674a4e941e55e69837120807af.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, generated-root publication, send, recv, connect, accept, UDP/TCP
payload transport, SSH, smoltcp, broad socket expansion, public stable socket
ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted evidence level is source/unit host/QEMU-substitute evidence over
shell-visible VFS/userspace /bin/sockdiag execution, the accepted private socket
open/bind/listen/close syscall path, descriptor-backed listening socket state,
deterministic controls, close/drop cleanup, waitpid/laststatus observation, and
unchanged accepted open/close diagnostics.

Selected next task:
phase12-network-shell-sockdiag-bind-listen-smoke-20260620.

Commit: recorded in durable supervisor state after commit creation.
