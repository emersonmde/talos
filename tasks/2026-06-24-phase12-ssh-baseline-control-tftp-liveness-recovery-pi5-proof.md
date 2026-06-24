# Phase 12.6 SSH baseline/control TFTP liveness recovery Pi 5 proof

Task id: phase12-ssh-baseline-control-tftp-liveness-recovery-pi5-proof-20260624
Status: accepted
Owner: worker
Classification: baseline-control-fetch-liveness-recovered

## Goal

Run one serialized Pi 5 baseline/control boot-request liveness recovery proof
after read-only reconciliation selected this exact discriminator.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and queue entries.
- tasks/2026-06-24-phase12-ssh-lab-tftp-capture-liveness-reconciliation.md.
- tasks/2026-06-23-phase12-ssh-boot-request-liveness-baseline-control.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock before
lab/hardware action. The task did not publish a selected archive, run OpenSSH,
change Talos runtime code, or attempt live reachability.

The pre-run baseline/control identity was the restored
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 tree with
effective_kernel=kernel_2712.img and root plus da591740-prefixed
kernel_2712.img entries at 104,136 bytes. GET / retained the deployed
404 endpoint-semantics response, so GET /status and GET /boot/files remained
the authoritative boot identity samples. The worker created snapshot
phase12-ssh-baseline-liveness-pre-20260624T041930Z, saved fresh serial and TFTP
cursors, and power-cycled the Pi once.

Stable same-cursor TFTP evidence advanced from cursor 4665101 to 4666452 and
retained 13 parsed events before restore, including two served
da591740/kernel_2712.img fetches at 104,136 bytes. Serial observation used the
saturated-cursor direct-read fallback from cursor 4194304, captured 6,555 bytes,
and saw firmware NETWORK markers, but it did not observe TALOS: kernel_main or
the run marker; serial runtime readiness remains unaccepted.

Final pre-restore, restore, and final restored identities all reported the same
baseline/control a0452458... tree with effective_kernel=kernel_2712.img.
baseline_control_fetch_observed=true. The selected next bounded task is
phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5-20260624.

## Evidence

- summary:
  tasks/evidence/2026-06-24-phase12-ssh-baseline-control-tftp-liveness-recovery-pi5-proof/baseline-control-liveness-recovery.summary.sanitized.json.
- pre-run identity and cursors:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-root-endpoint.sanitized.json,
  pre-run-snapshot-name.txt, pre-run-snapshot.sanitized.json,
  pre-serial-peek.sanitized.json, pre-tftp-tail.sanitized.json,
  serial-cursor.txt, and tftp-cursor.txt.
- hardware evidence:
  power-cycle.sanitized.json, serial-window.sanitized.json,
  serial-window.exit-code.txt, tftp-delta.sanitized.json, and
  tftp-delta.exit-code.txt.
- final identity and restore proof:
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json, final-restore.sanitized.json,
  final-status.sanitized.json, and final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware action and released it
  only after restoring and recording the pre-run baseline/control boot tree.
- fixed: retained baseline/control pre-run identity, GET / endpoint semantics,
  fresh serial cursor, fresh TFTP cursor, stable same-cursor TFTP delta, serial
  freshness summary, final pre-restore identity, restore identity, and
  validation outputs.
- fixed: recovered baseline/control TFTP liveness with two fresh same-run
  104,136-byte da591740/kernel_2712.img serves before restore.
- deferred: selected-candidate fetch reproducibility remains blocked until the
  selected v5 task proves or rejects it from a fresh selected publication.
- deferred: serial runtime readiness remains unaccepted because the saturated
  direct-read window showed firmware NETWORK markers but no TALOS kernel marker
  or run marker.
- not-an-issue: no selected archive publication, OpenSSH execution, runtime
  source change, helper repair, packet capture, remote-receipt claim,
  compatibility claim, phase transition, or ssh-ready claim was required or
  accepted.

## Validation

- serialized Pi 5 baseline/control boot-request liveness proof under
  hardwareTestLock: pass, baseline-control-fetch-liveness-recovered.
- candidate identity via lab API before power-cycle: pass. GET / returned the
  deployed 404 endpoint-semantics response; GET /status and GET /boot/files
  retained authoritative baseline/control identity.
- fresh serial cursor/nonce evidence where available: pass as status evidence.
  The saved cursor was saturated; the direct-read fallback captured firmware
  NETWORK markers but not Talos runtime readiness.
- TFTP delta via GET /tftp/logs from a fresh cursor: pass. Stable same-cursor
  delta advanced from 4665101 to 4666452 with 13 parsed events and two
  baseline/control kernel serves.
- restore proof showing hardwareTestLock.restored=true and prior boot identity
  restored: pass.
- jq empty on task-owned JSON evidence: pass.
- redaction review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static task/docs/source review, lab-controller API,
serialized Pi 5 hardware power/restore/TFTP/serial evidence, JSON syntax
check, redaction grep review, docs build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public kernel sizes, boot configuration keys, cursor numbers, TFTP
event status/filename/byte categories, serial byte counts and marker booleans,
validation commands, and classifications. It retains no raw serial text, raw
serial base64, raw TFTP log lines, client identities, user names, addresses,
MAC addresses, OpenSSH logs, known_hosts, key material, fingerprints,
signatures, session identifiers, channel identifiers, command bytes, payload
bytes, packet captures, boot artifact bytes, stable peer identifiers, or
private user data.

## Acceptance

Accepted as baseline-control-fetch-liveness-recovered.

baseline_control_fetch_observed=true.

selected_next_task=phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5-20260624.

planningNeeded=false.

No selected-candidate fetch, OpenSSH execution, TCP reachability, remote
receipt, compatibility, PTY/SCP/SFTP, broad command expansion, phase
transition, or ssh-ready=true is accepted from this task.
