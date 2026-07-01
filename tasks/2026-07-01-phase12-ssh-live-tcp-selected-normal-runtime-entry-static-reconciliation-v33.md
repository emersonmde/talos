# Phase 12 SSH Live TCP Selected Normal Runtime Entry Static Reconciliation V33

Task id: phase12-ssh-live-tcp-selected-normal-runtime-entry-static-reconciliation-v33-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-entry-static-discriminator-ready.

Evidence level: state inspection, accepted predecessor evidence inspection,
source/artifact/header/linker/output-path review, source and helper
implementation, non-published archive materialization/review, shell syntax
validation, cargo fmt/build, task-owned JSON evidence, docs build, and diff
checks. No hardware action, hardwareTestLock acquisition, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Goal

Reconcile the v32 selected normal-runtime pre-entry marker-missing result and
produce the smallest static repair/discriminator contract that can decide
whether the 152,144-byte selected normal-runtime Image enters Talos code at
all.

## Scope Performed

- Promoted this ready no-hardware task after the accepted v32 closeout and
  verified hardwareTestLock was unlocked/restored with no active supervisor
  intervention.
- Compared accepted v26 rust_entry/UART10 marker-loop evidence, accepted v31
  static assembly pre-entry provenance, accepted v32 selected-byte/no-marker
  hardware evidence, and the known-good production-timer control.
- Added a separate rpi5_ssh_service_smoltcp_entry_marker_loop scenario that
  implies the normal-runtime service feature at Rust cfg level, but uses an
  assembly-only repeated TALOS: asm_start loop before CPACR setup, BSS clear,
  stack setup, rust_entry, BootInfo parsing, target init, exceptions,
  kernel_main, networking, or service code.
- Added a task-specific boot-tree helper for that selected archive shape and
  materialized a non-published archive for v34.
- Left the existing rpi5_ssh_service_smoltcp_runtime_ready one-shot pre-entry
  provenance path intact for later continuation after the entry boundary is
  proved.

## Reconciliation

The v26 selected-entry proof remains a different artifact: a 45,400-byte
selected marker-loop image reached rust_entry and the UART10 output path. The
v31/v32 selected normal-runtime artifact is the larger 152,144-byte feature
image. It has a valid arm64 Image header, selected da591740/kernel_2712.img
service, root/selected kernel equality, and embedded TALOS: asm_start,
TALOS: asm_pre_rust_entry, rust_entry, route-start, and runtime-ready strings,
but v32 primary and unchanged rerun retained no ordered marker. The v32
known-good production-timer control retained the PASS marker on the same
capture path, so the next step should not be packet-I/O or OpenSSH. The first
missing fact is still selected normal-runtime entry before TALOS: asm_start.

The bounded discriminator is to keep the selected normal-runtime service cfg
and archive contract while replacing the one-shot pre-entry marker with a
repeated assembly TALOS: asm_start loop. If v34 retains the repeated marker,
the selected 152,144-byte Image entered Talos assembly and the next repair can
return to continuation after that boundary. If v34 again serves the selected
bytes but retains no TALOS: asm_start after known-good control and unchanged
rerun, the missing fact remains before selected Image entry rather than in
Rust, BootInfo, networking, or OpenSSH.

## V34 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-marker-route-static \
      scripts/rpi5-ssh-service-smoltcp-entry-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-entry-loop-v33-boot-tree
    tar -czf target/tmp/selected-normal-runtime-entry-loop-v33.tar.gz \
      -C target/tmp/selected-normal-runtime-entry-loop-v33-boot-tree .

- Source base before task changes:
  ec154f4c001f2e0e47184e642879a0fec6d70a32.
- Changed files:
  build.rs, src/arch/aarch64/boot.S, and
  scripts/rpi5-ssh-service-smoltcp-entry-marker-loop-boot-tree.sh.
- Archive path: target/tmp/selected-normal-runtime-entry-loop-v33.tar.gz.
- Archive SHA-256:
  cf57163942a3cc9989b6346a7c3bc3a30dd295118cbc86afbd5f0844118db0f3.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256:
  5aa2b4ab51afa018d4c39fc5843e5df01a76dbc42bce2b40287693b5c77d311d.
- Selected kernel size: 152,144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Expected marker family:

- TALOS: asm_start.

The v34 preflight must treat TALOS: asm_start as the predecessor-named
earliest marker and the primary marker. The loop intentionally does not reach
rust_entry or service code; this is an entry discriminator, not a
compatibility/service readiness claim. Fail-closed classifications are
selected-normal-runtime-entry-marker-retained,
blocked-selected-normal-runtime-entry-marker-missing, or
selected-normal-runtime-entry-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-entry-repair-preflight-v34-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, and phase transition remain blocked.

## Findings

- fixed: added a separate selected normal-runtime entry-loop discriminator
  rather than changing the existing continuation/runtime-ready scenario.
- fixed: preserved the selected normal-runtime Rust cfg and runtime marker
  strings in the artifact while stopping at a repeated assembly asm_start
  marker before Rust-side work.
- fixed: materialized and reviewed a non-published v33 archive with valid
  Image header fields, selected root/da591740 equality, and embedded expected
  marker/runtime strings.
- not-an-issue: v26 still proves only the earlier 45,400-byte selected-entry
  marker-loop artifact; it does not prove the 152,144-byte normal-runtime
  artifact.
- deferred: v34 must run serialized Pi 5 hardware evidence for this exact
  archive contract before any continuation repair can be selected.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-static-reconciliation-v33/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-static-reconciliation-v33/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-static-reconciliation-v33/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-static-reconciliation-v33/validation/archive-review.stdout.txt.
- Runtime archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-static-reconciliation-v33/validation/runtime-archive-review.stdout.txt.
- Archive/kernel metadata:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-static-reconciliation-v33/static/.

## Redaction Review

Task-owned evidence retains task ids, source/commit/path labels, hashes, byte
counts, marker names, classifications, validation outcomes, selected-tree
metadata, and local static reports. It does not retain raw serial text, raw
TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot
artifact bytes, private user data, stable secret-derived identifiers, or
unnecessary hardware data. The archive and boot tree remain under target/tmp
and are not committed.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before v33 promotion.
- sh -n on touched shell helper: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: pass.
- Non-published archive materialization plus scripts/rpi5-archive-review.sh
  and scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-entry-repair-preflight-v34-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
