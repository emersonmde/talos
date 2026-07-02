# Phase 12 SSH Live TCP Selected Normal Runtime No-Route-Start Provenance Reconciliation V58

Task id: phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58-20260702

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-no-route-start-discriminator-ready.

Evidence level: static inspection, non-published archive materialization/review, capture helper dry-run, task-owned JSON evidence, docs build, and diff checks. No hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O implementation, OpenSSH/generated-root retry, remote receipt, compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or phase transition was performed.

## Goal

Reconcile the v57 selected no-route-start frontier and select exactly one bounded repair/discriminator for the queued serialized Pi 5 preflight.

## Scope Performed

- Promoted this ready no-hardware reconciliation after supervisor planning resolved the post-v57 blocker.
- Compared the accepted route-start-good v49/v50 line, v51 runtime-ready archive, v56 marker-family contract, v57 no-route-start hardware evidence, and prior v33/v34 selected-entry discriminator proof.
- Re-materialized a current-source selected normal-runtime entry-loop discriminator archive for v59.
- Recorded a v59 dry-run capture contract that requires TALOS: asm_start and classifies deepest retained marker-family evidence without accepting packet-I/O or OpenSSH successors.

## Terminal Classification

selected-normal-runtime-no-route-start-discriminator-ready.

The accepted v57 facts prove selected post-power identity, selected same-window TFTP byte service, selected final pre-restore identity, known-good TFTP cursor health, and restore proof for the 152144-byte v51 runtime-ready archive. They do not prove runtime-ready because the serial window retained zero marker-family occurrences from TALOS: asm_start through runtime-ready.

The smallest bounded discriminator is to refresh the selected normal-runtime entry-loop archive shape that v33/v34 already established: keep the selected normal-runtime service cfg and 152144-byte Image contract, but loop on TALOS: asm_start before Rust-side work. This directly tests whether the latest selected no-route-start frontier is a capture/one-shot-marker issue or still missing before selected Talos Image entry.

## V59 Contract

Archive materialization:

    TALOS_CAPTURE_NONCE=runtime-no-route-start-v58 \
      scripts/rpi5-ssh-service-smoltcp-entry-marker-loop-boot-tree.sh \
      target/timer-irq-boot-source \
      target/tmp/selected-normal-runtime-entry-loop-v58-boot-tree
    tar -czf target/tmp/selected-normal-runtime-entry-loop-v58.tar.gz \
      -C target/tmp/selected-normal-runtime-entry-loop-v58-boot-tree .

- Source commit for artifact: f7f5133e3cc107d6d9079a9128effe38bb77b8c7.
- Archive path: target/tmp/selected-normal-runtime-entry-loop-v58.tar.gz.
- Archive SHA-256: 9988a761539867a50db538d64533df78b0af6d9cd3277ee0a1189cd3b2effc37.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256: 6a7b970144e43c5b57b343c5ee4ff1275b077403ee83c3806dedd740acc89301.
- Selected kernel size: 152144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Required success marker: TALOS: asm_start.
- Expected marker family: TALOS: asm_start; TALOS: asm_pre_rust_entry; TALOS: kernel_main; TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-no-route-start-v58; TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=runtime-no-route-start-v58; TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-no-route-start-v58.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-no-route-start-repair-preflight-v59-20260702.

planningNeeded: false.

## Findings

- fixed: refreshed the selected normal-runtime entry-loop discriminator from current source and gave v59 exact archive/kernel metadata.
- fixed: preserved the v57 boundary: selected identity/TFTP/final identity are proved; runtime-ready and packet-I/O remain blocked.
- fixed: made TALOS: asm_start the required v59 success marker and recorded fail-closed marker-family classifications for no-route-start, entry, route-start, runtime-blocked, runtime-ready, and inconclusive outcomes.
- not-an-issue: source routing and target selection for rpi5_ssh_service_smoltcp_runtime_ready are registered and still emit the expected route-start/runtime-blocked/runtime-ready strings.
- not-an-issue: the refreshed archive has valid Image header fields, selected root/da591740 equality, and no loader diagnostic token.
- deferred: v59 must run under hardwareTestLock before any closeout can move the frontier beyond selected no-route-start.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility/service readiness, ssh-ready=true, fake command expansion, broad shell work, hardware action, and phase transition as immediate successors.

## Evidence Map

- Classification: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58/classification.json.
- Evidence map: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58/evidence-map.json.
- Static comparison report: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58/static/static-comparison-report.md.
- Archive review: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58/validation/archive-review.stdout.txt.
- Runtime archive review: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58/validation/runtime-archive-review.stdout.txt.
- Capture helper dry-run: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58/validation/capture-bundle-dry-run.json.

## Redaction Review

This reconciliation retained no raw serial text, raw TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot artifact bytes, private user data, stable secret-derived identifiers, public-key blobs, signatures, fingerprints, operator identities, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin with no uncommitted Talos changes before promotion.
- sh -n: not run because no shell helper was touched.
- cargo fmt --all -- --check: not run because no Rust/build source was touched.
- cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json: pass as part of non-published archive materialization.
- Non-published archive materialization plus archive review: pass.
- Static binary/header/marker inspection: pass.
- Capture helper --dry-run for the v59 marker family and required marker: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-no-route-start-repair-preflight-v59-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
