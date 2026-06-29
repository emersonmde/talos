# Phase 12 SSH Live TCP Runtime Marker Route Reconciliation

Task id: phase12-ssh-live-tcp-runtime-marker-route-reconciliation-20260629

Status: accepted after commit.

Classification: runtime-marker-route-ready.

Evidence level: static inspection, unit tests, fmt check, non-published Pi 5
boot archive/static marker review, docs build, JSON/diff checks, and commit.

## Goal

Repair or classify the missing nonce-bearing
`TALOS: ssh-service-smoltcp-runtime-ready` runtime marker route after the v6
blocked-candidate-kernel-not-starting result, without hardware action or lab
publication.

## Scope Performed

- Added the rpi5_ssh_service_smoltcp_runtime_ready boot scenario to build-time
  scenario selection, Pi 5 boot entry routing, and the target dead-code allow
  list.
- Added `network::live_tcp_runtime_marker_route_report()` so the marker route
  depends on descriptor-facing loopback delivery and the accepted deterministic
  smoltcp runtime binding instead of direct string emission.
- Added `target::rpi5::run_ssh_service_smoltcp_runtime_ready_route()` to emit
  a nonce-bearing route-start marker and either the fail-closed blocked marker
  or the ready marker with explicit runtime-binding, descriptor, frame-count,
  and claim flags.
- Added task-owned Pi 5 boot-tree and static archive review helpers for the
  runtime-marker route.
- Updated Phase 12 docs and roadmap status.
- Stopped before hardware action, boot publication, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, or phase transition.

## Findings

- fixed: v6 had a clean helper-owned hardware run but no selected Pi 5 source
  route that could emit the nonce-bearing runtime-ready marker.
- fixed: the new boot scenario is wired through `build.rs`, `src/main.rs`,
  `src/boot/rpi5.rs`, and `src/target/rpi5.rs`.
- fixed: readiness is gated on deterministic smoltcp runtime boundary evidence
  and fail-closed claim flags; `ssh-ready=true` is not claimed.
- fixed: archive review checks marker tokens, run nonce, runtime binding, and
  claim flags in the generated kernel image without publishing to the lab.
- not-an-issue: the v6 TFTP selected-byte and final pre-restore identity
  evidence already isolated the blocker to runtime marker routing, not capture
  ownership or restored-control contamination.
- deferred: hardware preflight, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, compatibility, service success, ssh-ready=true, broad shell
  work, and phase transition remain deferred to explicit successor tasks.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-runtime-marker-route-reconciliation/evidence-map.json.
- Static route inspection:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-runtime-marker-route-reconciliation/validation/static-route-rg.txt.
- Focused runtime route test:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-runtime-marker-route-reconciliation/validation/cargo-test-live-tcp-runtime-marker-route.stdout.txt.
- Non-published archive review:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-runtime-marker-route-reconciliation/validation/runtime-marker-archive-review.stdout.txt.
- Generated boot tree manifest:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-runtime-marker-route-reconciliation/validation/runtime-marker-boot-tree-manifest.txt.

## Validation

- git status --short --branch before edits: pass; task-owned edits and evidence
  only.
- Static source inspection with rg: pass; route spans build.rs, src/main.rs,
  src/boot/rpi5.rs, src/network.rs, src/target/rpi5.rs, and task-owned scripts.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet
  live_tcp_runtime_marker_route_report_reaches_fail_closed_runtime_path: pass
  through the custom runner; 893 tests passed.
- Non-published Pi 5 boot-tree/archive static marker review: pass; no lab
  publication and no hardware power action.
- jq empty on task-owned evidence-map.json: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --check: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v7-20260629.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
