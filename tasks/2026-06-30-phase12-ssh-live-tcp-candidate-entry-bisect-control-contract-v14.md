# Phase 12 SSH Live TCP Candidate Entry Bisect Control Contract V14

Task id: phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14-20260630

Status: accepted after commit.

Classification: candidate-entry-bisect-control-ready.

Evidence level: no-hardware static review of accepted v12/v13/v14 task
records, non-published current-tree minimal entry-control archive metadata,
task-owned JSON evidence, docs build, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, broad shell work, or phase transition was
performed.

## Goal

Define one qualitatively different Pi 5 discriminator for the
candidate-specific selected-fetch/no-entry boundary before any new hardware
action.

## Selected Discriminator Contract

The next serialized hardware task must run the current-tree
rpi5_minimal_entry_control selected-path control and no other target. The
single variable being bracketed is the runtime path after the common selected
da591740/kernel_2712.img fetch:

- v12 failing endpoint: live TCP runtime-marker candidate, selected fetch
  proved, no retained rust_entry, boot-info-parsed, target-init,
  exceptions-ready, kernel_main, route-start, or runtime-ready markers.
- v13 passing endpoint: current-tree production-timer selected-path control,
  selected fetch proved, downstream rpi5-production-timer-preemption: PASS
  observed.
- v14 discriminator: current-tree minimal entry-control selected-path image,
  normal Pi 5 startup through kernel_main, immediate nonce-bearing
  TALOS: minimal-entry-control-ready marker, and no production-timer secondary
  route or live TCP route construction.

The hardware successor is:
phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14-20260630.

## Source And Archive Identity

- source commit for the static contract:
  54a4a718d364ffc8fee65abb5b766c6938165361.
- control helper: scripts/rpi5-minimal-entry-control-boot-tree.sh.
- archive review helper: scripts/rpi5-minimal-entry-control-archive-review.sh.
- boot source for non-published static materialization:
  target/tmp/rpi5-observed-gpio-status-known-good-tree.
- boot scenario: rpi5_minimal_entry_control.
- static review nonce: candidate-entry-bisect-v14-static.
- required hardware nonce form:
  candidate-entry-bisect-v14-<UTC timestamp>.
- selected fetch path: da591740/kernel_2712.img.
- reviewed current-tree selected kernel size: 52,840 bytes.
- reviewed current-tree selected kernel SHA-256:
  7647de5448a7c14f73441491bcb9e17be19c24551f0a9ad1d7b5ca8a8214b63a.
- reviewed non-published archive SHA-256:
  a4839ebbe33960b75b8b88206e4fa8382a4e255989eb2b1c65da4730ce1aad67.
- required marker:
  TALOS: minimal-entry-control-ready capture-nonce=<run nonce>.
- required static claim tokens include live-tcp-route=false, packet-io=false,
  openssh=false, ssh-ready=false, claims-service-success=false, and
  claims-phase-transition=false.

Generated boot/archive bytes were removed after static review. The hardware
successor must rebuild and publish its own run-unique archive and record its
own source/archive identity.

## Hardware Successor Contract

The successor must acquire hardwareTestLock before lab status reads that affect
the boot tree, publication, Pi 5 power action, or restore-affecting operation.
It must capture, in order:

1. pre-publication lab API identity and boot snapshot list;
2. run-unique archive identity and selected da591740/kernel_2712.img bytes;
3. pre-power fresh serial cursor/completeness diagnostics;
4. pre-power TFTP tail cursor;
5. one helper-owned Pi 5 power/capture window;
6. repaired stable same-cursor TFTP delta before restore;
7. final pre-restore lab identity;
8. marker classification for firmware NETWORK, rust_entry, boot-info-parsed,
   target-init, exceptions-ready, kernel_main, and the nonce-bearing minimal
   entry-control marker;
9. restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z;
10. post-restore lab API identity and hardwareTestLock release.

If capture/staging is inconclusive, the task must run the standard triage in
order before code changes: candidate/control identity, fresh serial cursor,
TFTP delta, known-good control if capture/staging is suspect, then a bounded
candidate/control rerun only when the recorded missing fact requires it.

## Fail-Closed Classifications

- candidate-entry-discriminator-passes: selected fetch identity, final
  pre-restore identity, restore proof, and nonce-bearing
  TALOS: minimal-entry-control-ready appear in the retained serial window. This
  may select the v14 candidate entry preflight only as a narrowed entry
  boundary; it must not select packet-I/O or OpenSSH directly.
- blocked-control-entry: selected fetch succeeds but the minimal control marker
  is absent.
- blocked-identity: selected fetch bytes/hash, final pre-restore identity, or
  selected mirror identity disagree.
- blocked-tftp-capture: the TFTP/log window cannot prove selected
  da591740/kernel_2712.img bytes before restore.
- blocked-restore: restore proof is missing or disagrees.
- inconclusive-with-required-discriminator: the first missing fact is named and
  no same-shaped retry is selected without supervisor planning.

Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility, service
success, ssh-ready=true, broad shell work, and phase transition remain blocked.

## Findings

- fixed: promoted exactly one mechanically unblocked no-hardware v14 bisect
  contract task.
- fixed: selected the current-tree minimal entry-control selected-path archive
  as the single hardware discriminator between the v12 live TCP no-entry
  endpoint and the v13 production-timer passing endpoint.
- fixed: recorded exact source/helper/archive identity, required marker,
  restore target, inconclusive-run triage, fail-closed classifications, and
  redaction rules for the hardware successor.
- not-an-issue: the prior minimal-entry control failure is treated as boundary
  context, not readiness evidence; this contract requires a fresh current-tree,
  run-unique helper-owned window and must fail closed if the minimal marker is
  still absent.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, and phase
  transition as permissible immediate successors.

## Evidence Map

- Task-owned evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14/evidence-map.json.
- Non-published static archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14/validation/minimal-entry-control-archive-review.stdout.txt.
- Static selected kernel metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14/static/kernel-bytes.txt and
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14/static/kernel-sha256.txt.
- Marker token review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14/static/marker-token-review.txt.

## Redaction Review

This no-hardware task retained task ids, commit ids, path labels, hashes, byte
counts, marker labels, classification strings, and validation command results.
It retained no raw serial text, raw TFTP peer/log-line fields, packet payloads,
SSH keys/session material, boot artifact bytes, private user data,
stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before promotion.
- Static review of accepted v12/v13 evidence and proposed discriminator
  metadata: pass.
- Non-published current-tree minimal entry-control archive review: pass.
- jq empty on touched JSON evidence/state files: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
