# Phase 12.6 SSH selected-candidate lab-capture discriminator

Task id: phase12-ssh-lab-capture-selected-candidate-discriminator-20260623
Status: accepted
Owner: worker
Classification: selected-candidate-fetch-observed

## Goal

Prove or disprove that the selected 87,432-byte Talos candidate is fetched
through the lab TFTP path before retrying the live OpenSSH discriminator.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-lab-capture-selected-candidate-discriminator-20260623.
- tasks/2026-06-23-phase12-ssh-lab-capture-regression-reconciliation.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry.md.
- tasks/2026-06-23-phase12-ssh-lab-boot-capture-preflight.md.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/archive-review.txt.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/live-openssh-discriminator.summary.sanitized.json.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock before
any boot publication, power cycle, serial capture, or TFTP observation. The
task reused the exact previously reviewed selected candidate archive from the
live OpenSSH retry, preserving the 87,432-byte kernel_2712.img and kernel hash
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5 rather than
rebuilding a different input.

The deployed lab API returned 404 for GET /, so boot identity evidence used the
already documented authoritative GET /status endpoint. The run captured
pre-publication status/files/snapshots, created a pre-run snapshot, retained
fresh serial and TFTP cursors, published the selected candidate archive, power
cycled once, collected a stable same-cursor TFTP delta before restore, captured
serial status, recorded final pre-restore identity, restored the pre-run
snapshot, and released hardwareTestLock with the prior boot tree restored.

The stable TFTP delta observed 13 sanitized events from the saved cursor,
including two served da591740/kernel_2712.img fetches at 87,432 bytes. Final
pre-restore identity still reported selected tree
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333, and final
post-restore identity returned to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator/selected-candidate-discriminator.summary.sanitized.json.
- pre-run identity and snapshot:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-run-snapshot.sanitized.json, and
  pre-run-snapshot-name.txt.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, and archive-sizes.txt.
- fresh cursor evidence:
  pre-serial-peek.sanitized.json and pre-tftp-tail.sanitized.json.
- publication, power, and final selected identity:
  publish.sanitized.json, post-publish-status.sanitized.json,
  post-publish-boot-files.sanitized.json, power-cycle.sanitized.json,
  post-power-status.sanitized.json, final-pre-restore-status.sanitized.json,
  and final-pre-restore-boot-files.sanitized.json.
- selected-candidate fetch proof:
  tftp-delta.sanitized.json and tftp-delta.exit-code.txt.
- serial status:
  serial-observe.request.sanitized.json, serial-observe.sanitized.json, and
  serial-observe.exit-code.txt.
- restore proof:
  final-restore.sanitized.json, final-status.sanitized.json, and
  final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized
  lab/hardware window, with restore proof retained before release.
- fixed: used GET /status for authoritative boot identity after the deployed
  lab API returned GET / as 404 unknown endpoint.
- fixed: proved selected-candidate-fetch-observed=true with stable
  same-cursor TFTP evidence before restore: da591740/kernel_2712.img was served
  twice at the selected 87,432-byte size.
- fixed: restored the pre-run boot tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 after the
  selected-candidate discriminator.
- deferred: the saturated serial cursor direct-read path retained serial
  status only; selected-candidate acceptance is based on TFTP identity and
  final pre-restore/restore identity, not Talos runtime serial output.
- deferred: live OpenSSH retry-v2 is selected as the next bounded task, but no
  OpenSSH action, network client connection, live reachability, remote receipt,
  compatibility, PTY/SCP/SFTP, broad command expansion, phase transition, or
  ssh-ready=true is accepted here.
- not-an-issue: a restored known-good control rerun was skipped because this
  task's selected-candidate TFTP proof was decisive; the accepted
  lab-boot-capture-fresh task already records restored-control capture health,
  and another control would not prove the selected candidate was fetched.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab discriminator evidence with hardwareTestLock owned by
  this task: pass; stable same-cursor TFTP delta observed selected
  da591740/kernel_2712.img fetches at 87,432 bytes.
- restore proof showing hardwareTestLock.restored=true and the prior accepted
  boot identity restored: pass.
- jq empty on task-owned JSON evidence: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static task/docs/source review, static boot artifact
inspection, lab-controller API, serialized Pi 5 hardware power/restore/TFTP/
serial evidence, JSON syntax check, docs build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public archive/kernel hashes and sizes, boot configuration keys, cursor
numbers, TFTP event filename/status/byte categories, serial byte counts and
fixed marker booleans, validation commands, and classifications. It retains no
raw OpenSSH output, raw serial text, raw TFTP log lines, client identities,
user names, addresses, MAC addresses, host keys, authorized keys,
fingerprints, signatures, session identifiers, channel identifiers, command
bytes, payload bytes, packet captures, boot artifact bytes, stable peer
identifiers, or private user data.

## Acceptance

Accepted as selected-candidate-fetch-observed=true.

selected_next_task=phase12-ssh-live-openssh-client-discriminator-retry-v2-20260623.
planningNeeded=false.

No live reachability, remote receipt, compatibility, PTY/SCP/SFTP, broad
command expansion, phase transition, or ssh-ready=true is accepted.
