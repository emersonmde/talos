# Phase 12.6 SSH lab TFTP capture liveness reconciliation

Task id: phase12-ssh-lab-tftp-capture-liveness-reconciliation-20260624

Status: accepted

Classification: baseline-control-liveness-retry-authorized

## Goal

Reconcile the lab-controller/TFTP capture liveness blocker from the accepted
baseline-control-fetch-missing proof before any further selected-candidate
fetch retry or live OpenSSH attempt.

## References

- tasks/2026-06-23-phase12-ssh-boot-request-liveness-baseline-control.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v6.md.
- tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-liveness-bracket-pi5-proof.md.
- tasks/evidence/2026-06-24-phase12-ssh-lab-tftp-capture-liveness-reconciliation/reconciliation-summary.sanitized.json.

## Problem Statement

The expected baseline/control power-cycle invariant is: after saving a fresh
TFTP cursor and power-cycling the Pi 5 while the restored baseline/control boot
tree is active, a stable same-cursor TFTP delta should either contain
parseable boot-request events for da591740/kernel_2712.img, prove that the Pi
did not reboot/request TFTP, or prove that the capture/log cursor is not
observing the served root.

The accepted evidence is contradictory. The earlier baseline/control proof
advanced from cursor 4658346 to 4659697 and retained 13 parsed events including
two kernel serves. The selected-fetch v4 proof advanced from cursor 4659697 to
4661048 and retained 13 parsed events including two selected-window kernel
serves before restore. Later, v6 recorded stable zero events from cursor
4662399 after selected publication, and the bracket task recorded stable zero
events from cursor 4663750 after a baseline/control power-cycle.

Read-only replay through the deployed lab API now returns events from both
previous stable-zero cursors: cursor 4662399 returns 26 parsed events and
cursor 4663750 returns 13 parsed events. Because the lab-controller computes
the retained bytes field from the current served root at query time, this
replay proves cursor/log-event visibility but does not prove selected-vs-
baseline byte identity for earlier selected-root windows after restore.

## Assumptions

- fixed: treated GET /status and GET /boot/files as authoritative boot identity
  endpoints; GET / returns the deployed 404 endpoint-semantics response.
- fixed: kept the reconciliation read-only. No POST, boot publication, restore,
  power-cycle, hardware test, or OpenSSH launch was performed.
- deferred: the exact latency source between power-cycle, dnsmasq log append,
  and the stable-delta helper's zero-event classification is not proven here.
- not-an-issue: current read-only replay after restore is still useful for
  event/cursor accounting, as long as it is not used to reclassify selected
  archive byte identity.

## Approaches Considered

Approach A is a read-only cursor-accounting reconciliation: replay the retained
fresh cursors through GET /tftp/logs, compare cursor_start/cursor_end/log_size,
event counts, serial-byte summaries, and current boot identity, and decide
whether the blocker is a no-boot condition or a capture timing/visibility
condition. This task executed approach A.

Approach B is a bounded hardware discriminator: repeat one baseline/control
power-cycle from a fresh cursor, retain status/boot-files identity before and
after, and require stable same-cursor TFTP evidence before any selected-candidate
retry. This is selected as the next task because the read-only replay indicates
the Pi likely did request TFTP after the bracket cursor, but the acceptance
proof must be collected in-window before restore.

Approach C would repair or refactor lab helper timing. It is deferred because
the next hardware discriminator can produce the required acceptance evidence
without changing lab tooling first.

## Comparison

| Task | Accepted classification | Cursor | Accepted events | Replay observation |
| --- | --- | --- | --- | --- |
| baseline-control | baseline-control-fetch-observed | 4658346 -> 4659697 | 13 events, two baseline kernel serves | Current replay from 4658346 returns events. |
| selected fetch v4 | selected-candidate-fetch-observed | 4659697 -> 4661048 | 13 events, two selected-window kernel serves before restore | Current replay from 4659697 returns events, but post-restore byte labels are not used. |
| v6 preclient gate | stable-zero-tftp-after-selected-publish | 4662399 -> 4662399 | zero events | Current replay from 4662399 returns 26 events. |
| liveness bracket | baseline-control-fetch-missing | 4663750 -> 4663750 | zero events | Current replay from 4663750 returns 13 events. |

## Findings

- fixed: captured sanitized read-only GET /status, GET /boot/files, GET /,
  and GET /tftp/logs observations without raw TFTP lines, client identifiers,
  serial text, OpenSSH output, key material, or address identifiers.
- fixed: compared accepted baseline-control, selected fetch v4, v6 stable-zero,
  and bracket baseline-control-fetch-missing evidence in one retained summary.
- fixed: classified the blocker as baseline-control-liveness-retry-authorized
  because current read-only cursor replay shows that previously stable-zero
  cursors now expose parsed TFTP events, so the next decisive step is a single
  in-window baseline/control recovery proof.
- deferred: selected-candidate fetch reproducibility and live OpenSSH remain
  blocked until the baseline/control liveness recovery task accepts fresh
  same-task TFTP liveness.
- not-an-issue: no runtime source, helper repair, boot publication, hardware
  power action, OpenSSH launch, remote receipt, compatibility, phase transition,
  or ssh-ready claim was required or accepted.

## Validation

- read-only lab API observations only: pass. Used GET /status, GET /boot/files,
  GET /, and GET /tftp/logs with explicit cursors; no POST, boot archive
  publication, restore, power-cycle, hardware test, or OpenSSH launch occurred.
- comparison against retained evidence: pass. See
  reconciliation-summary.sanitized.json.
- jq empty on task-owned JSON evidence: pass.
- redaction review: pass. Retained JSON and markdown include no raw serial
  text, raw TFTP log lines, client identifiers, key material, fingerprints,
  known_hosts, raw OpenSSH logs, or stable peer identifiers.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass.
- Rust fmt/tests: conditionally skipped; no Rust source, tests, scripts that
  generate Rust artifacts, Cargo metadata, or lab helper source changed.

## Decision

Accepted as baseline-control-liveness-retry-authorized.

baseline_control_fetch_observed=false for this read-only reconciliation; it did
not run hardware and does not replace the failed bracket proof.

selected_candidate_fetch_observed=false.

selected_next_task=phase12-ssh-baseline-control-tftp-liveness-recovery-pi5-proof-20260624.

planningNeeded=false.

No selected-candidate fetch retry, live OpenSSH attempt, remote receipt,
compatibility, PTY/SCP/SFTP, broad command expansion, phase transition, or
ssh-ready=true is accepted.
