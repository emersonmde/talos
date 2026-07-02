# Phase 12 SSH Live TCP Selected Normal Runtime Rust Entry After Pre-Rust Reconciliation V63

Task id: phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63-20260702

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-rust-entry-discriminator-ready.

Evidence level: accepted v62 selected pre-rust closeout inspection, accepted
v37/v38 rust_entry line inspection, current source/helper inspection,
non-published archive materialization/review, capture helper dry-run, shell
syntax validation, cargo fmt/build, task-owned JSON evidence, docs build, and
diff checks. No hardware action, hardwareTestLock acquisition, lab publication,
boot snapshot mutation, Pi 5 power cycle, BootInfo parsing proof, target-init
proof, exceptions proof, kernel_main proof, route-start proof, runtime-ready
proof, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake/kernel-backed command
expansion, broad shell work, or phase transition was performed.

## Goal

After v62 proved selected TALOS: asm_pre_rust_entry on Pi 5, refresh the next
smallest current-source discriminator for TALOS: rust_entry before BootInfo,
target init, exceptions, kernel_main, route-start, runtime-ready, packet-I/O,
or OpenSSH work.

## Scope Performed

- Promoted this queued no-hardware task after the accepted v62 closeout
  selected it and verified planningNeeded=false, supervisorIntervention.active
  false, and hardwareTestLock unlocked/restored.
- Compared the accepted v62 selected pre-rust proof with the accepted v37/v38
  rust_entry continuation line and current source/helper state.
- Re-materialized the selected normal-runtime rust_entry marker-loop archive
  from current source without publishing it to the lab.
- Re-ran the generic and rust_entry-specific archive reviews and the capture
  helper dry-run for the next serialized Pi 5 proof.

## Reconciliation

v62 proves the selected normal-runtime candidate reached TALOS:
asm_pre_rust_entry 535 times on Pi 5 after selected post-power identity,
selected same-window TFTP service, selected final pre-restore identity, and
restore proof. The same v62 window did not retain TALOS: rust_entry,
route-start, runtime-blocked, or runtime-ready markers, so the next feature
boundary remains entering Rust.

The earlier v37/v38 line already added and hardware-proved the narrow
rust_entry marker-loop shape. Current source still exposes the same
rpi5_ssh_service_smoltcp_rust_entry_marker_loop boot scenario and review
helper. The refreshed v63 artifact keeps the selected normal-runtime service
route linked while looping immediately after rust_entry begins on TALOS:
rust_entry. It explicitly withholds BootInfo parsing, target init, exceptions,
kernel_main, route-start, runtime-ready, packet-I/O, service success,
ssh-ready, and phase-transition claims.

## V64 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-rust-entry-v63-boot-tree
    tar -czf target/tmp/selected-normal-runtime-rust-entry-v63.tar.gz \
      -C target/tmp/selected-normal-runtime-rust-entry-v63-boot-tree .

- Source commit used for materialization:
  ca15cbd2c36619813ff70517c1e99c6c7d018bbd.
- Archive path: target/tmp/selected-normal-runtime-rust-entry-v63.tar.gz.
- Archive SHA-256:
  7211853ae0fe6008b10b340725799503ff3ff9be46518428d2e5d3fdbf4e641f.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  347679f5797d2c99d61a56d5b250ee0245a0f19e9ac5f927491c4b9a019709c6.
- Selected kernel size: 152,816 bytes.
- Image header: text_offset=0, header_image_size=152816, flags=12,
  magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: rust_entry.
- TALOS: ssh-service-smoltcp-runtime-route-start
  capture-nonce=runtime-marker-route-static.
- TALOS: ssh-service-smoltcp-runtime-blocked
  capture-nonce=runtime-marker-route-static.
- TALOS: ssh-service-smoltcp-runtime-ready
  capture-nonce=runtime-marker-route-static.

The next serialized Pi 5 preflight must treat TALOS: rust_entry as the
required marker. Fail-closed classifications are
selected-normal-runtime-rust-entry-marker-retained,
blocked-selected-normal-runtime-rust-entry-marker-missing, or
selected-normal-runtime-rust-entry-inconclusive-after-triage.

Objectively specified next task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64-20260702.

planningNeeded: true, because the worker is not allowed to create taskQueue
entries and no v64 hardware preflight is currently queued.

Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility/service
readiness, ssh-ready=true, fake command expansion, broad shell work, hardware
action outside a supervisor-queued serialized preflight, and phase transition
remain blocked.

## Findings

- fixed: refreshed the current-source selected rust_entry marker-loop archive
  after the newer v62 selected pre-rust proof.
- fixed: preserved the exact boundary: TALOS: rust_entry is the next required
  proof after TALOS: asm_pre_rust_entry, before BootInfo or later runtime
  milestones.
- fixed: archive review proves valid Image header fields, selected
  root/da591740 equality, required marker strings, and negative claim tokens
  for later milestones.
- fixed: capture helper dry-run records the v64 candidate contract with no
  hardware actions and fail-closed classifications.
- not-an-issue: no code change was required; the current source already
  contains the narrow rust_entry loop and review helper from the accepted v37
  line.
- deferred: serialized Pi 5 evidence for the refreshed v63 archive must be
  planned as a separate hardware task before accepting rust_entry for this
  branch.
- removed: BootInfo parsing, target init, exceptions, kernel_main, route-start,
  runtime-ready, packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate worker-selected
  successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63/validation/archive-review.stdout.txt.
- Rust-entry archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63/validation/rust-entry-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts,
marker names, helper arguments, classifications, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH/session/key material, boot artifact bytes, private data, public
keys, signatures, fingerprints, operator identities, or stable secret-derived
identifiers. The archive and boot tree are retained only under target/tmp and
are not committed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 286].
- sh -n:
  scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-boot-tree.sh,
  scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-archive-review.sh,
  and scripts/rpi5-capture-invariant-proof-bundle.sh pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific rust_entry archive review: pass.
- Capture helper --dry-run for the v64 marker family and required marker:
  pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64-20260702.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
