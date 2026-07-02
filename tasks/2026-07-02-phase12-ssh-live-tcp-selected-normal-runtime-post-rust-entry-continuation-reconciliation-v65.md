# Phase 12 SSH Live TCP Selected Normal Runtime Post Rust Entry Continuation Reconciliation V65

Task id: phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65-20260702

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-post-rust-entry-discriminator-ready.

Evidence level: state inspection, accepted v64 rust_entry closeout evidence
inspection, current source/helper review, non-published archive
materialization/review, capture helper dry-run, shell syntax validation,
cargo build, task-owned JSON evidence, docs build, and diff checks. No
hardware action, hardwareTestLock acquisition, lab publication, boot snapshot
mutation, Pi 5 power cycle, target-init proof, exceptions proof, kernel_main
proof, route-start/runtime-ready proof, packet-I/O implementation,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Create the next smallest selected normal-runtime continuation discriminator
after v64: a source/static, non-published artifact contract for proving
BootInfo parsing after rust_entry and before target init or later
runtime/service work.

## Scope Performed

- Promoted this queued no-hardware task after the accepted v64 closeout proved
  the selected normal-runtime rust_entry frontier and selected this exact task.
- Compared the v64 hardware-proved rust_entry boundary with current
  src/main.rs rust_entry flow, build.rs boot-scenario routing, selected
  BootInfo marker-loop helpers, and capture helper marker-family handling.
- Reused the existing rpi5_ssh_service_smoltcp_bootinfo_marker_loop source and
  helpers because current source still provides the correct next feature-led
  discriminator: parse BootInfo, then loop on TALOS: boot info parsed before
  target init, exceptions, kernel_main, route-start, runtime-ready,
  packet-I/O, or service readiness.
- Materialized and reviewed a non-published v65 BootInfo archive for a future
  serialized Pi 5 task.
- Recorded the future capture helper dry-run contract.

## Reconciliation

v64 proves the selected normal-runtime archive class reaches TALOS: rust_entry
on Pi 5 with selected-byte TFTP service and restore proof. The v64 fresh serial
window retained TALOS: rust_entry 208 times and retained zero later marker
family occurrences, so the next missing fact is after rust_entry.

The next ordered source fact is BootInfo parsing: rust_entry must call
BootInfo::from_aarch64_x0(dtb_pa), emit the existing BootInfo early-phase line,
then enter the BootInfo marker loop before target::init(&boot_info). The
current source already contains that exact scenario and helper surface. This
task therefore produces a fresh current-source archive contract instead of
editing runtime code.

## Future Hardware Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-bootinfo-v65-boot-tree
    tar -czf target/tmp/selected-normal-runtime-bootinfo-v65.tar.gz \
      -C target/tmp/selected-normal-runtime-bootinfo-v65-boot-tree .

- Changed files: task record, task-owned evidence, docs, and supervisor state
  only. No source/helper code changes were required.
- Archive path: target/tmp/selected-normal-runtime-bootinfo-v65.tar.gz.
- Archive SHA-256:
  68a3e9356753c66b646477880f786fc10a01b021bd8758d19484f409df81ad9d.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  87bbaab6842cbd83c1dff548d81151af6f9ff5309236b7ba65481174560987a8.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: rust_entry.
- TALOS: boot info parsed.

A future hardware preflight must treat TALOS: boot info parsed as the required
marker. The loop intentionally stops before target init, exceptions,
kernel_main, route-start, runtime-ready, packet-I/O, OpenSSH compatibility, or
service readiness. Suggested fail-closed classifications are
selected-normal-runtime-bootinfo-marker-retained,
blocked-selected-normal-runtime-bootinfo-marker-missing, or
selected-normal-runtime-bootinfo-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66-20260702.

planningNeeded: true.

Reason: the serialized v66 Pi 5 BootInfo preflight is objectively specified by
this contract, but no v66 task currently exists in taskQueue. The supervisor
must instantiate the exact bounded hardware task before the worker performs any
hardware action.

Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility/service
readiness, ssh-ready=true, fake command expansion, broad shell work, hardware
action, and phase transition remain blocked.

## Findings

- fixed: reconciled the accepted v64 selected rust_entry frontier with current
  source and selected BootInfo helper state.
- fixed: materialized and reviewed a fresh non-published selected BootInfo
  archive with valid Image header fields, selected root/da591740 equality, and
  embedded BootInfo discriminator strings.
- fixed: recorded a dry-run hardware capture contract for the next serialized
  BootInfo preflight without publishing to the lab or acquiring hardware.
- not-an-issue: no code changes were needed because the existing BootInfo
  marker-loop scenario already implements the smallest post-rust-entry
  feature discriminator.
- not-an-issue: contiguous downstream runtime-route strings remain in the
  image because preserving the selected normal-runtime service shape is part
  of the contract; the BootInfo marker line itself withholds later milestone
  claims.
- deferred: a future serialized Pi 5 preflight must prove this exact archive
  reaches TALOS: boot info parsed before accepting BootInfo parsing or any
  later normal-runtime milestone.
- removed: target-init/exceptions/kernel_main proof, route-start/runtime-ready
  proof, packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65/validation/archive-review.stdout.txt.
- BootInfo archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65/validation/bootinfo-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65/validation/capture-bundle-dry-run.json.

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
  [ahead 289].
- sh -n: scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-boot-tree.sh,
  scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-archive-review.sh, and
  scripts/rpi5-capture-invariant-proof-bundle.sh pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass through archive
  materialization.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific BootInfo archive review: pass.
- Capture helper --dry-run for the future marker family and required marker:
  pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66-20260702.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
