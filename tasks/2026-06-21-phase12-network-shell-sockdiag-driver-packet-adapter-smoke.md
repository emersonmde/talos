# Phase 12.4 Shell Sockdiag Driver Packet Adapter Smoke

Task: phase12-network-shell-sockdiag-driver-packet-adapter-smoke-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-driver-packet-adapter-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence proving shell-visible /bin/sockdiag
observes deterministic DriverPacketAdapter substrate state through the accepted
VFS/userspace/private socket ABI path.

This task does not change source behavior, acquire hardwareTestLock, mutate the
lab, publish a boot artifact, retain generated-root evidence, perform live
packet I/O, claim Pi 5 hardware behavior, claim hardware reachability, accept
SSH, add UDP/raw sockets, add libc/std wrappers, accept POSIX/Linux
compatibility, accept a public stable socket ABI, broaden sockets, or
transition phase.

## Findings And Dispositions

- fixed: Added scripts/qemu-shell-sockdiag-driver-packet-adapter-smoke.sh as
  the task-owned retained host/QEMU-substitute smoke command.
- fixed: Retained smoke output under
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/ with
  command log, source anchors, transcript, classification, and evidence map.
- fixed: The smoke records exec /bin/sockdiag through VFS executable
  lookup/open/read, startup ABI, userspace_socket_abi wrapper-built calls,
  descriptor-backed socket dispatch, private smoltcp TCP bridge continuity,
  deterministic DriverPacketAdapter RX/TX and backpressure observations,
  waitpid, and laststatus.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, userspace ABI wrapper dispatch, unchanged local
  socket diagnostics, unchanged smoltcp bridge coverage, unchanged /bin/pingdiag
  behavior, copied adapter RX/TX movement, TX backpressure, capacity/device
  errors, and bounded syscall vocabulary.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The retained transcript records six
  passing 698-test invocations and labels the intended boundary checks.
- removed: No source behavior change, lab artifact, hardware claim, generated
  root publication, live packet I/O claim, public socket ABI claim, POSIX/Linux
  compatibility claim, UDP/raw socket claim, SSH claim, broad socket claim, or
  phase-transition claim was added.
- deferred: The selected closeout task remains responsible for reconciling
  this retained adapter smoke evidence before supervisor planning decides any
  broader socket/network direction.

## Evidence

- Smoke command:
  scripts/qemu-shell-sockdiag-driver-packet-adapter-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/smoke-transcript.md.
- Command transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/qemu-shell-sockdiag-driver-packet-adapter-smoke.log.
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/source-anchors.txt.
- Classification:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/evidence-map.json.
- Accepted predecessor:
  phase12-network-shell-sockdiag-driver-packet-adapter-core-20260621 accepted
  and committed at 25b6fa53103537a07b0127e19db8b229e58155c3.

## Validation

- scripts/qemu-shell-sockdiag-driver-packet-adapter-smoke.sh: passed, six
  host/QEMU-substitute test invocations each reporting 698 no_std tests passed.
- cargo fmt --all -- --check: not run because no Rust/source formatting change
  was made in this smoke task.
- cargo -Zjson-target-spec test --quiet: not run because no source behavior
  changed; the task-owned smoke ran the focused QEMU-substitute filters.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, UDP/raw socket work, SSH, broad socket
expansion, POSIX/Linux compatibility acceptance, public stable socket ABI
acceptance, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-driver-packet-adapter-closeout-20260621.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace /bin/sockdiag execution, VFS executable identity,
startup ABI, userspace_socket_abi helper constructors, descriptor-backed socket
dispatch, private smoltcp TCP bridge continuity, deterministic
DriverPacketAdapter RX/TX movement, TX backpressure preserving queued RX,
capacity/error controls, waitpid, laststatus, deterministic controls, unchanged
accepted local socket diagnostics, unchanged /bin/pingdiag, and unchanged
bounded syscall vocabulary. Kernel fake networking commands, live driver
programming, live packet I/O, hardware reachability, SSH, lab mutation, boot
publication, generated-root publication, broad socket expansion, UDP/raw
sockets, libc/std wrappers, POSIX/Linux compatibility, public stable socket ABI
acceptance, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
