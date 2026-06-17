# Phase 10 Pi 5 Command0 TFTP Selected-Kernel Precondition Source Contract

Task id: phase10-pi5-command0-tftp-selected-kernel-precondition-source-contract-20260617

Status: accepted

Classification:
command0-tftp-selected-kernel-precondition-source-contract-core-selected

Evidence level: static/source/task evidence inspection, accepted command0
write-delivery source/guard/proof/closeout evidence, task-owned JSON evidence,
docs build, and diff checks. No implementation work, hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, command-loop feature
change, storage, networking, SSH, Phase 11/12 expansion, or phase transition was
performed.

## Goal

Define the smallest source/static reconciliation for the selected-tree/TFTP-
served kernel mismatch that blocks command0 write-delivery evidence.

## First Failing Invariant

The first failing invariant is selected-tree/TFTP-served-kernel agreement, not
command-loop behavior.

After a candidate boot archive is selected and published, and before command0
behavior evidence is evaluated, the proof must show that the same power cycle
served the selected kernel bytes through TFTP. For the accepted blocked
command0 write-delivery proof, the selected candidate archive expected:

- archive SHA-256:
  8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c;
- kernel_2712.img SHA-256:
  c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd;
- kernel_2712.img byte count: 208984;
- initramfs_2712 byte count: 662.

The retained lab API status after candidate publication listed
kernel_2712.img and da591740/kernel_2712.img at 208984 bytes with
effective_kernel=kernel_2712.img. The same-cursor TFTP requery, however,
recorded da591740/kernel_2712.img served at 104136 bytes twice. After
known-good control and candidate rerun, the rerun again recorded 104136-byte
kernel_2712.img serves, while the expected 208984-byte selected candidate fetch
was absent. That makes command0 write delivery non-evaluable because the proof
cannot tie the serial behavior to the selected command0 candidate.

## Compared Approaches

### Publication/Served-Root Preflight Guard

Implement a local/static preflight guard in the proof-helper surface that
compares the selected archive/kernel identity against retained lab evidence
before any command0 behavior claim is accepted. The guard should reject:

- no fresh TFTP delta after candidate publication;
- baseline-sized TFTP fetches under candidate identity;
- TFTP deltas missing da591740/kernel_2712.img;
- final pre-restore identity that no longer contains the selected kernel;
- restore failure or missing restore proof;
- stale serial-only evidence when selected-kernel/TFTP agreement is absent.

This is the selected approach because it is directly implementable in the Talos
repo, it can be validated against retained negative evidence, and it prevents a
same-shaped command0 behavior retry from accepting output produced by a
different served kernel.

### Lab/Boot-Root Reconciliation Path

Investigate why lab API-visible /boot-root files reported 208984-byte selected
kernel entries while dnsmasq served 104136-byte baseline bytes from
/var/tftpboot/da591740/kernel_2712.img. Possible layers include archive
publication, prefix/root mirroring, dnsmasq served-root/cache behavior,
snapshot/restore ordering, and lab API status reporting.

This path is not selected as the immediate worker-owned follow-up because it
may require lab-service changes or operator inspection outside the Talos repo.
It remains the right escalation if the local/static preflight guard cannot
express the mismatch, or if a future Pi 5 precondition proof shows the mismatch
persists despite a correct preflight contract.

## Rejected Retry Shapes

The following are rejected until selected-tree/TFTP-served-kernel agreement is
proven:

- timing-only, wait-count, marker-name, or cursor-only command0 retries;
- command-loop source changes aimed at a run whose served kernel is not proven;
- source-response retention retries;
- acceptance based on prompt, /serial/write, ready command=1, or serial output
  alone when TFTP served baseline-sized kernel bytes;
- treating lab API-visible boot files as sufficient without same-power-cycle
  TFTP byte agreement.

## Selected Follow-Up Surface

The selected dependency-gated follow-up is
phase10-pi5-command0-tftp-selected-kernel-precondition-core-20260617.

That task may edit only surfaces directly needed to make the precondition
locally/static-checkable before a hardware proof:

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh or a
  directly paired proof-helper/preflight validator;
- task-owned positive and negative fixtures for selected-kernel/TFTP agreement;
- task-owned evidence under
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-core/;
- the core task record;
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md,
  docs/src/project/lab-controller.md, and docs/src/roadmap.md only if the
  evidence contract changes.

The core task must preserve command0-write-delivery-guard-v1 and
command0-source-response-retention-guard-v2 as later transaction gates. A
selected-kernel/TFTP precondition pass must not by itself accept command0 write
delivery, command0 source-response retention, or generated-root command-input
success.

## Findings

- fixed: restated the first failing invariant as selected-tree/TFTP-served-
  kernel mismatch rather than command-loop behavior.
- fixed: recorded accepted evidence references for the 208984-byte expected
  candidate kernel and repeated 104136-byte baseline TFTP serves across the
  first candidate, known-good control, and candidate rerun.
- fixed: compared a proof-helper preflight guard with a lab/boot-root
  reconciliation path and selected the guard as the next repo-owned bounded
  task.
- deferred: lab-service served-root/cache diagnosis remains deferred until a
  guard or precondition proof requires that escalation.
- rejected: same-shaped timing, wait-count, marker-name, cursor-only, command-
  loop, source-response-retention, storage, networking, SSH, Phase 11/12
  expansion, and phase-transition retries before selected-kernel/TFTP agreement
  is proven.
- not-an-issue: no hardware lock, boot publication, lab mutation, or code change
  was required because this is a source/static contract.

## Evidence

- Write-delivery source contract:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-source-contract.md.
- Write-delivery guard core:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-guard-core.md.
- Write-delivery Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof.md.
- Write-delivery Pi 5 proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/classification.json.
- Write-delivery Pi 5 proof evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/evidence-map.json.
- First candidate archive review:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/candidate-command0-write-delivery-20260617T103448Z/archive-static-review.json.
- First candidate post-publish status:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/candidate-command0-write-delivery-20260617T103448Z/post-publish-status.json.
- First candidate TFTP requery:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/candidate-command0-write-delivery-20260617T103448Z/tftp-delta-after-restore-requery.json.
- Known-good control TFTP:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/triage-control-and-rerun-20260617T103840Z/known-good-control/tftp-delta-stable.json.
- Candidate rerun archive review:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/triage-control-and-rerun-20260617T103840Z/candidate-rerun/archive-static-review.json.
- Candidate rerun post-publish status:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/triage-control-and-rerun-20260617T103840Z/candidate-rerun/post-publish-status.json.
- Candidate rerun TFTP requery:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/triage-control-and-rerun-20260617T103840Z/candidate-rerun/tftp-delta-after-restore-requery.json.
- Write-delivery closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-closeout.md.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-source-contract/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-source-contract/evidence-map.json.

## Acceptance Check

- First failing invariant is stated as selected-tree/TFTP-served-kernel
  mismatch, not command-loop behavior: satisfied.
- Accepted evidence references record expected candidate byte count 208984 and
  observed baseline byte count 104136 across first candidate and rerun:
  satisfied.
- At least two qualitatively different approaches are compared, including a
  publication/served-root preflight guard and lab/boot-root reconciliation:
  satisfied.
- Same-shaped timing, wait-count, marker-name, cursor-only, or command-loop
  retries are rejected until selected-tree/TFTP-served-kernel agreement is
  proven: satisfied.
- selected_next_task is
  phase10-pi5-command0-tftp-selected-kernel-precondition-core-20260617:
  satisfied.
- Rejected claims include command0 write-delivery success, command0
  source-response retention success, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition:
  satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-tftp-selected-kernel-precondition-core-20260617 on
the next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not run hardware or retry command0 behavior directly from this
source/static contract.
