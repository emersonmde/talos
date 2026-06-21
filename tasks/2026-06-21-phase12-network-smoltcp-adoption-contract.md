# Phase 12.4 smoltcp Adoption Contract

Task: phase12-network-smoltcp-adoption-contract-20260621

Status: accepted

Classification: phase12-network-smoltcp-adoption-contract-accepted

## Scope

Define the first bounded smoltcp adoption boundary after the accepted
cross-process local socket frontier. This task decides whether smoltcp should
enter Talos as the TCP/IP stack for the next host-only transport slice, names
the exact dependency and adapter boundary, and selects the follow-up task only
if the implementation path is objective.

This task does not add runtime source behavior. It does not edit Cargo
dependencies, compile smoltcp into Talos, implement a smoltcp device adapter,
run a TCP handshake, bridge Talos socket syscalls to smoltcp, perform Pi 5
hardware work, acquire hardwareTestLock, mutate the lab, publish a boot image,
accept live packet I/O, accept public stable socket ABI behavior, accept SSH,
broaden socket behavior, or transition phase.

## Findings And Dispositions

- fixed: smoltcp is accepted as the bounded first external TCP/IP stack
  dependency because it is no_std-capable, designed for bare-metal systems, and
  exposes socket, interface, phy, wire, and storage layers without requiring a
  host OS networking backend.
- fixed: The first implementation boundary is dependency-only plus a fail-closed
  Talos-owned module boundary. It should add smoltcp 0.13.1 with
  default-features=false and only Ethernet, IPv4, and TCP-oriented features
  needed for later host-only handshake work.
- fixed: Cargo dependency review found smoltcp 0.13.1 requires Rust 1.91 and
  Talos currently builds with rustc 1.97.0-nightly, so the MSRV is mechanically
  compatible with the current toolchain.
- fixed: The dependency boundary must not enable std, log, libc, raw-socket,
  tuntap, DHCP, DNS, IPv6, fragmentation, async, multicast, auto ICMP reply, or
  host OS phy features. Those features would widen the surface beyond the
  host-only Ethernet/IPv4/TCP adapter path.
- fixed: State ownership remains Talos-owned. The follow-up dependency core may
  define a small wrapper module for future smoltcp ownership, but sockets,
  process descriptors, scheduler waits, packet queues, and NetworkDevice
  behavior remain owned by existing Talos types until later explicit adapter
  tasks move frames or prove TCP behavior.
- fixed: The selected feature set for the follow-up core is
  default-features=false with medium-ethernet, proto-ipv4, socket-tcp, and
  bounded count features only as required by compilation. UDP is deliberately
  deferred because the currently planned evidence path is a host-only TCP
  handshake, not UDP payload transport.
- fixed: The next implementation task owns only Cargo.toml/Cargo.lock changes
  and minimal source tests proving the dependency boundary compiles and fails
  closed without accepting TCP/UDP behavior.
- not-an-issue: smoltcp's default feature set includes std, host phy backends,
  IPv6, DHCP/DNS, UDP/TCP, async, multicast, and automatic ICMP behavior. That
  is not a blocker because this contract rejects default features and requires
  a narrow explicit feature list.
- not-an-issue: Existing Talos packet pump, packet queues, runtime ping,
  pingdiag, and sockdiag surfaces are sufficient predecessor evidence to define
  the smoltcp adoption boundary. They are regression/control surfaces, not
  proof of smoltcp TCP behavior.
- deferred: smoltcp packet-device adapter behavior, host-only TCP handshake
  evidence, socket syscall bridging, UDP/TCP payload transport acceptance, live
  driver adapters, live packet I/O, hardware reachability, SSH, public socket
  ABI acceptance, and broad socket expansion remain future tasks.
- removed: No runtime source implementation, dependency edit, hardware action,
  lab mutation, boot publication, or TCP/UDP behavior claim was added.

## Accepted Contract

Adopt smoltcp as the first bounded TCP/IP stack dependency for Phase 12.4, but
only through a dependency-core task before any packet movement or TCP behavior
claim.

The follow-up core task should add:

- smoltcp version: 0.13.1
- dependency shape: default-features = false
- required first features: medium-ethernet, proto-ipv4, socket-tcp
- optional bounded configuration features: only the smallest count/buffer
  features needed to compile and test the no_std boundary deterministically
- rejected features for this slice: std, log, libc, phy-raw_socket,
  phy-tuntap_interface, medium-ip, medium-ieee802154, proto-ipv6,
  proto-dhcpv4, proto-dns, fragmentation, socket-udp, socket-raw, socket-icmp,
  async, multicast, auto-icmp-echo-reply, defmt, and packetmeta-id

The source owner for the first boundary should be a small Talos network module
area near src/network.rs or a subordinate module named by the implementation
task. The module may type-check a narrow future-smoltcp ownership boundary and
deterministic fail-closed/no-op behavior, but it must not drive frames through
smoltcp, expose smoltcp through the Talos socket syscall table, allocate a
public socket ABI, or claim UDP/TCP payload transport.

State ownership remains explicit:

- Talos NetworkDevice and PacketQueueNetworkDevice own raw frame ingress and
  egress until the packet-device adapter task.
- smoltcp Interface, Device, socket set, TCP buffers, timestamps, and polling
  cadence must be owned behind a Talos wrapper before they can affect runtime
  behavior.
- Talos process descriptor tables, NetworkSocketDescriptorTable,
  SocketPollWaitTable, scheduler wait state, /bin/sockdiag, and /bin/pingdiag
  remain separate from smoltcp until a later bridge task explicitly maps them.
- Failure modes at the dependency core boundary are deterministic no-op or
  fail-closed outcomes; no packet may be consumed, emitted, or reported as TCP
  progress by merely adding the dependency.

The selected next bounded task is
phase12-network-smoltcp-no-std-dependency-core-20260621.

## Evidence

- static source/task/docs/dependency review:
  - src/network.rs owns the accepted NetworkDevice trait, PacketQueueNetworkDevice,
    fixed packet queues, host-only packet pump, ARP/IPv4/ICMP helpers, route
    policy, packet service, and the later local socket backing types.
  - src/syscall.rs and src/posix.rs own the accepted private socket syscall,
    process descriptor, copy-in/copy-out, and bounded poll-wait surfaces that
    remain separate from smoltcp in this contract.
  - tasks/2026-06-21-phase12-network-shell-sockdiag-cross-process-local-socket-closeout.md
    records the immediate predecessor frontier: private cross-process local
    socket rendezvous over VFS/userspace /bin/sockdiag only.
  - Accepted packet pump, packet queue, pingdiag, and sockdiag task records
    remain regression/control evidence for Talos-owned frame and descriptor
    boundaries before smoltcp adoption.
  - cargo search/cargo info reported smoltcp 0.13.1 as a bare-metal no-heap
    TCP/IP stack with rust-version 1.91, documentation at docs.rs, and the
    feature vocabulary used by this contract.
  - Local registry inspection of smoltcp 0.13.1 confirmed no_std is active
    when neither tests nor std are enabled, and confirmed compile-time feature
    checks for at least one protocol, socket type, and medium when socket
    support is enabled.

## Validation

- static source/task/docs/dependency review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No runtime source implementation, Cargo dependency edit, generated userland
change, smoke harness, retained execution transcript, Pi 5 hardware run,
hardwareTestLock acquisition, boot archive publication, lab mutation, power
cycle, live driver adapter, live packet I/O, hardware reachability, UDP/TCP
payload transport, socket syscall bridge, SSH, broad socket expansion, public
stable socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is a contract-only smoltcp adoption decision. smoltcp
may enter Talos through the next dependency-core task as a no_std, non-std,
host-only Ethernet/IPv4/TCP dependency boundary with deterministic fail-closed
behavior. That evidence will not accept TCP/UDP payload transport, live packet
I/O, hardware reachability, socket syscall bridging, SSH, public stable socket
ABI behavior, broad socket expansion, or phase transition.

Selected next task:
phase12-network-smoltcp-no-std-dependency-core-20260621.

Commit: recorded in durable supervisor state after commit creation.
