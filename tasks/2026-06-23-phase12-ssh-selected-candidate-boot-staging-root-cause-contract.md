# Phase 12.6 SSH selected-candidate boot-staging root-cause contract

Task id: phase12-ssh-selected-candidate-boot-staging-root-cause-contract-20260623
Status: accepted
Owner: worker
Classification: selected-candidate-root-cause-contract

## Goal

Reframe the baseline-fetch-after-selected-publish blocker from first principles
before any further selected-candidate retry or live OpenSSH action.

## Reviewed Inputs

- memory/talos-supervisor-state.json currentTask and taskQueue entries for this
  task, retry-v3/closeout-v3, the no-power discriminator, fetch-v3, retry-v4,
  and closeout-v4.
- tasks/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-evidence-contradiction-repair.md.
- tasks/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator.md.
- tasks/2026-06-23-phase12-ssh-lab-boot-capture-preflight.md.
- tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/selected-candidate-rerun-v2.summary.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/archive-review.txt.
- tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/candidate-rerun-post-publish-status.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/candidate-rerun-post-publish-boot-files.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/candidate-rerun-tftp-delta.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/candidate-rerun-final-pre-restore-status.sanitized.json.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- scripts and source references found by static search for boot/files, TFTP,
  effective_kernel, tree_hash, kernel_2712, snapshot, restore, publish, and
  baseline-fetch wording.

## First-Principles Problem Statement

The live OpenSSH discriminator is blocked on a more basic boot identity
precondition: after publishing a selected Talos candidate archive, the Pi 5 lab
must prove that the same selected kernel_2712.img is visible through the lab API
and served through the TFTP path before any runtime/OpenSSH observation can be
trusted. Rerun-v2 contradicted that precondition. The lab API reported the
selected post-publish tree, but the same-run TFTP fetches and final pre-restore
identity matched the restored baseline/control tree instead.

The problem to solve is not whether OpenSSH can connect. The current named
unknown is which layer broke the selected archive publication to actual served
boot identity chain.

## Expected Invariant

For one selected archive and one serialized task-owned lab window, this invariant
must hold unless the task records an explicit restore after capture:

1. The selected archive's root and da591740/kernel_2712.img are the expected
   public size/hash category.
2. PUT /boot/archive publishes that archive successfully.
3. GET /status reports the selected tree_hash, configured_kernel, and
   effective_kernel before any power-cycle.
4. GET /boot/files exposes the same selected boot tree and selected
   da591740/kernel_2712.img size/category before any power-cycle.
5. A fresh TFTP delta after the power-cycle serves da591740/kernel_2712.img
   bytes matching the selected archive, or an explicitly equivalent public
   lab-controller signal proves the served root.
6. The final pre-restore status still reports the selected tree until the task
   performs the recorded restore.
7. The restore returns status and boot/files identity to the pre-run baseline.

If any of these observations disagree, live runtime, TCP, or OpenSSH evidence is
secondary and must not be used as progress until the first failing invariant is
isolated.

## Contradicting Evidence

- The accepted contradiction repair quarantined the earlier selected-candidate
  task because retained JSON showed capture-chain-inconclusive,
  selected_candidate_fetch_observed=false, selected_next_task=null, two
  104,136-byte da591740/kernel_2712.img fetches, and final pre-restore identity
  on the baseline/control tree.
- Rerun-v2 reused the reviewed archive
  target/phase12-ssh-live-openssh-retry-boot.tar.gz with sha256
  2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01 and
  87,432-byte kernel_2712.img entries.
- Rerun-v2 candidate publication reported selected tree
  fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333 with
  effective_kernel=kernel_2712.img and boot/files showing 87,432-byte
  da591740/kernel_2712.img before power.
- Rerun-v2 same-root TFTP evidence before restore then served
  da591740/kernel_2712.img twice at 104,136 bytes, the baseline/control size,
  not the selected 87,432-byte candidate size.
- Rerun-v2 final pre-restore status was already the baseline/control tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, before the
  task's recorded final restore.

These observations make retry-v3 and closeout-v3 unsafe to promote. They depend
on selected-candidate-fetch-observed=true, and rerun-v2 accepted the opposite.

## Unproven Assumptions By Layer

- archive contents: the retained archive review proves public size/category, but
  not that every future helper will publish that exact archive without
  replacement.
- archive extraction: PUT /boot/archive success and boot/files visibility imply
  extraction, but the served TFTP path still needs a no-power same-root check
  before power-cycle side effects are introduced.
- status tree_hash/effective_kernel: status reports the visible lab API boot
  tree and config parsing, but the rerun-v2 contradiction proves status alone is
  not enough to accept actual served bytes.
- boot/files root visibility: boot/files showed the selected candidate before
  power in rerun-v2, but the next discriminator must verify this immediately and
  treat any status/files mismatch as publish-root-mismatch.
- TFTP server root: TFTP logs are the authoritative same-run served-byte signal
  for Pi firmware fetches, but the selected post-publish boot/files tree has not
  yet been proved to be the same root observed by dnsmasq after publication.
- TFTP log byte-size attribution: sanitized events keep filename/status/byte
  counts, which is sufficient for public kernel size categories but not for
  full byte identity unless paired with archive/boot-files hashes or an
  equivalent served-root signal.
- snapshot/restore semantics: restore proof returned the baseline tree, but
  rerun-v2 final pre-restore identity unexpectedly matched baseline, so the next
  task must record restore timing and reject any pre-restore baseline transition.
- power-cycle side effects: the mismatch may occur only after power-cycle, so
  the first follow-up must remove power from the discriminator.
- helper sequencing: prior helpers may have overwritten pre-restore evidence in
  the initial rerun-v2 attempt, so follow-ups must capture post-publish
  status/files before power, then TFTP/final identity before restore, in that
  order.

## Follow-Up Approaches

Approach A, no-power publish/root discriminator:

- Acquire hardwareTestLock only for lab boot publication/restore, with no
  power-cycle and no OpenSSH.
- Publish the selected archive, then immediately compare status tree_hash,
  effective_kernel, boot/files kernel_2712.img size/category, and TFTP tail
  cursor context before restore.
- Accept selected-root-visible=true only if status and boot/files both expose
  the selected tree/kernel before restore. If they disagree, classify
  publish-root-mismatch or lab-api-visible-root-divergence and stop before any
  hardware retry.

Approach B, hardware-backed selected-fetch discriminator:

- After a no-power root discriminator has accepted selected-root-visible=true,
  acquire hardwareTestLock, publish the selected archive, power-cycle once, and
  require a fresh same-run TFTP delta proving the 87,432-byte selected
  da591740/kernel_2712.img fetch before restore.
- Accept selected-candidate-fetch-observed=true only from that fresh served-byte
  proof or an equivalent public lab-controller served-root signal. If the
  TFTP/final identity still falls back to baseline, fail closed with the first
  failing invariant.

Approach C, helper/source quarantine:

- If Approach A fails, quarantine the affected publication/root helper path
  instead of stacking another retry. The next supervisor task should inspect or
  repair lab-service publication/root semantics with explicit operator or
  deployed-service evidence before any hardware-backed selected fetch is
  promoted.

## Smallest Decisive Discriminator

The selected next discriminator is the no-power publish/root discriminator:
phase12-ssh-selected-candidate-no-power-publish-root-discriminator-20260623.

It is the smallest decisive step because rerun-v2 already proved that hardware
TFTP/final identity can contradict post-publish status. Removing the
power-cycle separates the publication/root visibility layer from firmware fetch
and runtime behavior. If status and boot/files do not agree immediately after
publish, no hardware retry is justified. If they do agree, the hardware-backed
selected-fetch discriminator can test only the remaining power/TFTP path.

## Workaround Removal And Quarantine Plan

- retry-v3 and closeout-v3 remain blocked/superseded and must not be promoted
  from rerun-v2.
- live OpenSSH retry-v4 remains dependency-gated behind an accepted
  selected-candidate-fetch-after-root-cause-v3 task with
  selected-candidate-fetch-observed=true.
- Prior selected-candidate acceptance prose is superseded by retained JSON and
  contradiction repair; it must not unblock live-client work.
- If the no-power discriminator fails, supervisor planning is required before
  another helper or hardware retry. The next plan should either repair the
  lab-service publication/root contract or explicitly quarantine the helper path
  that produced the mismatch.
- If the no-power discriminator passes, the next task may promote only the
  hardware-backed selected-fetch-v3 task already queued by the supervisor.

## Findings And Disposition

- fixed: stated the first-principles selected archive publication to served
  kernel identity invariant and named the current unknown as the publication/
  root/TFTP identity layer, not OpenSSH behavior.
- fixed: selected the no-power publish/root discriminator as the smallest
  decisive next task and preserved selected-fetch-v3 as the follow-up only if
  selected-root-visible=true is accepted.
- fixed: retained retry-v3 and closeout-v3 as blocked/superseded; no live
  OpenSSH task is selected from rerun-v2.
- deferred: hardware-backed selected-fetch-v3 and live OpenSSH retry-v4 remain
  dependency-gated behind the no-power discriminator and fresh selected fetch
  proof.
- not-an-issue: no Talos runtime source change was needed; this task only
  reconciles the boot-staging evidence contract.
- not-an-issue: docs/src/project/lab-controller.md already documents status,
  boot/files, TFTP logs, snapshot, and restore semantics sufficiently for the
  selected no-power discriminator; no lab API contract wording changed.

## Validation

- static task/docs/source/evidence review: pass.
- jq empty on memory/talos-supervisor-state.json: pass.
- jq empty on task-owned JSON evidence: conditional skip, no JSON evidence
  created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static task/docs/source/evidence review, JSON syntax check,
docs build, and diff checks.

## Redaction Review

Pass. Durable evidence and docs retain only task ids, file paths, public archive
hashes, public tree hashes, kernel size categories, TFTP event byte categories,
validation commands, and classifications. No raw serial text, raw TFTP lines,
client identities, user names, addresses, MAC addresses, host keys, authorized
keys, fingerprints, signatures, session identifiers, command bytes, packet
captures, boot artifact bytes, stable peer identifiers, or private user data are
retained.

## Acceptance

Accepted as selected-candidate-root-cause-contract.

selected_next_task=phase12-ssh-selected-candidate-no-power-publish-root-discriminator-20260623.

planningNeeded=false.

No boot archive publication, restore, snapshot mutation, power-cycle,
serial/TFTP live capture, OpenSSH execution, TCP reachability, remote receipt,
compatibility, PTY/SCP/SFTP, broad command expansion, phase transition, or
ssh-ready=true is accepted.
