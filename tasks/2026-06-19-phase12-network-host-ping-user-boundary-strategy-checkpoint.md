# Phase 12.3 Host Ping User-Boundary Strategy Checkpoint

Task: phase12-network-host-ping-user-boundary-strategy-checkpoint-20260619
Status: accepted

## Goal

Reconcile the accepted host-only single-ping evidence against the next useful
user-visible networking path without treating fake shell command expansion as
feature progress.

## Scope

- Reconcile the accepted host-only single-ping frontier against Phase 12.3 and
  Phase 12.4 roadmap requirements.
- Classify the next objective feature step among packet queue/pump,
  caller-driven timer integration, neighbor-discovery state expansion,
  driver-adapter contract, socket/user-program boundary, smoltcp adoption
  checkpoint, live hardware/link blocker, or explicit pause.
- Select a next task only if it is already objectively defined with complete
  dependencies and gates; otherwise leave planningNeeded=true for supervisor
  task creation.

## Non-Goals

- No code implementation beyond documentation and evidence for this checkpoint.
- No fake/kernel-backed shell ping command as feature progress.
- No socket API, user program, UDP/TCP, smoltcp adoption, live driver adapter,
  live packet I/O, Pi 5 hardware run, lab mutation, boot publication, network
  reachability claim, SSH, or phase transition.
- No revival of Phase 12.1 same-shaped link polling or hardware discriminator
  retries.

## Review

The accepted single-ping frontier is host-only QEMU/substitute plus source/unit
evidence over caller-owned buffers and fake/trait-level NetworkDevice behavior.
It covers one route-aware transaction: unresolved ARP pending, matching ARP
advancement to one ICMP echo transmit, in-flight recording, matching echo
reply completion, final idle status, caller-driven retry exhaustion, and
explicit timeout.

That evidence is enough to plan toward a real user boundary, but not enough to
expose a shell command. A shell-visible ping command would currently be a fake
kernel-backed command unless it is backed by accepted userspace, descriptor,
socket, and network-stack layers. The existing command loop can remain a
regression/control surface only.

## Strategy Classification

- packet queue/pump: fixed as the strongest next feature direction. The next
  useful boundary should turn the accepted single-ping transaction into a
  caller-driven network service/pump that can own one transaction, consume
  received frames, produce transmit attempts through NetworkDevice, expose
  status, and accept explicit retry/timeout advancement without autonomous
  timers.
- caller-driven timer integration: fixed as part of the same near-term pump
  direction, but still caller-driven. Autonomous scheduler timers remain
  deferred until a later explicit task.
- neighbor-discovery state expansion: deferred. The current accepted single
  pending request, ARP cache, next-hop identity, retry budget, and matching ARP
  advancement are sufficient for one transaction; multi-entry neighbor state is
  not the smallest user-boundary step.
- driver-adapter contract: deferred. It is the right path before live packet
  I/O, but it should attach after the stack has a stable pump/service contract
  to adapt to.
- socket/user-program boundary: deferred. Socket and user-program work must not
  precede the pump contract and should stay backed by existing descriptor,
  syscall, VFS, and userspace layers when planned.
- smoltcp adoption checkpoint: deferred. The current custom boundary is still
  small and useful for proving Talos responsibilities; adopting smoltcp before
  the pump and driver-adapter seams are stable would hide the next kernel-owned
  decisions.
- live hardware/link blocker: not-an-issue for this host-only checkpoint.
  Phase 12.1 link work remains paused at the accepted timeout/link-not-ready
  frontier and should not be revived by this task.
- explicit pause: not-an-issue. There is an objective next direction, but this
  worker may not create the next task.

## Findings

- fixed: Separated feature progress from regression/control surfaces. A fake
  shell ping command is rejected until it is backed by accepted userspace,
  descriptor, socket, and network-stack layers.
- fixed: Classified the next useful feature direction as a caller-driven
  single-transaction packet pump/service boundary over the accepted
  SinglePingTransaction and NetworkDevice contracts.
- deferred: Supervisor task creation is required for the next implementation
  slice because no later queued task already contains complete scope,
  dependencies, non-goals, acceptance criteria, validation gates, docs, and
  evidence requirements.
- deferred: Live driver adapters, socket/user-program boundaries, smoltcp,
  UDP/TCP, packet queues beyond the bounded pump, hardware packet I/O, SSH, and
  reachability remain future work.
- not-an-issue: Phase 12.1 link hardware remains paused; this checkpoint does
  not authorize same-shaped link polling or hardware discriminator retries.

## Evidence

- Accepted QEMU/substitute smoke closeout:
  tasks/2026-06-19-phase12-network-single-ping-transaction-qemu-smoke-closeout.md
- Accepted smoke transcript:
  tasks/evidence/2026-06-19-qemu-single-ping-transaction-smoke/qemu-single-ping-transaction-smoke.log
- Source/unit evidence: src/network.rs SinglePingTransaction,
  SinglePingTransactionStatus, retry_single_ping_transaction_arp_request,
  timeout_single_ping_transaction, and
  qemu_substitute_single_ping_transaction_smoke_covers_lifecycle_and_retry_timeout.
- Roadmap requirements: Milestone 12.3 IP Stack and Milestone 12.4 Socket
  Integration.
- static/source/task/evidence review: pass
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation before commit: git diff --cached --check

## Outcome

selected_next_task=null
planningNeeded=true

planningReason=Host-only single-ping evidence supports a caller-driven
single-transaction packet pump/service boundary as the next feature direction,
but no queued follow-up task already has complete objective scope,
dependencies, non-goals, acceptance criteria, validation gates, docs, and
evidence requirements. Supervisor planning is required to create exactly one
bounded implementation task before shell ping, sockets, live driver adapters,
smoltcp, hardware packet I/O, reachability, SSH, or phase transition.
