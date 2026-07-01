# Phase 12 SSH Live TCP Selected Normal Runtime Kernel Main Contract Reconciliation V47

Task id: phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47-20260701

Status: accepted.

Classification: selected-normal-runtime-kernel-main-contract-reconciled-ready.

Evidence level: static archive/image inspection, task-owned JSON evidence,
capture helper dry-run, docs build, and diff checks. No hardwareTestLock
acquisition, lab publication, boot snapshot mutation, Pi 5 power cycle,
serial/TFTP capture, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.

## Goal

Reconcile the v45/v46 selected kernel SHA mismatch without hardware and produce
one internally consistent kernel_main archive contract for the next serialized
Pi 5 preflight.

## Scope Performed

- Promoted this ready no-hardware task after v46 accepted as
  blocked-selected-normal-runtime-kernel-main-preflight at commit
  b9acf845953c4aa5bc1a96392487a43058196b1c.
- Recomputed the archive SHA-256 and exact selected member
  ./da591740/kernel_2712.img SHA-256 directly from
  target/tmp/selected-normal-runtime-kernel-main-v45.tar.gz.
- Compared the recomputed member hash with v45 static evidence, v45
  classification/task/docs claims, and v46 blocker evidence.
- Corrected the stale v45 task/docs/evidence claim by preserving the stale
  hash as the v46 blocker reason and selecting the recomputed archive member
  hash as the authoritative v48 contract.
- Reviewed the existing non-published archive with the kernel_main marker-loop
  archive review and dry-ran the successor capture helper contract.

## Terminal Classification

selected-normal-runtime-kernel-main-contract-reconciled-ready.

The authoritative non-published archive contract for the next hardware
preflight is:

- Archive path: target/tmp/selected-normal-runtime-kernel-main-v45.tar.gz.
- Archive SHA-256:
  72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19.
- Selected fetch path: da591740/kernel_2712.img.
- Exact tar member used for recomputation: ./da591740/kernel_2712.img.
- Selected kernel SHA-256:
  2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5.
- Selected kernel size: 152,896 bytes.
- Required marker:
  TALOS: kernel_main capture-nonce=runtime-marker-route-static.
- Restore snapshot: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Source commit: 773dae203cbe36a0795e3a1d861587da8540e2d3.
- Successor task:
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48-20260701.

The stale SHA-256
96057d2f8970808011a308f7b3a92da6feb85097b44590947a1ac145f85c6be6 is
quarantined as a stale v45 classification/task/docs claim and as the explicit
v46 blocker gate. It is not authoritative for publication. The authoritative
contract is the archive SHA plus the recomputed hash and byte count for the
exact selected member that the Pi 5 will fetch.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48-20260701.

planningNeeded: false.

Later facts remain unproved: selected Pi 5 kernel_main marker retention,
route-start, runtime-ready, packet-I/O, OpenSSH compatibility, remote receipt,
service readiness, ssh-ready=true, fake command expansion, broad shell work,
and phase transition.

## Findings

- fixed: v45 task/docs/evidence now record the recomputed selected member hash
  as the authoritative selected kernel SHA while preserving the stale hash as
  the v46 blocker reason.
- fixed: v48 successor contract now names one archive path, archive SHA,
  selected fetch path, selected kernel SHA, byte count, marker, restore
  snapshot, source commit, and successor task id.
- not-an-issue: the non-published archive SHA, byte count, and kernel_main
  marker-loop archive review remained internally consistent.
- deferred: Pi 5 evidence that the selected archive reaches kernel_main belongs
  to the serialized v48 hardware preflight.
- removed: old paused v46 closeout, old route-start v47, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility/service
  readiness, ssh-ready=true, fake command expansion, broad shell work, and
  phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/evidence-map.json.
- Hash reconciliation matrix:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/static/hash-reconciliation-matrix.md.
- Archive SHA:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/static/archive-sha256.txt.
- Recomputed selected member SHA:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/static/recomputed-da591740-kernel-sha256.txt.
- Recomputed selected member size:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/static/recomputed-da591740-kernel-size.txt.
- Kernel_main archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/validation/kernel-main-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned evidence retains task ids, hashes, byte counts, marker names,
classifications, helper dry-run command contracts, and validation outcomes. It
does not retain raw serial text, raw TFTP peer/log-line fields, packet payloads,
SSH/session/key material, boot artifact bytes, private data, or stable
secret-derived identifiers.

## Validation

- git status --short --branch before edits/action: pass.
- jq empty on referenced and task-owned JSON evidence plus supervisor state:
  pass.
- Recompute archive SHA-256 and selected kernel SHA-256 from the same
  tarball/member path: pass; member path ./da591740/kernel_2712.img.
- Compare recomputed hashes against v45 static/kernel-sha256.txt, v45
  classification, v45 task/docs claims, and v46 classification: pass; stale
  96057d2f... was corrected/quarantined.
- scripts/rpi5-ssh-service-smoltcp-kernel-main-marker-loop-archive-review.sh:
  pass.
- Capture helper --dry-run for v48 successor: pass.
- sh -n on touched shell helpers: not run; no shell helpers were touched.
- cargo fmt/build: not run; no Rust/build source was touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
