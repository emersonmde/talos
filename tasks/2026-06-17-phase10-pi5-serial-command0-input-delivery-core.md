# Phase 10 Pi 5 Serial Command0 Input-Delivery Core

Task id: phase10-pi5-serial-command0-input-delivery-core-20260617

Status: accepted

Classification:
command0-input-delivery-core-paused-outside-source-control

Evidence level: static/source inspection, helper fix, QEMU/substitute serial
ingress smoke, no_std unit test suite, task-owned JSON evidence, docs update,
and diff checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, storage work, networking, SSH, Phase 11/12
expansion, or phase transition was performed by this task.

## Goal

Find and fix the smallest local/source cause preventing an accepted
/serial/write payload from reaching command0 after the selected-kernel/TFTP
boundary recovered.

## Result

No kernel command-loop source defect was found. The local input path still has
the expected invariant:

- src/target/rpi5.rs runs the generated-root proof through
  runtime-console0/bcm2712-uart10-pl011 and reports ready command=0 before
  command input.
- src/local_command_loop.rs writes talos>, polls the runtime-console0 input
  backend through the TTY canonical-lite path, accepts both CR and LF as line
  terminators, dispatches rootinfo through write_generated_root_selection_line,
  and records command line/dispatch/response metadata.
- src/pl011.rs exposes UART RX through poll_read_byte on UART10 for the Pi
  local-command scenarios.

The prior hardware blocker is therefore not explained by a local command-loop
or rootinfo dispatch defect. It remains at the external/lab boundary: after a
visible command0 prompt and successful /serial/write of rootinfo, the bounded
post-write /serial/observe retained zero bytes. A follow-up Pi proof is not
selected by this core because the local/source work does not give a concrete
new hardware discriminator.

One helper defect was fixed: scripts/qemu-local-serial-write-ingress-control.sh
expected the old short builtins banner. The smoke now checks the current
expanded generated-root boundary marker separately while retaining the
runtime-console/descriptor-backed input checks.

## Findings

- fixed: qemu-local-serial-write-ingress-control no longer fails on unrelated
  LOCAL_COMMAND_BUILTIN_BOUNDARY growth.
- not-an-issue: rootinfo dispatch is present and returns Handled after writing
  the generated-root selection line.
- not-an-issue: canonical-lite input accepts the newline shape used by
  /serial/write append_newline=true.
- not-an-issue: the Pi generated-root proof wrapper polls
  runtime-console0/bcm2712-uart10-pl011 for command input.
- blocked: the accepted hardware blocker still lacks evidence that
  /serial/write bytes reached the Pi UART10 RX/capture boundary after the
  visible prompt.
- deferred: any Pi 5 hardware rerun requires supervisor planning for a new
  discriminator of the lab serial write-to-UART10/capture boundary.
- rejected: source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Prior blocker commit:
  e64810f3ec2cc88eeb65a600d1edfaa5f925ec2d.
- Prior blocker task:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery.md.
- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-input-delivery-core/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-input-delivery-core/evidence-map.json.
- Source inspection evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-input-delivery-core/source-inspection.json.
- QEMU/substitute serial ingress control:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-input-delivery-core/qemu-local-serial-write-ingress-control.log.

## Acceptance Check

- Identify the first source-level invariant, or classify outside source control:
  satisfied. The local source invariant is intact; the remaining unproven
  assumption is lab /serial/write delivery into the same Pi UART10 RX/capture
  boundary after a visible prompt.
- If a source or helper defect is found, fix it and record findings:
  satisfied for the QEMU ingress smoke helper.
- If no input-path source fix is made, record selected_next_task=null with
  planningNeeded=true: satisfied.
- Preserve rejected claims from the prior blocker: satisfied.
- Select Pi 5 input-delivery proof only if local/static core gives a concrete
  reason another hardware run can discriminate the fixed path: not selected.

## Validation

- static/source inspection: pass.
- helper fix: scripts/qemu-local-serial-write-ingress-control.sh updated.
- bash -n scripts/qemu-local-serial-write-ingress-control.sh: pass.
- cargo -Zjson-target-spec test --quiet
  local_command_loop_handles_empty_and_unknown_input_visibly: pass; the no_std
  harness ran 531 tests.
- QEMU/substitute serial ingress smoke: pass; prompt-delayed serial socket
  writes reached help, status, stdio, echo, empty, and unknown-command
  dispatch, with classification serial-write-ingress-control-complete.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

selected_next_task=null and planningNeeded=true. Supervisor planning is required
for any new discriminator of the lab serial write-to-UART10 delivery/capture
boundary before a Pi 5 hardware proof is promoted.
