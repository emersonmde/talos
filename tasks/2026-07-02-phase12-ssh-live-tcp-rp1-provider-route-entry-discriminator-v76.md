# Phase 12 Live TCP RP1 Provider Route-Entry Discriminator V76

Task: phase12-ssh-live-tcp-rp1-provider-route-entry-discriminator-v76-20260702
Status: accepted
Terminal classification: provider-route-entry-source-repaired

## Summary

This task repaired the source-visible route-entry discriminator after the v75
Pi 5 run proved selected kernel TFTP service but retained no Talos route-start,
runtime-blocked, or runtime-ready markers. The selected runtime route now emits
a provider-route-entry route-start marker before the provider-bound runtime
report and keeps blocked/error marker outputs explicitly fail-closed for packet
I/O, reachability, remote receipt, OpenSSH compatibility, service success,
ssh_ready, and phase transition. No lab publication, boot mutation, Pi 5 power
action, packet I/O, OpenSSH/generated-root retry, or phase transition was
performed.

## Source Path Map

- early assembly: TALOS: asm_start then TALOS: asm_pre_rust_entry from
  src/arch/aarch64/boot.S.
- early Rust: src/main.rs emits rust_entry, boot-info-parsed, target-init, and
  exceptions-ready phase lines for the runtime-ready scenario.
- kernel entry: src/boot/rpi5.rs emits TALOS: kernel_main before
  report_boot_identity.
- report boot identity: report_boot_identity prints the current boot identity
  and service metadata before scenario-specific runtime route work.
- route start: src/target/rpi5.rs now emits
  TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=<nonce>
  source=network-device-smoltcp-runtime provider-route-entry=source-bound-rp1
  claims-runtime-ready=false.
- provider runtime report: the route calls
  live_tcp_runtime_marker_route_report_with_source_bound_rp1_provider, which
  uses the accepted source-bound RP1 provider report in src/network.rs.
- terminal marker: success emits TALOS: ssh-service-smoltcp-runtime-ready;
  fail-closed provider/runtime/error paths emit
  TALOS: ssh-service-smoltcp-runtime-blocked.

Because v75 retained no early Talos, route-start, runtime-blocked, or
runtime-ready markers despite selected TFTP service, the next Pi 5 proof should
classify the deepest retained marker in this hierarchy. The refreshed marker
family distinguishes: before early Talos, before provider route-start, provider
runtime blocked/error, or provider runtime ready.

## Archive Identity

- non-published archive:
  target/talos-rpi5-rp1-provider-route-entry-v76-boot.tar.gz
- archive SHA-256:
  e2779312ef50ddb55573524c4079608c2d0ee0626cdb1e57e8e30b1c43269332
- archive bytes: 301590
- selected da591740/kernel_2712.img bytes: 155096
- selected da591740/kernel_2712.img SHA-256:
  6fc026100f0ea9e5157997eec12e1b3cc12000fdae243067f4349c7f4abffc20
- capture nonce: route-entry-v76
- restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z

Expected marker hierarchy for the hardware successor:

~~~text
TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=route-entry-v76
TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=route-entry-v76
TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=route-entry-v76
~~~

Required ready marker:

~~~text
TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=route-entry-v76 runtime-binding=accepted-deterministic-device-interface-delivery descriptor-facing-connection-delivered=true deterministic-device-interface-bound=true hardware-frame-provider-bound=true hardware-frame-provider-classification=rp1-ethernet-hardware-frame-provider-source-bound-local-only driver-packet-rx-frames=6 driver-packet-tx-frames=6 live-packet-io-accepted=false live-reachability-accepted=false remote-receipt-accepted=false compatibility-accepted=false ssh-ready=false claims-service-success=false claims-phase-transition=false
~~~

## Findings

- fixed: The route-start marker now identifies the source-bound RP1 provider
  route-entry discriminator and explicitly withholds runtime-ready claims.
- fixed: The provider runtime-blocked path now reports descriptor/device
  delivery booleans and preserves false live packet I/O, reachability, remote
  receipt, OpenSSH compatibility, service success, ssh_ready, and phase
  transition claims.
- fixed: The runtime error path now carries the same false safety claims.
- fixed: The runtime-ready archive review now checks the provider route-entry
  source marker, provider classifications, fail-closed error marker, and
  route/blocked/ready marker family.
- not-an-issue: The byte-emitted early Rust phase lines are not contiguous
  strings tokens; the task records them in the source-path map rather than
  treating archive string extraction as proof of those lines.
- deferred: v72 kernel_main, v60 runtime-ready, v53 packet-I/O, live packet I/O,
  OpenSSH/generated-root retry, fake command expansion, broad shell work, and
  phase transition remain blocked or deferred.

## Evidence

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit/substitute tests: cargo -Zjson-target-spec test --quiet passed with
  898 tests.
- image/archive inspection: scripts/rpi5-archive-review.sh passed for the
  non-published v76 archive.
- image/archive inspection:
  scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh
  target/talos-rpi5-rp1-provider-route-entry-v76-boot.tar.gz route-entry-v76
  passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- diff hygiene: git diff --check and git diff --cached --check passed.

## Disposition

selected_next_task:
phase12-ssh-live-tcp-pi5-rp1-provider-route-entry-preflight-v77-20260702

planningNeeded: false

Redaction review: retained evidence is limited to task ids, source-path notes,
archive/kernel digests and byte counts, marker names/counting contract,
validation transcripts, and restore target. No packet payloads, SSH
key/session material, private user data, raw hardware log bytes, or stable
external identifiers are retained.
