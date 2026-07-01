# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Route Start Continuation Preflight V50

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-route-start-marker-retained.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, fresh serial cursor/drain, stable same-cursor TFTP evidence,
serial hardware marker summary, boot-staging identity discriminator, restore
proof, redacted task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the v49-selected normal-runtime route-start continuation contract on the
Pi 5 and classify whether the selected Image reaches TALOS:
ssh-service-smoltcp-runtime-route-start
capture-nonce=runtime-marker-route-static before runtime-ready, packet-I/O,
OpenSSH, service readiness, ssh-ready=true, fake command expansion, broad
shell work, or phase transition.

## Scope Performed

- Promoted this ready hardware task after v49 accepted
  selected-normal-runtime-route-start-discriminator-ready and selected this
  exact task.
- Acquired hardwareTestLock before lab publication, boot mutation, Pi 5 power
  action, or hardware capture.
- Published only the v49 route-start marker-loop archive:
  target/tmp/selected-normal-runtime-route-start-v49.tar.gz.
- Captured selected identity, fresh serial/TFTP cursors, same-window TFTP,
  serial marker output, final pre-restore identity, and restore proof.
- Redacted raw serial text plus raw TFTP peer/log-line fields from retained
  task-owned JSON after deriving marker counts and byte-count summaries.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Candidate: selected-normal-runtime-route-start-v50.

The accepted run published selected tree
e1c8ce434afb82517063c9535f53d127ae220b76e2756d65b110fc808193ac63 with
effective kernel_2712.img. The final pre-restore identity remained on that
same selected tree. Stable same-cursor TFTP served
da591740/kernel_2712.img twice at 152,640 bytes. The saturated direct-read
serial window retained TALOS: ssh-service-smoltcp-runtime-route-start
capture-nonce=runtime-marker-route-static 2,326 times. Post-restore identity
returned to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Terminal Classification

selected-normal-runtime-route-start-marker-retained.

This proves the selected 152,640-byte normal-runtime route-start marker-loop
archive reaches the runtime route-start boundary on Pi 5 with selected-byte
TFTP service and restore proof. It does not prove runtime-ready, packet-I/O,
OpenSSH compatibility, remote receipt, service readiness, ssh-ready=true, fake
command expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-closeout-v50-20260701.

planningNeeded: false.

## Findings

- fixed: published and ran the v49-selected route-start continuation contract
  under hardwareTestLock with selected-byte TFTP evidence and restore proof.
- fixed: accepted only the run whose selected tree identity, selected fetch byte
  count, TFTP delta, final pre-restore identity, serial marker output, and
  restore proof are joined under the same candidate run.
- fixed: redacted raw serial text plus raw TFTP peer/log-line fields from
  retained task-owned JSON while preserving marker counts, byte counts, hashes,
  and classifications.
- not-an-issue: scripts/rpi5-observe-serial-window.sh returned exit 1 because
  the exact has_required_marker flag did not trip, but the same redacted
  capture summary's marker-family and required-marker counts retained the
  required route-start marker 2,326 times and the identity-join contract had no
  rejection reasons.
- not-an-issue: the optional capture-window v5 checker is stricter than this
  task's contract because it requires a run-unique nonce; v49 explicitly
  selected the constant runtime-marker-route-static route-start archive, and
  the required route-start marker was absent before power and present after
  power under marker-family freshness.
- deferred: runtime-ready, packet-I/O, OpenSSH compatibility, remote receipt,
  service readiness, ssh-ready=true, fake command expansion, broad shell work,
  and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50/evidence-map.json.
- Redacted run summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50/lab/run-summary-redacted.json.
- Accepted candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50/lab/v50-candidate/.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50/validation/route-start-archive-review.stdout.txt.
- Boot-staging identity discriminator:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50/lab/v50-candidate/boot-staging-identity-check.json.

## Redaction Review

Task-owned aggregate summaries retain task ids, run labels, hashes, byte
counts, marker names, marker counts, selected-tree hashes, classifications, and
validation outcomes. Retained JSON evidence has raw serial text, raw TFTP
peer/log-line fields, packet payloads, SSH/session/key material, boot artifact
bytes, private data, and stable secret-derived identifiers redacted or absent.

## Validation

- git status --short --branch before lab action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- hardwareTestLock acquisition before lab publication/power/capture: pass.
- Lab API identity after publication, final pre-restore, and after restore:
  pass.
- fresh serial cursor/drain and GET /tftp/logs cursor before Pi 5 power action:
  pass.
- stable same-cursor TFTP delta before restore: pass.
- boot-staging identity discriminator: pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10: pass.
- known-good control: not run because the first candidate was decisive.
- candidate rerun: not run because no inconclusive identity, serial freshness,
  or TFTP capture reason remained.
- sh -n: not run because no shell helper was touched.
- cargo fmt --all -- --check: not run because no Rust/build source was
  touched.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: not run because no Rust/build,
  target, linker, or source routing was touched.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

Implementation commit: recorded in supervisor state after commit creation.
