# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime BootInfo After Rust Entry Preflight V66

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66-20260702

Status: accepted after serialized Pi 5 validation and inconclusive-run triage.

Classification: inconclusive-selected-normal-runtime-bootinfo-preflight.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, selected TFTP service, serial hardware output, known-good control,
candidate rerun, restore proof, task-owned JSON evidence, docs build, and diff
checks.

## Goal

Run the v65-selected BootInfo marker-loop archive on the Pi 5 and classify
whether the selected normal-runtime image retains the next post-rust-entry
BootInfo marker before target-init, kernel_main, route-start, runtime-ready,
packet-I/O, or OpenSSH work.

## Scope Performed

- Promoted the ready v66 hardware preflight after accepted v65 selected this
  exact task and supervisor planning added it to the queue.
- Acquired hardwareTestLock before lab publication, boot mutation, Pi 5 power
  action, serial capture, or TFTP capture.
- Published only the v65 selected normal-runtime BootInfo archive:
  target/tmp/selected-normal-runtime-bootinfo-v65.tar.gz.
- Captured a first candidate, then performed the required inconclusive-run
  triage after serial freshness rejected the first identity join.
- Ran a known-good baseline control, then republished and reran the candidate.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and released
  hardwareTestLock.

## Hardware Result

Archive: target/tmp/selected-normal-runtime-bootinfo-v65.tar.gz.

Archive SHA-256:
68a3e9356753c66b646477880f786fc10a01b021bd8758d19484f409df81ad9d.

Selected da591740/kernel_2712.img: 152,880 bytes, SHA-256
87bbaab6842cbd83c1dff548d81151af6f9ff5309236b7ba65481174560987a8.

The first candidate published the selected tree
f5b5d23af2dffc60fd61a8bb2ea5bdf9c1f433b69694fd3efdd7f51793a68632, served
da591740/kernel_2712.img twice at 152,880 bytes, and retained TALOS: boot info
parsed 184 times. It was rejected for decisive classification because stale
pre-power serial output prevented serial-freshness-v1 from proving a clean
post-power window.

The triage baseline control observed 13 TFTP events after a known-good power
cycle and restore. The candidate rerun then had decisive identity join:
selected TFTP served da591740/kernel_2712.img twice at 152,880 bytes, final
pre-restore identity remained selected, restore returned the lab to the
accepted baseline tree, and the serial window retained TALOS: boot info parsed
192 times.

The candidate rerun did not retain a separate TALOS: rust_entry line in the
same selected serial window. A wider second rerun retained TALOS: boot info
parsed 2,102 times but still did not retain TALOS: rust_entry and was rejected
for serial freshness after the BootInfo loop saturated the retained serial tail.

## Terminal Classification

inconclusive-selected-normal-runtime-bootinfo-preflight.

The Pi 5 selected archive proves selected-byte TFTP service and repeated
BootInfo marker output, but the same-window TALOS: rust_entry retention required
for selected-normal-runtime-bootinfo-marker-retained was not captured. Accepted
v64 and current source establish rust_entry as the predecessor generally; this
preflight does not upgrade that to a same-window retained v66 hardware fact.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-after-rust-entry-closeout-v66-20260702.

planningNeeded: false.

## Findings

- fixed: ran the selected BootInfo archive under hardwareTestLock with selected
  TFTP service, serial marker evidence, known-good control, candidate rerun, and
  restore proof.
- fixed: restored the lab to the accepted baseline before releasing
  hardwareTestLock.
- fixed: recorded why the first candidate was not decisive
  (serial-freshness-v1-not-proven) and followed the standard triage order before
  rerunning.
- deferred: same-window TALOS: rust_entry retention was not captured by the v66
  BootInfo candidate windows.
- deferred: target init, exceptions, kernel_main, route-start, runtime-ready,
  packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake
  command expansion, broad shell work, and phase transition remain unproved.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66/evidence-map.json.
- Static archive summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66/validation/static-archive-review-summary.json.
- First candidate summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66/lab/v66-candidate/capture-invariant-summary.json.
- Triage summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66/lab/v66-inconclusive-triage/triage-summary.json.
- Candidate rerun summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-after-rust-entry-preflight-v66/lab/v66-candidate-rerun/capture-invariant-summary.json.

## Redaction Review

Task-owned aggregate summaries retain task ids, run labels, hashes, byte
counts, cursor metadata, marker names, marker counts, selected-tree hashes,
classifications, and validation outcomes. They do not retain raw serial text,
raw TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot
artifact bytes, private data, or stable secret-derived identifiers in the task
record or classification.

## Validation

- git status --short --branch before lab publication/hardware action:
  ## main...origin/main [ahead 290].
- jq empty on supervisor state before/after hardwareTestLock changes and on
  task-owned JSON evidence: pass.
- Static archive/image review before publication: pass.
- Capture helper dry-run: pass for required marker TALOS: boot info parsed.
- Lab API candidate identity before publication, after publication,
  post-power/pre-observe, final pre-restore, and after restore: recorded.
- Fresh serial cursor/drain: recorded; first candidate rejected for
  serial-freshness-v1-not-proven, candidate rerun identity join accepted.
- GET /tftp/logs cursor delta: recorded; selected candidate rerun served
  da591740/kernel_2712.img twice at 152,880 bytes.
- Known-good control: recorded 13 TFTP events before candidate rerun.
- Restore proof to accepted baseline before releasing hardwareTestLock: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-after-rust-entry-closeout-v66-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
