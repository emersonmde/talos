# Phase 10 Pi 5 Serial Command 0 Write-Delivery Pi 5 Proof

Task id: phase10-pi5-serial-command0-write-delivery-pi5-proof-20260617

Status: accepted

Classification:
command0-write-delivery-blocked-tftp-served-kernel-mismatch-after-control-rerun

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, same-cursor TFTP log evidence, serial direct-read hardware
output, known-good control triage, candidate rerun, restore proof,
task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the serialized Pi 5 proof selected by the command 0 write-delivery guard
core and determine whether rootinfo written through /serial/write reaches the
command 0 transaction after same-boot generated-root readiness.

## Result

The proof did not accept command 0 write delivery. The first candidate retained
same-boot firmware-initramfs valid-artifact readiness, ready command=0, a
visible prompt, and an accepted 9-byte /serial/write for rootinfo. Its
post-write direct-read retained ready command=1, but did not retain rootinfo,
the command 0 line marker, dispatch command=0 status=handled, or responses=1.

That first candidate was not accepted as decisive command-delivery evidence
because the same-cursor TFTP log requery showed kernel_2712.img fetches of
104136 bytes, not the selected candidate's expected 208984-byte kernel. The
worker ran the required inconclusive-run triage: restored known-good control,
then republished and reran the candidate. The rerun again showed baseline-sized
104136-byte kernel_2712.img TFTP fetches instead of the expected 208984-byte
candidate fetch, so command0 write delivery remains blocked at the
TFTP-served selected-tree precondition rather than at a command-loop behavior
claim.

The boot tree was restored after the rerun.

## Findings

- fixed: hardwareTestLock acquisition, candidate publication records,
  archive/static review, final identity checks, and restore evidence were
  retained.
- fixed: the first candidate retained the command0 readiness/write attempt but
  did not retain enough ordered command0 write-delivery evidence to accept the
  guard.
- blocked: same-cursor TFTP deltas after candidate publication and after
  known-good-control/candidate-rerun triage retained baseline-sized 104136-byte
  kernel_2712.img fetches, not the expected 208984-byte selected candidate
  fetch.
- deferred: command0 write-delivery behavior remains non-evaluable until the
  TFTP-served selected-tree precondition is reconciled.
- rejected: command0 write-delivery success, command0 source-response
  retention success, generated-root command-input success, storage,
  networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/evidence-map.json.
- First candidate:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/candidate-command0-write-delivery-20260617T103448Z/.
- Known-good control and candidate rerun:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof/triage-control-and-rerun-20260617T103840Z/.

## Acceptance Check

- Accepted write-delivery proof retains ready command=0, prompt, fresh
  pre-write boundary, accepted 9-byte rootinfo write, command 0 line/dispatch/
  responses/ready evidence, final identity, and restore evidence: not
  satisfied; the selected-tree TFTP precondition failed.
- Blocked proof records the first failing invariant and does not claim
  generated-root command-input success: satisfied as same-cursor TFTP-served
  selected-tree mismatch after known-good-control/candidate-rerun triage.
- Any inconclusive run triggers candidate identity, fresh serial cursor, TFTP
  delta, known-good control, and candidate rerun triage before code changes:
  satisfied.
- hardwareTestLock is released and restored=true with final identity and
  restore evidence: satisfied.
- selected_next_task=phase10-pi5-serial-command0-write-delivery-closeout-20260617
  after accepted or blocked proof: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked with retained
  selected-tree TFTP mismatch evidence.
- lab API candidate identity before power: pass for API-visible post-publish
  tree identity.
- fresh serial cursor/readiness record: pass on first candidate, but not
  accepted as command-delivery evidence because TFTP-served bytes mismatched.
- GET /tftp/logs delta: blocked; first candidate and rerun retained 104136-byte
  kernel_2712.img fetches instead of the expected 208984-byte candidate fetch.
- known-good control before candidate rerun: pass.
- boot restore proof and hardwareTestLock restored=true: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-write-delivery-closeout-20260617 on the
next worker wake if dependencies remain satisfied, the repository is clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
Do not infer command0 write-delivery success, generated-root command-input
success, or transition to Phase 11/12 from this blocked proof.
