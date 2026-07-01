# Phase 12 SSH Live TCP Selected Normal Runtime Kernel Main Continuation Reconciliation V45

Task id: phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-kernel-main-discriminator-ready.

Evidence level: static inspection, source implementation, fmt/typecheck/build,
non-published archive materialization/review, capture helper dry-run,
task-owned JSON evidence, docs build, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the accepted v44 exceptions-ready frontier and define the next
selected normal-runtime kernel_main discriminator without shrinking acceptance
toward route-start, runtime-ready, packet-I/O, OpenSSH, or service-readiness
claims.

## Scope Performed

- Promoted this queued no-hardware reconciliation after v44 accepted
  selected-normal-runtime-exceptions-frontier-proved and selected this task.
- Compared the accepted v34/v36/v38/v40/v42/v43/v44 chain against the current
  kernel_main boundary in src/main.rs and src/boot/rpi5.rs.
- Added a selected normal-runtime kernel_main marker-loop scenario that emits
  TALOS: kernel_main capture-nonce=runtime-marker-route-static only after
  boot::rpi5::kernel_main starts and before boot identity, route-start,
  runtime-ready, packet-I/O, service success, ssh-ready, or phase-transition
  paths.
- Materialized and reviewed a non-published v45 archive for the future hardware
  discriminator.
- Recorded the future capture helper dry-run contract for a serialized Pi 5
  preflight.

## Terminal Classification

selected-normal-runtime-kernel-main-discriminator-ready.

The accepted selected normal-runtime chain now has a concrete no-hardware
successor contract after v44 exceptions-ready proof. v45 preserves the
downstream selected normal-runtime route in the artifact, but the new first
required successor marker is emitted before route-start and explicitly says
claims-route-start=false, claims-runtime-ready=false, claims-packet-io=false,
claims-service-success=false, claims-ssh-ready=false, and
claims-phase-transition=false.

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-kernel-main-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-kernel-main-v45-boot-tree
    tar -czf target/tmp/selected-normal-runtime-kernel-main-v45.tar.gz \
      -C target/tmp/selected-normal-runtime-kernel-main-v45-boot-tree .

- Changed files: build.rs, src/boot/rpi5.rs, src/target/rpi5.rs,
  scripts/rpi5-ssh-service-smoltcp-kernel-main-marker-loop-boot-tree.sh, and
  scripts/rpi5-ssh-service-smoltcp-kernel-main-marker-loop-archive-review.sh.
- Archive path: target/tmp/selected-normal-runtime-kernel-main-v45.tar.gz.
- Archive SHA-256:
  72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  96057d2f8970808011a308f7b3a92da6feb85097b44590947a1ac145f85c6be6.
- Selected kernel size: 152,896 bytes.
- Image header: text_offset=0, header_image_size=152896, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family for a future hardware preflight:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: rust_entry.
- TALOS: boot info parsed.
- TALOS: target init.
- TALOS: exceptions ready.
- TALOS: kernel_main.

A future hardware preflight must treat
TALOS: kernel_main capture-nonce=runtime-marker-route-static as the required
marker. The loop intentionally stops before route-start, runtime-ready,
packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake
command expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46-20260701.

planningNeeded: true.

Reason: no explicit v46 kernel_main Pi 5 hardware preflight task currently
exists in taskQueue; the supervisor must instantiate the next bounded serialized
task before the worker performs hardware action.

## Findings

- fixed: added a selected normal-runtime kernel_main marker-loop boot scenario
  instead of reusing route-start or runtime-ready as the next proof boundary.
- fixed: placed the new marker loop immediately after entering
  boot::rpi5::kernel_main, before boot identity, route-start, runtime-ready,
  packet-I/O, or service-readiness paths.
- fixed: kept the downstream runtime route linked in the archive by matching
  the existing non-diverging marker-loop pattern rather than making the
  kernel_main loop a statically divergent path.
- fixed: materialized and reviewed a non-published v45 archive with valid Image
  header fields, selected root/da591740 equality, and embedded kernel_main
  discriminator strings.
- not-an-issue: contiguous downstream runtime-route strings remain in the image
  because preserving the selected normal-runtime service shape is part of the
  contract; the kernel_main marker line itself withholds later milestone claims.
- deferred: a future serialized Pi 5 preflight must prove this exact archive
  reaches TALOS: kernel_main before accepting kernel_main or any later
  normal-runtime milestone.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45/validation/archive-review.stdout.txt.
- Kernel_main archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45/validation/kernel-main-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts,
marker names, helper arguments, classifications, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH/session/key material, boot artifact bytes, private data, or
stable secret-derived identifiers. The archive and boot tree are retained only
under target/tmp and are not committed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 262].
- sh -n:
  scripts/rpi5-ssh-service-smoltcp-kernel-main-marker-loop-boot-tree.sh and
  scripts/rpi5-ssh-service-smoltcp-kernel-main-marker-loop-archive-review.sh
  pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific kernel_main archive review: pass.
- Capture helper --dry-run for the future marker family and required marker:
  pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46-20260701.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
