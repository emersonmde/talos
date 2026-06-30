# Phase 12 SSH Live TCP Current-Tree Production-Timer Control Contract

Task id: phase12-ssh-live-tcp-current-tree-production-timer-control-contract-20260630

Status: accepted after commit.

Classification: current-tree-production-timer-control-ready.

Evidence level: static accepted-evidence review, non-published Pi 5
boot-tree/archive materialization, source/symbol/marker-route review,
task-owned JSON evidence, docs build, and diff checks. No hardware,
lab publication, boot snapshot mutation, Pi 5 power action, packet I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, broad shell work, or phase transition was
performed.

## Goal

Define the smallest current-tree production-timer control contract before any
minimal-control v2 hardware, live TCP candidate preflight, packet-I/O
discriminator, or OpenSSH/generated-root retry.

## Scope Performed

- Promoted the ready no-hardware contract after supervisor planning resolved
  the accepted blocked-no-entry-artifact-repair frontier.
- Reviewed the accepted entry-artifact delta reconciliation, minimal-control
  blocker, selected known-good baseline, and current production-timer route.
- Materialized a fresh non-published current-tree production-timer Pi 5 boot
  tree/archive from the existing production-timer boot source, then removed the
  generated boot bytes after retaining manifests, hashes, header fields,
  symbol output, marker tokens, and archive-review output.
- Confirmed the selected da591740/kernel_2712.img mirror, accepted arm64 Image
  header fields, 104,136-byte current-tree production-timer kernel, expected
  rpi5-production-timer-preemption: PASS marker token, and startup/kernel_main
  to production-timer proof route metadata.
- Stopped before hardware, lab publication, live TCP candidate retry,
  packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility,
  service success, ssh-ready=true, broad shell work, or phase transition.

## Terminal Classification

current-tree-production-timer-control-ready.

The current tree can materialize the same selected-path production-timer
control shape as the accepted marker-positive known-good lineage. Static
archive review passes with root and da591740/kernel_2712.img identical,
kernel_size=104,136, text_offset=0, header_image_size=104,136, flags=12, and
magic=ARMd. The archive keeps the required selected fetch path and contains the
rpi5-production-timer-preemption: PASS token. Symbol review keeps _start,
rust_entry, kernel_main, boot::rpi5::kernel_main, and
run_production_timer_preemption_proof in the expected current-tree image.

selected_next_task:
phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator-20260630.

planningNeeded: false.

## Findings

- fixed: selected the explicit serialized Pi 5 current-tree production-timer
  entry baseline discriminator as the next task, rather than a blind
  minimal-control v2 or live TCP retry.
- not-an-issue: current-tree production-timer materialization preserves the
  selected da591740/kernel_2712.img mirror and accepted Image header shape.
- not-an-issue: the expected rpi5-production-timer-preemption: PASS marker
  token is present in the current-tree production-timer image.
- not-an-issue: source/symbol review preserves the startup/rust_entry/
  kernel_main route into the production-timer proof.
- deferred: the Pi 5 discriminator, minimal-control v2, live TCP candidate
  preflight, packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, and phase
  transition remain deferred until their explicit queued dependencies are met.
- removed: generated non-published boot tree and archive bytes were removed
  after metadata retention.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-current-tree-production-timer-control-contract/evidence-map.json.
- Fresh static materialization metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-current-tree-production-timer-control-contract/materialized/.
- Static header/token/symbol review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-current-tree-production-timer-control-contract/static/.
- Archive-review validation:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-current-tree-production-timer-control-contract/validation/archive-review.stdout.txt.

## Redaction Review

Task-owned evidence records task ids, source commit, path labels,
tree/archive hashes, Image header fields, byte counts, marker token presence,
symbol names, validation commands/results, and selected successor metadata. It
does not retain packet payloads, SSH keys/session material, boot artifact
bytes, private user data, secret-derived identifiers, or raw hardware
serial/TFTP logs.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Static review of accepted entry-artifact delta reconciliation,
  minimal-control blocker, selected known-good baseline, and current-tree
  production-timer route source: pass.
- Non-published Pi 5 boot-tree/archive materialization and static review:
  pass for the current-tree production-timer control; boot bytes removed after
  metadata retention.
- rpi5 archive/helper validation: pass; selected da591740/kernel_2712.img
  visibility, Image header fields, byte/hash metadata, marker tokens, symbol
  route expectations, restore target, and fail-closed non-claims are retained.
- cargo fmt --all -- --check: not run; Rust source was not touched.
- cargo -Zjson-target-spec test --quiet: not run; Rust source or target
  routing was not touched.
- sh -n: not run; shell helpers/classifiers were not touched.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
