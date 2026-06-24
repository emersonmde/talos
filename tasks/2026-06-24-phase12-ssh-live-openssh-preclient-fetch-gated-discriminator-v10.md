# Phase 12.6 SSH live OpenSSH pre-client fetch-gated discriminator v10

Task id: phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10-20260624
Status: accepted
Owner: worker
Classification: live-openssh-client-discriminator-attempted-after-selected-fetch

## Goal

Retry the live OpenSSH discriminator only after v10 proves selected-byte TFTP
service in the extended pre-restore window.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and taskQueue entries.
- tasks/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10.md.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.

## Execution Summary

The worker promoted the ready v10 live OpenSSH discriminator task and acquired
hardwareTestLock before selected archive publication, Pi 5 power action, TFTP
capture, and any OpenSSH-eligible action. The accepted selected archive was
target/phase12-ssh-live-openssh-retry-boot.tar.gz with sha256
2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01 and
kernel_2712.img sha256
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5. The
selected kernel_2712.img entries were 87,432 bytes.

An initial attempt was interrupted by evidence collection timing and later
sampled TFTP after restore had changed the served byte labels. That attempt is
quarantined under attempt1-contaminated and is not used for selected-byte
identity or OpenSSH launch acceptance.

The clean rerun started from the restored baseline/control a0452458... tree,
created snapshot phase12-ssh-v10-openssh-clean-pre-20260624T074002Z, published
the selected fe9a0d98... tree, saved fresh serial and TFTP cursors, and
power-cycled the Pi 5 once. The pre-client TFTP gate retained same-cursor
pre-restore samples from cursor 4671856 at scheduled elapsed seconds 0, 5, 10,
20, 30, 45, and 60. The 30-second sample advanced to cursor 4673207 with 13
parsed events, including two served da591740/kernel_2712.img fetches at the
selected 87,432-byte category and no baseline 104,136-byte kernel fetches.
Samples at 45 and 60 seconds remained stable at that selected event set while
the selected tree stayed published.

Because the same-task pre-client selected-fetch gate passed, the worker
launched exactly one bounded workspace-local OpenSSH client discriminator. The
sanitized public result was no-tcp-connect with exit category tcp-timeout and
client exit code 255. Raw OpenSSH output was deleted and not retained. The
task accepts only that a bounded live client attempt was made after selected
TFTP byte service was proved in-window; it does not accept live reachability,
remote receipt, compatibility, phase transition, or ssh-ready=true.

Final pre-restore identity still reported the selected fe9a0d98... tree.
Restore returned the lab boot tree to the baseline/control a0452458... tree,
and hardwareTestLock was released with restored=true.

## Evidence

- clean rerun summary:
  tasks/evidence/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10/rerun-clean/live-openssh-preclient-fetch-gated-discriminator-v10.summary.sanitized.json.
- quarantined initial attempt:
  tasks/evidence/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10/attempt1-contaminated/.
- selected archive and client preflight:
  rerun-clean/archive-sha256.txt, archive-kernel-sha256.txt,
  archive-kernel-sizes.txt, archive-review.txt, and
  openssh-client-preflight.sanitized.json.
- lab identity, publication, power, and gate evidence:
  rerun-clean/pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-run-snapshot.sanitized.json, publish.sanitized.json,
  post-publish-status.sanitized.json, post-publish-boot-files.sanitized.json,
  pre-serial-peek.sanitized.json, pre-tftp-tail.sanitized.json,
  power-cycle.sanitized.json, tftp-preclient-samples/*.sanitized.json, and
  tftp-preclient-gate-summary.sanitized.json.
- conditional live client evidence:
  rerun-clean/openssh-attempt.sanitized.json.
- final and restore proof:
  rerun-clean/serial-window.sanitized.json,
  final-pre-client-status.sanitized.json,
  final-pre-restore-status.sanitized.json,
  final-pre-restore-tftp.sanitized.json, final-restore.sanitized.json,
  final-status.sanitized.json, and final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware/OpenSSH-eligible action
  and released it only after restoring the pre-run baseline/control tree.
- fixed: retained selected archive identity, selected boot tree identity, fresh
  cursors, same-cursor pre-restore pre-client TFTP gate samples, final
  pre-restore identity, restore proof, validation outputs, and redaction review.
- fixed: enforced the OpenSSH launch gate; the client was launched only after
  selected-byte da591740/kernel_2712.img service was observed in-window.
- fixed: retained a sanitized bounded OpenSSH result with raw output deleted.
- removed: the interrupted first attempt is quarantined from selected-byte
  identity and OpenSSH acceptance because later samples were contaminated by
  restore-time byte labeling.
- deferred: live reachability, remote receipt, compatibility, PTY/SCP/SFTP,
  broad command expansion, phase transition, and ssh-ready=true remain
  unaccepted.
- not-an-issue: no Talos runtime source, lab helper source, package
  installation, raw OpenSSH transcript retention, or key material retention was
  required for this discriminator task.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock with restore proof:
  pass, clean rerun restored baseline/control tree.
- pre-client selected-fetch gate using same-cursor pre-restore TFTP evidence:
  pass, selected 87,432-byte kernel serves observed before OpenSSH launch.
- bounded OpenSSH client discriminator only after gate pass: pass, launched
  once and classified as no-tcp-connect/tcp-timeout.
- jq empty on task-owned JSON evidence: pass.
- redaction review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static archive/client inspection, lab-controller API,
serialized Pi 5 hardware power/restore/TFTP/serial evidence, sanitized bounded
OpenSSH client execution, JSON syntax check, redaction grep review, docs build,
and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree and
archive/kernel hashes, public kernel byte categories, boot configuration keys,
snapshot names, cursor numbers, sanitized TFTP event status/filename/byte
categories, serial byte counts and marker booleans, OpenSSH invocation class,
public phase/exit categories, validation commands, and classifications. It
retains no raw OpenSSH output, raw serial text, raw serial base64, raw TFTP log
lines, client identities, user names, addresses, MAC addresses, host keys,
authorized keys, fingerprints, signatures, session identifiers, channel
identifiers, command bytes, payload bytes, packet captures, stable peer
identifiers, or private user data.

## Acceptance

Accepted as live-openssh-client-discriminator-attempted-after-selected-fetch.

preclient_selected_fetch_gate_passed=true.
live_openssh_client_discriminator_observed=false.
openssh_public_phase=no-tcp-connect.
openssh_exit_category=tcp-timeout.
selected_next_task=phase12-ssh-tftp-capture-invariant-closeout-20260624.
planningNeeded=false.

No TCP reachability, remote receipt, compatibility, PTY/SCP/SFTP, broad command
expansion, phase transition, or ssh-ready=true is accepted.
