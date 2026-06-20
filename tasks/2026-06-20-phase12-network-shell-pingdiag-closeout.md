# Phase 12.4 Shell Pingdiag Closeout

Task: phase12-network-shell-pingdiag-closeout-20260620

Status: accepted

Classification: phase12-network-shell-pingdiag-closeout-accepted

## Scope

Close out the shell-visible `/bin/pingdiag` core before retained smoke or
broader networking work. This reconciles the accepted contract, core source,
tests, docs, task evidence, durable-state frontier, accepted claims, and
rejected claims.

This closeout does not add runtime source behavior. It does not accept public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, Pi 5 hardware work,
hardwareTestLock acquisition, lab mutation, boot publication, SSH, smoltcp,
UDP/TCP, broad shell expansion, broad socket expansion, Phase 12.1 hardware
retry, or a phase transition.

## Findings And Dispositions

- not-an-issue: The shell-visible `/bin/pingdiag` core is accepted. The
  evidence level remains source/unit host/QEMU-substitute over the VFS
  executable identity, command-loop execution transcript, diagnostic SVC
  user-argument decoding, process-local descriptor ownership, UserMapping
  copy-in/copy-out, packet queues, and
  `PacketQueueNetworkDevice::pump_driver`.
- not-an-issue: The core uses the existing VFS open/read, ELF planning,
  startup ABI, lifecycle, `waitpid`, and `laststatus` transcript surfaces
  before running the accepted diagnostic lifecycle. It is not a kernel-backed
  fake shell command.
- not-an-issue: The focused shell test proves open/start/pump/status/result
  and close over deterministic ARP and ICMP progression, including outbound
  ARP/ICMP transfer through `PacketQueueNetworkDevice::pump_driver`, injected
  reply progression, completed result/status copy-out, descriptor close, and
  lifecycle observation.
- not-an-issue: Deterministic negative/control evidence remains covered by the
  focused shell transcript plus accepted diagnostic SVC tests: malformed
  arguments, missing executable identity, wrong owner/descriptor, invalid and
  closed descriptors, queue capacity/backpressure, timeout/retry, transmit and
  receive device errors, close/drop behavior, and unchanged
  SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- not-an-issue: The queued retained smoke task is objective and bounded. It has
  explicit dependencies, acceptance criteria, validation gates, evidence
  requirements, and rejected-claim boundaries for a host/QEMU-substitute
  transcript only.
- removed: No closeout-only runtime source cleanup was justified; no source
  behavior was changed in this closeout.
- deferred: Public sockets, stable/socket ABI acceptance, live driver
  adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
  mutation, boot publication, broad shell expansion, broad socket expansion,
  Phase 12.1 hardware retry, and phase transition remain deferred.

## Evidence Reviewed

- Contract task:
  tasks/2026-06-20-phase12-network-shell-pingdiag-contract.md.
- Core task:
  tasks/2026-06-20-phase12-network-shell-pingdiag-core.md.
- Source:
  - src/initramfs.rs `PHASE12_PINGDIAG_PATH` and generated-root `/bin`
    entry.
  - src/local_command_loop.rs shell VFS exec path,
    `exec_shell_pingdiag_diagnostic`, transcript formatting, and
    `local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers`.
  - src/syscall.rs `VfsPingDiagnosticSvcFixture` and process-local ping
    diagnostic controls.
  - src/network.rs `PacketQueueNetworkDevice` and
    `PacketQueueNetworkDevice::pump_driver`.
- Phase 12 architecture: docs/src/project/phase12-networking-ssh.md.
- Roadmap frontier: docs/src/roadmap.md.
- Durable queued retained smoke task:
  phase12-network-shell-pingdiag-smoke-20260620.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, public socket API, stable syscall ABI acceptance,
socket syscall ABI acceptance, live driver adapter, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, broad shell expansion, broad socket
expansion, or phase transition was performed.

## Acceptance

Accepted.

The accepted evidence level remains source/unit host/QEMU-substitute over
shell-visible VFS/userspace diagnostic execution. The accepted boundary is the
experimental `/bin/pingdiag` transcript through the VFS executable identity,
diagnostic SVC user-argument bridge, process-local descriptor ownership,
UserMapping copy-in/copy-out, fixed-capacity packet queues,
`PacketQueueNetworkDevice::pump_driver`, status/result copy-out,
close/drop behavior, and `waitpid`/`laststatus` lifecycle observation.

selected_next_task=phase12-network-shell-pingdiag-smoke-20260620.

planningNeeded=false.

The retained smoke task is mechanically unblocked only after this closeout is
committed and may retain a task-owned host/QEMU-substitute transcript. It must
not claim public sockets, stable/socket ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, lab mutation, boot publication,
broad socket expansion, Phase 12.1 hardware retry, or a phase transition.

Commit: recorded in durable supervisor state after commit creation.
