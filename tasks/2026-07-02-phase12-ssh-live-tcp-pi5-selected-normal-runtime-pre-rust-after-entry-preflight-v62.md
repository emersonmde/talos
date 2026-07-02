# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Pre-Rust After-Entry Preflight V62

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62-20260702

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-pre-rust-marker-retained.

Evidence level: static archive/image review, lab-controller API, serial hardware boot/output summary, TFTP log delta, restore proof, task-owned JSON evidence, docs build, and redaction review.

## Goal

Run the selected v61 normal-runtime pre-rust discriminator on the Pi 5 and determine whether the selected image reaches TALOS: asm_pre_rust_entry after the accepted TALOS: asm_start frontier.

## Scope Performed

- Promoted the queued v62 hardware preflight after accepted v61 selected this exact task.
- Acquired hardwareTestLock before lab publication, boot snapshot mutation, Pi 5 power action, or hardware capture.
- Re-reviewed the accepted v61 archive:
  target/tmp/selected-normal-runtime-pre-rust-v61.tar.gz.
- Published the v61 selected pre-rust archive and ran one foreground Pi 5 capture chain.
- Restored the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and released hardwareTestLock.

## Terminal Classification

selected-normal-runtime-pre-rust-marker-retained.

The decisive candidate facts are:

- Archive SHA-256:
  6e7a35f4d875a510719ca8fbdb256f6513d8d0b1eb6c5e321e198b75f8878cd9.
- Selected tree:
  2f4d07fc983ec52c2a23dbc358f7730bd608ed27ff95fea3a5ebc7784b1c6823.
- Selected fetch: da591740/kernel_2712.img.
- Selected kernel size: 152,144 bytes.
- Selected kernel SHA-256:
  90c72361bc67be8933436ddc5e6807dc127a8cb329a3fcab49404c10f8086d59.
- Post-power/pre-observe identity: selected tree retained.
- Same-window TFTP: two selected da591740/kernel_2712.img serves at 152,144 bytes.
- Final pre-restore identity: selected tree retained with the selected fetch at 152,144 bytes.
- Serial marker family: TALOS: asm_pre_rust_entry retained 535 times; TALOS: asm_start, TALOS: rust_entry, TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-pre-rust-v61, TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=runtime-pre-rust-v61, and TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-pre-rust-v61 retained zero occurrences in the fresh post-power window.
- Restore proof: post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

This is selected pre-rust proof, not rust_entry, route-start, runtime-ready, packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or phase transition proof.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-closeout-v62-20260702.

planningNeeded: false.

## Findings

- fixed: the selected pre-rust discriminator retained fresh TALOS: asm_pre_rust_entry output on Pi 5 after selected identity and selected TFTP service.
- fixed: candidate identity, fresh serial window, same-cursor TFTP delta, final pre-restore identity, restore proof, and marker-family counts were captured in task-owned evidence.
- not-an-issue: known-good control and candidate rerun were not required because first candidate evidence was decisive, not inconclusive.
- deferred: rust_entry, route-start, runtime-blocked, and runtime-ready markers were not retained; closeout must keep packet-I/O/OpenSSH blocked until a deeper runtime frontier is proved.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62/evidence-map.json.
- Archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62/validation/archive-review.stdout.txt.
- Runtime archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62/validation/runtime-archive-review.stdout.txt.
- Candidate summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62/lab/v62-candidate/candidate-summary.json.
- Capture helper summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-after-entry-preflight-v62/lab/v62-candidate/capture-helper.stdout.json.

## Redaction Review

Task-owned JSON evidence redacts raw serial text and raw TFTP peer/log-line fields after derived summaries are created. It retains counts, filenames, byte counts, cursor metadata, tree hashes, marker-family classifications, and restore status. No private key, seed, public-key blob, signature, fingerprint, operator identity, or stable secret-derived identifier is retained.

## Validation

- git status --short --branch before lab publication/hardware action:
  ## main...origin/main [ahead 284].
- jq empty on task-owned JSON evidence and supervisor state before and after lock changes: pass.
- Static archive/image review before publication: pass; archive SHA-256 6e7a35f4d875a510719ca8fbdb256f6513d8d0b1eb6c5e321e198b75f8878cd9 and selected kernel SHA-256 90c72361bc67be8933436ddc5e6807dc127a8cb329a3fcab49404c10f8086d59.
- Lab API candidate identity before publication, after publication, post-power/pre-observe, final pre-restore, and after restore: recorded.
- Fresh serial cursor and marker-family serial observation: recorded; TALOS: asm_pre_rust_entry retained 535 times.
- GET /tftp/logs cursor delta: recorded; selected candidate served the selected fetch twice at 152,144 bytes.
- Known-good control: not run because the first candidate was decisive.
- Candidate rerun: not run because the first candidate was decisive.
- Restore proof to accepted baseline before releasing hardwareTestLock: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-closeout-v62-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
