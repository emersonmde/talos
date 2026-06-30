# Phase 12 SSH Live TCP Entry Artifact Delta Reconciliation

Task id: phase12-ssh-live-tcp-entry-artifact-delta-reconciliation-20260630

Status: accepted after commit.

Classification: blocked-no-entry-artifact-repair.

Evidence level: static accepted-evidence review, non-published Pi 5
boot-tree/archive materialization, source/startup/marker-route comparison,
task-owned JSON evidence, docs build, and diff checks. No hardware,
lab publication, boot snapshot mutation, Pi 5 power action, packet I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, broad shell work, or phase transition was
performed.

## Goal

Reconcile the failed selected-fetch candidate and minimal entry-control
artifacts against the passing selected known-good control after the known-good
baseline proved that the selected-path capture contract can observe Talos-side
output.

## Scope Performed

- Promoted the ready no-hardware reconciliation task after the selected-path
  known-good entry baseline accepted known-good-entry-baseline-passes.
- Reviewed the accepted v8 candidate no-entry blocker, v9 entry-marker blocker,
  minimal entry-control contract, minimal entry-control Pi 5 blocker,
  selected-path entry-control reconciliation, and selected known-good entry
  baseline.
- Materialized fresh non-published current-tree minimal-control and
  runtime-marker-route boot trees/archives for static review only, then removed
  generated boot bytes after retaining manifests, hashes, header fields, token
  reviews, and archive-review output.
- Compared selected/root kernel identity, Image headers, selected
  da591740/kernel_2712.img mirror path, startup/kernel_main route ownership,
  minimal-control marker source, runtime-marker route source, and helper
  boundaries.
- Stopped before hardware, lab publication, packet-I/O, live TCP candidate
  retry, OpenSSH/generated-root retry, remote receipt, compatibility, service
  success, ssh-ready=true, broad shell work, or phase transition.

## Terminal Classification

blocked-no-entry-artifact-repair.

No bounded current-tree source, script, archive, or docs repair was found:

- the current minimal-control static archive review passes, keeps root and
  da591740/kernel_2712.img identical, and records a valid arm64 Image header
  with text_offset=0, header_image_size=52,840, flags=12, and magic=ARMd;
- the current runtime-marker-route static archive review passes, keeps root and
  da591740/kernel_2712.img identical, and records a valid arm64 Image header
  with text_offset=0, header_image_size=152,152, flags=12, and magic=ARMd;
- both reviewed archives retain the expected selected-fetch path and marker
  tokens for their contracts;
- source inspection keeps the minimal-control marker immediately after
  boot::rpi5::kernel_main emits the existing KernelMain early-phase line and
  before BootInfo/reporting/runtime work;
- source inspection keeps the runtime-marker route after boot identity
  reporting, with route-start and runtime-ready marker paths still compiled for
  rpi5_ssh_service_smoltcp_runtime_ready.

The minimal entry-control diagnostic is not quarantined as invalid: it is still
the thinnest current-tree selected-path check for the normal
startup/rust_entry/kernel_main route before live TCP work. The accepted v9 and
minimal-control hardware blockers agree on the same first missing fact:
selected current-tree kernel bytes are served, but no retained post-power
serial window shows TALOS: kernel_main, the minimal-control marker, runtime
route-start, or runtime-ready. The accepted known-good selected baseline proves
the selected-path capture contract can observe the older a0452458... control's
downstream rpi5-production-timer-preemption: PASS marker, but it does not prove
that current-tree candidate/minimal artifacts reach Rust-side entry.

selected_next_task: null.

planningNeeded: true.

planningReason: The no-hardware source/artifact comparison found no bounded
repair and did not quarantine the minimal diagnostic. The first missing fact
remains current-tree selected-fetch/no-entry: selected minimal and runtime
candidate bytes are served, while the selected known-good control emits a
Talos-side marker under the same capture path. Supervisor planning is required
before any minimal-control v2 hardware proof, live TCP candidate preflight,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility,
service success, ssh-ready=true, broad shell work, or phase transition.

## Findings

- not-an-issue: selected da591740/kernel_2712.img mirroring is present for both
  freshly materialized minimal-control and runtime-marker-route boot trees.
- not-an-issue: Image header fields and selected/root kernel identity match the
  accepted Pi 5 Image contract shape; no header/file-size repair was found.
- not-an-issue: minimal-control marker source remains on the normal Pi 5
  startup/rust_entry/kernel_main route and is not quarantined as
  non-representative.
- not-an-issue: runtime-marker-route source and archive tokens remain present;
  the hardware blocker is earlier than live TCP runtime readiness.
- blocked: no bounded source/artifact/script/docs repair explains why current
  selected minimal/runtime artifacts do not reach Rust-side entry after proven
  selected TFTP fetch.
- deferred: minimal-control v2 hardware, candidate preflight, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, and phase transition remain deferred until
  supervisor planning selects a new discriminator.
- removed: generated non-published boot trees and archives were removed after
  static review; durable evidence keeps only metadata, manifests, hashes, and
  review output.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-entry-artifact-delta-reconciliation/evidence-map.json.
- Fresh static materialization metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-entry-artifact-delta-reconciliation/materialized/.
- Static header/token review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-entry-artifact-delta-reconciliation/static/.
- Archive-review validation:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-entry-artifact-delta-reconciliation/validation/minimal-archive-review.stdout.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-entry-artifact-delta-reconciliation/validation/runtime-archive-review.stdout.txt.

## Redaction Review

Task-owned evidence records task ids, source commit, path labels, tree/archive
hashes, Image header fields, byte counts, marker token presence, validation
commands/results, and selected successor/blocker metadata. It does not retain
packet payloads, SSH keys/session material, boot artifact bytes, private user
data, secret-derived identifiers, or raw hardware serial/TFTP logs.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Static review of accepted v8 candidate, v9 entry-marker, minimal-control,
  selected-path reconciliation, and selected known-good baseline task
  records/evidence maps: pass.
- Non-published Pi 5 boot-tree/archive materialization and static review:
  pass for current-tree minimal-control and runtime-marker-route paths; boot
  bytes removed after metadata retention.
- rpi5 archive/helper validation: pass; selected da591740/kernel_2712.img
  visibility, header fields, byte/hash metadata, marker tokens, and
  symbol/route expectations are retained as static evidence.
- cargo fmt --all -- --check: not run; Rust source was not touched.
- cargo -Zjson-target-spec test --quiet: not run; Rust source or target
  routing was not touched.
- sh -n: not run; shell helpers/classifiers were not touched.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
