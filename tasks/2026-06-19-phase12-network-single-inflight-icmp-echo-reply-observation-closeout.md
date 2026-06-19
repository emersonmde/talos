# Phase 12.3 Single-Inflight ICMP Echo Reply Observation Closeout

Task id: phase12-network-single-inflight-icmp-echo-reply-observation-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T18:09:00Z
Accepted: 2026-06-19T18:09:00Z

## Goal

Checkpoint the accepted host-only single-inflight ICMP echo reply observation
frontier before any user-visible ping command, socket surface, live driver
adapter, or broader packet queue.

## Scope

- Reconcile the accepted single-inflight ICMP echo reply observation
  implementation, tests, docs, task evidence, and commit.
- Record the accepted ping-like host-only completion boundary and rejected
  claims.
- Decide whether the host-only ping transaction frontier closeout is
  mechanically unblocked.

## Non-Goals

- No implementation work.
- No multi-entry packet queue, autonomous retry timer, scheduler wakeup,
  dynamic routing, DHCP, DNS, live driver adapter, live packet I/O, Pi 5
  hardware run, lab mutation, boot publication, smoltcp adoption, sockets,
  SSH, network reachability claim, or phase transition.
- No RP1/BCM54213PE hardware/link readiness change.
- No acceptance of live NetworkDevice implementation from fake/mock tests.
- No shell ping command or socket API.

## Reconciliation

The accepted core task committed as a7e4ca05e60d90119a47be797f43b06e4f8037d7.
It adds fixed-capacity SingleInflightIcmpEcho state, request recording, direct
reply observation, and a NetworkDevice receive poll helper. Source and tests
show one recorded in-flight request completes only when a received Ethernet,
IPv4, ICMP echo reply matches local endpoint addressing, remote source IPv4,
identifier, sequence number, payload bytes, and valid IPv4/ICMP checksums.

Nonmatching replies, malformed or unsupported frames, no-inflight, no-frame,
receive-buffer pressure, receive errors, duplicate in-flight records, and
payload-capacity pressure are deterministic and do not produce success.

## Findings

- fixed: The checkpoint records that host-only ICMP echo reply observation now
  completes one in-flight request with deterministic match criteria.
- fixed: The checkpoint preserves that the accepted boundary is source/test
  evidence over caller-owned receive storage and fake/trait-level NetworkDevice
  receive only.
- deferred: outbound transmit-to-in-flight integration, shell ping, packet
  queues, autonomous polling/timers, live driver adapters, sockets, SSH,
  smoltcp adoption, reachability, hardware, lab mutation, boot publication, and
  phase transition remain future work.
- removed: no source, docs, or task evidence were removed.
- not-an-issue: The accepted pending ARP-to-ICMP path remains a separate
  frontier; this closeout does not add hidden timers, packet queues, or live
  driver behavior.

## Validation

- static/source/task evidence review:
  - src/network.rs: InflightIcmpEchoRequest, SingleInflightIcmpEcho,
    InflightIcmpEchoResult, InflightIcmpEchoPollResult,
    observe_single_inflight_ipv4_icmp_echo_reply, and
    poll_single_inflight_ipv4_icmp_echo_reply.
  - src/network.rs tests: single_inflight_icmp_echo_reply_*.
  - tasks/2026-06-19-phase12-network-single-inflight-icmp-echo-reply-observation-core.md.
- jq evidence validation:
  - result: not applicable; this closeout created no task-owned JSON evidence.
- diff whitespace check:
  - git diff --check
  - result: pass.
- docs build:
  - /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check:
  - git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted Phase 12.3 boundary now includes host/testable single-inflight
ICMP echo reply observation: one recorded request can be completed by a
matching inbound Ethernet/IPv4/ICMP echo reply and cleared on success. This is
not a live ping path and does not imply network reachability.

## Rejected Claims

- No live packet I/O, driver adapter, interrupt loop, packet queue, autonomous
  polling/timer, timeout, shell ping command, socket, SSH, UDP/TCP, smoltcp
  adoption, network reachability, Pi 5 hardware proof, boot publication, lab
  mutation, or phase transition is accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.
- No outbound transmit helper automatically creates in-flight state yet.

## Evidence

- Core commit: a7e4ca05e60d90119a47be797f43b06e4f8037d7.
- Core task record:
  tasks/2026-06-19-phase12-network-single-inflight-icmp-echo-reply-observation-core.md.
- Source/test evidence: src/network.rs single-inflight ICMP echo request,
  observation, poll helper, and single_inflight_icmp_echo_reply_* tests.

## Next Action

selected_next_task=phase12-network-host-ping-transaction-frontier-closeout-20260619.
Promote that closeout on a later worker wake if dependencies remain satisfied
and git status is clean. Do not promote live driver, hardware, socket, shell
ping, SSH, smoltcp, reachability, lab, boot publication, or phase transition
directly from this closeout.
