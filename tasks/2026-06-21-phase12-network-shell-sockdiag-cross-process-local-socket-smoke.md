# Phase 12.4 Shell Sockdiag Cross-Process Local Socket Smoke

Task: phase12-network-shell-sockdiag-cross-process-local-socket-smoke-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-cross-process-local-socket-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for shell-visible /bin/sockdiag
cross-process local socket rendezvous. The smoke records VFS/userspace
executable lookup/open/read, distinct server/client ProcessOwnerId descriptor
owners, cross-process connect/accept, bidirectional payload transfer,
readiness and bounded wait wakeups, cleanup/EOF, deterministic controls,
waitpid, and laststatus.

This task does not accept Pi 5 hardware behavior, hardwareTestLock
acquisition, lab mutation, boot publication, generated-root publication, live
driver adapters, live packet I/O, hardware reachability, UDP/TCP payload
transport, SSH, smoltcp, public stable socket ABI acceptance, broad socket
expansion, or a phase transition.

## Findings And Dispositions

- fixed: Added scripts/qemu-shell-sockdiag-cross-process-local-socket-smoke.sh
  as the task-owned host/QEMU-substitute smoke command.
- fixed: Retained smoke output under
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/
  with command log, source anchors, transcript, classification, and evidence
  map.
- fixed: The first smoke attempt exposed predecessor drift: Connected/Accepted
  state comparisons omitted the accepted connection_id field, and listen()
  checked duplicate active listeners while holding a mutable socket borrow.
  The task fixed those compile blockers without changing the accepted runtime
  contract.
- fixed: The cross-process poll-wait test now drains payload bytes before
  asserting a peer-close hangup wait blocks, so the test proves the intended
  hangup wake instead of an already-readable payload fast path.
- fixed: The retained positive path proves exec /bin/sockdiag through VFS
  executable lookup, VFS open/read, startup ABI, server-owned listener,
  client-owned connected socket, server-owned accepted socket, payload
  transfer, listener/payload bounded waits, peer-close hangup readiness,
  cleanup release, waitpid, and laststatus.
- fixed: Deterministic controls remain retained for same descriptor numbers in
  different process owners, malformed arguments, missing executable identity,
  prior process-local sockdiag diagnostics, /bin/pingdiag, and bounded syscall
  vocabulary.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The retained transcript records five
  passing 686-test invocations and labels the intended boundary checks.
- removed: No lab artifact, hardware claim, public socket ABI claim, UDP/TCP
  transport claim, broad socket claim, or phase-transition claim was added.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any broader socket/network direction.

## Evidence

- Smoke command:
  scripts/qemu-shell-sockdiag-cross-process-local-socket-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/smoke-transcript.md.
- Command transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/qemu-shell-sockdiag-cross-process-local-socket-smoke.log.
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/source-anchors.txt.
- Classification:
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/evidence-map.json.
- Accepted predecessor:
  phase12-network-shell-sockdiag-cross-process-local-socket-core-20260621
  accepted and committed at a84e52071ed986d6d22e054173945db322f92b83.

## Validation

- scripts/qemu-shell-sockdiag-cross-process-local-socket-smoke.sh: passed,
  five host/QEMU-substitute test invocations each reporting 686 no_std tests
  passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, UDP/TCP payload transport, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, or phase
transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-cross-process-local-socket-closeout-20260621.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace /bin/sockdiag execution, VFS executable identity,
startup ABI, distinct server/client ProcessOwnerId descriptor owners,
server-owned listener/accepted descriptors, client-owned connected descriptor,
private cross-process local socket rendezvous, bidirectional payload transfer,
bounded readiness/wait wakeups, peer close, cleanup release, waitpid,
laststatus, deterministic controls, unchanged accepted process-local socket
diagnostics, unchanged /bin/pingdiag, and unchanged bounded syscall
vocabulary. Kernel fake commands, UDP/TCP payload transport, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, lab mutation,
boot publication, generated-root publication, broad socket expansion, public
stable socket ABI acceptance, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
