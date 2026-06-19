# Phase 12.3 Pending-Aware ARP Reply Poll Closeout

Task id: phase12-network-pending-aware-arp-reply-poll-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T18:10:00Z
Accepted: 2026-06-19T18:10:00Z

## Goal

Close out the host-only pending-aware ARP reply poll slice and record the
accepted boundary before any echo-reply observation, live packet I/O, socket,
SSH, hardware, or phase-transition work starts.

## Scope

- Review the accepted pending-aware ARP reply poll source, tests, docs, task
  record, evidence, and git commit.
- Record whether matching ARP reply receipt now advances a single pending
  route-aware ICMP echo request to trait-level transmit.
- Preserve rejected claims for live packet I/O, driver adapters, sockets, SSH,
  smoltcp, hardware, lab mutation, boot publication, reachability, and phase
  transition.
- Select the next queued ICMP echo reply observation task only because the core
  implementation is accepted and committed.

## Non-Goals

- No implementation work in src/network.rs.
- No multi-entry packet queue, autonomous retry timer, scheduler wakeup,
  dynamic routing, DHCP, DNS, live driver adapter, live packet I/O, Pi 5
  hardware run, lab mutation, boot publication, smoltcp adoption, socket, SSH,
  network reachability claim, shell ping command, or phase transition.
- No RP1/BCM54213PE hardware or link-readiness change.

## Review

Reviewed:

- src/network.rs PendingIcmpEchoPollResult,
  poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request,
  learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request, and
  retained local dispatch/poll helpers.
- src/network.rs pending_arp_reply_poll_* tests and retained local ARP/ICMP
  request dispatch regression tests.
- tasks/2026-06-19-phase12-network-pending-aware-arp-reply-poll-core.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- Git commit 6aed60c46fbb627ee82ea7a9fafd63f6a7d9d3f4.

## Findings

- fixed: The closeout reconciles the accepted host/testable pending-aware ARP
  reply poll boundary with source, unit-test, docs, task, and commit records.
- fixed: A matching ARP reply receipt now advances one stored route-aware
  pending ICMP echo request to exactly one trait-level ICMP transmit and clears
  pending only after successful ICMP transmit.
- fixed: Docs now record the closeout boundary and selected next echo-reply
  observation task without accepting live networking or user-facing ping.
- deferred: ICMP echo reply completion tracking, packet queues, autonomous
  polling/timers, live driver adapters, sockets, shell ping, SSH, smoltcp
  adoption, reachability, hardware, lab mutation, boot publication, and phase
  transition remain future work.
- removed: no source APIs, tests, docs, dependencies, task records, or evidence
  were removed.
- not-an-issue: selecting the queued single-inflight ICMP echo reply
  observation core is mechanical because the poll core is accepted and
  committed and the follow-up task already has explicit dependencies,
  acceptance criteria, validation gates, docs, evidence, scope, and non-goals.

## Accepted Boundary

The accepted boundary remains host/testable and allocation-free. One caller
driven NetworkDevice receive can process a matching ARP reply for the stored
pending next-hop IPv4, learn that next-hop neighbor into a caller-provided
ArpCache, transmit exactly one Ethernet/IPv4/ICMP echo request through
NetworkDevice, and clear the pending request only after successful ICMP
transmit.

Gateway-routed pending requests still preserve final IPv4 destination and ARP
next-hop identity separately: the ARP reply matches and learns the gateway
next hop while the emitted IPv4 packet targets the final destination. No-frame,
no-pending, receive-buffer pressure, receive errors, nonmatching ARP,
malformed ARP, output-buffer pressure, and transmit errors have deterministic
outcomes and preserve pending state except on successful ICMP transmit.

## Rejected Claims

- No live driver adapter, live packet I/O, packet capture, ping behavior,
  network reachability, sockets, shell ping command, SSH, UDP/TCP, smoltcp
  adoption, DHCP, DNS, dynamic routing, autonomous timer, scheduler wakeup,
  packet queue, multi-entry neighbor-discovery queue, RP1 Ethernet readiness,
  DMA descriptor ownership, interrupt handling, Pi 5 hardware proof, boot
  publication, lab mutation, or phase transition is accepted.
- No live NetworkDevice implementation or hardware readiness claim is accepted
  from fake/mock tests.
- No ICMP echo reply observation or completion tracking is accepted by this
  closeout; that remains the selected next implementation task.

## Validation

- static/source/task evidence review:
  src/network.rs pending-aware poll source and tests; retained local dispatch
  tests; Phase 12 docs; roadmap; task record; and git commit
  6aed60c46fbb627ee82ea7a9fafd63f6a7d9d3f4 reviewed.
  - result: pass.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-pending-aware-arp-reply-poll-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Evidence

- src/network.rs pending-aware ARP reply poll implementation and
  pending_arp_reply_poll_* tests.
- src/network.rs retained local ARP reply and inbound ICMP echo
  reply-to-request dispatch/poll tests.
- tasks/2026-06-19-phase12-network-pending-aware-arp-reply-poll-core.md.
- tasks/evidence/2026-06-19-phase12-network-pending-aware-arp-reply-poll-closeout/classification.json.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Next Action

selected_next_task=phase12-network-single-inflight-icmp-echo-reply-observation-core-20260619.
Promote that task on a later worker wake if dependencies remain satisfied and
git status is clean. Do not promote packet queues, live driver transmit,
hardware, sockets, shell ping, SSH, smoltcp adoption, ping/network
reachability, or any phase transition directly from this closeout.
