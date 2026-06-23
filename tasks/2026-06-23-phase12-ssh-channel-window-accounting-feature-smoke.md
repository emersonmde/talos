# Phase 12.6 SSH channel-window accounting feature smoke

Task id: phase12-ssh-channel-window-accounting-feature-smoke-20260623

Status: accepted

Classification: phase12-ssh-channel-window-accounting-feature-smoke-accepted

## Goal

Retain bounded feature smoke/regression evidence for the accepted local modeled
SSH channel-window accounting core from
phase12-ssh-channel-window-accounting-core-20260623.

## Scope

- Reused the accepted local no_std test surface in src/ssh_service_readiness.rs.
- Exercised accepted inbound SSH_MSG_CHANNEL_DATA receive-window decrement,
  low-water SSH_MSG_CHANNEL_WINDOW_ADJUST emission, outbound stdout/stderr
  send-budget decrement, and inbound SSH_MSG_CHANNEL_WINDOW_ADJUST accounting.
- Retained regression coverage for previous channel-data/stdio behavior,
  over-window data, malformed WINDOW_ADJUST, overflow, lifecycle/prerequisite
  rejection, redaction-sensitive input, and ssh-ready=false counters.
- Added no new source behavior, no live socket delivery, no hardware/lab
  action, no boot publication, no OpenSSH compatibility claim, no phase
  transition, and no ssh-ready=true claim.

## Smoke Evidence

The task-owned feature evidence is the existing local no_std unit coverage:

- channel_window_accounting_decrements_inbound_data_without_socket_readiness:
  covers accepted inbound SSH_MSG_CHANNEL_DATA decrementing the Talos local
  receive-window counter, setting channel-window-management=true, keeping
  live-reachability=false, and keeping ssh-ready=false.
- channel_window_accounting_emits_local_window_adjust_at_threshold: covers
  local low-water receive-window behavior and modeled
  SSH_MSG_CHANNEL_WINDOW_ADJUST emission with public bytes-to-add counter
  evidence only.
- channel_window_accounting_decrements_outbound_and_accepts_window_adjust:
  covers stdout/stderr outbound send-budget decrement and inbound
  SSH_MSG_CHANNEL_WINDOW_ADJUST increasing the modeled remote send budget
  without retaining payload bytes.
- channel_window_accounting_rejects_over_window_data_without_mutation: covers
  inbound and outbound over-window rejection with no stdio delivery, no
  channel-window-management success, and no state mutation.
- channel_window_adjust_fails_closed_for_malformed_or_overflow: covers zero,
  malformed, and overflow WINDOW_ADJUST controls.
- channel_data_stdio_accepts_local_modeled_inbound_stdin,
  channel_data_stdio_reports_local_stdout_and_stderr_packet_shapes,
  channel_data_stdio_fails_closed_for_prerequisites_and_lifecycle, and
  channel_data_stdio_fails_closed_for_message_shape_and_output_controls:
  retain the prior channel-data/stdio bridge regressions below the
  window-accounting layer.

The local harness command was:

    cargo -Zjson-target-spec test channel_window_accounting --quiet

The repository's custom no_std harness ran the full 804-test corpus for that
command and passed. This is local unit-test evidence only; it is not live
encrypted socket delivery, remote receipt, hardware reachability,
OpenSSH/POSIX/Linux compatibility, a phase transition, or ssh-ready=true.

## Findings

- not-an-issue: the accepted core task already contains the feature and
  regression tests needed for this smoke slice, so no new source behavior was
  required.
- fixed: retained explicit task-owned evidence mapping accepted inbound data,
  outbound stdout/stderr, and inbound WINDOW_ADJUST behavior to local modeled
  channel-window accounting.
- fixed: retained regression evidence for over-window, malformed, overflow,
  lifecycle/prerequisite, redaction-sensitive, and prior channel-data/stdio
  fail-closed paths.
- deferred: live encrypted socket delivery, remote receipt, hardware proof,
  OpenSSH/POSIX/Linux compatibility, EOF/close/exit-status behavior, broad
  command expansion, phase transition, and ssh-ready=true remain for later
  explicit tasks.

## Evidence

- Feature smoke/unit tests:
  cargo -Zjson-target-spec test channel_window_accounting --quiet: pass; custom
  no_std harness ran 804 tests.
- Full unit gate:
  cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, 804
  no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, public SSH
message names and numbers, fixed test/function names, public counters,
booleans, public lengths/categories, validation commands, and classifications.
It retains no private user data, channel identifiers, payload bytes, command
bytes, key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as bounded local feature smoke/regression evidence for the modeled SSH
channel-window accounting layer.

selected_next_task=phase12-ssh-channel-window-accounting-closeout-20260623.
