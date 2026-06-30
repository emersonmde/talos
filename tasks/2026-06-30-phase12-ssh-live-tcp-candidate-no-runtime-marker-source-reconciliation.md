# Phase 12 SSH Live TCP Candidate No-Runtime-Marker Source Reconciliation

Task id: phase12-ssh-live-tcp-candidate-no-runtime-marker-source-reconciliation-20260630

Status: accepted after commit.

Classification: candidate-runtime-marker-route-repair-ready.

Evidence level: accepted v10 hardware evidence review, static source and
artifact inspection, non-published Pi 5 archive materialization, helper
classifier repair, focused shell fixture validation, docs build, and diff
checks. No hardware, lab publication, boot snapshot mutation, Pi 5 power
action, live TCP retry, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility claim, service success claim, ssh-ready=true, broad
shell work, or phase transition was performed.

## Goal

Reconcile the accepted v10 result where selected candidate bytes were served
but no nonce-bearing ssh-service-smoltcp route-start/runtime-ready marker
appeared, and implement only bounded source/artifact/helper repairs.

## Scope Performed

- Promoted this no-hardware reconciliation after the accepted v10 Pi 5
  candidate preflight selected it.
- Reviewed accepted v10 evidence, current-tree production-timer control
  evidence, runtime-marker source routing, boot scenario wiring, archive
  materialization, Image layout, symbol layout, marker tokens, and serial
  readiness helper semantics.
- Materialized a fresh non-published
  rpi5_ssh_service_smoltcp_runtime_ready candidate archive with capture nonce
  candidate-no-runtime-marker-reconciliation, retained metadata only, and
  removed generated boot bytes.
- Repaired the serial readiness helper so runtime-marker tasks can require the
  exact route-start and runtime-ready markers while making TALOS: kernel_main
  optional when the accepted marker contract says it is metadata-only.
- Stopped before hardware, lab publication, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, or phase transition.

## Terminal Classification

candidate-runtime-marker-route-repair-ready.

No bounded Rust source or archive-materialization defect was found: the fresh
non-published candidate archive keeps matching root/selected
da591740/kernel_2712.img, valid arm64 Image header fields, and the expected
route-start/runtime-ready tokens. Optimized symbol/layout inspection keeps
_start, rust_entry, boot::rpi5::kernel_main,
run_ssh_service_smoltcp_runtime_ready_route, and
live_tcp_runtime_marker_route_report in the Pi 5 image.

A bounded helper/classifier defect was found and fixed. The prior
scripts/rpi5-observe-runtime-readiness.sh contract always required
TALOS: kernel_main in addition to the configured success marker. Accepted
production-timer controls already established that TALOS: kernel_main can be
metadata-only absent when a downstream marker is present under the selected
capture contract. For the runtime-marker candidate, candidate-capture-ready
depends on the nonce-bearing route-start and runtime-ready markers, not on
kernel_main. The helper now supports
TALOS_READINESS_REQUIRED_MARKERS as a pipe-separated all-of marker list and
TALOS_READINESS_REQUIRE_KERNEL_MARKER=false for contracts where kernel_main is
metadata-only. The default remains kernel_main-required for existing callers.

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11-20260630.

planningNeeded: false.

## Findings

- not-an-issue: fresh non-published candidate materialization preserves the
  selected da591740/kernel_2712.img mirror, Image header fields, route-start
  token, runtime-ready token, nonce, and fail-closed non-claim tokens.
- not-an-issue: optimized symbol/layout inspection retains the normal
  _start -> rust_entry -> boot::rpi5::kernel_main path plus the runtime-marker
  route and network runtime report symbols.
- fixed: scripts/rpi5-observe-runtime-readiness.sh can now check an all-of
  marker contract with kernel_main optional when the task contract explicitly
  treats kernel_main as metadata-only.
- fixed: focused fixture validation proves route-start plus runtime-ready can
  pass without kernel_main only when
  TALOS_READINESS_REQUIRE_KERNEL_MARKER=false, while the default
  kernel-required policy still rejects the same text.
- deferred: whether the v10 serial absence was a runtime-route failure or a
  candidate-specific hardware/capture limitation remains a hardware question
  for the selected v11 preflight.
- removed: generated non-published boot tree and archive bytes were removed
  after retaining metadata and validation output.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-no-runtime-marker-source-reconciliation/evidence-map.json.
- Source/artifact inspection:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-no-runtime-marker-source-reconciliation/static/source-artifact-inspection.md.
- Candidate archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-no-runtime-marker-source-reconciliation/validation/candidate-archive-review.stdout.txt.
- Helper validation:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-no-runtime-marker-source-reconciliation/validation/runtime-readiness-helper-optional-kernel.stdout.json and
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-no-runtime-marker-source-reconciliation/validation/runtime-readiness-helper-kernel-required.stdout.json.

## Redaction Review

This no-hardware task retained no new raw serial text, raw TFTP peer/log-line
fields, packet payloads, key material, session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data. Evidence is limited to task ids, hashes, byte counts, symbol names,
static strings, validation command results, and fixed classification strings.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted tracked Talos changes before task-owned evidence creation.
- Fresh non-published candidate/archive materialization: pass; selected
  da591740/kernel_2712.img matched root and archive review passed.
- Static review of marker ordering/checker expectations against source and
  task contract: pass; helper defect found and fixed.
- cargo fmt --check: not run; Rust source was not touched.
- cargo test: not run; Rust source was not touched and this task only changed a
  shell helper plus docs/evidence.
- sh -n for touched shell scripts: pass.
- Focused helper fixture validation: pass.
- jq empty on touched JSON evidence/state files: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
