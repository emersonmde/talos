# Phase 12.6 SSH boot-request liveness baseline/control

Task id: phase12-ssh-boot-request-liveness-baseline-control-20260623
Status: accepted
Owner: worker
Classification: baseline-control-fetch-observed

## Goal

Run one baseline/control boot-request liveness discriminator after selected
fetch v3 accepted stable-zero-tftp-after-selected-publish.

## Reviewed Inputs

- memory/talos-supervisor-state.json currentTask and taskQueue entries for this
  task, selected fetch v4, zero-TFTP closeout, retry-v4, and retry-v5.
- tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-root-cause-v3.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one ready task and acquired hardwareTestLock before
lab/hardware action. The task did not publish a selected archive or run
OpenSSH. It captured the restored baseline/control boot identity:

- boot tree: a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- effective kernel: kernel_2712.img.
- kernel_2712.img size: 104,136 bytes.

After saving fresh serial and TFTP cursors, the worker power-cycled the Pi and
captured a stable same-cursor TFTP delta before restore. The delta advanced
from cursor 4658346 to 4659697 and retained 13 parsed events, including two
served da591740/kernel_2712.img fetches at 104,136 bytes. Serial observation
from the saturated cursor returned zero bytes, so serial runtime readiness is
not accepted from this run. Final pre-restore and restore identity both showed
the baseline/control tree.

Because same-run TFTP evidence proves the restored baseline/control
kernel_2712.img was served after the power-cycle and before restore,
baseline_control_fetch_observed=true. The selected next bounded task is
phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4-20260623.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-boot-request-liveness-baseline-control/baseline-control-liveness.summary.sanitized.json.
- selected fetch v3 reference:
  tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-root-cause-v3.md
  and its selected-fetch-v3.summary.sanitized.json evidence.
- pre-run identity and cursors:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-run-snapshot-name.txt,
  pre-run-snapshot.sanitized.json, pre-serial-peek.sanitized.json,
  serial-cursor.txt, pre-tftp-tail.sanitized.json, and tftp-cursor.txt.
- hardware evidence:
  power-cycle.sanitized.json, serial-observe.sanitized.json,
  serial-observe.exit-code.txt, tftp-delta.sanitized.json, and
  tftp-delta.exit-code.txt.
- final identity and restore proof:
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json, final-restore.sanitized.json,
  final-status.sanitized.json, and final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware action and released it
  only after restoring the pre-run baseline/control boot tree.
- fixed: retained baseline/control identity, selected-fetch-v3 proof reference,
  effective kernel, restored kernel hash/size category, fresh serial cursor,
  fresh TFTP cursor/tail, stable same-cursor TFTP delta, final pre-restore
  identity, restore identity, and redaction review.
- fixed: proved baseline/control boot-request liveness through two fresh
  same-run da591740/kernel_2712.img TFTP serves at 104,136 bytes.
- deferred: serial runtime readiness remains unaccepted because the saturated
  serial cursor returned zero bytes; this does not block the TFTP boot-request
  liveness claim accepted here.
- deferred: selected-candidate fetch v4 is the only selected next task; live
  OpenSSH retry-v5 remains dependency-gated behind that future selected fetch
  proof.
- not-an-issue: no selected archive publication, OpenSSH execution, Rust source
  change, or lab helper change was needed for this bounded discriminator.
- removed: retry-v4 and closeout-v4 remain blocked/superseded and are not
  revived by this baseline/control result.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab baseline/control liveness evidence with hardwareTestLock
  owned by this task: pass, baseline-control-fetch-observed.
- inconclusive-run triage evidence before code changes: current identity, fresh
  serial cursor, fresh TFTP cursor/delta, selected-fetch-v3 proof reference,
  final pre-restore identity, and restore identity retained.
- restore proof showing hardwareTestLock.restored=true and the prior boot
  identity restored: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static task/docs/source review, lab-controller API,
serialized Pi 5 hardware power/restore/TFTP/serial evidence, JSON syntax
check, docs build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public kernel sizes, boot configuration keys, cursor numbers, TFTP
event status/filename/byte categories, serial byte counts, validation commands,
and classifications. It retains no raw OpenSSH output, raw serial text, raw
serial base64, raw TFTP lines, client identities, user names, addresses, MAC
addresses, host keys, authorized keys, fingerprints, signatures, session
identifiers, channel identifiers, command bytes, payload bytes, packet
captures, boot artifact bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted as baseline-control-fetch-observed.

baseline_control_fetch_observed=true.

selected_next_task=phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4-20260623.

planningNeeded=false.

No selected-candidate fetch, OpenSSH execution, TCP reachability, remote
receipt, compatibility, PTY/SCP/SFTP, broad command expansion, phase
transition, or ssh-ready=true is accepted.
