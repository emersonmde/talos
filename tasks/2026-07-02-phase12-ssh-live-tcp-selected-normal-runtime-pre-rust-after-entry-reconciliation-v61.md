# Phase 12 SSH Live TCP Selected Normal Runtime Pre-Rust After-Entry Reconciliation V61

Task id: phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61-20260702

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-pre-rust-discriminator-ready.

Evidence level: static inspection, non-published archive materialization/review, static disassembly inspection, capture helper dry-run, task-owned JSON evidence, docs build, and diff checks. No hardware action, hardwareTestLock acquisition, lab publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O implementation, OpenSSH/generated-root retry, remote receipt, compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or phase transition was performed.

## Goal

After v59 proved selected TALOS: asm_start on Pi 5, define the smallest current-source discriminator for the next boundary: TALOS: asm_pre_rust_entry before rust_entry.

## Scope Performed

- Continued the ready no-hardware v61 task after supervisor planning selected this exact post-entry/pre-rust reconciliation.
- Compared accepted v59 entry-loop evidence with the earlier accepted v35/v36 pre-rust discriminator line and current source/helper state.
- Re-materialized a non-published selected normal-runtime pre-rust marker-loop archive from current source.
- Recorded the v62 serialized Pi 5 preflight contract with TALOS: asm_pre_rust_entry as the required marker.
- Stopped before hardware action, route-start/runtime-ready claims, packet-I/O, OpenSSH/generated-root retry, fake command expansion, broad shell work, and phase transition.

## Terminal Classification

selected-normal-runtime-pre-rust-discriminator-ready.

v59 resolved selected staging, TFTP byte service, final pre-restore identity, marker-family serial observation, and restore proof through TALOS: asm_start. It did not prove CPACR setup, BSS clear, stack setup, rust_entry, BootInfo parsing, target init, exceptions, kernel_main, route-start, runtime-blocked, runtime-ready, packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake command expansion, broad shell work, or phase transition.

The current-source pre-rust discriminator keeps the selected normal-runtime service cfg and boot-tree shape but loops at TALOS: asm_pre_rust_entry after CPACR enable, BSS clear, and stack setup. Static disassembly shows the loop at _start+0x90 through _start+0xac, with the later rust_entry branch at _start+0xb4. That makes TALOS: asm_pre_rust_entry the exact next hardware boundary after the accepted v59 TALOS: asm_start proof.

## V62 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-pre-rust-v61 \
      scripts/rpi5-ssh-service-smoltcp-pre-rust-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-pre-rust-v61-boot-tree
    tar -czf target/tmp/selected-normal-runtime-pre-rust-v61.tar.gz \
      -C target/tmp/selected-normal-runtime-pre-rust-v61-boot-tree .

- Source commit for artifact: dc7f065f3fe6df3abfa874ddc65c47a5d371e2c6.
- Archive path: target/tmp/selected-normal-runtime-pre-rust-v61.tar.gz.
- Archive SHA-256: 6e7a35f4d875a510719ca8fbdb256f6513d8d0b1eb6c5e321e198b75f8878cd9.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256: 90c72361bc67be8933436ddc5e6807dc127a8cb329a3fcab49404c10f8086d59.
- Selected kernel size: 152144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Required success marker: TALOS: asm_pre_rust_entry.
- Marker family: TALOS: asm_start; TALOS: asm_pre_rust_entry; TALOS: rust_entry; TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-pre-rust-v61; TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=runtime-pre-rust-v61; TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-pre-rust-v61.
- Fail-closed classifications: selected-normal-runtime-entry-marker-retained; blocked-selected-normal-runtime-pre-rust-marker-missing; selected-normal-runtime-pre-rust-marker-retained; selected-normal-runtime-rust-entry-marker-retained; selected-normal-runtime-route-start-marker-retained; selected-normal-runtime-runtime-blocked-marker-retained; selected-normal-runtime-runtime-ready-marker-retained; selected-normal-runtime-pre-rust-inconclusive-after-triage.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62-20260702.

planningNeeded: false.

## Findings

- fixed: refreshed the selected normal-runtime pre-rust marker-loop archive contract from current source.
- fixed: recorded exact archive/kernel/Image metadata, required marker, marker family, restore target, fail-closed classifications, and capture-helper dry-run output for v62.
- fixed: separated this successor from v59: v59 stopped at TALOS: asm_start; v62 will test TALOS: asm_pre_rust_entry after assembly setup and before rust_entry.
- not-an-issue: no source code change was needed because the v35 helper/scenario still matches the next boundary.
- deferred: v62 must acquire hardwareTestLock and provide serialized Pi 5 identity/TFTP/serial/restore evidence before the frontier can move past pre-rust.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility/service readiness, ssh-ready=true, fake command expansion, broad shell work, hardware action, and phase transition as immediate v61 outcomes.

## Evidence Map

- Classification: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61/classification.json.
- Evidence map: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61/evidence-map.json.
- Static comparison report: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61/static/static-comparison-report.md.
- Archive review: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61/validation/archive-review.stdout.txt.
- Runtime archive review: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61/validation/runtime-archive-review.stdout.txt.
- Capture helper dry run: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61/validation/capture-bundle-dry-run.json.
- Static metadata: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61/static/.

## Redaction Review

Task-owned evidence retains task ids, source/path labels, hashes, byte counts, marker names, classifications, validation outcomes, selected-tree metadata, and local static reports. It does not retain raw serial text, raw TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot artifact bytes, private user data, stable secret-derived identifiers, public-key blobs, signatures, fingerprints, operator identities, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin with only untracked v61 evidence from a previous partial wake before this worker continued.
- sh -n on touched shell helpers: not run because no shell helper was touched.
- cargo fmt --all -- --check: not run because no Rust/build source was touched.
- cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json: pass as part of non-published archive materialization.
- Non-published archive materialization plus scripts/rpi5-archive-review.sh and scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh: pass.
- Static binary/header/marker/disassembly inspection: pass.
- Capture helper --dry-run for the v62 marker family and required marker: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
