# Phase 12 SSH Live TCP Selected Normal Runtime Runtime-Ready Marker-Family Reconciliation V56

Task id: phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-runtime-ready-marker-family-discriminator-ready.

Evidence level: static inspection, shell syntax check, non-published archive review, capture helper dry-run, task-owned JSON evidence, docs build, and diff checks. No hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim, service readiness, ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or phase transition was performed.

## Goal

Prepare a no-hardware marker-family discriminator after v55 proved selected post-power identity, selected same-window TFTP byte service, and selected final pre-restore identity, but retained zero runtime-ready marker occurrences.

## Scope Performed

- Promoted this ready no-hardware reconciliation after supervisor planning resolved the v55 planning request.
- Compared the accepted v51 archive/source contract, v55 evidence, the runtime-ready route in src/target/rpi5.rs, the runtime marker report in src/network.rs, the archive review helper, and the capture invariant helper dry-run.
- Kept the v51 non-published archive authoritative because the source/image already emits route-start, runtime-blocked, and runtime-ready markers with the same capture nonce.
- Tightened scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh so the review requires and reports the runtime-blocked marker alongside route-start and runtime-ready.
- Tightened scripts/rpi5-capture-invariant-proof-bundle.sh dry-run output so the successor contract records required_success_marker and explicit fail-closed marker-family classifications.

## Terminal Classification

selected-normal-runtime-runtime-ready-marker-family-discriminator-ready.

The accepted v56 contract selects the next serialized Pi 5 hardware preflight:

phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57-20260701.

The v57 preflight must use this marker family:

- TALOS: asm_start.
- TALOS: asm_pre_rust_entry.
- TALOS: kernel_main.
- TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-ready-static.
- TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=runtime-ready-static.
- TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static.

Required success marker:

TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static.

Fail-closed classifications:

- selected-normal-runtime-no-route-start-marker-retained.
- selected-normal-runtime-route-start-only-marker-retained.
- selected-normal-runtime-runtime-blocked-marker-retained.
- selected-normal-runtime-runtime-ready-marker-retained.
- inconclusive-selected-normal-runtime-runtime-ready-marker-family-evidence.

Archive contract:

- Archive path: target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Archive SHA-256: 44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256: b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- Selected kernel size: 152,144 bytes.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

Packet-I/O remains dependency-blocked until a later closeout proves runtime-ready on Pi 5 with selected identity/TFTP/serial evidence.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57-20260701.

planningNeeded: false.

## Findings

- fixed: v55 marker-family evidence only counted runtime-ready markers, so a missing runtime-ready line could not distinguish route-start-only from runtime-blocked.
- fixed: the archive review now requires and reports TALOS: ssh-service-smoltcp-runtime-blocked with capture-nonce=runtime-ready-static.
- fixed: capture-bundle dry-run output now records required_success_marker and the v57 fail-closed marker-family classifications.
- not-an-issue: no Rust route change is needed because the existing route already emits route-start before the runtime report and emits either runtime-ready or runtime-blocked with the same nonce.
- not-an-issue: the v51 archive remains the selected artifact; the image bytes and selected kernel SHA/size did not change.
- deferred: v57 must run under hardwareTestLock before runtime-ready, packet-I/O, OpenSSH/generated-root retry, service readiness, ssh-ready, fake command expansion, broad shell work, or phase transition can be selected.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56/evidence-map.json.
- Static comparison report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56/static/static-comparison-report.md.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56/validation/archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56/validation/capture-bundle-dry-run.json.

## Redaction Review

This reconciliation retained no raw serial text, raw TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot artifact bytes, private user data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: ## main...origin/main [ahead 276].
- sh -n scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh: pass.
- sh -n scripts/rpi5-capture-invariant-proof-bundle.sh: pass.
- Task-specific runtime-ready archive review: pass.
- Capture helper --dry-run for the v57 marker family, required success marker, and fail-closed classifications: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57-20260701.

planningNeeded: false.

Commit: pending final commit.
