# Phase 12.4 Shell Pingdiag Smoke

Task: phase12-network-shell-pingdiag-smoke-20260620

Status: accepted

Classification: phase12-network-shell-pingdiag-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the shell-visible
`/bin/pingdiag` diagnostic path. The smoke records a command-loop transcript
that opens, starts, pumps, observes status/result, and closes `/bin/pingdiag`
through the accepted VFS/userspace diagnostic plumbing, process-local
descriptor ownership, UserMapping copy-in/copy-out, packet queues, and
`PacketQueueNetworkDevice::pump_driver`.

This task does not add source runtime behavior. It does not accept public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, Pi 5 hardware work,
hardwareTestLock acquisition, lab mutation, boot publication, SSH, smoltcp,
UDP/TCP, broad shell expansion, broad socket expansion, Phase 12.1 hardware
retry, or a phase transition.

## Findings And Dispositions

- fixed: Added `scripts/qemu-shell-pingdiag-smoke.sh` as the task-owned
  host/QEMU-substitute smoke command for the shell-visible `/bin/pingdiag`
  boundary.
- fixed: Retained smoke output under
  `tasks/evidence/2026-06-20-shell-pingdiag-smoke/` with command log, source
  anchors, transcript, classification, and evidence map.
- fixed: The retained positive path proves `exec /bin/pingdiag` through VFS
  executable lookup, VFS open/read, startup ABI, diagnostic SVC user-argument
  decoding, process-local descriptor open/start/pump/status/result/close,
  outbound ARP and IPv4/ICMP transfer through
  `PacketQueueNetworkDevice::pump_driver`, injected ARP/ICMP reply
  progression, completed status/result copy-out, `waitpid`, and
  `laststatus`.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, owner/descriptor failures, invalid and closed
  descriptors, queue capacity/backpressure, caller buffer pressure, malformed
  received frames, timeout/retry, transmit and receive device errors,
  close/drop behavior, and unchanged
  `SyscallNumber`/`STABLE_SVC_IMMEDIATE`/`TALOS_*` vocabulary.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The transcript records that behavior and
  labels the intended boundary checks; the evidence remains
  host/QEMU-substitute smoke and includes five passing 663-test invocations.
- removed: No runtime source behavior, public socket surface, stable ABI, live
  driver path, lab artifact, hardware claim, or phase-transition claim was
  added by this retained smoke task.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any live driver adapter, live packet
  I/O, public socket, SSH, or phase-transition direction.

## Evidence

- Smoke command:
  `scripts/qemu-shell-pingdiag-smoke.sh`.
- Retained transcript:
  `tasks/evidence/2026-06-20-shell-pingdiag-smoke/smoke-transcript.md`.
- Command transcript:
  `tasks/evidence/2026-06-20-shell-pingdiag-smoke/qemu-shell-pingdiag-smoke.log`.
- Source anchors:
  `tasks/evidence/2026-06-20-shell-pingdiag-smoke/source-anchors.txt`.
- Classification:
  `tasks/evidence/2026-06-20-shell-pingdiag-smoke/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-20-shell-pingdiag-smoke/evidence-map.json`.
- Accepted predecessor:
  phase12-network-shell-pingdiag-closeout-20260620 accepted and committed at
  14b41fc96f9d9dcefda7e7c9d4baff71a99b962f.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 663 no_std tests.
- scripts/qemu-shell-pingdiag-smoke.sh: passed, five host/QEMU-substitute test
  invocations each reporting 663 no_std tests passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, public socket API, stable syscall ABI acceptance, socket syscall
ABI acceptance, SSH, smoltcp, UDP/TCP, broad shell expansion, broad socket
expansion, Phase 12.1 hardware retry, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-pingdiag-smoke-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace diagnostic execution, command-loop transcript,
VFS executable identity, diagnostic SVC user-argument decoding,
process-local descriptor ownership, UserMapping copy-in/copy-out,
fixed-capacity packet queues, `PacketQueueNetworkDevice::pump_driver`,
caller-owned buffers, task-owned state, status/result copy-out, close/drop
behavior, `waitpid`, `laststatus`, and unchanged
`SyscallNumber`/`STABLE_SVC_IMMEDIATE`/`TALOS_*` vocabulary. Kernel fake
commands, public sockets, stable/socket ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad socket expansion, and
phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
