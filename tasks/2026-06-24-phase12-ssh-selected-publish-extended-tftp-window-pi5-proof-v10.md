# Phase 12.6 SSH selected-publish extended TFTP window proof v10

Task id: phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10-20260624
Status: accepted
Owner: worker
Classification: selected-publish-extended-window-fetch-observed

## Goal

Run one changed-timing selected-publish Pi 5 discriminator that keeps the
selected tree published through an extended pre-restore TFTP visibility window,
distinguishing selected-byte service from delayed/cursor-invisible logging
before any OpenSSH retry.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and taskQueue entries.
- tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-reconciliation-v9.md.
- tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v8.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted the ready v10 task, acquired hardwareTestLock before boot
publication, serial/TFTP capture, Pi 5 power action, and restore-sensitive lab
action, and released the lock only after restoring the pre-run baseline/control
tree.

The selected archive was target/phase12-ssh-live-openssh-retry-boot.tar.gz with
archive sha256
2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01 and
kernel_2712.img sha256
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5. The root
and da591740-prefixed kernel_2712.img and kernel8.img entries were 87,432
bytes.

Pre-run identity was the restored baseline/control tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
104,136-byte kernel entries. The worker created snapshot
phase12-ssh-v10-pre-20260624T071507Z, published the selected archive, verified
post-publish status exposed selected tree
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333 with
87,432-byte kernel entries, saved fresh serial and TFTP cursors, and
power-cycled the Pi 5 once. OpenSSH was not launched.

The retained pre-restore TFTP cadence sampled GET /tftp/logs from the same
cursor 4669154 at scheduled elapsed seconds 0, 5, 10, 20, 30, 45, 60, 75, 90,
105, and 120 while the selected tree remained published. Samples through 20
seconds stayed at zero parsed events. The 30-second sample advanced to cursor
4670505 with 13 parsed events, including two served
da591740/kernel_2712.img fetches at the selected 87,432-byte category and no
baseline 104,136-byte kernel fetches. Samples through 120 seconds remained
stable at the same cursor and event set.

Final pre-restore status still exposed the selected tree. Serial observation
used the saturated-cursor direct-read fallback from cursor 4194304, retained no
raw text, captured two bytes, and did not observe TALOS: kernel_main or
firmware NETWORK markers. Restore returned the boot tree to the
baseline/control a0452458... tree with 104,136-byte kernel entries.

During evidence review, the worker found the initial /boot/files sanitizer had
kept action/ok but dropped files from the deployed response's boot.files shape.
The worker reacquired hardwareTestLock for a no-power evidence retention
repair, republished the same selected archive, recaptured GET /boot/files with
the corrected sanitizer, restored the same baseline/control tree, and released
the lock. This repair did not power-cycle the Pi, launch OpenSSH, or contribute
to selected-byte TFTP proof; the accepted TFTP classification still comes only
from the original pre-restore cadence samples.

selected_publish_extended_window_fetch_observed=true. The dependency-gated
next bounded task is
phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10-20260624; no
OpenSSH, TCP reachability, remote receipt, compatibility, phase transition, or
ssh-ready=true claim is accepted from this task.

## Evidence

- summary:
  tasks/evidence/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10/selected-publish-extended-window-v10.summary.sanitized.json.
- archive review:
  archive-review.txt, archive-sha256.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt in the same evidence directory.
- pre-run identity and restore point:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-root-endpoint.sanitized.json,
  pre-run-snapshot-name.txt, and pre-run-snapshot.sanitized.json.
- publication and hardware evidence:
  publish.sanitized.json, post-publish-status.sanitized.json,
  post-publish-boot-files.sanitized.json, post-publish-root-endpoint.sanitized.json,
  pre-serial-peek.sanitized.json, serial-cursor.txt,
  pre-tftp-tail.sanitized.json, tftp-cursor.txt,
  power-cycle.sanitized.json, tftp-cadence-samples/*.sanitized.json,
  tftp-cadence-summary.sanitized.json, final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json,
  final-pre-restore-tftp.sanitized.json, serial-window.sanitized.json, and
  serial-window.exit-code.txt.
- restore proof:
  final-restore.sanitized.json, final-status.sanitized.json, and
  final-boot-files.sanitized.json.
- no-power boot-files evidence retention repair:
  boot-files-repair-snapshot-name.txt,
  boot-files-repair-pre-snapshot.sanitized.json,
  boot-files-repair-publish.sanitized.json,
  boot-files-repair-selected-status.sanitized.json,
  boot-files-repair-restore.sanitized.json, and the corrected
  post-publish-boot-files.sanitized.json / final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around selected publication,
  Pi 5 power/TFTP/serial capture, and restore-sensitive lab action.
- fixed: retained selected archive identity, selected boot tree identity,
  fresh cursors, fixed-cadence pre-restore TFTP samples, final pre-restore
  identity, restore proof, validation outputs, and redaction review.
- fixed: corrected the /boot/files evidence sanitizer and recaptured selected
  and restored boot-files evidence under a no-power lock-protected
  publish/restore repair.
- fixed: proved selected-byte service in-window before restore with two
  da591740/kernel_2712.img serves at 87,432 bytes while the selected tree still
  exposed the same fe9a0d98... identity.
- fixed: preserved v9's quarantine: post-restore replay was not used for
  selected-vs-baseline byte identity.
- deferred: live OpenSSH remains blocked until the dependency-gated v10
  pre-client fetch task observes selected-byte service in-window before launch.
- not-an-issue: no Talos runtime source, lab helper source, package
  installation, OpenSSH output, remote receipt, compatibility, phase
  transition, or ssh-ready claim was required or accepted.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock with pre-run snapshot
  and restore proof: pass, selected-publish-extended-window-fetch-observed.
- candidate identity via lab API GET /status, GET /boot/files, and GET / before
  power-cycle: pass. GET / retained deployed 404 endpoint-semantics evidence;
  GET /status and GET /boot/files retained authoritative selected identity.
- fresh serial cursor/window evidence: pass as sanitized cursor/window
  evidence; serial runtime readiness remains unaccepted.
- TFTP cadence evidence from GET /tftp/logs using the same saved cursor before
  restore: pass, 11 retained samples over the bounded extended pre-restore
  window.
- known-good baseline/control restore proof: pass, final restored identity
  matched the pre-run baseline/control tree.
- jq empty on task-owned JSON evidence: pass.
- redaction review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static archive inspection, lab-controller API, serialized Pi
5 hardware power/restore/TFTP/serial evidence, JSON syntax check, redaction
grep review, docs build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree and
archive/kernel hashes, public kernel byte categories, boot configuration keys,
snapshot names, cursor numbers, sanitized TFTP event status/filename/byte
categories, serial byte counts and marker booleans, validation commands, and
classifications. It retains no raw serial text, raw serial base64, raw TFTP
lines, client identities, user names, addresses, MAC addresses, OpenSSH logs,
known_hosts, host keys, authorized keys, key material, fingerprints,
signatures, session identifiers, channel identifiers, command bytes, payload
bytes, packet captures, boot artifact bytes, stable peer identifiers, or
private user data.

## Acceptance

Accepted as selected-publish-extended-window-fetch-observed.

selected_publish_extended_window_fetch_observed=true.

selected_next_task=phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10-20260624.

planningNeeded=false.

No OpenSSH execution, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted from this task.
