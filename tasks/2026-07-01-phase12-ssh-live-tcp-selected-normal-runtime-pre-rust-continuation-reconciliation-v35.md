# Phase 12 SSH Live TCP Selected Normal Runtime Pre-Rust Continuation Reconciliation V35

Task id: phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-pre-rust-discriminator-ready.

Evidence level: state inspection, accepted predecessor evidence inspection,
source/helper/archive review, source and helper implementation, non-published
archive materialization/review, dry-run capture contract inspection, shell
syntax validation, cargo fmt/build, task-owned JSON evidence, docs build, and
diff checks. No hardware action, hardwareTestLock acquisition, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Create the next smallest selected normal-runtime continuation discriminator
after v34: prove statically that a selected feature archive can loop at
TALOS: asm_pre_rust_entry after the assembly entry setup and before rust_entry.

## Scope Performed

- Continued the ready no-hardware v35 task after v34 closeout commit
  c150e4a72aef12090bc1a82325d57deed86fe016.
- Reviewed the selected normal-runtime startup path, assembly entry markers,
  CPACR/BSS/stack handoff, rust_entry branch point, target routing, linker/Image
  header constraints, boot-tree helper shape, and the v34 helper classification
  mismatch.
- Added a separate rpi5_ssh_service_smoltcp_pre_rust_marker_loop boot scenario
  that keeps the selected normal-runtime service cfg route while stopping in
  assembly before rust_entry.
- Added a task-specific boot-tree helper for the selected pre-rust archive
  shape.
- Fixed the capture helper classification mismatch so future summaries classify
  retained required-marker occurrences or fresh marker-family evidence as
  post-handoff-marker-visible.

## Reconciliation

v34 proved the selected 152,144-byte normal-runtime entry-loop archive can reach
TALOS: asm_start on Pi 5. It did not prove CPACR setup, BSS clear, stack setup,
rust_entry, BootInfo parsing, target init, exceptions, kernel_main, packet-I/O,
remote receipt, compatibility/service readiness, ssh-ready=true, or OpenSSH.

The bounded v35 discriminator keeps the selected normal-runtime feature route
and archive shape but moves the loop to TALOS: asm_pre_rust_entry after CPACR
enable, BSS clear, and stack setup. Static disassembly shows the
asm_pre_rust_entry loop at _start+0x90 through _start+0xac and the later
rust_entry branch at _start+0xb4, making rust_entry and later normal-runtime
progress the first missing facts for the successor hardware run.

## V36 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-pre-rust-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-pre-rust-v35-boot-tree
    tar -czf target/tmp/selected-normal-runtime-pre-rust-v35.tar.gz \
      -C target/tmp/selected-normal-runtime-pre-rust-v35-boot-tree .

- Source base before task changes:
  c150e4a72aef12090bc1a82325d57deed86fe016.
- Changed files:
  build.rs, src/arch/aarch64/boot.S,
  scripts/rpi5-ssh-service-smoltcp-pre-rust-marker-loop-boot-tree.sh,
  scripts/rpi5-capture-invariant-proof-bundle.sh, docs, and task evidence.
- Archive path: target/tmp/selected-normal-runtime-pre-rust-v35.tar.gz.
- Archive SHA-256:
  2e2f538a7453c6fbce6b05c0c053b282d5e24c8f2d798e4893a2607fc7e7a0b2.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  afd9f9550e2abbdcba80520eb7c3527f1f3a3c3b383a432e8fe98c2381f8c7c1.
- Selected kernel size: 152,144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.

The v36 preflight must treat TALOS: asm_pre_rust_entry as the required marker.
The loop intentionally does not reach rust_entry or service code; this is a
pre-rust continuation discriminator, not a compatibility/service readiness
claim. Fail-closed classifications are
selected-normal-runtime-pre-rust-marker-retained,
blocked-selected-normal-runtime-pre-rust-marker-missing, or
selected-normal-runtime-pre-rust-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, and phase transition remain blocked.

## Findings

- fixed: added a separate selected normal-runtime pre-rust loop discriminator
  rather than changing the existing runtime-ready path.
- fixed: preserved selected normal-runtime Rust cfg and fail-closed runtime
  marker strings while looping at TALOS: asm_pre_rust_entry before rust_entry.
- fixed: materialized and reviewed a non-published v35 archive with valid
  Image header fields, selected root/da591740 equality, and embedded expected
  marker/runtime strings.
- fixed: repaired the v34 capture helper classification mismatch for future
  hardware summaries that depend on marker-family freshness.
- not-an-issue: v34 proves selected normal-runtime assembly entry on Pi 5 but
  does not prove pre-rust setup completion or rust_entry for this archive.
- deferred: v36 must run serialized Pi 5 hardware evidence for this exact
  archive contract before any rust_entry/later continuation repair can be
  selected.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/validation/archive-review.stdout.txt.
- Runtime archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/validation/runtime-archive-review.stdout.txt.
- Capture helper dry run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/validation/capture-bundle-dry-run.json.
- V34 helper classification replay:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/validation/v34-helper-classification-replay.json.
- Archive/kernel metadata:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35/static/.

## Redaction Review

Task-owned evidence retains task ids, source/commit/path labels, hashes, byte
counts, marker names, classifications, validation outcomes, selected-tree
metadata, and local static reports. It does not retain raw serial text, raw
TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot
artifact bytes, private user data, stable secret-derived identifiers, or
unnecessary hardware data. The archive and boot tree remain under target/tmp
and are not committed.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before v35 promotion.
- sh -n on touched shell helpers: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization plus scripts/rpi5-archive-review.sh
  and scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh: pass.
- Capture bundle --dry-run contract for TALOS: asm_pre_rust_entry and marker
  family: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
