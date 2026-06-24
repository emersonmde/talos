# Phase 12.6 SSH TFTP capture invariant reconciliation v9

Task id: phase12-ssh-tftp-capture-invariant-reconciliation-v9-20260624
Status: accepted
Owner: worker
Classification: capture-helper-timing-root-cause-ready

## Goal

Resolve the repeated selected-publish/pre-client TFTP capture invariant failure
before any further selected-candidate fetch retry or OpenSSH launch.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task, supervisor intervention, and
  taskQueue entry for this task.
- tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v8.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v6.md.
- tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-liveness-bracket-pi5-proof.md.
- tasks/2026-06-24-phase12-ssh-lab-tftp-capture-liveness-reconciliation.md.
- tasks/2026-06-24-phase12-ssh-baseline-control-tftp-liveness-recovery-pi5-proof.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Required-Before-Resume Checklist

Problem statement: after selected archive publication and before restore, the
expected path is selected boot archive published, Pi 5 power-cycle, firmware
request for da591740/kernel_2712.img, dnsmasq/TFTP log append, lab API cursor
advance, and pre-restore evidence collection while GET /status and
GET /boot/files still expose the selected tree. The accepted proof can then say
whether the selected 87,432-byte kernel was served in-window.

Invariant: after selected publication and before restore, fresh in-window TFTP
evidence must either show selected kernel_2712.img serves, prove the Pi did not
request TFTP, or prove the capture/log cursor is not observing the served root
or window. A stable zero-event helper result is not enough once later replay
from the same cursor exposes events.

Contradicting evidence: v5 accepted selected-candidate fetch with cursor
4666452 advancing to 4667803 and two selected 87,432-byte kernel serves. V8
immediately afterward published the same selected archive, saved cursor
4667803, power-cycled, and accepted a stable zero-event pre-client gate before
restore. Earlier v6 and bracket stable-zero cursors later replayed with events,
and this task's read-only replay now shows cursor 4667803 returns 13 parsed
events after restore.

Unproven assumptions:

- fixed: selected publication identity was verified by post-publish status and
  boot-files evidence in v5, v6, and v8.
- fixed: GET /tftp/logs cursor replay can prove delayed log-event visibility
  when the same cursor later returns events.
- deferred: dnsmasq log append latency versus helper sampling cadence is not
  yet isolated to one component.
- deferred: same-cursor stable-zero results do not prove no boot request unless
  paired with an independent in-window no-request discriminator.
- deferred: post-restore replay byte labels cannot prove selected-vs-baseline
  identity because the endpoint computes bytes from the current served root.
- not-an-issue: the restored baseline/control tree remains the correct safety
  restore point; the problem is evidence timing, not boot-tree restoration.

Approach A, read-only replay: replay retained cursors through GET /tftp/logs,
compare previous in-window stable-zero classifications with current parsed
event visibility, and retain only sanitized cursor, count, filename category,
and byte-category summaries. This task executed approach A.

Approach B, changed-timing hardware discriminator: publish the selected archive,
save a fresh TFTP cursor, power-cycle once, keep the selected tree published
through an extended pre-restore log-visibility window, sample GET /tftp/logs
from the same cursor on a fixed cadence, retain cursor_end and event categories
for every sample, then restore. This differs from the stale retries because it
changes capture timing and explicitly prevents post-restore byte relabeling
from being mistaken for selected-byte proof.

Smallest decisive discriminator: one selected-publish hardware proof with an
extended pre-restore TFTP visibility window. It distinguishes no boot request
from delayed/cursor-invisible logging by requiring either selected 87,432-byte
events while the selected tree is still published, no cursor advance throughout
the bounded pre-restore window plus independent serial/power evidence, or a
precise capture/cursor blocker. OpenSSH remains disallowed until this
discriminator proves selected-byte fetch in-window.

Workaround quarantine/removal plan: stale v8 closeout, remote-receipt,
compatibility, phase transition, and ssh-ready paths remain blocked. Stable-zero
helper outputs from v6, bracket, and v8 are quarantined as fail-closed
in-window evidence only; they must not be used as durable no-request proof once
same-cursor replay later exposes events. Post-restore replay may prove cursor
visibility but must not reclassify selected-vs-baseline byte identity.

## Read-Only Replay Summary

This task performed only read-only lab API inspection. It used GET /status,
GET /boot/files, GET /, and GET /tftp/logs with retained cursors. It did not
call POST endpoints, publish a boot tree, power-cycle hardware, run OpenSSH,
or acquire hardwareTestLock.

Current restored boot-files evidence reports kernel_2712.img entries at
104,136 bytes. GET / returns the deployed 404 endpoint-semantics response.
Replay from v8 cursor 4667803 now advances to cursor 4669154 and returns 13
parsed events, including two da591740/kernel_2712.img events labeled 104,136
bytes under the current restored root. Replay from v6 cursor 4662399 now
advances to the same cursor_end and returns 65 parsed events, including ten
baseline-labeled kernel events.

The replay proves the stable-zero helper result was not a durable no-boot-
request proof. It also does not prove selected-byte service for v8 because the
byte labels are computed from the currently restored root. The next boundary is
therefore capture-helper-timing-root-cause-ready, with supervisor planning
still required to encode the changed-timing discriminator as a separate bounded
task.

## Findings And Disposition

- fixed: answered the supervisor intervention checklist without hardware,
  boot publication, OpenSSH, or POST lab endpoints.
- fixed: retained sanitized read-only replay evidence showing v8 cursor 4667803
  now exposes parsed TFTP events after restore.
- fixed: quarantined post-restore replay as cursor/log visibility evidence, not
  selected-byte identity evidence.
- deferred: the exact dnsmasq append latency, lab endpoint cursor semantics, or
  helper sampling cadence root cause remains for the next changed-timing
  discriminator or helper repair task.
- deferred: selected-candidate fetch retry and live OpenSSH remain blocked until
  a future explicit task proves selected-byte service in-window.
- not-an-issue: no source, lab helper, Cargo metadata, hardware state, package
  installation, OpenSSH output, remote receipt, compatibility, phase transition,
  or ssh-ready claim was required or accepted.

## Evidence

- read-only replay summary:
  tasks/evidence/2026-06-24-phase12-ssh-tftp-capture-invariant-reconciliation-v9/read-only-replay-summary.sanitized.json.
- accepted v5 selected fetch proof:
  tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5.md
  and commit 63507f3de6b202174dd1905a9b22bd9b1ddd2551.
- accepted v8 fail-closed pre-client gate:
  tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v8.md
  and commit 35bffa16800b1a6410887298c27516d624099ced.
- accepted v6 stable-zero blocker:
  tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v6.md.
- accepted bracket baseline-control-fetch-missing proof:
  tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-liveness-bracket-pi5-proof.md.
- accepted liveness reconciliation:
  tasks/2026-06-24-phase12-ssh-lab-tftp-capture-liveness-reconciliation.md.
- accepted baseline recovery proof:
  tasks/2026-06-24-phase12-ssh-baseline-control-tftp-liveness-recovery-pi5-proof.md.

## Validation

- read-only lab API inspection: pass. Only GET endpoints were used; no POST,
  hardware action, boot publication, or OpenSSH launch occurred.
- jq empty on task-owned JSON evidence: pass.
- redaction review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static task/docs/source review, read-only lab-controller API
inspection, JSON syntax check, redaction grep review, docs build, and diff
checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree and
kernel byte categories, boot configuration keys, cursor numbers, parsed TFTP
event counts, public filename categories, validation commands, and
classifications. It retains no raw serial text, raw serial base64, raw TFTP
lines, client identities, user names, addresses, MAC addresses, OpenSSH logs,
known_hosts, host keys, authorized keys, key material, fingerprints,
signatures, session identifiers, channel identifiers, command bytes, payload
bytes, packet captures, boot artifact bytes, stable peer identifiers, or
private user data.

## Acceptance

Accepted as capture-helper-timing-root-cause-ready.

selected_next_task=null.
planningNeeded=true.

planningReason=Supervisor planning is required to encode the changed-timing
selected-publish discriminator or helper repair as an explicit bounded task;
the worker must not create a new task or retry selected fetch/OpenSSH from this
reconciliation.

No selected-candidate fetch retry, OpenSSH launch, TCP reachability, remote
receipt, compatibility, PTY/SCP/SFTP, broad command expansion, phase
transition, or ssh-ready=true is accepted.
