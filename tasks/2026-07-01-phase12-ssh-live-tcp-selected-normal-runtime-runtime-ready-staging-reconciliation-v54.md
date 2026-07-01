# Phase 12 SSH Live TCP Selected Normal Runtime Runtime-Ready Staging Reconciliation V54

Task id: phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-staging-reconciliation-v54-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-runtime-ready-control-discriminator-ready.

Evidence level: static inspection, accepted v51 archive contract review,
accepted v52 hardware evidence review, bounded shell helper change, capture
helper dry-run, task-owned JSON evidence, docs build, and diff checks. No
hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
packet-I/O implementation, OpenSSH/generated-root retry, service readiness,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Goal

Repair or decisively discriminate the v52 selected-runtime staging/publication
gap before any packet-I/O, OpenSSH, or phase-transition work.

## Scope Performed

- Promoted this ready no-hardware task after the accepted blocked v52 closeout
  and supervisor planning refresh.
- Reconciled the accepted v51 selected archive metadata against the accepted
  v52 TFTP byte counts, final pre-restore identity, serial marker absence, and
  restore proof.
- Updated scripts/rpi5-capture-invariant-proof-bundle.sh to capture an
  immediate post-power, pre-serial-observe boot identity checkpoint.
- Regenerated the future v55 capture helper dry-run contract with the new
  post_power_pre_observe_identity field.
- Stopped before hardware action, packet-I/O, OpenSSH/generated-root retry,
  service readiness, ssh-ready=true, fake command expansion, broad shell work,
  or phase transition.

## Terminal Classification

selected-normal-runtime-runtime-ready-control-discriminator-ready.

v51 remains the authoritative runtime-ready archive contract:

- Archive path: target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Archive SHA-256:
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel size: 152,144 bytes.
- Selected kernel SHA-256:
  b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- Selected tree hash:
  c49997afe4dd2136706ad4f0dc05326d93abf60593c8a01104472984d5481bbc.
- Required marker:
  TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

v52 proved selected preflight identity before power action, then observed a
decisive staging gap: stable same-cursor TFTP served
da591740/kernel_2712.img twice at 104,136 bytes instead of 152,144 bytes,
final pre-restore identity was baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and serial
retained zero occurrences of the runtime-ready marker.

The accepted v54 change adds this checkpoint to the next capture bundle:

- post-power-pre-observe-status.json.
- post-power-pre-observe-root-endpoint.json and body.
- post-power-pre-observe-root.json.
- post-power-pre-observe-boot-files.json.
- capture-window-order stage post_power_pre_observe_identity.
- capture-invariant-summary.json fields post_power_pre_observe_identity and
  proof_run_identity.post_power_pre_observe.

That new evidence source/timing is the smallest accepted discriminator: the
next Pi 5 run can distinguish selected identity lost immediately after power
cycle, selected identity retained while dnsmasq serves baseline bytes, or
selected identity/TFTP retained while runtime-ready marker execution fails.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55-20260701.

planningNeeded: false.

## Findings

- fixed: added the immediate post-power/pre-observe identity checkpoint to the
  Pi 5 capture helper and dry-run contract.
- fixed: recorded the new checkpoint in the annotated capture summary and
  proof-run identity map for future v55 classification.
- fixed: reconciled v51 selected archive metadata against v52 baseline-sized
  TFTP service, baseline final pre-restore identity, zero runtime-ready marker
  retention, and restore proof.
- not-an-issue: no Rust runtime or archive source repair was justified by this
  no-hardware reconciliation; v51 archive metadata remains internally
  consistent and selected preflight identity was observed before power in v52.
- deferred: the next serialized Pi 5 preflight must run the revised helper and
  classify the new post-power identity against TFTP byte counts, final
  pre-restore identity, serial marker retention, and restore proof.
- removed: packet-I/O, OpenSSH/generated-root retry, service readiness,
  ssh-ready=true, fake command expansion, broad shell work, and phase
  transition as successors before v55 hardware evidence.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-staging-reconciliation-v54/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-staging-reconciliation-v54/evidence-map.json.
- Static reconciliation report:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-staging-reconciliation-v54/static/staging-reconciliation-report.md.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-staging-reconciliation-v54/validation/capture-bundle-dry-run.json.

## Redaction Review

This task retained no raw serial text, raw TFTP peer/log-line fields, packet
payloads, SSH/session/key material, boot artifact bytes, private user data,
stable secret-derived identifiers, public-key blobs, signatures, fingerprints,
digests, or operator identities.

## Validation

- git status --short --branch before edits/action: ## main...origin/main
  [ahead 273].
- jq empty on supervisor state and task-owned JSON evidence: pass.
- sh -n scripts/rpi5-capture-invariant-proof-bundle.sh: pass.
- Capture helper --dry-run with the v51/v55 marker family: pass; the dry-run
  includes post_power_pre_observe_identity and the post-power files.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
