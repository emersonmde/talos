# Phase 12 SSH Live TCP Selected Normal Runtime Runtime-Ready Continuation Reconciliation V51

Task id: phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-runtime-ready-discriminator-ready.

Evidence level: static inspection, existing source contract review, targeted
unit test, non-published archive materialization/review, capture helper
dry-run, task-owned JSON evidence, docs build, and diff checks. No hardware
action, lab publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Prepare the next no-hardware runtime-ready discriminator only after the
accepted v50 closeout proved the selected Pi 5 route-start frontier.

## Scope Performed

- Promoted this queued no-hardware reconciliation after v50 accepted
  selected-normal-runtime-route-start-frontier-proved and selected this task.
- Compared the accepted route-start frontier against the existing
  rpi5_ssh_service_smoltcp_runtime_ready route in src/target/rpi5.rs and the
  deterministic live TCP runtime marker contract in src/network.rs.
- Reused the existing runtime-ready route because it emits the first new
  required marker only after route-start and only when the deterministic
  descriptor/device runtime report is accepted while packet-I/O, reachability,
  remote receipt, compatibility, ssh-ready, service success, and phase
  transition remain false.
- Materialized and reviewed a non-published v51 archive for the future
  serialized Pi 5 preflight.
- Recorded the future capture helper dry-run contract for that preflight.

## Terminal Classification

selected-normal-runtime-runtime-ready-discriminator-ready.

The accepted selected normal-runtime route-start chain now has a concrete
no-hardware successor contract. The first required successor marker is emitted
by the selected normal-runtime route after TALOS:
ssh-service-smoltcp-runtime-route-start and is gated on
live_tcp_runtime_marker_route_report().marker_route_ready().

Required marker:

TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static

The runtime-ready line records
runtime-binding=accepted-deterministic-device-interface-delivery,
descriptor-facing-connection-delivered=true,
deterministic-device-interface-bound=true,
hardware-frame-provider-bound=false, equal nonzero driver packet rx/tx counts,
live-packet-io-accepted=false, live-reachability-accepted=false,
remote-receipt-accepted=false, compatibility-accepted=false, ssh-ready=false,
claims-service-success=false, and claims-phase-transition=false.

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-ready-static \
      scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-runtime-ready-v51-boot-tree
    tar -czf target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz \
      -C target/tmp/selected-normal-runtime-runtime-ready-v51-boot-tree .

- Changed files: none in source or shell helpers.
- Archive path: target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Archive SHA-256:
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- Selected kernel size: 152,144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12,
  magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family for a future hardware preflight:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: kernel_main.
- TALOS: ssh-service-smoltcp-runtime-route-start.
- TALOS: ssh-service-smoltcp-runtime-ready.

A future hardware preflight must treat TALOS:
ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static as the
required marker. The contract intentionally stops before packet-I/O,
OpenSSH compatibility, service readiness, ssh-ready=true, fake command
expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52-20260701.

planningNeeded: true.

Reason: the runtime-ready discriminator contract is ready, but no explicit v52
Pi 5 hardware preflight task currently exists in taskQueue; the supervisor must
instantiate the next bounded serialized task before the worker performs
hardware action.

## Findings

- fixed: reconciled the accepted v50 route-start frontier with the existing
  runtime-ready route and deterministic live TCP runtime marker report.
- fixed: materialized and reviewed a non-published v51 runtime-ready archive
  with valid Image header fields, selected root/da591740 equality, and embedded
  runtime-ready discriminator strings.
- not-an-issue: no source change was needed because the existing
  rpi5_ssh_service_smoltcp_runtime_ready route already emits route-start before
  runtime-ready and withholds packet-I/O, reachability, remote receipt,
  compatibility, ssh-ready, service success, and phase-transition claims.
- deferred: a future serialized Pi 5 preflight must prove this exact selected
  archive reaches TALOS: ssh-service-smoltcp-runtime-ready before accepting
  runtime-ready or any later normal-runtime milestone.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51/validation/archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51/validation/capture-bundle-dry-run.json.

## Redaction Review

This reconciliation retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 270].
- cargo test live_tcp_runtime_marker_route_report_reaches_fail_closed_runtime_path:
  initial invocation without -Zjson-target-spec failed with the expected target
  spec requirement; second invocation without the local QEMU path failed before
  test execution; final invocation with -Zjson-target-spec and the local QEMU
  path passed the no_std suite with 893 tests.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific runtime-ready archive review: pass.
- Capture helper --dry-run for the future marker family and required marker:
  pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52-20260701.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
