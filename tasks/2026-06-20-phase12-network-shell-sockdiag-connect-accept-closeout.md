# Phase 12.4 Shell Sockdiag Connect/Accept Closeout

Task: phase12-network-shell-sockdiag-connect-accept-closeout-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-connect-accept-closeout-accepted

## Scope

Close out the shell-visible local socket connect/accept frontier after the
accepted socket connect/accept ABI contract, descriptor-backed core,
shell-visible /bin/sockdiag core, and retained smoke evidence. This closeout
reconciles accepted claims, rejected claims, and remaining gaps before any
send/recv, payload transport, live packet I/O, public socket ABI acceptance,
SSH, or phase-transition work.

This task does not add runtime source implementation. It does not accept Pi 5
hardware behavior, hardwareTestLock acquisition, lab mutation, boot
publication, generated-root publication, live driver adapters, live packet
I/O, hardware reachability, send, recv, payload bytes, poll/blocking network
I/O, UDP/TCP payload transport, SSH, smoltcp, broad socket expansion, public
stable socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: Reconciled the connect/accept chain as accepted only at source/unit
  plus host/QEMU-substitute smoke level over the VFS/userspace /bin/sockdiag
  path.
- fixed: Confirmed the accepted shell evidence resolves /bin/sockdiag through
  VFS executable lookup/open/read before exercising TALOS_SOCKET_SYSCALL = 6,
  TALOS_BIND_SYSCALL = 7, TALOS_LISTEN_SYSCALL = 8,
  TALOS_CONNECT_SYSCALL = 9, TALOS_ACCEPT_SYSCALL = 10, process descriptor
  socket ownership, descriptor-backed local listener/client/accepted socket
  state, TALOS_CLOSE_SYSCALL = 2, waitpid, and laststatus.
- fixed: Confirmed deterministic controls remain retained for malformed
  arguments, missing executable identity, unsupported socket domain/type/
  protocol, listen-before-bind, invalid bind endpoint, invalid backlog,
  repeated bind, repeated listen backlog update, accept-before-connect, missing
  listener, full pending queue, non-socket descriptors, invalid and closed
  descriptors, wrong-owner backing, descriptor and socket backing capacity,
  scalar dispatch ENOTSUP outside the socket-table-aware path, bounded syscall
  vocabulary, unchanged socket open/close behavior, unchanged bind/listen
  behavior, and unchanged /bin/pingdiag behavior.
- fixed: Recorded the remaining gaps explicitly: send/recv, payload bytes,
  poll/blocking network I/O, UDP/TCP payload transport, cross-process socket
  semantics, global port registry/address-conflict policy, live driver
  adapters, live packet I/O, hardware reachability, SSH, public stable socket
  ABI acceptance, broad socket expansion, and phase transition are not
  accepted by this chain.
- not-an-issue: The retained smoke transcript labels its evidence as
  host/QEMU-substitute only. The no_std QEMU runner's full-suite execution for
  each filtered invocation does not broaden the accepted claim.
- removed: No stale runtime source, generated-root artifact, hardware
  artifact, live packet I/O claim, public socket ABI claim, or phase-transition
  claim was introduced by the closeout.
- deferred: Supervisor planning is required for any next bounded Phase 12.4
  socket or network task.

## Evidence

- Socket connect/accept ABI contract:
  tasks/2026-06-20-phase12-network-socket-connect-accept-abi-contract.md.
- Socket connect/accept core:
  tasks/2026-06-20-phase12-network-socket-connect-accept-core.md.
- Shell sockdiag connect/accept core:
  tasks/2026-06-20-phase12-network-shell-sockdiag-connect-accept-core.md.
- Retained smoke task:
  tasks/2026-06-20-phase12-network-shell-sockdiag-connect-accept-smoke.md.
- Retained smoke transcript:
  tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/smoke-transcript.md.
- Smoke classification:
  tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/classification.json.
- Smoke evidence map:
  tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/evidence-map.json.

## Accepted Boundary

The accepted boundary is shell-visible local socket connect/accept over the
VFS/userspace syscall path:

- /bin/sockdiag exists as a read-only initramfs executable identity.
- Shell exec /bin/sockdiag uses the existing VFS open/read execution path.
- Startup ABI and process lifecycle accounting are preserved.
- AF_INET=2, SOCK_STREAM=1, protocol=0 opens through the private experimental
  TALOS_SOCKET_SYSCALL = 6.
- Bind records the accepted local endpoint through private
  TALOS_BIND_SYSCALL = 7.
- Listen records bounded backlog and listening state through private
  TALOS_LISTEN_SYSCALL = 8.
- Connect uses private TALOS_CONNECT_SYSCALL = 9 to target exactly one
  current-process listener by local IPv4/port and records Connected client
  state with synthetic local endpoint ownership.
- Accept uses private TALOS_ACCEPT_SYSCALL = 10 to dequeue one pending local
  peer and create a new current-process descriptor backed by Accepted
  server-side socket state.
- The process descriptors are DescriptorObjectKind::Socket values owned by the
  current process with bounded backing state.
- Close uses TALOS_CLOSE_SYSCALL = 2 and drops the relevant process descriptor
  and socket backing state.
- Shell-visible observation goes through waitpid and laststatus.
- /bin/pingdiag and the accepted descriptor/syscall regression surfaces remain
  unchanged.

This boundary is not a TCP/IP stack, not live driver integration, not packet
I/O, not public libc/POSIX socket compatibility, and not SSH readiness.

## Remaining Gaps

The next socket/network work still requires explicit supervisor planning. At
minimum, later tasks must separately define and validate:

- send and recv semantics, caller buffers, partial I/O, payload bytes, and
  error mapping.
- poll/blocking behavior, wait queues, readiness, and cancellation/error
  semantics.
- UDP/TCP payload transport and any smoltcp or equivalent integration.
- cross-process sockets, global port registry/address-conflict policy, and
  listener lookup beyond the accepted current-process diagnostic model.
- live driver adapters, packet queue coupling, transmit/receive completion,
  and hardware-backed packet I/O.
- Pi 5 hardware reachability, lab evidence, boot publication, and restore
  rules if hardware is selected.
- SSH prerequisites including TCP stability, entropy, crypto, keys, userspace
  service shape, authentication policy, and operational exposure controls.
- any public stable socket ABI or phase transition claim.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, send/recv, payload bytes,
poll/blocking network I/O, UDP/TCP payload transport, SSH, smoltcp, broad
socket expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=null.

planningNeeded=true.

The accepted evidence level remains source/unit plus host/QEMU-substitute
smoke evidence over shell-visible VFS/userspace /bin/sockdiag execution, the
selected socket open/bind/listen/connect/accept/close syscall path, process
descriptor ownership, descriptor-backed local listener/client/accepted socket
state, close/drop behavior, waitpid/laststatus observation, deterministic
controls, unchanged accepted diagnostics, and unchanged bounded syscall
vocabulary. Supervisor planning is required before any broader socket,
send/recv, UDP/TCP payload transport, live packet I/O, SSH, hardware work, or
phase transition.

Commit: recorded in durable supervisor state after commit creation.
