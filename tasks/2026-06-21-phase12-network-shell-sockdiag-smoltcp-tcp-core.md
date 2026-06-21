# Phase 12.4 Shell Sockdiag smoltcp TCP Core

Task: phase12-network-shell-sockdiag-smoltcp-tcp-core-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-smoltcp-tcp-core-accepted

## Scope

Expose the accepted private host-only smoltcp TCP bridge through the existing
shell-visible /bin/sockdiag VFS/userspace diagnostic. This task owns only
source/unit behavior over the existing private socket syscall path and the
accepted descriptor-backed bridge records.

This task does not add retained smoke evidence, live packet I/O, hardware/lab
action, boot publication, SSH, public stable socket ABI acceptance, broad
socket expansion, UDP/raw sockets, or a phase transition.

## Findings And Dispositions

- fixed: src/local_command_loop.rs now tags the local command boundary with
  +shell-sockdiag-vfs-userspace-smoltcp-tcp and reports private smoltcp TCP
  bridge fields from /bin/sockdiag.
- fixed: /bin/sockdiag verifies that connect creates a
  SmoltcpSocketBridgeRecord whose handshake reaches Established on both
  client and server sides before the diagnostic continues.
- fixed: /bin/sockdiag verifies that accept attaches the accepted socket
  descriptor to the bridge record and that one bounded client send records a
  smoltcp payload-transfer observation before the existing descriptor-backed
  recv path reads the payload.
- fixed: shell-visible output now includes bounded success fields:
  connection id, Established handshake states, handshake step/frame counters,
  accepted-descriptor attachment, payload transfer count/length, and
  Established payload states.
- fixed: src/network.rs now avoids the stale smoltcp RecvError::Exhausted
  variant by checking can_recv before recv_slice in the accepted private
  bridge drive loop. This preserves the same bounded no-data behavior while
  restoring compilation against the pinned smoltcp 0.13.1 dependency.
- not-an-issue: Existing local socket, cross-process local socket, poll,
  poll-wait, pingdiag, malformed argument, and missing executable diagnostics
  remain regression/control surfaces.
- deferred: Retained smoke transcript, live driver adapters, live packet I/O,
  hardware reachability, SSH, public stable socket ABI acceptance, broad
  socket expansion, UDP/raw sockets, and phase transition remain later
  explicit tasks.
- removed: No fake kernel-backed TCP command path, lab mutation, hardware
  action, boot publication, public ABI claim, or phase-transition claim was
  added.

## Implementation

- src/local_command_loop.rs extends LocalCommandSockdiagRecord and
  write_exec_sockdiag_line with private smoltcp TCP bridge evidence fields.
- /bin/sockdiag continues to execute through VFS/userspace executable lookup
  and the accepted private socket syscalls.
- The diagnostic reads SmoltcpSocketBridgeRecord from the same
  descriptor-backed connection used for the existing local send/recv path.

## Evidence

- focused source/unit plus host/QEMU-substitute:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls
  - result: passed
  - output summary: test result ok; 693 passed
- focused bridge regression plus host/QEMU-substitute:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet talos_smoltcp_socket_bridge_transfers_payload_through_private_syscalls
  - result: passed
  - output summary: test result ok; 693 passed
- full no_std suite:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: passed
  - output summary: test result ok; 693 passed
- test-count check:
  - command: rg -n '#\[test_case\]' src | wc -l
  - result: 693

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet
  local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls:
  passed.
- cargo -Zjson-target-spec test --quiet
  talos_smoltcp_socket_bridge_transfers_payload_through_private_syscalls:
  passed.
- cargo -Zjson-target-spec test --quiet with project QEMU path: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No retained smoke transcript, live packet I/O, Pi 5 hardware run,
hardwareTestLock acquisition, lab mutation, boot publication, hardware
reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, UDP/raw sockets, or phase transition was performed.

## Acceptance

Accepted.

The accepted frontier is shell-visible source/unit coverage for the private
host-only smoltcp TCP bridge through VFS/userspace /bin/sockdiag and the
existing private syscall/descriptor path. Evidence is source/unit plus
host/QEMU-substitute only.

Selected next task:
phase12-network-shell-sockdiag-smoltcp-tcp-smoke-20260621.

Commit: recorded in durable supervisor state after commit creation.
