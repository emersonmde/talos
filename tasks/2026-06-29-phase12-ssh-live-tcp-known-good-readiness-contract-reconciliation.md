# Phase 12 SSH Live TCP Known-Good Readiness Contract Reconciliation

Task id: phase12-ssh-live-tcp-known-good-readiness-contract-reconciliation-20260629

Status: accepted after commit.

Classification: known-good-serial-window-discriminator-ready.

Evidence level: static task/evidence/source/docs review, known-good readiness
classifier replay over retained hardware evidence, task-owned JSON evidence,
docs build, and diff checks. No Pi 5 hardware/lab action, hardwareTestLock
acquisition, boot publication, power-cycle, candidate run/rerun, packet-I/O
discriminator, OpenSSH/generated-root retry, remote receipt, compatibility
claim, service success claim, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition was performed.

## Goal

Reconcile the accepted blocked-known-good-marker-absent result into an explicit
static/source evidence contract before any candidate retry or repeated hardware
run.

## Scope Performed

- Reviewed the accepted lab-capture contract reconciliation and the
  known-good-only capture discriminator task/evidence.
- Reviewed the v3 known-good readiness classifier and runtime-readiness
  retention/observe helpers.
- Replayed the v3 classifier over the retained known-good discriminator
  artifacts.
- Reviewed prior known-good readiness evidence where the production success
  marker was observed through the same saturated-cursor direct-read fallback.
- Updated Phase 12 docs and roadmap with the selected next discriminator.

## Contract Reconciliation

- production success marker: required and still missing for the current
  restored known-good tree. The retained 2026-06-29 known-good run proves stable
  /status identity and TFTP fetches, but the primary serial artifact lacks
  rpi5-production-timer-preemption: PASS. Prior 2026-06-10 known-good evidence
  observed that marker through the same saturated-cursor direct-read fallback,
  so this is not a static marker-name defect.
- kernel_main metadata policy: not required for v3 known-good readiness. The
  classifier correctly records TALOS: kernel_main absence as retained risk
  metadata because prior valid-marker evidence also omitted kernel_main.
- saturated serial cursor/direct-read fallback: allowed as a capture fallback,
  but not sufficient to prove serial-window completeness when the marker is
  absent. The first missing fact is whether the current known-good boot window
  failed to reach the marker or whether the saturated/direct-read path missed
  the relevant serial interval.
- same-run status/TFTP identity: repaired and accepted for the known-good-only
  run. Pre-power, final-pre-restore, and post-restore /status samples all
  reported tree 6ead8933... with effective_kernel=kernel_2712.img.
- stable kernel fetch count/bytes: accepted for the known-good-only run. The
  stable same-cursor TFTP delta retained two da591740/kernel_2712.img serves,
  both 82045 bytes.
- candidate preflight readiness: not accepted. Candidate preflight v3 remains
  blocked until a known-good serial-window completeness discriminator either
  observes the required marker under the reconciled contract or records a more
  precise first missing fact.

## Findings

- fixed: the first missing fact is now explicit: serial-window completeness for
  the current known-good boot tree, not /status identity or TFTP capture.
- fixed: candidate preflight v3 remains blocked because the retained
  known-good evidence does not satisfy the v3 production-marker gate.
- fixed: the next selected task is the queued known-good serial-window
  completeness discriminator rather than another candidate preflight.
- not-an-issue: the v3 classifier's kernel_main metadata-only policy is
  consistent with prior accepted known-good readiness evidence.
- not-an-issue: no helper/source defect was found in the v3 classifier,
  retention wrapper, or observe helper during static review.
- deferred: candidate archive publication, packet I/O, OpenSSH/generated-root
  retry, remote receipt, compatibility, service success, ssh-ready=true, and
  phase transition remain deferred.
- removed: no source, helper, task, or evidence artifact was removed.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-readiness-contract-reconciliation/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-readiness-contract-reconciliation/evidence-map.json.
- Classifier replay:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-readiness-contract-reconciliation/known-good-readiness-v3-replay.json.
- Replay summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-readiness-contract-reconciliation/replay-summary.json.
- Reviewed known-good discriminator:
  tasks/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator.md.
- Reviewed lab-capture reconciliation:
  tasks/2026-06-29-phase12-ssh-live-tcp-lab-capture-contract-reconciliation.md.
- Reviewed prior marker-positive known-good proof:
  tasks/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof.md.
- Reviewed helper/classifier source:
  scripts/rpi5-known-good-readiness-v3-classify.sh,
  scripts/rpi5-observe-runtime-readiness.sh, and
  scripts/rpi5-retain-runtime-readiness-primary.sh.

## Redaction Review

Task-owned JSON evidence records task ids, source paths, public classifier
names, tree-hash prefixes, kernel byte counts, cursor offsets, validation
commands/results, and metadata-only labels. It does not add peer identifiers,
addresses, packet payload contents, key material, session material, boot
artifact bytes, private user data, stable secret-derived identifiers, or
unnecessary hardware data.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task promotion.
- Static review of retained known-good discriminator evidence and prior
  known-good readiness boundary tasks: pass.
- sh -n scripts/rpi5-known-good-readiness-v3-classify.sh: pass.
- sh -n scripts/rpi5-observe-runtime-readiness.sh: pass.
- sh -n scripts/rpi5-retain-runtime-readiness-primary.sh: pass.
- Known-good readiness classifier replay over retained 2026-06-29 artifacts:
  pass; reproduced exit 1 with known-good-readiness-v3-blocked and only
  missing-production-success-marker.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator-20260629.

planningNeeded: false.

Candidate preflight v3 remains blocked. The next bounded worker task must prove
the known-good serial-window completeness boundary with a qualitatively
different discriminator before any candidate preflight v3, packet-I/O
discriminator, OpenSSH/generated-root retry, remote receipt, compatibility
claim, service success claim, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition.

Commit: recorded in talos-supervisor-state.json after final commit.
