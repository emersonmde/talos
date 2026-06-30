# Phase 12 SSH Live TCP Selected-Image Handoff Contract Reconciliation V19

Task id: phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19-20260630

Status: accepted after commit.

Classification: selected-image-handoff-control-discriminator-ready.

Evidence level: accepted v17/v18 evidence inspection, static source/helper
review, non-published current-tree production-timer selected-path control
materialization, Image header/symbol/token/archive inspection, task-owned JSON
evidence, docs build, and diff checks. No hardware action, lab publication,
boot snapshot mutation, Pi 5 power action, packet I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Goal

Reconcile the v18 selected-byte/no-_start-marker result from source, artifact,
and firmware-contract evidence, then select exactly one bounded next proof
without treating selected-byte service as entry proof.

## First-Principles Problem Statement

The Pi 5 firmware serves selected da591740/kernel_2712.img bytes and then
loads an arm64 Image whose header starts at _start. For v18, the selected
Image's first post-header path should have repeatedly emitted
TALOS: boot-contract-v18 directly from _start before CPACR setup, BSS clear,
stack setup, rust_entry, kernel_main, networking, packet I/O, OpenSSH, or
shell behavior. The observed hardware facts prove selected-byte service, final
pre-restore identity, and restore, but they do not prove the selected Image
executed its first post-header instruction.

The invariant that should hold is: for a selected-path Pi 5 Image, a fresh
hardware window with selected da591740/kernel_2712.img byte agreement should
either retain the expected Talos-side entry/progress marker, prove that this
specific Image shape does not reach observable entry under the firmware
contract, or fail closed with the exact capture/staging/restore fact that is
missing.

## Contradicting Evidence

- v18 selected the repeated compact boot-contract discriminator and proved two
  selected da591740/kernel_2712.img serves at 87,432 bytes with SHA-256
  fb501f7374888158c60f090b3cc0805f8fda97d98fd18e966c307310b5c00753, final
  pre-restore selected identity, and restore to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- The v18 fresh serial window retained firmware NETWORK output but no
  TALOS: boot-contract-v18, TALOS: kernel_main, or later Talos marker.
- Accepted current-tree production-timer selected-path controls from the same
  broader frontier reached rpi5-production-timer-preemption: PASS under the
  normal selected-path contract with 104,136-byte selected kernels, which means
  selected-path handoff is not globally broken.

## Unproven Assumptions

- Whether the compact pre-stack UART10 marker loop is a valid observable proxy
  for normal selected Image execution on this firmware path.
- Whether missing v18 output reflects firmware/Image handoff absence,
  shape-specific early UART/parking behavior, or a serial-window artifact that
  does not affect a full current-tree route to downstream PASS.
- Whether a source repair is justified before a fresh control discriminator
  re-establishes the selected-path handoff baseline adjacent to the v18 result.

## Reconciliation

Static review found no bounded source, linker, Image header, selected mirror,
config/kernel naming, or archive-helper defect that would explain the v18
result without new hardware evidence:

- linker-rpi5.ld still places _start at the selected Image base with
  KERNEL_IMAGE_TEXT_OFFSET=0.
- src/arch/aarch64/boot.S still emits a normal arm64 Image header and the v18
  compact marker loop is present in the materialized v17/v18 artifact.
- scripts/rpi5-boot-tree.sh still removes forced kernel_address and mirrors
  kernel_2712.img/kernel8.img under da591740/.
- Accepted v18 hardware evidence proves selected bytes, final identity, and
  restore; it does not prove entry.

No source repair is accepted in this task. The next bounded discriminator is a
control, not a same-shape v18 retry: rematerialize the current-tree
production-timer selected-path control from source commit
309b85265cfe90b1802368bf05f44bd8e9af6ad4, require root and
da591740/kernel_2712.img equality at 104,136 bytes with SHA-256
2343a009a14972d050ccf0fc706539163b6b5cb3ee3717b9cb6753f2ec7c2328, and expect
rpi5-production-timer-preemption: PASS. This control is qualitatively
different from v18 because it exercises the full current-tree startup,
rust_entry, kernel_main, and production-timer route instead of a compact
pre-stack UART marker loop.

## Terminal Classification

selected-image-handoff-control-discriminator-ready.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19-20260630.

planningNeeded: false.

The v19 hardware task must use the non-published materialization contract
recorded in this task:

- Source commit: 309b85265cfe90b1802368bf05f44bd8e9af6ad4.
- Archive materialization: scripts/rpi5-production-timer-preemption-boot-tree.sh
  target/rpi5-production-timer-preemption-boot-tree into a task-owned
  production-timer control archive.
- Selected fetch path: da591740/kernel_2712.img.
- Expected selected kernel: 104,136 bytes, SHA-256
  2343a009a14972d050ccf0fc706539163b6b5cb3ee3717b9cb6753f2ec7c2328.
- Expected earliest decisive marker:
  rpi5-production-timer-preemption: PASS. TALOS: kernel_main and earlier phase
  lines remain metadata-only when PASS is present.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Fail-closed classifications: selected-image-handoff-entry-reached if PASS
  appears with selected-byte/final-identity/restore proof;
  blocked-selected-image-handoff-after-v19-discriminator if selected bytes and
  restore/final identity are decisive but PASS is absent; inconclusive only for
  a named capture, staging, identity, or restore gap after the standard triage
  order.
- Redaction rules: retain task ids, source/commit/path labels, hashes, byte
  counts, marker labels, classification, and validation results; do not retain
  packet payloads, SSH/session/key material, unnecessary raw hardware fields,
  private user data, or boot artifact bytes outside task-owned run evidence.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, minimal-entry route repair, broad shell work,
and phase transition remain blocked. The existing
phase12-ssh-live-tcp-minimal-entry-route-repair-v17-20260630 remains blocked
because v18 did not prove handoff-entry-reached.

## Findings

- fixed: reconciled the v18 result as selected-byte service without entry
  proof and preserved the exact v18 facts.
- fixed: selected a bounded current-tree production-timer selected-path control
  discriminator rather than a v18 same-shape retry.
- not-an-issue: linker-rpi5.ld, the arm64 Image header shape,
  kernel_2712.img/kernel8.img naming, selected da591740 mirroring, and restore
  target remain consistent with accepted selected-path controls.
- not-an-issue: current-tree production-timer materialization preserves a valid
  Image header, root/selected kernel equality, and the expected PASS token.
- deferred: the hardware task must decide whether the full current-tree
  selected-path control still reaches PASS adjacent to the v18 blocker.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  service-readiness claims, minimal-entry route repair, broad shell work, and
  phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19/classification.json.
- Static control discriminator contract:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19/static/control-discriminator-contract.txt.
- Header, token, and symbol review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19/static/image-header.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19/static/token-review.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19/static/symbols.txt.
- Archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19/validation/production-timer-control-archive-review.stdout.txt.

## Redaction Review

Task-owned evidence records task ids, source commit labels, path labels,
hashes, byte counts, marker labels, source/symbol snippets, validation
command results, and selected successor metadata. It retains no raw hardware
serial text, raw TFTP peer/log-line fields, packet payloads, SSH/session/key
material, boot artifact bytes, private user data, stable secret-derived
identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned promotion.
- jq empty on referenced JSON evidence and new task-owned JSON: pass.
- sh -n on touched shell helpers: not run; no shell helpers were touched.
- cargo fmt --all -- --check: not run; no Rust source was touched.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json with
  TALOS_BOOT_SCENARIO=rpi5_production_timer_preemption_proof: pass via
  non-published production-timer control materialization.
- Non-published archive materialization plus header/symbol/token/root-selected
  equality review: pass; generated boot tree and archive bytes were removed
  after metadata retention.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
