# Phase 12.4 Shell Sockdiag Open/Close Closeout

Task: phase12-network-shell-sockdiag-open-close-closeout-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-open-close-closeout-accepted

## Scope

Close out the shell-visible socket open/close frontier after the accepted
ABI contract, socket open/close core, shell /bin/sockdiag core, and retained
smoke evidence. This closeout reconciles accepted claims, rejected claims, and
the remaining gaps before any broader socket or networking work.

This task does not add runtime source implementation. It does not accept Pi 5
hardware behavior, hardwareTestLock acquisition, lab mutation, boot
publication, live driver adapters, live packet I/O, network reachability,
send, recv, bind, connect, listen, accept, poll/blocking network I/O, UDP/TCP
payload transport, SSH, smoltcp, broad socket expansion, public stable socket
ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: Reconciled the socket open/close chain as accepted only at
  source/unit plus host/QEMU-substitute smoke level over the VFS/userspace
  /bin/sockdiag path.
- fixed: Confirmed the accepted shell evidence resolves /bin/sockdiag
  through VFS executable lookup/open/read before exercising
  TALOS_SOCKET_SYSCALL = 6, process descriptor ownership, socket backing
  state, TALOS_CLOSE_SYSCALL = 2, waitpid, and laststatus.
- fixed: Confirmed deterministic controls remain retained for malformed
  arguments, missing executable identity, unsupported domain/type/protocol,
  invalid and closed descriptors, wrong-owner backing, descriptor capacity,
  socket backing capacity, scalar dispatch ENOTSUP outside the socket-aware
  path, bounded syscall vocabulary, and unchanged /bin/pingdiag behavior.
- fixed: Recorded the remaining gaps explicitly: send/recv, bind/connect,
  listen/accept, poll/blocking network I/O, UDP/TCP payload transport, live
  driver adapters, live packet I/O, hardware reachability, SSH, public stable
  socket ABI acceptance, and phase transition are not accepted by this chain.
- not-an-issue: The retained smoke transcript labels its evidence as
  host/QEMU-substitute only. The no_std QEMU runner's full-suite execution for
  each filtered invocation does not broaden the accepted claim.
- removed: No stale runtime source, generated-root artifact, hardware
  artifact, live packet I/O claim, or phase-transition claim was introduced
  by the closeout.
- deferred: Supervisor planning is required for any next bounded Phase 12.4
  socket or network task.

## Evidence

- Socket ABI contract:
  tasks/2026-06-20-phase12-network-socket-open-close-abi-contract.md.
- Socket open/close core:
  tasks/2026-06-20-phase12-network-socket-open-close-core.md.
- Shell sockdiag open/close core:
  tasks/2026-06-20-phase12-network-shell-sockdiag-open-close-core.md.
- Retained smoke task:
  tasks/2026-06-20-phase12-network-shell-sockdiag-open-close-smoke.md.
- Retained smoke transcript:
  tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/smoke-transcript.md.
- Smoke classification:
  tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/classification.json.
- Smoke evidence map:
  tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/evidence-map.json.

## Accepted Boundary

The accepted boundary is shell-visible socket open/close through the
VFS/userspace syscall path:

- /bin/sockdiag exists as a read-only initramfs executable identity.
- Shell exec /bin/sockdiag uses the existing VFS open/read execution path.
- Startup ABI and process lifecycle accounting are preserved.
- AF_INET=2, SOCK_STREAM=1, protocol=0 opens through the private experimental
  TALOS_SOCKET_SYSCALL = 6.
- The process descriptor is a DescriptorObjectKind::Socket owned by the
  current process with bounded backing state.
- Close uses TALOS_CLOSE_SYSCALL = 2 and drops both process descriptor and
  socket backing state.
- Shell-visible observation goes through waitpid and laststatus.
- /bin/pingdiag and the accepted descriptor/syscall regression surfaces remain
  unchanged.

This boundary is not a TCP/IP stack, not live driver integration, not packet
I/O, not public libc/POSIX socket compatibility, and not SSH readiness.

## Remaining Gaps

The next socket/network work still requires explicit supervisor planning. At
minimum, later tasks must separately define and validate:

- send and recv semantics, caller buffers, partial I/O, and error mapping.
- bind, connect, listen, accept, local/remote endpoint state, and descriptor
  state transitions.
- poll/blocking behavior, wait queues, readiness, and cancellation/error
  semantics.
- UDP/TCP payload transport and any smoltcp or equivalent integration.
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
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, generated-root publication, send/recv, bind/connect/listen/accept,
UDP/TCP payload transport, SSH, smoltcp, broad socket expansion, public stable
socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=null.

planningNeeded=true.

The accepted evidence level remains source/unit plus host/QEMU-substitute
smoke evidence over shell-visible VFS/userspace /bin/sockdiag execution, the
selected socket open/close syscall path, process descriptor ownership,
close/drop behavior, waitpid/laststatus observation, deterministic controls,
and unchanged accepted diagnostics. Supervisor planning is required before any
broader socket, send/recv, UDP/TCP, live packet I/O, SSH, hardware work, or
phase transition.

Commit: recorded in durable supervisor state after commit creation.
