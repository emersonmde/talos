# Phase 12 SSH Live TCP Minimal-Entry Route Repair V17

Task id: phase12-ssh-live-tcp-minimal-entry-route-repair-v17-20260630

Status: accepted after commit.

Classification: minimal-entry-polled-console-repair-supervisor-preflight-required.

Evidence level: no-hardware source/helper repair, non-published Pi 5
boot-tree/archive materialization, image/token/static inspection, task-owned
JSON evidence, docs build, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Repair or precisely classify the minimal-entry route after the v19 selected
current-tree handoff proof showed that selected bytes can reach a downstream
production-timer PASS marker.

## Repair

The v15 minimal-entry preflight served the repaired selected image twice and
held final pre-restore identity, but it expected decisive progress from
assembly/early UART phase lines and a minimal-entry marker written with
write_early_static/write_uart10_byte_early_phase. The accepted v19 handoff
discriminator proved a different visibility fact: a normal selected current
tree retained rpi5-production-timer-preemption: PASS from the polled runtime
console path, while the direct-read window still did not retain kernel_main or
other early phase lines.

This task therefore repaired the minimal-entry marker route, not the acceptance
boundary:

- src/target/rpi5.rs now emits TALOS: minimal-entry-control-ready through
  crate::print!/println!, which uses the polled firmware_console runtime
  console path.
- The marker source token changed from source=kernel-main-entry-control to
  source=kernel-main-entry-control-polled-console.
- scripts/rpi5-minimal-entry-control-archive-review.sh now requires the
  polled-console source token so stale early-writer-only artifacts fail closed.

The selected-fetch path and no-service-claim guards remain unchanged:
da591740/kernel_2712.img, live-tcp-route=false, packet-io=false,
openssh=false, ssh-ready=false, claims-service-success=false, and
claims-phase-transition=false.

## Static Materialization

Non-published static materialization used:

- boot source: target/tmp/rpi5-observed-gpio-status-known-good-tree.
- capture nonce: phase12-route-repair-v17-static.
- helper: scripts/rpi5-minimal-entry-control-boot-tree.sh.
- archive review helper: scripts/rpi5-minimal-entry-control-archive-review.sh.
- selected path: da591740/kernel_2712.img.
- kernel byte count: 52,728.
- kernel SHA-256:
  ccc95535706f1d896788800c8bba712cdeaac8bc6fedbbae313de06623040b33.
- non-published archive SHA-256:
  c5c48060056f7e945f8c333dec893d4ebe10885d2b0886419937ab0a9611389f.
- Image header: text_offset=0, header_image_size=52,728, flags=12.

The generated boot tree and archive bytes were removed after metadata and
token evidence were retained.

## Findings

- fixed: minimal-entry-control-ready now uses the same polled runtime console
  path that v19 proved can retain a selected-image downstream marker.
- fixed: minimal entry-control archive review now rejects artifacts that do
  not identify source=kernel-main-entry-control-polled-console.
- deferred: a serialized Pi 5 hardware preflight must prove whether the
  repaired polled-console minimal-entry marker is retained after selected-byte
  service.
- not-an-issue: assembly-entry provenance strings remain in the artifact as
  diagnostic context, but the repaired decisive marker no longer depends on
  the unpolled early writer.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness claim, fake command expansion, broad shell
  work, and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-route-repair-v17/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-route-repair-v17/classification.json.
- Static image evidence:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-route-repair-v17/static/.
- Validation output:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-route-repair-v17/validation/.

## Redaction Review

This task summary retains task ids, path labels, hashes, byte counts, marker
labels, classifications, validation command results, and selected successor
metadata. It omits raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH keys/session material, private user data, stable secret-derived
identifiers, and unnecessary hardware data. No hardware endpoint artifacts
were created by this no-hardware task.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: the no-hardware repair is complete, but no existing queued
hardware preflight has this repaired v17 predecessor and exact polled-console
marker contract. Supervisor planning must define the next serialized Pi 5
preflight before any hardware run, packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility/service readiness claim, broad shell work, or
phase transition.

Commit: recorded in talos-supervisor-state.json after final commit.
