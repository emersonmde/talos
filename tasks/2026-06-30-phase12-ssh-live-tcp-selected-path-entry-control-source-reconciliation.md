# Phase 12 SSH Live TCP Selected-Path Entry-Control Source Reconciliation

Task id: phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation-20260630

Status: accepted after commit.

Classification: serial-capture-control-required.

Evidence level: static accepted-evidence review, non-published Pi 5
boot-tree/archive materialization, source/linker/startup/marker-route review,
known-good/control evidence comparison, task-owned JSON evidence, docs build,
and diff checks. No lab publication, boot snapshot mutation, hardwareTestLock
acquisition, Pi 5 power action, live TCP candidate retry, packet-I/O
discriminator, OpenSSH/generated-root retry, remote receipt, compatibility
claim, service success claim, ssh-ready=true, broad shell work, or phase
transition was performed.

## Goal

Reconcile the selected-path entry-control blocker after the minimal-control Pi 5
run preserved selected TFTP fetch identity but emitted no nonce-bearing
minimal-entry-control marker.

## Scope Performed

- Promoted the ready no-hardware reconciliation task only after the accepted
  minimal-control Pi 5 discriminator recorded blocked-selected-path-entry-control.
- Reviewed the accepted v9 capture-invariant reconciliation, selected-fetch
  no-entry source reconciliation, minimal-entry-control contract, minimal-control
  Pi 5 blocker, selected known-good/control baseline, known-good serial-window
  completeness result, and earlier kernel-entry serial beacon proof.
- Rebuilt a non-published rpi5_minimal_entry_control boot tree and archive from
  current source using the selected da591740/kernel_2712.img mirror path and a
  fresh static review nonce.
- Inspected Image header fields, selected/root kernel byte identity, entry
  symbols, normal startup/kernel_main route, minimal-control marker tokens, and
  helper assumptions.
- Stopped before hardware, lab publication, packet-I/O, live TCP candidate
  retry, OpenSSH/generated-root retry, remote receipt, compatibility, service
  success, ssh-ready=true, broad shell work, or phase transition.

## Terminal Classification

serial-capture-control-required.

No bounded source/archive/lab-identity defect was found in the minimal-control
path:

- non-published materialization still mirrors root kernel_2712.img under
  da591740/kernel_2712.img;
- archive review reports text_offset=0, header_image_size=52,856, flags=12,
  and a 52,856-byte kernel image;
- symbol inspection keeps _start and __kernel_start at 0x200000,
  run_minimal_entry_control_marker at 0x202d20, talos::kernel_main at
  0x202f9c, rust_entry at 0x202fa0, boot::rpi5::kernel_main at 0x2039bc, and
  __kernel_image_end at 0x20ce78;
- source review keeps the minimal marker after the existing boot::rpi5
  kernel_main early-phase marker and before BootInfo/reporting/runtime work;
- token review keeps the nonce-bearing minimal-control marker, selected fetch
  path, expected-previous-marker=kernel_main, and fail-closed non-claims.

The first missing fact is therefore not a static route/header/token defect. It
is that no retained same-control selected-path baseline proves the current
a0452458... selected control emits an expected Talos-side entry marker under the
same capture contract after the minimal-control blocker. The smallest
materially different next discriminator is the already queued known-good
selected-path entry baseline proof. That proof can distinguish a generic
selected-path serial/entry capture outage from a minimal-control/source-artifact
specific blocker without retrying the live TCP candidate.

The successor contract is exact: restore
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, expect tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
effective_kernel=kernel_2712.img, expect two selected
da591740/kernel_2712.img fetches at 104,136 bytes, and check for the accepted
known-good marker rpi5-production-timer-preemption: PASS. TALOS: kernel_main
remains metadata-only for the selected v10 control when the downstream PASS
marker appears. The fail-closed classifications remain exactly
known-good-entry-baseline-passes, blocked-known-good-entry-baseline,
blocked-control-identity, blocked-control-tftp-capture, blocked-restore, and
inconclusive-with-required-discriminator.

selected_next_task:
phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator-20260630.

planningNeeded: false.

## Findings

- not-an-issue: the minimal-control selected-path boot tree still mirrors root
  Pi 5 boot files under da591740/.
- not-an-issue: Image header fields, root/selected kernel byte identity,
  startup route, rust_entry, talos::kernel_main, and boot::rpi5::kernel_main
  remain present in the materialized archive.
- not-an-issue: the minimal-control marker source remains wired immediately
  after the existing kernel_main marker and before BootInfo/reporting/runtime
  work.
- not-an-issue: the accepted minimal-control Pi 5 blocker already proved
  selected TFTP fetch and final pre-restore identity, so a same-shaped source
  rebuild is not a hardware result.
- deferred: a known-good selected-path entry baseline hardware discriminator is
  required to decide whether the missing marker is a generic selected-path
  serial/entry boundary or minimal-control specific.
- deferred: live TCP candidate retry, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, compatibility, service success, ssh-ready=true, broad shell
  work, and phase transition remain blocked.
- removed: generated archive and boot tree remain untracked target/tmp
  artifacts and are not part of durable evidence.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/evidence-map.json.
- Source/route review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/static/source-route-review.txt.
- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/validation/archive-review.stdout.txt.
- Minimal-control archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/validation/minimal-entry-control-archive-review.stdout.txt.
- Kernel bytes/hash/header:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/static/kernel-bytes.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/static/kernel-sha256.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/static/image-header-words.txt.
- Token and symbol review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/static/token-review.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation/static/symbols.txt.

## Redaction Review

This no-hardware task retained no raw serial text, raw TFTP peer/log-line
fields, packet payloads, key material, session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data. Evidence is limited to task ids, hashes, byte counts, symbol addresses,
static token checks, validation command results, and fixed classification
strings.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Review accepted task records and task-owned JSON evidence for v9,
  minimal-control contract, and minimal-control discriminator: pass.
- Non-published minimal-control boot-tree/archive materialization and static
  selected-path/Image/header/token review: pass.
- Source/static review of Pi 5 linker/startup/rust_entry/kernel_main and
  minimal-control marker route: pass.
- Known-good/control evidence comparison from retained task records only: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source or
  target routing touched.
- Focused helper/script validation: pass; rpi5-archive-review.sh and
  rpi5-minimal-entry-control-archive-review.sh accepted the materialized
  archive.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- Redaction search: pass; task-owned evidence has no raw serial, raw TFTP
  peer/log-line fields, packet payloads, or key/session material. Broader docs
  hits are pre-existing SSH algorithm prose.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
