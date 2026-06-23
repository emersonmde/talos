# Phase 12.6 SSH channel-window accounting closeout

Task id: phase12-ssh-channel-window-accounting-closeout-20260623

Status: accepted

Classification: phase12-ssh-channel-window-accounting-closeout-accepted

## Goal

Reconcile the accepted SSH channel-window accounting contract, source
implementation, feature smoke evidence, docs, validation, redaction boundary,
and readiness counters.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-channel-window-accounting-contract.md.
- tasks/2026-06-23-phase12-ssh-channel-window-accounting-core.md.
- tasks/2026-06-23-phase12-ssh-channel-window-accounting-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/ssh_service_readiness.rs.

## Closeout

The reconciled frontier covers only local modeled SSH channel-data/stdio plus
local channel-window accounting on the accepted authenticated session channel,
shell attachment, local process/session ownership, fd0/fd1/fd2 stdio
ownership, and open lifecycle path.

Inbound SSH_MSG_CHANNEL_DATA can decrement the fixed Talos local receive window
before accepted local stdin delivery and can emit a modeled
SSH_MSG_CHANNEL_WINDOW_ADJUST when the remaining local receive window reaches
or crosses the low-water threshold. Outbound stdout and stderr packet-shape
reports decrement the peer receive-window/send-budget counter. Inbound
SSH_MSG_CHANNEL_WINDOW_ADJUST can add a nonzero public bytes-to-add counter to
that outbound budget only when the addition does not overflow.

The closeout found one contract/source edge: the core implementation emitted a
local WINDOW_ADJUST only below the low-water threshold, while the accepted
contract said at or below. This task fixed the boundary to include exact
threshold equality and adjusted the focused unit evidence to exercise that
case.

Accepted counters are authentication-success=true, session-count=1,
channel-count=1, shell-request-count=1, shell-attached=true,
channel-data-stdio-local=true, and channel-window-management=true only for the
local modeled success paths. live-reachability=false and ssh-ready=false
remain authoritative.

This closeout does not accept live encrypted socket delivery, remote receipt,
hardware reachability, OpenSSH/POSIX/Linux compatibility, EOF/close/exit-status
behavior, broad command expansion, a phase transition, or ssh-ready=true.

## Findings

- fixed: reconciled contract, core, feature-smoke, docs, and source evidence
  for local modeled channel-window accounting.
- fixed: changed the low-water WINDOW_ADJUST trigger from below-threshold only
  to at-or-below-threshold, matching the accepted contract.
- fixed: updated the focused threshold unit test to prove exact-threshold
  WINDOW_ADJUST emission.
- fixed: retained explicit docs language that the accepted frontier is local
  modeled channel-data/stdio plus channel-window accounting only.
- deferred: live encrypted socket delivery, remote receipt, hardware proof,
  OpenSSH/POSIX/Linux compatibility, EOF/close/exit-status behavior, broad
  command expansion, phase transition, and ssh-ready=true remain for later
  explicit tasks.
- not-an-issue: no Pi 5 hardware, lab-controller API action, boot archive
  publication, or external action is required for this local closeout.

## Evidence

- Static review: accepted contract/core/feature-smoke task records, Phase 12
  SSH doc, roadmap, and src/ssh_service_readiness.rs reviewed.
- Source fix: src/ssh_service_readiness.rs now emits modeled local
  SSH_MSG_CHANNEL_WINDOW_ADJUST when the remaining receive window is less than
  or equal to SSH_CHANNEL_LOCAL_RECEIVE_WINDOW_ADJUST_THRESHOLD_BYTES.
- Focused smoke/regression:
  cargo -Zjson-target-spec test channel_window_accounting --quiet: pass; custom
  no_std harness ran 804 tests.
- Full unit gate:
  cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, 804
  no_std tests passed.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, public SSH
message names and numbers, fixed labels, fixed test/function names, public
counters, booleans, public lengths/categories, validation commands, and
classifications. It retains no private user data, channel identifiers, payload
bytes, command bytes, key/session material, live peer data, hardware data, or
boot artifacts.

## Result

Accepted as the bounded local modeled SSH channel-window accounting closeout.

selected_next_task=null. planningNeeded=true because no explicit queued/ready
follow-up task exists after this closeout for the worker to promote without
supervisor planning. Supervisor planning is required before live encrypted
socket delivery, hardware reachability, compatibility discrimination,
EOF/close/exit-status behavior, broad command expansion, phase transition, or
ssh-ready=true.
