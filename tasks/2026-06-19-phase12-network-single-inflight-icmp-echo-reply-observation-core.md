# Phase 12.3 Single-Inflight ICMP Echo Reply Observation Core

Task id: phase12-network-single-inflight-icmp-echo-reply-observation-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T17:54:09Z
Accepted: 2026-06-19T18:00:00Z

## Goal

Implement the thinnest host-only boundary for observing one in-flight IPv4 ICMP
echo request and completing it only when a received echo reply matches the
stored request identity.

## Scope

- Add an allocation-free single in-flight ICMP echo request record.
- Add host-only helpers to record one in-flight request and observe one
  received Ethernet/IPv4/ICMP echo reply.
- Add a NetworkDevice receive poll helper for the in-flight reply observation
  boundary.
- Complete and clear the record only when source/destination IPv4, identifier,
  sequence number, and payload match with valid IPv4 and ICMP checksums.
- Cover match, mismatch, malformed/unsupported, no-inflight, no-frame, receive
  pressure, receive errors, duplicate in-flight, and payload pressure cases
  with unit tests.

## Non-Goals

- No live packet I/O, driver adapter, interrupt loop, packet queue, autonomous
  polling/timer, timeout behavior, shell ping command, socket, SSH, smoltcp
  adoption, Pi 5 hardware run, lab mutation, boot publication, reachability
  claim, or phase transition.
- No automatic wiring from outbound transmit helpers into in-flight tracking.
- No multi-entry in-flight table or retry/timeout state.

## Implementation

src/network.rs now includes:

- InflightIcmpEchoRequest, storing local endpoint, destination IPv4,
  identifier, sequence number, and fixed-capacity payload bytes.
- SingleInflightIcmpEcho, storing at most one in-flight request.
- InflightIcmpEchoResult and InflightIcmpEchoPollResult.
- record_single_inflight_ipv4_icmp_echo_request.
- observe_single_inflight_ipv4_icmp_echo_reply.
- poll_single_inflight_ipv4_icmp_echo_reply.

The observation helper parses a received Ethernet/IPv4/ICMP frame and accepts a
reply only when it is addressed to the local MAC, has source IPv4 equal to the
stored destination, destination IPv4 equal to the local endpoint, carries ICMP
echo-reply type/code with valid checksums, and has matching identifier,
sequence number, and payload bytes. Mismatches preserve the in-flight record.
Malformed or unsupported frames return explicit ReplyError values and also
preserve the record.

## Findings

- fixed: A single in-flight IPv4 ICMP echo request can now be recorded in
  fixed-capacity storage and completed by a matching received echo reply.
- fixed: Reply matching checks local endpoint addressing, remote source IPv4,
  identifier, sequence number, payload bytes, IPv4 checksum, and ICMP checksum.
- fixed: Source IPv4, destination IPv4, identifier, sequence, and payload
  mismatches deterministically preserve the in-flight record without success.
- fixed: No-inflight, no-frame, receive-buffer pressure, receive errors,
  unsupported EtherType, truncated frames, invalid ICMP checksum, echo-request
  type instead of echo-reply type, duplicate in-flight records, and payload
  capacity pressure have explicit test coverage.
- deferred: outbound transmit-to-in-flight integration, shell ping, packet
  queues, autonomous polling/timers, timeouts, live driver adapters, sockets,
  SSH, smoltcp adoption, reachability, hardware, lab mutation, boot
  publication, and phase transition remain future work.
- removed: no existing source API, task evidence, or docs were removed.
- not-an-issue: The accepted pending ARP-to-ICMP path remains separate; this
  task adds reply observation state and does not alter pending-request clearing
  semantics.

## Validation

- source inspection:
  src/network.rs InflightIcmpEchoRequest, SingleInflightIcmpEcho,
  InflightIcmpEchoResult, InflightIcmpEchoPollResult,
  observe_single_inflight_ipv4_icmp_echo_reply, and
  poll_single_inflight_ipv4_icmp_echo_reply.
- fmt/lint/typecheck:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; cargo fmt --all -- --check
  - result: initial fail before formatting; cargo fmt --all applied.
- fmt/lint/typecheck after formatting:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: pass, 620 tests.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass, search index warning only.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is host/testable single-inflight ICMP echo reply
observation over caller-owned receive storage and fake/trait-level
NetworkDevice receive. One recorded in-flight echo request can be completed
only by a matching inbound Ethernet/IPv4/ICMP echo reply. All nonmatching or
malformed cases preserve the record and do not claim success.

## Rejected Claims

- No live packet I/O, driver adapter, interrupt loop, packet queue, autonomous
  polling/timer, timeout, shell ping command, socket, SSH, UDP/TCP, smoltcp
  adoption, network reachability, Pi 5 hardware proof, boot publication, lab
  mutation, or phase transition is accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.
- No outbound transmit helper automatically creates in-flight state yet.

## Evidence

- src/network.rs:
  InflightIcmpEchoRequest, SingleInflightIcmpEcho,
  record_single_inflight_ipv4_icmp_echo_request,
  observe_single_inflight_ipv4_icmp_echo_reply, and
  poll_single_inflight_ipv4_icmp_echo_reply.
- src/network.rs tests:
  single_inflight_icmp_echo_reply_poll_matches_and_clears_request,
  single_inflight_icmp_echo_reply_observation_rejects_nonmatching_reply_fields,
  single_inflight_icmp_echo_reply_poll_distinguishes_empty_and_receive_errors,
  and
  single_inflight_icmp_echo_reply_observation_reports_malformed_and_payload_pressure.

## Next Action

selected_next_task=phase12-network-single-inflight-icmp-echo-reply-observation-closeout-20260619.
Promote that closeout on a later worker wake if dependencies remain satisfied
and git status is clean. Do not promote live driver, hardware, socket, shell
ping, SSH, smoltcp, reachability, lab, boot publication, or phase transition
directly from this implementation.
