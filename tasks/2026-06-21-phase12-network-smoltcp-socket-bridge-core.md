# Phase 12.4 smoltcp Socket Bridge Core

Task: phase12-network-smoltcp-socket-bridge-core-20260621

Status: accepted

Classification: phase12-network-smoltcp-socket-bridge-core-accepted

## Scope

Implement the first private descriptor-backed AF_INET/SOCK_STREAM bridge to the
accepted host-only smoltcp TCP frontier. This task owns source/unit behavior
only: socket, bind, listen, connect, accept, send, recv, poll, poll-wait, and
close are exercised through the existing private syscall dispatch and process
descriptor ownership path.

This task does not add a shell-visible /bin/sockdiag TCP diagnostic, retained
smoke transcript, live driver adapter, live packet I/O, Pi 5 hardware run,
hardwareTestLock acquisition, lab mutation, boot publication, hardware
reachability, SSH, public stable socket ABI acceptance, broad socket expansion,
UDP/raw sockets, or a phase transition.

## Findings And Dispositions

- fixed: src/network.rs now stores fixed-capacity SmoltcpSocketBridgeRecord
  entries alongside NetworkSocketDescriptorTable entries. Bridge records are
  keyed by private connection id, preserve client/listener/accepted descriptor
  ownership, and record smoltcp Established observations plus payload-transfer
  observations.
- fixed: connect creates a bridge record only after a bounded host-only
  smoltcp TCP handshake reaches Established over SmoltcpPacketDeviceAdapter,
  fixed packet queues, fixed TCP buffers, explicit MAC/IP endpoints, and
  deterministic poll steps.
- fixed: send through TALOS_SEND_SYSCALL records a bounded smoltcp payload
  transfer before enqueueing bytes for the existing descriptor-backed recv
  path. TALOS_RECV_SYSCALL, TALOS_POLL_SYSCALL, and TALOS_POLL_WAIT_SYSCALL
  continue to observe the existing private readiness vocabulary.
- fixed: close/drop cleanup removes bridge records for client, listener, or
  accepted descriptors while preserving deterministic peer hangup/EPIPE
  behavior through the accepted socket-table state.
- fixed: focused source/unit coverage proves syscall-path success, smoltcp
  Established state, one bounded payload transfer, poll readiness,
  backpressure/no-progress, wrong-owner EBADF, cleanup release, and peer hangup.
- not-an-issue: Accepted local socket rendezvous, pingdiag, sockdiag local
  socket diagnostics, and poll-wait behavior remain regression/control
  surfaces; they still use the private descriptor/socket table path.
- deferred: /bin/sockdiag TCP diagnostic source, retained smoke transcript,
  live driver adapters, live packet I/O, hardware reachability, SSH, public
  stable socket ABI acceptance, broad socket expansion, UDP/raw sockets, and
  phase transition remain later explicit tasks.
- removed: No fake kernel-backed shell command, live packet path, hardware
  action, lab mutation, boot publication, SSH claim, public ABI claim, or
  phase-transition claim was added.

## Implementation

- src/network.rs adds:
  - SmoltcpSocketBridgeObservation
  - SmoltcpSocketBridgeRecord
  - fixed-capacity bridge records in NetworkSocketDescriptorTable
  - deterministic smoltcp handshake/payload helpers over
    SmoltcpPacketDeviceAdapter and fixed packet queues
  - bridge record creation on connect, accepted descriptor attachment on
    accept, payload-transfer recording on send, and cleanup on close/owner
    cleanup
- src/syscall.rs tests add:
  - talos_smoltcp_socket_bridge_transfers_payload_through_private_syscalls
  - test helpers for socket backing descriptors and connection ids

## Evidence

- source/unit plus host/QEMU-substitute:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet talos_smoltcp_socket_bridge_transfers_payload_through_private_syscalls
  - result: passed
  - output summary: quiet focused test passed
- full no_std suite:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: passed
  - output summary: quiet full suite passed; static #[test_case] count is 693
- test-count check:
  - command: rg -n '#\\[test_case\\]' src | wc -l
  - result: 693

## Validation

- cargo fmt --all -- --check: passed after formatting.
- cargo -Zjson-target-spec test --quiet
  talos_smoltcp_socket_bridge_transfers_payload_through_private_syscalls:
  passed.
- cargo -Zjson-target-spec test --quiet with project QEMU path: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No shell-visible /bin/sockdiag TCP diagnostic, retained smoke transcript, live
driver adapter, live packet I/O, Pi 5 hardware run, hardwareTestLock
acquisition, lab mutation, boot publication, hardware reachability, SSH,
public stable socket ABI acceptance, broad socket expansion, UDP/raw sockets,
or phase transition was performed.

## Acceptance

Accepted.

The accepted frontier is private descriptor-backed AF_INET/SOCK_STREAM
host-only smoltcp TCP bridge behavior through the existing Talos private
syscall dispatch and process descriptor ownership path. Evidence is
source/unit plus host/QEMU-substitute only.

Selected next task:
phase12-network-shell-sockdiag-smoltcp-tcp-core-20260621.

Commit: recorded in durable supervisor state after commit creation.
