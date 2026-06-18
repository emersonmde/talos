# Phase 10 Pi 5 Command0 Final Identity Regression Pi 5 Proof

Task id: phase10-pi5-command0-final-identity-regression-pi5-proof-20260618

Status: accepted

Classification:
command0-final-identity-command0-delivery-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, stable same-power-cycle TFTP delta, direct-read serial
capture after saturated cursor, immediate post-command identity sample,
final pre-restore identity sample, task-owned JSON evidence, restore proof,
and diff checks.

## Goal

Run one serialized Pi 5 discriminator proof for the command0 final
pre-restore identity regression, sampling lab identity immediately after the
command write/read window and before any restore or cleanup side effect.

## Result

The generated-root command-input candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with archive SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c. Its
da591740/kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
byte count was 208984.

Post-publish status reported selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 and
effective kernel kernel_2712.img. The stable same-power-cycle TFTP delta
retained two 208984-byte da591740/kernel_2712.img serves, so the
selected-kernel/TFTP precondition passed.

The final-identity regression did not reproduce in this run. Immediate
post-command status and final pre-restore status both remained on selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 with the
208984-byte selected kernel. The explicit restore returned the lab to baseline
tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
before hardwareTestLock release.

Command0 input delivery remains unaccepted. The serial cursor was saturated, so
the readiness helper used direct /serial/read and retained the ready command=0
marker, but did not retain TALOS: kernel_main in that window and continued
waiting until command-loop input timeouts advanced the prompt. The subsequent
rootinfo write was accepted with 9 bytes and reached command=4, not command=0:
the prearmed read retained rootinfo, dispatch command=4 status=handled, and
ready-for-next/final FAIL output. That does not satisfy the ordered command0
serial-delivery gate selected by the discriminator.

## Findings

- fixed: hardwareTestLock was acquired before boot archive publication and
  released only after post-restore baseline status evidence.
- fixed: immediate post-command lab identity was sampled before restore or
  cleanup ambiguity.
- fixed: the prior final-pre-restore baseline identity regression was
  discriminated from this run; immediate and final pre-restore identity stayed
  selected.
- blocked: ordered command0 serial delivery was not reproduced because rootinfo
  reached command=4 after readiness waiting/input timeouts, not command=0.
- not-an-issue: known-good/control was not run because the candidate produced a
  terminal blocked classification rather than capture/staging inconclusive
  evidence.
- rejected: command0 input delivery acceptance, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Evidence

- Accepted discriminator core:
  tasks/2026-06-18-phase10-pi5-command0-final-identity-regression-discriminator-core.md.
- Task classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/evidence-map.json.
- Proof evidence:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/final-identity-regression-proof-evidence.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/tftp/tftp-delta-stable-pre-command.json.
- Readiness capture:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/serial/readiness-summary.json.
- Prearmed serial read:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/serial/command0-prearmed-read.json.
- Immediate post-command identity:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/immediate-post-command-status.json.
- Final pre-restore identity:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/final-pre-restore-status.json.
- Restore proof:
  tasks/evidence/2026-06-18-phase10-pi5-command0-final-identity-regression-pi5-proof/candidate-final-identity-regression-20260618T012900Z/restore/post-restore-status.json.

## Acceptance Check

- HardwareTestLock is acquired before lab mutation and released only after
  restore evidence shows baseline restored: satisfied.
- Candidate identity, fresh serial cursor, TFTP delta, and known-good control
  decision are recorded before candidate execution: satisfied for candidate
  identity/serial/TFTP; control was recorded not required because the run ended
  in a terminal blocked classification, not inconclusive capture/staging.
- Proof captures whether final selected identity remains stable immediately
  after ordered command0 delivery and before explicit restore: partially
  satisfied; identity was captured immediately after the command write/read
  window and stayed selected, but ordered command0 delivery did not pass.
- Proof produces exactly one terminal classification from the accepted core
  contract: satisfied with command0-final-identity-command0-delivery-blocked.
- Command0 input delivery is accepted only if selected-kernel/TFTP precondition,
  ordered command0 serial delivery, and final pre-restore selected identity all
  pass: satisfied by rejection; ordered command0 serial delivery failed.
- Inconclusive capture/staging evidence does not unblock source-response
  retention or generated-root command-input success: satisfied; no follow-on
  success claim is accepted.

## Validation

- Pi 5 serialized hardware proof with hardwareTestLock: terminal blocked at
  ordered command0 serial delivery.
- Inconclusive-run triage before retry or code changes: not applicable; the run
  was terminal blocked, not inconclusive.
- jq empty on task-owned JSON evidence: pass.
- evidence map references every retained status, serial, TFTP, publish, and
  restore artifact: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-final-identity-regression-closeout-20260618 on the next
worker wake if dependencies remain satisfied, hardwareTestLock is unlocked and
restored, supervisorIntervention is inactive, and the repository has no
conflicting uncommitted changes. The closeout must reconcile that selected
identity stayed stable while ordered command0 delivery failed in this run, and
must not accept source-response retention, generated-root command-input
success, storage, networking, SSH, Phase 11/12 expansion, or a phase transition.
