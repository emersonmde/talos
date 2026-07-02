# Phase 12 Live TCP Pi 5 Selected-Kernel Entry Retention Repair V78

Task: phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-repair-v78-20260702
Status: accepted
Terminal classification: selected-kernel-entry-retention-repair-ready

## Summary

This no-hardware task repaired the selected runtime-ready route so the next Pi 5
preflight can distinguish a true failure before kernel_main from the prior
one-shot serial-retention gap. The selected route still executes the
source-bound RP1 provider report once; after that it continuously replays a
compact, fail-closed retention hierarchy: kernel_main retained, provider
route-entry retained, and the terminal ready/blocked/error runtime marker.
No lab publication, boot mutation, Pi 5 power action, packet I/O, OpenSSH
compatibility, service success, ssh_ready=true, or phase transition was
performed.

## V71 / V76 / V78 Comparison

| Field | V71 accepted frontier | V76/V77 blocked frontier | V78 repaired candidate |
| --- | --- | --- | --- |
| Archive | target/tmp/selected-normal-runtime-exceptions-ready-v70.tar.gz | target/talos-rpi5-rp1-provider-route-entry-v76-boot.tar.gz | target/talos-rpi5-selected-kernel-entry-retention-v78-boot.tar.gz |
| Archive SHA-256 | 18007965ceb10991766e01ab2cf4d6f468530eca97d1a8c3a016a39b0402396b | e2779312ef50ddb55573524c4079608c2d0ee0626cdb1e57e8e30b1c43269332 | 7149fcad9aa29159b3e68e0875c89b41930cd1c39ca382a1cd409593972ebcb6 |
| Selected kernel bytes | 152,880 | 155,096 | 156,040 |
| Selected kernel SHA-256 | 7a62150e4232fc8215a7c7ec8e502697bdabb3a9e6bcd62f640c75aba722e455 | 6fc026100f0ea9e5157997eec12e1b3cc12000fdae243067f4349c7f4abffc20 | ba899e55a5ebe6beeac441d74590985b6aa1be046f57d13324f2d9e953ea9650 |
| Header facts | text_offset=0, header_image_size=152880, flags=12 | text_offset=0, header_image_size=155096, flags=12 | text_offset=0, header_image_size=156040, flags=12 |
| Marker shape | retained exceptions-ready loop after target init | one-shot provider route-start and one-shot runtime marker after provider report | retained kernel_main, provider route-start, and terminal runtime marker after provider report |
| Pi 5 result | selected TFTP plus 881 exceptions-ready marker occurrences | selected TFTP and final selected identity, but no kernel_main/route/runtime markers | no hardware run in this task; selected for v79 |

The v77 first missing fact remains selected Pi 5 serial retention of kernel_main
after selected kernel TFTP service until v79 runs. V78 changes the selected
archive contract so a future serial window can move the frontier beyond
kernel_main if it retains the replayed kernel_main/provider route/runtime
hierarchy, or can decisively remain blocked before kernel_main if it does not.

## Source And Archive Changes

- src/target/rpi5.rs: the selected
  rpi5_ssh_service_smoltcp_runtime_ready route now prints the provider
  route-start marker once, computes
  live_tcp_runtime_marker_route_report_with_source_bound_rp1_provider() once,
  then continuously replays the retention hierarchy with all live packet I/O,
  reachability, remote receipt, compatibility, service success, ssh_ready, and
  phase-transition claims false.
- scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh: the review
  now fails closed unless the selected image contains the v78 retention
  contract, provider route-entry marker vocabulary, terminal runtime marker
  vocabulary, nonce, and false readiness claims.
- Non-published archive:
  target/talos-rpi5-selected-kernel-entry-retention-v78-boot.tar.gz.
- Archive SHA-256:
  7149fcad9aa29159b3e68e0875c89b41930cd1c39ca382a1cd409593972ebcb6.
- Archive bytes: 302,817.
- Selected da591740/kernel_2712.img bytes: 156,040.
- Selected da591740/kernel_2712.img SHA-256:
  ba899e55a5ebe6beeac441d74590985b6aa1be046f57d13324f2d9e953ea9650.
- Capture nonce: entry-retention-v78.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected v79 marker hierarchy:

~~~text
TALOS: kernel_main capture-nonce=entry-retention-v78 selected-kernel-entry-retention=v78
TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=entry-retention-v78
TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=entry-retention-v78
TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=entry-retention-v78
~~~

## Findings

- fixed: The provider runtime route no longer depends on one-shot serial output
  for the kernel_main/route/runtime frontier; it retains the hierarchy after
  the provider report reaches a terminal local outcome.
- fixed: The archive review now proves the v78 retention contract is embedded
  in the selected image and still fails closed on packet I/O, reachability,
  remote receipt, OpenSSH compatibility, service success, ssh_ready, and phase
  transition claims.
- not-an-issue: V78 does not claim hardware proof; the first missing hardware
  fact remains unresolved until the queued v79 serialized Pi 5 preflight.
- deferred: Packet I/O, remote receipt, OpenSSH/generated-root retry,
  compatibility, service success, ssh_ready=true, fake command expansion, broad
  shell work, and phase transition remain blocked.

## Evidence

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit/substitute tests: cargo -Zjson-target-spec test --quiet passed with 898
  tests.
- shell syntax: sh -n on the touched runtime-ready archive-review script passed.
- image/archive inspection: scripts/rpi5-archive-review.sh passed for the v78
  archive.
- image/archive inspection:
  scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh
  target/talos-rpi5-selected-kernel-entry-retention-v78-boot.tar.gz
  entry-retention-v78 passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- JSON validation: jq empty passed on supervisor state and task-owned evidence.
- diff hygiene: git diff --check and git diff --cached --check passed.

## Disposition

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-preflight-v79-20260702

planningNeeded: false

Redaction review: retained evidence is limited to source-path notes, task ids,
archive/kernel digests and byte counts, marker vocabulary, validation
summaries, and restore target. No packet payloads, SSH key/session material,
private user data, raw hardware log bytes, or stable external identifiers are
retained.

Commit: recorded in talos-supervisor-state.json after final commit.
