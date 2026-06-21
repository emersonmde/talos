# Phase 12.4 smoltcp No-Std Dependency Core

Task: phase12-network-smoltcp-no-std-dependency-core-20260621

Status: accepted

Classification: phase12-network-smoltcp-no-std-dependency-core-accepted

## Scope

Add the minimal smoltcp dependency selected by the accepted adoption contract
and introduce the thinnest Talos-owned source boundary that proves the
dependency compiles under the existing no_std/custom-target gates without
accepting packet movement or TCP/UDP behavior.

This task does not establish a TCP connection, transport UDP/TCP payload bytes,
bridge Talos socket syscalls to smoltcp, replace accepted ARP/ICMP/pingdiag or
sockdiag behavior, implement a live driver adapter, perform live packet I/O,
run hardware, mutate the lab, publish a boot image, accept hardware
reachability, accept SSH, accept a public stable socket ABI, broaden socket
behavior, or transition phase.

## Findings And Dispositions

- fixed: Cargo now pins smoltcp 0.13.1 with default-features=false and only
  medium-ethernet, proto-ipv4, and socket-tcp enabled for this slice.
- fixed: Cargo.lock records the mechanically required transitive crates
  selected by that no-default feature set: bitflags, byteorder, cfg-if, hash32,
  heapless, managed, and stable_deref_trait.
- fixed: src/network.rs now owns SmoltcpDependencyCore beside the accepted
  NetworkDevice and PacketQueueNetworkDevice boundary. The core records only
  smoltcp EthernetAddress, Ipv4Cidr, and closed TCP state.
- fixed: SmoltcpDependencyCore::poll_without_device returns
  NoDeviceBound deterministically, proving the dependency core fails closed
  until a later task explicitly binds packet-device adapter behavior.
- fixed: Focused source/unit coverage proves the smoltcp Ethernet, IPv4, and
  TCP feature symbols are available while TCP state remains Closed and no
  device-bound poll consumes or emits frames.
- not-an-issue: The first cargo test attempt failed because QEMU was not on
  PATH. Re-running with the project QEMU path from workspace tool notes passed
  the full no_std QEMU-substitute suite.
- not-an-issue: No bounded count/buffer smoltcp features were needed for this
  dependency-core slice; the selected minimal feature set compiled as-is.
- deferred: Moving frames through smoltcp, defining Interface/Device ownership,
  polling socket sets, host-only TCP handshake evidence, socket syscall
  bridging, live packet I/O, hardware reachability, SSH, and public ABI
  acceptance remain future explicit tasks.
- removed: No fake TCP/UDP behavior, shell diagnostic expansion, hardware
  action, lab mutation, boot publication, or live packet I/O claim was added.

## Implementation

- Cargo.toml adds:
  - smoltcp = 0.13.1
  - default-features = false
  - features = medium-ethernet, proto-ipv4, socket-tcp
- Cargo.lock records smoltcp 0.13.1 and only the transitive crates required by
  that dependency resolution.
- src/network.rs adds:
  - SmoltcpDependencyCorePollResult::NoDeviceBound
  - SmoltcpDependencyCore
  - constructor/accessors for the selected smoltcp Ethernet, IPv4 CIDR, and TCP
    state symbols
  - poll_without_device() as the deterministic fail-closed boundary
- src/network.rs tests add:
  - smoltcp_dependency_core_keeps_tcp_closed_until_device_adapter_exists

## Evidence

- source/unit plus host/QEMU-substitute:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: passed
  - output summary: running 687 talos no_std tests; test result: ok. 687 passed
- fmt:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo fmt --all -- --check
  - result: passed after formatting the new test call shape
- environment control:
  - command: . "$HOME/.cargo/env"; cargo -Zjson-target-spec test --quiet
  - result: failed before source execution because qemu-system-aarch64 was not
    on PATH
  - disposition: not-an-issue; rerun with the project QEMU path passed

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet with project QEMU path: passed.
- git diff --check: passed.
- mdbook build: passed with existing large search-index warning.
- git diff --cached --check: passed.

No TCP connection establishment, UDP/TCP payload transport, socket syscall
bridge, shell diagnostic expansion, public stable socket ABI acceptance, live
driver adapter, live packet I/O, Pi 5 hardware run, hardwareTestLock
acquisition, lab mutation, boot publication, hardware reachability, SSH, broad
socket expansion, or phase transition was performed.

## Acceptance

Accepted.

The accepted frontier is a no_std smoltcp dependency core and deterministic
fail-closed Talos source boundary only. The next objective task is
phase12-network-smoltcp-packet-device-adapter-core-20260621, which may connect
smoltcp to accepted fixed packet/device state through host-only fake-device
tests. That later task must still avoid TCP handshake acceptance, socket
syscall bridging, live packet I/O, hardware reachability, SSH, public ABI
acceptance, broad socket expansion, and phase transition.

Selected next task:
phase12-network-smoltcp-packet-device-adapter-core-20260621.

Commit: recorded in durable supervisor state after commit creation.
