# Phase 12 Entropy SSH Strategy Contract

Task id: phase12-entropy-ssh-strategy-contract-20260621

Status: accepted

Classification:
phase12-entropy-ssh-strategy-contract-accepted

Evidence level: static source/task/docs/evidence review, docs build, and diff
checks. No runtime implementation, dependency adoption, crypto implementation,
host key generation, SSH server, live packet I/O, hardware reachability,
hardware/lab action, UDP/raw sockets, libc/std wrappers, public ABI/POSIX/Linux
compatibility claim, broad socket expansion, or phase transition was performed.

## Goal

Define the smallest defensible SSH-enabling strategy after accepted host-only
sockets and blocked live Ethernet link readiness, focusing on entropy,
crypto/key management, and implementation/porting choice.

## Scope Performed

- Reviewed the accepted network frontier pause and SSH strategy checkpoint.
- Reviewed accepted host-only socket, smoltcp, userspace socket ABI, and
  DriverPacketAdapter evidence.
- Reviewed retained link-hardware blockers and the paused live Ethernet
  expansion boundary.
- Reviewed repository source for existing entropy, random, crypto, SSH, and
  dependency surfaces.
- Evaluated three strategy options: later port of an existing SSH server, later
  smaller Talos-first SSH service, or blocking on missing prerequisites.
- Selected phase12-entropy-source-contract-20260621 as the next bounded task.

## Findings

- fixed: the near-term SSH-enabling path is prerequisite-first: define and then
  implement a local entropy diagnostic/source boundary before any SSH server,
  host key generation, or crypto dependency adoption.
- fixed: entropy/key-management prerequisites are now concrete enough for the
  next worker task: identify accepted entropy inputs, deterministic controls,
  fail-closed reporting labels, host-key readiness blockers, and rejected
  cryptographic-strength claims.
- deferred: a future existing-server port remains preferable to writing a full
  SSH protocol stack by hand once Talos has entropy, key storage/provisioning,
  crypto dependencies, process/service plumbing, and libc/std constraints clear.
- deferred: a smaller Talos-first Rust SSH service may be the first practical
  service implementation if OpenSSH's libc/std/filesystem/process assumptions
  are still too large when the prerequisite stack matures.
- blocked: premature SSH service acceptance remains blocked by missing entropy,
  host-key generation/provisioning policy, authorized-key storage, service
  lifecycle/exposure controls, and live or substitute transport acceptance.
- rejected: adopting a crypto or SSH dependency before an entropy source
  contract would hide the kernel responsibility this milestone is meant to
  expose.
- rejected: live packet I/O, hardware reachability, SSH service acceptance,
  public ABI/POSIX/Linux compatibility, broad socket expansion, and phase
  transition are not accepted by this strategy contract.
- removed: no source behavior, dependency, hardware/lab helper, task evidence,
  or documentation path was removed.
- not-an-issue: selecting an entropy source contract next is not a crypto or
  SSH-readiness claim; it is the narrowest prerequisite task that can produce
  objective follow-up work locally.

## Strategy Decision

The selected strategy is prerequisite-first SSH enablement:

1. Establish a Talos-owned entropy source contract and diagnostic frontier.
2. Use that diagnostic to classify available entropy inputs and fail-closed
   behavior without claiming cryptographic strength.
3. Later define key-management policy for host keys, authorized keys,
   provisioning, persistence, regeneration, and exposure controls.
4. Only after entropy/key prerequisites are accepted, decide whether the first
   SSH service implementation should be an existing server port or a smaller
   Talos-first Rust service.

OpenSSH remains the compatibility target, but it is not the near-term first
implementation target. The near-term work must expose Talos kernel and
userspace responsibilities rather than importing an SSH stack before entropy,
key policy, filesystem/process expectations, and transport boundaries are
ready.

## Entropy And Key-Management Prerequisites

The next entropy source contract must name accepted and rejected input classes.
Candidate input classes for source/design review include boot-time counters,
timer jitter, scheduler/process events, serial/input timing, hardware-provided
randomness if source-grounded later, and explicit operator-provisioned seed
material. The contract must also define deterministic controls that prove the
diagnostic fails closed when no accepted input exists.

Key-management follow-up tasks remain deferred until entropy is bounded. They
must define host-key generation versus provisioning, storage location and
persistence, authorized-key storage, regeneration policy, service exposure
defaults, audit labels, memory lifetime expectations, and failure modes.

No entropy diagnostic may claim cryptographic strength, SSH readiness, hardware
randomness, live packet I/O, reachability, or phase transition until a later
task proves those claims with explicit evidence.

## Evidence

- Network frontier pause and SSH strategy checkpoint:
  tasks/2026-06-21-phase12-network-frontier-pause-and-ssh-strategy-checkpoint.md.
- Driver packet adapter closeout:
  tasks/2026-06-21-phase12-network-driver-packet-adapter-closeout.md.
- BCM54213PE lifecycle ownership closeout:
  tasks/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout.md.
- Link-ready discriminator source contract:
  tasks/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract.md.
- Source surface review: Cargo.toml, src/network.rs,
  src/userspace_socket_abi.rs, src/syscall.rs, and repository-wide search for
  entropy/random/crypto/SSH terms.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static source/task/docs/evidence review: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Strategy contract names the chosen near-term SSH-enabling path and rejects
  premature SSH service acceptance: satisfied.
- Entropy/key-management prerequisites are concrete enough for a worker to
  implement or block mechanically: satisfied.
- selected_next_task=phase12-entropy-source-contract-20260621: satisfied.
- No live packet I/O, hardware reachability, SSH service, public ABI/POSIX/Linux
  compatibility, broad expansion, or phase transition claim is made: satisfied.

## Next Action

Promote phase12-entropy-source-contract-20260621 on the next worker wake if
dependencies remain satisfied. The next task must remain a source/design
contract for entropy diagnostics; it must not implement crypto, generate host
keys, accept SSH readiness, adopt SSH dependencies, or claim live transport.
