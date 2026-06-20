# Phase 12.4 Shell Pingdiag Core

Task: phase12-network-shell-pingdiag-core-20260620

Status: accepted

Classification: phase12-network-shell-pingdiag-core-accepted

## Scope

Implement the smallest shell-visible `/bin/pingdiag` diagnostic path backed by
the accepted VFS/userspace diagnostic plumbing. The command is visible in the
read-only initramfs under `/bin`, executes through the existing local shell
`exec` path, opens the VFS-backed diagnostic identity, starts and pumps the
process-local descriptor through `PacketQueueNetworkDevice::pump_driver`,
reports status/result, and closes the descriptor.

This task does not add public sockets, stable syscall ABI acceptance, socket
ABI acceptance, live driver adapters, live packet I/O, hardware reachability,
Pi 5 hardware work, lab mutation, boot publication, SSH, smoltcp, UDP/TCP,
broad shell grammar, PATH, quoting, variables, pipes, redirection support for
`/bin/pingdiag`, broad socket expansion, Phase 12.1 hardware retry, or a phase
transition.

## Findings And Dispositions

- fixed: `/bin/pingdiag` now has a real read-only initramfs executable identity
  and appears in `/bin` listings.
- fixed: `exec /bin/pingdiag` follows the existing VFS open/read, ELF planning,
  startup ABI, lifecycle, waitpid, and laststatus shell transcript path before
  running the accepted diagnostic SVC lifecycle.
- fixed: The shell transcript records open/start/pump/status/result/close over
  process-local descriptor ownership, UserMapping copy-in/copy-out, packet
  queues, and `PacketQueueNetworkDevice::pump_driver`.
- fixed: Focused shell coverage checks success, malformed arguments, missing
  executable identity, accepted error/control vocabulary, waitpid, laststatus,
  and unchanged `SyscallNumber`/`STABLE_SVC_IMMEDIATE`/`TALOS_*` boundaries.
- not-an-issue: Existing `src/syscall.rs` diagnostic tests already cover wrong
  owner/descriptor, invalid and closed descriptors, queue backpressure,
  timeout/retry, device transmit/receive errors, close/drop behavior, user
  copy faults, and packet queue controls under the same diagnostic SVC path.
- deferred: Public sockets, stable/socket ABI acceptance, live driver adapters,
  live packet I/O, hardware reachability, smoltcp, UDP/TCP, SSH, Phase 12.1
  hardware retry, broad shell expansion, and phase transition remain deferred.
- removed: No dead code was removed; the smallest safe change was additive
  wiring around the accepted diagnostic layers.

## Evidence

- source/unit host/QEMU-substitute:
  - `src/initramfs.rs` adds `/bin/pingdiag` as a read-only executable fixture.
  - `src/local_command_loop.rs` routes `exec /bin/pingdiag` through VFS exec,
    diagnostic SVC user-argument decoding, process-local descriptor ownership,
    UserMapping copy-in/copy-out, packet queues, and
    `PacketQueueNetworkDevice::pump_driver`.
  - `src/syscall.rs` exposes the minimal crate-internal constructors required
    for the shell diagnostic transcript to initialize accepted no-frame state.
  - `local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers`
    proves the shell-visible transcript plus malformed-argument and
    missing-executable controls.

## Validation

- cargo fmt --all: passed
- cargo -Zjson-target-spec test --quiet: passed
- git diff --check: passed
- /home/node/.cargo/bin/mdbook build: passed
- git diff --cached --check: passed

## Next Action

Selected next task:
phase12-network-shell-pingdiag-closeout-20260620.

The next task may only reconcile the accepted shell-visible `/bin/pingdiag`
core source, tests, docs, task evidence, accepted claims, and rejected claims.
It must not start public sockets, live driver adapters, hardware reachability,
SSH, or phase-transition work.
