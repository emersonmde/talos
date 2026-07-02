# Phase 12 SSH Live TCP Selected Normal Runtime BootInfo Rust Entry Lineage Reconciliation V68

Task id: phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-rust-entry-lineage-reconciliation-v68-20260702

Status: accepted after no-hardware lineage reconciliation.

Classification: selected-normal-runtime-bootinfo-frontier-proved-by-lineage.

Evidence level: accepted v64 rust_entry Pi 5 proof, accepted v65 selected
BootInfo archive/source contract, accepted v66 selected TFTP and BootInfo
hardware evidence, static source/control-flow inspection, task-owned JSON
evidence, docs build, and diff checks. No hardware action, hardwareTestLock
acquisition, lab publication, boot snapshot mutation, Pi 5 power cycle, target
init proof, exceptions proof, kernel_main proof, route-start/runtime-ready
proof, packet-I/O implementation, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.

## Goal

Resolve the v66 BootInfo closeout by checking whether retained selected
BootInfo marker evidence plus source/archive lineage can prove the selected
normal-runtime BootInfo frontier even though the decisive v66 serial window did
not retain a separate same-window TALOS: rust_entry line.

## Scope Performed

- Promoted the ready no-hardware v68 task after the supervisor resolved
  planningNeeded from the inconclusive v66 closeout.
- Re-read accepted v64, v65, and v66 task-owned JSON evidence and task records.
- Reviewed the current source path for
  rpi5_ssh_service_smoltcp_bootinfo_marker_loop in src/main.rs and
  src/target/rpi5.rs.
- Preserved v66's selected-byte hardware facts: the v65 BootInfo archive was
  published, da591740/kernel_2712.img was served twice at 152,880 bytes in the
  decisive candidate rerun, final pre-restore identity stayed selected, the
  lab restored to the accepted baseline, and TALOS: boot info parsed was
  retained 192 times.
- Treated the absent same-window TALOS: rust_entry line as a marker-capture
  limitation for v66, not as a control-flow contradiction, because the selected
  BootInfo marker-loop source can only emit its loop marker after entering
  rust_entry and parsing BootInfo.

## Lineage Proof

v64 independently proves selected-byte execution can reach rust_entry on the Pi
5: the selected rust_entry archive was served by TFTP and retained TALOS:
rust_entry 208 times.

v65 then materialized the selected BootInfo marker-loop archive from current
source. The archive review recorded SHA-256
68a3e9356753c66b646477880f786fc10a01b021bd8758d19484f409df81ad9d, selected
kernel SHA-256
87bbaab6842cbd83c1dff548d81151af6f9ff5309236b7ba65481174560987a8, selected
kernel size 152,880 bytes, and embedded the required marker family with
TALOS: boot info parsed.

Current source proves the ordering for that selected scenario. The
talos_boot_scenario=rpi5_ssh_service_smoltcp_bootinfo_marker_loop path is not
one of the early diagnostic returns excluded from the normal rust_entry body.
It executes inside rust_entry, writes the RustEntry early-phase line, calls
BootInfo::from_aarch64_x0(dtb_pa), writes the BootInfoParsed early-phase line,
and only then calls run_ssh_service_smoltcp_bootinfo_marker_loop(). That loop
is the only source location that emits the selected
TALOS: boot info parsed marker and its negative claims for target init,
exceptions, kernel_main, route-start, runtime-ready, packet-I/O, service
success, ssh-ready, and phase transition.

v66 proves the selected BootInfo archive ran on Pi 5 with selected TFTP byte
service and retained TALOS: boot info parsed 192 times in the decisive
candidate rerun. Since the source/archive path cannot reach that marker without
entering rust_entry and parsing BootInfo first, the retained BootInfo marker is
accepted as a selected normal-runtime BootInfo frontier by lineage. The v66
same-window TALOS: rust_entry absence remains recorded as a capture limitation
of the v66 marker-family contract, not a blocker for the lineage proof.

## Terminal Classification

selected-normal-runtime-bootinfo-frontier-proved-by-lineage.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67-20260702.

planningNeeded: false.

This task does not prove target init, exceptions, kernel_main, route-start,
runtime-ready, packet-I/O, OpenSSH/generated-root behavior, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, or any phase transition.

## Findings

- fixed: reconciled v64 rust_entry proof, v65 selected BootInfo source/archive
  contract, and v66 selected BootInfo hardware evidence into a lineage proof.
- fixed: identified that the v66 absence of a same-window TALOS: rust_entry
  line is not a source/control-flow contradiction because TALOS: boot info
  parsed is emitted only from the rust_entry-owned BootInfo marker-loop path.
- fixed: selected the already queued v67 post-BootInfo continuation
  reconciliation and set planningNeeded=false.
- not-an-issue: no source change was required; the current source already
  orders rust_entry -> BootInfo::from_aarch64_x0 -> BootInfo marker loop for
  the selected scenario.
- deferred: target init, exceptions, kernel_main, route-start/runtime-ready,
  packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake
  command expansion, broad shell work, and phase transition remain unproved.
- removed: hardware retry as an immediate requirement for this task; the
  lineage proof uses retained hardware evidence and static source/archive
  ordering only.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-rust-entry-lineage-reconciliation-v68/classification.json.
- Lineage matrix:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-rust-entry-lineage-reconciliation-v68/lineage-matrix.json.
- Static lineage review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-rust-entry-lineage-reconciliation-v68/static/source-lineage-review.md.

## Redaction Review

Task-owned evidence retains task ids, source paths, hashes, byte counts, marker
names, marker counts, selected-tree hashes, classifications, and validation
outcomes. It does not retain raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private data,
stable secret-derived identifiers, public-key blobs, signatures, fingerprints,
operator identities, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before promotion.
- jq empty on v64/v65/v66 task-owned JSON evidence and supervisor state: pass.
- Retained evidence review: pass; v64 rust_entry proof, v65 selected BootInfo
  archive/source contract, v66 selected identity/TFTP/BootInfo counts, and
  restore proof are recorded.
- Static source/archive lineage review: pass; source ordering proves the
  selected BootInfo marker cannot be emitted without entering rust_entry and
  parsing BootInfo.
- Finding dispositions: recorded.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-post-bootinfo-continuation-reconciliation-v67-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
