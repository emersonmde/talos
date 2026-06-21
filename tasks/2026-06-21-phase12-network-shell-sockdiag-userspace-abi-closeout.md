# Phase 12.4 Shell Sockdiag Userspace ABI Closeout

Task: phase12-network-shell-sockdiag-userspace-abi-closeout-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-userspace-abi-closeout-accepted

## Scope

Close out the documented userspace socket ABI frontier after the accepted
private ABI contract, no_std helper core, shell-visible /bin/sockdiag core,
and retained host/QEMU-substitute smoke evidence. This reconciliation records
the accepted evidence level, accepted claims, rejected claims, remaining gaps,
and planning status before any live packet I/O, hardware reachability, SSH,
UDP/raw sockets, public POSIX compatibility claim, broader socket expansion,
or phase transition.

This task does not implement runtime behavior beyond closeout consistency
documentation. It does not acquire hardwareTestLock, mutate the lab, publish a
boot image, retain generated-root evidence, claim live packet I/O, claim Pi 5
hardware behavior, claim hardware reachability, accept SSH, add UDP/raw
sockets, accept POSIX/Linux compatibility, promote a public stable socket ABI,
broaden sockets, or transition phase.

## Findings And Dispositions

- fixed: Reconciled the accepted sequence from private userspace socket ABI
  contract, no_std helper core, shell-visible /bin/sockdiag source/unit core,
  and retained host/QEMU-substitute smoke evidence.
- fixed: The accepted frontier is shell-visible VFS/userspace /bin/sockdiag
  execution through the documented private userspace_socket_abi helper
  constructors and existing descriptor-backed host-only smoltcp TCP bridge.
- fixed: The accepted evidence covers svc #0/x8 selector shape, negative errno
  return vocabulary, AF_INET/SOCK_STREAM subset, socket/bind/listen/connect/
  accept/send/recv/poll/poll-wait/close wrappers, 16-byte ABI PollEntry
  fd/events/revents layout, process descriptor ownership, Established smoltcp
  handshake states, deterministic frame/step counters, accepted descriptor
  attachment, one bounded payload-transfer observation, waitpid, and
  laststatus.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, unchanged local socket diagnostics, unchanged
  /bin/pingdiag behavior, ABI constant/wrapper coverage, and bounded syscall
  vocabulary.
- fixed: Roadmap and Phase 12 networking docs now state that this closeout
  freezes the userspace socket ABI frontier at source/unit plus
  host/QEMU-substitute evidence and requires supervisor planning before any
  next bounded socket/network task.
- not-an-issue: The retained smoke evidence is host/QEMU-substitute, not Pi 5
  hardware evidence. That is sufficient because this closeout accepts no live
  driver adapter, live packet I/O, hardware reachability, boot publication,
  lab mutation, or SSH claim.
- not-an-issue: The ABI remains a private Talos no_std/userspace helper surface
  over the accepted private socket syscall selectors. This closeout does not
  promote it to a public stable ABI, libc/std socket layer, or POSIX/Linux
  compatibility claim.
- removed: No source behavior change, hardware artifact, generated-root
  artifact, live packet I/O claim, hardware reachability claim, public socket
  ABI claim, POSIX/Linux compatibility claim, SSH claim, broad expansion,
  UDP/raw socket claim, or phase-transition claim was introduced.
- deferred: Live driver adapters, live packet I/O, Pi 5 hardware reachability,
  SSH strategy and entropy, public ABI stabilization, libc/std socket wrappers,
  UDP/raw sockets, broader socket expansion, and any phase transition remain
  future supervisor-planned work.

## Evidence

- Contract:
  tasks/2026-06-21-phase12-network-socket-userspace-abi-contract.md.
- ABI helper core:
  tasks/2026-06-21-phase12-network-socket-userspace-abi-core.md.
- Shell-visible diagnostic core:
  tasks/2026-06-21-phase12-network-shell-sockdiag-userspace-abi-core.md.
- Retained smoke task:
  tasks/2026-06-21-phase12-network-shell-sockdiag-userspace-abi-smoke.md.
- Smoke transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/smoke-transcript.md.
- Smoke classification:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/classification.json.
- Smoke evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/evidence-map.json.

## Accepted Boundary

The accepted evidence level is source/unit plus retained host/QEMU-substitute
smoke evidence only. The accepted boundary is shell-visible VFS/userspace
/bin/sockdiag observation of the documented private Talos userspace socket ABI
over the existing descriptor-backed host-only smoltcp TCP bridge:

- /bin/sockdiag resolves through read-only VFS executable lookup, open, and
  read.
- The diagnostic records userspace-socket-abi-v1 and builds documented private
  SocketAbiCall values for socket, bind, listen, connect, accept, send, recv,
  poll, poll_wait, and close.
- The ABI helper surface preserves the private svc #0/x8 selector shape,
  x0..x5 scalar arguments, negative x0 errno returns, AF_INET/SOCK_STREAM
  subset, bounded payload queues, and 16-byte PollEntry user-memory layout.
- Process-visible descriptors stay owned by their ProcessOwnerId descriptor
  tables while the existing descriptor-backed socket dispatch reports the
  host-only SmoltcpSocketBridgeRecord.
- Shell observation includes Established smoltcp client/server states,
  deterministic frame/step counters, accepted-descriptor attachment, one
  bounded payload-transfer observation, waitpid, and laststatus.
- Malformed arguments, missing executable identity, accepted local socket
  diagnostics, accepted /bin/pingdiag behavior, ABI constant/wrapper coverage,
  and bounded syscall vocabulary remain deterministic controls.

This boundary is not live driver integration, not hardware-backed networking,
not SSH readiness, not UDP/raw socket behavior, not libc/std socket support,
not public stable ABI acceptance, and not POSIX/Linux compatibility.

## Remaining Gaps

The next socket/network work still requires explicit supervisor planning. Later
tasks must separately define and validate:

- live driver adapters and coupling between hardware receive/transmit and the
  accepted Talos packet queue/smoltcp adapter layers.
- live packet I/O completion, scheduling, backpressure, error mapping, and
  packet ownership rules.
- Pi 5 hardware reachability, lab evidence, boot publication, and restore rules
  if hardware is selected.
- SSH prerequisites including TCP stability on live hardware, entropy, crypto,
  host keys, userspace service shape, authentication policy, and exposure
  controls.
- public ABI stabilization, libc/std socket wrappers, or POSIX/Linux
  compatibility claims for the socket surface.
- UDP/raw socket behavior, broader socket option/error semantics, and any
  phase transition.

## Validation

- static source/task/evidence review: passed. Reviewed userspace_socket_abi
  helper source anchors, local_command_loop /bin/sockdiag ABI anchors, private
  socket dispatch anchors, the ABI contract task, ABI helper core task,
  shell-visible diagnostic task, retained smoke task, smoke transcript,
  classification, evidence map, roadmap frontier, and Phase 12 networking doc.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, SSH, UDP/raw socket work, broad socket
expansion, public stable socket ABI acceptance, POSIX/Linux compatibility
claim, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=null.

planningNeeded=true.

The documented private userspace socket ABI frontier is closed at source/unit
plus retained host/QEMU-substitute evidence over shell-visible VFS/userspace
/bin/sockdiag execution through userspace_socket_abi helper constructors and
the existing descriptor-backed host-only smoltcp TCP bridge. Live driver
adapters, live packet I/O, Pi 5 hardware behavior, lab mutation, boot
publication, generated-root publication, hardware reachability, SSH, UDP/raw
sockets, libc/std socket wrappers, POSIX/Linux compatibility, public stable
socket ABI acceptance, broad socket expansion, and phase transition remain
rejected. Supervisor planning is required before any next bounded
socket/network task.

Commit: recorded in durable supervisor state after commit creation.
