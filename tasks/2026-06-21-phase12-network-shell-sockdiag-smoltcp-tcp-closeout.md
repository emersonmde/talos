# Phase 12.4 Shell Sockdiag smoltcp TCP Closeout

Task: phase12-network-shell-sockdiag-smoltcp-tcp-closeout-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-smoltcp-tcp-closeout-accepted

## Scope

Close out the shell-visible smoltcp TCP diagnostic frontier after the accepted
private smoltcp socket bridge core, shell-visible /bin/sockdiag source/unit
diagnostic, and retained host/QEMU-substitute smoke evidence. This
reconciliation records the accepted evidence level, accepted claims, rejected
claims, remaining gaps, and planning status before any live packet I/O,
hardware reachability, SSH, public ABI stabilization, or broader socket/network
direction.

This task does not implement runtime behavior. It does not change source,
acquire hardwareTestLock, mutate the lab, publish a boot image, retain
generated-root evidence, claim live packet I/O, claim Pi 5 hardware behavior,
claim hardware reachability, accept SSH, accept a public stable socket ABI,
broaden sockets, add UDP/raw sockets, or transition phase.

## Findings And Dispositions

- fixed: Reconciled the accepted sequence from private host-only smoltcp socket
  bridge core, shell-visible /bin/sockdiag source/unit diagnostic, and retained
  host/QEMU-substitute smoke evidence.
- fixed: The accepted frontier is shell-visible VFS/userspace /bin/sockdiag
  execution of the private descriptor-backed host-only smoltcp TCP bridge
  diagnostic through existing private socket syscalls.
- fixed: The accepted evidence covers VFS executable lookup/open/read, startup
  ABI, socket/bind/listen/connect/accept/send/recv/poll/poll-wait/close
  syscall dispatch, descriptor ownership, SmoltcpSocketBridgeRecord reporting,
  Established client/server handshake states, deterministic step/frame
  counters, accepted-descriptor attachment, one bounded payload-transfer
  observation, waitpid, and laststatus.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, unchanged local socket diagnostics, unchanged
  /bin/pingdiag, the predecessor smoltcp bridge regression, and bounded syscall
  vocabulary.
- fixed: Roadmap and Phase 12 networking docs now state that this closeout
  freezes the shell-visible smoltcp TCP diagnostic frontier and requires
  supervisor planning before any later bounded socket/network direction.
- not-an-issue: The retained smoke evidence is host/QEMU-substitute, not Pi 5
  hardware evidence. That is sufficient because this closeout accepts no live
  packet I/O, hardware reachability, boot publication, lab mutation, or SSH
  claim.
- not-an-issue: The accepted smoltcp TCP bridge remains private experimental
  Talos behavior over existing private syscall numbers; this closeout does not
  promote it to a public stable socket ABI or Linux compatibility claim.
- removed: No source behavior change, hardware artifact, generated-root
  artifact, live packet I/O claim, public socket ABI claim, SSH claim, broad
  expansion, UDP/raw socket claim, or phase-transition claim was introduced.
- deferred: Live driver adapters, live packet I/O, Pi 5 hardware reachability,
  SSH, public ABI stabilization, broader socket expansion, UDP/raw sockets, and
  any phase transition remain future supervisor-planned work.

## Evidence

- Private smoltcp socket bridge core:
  tasks/2026-06-21-phase12-network-smoltcp-socket-bridge-core.md.
- Shell-visible diagnostic core:
  tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-core.md.
- Retained smoke task:
  tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-smoke.md.
- Smoke transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/smoke-transcript.md.
- Smoke classification:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/classification.json.
- Smoke evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/evidence-map.json.

## Accepted Boundary

The accepted evidence level is source/unit plus retained host/QEMU-substitute
smoke evidence only. The accepted boundary is shell-visible VFS/userspace
/bin/sockdiag observation of the private descriptor-backed host-only smoltcp
TCP bridge:

- /bin/sockdiag resolves through read-only VFS executable lookup, open, and
  read.
- The diagnostic runs the existing private socket, bind, listen, connect,
  accept, send, recv, poll, poll-wait, and close syscall dispatch path.
- Process-visible descriptors stay owned by their ProcessOwnerId descriptor
  tables.
- The private SmoltcpSocketBridgeRecord reports the connection id, Established
  client/server handshake states, deterministic handshake step/frame counters,
  accepted-descriptor attachment, one bounded payload-transfer observation,
  and Established payload states.
- Shell observation includes waitpid and laststatus.
- Malformed arguments, missing executable identity, accepted local socket
  diagnostics, accepted /bin/pingdiag behavior, and bounded syscall vocabulary
  remain deterministic controls.

This boundary is not live driver integration, not hardware-backed networking,
not public libc/POSIX socket compatibility, and not SSH readiness.

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
- public ABI stabilization or POSIX/Linux compatibility claims for the socket
  surface.
- UDP/raw socket behavior, broader socket option/error semantics, and any phase
  transition.

## Validation

- static source/task/evidence review: passed. Reviewed the smoltcp socket
  bridge core task, shell-visible diagnostic task, retained smoke task, smoke
  transcript, classification, evidence map, roadmap frontier, and Phase 12
  networking doc.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, SSH, UDP/raw socket work, broad socket
expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=null.

planningNeeded=true.

The accepted evidence level remains source/unit plus retained
host/QEMU-substitute smoke evidence over shell-visible VFS/userspace
/bin/sockdiag execution of the private descriptor-backed host-only smoltcp TCP
bridge diagnostic. Live driver adapters, live packet I/O, Pi 5 hardware
behavior, lab mutation, boot publication, generated-root publication, hardware
reachability, SSH, broad socket expansion, UDP/raw sockets, public stable
socket ABI acceptance, and phase transition remain rejected. Supervisor
planning is required before any next bounded socket/network task.

Commit: recorded in durable supervisor state after commit creation.
