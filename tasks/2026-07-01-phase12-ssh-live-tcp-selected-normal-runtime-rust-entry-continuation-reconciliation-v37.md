# Phase 12 SSH Live TCP Selected Normal Runtime Rust Entry Continuation Reconciliation V37

Task id: phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-rust-entry-discriminator-ready.

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
after v36: a source/static, non-published artifact contract for proving entry
into rust_entry before any packet-I/O or OpenSSH work.

## Scope Performed

- Promoted this ready no-hardware task after the accepted v36 closeout and
  verified top-level planningNeeded=false, hardwareTestLock unlocked/restored,
  and no active supervisor intervention.
- Compared the accepted v34 asm_start proof, v35 static pre-rust contract, and
  v36 Pi 5 asm_pre_rust_entry proof with boot.S, build.rs scenario routing,
  rust_entry dispatch, selected archive helpers, and marker-family capture
  handling.
- Added a separate rpi5_ssh_service_smoltcp_rust_entry_marker_loop scenario
  that keeps the selected normal-runtime service cfg shape and assembly
  provenance markers, enters rust_entry, and loops on TALOS: rust_entry before
  BootInfo parsing or later normal-runtime milestones.
- Materialized and reviewed a non-published v37 archive for v38.
- Added a task-specific archive review helper and recorded the v38 capture
  helper dry-run contract.

## Reconciliation

v34 proves the selected normal-runtime archive class can enter Talos assembly
and retain TALOS: asm_start on Pi 5. v35/v36 prove the next assembly handoff:
after CPACR setup, BSS clear, and stack setup, the selected archive can retain
TALOS: asm_pre_rust_entry. Those facts still do not prove that the selected
normal-runtime artifact enters Rust.

v37 keeps the selected rpi5_ssh_service_smoltcp_runtime_ready route linked into
the artifact, but adds an earlier Rust-side discriminator that emits TALOS:
rust_entry immediately after rust_entry begins. The marker loop explicitly
withholds BootInfo parsing, target init, exceptions, kernel_main, packet-I/O,
service success, ssh-ready, and phase-transition claims.

## V38 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-rust-entry-v37-boot-tree
    tar -czf target/tmp/selected-normal-runtime-rust-entry-v37.tar.gz \
      -C target/tmp/selected-normal-runtime-rust-entry-v37-boot-tree .

- Changed files: build.rs, src/main.rs, src/target/rpi5.rs,
  scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-boot-tree.sh, and
  scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-archive-review.sh.
- Archive path: target/tmp/selected-normal-runtime-rust-entry-v37.tar.gz.
- Archive SHA-256:
  b8014b4b935bd81c3fdb077046cc5b10071b57d71af628678c9def68f8b43053.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  c1c1c864ca89babb516c11ce0f52357c69b79c2c6034e42150494a043658f9bc.
- Selected kernel size: 152816 bytes.
- Image header: text_offset=0, header_image_size=152816, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: rust_entry.

The v38 preflight must treat TALOS: rust_entry as the required marker. The
loop intentionally stops before BootInfo parsing, target init, exceptions,
kernel_main, packet-I/O, OpenSSH compatibility, or service readiness. Fail-
closed classifications are selected-normal-runtime-rust-entry-marker-retained,
blocked-selected-normal-runtime-rust-entry-marker-missing, or
selected-normal-runtime-rust-entry-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, hardware action outside the queued v38 preflight, and phase
transition remain blocked.

## Findings

- fixed: added the selected normal-runtime rust_entry marker-loop boot
  scenario rather than changing the existing runtime-ready route.
- fixed: moved the next required proof from asm_pre_rust_entry to a marker
  emitted by Rust code after rust_entry begins.
- fixed: kept downstream runtime-route code linked into the selected archive by
  making the loop returnable in the type system while preserving an always-
  false runtime exit guard.
- fixed: materialized and reviewed a non-published v37 archive with valid Image
  header fields, selected root/da591740 equality, and embedded ordered marker
  strings.
- not-an-issue: the local objdump binary cannot disassemble the AArch64 ELF;
  symbol evidence and source/static artifact review still establish the
  handoff contract.
- deferred: v38 must run serialized Pi 5 hardware evidence for this exact
  archive contract before accepting rust_entry or any later normal-runtime
  milestone.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37/validation/archive-review.stdout.txt.
- Rust-entry archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37/validation/rust-entry-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-reconciliation-v37/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts,
marker names, helper arguments, classifications, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet payloads,
SSH/session/key material, boot artifact bytes, private data, or stable
secret-derived identifiers. The archive and boot tree are retained only under
target/tmp and are not committed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 250].
- sh -n:
  scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-boot-tree.sh and
  scripts/rpi5-ssh-service-smoltcp-rust-entry-marker-loop-archive-review.sh
  pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization and scripts/rpi5-archive-review.sh:
  pass.
- Task-specific rust_entry archive review: pass.
- Capture helper --dry-run for the v38 marker family and required marker: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
