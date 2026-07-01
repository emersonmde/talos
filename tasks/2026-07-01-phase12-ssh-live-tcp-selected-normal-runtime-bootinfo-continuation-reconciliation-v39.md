# Phase 12 SSH Live TCP Selected Normal Runtime BootInfo Continuation Reconciliation V39

Task id: phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-bootinfo-discriminator-ready.

Evidence level: state inspection, accepted predecessor evidence inspection,
source/artifact/header/output-path review, source and helper implementation,
non-published archive materialization/review, capture helper dry-run, shell
syntax validation, cargo fmt/build, task-owned JSON evidence, docs build, and
diff checks. No hardware action, hardwareTestLock acquisition, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Create the next smallest selected normal-runtime continuation discriminator
after v38: a source/static, non-published artifact contract for proving
BootInfo parsing after rust_entry and before target init or later
runtime/service work.

## Scope Performed

- Promoted this ready no-hardware task after the accepted v38 closeout and
  verified top-level planningNeeded=false, hardwareTestLock unlocked/restored,
  and no active supervisor intervention.
- Compared v34 asm_start, v36 asm_pre_rust_entry, v37 rust_entry contract, and
  v38 Pi 5 rust_entry evidence with rust_entry BootInfo handling, early serial
  output, build.rs scenario routing, selected archive helpers, and capture
  helper marker-family handling.
- Added a separate rpi5_ssh_service_smoltcp_bootinfo_marker_loop scenario that
  keeps the selected normal-runtime service cfg/root shape, parses the
  firmware/x0 BootInfo handoff, and loops on TALOS: boot info parsed before
  target init, exceptions, kernel_main, route-start, runtime-ready,
  packet-I/O, or service readiness.
- Materialized and reviewed a non-published v39 archive for v40.
- Added a task-specific archive review helper and recorded the v40 capture
  helper dry-run contract.

## Reconciliation

v34 proves the selected normal-runtime archive class can enter Talos assembly
and retain TALOS: asm_start on Pi 5. v36 proves the next assembly handoff at
TALOS: asm_pre_rust_entry. v37 defined the rust_entry marker-loop contract,
and v38 proved the selected 152,816-byte archive reaches TALOS: rust_entry on
Pi 5.

v39 keeps the selected rpi5_ssh_service_smoltcp_runtime_ready route linked
into the artifact, but adds an earlier BootInfo discriminator after
BootInfo::from_aarch64_x0(dtb_pa) and before target::init(&boot_info). The
marker loop explicitly withholds target init, exceptions, kernel_main,
route-start, runtime-ready, packet-I/O, service success, ssh-ready, and
phase-transition claims.

## V40 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-bootinfo-v39-boot-tree
    tar -czf target/tmp/selected-normal-runtime-bootinfo-v39.tar.gz \
      -C target/tmp/selected-normal-runtime-bootinfo-v39-boot-tree .

- Changed files: build.rs, src/main.rs, src/target/rpi5.rs,
  scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-boot-tree.sh, and
  scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-archive-review.sh.
- Archive path: target/tmp/selected-normal-runtime-bootinfo-v39.tar.gz.
- Archive SHA-256:
  23ba0d4dee7cde85e6b6ef914528f209c20cebf0edc022723a4bd1c84ea4cec5.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  62c285790a87b3ab8395aa9dcbd8167318506c940fe5a4f61e07371c0806486b.
- Selected kernel size: 152,880 bytes.
- Image header: text_offset=0, header_image_size=152880, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: rust_entry.
- TALOS: boot info parsed.

The v40 preflight must treat TALOS: boot info parsed as the required marker.
The loop intentionally stops before target init, exceptions, kernel_main,
route-start, runtime-ready, packet-I/O, OpenSSH compatibility, or service
readiness. Fail-closed classifications are
selected-normal-runtime-bootinfo-marker-retained,
blocked-selected-normal-runtime-bootinfo-marker-missing, or
selected-normal-runtime-bootinfo-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, hardware action outside the queued v40 preflight, and phase
transition remain blocked.

## Findings

- fixed: added the selected normal-runtime BootInfo marker-loop boot scenario
  rather than changing the existing runtime-ready route.
- fixed: moved the next required proof from rust_entry to a marker emitted only
  after BootInfo::from_aarch64_x0(dtb_pa) returns.
- fixed: kept downstream runtime-route code linked into the selected archive by
  making the loop returnable in the type system while preserving an always-
  false runtime exit guard.
- fixed: materialized and reviewed a non-published v39 archive with valid
  Image header fields, selected root/da591740 equality, and embedded BootInfo
  discriminator strings.
- not-an-issue: contiguous downstream runtime-route strings remain in the image
  because preserving the selected normal-runtime service shape is part of the
  contract; the BootInfo marker line itself withholds all later milestone
  claims.
- not-an-issue: TALOS: rust_entry is emitted by the existing byte-wise
  early-phase writer, not required as a contiguous image string in the v39
  archive helper; v38 already proves that predecessor hardware fact.
- deferred: v40 must run serialized Pi 5 hardware evidence for this exact
  archive contract before accepting BootInfo parsing or any later
  normal-runtime milestone.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39/validation/archive-review.stdout.txt.
- BootInfo archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39/validation/bootinfo-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-reconciliation-v39/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts,
marker names, helper arguments, classifications, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH/session/key material, boot artifact bytes, private data, or
stable secret-derived identifiers. The archive and boot tree are retained only
under target/tmp and are not committed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 253].
- sh -n:
  scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-boot-tree.sh and
  scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-archive-review.sh
  pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific BootInfo archive review: pass.
- Capture helper --dry-run for the v40 marker family and required marker: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
