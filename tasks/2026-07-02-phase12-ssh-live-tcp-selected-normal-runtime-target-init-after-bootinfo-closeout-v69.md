# Phase 12 SSH Live TCP Selected Normal Runtime Target Init After BootInfo Closeout V69

Task id: phase12-ssh-live-tcp-selected-normal-runtime-target-init-after-bootinfo-closeout-v69-20260702

Status: accepted after no-hardware evidence reconciliation.

Classification: selected-normal-runtime-target-init-frontier-proved.

Evidence level: accepted v68 BootInfo lineage proof, accepted v67 target-init
archive/source contract, accepted v69 serialized Pi 5 hardware preflight
evidence, static source/control-flow inspection, task-owned JSON evidence, docs
build, and diff checks. No hardware action, hardwareTestLock acquisition, lab
publication, boot snapshot mutation, Pi 5 power cycle, serial capture, TFTP
capture, route-start proof, runtime-ready proof, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the v69 target-init preflight evidence and decide whether the
selected normal-runtime frontier can advance through target init without
shrinking acceptance toward later networking or shell behavior.

## Scope Performed

- Promoted the queued no-hardware v69 closeout after the accepted v69 preflight
  selected this exact task.
- Reviewed the v69 task record, classification JSON, evidence map, first run
  summary, inconclusive triage summary, selected TFTP facts, serial marker
  counts, and restore proof.
- Reviewed the current selected target-init marker-loop source in
  `src/main.rs` and `src/target/rpi5.rs`.
- Preserved v69 selected-byte facts: the v67 target-init archive was served
  from selected tree
  `3a87fb0afcb024cd6cec78652e42935ce276f95471de525f697611c2bc8f4cf1`, and
  `da591740/kernel_2712.img` was served twice at 152,880 bytes in both the
  first selected run and the candidate rerun.
- Preserved v69 marker facts: the first selected run retained 1,978
  `TALOS: target init` occurrences; the candidate rerun retained 382
  `TALOS: target init` occurrences; both retained zero literal
  `TALOS: boot info parsed` occurrences in the same selected serial windows.
- Stopped before exceptions, kernel_main, route-start, runtime-ready,
  packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, or phase transition.

## Lineage Proof

v68 proves the selected normal-runtime BootInfo frontier by lineage. v67 then
materialized the selected target-init marker-loop archive from current source:
archive SHA-256
`18270d2ca0bef45c72898beaa55971b48d748f3a87a767556074423821f17352`, selected
`da591740/kernel_2712.img` size 152,880 bytes, and selected kernel SHA-256
`4513bd97689673f904a849b60aee0377d6ddcc813ad0d00a18e422b3cc52ef82`.

Current source proves the ordering for that selected scenario. The
`talos_boot_scenario=rpi5_ssh_service_smoltcp_target_init_marker_loop` path is
not one of the early diagnostic returns excluded from the normal `rust_entry`
body. It writes the RustEntry early-phase line, calls
`BootInfo::from_aarch64_x0(dtb_pa)`, writes the BootInfoParsed early-phase
line, calls `target::init(&boot_info)`, writes the TargetInit early-phase
line, and only then calls
`target::rpi5::run_ssh_service_smoltcp_target_init_marker_loop()`. That loop
is the selected source location that emits `TALOS: target init` with
`claims-bootinfo-parsed=true` and negative claims for exceptions ready,
kernel_main, route-start, runtime-ready, packet-I/O, service success,
ssh-ready, and phase transition.

v69 proves the selected target-init archive ran on Pi 5 with selected TFTP byte
service and retained `TALOS: target init` in two selected windows. The literal
`TALOS: boot info parsed` absence in those same v69 serial windows is accepted
as a marker-family capture limitation, not a blocker for the lineage proof,
because the selected target-init marker cannot be emitted before BootInfo
parsing and `target::init(&boot_info)` return on the current source path.

## Terminal Classification

selected-normal-runtime-target-init-frontier-proved.

selected_next_task: null.

planningNeeded: true.

The frontier advances only through selected normal-runtime target init.
Exceptions ready, kernel_main, route-start, runtime-ready, packet-I/O,
OpenSSH/generated-root behavior, remote receipt, compatibility/service
readiness, ssh-ready=true, fake command expansion, broad shell work, and phase
transition remain unproved. The supervisor must instantiate the next explicit
bounded task before further worker progress because taskQueue has no successor
with current dependencies.

## Findings

- fixed: reconciled v68 BootInfo lineage proof, v67 selected target-init
  source/archive contract, and v69 selected TFTP plus target-init hardware
  evidence into a target-init frontier proof.
- fixed: identified the absent same-window literal `TALOS: boot info parsed`
  line as a v69 marker-family capture limitation rather than a source/control
  contradiction.
- fixed: preserved restore proof from v69: the lab returned to
  `phase12-ssh-v10-openssh-clean-pre-20260624T074100Z` with tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- not-an-issue: no additional hardware action is needed for this closeout; it
  is a reconciliation task over already accepted v69 evidence.
- deferred: exceptions, kernel_main, route-start, runtime-ready, packet-I/O,
  OpenSSH compatibility, service readiness, ssh-ready=true, fake command
  expansion, broad shell work, and phase transition remain unproved.
- removed: any immediate route-start/runtime-ready/OpenSSH successor from this
  wake; no explicit queued task exists for the next boundary.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-target-init-after-bootinfo-closeout-v69/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-target-init-after-bootinfo-closeout-v69/evidence-map.json.
- Closeout summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-target-init-after-bootinfo-closeout-v69/target-init-closeout-summary.json.
- Static source lineage review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-target-init-after-bootinfo-closeout-v69/static/source-lineage-review.md.
- Accepted v69 preflight:
  tasks/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-after-bootinfo-preflight-v69.md.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, public-key blobs, signatures,
fingerprints, operator identities, or unnecessary hardware data. It references
task-owned hardware summaries retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on supervisor state and task-owned JSON evidence: pass.
- Retained evidence review: pass; v69 selected identity, selected TFTP service,
  serial marker counts, inconclusive triage, and restore proof are recorded.
- Static source lineage review: pass; source ordering proves the selected
  target-init marker cannot be emitted without BootInfo parsing and
  target::init return.
- Finding dispositions: recorded.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
