# Phase 12 SSH Live TCP Selected Normal Runtime Route Start Continuation Reconciliation V49

Task id: phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-route-start-discriminator-ready.

Evidence level: static inspection, source implementation, fmt/typecheck/build,
non-published archive materialization/review, capture helper dry-run,
task-owned JSON evidence, docs build, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Prepare the next no-hardware route-start discriminator only after the accepted
v48 closeout proved the selected Pi 5 kernel_main frontier.

## Scope Performed

- Promoted this queued no-hardware reconciliation after v48 accepted
  selected-normal-runtime-kernel-main-frontier-proved and selected this task.
- Compared the accepted selected normal-runtime frontier against the runtime
  route-start boundary in src/boot/rpi5.rs and src/target/rpi5.rs.
- Added a selected normal-runtime route-start marker-loop scenario that emits
  TALOS: ssh-service-smoltcp-runtime-route-start
  capture-nonce=runtime-marker-route-static after the accepted kernel_main
  frontier and before runtime-ready, packet-I/O, service success, ssh-ready, or
  phase-transition claims.
- Materialized and reviewed a non-published v49 archive for the future
  serialized Pi 5 preflight.
- Recorded the future capture helper dry-run contract for that preflight.

## Terminal Classification

selected-normal-runtime-route-start-discriminator-ready.

The accepted selected normal-runtime chain now has a concrete no-hardware
successor contract after v48 kernel_main proof. The first required successor
marker is emitted at the runtime route-start boundary and explicitly says
selected-normal-runtime-route-start=true, claims-runtime-ready=false,
claims-packet-io=false, claims-service-success=false,
claims-ssh-ready=false, and claims-phase-transition=false.

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-route-start-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-route-start-v49-boot-tree
    tar -czf target/tmp/selected-normal-runtime-route-start-v49.tar.gz \
      -C target/tmp/selected-normal-runtime-route-start-v49-boot-tree .

- Changed files: build.rs, src/boot/rpi5.rs, src/target/rpi5.rs,
  scripts/rpi5-ssh-service-smoltcp-route-start-marker-loop-boot-tree.sh, and
  scripts/rpi5-ssh-service-smoltcp-route-start-marker-loop-archive-review.sh.
- Archive path: target/tmp/selected-normal-runtime-route-start-v49.tar.gz.
- Archive SHA-256:
  16a8c14c33430f09682b6cb5a725c75f1e392f2372671ed3fea4a8b39ac609a4.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  370cbe055e4836d9605a318704e9337e112f9cbdf57743addc7ec9b13ef28467.
- Selected kernel size: 152,640 bytes.
- Image header: text_offset=0, header_image_size=152640, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family for a future hardware preflight:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: kernel_main.
- TALOS: ssh-service-smoltcp-runtime-route-start.

A future hardware preflight must treat TALOS:
ssh-service-smoltcp-runtime-route-start
capture-nonce=runtime-marker-route-static as the required marker. The loop
intentionally stops before runtime-ready, packet-I/O, OpenSSH compatibility,
service readiness, ssh-ready=true, fake command expansion, broad shell work, or
phase transition.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50-20260701.

planningNeeded: true.

Reason: no explicit v50 route-start Pi 5 hardware preflight task currently
exists in taskQueue; the supervisor must instantiate the next bounded
serialized task before the worker performs hardware action.

## Findings

- fixed: added a selected normal-runtime route-start marker-loop boot scenario
  instead of reusing runtime-ready as the next proof boundary.
- fixed: placed the new marker loop at the runtime route-start boundary after
  the accepted kernel_main frontier and before runtime-ready, packet-I/O,
  service success, ssh-ready, or phase-transition paths.
- fixed: kept the downstream normal runtime route linked in the archive by
  matching the established non-diverging marker-loop pattern.
- fixed: materialized and reviewed a non-published v49 archive with valid Image
  header fields, selected root/da591740 equality, and embedded route-start
  discriminator strings.
- not-an-issue: downstream runtime-ready strings remain in the image because
  preserving the selected normal-runtime service shape is part of the contract;
  the route-start marker line itself withholds later milestone claims.
- deferred: a future serialized Pi 5 preflight must prove this exact archive
  reaches TALOS: ssh-service-smoltcp-runtime-route-start before accepting
  route-start or any later normal-runtime milestone.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49/validation/archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49/validation/capture-bundle-dry-run.json.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 267].
- sh -n:
  scripts/rpi5-ssh-service-smoltcp-route-start-marker-loop-boot-tree.sh and
  scripts/rpi5-ssh-service-smoltcp-route-start-marker-loop-archive-review.sh
  pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific route-start archive review: pass.
- Capture helper --dry-run for the future marker family and required marker:
  pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50-20260701.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
