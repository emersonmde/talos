# Phase 12.6 SSH lab-capture regression reconciliation

Task id: phase12-ssh-lab-capture-regression-reconciliation-20260623
Status: accepted
Owner: worker
Classification: lab-capture-regression-reconciled-selected-candidate-discriminator-needed

## Goal

Reconcile the lab-capture-regressed live OpenSSH discriminator evidence from
first principles before any same-shaped hardware or OpenSSH retry.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-lab-capture-regression-reconciliation-20260623.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-discriminator-pi5-evidence.md.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-tooling-preflight.md.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- tasks/2026-06-23-phase12-ssh-lab-boot-capture-preflight.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry.md.
- tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/lab-boot-capture-preflight.summary.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/tftp-delta-before-restore.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/live-openssh-discriminator.summary.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/tftp-delta.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/post-publish-status.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/post-power-status.sanitized.json.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/final-status.sanitized.json.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## First-Principles Reconciliation

Problem statement: a live OpenSSH discriminator is meaningful only if the same
run first proves that the selected Talos Pi 5 boot tree was published, fetched
from the TFTP root, and remained the final pre-restore boot identity. The
OpenSSH client result cannot upgrade any SSH frontier when the boot evidence
does not prove which kernel image was exercised.

Invariant that should hold: after publishing a selected candidate and before
the hardware power cycle, GET /status, the visible boot files, the saved TFTP
cursor, the stable pre-restore TFTP delta, and the final pre-restore identity
must all join to the selected candidate tree unless an explicit restore occurs
after capture. The TFTP delta must be collected before restore because served
file byte counts are computed from the current TFTP root at query time.

Contradicting evidence: the lab boot-capture preflight accepted a restored
control tree as fresh because a same-cursor TFTP delta recorded 13 events,
including two served da591740/kernel_2712.img fetches at 104,136 bytes, and
final pre-restore/post-restore identity matched the control tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The later
live retry published candidate tree
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333 with an
87,432-byte kernel_2712.img; GET /status still reported that candidate after
power, but the saved-cursor TFTP delta had zero events and the saturated serial
cursor produced zero fresh bytes. That proves the OpenSSH attempt ran after a
candidate publication, but it does not prove the Pi fetched or executed that
candidate.

Unproven assumptions:

- Publication path: PUT /boot/archive updating GET /status is not by itself
  proof that the Pi firmware fetched from that tree on the following power
  cycle.
- TFTP cursor semantics: the retry evidence records a zero-event TFTP delta,
  but selected-candidate acceptance needs an explicitly stable same-cursor
  sample before restore, with talos_tftp_stability or equivalent evidence.
- Effective-kernel resolution: kernel_2712.img remained configured and
  effective, but the retry did not retain a selected-candidate fetch event for
  that filename.
- Serial freshness: the retained serial cursor was saturated and produced zero
  fresh bytes, so serial cannot distinguish TFTP capture blindness from a boot
  path that made no observable progress.
- OpenSSH timing: the TCP timeout is only secondary client evidence because the
  exercised boot identity was not proven.
- Control applicability: a restored-control TFTP fetch proves the lab can
  observe baseline/control boots, not that the same capture chain observes a
  freshly published candidate boot.

## Recovery Approaches

Approach 1, lab/API publication-path trace with no hardware power first:
publish a selected candidate, then compare GET /status, GET /boot/files, the
selected kernel_2712.img size/hash category, and a TFTP-tail cursor without a
power cycle. This can prove whether the API-visible boot root matches the
selected candidate before hardware is involved, but it cannot prove a Pi fetch.

Approach 2, hardware-backed minimal selected-candidate sentinel: under
hardwareTestLock, publish the selected candidate, save fresh serial and TFTP
cursors, power-cycle once, collect a stable pre-restore TFTP delta and final
pre-restore identity, then restore. This omits OpenSSH and any new runtime
feature claim, so the result isolates the selected-candidate fetch question
without mixing in TCP, SSH, authentication, or shell behavior.

Smallest decisive discriminator: run the queued no-OpenSSH selected-candidate
lab-capture discriminator. It should accept
selected-candidate-fetch-observed=true only if the fresh stable TFTP delta or
an equivalent public lab-controller signal proves the selected kernel_2712.img
was served after publication and before restore. Otherwise it must fail closed
with a concrete lab-capture blocker.

Workaround removal/quarantine plan: do not use the accepted
lab-boot-capture-fresh preflight alone to unblock another live OpenSSH retry.
It remains useful evidence that the control tree can produce observable TFTP
events, but it is quarantined as insufficient for selected-candidate fetch. The
stale original live OpenSSH closeout remains blocked, and the remote-receipt
contract remains gated behind a retry-v2 closeout after a selected-candidate
fetch is proven.

## Findings And Disposition

- fixed: reconciled the contradiction without lab/hardware/OpenSSH action and
  preserved the live retry's fail-closed lab-capture-regressed classification.
- fixed: recorded that GET /status candidate identity after publish/power is
  necessary but not sufficient without same-run selected-candidate TFTP fetch
  or equivalent public lab-controller evidence.
- fixed: selected the already queued no-OpenSSH selected-candidate
  lab-capture discriminator as the next objective worker task.
- removed: the lab boot-capture preflight is no longer treated as a sufficient
  standalone precondition for another live OpenSSH retry.
- deferred: any live OpenSSH retry, remote-receipt contract, compatibility
  claim, PTY/SCP/SFTP, broad command expansion, phase transition, and
  ssh-ready=true remain blocked until selected-candidate fetch is accepted.
- not-an-issue: no Talos runtime code change is warranted by this
  reconciliation; the current blocker is evidence-chain identity, not a
  source-level SSH defect.

## Selected Next Task

selected_next_task=phase12-ssh-lab-capture-selected-candidate-discriminator-20260623.

planningNeeded=false.

## Validation

- static task/docs/evidence review: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests,
  scripts that generate Rust artifacts, Cargo metadata, or lab helper source
  touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static task/docs/evidence review. No lab/hardware action, boot
publication, power-cycle, OpenSSH execution, network connection, runtime
behavior change, live reachability claim, remote receipt claim, compatibility
claim, broad command expansion, phase transition, or ssh-ready=true was
performed.

## Redaction Review

Pass. Retained evidence in this task record is limited to public task ids,
file paths, boot tree hashes, public kernel filenames and size categories,
TFTP event counts, fixed classifications, validation commands, and selected
next task ids. It retains no raw OpenSSH logs, raw serial text, raw TFTP log
lines, client identities, user names, addresses, MAC addresses, host keys,
authorized keys, fingerprints, signatures, session identifiers, channel
identifiers, command bytes, payload bytes, packet captures, boot artifact
bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted as
lab-capture-regression-reconciled-selected-candidate-discriminator-needed.

No live reachability, remote receipt, compatibility, PTY/SCP/SFTP, broad
command expansion, phase transition, or ssh-ready=true is accepted.
