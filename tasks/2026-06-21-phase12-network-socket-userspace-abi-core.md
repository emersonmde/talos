# Phase 12.4 Socket Userspace ABI Core

Task: phase12-network-socket-userspace-abi-core-20260621

Status: accepted

Classification: phase12-network-socket-userspace-abi-core-accepted

## Scope

Compile the documented private Talos userspace socket ABI constants and narrow
no_std wrapper helpers against the accepted descriptor-backed socket/smoltcp
TCP contract. Prove one documented wrapper path reaches the accepted host-only
smoltcp TCP socket bridge through source/unit evidence.

This task does not publish a boot image, acquire hardwareTestLock, mutate the
lab, perform live packet I/O, add SSH, add UDP/raw sockets, accept libc/std or
POSIX/Linux compatibility, promote the ABI to public stable status, broaden
sockets, or transition phase.

## Findings And Dispositions

- fixed: Added src/userspace_socket_abi.rs as the private no_std socket ABI
  helper module. It mirrors the accepted selector vocabulary, AF_INET stream
  constants, poll entry layout, bounded poll/wait constants, and errno values
  from src/syscall.rs and src/network.rs.
- fixed: Added const wrapper constructors for socket, bind, listen, connect,
  accept, send, recv, poll, poll_wait, and close. Each wrapper produces the
  documented x8 selector and x0 through x5 scalar argument layout.
- fixed: Added a target_arch="aarch64" svc #0 invoke hook for the wrapper
  call descriptor while keeping host/source-unit tests on the deterministic
  dispatch path.
- fixed: Added source/unit coverage proving the wrapper-built socket, bind,
  listen, connect, accept, send, and recv path reaches the accepted host-only
  SmoltcpSocketBridgeRecord behavior, including Established handshake state,
  accepted descriptor attachment, one payload-transfer observation, and recv
  delivery.
- fixed: Added source/unit coverage proving wrapper constants and the 16-byte
  little-endian poll entry codec match the accepted private kernel contract.
- not-an-issue: The module is private crate-owned vocabulary. It is a bridge
  between accepted Talos userspace ABI documentation and current kernel
  dispatch tests; it does not claim Linux syscall-number compatibility or libc
  support.
- removed: No live driver adapter, live packet I/O, Pi 5 hardware behavior,
  generated-root publication, public ABI claim, SSH claim, UDP/raw socket work,
  broad socket expansion, or phase-transition claim was introduced.
- deferred: Shell-visible /bin/sockdiag use of the new wrapper module,
  retained ABI smoke evidence, live driver adapters, live packet I/O,
  hardware reachability, SSH strategy, UDP/raw sockets, and libc/std socket
  wrappers remain future explicit tasks.

## Implementation

- src/userspace_socket_abi.rs:
  - private constants alias the accepted TALOS_CLOSE=2, TALOS_SOCKET=6,
    TALOS_BIND=7, TALOS_LISTEN=8, TALOS_CONNECT=9, TALOS_ACCEPT=10,
    TALOS_SEND=11, TALOS_RECV=12, TALOS_POLL=13, and TALOS_POLL_WAIT=14
    selectors.
  - SocketAbiCall stores the syscall selector and six scalar arguments and can
    convert to SyscallArguments for source/unit dispatch.
  - PollEntry encodes and decodes the accepted 16-byte little-endian
    fd/events/revents user-memory layout.
  - The aarch64-only invoke hook emits svc #0 with x8 as selector and x0
    through x5 as arguments. Host tests do not execute inline assembly.
- src/main.rs:
  - registers the private module with dead-code allowance outside test builds.

## Evidence

- Focused source/unit test
  userspace_socket_abi_constants_match_private_kernel_contract: passed under
  cargo -Zjson-target-spec test --quiet. Covers selector constants,
  AF_INET/SOCK_STREAM/protocol constants, poll constants, errno aliases, and
  PollEntry encode/decode.
- Focused source/unit test
  userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge: passed under
  cargo -Zjson-target-spec test --quiet. The test routes wrapper-built calls
  through dispatch_process_descriptor_with_socket_table, observes the accepted
  smoltcp Established handshake record after connect, observes accepted
  descriptor attachment after accept, records one payload transfer after send,
  and receives the payload through the accepted recv path.
- Full gate cargo -Zjson-target-spec test --quiet: passed.
- Formatting gate cargo fmt --all -- --check: passed after formatter ordering
  was applied with cargo fmt --all.
- Diff hygiene git diff --check: passed.
- Docs validation /home/node/.cargo/bin/mdbook build: passed.
- Cached diff hygiene git diff --cached --check: passed before commit.

## Accepted Boundary

The accepted evidence level is source/unit plus host/QEMU-substitute cargo
test. The accepted boundary is a private Talos no_std/userspace socket ABI
helper surface that matches the already accepted descriptor-backed
AF_INET/SOCK_STREAM and host-only smoltcp TCP bridge behavior.

This task does not accept live driver adapters, live packet I/O, Pi 5 hardware
reachability, SSH, UDP/raw sockets, libc/std socket wrappers, POSIX/Linux
compatibility, public stable socket ABI acceptance, broad socket expansion, or
phase transition.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-userspace-abi-core-20260621.

The next bounded task may wire shell-visible /bin/sockdiag diagnostics through
the private userspace_socket_abi wrapper surface. It must not broaden beyond
diagnostic use of the documented private ABI or claim retained ABI smoke, live
packet I/O, hardware reachability, SSH, UDP/raw sockets, libc/std,
POSIX/Linux compatibility, public stable ABI acceptance, broad socket
expansion, or phase transition.

Commit: recorded in durable supervisor state after commit creation.
