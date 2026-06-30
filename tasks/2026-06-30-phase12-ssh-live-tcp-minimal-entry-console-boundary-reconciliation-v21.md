# Phase 12 SSH Live TCP Minimal-Entry Console Boundary Reconciliation V21

Task id: phase12-ssh-live-tcp-minimal-entry-console-boundary-reconciliation-v21-20260630

Status: accepted after commit.

Classification: minimal-entry-console-boundary-repair-ready.

Evidence level: no-hardware source/helper repair, static inspection of accepted
v17/v19/v20 evidence, non-published Pi 5 boot-tree/archive materialization,
image/header/string/root-selected equality review, task-owned JSON evidence,
docs build, shell syntax check, Rust format/build checks, and diff checks. No
hardware action, hardwareTestLock acquisition, lab publication, boot snapshot
mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.

## Goal

Reconcile why the repaired v17 minimal-entry polled-console marker remained
absent in v20 despite decisive selected-byte service, and select exactly one
bounded successor before any packet-I/O or OpenSSH work.

## Repair

The v20 selected archive was real and restored cleanly: selected
da591740/kernel_2712.img was served twice at 52,728 bytes, final pre-restore
identity held, restore succeeded, pre-power serial was drained empty, and
TALOS: minimal-entry-control-ready had zero occurrences.

The source review found the v17 repair still stopped at the first
boot::rpi5::kernel_main entry before report_boot_identity exercised the normal
firmware_console/println! path. That made the minimal-entry route depend on a
runtime-console call at a boundary that the accepted v19 production-timer proof
had not actually proved; v19 retained a downstream production-timer marker
after the normal boot reporting/runtime-console path was already active.

This task repairs that boundary without changing the selected fetch path or
making any service claim:

- src/boot/rpi5.rs now emits a pre-boot-identity boundary marker, continues
  through report_boot_identity, and only then stops in the minimal-entry
  control marker.
- src/target/rpi5.rs adds
  TALOS: minimal-entry-console-boundary-start through the direct early UART
  writer with source=kernel-main-entry-direct-uart and boundary-stage=pre-boot-identity.
- TALOS: minimal-entry-control-ready now identifies
  source=kernel-main-post-boot-identity-polled-console and
  boundary-stage=post-boot-identity under
  contract-id=phase12-ssh-live-tcp-minimal-entry-control-v2.
- scripts/rpi5-minimal-entry-control-archive-review.sh now fails closed unless
  both boundary markers, both source tokens, both boundary-stage tokens, the
  selected fetch path, and no-service/no-phase-transition guards are present.

This is qualitatively different from v20: a v22 run can distinguish no selected
kernel entry, pre-boot-identity direct-writer visibility only, and
post-boot-identity polled-console visibility, instead of repeating a single
same-boundary marker check.

## Static Materialization

Non-published materialization used:

- boot source: target/tmp/rpi5-observed-gpio-status-known-good-tree.
- capture nonce: phase12-console-boundary-v21-static.
- helper: scripts/rpi5-minimal-entry-control-boot-tree.sh.
- archive review helper: scripts/rpi5-minimal-entry-control-archive-review.sh.
- selected path: da591740/kernel_2712.img.
- kernel byte count: 69,816.
- kernel SHA-256:
  22ed9e1b6f0c04a28a662c55ddb48769505001c53caab40a04cbea40fa397cb7.
- non-published archive SHA-256:
  3c71a4527ed7cbf690b64257fc575f2a901a25bc428124d30f8f596f26e68f8f.
- Image header: text_offset=0, header_image_size=69,816, flags=12.
- root kernel_2712.img and da591740/kernel_2712.img are byte-identical.

The generated boot tree and archive remain under target/tmp only as
non-published local materialization; retained task evidence records metadata,
tokens, and review output.

## Selected Successor

selected_next_task:
phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22-20260630.

planningNeeded: false.

The successor must use source commit from this accepted repair, archive
target/tmp/minimal-entry-console-boundary-v21-static.tar.gz or a freshly
materialized equivalent from the same source, selected da591740/kernel_2712.img
at 69,816 bytes with SHA-256
22ed9e1b6f0c04a28a662c55ddb48769505001c53caab40a04cbea40fa397cb7, expected
markers:

- TALOS: minimal-entry-console-boundary-start capture-nonce=phase12-console-boundary-v21-static
- TALOS: minimal-entry-control-ready capture-nonce=phase12-console-boundary-v21-static

Marker source/console path:

- early marker: source=kernel-main-entry-direct-uart,
  boundary-stage=pre-boot-identity.
- ready marker: source=kernel-main-post-boot-identity-polled-console,
  boundary-stage=post-boot-identity.

Restore target:
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z or the current accepted
control snapshot named by the v22 task.

Fail-closed classifications for v22:
minimal-entry-console-boundary-marker-retained,
blocked-minimal-entry-console-boundary-marker-missing, or
minimal-entry-console-boundary-inconclusive-after-triage.

Redaction rules: retain task ids, path labels, hashes, byte counts, marker
names, classifications, validation outcomes, and selected successor metadata;
omit raw serial text unless needed for the v22 hardware evidence, raw TFTP
peer/log-line fields unless redacted by the hardware task, packet payloads,
SSH/session/key material, private user data, stable secret-derived identifiers,
and unnecessary hardware data.

## Findings

- fixed: v17's marker placement incorrectly treated a first-kernel_main
  runtime-console write as equivalent to the downstream production-timer
  console path that v19 proved; the marker now runs after report_boot_identity.
- fixed: added a direct early boundary marker so the next run can distinguish
  selected kernel entry from post-boot-identity console readiness.
- fixed: archive review now rejects stale v1/single-marker artifacts.
- removed: stale acceptance logic that selected-byte service alone proves the
  minimal-entry marker visibility boundary.
- removed: the old runtime-marker post-minimal-entry v21 preflight remains
  blocked because v20 did not retain the required marker.
- deferred: only the serialized v22 Pi 5 preflight can decide whether the
  repaired boundary markers are retained on hardware.
- not-an-issue: no Image header, linker placement, selected root mirror, or
  da591740 selected-fetch-path defect was found in static review.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-console-boundary-reconciliation-v21/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-console-boundary-reconciliation-v21/classification.json.
- Static materialization:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-console-boundary-reconciliation-v21/static/.
- Validation output:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-minimal-entry-console-boundary-reconciliation-v21/validation/.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before promotion.
- jq empty on referenced and task-owned JSON evidence: pass.
- sh -n on scripts/rpi5-minimal-entry-control-archive-review.sh: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json:
  pass.
- Non-published archive materialization plus header/string/root-selected
  equality review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
