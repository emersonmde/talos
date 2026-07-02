# Phase 12 SSH Live TCP Selected Normal Runtime Exceptions After Target Init Reconciliation V70

Task id: phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70-20260702

Status: accepted after no-hardware source/static reconciliation.

Classification: selected-normal-runtime-exceptions-discriminator-ready.

Evidence level: git status inspection, accepted v69 target-init frontier
proof, current source/helper review, non-published archive
materialization/review, capture helper dry-run, task-owned JSON evidence, docs
build, and diff checks. No hardware action, hardwareTestLock acquisition, lab
publication, boot snapshot mutation, Pi 5 power cycle, serial capture, TFTP
capture, kernel_main proof, route-start proof, runtime-ready proof,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim,
service success claim, ssh-ready=true, fake/kernel-backed command expansion,
broad shell work, or phase transition was performed.

## Goal

Define the next smallest current-source discriminator after the accepted
selected normal-runtime target-init frontier and before kernel_main,
route-start, runtime-ready, packet-I/O, or OpenSSH work.

## Scope Performed

- Continued this in-progress no-hardware task after v69 accepted the selected
  normal-runtime target-init frontier and the supervisor queued this exact
  reconciliation.
- Compared the accepted v69 target-init proof with current source and helper
  state.
- Selected the existing exceptions-ready marker-loop path as the next
  feature-led discriminator because source order reaches it only after
  arch::aarch64::exceptions::init() returns after target init.
- Materialized and reviewed a non-published selected exceptions-ready archive
  for the queued serialized v71 Pi 5 task.
- Recorded the future capture helper dry-run contract without publishing the
  archive or acquiring hardwareTestLock.

## Reconciliation

v69 proves the selected normal-runtime target-init frontier by tying the v67
selected target-init archive/source contract and v69 selected TFTP plus
target-init marker evidence to source lineage. The next ordered current-source
fact after that frontier is arch::aarch64::exceptions::init() returning:
src/main.rs calls arch::aarch64::exceptions::init(), writes the ExceptionsReady
early-phase line, and for talos_boot_scenario
rpi5_ssh_service_smoltcp_exceptions_ready_marker_loop enters
run_ssh_service_smoltcp_exceptions_ready_marker_loop().

That loop emits TALOS: exceptions ready with claims-bootinfo-parsed=true,
claims-target-init=true, and negative claims for kernel_main, route-start,
runtime-ready, packet-I/O, service success, ssh-ready, and phase transition.
The existing helper surface can build and review the selected archive, so no
source edit was required.

## Future Hardware Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-exceptions-ready-v70-boot-tree
    tar -czf target/tmp/selected-normal-runtime-exceptions-ready-v70.tar.gz \
      -C target/tmp/selected-normal-runtime-exceptions-ready-v70-boot-tree .

- Changed files: task record, task-owned evidence, docs, and supervisor state
  only. No source/helper code changes were required.
- Archive path: target/tmp/selected-normal-runtime-exceptions-ready-v70.tar.gz.
- Archive SHA-256:
  18007965ceb10991766e01ab2cf4d6f468530eca97d1a8c3a016a39b0402396b.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  7a62150e4232fc8215a7c7ec8e502697bdabb3a9e6bcd62f640c75aba722e455.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: rust_entry.
- TALOS: boot info parsed.
- TALOS: target init.
- TALOS: exceptions ready.

A future hardware preflight must treat TALOS: exceptions ready
capture-nonce=runtime-marker-route-static as the required marker and must
preserve selected archive identity, selected TFTP service, serial freshness,
final pre-restore identity, and restore proof. Suggested fail-closed
classifications are selected-normal-runtime-exceptions-marker-retained,
blocked-selected-normal-runtime-exceptions-marker-missing, or
inconclusive-selected-normal-runtime-exceptions-preflight.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71-20260702.

planningNeeded: false.

Kernel_main proof, route-start proof, runtime-ready proof, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility/service readiness,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work,
hardware action, and phase transition remain blocked.

## Findings

- fixed: reconciled the accepted v69 target-init frontier with current
  exceptions-ready marker-loop source/helper state.
- fixed: materialized and reviewed a fresh non-published selected
  exceptions-ready archive with valid Image header fields, selected
  root/da591740 equality, and embedded exceptions-ready discriminator strings.
- fixed: recorded a dry-run hardware capture contract for the queued serialized
  v71 exceptions-ready preflight without publishing to the lab or acquiring
  hardware.
- not-an-issue: no code changes were needed because the existing
  exceptions-ready marker-loop scenario already implements the smallest
  post-target-init discriminator.
- deferred: a future serialized Pi 5 preflight must prove this exact archive
  reaches TALOS: exceptions ready before accepting exceptions or later
  milestones.
- removed: kernel_main proof, route-start/runtime-ready proof, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility/service
  readiness, ssh-ready=true, fake command expansion, broad shell work,
  hardware action, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70/validation/archive-review.stdout.txt.
- Exceptions-ready archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70/validation/exceptions-ready-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-reconciliation-v70/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts,
marker names, helper arguments, classifications, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH/session/key material, boot artifact bytes, private data, stable
secret-derived identifiers, public-key blobs, signatures, fingerprints,
operator identities, or unnecessary hardware data. The archive and boot tree
are retained only under target/tmp and are not committed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 297] with only this task-owned evidence directory untracked from the
  already-started task.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass through archive
  materialization.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific exceptions-ready archive review: pass.
- Capture helper --dry-run for the future marker family and required marker:
  pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
