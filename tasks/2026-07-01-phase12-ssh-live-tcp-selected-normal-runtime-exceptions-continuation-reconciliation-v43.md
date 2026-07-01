# Phase 12 SSH Live TCP Selected Normal Runtime Exceptions Continuation Reconciliation V43

Task id: phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-exceptions-discriminator-ready.

Evidence level: state inspection, accepted predecessor evidence inspection,
source/artifact/header/output-path review, source and helper implementation,
non-published archive materialization/review, capture helper dry-run, shell
syntax validation, cargo fmt/build, task-owned JSON evidence, docs build, and
diff checks. No hardware action, hardwareTestLock acquisition, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Goal

Create the next smallest selected normal-runtime continuation discriminator
after v42: a source/static, non-published artifact contract for proving
exceptions initialization after target init and before kernel_main or later
runtime/service work.

## Scope Performed

- Promoted this queued no-hardware task after the accepted v42 closeout and
  verified top-level planningNeeded=false, hardwareTestLock unlocked/restored,
  and no active supervisor intervention.
- Compared the accepted v34 asm_start proof, v36 asm_pre_rust_entry proof, v38
  rust_entry proof, v40 BootInfo proof, v41 target-init contract, v42 Pi 5
  target-init proof, and v42 closeout with the exceptions-ready boundary.
- Added a separate rpi5_ssh_service_smoltcp_exceptions_ready_marker_loop
  scenario that preserves the selected normal-runtime service cfg/root shape,
  reaches arch::aarch64::exceptions::init(), and loops on TALOS: exceptions
  ready before kernel_main, route-start, runtime-ready, packet-I/O, or service
  readiness.
- Materialized and reviewed a non-published v43 archive for a future serialized
  Pi 5 task.
- Recorded the future capture helper dry-run contract.

## Reconciliation

v34 proves the selected normal-runtime archive class can enter Talos assembly
and retain TALOS: asm_start on Pi 5. v36 proves the next assembly handoff at
TALOS: asm_pre_rust_entry. v38 proves the selected normal-runtime archive
reaches TALOS: rust_entry. v40 proves the selected BootInfo boundary at TALOS:
boot info parsed. v42 proves the selected target-init boundary at TALOS: target
init.

v43 keeps the selected rpi5_ssh_service_smoltcp_runtime_ready route linked into
the artifact, but adds an earlier exceptions-ready discriminator after
arch::aarch64::exceptions::init() returns and before kernel_main(&boot_info).
The marker loop explicitly withholds kernel_main, route-start, runtime-ready,
packet-I/O, service success, ssh-ready, and phase-transition claims.

## Future Hardware Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-exceptions-ready-v43-boot-tree
    tar -czf target/tmp/selected-normal-runtime-exceptions-ready-v43.tar.gz \
      -C target/tmp/selected-normal-runtime-exceptions-ready-v43-boot-tree .

- Changed files: build.rs, src/main.rs, src/target/rpi5.rs,
  scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-boot-tree.sh,
  and
  scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-archive-review.sh.
- Archive path: target/tmp/selected-normal-runtime-exceptions-ready-v43.tar.gz.
- Archive SHA-256:
  44733062c516f899b43b3a31241ac19c1bec4dd51d768260e56b066c0db15fed.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  34f17bf595aef2c658f9b95b81703a0c62c0051b6793ab8c7d527882dc164682.
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

A future hardware preflight must treat TALOS: exceptions ready as the required
marker. The loop intentionally stops before kernel_main, route-start,
runtime-ready, packet-I/O, OpenSSH compatibility, or service readiness.
Suggested fail-closed classifications are
selected-normal-runtime-exceptions-ready-marker-retained,
blocked-selected-normal-runtime-exceptions-ready-marker-missing, or
selected-normal-runtime-exceptions-ready-inconclusive-after-triage.

selected_next_task: null.

planningNeeded: true.

Reason: no explicit v44 exceptions-ready Pi 5 hardware preflight task currently
exists in taskQueue; the supervisor must instantiate the next bounded task
before the worker performs hardware action.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, hardware action, and phase transition remain blocked.

## Findings

- fixed: added the selected normal-runtime exceptions-ready marker-loop boot
  scenario rather than changing the existing runtime-ready route.
- fixed: moved the next required proof from target init to a marker emitted only
  after arch::aarch64::exceptions::init() returns.
- fixed: kept downstream runtime-route code linked into the selected archive by
  preserving the rpi5_ssh_service_smoltcp_runtime_ready implied scenario.
- fixed: materialized and reviewed a non-published v43 archive with valid Image
  header fields, selected root/da591740 equality, and embedded
  exceptions-ready discriminator strings.
- not-an-issue: contiguous downstream runtime-route strings remain in the image
  because preserving the selected normal-runtime service shape is part of the
  contract; the exceptions-ready marker line itself withholds later milestone
  claims.
- deferred: a future serialized Pi 5 preflight must prove this exact archive
  reaches TALOS: exceptions ready before accepting exceptions initialization or
  any later normal-runtime milestone.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43/validation/archive-review.stdout.txt.
- Exceptions-ready archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43/validation/exceptions-ready-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts,
marker names, helper arguments, classifications, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH/session/key material, boot artifact bytes, private data, or
stable secret-derived identifiers. The archive and boot tree are retained only
under target/tmp and are not committed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 259].
- sh -n:
  scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-boot-tree.sh
  and
  scripts/rpi5-ssh-service-smoltcp-exceptions-ready-marker-loop-archive-review.sh
  pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
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

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
