# Phase 12.3 Single-Pending ARP Retry Closeout

Task id: phase12-network-single-pending-arp-retry-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T16:50:30Z
Accepted: 2026-06-19T16:52:00Z

## Goal

Close out the host-only single-pending ARP retry frontier and decide whether
the route-aware outbound host frontier is ready for a checkpoint.

## Scope

- Review the accepted retry source, tests, docs, task record, evidence, and
  git commit.
- Record the explicit retry semantics, pending-state behavior, and rejected
  live-networking/timer claims.
- Select the route-aware outbound frontier checkpoint only if the route-aware
  and retry slices are accepted with committed evidence.

## Non-Goals

- No implementation work in src/network.rs.
- No timer wheel, scheduler integration, multi-entry packet queue, live driver
  adapter, hardware run, lab mutation, boot publication, smoltcp adoption,
  sockets, SSH, ping/network reachability claim, or phase transition.

## Review

Reviewed:

- src/network.rs PendingIcmpEchoRequest, PendingIcmpEchoResult,
  SinglePendingIcmpEcho,
  transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget,
  and retry_single_pending_ipv4_icmp_echo_arp_request.
- src/network.rs single_pending_arp_retry_* tests and retained route-aware
  single-pending regression tests.
- tasks/2026-06-19-phase12-network-single-pending-arp-retry-core.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- Git commit a17b86e70bb596925e43fcb2ab2ec2eb28f2a797.

## Findings

- fixed: The closeout reconciles the accepted host/testable explicit ARP retry
  behavior with source, unit-test, docs, task, and evidence records.
- fixed: Docs now record the closeout boundary and select the route-aware
  outbound frontier checkpoint without expanding into driver, hardware, timer,
  socket, or reachability work.
- deferred: packet queues, autonomous timers, scheduler integration, live
  driver transmit, hardware packet I/O, sockets, SSH, smoltcp, reachability,
  lab mutation, boot publication, and phase transition remain future work.
- removed: no source APIs, tests, docs, dependencies, task records, or evidence
  were removed.
- not-an-issue: selecting the queued route-aware outbound frontier closeout is
  a same-milestone checkpoint after accepted route-aware pending and explicit
  retry work; it does not authorize implementation or phase transition.

## Accepted Boundary

The accepted boundary remains host/testable and allocation-free. A stored
single pending ICMP echo request may carry a deterministic ARP retry budget.
When a caller explicitly invokes
retry_single_pending_ipv4_icmp_echo_arp_request and budget remains, Talos
re-emits exactly one ARP request for the stored next-hop IPv4 through the
caller-provided output buffer and fake/trait-level NetworkDevice transmit.

Successful ARP retry transmit decrements the stored retry budget and keeps the
pending ICMP request available for later matching next-hop ARP resolution.
Budget exhaustion reports the stored final destination and next-hop IPv4 while
preserving pending state with zero remaining retries. No-pending, output-buffer
pressure, and transmit-error paths are deterministic and preserve or report
state according to the retry contract.

## Rejected Claims

- No autonomous retry timing, timer wheel, scheduler wakeup, multi-entry packet
  queue, neighbor-discovery queue, dynamic routing, DHCP, DNS, live driver
  adapter, live packet I/O, packet capture, ping behavior, network
  reachability, sockets, SSH, UDP/TCP, smoltcp adoption, RP1 driver adapter
  readiness, DMA descriptor ownership, interrupt handling, Pi 5 hardware proof,
  boot publication, lab mutation, or phase transition is accepted.
- No live NetworkDevice implementation or hardware readiness claim is accepted
  from fake/mock tests.

## Validation

- static/source/task evidence review:
  src/network.rs, retry tests, retained route-aware pending tests, core task
  record, Phase 12 docs, roadmap, and git commit
  a17b86e70bb596925e43fcb2ab2ec2eb28f2a797 reviewed.
  - result: pass.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-single-pending-arp-retry-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Evidence

- src/network.rs explicit ARP retry implementation and
  single_pending_arp_retry_* tests.
- src/network.rs retained route-aware pending tests.
- tasks/2026-06-19-phase12-network-single-pending-arp-retry-core.md.
- tasks/evidence/2026-06-19-phase12-network-single-pending-arp-retry-closeout/classification.json.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Next Action

selected_next_task=phase12-network-phase12-3-route-aware-outbound-frontier-closeout-20260619.
Promote that checkpoint on a later worker wake if dependencies remain
satisfied and git status is clean. Do not promote packet queues, live driver
transmit, hardware, sockets, SSH, smoltcp adoption, ping/network reachability,
lab mutation, boot publication, or any phase transition directly from this
closeout.
