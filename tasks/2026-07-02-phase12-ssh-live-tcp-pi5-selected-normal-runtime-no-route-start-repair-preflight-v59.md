# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime No-Route-Start Repair Preflight V59

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-no-route-start-repair-preflight-v59-20260702

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-entry-marker-retained.

Evidence level: static archive/image review, lab-controller API, serial hardware boot/output summary, TFTP log delta, restore proof, task-owned JSON evidence, docs build, and redaction review.

## Goal

Run the selected v58 normal-runtime entry-loop discriminator on the Pi 5 and determine whether the selected no-route-start frontier is before Talos Image entry or later in the early normal-runtime path.

## Scope Performed

- Promoted the queued v59 hardware preflight after accepted v58 selected this exact task.
- Acquired hardwareTestLock before lab publication, boot snapshot mutation, Pi 5 power action, or hardware capture.
- Re-reviewed the accepted v58 archive:
  target/tmp/selected-normal-runtime-entry-loop-v58.tar.gz.
- Published the v58 selected entry-loop archive and ran one foreground Pi 5 capture chain.
- Restored the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and released hardwareTestLock.

## Terminal Classification

selected-normal-runtime-entry-marker-retained.

The decisive candidate facts are:

- Archive SHA-256:
  9988a761539867a50db538d64533df78b0af6d9cd3277ee0a1189cd3b2effc37.
- Selected tree:
  c8a7e7d3de13900ab5d87b17040f82b85e6e2a557a9de1e6f882812c448f6a0f.
- Selected fetch: da591740/kernel_2712.img.
- Selected kernel size: 152,144 bytes.
- Selected kernel SHA-256:
  6a7b970144e43c5b57b343c5ee4ff1275b077403ee83c3806dedd740acc89301.
- Post-power/pre-observe identity: selected tree retained.
- Same-window TFTP: two selected da591740/kernel_2712.img serves at 152,144 bytes.
- Final pre-restore identity: selected tree retained with the selected fetch at 152,144 bytes.
- Serial marker family: TALOS: asm_start retained 547 times; TALOS: asm_pre_rust_entry, TALOS: kernel_main, TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-no-route-start-v58, TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=runtime-no-route-start-v58, and TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-no-route-start-v58 retained zero occurrences.
- Restore proof: post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

This is selected entry proof, not route-start or runtime-ready proof. Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim, service readiness, ssh-ready=true, fake/kernel-backed command expansion, broad shell work, and phase transition remain blocked.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-repair-closeout-v59-20260702.

planningNeeded: false.

## Findings

- fixed: the selected entry-loop discriminator retained fresh TALOS: asm_start output on Pi 5 after selected identity and selected TFTP service.
- fixed: candidate identity, fresh serial cursor, TFTP delta, final pre-restore identity, restore proof, and marker-family counts were captured in task-owned evidence.
- not-an-issue: known-good control was not required because the first candidate evidence was decisive, not inconclusive.
- deferred: no asm_pre_rust_entry, kernel_main, route-start, runtime-blocked, or runtime-ready marker was retained; closeout must keep packet-I/O/OpenSSH blocked until a deeper runtime frontier is proved.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-no-route-start-repair-preflight-v59/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-no-route-start-repair-preflight-v59/evidence-map.json.
- Archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-no-route-start-repair-preflight-v59/validation/archive-review.stdout.txt.
- Candidate summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-no-route-start-repair-preflight-v59/lab/v59-candidate/candidate-summary.json.

## Redaction Review

Task-owned JSON evidence was redacted after derived summaries were created. Raw serial text/base64 and TFTP peer/log-line fields were replaced with REDACTED while retaining counts, filenames, byte counts, cursor metadata, tree hashes, marker-family classifications, and restore status. No private key, seed, public-key blob, signature, fingerprint, operator identity, or stable secret-derived identifier is retained.

## Validation

- git status --short --branch before lab publication/hardware action:
  ## main...origin/main [ahead 281].
- jq empty on task-owned JSON evidence and supervisor state before and after lock changes: pass.
- Static archive/image review before publication: pass; archive SHA-256 9988a761539867a50db538d64533df78b0af6d9cd3277ee0a1189cd3b2effc37 and selected kernel SHA-256 6a7b970144e43c5b57b343c5ee4ff1275b077403ee83c3806dedd740acc89301.
- Lab API candidate identity before publication, after publication, post-power/pre-observe, final pre-restore, and after restore: recorded.
- Fresh serial cursor and marker-family serial observation: recorded; TALOS: asm_start retained, deeper normal-runtime markers absent.
- GET /tftp/logs cursor delta: recorded; selected candidate served the selected fetch twice at 152,144 bytes.
- Known-good control: not run because the first candidate was decisive.
- Restore proof to accepted baseline before releasing hardwareTestLock: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-repair-closeout-v59-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
