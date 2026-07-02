# Phase 12 SSH Live TCP Selected Normal Runtime Post BootInfo Continuation Reconciliation V67

Task id: phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67-20260702

Status: accepted after no-hardware source/static reconciliation.

Classification: selected-normal-runtime-post-bootinfo-discriminator-ready.

Evidence level: git status inspection, accepted v68 BootInfo lineage proof,
current source/helper review, non-published archive materialization/review,
capture helper dry-run, task-owned JSON evidence, docs build, and diff checks.
No hardware action, hardwareTestLock acquisition, lab publication, boot
snapshot mutation, Pi 5 power cycle, route-start proof, runtime-ready proof,
packet-I/O implementation, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed
command expansion, broad shell work, or phase transition was performed.

## Goal

Define the next smallest current-source discriminator after the accepted
selected normal-runtime BootInfo frontier and before later normal-runtime,
packet-I/O, or OpenSSH work.

## Scope Performed

- Promoted this queued no-hardware task after v68 accepted the selected
  normal-runtime BootInfo frontier by lineage and selected this exact task.
- Compared the accepted v66/v68 BootInfo proof with current source and helper
  state.
- Selected the existing target-init marker-loop path as the next feature-led
  discriminator because source order reaches it only after
  BootInfo::from_aarch64_x0(dtb_pa), BootInfoParsed early-phase output, and
  target::init(&boot_info).
- Materialized and reviewed a non-published selected target-init archive for a
  future serialized Pi 5 task.
- Recorded the future capture helper dry-run contract without publishing the
  archive or acquiring hardwareTestLock.

## Reconciliation

v68 proves the selected normal-runtime BootInfo frontier by tying accepted v64
rust_entry proof, the v65 BootInfo archive/source contract, and v66 selected
TFTP plus TALOS: boot info parsed evidence to source lineage. The next ordered
current-source fact after that frontier is target::init(&boot_info) returning:
src/main.rs calls target::init(&boot_info), writes the TargetInit early-phase
line, and for talos_boot_scenario
rpi5_ssh_service_smoltcp_target_init_marker_loop enters
run_ssh_service_smoltcp_target_init_marker_loop().

That loop emits TALOS: target init with claims-bootinfo-parsed=true and
negative claims for exceptions ready, kernel_main, route-start, runtime-ready,
packet-I/O, service success, ssh-ready, and phase transition. The existing
helper surface can build and review the selected archive, so no source edit was
required.

## Future Hardware Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-target-init-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-target-init-v67-boot-tree
    tar -czf target/tmp/selected-normal-runtime-target-init-v67.tar.gz \
      -C target/tmp/selected-normal-runtime-target-init-v67-boot-tree .

- Changed files: task record, task-owned evidence, docs, and supervisor state
  only. No source/helper code changes were required.
- Archive path: target/tmp/selected-normal-runtime-target-init-v67.tar.gz.
- Archive SHA-256:
  18270d2ca0bef45c72898beaa55971b48d748f3a87a767556074423821f17352.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  4513bd97689673f904a849b60aee0377d6ddcc813ad0d00a18e422b3cc52ef82.
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

A future hardware preflight must treat TALOS: target init as the required
marker and must preserve selected archive identity, selected TFTP service,
serial freshness, final pre-restore identity, and restore proof. Suggested
fail-closed classifications are
selected-normal-runtime-target-init-marker-retained,
blocked-selected-normal-runtime-target-init-marker-missing, or
selected-normal-runtime-target-init-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-after-bootinfo-preflight-v69-20260702.

planningNeeded: true.

Reason: the serialized v69 Pi 5 target-init preflight is objectively specified
by this contract, but no v69 task currently exists in taskQueue. The supervisor
must instantiate the exact bounded hardware task before the worker performs any
hardware action.

Route-start proof, runtime-ready proof, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility/service readiness, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, hardware action, and
phase transition remain blocked.

## Findings

- fixed: reconciled the accepted v68 BootInfo frontier with current
  target-init marker-loop source/helper state.
- fixed: materialized and reviewed a fresh non-published selected target-init
  archive with valid Image header fields, selected root/da591740 equality, and
  embedded target-init discriminator strings.
- fixed: recorded a dry-run hardware capture contract for the next serialized
  target-init preflight without publishing to the lab or acquiring hardware.
- not-an-issue: no code changes were needed because the existing target-init
  marker-loop scenario already implements the smallest post-BootInfo
  discriminator.
- deferred: a future serialized Pi 5 preflight must prove this exact archive
  reaches TALOS: target init before accepting target init or later milestones.
- removed: route-start/runtime-ready proof, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility/service
  readiness, ssh-ready=true, fake command expansion, broad shell work,
  hardware action, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67/validation/archive-review.stdout.txt.
- Target-init archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67/validation/target-init-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts,
marker names, helper arguments, classifications, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet payloads,
SSH/session/key material, boot artifact bytes, private data, stable
secret-derived identifiers, public-key blobs, signatures, fingerprints,
operator identities, or unnecessary hardware data. The archive and boot tree
are retained only under target/tmp and are not committed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 293].
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass through archive
  materialization.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific target-init archive review: pass.
- Capture helper --dry-run for the future marker family and required marker:
  pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-after-bootinfo-preflight-v69-20260702.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
